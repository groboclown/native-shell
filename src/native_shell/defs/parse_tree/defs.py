"""Abstract Base Classes for the conversion from the
basic script parsing into the final syntax tree.

"""

from typing import Iterable, Sequence, List, Mapping, Dict, Union, Optional, Any, cast
from typing_extensions import Protocol
from ..basic import SimpleParameter, NodeReference
from ..syntax_tree import (
    AbcType,
    TypeParameter,
    BasicType,
    ListType,
    BASIC_TYPE_NAMES,
    LIST_TYPE_NAME,
)
from ...util.message import i18n as _
from ...util.message import UserMessage
from ...util.result import Result, ResultGen, Problem, SourcePath


INVALID_NODE_ID = "<invalid>"


class ParsedNodeId:
    """An identifier for any parsed node."""

    __slots__ = ("__source", "__ref", "__node_ptr")

    def __init__(
        self,
        *,
        source: SourcePath,
        ref: NodeReference,
    ) -> None:
        self.__source = source
        self.__ref = ref
        self.__node_ptr = "/".join((str(p) for p in source))

    @property
    def source(self) -> SourcePath:
        """Get the source location for the node."""
        return self.__source

    @property
    def ref(self) -> NodeReference:
        """Path to this node in the tree.  Very similar to the source,
        but doesn't include information like the source script file position."""
        return self.__ref

    @property
    def node_ptr(self) -> str:
        """The string identifier for the node.  This is equivalent to an
        absolute reference path."""
        return self.__node_ptr

    def __str__(self) -> str:
        return self.__node_ptr


class ParentReference:
    """Information about the parent in the parsed node tree.

    If the node is a list container, then the parameter type is the parameter type
    of the parent.
    """

    __slots__ = ("__node", "__key", "__parameter_type")

    def __init__(
        self,
        *,
        node: Union["ParsedParameterNode", "ParsedListNode"],
        key: Union[str, int],
        parameter_type: Optional[TypeParameter] = None,
    ) -> None:
        # Runtime checks.
        if isinstance(key, str) and not isinstance(node, ParsedParameterNode):
            raise ValueError("Only parameter nodes can have string keys")
        if isinstance(key, int) and not isinstance(node, ParsedListNode):
            raise ValueError("Only list nodes can have int keys")

        self.__node = node
        self.__key = key
        self.__parameter_type = parameter_type

    @property
    def node(self) -> Union["ParsedParameterNode", "ParsedListNode"]:
        """The parent node.  Must be one of the container types."""
        return self.__node

    @property
    def key(self) -> Union[str, int]:
        """The key in the ``node``."""
        return self.__key

    def get_parameter_type(self) -> Optional[TypeParameter]:
        """The property type for the ``key`` in the ``node``.
        If the node is a list container, then the returned value is
        the same as the node's own parameter type.

        If the value returned is None, then it hasn't been discovered yet.
        """
        return self.__parameter_type

    def set_parameter_type(self, param_type: TypeParameter) -> None:
        """Set the parameter type.  May only be called once."""
        if self.__parameter_type is not None:
            raise RuntimeError(
                f"attempted to set parameter type on {self} twice; "
                f"already assigned to {self.__parameter_type}, tried to set to "
                f"{param_type}."
            )
        self.__parameter_type = param_type

    def __str__(self) -> str:
        return f"{self.__node!r}@{self.__key}"

    def __repr__(self) -> str:
        return f"{self.__node!r}@{self.__key}"


