"""The default AbcType implementation."""

from typing import Callable, Optional
from ..defs.node_type import AbcType, AbcListType
from ..util.message import I18n
from ..util.result import SourcePath


class DefaultListType(AbcListType):
    """Default list type implementation."""

    def __init__(
            self,
            *,
            source: SourcePath,
            type_id: str,
            title: I18n,
            description: I18n,
            item_checker: Callable[[AbcType], bool],
            min_count: int = 0,
            max_count: Optional[int] = None,
    ) -> None:
        AbcListType.__init__(
            self,

        )
        self.__source = source
        self.__type_id = type_id
        self.__title = title
        self.__description = description
        self.__item_checker = item_checker
        self.__min_size = min_count
        self.__max_size = max_count

    def source(self) -> SourcePath:
        return self.__source

    def type_id(self) -> str:
        return self.__type_id

    def title(self) -> I18n:
        return self.__title

    def description(self) -> I18n:
        return self.__description

    def get_minimum_count(self) -> int:
        return self.__min_size

    def get_maximum_count(self) -> Optional[int]:
        return self.__max_size

    def is_type_allowed(self, other: AbcType) -> bool:
        return self.__item_checker(other)

    def __repr__(self) -> str:
        return f"ListType({self.__title})"


def create_explicit_list_type(
        *,
        source: SourcePath,
        type_id: str,
        title: I18n,
        description: I18n,
        item_type: AbcType,
        min_count: int = 0,
        max_count: Optional[int] = None,
) -> AbcListType:
    """Create a list type that only takes one explicit type of value."""
    return DefaultListType(
        source=source,
        type_id=type_id,
        title=title,
        description=description,
        item_checker=lambda t: t is item_type,
        min_count=min_count,
        max_count=max_count,
    )
