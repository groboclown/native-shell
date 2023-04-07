"""Handles references to node parameters."""

from typing import Sequence, Dict, Union, Optional
from .tree_visitor import walk_all
from ..defs.syntax_tree import SyntaxNode, SimpleParameter
from ..defs.add_ins import GeneratedCode, CodePurpose


class CachedRef:
    """A cached reference."""

    __slots__ = ("val", "code")

    def __init__(self, value: Union[SyntaxNode, SimpleParameter]) -> None:
        self.val = value
        self.code: Dict[CodePurpose, GeneratedCode] = {}


class ReferenceStore:
    """Stores references to all the nodes."""

    # Right now this just stores everything.  That would mean this could become
    #   a significant memory user.
    __slots__ = ("__refs",)

    def __init__(self, root: SyntaxNode) -> None:
        self.__refs: Dict[Sequence[str], CachedRef] = {}

        def visitor(ref: Sequence[str], val: Union[SyntaxNode, SimpleParameter]) -> bool:
            if not isinstance(ref, tuple):
                ref = tuple(ref)
            self.__refs[ref] = CachedRef(val)
            return False

        walk_all(root, visitor)

    def find(self, ref: Sequence[str]) -> Optional[CachedRef]:
        """Find the cached reference."""
        if not isinstance(ref, tuple):
            ref = tuple(ref)
        return self.__refs.get(ref)
