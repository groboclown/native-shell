"""Validates a parsed node for correctness.

Most of these checks end up just ensuring that the script parser
creates the right structure, and that all the finalization parts
up to this point worked right.  That is to say, it's just checking
for bugs in the parser.
"""

from typing import Mapping, Callable
from ..defs.basic import SimpleParameter
from ..defs.parse_tree import (
    AbcParsedNode,
    ParsedListNode,
    ParsedParameterNode,
    ParsedSimpleNode,
)
from ..defs.node_type import (
    AbcType,
    BasicType,
    BasicTypeId,
    ListType,
    ConstructType,
)
from ..util.message import i18n as _
from ..util.result import Problem


def validate_node(node: AbcParsedNode) -> None:
    """Validates the node.  It will update the problems within the node.
    At this point in the processing, the root node does not have a type
    assigned to it, but everything else should.

    If this method is called on a node with existing problems, then there's
    a chance that problems will be duplicated in the output.
    """

    # General approach for these checks:
    #   - ensure the expected parent type matches the node type.
    #   - ensure expected children exist, and no extra children.
    # The parent is not responsible for checking the child type.
    # Otherwise, errors would be duplicated.

    node_type = node.get_assigned_type()
    parent = node.get_parent()

    if parent:
        # Simple early check.
        if not node_type:
            # The node has a parent, so it's not the root node.
            # And, at this point in processing, only the root node should have
            # no type assignment.
            node.add_problem(
                Problem.as_validation(
                    node.node_id.source,
                    _("No type discovered for {type}"),
                    type=node.type_id,
                )
            )
            return

        parameter_type = parent.get_parameter_type()
        # If the parent node is the root node, then the parent type and parameter
        # type are None at this point.
        if parameter_type and not parameter_type.is_type_allowed(node_type):
            node.add_problem(
                Problem.as_validation(
                    node.node_id.source,
                    _("node has type {typ} while parent '{key}' is a {par}"),
                    typ=repr(node_type),
                    key=parameter_type.key(),
                    par=repr(parameter_type),
                )
            )

        # The parent for the node has already been checked.  The
        #   specific instanceof checks are for checking their contents and children.
        if isinstance(node, ParsedSimpleNode):
            check_simple_node(node, node_type)
        elif isinstance(node, ParsedListNode):
            check_list_node(node, node_type)
        elif isinstance(node, ParsedParameterNode):
            check_parameter_node(node, node_type)
        else:
            node.add_problem(
                Problem.as_validation(
                    node.node_id.source,
                    _("Unknown node type {typ}"),
                    typ=repr(type(node)),
                )
            )
    else:
        check_root_node(node)


def _is_reference_type(value: SimpleParameter) -> bool:
    # reference type has extra checks that should be there,
    # Specifically, is-a-list-of-strings and has non-zero length.
    if not isinstance(value, (tuple, list)):
        return False
    if len(value) <= 0:
        return False
    for item in value:
        if not isinstance(item, str) or len(item) <= 0:
            return False
    return True


_SIMPLE_TYPE: Mapping[BasicTypeId, Callable[[SimpleParameter], bool]] = {
    "string": lambda p: isinstance(p, str),
    "number": lambda p: isinstance(p, (float, int)),
    "integer": lambda p: isinstance(p, int),
    "boolean": lambda p: isinstance(p, bool),
    "reference": _is_reference_type,
}


def check_simple_node(node: ParsedSimpleNode, node_type: AbcType) -> None:
    """Perform checks on a simple node."""
    if not isinstance(node_type, BasicType):
        node.add_problem(
            Problem.as_validation(
                node.node_id.source,
                _("node constructed as a {clz}, but type {typ} doesn't match"),
                clz=repr(type(node)),
                typ=repr(node_type),
            )
        )
        # Type doesn't match node class, so abort checking.
        return

    # Else there isn't a parameter type for this node.  That means
    #   that the node is on a key that the parameter node didn't define.
    #   This situation will be checked in the parent node, when it verifies
    #   the key existence.

    # Check the actual type of the value against the node type.
    value = node.value
    type_checker = _SIMPLE_TYPE.get(node.type_id)
    if not type_checker or not type_checker(value):
        node.add_problem(
            Problem.as_validation(
                node.node_id.source,
                _("Node marked as a {exp}, but the value is a {typ}"),
                exp=node.type_id,
                typ=repr(type(value)),
            )
        )


def check_list_node(node: ParsedListNode, node_type: AbcType) -> None:
    """Perform checks on a list node and the children count."""
    if not isinstance(node_type, ListType):
        node.add_problem(
            Problem.as_validation(
                node.node_id.source,
                _("node constructed as a {clz}, but type {typ} doesn't match"),
                clz=repr(type(node)),
                typ=repr(node_type),
            )
        )
        # Type doesn't match node class, so abort checking.
        return

    children = tuple(node.values())
    count = len(children)
    max_count = node_type.get_maximum_count()
    if max_count is not None and count > max_count:
        node.add_problem(
            Problem.as_validation(
                node.node_id.source,
                _("node can have a maximum of {mx} values, but it has {count} items"),
                mx=node_type.get_maximum_count(),
                count=count,
            )
        )
    if count < node_type.get_minimum_count():
        node.add_problem(
            Problem.as_validation(
                node.node_id.source,
                _("node requires at least {mn} values, but it has {count} items"),
                mn=node_type.get_minimum_count(),
                count=count,
            )
        )


def check_parameter_node(node: ParsedParameterNode, node_type: AbcType) -> None:
    """Perform checks on a parameter node."""
    if not isinstance(node_type, ConstructType):
        node.add_problem(
            Problem.as_validation(
                node.node_id.source,
                _("node constructed as a {clz}, but type {typ} doesn't match"),
                clz=repr(type(node)),
                typ=repr(node_type),
            )
        )
        # Type doesn't match node class, so abort checking.
        return

    # Now ensure all the required parameters exist, and that all other
    # parameters are optional.
    expected_parameters = {p.key(): p for p in node_type.parameters()}
    for key, child in node.mapping().items():
        param = expected_parameters.get(str(key))
        if not param:
            # Attach the problem to the child.
            node.add_problem(
                Problem.as_validation(
                    child.node_id.source,
                    _("Node in unknown parent parameter {key}"),
                    key=key,
                )
            )
        else:
            # Mark the key as found.
            del expected_parameters[str(key)]
            # Do not perform type checking on this child; that's
            # done by the child.

    # Ensure all required keys exist.
    for param in expected_parameters.values():
        if param.is_required():
            node.add_problem(
                Problem.as_validation(
                    node.node_id.source,
                    _("Did not include required parameter {key}"),
                    key=param.key(),
                )
            )


def check_root_node(node: AbcParsedNode) -> None:
    """Ensure the root node is fine.  The node must not have a parent
    (must be checked before calling here)."""
    if not isinstance(node, ParsedParameterNode):
        node.add_problem(
            Problem.as_validation(
                node.node_id.source,
                _("Root node must be a parameter node, found {typ}"),
                typ=node.type_id,
            )
        )
