"""Unified error and warning reporting system."""

from typing import (
    Sequence,
    List,
    Iterable,
    Callable,
    Generic,
    TypeVar,
    Literal,
    Union,
    Optional,
    Any,
    cast,
)
import collections.abc
from .message import UserMessage, UserMessageData, I18n


_T = TypeVar("_T")
_R = TypeVar("_R")
_T_co = TypeVar("_T_co", covariant=True)


SourcePathElement = Union[str, int]
SourcePath = Sequence[SourcePathElement]
Source = Union[SourcePathElement, SourcePath]
ProblemLevel = Literal["error", "warning", "info"]


class Problem:
    """A container for things that went wrong.

    In the best of worlds, it allows for localization of error messages.
    """

    __slots__ = ("_source", "_message", "_level")

    def __init__(
        self,
        *,
        source: SourcePath,
        level: ProblemLevel,
        message: UserMessage,
    ) -> None:
        self._source = source
        self._message = message
        self._level = level

    @staticmethod
    def as_validation(
        __source: SourcePath,
        __message: Union[UserMessage, I18n],
        **__arguments: UserMessageData,
    ) -> "Problem":
        """Create a validation problem."""
        if isinstance(__message, UserMessage):
            if len(__arguments) > 0:
                raise RuntimeError("Cannot pass arguments with a user namessage")
            return Problem(
                source=__source,
                level="error",
                message=__message,
            )
        return Problem(
            source=__source,
            level="error",
            message=UserMessage(__message, **__arguments),
        )

    @property
    def is_error(self) -> bool:
        """Is this an error-level problem?"""
        return self._level == "error"

    @property
    def is_warning(self) -> bool:
        """Is this a warning-level problem?"""
        return self._level == "warning"

    @property
    def is_info(self) -> bool:
        """Is this an informative message?"""
        return self._level == "info"

    @property
    def source(self) -> SourcePath:
        """The source of the problem."""
        return self._source

    @property
    def message(self) -> UserMessage:
        """The raw problem message."""
        return self._message

    def msg(self) -> str:
        """The translated message."""
        return self._message.msg()

    def __repr__(self) -> str:
        return (
            f"[{self._level.upper()}] "
            f"{'/'.join([str(p) for p in self._source])} - "
            f"{self.msg()}"
        )


class Result(Generic[_T_co]):
    """Standard return value that can contain a value, errors, and
    warnings."""

    __slots__ = ("__value", "__problems", "__invalid")

    def __init__(
        self,
        *,
        value: Optional[_T_co],
        problems: Sequence[Problem],
        invalid: bool,
    ) -> None:
        if value is not None and invalid:
            raise ValueError("Invalid results must have None type values.")
        self.__value = value
        self.__problems = tuple(problems)
        self.__invalid = invalid

    @staticmethod
    def as_value(
        value: _T,
        *problems: Union[Problem, Iterable[Problem]],
    ) -> "Result[_T]":
        """Create a result with a valid value.

        It may contain problems, but if there are errors, the result
        will still be considered valid.
        """
        return Result(
            value=value,
            problems=_flatten_problems(problems),
            invalid=False,
        )

    @staticmethod
    def as_none(
        *problems: Union[Problem, Iterable[Problem]],
    ) -> "Result[None]":
        """Create a result with a valid value which is None."""
        if problems:
            # At least one problem argument.  Can't save memory.
            return Result(
                value=None,
                problems=_flatten_problems(problems),
                invalid=False,
            )
        # Save some memory.
        return _VALID_NONE_RESULT

    @staticmethod
    def as_error(
        *problems: Union[Problem, Iterable[Problem]],
    ) -> "Result[_T_co]":
        """Create an invalid result.

        There may not be any problems, or there may be problems with no
        errors, but the result is still marked as invalid.
        """
        return Result(value=None, problems=_flatten_problems(problems), invalid=True)

    @property
    def is_valid(self) -> bool:
        """Is this result a valid result?"""
        return not self.__invalid

    @property
    def is_not_valid(self) -> bool:
        """Is this result an invalid result?"""
        return self.__invalid

    @property
    def problems(self) -> Sequence[Problem]:
        """The underlying problems."""
        return self.__problems

    def map_to(self, callback: Callable[[_T_co], _R]) -> "Result[_R]":
        """Map the valid value to another type."""
        if self.__invalid:
            return cast(Result[_R], self)
        return Result(
            # Note: the value may be None if the _T is optional, but
            #   there are cases where it isn't optional, so we need to
            #   coerce a valid mapping.
            value=callback(cast(_T_co, self.__value)),
            problems=self.__problems,
            invalid=False,
        )

    def map_result(self, callback: "Callable[[_T_co], Result[_R]]") -> "Result[_R]":
        """Map the valid value to another result."""
        if self.__invalid:
            return cast(Result[_R], self)
        return callback(cast(_T_co, self.__value))

    def optional(self, default: Optional[_T_co] = None) -> Optional[_T_co]:
        """Get the value or the default if this is invalid."""
        if self.__invalid:
            return default
        return self.__value

    def required(self) -> _T_co:
        """If the value is asserted to be valid, then this returns the
        underlying value.  Otherwise, a runtime error is raised."""
        if self.__invalid:
            raise RuntimeError("Value is not valid.")
        return cast(_T_co, self.__value)


