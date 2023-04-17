"""The default AbcType implementation."""

from typing import Callable, Union, Optional
from .default_parameter import DefaultTypeParameter, create_explicit_type_parameter
from ..defs.node_type import AbcType, ListType, AbcTypeParameter
from ..util.message import i18n as _
from ..util.message import I18n
from ..util.result import SourcePath


LIST_TYPE_KEY = "items"
LIST_TYPE_ITEM_TITLE = _("items")
LIST_TYPE_ITEM_DESCRIPTION = _("items contained in the list")


def create_list_type_item_parameter(
    type_checker: Union[AbcType, Callable[[AbcType], bool]],
) -> AbcTypeParameter:
    """Create a list type's item parameter with a checked type - either it must
    match the given type, or the callable must return True."""
    if isinstance(type_checker, AbcType):
        return create_explicit_type_parameter(
            key=LIST_TYPE_KEY,
            title=LIST_TYPE_ITEM_TITLE,
            description=LIST_TYPE_ITEM_DESCRIPTION,
            required=False,
            type_val=type_checker,
        )

    return DefaultTypeParameter(
        key=LIST_TYPE_KEY,
        title=LIST_TYPE_ITEM_TITLE,
        description=LIST_TYPE_ITEM_DESCRIPTION,
        required=False,
        type_checker=type_checker,
    )


def create_list_type(
    *,
    source: SourcePath,
    type_id: str,
    title: I18n,
    description: I18n,
    type_checker: Union[AbcType, Callable[[AbcType], bool]],
    min_count: int = 0,
    max_count: Optional[int] = None,
) -> ListType:
    """Create a list type that only takes one explicit type of value."""
    return ListType(
        source=source,
        type_id=type_id,
        title=title,
        description=description,
        items=create_list_type_item_parameter(type_checker),
        minimum_count=min_count,
        maximum_count=max_count,
    )