# pylint:disable=missing-function-docstring
class AbcParsedNode(Protocol):
    """The generic type conformity for all ParsedNode classes.

    Container nodes provide implementation specific add child methods.

    Even non-container nodes must implement the basic child fetching
    calls, but they will return nothing, and they will not provide add child
    methods.
    """

    @property
    def node_id(self) -> ParsedNodeId:
        ...

    @property
    def type_id(self) -> str:
        ...

    def get_parent(self) -> Optional[ParentReference]:
        ...

    def set_parent(self, parent: ParentReference) -> None:
        ...

    def problems(self) -> Sequence[Problem]:
        ...

    def is_not_valid(self) -> bool:
        ...

    def add_problem(
        self,
        *values: Union[
            Problem,
            Iterable[Problem],
            Result[Any],
            Iterable[Result[Any]],
        ],
    ) -> None:
        ...

    def get_assigned_type(self) -> Union[BasicType, ListType, AbcType, None]:
        ...

    def mapping(self) -> Mapping[Union[str, int], "AbcParsedNode"]:
        ...

    def keys(self) -> Iterable[Union[str, int]]:
        ...

    def values(self) -> Iterable["AbcParsedNode"]:
        ...

    def replace_value(self, key: Union[str, int], node: "AbcParsedNode") -> "AbcParsedNode":
        ...

    def cleanup(self) -> None:
        ...


def _ensure_parent_not_set(node_id: ParsedNodeId, parent: Optional[ParentReference]) -> None:
    if parent is not None:
        raise RuntimeError(f"Attempted to set parent for {node_id}")


def _ensure_type_not_set(node_id: ParsedNodeId, type_val: Optional[AbcType]) -> None:
    if type_val is not None:
        raise RuntimeError(f"Attempted to set type for {node_id}")


def _ensure_type_property_not_set(node_id: ParsedNodeId, type_val: Optional[TypeParameter]) -> None:
    if type_val is not None:
        raise RuntimeError(f"Attempted to set item type for {node_id}")


class ParsedSimpleNode:
    """A simple node from the parsed source file.

    The node is a constant value with a simple type.
    """

    __slots__ = ("__id", "__parent", "__value", "__type_id", "__problems", "__type")

    def __init__(
        self,
        *,
        node_id: ParsedNodeId,
        type_id: str,
        value: SimpleParameter,
    ) -> None:
        self.__id = node_id
        self.__parent: Optional[ParentReference] = None
        self.__value = value
        self.__type_id = type_id
        self.__problems = ResultGen()
        self.__type: Optional[BasicType] = None
        if type_id not in BASIC_TYPE_NAMES:
            self.__problems.add(
                Problem.as_validation(
                    node_id.source,
                    UserMessage(_("Assigned simple node to not basic type {name}"), name=type_id),
                )
            )
        else:
            self.__type = cast(BasicType, type_id)

    @property
    def node_id(self) -> ParsedNodeId:
        """The ID for the node."""
        return self.__id

    @property
    def type_id(self) -> str:
        """The declared value type of this node."""
        return self.__type_id

    def get_assigned_type(self) -> Union[BasicType, ListType, AbcType, None]:
        """Get the assigned type."""
        return self.__type

    @property
    def value(self) -> SimpleParameter:
        """The constant value for this node."""
        return self.__value

    def get_parent(self) -> Optional[ParentReference]:
        """Get the parent reference, if there is one and it's been
        discovered."""
        return self.__parent

    def set_parent(self, parent: ParentReference) -> None:
        """Set the parent reference.  Can only be called once."""
        _ensure_parent_not_set(self.__id, self.__parent)
        self.__parent = parent

    def problems(self) -> Sequence[Problem]:
        """Get the registered problems for this node."""
        return self.__problems.problems

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
        """Add problems to this node.  They should all be related just to this
        one node."""
        self.__problems.add(*values)

    def mapping(self) -> Mapping[Union[str, int], AbcParsedNode]:
        """Returns an empty map."""
        return {}

    def keys(self) -> Iterable[Union[str, int]]:
        """Returns an empty sequence."""
        return ()

    def values(self) -> Iterable[AbcParsedNode]:
        """Returns an empty sequence."""
        return ()

    def replace_value(self, key: Union[str, int], _node: AbcParsedNode) -> AbcParsedNode:
        """Raises a runtime error."""
        raise RuntimeError(f"Attempted to replace '{key}' on non-container node {self!r}")

    def cleanup(self) -> None:
        """Clean up the memory used by this node.  Does not clean up problems."""
        self.__parent = None

    def __str__(self) -> str:
        return str(self.__id)

    def __repr__(self) -> str:
        return f"ParsedSimpleNode({self.__id})"


