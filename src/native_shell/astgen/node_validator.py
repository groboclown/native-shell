"""Validates a parsed node for correctness.

Most of these checks end up just ensuring that the script parser
creates the right structure, and that all the finalization parts
up to this point worked right.  That is to say, it's just checking
for bugs in the parser.
"""

from typing import Mapping, Callable, Union, cast
from ..defs.basic import SimpleParameter
from ..defs.parse_tree import (
    AbcParsedNode,
    ParsedListNode,
    ParsedParameterNode,
    ParsedSimpleNode,
    ParentReference,
)
from ..defs.syntax_tree import (
    BasicType,
    ListType,
    AbcType,
    BASIC_TYPE_NAMES,
    LIST_TYPE_NAME,
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

        # General approach for these checks:
        #   - ensure the expected parent type matches the node type.
        #   - ensure expected children exist, and no extra children.
        # The parent is not responsible for checking the child type.
        # Otherwise, errors would be duplicated.

        if isinstance(node, ParsedSimpleNode):
            check_simple_node(node, node_type, parent)
        elif isinstance(node, ParsedListNode):
            check_list_node(node, node_type, parent)
        elif isinstance(node, ParsedParameterNode):
            check_parameter_node(node, node_type, parent)
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


_SIMPLE_TYPE: Mapping[BasicType, Callable[[SimpleParameter], bool]] = {
    "string": lambda p: isinstance(p, str),
    "number": lambda p: isinstance(p, (float, int)),
    "integer": lambda p: isinstance(p, int),
    "boolean": lambda p: isinstance(p, bool),
    "reference": _is_reference_type,
}


def check_simple_node(
    node: ParsedSimpleNode,
    orig_node_type: Union[BasicType, ListType, AbcType],
    parent: ParentReference,
) -> None:
    """Perform checks on a simple node."""
    if orig_node_type not in BASIC_TYPE_NAMES:
        node.add_problem(
            Problem.as_validation(
                node.node_id.source,
                _("node constructed as a {clz}, but type {typ} doesn't match"),
                clz=repr(type(node)),
                typ=repr(orig_node_type),
            )
        )
        # Type doesn't match node class, so abort checking.
        return

    # For mypy.  The above check essentially did this.
    node_type = cast(BasicType, orig_node_type)

    if isinstance(parent.node, ParsedListNode):
        # Synthetic list parent handler.
        parent_param = parent.node.get_item_type()
        if parent_param and node_type != parent_param.type():
            node.add_problem(
                Problem.as_validation(
                    node.node_id.source,
                    _("node has type {typ} while parent is a list of type {par}"),
                    typ=repr(node_type),
                    par=repr(parent_param.type()),
                )
            )
        # If parent_param is None, then that's the parent's problem.

    if isinstance(parent.node, ParsedParameterNode):
        # Parent is the only other parent storage type.
        param_type = parent.get_parameter_type()

        # If the parent is the root node, then there won't be a parameter type.
        if param_type:
            # There's a parameter type, so it must match up.
            if param_type.type() != node_type:
                node.add_problem(
                    Problem.as_validation(
                        node.node_id.source,
                        _("Parent required {key} to be a {par}, but it is a {typ}"),
                        typ=repr(node_type),
                        key=param_type.key(),
                        par=repr(param_type.type()),
                    )
                )
            if param_type.is_list():
                node.add_problem(
                    Problem.as_validation(
                        node.node_id.source,
                        _("Parent required {key} to be a list of {par}, but it is a {typ}"),
                        typ=repr(node_type),
                        key=param_type.key(),
                        par=repr(param_type.type()),
                    )
                )
        # Else there isn't a parameter type for this node.  That means
        #   that the node is on a key that the parameter node didn't define.
        #   This situation will be checked in the parent node, when it verifies
        #   the key existence.

    # Check the actual type of the value against the node type.
    value = node.value
    type_checker = _SIMPLE_TYPE.get(node_type)
    if not type_checker or not type_checker(value):
        node.add_problem(
            Problem.as_validation(
                node.node_id.source,
                _("Node marked as a {exp}, but the value is a {typ}"),
                exp=node_type,
                typ=repr(type(value)),
            )
        )


def check_list_node(
    node: ParsedListNode,
    node_type: Union[BasicType, ListType, AbcType],
    parent: ParentReference,
) -> None:
    """Perform checks on a list node."""
    if node_type != LIST_TYPE_NAME:
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

    # node.get_item_type - if this is None, then that usually means the
    #   parent didn't have a parameter for this node.  That's a problem
    #   for the parent to report, not this node.

    if isinstance(parent.node, ParsedListNode):
        # Because list nodes are synthetic, and just the semantics
        # of the tree structure, we can't have lists of lists directly.
        node.add_problem(
            Problem.as_validation(
                node.node_id.source,
                _("list nodes cannot have parents that are list nodes."),
            )
        )

    if isinstance(parent.node, ParsedParameterNode):
        item_type = node.get_item_type()
        # Parent is the only other parent storage type.
        param_type = parent.get_parameter_type()

        # If the parent is the root node, then there won't be a parameter type.
        if param_type:
            # There's a parameter type, so it must match up.
            if item_type and param_type.type() != item_type.type():
                node.add_problem(
                    Problem.as_validation(
                        node.node_id.source,
                        _("Parent required {key} to be a {par}, but it is a {typ}"),
                        typ=repr(node_type),
                        key=param_type.key(),
                        par=repr(param_type.type()),
                    )
                )
            if not param_type.is_list():
                node.add_problem(
                    Problem.as_validation(
                        node.node_id.source,
                        _("Parent required {key} to be a {par}, but it is a list of {typ}"),
                        typ=repr(node.get_item_type()),
                        key=param_type.key(),
                        par=repr(param_type.type()),
                    )
                )
        # Else there isn't a parameter type for this node.  That means
        #   that the node is on a key that the parameter node didn't define.
        #   This situation will be checked in the parent node, when it verifies
        #   the key existence.

    # ... do not check children types.


def check_parameter_node(
    node: ParsedParameterNode,
    node_type: Union[BasicType, ListType, AbcType],
    parent: ParentReference,
) -> None:
    """Perform checks on a parameter node."""
    if not isinstance(node_type, AbcType):
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
    # From here on, AbcType checks must be "is" or "is not", not == or !=.

    if isinstance(parent.node, ParsedListNode):
        # Synthetic list parent handler.
        if node_type is not parent.node.get_item_type():
            node.add_problem(
                Problem.as_validation(
                    node.node_id.source,
                    _("node has type {typ} while parent is a list of type {par}"),
                    typ=repr(node_type),
                    par=repr(parent.node.get_item_type()),
                )
            )

    if isinstance(parent.node, ParsedParameterNode):
        # Parent is the only other parent storage type.
        param_type = parent.get_parameter_type()

        # If the parent is the root node, then there won't be a parameter type.
        if param_type:
            # There's a parameter type, so it must match up.
            if node_type is not param_type.type():
                node.add_problem(
                    Problem.as_validation(
                        node.node_id.source,
                        _("Parent required {key} to be a {par}, but it is a {typ}"),
                        typ=repr(node_type),
                        key=param_type.key(),
                        par=repr(param_type.type()),
                    )
                )
            if param_type.is_list():
                node.add_problem(
                    Problem.as_validation(
                        node.node_id.source,
                        _("Parent required {key} to be a list of {par}, but it is a {typ}"),
                        typ=repr(node_type),
                        key=param_type.key(),
                        par=repr(param_type.type()),
                    )
                )
        # Else there isn't a parameter type for this node.  That means
        #   that the node is on a key that the parameter node didn't define.
        #   This situation will be checked in the parent node, when it verifies
        #   the key existence.

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
