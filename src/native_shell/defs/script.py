"""The parsed user script."""

from typing import Sequence, Tuple, Iterable, Dict, Mapping, Callable, Union, Optional
import datetime
from .add_ins import AddInTypeHandler, AddInMetaTypeHandler, AddIn
from .node_type import AbcType, AbcMetaType
from .syntax_tree import SyntaxNode
from .parse_tree import AbcParsedNode
from ..util.message import i18n as _
from ..util.message import UserMessage
from ..util.result import SourcePath, Result, Problem, ResultGen


class ScriptSource:
    """Where the script came from."""

    __slots__ = (
        "__source",
        "__hash",
        "__when",
    )

    def __init__(
        self,
        *,
        source: SourcePath,
        src_hash: str,
        when: datetime.datetime,
    ) -> None:
        self.__source = source
        self.__hash = src_hash
        self.__when = when

    @property
    def source(self) -> SourcePath:
        """The script source location."""
        return self.__source

    @property
    def source_hash(self) -> str:
        """The secure hash for the sources constructing the script."""
        return self.__hash

    @property
    def when(self) -> datetime.datetime:
        """The last modified time of the script."""
        return self.__when


class TypeHandlerStore:
    """Stores add-in type handlers and allows for easy reference.

    This should probably include a built-in List type handler to allow for field access.
    """

    __slots__ = ("__types",)

    def __init__(
        self,
        type_map: Mapping[str, AddInTypeHandler],
    ) -> None:
        self.__types = type_map

    def all(self) -> Iterable[AddInTypeHandler]:
        """Get all type handlers known."""
        return self.__types.values()

    def get(self, type_id: Union[str, AbcType, None]) -> Optional[AddInTypeHandler]:
        """Get the type handler with the given type name or type."""
        if type_id is None:
            return None
        if isinstance(type_id, AbcType):
            type_id = type_id.type_id()
        return self.__types.get(type_id)

    def include_only(self, types: Iterable[str]) -> "TypeHandlerStore":
        """Create a new store containing only the referenced types.
        Does not cause an error if a requested type is not in the known type handlers."""
        ret: Dict[str, AddInTypeHandler] = {}
        for key, val in self.__types.items():
            if key in types:
                ret[key] = val
        return TypeHandlerStore(ret)

    def add_dynamic(self, handler: AddInTypeHandler) -> None:
        """Add a dynamically generated type handler.  It can't conflict with
        any existing type handler."""
        if handler.type().type_id() in self.__types:
            raise ValueError(f"already registered type with id {handler.type().type_id()}")
        new_types = dict(self.__types)
        new_types[handler.type().type_id()] = handler
        self.__types = new_types


class HandlerStore:
    """Stores the add-in type and meta-type handlers and allows for easy reference."""

    __slots__ = ("__types", "__meta")

    def __init__(
        self,
        type_map: Mapping[str, AddInTypeHandler],
        meta_map: Mapping[str, AddInMetaTypeHandler],
    ) -> None:
        self.__types = TypeHandlerStore(type_map)
        self.__meta = meta_map

    @staticmethod
    def create(source: SourcePath, add_ins: Iterable[AddIn]) -> "Result[HandlerStore]":
        """Create a type handler store from the add-ins."""
        res = ResultGen()
        type_handlers: Dict[str, AddInTypeHandler] = {}
        meta_handlers: Dict[str, AddInMetaTypeHandler] = {}

        for add_in in add_ins:
            for ath in add_in.type_handlers():
                key = ath.type().type_id()
                if key in type_handlers or key in meta_handlers:
                    res.add(
                        Problem.as_validation(
                            source,
                            UserMessage(
                                _("duplicate type registration: '{key}'; found in {addin}"),
                                key=key,
                                addin=add_in.include_name(),
                            ),
                        )
                    )
                else:
                    type_handlers[key] = ath
            for amh in add_in.meta_types():
                key = amh.meta_type().type_id()
                if key in type_handlers or key in meta_handlers:
                    res.add(
                        Problem.as_validation(
                            source,
                            UserMessage(
                                _("duplicate type registration: '{key}'; found in {addin}"),
                                key=key,
                                addin=add_in.include_name(),
                            ),
                        )
                    )
                else:
                    meta_handlers[key] = amh

        return res.build(HandlerStore(type_handlers, meta_handlers))

    def get_type_handler(self, type_id: Union[str, AbcType, None]) -> Optional[AddInTypeHandler]:
        """Get the type handler with the given type name or type."""
        return self.__types.get(type_id)

    def get_meta_handler(
        self, meta_id: Union[str, AbcMetaType, None]
    ) -> Optional[AddInMetaTypeHandler]:
        """Get the type handler with the given type name or type."""
        if meta_id is None:
            return None
        if isinstance(meta_id, AbcMetaType):
            meta_id = meta_id.type_id()
        return self.__meta.get(meta_id)

    def has_type_handler(self, type_id: Union[str, AbcType, None]) -> bool:
        """Is this type handler known?"""
        return self.get_type_handler(type_id) is not None

    def has_meta_handler(self, type_id: Union[str, AbcMetaType, None]) -> bool:
        """Is this meta-type handler known?"""
        return self.get_meta_handler(type_id) is not None

    def as_type_handler_store(self) -> TypeHandlerStore:
        """Return the store for just the type handlers."""
        return self.__types


