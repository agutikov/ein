"""Stacked, copy-on-write versioning over :class:`State`.

A :class:`VersionedState` wraps an underlying ``State`` together with two
auxiliary deltas (``_additions``, ``_deletes``) for the partial-rewrite
work tracked in the 2021 PoC. The rewrite itself is deferred to the full
rewrite; this module preserves the original semantics — only the
attribute names of the original PoC have been clarified.
"""
from __future__ import annotations

from copy import deepcopy

from .state import State


class VersionedState:
    """A stackable copy-on-write layer over a base ``VersionedState`` or root."""

    def __init__(self, base: VersionedState | None = None) -> None:
        self._base: VersionedState | None = base
        self._readonly: bool = False
        if base is not None:
            self._level: int = base._level + 1
            base._readonly = True
            self._state: State = deepcopy(base._state)
        else:
            self._level = 0
            self._state = State()
        self._additions: State = State()
        self._deletes: State = State()

    # ------------------------------------------------------------------

    @property
    def state(self) -> State:
        return self._state

    @property
    def level(self) -> int:
        return self._level

    @property
    def readonly(self) -> bool:
        return self._readonly

    def inc_version(self) -> VersionedState:
        """Push a new editable layer on top; current layer becomes read-only."""
        return VersionedState(self)

    # ------------------------------------------------------------------

    def dump(self, level: int = 0) -> str:
        """Pretty-print the chain. TODO: complete in full rewrite."""
        out = ""
        if self._base is not None:
            out = self._base.dump(level - 1)
        out += f"\n# Level {level}\n\n"
        out += self._state.dump()
        return out

    def dot(self, colorfull: bool = True) -> str:
        """Render the current layer as DOT. TODO: cross-layer rendering."""
        return self._state.dot(colorfull=colorfull)
