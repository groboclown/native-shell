"""Visitor pattern for the syntax builder tree."""

from typing import List, Callable, Optional
from ..defs.parse_tree.defs import ParsedNode


def visit_parsed_node(
    root: ParsedNode,
    visitor: Callable[[ParsedNode], None],
) -> None:
    """Visit the node in the correct ordering (post).

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
        key: str,
        current: ParsedNode,
    ) -> None:
        self.parent = parent
        self.key = key
        self.current = current
        self.remaining = set(current.node_parameter_map().keys())

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
        items = list(self.current.node_parameter_map().items())
        items.sort(key=lambda k: k[0])
        return [
            VisitState(
                parent=self,
                key=k,
                current=v,
            )
            for k, v in items
        ]
