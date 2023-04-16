"""Parameter types for common user script inputs."""

from ...defs.node_type import (
    BasicType, AbcListType,
    INTEGER_TYPE, NUMBER_TYPE, BOOLEAN_TYPE,
    STRING_TYPE, REFERENCE_TYPE,
)
from ...helpers import create_explicit_list_type
from ...util.message import i18n


def _mk_basic_list(basic_type: BasicType) -> AbcListType:
    name = f"list({basic_type.type_id()})"
    return create_explicit_list_type(
        source=("core", "param-types", name),
        type_id=name,
        title=i18n(name),
        description=i18n(name),
        item_type=basic_type,
        min_count=0,
        max_count=None,
    )


INTEGER_LIST_TYPE = _mk_basic_list(INTEGER_TYPE)
NUMBER_LIST_TYPE = _mk_basic_list(NUMBER_TYPE)
BOOLEAN_LIST_TYPE = _mk_basic_list(BOOLEAN_TYPE)
STRING_LIST_TYPE = _mk_basic_list(STRING_TYPE)
REFERENCE_LIST_TYPE = _mk_basic_list(REFERENCE_TYPE)
