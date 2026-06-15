"""One-step contradiction lookahead — S1.5.6 T1.5.6.2.

A hypothesis-generation filter: before a candidate fact ``h`` is
emitted (and later forked + saturated by the lattice search), ask the
cheap question — *"does adding ``h`` to the already-saturated KB
produce a contradiction in a single rule firing?"*. If yes, drop
``h``; it would only fork, saturate, and die.

This is the index-based fast path the S1.5.4 "Topic B Tier B"
complaint asked for: paying for a fork + full saturation just to
learn a hypothesis dies in one rule step is wasteful.
:meth:`Lookahead.dies_immediately` runs *one* rule step instead.

This is a *hypgen-time* filter: it prunes a candidate at generation
time — before any fork or saturation — rather than discovering the
death only after forking and saturating a branch.

Mechanism
---------
The candidate ``h`` is injected into each ``:match`` premise whose
relation it unifies with; the rule's *other* premises are run
against the saturated KB through the ordinary matcher (no fork, no
mutation). For every resulting ``:assert`` fact ``f`` the engine
checks whether the would-be KB (``h`` + ``f`` added) holds a
contradiction the P1.4 ``ContradictionDetector`` would flag:

- ``f`` is ``(false)``                  — direct ⊥;
- ``f`` is ``(not h)``                  — ``h`` self-falsified;
- ``f`` is ``(not g)``, ``g`` already a REASONING-layer fact
                                        — a same-layer ``(g, ¬g)`` pair;
- ``f`` is positive and ``(not f)`` already a REASONING-layer fact
                                        — ``h`` derives a forbidden fact.

The REASONING-layer guards mirror the detector exactly: a
cross-layer ``(X, ¬X)`` (a derived ``(not g)`` against an authored
FACT-layer ``g``) is *not* a contradiction, so the lookahead must
not kill on it. The filter only ever *under*-approximates death —
a missed death just forks and dies normally; it never reports a
live hypothesis as dead.
"""
from __future__ import annotations

from ein_bot.kb.entities import Fact, Layer
from ein_bot.kb.store import KnowledgeBase

from . import match, primitives
from .compile import Join, JoinPlan, NestedPattern, Scan
from .engine import Engine
from .firing import build_fact
from .match import _bind_args


class Lookahead:
    """One-step contradiction simulator over a fixed rule set.

    Compile the rule plans once (``Lookahead(kb)``); reuse
    :meth:`dies_immediately` across every candidate. The compiled
    plans bake in activator-bound relation names, so a Lookahead
    built at the root KB stays valid for every fork (forks share
    the rule set).
    """

    def __init__(self, kb: KnowledgeBase) -> None:
        engine = Engine(kb)
        engine.compile_all()
        # `compile_all` walks `kb.rules` only — `(hrule …)` rules
        # live in `kb.hrules` and are never compiled here, so no
        # filtering is needed (S1.5.6b).
        self._plans: tuple[JoinPlan, ...] = tuple(engine.cache.values())

    def dies_immediately(self, kb: KnowledgeBase, h: Fact) -> bool:
        """True iff adding ``h`` to ``kb`` yields a one-step contradiction.

        ``kb`` is expected to be saturated — the engine only calls
        this on a post-saturation KB. Read-only: no fork, no
        mutation of ``kb``.
        """
        for plan in self._plans:
            # A rule may conclude SEVERAL facts (S1.8.A13) — any one could
            # contradict `h`, so probe every fact-shaped conclusion.
            fact_templates = [
                t for t in plan.assert_templates
                if isinstance(t, NestedPattern)
            ]
            if not fact_templates:
                continue
            for idx, step in enumerate(plan.steps):
                if not isinstance(step, (Scan, Join)):
                    continue
                if step.relation != h.relation_name:
                    continue
                # Inject `h` into this premise — unify its args.
                seed = _bind_args(
                    step.arg_slots, h.args, dict(plan.bindings_seed),
                )
                if seed is None:
                    continue
                # Run the rule's other premises against the KB; the
                # injected premise is supplied by `h`.
                rest = plan.steps[:idx] + plan.steps[idx + 1:]
                probe = JoinPlan(
                    rule_name=plan.rule_name,
                    activator_args=plan.activator_args,
                    bindings_seed=seed,
                    steps=rest,
                    why=plan.why,
                )
                for bindings, _premises in match.run(probe, kb):
                    for template in fact_templates:
                        try:
                            f = build_fact(template, bindings)
                        except (KeyError, TypeError):
                            # Defensive: a malformed assert template
                            # never kills a candidate.
                            continue
                        if _is_contradiction(kb, f, h):
                            return True
        return False


def _is_contradiction(kb: KnowledgeBase, f: Fact, h: Fact) -> bool:
    """True iff a KB holding ``h`` + the derived ``f`` is contradictory.

    Mirrors :class:`~ein_bot.inference.contradiction.ContradictionDetector`
    — the direct-⊥ shape and the same-layer ``(X, ¬X)`` pair. Both
    ``h`` and any rule-derived ``f`` live at the REASONING layer.
    """
    # Direct ⊥ — a `(false …)` fact.
    if f.relation_name == primitives.FALSE:
        return True
    # `f` is a negative `(not g)`.
    if f.relation_name == primitives.NOT:
        if not f.args or not isinstance(f.args[0], Fact):
            return False
        g = f.args[0]
        # `(not h)` against `h` — both REASONING, a guaranteed pair.
        if (g.relation_name, g.args) == (h.relation_name, h.args):
            return True
        # `(not g)` against an existing positive `g` — a pair only
        # when `g` is itself REASONING-layer (cross-layer is not a
        # contradiction; see the detector's module docstring).
        existing = kb._fact_by_id(g.relation_name, g.args)
        return existing is not None and existing.layer is Layer.REASONING
    # `f` is positive — `h` would derive it. A contradiction iff
    # `(not f)` already exists at the REASONING layer.
    if (f.relation_name, f.args) not in kb._negated_facts:
        return False
    neg = kb._fact_by_id(primitives.NOT, (f,))
    return neg is not None and neg.layer is Layer.REASONING


__all__ = ["Lookahead"]
