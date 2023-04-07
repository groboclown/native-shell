"""Different kinds of files and how to assemble them."""

from typing import Mapping, Dict, Sequence, List, Callable
from ..defs.script import PreparedScript
from ..util.result import Result


class AssembledFile:
    """A file that is joined up with parts."""

    __slots__ = (
        "location",
        "content",
        "format_parts",
        "part_joiners",
    )

    def __init__(
        self,
        /,
        location: str,
        content: str,
        part_joiners: Mapping[str, Callable[[Sequence[str]], str]],
    ) -> None:
        self.location = location
        self.content = content
        self.part_joiners = part_joiners
        self.format_parts: Dict[str, List[str]] = {key: [] for key in part_joiners.keys()}

    def add_part(self, key: str, value: str) -> None:
        """Add the part value."""
        self.format_parts[key].append(value)


FileAssembler = Callable[[PreparedScript], Result[AssembledFile]]
