"""d=0 negative-completion — S1.5a.19 T1.5a.19.6.

The six rules shipped in `examples/zebra2.ein` (commit `455bfd6`)
close the negative-derivation gap at depth 0 — given
functional/injective constraints + positives, or adjacency
activators alone, derive the negatives the NL Zebra walkthrough
names at d=0. Pre-S1.5a.19 those negatives only surfaced via
fork + saturate + back-prop on a refuted candidate; the rules
let saturation land them directly. See
`docs/kernel/inference/README.md` § "d=0 negative-completion
(S1.5a.19)" for the family overview.

Each test below builds a minimal KB containing the rule under
test + its `derive-…` activator + just enough ontology to make
the rule fire, then saturates and asserts on
``kb._negated_facts``.
"""
from __future__ import annotations

from ein_bot.inference.saturator import Saturator
from ein_bot.ir import parse
from ein_bot.kb.store import KnowledgeBase


def _saturated(text: str) -> KnowledgeBase:
    kb = KnowledgeBase.from_ir(parse(text))
    list(Saturator(kb).saturate())
    return kb


# ── functional-negative ─────────────────────────────────────────────

_FUNCTIONAL_NEG = """
(rules
  (rule functional-negative (?R)
    :match  (and (?R ?a ?b)
                 (relation ?R ?A ?B)
                 (is-a ?b_other ?B)
                 (neq ?b_other ?b))
    :assert (not (?R ?a ?b_other))
    :why    "functional negative"
    :priority 240)
  (rule derive-functional-negative ()
    :match  (functional ?R)
    :assert (functional-negative ?R)
    :why    "functional ⟹ functional-negative active."
    :priority 100))
(ontology
  (relation color-loc Color House)
  (functional color-loc)
  (is-a Yellow Color) (is-a Red Color)
  (is-a House-1 House) (is-a House-2 House) (is-a House-3 House))
(facts
  (color-loc Yellow House-1 :source "(1)"))
"""


def test_functional_negative_excludes_other_slot1_for_same_slot0():
    """(color-loc Yellow House-1) ∧ functional color-loc ⟹
    (not (color-loc Yellow House-{2,3})) — every alternative
    in slot 1's type-domain except the bound one."""
    kb = _saturated(_FUNCTIONAL_NEG)
    assert ("color-loc", ("Yellow", "House-2")) in kb._negated_facts
    assert ("color-loc", ("Yellow", "House-3")) in kb._negated_facts


def test_functional_negative_does_not_exclude_same_pair():
    """The positive (color-loc Yellow House-1) is NOT negated."""
    kb = _saturated(_FUNCTIONAL_NEG)
    assert ("color-loc", ("Yellow", "House-1")) not in kb._negated_facts


def test_functional_negative_does_not_exclude_slot0_alternatives():
    """functional-negative is about slot 1 only; (color-loc Red House-1)
    is the *injective* family's job, must NOT be derived here."""
    kb = _saturated(_FUNCTIONAL_NEG)
    assert ("color-loc", ("Red", "House-1")) not in kb._negated_facts


# ── injective-negative ──────────────────────────────────────────────

_INJECTIVE_NEG = """
(rules
  (rule injective-negative (?R)
    :match  (and (?R ?a ?b)
                 (relation ?R ?A ?B)
                 (is-a ?a_other ?A)
                 (neq ?a_other ?a))
    :assert (not (?R ?a_other ?b))
    :why    "injective negative"
    :priority 240)
  (rule derive-injective-negative ()
    :match  (injective ?R)
    :assert (injective-negative ?R)
    :why    "injective ⟹ injective-negative active."
    :priority 100))
(ontology
  (relation color-loc Color House)
  (injective color-loc)
  (is-a Yellow Color) (is-a Red Color) (is-a Blue Color)
  (is-a House-1 House) (is-a House-2 House))
(facts
  (color-loc Yellow House-1 :source "(1)"))
"""


def test_injective_negative_excludes_other_slot0_for_same_slot1():
    """(color-loc Yellow House-1) ∧ injective color-loc ⟹
    (not (color-loc {Red,Blue} House-1))."""
    kb = _saturated(_INJECTIVE_NEG)
    assert ("color-loc", ("Red", "House-1")) in kb._negated_facts
    assert ("color-loc", ("Blue", "House-1")) in kb._negated_facts


