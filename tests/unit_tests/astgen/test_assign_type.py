"""Test the module."""

import unittest
from helpers.parsed import (
    mk_simple,
    mk_parameter,
)
from native_shell.astgen import assign_type
from native_shell.astgen.typed_tree import TypedTree
from native_shell.defs.node_type import INTEGER_TYPE
from native_shell.defs.script import TypeHandlerStore
from native_shell.util.message import i18n
from native_shell.util.result import Problem


class AssignTypeTest(unittest.TestCase):
    """Test functions in the module."""

    def test_assign_types_to_node__problems(self) -> None:
        """Test assign_types_to_node with a node that has problems."""
        # The parameter has type "foo" which is not in the type store.
        #   This ensures that the node is not parsed but skipped
        #   because it has a problem when entered into the function.
        node = mk_parameter(["x"], "foo")
        self.assertIsNone(node.get_assigned_type())
        root = mk_parameter([], "", node=node)
        problem = Problem.as_validation(["y"], i18n("Skip"))
        node.add_problem(problem)
        tree = TypedTree(root, TypeHandlerStore({}))

        assign_type.assign_types_to_node(node, tree)

        self.assertEqual(1, len(node.problems()))
        self.assertIs(problem, node.problems()[0])
        self.assertEqual([], root.problems())
        self.assertIsNone(node.get_assigned_type())

    def test_assign_types_to_node__root_node(self) -> None:
        """Test assign_types_to_node with a root node."""
        # Root nodes do not have a type.
        node = mk_simple(["x"], 1)
        root = mk_parameter([], "", node=node)
        tree = TypedTree(root, TypeHandlerStore({}))

        assign_type.assign_types_to_node(root, tree)

        self.assertEqual([], root.problems())
        self.assertEqual([], node.problems())
        self.assertEqual(INTEGER_TYPE, node.get_assigned_type())

    def test_assign_types_to_node__simple(self) -> None:
        """Test assign_types_to_node with a simple node."""
        node = mk_simple(["x"], 1)
        root = mk_parameter([], "", node=node)
        tree = TypedTree(root, TypeHandlerStore({}))

        assign_type.assign_types_to_node(node, tree)

        self.assertEqual([], node.problems())
        self.assertEqual([], root.problems())
        self.assertEqual(INTEGER_TYPE, node.get_assigned_type())
