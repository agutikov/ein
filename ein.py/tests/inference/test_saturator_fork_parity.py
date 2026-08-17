"""Saturator fork parity — S1.5a.2a T1.5a.2a.1.b.

`Saturator(kb).saturate()` must produce the same fact set whether
the caller saturates `kb` directly or `kb.fork()`. Pre-S1.5a.2a
the two paths diverged because `engine._activators_for(rule)`
consulted `rule.applications`, which delegates to the rule's
load-time `_kb` (the *parent* KB after `fork`) — runtime-derived
activator facts on the fork were invisible to plan compilation,
so the rules they would have triggered never enqueued.

The fix routes `_activators_for` through the engine's own
`self.kb._rule_apps_by_rule`. This test pins the parity so the
bug can't reappear without a loud failure.
"""
from __future__ import annotations

import json
import os
import subprocess
import sys
from collections import Counter
from pathlib import Path

from ein.inference.canon import state_key
from ein.inference.closed import emit_closed
from ein.inference.saturator import Saturator
from ein.inference.world import World
from ein.ir import parse
from ein.kb.store import KnowledgeBase

REPO = Path(__file__).resolve().parents[3]
ZEBRA2 = REPO / "examples" / "zebra2.ein"


def _saturate(kb: KnowledgeBase) -> tuple[KnowledgeBase, int]:
    """Run the standard root pipeline on `kb`; return (kb, firing_count)."""
    emit_closed(kb)
    firings = list(Saturator(kb).saturate())
    return kb, len(firings)


def test_saturate_direct_and_fork_produce_same_facts():
    """Same input, same saturation regardless of fork-vs-direct.

    The reproducer the bug surfaced through was zebra2's
    `adjacent-via-fwd` + `disjunctive-prune`: their activators are
    runtime-derived (by `derive-adjacent-via-fwd` etc.). On the
    fork path, plan compilation read the parent's empty activator
    map and never compiled the rules; 5 disjunctive-prune firings
    + 1 cascade vanished.
    """
    text = ZEBRA2.read_text()

    direct, n_direct = _saturate(KnowledgeBase.from_ir(parse(text)))
    forked, n_forked = _saturate(KnowledgeBase.from_ir(parse(text)).fork())

    assert n_direct == n_forked, (
        f"firing-count divergence: direct={n_direct}, fork={n_forked}"
    )

    c_direct = Counter(f.relation_name for f in direct.facts)
    c_forked = Counter(f.relation_name for f in forked.facts)
    diffs = {
        k: (c_direct.get(k, 0), c_forked.get(k, 0))
        for k in set(c_direct) | set(c_forked)
        if c_direct.get(k) != c_forked.get(k)
    }
    assert not diffs, f"fact-count divergence by relation: {diffs}"


def test_runtime_derived_activator_compiles_on_fork():
    """A minimal reproducer: a rule whose activator is produced by
    another rule's firing must still compile on the fork.

    Setup:
    - `meta-derive` fires on `(trigger ?x)` facts and asserts
      `(target ?x)` — activators for the `target` rule.
    - `target` rule fires on `(trigger ?y)` premise and asserts
      `(done ?y)`.
    - With `(trigger X)` declared, the chain meta-derive → target
      should fire and produce `(done X)`.
    """
    text = """
    (rule meta-derive ()
      :match  (trigger ?x)
      :assert (target ?x)
      :why    "trigger ⟹ target activator"
      :priority 100)
    (rule target (?x)
      :match  (trigger ?y)
      :assert (done ?y)
      :why    "trigger fires target"
      :priority 200)
    (relation trigger T)
    (relation done T)
    (relation target T)
    (trigger X :source "(1)")
    """
    kb = KnowledgeBase.from_ir(parse(text))
    fork = kb.fork()
    emit_closed(fork)
    list(Saturator(fork).saturate())
    done = [f for f in fork.facts if f.relation_name == "done"]
    assert any(f.args == ("X",) for f in done), (
        "runtime-derived activator (target X) didn't trigger the target "
        "rule on the forked KB"
    )


