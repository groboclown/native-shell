"""The default AbcType implementation."""

from typing import Callable
from ..defs.node_type import AbcType, AbcTypeParameter
from ..util.message import I18n


class ExplicitTypeParameter(AbcTypeParameter):
    """Default type implementation."""

    def __init__(
        self,
        *,
        key: str,
        type_val: AbcType,
        title: I18n,
        description: I18n,
        required: bool,
    ) -> None:
        self.__key = key
        self.__type = type_val
        self.__title = title
        self.__description = description
        self.__required = required

    def key(self) -> str:
        return self.__key

    def type(self) -> AbcType:
        return self.__type

    def title(self) -> I18n:
        return self.__title

    def description(self) -> I18n:
        return self.__description

    def is_required(self) -> bool:
        return self.__required

    def is_type_allowed(self, other: AbcType) -> bool:
        return self.__type is other

    def __repr__(self) -> str:
        return f"TypeParameter({self.__key}: {self.__title}, ({self.__type}))"


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
