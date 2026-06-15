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

from collections import Counter
from pathlib import Path

from ein.inference.closed import emit_closed
from ein.inference.saturator import Saturator
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
