"""Pinning tests for the `absent` (NAF) semantics doc — P1.21 S1.21.4.

Each test pins one edge of the semantics stated in
``docs/kernel/inference/absent_semantics.md`` (the section it pins is
named in its docstring). Seven tests assert *current* behaviour — they
are executable law for the doc, not aspirations. The eighth
(``test_or_disjunct_absent_not_reevaluated_at_fire_time``) pins the
**D5 divergence** as ``xfail(strict=True)``: it asserts the *sound*
behaviour, currently fails (the engine's ``absents_still_pass`` walks
``plan.steps`` only, skipping ``extra_match_plans`` guards), and will
flip loudly when the engine fix lands — at which point the doc's D5
paragraph must be retired together with the marker.

Probes P1..P8 are from the investigation report
``plans/m1_core_graph_reasoning/p1.21_review_response/reports/r4_absent_semantics.md`` §2.
"""
from __future__ import annotations

import pytest

from ein.inference.commitment import try_commitment_set
from ein.inference.lookahead import Lookahead
from ein.inference.saturator import Saturator
from ein.ir import parse
from ein.kb.entities import Fact, Layer
from ein.kb.store import KnowledgeBase


def _kb(text: str) -> KnowledgeBase:
    return KnowledgeBase.from_ir(parse(text))


def _rels(kb: KnowledgeBase, name: str) -> set[tuple]:
    return {f.args for f in kb.facts if f.relation_name == name}


# ── P1 → §Evaluation points (E3: never after the firing) ───────────


def test_fire_then_arrive_keeps_both():
    """§Evaluation points (E3). A firing whose ``absent`` held at fire
    time is never revisited: the gate (priority 100) fires before
    ``derive-q`` (priority 200) populates the watched relation, and the
    later arrival retracts nothing — the final KB holds BOTH ``p`` and
    ``q``. This also rules out closed-world-over-the-final-closure:
    ``q`` is in the closure, yet ``(absent (q ?x))`` licensed ``p``."""
    sat = Saturator(_kb("""
    (rule gate ()
      :match (and (seed ?x) (absent (q ?x)))
      :assert (p ?x)
      :why "p unless q" :priority 100)
    (rule derive-q ()
      :match (t ?x)
      :assert (q ?x)
      :why "derive q" :priority 200)
    (relation seed T)
    (relation t T)
    (relation p T)
    (relation q T)
    (seed A :source "(1)")
    (t A :source "(2)")
    """))
    list(sat.saturate())
    assert _rels(sat.kb, "p") == {("A",)}
    assert _rels(sat.kb, "q") == {("A",)}
    assert sat.naf_dropped == 0


# ── P2 → §Explicitly not provided (order-defined result) ───────────


def test_priority_swap_changes_outcome():
    """§Explicitly not provided. The SAME program as P1 with the two
    priorities swapped yields a different model: ``q`` now arrives
    before the gate pops, the fire-time re-eval (E2) drops the firing,
    and ``p`` is never derived. On non-stratified programs the result
    is defined by priority bands + FIFO order — operational, not
    model-theoretic."""
    sat = Saturator(_kb("""
    (rule gate ()
      :match (and (seed ?x) (absent (q ?x)))
      :assert (p ?x)
      :why "p unless q" :priority 200)
    (rule derive-q ()
      :match (t ?x)
      :assert (q ?x)
      :why "derive q" :priority 100)
    (relation seed T)
    (relation t T)
    (relation p T)
    (relation q T)
    (seed A :source "(1)")
    (t A :source "(2)")
    """))
    list(sat.saturate())
    assert _rels(sat.kb, "p") == set()
    assert _rels(sat.kb, "q") == {("A",)}
    assert sat.naf_dropped == 1


# ── P3 → §Explicitly not provided (no stable-model discipline) ─────


def test_unstratified_loop_converges():
    """§Explicitly not provided. ``p ← absent q; q ← p`` has NO stable
    model (the reduct never reproduces the candidate set), yet the
    engine accepts it and converges to ``{p, q}`` — the fixpoint is
    *supported at fire time*, not stable. E3: once ``p`` fired, the
    ``q`` it enables does not retract it."""
    sat = Saturator(_kb("""
    (rule derive-p ()
      :match (and (seed ?x) (absent (q ?x)))
      :assert (p ?x)
      :why "p unless q" :priority 100)
    (rule derive-q ()
      :match (p ?x)
      :assert (q ?x)
      :why "q from p" :priority 100)
    (relation seed T)
    (relation p T)
    (relation q T)
    (seed A :source "(1)")
    """))
    list(sat.saturate())
    assert _rels(sat.kb, "p") == {("A",)}
    assert _rels(sat.kb, "q") == {("A",)}


# ── P4 → §Explicitly not provided (no stratification check) ────────


def test_mutual_naf_picks_queue_order():
    """§Explicitly not provided. ``p ← absent q; q ← absent p`` is
    unstratifiable and has TWO stable models ({p} and {q}); the engine
    accepts it and deterministically picks one by FIFO tiebreak at
    equal priority — the first-declared rule fires, the second is
    dropped by the fire-time re-eval (E2)."""
    sat = Saturator(_kb("""
    (rule derive-p ()
      :match (and (seed ?x) (absent (q ?x)))
      :assert (p ?x)
      :why "p unless q" :priority 100)
    (rule derive-q ()
      :match (and (seed ?x) (absent (p ?x)))
      :assert (q ?x)
      :why "q unless p" :priority 100)
    (relation seed T)
    (relation p T)
    (relation q T)
    (seed A :source "(1)")
    """))
    list(sat.saturate())
    assert _rels(sat.kb, "p") == {("A",)}
    assert _rels(sat.kb, "q") == set()
    assert sat.naf_dropped == 1


