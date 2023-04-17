"""Test the module."""

import unittest
from native_shell.cli import main


class MainTest(unittest.TestCase):
    """Test the module functions."""

    def test_cli_main__help(self) -> None:
        """Test the main program requesting help."""

        try:
            main.cli_main(["cli-main", "--help"])
        except SystemExit as err:
            self.assertEqual(0, err.code)
