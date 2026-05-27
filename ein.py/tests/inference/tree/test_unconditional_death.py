"""Unconditional-death predicate tests — S1.5.7 T1.5.7.1.

``is_unconditional_death`` decides whether a dead branch's
contradiction depends on a speculative fact. The walk is transitive:
a ``rule``-kind unsat-core fact that derives from a hypothesis several
firings deep is *conditional* even though its own provenance kind is
``'rule'`` — the case the shallow ``provenance.kind`` read misses.
"""
from __future__ import annotations

from ein_bot.inference.tree.back_prop import (
    is_unconditional_death,
    reaches_hypothesis,
)
from ein_bot.kb.entities import Fact, Layer
from ein_bot.kb.provenance import Provenance
from ein_bot.kb.store import KnowledgeBase


def _kb() -> KnowledgeBase:
    kb = KnowledgeBase()
    kb.rebuild_indexes()
    return kb


def _put(kb: KnowledgeBase, fact: Fact) -> Fact:
    """Add + index a fact; return the stored canonical Fact."""
    stored = kb.add_fact(fact)
    kb._index_fact(stored)
    return stored


def _source(kb: KnowledgeBase, rel: str, args: tuple, src: str = "(1)") -> Fact:
    return _put(kb, Fact(
        relation_name=rel, args=args, layer=Layer.FACT,
        provenance=Provenance.from_source(src),
    ))


def _hypothesis(kb: KnowledgeBase, rel: str, args: tuple,
                branch: int = 1) -> Fact:
    return _put(kb, Fact(
        relation_name=rel, args=args, layer=Layer.REASONING,
        provenance=Provenance.from_hypothesis(branch=branch),
    ))


def _derived(kb: KnowledgeBase, rel: str, args: tuple, *,
             rule: str, premises: list[Fact]) -> Fact:
    """A rule-kind fact deriving from `premises`."""
    return _put(kb, Fact(
        relation_name=rel, args=args, layer=Layer.REASONING,
        provenance=Provenance.from_rule(
            rule=rule,
            premises_raw=tuple((p.relation_name, p.args) for p in premises),
        ),
    ))


# ── Base / defensive cases ─────────────────────────────────────────


def test_empty_core_is_conditional():
    """An empty unsat-core is unattributable — never licenses back-prop."""
    assert is_unconditional_death(_kb(), frozenset()) is False


def test_unprovenanced_fact_is_unconditional():
    """A fact with no provenance is a given, not a hypothesis."""
    kb = _kb()
    orphan = _put(kb, Fact(relation_name="r", args=("A", "B"),
                           layer=Layer.FACT, provenance=None))
    assert is_unconditional_death(kb, frozenset({orphan})) is True


# ── The four T1.5.7.1.b cases ──────────────────────────────────────


def test_all_source_chain_is_unconditional():
    """A source fact + a rule-fact whose chain grounds out at sources."""
    kb = _kb()
    s = _source(kb, "s", ("A",))
    d = _derived(kb, "d", ("A",), rule="r1", premises=[s])
    assert is_unconditional_death(kb, frozenset({s, d})) is True


def test_direct_hypothesis_is_conditional():
    """A hypothesis-kind fact in the core — the obvious conditional case."""
    kb = _kb()
    h = _hypothesis(kb, "h", ("A",))
    assert is_unconditional_death(kb, frozenset({h})) is False


def test_deep_hypothesis_is_conditional():
    """A rule-kind core fact two firings above a hypothesis.

    Its own ``provenance.kind`` is ``'rule'``, so the shallow read
    would wrongly call this unconditional; the transitive walk catches
    it.
    """
    kb = _kb()
    h = _hypothesis(kb, "h", ("A",))
    d1 = _derived(kb, "d1", ("A",), rule="r1", premises=[h])
    d2 = _derived(kb, "d2", ("A",), rule="r2", premises=[d1])
    assert d2.provenance.kind == "rule"          # shallow read sees 'rule'
    assert is_unconditional_death(kb, frozenset({d2})) is False


