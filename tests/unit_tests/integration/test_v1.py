"""Full integration test with the v1 parser."""

import unittest
import datetime
from native_shell.addin_loader import load_add_ins
from native_shell.astgen import generate_prepared_script
from native_shell.codegen import assemble_code
from native_shell.defs.script import ScriptSource
from native_shell.script_parser.v1 import parse_v1


class V1ScriptIntegrationTest(unittest.TestCase):
    """Test running with the v1 script parser."""

    def test_run(self) -> None:
        """Test the execution."""

        res = (
            parse_v1(
                (
                    (
                        ScriptSource(
                            source=("test-v1.yaml",),
                            src_hash="???",
                            when=datetime.datetime.now(),
                        ),
                        SCRIPT_1,
                    ),
                )
            )
            .map_result(load_add_ins)
            .map_result(lambda script: generate_prepared_script(script, 10))
            .map_result(assemble_code)
        )
        self.assertEqual(
            [],
            [repr(p) for p in res.problems],
        )


SCRIPT_1 = b"""

name: test-echo
version: 1

require-libs:
  - core

main:
  as: core.echo
  with:
    text:
      as-list: string
      items:
        - Hello,
        - world!
    stdout:
      as: boolean
      value: true

"""
