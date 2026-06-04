"""Reflective rule-implication — a *derived* fact can activate a rule
(P1.8 S1.8.A9 / F5 rung 2).

The S1.8.A9 doc proposed a `compile_for`-after-firing fix because it believed
`Saturator._enqueue_pass` iterated a *frozen* compile cache. That premise is
stale: the shipped `_enqueue_pass` calls `engine.compile_all()` at the **start
of every pass**, and `compile_all` reads the **live** `kb._rule_apps_by_rule`
index (which `_index_fact` updates as facts are written). So a fact a rule
*derives* — e.g. `(symmetric foo)` — becomes an activator for the matching
generic rule on the very next pass, with no extra code. This file pins that
capability (direct + indirect-via-`imply1`) and its termination, so it can't
silently regress.
"""
from __future__ import annotations

from ein_bot.inference.saturator import Saturator
from ein_bot.ir import parse
from ein_bot.kb.store import KnowledgeBase


def _kb(src: str) -> KnowledgeBase:
    return KnowledgeBase.from_ir(parse(src))


def _productive(kb: KnowledgeBase):
    return [
        (f.rule, f.derived.relation_name, f.derived.args)
        for f in Saturator(kb).saturate(max_steps=2000)
        if not f.redundant
    ]


def test_derived_fact_activates_a_generic_rule():
    """A rule derives `(symmetric foo)`; the generic `symmetric` rule must then
    fire on `foo` — `(foo A B) ⇒ (foo B A)` — even though `(symmetric foo)`
    didn't exist at load time."""
    kb = _kb("""
    (rule fn-implies-sym ()
      :match (functional ?r) :assert (symmetric ?r) :why "fn⇒sym" :priority 50)
    (rule symmetric (?rel)
      :match (?rel ?a ?b) :assert (?rel ?b ?a) :why "sym" :priority 100)
    (relation foo T T) (relation functional T)
    (functional foo :source "(1)")
    (foo A B :source "(2)")
    """)
    fired = _productive(kb)
    assert ("fn-implies-sym", "symmetric", ("foo",)) in fired   # the activator
    assert ("symmetric", "foo", ("B", "A")) in fired            # reflective firing


def test_imply1_chain():
    """The full `imply1` path: `(imply1 functional symmetric)` +
    `(functional foo)` → `(symmetric foo)` → `symmetric` fires on `foo`."""
    kb = _kb("""
    (rule imply1 (?p ?q)
      :match (?p ?a) :assert (?q ?a) :why "{?p}⇒{?q}" :priority 50)
    (rule symmetric (?rel)
      :match (?rel ?a ?b) :assert (?rel ?b ?a) :why "sym" :priority 100)
    (relation foo T T) (relation functional T)
    (imply1 functional symmetric)
    (functional foo :source "(1)")
    (foo A B :source "(2)")
    """)
    fired = _productive(kb)
    assert ("imply1", "symmetric", ("foo",)) in fired
    assert ("symmetric", "foo", ("B", "A")) in fired


def test_reflective_loop_terminates():
    """A rule that (transitively) derives its own activator must still reach a
    fixpoint — the `_fired` / `_seen` dedup bounds the loop (Q-S1.8.A9.A)."""
    kb = _kb("""
    (rule a (?rel) :match (?rel ?x ?y) :assert (mark ?rel) :why "w" :priority 100)
    (rule mark () :match (mark ?r) :assert (a ?r) :why "w" :priority 100)
    (relation k T T) (relation mark T) (relation a T)
    (a k)
    (k A B :source "(1)")
    """)
    # Reaches a fixpoint (no SaturatorStepLimitError) within a modest budget.
    n = sum(1 for _ in Saturator(kb).saturate(max_steps=5000))
    assert n < 100  # bounded, not runaway


def test_non_reflective_conclusion_unaffected():
    """A derived fact whose head is NOT a rule name (an ordinary relation)
    activates nothing extra — reflective firing is specifically about
    rule-named conclusions."""
    kb = _kb("""
    (rule r () :match (seed ?a) :assert (plain ?a) :why "w" :priority 100)
    (relation seed T) (relation plain T)
    (seed X :source "(1)")
    """)
    fired = _productive(kb)
    assert fired == [("r", "plain", ("X",))]
