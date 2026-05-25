"""Per-rule positive + negative tests — S1.3.2 T1.3.2.8.

For each of the 7 M1-core rules, a positive scenario (rule fires,
produces the expected derived fact) and a negative scenario (rule
does NOT fire — missing activator, wrong types, or guard prunes).
"""
from __future__ import annotations

from ein_bot.inference.engine import Engine
from ein_bot.ir import parse
from ein_bot.kb.entities import Fact, Layer
from ein_bot.kb.provenance import Provenance
from ein_bot.kb.store import KnowledgeBase


def _engine(text: str) -> Engine:
    kb = KnowledgeBase.from_ir(parse(text))
    eng = Engine(kb)
    eng.compile_all()
    return eng


# ── symmetric ──────────────────────────────────────────────────────


def test_symmetric_positive():
    eng = _engine("""
    (rules (rule symmetric (?rel)
      :match (?rel ?a ?b) :assert (?rel ?b ?a) :why "s" :priority 100))
    (ontology (relation r T T) (symmetric r))
    (facts (r A B :source "(1)"))
    """)
    firings = list(eng.saturate())
    assert any(
        f.rule == "symmetric"
        and f.derived.relation_name == "r"
        and f.derived.args == ("B", "A")
        for f in firings
    )


def test_symmetric_negative_no_activator():
    """No `(symmetric r)` activator → rule sits dormant."""
    eng = _engine("""
    (rules (rule symmetric (?rel)
      :match (?rel ?a ?b) :assert (?rel ?b ?a) :why "s" :priority 100))
    (ontology (relation r T T))
    (facts (r A B :source "(1)"))
    """)
    firings = list(eng.saturate())
    assert not any(f.rule == "symmetric" for f in firings)


# ── transitive ─────────────────────────────────────────────────────


def test_transitive_positive():
    eng = _engine("""
    (rules (rule transitive (?rel)
      :match (and (?rel ?a ?b) (?rel ?b ?c) (neq ?a ?c))
      :assert (?rel ?a ?c) :why "t" :priority 200))
    (ontology (relation r T T) (transitive r))
    (facts (r A B :source "(1)") (r B C :source "(2)"))
    """)
    firings = list(eng.saturate())
    assert any(
        f.rule == "transitive" and f.derived.args == ("A", "C")
        for f in firings
    )


def test_transitive_negative_neq_prunes_cycle():
    """A-B + B-A: neq prunes the (A, A) and (B, B) cycles."""
    eng = _engine("""
    (rules (rule transitive (?rel)
      :match (and (?rel ?a ?b) (?rel ?b ?c) (neq ?a ?c))
      :assert (?rel ?a ?c) :why "t" :priority 200))
    (ontology (relation r T T) (transitive r))
    (facts (r A B :source "(1)") (r B A :source "(2)"))
    """)
    firings = list(eng.saturate())
    # No (r A A) or (r B B) firings — only what transitivity actually
    # produces (nothing for a 2-cycle).
    assert all(f.derived.args[0] != f.derived.args[1] for f in firings)


# ── implies ────────────────────────────────────────────────────────


def test_implies_positive():
    eng = _engine("""
    (rules (rule implies (?p ?q)
      :match (?p ?a ?b) :assert (?q ?a ?b) :why "i" :priority 100))
    (ontology
      (relation right-of T T)
      (relation next-to T T)
      (implies right-of next-to))
    (facts (right-of H2 H1 :source "(1)"))
    """)
    firings = list(eng.saturate())
    assert any(
        f.rule == "implies"
        and f.derived.relation_name == "next-to"
        and f.derived.args == ("H2", "H1")
        for f in firings
    )


def test_implies_negative_wrong_relation():
    """A `(right-of …)` fact does NOT trigger `implies` if the
    activator names a different source relation."""
    eng = _engine("""
    (rules (rule implies (?p ?q)
      :match (?p ?a ?b) :assert (?q ?a ?b) :why "i" :priority 100))
    (ontology
      (relation co-located T T) (relation next-to T T)
      (implies co-located next-to))
    (facts (right-of A B :source "(1)"))
    """)
    firings = list(eng.saturate())
    assert not any(f.rule == "implies" for f in firings)