def test_injective_negative_does_not_exclude_slot1_alternatives():
    """injective-negative is about slot 0 only; (color-loc Yellow House-2)
    is the *functional* family's job, must NOT be derived here."""
    kb = _saturated(_INJECTIVE_NEG)
    assert ("color-loc", ("Yellow", "House-2")) not in kb._negated_facts


# ── co-located-negative (chained through functional-negative) ───────

_CO_LOCATED_NEG = """
(rules
  (rule functional-negative (?R)
    :match  (and (?R ?a ?b)
                 (relation ?R ?A ?B)
                 (is-a ?b_other ?B)
                 (neq ?b_other ?b))
    :assert (not (?R ?a ?b_other))
    :why    "functional negative"
    :priority 240)
  (rule derive-functional-negative ()
    :match  (functional ?R)
    :assert (functional-negative ?R)
    :why    "functional ⟹ functional-negative active."
    :priority 100)

  (rule co-located-negative (?R1 ?V1 ?R2 ?V2)
    :match  (not (?R1 ?V1 ?h))
    :assert (not (?R2 ?V2 ?h))
    :why    "co-located: (not R1 V1 h) ⟹ (not R2 V2 h)."
    :priority 200)
  (rule derive-co-located-negative ()
    :match  (co-located ?R1 ?V1 ?R2 ?V2)
    :assert (co-located-negative ?R1 ?V1 ?R2 ?V2)
    :why    "co-located activator ⟹ co-located-negative active."
    :priority 100))
(ontology
  (relation color-loc  Color  House)
  (relation nation-loc Nation House)
  (functional color-loc)
  (co-located color-loc Yellow nation-loc Norwegian)
  (is-a Yellow Color)
  (is-a Norwegian Nation)
  (is-a House-1 House) (is-a House-2 House))
(facts
  (color-loc Yellow House-1 :source "(1)"))
"""


def test_co_located_negative_propagates_negative_across_equivalence():
    """functional ⟹ ¬(color-loc Yellow House-2); the co-located activator
    propagates that to ¬(nation-loc Norwegian House-2)."""
    kb = _saturated(_CO_LOCATED_NEG)
    # functional-negative produces the seed:
    assert ("color-loc", ("Yellow", "House-2")) in kb._negated_facts
    # co-located-negative propagates it:
    assert ("nation-loc", ("Norwegian", "House-2")) in kb._negated_facts


def test_co_located_negative_does_not_propagate_unrelated_negatives():
    """A negative on a different value (Blue not declared co-located with
    anything here) must not propagate."""
    kb = _saturated(_CO_LOCATED_NEG)
    # No co-located activator pairs (color-loc Blue) with anything —
    # so even if Blue were negated, no nation-loc would mirror it.
    # Conversely, (nation-loc Norwegian House-1) is *positively* implied
    # by co-located on the original positive ⟹ NOT in negated_facts.
    assert ("nation-loc", ("Norwegian", "House-1")) not in kb._negated_facts


# ── adjacent-via-endpoint-{fwd,bwd} — structural endpoint exclusions ─

