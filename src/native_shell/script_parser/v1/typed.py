"""A very, very trivial script file."""

from typing import Dict, Optional, Any, cast
from .basic import parse_basic_type
from ...defs.basic import mk_ref
from ...defs.parse_tree import (
    AbcParsedNode,
    ParsedNodeId,
    ParsedListNode,
    ParsedSimpleNode,
    ParsedParameterNode,
)
from ...defs.node_type import BASIC_TYPES, BasicType, BasicTypeId
from ...util.message import i18n as _
from ...util.result import Problem, ResultGen


def parse_typed_node(  # pylint:disable=too-many-branches
    *,
    parent: ParsedNodeId,
    node_key: str,
    data: Dict[str, Any],
    res: ResultGen,
) -> Optional[AbcParsedNode]:
    """For now, do nothing."""
    if not data:
        return None

    is_list: bool
    as_list_type = data.get("as-list")
    as_type = data.get("as")
    if (as_type and as_list_type) or (not as_type and not as_list_type):
        res.add(
            Problem.as_validation(
                (*parent.source, node_key),
                _("exactly one of 'as' and 'as-list' can be present"),
            )
        )

    name = "as"
    is_list = False
    if as_list_type:
        as_type = as_list_type
        name = "as-list"
        is_list = True
    if not isinstance(as_type, str):
        res.add(
            Problem.as_validation(
                (*parent.source, node_key),
                _("'{key}' key must be present and name a type"),
                key=name,
            )
        )
        return None
    del data[name]

    if as_type in BASIC_TYPES:
        # if as_type is a key in BASIC_TYPES, then it's a BasicTypeId, because that's
        #   the only allowed keys.
        return parse_basic_node(
            parent=parent,
            node_key=node_key,
            data=data,
            res=res,
            is_list=is_list,
            as_type=BASIC_TYPES[cast(BasicTypeId, as_type)],
        )

    ret: Optional[AbcParsedNode]
    if is_list:
        value_list = data.get("items")
        if value_list is None:
            res.add(
                Problem.as_validation(
                    (*parent.source, node_key),
                    _("basic type {typ} must have a 'value'"),
                    typ=as_type,
                )
            )
            return None
        del data["items"]
        if not isinstance(value_list, (tuple, list)):
            res.add(
                Problem.as_validation(
                    (*parent.source, node_key, "list"),
                    _("type with an 'items' must have it be a list"),
                )
            )
            return None
        pln = ParsedListNode(
            node_id=ParsedNodeId(
                source=(*parent.source, node_key),
                ref=mk_ref((*parent.ref, node_key)),
            ),
        )
        ret = pln
        for item in value_list:
            if not isinstance(item, dict):
                res.add(
                    Problem.as_validation(
                        (*parent.source, node_key, "items"),
                        _("type with an 'items' must have each one be a dict"),
                    )
                )
                continue
            # This implies that parameters cannot be lists of lists, which is fine.
            node = parse_parameter_node(
                parent=parent,
                node_key=node_key,
                data=item,
                res=res,
                as_type=as_type,
            )
            if node:
                pln.add_value(node)
    else:
        ret = parse_parameter_node(
            parent=parent,
            node_key=node_key,
            data=data,
            res=res,
            as_type=as_type,
        )

    if data:
        # Not a stop-right-now error
        res.add(
            Problem.as_validation(
                (*parent.source, node_key),
                _("type supports only 'items' or 'with'; found additional: {keys}"),
                keys=repr(tuple(data.keys())),
            )
        )
    return ret


def parse_parameter_node(
    *,
    parent: ParsedNodeId,
    node_key: str,
    data: Dict[str, Any],
    res: ResultGen,
    as_type: str,
) -> Optional[AbcParsedNode]:
    """Parse the data into a parameter node."""
    with_val = data.get("with")
    if with_val is None or not isinstance(with_val, dict):
        res.add(
            Problem.as_validation(
                (*parent.source, node_key),
                _("must have a 'with' setting"),
            )
        )
        return None
    del data["with"]

    ret = ParsedParameterNode(
        node_id=ParsedNodeId(
            source=(*parent.source, node_key),
            ref=mk_ref((*parent.ref, node_key)),
        ),
        type_id=as_type,
    )
    for key, val in with_val.items():
        if not isinstance(val, dict):
            res.add(
                Problem.as_validation(
                    (*parent.source, node_key),
                    _("has '{key}' but it is not a dict"),
                    key=key,
                )
            )
            continue
        parsed = parse_typed_node(
            parent=ret.node_id,
            node_key=key,
            data=val,
            res=res,
        )
        if parsed:
            ret.set_parameter(key, parsed)

    if data:
        # Not a stop-right-now error
        res.add(
            Problem.as_validation(
                (*parent.source, node_key),
                _("type supports only 'as' and 'with'; found additional: {keys}"),
                keys=repr(tuple(data.keys())),
            )
        )
    return ret


def parse_basic_node(
    *,
    parent: ParsedNodeId,
    node_key: str,
    data: Dict[str, Any],
    res: ResultGen,
    is_list: bool,
    as_type: BasicType,
) -> Optional[AbcParsedNode]:
    """Parse a basic node."""
    ret: Optional[AbcParsedNode]

    if is_list:
        value_list = data.get("items")
        if value_list is None:
            res.add(
                Problem.as_validation(
                    (*parent.source, node_key),
                    _("basic type {typ} must have a 'value'"),
                    typ=repr(as_type),
                )
            )
            return None
        del data["items"]
        if not isinstance(value_list, (tuple, list)):
            res.add(
                Problem.as_validation(
                    (*parent.source, node_key, "items"),
                    _("basic type with an 'items' must have it be a list"),
                )
            )
            return None
        pln = ParsedListNode(
            node_id=ParsedNodeId(
                source=(*parent.source, node_key),
                ref=mk_ref((*parent.ref, node_key)),
            ),
        )
        ret = pln
        for item in value_list:
            simple = parse_basic_type(parent, node_key, as_type, item)
            res.add(simple)
            pln.add_value(
                ParsedSimpleNode(
                    node_id=ParsedNodeId(
                        source=(*parent.source, node_key),
                        ref=mk_ref((*parent.ref, node_key)),
                    ),
                    type_val=as_type,
                    value=simple.optional() or "",
                )
            )
    else:
        value = data.get("value")
        if value is None:
            res.add(
                Problem.as_validation(
                    (*parent.source, node_key),
                    _("basic type {typ} must have a 'value'"),
                    typ=repr(as_type),
                )
            )
            return None
        del data["value"]

        # we can cast here because of the "as_type in BASIC_TYPE_NAMES" check.
        simple = parse_basic_type(parent, node_key, as_type, value)
        res.add(simple)
        ret = ParsedSimpleNode(
            node_id=ParsedNodeId(
                source=(*parent.source, node_key),
                ref=mk_ref((*parent.ref, node_key)),
            ),
            type_val=as_type,
            value=simple.optional() or "",
        )

    if data:
        # Not a stop-right-now error
        res.add(
            Problem.as_validation(
                (*parent.source, node_key),
                _("basic type supports only 'list' or 'value'; found additional: {keys}"),
                keys=repr(tuple(data.keys())),
            )
        )
    return ret
