"""Built-in predicate registry — S1.3.1 T1.3.1.1.

Predicates are the kernel's structural logic primitives — they
appear inside ``:match`` clauses as guard premises (e.g.
``(neq ?a ?c)``) and are evaluated against the current bindings,
NOT looked up in the KB's facts. This is what distinguishes them
from ordinary relations: a relation's truth is data; a predicate's
truth is computed.

Q33 (resolved 2026-05-20) caps the M1 registry at **two predicates**:

- ``eq``  — `(eq ?a ?b)`  is true iff the two slots resolve equal.
- ``neq`` — `(neq ?a ?b)` is true iff the two slots resolve unequal.

All numeric / set / cardinality / variadic / aggregation primitives
are deferred to followups; they are NOT in this registry.

`not` is **not** a predicate — it is a structural wrapper handled by
the matcher (negation-as-failure on a sub-pattern). See
:mod:`ein.inference.match`.

The registry is consulted in two places:

1. **Loader** (:mod:`ein.kb.from_ir`) — before auto-vivifying an
   undeclared fact head as an open-world Relation, the loader checks
   ``is_predicate(head)`` to suppress phantom ``eq`` / ``neq``
   entries in :attr:`KnowledgeBase.relations`.
2. **Matcher** (:mod:`ein.inference.match`) — when compiling a
   ``:match`` sub-form whose head is a predicate name, the compiler
   emits a ``Guard`` opcode instead of a ``Scan``; the runtime
   evaluates the predicate function and either passes through the
   current bindings or prunes the branch.
"""
from __future__ import annotations

from collections.abc import Callable
from typing import Any

from .resolve import resolve_leaf

# A Predicate is a callable taking (bindings, args) and returning bool.
# `bindings` maps variable name → resolved value (str | int | Fact).
# `args` is the tuple of raw IR slot nodes from the rule body (Var,
# Atom, Int, or Fact — the compile pass does not pre-resolve them).
Predicate = Callable[[dict[str, Any], tuple[Any, ...]], bool]


def _eq(bindings: dict[str, Any], args: tuple[Any, ...]) -> bool:
    return resolve_leaf(args[0], bindings) == resolve_leaf(args[1], bindings)


def _neq(bindings: dict[str, Any], args: tuple[Any, ...]) -> bool:
    return resolve_leaf(args[0], bindings) != resolve_leaf(args[1], bindings)


_REGISTRY: dict[str, Predicate] = {
    "eq":  _eq,
    "neq": _neq,
}


def is_predicate(name: str) -> bool:
    """True iff `name` is a registered built-in predicate."""
    return name in _REGISTRY


def get(name: str) -> Predicate | None:
    """Return the predicate function for `name`, or None."""
    return _REGISTRY.get(name)


def register(name: str, fn: Predicate) -> None:
    """Register a new predicate.

    M1 ships with ``eq`` + ``neq`` only (Q33); ``register`` exists
    for followups (numeric / set / aggregation primitives land here
    when a future puzzle needs them).
    """
    _REGISTRY[name] = fn


def names() -> tuple[str, ...]:
    """All registered predicate names, sorted."""
    return tuple(sorted(_REGISTRY))


__all__ = ["Predicate", "get", "is_predicate", "names", "register"]