# ── square-fwd ─────────────────────────────────────────────────────


def test_square_fwd_positive():
    """Square: from (R a b) + (R x y) + (co-located a x), derive (co-located b y)."""
    eng = _engine("""
    (rules
      (rule square-fwd (?R)
        :match (and (?R ?a ?b) (?R ?x ?y) (co-located ?a ?x))
        :assert (co-located ?b ?y) :why "sq" :priority 200))
    (ontology
      (relation right-of T T) (relation co-located T T)
      (square-fwd right-of))
    (facts
      (right-of H2 H1 :source "(1)")
      (right-of H3 H2 :source "(2)")
      (co-located H2 H2 :source "(3)"))
    """)
    firings = list(eng.saturate())
    # (right-of H2 H1) + (right-of H3 H2) + (co-located H2 H2) -> derive (co-located H1 H2).
    # Map to slots: ?a=H2, ?b=H1, ?x=H3, ?y=H2 ⇒ require (co-located H2 H3) — missing.
    # Try other binding: ?a=H3, ?b=H2, ?x=H2, ?y=H1 + (co-located H3 H2) — missing.
    # ?a=H2, ?b=H1, ?x=H2, ?y=H1: (co-located H2 H2) ✓ ⇒ derive (co-located H1 H1).
    derived = {(f.derived.relation_name, f.derived.args) for f in firings}
    assert ("co-located", ("H1", "H1")) in derived


def test_square_fwd_negative_missing_bridge():
    """Same fact graph but without the co-located bridge → no firing."""
    eng = _engine("""
    (rules
      (rule square-fwd (?R)
        :match (and (?R ?a ?b) (?R ?x ?y) (co-located ?a ?x))
        :assert (co-located ?b ?y) :why "sq" :priority 200))
    (ontology
      (relation right-of T T) (relation co-located T T)
      (square-fwd right-of))
    (facts
      (right-of H2 H1 :source "(1)")
      (right-of H3 H2 :source "(2)"))
    """)
    firings = list(eng.saturate())
    assert not any(f.rule == "square-fwd" for f in firings)


# ── square-bwd ─────────────────────────────────────────────────────


def test_square_bwd_positive():
    eng = _engine("""
    (rules
      (rule square-bwd (?R)
        :match (and (?R ?a ?b) (?R ?x ?y) (co-located ?b ?y))
        :assert (co-located ?a ?x) :why "sq" :priority 200))
    (ontology
      (relation right-of T T) (relation co-located T T)
      (square-bwd right-of))
    (facts
      (right-of H2 H1 :source "(1)")
      (right-of H3 H2 :source "(2)")
      (co-located H1 H1 :source "(3)"))
    """)
    firings = list(eng.saturate())
    # ?a=H2 ?b=H1 ?x=H2 ?y=H1 + (co-located H1 H1) ⇒ derive (co-located H2 H2).
    derived = {(f.derived.relation_name, f.derived.args) for f in firings}
    assert ("co-located", ("H2", "H2")) in derived


def test_square_bwd_negative_no_activator():
    eng = _engine("""
    (rules
      (rule square-bwd (?R)
        :match (and (?R ?a ?b) (?R ?x ?y) (co-located ?b ?y))
        :assert (co-located ?a ?x) :why "sq" :priority 200))
    (ontology (relation right-of T T) (relation co-located T T))
    (facts
      (right-of H2 H1 :source "(1)")
      (right-of H3 H2 :source "(2)")
      (co-located H1 H1 :source "(3)"))
    """)
    firings = list(eng.saturate())
    assert not any(f.rule == "square-bwd" for f in firings)


# ── type-exclusivity ───────────────────────────────────────────────


