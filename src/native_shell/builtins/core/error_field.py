"""A Go error.  By convention, these are the 'err' field of executable types."""

from typing import Iterable
from ...defs.add_ins import (
    AddInTypeHandler,
    GeneratedCode,
    CodeTemplate,
)
from ...defs.basic import mk_ref
from ...defs.syntax_tree import AbcType, SyntaxNode
from ...helpers import (
    DefaultType,
    mk_var_name,
)
from ...util.message import i18n as _
from ...util.result import Result


ERROR_TYPE = DefaultType(
    source=("core", "types", "Error"),
    type_id="core.types.error",
    title=_("Error"),
    description=_("An error field."),
    parameters=(),
    fields=(),
)


class ErrorField(AddInTypeHandler):
    """The standard 'error' field type."""

    def type(self) -> AbcType:
        return ERROR_TYPE

    def shared_code(self) -> Iterable[GeneratedCode]:
        # This requires importing the errors type.
        return (
            GeneratedCode(
                ref=mk_ref([str(p) for p in ERROR_TYPE.source()]),
                purpose="import_as",
                template=CodeTemplate(("errors",)),
            ),
        )

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
                    template=CodeTemplate((f"var {var_name} error\n",)),
                ),
                GeneratedCode(
                    ref=node.node_id(),
                    purpose="get_field_value",
                    template=CodeTemplate((var_name,)),
                ),
            )
        )


ERROR_FIELD = ErrorField()