class InitialScript:
    """A pass at constructing the concrete script.  There may still be
    meta-type nodes and problems."""

    __slots__ = (
        "__source",
        "__name",
        "__version",
        "__bin_location",
        "__add_in_names",
        "__tree",
    )

    def __init__(
        self,
        *,
        source: ScriptSource,
        name: str,
        version: str,
        bin_location: str,
        add_in_names: Iterable[str],
        tree: AbcParsedNode,
    ) -> None:
        self.__source = source
        self.__name = name
        self.__version = version
        self.__bin_location = bin_location
        self.__add_in_names = tuple(add_in_names)
        self.__tree = tree

    @property
    def script_source(self) -> ScriptSource:
        """Where the script came from."""
        return self.__source

    @property
    def name(self) -> str:
        """Name of the script.  Will be used to construct the
        compiled command."""
        return self.__name

    @property
    def version(self) -> str:
        """Version of the script."""
        return self.__version

    @property
    def bin_location(self) -> str:
        """Binary output file location, for the makefile."""
        return self.__bin_location

    @property
    def add_in_names(self) -> Sequence[str]:
        """Add-ins requested by the script."""
        return self.__add_in_names

    @property
    def tree(self) -> AbcParsedNode:
        """The parsed, expanded syntax tree."""
        return self.__tree


class StagingScript:
    """A pass at constructing the concrete script.  There may still be
    meta-type nodes and problems."""

    __slots__ = (
        "__source",
        "__name",
        "__version",
        "__bin_location",
        "__add_ins",
        "__tree",
    )

    def __init__(
        self,
        *,
        source: ScriptSource,
        name: str,
        version: str,
        bin_location: str,
        add_ins: Iterable[AddIn],
        tree: AbcParsedNode,
    ) -> None:
        self.__source = source
        self.__name = name
        self.__version = version
        self.__bin_location = bin_location
        self.__add_ins = tuple(add_ins)
        self.__tree = tree

    @property
    def script_source(self) -> ScriptSource:
        """Where the script came from."""
        return self.__source

    @property
    def name(self) -> str:
        """Name of the script.  Will be used to construct the
        compiled command."""
        return self.__name

    @property
    def version(self) -> str:
        """Version of the script."""
        return self.__version

    @property
    def bin_location(self) -> str:
        """Binary output file location, for the makefile."""
        return self.__bin_location

    @property
    def add_ins(self) -> Sequence[AddIn]:
        """Add-ins used by the script."""
        return self.__add_ins

    @property
    def tree(self) -> AbcParsedNode:
        """The parsed, expanded syntax tree."""
        return self.__tree


class PreparedScript:
    """A user script that's been parsed into the concrete syntax tree.
    Contains everything necessary to construct the code."""

    __slots__ = (
        "__source",
        "__name",
        "__bin_location",
        "__version",
        "__type_handlers",
        "__tree",
    )

    def __init__(
        self,
        *,
        source: ScriptSource,
        name: str,
        version: str,
        bin_location: str,
        type_handlers: TypeHandlerStore,
        tree: SyntaxNode,
    ) -> None:
        self.__source = source
        self.__name = name
        self.__version = version
        self.__bin_location = bin_location
        self.__type_handlers = type_handlers
        self.__tree = tree

    @property
    def script_source(self) -> ScriptSource:
        """Where the script came from."""
        return self.__source

    @property
    def name(self) -> str:
        """Name of the script.  Will be used to construct the
        compiled command."""
        return self.__name

    @property
    def version(self) -> str:
        """Version of the script."""
        return self.__version

    @property
    def bin_location(self) -> str:
        """Binary output file location, for the makefile."""
        return self.__bin_location

    @property
    def type_handlers(self) -> TypeHandlerStore:
        """Add-ins type handlers used by the script."""
        return self.__type_handlers

    @property
    def tree(self) -> SyntaxNode:
        """The parsed, expanded syntax tree."""
        return self.__tree


# A ScriptParser transforms the user script into the staging pass.
ScriptParser = Callable[
    [Sequence[Tuple[ScriptSource, bytes]]],
    Result[InitialScript],
]
