"""The default AbcType implementation."""

from ..defs.node_type import AbcType, AbcTypeField
from ..util.message import I18n


class DefaultTypeField(AbcTypeField):
    """Default type implementation."""

    def __init__(
        self,
        *,
        key: str,
        type_val: AbcType,
        title: I18n,
        description: I18n,
        usable_before_invoking: bool,
    ) -> None:
        self.__key = key
        self.__type = type_val
        self.__title = title
        self.__description = description
        self.__usable = usable_before_invoking

    def key(self) -> str:
        return self.__key

    def type(self) -> AbcType:
        return self.__type

    def title(self) -> I18n:
        return self.__title

    def description(self) -> I18n:
        return self.__description

    def is_usable_before_invoking(self) -> bool:
        return self.__usable
