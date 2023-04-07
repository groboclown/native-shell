"""Different types of node tree visitors."""

from typing import Sequence, List, Callable, Union
from ..defs.syntax_tree import SyntaxNode, SimpleParameter


def walk_all(
    root: SyntaxNode,
    visitor: Callable[
        [Sequence[str], Union[SyntaxNode, SimpleParameter]],
        bool,
    ],
) -> None:
    """A simple walk of the node and its children (parent first).
    This passes the node id + the value.  It walks both nodes and simple parameters.
    If the visitor returns True, then the walk stops immediately.
    """
    stack: List[SyntaxNode] = [root]
    while stack:
        node = stack.pop()
        # Walk the parent first
        if visitor(node.node_id(), node):
            return
        # Walk the simple parameters next.
        # Put the node children in the stack for later calling.
        for key, child in node.values().items():
            if isinstance(child, SyntaxNode):
                stack.append(child)
            else:
                if visitor([*node.node_id(), key], child):
                    return


def walk_nodes(
    root: SyntaxNode,
    visitor: Callable[[SyntaxNode], bool],
) -> None:
    """A simple walk of the node and its children (parent first).
    This passes the node id + the value.  It walks both nodes and simple parameters.
    If the visitor returns True, then the walk stops immediately.
    """
    stack: List[SyntaxNode] = [root]
    while stack:
        node = stack.pop()
        # Walk the parent first
        if visitor(node):
            return
        # Put the node children in the stack for later calling.
        for child in node.values().values():
            if isinstance(child, SyntaxNode):
                stack.append(child)
            # Ignore simple children
