"""Contradiction detector tests — S1.4.1 / P1.4."""
from __future__ import annotations

from pathlib import Path

from ein.inference.contradiction import (
    Contradiction,
    ContradictionDetector,
)
from ein.inference.saturator import Saturator
from ein.ir import parse
from ein.kb.entities import Fact, Layer
from ein.kb.provenance import Provenance
from ein.kb.store import KnowledgeBase

REPO = Path(__file__).resolve().parents[3]
ZEBRA = REPO / "examples" / "zebra.ein"


def _kb(text: str = "") -> KnowledgeBase:
    if not text:
        kb = KnowledgeBase()
        kb.rebuild_indexes()
        return kb
    return KnowledgeBase.from_ir(parse(text))


def _put(kb: KnowledgeBase, fact: Fact) -> Fact:
    """Add + index a fact in one call."""
    stored = kb.add_fact(fact)
    kb._index_fact(stored)
    return stored


# ── Base cases ─────────────────────────────────────────────────────


def test_empty_kb():
    kb = _kb()
    d = ContradictionDetector(kb)
    assert d.detect() == ()
    assert not d.has_contradiction()


def test_positive_only_no_conflict():
    kb = _kb("""
    (relation r T T)
    (r A B :source "(1)")
    """)
    d = ContradictionDetector(kb)
    assert d.detect() == ()
    assert not d.has_contradiction()


def test_negative_only_no_conflict():
    """A `(not X)` without the matching positive is not a conflict."""
    kb = _kb("""
    (relation r T T)
    (not (r A B) :source "(1)")
    """)
    d = ContradictionDetector(kb)
    assert d.detect() == ()
    assert not d.has_contradiction()


def test_different_inner_no_conflict():
    """`(not (r A B))` + `(r A C)` — different args, no pair."""
    kb = _kb("""
    (relation r T T)
    (r A C :source "(1)")
    (not (r A B) :source "(2)")
    """)
    d = ContradictionDetector(kb)
    assert d.detect() == ()


# ── Positive cases ─────────────────────────────────────────────────


def test_same_layer_conflict_in_reasoning():
    """Both X and (not X) in REASONING — one Contradiction."""
    kb = _kb('(relation r T T)')

    positive = _put(kb, Fact(
        relation_name="r",
        args=("A", "B"),
        layer=Layer.REASONING,
        provenance=Provenance.from_rule(rule="some-rule"),
    ))
    negative = _put(kb, Fact(
        relation_name="not",
        args=(Fact(relation_name="r", args=("A", "B"),
                   layer=Layer.REASONING),),
        layer=Layer.REASONING,
        provenance=Provenance.from_rule(rule="type-exclusivity"),
    ))

    d = ContradictionDetector(kb)
    pairs = d.detect()
    assert len(pairs) == 1
    c = pairs[0]
    assert c.positive is positive
    assert c.negative is negative
    assert d.has_contradiction()


def test_conflict_between_two_stated_facts():
    """Both X and (not X) authored with a `:source` — a contradiction."""
    kb = _kb('(relation r T T)')
    _put(kb, Fact(
        relation_name="r", args=("A", "B"),
        provenance=Provenance.from_source(source="(1)"),
    ))
    _put(kb, Fact(
        relation_name="not",
        args=(Fact(relation_name="r", args=("A", "B")),),
        provenance=Provenance.from_source(source="(2)"),
    ))

    pairs = ContradictionDetector(kb).detect()
    assert len(pairs) == 1


# ── Origin-independence — S1.22.1b ─────────────────────────────────


def test_derived_negative_contradicts_stated_positive():
    """A rule-derived `(not X)` against an authored, `:source`-carrying
    `X` **is** a contradiction (S1.22.1b).

    Until S1.22.1b the detector skipped this pair because the two facts
    sat in different knowledge layers, so a KB that flatly contradicted
    its own clues reported nothing. This is the unit-level pin of the
    fix; :func:`test_mutually_negating_clues_are_detected` is the
    end-to-end one.
    """
    kb = _kb("""
    (relation r T T)
    (rule deny () :match (r ?a ?b) :assert (not (r ?a ?b))
      :why "deny" :priority 100)
    (r A B :source "(1)")
    """)
    list(Saturator(kb).saturate())

    pairs = ContradictionDetector(kb).detect()
    assert len(pairs) == 1
    assert pairs[0].positive.source == "(1)"
    assert pairs[0].negative.rule_name == "deny"