# ── S1.22.0 — parity + determinism of the S1.21.8 boundary state ───────


_FORALL = """
(relation row T)
(relation cell T T)
(relation blocked T)
(relation full T)
(relation done T)

; `(full ?r)` iff EVERY cell of ?r is blocked — a forall, i.e. a nested
; absent, so its guard is non-monotone and its candidates stay parked.
(rule all-blocked ()
  :match  (and (row ?r) (absent (and (cell ?r ?c) (absent (blocked ?c)))))
  :assert (full ?r) :why "every cell of {?r} is blocked" :priority 200)
(rule finish ()
  :match  (and (full ?r) (absent (done ?r)))
  :assert (done ?r) :why "close {?r}" :priority 300)

(row R1 :source "(1)") (row R2 :source "(2)")
(cell R1 C1 :source "(3)") (cell R1 C2 :source "(4)")
(cell R2 C3 :source "(5)")
(blocked C1 :source "(6)") (blocked C2 :source "(7)")
"""


def _boundary_observables(kb: KnowledgeBase) -> dict:
    """Everything the S1.21.8 two-phase loop exposes, plus the fact set."""
    sat = Saturator(kb)
    firings = list(sat.saturate())
    return {
        "firings": len(firings),
        "naf_rounds": sat.naf_rounds,
        "naf_admitted": sat.naf_admitted,
        "naf_retired": sat.naf_retired,
        "naf_dropped": sat.naf_dropped,
        "parked_left": len(sat._parked),
        "facts": state_key(kb),
    }


def test_fork_parity_extends_to_the_boundary():
    """S1.22.0 (angle C1). The existing parity test pins the fact SET; the
    boundary added state of its own (`_parked`, `_park_stamp`, `naf_rounds`,
    `naf_admitted`, `naf_retired`), and a fork must agree on all of it.

    On a `forall`-shaped fixture rather than zebra2's: a nested absent is the
    one guard shape that can flip fail -> pass, so it is the shape whose
    candidates park, get re-judged, and are never retired.
    """
    direct = _boundary_observables(KnowledgeBase.from_ir(parse(_FORALL)))
    forked = _boundary_observables(KnowledgeBase.from_ir(parse(_FORALL)).fork())
    assert direct == forked

    # The fixture must actually exercise the boundary, or the parity is vacuous.
    assert direct["naf_rounds"] > 1
    assert direct["naf_admitted"] > 0
    assert direct["parked_left"] > 0          # R2 stays a standing question
    assert {a for r, a in direct["facts"] if r == "full"} == {("R1",)}


def test_boundary_state_does_not_leak_into_state_key():
    """S1.22.0 (angle C3). Boundary admission order decides the answer on a
    non-stratified rule set, so none of the state that drives it may reach
    lattice identity — two branches that saturate to the same model must
    still collapse to one node.

    `state_key` is `(relation_name, args)`-only by S1.21.1, so this holds by
    construction; pinned so a future field cannot quietly leak in.
    """
    kb = KnowledgeBase.from_ir(parse(_FORALL))
    sat = Saturator(kb)
    list(sat.saturate())
    before = state_key(kb)

    sat.naf_rounds = 999
    sat.naf_admitted = 999
    sat.naf_retired = 999
    sat.naf_dropped = 999
    sat._park_stamp[12345] = (7, 7)
    kb._alt_justifications.setdefault(("full", ("R1",)), ())

    assert state_key(kb) == before
    assert all(len(entry) == 2 for entry in before)   # (relation, args) only


# The classic unstratifiable program, at ONE priority band: `p <- absent q`
# and `q <- absent p`. Both guards pass against the empty world, and
# `_admit_from_boundary` admits exactly one candidate per round — so which of
# the two stable models the engine lands in is decided purely by enqueue
# order. Nothing else in the suite makes that order observable.
_UNSTRATIFIED = """
(relation seed T)
(relation p T)
(relation q T)
(rule mk-p () :match (and (seed ?x) (absent (q ?x)))
  :assert (p ?x) :why "no q" :priority 100)
(rule mk-q () :match (and (seed ?x) (absent (p ?x)))
  :assert (q ?x) :why "no p" :priority 100)
(seed A :source "(1)")
"""


