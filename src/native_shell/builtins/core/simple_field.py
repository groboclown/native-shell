"""A Go error.  By convention, these are the 'err' field of executable types."""

from typing import Iterable
from ...defs.add_ins import (
    AddInTypeHandler,
    GeneratedCode,
    CodeTemplate,
)
from ...defs.syntax_tree import AbcType, SyntaxNode
from ...helpers import (
    DefaultType,
    mk_var_name,
)
from ...util.message import i18n
from ...util.result import Result


class SimpleField(AddInTypeHandler):
    """A simple field type."""

    def __init__(self, go_type: str, type_val: AbcType) -> None:
        self.go_type = go_type
        self.type_val = type_val

    def type(self) -> AbcType:
        return self.type_val

    def shared_code(self) -> Iterable[GeneratedCode]:
        # Does not generate shared code.
        return ()

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
    return DefaultType(
        source=("core", "types", name),
        type_id=f"core.types.{name}",
        title=i18n(name),
        description=i18n(name),
        parameters=(),
        fields=(),
    )


INT_TYPE = _mk_type("int")
INT_FIELD = SimpleField("int", INT_TYPE)

UINT8_TYPE = _mk_type("uint8")
UINT8_FIELD = SimpleField("uint8", UINT8_TYPE)

INT8_TYPE = _mk_type("int8")
INT8_FIELD = SimpleField("int8", INT8_TYPE)

UINT16_TYPE = _mk_type("uint16")
UINT16_FIELD = SimpleField("uint16", UINT16_TYPE)

INT16_TYPE = _mk_type("int16")
INT16_FIELD = SimpleField("int16", INT16_TYPE)

UINT32_TYPE = _mk_type("uint32")
UINT32_FIELD = SimpleField("uint32", UINT32_TYPE)

INT32_TYPE = _mk_type("int32")
INT32_FIELD = SimpleField("int32", INT32_TYPE)

UINT64_TYPE = _mk_type("uint64")
UINT64_FIELD = SimpleField("uint64", UINT64_TYPE)

INT64_TYPE = _mk_type("int64")
INT64_FIELD = SimpleField("int64", INT64_TYPE)

FLOAT_TYPE = _mk_type("float")
FLOAT_FIELD = SimpleField("float", FLOAT_TYPE)

FLOAT32_TYPE = _mk_type("float32")
FLOAT32_FIELD = SimpleField("float32", FLOAT32_TYPE)

FLOAT64_TYPE = _mk_type("float64")
FLOAT64_FIELD = SimpleField("float64", FLOAT64_TYPE)

BOOL_TYPE = _mk_type("bool")
BOOL_FIELD = SimpleField("bool", BOOL_TYPE)

STRING_TYPE = _mk_type("string")
STRING_FIELD = SimpleField("string", STRING_TYPE)
