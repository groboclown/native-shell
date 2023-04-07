"""Check if the builder has any meta-type nodes left."""

from typing import Iterable, List
from .node_visit import visit_building_node
from ..defs.syntax_tree import AbcSyntaxBuildingNode, AbcMetaType


def get_meta_nodes(
    root: AbcSyntaxBuildingNode,
) -> Iterable[AbcSyntaxBuildingNode]:
    """Get the building nodes that are meta-type nodes."""
    ret: List[AbcSyntaxBuildingNode] = []

    def visitor(node: AbcSyntaxBuildingNode) -> None:
        build_type = node.build_type()
        if isinstance(build_type, AbcMetaType):
            ret.append(node)

    visit_building_node(root, visitor)
    return ret