def test_provenance_cycle_terminates():
    """A self-referential provenance chain must not infinite-loop.

    Load-time ``detect_provenance_cycles`` rejects these, but the
    ``visited`` guard is the defensive backstop if one ever forms.
    """
    kb = _kb()
    c1 = _put(kb, Fact(
        relation_name="c1", args=("A",), layer=Layer.REASONING,
        provenance=Provenance.from_rule(
            rule="r", premises_raw=(("c2", ("A",)),)),
    ))
    _put(kb, Fact(
        relation_name="c2", args=("A",), layer=Layer.REASONING,
        provenance=Provenance.from_rule(
            rule="r", premises_raw=(("c1", ("A",)),)),
    ))
    # No hypothesis anywhere in the cycle ⇒ unconditional; the call
    # must return rather than recurse forever.
    assert is_unconditional_death(kb, frozenset({c1})) is True


# ── rejected kind + mixed core ─────────────────────────────────────


def test_rejected_kind_is_conditional():
    """A ``rejected``-kind (retracted-hypothesis) fact is still
    speculative — a death resting on it cannot back-propagate."""
    kb = _kb()
    rej = _put(kb, Fact(
        relation_name="h", args=("A",), layer=Layer.REASONING,
        provenance=Provenance.rejected(branch=2),
    ))
    assert is_unconditional_death(kb, frozenset({rej})) is False


def test_mixed_core_one_conditional_fact_taints_all():
    """One hypothesis-tainted fact among otherwise-unconditional ones
    makes the whole death conditional."""
    kb = _kb()
    s = _source(kb, "s", ("A",))
    clean = _derived(kb, "d", ("A",), rule="r1", premises=[s])
    h = _hypothesis(kb, "h", ("B",))
    tainted = _derived(kb, "t", ("B",), rule="r2", premises=[h])
    assert is_unconditional_death(kb, frozenset({clean, tainted})) is False


# ── reaches_hypothesis (the reusable single-fact walk) ─────────────


def test_reaches_hypothesis_single_fact():
    """The public single-fact walk T1.5.7.6 reuses for forced-deduction
    classification."""
    kb = _kb()
    s = _source(kb, "s", ("A",))
    h = _hypothesis(kb, "h", ("A",))
    forced = _derived(kb, "f", ("A",), rule="r1", premises=[s])
    speculative = _derived(kb, "g", ("A",), rule="r2", premises=[h])
    assert reaches_hypothesis(kb, forced) is False
    assert reaches_hypothesis(kb, speculative) is True


# ── own_hypothesis exemption (T1.5.7.2 wiring) ─────────────────────


def test_own_hypothesis_excluded_is_unconditional():
    """The branch's own hypothesis in its core is benign — exempted
    from the count, so the death reads as unconditional."""
    kb = _kb()
    h = _hypothesis(kb, "h", ("A",))
    assert is_unconditional_death(
        kb, frozenset({h}), own_hypothesis=h) is True


def test_own_plus_other_hypothesis_is_conditional():
    """Own hypothesis is exempt, but a *second* hypothesis is not —
    the death rests on an external speculation."""
    kb = _kb()
    h = _hypothesis(kb, "h", ("A",))
    other = _hypothesis(kb, "other", ("B",), branch=9)
    assert is_unconditional_death(
        kb, frozenset({h, other}), own_hypothesis=h) is False


def test_own_hypothesis_reached_transitively_excluded():
    """A core fact rule-derived from the *own* hypothesis is still
    benign — the ``own`` exemption applies at the hypothesis terminal,
    so it covers facts transitively derived from it too."""
    kb = _kb()
    h = _hypothesis(kb, "h", ("A",))
    derived = _derived(kb, "d", ("A",), rule="r1", premises=[h])
    assert is_unconditional_death(
        kb, frozenset({derived}), own_hypothesis=h) is True
