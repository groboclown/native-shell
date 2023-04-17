"""Validations for constructing a syntax tree.

These are global validations.  They are used in later phases of parsing,
but are more helpful if early stages run these.

All validations return a Result, to better indicate whether the input
is valid or not valid.
"""

from ...util.message import i18n as _
from ...util.message import UserMessage
from ...util.result import Result, ResultGen, Problem, SourcePath, SourcePathElement


def validate_source_path(path: SourcePath) -> Result[None]:
    """Checks if the source path pieces conform to the requirements."""
    if len(path) <= 0:
        return Result.as_error(
            Problem.as_validation(
                path,
                UserMessage(_("source paths must have at least 1 element")),
            )
        )
    res = ResultGen()
    for element in path:
        res.add(validate_source_path_element(path, element))
    return res.build(None)


def validate_source_path_element(
    source: SourcePath,
    element: SourcePathElement,
) -> Result[None]:
    """Checks if the soruce path element conforms to the requirements."""
    if isinstance(element, int):
        if element < 0:
            return Result.as_error(
                Problem.as_validation(
                    source,
                    UserMessage(
                        _("integer path elements must be non-negative, found {value}"),
                        value=element,
                    ),
                )
            )
    else:
        if element == "":
            return Result.as_error(
                Problem.as_validation(
                    source,
                    UserMessage(
                        _("path elements must contain text, found '{value}'"),
                        value=element,
                    ),
                )
            )
        if not element.isprintable() or "/" in element:
            return Result.as_error(
                Problem.as_validation(
                    source,
                    UserMessage(
                        _(
                            "path elements must contain printable unicode and "
                            "not '/', found '{value}'"
                        ),
                        value=element,
                    ),
                )
            )
    return Result.as_none()
