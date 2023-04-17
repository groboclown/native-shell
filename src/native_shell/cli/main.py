"""CLI entrypoint."""

from typing import Sequence
import os
import argparse
import datetime
import hashlib
from ..defs.script import ScriptSource
from ..script_parser.v1 import parse_v1
from ..addin_loader import load_add_ins
from ..astgen import generate_prepared_script
from ..codegen import assemble_code


def cli_main(args: Sequence[str]) -> int:
    """Called from the __main__."""
    parser = argparse.ArgumentParser(
        prog="native_shell",
        description="Converts a shell-like script into a Golang project.",
    )
    parser.add_argument(
        "--out",
        dest="out_dir",
        action="store",
        help="The directory to store the generated script; defaults to the current directory.",
    )
    parser.add_argument(
        "scriptfile",
        help="The script file to transpile",
    )

    parsed = parser.parse_args(args[1:])
    out_dir = parsed.out_dir or os.path.curdir
    script_file = parsed.scriptfile

    if not os.path.isfile(script_file):
        print(f"ERROR: no such file {script_file}")
        return 1
    if not os.path.isdir(out_dir):
        os.makedirs(out_dir, exist_ok=True)
        if not os.path.isdir(out_dir):
            print(f"ERROR: could not create directory {out_dir}")
            return 1

    with open(script_file, "rb") as fis:
        contents = fis.read()

    hash_func = hashlib.new("sha256")
    hash_func.update(contents)

    res = (
        parse_v1(
            (
                (
                    ScriptSource(
                        source=(script_file,),
                        src_hash=hash_func.hexdigest(),
                        when=datetime.datetime.fromtimestamp(os.path.getmtime(script_file)),
                    ),
                    contents,
                ),
            )
        )
        .map_result(load_add_ins)
        .map_result(lambda script: generate_prepared_script(script, 10))
        .map_result(assemble_code)
    )
    for problem in res.problems:
        print(str(problem))
    if res.is_not_valid:
        return 2

    assembled = res.required()
    with open(os.path.join(out_dir, "Makefile"), "w", encoding="UTF-8") as fos:
        fos.write(assembled.makefile)
    with open(os.path.join(out_dir, "go.mod"), "w", encoding="UTF-8") as fos:
        fos.write(assembled.go_mod)
    with open(os.path.join(out_dir, "main.go"), "w", encoding="UTF-8") as fos:
        fos.write(assembled.main_go)

    return 0
