"""The parsed user script."""

from typing import Sequence, Tuple, Callable
import datetime
from .add_ins import AddInTypeHandler
from .syntax_tree import AbcSyntaxNode, AbcSyntaxBuildingNode
from ..util.result import SourcePath, Result


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


class StagingScript:
    """A pass at constructing the concrete script.  There may still be
    meta-type nodes and problems."""

    __slots__ = (
        "__source",
        "__name",
        "__version",
        "__add_ins",
        "__tree",
    )

    def __init__(
        self,
        *,
        source: ScriptSource,
        name: str,
        version: str,
        add_in_handlers: Sequence[AddInTypeHandler],
        tree: AbcSyntaxBuildingNode,
    ) -> None:
        self.__source = source
        self.__name = name
        self.__version = version
        self.__add_ins = tuple(add_in_handlers)
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
    def add_in_handlers(self) -> Sequence[AddInTypeHandler]:
        """Add-ins used by the script."""
        return self.__add_ins

    @property
    def tree(self) -> AbcSyntaxBuildingNode:
        """The parsed, expanded syntax tree."""
        return self.__tree


class PreparedScript:
    """A user script that's been parsed into the concrete syntax tree.
    Contains everything necessary to construct the code."""

    __slots__ = (
        "__source",
        "__name",
        "__version",
        "__add_ins",
        "__tree",
    )

    def __init__(
        self,
        *,
        source: ScriptSource,
        name: str,
        version: str,
        add_in_handlers: Sequence[AddInTypeHandler],
        tree: AbcSyntaxNode,
    ) -> None:
        self.__source = source
        self.__name = name
        self.__version = version
        self.__add_ins = tuple(add_in_handlers)
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
    def add_in_handlers(self) -> Sequence[AddInTypeHandler]:
        """Add-ins used by the script."""
        return self.__add_ins

    @property
    def tree(self) -> AbcSyntaxNode:
        """The parsed, expanded syntax tree."""
        return self.__tree


# A ScriptParser transforms the user script into the staging pass.
ScriptParser = Callable[
    [Sequence[Tuple[ScriptSource, bytes]]],
    Result[StagingScript],
]
