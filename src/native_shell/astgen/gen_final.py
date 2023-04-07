"""Convert the ParsedNode into the AbcSyntaxNode."""

from typing import Set, Tuple, Dict, Union
from .node_visit import visit_parsed_node
from ..defs.parse_tree import ParsedNode
from ..defs.syntax_tree import SyntaxNode, SimpleParameter
from ..defs.script import StagingScript, PreparedScript, HandlerStore, TypeHandlerStore
from ..util.message import i18n as _
from ..util.message import UserMessage
from ..util.result import Result, ResultGen, Problem


# TODO this still needs to perform node validation based on the type.


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
            .map_result(collect_errors)
            .map_result(
                lambda node: finish_tree(
                    root=node,
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


def finish_tree(
    root: ParsedNode,
    handlers: TypeHandlerStore,
) -> Result[Tuple[SyntaxNode, TypeHandlerStore]]:
    """Finish the conversion from a no-problem tree to a syntax tree."""
    res = ResultGen()
    referenced_types: Set[str] = set()
    gen: Dict[str, SyntaxNode] = {}

    def visitor(node: ParsedNode) -> None:
        # When this is called, all the children should already be created.
        children: Dict[str, Union[SyntaxNode, SimpleParameter]] = {}
        n_type = node.assigned_type()
        if not n_type or node.is_not_valid():
            # Don't handle this node.  The errors should have already errored out,
            # though.
            res.force_not_valid()
            return
        referenced_types.add(n_type.type_id())
        valid = True
        for key, child in node.parameter_map().items():
            if isinstance(child, ParsedNode):
                child_gen = gen.get(child.node_ptr())
                if not child_gen:
                    res.add(
                        Problem.as_validation(
                            node.source(),
                            UserMessage(
                                _("Did not generate child {key} of {ptr}"),
                                key=key,
                                ptr=node.node_ptr(),
                            ),
                        )
                    )
                    valid = False
                else:
                    children[key] = child_gen
            else:
                # It's a simple type, directly add it.
                children[key] = child
        if valid:
            gen[node.node_ptr()] = SyntaxNode(
                source=node.source(),
                node_id=node.node_id(),
                node_type=n_type,
                values=children,
            )

    visit_parsed_node(root, visitor)
    # This *shouldn't* cause an exception, if everything is programmed right.
    return res.build_with(
        lambda: (
            gen[root.node_ptr()],
            handlers.include_only(referenced_types),
        )
    )


def collect_errors(root: ParsedNode) -> Result[ParsedNode]:
    """Collect all the errors in the tree and return it as a result.

    If the result is valid, then the node tree has no problems.
    """
    res = ResultGen()

    def visitor(node: ParsedNode) -> None:
        res.add(node.problems())

    visit_parsed_node(root, visitor)
    return res.build(root)


def expand_meta_types(
    *,
    root: ParsedNode,
    handlers: HandlerStore,
    max_meta_count: int,
) -> Result[ParsedNode]:
    """Expand the root node until there are no more meta type nodes.

    It will expand at most max_meta_count times across the entire tree.
    """
    found_meta = [False]
    res = ResultGen()

    def visitor(node: ParsedNode) -> None:
        # Skip nodes with problems.
        if node.is_valid():
            type_id = node.type_id()
            meta_handler = handlers.get_meta_handler(type_id)
            type_handler = handlers.get_type_handler(type_id)
            if meta_handler:
                # Can't have a meta at the root node.  It doesn't make sense.
                parent = node.parent_node()
                parent_key = node.parent_key()
                if parent is None or parent_key is None:
                    node.add_problem(
                        Problem.as_validation(
                            node.source(),
                            UserMessage(_("Root node cannot be of a meta-type")),
                        )
                    )
                    return

                # One pass at expansion
                child_res = meta_handler.translate(node)
                if child_res.is_not_valid:
                    node.add_problem(child_res)
                    # Do not mark as found.
                    return
                found_meta[0] = True
                parent.replace_parameter(parent_key, child_res.required())
            elif type_handler is not None and node.assigned_type() is None:
                node.set_type(type_handler.type())
            else:
                node.add_problem(
                    Problem.as_validation(
                        node.source(),
                        UserMessage(
                            _("Node has unknown type id '{type_id}'"),
                            type_id=type_id,
                        ),
                    )
                )

    for _ in range(0, max_meta_count):
        found_meta[0] = False
        visit_parsed_node(root, visitor)
        if not found_meta[0]:
            # All clean; as far as embedded meta-types goes.
            return res.build(root)
    res.add(
        Problem.as_validation(
            root.source(),
            UserMessage(
                _(
                    "Meta-Type generators in tree created too many meta-type "
                    "generators.  Quit after {n} attempts"
                ),
                n=max_meta_count,
            ),
        )
    )
    return Result.as_error(res.problems)
