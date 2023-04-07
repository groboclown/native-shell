"""Abstract Base Classes for the conversion from the
basic script parsing into the final syntax tree.

"""

from typing import Iterable, Sequence, List, Mapping, Dict, Union, Optional, Any
from ..basic import SimpleParameter, NodeReference
from ..syntax_tree import AbcType, validate_source_path
from ...util.message import i18n as _
from ...util.message import UserMessage
from ...util.result import Result, ResultGen, Problem, SourcePath

INVALID_NODE_ID = "<invalid>"

ParsedParameter = Union["ParsedNode", SimpleParameter]


class ParsedNode:  # pylint:disable=too-many-instance-attributes,protected-access
    """A node in the syntax tree that can potentially contain a generator.
    It's a simplified view of the syntax tree.  The preparer will convert these
    into the full AbcSyntaxNode types.

    This allows for updating the node after it's been initially created (it's mutable).
    This should help make construction of the parsed tree easier.
    """

    __slots__ = (
        "__source",
        "__node_ptr",
        "__node_id",
        "__type_id",
        "__problems",
        "_parameters",
        "__items",
        # The next items are protected space, so that one node
        #   can touch around the parent and children directly.
        "_assigned_type",
        "_parent",
        "_parent_key",
    )

    def __init__(
        self,
        *,
        source: SourcePath,
        node_id: NodeReference,
        type_id: str,
        problems: Iterable[Problem] = (),
    ) -> None:
        self.__source = tuple(source)
        self.__node_id = node_id
        self.__problems = ResultGen()
        self.__problems.add(problems)
        source_validation = validate_source_path(source)
        self.__problems.add(source_validation)
        if source_validation.is_valid:
            self.__node_ptr = _mk_node_id(source)
        else:
            self.__node_ptr = INVALID_NODE_ID
        self.__type_id = type_id
        self._assigned_type: Optional[AbcType] = None
        self._parameters: Dict[str, ParsedParameter] = {}
        self._parent: Optional[ParsedNode] = None
        self._parent_key: Optional[str] = None

    def source(self) -> SourcePath:
        """Get the source location for the node."""
        return self.__source

    def node_id(self) -> NodeReference:
        """Path to this node in the tree.  Very similar to the source,
        but doesn't include information like the source script file position."""
        return self.__node_id

    def node_ptr(self) -> str:
        """The string identifier for the node.  This is equivalent to an
        absolute reference path."""
        return self.__node_ptr

    def type_id(self) -> str:
        """Get the node's type ID."""
        return self.__type_id

    def set_type(self, type_val: AbcType) -> None:
        """Assign this node's type instance based on the type id.  This can only be called once.
        If the type has already been assigned, this will raise a runtime error.
        """
        if self._assigned_type is not None:
            raise RuntimeError(f"Already assigned a type to node {self.__source}")
        self._assigned_type = type_val

    def assigned_type(self) -> Optional[AbcType]:
        """Get the assigned type, if one has been assigned."""
        return self._assigned_type

    def parent_node(self) -> "Optional[ParsedNode]":
        """Get the parent node, if this one has been assigned as a child of another."""
        return self._parent

    def parent_key(self) -> Optional[str]:
        """The parameter key this node is a child of."""
        return self._parent_key

    def problems(self) -> Sequence[Problem]:
        """Get any problems associated with this node.

        This does not include parameter value problems.
        """
        return self.__problems.problems

    def is_valid(self) -> bool:
        """Is this node valid?"""
        return self.__problems.is_valid()

    def is_not_valid(self) -> bool:
        """Is this node not valid?"""
        return self.__problems.is_not_valid()

    def add_problem(
        self,
        *values: Union[
            Problem,
            Iterable[Problem],
            Result[Any],
            Iterable[Result[Any]],
        ],
    ) -> None:
        """Add the problem arguments into the built-up list of problems."""
        self.__problems.add(*values)

    def parameter_size(self) -> int:
        """Number of items in this node."""
        return len(self._parameters)

    def parameter_map(self) -> Mapping[str, ParsedParameter]:
        """Get the values for the parameters, as a map.

        These are the child nodes.
        """
        return self._parameters

    def parameter_list(self) -> Result[Sequence[ParsedParameter]]:
        """Attempts to generate a list of child nodes.  Only possible if each
        of the keys are numbers.  The nodes are returned in order based on the
        original keys sorted by numeric value, but the actual index may not match
        up to the original key."""
        i_keys: Dict[int, str] = {}
        for s_key in self._parameters:
            try:
                i_key = int(s_key)
                i_keys[i_key] = s_key
            except ValueError:
                return Result.as_error(
                    Problem.as_validation(
                        self.__source,
                        UserMessage(
                            _("cannot convert parameters to list; key is not an int: {key}"),
                            key=s_key,
                        ),
                    )
                )
        ret: List[ParsedParameter] = []
        for i_key in sorted(i_keys.keys()):
            ret.append(self._parameters[i_keys[i_key]])
        return Result.as_value(ret)

    def set_parameter(self, key: int | str, child: ParsedParameter) -> None:
        """Add a parameter to the list.  The child can't have been assigned as a child
        to another node.  Additionally, the key can't have already been assigned.
        Violating these rules is a programmer bug, and raises an immediate exception."""
        s_key = str(key)
        if s_key in self._parameters:
            raise RuntimeError(
                f"attempted to add node {child!r} as parameter {key} of node {self.__source}; "
                f" but that parameter is already assigned to ({self._parameters[s_key]!r})"
            )

        if isinstance(child, ParsedNode):
            if child._parent is not None:
                raise RuntimeError(
                    f"attempted to add node {child!r} as parameter {key} of node {self.__source}; "
                    f" but it is already a child to another node ({child._parent.source})"
                )
            child._parent = self
            child._parent_key = s_key
        self._parameters[s_key] = child

    def replace_parameter(self, key: int | str, child: ParsedParameter) -> ParsedParameter:
        """Explicitly replace a child node with another one.  This allows for expanding
        meta-types into their transformed version.  Returns the replaced value.
        The parameter must already exist.
        """
        s_key = str(key)
        ret = self._parameters.get(s_key)
        if ret is None:
            raise RuntimeError(
                f"attempted to replace parameter {key} in {self.__source} with "
                f"{child!r}, but the parameter was not previously assigned."
            )
        if isinstance(child, ParsedNode):
            if child._parent is not None:
                raise RuntimeError(
                    f"attempted to add node {child!r} as parameter {key} of node {self.__source}; "
                    f" but it is already a child to another node ({child._parent.source})"
                )
            child._parent = self
            child._parent_key = s_key
        self._parameters[s_key] = child
        # Clean out references in the returned node.
        if isinstance(ret, ParsedNode):
            ret._parent = None
            ret._parent_key = None
        return ret

    def node_parameter_map(self) -> "Mapping[str, ParsedNode]":
        """Get the parameter values that are themselves ParsedNode instances."""
        ret: Dict[str, ParsedNode] = {}
        for key, val in self._parameters.items():
            if isinstance(val, ParsedNode):
                ret[key] = val
        return ret

    def simple_parameter_map(self) -> Mapping[str, SimpleParameter]:
        """Get only the simple parameter values."""
        ret: Dict[str, SimpleParameter] = {}
        for key, val in self._parameters.items():
            if not isinstance(val, ParsedNode):
                ret[key] = val
        return ret

    def close(self) -> None:
        """Immediately clean up this node and its children.
        The problems will still be kept.  Only cleans down the tree, not up."""
        # Clean up children first.  This is a very very simple stack approach to
        #   help prevent stack overflow issues.
        stack: List[ParsedNode] = [self]
        while stack:
            node = stack.pop()
            stack.extend(node.node_parameter_map().values())
            node._parent = None
            node._parameters = {}
            node._assigned_type = None

    def __repr__(self) -> str:
        return self.__node_ptr


def _mk_node_id(source: SourcePath) -> str:
    """Create a node ID for the node at the path."""
    return "/".join((str(p) for p in source))