def test_admission_order_decides_the_model_on_a_non_stratified_program():
    """Precondition for the determinism pin below — and the divergence
    `absent_semantics.md` §Divergence introduced by the fix documents.

    If this ever became order-*insensitive*, the seed sweep would be pinning
    nothing.
    """
    kb = KnowledgeBase.from_ir(parse(_UNSTRATIFIED))
    sat = Saturator(kb)
    list(sat.saturate())
    derived = {r for r in ("p", "q") if kb._fact_by_id(r, ("A",)) is not None}
    assert derived == {"p"}, (
        "exactly one of the two stable models must be produced, and which "
        "one is decided by boundary admission order"
    )
    assert sat.naf_retired == 1      # the loser is retired, not left parked


def test_boundary_admission_is_not_hash_derived():
    """S1.22.0 (angle C2). `_parked` is a heap on `(priority, tiebreaker)`
    and the tiebreaker follows enqueue order, which follows `cache.values()`
    / `pos_index` iteration — none of it hash-derived. Since S1.21.8 the
    admission order *decides the answer* on non-stratified rule sets
    (`absent_semantics.md` §Divergence introduced by the fix), so a
    hash-order dependence would be a nondeterministic solver.

    Run on the unstratified program, where the answer itself moves with the
    order, and on the `forall` fixture, which parks and re-judges.
    """
    script = "\n".join((
        "import json, sys",
        f"sys.path.insert(0, {str(REPO / 'ein.py' / 'src')!r})",
        "from ein.ir import parse",
        "from ein.kb.store import KnowledgeBase",
        "from ein.inference.saturator import Saturator",
        "from ein.inference.canon import state_key",
        "out = []",
        f"for text in json.loads({json.dumps([_UNSTRATIFIED, _FORALL])!r}):",
        "    kb = KnowledgeBase.from_ir(parse(text))",
        "    sat = Saturator(kb)",
        "    fs = [(f.rule, sorted((d.relation_name, d.args)"
        "           for d in f.derived)) for f in sat.saturate()]",
        "    out.append([fs, repr(state_key(kb)),",
        "                [sat.naf_rounds, sat.naf_admitted, sat.naf_retired]])",
        "print(json.dumps(out))",
    ))

    results = set()
    for seed in ("0", "1", "42", "987654321"):
        env = {**os.environ, "PYTHONHASHSEED": seed}
        out = subprocess.run(
            [sys.executable, "-c", script],
            capture_output=True, text=True, env=env, check=True,
        ).stdout
        results.add(out.strip())
    assert len(results) == 1, (
        f"boundary admission varies with PYTHONHASHSEED: {len(results)} "
        f"distinct outcomes across seeds"
    )


def test_the_live_engine_never_populates_world_commitment():
    """S1.22.0 (angle C4) — `World.commitment` is deliberately inert.

    A `World` is documented as branch-relative: `absent(P)` means "P does not
    follow from the givens **and this commitment**". The saturator always
    builds `World(self.kb)`, and that is correct rather than an oversight —
    in a branch, `self.kb` IS the fork, whose facts already include the
    committed hypotheses, so the query is branch-relative by construction.
    The parameter carries the contract for readers; nothing reads it.

    Wiring it was considered and rejected: no consumer exists, and the one
    that would want it — branch-relative *negative provenance* — needs a
    field on `Provenance`, not on `World`. This pin makes populating it a
    deliberate act: whoever does must say what reads it.
    """
    seen: list[tuple] = []
    original = World.__init__

    def traced(self, kb, commitment=()):
        seen.append(commitment)
        original(self, kb, commitment)

    World.__init__ = traced
    try:
        kb = KnowledgeBase.from_ir(parse(_FORALL))
        list(Saturator(kb).saturate())
    finally:
        World.__init__ = original

    assert seen, "fixture built no World at all — the pin would be vacuous"
    assert not any(seen), (
        "the live engine now populates World.commitment; decide what reads "
        "it (see this test's docstring) rather than leaving it inert"
    )
