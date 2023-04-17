"""Field types that relate to basic Go types."""

from typing import Iterable
from ...defs.add_ins import (
    AddInTypeHandler,
    GeneratedCode,
    CodeTemplate,
)
from ...defs.basic import mk_ref
from ...defs.node_type import AbcType, ConstructType
from ...defs.syntax_tree import SyntaxNode
from ...helpers import (
    mk_var_name,
)
from ...util.message import i18n
from ...util.result import Result


class SimpleFieldHandler(AddInTypeHandler):
    """A simple field type."""

    def __init__(
        self,
        go_type: str,
        type_val: AbcType,
        imports: Iterable[str] = (),
    ) -> None:
        self.go_type = go_type
        self.type_val = type_val
        self.import_templates: Iterable[GeneratedCode]
        if imports:
            self.import_templates = (
                GeneratedCode(
                    ref=mk_ref([str(p) for p in type_val.source()]),
                    purpose="import_as",
                    template=CodeTemplate(tuple(imports)),
                ),
            )
        else:
            self.import_templates = ()

    def type(self) -> AbcType:
        return self.type_val

    def shared_code(self) -> Iterable[GeneratedCode]:
        return self.import_templates

    def instance_code(  # pylint:disable=too-many-locals
        self,
        node: SyntaxNode,
    ) -> Result[Iterable[GeneratedCode]]:
        var_name = mk_var_name(node.node_id())
        return Result.as_value(
            (
                GeneratedCode(
                    ref=node.node_id(),
                    purpose="define_field",
                    template=CodeTemplate((f"var {var_name} {self.go_type}\n",)),
                ),
                GeneratedCode(
                    ref=node.node_id(),
                    purpose="get_field_value",
                    template=CodeTemplate((var_name,)),
                ),
            )
        )


def _mk_type(name: str) -> AbcType:
    # These simple types explicitly relate to Golang structures,
    # not user-passed values.  As such, they are turned into
    # parameter types with no fields or .
    return ConstructType(
        source=("core", "field-types", name),
        type_id=f"core.types.{name}",
        title=i18n(name),
        description=i18n(name),
        parameters=(),
        fields=(),
    )


INT_FIELD_TYPE = _mk_type("int")
INT_FIELD_HANDLER = SimpleFieldHandler("int", INT_FIELD_TYPE)

UINT8_FIELD_TYPE = _mk_type("uint8")
UINT8_FIELD_HANDLER = SimpleFieldHandler("uint8", UINT8_FIELD_TYPE)

INT8_FIELD_TYPE = _mk_type("int8")
INT8_FIELD_HANDLER = SimpleFieldHandler("int8", INT8_FIELD_TYPE)

UINT16_FIELD_TYPE = _mk_type("uint16")
UINT16_FIELD_HANDLER = SimpleFieldHandler("uint16", UINT16_FIELD_TYPE)

INT16_FIELD_TYPE = _mk_type("int16")
INT16_FIELD_HANDLER = SimpleFieldHandler("int16", INT16_FIELD_TYPE)

UINT32_FIELD_TYPE = _mk_type("uint32")
UINT32_FIELD_HANDLER = SimpleFieldHandler("uint32", UINT32_FIELD_TYPE)

INT32_FIELD_TYPE = _mk_type("int32")
INT32_FIELD_HANDLER = SimpleFieldHandler("int32", INT32_FIELD_TYPE)

UINT64_FIELD_TYPE = _mk_type("uint64")
UINT64_FIELD_HANDLER = SimpleFieldHandler("uint64", UINT64_FIELD_TYPE)

INT64_FIELD_TYPE = _mk_type("int64")
INT64_FIELD = SimpleFieldHandler("int64", INT64_FIELD_TYPE)

FLOAT_FIELD_TYPE = _mk_type("float")
FLOAT_FIELD_HANDLER = SimpleFieldHandler("float", FLOAT_FIELD_TYPE)

FLOAT32_FIELD_TYPE = _mk_type("float32")
FLOAT32_FIELD_HANDLER = SimpleFieldHandler("float32", FLOAT32_FIELD_TYPE)

FLOAT64_FIELD_TYPE = _mk_type("float64")
FLOAT64_FIELD_HANDLER = SimpleFieldHandler("float64", FLOAT64_FIELD_TYPE)

BOOL_FIELD_TYPE = _mk_type("bool")
BOOL_FIELD_HANDLER = SimpleFieldHandler("bool", BOOL_FIELD_TYPE)

STRING_FIELD_TYPE = _mk_type("string")
STRING_FIELD_HANDLER = SimpleFieldHandler("string", STRING_FIELD_TYPE)

ERROR_FIELD_TYPE = _mk_type("error")
ERROR_FIELD_HANDLER = SimpleFieldHandler("error", ERROR_FIELD_TYPE)

OS_FILE_FIELD_TYPE = _mk_type("file")
OS_FILE_FIELD_HANDLER = SimpleFieldHandler("*os.File", OS_FILE_FIELD_TYPE, ("os",))