class ParsedListNode:
    """A node that contains other nodes in an ordered list."""

    __slots__ = ("__id", "__parent", "__items", "__item_type", "__problems")

    def __init__(
        self,
        *,
        node_id: ParsedNodeId,
    ) -> None:
        self.__id = node_id
        self.__parent: Optional[ParentReference] = None
        self.__problems = ResultGen()
        self.__items: List[AbcParsedNode] = []
        self.__item_type: Optional[TypeParameter] = None

    @property
    def node_id(self) -> ParsedNodeId:
        """The ID for the node."""
        return self.__id

    @property
    def type_id(self) -> str:
        """The declared value type of this node.  List nodes can only be of type list."""
        return LIST_TYPE_NAME

    def get_item_type(self) -> Union[TypeParameter, None]:
        """Get the item type."""
        return self.__item_type

    def set_item_type(self, type_val: TypeParameter) -> None:
        """Sets the item type.  May only be called once."""
        _ensure_type_property_not_set(self.node_id, self.__item_type)
        self.__item_type = type_val

    def get_assigned_type(self) -> Union[BasicType, ListType, AbcType, None]:
        """Get the assigned type."""
        return LIST_TYPE_NAME

    def get_parent(self) -> Optional[ParentReference]:
        """Get the parent reference, if there is one and it's been
        discovered."""
        return self.__parent

    def set_parent(self, parent: ParentReference) -> None:
        """Set the parent reference.  Can only be called once."""
        _ensure_parent_not_set(self.__id, self.__parent)
        self.__parent = parent

    def problems(self) -> Sequence[Problem]:
        """Get the registered problems for this node."""
        return self.__problems.problems

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
        """Add problems to this node.  They should all be related just to this
        one node."""
        self.__problems.add(*values)

    def mapping(self) -> Mapping[Union[str, int], AbcParsedNode]:
        """Get the contained values as a mapping from the key to the node."""
        return {idx: self.__items[idx] for idx in range(len(self.__items))}

    def keys(self) -> Iterable[Union[str, int]]:
        """All "keys" in this container."""
        return range(len(self.__items))

    def values(self) -> Iterable[AbcParsedNode]:
        """Get the items contained in this list container."""
        return self.__items

    def add_value(self, node: AbcParsedNode) -> Union[str, int]:
        """Connects the given node to the end of the contained list.  Returns
        the index of the connected node."""
        p_node = assert_is_parsed_node(node)
        index = len(self.__items)
        # Set the child node's parent first, as it can fail.
        node.set_parent(ParentReference(node=self, key=index))
        self.__items.append(p_node)
        return index

    def replace_value(self, key: Union[str, int], node: AbcParsedNode) -> AbcParsedNode:
        """Replace an existing node with another one.  This will return the
        replaced child node.  It will need to be cleaned up to ensure proper
        memory handling."""
        p_node = assert_is_parsed_node(node)

        # This will generate an IndexError or ValueError depending on the
        #   various problems that can occur.
        index = int(key)
        ret = self.__items[index]

        node.set_parent(ParentReference(node=self, key=index))
        self.__items[index] = p_node

        return ret

    def cleanup(self) -> None:
        """Clean up the memory used by this node.  Does not clean up problems or
        children's values, but they are removed."""
        self.__parent = None
        self.__items.clear()

    def __str__(self) -> str:
        return str(self.__id)

    def __repr__(self) -> str:
        return f"ParsedListNode({self.__id})"


