"""The Native Shell syntax tree.

The heart of the system.
"""

from .node_defs import (
    SyntaxNode,
    SyntaxParameter,
)
from .validations import (
    validate_source_path,
    validate_source_path_element,
)
