"""Test the module."""

import unittest
import datetime
from native_shell.script_parser import v1
from native_shell.defs.script import ScriptSource


class TestV1(unittest.TestCase):
    """Test the parser."""

    def test_simple_1(self) -> None:
        """Test a very simple script.  It won't compile, but it will parse."""
        res = v1.parse_v1(((_mk_ss(), b"main: {}"),))
        self.assertEqual([], [repr(p) for p in res.problems])
        script = res.required()
        self.assertEqual("test", script.name)
        self.assertEqual("1", script.version)
        self.assertEqual(("core",), script.add_in_names)

    def test_simple_2(self) -> None:
        """Test a very simple script.  It won't compile, but it will parse."""
        res = v1.parse_v1(
            (
                (
                    _mk_ss(),
                    b"main: {}\nname: test-simple-2\nversion: '3'\nrequire-libs: ['a', 'b']",
                ),
            )
        )
        self.assertEqual([], [repr(p) for p in res.problems])
        script = res.required()
        self.assertEqual("test-simple-2", script.name)
        self.assertEqual("3", script.version)
        self.assertEqual(("core", "a", "b"), script.add_in_names)


def _mk_ss() -> ScriptSource:
    return ScriptSource(
        source=["test"],
        src_hash="???",
        when=datetime.datetime.now(),
    )
