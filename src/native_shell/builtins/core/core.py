"""The core add-in."""

from .echo import ECHO
from .error_field import ERROR_FIELD
from .simple_field import (
    INT_FIELD,
    INT8_FIELD,
    UINT8_FIELD,
    INT16_FIELD,
    UINT16_FIELD,
    INT32_FIELD,
    UINT32_FIELD,
    INT64_FIELD,
    UINT64_FIELD,
    FLOAT_FIELD,
    FLOAT32_FIELD,
    FLOAT64_FIELD,
    BOOL_FIELD,
    STRING_FIELD,
)
from .os_file_field import OS_FILE_FIELD
from ...defs.add_ins import AddIn


CORE = AddIn(
    name="core",
    description="Core functionality",
    include_name="core",
    type_handlers=(
        ECHO,
        ERROR_FIELD,
        INT_FIELD,
        INT8_FIELD,
        UINT8_FIELD,
        INT16_FIELD,
        UINT16_FIELD,
        INT32_FIELD,
        UINT32_FIELD,
        INT64_FIELD,
        UINT64_FIELD,
        FLOAT_FIELD,
        FLOAT32_FIELD,
        FLOAT64_FIELD,
        BOOL_FIELD,
        STRING_FIELD,
        OS_FILE_FIELD,
    ),
    meta_types=(),
)
