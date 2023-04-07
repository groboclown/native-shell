"""Test the node visit module."""

from typing import List
import unittest
from native_shell.astgen import node_visit
from native_shell.defs import build_ref
from native_shell.defs.parse_tree import ParsedNode


class NodeVisitTest(unittest.TestCase):
    """Test the module functions."""

    def test_visit_building_node__one_node(self) -> None:
        """Test visit_building_node with just one node, no children."""
        root = ParsedNode(source=["."], type_id="t1", node_id=build_ref("."))
        visitor = MockVisitor()
        node_visit.visit_parsed_node(root, visitor)
        self.assertEqual(
            ["."],
            visitor.as_node_ids(),
        )

    def test_visit_building_node__four_nodes(self) -> None:
        """Test visit_building_node with four nodes."""
        root = ParsedNode(source=["."], type_id="t1", node_id=build_ref("."))
        ch1 = ParsedNode(source=[".", "1"], type_id="tt", node_id=build_ref(".", "1"))
        root.set_parameter("1", ch1)
        ch2 = ParsedNode(source=[".", "2"], type_id="tt", node_id=build_ref(".", "2"))
        root.set_parameter("2", ch2)
        ch11 = ParsedNode(source=[".", "1", "1"], type_id="tt", node_id=build_ref(".", "1", "1"))
        ch1.set_parameter("1", ch11)
        ch12 = ParsedNode(source=[".", "1", "2"], type_id="tt", node_id=build_ref(".", "1", "2"))
        ch1.set_parameter("2", ch12)
        visitor = MockVisitor()
        node_visit.visit_parsed_node(root, visitor)
        self.assertEqual(
            ["./2", "./1/2", "./1/1", "./1", "."],
            visitor.as_node_ids(),
        )

    def test_visit_building_node__simple_types(self) -> None:
        """Test visit_building_node with simple type nodes."""
        root = ParsedNode(source=["."], type_id="t1", node_id=build_ref(["."]))
        root.set_parameter(0, "a")
        root.set_parameter(1, "b")
        visitor = MockVisitor()
        node_visit.visit_parsed_node(root, visitor)
        self.assertEqual(
            ["."],
            visitor.as_node_ids(),
        )


class MockVisitor:
    """A visitor that tracks the order of visitation."""

    def __init__(self) -> None:
        self.order: List[ParsedNode] = []

    def __call__(self, node: ParsedNode) -> None:
        self.order.append(node)

    def as_node_ids(self) -> List[str]:
        """Node ids"""
        return [n.node_ptr() for n in self.order]
