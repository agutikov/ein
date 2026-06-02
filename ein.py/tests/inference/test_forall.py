"""Tests for the `(forall ?b (G) (B))` parser sugar — S1.5.8c T1.5.8c.3a.

`forall` is sugar that desugars at compile time to
`(absent (and G (absent B)))`. The matcher sees only the
desugared form; this test file pins both the surface semantics
(observable behaviour for users of the rule) and the desugaring
(the safety check that the bound var must appear in the guard).
"""
from __future__ import annotations

import pytest

from ein_bot.inference import match
from ein_bot.inference.compile import AbsentGuard, compile_rule
from ein_bot.ir import parse
from ein_bot.kb.store import KnowledgeBase


def _kb(text: str) -> KnowledgeBase:
    return KnowledgeBase.from_ir(parse(text))


def _run(kb: KnowledgeBase, rule_name: str) -> list:
    """Return the (bindings, premises) list for a rule's compiled plan."""
    rule = kb.rules[rule_name]
    # Empty-param rules use a single None activator; param rules read
    # from kb._facts_by_relation[rule_name].
    if not rule.params:
        activator = None
    else:
        activator = kb._facts_by_relation[rule_name][0]
    plan = compile_rule(rule, activator)
    return list(match.run(plan, kb))


def test_forall_desugars_to_nested_absent_guards():
    """The compiled plan for a forall rule must contain an
    AbsentGuard wrapping another AbsentGuard — the classical
    ¬∃x. G(x) ∧ ¬B(x) reduction."""
    kb = _kb("""
    (rule undefeated ()
      :match (and (player ?p)
                  (forall ?q
                    (and (player ?q) (neq ?p ?q))
                    (beats ?p ?q)))
      :assert (undefeated ?p)
      :why "u")
    (relation player T T) (relation beats T T)
    """)
    rule = kb.rules["undefeated"]
    plan = compile_rule(rule, None)
    # Find the outer AbsentGuard (the forall's outermost).
    outer = next(s for s in plan.steps if isinstance(s, AbsentGuard))
    # Inside its sub_steps there must be ANOTHER AbsentGuard
    # (the inner `(absent B)`).
    assert any(isinstance(s, AbsentGuard) for s in outer.sub_steps), (
        "forall must compile to (absent (and G (absent B))) — "
        "the inner absent is missing"
    )


def test_forall_all_pass_fires():
    """All ?q-bindings satisfy the body → forall is true → rule fires."""
    kb = _kb("""
    (rule undefeated ()
      :match (and (player ?p)
                  (forall ?q
                    (and (player ?q) (neq ?p ?q))
                    (beats ?p ?q)))
      :assert (undefeated ?p)
      :why "u")
    (relation player T T) (relation beats T T)
    (player Alice :source "(1)")
    (player Bob   :source "(2)")
    (player Carol :source "(3)")
    (beats Alice Bob   :source "(4)")
    (beats Alice Carol :source "(5)")
    """)
    results = _run(kb, "undefeated")
    # Alice is undefeated (beats both Bob and Carol).
    # Bob is not (doesn't beat anyone).
    # Carol is not.
    alice_results = [r for r, _ in results if r["p"] == "Alice"]
    assert len(alice_results) == 1
    other_results = [r for r, _ in results if r["p"] != "Alice"]
    assert other_results == []


def test_forall_any_fail_does_not_fire():
    """If even one ?q-binding lacks the body, forall fails."""
    kb = _kb("""
    (rule undefeated ()
      :match (and (player ?p)
                  (forall ?q
                    (and (player ?q) (neq ?p ?q))
                    (beats ?p ?q)))
      :assert (undefeated ?p)
      :why "u")
    (relation player T T) (relation beats T T)
    (player Alice :source "(1)")
    (player Bob   :source "(2)")
    (player Carol :source "(3)")
    ;; Alice beats Bob but NOT Carol — undefeated must fail.
    (beats Alice Bob :source "(4)")
    """)
    results = _run(kb, "undefeated")
    assert results == []


def test_forall_empty_domain_is_vacuously_true():
    """`(forall ?q (G) (B))` with G yielding no bindings →
    vacuously true. The rule fires."""
    kb = _kb("""
    (rule lone-undefeated ()
      :match (and (player ?p)
                  (forall ?q
                    (and (player ?q) (neq ?p ?q))
                    (beats ?p ?q)))
      :assert (undefeated ?p)
      :why "alone, hence undefeated")
    (relation player T T) (relation beats T T)
    (player Alice :source "(1)")
    """)
    results = _run(kb, "lone-undefeated")
    # Alice has no other player to beat → forall vacuous-true.
    assert len(results) == 1
    assert results[0][0]["p"] == "Alice"


def test_forall_rejects_bound_not_in_guard():
    """Safety: ?bound must appear in the guard; otherwise the
    matcher has no enumerable domain. Load-time rejection."""
    text = """
    (rule bad-rule ()
      :match (forall ?v
                (player ?other)
                (whatever ?v))
      :assert (oops)
      :why "wrong")
    (relation player T T) (relation whatever T T)
    """
    kb = _kb(text)
    with pytest.raises(ValueError, match=r"forall .v: bound var does not appear"):
        compile_rule(kb.rules["bad-rule"], None)


def test_forall_bound_does_not_escape():
    """The bound var `?q` is local to the forall — it shouldn't
    appear in the rule's :assert conclusion or be bound for
    outer steps after the forall."""
    kb = _kb("""
    (rule all-beaten ()
      :match (and (player ?p)
                  (forall ?q
                    (and (player ?q) (neq ?p ?q))
                    (beats ?p ?q)))
      :assert (all-beaten ?p)
      :why "all")
    (relation player T T) (relation beats T T)
    (player Alice :source "(1)")
    (player Bob   :source "(2)")
    (beats Alice Bob :source "(3)")
    """)
    results = _run(kb, "all-beaten")
    assert len(results) == 1
    # ?q is NOT in the outer bindings — the AbsentGuard's
    # sub-plan bindings don't escape.
    assert "q" not in results[0][0]
