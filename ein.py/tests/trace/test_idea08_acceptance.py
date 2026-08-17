"""idea-08 trace-fidelity acceptance — S1.6.5 (M1 acceptance criterion #3).

Every named move in the human zebra walkthrough must correspond to a
named rule firing in the engine. The mapping is frozen in
``plans/m1_core_graph_reasoning/p1.6_rendering_and_trace/s1.6.5_idea08_checklist.md``
(structural, not literal — see the equivalence notes there).

Two levels:

- **always** — every rule the walkthrough names is *defined* in the
  zebra2 library (a fast, static regression on the rule library);
- **EIN_RUN_SLOW=1** (PyPy-friendly) — those rules actually *fire* on a
  zebra2 solve. The exhaustive zebra2 ``solve`` is ~35s on CPython, so it
  is gated, matching the existing slow-test convention.

Both levels are mirrored for ``examples/zebra.ein`` (S1.22.1a), which walks
the *same* human solution over one generic ``co-located`` relation instead of
five typed ones. The docs claim a rule-for-rule correspondence
(``docs/kernel/inference/README.md`` §The same inference over ONE generic
relation); these tests are what makes that claim a regression rather than
prose.
"""
from __future__ import annotations

import os
from pathlib import Path

import pytest

from ein.inference.monotonic import solve
from ein.ir import Atom, SForm, parse
from ein.kb import KnowledgeBase
from ein.kb.imports import resolve_imports

REPO = Path(__file__).resolve().parents[3]
ZEBRA2 = REPO / "examples" / "zebra2.ein"
ZEBRA = REPO / "examples" / "zebra.ein"

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


# ── the generic-link encoding (S1.22.1a) ───────────────────────────
# zebra.ein reaches the same conclusions over one `co-located` equivalence.
# The correspondence is documented in `docs/kernel/inference/README.md`; this
# is its machine-checkable form. Read it as the same table:
#
#   zebra2                                     zebra.ein
#   ─────────────────────────────────────────  ──────────────────────────
#   co-located (4-ary propagation)             slot-locate
#   functional-negative / injective-negative   slot-occupied
#   domain-elimination                         slot-elimination
#   range-elimination                          slot-fill
#   adjacent-via-{fwd,bwd}                     slot-adjacent-{fwd,bwd}
#   disjunctive-prune-{fwd,bwd}                slot-prune-{fwd,bwd}
#   adjacent-via-endpoint-{fwd,bwd}            slot-endpoint-{fwd,bwd}
#   (no counterpart — is-a is directed)        symmetric-negative
WALKTHROUGH_RULES_GENERIC = frozenset({
    "slot-locate", "slot-occupied", "slot-exclusive", "slot-negative",
    "slot-elimination", "slot-fill",
    "slot-adjacent-fwd", "slot-adjacent-bwd",
    "slot-prune-fwd", "slot-prune-bwd",
    "slot-endpoint-fwd", "slot-endpoint-bwd",
    "symmetric", "symmetric-negative",
})

# What must actually FIRE on a zebra.ein solve: the whole correspondence above,
# plus the two negative companions of the spatial propagation. Unlike zebra2 —
# whose firing target is a strict subset of its library — every inference rule
# this encoding provides fires on the solution path. The only exclusions are
# `slot-no-room` / `slot-no-fill`, which are ⊥-rules: they fire on dead
# branches, not on the path a solution records.
FIRING_TARGET_GENERIC = WALKTHROUGH_RULES_GENERIC | frozenset({
    "slot-adjacent-fwd-neg", "slot-adjacent-bwd-neg",
})


def _rule_names(path: Path) -> set[str]:
    # Rules the puzzle PROVIDES — resolve imports first, so rules promoted to
    # the stdlib (S1.8.A5-tail: symmetric/transitive/includes; S1.22.1a: the
    # whole std.slots stack) count as provided, not just the ones still defined
    # inline. (P1.7c: rules are flat top-level `(rule …)` / `(hrule …)` forms.)
    forms = resolve_imports(parse(path.read_text()), base_dir=path.parent)
    return {f.args[0].name for f in forms
            if isinstance(f, SForm) and f.head.name in ("rule", "hrule")
            and f.args and isinstance(f.args[0], Atom)}


# ── always-on: library coverage ────────────────────────────────────

def test_zebra2_library_defines_walkthrough_rules():
    missing = WALKTHROUGH_RULES - _rule_names(ZEBRA2)
    assert not missing, (
        f"walkthrough names rules absent from the zebra2 library: {sorted(missing)}"
    )


def test_zebra_generic_library_defines_walkthrough_rules():
    missing = WALKTHROUGH_RULES_GENERIC - _rule_names(ZEBRA)
    assert not missing, (
        "the generic-link encoding is missing counterparts of walkthrough "
        f"rules: {sorted(missing)}"
    )


# ── slow: firing coverage ──────────────────────────────────────────

@pytest.mark.skipif(
    not os.environ.get("EIN_RUN_SLOW"),
    reason="the exhaustive zebra2 solve is ~35s on CPython; "
           "set EIN_RUN_SLOW=1 or run via PyPy",
)
def test_zebra2_fires_walkthrough_rules():
    kb = KnowledgeBase.from_ir(parse(ZEBRA2.read_text()))
    verdict, _ = solve(kb, stop_after=None, max_set_size=3, store_lattice=True)
    fired = {f.rule for rec in verdict.proof.solutions for f in rec.firings}
    missing = FIRING_TARGET - fired
    assert not missing, (
        f"walkthrough rules that did not fire: {sorted(missing)}\n"
        f"fired: {sorted(fired)}"
    )


@pytest.mark.skipif(
    not os.environ.get("EIN_RUN_SLOW"),
    reason="the zebra.ein solve is ~3s on PyPy and much slower on CPython; "
           "set EIN_RUN_SLOW=1 or run via PyPy",
)
def test_zebra_generic_fires_walkthrough_rules():
    """The generic-link encoding reaches the walkthrough by its own rules.

    `stop_after=1` rather than an exhaustive run: this asserts over the
    *solution path*, and the exhaustive certification is the acceptance
    gate's job (`acceptance/test_zebra_two_ontologies.py`).
    """
    kb = KnowledgeBase.from_ir(parse(ZEBRA.read_text()))
    verdict, _ = solve(kb, stop_after=1, max_set_size=3, store_lattice=True)
    fired = {f.rule for rec in verdict.proof.solutions for f in rec.firings}
    missing = FIRING_TARGET_GENERIC - fired
    assert not missing, (
        f"walkthrough counterparts that did not fire: {sorted(missing)}\n"
        f"fired: {sorted(fired)}"
    )
