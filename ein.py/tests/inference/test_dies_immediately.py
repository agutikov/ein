"""One-step lookahead — S1.5.6 T1.5.6.2 / T1.5.6.3.

Tests :meth:`Lookahead.dies_immediately`: a candidate is killed iff
adding it to the *already-saturated* KB yields a contradiction in
a single rule firing. Covers the contradiction shapes (positive
collision, direct ⊥, ``(not h)`` self-kill), the REASONING-layer
soundness guard, and the ``:enable-pre-branch-lookahead`` gate.
"""
from ein_bot.inference.config import SolverConfig
from ein_bot.inference.hypgen import generate_hypotheses_with_stats
from ein_bot.inference.lookahead import Lookahead, _is_contradiction
from ein_bot.inference.saturator import Saturator
from ein_bot.ir import parse
from ein_bot.kb.entities import Fact, Layer
from ein_bot.kb.store import KnowledgeBase


def _saturated_kb(text: str) -> KnowledgeBase:
    kb = KnowledgeBase.from_ir(parse(text))
    list(Saturator(kb).saturate())
    return kb


# A puzzle whose saturation derives the `(not (co-located …))`
# facts the transitive-join lookahead chains through. Red↔H1 is
# the only anchor.
_PUZZLE = """
(rule symmetric (?rel)
  :match  (?rel ?a ?b)
  :assert (?rel ?b ?a)
  :why "s" :priority 100)
(rule transitive (?rel)
  :match  (and (?rel ?a ?b) (?rel ?b ?c) (neq ?a ?c))
  :assert (?rel ?a ?c)
  :why "t" :priority 200)
(rule sibling-exclusive (?siblings-via ?exclusive-under)
  :match  (and (?siblings-via ?a ?T) (?siblings-via ?b ?T) (neq ?a ?b))
  :assert (not (?exclusive-under ?a ?b))
  :why "sib" :priority 300)
(relation is-a       T T)
(relation co-located T T)
(symmetric         co-located)
(transitive        co-located)
(sibling-exclusive is-a co-located)
(is-a Color T) (is-a House T)
(is-a Red Color) (is-a Blue Color)
(is-a H1  House) (is-a H2  House)
(co-located Red H1 :source "(1)")
"""


def test_transitive_join_death():
    """(co-located Blue H1) dies in one step: Blue↔H1 joins the
    saturated H1↔Red edge under `transitive` to give
    (co-located Blue Red), which `sibling-exclusive` already
    forbade — a positive-fact-collides-with-a-negative kill."""
    kb = _saturated_kb(_PUZZLE)
    h = Fact("co-located", ("Blue", "H1"), layer=Layer.REASONING)
    assert Lookahead(kb).dies_immediately(kb, h) is True


def test_survivor_is_alive():
    """(co-located Blue H2) has no one-step contradiction — H2
    carries no co-located fact to chain through."""
    kb = _saturated_kb(_PUZZLE)
    h = Fact("co-located", ("Blue", "H2"), layer=Layer.REASONING)
    assert Lookahead(kb).dies_immediately(kb, h) is False


def test_direct_false_kill():
    """A candidate that makes a relation non-functional trips the
    `functional` rule's `:assert (false)` — direct ⊥."""
    kb = _saturated_kb("""
    (rule functional (?R)
      :match  (and (?R ?a ?b) (?R ?a ?c) (neq ?b ?c))
      :assert (false)
      :why "fn" :priority 250)
    (relation co-located T T)
    (functional co-located)
    (co-located Red H1 :source "(1)")
    """)
    h = Fact("co-located", ("Red", "H2"), layer=Layer.REASONING)
    assert Lookahead(kb).dies_immediately(kb, h) is True


def test_self_negating_rule():
    """A rule that asserts `(not h)` straight from `h` — the
    self-falsification shape."""
    kb = _saturated_kb("""
    (rule deny (?R)
      :match  (?R ?a ?b)
      :assert (not (?R ?a ?b))
      :why "deny" :priority 100)
    (relation r T T)
    (deny r)
    """)
    h = Fact("r", ("A", "B"), layer=Layer.REASONING)
    assert Lookahead(kb).dies_immediately(kb, h) is True


def test_is_contradiction_layer_guard():
    """A derived `(not g)` against a FACT-layer `g` is a cross-layer
    pair — NOT a contradiction; the same shape at REASONING is. The
    guard is what stops the lookahead false-killing a live
    hypothesis."""
    kb = _saturated_kb("""
    (relation r T T)
    (r A B :source "given")
    """)
    g_fact = kb._fact_by_id("r", ("A", "B"))
    assert g_fact is not None and g_fact.layer is Layer.FACT

    h = Fact("co-located", ("X", "Y"), layer=Layer.REASONING)
    not_fact_layer = Fact("not", (Fact("r", ("A", "B")),))
    assert _is_contradiction(kb, not_fact_layer, h) is False

    reasoning_g = Fact("r", ("C", "D"), layer=Layer.REASONING)
    kb.add_fact(reasoning_g)
    kb._index_fact(reasoning_g)
    not_reasoning = Fact("not", (Fact("r", ("C", "D")),))
    assert _is_contradiction(kb, not_reasoning, h) is True


def test_config_off_keeps_doomed_candidate():
    """With `:enable-pre-branch-lookahead false` the filter is not
    applied — the doomed candidate is generated, counter stays 0."""
    kb = _saturated_kb(_PUZZLE)
    kb.config = SolverConfig(enable_pre_branch_lookahead=False)
    facts, stats = generate_hypotheses_with_stats(kb)
    assert stats.filtered.get("lookahead_killed", 0) == 0
    assert Fact("co-located", ("Blue", "H1")) in facts


def test_config_on_kills_doomed_candidate():
    """With the lookahead on (the default) the doomed candidate is
    dropped and `HypGenStats` records the per-filter count."""
    kb = _saturated_kb(_PUZZLE)
    kb.config = SolverConfig()
    facts, stats = generate_hypotheses_with_stats(kb)
    assert stats.filtered.get("lookahead_killed", 0) > 0
    assert Fact("co-located", ("Blue", "H1")) not in facts


def test_no_rules_never_kills():
    """A KB with no rules has no plans — nothing can die in one
    step, so every candidate is reported alive."""
    kb = _saturated_kb('(relation co-located T T)')
    h = Fact("co-located", ("A", "B"), layer=Layer.REASONING)
    assert Lookahead(kb).dies_immediately(kb, h) is False
