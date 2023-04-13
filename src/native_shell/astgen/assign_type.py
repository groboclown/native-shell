"""Assigns types to a parsed node."""

from typing import Optional

from .typed_tree import TypedTree
from ..defs.parse_tree import (
    AbcParsedNode,
    ParsedListNode,
    ParsedParameterNode,
)
from ..defs.syntax_tree import (
    TypeParameter,
)
from ..util.message import i18n as _
from ..util.message import UserMessage
from ..util.result import Problem


def assign_types_to_node(  # pylint:disable=too-many-return-statements,too-many-branches
    node: AbcParsedNode,
    tree: TypedTree,
) -> None:
    """Assign types to the node.

    The parent must have its types assigned first.
    """
    print(f"Visiting {node}")

    # Only care about valid nodes.
    if node.is_not_valid():
        print(" - not valid; skipping")
        return

    parent = node.get_parent()
    if parent is None:
        if node.type_id:
            node.add_problem(
                Problem.as_validation(
                    node.node_id.source,
                    UserMessage(
                        _("root node must have an empty type id; found '{type_id}'"),
                        type_id=node.type_id,
                    ),
                )
            )
            # Invalid, so quit immediately
            return

        # The root node.  We don't care about type checking this directly.
        print(" - root node; skipping")
        return

    if isinstance(node, ParsedParameterNode):
        if setup_types_for_parameter_node(node, tree):
            return

    if parent.node.is_not_valid() or parent.node.type_id == "":
        # Do not parse parent info for "node" if the parent is bad,
        #   or if it's the root node.
        if parent.node.is_not_valid():
            print(" - parent isn't valid; skipping parent check.")
            print(" -- " + (" -- \n".join([repr(p) for p in parent.node.problems()])))
        else:
            print(" - parent is root node ; skip parent type check")
        return
    if isinstance(parent.node, ParsedListNode):
        print(" - parent is a list node")
        # The parent type is declared directly in the list.
        item_type = parent.node.get_item_type()
        if item_type is None:
            # Parent should be fully parsed.
            # However, if it encountered an error and *its* parent was invalid,
            # then it won't be set right.
            # So skip this invalid node.
            print(" -- parent had an invalid state; skipping")
            return
        # Note: the parent item type doesn't need to be is_list;
        #  the parent's parent's parameter type must be is_list.
        print(f" -- setting {node} parent parameter type as {item_type}")
        parent.set_parameter_type(item_type)
    elif isinstance(parent.node, ParsedParameterNode):
        # Find the key in the parent type handler.
        parent_handler = tree.handlers.get(parent.node.type_id)
        if parent_handler:
            param_type: Optional[TypeParameter] = None
            for kpt in parent_handler.type().parameters():
                if kpt.key() == parent.key:
                    param_type = kpt
                    break
            if param_type is None:
                # Invalid key!
                node.add_problem(
                    Problem.as_validation(
                        node.node_id.source,
                        UserMessage(
                            _("node {node} defined as child of {parent} at undefined key {key}"),
                            node=repr(node),
                            parent=repr(parent.node),
                            key=parent.key,
                        ),
                    )
                )
                # Invalid, so quit immediately
                return
            if isinstance(node, ParsedListNode):
                # Do not check whether the type is a list or not.
                node.set_item_type(param_type)
            tree.mark_referenced(param_type)
            parent.set_parameter_type(param_type)
        else:
            # The handler isn't found, then the parent *should* be bad.
            # But, we already checked if it's bad, so mark as a fail-safe.
            node.add_problem(
                Problem.as_validation(
                    node.node_id.source,
                    UserMessage(
                        _("child {node} has parent {parent} with unknown type {type}"),
                        node=repr(node),
                        parent=repr(parent.node),
                        type=parent.node.type_id,
                    ),
                )
            )
    else:
        # It's a simple type, which is a parent?  That doesn't
        # make sense.
        raise RuntimeError(f"non-container node ({parent.node}) has child ({node})")


def setup_types_for_parameter_node(node: ParsedParameterNode, tree: TypedTree) -> bool:
    """Set up the parameter type stuff.

    Returns True on error."""
    # Only the parameter node type has an assignable type.
    print(" - parameter type...")
    type_handler = tree.handlers.get(node.type_id)
    if type_handler is None:
        node.add_problem(
            Problem.as_validation(
                node.node_id.source,
                UserMessage(
                    _("unknown type '{type_id}'"),
                    type_id=node.type_id,
                ),
            )
        )
        # Invalid, so quit immediately
        return True
    tree.mark_referenced(type_handler)
    node.set_type(type_handler.type())
    # Need to explicitly add type references for the fields.
    for field in type_handler.type().fields():
        field_handler = tree.handlers.get(field.type())
        if field_handler is None:
            node.add_problem(
                Problem.as_validation(
                    node.node_id.source,
                    _("field {key} registered with type '{typ}' that has no handler"),
                    typ=node.type_id,
                    key=field.key(),
                )
            )
        else:
            tree.mark_referenced(field_handler)
    print(f" -- found type handler {type_handler}")
    return False
