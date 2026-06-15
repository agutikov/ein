"""Rule-body / ⊥ structural primitives — the kernel's reserved
non-relation vocabulary, declared in one place (S1.7.25 T1.7.25.1).

These are the structural forms a rule body is built from — *not*
relations (their truth is not data in the KB) and *not* predicates
(``eq`` / ``neq``, computed guards — those live in
:mod:`ein.inference.predicates`). They are the calculus the
compiler, matcher, and contradiction detector interpret directly.

The point of this module is purely to **name the vocabulary once**:
each primitive has a name constant + a one-line semantic + the
canonical site that implements its deep behaviour. The scattered
``head_name == "and"`` / ``_facts_by_relation["not"]`` literals across
``compile.py`` / ``contradiction.py`` / ``lookahead.py`` /
``solver.py`` / ``hypgen.py`` reference these constants, so
adding/auditing a kernel primitive is a one-file change.
The deep behaviour stays where it lives (this is the ``predicates.py``
*registry* pattern, not a behavioural move).

The M1 kernel primitives are ``not`` ``false`` ``and`` ``or`` ``absent``.
``open`` / ``forall`` used to live here too as compile-time *sugar*; since
P1.8 S1.5.9 they are ordinary ein-lang ``(macro …)`` declarations (the ``std.macro``
module, ``ein/stdlib/macro.ein``) expanded at load time, so they are no
longer kernel vocabulary and no longer appear in this registry.

See ``docs/kernel/ir/03-ein-lang/06_reserved_names.md`` (surface) and
``plans/m1_core_graph_reasoning/p1.7_bootstrapping_zebra/s1.7.25_reserved_names_encapsulate_document.md``.
"""
from __future__ import annotations

from dataclasses import dataclass

from ein.inference import predicates

# ── Name constants (M1 kernel rule-body / ⊥ calculus) ──────────────
NOT    = "not"
FALSE  = "false"
AND    = "and"
OR     = "or"
ABSENT = "absent"


@dataclass(frozen=True)
class Primitive:
    """One structural primitive's metadata (name + meaning + site)."""
    name:   str
    arity:  str          # human-readable ("1", "0+", "2+", "3")
    role:   str          # one-line semantic
    site:   str          # canonical implementing site


_REGISTRY: dict[str, Primitive] = {
    NOT: Primitive(
        NOT, "1",
        "propositional negation; `(not X)` is a stored octagon fact whose "
        "single arg is the negated proposition",
        "matcher (`match.py`) + contradiction detector (`contradiction.py`)",
    ),
    FALSE: Primitive(
        FALSE, "0+",
        "direct ⊥ — `(false)` asserts the firing rule reached a "
        "contradiction (args empty by convention)",
        "contradiction detector (`contradiction.py`)",
    ),
    AND: Primitive(
        AND, "2+",
        "conjunction; flattened into sibling premises of the same plan",
        "compiler (`compile.py:_compile_premise`)",
    ),
    OR: Primitive(
        OR, "2+",
        "disjunction; a top-level `(or …)` in a `:match` is lowered to one "
        "match plan per disjunct at COMPILE time (S1.8.A13), on one rule",
        "compiler (`compile._match_disjuncts` / `compile_rule`) + guard",
    ),
    ABSENT: Primitive(
        ABSENT, "1",
        "negation-as-failure on a sub-pattern (`AbsentGuard`)",
        "compiler (`compile.py`) + matcher (`match.py`)",
    ),
}

# The kept rule-body / ⊥ primitives (the kernel's structural vocabulary).
STRUCTURAL: frozenset[str] = frozenset(_REGISTRY)


def is_structural(name: str) -> bool:
    """True iff `name` is a structural rule-body / ⊥ primitive."""
    return name in STRUCTURAL


def non_object_names() -> frozenset[str]:
    """Reserved names that are NEVER guessable objects — the rule-body /
    ⊥ primitives plus the computed predicates (`eq` / `neq`). Consumed by
    :func:`ein.inference.hypgen._candidate_objects` so the blind
    enumerator never proposes a primitive name as a graph object."""
    return STRUCTURAL | frozenset(predicates.names())


def get(name: str) -> Primitive | None:
    """The :class:`Primitive` metadata for `name`, or None."""
    return _REGISTRY.get(name)


def names() -> tuple[str, ...]:
    """All declared primitive names, sorted."""
    return tuple(sorted(_REGISTRY))


__all__ = [
    "ABSENT", "AND", "FALSE", "NOT", "OR",
    "STRUCTURAL",
    "Primitive", "get", "is_structural", "names", "non_object_names",
]
