"""A very, very trivial script file."""

from typing import Dict, Any
from .typed import parse_typed_node
from ...defs.basic import mk_ref
from ...defs.parse_tree import ParsedNodeId, ParsedParameterNode
from ...util.message import i18n as _
from ...util.result import Problem, ResultGen, SourcePath


def parse_root_node(src: SourcePath, data: Dict[str, Any], res: ResultGen) -> ParsedParameterNode:
    """Parse the root node.

    Everything in this node is a command.
    """
    ret = ParsedParameterNode(
        node_id=ParsedNodeId(source=src, ref=mk_ref(())),
        type_id="",
    )
    for key, val in data.items():
        if not isinstance(val, dict):
            res.add(
                Problem.as_validation(
                    (*src, key),
                    _("root node must contain only dictionaries."),
                )
            )
        else:
            node = parse_typed_node(
                parent=ret.node_id,
                node_key=key,
                data=val,
                res=res,
            )
            if node:
                ret.set_parameter(key, node)
    return ret
