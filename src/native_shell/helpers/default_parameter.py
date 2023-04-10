"""The default AbcType implementation."""

from ..defs.syntax_tree import AbcType, TypeParameter, BasicType
from ..util.message import I18n


class DefaultTypeParameter(TypeParameter):
    """Default type implementation."""

    def __init__(
        self,
        *,
        key: str,
        is_list: bool,
        type_val: BasicType | AbcType,
        title: I18n,
        description: I18n,
        required: bool,
    ) -> None:
        self.__key = key
        self.__is_list = is_list
        self.__type = type_val
        self.__title = title
        self.__description = description
        self.__required = required

    def key(self) -> str:
        return self.__key

    def is_list(self) -> bool:
        return self.__is_list

    def type(self) -> BasicType | AbcType:
        return self.__type

    def title(self) -> I18n:
        return self.__title

    def description(self) -> I18n:
        return self.__description

    def is_required(self) -> bool:
        return self.__required
