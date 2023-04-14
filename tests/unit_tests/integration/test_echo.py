"""Test an integration that executes a single echo."""

import unittest
import datetime
from helpers.parsed import mk_parameter, mk_list, mk_simple
from native_shell.defs.script import StagingScript, ScriptSource
from native_shell.astgen import generate_prepared_script
from native_shell.codegen import assemble_code
from native_shell.builtins.core import CORE
from native_shell.builtins.core.echo import ECHO


class JustEchoIntegrationTest(unittest.TestCase):
    """Test running just an echo."""

    def test_run(self) -> None:
        """Test the execution."""
        staging = mk_script()
        prepared_res = generate_prepared_script(staging, 1)
        self.assertEqual(
            [],
            [repr(p) for p in prepared_res.problems],
        )
        prepared = prepared_res.required()
        assembled_res = assemble_code(prepared)
        self.assertEqual(
            [],
            [repr(p) for p in assembled_res.problems],
        )
        assembled = assembled_res.required()
        self.assertEqual(
            """
module just-echo

go 1.20

""",
            assembled.go_mod,
        )
        self.assertEqual(
            """.PHONY: build clean

build:
\tmkdir -p bin
\tgo fmt ./...
\tgo build -o bin/just-echo .

clean:
\ttest -f bin/just-echo && rm bin/just-echo

""",
            assembled.makefile,
        )
        self.assertEqual(
            """
package main

import (
\t"fmt"
\t"os"
)

// ('main', 'err')
var MainErr error
// ('main', 'fileno')
var MainFileno *os.File


func main() {
\t// ()
MainFileno = os.Stdout
_, MainErr = fmt.Fprintf(MainFileno, "%s\\n", "Hello, world!")


}

""",
            assembled.main_go,
        )


def mk_script() -> StagingScript:
    """Create the simple script."""
    root = mk_parameter(
        ["test-script"],
        "",  # root node must have empty type id
        main=mk_parameter(
            ["test-script", "main"],
            ECHO.type().type_id(),
            text=mk_list(
                ["test-script", "main", "text"],
                mk_simple(["test-script", "main", "text", "0"], "Hello, world!"),
            ),
            stdout=mk_simple(["main", "stdout"], True),
        ),
    )
    return StagingScript(
        source=ScriptSource(source=("test",), src_hash="??", when=datetime.datetime.now()),
        name="just-echo",
        version="1.0",
        bin_location="bin/just-echo",
        add_ins=(CORE,),
        tree=root,
    )