def test_type_exclusivity_positive():
    eng = _engine("""
    (rules (rule type-exclusivity (?R)
      :match (and (instance ?a ?T) (instance ?b ?T) (neq ?a ?b))
      :assert (not (?R ?a ?b)) :why "x" :priority 300))
    (ontology
      (type Color) (instance Red Color) (instance Blue Color)
      (relation co-located T T)
      (type-exclusivity co-located))
    """)
    firings = list(eng.saturate())
    # Two distinct-instance pairs → 2 firings (Red/Blue, Blue/Red).
    not_firings = [f for f in firings if f.derived.relation_name == "not"]
    assert len(not_firings) == 2
    # Each inner arg is a nested Fact (co-located).
    for f in not_firings:
        inner = f.derived.args[0]
        assert isinstance(inner, Fact)
        assert inner.relation_name == "co-located"


def test_type_exclusivity_negative_same_instance():
    """Only one instance → no neq-passing pair → no firing."""
    eng = _engine("""
    (rules (rule type-exclusivity (?R)
      :match (and (instance ?a ?T) (instance ?b ?T) (neq ?a ?b))
      :assert (not (?R ?a ?b)) :why "x" :priority 300))
    (ontology
      (type Color) (instance Red Color)
      (relation co-located T T)
      (type-exclusivity co-located))
    """)
    firings = list(eng.saturate())
    assert not any(f.rule == "type-exclusivity" for f in firings)


# ── square-unique ─────────────────────────────────────────────────


def test_square_unique_corner_inference():
    """Norwegian in House-1 + next-to Blue ⇒ Blue in House-2.

    The exact Zebra walkthrough step the rule was added to close.
    Idea-08 explanation-completeness requires this firing.
    """
    eng = _engine("""
    (rules
      (rule square-unique (?R ?T)
        :match (and (?R ?a ?b) (?R ?x ?y) (instance ?x ?T)
                    (co-located ?a ?x)
                    (absent (and (?R ?x ?z) (neq ?y ?z))))
        :assert (co-located ?b ?y)
        :why "u" :priority 200))
    (ontology
      (type House) (type Nationality) (type Color)
      (instance House-1 House) (instance House-2 House)
      (instance Norwegian Nationality) (instance Blue Color)
      (relation co-located T T) (relation next-to T T)
      (square-unique next-to House))
    (facts
      (co-located Norwegian House-1 :source "(10)")
      (next-to Norwegian Blue :source "(15)")
      (next-to House-1 House-2 :source "(1)"))
    """)
    firings = list(eng.saturate())
    matched = [
        f for f in firings if f.rule == "square-unique" and not f.redundant
    ]
    assert len(matched) == 1
    assert matched[0].derived.relation_name == "co-located"
    assert matched[0].derived.args == ("Blue", "House-2")


def test_square_unique_does_not_fire_on_attribute_pair():
    """Soundness: the `(instance ?x ?T)` premise prevents the rule
    from firing with ?x bound to an attribute (Norwegian, Blue) whose
    "uniqueness" is just *incidental* (only one stated next-to fact).

    Without this premise, the rule would derive wrong facts like
    (co-located House-3 Norwegian) by treating Blue as if it had a
    unique spatial neighbour."""
    eng = _engine("""
    (rules
      (rule square-unique (?R ?T)
        :match (and (?R ?a ?b) (?R ?x ?y) (instance ?x ?T)
                    (co-located ?a ?x)
                    (absent (and (?R ?x ?z) (neq ?y ?z))))
        :assert (co-located ?b ?y)
        :why "u" :priority 200))
    (ontology
      (type House) (type Nationality)
      (instance Norwegian Nationality)
      (relation co-located T T) (relation next-to T T)
      ;; NOTE: activator names House, but House has no instances here.
      (square-unique next-to House))
    (facts
      (next-to Norwegian Blue :source "(1)"))
    """)
    firings = list(eng.saturate())
    matched = [f for f in firings if f.rule == "square-unique"]
    # No House instances → guard's (instance ?x House) never matches → no firing.
    assert not matched


