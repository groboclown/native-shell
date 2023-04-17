"""Abstract Base Classes for the type definitions.

Due to the tangled nature of the syntax tree nodes and types,
the two groups are joined together here.

The types are defined in the built-ins or add-ins.  Those contain
the implementation details related to the types.
"""

from typing import Iterable, Sequence, Literal, Optional
from abc import ABC
from ...util.message import I18n
from ...util.result import SourcePath


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
    """The Abstract Base Class for concrete types.

    This is a marker for separating types between meta-types and concrete types.
    """


class AbcTypeProperty:
    """A value contained within a type."""

    def key(self) -> str:
        """The property's key.  It's how the property is referenced by the script."""
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


class AbcTypeParameter(AbcTypeProperty, ABC):
    """A parameter accepted by a type.

    Parameters are keyed values, and the type definition must provide the
    a value with an allowed type.
    """

    def is_required(self) -> bool:
        """Is this parameter required to be specified?"""
        raise NotImplementedError

    def is_type_allowed(self, other: AbcType) -> bool:
        """For this type, used as a required parameter type, does the other
        type satisfy the requirements for this?

        This should be a type *value* validator, but that introduces complex
        dependency issues.  Instead, type validation is handled here.

        Most types should have a simple "return some_type is other" check
        """
        raise NotImplementedError


class AbcTypeField(AbcTypeProperty, ABC):
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

    def type(self) -> AbcType:
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


class ConstructType(AbcType):
    """A type that relates to generated code constructed from user definitions.  They
    can have parameters, which the user passes to the value, and they can generate fields,
    which are outputs from the generated code.  Parameters and code may reference fields,
    but parameters are only used by the type handler for the value when generating the code."""

    def __init__(
        self,
        *,
        source: SourcePath,
        type_id: str,
        title: I18n,
        description: I18n,
        parameters: Iterable[AbcTypeParameter],
        fields: Iterable[AbcTypeField],
    ) -> None:
        self.__source = source
        self.__type_id = type_id
        self.__title = title
        self.__description = description
        self.__parameters = tuple(parameters)
        self.__fields = tuple(fields)

    def source(self) -> SourcePath:
        return self.__source

    def type_id(self) -> str:
        return self.__type_id

    def title(self) -> I18n:
        return self.__title

    def description(self) -> I18n:
        return self.__description

    def parameters(self) -> Sequence[AbcTypeParameter]:
        """Get the parameters accepted by this type."""
        return self.__parameters

    def fields(self) -> Sequence[AbcTypeField]:
        """Get the additional provided keys for this type.
        These are read-only from outside the type, and are provided
        for checking available fields and ."""
        return self.__fields

    def __repr__(self) -> str:
        return self.__title


class ListType(AbcType):
    """The Abstract Base Class of the list types.  The mapping of values for the syntax
    node will be a conversion of the index to a string key.

    The methods ``get_minimum_count``, ``get_maximum_count``, and ``is_type_allowed`` are
    only useful when this list type is used as a parameter.  For field types, the type
    is used as a marker that the underlying generated code will have special keys.
    """

    def __init__(
        self,
        *,
        source: SourcePath,
        type_id: str,
        title: I18n,
        description: I18n,
        items: AbcTypeParameter,
        minimum_count: int,
        maximum_count: Optional[int],
    ) -> None:
        self.__source = source
        self.__type_id = type_id
        self.__title = title
        self.__description = description
        self.__items = items
        self.__minimum_count = minimum_count
        self.__maximum_count = maximum_count

    def source(self) -> SourcePath:
        return self.__source

    def type_id(self) -> str:
        return self.__type_id

    def title(self) -> I18n:
        return self.__title

    def description(self) -> I18n:
        return self.__description

    def get_minimum_count(self) -> int:
        """Get the minimum number of items required by the list."""
        return self.__minimum_count

    def get_maximum_count(self) -> Optional[int]:
        """Get the maximum number of items required by the list.

        Returns None if there is no maximum count."""
        return self.__maximum_count

    def get_item_parameter(self) -> AbcTypeParameter:
        """A reference to the items stored in the list, as a parameter.
        The 'key' for the parameter should be 'items'.  'is_required' should
        be false."""
        return self.__items

    def __repr__(self) -> str:
        return self.__title


# Matches with SimpleParameter types.
BasicTypeId = Literal["integer", "number", "boolean", "string", "reference"]
BASIC_TYPE_IDS: Sequence[BasicTypeId] = (
    "integer",
    "number",
    "boolean",
    "string",
    "reference",
)


class BasicType(AbcType):
    """A basic value store.  These are not stored in the syntax tree as a node, but as a
    simple parameter."""

    def __init__(
        self,
        *,
        source: SourcePath,
        type_id: BasicTypeId,
        title: I18n,
        description: I18n,
    ) -> None:
        self.__source = source
        self.__type_id = type_id
        self.__title = title
        self.__description = description

    def source(self) -> SourcePath:
        return self.__source

    def type_id(self) -> str:
        return self.__type_id

    def title(self) -> I18n:
        return self.__title

    def description(self) -> I18n:
        return self.__description

    def __repr__(self) -> str:
        return self.__type_id


class AbcMetaType(AbcBaseType, ABC):
    """The Abstract Base Class of the generator meta-types.

    These build out other types.
    """

    def meta_parameters(self) -> Sequence[AbcTypeParameter]:
        """Get the parameters accepted by this type."""
        raise NotImplementedError
