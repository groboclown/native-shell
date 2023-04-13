"""A very, very trivial script file."""

from typing import Sequence, Tuple, List, Dict, Any
import yaml
from .root import parse_root_node
from ...defs.script import InitialScript, ScriptSource
from ...util.message import UserMessage
from ...util.message import i18n as _
from ...util.result import Result, Problem, ResultGen


def parse_v1(source: Sequence[Tuple[ScriptSource, bytes]]) -> Result[InitialScript]:
    """Parse v1 of the script."""
    if len(source) != 1:
        return Result.as_error(
            Problem.as_validation(
                ("parser",),
                _("v1 parser only supports 1 source file"),
            )
        )
    script_source, script_contents = source[0]

    try:
        raw_data = yaml.safe_load(script_contents)
    except Exception as err:  # pylint:disable=broad-except
        return Result.as_error(
            Problem(
                source=script_source.source,
                level="error",
                message=UserMessage(
                    _("Parsing source generated error {err}"),
                    err=err,
                ),
            )
        )
    if not isinstance(raw_data, dict):
        return Result.as_error(
            Problem.as_validation(
                script_source.source,
                _("Script must be a mapping with at least the 'main' element"),
            )
        )

    res = ResultGen()
    script_name = parse_script_name(script_source, raw_data, res)
    script_version = parse_script_version(script_source, raw_data, res)
    add_ins = parse_required_add_ins(script_source, raw_data, res)
    tree = parse_root_node(script_source.source, raw_data, res)
    return res.build(
        InitialScript(
            source=script_source,
            name=script_name,
            version=script_version,
            add_in_names=add_ins,
            tree=tree,
        )
    )


def parse_script_name(
    script_source: ScriptSource,
    data: Dict[str, Any],
    res: ResultGen,
) -> str:
    """Extract the script name from the source."""
    if "name" in data:
        name = data["name"]
        # Remove the field so it isn't picked up by the root parser.
        del data["name"]
        if name and isinstance(name, str):
            return name
        res.add(
            Problem.as_validation(
                (*script_source.source, "name"),
                _("'name' if given must be a non-empty string"),
            )
        )
    return str(script_source.source[-1])


def parse_script_version(
    script_source: ScriptSource,
    data: Dict[str, Any],
    res: ResultGen,
) -> str:
    """Extract the script version from the source."""
    if "version" in data:
        version = data["version"]
        # Remove the field so it isn't picked up by the root parser.
        del data["version"]
        if version and isinstance(version, (str, int, float)):
            return str(version)
        res.add(
            Problem.as_validation(
                (*script_source.source, "name"),
                _("'version' if given must be a non-empty string"),
            )
        )
    return "1"


def parse_required_add_ins(
    script_source: ScriptSource,
    data: Dict[str, Any],
    res: ResultGen,
) -> List[str]:
    """Extract the add-ins from the source."""
    # Always include core.
    ret: List[str] = ["core"]
    if "require-libs" in data:
        required_raw = data["require-libs"]
        del data["require-libs"]
        if isinstance(required_raw, (tuple, list)):
            index = 0
            for item in required_raw:
                if isinstance(item, str):
                    ret.append(item)
                else:
                    res.add(
                        Problem.as_validation(
                            (*script_source.source, "require-libs", index),
                            _("require-libs must be a list of strings"),
                        )
                    )
                index += 1
        else:
            res.add(
                Problem.as_validation(
                    (*script_source.source, "require-libs"),
                    _("require-libs must be a list of strings"),
                )
            )
    return ret
