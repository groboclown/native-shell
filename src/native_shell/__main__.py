"""CLI entrypoint for the module."""
import sys
from native_shell.cli.main import cli_main

sys.exit(cli_main(sys.argv))
