"""Test the node visit module."""

from typing import List
import unittest
from native_shell.astgen import node_visit
from native_shell.defs import build_ref
from native_shell.defs.basic import SimpleParameter
from native_shell.defs.parse_tree import (
    AbcParsedNode,
    ParsedSimpleNode,
    ParsedParameterNode,
    ParsedListNode,
    ParsedNodeId,
)


class NodeVisitTest(unittest.TestCase):
    """Test the module functions."""

    def test_visit_building_node__one_node(self) -> None:
        """Test visit_building_node with just one node, no children."""
        root = _mk_parameter(["."], "t1")
        visitor = MockVisitor()
        node_visit.post_visit_parsed_node(root, visitor)
        self.assertEqual(
            ["//."],
            visitor.as_node_ids(),
        )

    def test_visit_building_node__addl_nodes(self) -> None:
        """Test visit_building_node with multiple nodes."""
        root = _mk_parameter(
            ["."],
            "t1",
            a=_mk_parameter(
                [".", "a"],
                "tt",
                a=_mk_parameter([".", "a", "a"], "taa"),
            ),
            b=_mk_parameter([".", "b"], "tb"),
            c=_mk_list(
                [".", "c"],
                _mk_simple([".", "c", "0"], "a"),
                _mk_simple([".", "c", "1"], 1.1),
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
        root = _mk_parameter(["."], "r")
        root.set_parameter("a", _mk_simple([".", "a"], 1))
        root.set_parameter("b", _mk_simple([".", "b"], False))
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


def _mk_simple(path: List[str], value: SimpleParameter) -> ParsedSimpleNode:
    if isinstance(value, str):
        type_id = "string"
    elif isinstance(value, bool):
        type_id = "boolean"
    elif isinstance(value, int):
        type_id = "integer"
    elif isinstance(value, float):
        type_id = "float"
    else:
        type_id = "reference"
    return ParsedSimpleNode(
        node_id=_mk_node_id(path),
        type_id=type_id,
        value=value,
    )


def _mk_parameter(
    __path: List[str],
    __type_id: str,
    **__params: AbcParsedNode,
) -> ParsedParameterNode:
    ret = ParsedParameterNode(
        node_id=_mk_node_id(__path),
        type_id=__type_id,
    )
    for key, param in __params.items():
        ret.set_parameter(key, param)
    return ret


def _mk_list(
    __path: List[str],
    *__items: AbcParsedNode,
) -> ParsedListNode:
    ret = ParsedListNode(node_id=_mk_node_id(__path))
    for item in __items:
        ret.add_value(item)
    return ret


def _mk_node_id(path: List[str]) -> ParsedNodeId:
    return ParsedNodeId(
        source=("/", *path),
        ref=build_ref(path),
    )