class ParsedParameterNode:
    """A node that contains keyed parameters."""

    __slots__ = ("__id", "__parent", "__params", "__type_id", "__type", "__problems")

    def __init__(
        self,
        *,
        node_id: ParsedNodeId,
        type_id: str,
    ) -> None:
        self.__id = node_id
        self.__parent: Optional[ParentReference] = None
        self.__type_id = type_id
        self.__problems = ResultGen()
        self.__params: Dict[str, AbcParsedNode] = {}
        self.__type: Optional[AbcType] = None

    @property
    def node_id(self) -> ParsedNodeId:
        """The ID for the node."""
        return self.__id

    @property
    def type_id(self) -> str:
        """The declared value type of this node."""
        return self.__type_id

    def get_parent(self) -> Optional[ParentReference]:
        """Get the parent reference, if there is one and it's been
        discovered."""
        return self.__parent

    def set_parent(self, parent: ParentReference) -> None:
        """Set the parent reference.  Can only be called once."""
        _ensure_parent_not_set(self.__id, self.__parent)
        self.__parent = parent

    def set_type(self, type_val: AbcType) -> None:
        """Attempt to set the type."""
        _ensure_type_not_set(self.__id, self.__type)
        self.__type = type_val

    def get_assigned_type(self) -> Union[BasicType, ListType, AbcType, None]:
        """Get the assigned type."""
        return self.__type

    def problems(self) -> Sequence[Problem]:
        """Get the registered problems for this node."""
        return self.__problems.problems

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
        """Add problems to this node.  They should all be related just to this
        one node."""
        self.__problems.add(*values)

    def mapping(self) -> Mapping[Union[str, int], AbcParsedNode]:
        """Get the contained values as a mapping from the key to the node."""
        return cast(Mapping[Union[str, int], AbcParsedNode], self.__params)

    def keys(self) -> Iterable[Union[str, int]]:
        """All "keys" in this container."""
        return self.__params.keys()

    def values(self) -> Iterable[AbcParsedNode]:
        """Get the items contained in this list container."""
        return self.__params.values()

    def set_parameter(self, key: str, node: AbcParsedNode) -> None:
        """Connects the given node to the end of the contained list.  Returns
        the index of the connected node."""
        p_node = assert_is_parsed_node(node)

        if key in self.__params:
            raise ValueError(
                f"Attempted to replace a parameter at {key} when setting it; "
                f"container: {self}, existing node: {self.__params[key]}, "
                f"set node: {node}"
            )

        # Set the child node's parent first, as it can fail.
        node.set_parent(ParentReference(node=self, key=key))
        self.__params[key] = p_node

    def replace_value(self, key: Union[str, int], node: AbcParsedNode) -> AbcParsedNode:
        """Replace an existing node with another one.  This will return the
        replaced child node.  It will need to be cleaned up to ensure proper
        memory handling."""
        p_node = assert_is_parsed_node(node)
        s_key = str(key)

        # This will generate a KeyError if it's not replacing.
        ret = self.__params[s_key]

        node.set_parent(ParentReference(node=self, key=key))
        self.__params[s_key] = p_node

        return ret

    def cleanup(self) -> None:
        """Clean up the memory used by this node.  Does not clean up problems or
        children's values, but they are removed."""
        self.__parent = None
        self.__params.clear()

    def __str__(self) -> str:
        return str(self.__id)

    def __repr__(self) -> str:
        return f"ParsedParameterNode({self.__id!r})"


def assert_is_parsed_node(node: object) -> AbcParsedNode:
    """Ensure the given node is of the right type."""
    # This also ensures that each of the declared types has
    #   the correct structural subtype
    # https://mypy.readthedocs.io/en/stable/protocols.html#protocol-types
    if isinstance(node, ParsedListNode):
        return node
    if isinstance(node, ParsedParameterNode):
        return node
    if isinstance(node, ParsedSimpleNode):
        return node
    raise AssertionError(f"Not a ParsedNode: {node}")
