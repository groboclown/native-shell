"""Abstract Base Classes for the add-ins."""

from typing import Iterable, Iterator, Sequence, Literal, Union
from ..basic import NodeReference
from ..syntax_tree import AbcType, AbcMetaType, SyntaxNode
from ..parse_tree import AbcParsedNode
from ...util.result import Result


CodePurpose = Literal[
    # get_field_value - read the value in a field.
    #   This is valid only if the field can be read before execution,
    #   or if execution has completed.  This can also be used for reading
    #   a static value.
    "get_field_value",
    # define_field - declare the storage for a field or a static value.
    "define_field",
    # initialize_field - set the initial value for a field or a static value.
    "initialize_field",
    # create_parameter_const - create a constant which stores the parameter value.
    "create_parameter_const",
    # The module to add to the code, as though added with `go get`.  The template
    #   underlying the modules MUST be one module per string.  It CANNOT have any
    #   references.  The text will be stripped of surrounding whitespace.
    "modules",
    # The import statements for the modules.  Normally, this is just the same
    #   as the corresponding module, but can also be a 'name "module reference"'.
    #   If the value does not include a '"' character, the value will be surrounded
    #   with '"'.
    "import_as",
    # The codeblock that runs the code.
    "execute",
    # Close out the fields and other values that were opened by the execution block.
    "finalize",
]
CodeReferencePurpose = Literal["get_field_value", "execute"]
CODE_PURPOSE_NAMES = (
    "get_field_value",
    "define_field",
    "initialize_field",
    "create_parameter_const",
    "modules",
    "import_as",
    "execute",
    "finalize",
)


class CodeReference:
    """A reference to another piece of code to be inserted at this location."""

    __slots__ = ("__ident", "__purpose")

    def __init__(self, ident: NodeReference, purpose: CodeReferencePurpose) -> None:
        self.__ident = ident
        self.__purpose = purpose

    @property
    def ref(self) -> NodeReference:
        """The code reference identity."""
        return self.__ident

    @property
    def purpose(self) -> CodeReferencePurpose:
        """The purpose version of this code reference."""
        return self.__purpose

    def __str__(self) -> str:
        return f"{self.__ident}/{self.__purpose}"


class CodeTemplate:
    """A parsed template, that is a series of text and references."""

    __slots__ = ("__parts",)

    def __init__(self, parts: Iterable[Union[CodeReference, str]]) -> None:
        self.__parts = tuple(parts)

    @property
    def parts(self) -> Sequence[Union[CodeReference, str]]:
        """Direct access to the parts of the template."""
        return self.__parts

    def __len__(self) -> int:
        return len(self.__parts)

    def __iter__(self) -> Iterator[Union[CodeReference, str]]:
        return iter(self.__parts)


class GeneratedCode:
    """A bit of code that is embeddable in other places.

    These are named with an identifier and a purpose.
    """

    __slots__ = ("__ref", "__purpose", "__template")

    def __init__(
        self,
        *,
        ref: NodeReference,
        purpose: CodePurpose,
        template: CodeTemplate,
    ) -> None:
        self.__ref = ref
        self.__purpose = purpose
        self.__template = template

    @property
    def ref(self) -> NodeReference:
        """The code identifier reference."""
        return self.__ref

    @property
    def purpose(self) -> CodePurpose:
        """Purpose for the code."""
        return self.__purpose

    @property
    def template(self) -> CodeTemplate:
        """The code with possible references."""
        return self.__template

    def __str__(self) -> str:
        return f"{self.__ref}/{self.__purpose}"


class AddInTypeHandler:
    """A type specific to an add-in, along with how the
    add-in uses this to handle the syntax nodes of the type.

    By convention, executable type handlers have an 'err' field that
    contains the error result (nil or an error) for the execution.
    """

    def type(self) -> AbcType:
        """The type representation for this handler."""
        raise NotImplementedError

    def shared_code(self) -> Iterable[GeneratedCode]:
        """All the code that is required to include in the source
        if this type is used.  The code must be read-only and stateless.
        The generated code's identity must be in the form, by convention:
        "['static', (type id), (code id)]"
        """
        raise NotImplementedError

    def instance_code(self, node: SyntaxNode) -> Result[Iterable[GeneratedCode]]:
        """Constructs the code templates that this specific node in the tree
        needs to run.  If it includes static code, then it must not conflict with
        code returned by ``shared_code``."""
        raise NotImplementedError


class AddInMetaTypeHandler:
    """A meta-type definition for an add-in."""

    def meta_type(self) -> AbcMetaType:
        """The type representation for this handler."""
        raise NotImplementedError

    def translate(self, tree: AbcParsedNode) -> Result[AbcParsedNode]:
        """Translates the tree into another tree through the meta-type rules."""
        raise NotImplementedError


class AddIn:
    """A basic add-in.

    Add-ins define new capabilities that a script can use.
    """

    __slots__ = ("__name", "__desc", "__incl", "__handlers", "__meta_types")

    def __init__(
        self,
        *,
        name: str,
        description: str,
        include_name: str,
        type_handlers: Iterable[AddInTypeHandler],
        meta_types: Iterable[AddInMetaTypeHandler],
    ) -> None:
        self.__name = name
        self.__desc = description
        self.__incl = include_name
        self.__handlers = tuple(type_handlers)
        self.__meta_types = tuple(meta_types)

    def name(self) -> str:
        """The add-in formal name."""
        return self.__name

    def description(self) -> str:
        """A long description of the add-in."""
        return self.__desc

    def include_name(self) -> str:
        """The name of the add-in as formally declared in the script
        when including this add-in."""
        return self.__incl

    def type_handlers(self) -> Sequence[AddInTypeHandler]:
        """The type handlers the add-in can handle."""
        return self.__handlers

    def meta_types(self) -> Sequence[AddInMetaTypeHandler]:
        """The meta-type handlers declared by this add-in."""
        return self.__meta_types
