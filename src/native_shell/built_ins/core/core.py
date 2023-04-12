"""The core add-in."""

from .echo import ECHO
from .error import ERROR
from ...defs.add_ins import AddIn


CORE = AddIn(
    name="core",
    description="Core functionality",
    include_name="core",
    type_handlers=(ECHO, ERROR),
    meta_types=(),
)
