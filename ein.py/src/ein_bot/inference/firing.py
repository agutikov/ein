"""Firing record + ``:assert`` substitution — S1.3.1 T1.3.1.5.

A :class:`Firing` records one successful rule application: the rule
that fired, the activator binding that authorised it, the variable
bindings produced by the matcher, the new :class:`Fact` it derived,
and the premise facts the matcher consumed.

The asserter (:func:`fire`) substitutes the bindings into the rule's
``:assert`` template, builds the derived :class:`Fact` (recursing
into :class:`NestedPattern` arg slots — Q40 Option A's nested-fact
construction), and writes it to the KB on the REASONING layer with
:class:`Provenance.from_rule` provenance threading the premise
fact-ids.

A negative assertion ``:assert (not X)`` produces a :class:`Fact`
with ``relation_name='not'`` and a single ``Fact``-typed arg — the
inner positive proposition. The contradiction detector (S1.3.3 /
P1.4 collapse, see :mod:`plans/ideas.md`) finds ``(X, (not X))``
pairs in the same layer.
"""
from __future__ import annotations

from dataclasses import dataclass
from typing import Any

from ein_bot.ir.types import Var
from ein_bot.kb.entities import Fact, Layer
from ein_bot.kb.provenance import Provenance
from ein_bot.kb.store import KnowledgeBase

from .compile import JoinPlan, NestedPattern
from .resolve import resolve_leaf

# ── Firing ─────────────────────────────────────────────────────────


@dataclass(frozen=True)
class Firing:
    """One rule application — successful or redundant.

    `derived` is the new fact written to the REASONING layer (or, if
    `redundant=True`, the pre-existing fact the matcher would have
    re-derived; no second insertion is performed).

    `redundant` (S1.3.3) marks a firing whose conclusion was already
    present in the KB. The matcher still produced the binding —
    pedagogically relevant for the trace renderer — but
    `kb.add_fact`'s dedupe returned the existing fact and the
    saturator chose to skip applying. The trace shows the firing
    "considered" without double-displaying the conclusion.
    """
    rule: str
    activator: tuple[str, ...]
    bindings: dict[str, Any]
    # S1.8.A13: the facts a single rule application concludes. One template →
    # a 1-tuple; a `:assert (and …)` → N facts, all sharing one Provenance (one
    # application fanning out to N derived nodes). A `redundant` firing's tuple
    # holds the pre-existing fact(s) the matcher would have re-derived.
    derived: tuple[Fact, ...]
    premises: tuple[Fact, ...]
    redundant: bool = False


# ── :assert substitution ───────────────────────────────────────────


def _unbound_var(slot: Var, bindings: dict[str, Any]) -> Any:
    """Fail-loud unbound-Var policy for ``:assert`` resolution.

    The matcher should never invoke firing with unbound vars in the
    assert template, so an unbound one is an invariant violation rather
    than a silent skip.
    """
    raise KeyError(
        f"unbound var ?{slot.name} in :assert — bindings: {bindings}",
    )


def _resolve(slot: object, bindings: dict[str, Any]) -> Any:
    """Walk a slot under bindings, resolving Vars to their bound values.

    Returns:
    - str / int / Fact for leaves (Var-bound or Atom/Int literals);
    - Fact for NestedPattern slots — constructed recursively.

    Raises KeyError if a Var is unbound (see :func:`_unbound_var`).
    Leaf cases delegate to the shared :func:`resolve_leaf`.
    """
    if isinstance(slot, NestedPattern):
        return Fact(
            relation_name=slot.relation,
            args=tuple(_resolve(a, bindings) for a in slot.arg_slots),
            layer=Layer.REASONING,
        )
    return resolve_leaf(slot, bindings, _unbound_var)


def build_fact(
    template: object,
    bindings: dict[str, Any],
    layer: Layer = Layer.REASONING,
    provenance: Provenance | None = None,
) -> Fact:
    """Construct the derived :class:`Fact` from an ``:assert`` template.

    The top-level template is expected to be a :class:`NestedPattern`
    (a fact-shaped form). Inner relational args become nested ``Fact``
    instances (Q40); leaf args become their resolved literals.
    """
    if not isinstance(template, NestedPattern):
        raise TypeError(
            f"expected NestedPattern at :assert top-level, got {type(template).__name__}",
        )
    return Fact(
        relation_name=template.relation,
        args=tuple(_resolve(a, bindings) for a in template.arg_slots),
        layer=layer,
        provenance=provenance,
    )


def fire(
    plan: JoinPlan,
    bindings: dict[str, Any],
    premises: tuple[Fact, ...],
    kb: KnowledgeBase,
) -> Firing | None:
    """Build the derived fact + provenance and write it to the KB.

    Returns the :class:`Firing` record on success. If the rule has
    no ``:assert`` template (defensive — shouldn't happen for a
    well-formed rule), returns None.
    """
    if not plan.assert_templates:
        return None

    # Stringify bindings for Provenance.bindings (tuple[(str, str), ...]).
    # Non-string values (int / Fact) are str()'d — losing detail but
    # keeping the provenance record stable across layers.
    binding_pairs: tuple[tuple[str, str], ...] = tuple(
        (k, str(v)) for k, v in bindings.items()
    )
    premises_raw = tuple(
        (p.relation_name, p.args) for p in premises
    )
    # One Provenance per application — every conclusion of this firing shares
    # it (S1.8.A13: one rule application → N derived facts, one node fanning out).
    provenance = Provenance.from_rule(
        rule=plan.rule_name,
        premises_raw=premises_raw,
        bindings=binding_pairs,
    )

    # Build + store each templated conclusion. add_and_index_fact dedups by
    # (relation_name, args), so an already-known conclusion is returned (and
    # indexed) once — a partially-novel multi-assert still records every fact.
    stored = tuple(
        kb.add_and_index_fact(build_fact(
            template, bindings, layer=Layer.REASONING, provenance=provenance,
        ))
        for template in plan.assert_templates
    )

    # Activator args as strings — for the Firing record only.
    activator = tuple(
        str(a) for a in plan.activator_args
    )
    return Firing(
        rule=plan.rule_name,
        activator=activator,
        bindings=dict(bindings),
        derived=stored,
        premises=premises,
    )


__all__ = ["Firing", "build_fact", "fire"]
