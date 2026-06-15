"""Saturator NAF semantics — S1.5a.1 T1.5a.1.1.

Covers the fire-time re-evaluation of ``(absent P)`` premises. The
pre-S1.5a.1 saturator evaluated AbsentGuards at *first-enqueue*
time and kept the verdict through the heap; a rule whose NAF
referenced a derived relation could enqueue with NAF=pass before
the derivation chain had populated the watched relation, and then
fire on a stale verdict at dequeue.

The fix re-runs each AbsentGuard's sub-plan with the dequeued
bindings against the *current* KB; if any sub-plan now yields a
match the firing is dropped (recorded as ``_fired`` so the queue
stops churning on it).

Tests:

- ``test_naf_preserved_when_derived_fact_never_arrives`` — the
  positive control: with no rule that derives the watched relation,
  the gated firing must fire (NAF passes at enqueue, still passes
  at fire).
- ``test_naf_dropped_when_derived_fact_arrives_between_enqueue_and_fire``
  — the race scenario: priority-100 rule derives R, then the
  priority-200 gated rule pops. Old engine: fires (stale NAF
  verdict). New engine: re-evaluates, sees R, drops the firing.
- ``test_naf_dropped_counter_visibility`` — the metric exposed for
  P1.5a tracing: each dropped firing bumps ``_naf_dropped``.
- ``test_naf_over_zebra_shaped_derivation_chain`` — full
  ``right-of`` + ``(symmetric next-to)`` + ``(includes right-of
  next-to)`` chain feeding an NAF on ``next-to``. The shape that
  motivated the fix in the first place (parent README §(a)).
"""
from __future__ import annotations

from ein.inference.saturator import Saturator
from ein.ir import parse
from ein.kb.store import KnowledgeBase


def _sat(text: str) -> Saturator:
    return Saturator(KnowledgeBase.from_ir(parse(text)))


def test_naf_preserved_when_derived_fact_never_arrives():
    """Gate's NAF over relation ``r`` passes; no rule derives ``r``,
    so the gated firing must fire exactly once."""
    sat = _sat("""
    (rule gate ()
      :match (and (trigger ?a ?b) (absent (r ?a ?b)))
      :assert (gated ?a ?b)
      :why "gate fires when r absent"
      :priority 200)
    (relation trigger T T)
    (relation r T T)
    (relation gated T T)
    (trigger X Y :source "(1)")
    """)
    firings = list(sat.saturate())
    productive = [f for f in firings if not f.redundant]
    assert len(productive) == 1
    assert productive[0].rule == "gate"
    assert productive[0].derived[0].relation_name == "gated"
    assert productive[0].derived[0].args == ("X", "Y")


def test_naf_dropped_when_derived_fact_arrives_between_enqueue_and_fire():
    """The race scenario.

    ``derive-r`` (priority 100) and ``gate`` (priority 200) are
    enqueued in the same pass. At enqueue time gate's
    ``(absent (r ?a ?b))`` sees an empty ``r`` relation and passes.
    Then derive-r fires (priority 100 first), populating ``(r X Y)``.
    Then gate pops: under the old enqueue-time semantics it would
    fire on the stale verdict; under fire-time re-eval it must drop.
    """
    sat = _sat("""
    (rule derive-r ()
      :match (raw ?a ?b)
      :assert (r ?a ?b)
      :why "derive r from raw"
      :priority 100)
    (rule gate ()
      :match (and (trigger ?a ?b) (absent (r ?a ?b)))
      :assert (gated ?a ?b)
      :why "gate fires when r absent"
      :priority 200)
    (relation raw T T)
    (relation r T T)
    (relation trigger T T)
    (relation gated T T)
    (raw X Y :source "(1)")
    (trigger X Y :source "(2)")
    """)
    list(sat.saturate())
    # r derived by priority-100 rule.
    r_facts = [f for f in sat.kb.facts if f.relation_name == "r"]
    assert any(f.args == ("X", "Y") for f in r_facts), (
        "precondition: derive-r should have produced (r X Y)"
    )
    # gated must NOT be in the KB — gate's NAF should fail at fire time.
    gated_facts = [f for f in sat.kb.facts if f.relation_name == "gated"]
    assert gated_facts == [], (
        "gate fired on stale enqueue-time NAF verdict; expected "
        "fire-time re-eval to drop the firing since (r X Y) was "
        "derived before gate dequeued."
    )


def test_naf_dropped_counter_visibility():
    """The dropped-firing counter is incremented when NAF re-eval
    rejects a firing. The plain count is enough for P1.5a tracing;
    P1.9 may extend with per-rule breakdown."""
    sat = _sat("""
    (rule derive-r ()
      :match (raw ?a ?b)
      :assert (r ?a ?b)
      :why "derive r from raw"
      :priority 100)
    (rule gate ()
      :match (and (trigger ?a ?b) (absent (r ?a ?b)))
      :assert (gated ?a ?b)
      :why "gate fires when r absent"
      :priority 200)
    (relation raw T T)
    (relation r T T)
    (relation trigger T T)
    (relation gated T T)
    (raw X Y :source "(1)")
    (trigger X Y :source "(2)")
    """)
    list(sat.saturate())
    assert sat.naf_dropped >= 1, (
        f"expected at least one NAF-re-eval drop; got {sat.naf_dropped}"
    )