# ── P5 → §Worlds / C6 (absent is world-relative) ───────────────────


def test_absent_is_branch_relative():
    """§Worlds / C6. The same ground query — ``(absent (r A B))`` —
    answers differently in different worlds: in the fork carrying the
    commitment ``{(r A B)}`` the guard fails and ``gated`` is never
    derived; in root (no commitment) it passes and ``gated`` is
    derived. ``absent`` is a world-relative query, never a ground atom
    whose value could be cached or written back."""
    kb = _kb("""
    (rule gate ()
      :match (and (seed ?x) (absent (r A B)))
      :assert (gated ?x)
      :why "gated unless (r A B)" :priority 100)
    (relation seed T)
    (relation r T T)
    (relation gated T)
    (seed A :source "(1)")
    """)
    # Fork world: root + the commitment hypothesis, saturated.
    result = try_commitment_set(kb, (("r", ("A", "B")),))
    assert result.kind == "alive"
    assert _rels(result.kb, "gated") == set()
    # Root world: same program, no commitment.
    list(Saturator(kb).saturate())
    assert _rels(kb, "gated") == {("A",)}


# ── P6 → §Divergences (D3: lookahead's NAF world excludes h) ───────


def test_lookahead_naf_world_excludes_candidate():
    """§Divergences D3 — pins CURRENT behaviour, not an endorsement.
    The rule ``false ← (cand ?x) ∧ (absent (cand ?x))`` can never fire
    in any real match (its premises are jointly unsatisfiable in one
    world), yet ``dies_immediately`` reports the candidate dead: the
    probe posits ``h`` into the positive premise while running the
    ``AbsentGuard`` against the KB *without* ``h`` — the NAF queries a
    different world than the probe hypothesises. Recorded as open
    question D3 in the P1.21 phase README; if the lookahead world is
    ever unified, this assertion flips to ``False``."""
    kb = _kb("""
    (rule self-block ()
      :match (and (cand ?x) (absent (cand ?x)))
      :assert (false)
      :why "unsatisfiable in any one world" :priority 100)
    (relation cand T)
    """)
    list(Saturator(kb).saturate())
    h = Fact("cand", ("A",), layer=Layer.REASONING)
    assert Lookahead(kb).dies_immediately(kb, h) is True


# ── P7 → §Definition (inner free vars are existential) ─────────────


def test_absent_nested_and_is_existential():
    """§Definition. In ``(absent (and (g ?x ?y) (h ?y)))`` the guard
    fails iff SOME extension of the outer bindings matches the whole
    conjunction — ``absent`` is ¬∃ over the sub-pattern's unbound vars.
    A witness pair exists for A only, so ``ok`` is derived for B
    only."""
    sat = Saturator(_kb("""
    (rule gate ()
      :match (and (seed ?x) (absent (and (g ?x ?y) (h ?y))))
      :assert (ok ?x)
      :why "ok unless a g-h witness exists" :priority 100)
    (relation seed T)
    (relation g T T)
    (relation h T)
    (relation ok T)
    (seed A :source "(1)")
    (seed B :source "(2)")
    (g A W :source "(3)")
    (h W :source "(4)")
    """))
    list(sat.saturate())
    assert _rels(sat.kb, "ok") == {("B",)}


# ── P8 → §Divergences (D5: or-disjunct guards skip fire-time) ──────


@pytest.mark.xfail(
    strict=True,
    reason="D5 (P1.21 R4): absents_still_pass walks plan.steps only, so "
    "AbsentGuards inside extra_match_plans or-disjuncts (S1.8.A13) get no "
    "fire-time re-eval — the firing commits on a stale NAF verdict. "
    "Candidate fix: also walk plan.extra_match_plans. When the engine fix "
    "lands this xfail flips (strict) — retire the doc's D5 paragraph with it.",
)
def test_or_disjunct_absent_not_reevaluated_at_fire_time():
    """§Divergences D5 — asserts the SOUND behaviour (currently xfail).
    The gate matches via the second ``(or …)`` disjunct
    ``(and (t2 ?x) (absent (r2 ?x)))``; ``derive-r2`` (priority 100)
    populates ``r2`` before the gate (priority 200) pops. Sound
    fire-time semantics (E2) must drop the firing exactly as in P2 —
    today it fires, because the disjunct's guard lives in
    ``plan.extra_match_plans`` which ``absents_still_pass`` never
    walks."""
    sat = Saturator(_kb("""
    (rule gate ()
      :match (or (and (t1 ?x) (absent (r1 ?x)))
                 (and (t2 ?x) (absent (r2 ?x))))
      :assert (gated ?x)
      :why "gated via either NAF disjunct" :priority 200)
    (rule derive-r2 ()
      :match (raw ?x)
      :assert (r2 ?x)
      :why "derive r2" :priority 100)
    (relation t1 T)
    (relation t2 T)
    (relation r1 T)
    (relation r2 T)
    (relation raw T)
    (relation gated T)
    (t2 A :source "(1)")
    (raw A :source "(2)")
    """))
    list(sat.saturate())
    assert _rels(sat.kb, "r2") == {("A",)}, "precondition: r2 derived first"
    assert _rels(sat.kb, "gated") == set(), (
        "gate fired on a stale NAF verdict: (r2 A) was present at fire "
        "time but the or-disjunct's AbsentGuard was not re-evaluated."
    )
    assert sat.naf_dropped >= 1
