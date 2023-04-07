"""Abstract Base Classes for the syntax tree.

Due to the tangled nature of the syntax tree nodes and types,
the two groups are joined together here.

The types are defined in the built-ins or add-ins.  Those contain
the implementation details related to the types.
"""

from typing import Sequence, Mapping, Literal, Union
from abc import ABC
from ..basic import SimpleParameter, NodeReference
from ...util.message import I18n
from ...util.result import Result, SourcePath


BasicType = Literal["string", "number", "integer", "boolean"]


class AbcTypeProperty:
    """A value contained within a type."""

    def key(self) -> str:
        """The property's key.  It's how the property is referenced by the script."""
        raise NotImplementedError

    def is_list(self) -> bool:
        """Is this a list of items?  If this is true, then the
        ``type`` is the per-item type."""
        raise NotImplementedError

    def type(self) -> BasicType | "AbcType":
        """Get the underlying type for this property.

        Note that property types can't be generator meta-types.
        They can only reference concrete types whose values the generator will
        aim to construct."""
        raise NotImplementedError

    def title(self) -> I18n:
        """Get the title for the property.

        This should be a short, human-readable name for the property.
        """
        raise NotImplementedError

    def description(self) -> I18n:
        """Get the description for this property.

        It should be a "long description" with detailed explanation for
        the property and how it's used.
        """
        raise NotImplementedError


class TypeParameter(AbcTypeProperty, ABC):
    """A parameter accepted by a type.

    Parameters are keyed values, and the type definition must provide the
    required type.
    """

    def is_required(self) -> bool:
        """Is this parameter required to be specified?"""
        raise NotImplementedError

    def validator(self) -> "TypeValidator":
        """Get the handler for validating this specific parameter."""
        raise NotImplementedError


class TypeField(AbcTypeProperty, ABC):
    """Storage for a value in a type.

    A type may provide information usable by a script, such as a process ID,
    which the script cannot provide.
    """

    def is_usable_before_invoking(self) -> bool:
        """If True, then the script can reference this information
        before the tree node is executed.  Otherwise, referencing this
        before the owning node is run will cause an error."""
        raise NotImplementedError


class AbcBaseType:
    """The base of the type system.  This is for both the concrete types
    and the generator meta-types.
    """

    def source(self) -> SourcePath:
        """The source for this type.  This allows the user to know if the
        type came from a module, or defined in the script, or is built-in."""
        raise NotImplementedError

    def type_id(self) -> str:
        """Unique identifier for this type."""
        raise NotImplementedError

    def title(self) -> I18n:
        """Get the title for the parameter.

        This should be a short, human-readable name for the parameter.
        """
        raise NotImplementedError

    def description(self) -> I18n:
        """Get the description for this parameter.

        It should be a "long description" with detailed explanation for
        the parameter and how it's used.
        """
        raise NotImplementedError


class AbcType(AbcBaseType, ABC):
    """The Abstract Base Class of the concrete types.

    This covers everything from a simple constant to a complex code generator
    that solves all the world's problems.
    """

    def parameters(self) -> Sequence[TypeParameter]:
        """Get the parameters accepted by this type."""
        raise NotImplementedError

    def fields(self) -> Sequence[TypeField]:
        """Get the additional provided keys for this type.
        These are read-only from outside the type, and are provided
        for checking available fields and ."""
        raise NotImplementedError


class AbcMetaType(AbcBaseType, ABC):
    """The Abstract Base Class of the generator meta-types.

    These build out other types.
    """

    def meta_parameters(self) -> Sequence[TypeParameter]:
        """Get the parameters accepted by this type."""
        raise NotImplementedError


SyntaxParameter = Union["SyntaxNode", SimpleParameter]


class SyntaxNode:
    """A finalized node in the syntax tree.  It isn't 1-to-1 related to the
    underlying type system, because a node may be a list of nodes, if the
    parent parameter type has "is_list" marked.

    If the parent parameter marked this node as a list, then the parameter values
    returns the list translated into a map.  If order is important, then the caller
    must perform numeric sorting on the keys.

    A node has a type and associated parameter values.  It cannot
    be of a generator meta-type.  At this point in the script construction,
    all problems have been dealt with.
    """

    __slots__ = ("__source", "__node_id", "__type", "__values")

    def __init__(
        self,
        *,
        source: SourcePath,
        node_id: NodeReference,
        node_type: AbcType,
        values: Mapping[str, SyntaxParameter],
    ) -> None:
        # As this is the finalized form, we make copies of the compound types.
        self.__source = tuple(source)
        self.__node_id = node_id
        self.__type = node_type
        self.__values = dict(values)

    def source(self) -> SourcePath:
        """Get the source location for the node."""
        return self.__source

    def node_id(self) -> NodeReference:
        """The identifier for the node.  This is equivalent to an
        absolute reference path.  This is used to generate unique names
        for the node."""
        return self.__node_id

    def node_type(self) -> AbcType:
        """Get the node's type."""
        return self.__type

    def values(self) -> Mapping[str, SyntaxParameter]:
        """Get the values in this node.  These are either parameters or fields or,
        in the case of a list node, the list turned into a map by translating the
        index to a string."""
        return self.__values


class TypeValidator:
    """Validates whether a syntax node is compatible with a type."""

    def type_id(self) -> str:
        """Associated type's unique identifier."""
        raise NotImplementedError

    def validate(self, node: SyntaxNode) -> Result[None]:
        """Validate the node.  The result will either be valid or invalid."""
        raise NotImplementedError