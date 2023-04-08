"""Convert the ParsedNode into the AbcSyntaxNode."""

from typing import Set, Tuple, Dict, Optional, cast
from .node_visit import post_visit_parsed_node, pre_visit_parsed_node
from ..defs.add_ins import AddInTypeHandler
from ..defs.parse_tree import (
    AbcParsedNode,
    ParsedListNode,
    ParsedParameterNode,
    ParsedSimpleNode,
)
from ..defs.syntax_tree import (
    SyntaxNode,
    TypeParameter,
    AbcTypeProperty,
    AbcType,
    ListType,
    BasicType,
    SyntaxParameter,
    BASIC_TYPE_NAMES,
    validate_source_path,
)
from ..defs.script import StagingScript, PreparedScript, HandlerStore, TypeHandlerStore
from ..util.message import i18n as _
from ..util.message import UserMessage
from ..util.result import Result, ResultGen, Problem


def generate_prepared_script(
    staging: StagingScript,
    max_meta_count: int,
) -> Result[PreparedScript]:
    """Construct the syntax tree from the root parsed node."""
    return (
        HandlerStore.create(
            staging.script_source.source,
            staging.add_ins,
        )
        .map_result(
            lambda handlers: expand_meta_types(
                root=staging.tree,
                handlers=handlers,
                max_meta_count=max_meta_count,
            )
            .map_to(
                lambda node: assign_types(
                    root=node,
                    handlers=handlers.as_type_handler_store(),
                )
            )
            .map_to(validate_nodes)
            .map_result(collect_errors)
            .map_result(
                lambda typed_tree: finish_tree(
                    typed_tree=typed_tree,
                    handlers=handlers.as_type_handler_store(),
                )
            )
        )
        .map_to(
            lambda res: PreparedScript(
                source=staging.script_source,
                name=staging.name,
                version=staging.version,
                type_handlers=res[1],
                tree=res[0],
            )
        )
    )


class TypedTree:
    """A parsed node tree with types collected."""

    __slots__ = (
        "referenced_handlers",
        "handlers",
        "root",
    )

    def __init__(self, root: AbcParsedNode, handlers: TypeHandlerStore) -> None:
        self.root = root
        self.handlers = handlers
        self.referenced_handlers: Set[str] = set()

    def mark_referenced(
        self,
        value: AddInTypeHandler | TypeParameter,
    ) -> None:
        """Mark the type as referenced."""
        if isinstance(value, TypeParameter):
            param_type = value.type()
            if isinstance(param_type, AbcType):
                self.referenced_handlers.add(param_type.type_id())
        else:
            self.referenced_handlers.add(value.type().type_id())

    def get_handler(
        self,
        type_val: BasicType | ListType | AbcType | AbcTypeProperty | None,
    ) -> Optional[AddInTypeHandler]:
        """Get the type handler for the type."""
        if not type_val or isinstance(type_val, str):
            return None
        if isinstance(type_val, AbcTypeProperty):
            type_val_type = type_val.type()
            if isinstance(type_val_type, AbcType):
                return self.handlers.get(type_val_type)
            return None
        return self.handlers.get(type_val)


def finish_tree(
    typed_tree: TypedTree,
    handlers: TypeHandlerStore,
) -> Result[Tuple[SyntaxNode, TypeHandlerStore]]:
    """Finish the conversion from a no-problem tree to a syntax tree."""
    res = ResultGen()
    gen: Dict[str, SyntaxNode] = {}

    def visitor(node: AbcParsedNode) -> None:
        # When this is called, all the children should already be created.
        if isinstance(node, ParsedSimpleNode):
            # These are directly added to the generated node, not cached.
            return
        base_node_type = node.get_assigned_type()
        if not base_node_type or node.is_not_valid():
            # Don't handle this node.  The errors should have already errored out,
            # though.
            res.force_not_valid()
            return
        if base_node_type in BASIC_TYPE_NAMES:
            # This is a basic assertion for code correctness.
            # This type should only be possible for simple nodes, which
            # have already been omitted.
            raise RuntimeError(
                f"node {node!r} is marked as a complex parsed node "
                f"({type(base_node_type)}), but has simple assigned type ({base_node_type})"
            )
        n_type = cast(AbcType | ListType, base_node_type)

        children: Dict[str, SyntaxParameter] = {}
        valid = True

        for key, child in node.mapping().items():
            if isinstance(child, ParsedSimpleNode):
                # Just add the simple value directly.
                children[str(key)] = child.value
                continue

            child_gen = gen.get(child.node_id.node_ptr)
            if not child_gen:
                res.add(
                    Problem.as_validation(
                        node.node_id.source,
                        UserMessage(
                            _("Did not generate child {key} of {ptr}"),
                            key=key,
                            ptr=node.node_id.node_ptr,
                        ),
                    )
                )
                valid = False
            else:
                children[str(key)] = child_gen
        if valid:
            gen[node.node_id.node_ptr] = SyntaxNode(
                source=node.node_id.source,
                node_id=node.node_id.ref,
                node_type=n_type,
                values=children,
            )

    post_visit_parsed_node(typed_tree.root, visitor)
    # This *shouldn't* cause an exception, if everything is programmed right.
    return res.build_with(
        lambda: (
            gen[typed_tree.root.node_id.node_ptr],
            handlers.include_only(typed_tree.referenced_handlers),
        )
    )


