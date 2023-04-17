"""Basic definitions.

NodeReference is both a unique identifier for a node (the path along the syntax
tree to the node), as well as a pointer to a node's value.

SimpleParameter values are parameters under a node that have a simple type,
and don't have the heavyweight node definition.  It is still referenceable through
a NodeReference.
"""

from typing import Sequence, Iterable, List, NewType, Union
import collections.abc

NodeReference = NewType("NodeReference", Sequence[str])
SimpleParameter = Union[int, float, bool, str, NodeReference]


def mk_ref(path: Sequence[str]) -> NodeReference:
    """Create a node reference from a node path."""
    if isinstance(path, tuple):
        return NodeReference(path)
    return NodeReference(tuple(path))


def build_ref(*paths: Union[int, str, Iterable[str]]) -> NodeReference:
    """Create a node reference from a node path."""
    ret: List[str] = []
    for path in paths:
        if isinstance(path, collections.abc.Iterable):
            for item in path:
                ret.append(str(item))
        else:
            ret.append(str(path))
    return NodeReference(tuple(ret))