_ENDPOINT_FWD = """
(rules
  (rule adjacent-via-endpoint-fwd (?S ?R1 ?V1 ?R2 ?V2)
    :match  (and (relation ?R1 ?A ?B)
                 (is-a ?h1 ?B)
                 (absent (and (is-a ?h2 ?B) (?S ?h2 ?h1))))
    :assert (not (?R1 ?V1 ?h1))
    :why    "endpoint fwd"
    :priority 240)
  (rule derive-adjacent-via-fwd ()
    :match  (adjacent-via ?S ?R1 ?V1 ?R2 ?V2)
    :assert (adjacent-via-fwd ?S ?R1 ?V1 ?R2 ?V2)
    :why    "adjacent-via ⟹ adjacent-via-fwd."
    :priority 100)
  (rule derive-adjacent-via-endpoint-fwd ()
    :match  (adjacent-via-fwd ?S ?R1 ?V1 ?R2 ?V2)
    :assert (adjacent-via-endpoint-fwd ?S ?R1 ?V1 ?R2 ?V2)
    :why    "adjacent-via-fwd ⟹ adjacent-via-endpoint-fwd."
    :priority 200))
(ontology
  (relation color-loc Color House)
  (adjacent-via right-of color-loc Ivory color-loc Green)
  (is-a Ivory Color) (is-a Green Color)
  (is-a House-1 House) (is-a House-2 House)
  (is-a House-3 House) (is-a House-4 House) (is-a House-5 House))
(facts
  ;; right-of declares the spatial backbone — House-2 is right of
  ;; House-1, …, House-5 right of House-4. House-1 has no right-of
  ;; source ⟹ Ivory cannot live there (no h with (right-of h H1)).
  (right-of House-2 House-1 :source "(spatial)")
  (right-of House-3 House-2 :source "(spatial)")
  (right-of House-4 House-3 :source "(spatial)")
  (right-of House-5 House-4 :source "(spatial)"))
"""


def test_adjacent_via_endpoint_fwd_excludes_no_source_house():
    """For `(adjacent-via right-of color-loc Ivory color-loc Green)` the
    fwd rule looks for `?h2` with `(right-of ?h2 ?h1)`. House-5 has
    no such `?h2` (no house has higher index than 5), so Ivory
    cannot live at House-5."""
    kb = _saturated(_ENDPOINT_FWD)
    assert ("color-loc", ("Ivory", "House-5")) in kb._negated_facts


def test_adjacent_via_endpoint_fwd_does_not_exclude_interior():
    """All houses except the rightmost have a right-of source, so
    the fwd endpoint rule must not fire on them."""
    kb = _saturated(_ENDPOINT_FWD)
    for h in ("House-1", "House-2", "House-3", "House-4"):
        assert ("color-loc", ("Ivory", h)) not in kb._negated_facts


_ENDPOINT_BWD = """
(rules
  (rule adjacent-via-endpoint-bwd (?S ?R1 ?V1 ?R2 ?V2)
    :match  (and (relation ?R2 ?A ?B)
                 (is-a ?h2 ?B)
                 (absent (and (is-a ?h1 ?B) (?S ?h2 ?h1))))
    :assert (not (?R2 ?V2 ?h2))
    :why    "endpoint bwd"
    :priority 240)
  (rule derive-adjacent-via-bwd ()
    :match  (adjacent-via ?S ?R1 ?V1 ?R2 ?V2)
    :assert (adjacent-via-bwd ?S ?R1 ?V1 ?R2 ?V2)
    :why    "adjacent-via ⟹ adjacent-via-bwd."
    :priority 100)
  (rule derive-adjacent-via-endpoint-bwd ()
    :match  (adjacent-via-bwd ?S ?R1 ?V1 ?R2 ?V2)
    :assert (adjacent-via-endpoint-bwd ?S ?R1 ?V1 ?R2 ?V2)
    :why    "adjacent-via-bwd ⟹ adjacent-via-endpoint-bwd."
    :priority 200))
(ontology
  (relation color-loc Color House)
  (adjacent-via right-of color-loc Ivory color-loc Green)
  (is-a Ivory Color) (is-a Green Color)
  (is-a House-1 House) (is-a House-2 House)
  (is-a House-3 House) (is-a House-4 House) (is-a House-5 House))
(facts
  (right-of House-2 House-1 :source "(spatial)")
  (right-of House-3 House-2 :source "(spatial)")
  (right-of House-4 House-3 :source "(spatial)")
  (right-of House-5 House-4 :source "(spatial)"))
"""


def test_adjacent_via_endpoint_bwd_excludes_no_target_house():
    """The bwd rule looks for `?h1` with `(right-of ?h2 ?h1)` —
    i.e. ?h2 as the source. House-1 has no `(right-of H1 ?)` (no
    house has lower index than 1), so Green cannot live at House-1."""
    kb = _saturated(_ENDPOINT_BWD)
    assert ("color-loc", ("Green", "House-1")) in kb._negated_facts


