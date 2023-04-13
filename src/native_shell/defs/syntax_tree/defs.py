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
from ...util.result import SourcePath


BasicType = Literal["string", "number", "integer", "boolean", "reference"]
BASIC_TYPE_NAMES = ("string", "number", "integer", "boolean", "reference")

# The ListType is an artificial node constructed to be a container for
#   a parameter that contains a list of items.
ListType = Literal["list"]
LIST_TYPE_NAME: ListType = "list"


class AbcTypeProperty:
    """A value contained within a type."""

    def key(self) -> str:
        """The property's key.  It's how the property is referenced by the script."""
        raise NotImplementedError

    def is_list(self) -> bool:
        """Is this a list of items?  If this is true, then the
        ``type`` is the per-item type."""
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

    def type(self) -> Union[BasicType, "AbcType"]:
        """Get the underlying type for this property.

        Note that property types can't be generator meta-types.
        They can only reference concrete types whose values the generator will
        aim to construct.  List types specify the list item type here
        and the 'is_list' call marks it as a list."""
        raise NotImplementedError

    def is_required(self) -> bool:
        """Is this parameter required to be specified?"""
        raise NotImplementedError

    # This should include a type validator, but that introduces complex
    # dependency issues.  Instead, validation is performed by the type when
    # generating the code.


class TypeField(AbcTypeProperty, ABC):
    """Storage for a value in a type.

    Fields must be an AbcType, because their purpose is to express a value that
    can be set and read from.  It may be possible in theory to have a field be
    a list of values, but right now that isn't supported.

    A type may provide information usable by a script, such as a process ID,
    which the script cannot provide.

    If a field type has a parameter, it is ignored.

    Currently, a field's type may have its own fields, but they are also ignored.
    The meaning behind that, from a code perspective, would be a member to a structure.
    It's up to the caller that references that field to find the sub-field correctly.
    """

    def type(self) -> "AbcType":
        """Get the underlying type for this property.

        Note that property types can't be generator meta-types.
        They can only reference concrete types whose values the generator will
        aim to construct.
        Field types can't be basic types, because their only purpose for being
        declared is to be referencable through code, which means needing a type
        handler to do just that."""
        raise NotImplementedError

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
        node_type: Union[AbcType, ListType],
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

    def node_type(self) -> Union[AbcType, ListType]:
        """Get the node's type."""
        return self.__type

    def values(self) -> Mapping[str, SyntaxParameter]:
        """Get the values in this node.  These are either parameters or fields or,
        in the case of a list node, the list turned into a map by translating the
        index to a string."""
        return self.__values

    def __repr__(self) -> str:
        return "/".join(self.__node_id)
