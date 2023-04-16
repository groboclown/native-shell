"""The built-in basic types."""

from typing import Iterable, Mapping

from ...defs.add_ins import AddInTypeHandler, GeneratedCode
from ...defs.node_type import BasicType, BasicTypeId, AbcType
from ...defs.syntax_tree import SyntaxNode
from ...util.message import i18n as _
from ...util.result import Result


class BasicParameterHandler(AddInTypeHandler):
    """A simple parameter type.  Does not generate code."""

    def __init__(
        self,
        type_val: BasicType,
    ) -> None:
        self.type_val = type_val

    def type(self) -> AbcType:
        return self.type_val

    def shared_code(self) -> Iterable[GeneratedCode]:
        return ()

    def instance_code(
        self,
        node: SyntaxNode,
    ) -> Result[Iterable[GeneratedCode]]:
        return Result.as_value(())


INTEGER_TYPE = BasicType(
    source=("build-in", "integer"),
    type_id="integer",
    title=_("integer"),
    description=_("integer"),
)
INTEGER_TYPE_HANDLER = BasicParameterHandler(INTEGER_TYPE)

NUMBER_TYPE = BasicType(
    source=("built-in", "number"),
    type_id="number",
    title=_("number"),
    description=_("number"),
)
NUMBER_TYPE_HANDLER = BasicParameterHandler(NUMBER_TYPE)

BOOLEAN_TYPE = BasicType(
    source=("built-in", "boolean"),
    type_id="boolean",
    title=_("boolean"),
    description=_("boolean"),
)
BOOLEAN_TYPE_HANDLER = BasicParameterHandler(BOOLEAN_TYPE)

STRING_TYPE = BasicType(
    source=("built-in", "string"),
    type_id="string",
    title=_("string"),
    description=_("string"),
)
STRING_TYPE_HANDLER = BasicParameterHandler(STRING_TYPE)

REFERENCE_TYPE = BasicType(
    source=("built-in", "reference"),
    type_id="reference",
    title=_("reference"),
    description=_("reference"),
)
REFERENCE_TYPE_HANDLER = BasicParameterHandler(REFERENCE_TYPE)


BASIC_TYPES: Mapping[BasicTypeId, BasicType] = {
    "integer": INTEGER_TYPE,
    "number": NUMBER_TYPE,
    "boolean": BOOLEAN_TYPE,
    "string": STRING_TYPE,
    "reference": REFERENCE_TYPE,
}
