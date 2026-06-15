"""Shared leaf-slot resolution for the inference runtime.

Both rule firing (``:assert`` template construction in :mod:`firing`)
and predicate evaluation (``:eq`` / ``:neq`` in :mod:`predicates`)
resolve a raw IR slot — a :class:`Var`, :class:`Atom`, or :class:`Int`
— to its bound value or literal payload under a ``bindings`` dict.

The two call sites differ only in how they treat an **unbound** ``Var``:
firing fails loud (the matcher guarantees no unbound vars reach an
``:assert`` template, so an unbound one is an invariant violation),
while a predicate resolves it to ``None`` to make the failure visible
without crashing. That single divergence is the ``on_unbound`` policy
argument; the ``Var`` / ``Atom`` / ``Int`` / pass-through core is shared
so the two implementations cannot drift — historically they were
copy-pasted into the two modules with *swapped* argument order
(F-KER-7), a latent footgun this consolidation removes.
"""
from __future__ import annotations

from collections.abc import Callable
from typing import Any

from ein.ir.types import Atom, Int, Var

# Policy invoked for an unbound Var: receives the slot and the bindings
# (the latter lets a fail-loud policy report the full binding context).
UnboundPolicy = Callable[[Var, dict[str, Any]], Any]


def _return_none(slot: Var, bindings: dict[str, Any]) -> None:
    """Lenient default: an unbound Var resolves to ``None``."""
    return None


def resolve_leaf(
    slot: object,
    bindings: dict[str, Any],
    on_unbound: UnboundPolicy = _return_none,
) -> Any:
    """Resolve a leaf IR slot under ``bindings``.

    - ``Var``  — its bound value; ``on_unbound(slot, bindings)`` if absent.
    - ``Atom`` — the atom's name (str).
    - ``Int``  — the integer value.
    - anything else (an already-resolved str / int / Fact, or a compound
      the caller peels off before delegating here) — returned as-is.
    """
    if isinstance(slot, Var):
        if slot.name in bindings:
            return bindings[slot.name]
        return on_unbound(slot, bindings)
    if isinstance(slot, Atom):
        return slot.name
    if isinstance(slot, Int):
        return slot.value
    return slot
