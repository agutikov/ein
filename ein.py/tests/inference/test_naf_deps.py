"""Static NAF dependency map — S1.7.4 (`ein.inference.naf_deps`).

A rule's ``(absent …)`` guard *depends on a derived relation* when some
rule positively asserts the watched relation (or, Scope B, asserts its
``(not …)`` for a ``(absent (not …))`` guard). Such a rule's NAF is sound
only because of the S1.5a.1 fire-time re-evaluation; one watching a
declared-only relation behaves identically with or without it.

The map is read off a compile cache, so for puzzles whose NAF-bearing
rules have rule-derived activators (zebra2's ``adjacent-via-*``, the
elimination rules) the cache must be populated by an initial saturation
first — see :func:`test_zebra2_post_saturation_map`.
"""
from __future__ import annotations

from pathlib import Path

import pytest

from ein.inference import naf_deps
from ein.inference.config import SolverConfig
from ein.inference.engine import Engine
from ein.inference.monotonic import solve
from ein.inference.naf_deps import DerivedNafWarning
from ein.inference.saturator import Saturator
from ein.ir import parse
from ein.kb.store import KnowledgeBase

REPO_ROOT = Path(__file__).resolve().parents[3]
ZEBRA2 = REPO_ROOT / "examples" / "zebra2.ein"


def _kb(text: str) -> KnowledgeBase:
    return KnowledgeBase.from_ir(parse(text))


def _map(text: str) -> list[naf_deps.NafDep]:
    """NAF map of a freshly-loaded puzzle (param-less rules compile at
    load — no saturation needed for these unit fixtures)."""
    engine = Engine(_kb(text))
    engine.compile_all()
    return engine.naf_dependency_map()


def _dep(deps: list[naf_deps.NafDep], rule: str) -> naf_deps.NafDep:
    [d] = [d for d in deps if d.rule_name == rule]
    return d


# ── Unit: positive NAF, declared-only vs derived ─────────────────────

_DECLARED_ONLY = """
(rule probe ()
  :match  (and (seed ?a ?b) (absent (target ?a ?b)))
  :assert (out ?a ?b) :why "p" :priority 100)
(relation seed   T T)
(relation target T T)
(relation out    T T)
(seed A B :layer fact)
"""

_DERIVED = """
(rule probe ()
  :match  (and (seed ?a ?b) (absent (target ?a ?b)))
  :assert (out ?a ?b) :why "p" :priority 100)
(rule mk-target ()
  :match  (src ?a ?b)
  :assert (target ?a ?b) :why "m" :priority 100)
(relation seed   T T)
(relation target T T)
(relation out    T T)
(relation src    T T)
(seed A B :layer fact)
"""


def test_declared_only_naf_not_flagged():
    """`target` is asserted by no rule — probe's NAF is declared-only."""
    dep = _dep(_map(_DECLARED_ONLY), "probe")
    assert dep.derived == ()
    assert dep.declared_only == ("target",)


def test_derived_naf_flagged():
    """`mk-target` asserts `target`, so probe's NAF is derived."""
    dep = _dep(_map(_DERIVED), "probe")
    assert dep.derived == ("target",)
    assert dep.declared_only == ()


# ── Unit: negated NAF — Scope B (the (not (R …)) case) ───────────────

_NEG_DECLARED_ONLY = """
(rule probe ()
  :match  (and (seed ?a ?b) (absent (not (target ?a ?b))))
  :assert (out ?a ?b) :why "p" :priority 100)
(relation seed   T T)
(relation target T T)
(relation out    T T)
(seed A B :layer fact)
"""

_NEG_DERIVED = """
(rule probe ()
  :match  (and (seed ?a ?b) (absent (not (target ?a ?b))))
  :assert (out ?a ?b) :why "p" :priority 100)
(rule mk-neg ()
  :match  (src ?a ?b)
  :assert (not (target ?a ?b)) :why "n" :priority 100)
(relation seed   T T)
(relation target T T)
(relation out    T T)
(relation src    T T)
(seed A B :layer fact)
"""


def test_negated_naf_declared_only_not_flagged():
    """`(not target)` asserted by no rule → declared-only negated watch."""
    dep = _dep(_map(_NEG_DECLARED_ONLY), "probe")
    assert dep.derived == ()
    assert dep.declared_only == ("(not target)",)


def test_negated_naf_derived_flagged():
    """`mk-neg` asserts `(not (target …))` → the negated watch is derived
    (Scope B): a `(absent (not target))` over a rule-negated relation
    leans on the fire-time re-eval just like a positive derived NAF."""
    dep = _dep(_map(_NEG_DERIVED), "probe")
    assert dep.derived == ("(not target)",)
    assert dep.declared_only == ()


# ── Unit: warning emission (default off; explicit on) ────────────────


