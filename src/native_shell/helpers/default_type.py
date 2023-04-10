"""The default AbcType implementation."""

from typing import Iterable, Sequence
from ..defs.syntax_tree import AbcType, TypeParameter, TypeField
from ..util.message import I18n
from ..util.result import SourcePath


class DefaultType(AbcType):
    """Default type implementation."""

    def __init__(
        self,
        *,
        source: SourcePath,
        type_id: str,
        title: I18n,
        description: I18n,
        parameters: Iterable[TypeParameter],
        fields: Iterable[TypeField],
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

    def parameters(self) -> Sequence[TypeParameter]:
        return self.__parameters

    def fields(self) -> Sequence[TypeField]:
        return self.__fields