def test_adjacent_via_endpoint_bwd_does_not_exclude_interior():
    """All houses except the leftmost have a right-of target."""
    kb = _saturated(_ENDPOINT_BWD)
    for h in ("House-2", "House-3", "House-4", "House-5"):
        assert ("color-loc", ("Green", h)) not in kb._negated_facts


# ── adjacent-via-{fwd,bwd}-negative — contrapositive propagation ────

_ADJACENT_FWD_NEG = """
(rules
  ;; Seed a (not (R2 V2 h2)) via functional-negative on a known
  ;; positive in slot-1 type-domain.
  (rule functional-negative (?R)
    :match  (and (?R ?a ?b)
                 (relation ?R ?A ?B)
                 (is-a ?b_other ?B)
                 (neq ?b_other ?b))
    :assert (not (?R ?a ?b_other))
    :why    "functional negative"
    :priority 240)
  (rule derive-functional-negative ()
    :match  (functional ?R)
    :assert (functional-negative ?R)
    :why    "functional ⟹ functional-negative active."
    :priority 100)

  ;; The rule under test — contrapositive of adjacent-via-fwd.
  (rule adjacent-via-fwd-negative (?S ?R1 ?V1 ?R2 ?V2)
    :match  (and (not (?R2 ?V2 ?h2))
                 (?S ?h2 ?h1)
                 (absent (and (?S ?h_o ?h1) (neq ?h_o ?h2))))
    :assert (not (?R1 ?V1 ?h1))
    :why    "adjacent-via-fwd-negative"
    :priority 240)
  (rule derive-adjacent-via-fwd ()
    :match  (adjacent-via ?S ?R1 ?V1 ?R2 ?V2)
    :assert (adjacent-via-fwd ?S ?R1 ?V1 ?R2 ?V2)
    :why    "adjacent-via ⟹ adjacent-via-fwd."
    :priority 100)
  (rule derive-adjacent-via-fwd-negative ()
    :match  (adjacent-via-fwd ?S ?R1 ?V1 ?R2 ?V2)
    :assert (adjacent-via-fwd-negative ?S ?R1 ?V1 ?R2 ?V2)
    :why    "adjacent-via-fwd ⟹ adjacent-via-fwd-negative."
    :priority 200))
(ontology
  (relation color-loc Color House)
  (functional color-loc)
  (adjacent-via right-of color-loc Ivory color-loc Green)
  (is-a Ivory Color) (is-a Green Color) (is-a Blue Color)
  (is-a House-1 House) (is-a House-2 House) (is-a House-3 House))
(facts
  ;; Establish Green@House-3 ⟹ functional-negative gives
  ;; (not (color-loc Green House-2)). Then adjacent-via-fwd-negative
  ;; reads (not (color-loc Green House-2)) + (right-of H2 H1)
  ;; + unique-source(H1) ⟹ (not (color-loc Ivory House-1)).
  (color-loc Green House-3 :source "(1)")
  (right-of House-2 House-1 :source "(spatial)")
  (right-of House-3 House-2 :source "(spatial)"))
"""


def test_adjacent_via_fwd_negative_propagates_contrapositively():
    """¬(color-loc Green House-2) ∧ unique-right-of-source(House-1, House-2)
    ⟹ ¬(color-loc Ivory House-1)."""
    kb = _saturated(_ADJACENT_FWD_NEG)
    # Seed produced by functional-negative:
    assert ("color-loc", ("Green", "House-2")) in kb._negated_facts
    # Contrapositive propagation:
    assert ("color-loc", ("Ivory", "House-1")) in kb._negated_facts


