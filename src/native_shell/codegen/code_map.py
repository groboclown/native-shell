"""Handles references to node parameters."""

from typing import Sequence, List, Dict
from .tree_visitor import walk_nodes
from ..defs.basic import NodeReference
from ..defs.script import PreparedScript
from ..defs.syntax_tree import SyntaxNode
from ..defs.add_ins import GeneratedCode, CodePurpose
from ..util.result import Result, ResultGen


class CodeRef:
    """Code associated with a node in tree."""

    __slots__ = (
        "__ref",
        "__by_purpose",
        "__fetched",
    )

    def __init__(self, ref: NodeReference) -> None:
        self.__ref = ref
        self.__by_purpose: Dict[CodePurpose, List[GeneratedCode]] = {}
        self.__fetched = False

    def is_used(self) -> bool:
        """Is this node used yet?"""
        return self.__fetched

    def ref(self) -> NodeReference:
        """Reference to this code."""
        return self.__ref

    def get(self, purpose: CodePurpose) -> Sequence[GeneratedCode]:
        """Get the code at this node for the given purpose."""
        self.__fetched = True
        return self.__by_purpose.get(purpose, ())

    def add_code(self, code: GeneratedCode) -> None:
        """Add the code to this reference."""
        if self.__ref != code.ref:
            raise RuntimeError(f"Code at {code.ref} added to ref {self.__ref}")
        purposes = self.__by_purpose.get(code.purpose)
        if not purposes:
            purposes = []
            self.__by_purpose[code.purpose] = purposes
        purposes.append(code)


class CodeRefMap:
    """Maps references to CodeRef"""

    __slots__ = ("__map",)

    def __init__(self) -> None:
        self.__map: Dict[NodeReference, CodeRef] = {}

    def add(self, code: GeneratedCode) -> None:
        """Add the generated code to the ref map."""
        ref = self.__map.get(code.ref)
        if not ref:
            ref = CodeRef(code.ref)
            self.__map[code.ref] = ref
        ref.add_code(code)

    def contains(self, ref: NodeReference) -> bool:
        """Checks if the reference exists.  This allows for cache
        population to work right."""
        return ref in self.__map

    def get_for_purpose(
        self,
        lookup: NodeReference,
        purpose: CodePurpose,
    ) -> Sequence[GeneratedCode]:
        """Get the code for the reference."""
        ref = self.__map.get(lookup)
        if ref is None:
            return ()
        return ref.get(purpose)

    def get_all_for_purpose(self, purpose: CodePurpose) -> Sequence[GeneratedCode]:
        """Gets all generated code segments with the given purpose.
        This is necessary for things like 'modules' and 'include_as'.
        """
        ret: List[GeneratedCode] = []
        for code_ref in self.__map.values():
            ret.extend(code_ref.get(purpose))
        return ret


def create_code_map(script: PreparedScript) -> Result[CodeRefMap]:
    """Create the code map from the script."""

    res = ResultGen()
    ret = CodeRefMap()
    for handler in script.type_handlers.all():
        for code in handler.shared_code():
            ret.add(code)

    def visitor(node: SyntaxNode) -> bool:
        node_type = node.node_type()
        v_handler = script.type_handlers.get(node_type)
        if v_handler:
            # If there isn't a handler, then ignore it.  Errors were
            # already managed in the script construction.
            code_list = res.include(v_handler.instance_code(node), ())
            for v_code in code_list:
                ret.add(v_code)

        return False

    walk_nodes(script.tree, visitor)
    return res.build(ret)
