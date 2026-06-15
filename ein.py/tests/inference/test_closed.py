"""Closed-relation inference — `emit_closed` / `producible_relations`.

A declared relation is *closed* iff no rule can positively
conclude it. The closed-inference step emits `(__closed__ R)` before
saturation so the hypothesis generator skips R — replacing the
hand-written `(__closed__ …)` declarations. (Dunder convention: the
kernel trigger is `__closed__`, not the bare userspace `closed`.)
"""
from ein.inference.closed import emit_closed, producible_relations
from ein.ir import parse
from ein.kb.store import KnowledgeBase


def _kb(text: str) -> KnowledgeBase:
    return KnowledgeBase.from_ir(parse(text))


# `co-located` is propagated by `symmetric` — producible; `is-a` is
# concluded by no rule (`sibling-exclusive` only asserts a negative).
_PUZZLE = """
(rule symmetric (?rel)
  :match  (?rel ?a ?b)
  :assert (?rel ?b ?a)
  :why "s" :priority 100)
(rule sibling-exclusive (?siblings-via ?exclusive-under)
  :match  (and (?siblings-via ?a ?T) (?siblings-via ?b ?T) (neq ?a ?b))
  :assert (not (?exclusive-under ?a ?b))
  :why "sib" :priority 300)
(relation is-a       T T)
(relation co-located T T)
(symmetric         co-located)
(sibling-exclusive is-a co-located)
(is-a Thing T) (is-a A Thing) (is-a B Thing)
"""


def test_producible_relations():
    """A relation a rule positively asserts is producible; one only
    ever negated, or never asserted, is not."""
    assert producible_relations(_kb(_PUZZLE)) == frozenset({"co-located"})


def test_emit_closed_closes_inert_relation():
    """`is-a` — concluded by no rule — gets a `(__closed__ is-a)` fact."""
    kb = _kb(_PUZZLE)
    newly = emit_closed(kb)
    assert "is-a" in newly
    assert kb._fact_by_id("__closed__", ("is-a",)) is not None


def test_emit_closed_keeps_producible_open():
    """`co-located` — rule-propagated — is left open."""
    kb = _kb(_PUZZLE)
    emit_closed(kb)
    assert kb._fact_by_id("__closed__", ("co-located",)) is None


def test_emit_closed_idempotent():
    """A second pass is a no-op — a relation already `(__closed__ …)`
    is not re-emitted."""
    kb = _kb(_PUZZLE)
    emit_closed(kb)
    assert emit_closed(kb) == []


def test_negative_assert_is_not_production():
    """A rule whose only `:assert` is `(not …)` makes nothing
    producible — every declared relation closes."""
    kb = _kb("""
    (rule sibling-exclusive (?siblings-via ?exclusive-under)
      :match  (and (?siblings-via ?a ?T) (?siblings-via ?b ?T) (neq ?a ?b))
      :assert (not (?exclusive-under ?a ?b))
      :why "sib" :priority 300)
    (relation is-a       T T)
    (relation co-located T T)
    (sibling-exclusive is-a co-located)
    (is-a Thing T) (is-a A Thing) (is-a B Thing)
    """)
    assert producible_relations(kb) == frozenset()
    assert set(emit_closed(kb)) == {"is-a", "co-located"}
