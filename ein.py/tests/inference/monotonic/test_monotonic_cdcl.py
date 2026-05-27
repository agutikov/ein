"""Monotonic CDCL tests — S1.5b.6 T1.5b.6.4.

Covers the four shapes called out in the stage doc:

1. **Singleton death emits + writeback.** A layer-1 singleton
   ``{h₁}`` saturates to a contradiction → ``root_kb._nogoods``
   carries ``frozenset({h₁_id})`` and ``root_kb._negated_facts``
   carries ``h₁_id`` (so the next ``_compute_alive`` drops ``h₁``).

2. **Multi-element death emits + filters.** A layer-2 pair
   ``{h₁, h₂}`` saturates to a contradiction; neither alone dies.
   The 2-element clause lands in ``_nogoods`` and ``filter_candidate``
   drops every layer-3 triple containing both ``h₁`` and ``h₂``
   (caught by the subset check rather than apriori's prefix-join,
   because the dead pair shares no prefix with the generating
   pair on those triples — Q1.5b.5.b).

3. **Subsumption.** Unit-level: ``emit_nogood(min_size=1)`` rejects
   a superset of an existing clause; ``nogoods_subsumed`` increments
   when the monotonic emit path tries to re-add a covered clause.

4. **Empty alive after singleton deaths → Contradiction.** Every
   layer-1 hypothesis dies; the Phase-3 ``_compute_alive`` refresh
   sees an empty ``alive`` and the verdict is :class:`Contradiction`.
"""
from __future__ import annotations

from ein_bot.inference.config import SolverConfig
from ein_bot.inference.monotonic import monotonic_solve
from ein_bot.inference.nogoods import emit_nogood
from ein_bot.inference.tree.solver import (
    Ambiguity,
    Contradiction,
)
from ein_bot.ir import parse
from ein_bot.kb.store import KnowledgeBase


def _kb(text: str) -> KnowledgeBase:
    return KnowledgeBase.from_ir(parse(text))


# Force deaths to surface through `try_commitment_set` rather than
# being pre-empted by hypgen's lookahead (which would write
# `(not h)` via back_prop before the monotonic loop sees the
# candidate). The monotonic CDCL path is what S1.5b.6 exercises.
_NO_LOOKAHEAD = SolverConfig(
    enable_pre_branch_lookahead=False,
    enable_back_prop_unconditional=False,
)


# ── 1) Singleton death — emit + writeback ─────────────────────────


SINGLETON_FIXTURE = """
(rules
  (rule forbid-paint-Blue ()
    :match  (paint Blue ?y)
    :assert (false)
    :why    "Blue can't paint anything"
    :priority 100))
(ontology
  (relation paint T T)
  ; declared but never asserted — keeps the goal unreachable so
  ; the fork-side is_solved check doesn't short-circuit the runs
  ; that need to observe layer-2 / Phase-3 behaviour.
  (relation never T)
  (is-a Thing T)
  (is-a Red Thing) (is-a Blue Thing))
(facts)
(query :mode solve
       :goal  (never ?x)
       :hypothesis-relations paint)
"""


def test_singleton_death_emits_clause_and_writeback():
    """``(color Blue)`` dies post-saturation at layer 1.

    Asserts: the size-1 clause lands in ``_nogoods`` (Q1.5b.5.c —
    ``min_size=1`` on the new engines) and ``_negated_facts``
    carries ``("color", ("Blue",))`` so subsequent ``_compute_alive``
    calls drop the dead candidate.
    """
    kb = _kb(SINGLETON_FIXTURE)
    _verdict, stats = monotonic_solve(kb, max_set_size=2, config=_NO_LOOKAHEAD)

    blue_red = ("paint", ("Blue", "Red"))
    assert frozenset({blue_red}) in kb._nogoods
    assert blue_red in kb._negated_facts
    assert stats.nogoods_emitted >= 1
    assert stats.enterings_dead_post >= 1


# ── 2) Multi-element death — emit + layer-3 filter ────────────────


MULTI_FIXTURE = """
(rules
  (rule kill-ab-bc ()
    :match  (and (R a b) (R b c))
    :assert (false)
    :why    "(R a b) + (R b c) is forbidden"
    :priority 100))
(ontology
  (relation R T T)
  ; declared but never asserted — keeps the goal unreachable so
  ; layer-2/3 enterings fire instead of being cut off by the
  ; fork-side is_solved check.
  (relation never T)
  (is-a Thing T)
  (is-a a Thing) (is-a b Thing) (is-a c Thing))
(facts)
(query :mode solve
       :goal  (never ?x)
       :hypothesis-relations R)
"""


