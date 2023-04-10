"""Assemble the code into the different files."""

from typing import Set
from .code_map import create_code_map, CodeRefMap
from .expand_template import expand_template
from ..defs.script import PreparedScript
from ..defs.add_ins import CodePurpose
from ..util.result import Result, ResultGen


GO_VERSION = "1.20"


class AssembledCode:  # pylint:disable=too-few-public-methods
    """The code into designated files."""

    __slots__ = ("makefile", "go_mod", "main_go")

    def __init__(
        self,
        *,
        makefile: str,
        go_mod: str,
        main_go: str,
    ) -> None:
        self.makefile = makefile
        self.go_mod = go_mod
        self.main_go = main_go


def assemble_code(script: PreparedScript) -> Result[AssembledCode]:
    """Create the assembled code."""
    map_res = create_code_map(script)
    if map_res.is_not_valid:
        return Result.as_error(map_res.problems)

    res = ResultGen()
    res.add(map_res)
    go_mod = mk_go_mod(script)

    makefile = res.include(mk_makefile(script, map_res.required()), "")

    main_go = res.include(mk_main_go(script, map_res.required()), "")

    return res.build(
        AssembledCode(
            makefile=makefile,
            go_mod=go_mod,
            main_go=main_go,
        )
    )


def mk_go_mod(script: PreparedScript) -> str:
    """Create the go.mod file.  The 'require' lines are pulled in by the make file,
    as that requires also assembling the go.sum file."""
    return f"""
module {script.name}

go {GO_VERSION}

"""


def mk_makefile(script: PreparedScript, cr_map: CodeRefMap) -> Result[str]:
    """Creates the makefile for building the script."""
    # Needs to get all the modules to include.  They are added during the make process.
    modules: Set[str] = set()
    for code in cr_map.get_all_for_purpose("modules"):
        # The template for a module entry must be one module per string.
        for part in code.template.parts:
            if isinstance(part, str):
                modules.add(part.strip())
            else:
                raise RuntimeError(
                    f"add-in generated 'modules' purpose template with non-string: " f"{part!r}"
                )
    ret = ".PHONY: build clean\n\nbuild:"
    for name in sorted(list(modules)):
        ret += f"\tgo get {name}\n"  # pylint:disable=consider-using-join
    ret += "\tmkdir -p bin\n"
    ret += f"\tgo build -o bin/{script.name} .\n\n"
    ret += "clean:\n\ttest -d bin && rm -r bin\n\n"
    return Result.as_value(ret)


def mk_main_go(_script: PreparedScript, cr_map: CodeRefMap) -> Result[str]:
    """Create the main.go file."""
    res = ResultGen()
    imports_str = mk_imports(cr_map)
    static_str = res.include(mk_static(cr_map), "")
    main_str = mk_main_func(cr_map)

    return res.build(
        f"""
package main

import (
{imports_str}
)

{static_str}
{main_str}
"""
    )


def mk_imports(cr_map: CodeRefMap) -> str:
    """Create the list of imports."""
    imports: Set[str] = set()
    for code in cr_map.get_all_for_purpose("import_as"):
        # The template for a module entry must be one module per string.
        for part in code.template.parts:
            if isinstance(part, str):
                text = part.strip()
                if '"' not in text:
                    text = f'"{text}"'
                imports.add(f"\t{text}")
            else:
                raise RuntimeError(
                    f"add-in generated 'modules' purpose template with non-string: " f"{part!r}"
                )
    return "\n".join(sorted(list(imports)))


def mk_static(cr_map: CodeRefMap) -> Result[str]:
    """Create stuff for outside the main function."""
    res = ResultGen()
    ret = res.include(expand_all_purpose(cr_map, "create_parameter_const", ""), "") + res.include(
        expand_all_purpose(cr_map, "define_field", ""), ""
    )

    return res.build(ret)


def mk_main_func(cr_map: CodeRefMap) -> Result[str]:
    """Create the main execution stuff."""
    #     "initialize_field",
    res = ResultGen()
    ret = "\nfunc main() {\n"
    ret += res.include(expand_all_purpose(cr_map, "initialize_field", "\t"), "")

    ret += "\n\t// TODO include the main execution path.\n"

    ret += "\n}\n"
    return res.build(ret)


def expand_all_purpose(
    cr_map: CodeRefMap,
    purpose: CodePurpose,
    indent: str,
) -> Result[str]:
    """Expand all the code templates for a global purpose."""
    res = ResultGen()
    ret = ""

    for code in cr_map.get_all_for_purpose(purpose):
        part = res.include(
            expand_template(
                source=code.ref,
                template=code.template,
                refs=cr_map,
            ),
            "",
        )
        ret += f"{indent}// {code.ref!r}\n"
        for line in part.splitlines():
            line = line.rstrip()
            if line:
                ret += indent + line + "\n"
            else:
                ret += "\n"

    return res.build(ret)
