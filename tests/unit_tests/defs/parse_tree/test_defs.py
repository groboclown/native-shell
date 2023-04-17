"""Test the module."""

import unittest
from helpers.parsed import mk_simple, mk_list, mk_parameter
from native_shell.builtins.core import INTEGER_LIST_TYPE
from native_shell.defs.parse_tree import defs
from native_shell.defs.node_type import ConstructType
from native_shell.util.message import i18n


class ParseTreeDefsTest(unittest.TestCase):
    """Test the module functions."""

    def test_assert_is_parsed_node__not_a_node(self) -> None:
        """Test assert_is_parsed_node without a node object."""
        try:
            defs.assert_is_parsed_node("x")
        except AssertionError as err:
            self.assertEqual("AssertionError('Not a ParsedNode: x')", repr(err))


class ParsedSimpleNodeTest(unittest.TestCase):
    """Test the ParsedSimpleNode class."""

    def test_set_parent__already_set(self) -> None:
        """Test set_parent with the parent already set."""
        node = mk_simple(["test", "x", "1"], 1)
        mk_list(["test", "x"], node)
        try:
            mk_list(["test2", "f"], node)
        except RuntimeError as err:
            self.assertEqual("RuntimeError('Attempted to set parent for //x/1')", repr(err))


class ParsedParameterNodeTest(unittest.TestCase):
    """Test the ParsedParameterNode class."""

    def test_set_parent__already_set(self) -> None:
        """Test set_parent with the parent already set."""
        node = mk_parameter(["test", "x", "1"], "bark")
        mk_list(["test", "x"], node)
        try:
            mk_list(["test2", "f"], node)
        except RuntimeError as err:
            self.assertEqual("RuntimeError('Attempted to set parent for //x/1')", repr(err))

    def test_set_type__already_set(self) -> None:
        """Test set_type with the type already set, even if exactly already set."""
        node = mk_parameter(["test", "x", "1"], "tuna")
        type1 = ConstructType(
            source=[""],
            type_id="tuna",
            title=i18n("tuna"),
            description=i18n("tuna"),
            fields=(),
            parameters=(),
        )
        node.set_type(type1)
        try:
            node.set_type(type1)
        except RuntimeError as err:
            self.assertEqual("RuntimeError('Attempted to set type for //x/1')", repr(err))


class ParsedListNodeTest(unittest.TestCase):
    """Test the ParsedListNode class."""

    def test_set_parent__already_set(self) -> None:
        """Test set_parent with the parent already set."""
        node = mk_list(["test", "x", "1"])
        mk_list(["test", "x"], node)
        try:
            mk_list(["test2", "f"], node)
        except RuntimeError as err:
            self.assertEqual("RuntimeError('Attempted to set parent for //x/1')", repr(err))

    def test_set_item_type__already_set(self) -> None:
        """Test set_item_type with the value already set, even if exactly already set."""
        node = mk_list(["test", "x", "1"])
        node.set_type(INTEGER_LIST_TYPE)
        try:
            node.set_type(INTEGER_LIST_TYPE)
        except RuntimeError as err:
            self.assertEqual("RuntimeError('Attempted to set type for //x/1')", repr(err))
