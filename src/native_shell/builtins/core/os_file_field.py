"""A Go error.  By convention, these are the 'err' field of executable types."""

from typing import Iterable
from ...defs.basic import mk_ref
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
from ...util.message import i18n as _
from ...util.result import Result


OS_FILE_TYPE = DefaultType(
    source=("core", "types", "os-file"),
    type_id="core.types.os-file",
    title=_("os.File pointer"),
    description=_("*os.File"),
    parameters=(),
    fields=(),
)


class OsFileField(AddInTypeHandler):
    """A pointer to an os.File."""

    def type(self) -> AbcType:
        return OS_FILE_TYPE

    def shared_code(self) -> Iterable[GeneratedCode]:
        # Requires importing os
        return (
            GeneratedCode(
                ref=mk_ref([str(p) for p in OS_FILE_TYPE.source()]),
                purpose="import_as",
                template=CodeTemplate(("os",)),
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
                    template=CodeTemplate((f"var {var_name} *os.File\n",)),
                ),
                GeneratedCode(
                    ref=node.node_id(),
                    purpose="get_field_value",
                    template=CodeTemplate((var_name,)),
                ),
            )
        )


OS_FILE_FIELD = OsFileField()