def test_emit_warns_on_derived_dep():
    engine = Engine(_kb(_DERIVED))
    engine.compile_all()
    with pytest.warns(DerivedNafWarning, match=r"probe.*target"):
        flagged = naf_deps.emit_derived_naf_warnings(engine.cache)
    assert [d.rule_name for d in flagged] == ["probe"]


def test_emit_silent_on_declared_only():
    """No flagged dep → no warning fired (safe under filterwarnings=error)
    → returns the empty list."""
    engine = Engine(_kb(_DECLARED_ONLY))
    engine.compile_all()
    assert naf_deps.emit_derived_naf_warnings(engine.cache) == []


# ── Wired path: SolverConfig flag → solve() → _phase1_root emit ──────

_SOLVE_DERIVED = _DERIVED + "\n(query :goal (out ?x ?y))\n"


def test_solve_emits_when_flag_on():
    """The flag wires through `solve` → `_phase1_root`'s post-saturation
    emit. `probe`'s NAF over the rule-asserted `target` fires the warning
    during root saturation."""
    kb = _kb(_SOLVE_DERIVED)
    with pytest.warns(DerivedNafWarning, match=r"probe"):
        solve(kb, config=SolverConfig(warn_derived_naf=True), stop_after=1)


def test_solve_silent_by_default():
    """Default config leaves the flag off — `solve` completes without a
    DerivedNafWarning (which would raise under filterwarnings=error)."""
    kb = _kb(_SOLVE_DERIVED)
    verdict, _ = solve(kb, stop_after=1)  # no raise == no warning
    assert verdict is not None


# ── Smoke: zebra2, post-initial-saturation ───────────────────────────

# Every NAF-bearing rule in zebra2 watches a derived relation under
# Scope B: the spatial rules' next-to activators (next-to derived via
# symmetric + includes), the typechecks' is-a* (transitive closure), and
# the forall/totality rules' (not (*-loc …)) (derived by the *-negative
# companions). The right-of activators of the spatial rules are the
# declared-only siblings (see test_spatial_split_by_activator).
_EXPECTED_DERIVED_RULES = {
    "adjacent-via-fwd",
    "adjacent-via-bwd",
    "adjacent-via-fwd-negative",
    "adjacent-via-bwd-negative",
    "disjunctive-prune-fwd",
    "disjunctive-prune-bwd",
    "adjacent-via-endpoint-fwd",
    "adjacent-via-endpoint-bwd",
    "typecheck-arg-0",
    "typecheck-arg-1",
    "domain-elimination",
    "range-elimination",
    "total",
    "surjective",
}


def _zebra2_saturated_cache():
    kb = KnowledgeBase.from_ir(parse(ZEBRA2.read_text(encoding="utf-8")))
    sat = Saturator(kb)
    list(sat.saturate())  # populates the cache with rule-derived activators
    return sat.engine.cache


def test_zebra2_post_saturation_map():
    deps = naf_deps.compute_naf_map(_zebra2_saturated_cache())
    flagged = {d.rule_name for d in deps if d.derived}
    assert flagged == _EXPECTED_DERIVED_RULES


def test_zebra2_load_time_map_incomplete():
    """A map taken before saturation misses the rule-derived-activator
    rules — documents *why* the warning site is post-saturation."""
    kb = KnowledgeBase.from_ir(parse(ZEBRA2.read_text(encoding="utf-8")))
    engine = Engine(kb)
    engine.compile_all()
    flagged = {d.rule_name for d in engine.naf_dependency_map() if d.derived}
    # The spatial / elimination rules have no activators yet, so they are
    # absent from the load-time map.
    assert "adjacent-via-fwd" not in flagged
    assert flagged < _EXPECTED_DERIVED_RULES


def test_spatial_split_by_activator():
    """`adjacent-via-fwd` is derived-NAF on its next-to activator but
    declared-only on its right-of activator — the per-(rule, activator)
    granularity is load-bearing."""
    deps = naf_deps.compute_naf_map(_zebra2_saturated_cache())
    fwd = [d for d in deps if d.rule_name == "adjacent-via-fwd"]
    by_spatial = {d.activator_args[0]: d for d in fwd}
    assert by_spatial["next-to"].derived == ("next-to",)
    assert by_spatial["right-of"].derived == ()
    assert by_spatial["right-of"].declared_only == ("right-of",)


def test_zebra2_negated_naf_label():
    """The totality/elimination rules surface a `(not <*-loc>)` derived
    label — the Scope-B negated watch."""
    deps = naf_deps.compute_naf_map(_zebra2_saturated_cache())
    total = [d for d in deps if d.rule_name == "total" and d.derived]
    assert total
    assert all(
        any(lbl.startswith("(not ") for lbl in d.derived) for d in total
    )
