"""Compiler tests — S1.3.1 T1.3.1.9."""
from __future__ import annotations

from ein_bot.inference.compile import (
    AbsentGuard,
    Guard,
    Join,
    NestedPattern,
    Scan,
    compile_rule,
)
from ein_bot.ir import parse
from ein_bot.ir.types import Var
from ein_bot.kb.store import KnowledgeBase


def _kb_with(text: str) -> KnowledgeBase:
    return KnowledgeBase.from_ir(parse(text))


def test_compile_transitive_with_activator_bakes_relation():
    """For (transitive co-located), the plan's relation slots are
    `co-located` literals — no free `?rel` head var remains."""
    kb = _kb_with("""
    (rule transitive (?rel)
      :match (and (?rel ?a ?b) (?rel ?b ?c) (neq ?a ?c))
      :assert (?rel ?a ?c)
      :why "{?rel} is transitive")
    (relation co-located T T)
    (transitive co-located)
    """)
    rule = kb.rules["transitive"]
    activator = kb._facts_by_relation["transitive"][0]
    plan = compile_rule(rule, activator)

    assert plan.rule_name == "transitive"
    assert plan.activator_args == ("co-located",)
    assert plan.bindings_seed == {"rel": "co-located"}

    # Three steps: Scan, Join, Guard.
    assert len(plan.steps) == 3
    s0, s1, s2 = plan.steps
    assert isinstance(s0, Scan) and s0.relation == "co-located"
    assert isinstance(s1, Join) and s1.relation == "co-located"
    assert isinstance(s2, Guard) and s2.predicate == "neq"

    # No step references a free `?rel` head var (it was baked).
    for step in (s0, s1):
        for slot in step.arg_slots:
            assert not (isinstance(slot, Var) and slot.name == "rel")


def test_compile_join_shared_vars():
    """The second relation step records the shared var(s) with the first."""
    kb = _kb_with("""
    (rule transitive (?rel)
      :match (and (?rel ?a ?b) (?rel ?b ?c))
      :assert (?rel ?a ?c)
      :why "t")
    (relation r T T) (transitive r)
    """)
    plan = compile_rule(
        kb.rules["transitive"], kb._facts_by_relation["transitive"][0],
    )
    s0, s1 = plan.steps[:2]
    assert isinstance(s0, Scan)
    assert isinstance(s1, Join)
    assert s1.shared_vars == frozenset({"b"})


def test_compile_type_exclusivity_emits_negative_assert():
    """`:assert (not (co-located ?a ?b))` lowers to a nested-fact template."""
    kb = _kb_with("""
    (rule type-exclusivity ()
      :match (and (instance ?a ?T) (instance ?b ?T) (neq ?a ?b))
      :assert (not (co-located ?a ?b))
      :why "x")
    (type T)
    """)
    plan = compile_rule(kb.rules["type-exclusivity"], None)
    assert plan.activator_args == ()

    template = plan.assert_template
    assert isinstance(template, NestedPattern)
    assert template.relation == "not"
    # Outer arg is a nested co-located pattern.
    inner = template.arg_slots[0]
    assert isinstance(inner, NestedPattern)
    assert inner.relation == "co-located"


def test_compile_absent_premise_emits_absent_guard():
    """`(absent (rel a b))` inside :match emits an AbsentGuard
    (S1.5.8c K-Δ.2).

    The inner step is a Join (not Scan) because ?a and ?b are
    already bound by the outer `(?r ?a ?b)` premise — the inner
    relation must respect those bindings.
    """
    kb = _kb_with("""
    (rule guarded (?r)
      :match (and (?r ?a ?b) (absent (other ?a ?b)))
      :assert (ok ?a ?b)
      :why "g")
    (relation r T T) (relation other T T) (guarded r)
    """)
    plan = compile_rule(
        kb.rules["guarded"], kb._facts_by_relation["guarded"][0],
    )
    assert any(isinstance(s, AbsentGuard) for s in plan.steps)
    neg = next(s for s in plan.steps if isinstance(s, AbsentGuard))
    assert len(neg.sub_steps) == 1
    sub = neg.sub_steps[0]
    assert isinstance(sub, Join) and sub.relation == "other"
    assert sub.shared_vars == frozenset({"a", "b"})


def test_compile_not_premise_emits_scan_with_nested_pattern():
    """`(not (rel a b))` inside :match no longer means NAF
    (S1.5.8c K-Δ.1). It compiles as a generic relation pattern
    with head ``not`` and the inner expression as a
    NestedPattern arg, matching STORED ``(not …)`` facts in the
    KB.
    """
    kb = _kb_with("""
    (rule see-neg (?r)
      :match (and (?r ?a ?b) (not (other ?a ?b)))
      :assert (saw-neg ?a ?b)
      :why "saw stored neg of other for {?a},{?b}")
    (relation r T T) (relation other T T) (see-neg r)
    """)
    plan = compile_rule(
        kb.rules["see-neg"], kb._facts_by_relation["see-neg"][0],
    )
    # The (not …) premise must NOT compile to an AbsentGuard.
    assert not any(isinstance(s, AbsentGuard) for s in plan.steps)
    # It compiles as a Join on "not" with a NestedPattern arg.
    not_step = next(
        (s for s in plan.steps if isinstance(s, (Join, Scan)) and s.relation == "not"),
        None,
    )
    assert not_step is not None, "expected a Join/Scan on relation 'not'"
    assert len(not_step.arg_slots) == 1
    inner = not_step.arg_slots[0]
    assert isinstance(inner, NestedPattern)
    assert inner.relation == "other"


def test_compile_nested_match_pattern():
    """Q40 — `(hypothesis (co-located ?a ?b))` compiles to a Scan
    on `hypothesis` with a NestedPattern arg slot."""
    kb = _kb_with("""
    (rule hyp-test ()
      :match (hypothesis (co-located ?a ?b))
      :assert (caught ?a ?b)
      :why "h")
    (relation co-located T T) (relation hypothesis T)
    """)
    plan = compile_rule(kb.rules["hyp-test"], None)
    assert len(plan.steps) == 1
    s = plan.steps[0]
    assert isinstance(s, Scan) and s.relation == "hypothesis"
    nested = s.arg_slots[0]
    assert isinstance(nested, NestedPattern)
    assert nested.relation == "co-located"
    assert [getattr(x, "name", x) for x in nested.arg_slots] == ["a", "b"]


def test_compile_drops_where_keyword():
    """Q32 — `:where (neq ?a ?b)` is silently dropped at compile time
    (no Guard emitted). Authors should write positional `(neq …)` —
    the migration is mechanical."""
    kb = _kb_with("""
    (rule old-where (?r)
      :match (and (?r ?a ?b) :where (neq ?a ?b))
      :assert (?r ?b ?a)
      :why "ow")
    (relation r T T) (old-where r)
    """)
    plan = compile_rule(
        kb.rules["old-where"], kb._facts_by_relation["old-where"][0],
    )
    # Only the Scan(r) survives — the :where kw_pair is dropped.
    assert len(plan.steps) == 1
    assert isinstance(plan.steps[0], Scan)
