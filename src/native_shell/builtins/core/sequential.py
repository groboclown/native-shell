"""Execute operations in sequence."""

from typing import Iterable, Sequence, List, Union, Optional
from .simple_field import ERROR_FIELD_TYPE
from .consts import ERROR_FIELD_KEY
from ...defs.add_ins import AddInTypeHandler, GeneratedCode, CodeReference, CodeTemplate
from ...defs.node_type import AbcType, ConstructType, BOOLEAN_TYPE
from ...defs.syntax_tree import SyntaxNode, SyntaxParameter
from ...helpers import (
    DefaultTypeField,
    DefaultTypeParameter,
    create_delayed_list_type_parameter,
    create_explicit_type_parameter,
    mk_var_name,
    mk_field_ref,
    mk_param_code_ref,
)
from ...helpers.list_types import get_ordered_children
from ...util.message import i18n as _
from ...util.result import Result, ResultGen, Problem


SEQUENTIAL_RUN_KEY = "run"
SEQUENTIAL_RUN = create_delayed_list_type_parameter(
    key=SEQUENTIAL_RUN_KEY,
    title=_("run commands"),
    description=_("the commands to run, in order"),
    required=True,
)

SEQUENTIAL_ON_ERROR_KEY = "on error"
SEQUENTIAL_ON_ERROR = DefaultTypeParameter(
    key=SEQUENTIAL_ON_ERROR_KEY,
    title=_("on error"),
    description=_("command to run when a run command encounters an error"),
    required=False,
    type_checker=lambda t: isinstance(t, ConstructType),
)

SEQUENTIAL_REQUIRE_ALL_SUCCESS_KEY = "require all success"
SEQUENTIAL_REQUIRE_ALL_SUCCESS = create_explicit_type_parameter(
    key=SEQUENTIAL_REQUIRE_ALL_SUCCESS_KEY,
    title=_("require all success"),
    description=_("if true, require all commands run to succeed"),
    required=False,
    type_val=BOOLEAN_TYPE,
)

SEQUENTIAL_ERROR_FUNCTION_TYPE = ConstructType(
    source=("core", "sequence", "error-function"),
    type_id="core.sequence.error-function",
    title=_("sequence error function"),
    description=_("sequence error function"),
    fields=(),
    parameters=(),
)
SEQUENTIAL_ERROR_FUNC_KEY = "error-call"
SEQUENTIAL_ERROR_FUNC = DefaultTypeField(
    key=SEQUENTIAL_ERROR_FUNC_KEY,
    type_val=SEQUENTIAL_ERROR_FUNCTION_TYPE,
    title=_("command error call"),
    description=_("The error code called after error."),
    usable_before_invoking=True,
)

SEQUENTIAL_ERROR_KEY = ERROR_FIELD_KEY
SEQUENTIAL_ERROR = DefaultTypeField(
    key=SEQUENTIAL_ERROR_KEY,
    type_val=ERROR_FIELD_TYPE,
    title=_("command error"),
    description=_("The error state after execution."),
    usable_before_invoking=False,
)

SEQUENTIAL_TYPE = ConstructType(
    source=("core", "run"),
    type_id="core.run",
    title=_("run"),
    description=_("run commands in sequence"),
    parameters=(SEQUENTIAL_RUN, SEQUENTIAL_REQUIRE_ALL_SUCCESS, SEQUENTIAL_ON_ERROR),
    fields=(SEQUENTIAL_ERROR,),
)


class SequentialCommand(AddInTypeHandler):
    """Run other commands in a sequence.  A common error handler is executed if any
    of the commands' err field is nil after it completes."""

    def type(self) -> AbcType:
        """The type representation for this handler."""
        return SEQUENTIAL_TYPE

    def shared_code(self) -> Iterable[GeneratedCode]:
        """All the code that is required to include in the source
        if this type is used.  The code must be read-only and stateless.
        The generated code's identity must be in the form, by convention:
        "['static', (type id), (code id)]"
        """
        # All the child run nodes should have their shared code generated.
        return ()

    def instance_code(self, node: SyntaxNode) -> Result[Iterable[GeneratedCode]]:
        """Constructs the code templates that this specific node in the tree
        needs to run.  If it includes static code, then it must not conflict with
        code returned by ``shared_code``."""
        ret: List[GeneratedCode] = []
        res = ResultGen()
        err_field = _get_error_field(node)
        if err_field is None or not isinstance(err_field, SyntaxNode):
            raise RuntimeError("bad error field")

        run_code_node = node.values().get(SEQUENTIAL_RUN_KEY)
        if not run_code_node or not isinstance(run_code_node, SyntaxNode):
            # Nothing to do.
            res.add(
                Problem.as_validation(
                    node.source(),
                    _("Must supply {key} as a list of runnable commands"),
                    key=SEQUENTIAL_RUN_KEY,
                )
            )
            return res.build(())

        on_err_call = _gen_error_func(node, ret)

        calls: List[SyntaxParameter] = []
        if isinstance(run_code_node.node_type(), ConstructType):
            # Type checking shouldn't make this a thing, but here it is.
            calls.append(run_code_node)
        else:
            # Should be a list type.
            calls.extend(get_ordered_children(run_code_node))

        run_code: List[Union[CodeReference, str]] = []
        for call in calls:
            if not isinstance(call, SyntaxNode):
                continue
            run_code.append("\n")
            run_code.append(mk_param_code_ref(call, "execute"))
            call_err = _get_error_field(call)
            if call_err:
                run_code.extend(
                    (
                        f"\n{mk_var_name(err_field.node_id())} = ",
                        mk_param_code_ref(call_err, "get_field_value"),
                        "\n",
                    )
                )
                if on_err_call:
                    run_code.extend(
                        ("\nif ", mk_param_code_ref(call_err, "get_field_value"), " != nil {\n")
                    )
                    run_code.extend(on_err_call)
                    run_code.append("\n}\n")

        ret.append(
            GeneratedCode(
                ref=node.node_id(),
                purpose="execute",
                template=CodeTemplate(run_code),
            )
        )
        return res.build(ret)


SEQUENTIAL = SequentialCommand()


def _gen_error_func(
    node: SyntaxNode,
    code: List[GeneratedCode],
) -> Sequence[Union[CodeReference, str]]:
    on_err = node.values().get(SEQUENTIAL_ON_ERROR_KEY)
    if not on_err:
        # No on-error code to call
        return ()

    # Add the call as the function.
    func_ref = mk_field_ref(node, SEQUENTIAL_ERROR_FUNC)
    func_def: List[Union[CodeReference, str]] = [
        f"\nfunc {mk_var_name(func_ref)}() {{\n",
        mk_param_code_ref(on_err, "execute"),
        "}\n",
    ]
    code.append(
        GeneratedCode(
            ref=func_ref,
            purpose="define_field",
            template=CodeTemplate(func_def),
        )
    )
    return f"{mk_var_name(func_ref)}()\n"


def _get_error_field(node: SyntaxParameter) -> Optional[SyntaxNode]:
    if not isinstance(node, SyntaxNode):
        return None
    err = node.values().get(ERROR_FIELD_KEY)
    if err and isinstance(err, SyntaxNode) and err.node_type() is ERROR_FIELD_TYPE:
        return err
    return None
