"""Two-argument `sibling-exclusive` — S1.5.6 T1.5.6.1.

The rule library's `sibling-exclusive` takes two relation
parameters: `?siblings-via` (the relation whose shared parent
defines the sibling group) and `?exclusive-under` (the relation
those siblings cannot pairwise co-occur in). Both activator
patterns must produce the expected `(not …)` derivations.
"""
from ein_bot.inference.saturator import Saturator
from ein_bot.ir import parse
from ein_bot.kb.store import KnowledgeBase

_SIB = """
(rule sibling-exclusive (?siblings-via ?exclusive-under)
  :match  (and (?siblings-via ?a ?T) (?siblings-via ?b ?T) (neq ?a ?b))
  :assert (not (?exclusive-under ?a ?b))
  :why "sib" :priority 300)
(relation is-a       T T)
(relation co-located T T)
(sibling-exclusive is-a co-located)
(sibling-exclusive is-a is-a)
(is-a Color T)
(is-a Red Color) (is-a Blue Color)
"""


def _saturated(text: str) -> KnowledgeBase:
    kb = KnowledgeBase.from_ir(parse(text))
    list(Saturator(kb).saturate())
    return kb


def test_sibling_exclusive_is_two_param():
    """The rewritten rule declares two parameters."""
    kb = KnowledgeBase.from_ir(parse(_SIB))
    rule = kb.rules["sibling-exclusive"]
    assert len(rule.params) == 2


def test_co_located_activator_derives_negative():
    """(sibling-exclusive is-a co-located): is-a siblings cannot be
    co-located — both orderings derived."""
    kb = _saturated(_SIB)
    assert ("co-located", ("Red", "Blue")) in kb._negated_facts
    assert ("co-located", ("Blue", "Red")) in kb._negated_facts


def test_is_a_activator_derives_negative():
    """(sibling-exclusive is-a is-a): is-a siblings are not is-a
    each other — the leaf-pair exclusion."""
    kb = _saturated(_SIB)
    assert ("is-a", ("Red", "Blue")) in kb._negated_facts
    assert ("is-a", ("Blue", "Red")) in kb._negated_facts
