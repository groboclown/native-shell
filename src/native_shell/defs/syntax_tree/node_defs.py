"""Syntax tree structure."""

from typing import Mapping, Union
from ..node_type.defs import AbcType
from ..basic import SimpleParameter, NodeReference
from ...util.result import SourcePath

SyntaxParameter = Union["SyntaxNode", SimpleParameter]


class SyntaxNode:
    """A finalized node in the syntax tree.  It isn't 1-to-1 related to the
    underlying type system, because a node may be a list of nodes, if the
    parent parameter type has "is_list" marked.

    If the parent parameter marked this node as a list, then the parameter values
    returns the list translated into a map.  If order is important, then the caller
    must perform numeric sorting on the keys.

    A node has a type and associated parameter values.  It cannot
    be of a generator meta-type.  At this point in the script construction,
    all problems have been dealt with.
    """

    __slots__ = ("__source", "__node_id", "__type", "__values")

    def __init__(
        self,
        *,
        source: SourcePath,
        node_id: NodeReference,
        node_type: AbcType,
        values: Mapping[str, SyntaxParameter],
    ) -> None:
        # As this is the finalized form, we make copies of the compound types.
        self.__source = tuple(source)
        self.__node_id = node_id
        self.__type = node_type
        self.__values = dict(values)

    def source(self) -> SourcePath:
        """Get the source location for the node."""
        return self.__source

    def node_id(self) -> NodeReference:
        """The identifier for the node.  This is equivalent to an
        absolute reference path.  This is used to generate unique names
        for the node."""
        return self.__node_id

    def node_type(self) -> AbcType:
        """Get the node's type."""
        return self.__type

    def values(self) -> Mapping[str, SyntaxParameter]:
        """Get the values in this node.  These are either parameters or fields or,
        in the case of a list node, the list turned into a map by translating the
        index to a string."""
        return self.__values

    def __repr__(self) -> str:
        return "/".join(self.__node_id)
