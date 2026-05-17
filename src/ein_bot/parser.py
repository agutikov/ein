"""Parse a conditions file into State.

The grammar is whitespace-delimited, one statement per line:

- empty line                    — skipped
- ``OBJ``                       — object declaration
- ``SRC REL... DST`` (≥3 tokens) — relation; all middle tokens form *REL*

Anything else (a 2-token line) is rejected with ``ValueError``.
"""
from __future__ import annotations

from collections.abc import Iterable, Iterator
from os import PathLike
from typing import TYPE_CHECKING

if TYPE_CHECKING:
    from .state import State


def parse(lines: Iterable[str]) -> Iterator[tuple[str, ...]]:
    """Yield ``(obj,)`` or ``(src, rel, dst)`` tuples per non-empty line."""
    for line in lines:
        v = line.strip().split()
        if not v:
            continue
        if len(v) == 1:
            yield (v[0],)
        elif len(v) >= 3:
            yield (v[0], " ".join(v[1:-1]), v[-1])
        else:
            raise ValueError(f'invalid line (2 tokens): "{line.rstrip()}"')


def load_into(state: State, lines: Iterable[str]) -> None:
    """Apply each parsed statement to *state* in order."""
    for tup in parse(lines):
        if len(tup) == 1:
            state.obj(tup[0])
        else:
            state.rel(*tup)


def load_file(state: State, path: str | PathLike[str]) -> None:
    """Read *path* and load it into *state*."""
    with open(path, encoding="utf-8") as f:
        load_into(state, f)
