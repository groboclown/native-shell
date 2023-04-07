"""Generate the source files."""

from typing import Sequence, List
from .gen_files import AssembledFile, FileAssembler
from ..defs.script import PreparedScript
from ..util.result import Result, ResultGen


def generate_source_files(
    script: PreparedScript,
    files: Sequence[FileAssembler],
) -> Result[Sequence[AssembledFile]]:
    """Turn the script into source files, ready to be compiled."""
    res = ResultGen()
    ret: List[AssembledFile] = []
    for file_assembler in files:
        assembled_res = file_assembler(script)
        res.add(assembled_res)
        if assembled_res.is_valid:
            ret.append(assembled_res.required())
    return res.build(ret)
