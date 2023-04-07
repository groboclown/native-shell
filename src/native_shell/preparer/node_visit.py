"""Visitor pattern for the syntax builder tree."""

from typing import List, Callable, Optional
from ..defs.syntax_tree import AbcSyntaxBuildingNode


def visit_building_node(
    root: AbcSyntaxBuildingNode,
    visitor: Callable[[AbcSyntaxBuildingNode], None],
) -> None:
    """Visit the node in the correct ordering (post)."""
    stack: List[VisitState] = [
        VisitState(
            parent=None,
            key="",
            current=root,
        )
    ]

    while True:
        node = stack.pop()
        if not stack:
            break
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
        current: AbcSyntaxBuildingNode,
    ) -> None:
        self.parent = parent
        self.key = key
        self.current = current
        self.remaining = {n for n in current.parameter_values().keys()}

    def is_complete(self) -> bool:
        """Is this node finished being used?"""
        return len(self.remaining) <= 0

    def mark_visited(self) -> None:
        """Mark this node as visited."""
        if self.parent and self.key in self.parent.remaining:
            self.parent.remaining.remove(self.key)

    def create_children(self) -> "List[VisitState]":
        """Create visitor children."""
        return [
            VisitState(
                parent=self,
                key=k,
                current=v,
            )
            for k, v in self.current.parameter_values().items()
        ]
