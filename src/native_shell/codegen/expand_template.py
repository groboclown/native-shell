"""Expand a code template."""

from typing import List
from .code_map import CodeRefMap
from ..defs.basic import NodeReference
from ..defs.add_ins import CodeTemplate
from ..util.message import i18n as _
from ..util.message import UserMessage
from ..util.result import Result, ResultGen, Problem


def expand_template(
    source: NodeReference,
    template: CodeTemplate,
    refs: CodeRefMap,
) -> Result[str]:
    """Expand the template by in-lining references.

    This needs to include some checks to ensure fields that can't be
    referenced before execution aren't referenced before execution.
    """
    res = ResultGen()
    ret = _recursive_expand(
        ref=source,
        template=template,
        refs=refs,
        visiting=[],
        problems=res,
    )
    return res.build(ret)


def _recursive_expand(
    *,
    ref: NodeReference,
    template: CodeTemplate,
    refs: CodeRefMap,
    visiting: List[NodeReference],
    problems: ResultGen,
) -> str:
    # *shudder* recursion with uncontrolled stack depth.

    ret = ""
    for part in template.parts:
        if isinstance(part, str):
            ret += part
            continue

        # isinstance(part, CodeReference):
        if part in visiting:
            problems.add(
                Problem.as_validation(
                    tuple(ref),
                    UserMessage(
                        _("Reference {current} contains cyclic lookup to {other}"),
                        current=repr(ref),
                        other=repr(part),
                    ),
                )
            )
            ret += f"<cycle:{part}>"
            continue
        ref_template = refs.get_for_purpose(part.ref, part.purpose)
        if len(ref_template) <= 0:
            problems.add(
                Problem.as_validation(
                    tuple(part.ref),
                    UserMessage(
                        _("No template found for {purpose} at {ref}"),
                        ref=part.ref,
                        purpose=part.purpose,
                    ),
                )
            )
            ret += f"<unknown:{part}>"
            continue
        if len(ref_template) > 1:
            # There might be a correct way to handle this, but
            #   that would require knowing proper ordering which isn't guaranteed.
            problems.add(
                Problem.as_validation(
                    tuple(part.ref),
                    UserMessage(
                        _("Multiple templates found for {purpose} at {ref}"),
                        ref=part.ref,
                        purpose=part.purpose,
                    ),
                )
            )
            ret += f"<multiple:{part}>"
            continue
        visiting.append(ref)
        ret += _recursive_expand(
            ref=part.ref,
            template=ref_template[0].template,
            refs=refs,
            visiting=visiting,
            problems=problems,
        )
        visiting = visiting[:-1]
    return ret
