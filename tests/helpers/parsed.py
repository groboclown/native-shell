"""Helpers for creating parsed nodes."""

from typing import List
from native_shell.defs import build_ref
from native_shell.defs.basic import SimpleParameter
from native_shell.defs.parse_tree import (
    ParsedNodeId,
    AbcParsedNode,
    ParsedSimpleNode,
    ParsedListNode,
    ParsedParameterNode,
)


def mk_simple(path: List[str], value: SimpleParameter) -> ParsedSimpleNode:
    """Create a simple node"""
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
        node_id=mk_node_id(path[1:]),
        type_id=type_id,
        value=value,
    )


def mk_parameter(
    __path: List[str],
    __type_id: str,
    **__params: AbcParsedNode,
) -> ParsedParameterNode:
    """Create a parameter node"""
    ret = ParsedParameterNode(
        node_id=mk_node_id(__path[1:]),
        type_id=__type_id,
    )
    for key, param in __params.items():
        ret.set_parameter(key, param)
    return ret


def mk_list(
    __path: List[str],
    *__items: AbcParsedNode,
) -> ParsedListNode:
    """Create a list node"""
    ret = ParsedListNode(node_id=mk_node_id(__path[1:]))
    for item in __items:
        ret.add_value(item)
    return ret


def mk_node_id(path: List[str]) -> ParsedNodeId:
    """Create a node id"""
    return ParsedNodeId(
        source=("/", *path),
        ref=build_ref(path),
    )
