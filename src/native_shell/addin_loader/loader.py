"""Loads add-ins defined in a script."""

from typing import List, Set
from ..builtins.core import CORE
from ..defs.add_ins import AddIn
from ..defs.script import InitialScript, StagingScript
from ..util.message import i18n as _
from ..util.result import Result, Problem, ResultGen


def load_add_ins(script: InitialScript) -> Result[StagingScript]:
    """Load the add-ins and return them in the staging script."""
    res = ResultGen()
    loaded: Set[str] = set()
    add_ins: List[AddIn] = []
    for name in script.add_in_names:
        if name in loaded:
            # Allow duplicate includes.
            continue
        loaded.add(name)
        if name == "core":
            add_ins.append(CORE)
        else:
            res.add(
                Problem.as_validation(
                    script.script_source.source,
                    _("unsupported add-in {name}"),
                    name=name,
                )
            )
    return res.build(
        StagingScript(
            source=script.script_source,
            name=script.name,
            version=script.version,
            add_ins=add_ins,
            tree=script.tree,
        )
    )
