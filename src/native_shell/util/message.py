"""User messages."""

from typing import Iterable, Mapping, Union, Optional, NewType
from datetime import datetime, time, date
import gettext


SimpleUserMessageData = Union[str, int, float, bool, datetime, time, date, None, BaseException]
# Iterables must be of a unified type.
ListUserMessageData = Union[
    Iterable[str],
    Iterable[int],
    Iterable[float],
    Iterable[bool],
    Iterable[datetime],
    Iterable[time],
    Iterable[date],
]
UserMessageData = Union[
    SimpleUserMessageData,
    Mapping[str, SimpleUserMessageData],
    ListUserMessageData,
]
I18n = NewType("I18n", str)


def i18n(message: str) -> I18n:
    """
    Localization message string function, for message extraction.  This should generally be
    imported `as _` so that the localization helper tool can do its magic.
    :param message:
    :return:
    """
    return I18n(message)


class UserMessage:
    """
    A message that can be displayed to the end-user.
    """

    __slots__ = ("__message", "__args", "__cached")

    def __init__(
        self,
        message: I18n,
        **arguments: UserMessageData,
    ) -> None:
        self.__message = message
        # Note that, because the arguments are passed as key-values, this is not
        # a dictionary that the end-user can modify.  To conserve memory and speed up the
        # processing, the original mapping is stored.
        self.__args = arguments
        self.__cached: Optional[str] = None

    def msg(self) -> str:
        """Translate the message for display."""
        if self.__cached is None:
            self.__cached = gettext.gettext(self.__message).format(**self.__args)
        return self.__cached

    @property
    def message(self) -> I18n:
        """The message text, which should be localized."""
        return self.__message

    @property
    def arguments(self) -> Mapping[str, UserMessageData]:
        """The programmatic arguments for the message."""
        return dict(self.__args)

    def debug(self) -> str:
        """Return a "debug" version of the user message, meaning that it isn't translated."""
        return self.__message.format(**self.__args)

    def __repr__(self) -> str:
        msg = repr(self.__message)
        parts = ", ".join([f"{k}={repr(v)}" for k, v in self.__args.items()])
        return f"UserMessage({msg}, {parts})"

    def __eq__(self, other: object) -> bool:
        if not isinstance(other, UserMessage):
            return False
        return other.message == self.__message and other.arguments == self.__args

    def __ne__(self, other: object) -> bool:
        return not self.__eq__(other)
