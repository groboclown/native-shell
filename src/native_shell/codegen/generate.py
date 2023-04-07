"""Generate the source files."""

from typing import Sequence, List
from .gen_files import AssembledFile, FileAssembler
from ..defs.script import PreparedScript
from ..util.result import Result, ResultGen


# TODO this needs to be rebuilt from the ground up.
#   It will need to generate all the code pieces for all the syntax nodes,
#   then get the root "main" parameter and build that code.  This involves
#   filling in all the references and making sure there's no cycles.
#   Finally, the files need to be written.  There should just be one main.go
#   file and then simple templatized versions of standard go support files
#   Creating the go.mod file will require looking at actually loaded add-in
#   type handlers and extracting the imports.


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
