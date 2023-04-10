"""The core add-in."""

from .echo import ECHO
from ...defs.add_ins import AddIn


CORE = AddIn(
    name="core",
    description="Core functionality",
    include_name="core",
    type_handlers=(ECHO,),
    meta_types=(),
)
