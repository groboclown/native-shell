"""Builder nodes for replacing themselves in an immutable way."""

from typing import Mapping, Dict, Sequence
from .node_visit import visit_building_node
from ..defs.syntax_tree import (
    AbcSyntaxBuildingNode,
    AbcType,
    AbcMetaType,
)
from ..util.result import SourcePath, Problem


def create_building_node_lookup(
    root: AbcSyntaxBuildingNode,
) -> Mapping[str, AbcSyntaxBuildingNode]:
    """Create a lookup for nodes."""
    ret: Dict[str, AbcSyntaxBuildingNode] = {}

    def visitor(node: AbcSyntaxBuildingNode) -> None:
        ret[get_node_lookup_key(node)] = node

    visit_building_node(root, visitor)
    return ret


def replace_building_nodes(
    previous_root: AbcSyntaxBuildingNode,
    replacements: Mapping[str, AbcSyntaxBuildingNode],
    extra_problems: Mapping[str, Sequence[Problem]],
) -> AbcSyntaxBuildingNode:
    """Construct a new building node tree, replacing previous node IDs
    with new ones.  And add in extra problems to nodes."""
    replaced = dict(replacements)

    def visit(node: AbcSyntaxBuildingNode) -> None:
        # Called post, which means all the children
        #   will have already been called.
        key = get_node_lookup_key(node)
        if key not in replaced:
            # This node must be replaced using the new children.
            repl = PrepSyntaxBuildingNode(
                previous=node,
                replacements=replaced,
                extra_problems=extra_problems,
            )
            # Now add in this replaced value as the new
            replaced[key] = repl
        # else, it's already been replaced with something else,
        # so use that, even if it means throwing away all the work
        # done so far.

    visit_building_node(previous_root, visit)
    return replaced[get_node_lookup_key(previous_root)]


def get_node_lookup_key(node: AbcSyntaxBuildingNode) -> str:
    """Turn the node into a unique lookup key."""
    return ";".join(node.node_id())


class PrepSyntaxBuildingNode(AbcSyntaxBuildingNode):
    """A prep version of the building node."""

    def __init__(
        self,
        /,
        previous: AbcSyntaxBuildingNode,
        replacements: Mapping[str, AbcSyntaxBuildingNode],
        extra_problems: Mapping[str, Sequence[Problem]],
    ) -> None:
        self._source = previous.source()
        self._node_id = previous.node_id()
        self._type = previous.build_type()
        self._problems: Sequence[Problem] = tuple(
            (
                *previous.problems(),
                *extra_problems.get(get_node_lookup_key(previous), ()),
            )
        )
        self._params = {
            key: replacements.get(get_node_lookup_key(prev), prev)
            for key, prev in previous.parameter_values().items()
        }

    def source(self) -> SourcePath:
        return self._source

    def node_id(self) -> Sequence[str]:
        return self._node_id

    def build_type(self) -> AbcType | AbcMetaType:
        """Get the node's type."""
        return self._type

    def problems(self) -> Sequence[Problem]:
        """Get any problems associated with this node.

        This does not include parameter value problems.
        """
        return self._problems

    def parameter_values(self) -> "Mapping[str, AbcSyntaxBuildingNode]":
        """Get the values for the parameters."""
        return self._params
