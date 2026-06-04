"""`std.algebra` — converse / imply family + algebra lemmas (P1.8 S1.8.A7).

All rules are generic (parametrised over relations), imported flat with the
puzzle declaring activator facts. The algebra lemmas derive `converse` /
`symmetric` activators and rely on reflective rule-implication (A9): a derived
`(converse R R)` activates the `converse` rule on the next pass. The lemma loop
converges (every back-derivation already exists), pinned here.
"""
from __future__ import annotations

from ein_bot.inference.saturator import Saturator, SaturatorStepLimitError
from ein_bot.ir import parse
from ein_bot.kb.store import KnowledgeBase

_FAMILY = ("(import std.algebra :symbols "
           "(converse imply1 imply2-fwd imply2-reverse))\n")
_LEMMAS = ("(import std.algebra :symbols "
           "(converse symmetric-is-self-converse self-converse-is-symmetric "
           "converse-pair-symmetric))\n")
# A puzzle's own `symmetric` rule, for the lemma chain.
_SYM = ('(rule symmetric (?rel) :match (?rel ?a ?b) :assert (?rel ?b ?a) '
        ':why "s" :priority 100)\n')


def _derived(src: str, max_steps: int = 3000):
    kb = KnowledgeBase.from_ir(parse(src))
    fs = [f for f in Saturator(kb).saturate(max_steps=max_steps)
          if not f.redundant]
    return {(f.derived.relation_name, f.derived.args) for f in fs}


# ── imply / converse family ────────────────────────────────────────


def test_converse_mirrors():
    d = _derived(_FAMILY + "(relation r1 T T) (relation r2 T T)\n"
                 "(converse r1 r2)\n(r1 A B :source \"(1)\")")
    assert ("r2", ("B", "A")) in d


def test_imply1_property_to_property():
    """imply1's headline use: turn a property marker into another property."""
    d = _derived(_FAMILY + "(relation foo T T)\n"
                 "(imply1 functional closed)\n(functional foo)")
    assert ("closed", ("foo",)) in d


def test_imply2_fwd_copies():
    d = _derived(_FAMILY + "(relation r1 T T) (relation r2 T T)\n"
                 "(imply2-fwd r1 r2)\n(r1 A B :source \"(1)\")")
    assert ("r2", ("A", "B")) in d


def test_imply2_reverse_is_converse():
    d = _derived(_FAMILY + "(relation r1 T T) (relation r2 T T)\n"
                 "(imply2-reverse r1 r2)\n(r1 A B :source \"(1)\")")
    assert ("r2", ("B", "A")) in d


# ── algebra lemmas (reflective) ────────────────────────────────────


def test_symmetric_derives_self_converse():
    d = _derived(_LEMMAS + _SYM + "(relation knows T T)\n"
                 "(symmetric knows)\n(knows A B :source \"(1)\")")
    assert ("converse", ("knows", "knows")) in d
    # reflective: the derived (converse knows knows) (and symmetric) mirror it
    assert ("knows", ("B", "A")) in d


def test_self_converse_derives_symmetric():
    d = _derived(_LEMMAS + _SYM + "(relation knows T T)\n"
                 "(converse knows knows)\n(knows A B :source \"(1)\")")
    assert ("symmetric", ("knows",)) in d
    assert ("knows", ("B", "A")) in d


def test_converse_pair_is_symmetric():
    d = _derived(_LEMMAS + "(relation r1 T T) (relation r2 T T)\n"
                 "(converse r1 r2)\n"
                 "(r1 A B :source \"(1)\") (r2 C D :source \"(2)\")")
    assert ("converse", ("r2", "r1")) in d          # the swapped pair
    assert ("r2", ("B", "A")) in d                  # r1 → r2 mirror
    assert ("r1", ("D", "C")) in d                  # r2 → r1 mirror (reflective)


def test_lemma_loop_terminates():
    """symmetric ⇄ converse-R-R + converse-pair-symmetric must converge, not
    run away (the reflective dedup bounds the loop — Q-S1.8.A9.A)."""
    src = (_LEMMAS + _SYM + "(relation knows T T)\n"
           "(symmetric knows)\n(knows A B :source \"(1)\")")
    kb = KnowledgeBase.from_ir(parse(src))
    try:
        n = sum(1 for _ in Saturator(kb).saturate(max_steps=5000))
    except SaturatorStepLimitError:
        raise AssertionError("algebra lemma loop did not terminate") from None
    assert n < 100