def test_mutually_negating_clues_are_detected():
    """Two stated clues a rule mutually negates — the S1.22.1b reproducer.

    `(sits A S1)` and `(sits B S1)` are both given; `one-per-seat`
    derives `(not (sits B S1))` from the first and `(not (sits A S1))`
    from the second. The saturated KB holds two `(X, ¬X)` pairs and must
    report both. Before the fix it reported **zero** — the positives were
    FACT-layer and the negatives REASONING-layer.
    """
    kb = _kb("""
    (relation sits Person Seat)
    (relation is-a Thing Thing)
    (is-a A Person) (is-a B Person)
    (rule one-per-seat ()
      :match  (and (sits ?p1 ?s) (is-a ?p2 Person) (neq ?p1 ?p2))
      :assert (not (sits ?p2 ?s))
      :why "{?s} is taken by {?p1}" :priority 100)
    (sits A S1 :source "clue (1)")
    (sits B S1 :source "clue (2)")
    """)
    list(Saturator(kb).saturate())

    pairs = ContradictionDetector(kb).detect()
    assert len(pairs) == 2
    assert {p.positive.args for p in pairs} == {("A", "S1"), ("B", "S1")}
    assert all(p.positive.source is not None for p in pairs)
    assert all(p.negative.rule_name == "one-per-seat" for p in pairs)


# ── Multi-pair ─────────────────────────────────────────────────────


def test_multi_contradiction_kb():
    """Several (X, (not X)) pairs — detector reports all."""
    kb = _kb('(relation r T T)')
    for a, b in [("A", "B"), ("C", "D"), ("E", "F")]:
        _put(kb, Fact(
            relation_name="r", args=(a, b), layer=Layer.REASONING,
            provenance=Provenance.from_rule(rule="r1"),
        ))
        _put(kb, Fact(
            relation_name="not",
            args=(Fact(relation_name="r", args=(a, b), layer=Layer.REASONING),),
            layer=Layer.REASONING,
            provenance=Provenance.from_rule(rule="r2"),
        ))

    pairs = ContradictionDetector(kb).detect()
    assert len(pairs) == 3
    assert {p.positive.args for p in pairs} == {
        ("A", "B"), ("C", "D"), ("E", "F"),
    }


# ── Nested-fact safety ─────────────────────────────────────────────


def test_nested_inner_fact():
    """Inner positive itself has a nested-Fact arg (Q40-style).

    `(not (hypothesis (co-located N H_2)))` is a fact whose
    positive form is `(hypothesis (co-located N H_2))`. The
    detector should recognise the pair correctly via Fact.__eq__'s
    recursive comparison.
    """
    kb = _kb('(relation hypothesis T) (relation co-located T T)')
    inner_inner = Fact(
        relation_name="co-located", args=("Norwegian", "House-2"),
        layer=Layer.REASONING,
        provenance=Provenance.from_rule(rule="hyp"),
    )
    positive = _put(kb, Fact(
        relation_name="hypothesis",
        args=(inner_inner,),
        layer=Layer.REASONING,
        provenance=Provenance.from_rule(rule="hyp"),
    ))
    negative = _put(kb, Fact(
        relation_name="not",
        args=(Fact(
            relation_name="hypothesis",
            args=(Fact(relation_name="co-located",
                       args=("Norwegian", "House-2"),
                       layer=Layer.REASONING),),
            layer=Layer.REASONING,
        ),),
        layer=Layer.REASONING,
        provenance=Provenance.from_rule(rule="rebuttal"),
    ))

    pairs = ContradictionDetector(kb).detect()
    assert len(pairs) == 1
    assert pairs[0].positive is positive
    assert pairs[0].negative is negative


# ── Saturator integration ──────────────────────────────────────────


def test_saturator_contradictions_helper():
    """Saturator.contradictions() delegates to ContradictionDetector."""
    kb = _kb('(relation r T T)')
    _put(kb, Fact(relation_name="r", args=("A", "B"),
                  layer=Layer.REASONING,
                  provenance=Provenance.from_rule(rule="r1")))
    _put(kb, Fact(
        relation_name="not",
        args=(Fact(relation_name="r", args=("A", "B"), layer=Layer.REASONING),),
        layer=Layer.REASONING,
        provenance=Provenance.from_rule(rule="r2"),
    ))

    sat = Saturator(kb)
    pairs = sat.contradictions()
    assert len(pairs) == 1
    assert isinstance(pairs[0], Contradiction)


