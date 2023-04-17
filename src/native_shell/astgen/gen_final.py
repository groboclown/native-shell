"""Convert the ParsedNode into the AbcSyntaxNode."""

from typing import Tuple, Dict
from .node_visit import post_visit_parsed_node, pre_visit_parsed_node
from .typed_tree import TypedTree
from .gen_root_type import assign_root_node_type
from .node_validator import validate_node
from .assign_type import assign_types_to_node
from ..defs.basic import mk_ref
from ..defs.parse_tree import (
    AbcParsedNode,
    ParsedSimpleNode,
)
from ..defs.node_type import ConstructType, BasicType
from ..defs.syntax_tree import SyntaxNode, SyntaxParameter
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
                bin_location=staging.bin_location,
                type_handlers=res[1],
                tree=res[0],
            )
        )
    )


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
        node_type = node.get_assigned_type()
        if not node_type or node.is_not_valid():
            # Don't handle this node.  The errors should have already errored out,
            # though.
            res.force_not_valid()
            return
        if isinstance(node_type, BasicType):
            # This is a basic assertion for code correctness.
            # This type should only be possible for simple nodes, which
            # have already been omitted.
            raise RuntimeError(
                f"node {node!r} is marked as a complex parsed node "
                f"({type(node_type)}), but has simple assigned type ({node_type})"
            )

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

        if isinstance(node_type, ConstructType):
            # Fields will become their own synthetic syntax nodes.
            # This allows their generated code to be referencable.
            for field in node_type.fields():
                field_type = field.type()
                typed_tree.mark_referenced(field_type)
                children[field.key()] = SyntaxNode(
                    source=[*node.node_id.source, field.key()],
                    node_id=mk_ref([*node.node_id.ref, field.key()]),
                    node_type=field_type,
                    # This is something we may need to return to.
                    # Right now, the field types are strictly for
                    # value fetching and assignment.  It does not have
                    # parameters, but it might have its own fields.  If a field has
                    # sub-fields, then right now it's up to the caller to
                    # reference those.
                    values={},
                )

        if valid:
            gen[node.node_id.node_ptr] = SyntaxNode(
                source=node.node_id.source,
                node_id=node.node_id.ref,
                node_type=node_type,
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
        # Node ids are not source paths.
        # res.add(validate_source_path(node.node_id.ref))
        res.add(node.problems())

    post_visit_parsed_node(tree.root, visitor)
    return res.build(tree)


def validate_nodes(
    tree: TypedTree,
) -> TypedTree:
    """Check each node for whether it's valid against the type handler."""

    def visitor(node: AbcParsedNode) -> None:
        # Only examine still valid nodes.
        if node.is_not_valid():
            return

        validate_node(node)

    pre_visit_parsed_node(tree.root, visitor)
    return tree


def assign_types(  # pylint:disable=too-many-return-statements,R0915
    root: AbcParsedNode,
    handlers: TypeHandlerStore,
) -> TypedTree:
    """Ensures that each valid node has an assigned type."""
    ret = TypedTree(root, handlers)

    # Because this looks at parent types, the parent must
    #   be handled first.
    pre_visit_parsed_node(
        root,
        lambda node: assign_types_to_node(node, ret),
    )

    # Now the root node type needs to be generated dynamically.
    assign_root_node_type(ret)

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
    for _idx in range(0, max_meta_count):
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
