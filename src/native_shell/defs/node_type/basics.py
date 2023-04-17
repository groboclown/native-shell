"""The built-in basic types."""

from typing import Mapping
from .defs import BasicType, BasicTypeId
from ...util.message import i18n as _


INTEGER_TYPE = BasicType(
    source=("build-in", "integer"),
    type_id="integer",
    title=_("integer"),
    description=_("integer"),
)

NUMBER_TYPE = BasicType(
    source=("built-in", "number"),
    type_id="number",
    title=_("number"),
    description=_("number"),
)

BOOLEAN_TYPE = BasicType(
    source=("built-in", "boolean"),
    type_id="boolean",
    title=_("boolean"),
    description=_("boolean"),
)

STRING_TYPE = BasicType(
    source=("built-in", "string"),
    type_id="string",
    title=_("string"),
    description=_("string"),
)

REFERENCE_TYPE = BasicType(
    source=("built-in", "reference"),
    type_id="reference",
    title=_("reference"),
    description=_("reference"),
)

BASIC_TYPES: Mapping[BasicTypeId, BasicType] = {
    "integer": INTEGER_TYPE,
    "number": NUMBER_TYPE,
    "boolean": BOOLEAN_TYPE,
    "string": STRING_TYPE,
    "reference": REFERENCE_TYPE,
}
