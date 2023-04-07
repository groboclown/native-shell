"""Different kinds of files and how to assemble them."""

from typing import Mapping, Dict, Sequence, List, Callable
from ..defs.script import PreparedScript
from ..util.result import Result, ResultGen


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
        part_joiners: Mapping[str, Callable[[Sequence[str]], Result[str]]],
    ) -> None:
        self.location = location
        self.content = content
        self.part_joiners = part_joiners
        self.format_parts: Dict[str, List[str]] = {key: [] for key in part_joiners.keys()}

    def add_part(self, key: str, value: str) -> None:
        """Add the part value."""
        self.format_parts[key].append(value)

    def generate(self) -> Result[str]:
        """Create the assembled file's contents."""
        res = ResultGen()
        val = self.content.format(
            **{
                k: res.include(j(self.format_parts.get(k, ())), "")
                for k, j in self.part_joiners.items()
            }
        )
        return res.build(val)


FileAssembler = Callable[[PreparedScript], Result[AssembledFile]]