def test_square_unique_skips_middle_houses():
    """House-3 has two next-to neighbours (House-2, House-4) — guard
    fails for any binding with ?x = House-3."""
    eng = _engine("""
    (rules
      (rule square-unique (?R ?T)
        :match (and (?R ?a ?b) (?R ?x ?y) (instance ?x ?T)
                    (co-located ?a ?x)
                    (absent (and (?R ?x ?z) (neq ?y ?z))))
        :assert (co-located ?b ?y)
        :why "u" :priority 200))
    (ontology
      (type House) (type Nationality)
      (instance House-2 House) (instance House-3 House) (instance House-4 House)
      (instance Spaniard Nationality)
      (relation co-located T T) (relation next-to T T)
      (square-unique next-to House)
      (next-to House-2 House-3) (next-to House-3 House-2)
      (next-to House-3 House-4) (next-to House-4 House-3))
    (facts
      (co-located Spaniard House-3 :source "(1)")
      (next-to Spaniard Soda :source "(2)"))
    """)
    firings = list(eng.saturate())
    matched = [f for f in firings if f.rule == "square-unique"]
    # House-3 has two neighbours → guard fails → no firing.
    assert not matched


# ── hypothesis-contradiction ──────────────────────────────────────


def _add_synthetic_hyp(kb: KnowledgeBase, prop: Fact) -> None:
    """Helper: synthesise the (hypothesis ?h) + (contradiction-under ?h)
    pair the way P1.5 will eventually do."""
    hyp = Fact(
        relation_name="hypothesis", args=(prop,),
        layer=Layer.REASONING, provenance=Provenance.from_hypothesis(branch=1),
    )
    contra = Fact(
        relation_name="contradiction-under", args=(prop,),
        layer=Layer.REASONING, provenance=Provenance.from_hypothesis(branch=1),
    )
    kb.add_fact(prop)
    kb.add_fact(hyp)
    kb.add_fact(contra)
    kb.rebuild_indexes()


def test_hypothesis_contradiction_positive():
    """With both synthetic facts present, the rule fires and asserts
    (not <inner-prop>)."""
    eng = _engine("""
    (rules (rule hypothesis-contradiction ()
      :match (and (hypothesis ?h) (contradiction-under ?h))
      :assert (not ?h) :why "h" :priority 900))
    (ontology (relation co-located T T))
    """)
    # Synthetic inner proposition: (co-located Norwegian House-2)
    prop = Fact(
        relation_name="co-located",
        args=("Norwegian", "House-2"),
        layer=Layer.REASONING,
        provenance=Provenance.from_hypothesis(branch=1),
    )
    _add_synthetic_hyp(eng.kb, prop)

    firings = list(eng.saturate())
    # The rule yields (not <prop>) — a fact whose first arg is the
    # nested Fact `prop`.
    matched = [f for f in firings if f.rule == "hypothesis-contradiction"]
    assert len(matched) == 1
    derived = matched[0].derived
    assert derived.relation_name == "not"
    assert derived.args == (prop,)


def test_hypothesis_contradiction_negative_no_contradiction_fact():
    """Hypothesis fact present but no contradiction-under → no firing."""
    eng = _engine("""
    (rules (rule hypothesis-contradiction ()
      :match (and (hypothesis ?h) (contradiction-under ?h))
      :assert (not ?h) :why "h" :priority 900))
    (ontology (relation co-located T T))
    """)
    prop = Fact(
        relation_name="co-located",
        args=("Norwegian", "House-2"),
        layer=Layer.REASONING,
        provenance=Provenance.from_hypothesis(branch=1),
    )
    eng.kb.add_fact(prop)
    eng.kb.add_fact(Fact(
        relation_name="hypothesis", args=(prop,),
        layer=Layer.REASONING,
        provenance=Provenance.from_hypothesis(branch=1),
    ))
    # No (contradiction-under …) fact added.
    eng.kb.rebuild_indexes()

    firings = list(eng.saturate())
    assert not any(f.rule == "hypothesis-contradiction" for f in firings)
