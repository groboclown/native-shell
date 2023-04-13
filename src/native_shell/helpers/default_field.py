"""The default AbcType implementation."""

from ..defs.syntax_tree import AbcType, TypeField
from ..util.message import I18n


class DefaultTypeField(TypeField):
    """Default type implementation."""

    def __init__(
        self,
        *,
        key: str,
        is_list: bool,
        type_val: AbcType,
        title: I18n,
        description: I18n,
        usable_before_invoking: bool,
    ) -> None:
        self.__key = key
        self.__is_list = is_list
        self.__type = type_val
        self.__title = title
        self.__description = description
        self.__usable = usable_before_invoking

    def key(self) -> str:
        return self.__key

    def is_list(self) -> bool:
        return self.__is_list

    def type(self) -> AbcType:
        return self.__type

    def title(self) -> I18n:
        return self.__title

    def description(self) -> I18n:
        return self.__description

    def is_usable_before_invoking(self) -> bool:
        return self.__usable