_ADJACENT_BWD_NEG = """
(rules
  (rule functional-negative (?R)
    :match  (and (?R ?a ?b)
                 (relation ?R ?A ?B)
                 (is-a ?b_other ?B)
                 (neq ?b_other ?b))
    :assert (not (?R ?a ?b_other))
    :why    "functional negative"
    :priority 240)
  (rule derive-functional-negative ()
    :match  (functional ?R)
    :assert (functional-negative ?R)
    :why    "functional ⟹ functional-negative active."
    :priority 100)

  ;; Rule under test — bwd direction.
  (rule adjacent-via-bwd-negative (?S ?R1 ?V1 ?R2 ?V2)
    :match  (and (not (?R1 ?V1 ?h1))
                 (?S ?h2 ?h1)
                 (absent (and (?S ?h2 ?h_o) (neq ?h_o ?h1))))
    :assert (not (?R2 ?V2 ?h2))
    :why    "adjacent-via-bwd-negative"
    :priority 240)
  (rule derive-adjacent-via-bwd ()
    :match  (adjacent-via ?S ?R1 ?V1 ?R2 ?V2)
    :assert (adjacent-via-bwd ?S ?R1 ?V1 ?R2 ?V2)
    :why    "adjacent-via ⟹ adjacent-via-bwd."
    :priority 100)
  (rule derive-adjacent-via-bwd-negative ()
    :match  (adjacent-via-bwd ?S ?R1 ?V1 ?R2 ?V2)
    :assert (adjacent-via-bwd-negative ?S ?R1 ?V1 ?R2 ?V2)
    :why    "adjacent-via-bwd ⟹ adjacent-via-bwd-negative."
    :priority 200))
(ontology
  (relation color-loc Color House)
  (functional color-loc)
  (adjacent-via right-of color-loc Ivory color-loc Green)
  (is-a Ivory Color) (is-a Green Color) (is-a Blue Color)
  (is-a House-1 House) (is-a House-2 House) (is-a House-3 House))
(facts
  ;; Ivory@H1 ⟹ ¬(color-loc Ivory House-2) via functional-negative.
  ;; adjacent-via-bwd-negative reads ¬(color-loc Ivory House-2) +
  ;; (right-of House-3 House-2) + unique-target(House-3, House-2)
  ;; ⟹ ¬(color-loc Green House-3).
  (color-loc Ivory House-1 :source "(1)")
  (right-of House-2 House-1 :source "(spatial)")
  (right-of House-3 House-2 :source "(spatial)"))
"""


def test_adjacent_via_bwd_negative_propagates_contrapositively():
    """¬(color-loc Ivory House-2) ∧ unique-right-of-target(House-3, House-2)
    ⟹ ¬(color-loc Green House-3)."""
    kb = _saturated(_ADJACENT_BWD_NEG)
    # Seed from functional-negative:
    assert ("color-loc", ("Ivory", "House-2")) in kb._negated_facts
    # Contrapositive propagation:
    assert ("color-loc", ("Green", "House-3")) in kb._negated_facts


# ── Integration: zebra2 d=1 closure ─────────────────────────────────

def test_zebra2_solves_at_max_depth_1():
    """End-to-end: the six rules together collapse zebra2.ein into
    a depth-1 tree with a unique Solution. Guards against
    regressions to the pre-S1.5a.19 568-node Ambiguity. Skipped on
    CPython if the user environment is too slow — gating is via the
    `EIN_RUN_SLOW` env var (set in the PyPy runner)."""
    import os
    from pathlib import Path

    from ein_bot.inference.tree.solver import Solution, solve

    if not os.environ.get("EIN_RUN_SLOW"):
        import pytest
        pytest.skip("zebra2 d=1 solve is ~50s on CPython; "
                    "set EIN_RUN_SLOW=1 or run via bench_solve_pypy.sh")

    repo = Path(__file__).resolve().parents[4]
    kb = KnowledgeBase.from_ir(parse(
        (repo / "examples" / "zebra2.ein").read_text()))
    verdict = solve(kb, max_depth=1)
    assert isinstance(verdict, Solution), (
        f"expected Solution, got {type(verdict).__name__}")
    by_rel = {
        f.relation_name: f.args
        for fs in verdict.kb._facts_by_relation.values()
        for f in fs
        if f.relation_name in {"drink-loc", "pet-loc"}
        and len(f.args) == 2 and f.args[0] in {"Water", "Zebra"}
    }
    assert by_rel.get("drink-loc") == ("Water", "House-1"), (
        f"unexpected water binding: {by_rel.get('drink-loc')}")
    assert by_rel.get("pet-loc") == ("Zebra", "House-5"), (
        f"unexpected zebra binding: {by_rel.get('pet-loc')}")
