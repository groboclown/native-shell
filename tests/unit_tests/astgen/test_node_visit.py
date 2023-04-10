"""Test the node visit module."""

from typing import List
import unittest
from helpers.parsed import (
    mk_list,
    mk_parameter,
    mk_simple,
)
from native_shell.astgen import node_visit
from native_shell.defs.parse_tree import AbcParsedNode


class NodeVisitTest(unittest.TestCase):
    """Test the module functions."""

    def test_visit_building_node__one_node(self) -> None:
        """Test visit_building_node with just one node, no children."""
        root = mk_parameter(["."], "t1")
        visitor = MockVisitor()
        node_visit.post_visit_parsed_node(root, visitor)
        self.assertEqual(
            ["//."],
            visitor.as_node_ids(),
        )

    def test_visit_building_node__addl_nodes(self) -> None:
        """Test visit_building_node with multiple nodes."""
        root = mk_parameter(
            ["."],
            "t1",
            a=mk_parameter(
                [".", "a"],
                "tt",
                a=mk_parameter([".", "a", "a"], "taa"),
            ),
            b=mk_parameter([".", "b"], "tb"),
            c=mk_list(
                [".", "c"],
                mk_simple([".", "c", "0"], "a"),
                mk_simple([".", "c", "1"], 1.1),
            ),
        )
        visitor = MockVisitor()
        node_visit.post_visit_parsed_node(root, visitor)
        self.assertEqual(
            ["//./c/1", "//./c/0", "//./c", "//./b", "//./a/a", "//./a", "//."],
            visitor.as_node_ids(),
        )

    def test_visit_building_node__simple_types(self) -> None:
        """Test visit_building_node with simple type nodes."""
        root = mk_parameter(["."], "r")
        root.set_parameter("a", mk_simple([".", "a"], 1))
        root.set_parameter("b", mk_simple([".", "b"], False))
        visitor = MockVisitor()
        node_visit.post_visit_parsed_node(root, visitor)
        self.assertEqual(
            ["//./b", "//./a", "//."],
            visitor.as_node_ids(),
        )


class MockVisitor:
    """A visitor that tracks the order of visitation."""

    def __init__(self) -> None:
        self.order: List[AbcParsedNode] = []

    def __call__(self, node: AbcParsedNode) -> None:
        self.order.append(node)

    def as_node_ids(self) -> List[str]:
        """Node ids"""
        return [n.node_id.node_ptr for n in self.order]
