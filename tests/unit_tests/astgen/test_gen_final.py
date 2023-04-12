"""Test the module."""

import unittest
from helpers.parsed import mk_parameter, mk_list, mk_simple
from native_shell.astgen import gen_final
from native_shell.astgen.gen_root_type import RootNodeHandler
from native_shell.defs.script import TypeHandlerStore
from native_shell.built_ins.core import CORE
from native_shell.built_ins.core.echo import ECHO


class GenFinalTest(unittest.TestCase):
    """Tests for the module functions."""

    def test_assign_types__list(self) -> None:
        """Test the assign_types call with a list node."""

        # Requirement for entry into assign_types is only no meta-types.
        root = mk_parameter(
            [],
            "",  # root node must have empty type id
            main=mk_parameter(
                ["main"],
                ECHO.type().type_id(),
                text=mk_list(
                    ["main", "text"],
                    mk_simple(["main", "text", "0"], "Hello, world!"),
                ),
                stdout=mk_simple(["main", "stdout"], True),
            ),
        )
        res = gen_final.assign_types(
            root,
            TypeHandlerStore({h.type().type_id(): h for h in CORE.type_handlers()}),
        )
        self.assertEqual(
            [],
            [repr(p) for p in res.root.problems()],
        )
        root_type = res.root.get_assigned_type()
        self.assertIsNotNone(root_type)
        self.assertIsInstance(
            res.get_handler(root_type),
            RootNodeHandler,
        )
        self.assertEqual({"main"}, set(res.root.keys()))
