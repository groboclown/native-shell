"""Visitor pattern for the syntax builder tree."""

from typing import List, Tuple, Callable, Union, Optional
from ..defs.parse_tree import (
    AbcParsedNode,
)


def pre_visit_parsed_node(
    root: AbcParsedNode,
    visitor: Callable[[AbcParsedNode], None],
) -> None:
    """Visit the node in pre-ordering (node then all children)"""
    stack: List[AbcParsedNode] = [root]

    while stack:
        node = stack.pop()
        visitor(node)
        stack.extend((kn[1] for kn in ordered_parameters(node)))


def post_visit_parsed_node(
    root: AbcParsedNode,
    visitor: Callable[[AbcParsedNode], None],
) -> None:
    """Visit the node in the post ordering (all children first then the node).

    Not recursive, so stack size isn't a concern.
    """
    stack: List[VisitState] = [
        VisitState(
            parent=None,
            key="",
            current=root,
        )
    ]

    while stack:
        node = stack.pop()
        if node.is_complete():
            visitor(node.current)
            node.mark_visited()
        else:
            # This is fresh in the stack.  Add it back to check if it needs
            # to be visited.  This is put in first, so that the children
            # are parsed first.
            stack.append(node)
            # Then add in the children.
            stack.extend(node.create_children())


class VisitState:
    """State of the visitor"""

    __slots__ = ("parent", "key", "current", "remaining")

    def __init__(
        self,
        *,
        parent: "Optional[VisitState]",
        key: Union[str, int],
        current: AbcParsedNode,
    ) -> None:
        self.parent = parent
        self.key = key
        self.current = current
        self.remaining = set(current.keys())

    def is_complete(self) -> bool:
        """Is this node finished being used?"""
        return len(self.remaining) <= 0

    def mark_visited(self) -> None:
        """Mark this node as visited."""
        if self.parent and self.key in self.parent.remaining:
            self.parent.remaining.remove(self.key)

    def create_children(self) -> "List[VisitState]":
        """Create visitor children."""
        # These are sorted for consistent operation.
        return [
            VisitState(
                parent=self,
                key=k,
                current=v,
            )
            for k, v in ordered_parameters(self.current)
        ]


def ordered_parameters(node: AbcParsedNode) -> List[Tuple[Union[str, int], AbcParsedNode]]:
    """Return the parameters in correct parse order."""
    # Note: sort must guarantee that all the items sorted on are of the
    #   same type.  The container types are designed such that their
    #   keys have a uniform type, even though the type signatures suggest
    #   that they can contain both.

    return sorted(
        list(node.mapping().items()),
        key=lambda k: k[0],
    )
