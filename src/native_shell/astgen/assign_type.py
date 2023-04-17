"""Assigns types to a parsed node."""

from typing import Sequence
from .typed_tree import TypedTree
from ..defs.parse_tree import (
    AbcParsedNode,
    ParsedListNode,
    ParsedParameterNode,
)
from ..defs.node_type import ConstructType, ListType, AbcTypeParameter
from ..builtins.core.simple_parameters import UNTYPED_LIST_TYPE
from ..util.message import i18n as _
from ..util.message import UserMessage
from ..util.result import Problem


def assign_types_to_node(  # pylint:disable=too-many-branches
    node: AbcParsedNode,
    tree: TypedTree,
) -> None:
    """Assign types to the node.  Additionally, mark all the type handlers
    that are being used in this tree.

    The parent must have its types assigned first.
    """

    # Only care about valid nodes.
    if node.is_not_valid():
        return

    # Don't care about the root node.  It doesn't have a parent, and
    #   has its type assigned at the very end.
    parent = node.get_parent()
    if parent is None:
        return

    # Set up the parent parameter reference.
    parent_parameter = parent.get_parameter_type()
    if parent_parameter is None:
        # The root node does not have a parent.
        parent_type = parent.node.get_assigned_type()
        if parent_type and isinstance(parent_type, ListType):
            parent_parameter = parent_type.get_item_parameter()
            parent.set_parameter_type(parent_parameter)
        if parent_type and isinstance(parent_type, ConstructType):
            param_types: Sequence[AbcTypeParameter] = tuple(
                filter(
                    lambda t: t.key() == parent.key,  # type: ignore
                    parent_type.parameters(),
                )
            )
            if len(param_types) > 0:
                parent_parameter = param_types[0]
                parent.set_parameter_type(parent_parameter)
            # else this isn't a valid key, and that will be picked up during node validation.
        # else the parent type is BasicType, which shouldn't happen.

    if node.get_assigned_type() is not None:
        # The assigned type for this node has already happened.
        # Don't need to worry about it.
        # Note that this covers BasicType values.
        return

    handler = tree.get_handler(node.type_id)
    if handler is None:
        # No handler for the node.
        if isinstance(node, ParsedListNode):
            # For the case of a list, we'll have a make-shift solution.
            node.set_type(UNTYPED_LIST_TYPE)
        else:
            # Otherwise, it's a parameter / construct node.  These
            #   have to have types.
            node.add_problem(
                Problem.as_validation(
                    node.node_id.source,
                    UserMessage(
                        _("node has unknown type {type}"),
                        node=repr(node),
                        type=node.type_id,
                    ),
                )
            )
    else:
        tree.mark_referenced(handler)
        handler_type = handler.type()
        if isinstance(node, ParsedParameterNode):
            if not isinstance(handler_type, ConstructType):
                node.add_problem(
                    Problem.as_validation(
                        node.node_id.source,
                        _("node has parameters, but type is not a parameter type ({typ})"),
                        typ=repr(handler_type),
                    )
                )
            else:
                node.set_type(handler_type)
        elif isinstance(node, ParsedListNode):
            if not isinstance(handler_type, ListType):
                node.add_problem(
                    Problem.as_validation(
                        node.node_id.source,
                        _("node is a list, but type is not a list type ({typ})"),
                        typ=repr(handler_type),
                    )
                )
            else:
                node.set_type(handler_type)
        # else it's a basic node, which we have a type already set up.
