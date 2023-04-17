"""A very, very trivial script file."""

from typing import List, Any
from ...defs.basic import mk_ref, SimpleParameter
from ...defs.parse_tree import ParsedNodeId
from ...defs.node_type import BasicType
from ...util.message import i18n as _
from ...util.result import Result, Problem, ResultGen


def parse_basic_type(  # pylint:disable=too-many-return-statements,too-many-branches
    parent: ParsedNodeId,
    key: str,
    as_type: BasicType,
    value: Any,
) -> Result[SimpleParameter]:
    """Parse the value as the given basic type."""
    if as_type.type_id() == "string":
        if not isinstance(value, str):
            return Result.as_error(
                Problem.as_validation(
                    (*parent.source, key),
                    _("node with type {typ} contains a value of type {val}"),
                    typ=repr(as_type),
                    val=str(type(value)),
                )
            )
        return Result.as_value(value)
    if as_type.type_id() == "number":
        if not isinstance(value, (int, float)):
            return Result.as_error(
                Problem.as_validation(
                    (*parent.source, key),
                    _("node with type {typ} contains a value of type {val}"),
                    typ=repr(as_type),
                    val=str(type(value)),
                )
            )
        return Result.as_value(float(value))
    if as_type.type_id() == "integer":
        if not isinstance(value, int):
            return Result.as_error(
                Problem.as_validation(
                    (*parent.source, key),
                    _("node with type {typ} contains a value of type {val}"),
                    typ=repr(as_type),
                    val=str(type(value)),
                )
            )
        return Result.as_value(value)
    if as_type.type_id() == "boolean":
        if not isinstance(value, bool):
            return Result.as_error(
                Problem.as_validation(
                    (*parent.source, key),
                    _("node with type {typ} contains a value of type {val}"),
                    typ=repr(as_type),
                    val=str(type(value)),
                )
            )
        return Result.as_value(value)
    if as_type.type_id() == "reference":
        problems = ResultGen()
        ret: List[str] = []
        if not isinstance(value, (tuple, list)):
            problems.add(
                Problem.as_validation(
                    (*parent.source, key),
                    _("node with type {typ} contains a value of type {val}"),
                    typ=repr(as_type),
                    val=str(type(value)),
                )
            )
        else:
            index = 0
            for item in value:
                if not isinstance(item, str):
                    problems.add(
                        Problem.as_validation(
                            (*parent.source, key, index),
                            _("references must contain strings, found {typ}"),
                            typ=repr(type(item)),
                        )
                    )
                index += 1
        return make_ref_from_path(parent, key, ret, problems)
    return Result.as_error(
        Problem.as_validation(
            (*parent.source, key),
            _("unknown basic type {typ}"),
            typ=repr(as_type),
        )
    )


def make_ref_from_path(
    parent: ParsedNodeId,
    key: str,
    ref: List[str],
    res: ResultGen,
) -> Result[SimpleParameter]:
    """Turns the ref into a node reference, possibly following relative paths."""
    if not ref:
        return res.build(mk_ref(()))

    # Note: does not add the key; that's the key for the reference node, which the
    #   script doesn't mean to be relative to.
    in_rel = True
    ret: List[str] = list(parent.ref)
    for part in ref:
        if part == "^" and in_rel:
            # Go up one parent node.
            if len(ret) <= 0:
                res.add(
                    Problem.as_validation(
                        (*parent.source, key),
                        _("Invalid relative reference {ref}; asked to go outside the script."),
                        ref=repr(ref),
                    )
                )
            else:
                ret.pop()
        elif part == "." and in_rel:
            # Use the current node.  It's more of a marker for going down the
            # parent's tree, not it's parent.
            pass
        else:
            # End of relative pathing.
            in_rel = False
            ret.append(part)
    return res.build(mk_ref(ret))
