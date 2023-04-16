"""A parsed script tree with typing."""

from typing import Set, Union, Optional
from ..defs.add_ins import AddInTypeHandler
from ..defs.parse_tree import (
    AbcParsedNode,
    ParsedParameterNode,
)
from ..defs.syntax_tree import (
    AbcTypeParameter,
    AbcTypeField,
    AbcTypeProperty,
    AbcType,
    ListType,
    BasicType,
)
from ..defs.script import TypeHandlerStore


class TypedTree:
    """A parsed node tree with types collected."""

    __slots__ = (
        "referenced_handlers",
        "handlers",
        "root",
    )

    def __init__(self, root: AbcParsedNode, handlers: TypeHandlerStore) -> None:
        if not isinstance(root, ParsedParameterNode):
            raise RuntimeError(f"root node must be a parameter node; found {root!r}")
        self.root: ParsedParameterNode = root
        self.handlers = handlers
        self.referenced_handlers: Set[str] = set()

    def mark_referenced(
        self,
        value: Union[AddInTypeHandler, AbcTypeParameter],
    ) -> None:
        """Mark the type as referenced."""
        if isinstance(value, AbcTypeParameter):
            param_type = value.type()
            if isinstance(param_type, AbcType):
                self.referenced_handlers.add(param_type.type_id())
        else:
            self.referenced_handlers.add(value.type().type_id())

    def get_handler(
        self,
        type_val: Union[BasicType, ListType, AbcType, AbcTypeProperty, None],
    ) -> Optional[AddInTypeHandler]:
        """Get the type handler for the type."""
        if not type_val:
            return None
        if isinstance(type_val, (AbcTypeParameter, AbcTypeField)):
            type_val_type = type_val.type()
            if isinstance(type_val_type, AbcType):
                return self.handlers.get(type_val_type)
            return None
        if isinstance(type_val, AbcType):
            return self.handlers.get(type_val)
        # Else it's a basic type; return
        return None

    def assign_root_type(self, handler: AddInTypeHandler) -> None:
        """The root node is special and has its type dynamically constructed."""
        if self.root.get_assigned_type() is not None:
            raise RuntimeError("Already assigned root type.")
        self.root.set_type(handler.type())
        self.handlers.add_dynamic(handler)
        self.mark_referenced(handler)
