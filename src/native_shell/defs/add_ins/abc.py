"""Abstract Base Classes for the add-ins."""

from typing import Sequence
from ..syntax_tree import AbcType, AbcMetaType, AbcSyntaxBuildingNode
from ...util.result import Result


class AddInTypeHandler:
    """A type specific to an add-in, along with how the
    add-in uses this to handle the syntax nodes of the type."""

    def type(self) -> AbcType:
        """The type representation for this handler."""
        raise NotImplementedError

    def compile_imports(self) -> Sequence[str]:
        """All the compile imports required if this type is referenced."""
        raise NotImplementedError

    def compile_shared(self) -> Sequence[str]:
        """A collection of shared code snippets to include in the file
        if this type is referenced in the script."""
        raise NotImplementedError


class AddInMetaType:
    """A meta-type definition for an add-in."""

    def meta_type(self) -> AbcMetaType:
        """The type representation for this handler."""
        raise NotImplementedError

    def translate(self, tree: AbcSyntaxBuildingNode) -> Result[AbcSyntaxBuildingNode]:
        """Translates the tree into another tree through the meta-type rules."""
        raise NotImplementedError


class AbcAddIn:
    """A basic add-in.

    Add-ins define new capabilities that a script can use.
    """

    def name(self) -> str:
        """The add-in formal name."""
        raise NotImplementedError

    def description(self) -> str:
        """A long description of the add-in."""
        raise NotImplementedError

    def include_name(self) -> str:
        """The name of the add-in as formally declared in the script
        when including this add-in."""
        raise NotImplementedError

    def type_handlers(self) -> Sequence[AddInTypeHandler]:
        """The type handlers the add-in can handle."""
        raise NotImplementedError
