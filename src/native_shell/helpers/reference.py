"""Create references from other references."""

from typing import Sequence, Union, cast
from ..defs.basic import NodeReference, mk_ref
from ..defs.add_ins import CodeReference, CodeReferencePurpose
from ..defs.syntax_tree import SyntaxNode, TypeField, SyntaxParameter


def mk_field_ref(src: SyntaxNode, field: TypeField) -> NodeReference:
    """Create a reference to a field on a node."""
    return mk_ref([*cast(Sequence[str], src.node_id()), field.key()])


def mk_var_name(src: NodeReference) -> str:
    """Trun the reference into a Go variable name."""
    ret = ""
    as_upper = True
    for part in "_".join(src):
        assert isinstance(part, str)  # nosec  # for general typing
        if part.isalpha():
            if as_upper:
                part = part.upper()
                as_upper = False
            else:
                part = part.lower()
            ret += part
        elif part.isdigit():
            ret += part
            as_upper = True
        else:
            as_upper = True
    if not ret:
        raise RuntimeError(f"Failed to create variable name for {src}")
    if not ret[0].isalpha():
        ret = "A" + ret
    return ret


def mk_param_code_ref(
    param: SyntaxParameter,
    purpose: CodeReferencePurpose,
) -> Union[str, CodeReference]:
    """Create a code template reference to a parameter.  Basic types will be
    converted to Golang constant values."""
    if isinstance(param, SyntaxNode):
        return CodeReference(param.node_id(), purpose)
    if isinstance(param, (tuple, list)):
        # A reference.
        return CodeReference(cast(NodeReference, param), purpose)
    if isinstance(param, bool):
        return "true" if param else "false"
    if isinstance(param, int):
        return str(param)
    if isinstance(param, float):
        return str(param)
    if isinstance(param, str):
        ready = param.replace("\\", "\\\\").replace('"', '\\"')
        return f'"{ready}"'
    raise RuntimeError(f"No known conversion for {param!r}")
