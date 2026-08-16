"""Pinning tests for the `absent` (NAF) semantics doc — P1.21 S1.21.4.

Each test pins one edge of the semantics stated in
``docs/kernel/inference/absent_semantics.md`` (the section it pins is
named in its docstring). Every test asserts *current* behaviour — they are
executable law for the doc, not aspirations.

**S1.21.8 rewrote four of them.** The closure/worlds split moved `(absent …)`
off the closure and onto the boundary, so:

- P1 (``test_guard_is_judged_against_the_positive_fixpoint``) and P2
  (``test_priority_swap_does_not_change_outcome``) now agree with each other:
  a guard is judged against the positive fixpoint, so rule priority no longer
  decides what is derivable. P1's old outcome — ``q`` in the closure while
  ``(absent (q ?x))`` had licensed ``p`` — was the anomaly being removed.
- P6 (``test_lookahead_naf_world_includes_candidate``) flipped: the D3
  divergence is fixed, and the lookahead's guard is evaluated in the world
  that includes the candidate.
- P8 (``test_or_disjunct_absent_is_evaluated_on_the_boundary``) was a
  ``xfail(strict=True)`` pinning the D5 soundness gap. It passes now, and
  the doc's D5 paragraph retires with it.

P4 (``test_mutual_naf_picks_queue_order``) deliberately did NOT change — see
its docstring for why boundary admission is one candidate per round.

Probes P1..P8 are from the investigation report
``plans/m1_core_graph_reasoning/p1.21_review_response/reports/r4_absent_semantics.md`` §2.
"""
from __future__ import annotations

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


def test_guard_is_judged_against_the_positive_fixpoint():
    """§Evaluation points. S1.21.8 — the deliberate flip of
    ``test_fire_then_arrive_keeps_both``.

    That test pinned the anomaly: the gate (priority 100) popped before
    ``derive-q`` (priority 200) could populate the watched relation, its
    ``absent`` held at that moment, and the later arrival retracted nothing —
    so the final KB held BOTH ``p`` and ``q``, i.e. ``q`` was in the closure
    while ``(absent (q ?x))`` had licensed ``p``.

    The closure/worlds split removes it. ``derive-q`` is purely positive and
    runs in the closure; the gate carries a guard and is parked until the
    closure quiesces. By then ``(q A)`` exists, the guard fails on the
    boundary, and the gate is never admitted. ``W ⊭ ∃x̄.Pθ`` is now literal:
    the world the guard is asked about IS the closure.
    """
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
    assert _rels(sat.kb, "p") == set()
    assert _rels(sat.kb, "q") == {("A",)}
    assert sat.naf_dropped == 0
    assert sat.naf_rounds >= 1          # the boundary was consulted


# ── P2 → §Evaluation points (the result no longer moves with priority) ─


def test_priority_swap_does_not_change_outcome():
    """§Evaluation points. S1.21.8 — the deliberate flip of
    ``test_priority_swap_changes_outcome``.

    The SAME program as P1 with the two priorities swapped. It used to yield
    a *different* model — ``q`` arrived before the gate popped, the fire-time
    re-eval dropped the firing, and the result was defined by priority bands
    plus FIFO order rather than by the program.

    Now both orderings agree (``{q}``, no ``p``): the closure is positive and
    runs to a fixpoint no matter which rule has the lower band, and the guard
    is judged once against that fixpoint. Priority-band discipline is demoted
    from load-bearing to advisory — it still decides *firing order*, but no
    longer decides *what is derivable*.
    """
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
    assert sat.naf_dropped == 0


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
    equal priority — the first-declared rule fires, and the second is
    never admitted.

    S1.21.8 kept this outcome, and it is the reason boundary admission is
    **one candidate per round**. Both guards pass against the quiesced world
    (neither ``p`` nor ``q`` exists yet), so admitting the whole batch would
    derive BOTH — and ``{p, q}`` is not a model of this program under any
    reading. Admitting one, then re-quiescing, re-asks ``derive-q``'s guard
    in a world that now holds ``p``, where it correctly fails.

    Note what did NOT survive: the *mechanism*. The loser is no longer
    "dropped by a fire-time re-eval" — there is no fire-time re-eval, and
    ``naf_dropped`` is 0. It is simply never admitted.
    """
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
    assert sat.naf_dropped == 0
    assert sat.naf_admitted == 1       # one admitted, one never admitted


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


def test_lookahead_naf_world_includes_candidate():
    """§Divergences D3 — **fixed** by S1.21.8; this is the flip the old
    ``test_lookahead_naf_world_excludes_candidate`` predicted.

    The rule ``false ← (cand ?x) ∧ (absent (cand ?x))`` can never fire in any
    real match — its premises are jointly unsatisfiable in one world — yet
    ``dies_immediately`` used to report the candidate dead, because the probe
    posited ``h`` into the positive premise while running the guard against a
    KB *without* ``h``. The NAF answered about a different world than the
    probe hypothesised, killing a live hypothesis in violation of the
    lookahead's own "never reports a live hypothesis as dead" contract.

    The guard is now evaluated in the world ``kb`` with ``h`` added: it must find no
    match in ``kb`` **and** ``h`` must not create one. Here ``h`` is exactly
    ``(cand A)``, so it creates one, the guard fails, and the candidate
    survives.
    """
    kb = _kb("""
    (rule self-block ()
      :match (and (cand ?x) (absent (cand ?x)))
      :assert (false)
      :why "unsatisfiable in any one world" :priority 100)
    (relation cand T)
    """)
    list(Saturator(kb).saturate())
    h = Fact("cand", ("A",), layer=Layer.REASONING)
    assert Lookahead(kb).dies_immediately(kb, h) is False


def test_lookahead_still_kills_on_a_positive_rule():
    """The D3 fix must not disarm the filter: a purely positive rule that
    derives ``(false)`` from the candidate still kills it."""
    kb = _kb("""
    (rule blow-up ()
      :match (and (cand ?x) (bad ?x))
      :assert (false)
      :why "cand + bad is absurd" :priority 100)
    (relation cand T)
    (relation bad T)
    (bad A :source "(1)")
    """)
    list(Saturator(kb).saturate())
    assert Lookahead(kb).dies_immediately(
        kb, Fact("cand", ("A",), layer=Layer.REASONING)) is True
    assert Lookahead(kb).dies_immediately(
        kb, Fact("cand", ("B",), layer=Layer.REASONING)) is False


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


def test_or_disjunct_absent_is_evaluated_on_the_boundary():
    """§Divergences D5 — **fixed** by S1.21.8; the strict xfail now passes.

    The gate matches via the second ``(or …)`` disjunct
    ``(and (t2 ?x) (absent (r2 ?x)))``, and ``derive-r2`` populates ``r2``.
    The firing must not happen. It used to, because the disjunct's guard
    lived in ``plan.extra_match_plans`` and the fire-time re-check walked
    ``plan.steps`` only — a soundness gap that needed *remembering* to walk
    one more tuple.

    It is closed structurally rather than by remembering: guards are lifted
    per disjunct into ``plan.naf_guards``, and every match is produced by
    ``match.run_guarded``, which yields a match together with **its own
    disjunct's** guards. There is no longer a tuple a caller could forget.
    """
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
        "the or-disjunct's AbsentGuard was not evaluated on the boundary: "
        "(r2 A) is in the quiesced world, so the gate must not be admitted."
    )
    assert sat.naf_dropped == 0     # never admitted, rather than dropped
