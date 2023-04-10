"""The old 'echo' command."""

from typing import Iterable, List, Union
from ...defs.add_ins import (
    AddInTypeHandler,
    GeneratedCode,
    CodeTemplate,
    CodeReference,
)
from ...defs.syntax_tree import AbcType, SyntaxNode
from ...helpers import (
    DefaultType,
    DefaultTypeParameter,
    DefaultTypeField,
    mk_field_ref,
    mk_var_name,
    mk_param_code_ref,
)
from ...util.message import i18n as _
from ...util.message import UserMessage
from ...util.result import Result, ResultGen, Problem


ECHO_TEXT_KEY = "text"
ECHO_TEXT = DefaultTypeParameter(
    key=ECHO_TEXT_KEY,
    is_list=True,
    type_val="string",
    title=_("echo text"),
    description=_("Send formatted text to an output."),
    required=True,
)

ECHO_STDOUT_KEY = "stdout"
ECHO_STDOUT = DefaultTypeParameter(
    key=ECHO_STDOUT_KEY,
    is_list=False,
    type_val="boolean",
    title=_("stdout"),
    description=_("Should the echo text be sent to stdout?  Defaults to false."),
    required=False,
)

ECHO_STDERR_KEY = "stderr"
ECHO_STDERR = DefaultTypeParameter(
    key=ECHO_STDERR_KEY,
    is_list=False,
    type_val="boolean",
    title=_("stderr"),
    description=_("Should the echo text be sent to stderr?  Defaults to false."),
    required=False,
)

ECHO_WRITE_KEY = "write to"
ECHO_WRITE = DefaultTypeParameter(
    key=ECHO_WRITE_KEY,
    is_list=False,
    type_val="string",
    title=_("write to a file"),
    description=_("The filename to send the echo to; defaults to no file output."),
    required=False,
)

ECHO_APPEND_KEY = "append to"
ECHO_APPEND = DefaultTypeParameter(
    key=ECHO_APPEND_KEY,
    is_list=False,
    type_val="string",
    title=_("append to a file"),
    description=_("The filename to send the echo to; defaults to no file output."),
    required=False,
)

ECHO_FILENO_KEY = "fileno"
ECHO_FILENO = DefaultTypeField(
    key=ECHO_FILENO_KEY,
    is_list=False,
    type_val="integer",
    title=_("file descriptor"),
    description=_("The file descriptor written to by the echo operation."),
    usable_before_invoking=False,
)

ECHO_ERROR_KEY = "err"
ECHO_ERROR = DefaultTypeField(
    key=ECHO_ERROR_KEY,
    is_list=False,
    type_val="integer",
    title=_("file descriptor"),
    description=_("The file descriptor written to by the echo operation."),
    usable_before_invoking=False,
)

ECHO_TYPE = DefaultType(
    source=("core", "echo"),
    type_id="core.echo",
    title=_("echo"),
    description=_("Send output to stdout, stderr, or a file."),
    parameters=(ECHO_TEXT, ECHO_STDOUT, ECHO_STDERR, ECHO_WRITE, ECHO_APPEND),
    fields=(ECHO_FILENO,),
)


class EchoCommand(AddInTypeHandler):
    """The 'echo' command."""

    def type(self) -> AbcType:
        return ECHO_TYPE

    def shared_code(self) -> Iterable[GeneratedCode]:
        # Does not generate shared code.
        return ()

    def instance_code(  # pylint:disable=too-many-locals
        self,
        node: SyntaxNode,
    ) -> Result[Iterable[GeneratedCode]]:
        res = ResultGen()
        fileno_ref = mk_field_ref(node, ECHO_FILENO)
        err_ref = mk_field_ref(node, ECHO_ERROR)
        ret: List[GeneratedCode] = [
            # Will always have a fileno field.
            GeneratedCode(
                ref=fileno_ref,
                purpose="define_field",
                template=CodeTemplate(
                    (
                        f"var {mk_var_name(fileno_ref)} *os.File",
                        f"var {mk_var_name(err_ref)} error",
                    )
                ),
            ),
            # ... which needs to import "os"
            GeneratedCode(
                ref=node.node_id(),
                purpose="import_as",
                template=CodeTemplate(
                    (
                        "os",
                        "errors",
                        "fmt",
                    )
                ),
            ),
        ]

        exec_parts: List[Union[CodeReference, str]] = []
        # Exactly one of stdout, stderr, write, append must be given.
        stdout = node.values().get(ECHO_STDOUT_KEY)
        stderr = node.values().get(ECHO_STDERR_KEY)
        write = node.values().get(ECHO_WRITE_KEY)
        append = node.values().get(ECHO_APPEND_KEY)
        indent = ""
        tail = ""

        if stdout:
            if stderr or write or append:
                res.add(
                    Problem.as_validation(
                        node.source(),
                        UserMessage(_("exactly one of stdout, stderr, write, append can be used.")),
                    )
                )
            exec_parts.append(f"{mk_var_name(fileno_ref)} = os.Stdout\n")
        if stderr:
            if stdout or write or append:
                res.add(
                    Problem.as_validation(
                        node.source(),
                        UserMessage(_("exactly one of stdout, stderr, write, append can be used.")),
                    )
                )
            exec_parts.append(f"{mk_var_name(fileno_ref)} = os.Stderr\n")
        if write:
            if stdout or stderr or append:
                res.add(
                    Problem.as_validation(
                        node.source(),
                        UserMessage(_("exactly one of stdout, stderr, write, append can be used.")),
                    )
                )
            exec_parts.extend(
                (
                    f"{mk_var_name(fileno_ref)}, {mk_var_name(err_ref)} = os.Create(",
                    mk_param_code_ref(write, "get_field_value"),
                    ")\n",
                    # Unlike standard Go, we don't do the error check immediately
                    # after.  Instead, that's used for job execution flow.
                    f"if {mk_var_name(err_ref)} == nil {{\n",
                    f"\tdefer {mk_var_name(fileno_ref)}.Close()\n",
                )
            )
            tail = "}"
            indent = "\t"
        if append:
            if stdout or stderr or write:
                res.add(
                    Problem.as_validation(
                        node.source(),
                        UserMessage(_("exactly one of stdout, stderr, write, append can be used.")),
                    )
                )
            raise RuntimeError("Still need to implement append")

        text = node.values().get(ECHO_TEXT_KEY)
        format_str = ""
        text_bits: List[Union[CodeReference, str]] = []
        if text and isinstance(text, SyntaxNode):
            # It's supposed to be a list.
            index = 0
            while True:
                bit = text.values().get(str(index))
                if not bit:
                    break
                if len(text_bits) > 0:
                    format_str += " "
                format_str += "%s"
                text_bits.append(", ")
                text_bits.append(mk_param_code_ref(bit, "get_field_value"))
                index += 1
        exec_parts.append(
            f"{indent}{mk_var_name(err_ref)} = fmt.Fprintf({mk_var_name(fileno_ref)}, "
            f'"{format_str}\\n"'
        )
        exec_parts.extend(text_bits)
        exec_parts.append(")\n")
        exec_parts.append(tail)

        return res.build(ret)


ECHO = EchoCommand()