# ── End-to-end on zebra.ein ────────────────────────────────────────


def test_zebra_clean_state_no_contradictions():
    """Vanilla zebra.ein saturation — type-exclusivity produces
    many (not (co-located A B)) facts but none clash with positives
    (the puzzle is consistent)."""
    kb = KnowledgeBase.from_ir(parse(ZEBRA.read_text()))
    sat = Saturator(kb)
    list(sat.saturate())
    assert sat.contradictions() == ()
    assert not ContradictionDetector(kb).has_contradiction()


def test_zebra_injected_conflict_caught():
    """Inject a REASONING-layer positive that conflicts with a
    type-exclusivity derivation. The detector catches it after
    saturation."""
    kb = KnowledgeBase.from_ir(parse(ZEBRA.read_text()))
    sat = Saturator(kb)
    list(sat.saturate())
    # type-exclusivity derives (not (co-located Norwegian Spaniard))
    # in REASONING. Inject the positive in the same layer.
    _put(kb, Fact(
        relation_name="co-located",
        args=("Norwegian", "Spaniard"),
        layer=Layer.REASONING,
        provenance=Provenance.from_hypothesis(branch=1),
    ))
    pairs = sat.contradictions()
    assert len(pairs) >= 1
    matching = [
        p for p in pairs
        if p.positive.args == ("Norwegian", "Spaniard")
    ]
    assert len(matching) == 1
    assert matching[0].kind == "pair"


# ── Direct ⊥ — S1.5.4a Part 2 ─────────────────────────────────────


def test_direct_false_fact_is_contradiction():
    """A `(false)` fact is a `kind='direct'` contradiction —
    `positive` is None, `negative` is the fact itself, `witness`
    returns the negative for unsat-core walks."""
    kb = _kb('(relation r T T)')
    false_fact = _put(kb, Fact(
        relation_name="false", args=(), layer=Layer.REASONING,
        provenance=Provenance.from_rule(rule="functional"),
    ))

    d = ContradictionDetector(kb)
    cs = d.detect()
    assert len(cs) == 1
    c = cs[0]
    assert c.kind == "direct"
    assert c.positive is None
    assert c.negative is false_fact
    assert c.witness is false_fact
    assert d.has_contradiction()


def test_direct_false_and_pair_coexist():
    """Both `(false)` and `(X, (not X))` in the same KB → two
    Contradictions, one of each kind."""
    kb = _kb('(relation r T T)')
    positive = _put(kb, Fact(
        relation_name="r", args=("A", "B"), layer=Layer.REASONING,
        provenance=Provenance.from_hypothesis(branch=1),
    ))
    negative = _put(kb, Fact(
        relation_name="not",
        args=(Fact(relation_name="r", args=("A", "B"),
                   layer=Layer.REASONING),),
        layer=Layer.REASONING,
        provenance=Provenance.from_rule(rule="sibling-exclusive"),
    ))
    false_fact = _put(kb, Fact(
        relation_name="false", args=(), layer=Layer.REASONING,
        provenance=Provenance.from_rule(rule="functional"),
    ))

    cs = ContradictionDetector(kb).detect()
    kinds = {c.kind for c in cs}
    assert kinds == {"direct", "pair"}
    direct = next(c for c in cs if c.kind == "direct")
    pair = next(c for c in cs if c.kind == "pair")
    assert direct.negative is false_fact
    assert pair.positive is positive
    assert pair.negative is negative


def test_pair_kind_defaults_to_pair():
    """Existing call-sites that don't pass kind= explicitly still
    get the original `pair` shape — guards against silent
    behaviour change for unaware callers."""
    kb = _kb('(relation r T T)')
    _put(kb, Fact(
        relation_name="r", args=("A", "B"), layer=Layer.REASONING,
        provenance=Provenance.from_hypothesis(branch=1),
    ))
    _put(kb, Fact(
        relation_name="not",
        args=(Fact(relation_name="r", args=("A", "B"),
                   layer=Layer.REASONING),),
        layer=Layer.REASONING,
        provenance=Provenance.from_rule(rule="type-exclusivity"),
    ))
    cs = ContradictionDetector(kb).detect()
    assert len(cs) == 1
    assert cs[0].kind == "pair"
    # witness falls back to positive when present:
    assert cs[0].witness is cs[0].positive