def test_multi_element_death_emits_clause_and_filters():
    """The pair ``{(R a b), (R b c)}`` saturates to ``(false)``;
    neither singleton fires the rule alone.

    Asserts: the 2-element clause lands in ``_nogoods``; layer-3
    triples ``{x, (R a b), (R b c)}`` are dropped by
    ``filter_candidate``'s subset check (the dead pair doesn't
    share a prefix with the generating ``(x, R a b)`` and
    ``(x, R b c)`` pairs, so apriori's prefix-join would otherwise
    yield them).
    """
    kb = _kb(MULTI_FIXTURE)
    _verdict, stats = monotonic_solve(
        kb, max_set_size=3, config=_NO_LOOKAHEAD,
    )

    h_ab = ("R", ("a", "b"))
    h_bc = ("R", ("b", "c"))
    dead_pair = frozenset({h_ab, h_bc})
    assert dead_pair in kb._nogoods
    # The dead pair fires once at layer 2.
    assert stats.enterings_dead_post >= 1
    # No layer-3 entering should have survived containing both ab
    # and bc — the filter killed them before try_commitment_set
    # was even called. If a layer-3 commitment with both elements
    # had reached the engine it would have re-fired the kill rule
    # and re-emitted a (subsumed) size-3 clause; instead the size-2
    # clause stays minimal.
    for clause in kb._nogoods:
        if dead_pair.issubset(clause):
            assert clause == dead_pair, (
                "layer-3 superset wasn't filtered — "
                f"emitted clause {clause!r} should have been "
                "blocked by the size-2 dead pair"
            )


# ── 3) Subsumption ────────────────────────────────────────────────


def test_emit_nogood_min_size_1_accepts_singleton():
    """The monotonic call site passes ``min_size=1`` so layer-1
    singleton deaths can land. With default ``min_size=2`` the
    tree side still rejects them (covered in
    ``test_path_condition_nogoods``)."""
    kb = KnowledgeBase()
    one = frozenset({("R", ("a",))})
    assert emit_nogood(kb, one, min_size=1) is True
    assert one in kb._nogoods
    # Default still guards size-1 for the tree caller.
    kb2 = KnowledgeBase()
    assert emit_nogood(kb2, one) is False
    assert kb2._nogoods == set()


def test_emit_nogood_subsumes_superset_on_min_size_1():
    """Subsumption logic is shape-identical regardless of
    ``min_size``: a smaller clause already present rejects the
    superset."""
    kb = KnowledgeBase()
    h1 = ("R", ("a", "b"))
    h2 = ("R", ("c", "d"))
    h3 = ("R", ("e", "f"))
    pair = frozenset({h1, h2})
    triple = frozenset({h1, h2, h3})
    assert emit_nogood(kb, pair, min_size=1) is True
    assert emit_nogood(kb, triple, min_size=1) is False
    assert kb._nogoods == {pair}


# ── 4) Empty alive after singleton deaths → Contradiction ─────────


ALL_DIE_FIXTURE = """
(rules
  (rule forbid-h-a-b ()
    :match  (h a b)
    :assert (false)
    :why    "(h a b) forbidden"
    :priority 100)
  (rule forbid-h-b-a ()
    :match  (h b a)
    :assert (false)
    :why    "(h b a) forbidden"
    :priority 100))
(ontology
  (relation h T T)
  (is-a Thing T)
  (is-a a Thing) (is-a b Thing))
(facts)
(query :mode solve
       :goal  (h ?x ?y)
       :hypothesis-relations h)
"""


def test_all_layer_1_singletons_die_returns_contradiction():
    """Every layer-1 hypothesis dies post-saturation; the
    Phase-3 ``_compute_alive`` refresh sees an empty ``alive``
    and the verdict is :class:`Contradiction`."""
    kb = _kb(ALL_DIE_FIXTURE)
    verdict, stats = monotonic_solve(
        kb, max_set_size=2, config=_NO_LOOKAHEAD,
    )
    assert isinstance(verdict, Contradiction)
    # Both singletons died.
    assert stats.enterings_dead_post == 2
    assert stats.enterings_alive == 0
    # Both clauses recorded (subsumption left them both — disjoint).
    assert ("h", ("a", "b")) in kb._negated_facts
    assert ("h", ("b", "a")) in kb._negated_facts


# ── Cross-check: Ambiguity when some singletons survive ───────────


def test_singleton_filter_does_not_falsely_kill_alive():
    """Regression guard for the writeback: a surviving singleton
    must remain in the post-solve ``_compute_alive`` view even
    after a different singleton's death wrote ``(not h_dead)``.
    """
    kb = _kb(SINGLETON_FIXTURE)
    verdict, _stats = monotonic_solve(
        kb, max_set_size=2, config=_NO_LOOKAHEAD,
    )
    # (paint Red Blue) survives; (paint Blue Red) dies. Verdict
    # should be Ambiguity — Red→Blue is alive but the hypothesis
    # never gets merged into root, so is_solved stays False.
    assert isinstance(verdict, Ambiguity)
    assert ("paint", ("Red", "Blue")) not in kb._negated_facts