def collect_errors(tree: TypedTree) -> Result[TypedTree]:
    """Collect all the errors in the tree and return it as a result.

    The result is valid if and only if the node tree has no problems.
    """
    res = ResultGen()

    def visitor(node: AbcParsedNode) -> None:
        res.add(validate_source_path(node.node_id.ref))
        res.add(node.problems())

    post_visit_parsed_node(tree.root, visitor)
    return res.build(tree)


def validate_nodes(
    tree: TypedTree,
) -> TypedTree:
    """Check each node for whether it's valid against the type handler."""

    def visitor(node: AbcParsedNode) -> None:  # pylint:disable=too-many-branches
        # Only examine still valid nodes.
        if node.is_not_valid():
            return
        node_type = node.get_assigned_type()
        if node_type is None:
            # It's valid but no type.
            node.add_problem(
                Problem.as_validation(
                    node.node_id.source,
                    UserMessage(_("No type discovered for {type}"), type=node.type_id),
                )
            )
            return
        parent = node.get_parent()
        if parent:
            par_param_type = parent.get_parameter_type()
            if par_param_type and par_param_type.type() != node_type:
                node.add_problem(
                    Problem.as_validation(
                        node.node_id.source,
                        UserMessage(
                            _(
                                "parent parameter '{key}' requires type "
                                "'{param_type}', but node has type '{node_type}'."
                            ),
                            key=parent.key,
                            node_type=node.type_id,
                            param_type=repr(par_param_type.type()),
                        ),
                    )
                )

        if isinstance(node, ParsedListNode):
            # Ensure for lists, that the items are of the same type.
            item_type = node.get_item_type()
            if not item_type or not item_type.is_list():
                node.add_problem(
                    Problem.as_validation(
                        node.node_id.source,
                        UserMessage(
                            _("Node's parent isn't valid or did not mark this parameter as a list")
                        ),
                    )
                )
            else:
                required_type = item_type.type()
                for param_key, child in node.mapping().items():
                    if required_type != child.get_assigned_type():
                        node.add_problem(
                            Problem.as_validation(
                                node.node_id.source,
                                UserMessage(
                                    _(
                                        "expected children of type {req}, but '{key}' "
                                        "has type {child_type}"
                                    ),
                                    req=repr(required_type),
                                    key=param_key,
                                    child_type=repr(child.get_assigned_type()),
                                ),
                            )
                        )
        elif isinstance(node, ParsedParameterNode):
            # Ensure for parameters, each parameter is right and the required ones are present.
            if parent:
                par_param_type = parent.get_parameter_type()
                if par_param_type and par_param_type.is_list():
                    node.add_problem(
                        Problem.as_validation(
                            node.node_id.source,
                            UserMessage(_("Node's parent marked this node as list")),
                        )
                    )
            if not isinstance(node_type, AbcType):
                # Invalid state.
                raise RuntimeError("ParsedParameterNode has non AbcType assigned type")

            remaining_children = dict(node.mapping())
            for param in node_type.parameters():
                val = remaining_children.get(param.key())
                if val and val.get_assigned_type() != param.type():
                    val.add_problem(
                        Problem.as_validation(
                            val.node_id.source,
                            UserMessage(
                                _("Parent requires '{key}' of type '{req}', but found '{typ}'"),
                                key=param.key(),
                                req=repr(param.type()),
                                typ=val.type_id,
                            ),
                        )
                    )
                if not val and param.is_required():
                    node.add_problem(
                        Problem.as_validation(
                            node.node_id.source,
                            UserMessage(
                                _("Node requires '{key}' parameter, but it is missing"),
                                key=param.key(),
                            ),
                        )
                    )
            if remaining_children:
                node.add_problem(
                    Problem.as_validation(
                        node.node_id.source,
                        UserMessage(
                            _("Node contains unknown keys {keys}"),
                            keys=repr(remaining_children.keys()),
                        ),
                    )
                )
        elif isinstance(node, ParsedSimpleNode):
            # Ensure the value type matches the type name.
            if parent:
                par_param_type = parent.get_parameter_type()
                if par_param_type and par_param_type.is_list():
                    node.add_problem(
                        Problem.as_validation(
                            node.node_id.source,
                            UserMessage(_("Node's parent marked this node as list")),
                        )
                    )

    pre_visit_parsed_node(tree.root, visitor)
    return tree


