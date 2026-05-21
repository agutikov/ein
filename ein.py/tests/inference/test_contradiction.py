"""Contradiction detector tests — S1.4.1 / P1.4."""
from __future__ import annotations

from pathlib import Path

from ein_bot.inference.contradiction import (
    Contradiction,
    ContradictionDetector,
)
from ein_bot.inference.saturator import Saturator
from ein_bot.ir import parse
from ein_bot.kb.entities import Fact, Layer
from ein_bot.kb.provenance import Provenance
from ein_bot.kb.store import KnowledgeBase

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
    (ontology (relation r T T))
    (facts (r A B :source "(1)"))
    """)
    d = ContradictionDetector(kb)
    assert d.detect() == ()
    assert not d.has_contradiction()


def test_negative_only_no_conflict():
    """A `(not X)` without the matching positive is not a conflict."""
    kb = _kb("""
    (ontology (relation r T T))
    (facts (not (r A B) :source "(1)"))
    """)
    d = ContradictionDetector(kb)
    assert d.detect() == ()
    assert not d.has_contradiction()


def test_different_inner_no_conflict():
    """`(not (r A B))` + `(r A C)` — different args, no pair."""
    kb = _kb("""
    (ontology (relation r T T))
    (facts
      (r A C :source "(1)")
      (not (r A B) :source "(2)"))
    """)
    d = ContradictionDetector(kb)
    assert d.detect() == ()


# ── Positive cases ─────────────────────────────────────────────────


def test_same_layer_conflict_in_reasoning():
    """Both X and (not X) in REASONING — one Contradiction."""
    kb = _kb("(ontology (relation r T T))")

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
    assert c.layer is Layer.REASONING
    assert c.positive is positive
    assert c.negative is negative
    assert d.has_contradiction()


def test_same_layer_conflict_in_fact_layer():
    """Same pair entirely in FACT layer — also a contradiction."""
    kb = _kb("(ontology (relation r T T))")
    _put(kb, Fact(
        relation_name="r", args=("A", "B"), layer=Layer.FACT,
        provenance=Provenance.from_source(source="(1)"),
    ))
    _put(kb, Fact(
        relation_name="not",
        args=(Fact(relation_name="r", args=("A", "B"), layer=Layer.FACT),),
        layer=Layer.FACT,
        provenance=Provenance.from_source(source="(2)"),
    ))

    pairs = ContradictionDetector(kb).detect()
    assert len(pairs) == 1
    assert pairs[0].layer is Layer.FACT


# ── Cross-layer non-conflict ───────────────────────────────────────


def test_cross_layer_non_conflict():
    """Positive in FACT, negative in REASONING — NOT a contradiction
    (engine-design choice; see ContradictionDetector docstring)."""
    kb = _kb("(ontology (relation r T T))")
    _put(kb, Fact(
        relation_name="r", args=("A", "B"), layer=Layer.FACT,
        provenance=Provenance.from_source(source="(1)"),
    ))
    _put(kb, Fact(
        relation_name="not",
        args=(Fact(relation_name="r", args=("A", "B"), layer=Layer.REASONING),),
        layer=Layer.REASONING,
        provenance=Provenance.from_rule(rule="type-exclusivity"),
    ))

    pairs = ContradictionDetector(kb).detect()
    assert pairs == ()


# ── Multi-pair + layer scoping ─────────────────────────────────────


def test_multi_contradiction_kb():
    """Several (X, (not X)) pairs — detector reports all."""
    kb = _kb("(ontology (relation r T T))")
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
    derived_pairs = {(p.positive.args, p.negative.layer) for p in pairs}
    assert derived_pairs == {
        (("A", "B"), Layer.REASONING),
        (("C", "D"), Layer.REASONING),
        (("E", "F"), Layer.REASONING),
    }


def test_detect_layer_scopes_correctly():
    kb = _kb("(ontology (relation r T T))")
    # One conflict in REASONING.
    _put(kb, Fact(relation_name="r", args=("X", "Y"),
                  layer=Layer.REASONING,
                  provenance=Provenance.from_rule(rule="r1")))
    _put(kb, Fact(
        relation_name="not",
        args=(Fact(relation_name="r", args=("X", "Y"), layer=Layer.REASONING),),
        layer=Layer.REASONING,
        provenance=Provenance.from_rule(rule="r2"),
    ))
    # One conflict in FACT.
    _put(kb, Fact(relation_name="r", args=("P", "Q"),
                  layer=Layer.FACT,
                  provenance=Provenance.from_source(source="(1)")))
    _put(kb, Fact(
        relation_name="not",
        args=(Fact(relation_name="r", args=("P", "Q"), layer=Layer.FACT),),
        layer=Layer.FACT,
        provenance=Provenance.from_source(source="(2)"),
    ))

    d = ContradictionDetector(kb)
    assert len(d.detect()) == 2
    in_reasoning = d.detect_layer(Layer.REASONING)
    assert len(in_reasoning) == 1
    assert in_reasoning[0].positive.args == ("X", "Y")
    in_fact = d.detect_layer(Layer.FACT)
    assert len(in_fact) == 1
    assert in_fact[0].positive.args == ("P", "Q")
    in_ontology = d.detect_layer(Layer.ONTOLOGY)
    assert in_ontology == ()


# ── Nested-fact safety ─────────────────────────────────────────────


def test_nested_inner_fact():
    """Inner positive itself has a nested-Fact arg (Q40-style).

    `(not (hypothesis (co-located N H_2)))` is a fact whose
    positive form is `(hypothesis (co-located N H_2))`. The
    detector should recognise the pair correctly via Fact.__eq__'s
    recursive comparison.
    """
    kb = _kb("(ontology (relation hypothesis T) (relation co-located T T))")
    inner_inner = Fact(
        relation_name="co-located", args=("Norwegian", "House_2"),
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
                       args=("Norwegian", "House_2"),
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
    kb = _kb("(ontology (relation r T T))")
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
    assert matching[0].layer is Layer.REASONING


# ── Direct ⊥ — S1.5.4a Part 2 ─────────────────────────────────────


def test_direct_false_fact_is_contradiction():
    """A `(false)` fact in any layer is a `kind='direct'`
    contradiction — `positive` is None, `negative` is the fact
    itself, `witness` returns the negative for unsat-core walks."""
    kb = _kb("(ontology (relation r T T))")
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
    assert c.layer is Layer.REASONING
    assert d.has_contradiction()


def test_direct_false_and_pair_coexist():
    """Both `(false)` and `(X, (not X))` in the same KB → two
    Contradictions, one of each kind."""
    kb = _kb("(ontology (relation r T T))")
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
    kb = _kb("(ontology (relation r T T))")
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
