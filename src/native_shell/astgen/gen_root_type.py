"""Generate the root node type."""

from typing import List, Iterable
from .typed_tree import TypedTree
from ..defs.basic import mk_ref
from ..defs.add_ins import AddInTypeHandler, GeneratedCode, CodeTemplate
from ..defs.parse_tree import (
    ParsedListNode,
)
from ..defs.syntax_tree import (
    SyntaxNode,
    TypeParameter,
    TypeField,
    AbcType,
    LIST_TYPE_NAME,
)
from ..helpers import DefaultType, DefaultTypeParameter, mk_param_code_ref
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
    fields: List[TypeField] = []
    params: List[TypeParameter] = []

    index = 0
    for name, node in tree.root.mapping().items():
        node_type = node.get_assigned_type()
        if node_type is None or node_type == LIST_TYPE_NAME:
            # Ignore
            # ... list type at top level should generate an error.
            continue
        params.append(
            DefaultTypeParameter(
                key=str(name),
                is_list=isinstance(node, ParsedListNode),
                type_val=node_type,
                title=i18n(f"parameter {index}"),
                description=i18n(f"parameter {index}"),
                required=True,
            )
        )

    type_val = DefaultType(
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