def test_naf_preserved_when_derive_does_not_apply():
    """The NAF rule's binding must remain firable when the
    derivation chain doesn't produce the watched fact for THIS
    binding — even if it produces it for some other binding.

    Two raw inputs (X,Y) and (P,Q); derive-r runs only on (P,Q)
    (gated by a side predicate); gate's binding for (X,Y) sees
    ``r`` populated for (P,Q) only and must still fire.
    """
    sat = _sat("""
    (rule derive-r-PQ ()
      :match (raw P Q)
      :assert (r P Q)
      :why "derive r for (P,Q)"
      :priority 100)
    (rule gate ()
      :match (and (trigger ?a ?b) (absent (r ?a ?b)))
      :assert (gated ?a ?b)
      :why "gate fires when r absent for (?a,?b)"
      :priority 200)
    (relation raw T T)
    (relation r T T)
    (relation trigger T T)
    (relation gated T T)
    (raw P Q :source "(1)")
    (trigger X Y :source "(2)")
    """)
    list(sat.saturate())
    gated_facts = {f.args for f in sat.kb.facts if f.relation_name == "gated"}
    assert ("X", "Y") in gated_facts, (
        "gate should fire for (X,Y): r is populated only for (P,Q), "
        "so the per-binding NAF still passes."
    )


def test_naf_over_derived_next_to_zebra_shape():
    """The zebra2-motivating shape: ``next-to`` is derived from
    ``right-of`` via ``(includes right-of next-to)`` then closed by
    ``(symmetric next-to)``; a rule with ``(absent (next-to ?h_o ?h1))``
    must see the *closed* next-to relation at fire time.

    Setup: 3 houses in a row (H1 right-of H2 right-of H3 ... wait,
    using zebra's convention where H_{n+1} right-of H_n: (right-of H2 H1),
    (right-of H3 H2)). After includes+symmetric: next-to is
    {(H2,H1),(H1,H2),(H3,H2),(H2,H3)}. So H1 has exactly one
    next-to neighbour (H2); H2 has two (H1 and H3); H3 has one (H2).

    A gate rule ``:match (and (anchor ?h1) (absent (next-to ?h_o ?h1)))``
    over `?h_o` would falsely fire for any `?h_o` that hasn't yet been
    derived as next-to ?h1. With fire-time re-eval, the NAF sees the
    closed relation and gate fires only for genuine non-neighbours.
    """
    sat = _sat("""
    (rule symmetric (?rel)
      :match (?rel ?a ?b)
      :assert (?rel ?b ?a)
      :why "symmetric"
      :priority 100)
    (rule includes (?p ?q)
      :match (?p ?a ?b)
      :assert (?q ?a ?b)
      :why "includes"
      :priority 100)
    (rule gate-non-neighbour ()
      :match (and (anchor ?h1) (other ?h_o) (neq ?h_o ?h1)
                  (absent (next-to ?h_o ?h1)))
      :assert (non-neighbour ?h_o ?h1)
      :why "h_o is not next-to h1"
      :priority 250)
    (relation right-of  T T)
    (relation next-to   T T)
    (relation anchor    T)
    (relation other     T)
    (relation non-neighbour T T)
    (symmetric next-to)
    (includes  right-of next-to)
    (right-of H2 H1 :source "(1)")
    (right-of H3 H2 :source "(2)")
    (anchor H2 :source "(3)")
    (other H1 :source "(4)")
    (other H2 :source "(5)")
    (other H3 :source "(6)")
    """)
    list(sat.saturate())
    next_to = {f.args for f in sat.kb.facts if f.relation_name == "next-to"}
    assert ("H2", "H1") in next_to, "derived: (includes right-of next-to)"
    assert ("H1", "H2") in next_to, "derived: (symmetric next-to)"
    assert ("H3", "H2") in next_to, "derived: (includes right-of next-to)"
    assert ("H2", "H3") in next_to, "derived: (symmetric next-to)"
    # H1 and H3 are both next-to H2, so non-neighbour(H1,H2) and
    # non-neighbour(H3,H2) must NOT fire. (H2,H2) is excluded by neq.
    non_neighbour = {
        f.args for f in sat.kb.facts if f.relation_name == "non-neighbour"
    }
    assert ("H1", "H2") not in non_neighbour, (
        "H1 is next-to H2 (derived); gate must drop at fire time."
    )
    assert ("H3", "H2") not in non_neighbour, (
        "H3 is next-to H2 (derived); gate must drop at fire time."
    )
    # No other 'other' values, so non_neighbour should be empty.
    assert non_neighbour == set(), (
        f"expected no non-neighbour facts; got {non_neighbour}"
    )
