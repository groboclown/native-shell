"""The final pass to convert the builder into the syntax node."""

from typing import Sequence, Mapping, Dict, List, Callable
from .node_visit import visit_building_node
from ..defs.syntax_tree import (
    AbcType,
    AbcSyntaxBuildingNode,
    AbcSyntaxNode,
)
from ..util.message import i18n as _
from ..util.message import UserMessage
from ..util.result import Result, Problem, ResultGen, SourcePath


def finalize_tree(
    root: AbcSyntaxBuildingNode,
    is_error_callback: Callable[[Problem], bool],
) -> Result[AbcSyntaxNode]:
    """Constructs the final syntax node.

    If any node has problems, or if any node has a meta-type, then
    this generates an error.
    """
    res = ResultGen()
    errors: List[Problem] = []
    generated: Dict[str, AbcSyntaxNode] = {}

    def visit(node: AbcSyntaxBuildingNode) -> None:
        problems = node.problems()
        res.add(problems)
        is_ok = True
        for problem in problems:
            if is_error_callback(problem):
                # Do not add and mark as an error
                errors.append(problem)
                is_ok = False
        b_type = node.build_type()
        c_type: AbcType
        if isinstance(b_type, AbcType):
            c_type = b_type
        else:
            res.add(
                Problem.as_validation(
                    node.source(),
                    UserMessage(
                        _("Did not clear out all meta-types ({node_id})"),
                        node_id=node.node_id(),
                    ),
                ),
            )
            is_ok = False
        if is_ok:
            params: Dict[str, AbcSyntaxNode] = {}
            for key, param in node.parameter_values().items():
                lookup = get_node_lookup_key(param)
                if lookup not in generated:
                    is_ok = False
                    break
                params[key] = generated[lookup]
            if is_ok:
                generated[get_node_lookup_key(node)] = FinalSyntaxNode(
                    source=node.source(),
                    node_id=node.node_id(),
                    c_type=c_type,
                    params=params,
                )

    visit_building_node(root, visit)
    return res.build_with(lambda: generated[get_node_lookup_key(root)])


def get_node_lookup_key(node: AbcSyntaxBuildingNode) -> str:
    """Turn the node into a unique lookup key."""
    return ";".join(node.node_id())


class FinalSyntaxNode(AbcSyntaxNode):
    """The final pass generated node."""

    def __init__(
        self,
        /,
        source: SourcePath,
        node_id: Sequence[str],
        c_type: AbcType,
        params: Mapping[str, AbcSyntaxNode],
    ) -> None:
        self._source = source
        self._node_id = node_id
        self._type = c_type
        self._params = params

    def source(self) -> SourcePath:
        return self._source

    def node_id(self) -> Sequence[str]:
        return self._node_id

    def type(self) -> AbcType:
        return self._type

    def parameter_values(self) -> Mapping[str, AbcSyntaxNode]:
        return self._params