class ResultGen:
    """A builder for a Result."""

    __slots__ = ("__problems", "__invalid")

    def __init__(self) -> None:
        self.__problems: List[Problem] = []
        self.__invalid = False

    def is_valid(self) -> bool:
        """Is this generator valid right now?"""
        return not self.__invalid

    def is_not_valid(self) -> bool:
        """Is this generator not valid right now?"""
        return self.__invalid

    def force_valid(self) -> "ResultGen":
        """Force this generator to be valid."""
        self.__invalid = False
        return self

    def force_not_valid(self) -> "ResultGen":
        """Force this generator to be invalid."""
        self.__invalid = True
        return self

    def add(
        self,
        *values: Union[
            Problem,
            Iterable[Problem],
            Result[Any],
            Iterable[Result[Any]],
        ],
    ) -> "ResultGen":
        """Add problems or results to the builder.

        If any result is invalid, the result is then invalid.
        If any value is an error, the result is then invalid.
        """
        for value in values:
            if isinstance(value, collections.abc.Iterable):
                self.add(*value)
            elif isinstance(value, Problem):
                self.__problems.append(value)
                if value.is_error:
                    self.__invalid = True
            else:
                self.__problems.extend(value.problems)
                if value.is_not_valid:
                    self.__invalid = True
        return self

    @property
    def problems(self) -> Sequence[Problem]:
        """Get the current problem list."""
        return self.__problems

    def include(self, result: Result[_T], default: _T) -> _T:
        """Include the result in this generator and return its value.
        If it's invalid, return the default value in its place.
        """
        self.__problems.extend(result.problems)
        if result.is_not_valid:
            self.__invalid = True
            return default
        return result.required()

    def optional(self, result: Result[_T], default: Optional[_T] = None) -> Optional[_T]:
        """Include the result in this generator and return its value.
        If it's invalid, return the default value in its place.
        This can return None regardless of whether the result type is None or not.
        """
        self.__problems.extend(result.problems)
        if result.is_not_valid:
            self.__invalid = True
            return default
        return result.required()

    def build(self, value: _T) -> Result[_T]:
        """Build this generator into a result."""
        if self.__invalid:
            return Result.as_error(self.__problems)
        return Result.as_value(value, self.__problems)

    def build_with(self, callback: Callable[[], _T_co]) -> Result[_T_co]:
        """Build this generator into a result with the
        callback.  The callback is called only if the generator is valid."""
        if self.__invalid:
            return Result.as_error(self.__problems)
        return Result.as_value(callback(), self.__problems)

    def __repr__(self) -> str:
        return f"ResultGen({'invalid / ' if self.__invalid else ''}{len(self.__problems)})"


def _flatten_problems(
    problems: Sequence[Union[Problem, Iterable[Problem]]],
) -> Sequence[Problem]:
    if not problems:
        return ()

    ret: List[Problem] = []
    for problem in problems:
        if isinstance(problem, collections.abc.Iterable):
            ret.extend(problem)
        else:
            ret.append(problem)
    return ret


_VALID_NONE_RESULT: Result[None] = Result(value=None, problems=(), invalid=False)
