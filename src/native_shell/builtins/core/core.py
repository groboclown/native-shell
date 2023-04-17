"""The core add-in."""

from .echo import ECHO
from .simple_field import (
    INT_FIELD_HANDLER,
    INT8_FIELD_HANDLER,
    UINT8_FIELD_HANDLER,
    INT16_FIELD_HANDLER,
    UINT16_FIELD_HANDLER,
    INT32_FIELD_HANDLER,
    UINT32_FIELD_HANDLER,
    INT64_FIELD,
    UINT64_FIELD_HANDLER,
    FLOAT_FIELD_HANDLER,
    FLOAT32_FIELD_HANDLER,
    FLOAT64_FIELD_HANDLER,
    BOOL_FIELD_HANDLER,
    STRING_FIELD_HANDLER,
    ERROR_FIELD_HANDLER,
    OS_FILE_FIELD_HANDLER,
)
from .sequential import SEQUENTIAL
from ...defs.add_ins import AddIn


CORE = AddIn(
    name="core",
    description="Core functionality",
    include_name="core",
    type_handlers=(
        ECHO,
        SEQUENTIAL,
        ERROR_FIELD_HANDLER,
        INT_FIELD_HANDLER,
        INT8_FIELD_HANDLER,
        UINT8_FIELD_HANDLER,
        INT16_FIELD_HANDLER,
        UINT16_FIELD_HANDLER,
        INT32_FIELD_HANDLER,
        UINT32_FIELD_HANDLER,
        INT64_FIELD,
        UINT64_FIELD_HANDLER,
        FLOAT_FIELD_HANDLER,
        FLOAT32_FIELD_HANDLER,
        FLOAT64_FIELD_HANDLER,
        BOOL_FIELD_HANDLER,
        STRING_FIELD_HANDLER,
        OS_FILE_FIELD_HANDLER,
    ),
    meta_types=(),
)
