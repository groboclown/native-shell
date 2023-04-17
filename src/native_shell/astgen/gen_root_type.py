"""Generate the root node type."""

from typing import List, Iterable
from .typed_tree import TypedTree
from ..defs.basic import mk_ref
from ..defs.add_ins import AddInTypeHandler, GeneratedCode, CodeTemplate
from ..defs.node_type import (
    AbcTypeParameter,
    AbcTypeField,
    AbcType,
    ConstructType,
)
from ..defs.syntax_tree import SyntaxNode
from ..helpers import DefaultTypeParameter, mk_param_code_ref
from ..util.message import UserMessage, i18n
from ..util.result import Result, Problem

# Explict name set rather than double import.
_ = i18n


def assign_root_node_type(tree: TypedTree) -> AddInTypeHandler:
    """Create the root node type and its handler.

    The root node has the special characteristic of dynamically building up a
    reference tree for general use.
    """
    # Every included value is both a parameter and a field.
    fields: List[AbcTypeField] = []
    params: List[AbcTypeParameter] = []

    for name, _node in tree.root.mapping().items():
        params.append(
            DefaultTypeParameter(
                key=str(name),
                title=i18n(f"parameter {name}"),
                description=i18n(f"parameter {name}"),
                required=True,
                type_checker=lambda x: True,
            )
        )

    type_val = ConstructType(
        source=tree.root.node_id.source,
        type_id="-root",
        title=_("parsed script"),
        description=_("parsed script"),
        parameters=params,
        fields=fields,
    )
    ret = RootNodeHandler(type_val)
    tree.assign_root_type(ret)
    return ret


class RootNodeHandler(AddInTypeHandler):
    """Dynamically constructed root node."""

    def __init__(self, type_val: AbcType):
        self.__type = type_val

    def type(self) -> AbcType:
        return self.__type

    def shared_code(self) -> Iterable[GeneratedCode]:
        return ()

    def instance_code(self, node: SyntaxNode) -> Result[Iterable[GeneratedCode]]:
        # The "main" key is supposed to exist.
        main = node.values().get("main")
        if main is None:
            return Result.as_error(
                Problem.as_validation(
                    node.source(),
                    UserMessage(_("'main' must be included as a top level element in the script")),
                )
            )
        return Result.as_value(
            (
                GeneratedCode(
                    ref=mk_ref(node.node_id()),
                    purpose="execute",
                    template=CodeTemplate((mk_param_code_ref(main, "execute"),)),
                ),
            )
        )