def assign_types(
    root: AbcParsedNode,
    handlers: TypeHandlerStore,
) -> TypedTree:
    """Ensures that each valid node has an assigned type."""
    ret = TypedTree(root, handlers)

    # Because this looks at parent types, the parent must
    #   be handled first.
    def visitor(node: AbcParsedNode) -> None:  # pylint:disable=too-many-branches
        # Only care about valid nodes.
        if node.is_not_valid():
            return

        if isinstance(node, ParsedParameterNode):
            # Only the parameter node type has an assignable type.
            type_handler = handlers.get(node.type_id)
            if type_handler is None:
                node.add_problem(
                    Problem.as_validation(
                        node.node_id.source,
                        UserMessage(
                            _("unknown type '{type_id}'"),
                            type_id=node.type_id,
                        ),
                    )
                )
                # Invalid, so quit immediately
                return
            ret.mark_referenced(type_handler)
            node.set_type(type_handler.type())

        parent = node.get_parent()
        if parent:
            if parent.node.is_not_valid():
                # Do not parse child if the parent is bad.
                return
            if isinstance(parent.node, ParsedListNode):
                # The parent type is declared directly in the list.
                item_type = parent.node.get_item_type()
                if item_type is None:
                    # Parent should be fully parsed.  This is a bug.
                    raise RuntimeError(f"did not set list type for parent {parent!r}")
                parent.set_parameter_type(item_type)
            elif isinstance(parent.node, ParsedParameterNode):
                # Find the key in the parent type handler.
                parent_handler = handlers.get(parent.node.type_id)
                if parent_handler:
                    param_type: Optional[TypeParameter] = None
                    for kpt in parent_handler.type().parameters():
                        if kpt.key() == parent.key:
                            param_type = kpt
                            break
                    if param_type is None:
                        # Invalid key!
                        node.add_problem(
                            Problem.as_validation(
                                node.node_id.source,
                                UserMessage(
                                    _(
                                        "node {node} defined as child of {parent} "
                                        "at undefined key {key}"
                                    ),
                                    node=repr(node),
                                    parent=repr(parent.node),
                                    key=parent.key,
                                ),
                            )
                        )
                        # Invalid, so quit immediately
                        return
                    if isinstance(node, ParsedListNode):
                        # Do not check whether the type is a list or not.
                        node.set_item_type(param_type)
                    ret.mark_referenced(param_type)
                    parent.set_parameter_type(param_type)
                else:
                    # The handler isn't found, then the parent *should* be bad.
                    # But, we already checked if it's bad, so mark as a fail-safe.
                    node.add_problem(
                        Problem.as_validation(
                            node.node_id.source,
                            UserMessage(
                                _("child {node} has parent {parent} with unknown type {type}"),
                                node=repr(node),
                                parent=repr(parent.node),
                                type=parent.node.type_id,
                            ),
                        )
                    )
            else:
                # It's a simple type, which is a parent?  That doesn't
                # make sense.
                raise RuntimeError(f"non-container node ({parent.node}) has child ({node})")

    pre_visit_parsed_node(root, visitor)
    return ret


def expand_meta_types(
    *,
    root: AbcParsedNode,
    handlers: HandlerStore,
    max_meta_count: int,
) -> Result[AbcParsedNode]:
    """Expand the root node until there are no more meta type nodes.

    It will expand at most max_meta_count times across the entire tree.
    """
    found_meta = [False]
    res = ResultGen()

    def visitor(node: AbcParsedNode) -> None:
        if node.is_not_valid():
            # Skip nodes with problems.
            return
        # Though this could assign type handlers, that is delayed until
        #   after meta type expansion.
        type_id = node.type_id
        meta_handler = handlers.get_meta_handler(type_id)
        if meta_handler:
            parent = node.get_parent()
            if parent is None:
                # Can't have a meta at the root node.  It doesn't make sense.
                node.add_problem(
                    Problem.as_validation(
                        node.node_id.source,
                        UserMessage(
                            _("Root node cannot be of meta-type ({type})"),
                            type=type_id,
                        ),
                    )
                )
                # It's now in an error state.
                return

            # Perform one meta-type expansion
            child_res = meta_handler.translate(node)
            if child_res.is_not_valid:
                node.add_problem(child_res)
                # Do not mark as found.
                return
            found_meta[0] = True
            parent.node.replace_value(
                parent.key,
                child_res.required(),
            )

    # Perform at most max_meta_count expansions of
    #   meta-types.  This allows meta-types to generate trees that
    #   themselves include meta-types.  However, that can lead to
    #   a potential infinite replacement loop, so don't let that
    #   happen by capping the rerun count.
    for _ in range(0, max_meta_count):
        found_meta[0] = False
        post_visit_parsed_node(root, visitor)
        if not found_meta[0]:
            # All clean; as far as embedded meta-types goes.
            return res.build(root)
    res.add(
        Problem.as_validation(
            root.node_id.source,
            UserMessage(
                _(
                    "Meta-Type generators in tree created too many meta-type "
                    "generators.  Quit after {count} attempts"
                ),
                count=max_meta_count,
            ),
        )
    )
    return Result.as_error(res.problems)
