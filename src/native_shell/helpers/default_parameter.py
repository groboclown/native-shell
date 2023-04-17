"""The default AbcType implementation."""

from typing import Callable
from ..defs.node_type import AbcType, AbcTypeParameter, ListType
from ..util.message import I18n


def create_explicit_type_parameter(
    *,
    key: str,
    type_val: AbcType,
    title: I18n,
    description: I18n,
    required: bool,
) -> AbcTypeParameter:
    """Create a type parameter where values must be of an exact type."""
    return DefaultTypeParameter(
        key=key,
        title=title,
        description=description,
        required=required,
        type_checker=lambda t: t is type_val,
    )


def create_delayed_list_type_parameter(
    *,
    key: str,
    title: I18n,
    description: I18n,
    required: bool,
) -> AbcTypeParameter:
    """Create a type parameter where the values are lists,
    and the underlying item types will be checked later."""
    return DefaultTypeParameter(
        key=key,
        title=title,
        description=description,
        required=required,
        type_checker=lambda t: isinstance(t, ListType),
    )


class DefaultTypeParameter(AbcTypeParameter):
    """Default type implementation."""

    def __init__(
        self,
        *,
        key: str,
        title: I18n,
        description: I18n,
        required: bool,
        type_checker: Callable[[AbcType], bool],
    ) -> None:
        self.__key = key
        self.__title = title
        self.__description = description
        self.__required = required
        self.__checker = type_checker

    def key(self) -> str:
        return self.__key

    def title(self) -> I18n:
        return self.__title

    def description(self) -> I18n:
        return self.__description

    def is_required(self) -> bool:
        return self.__required

    def is_type_allowed(self, other: AbcType) -> bool:
        return self.__checker(other)

    def __repr__(self) -> str:
        return f"TypeParameter({self.__key}: {self.__title})"
