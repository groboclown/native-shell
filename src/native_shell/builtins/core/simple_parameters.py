"""Parameter types for common user script inputs."""

from ...defs.node_type import (
    BasicType,
    ListType,
    INTEGER_TYPE,
    NUMBER_TYPE,
    BOOLEAN_TYPE,
    STRING_TYPE,
    REFERENCE_TYPE,
    BASIC_TYPE_IDS,
)
from ...helpers import create_list_type
from ...util.message import i18n


def _mk_basic_list(basic_type: BasicType) -> ListType:
    name = f"list({basic_type.type_id()})"
    return create_list_type(
        source=("core", "param-types", name),
        type_id=name,
        title=i18n(name),
        description=i18n(name),
        type_checker=basic_type,
        min_count=0,
        max_count=None,
    )


INTEGER_LIST_TYPE = _mk_basic_list(INTEGER_TYPE)
NUMBER_LIST_TYPE = _mk_basic_list(NUMBER_TYPE)
BOOLEAN_LIST_TYPE = _mk_basic_list(BOOLEAN_TYPE)
STRING_LIST_TYPE = _mk_basic_list(STRING_TYPE)
REFERENCE_LIST_TYPE = _mk_basic_list(REFERENCE_TYPE)
UNTYPED_LIST_TYPE = create_list_type(
    source=("core", "param-types", "list(any)"),
    type_id="list(any)",
    title=i18n("list(any)"),
    description=i18n("list(any)"),
    type_checker=lambda t: True,
    min_count=0,
    max_count=None,
)
BASIC_LIST_TYPE = create_list_type(
    source=("core", "param-types", "list(basic)"),
    type_id="list(basic)",
    title=i18n("list(basic)"),
    description=i18n("list(basic)"),
    type_checker=lambda t: t.type_id() in BASIC_TYPE_IDS,
    min_count=0,
    max_count=None,
)
