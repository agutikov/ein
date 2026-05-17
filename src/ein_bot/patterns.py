"""Compile a user-supplied "pattern" into a predicate.

A *pattern* selects names (object names, relation names). It can be:

- ``None``                              — matches everything
- a callable ``str -> bool``            — used as-is
- a ``str``                             — compiled as a regex (``re.search``)
- a ``list``, ``set`` or ``tuple``      — membership test against the set
"""
from __future__ import annotations

import re
from collections.abc import Callable, Iterable

Pattern = None | str | Iterable[str] | Callable[[str], bool]


def compile_predicate(pattern: Pattern) -> Callable[[str], bool]:
    """Return a predicate ``str -> bool`` realising *pattern*.

    Raises ``TypeError`` for unsupported pattern types.
    """
    if pattern is None:
        return lambda _x: True
    if callable(pattern):
        return pattern
    if isinstance(pattern, str):
        rx = re.compile(pattern)
        return lambda x: rx.search(x) is not None
    if isinstance(pattern, (list, set, tuple)):
        names = set(pattern)
        return lambda x: x in names
    raise TypeError(f"unsupported pattern: {pattern!r}")
