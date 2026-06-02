"""idea-08 trace-fidelity acceptance — S1.6.5 (M1 acceptance criterion #3).

Every named move in the human zebra walkthrough must correspond to a
named rule firing in the engine. The mapping is frozen in
``plans/m1_core_graph_reasoning/p1.6_rendering_and_trace/s1.6.5_idea08_checklist.md``
(structural, not literal — see the equivalence notes there).

Two levels:

- **always** — every rule the walkthrough names is *defined* in the
  zebra2 library (a fast, static regression on the rule library);
- **EIN_RUN_SLOW=1** (PyPy-friendly) — those rules actually *fire* on a
  zebra2 solve. zebra2 gaps_solve is ~35s on CPython, so it is gated,
  matching the existing slow-test convention.
"""
from __future__ import annotations

import os
from pathlib import Path

import pytest

from ein_bot.inference.monotonic import gaps_solve
from ein_bot.ir import Atom, SForm, parse
from ein_bot.kb import KnowledgeBase

REPO = Path(__file__).resolve().parents[3]
ZEBRA2 = REPO / "examples" / "zebra2.ein"

# Rules the idea-08 / examples/README.md walkthrough names (∩ the
# zebra2 library). The frozen regression target, per the checklist.
WALKTHROUGH_RULES = frozenset({
    "adjacent-via-fwd", "co-located",
    "disjunctive-prune-bwd", "disjunctive-prune-fwd",
    "domain-elimination", "range-elimination",
    "functional", "symmetric", "total",
})

# The rules that should actually FIRE on a zebra2 solve (the property
# rules `functional`/`total` surface as their consequences — see the
# checklist's structural-equivalence notes — so the firing target maps
# `functional` → `functional-negative` and adds the `-bwd` dual).
FIRING_TARGET = frozenset({
    "adjacent-via-fwd", "adjacent-via-bwd", "co-located",
    "domain-elimination", "range-elimination",
    "disjunctive-prune-fwd", "disjunctive-prune-bwd",
    "functional-negative", "symmetric",
})


def _zebra2_rule_names() -> set[str]:
    # P1.7c: rules are flat top-level `(rule …)` / `(hrule …)` forms.
    forms = parse(ZEBRA2.read_text())
    return {f.args[0].name for f in forms
            if isinstance(f, SForm) and f.head.name in ("rule", "hrule")
            and f.args and isinstance(f.args[0], Atom)}


# ── always-on: library coverage ────────────────────────────────────

def test_zebra2_library_defines_walkthrough_rules():
    missing = WALKTHROUGH_RULES - _zebra2_rule_names()
    assert not missing, (
        f"walkthrough names rules absent from the zebra2 library: {sorted(missing)}"
    )


# ── slow: firing coverage ──────────────────────────────────────────

@pytest.mark.skipif(
    not os.environ.get("EIN_RUN_SLOW"),
    reason="zebra2 gaps_solve is ~35s on CPython; set EIN_RUN_SLOW=1 or run via PyPy",
)
def test_zebra2_fires_walkthrough_rules():
    kb = KnowledgeBase.from_ir(parse(ZEBRA2.read_text()))
    verdict, _ = gaps_solve(kb, max_set_size=3, store_lattice=True)
    fired = {f.rule for rec in verdict.proof.solutions for f in rec.firings}
    missing = FIRING_TARGET - fired
    assert not missing, (
        f"walkthrough rules that did not fire: {sorted(missing)}\n"
        f"fired: {sorted(fired)}"
    )
