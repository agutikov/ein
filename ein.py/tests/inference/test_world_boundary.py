"""Closure/worlds boundary + negative provenance — S1.21.8.

The stage's own machinery, as opposed to the semantics it changes (those are
pinned in ``test_absent_semantics.py``): the compile-time split, the boundary
view, the two-phase loop's observables, and the negative premises a firing
admitted through the boundary records.
"""
from __future__ import annotations

from ein.inference.compile import AbsentGuard, compile_rule, split_naf
from ein.inference.saturator import Saturator
from ein.inference.world import World, project
from ein.ir import parse
from ein.kb.store import KnowledgeBase

_MACRO = "(import std.macro :symbols (forall))\n"


def _kb(text: str, macros: bool = False) -> KnowledgeBase:
    return KnowledgeBase.from_ir(parse((_MACRO if macros else "") + text))


def _plan(kb: KnowledgeBase, name: str):
    rule = kb.rules[name]
    activator = None if not rule.params else kb._facts_by_relation[name][0]
    return compile_rule(rule, activator)


class TestCompileSplit:
    def test_closure_plan_is_purely_positive(self):
        kb = _kb("""
        (rule gate () :match (and (seed ?x) (absent (r ?x)))
          :assert (ok ?x) :why "g" :priority 100)
        (relation seed T) (relation r T) (relation ok T)
        """)
        plan = _plan(kb, "gate")
        assert not any(isinstance(s, AbsentGuard) for s in plan.steps)
        assert plan.has_naf
        (steps, guards) = plan.disjuncts()[0]
        assert [getattr(s, "relation", None) for s in steps] == ["seed"]
        assert len(guards) == 1

    def test_scope_preserves_the_guards_binding_context(self):
        # The whole reason a lifted guard is as strong as an in-place one:
        # `(absent (P ?x))` BEFORE `(Q ?x)` asks "no P at all"; after it,
        # "no P for this x". Scope records which question was written.
        early, _ = _kb("""
        (rule early () :match (and (absent (p ?x)) (q ?x))
          :assert (ok ?x) :why "e" :priority 100)
        (relation p T) (relation q T) (relation ok T)
        """), None
        plan = _plan(early, "early")
        ((g,),) = plan.naf_guards
        assert g.scope == frozenset()            # nothing bound yet

        late = _kb("""
        (rule late () :match (and (q ?x) (absent (p ?x)))
          :assert (ok ?x) :why "l" :priority 100)
        (relation p T) (relation q T) (relation ok T)
        """)
        ((g2,),) = _plan(late, "late").naf_guards
        assert g2.scope == frozenset({"x"})

    def test_order_of_the_guard_still_decides_the_answer(self):
        # ... and it must still decide it the same way after lifting.
        prog = """
        (rule gate () :match {body}
          :assert (ok ?x) :why "g" :priority 100)
        (relation p T) (relation q T) (relation ok T)
        (q A :source "(1)") (p B :source "(2)")
        """
        # `absent` first: ?x unbound, so it asks "is p empty?" — it is not.
        early = Saturator(_kb(prog.format(
            body="(and (absent (p ?x)) (q ?x))")))
        list(early.saturate())
        assert not [f for f in early.kb.facts if f.relation_name == "ok"]
        # `absent` after `(q ?x)`: asks "no p for A?" — correct, so it fires.
        late = Saturator(_kb(prog.format(
            body="(and (q ?x) (absent (p ?x)))")))
        list(late.saturate())
        assert [f.args for f in late.kb.facts if f.relation_name == "ok"] \
            == [("A",)]

    def test_each_or_disjunct_keeps_its_own_guards(self):
        # The D5 shape, structurally: guards are paired with the disjunct
        # that produced them, so no caller can walk one tuple and miss another.
        kb = _kb("""
        (rule gate ()
          :match (or (and (t1 ?x) (absent (r1 ?x)))
                     (and (t2 ?x) (absent (r2 ?x))))
          :assert (gated ?x) :why "g" :priority 100)
        (relation t1 T) (relation t2 T) (relation r1 T) (relation r2 T)
        (relation gated T)
        """)
        plan = _plan(kb, "gate")
        got = []
        for steps, guards in plan.disjuncts():
            assert not any(isinstance(s, AbsentGuard) for s in steps)
            got.append((
                [getattr(s, "relation", None) for s in steps],
                sorted(r for g in guards for r in g.watched),
            ))
        assert got == [(["t1"], ["r1"]), (["t2"], ["r2"])]

    def test_nested_absent_is_not_lifted_and_is_not_monotone(self):
        kb = _kb("""
        (rule undefeated ()
          :match (and (player ?p)
                      (forall ?q (and (player ?q) (neq ?p ?q)) (beats ?p ?q)))
          :assert (undefeated ?p) :why "u" :priority 100)
        (relation player T) (relation beats T T) (relation undefeated T)
        """, macros=True)
        ((outer,),) = _plan(kb, "undefeated").naf_guards
        assert any(isinstance(s, AbsentGuard) for s in outer.sub_steps)
        # A nested absent can flip fail -> pass as the KB grows, so the
        # candidate must stay parked rather than be retired on first failure.
        assert outer.monotone is False
        assert outer.watched == frozenset({"player", "beats"})

    def test_plain_absent_is_monotone(self):
        kb = _kb("""
        (rule gate () :match (and (seed ?x) (absent (r ?x)))
          :assert (ok ?x) :why "g" :priority 100)
        (relation seed T) (relation r T) (relation ok T)
        """)
        ((g,),) = _plan(kb, "gate").naf_guards
        assert g.monotone is True
        assert g.watched == frozenset({"r"})

    def test_scope_includes_the_rules_activator_parameters(self):
        # REGRESSION. `scope` used to start empty and collect only vars bound
        # by positive Scan/Join steps, so a rule *parameter* — bound from the
        # activator, in scope for the whole match — was projected away at the
        # boundary. Harmless for relation slots (`_slot` substitutes a bound
        # param to a literal at compile time) but NOT for a predicate `Guard`,
        # which is compiled from the raw IR: `(eq ?y ?P)` resolved `?P` to
        # None, the negative query found nothing, and the guard passed when it
        # had to fail.
        kb = _kb("""
        (rule gate (?P)
          :match  (and (trigger ?a) (absent (and (r ?a ?y) (eq ?y ?P))))
          :assert (gated ?a) :why "g" :priority 200)
        (relation trigger T) (relation r T T) (relation gated T)
        (relation gate T)
        (gate BAD :source "activator")
        (trigger X :source "(1)")
        (r X BAD :source "(2)")
        """)
        ((g,),) = _plan(kb, "gate").naf_guards
        assert "P" in g.scope
        list(Saturator(kb).saturate())
        assert not [f for f in kb.facts if f.relation_name == "gated"], (
            "the guard must see ?P: (r X BAD) with BAD=?P blocks the firing"
        )

    def test_split_naf_on_a_plan_with_no_guards_is_identity(self):
        steps = _plan(_kb("""
        (rule plain () :match (seed ?x) :assert (ok ?x) :why "p" :priority 100)
        (relation seed T) (relation ok T)
        """), "plain").steps
        positive, guards = split_naf(steps)
        assert positive == steps and guards == ()


class TestBoundary:
    def test_project_restricts_to_scope(self):
        assert project({"a": 1, "b": 2}, frozenset({"a"})) == {"a": 1}
        assert project({"a": 1}, frozenset()) == {}

    def test_holds_and_absent_are_world_queries(self):
        kb = _kb("""
        (rule gate () :match (and (seed ?x) (absent (r ?x)))
          :assert (ok ?x) :why "g" :priority 100)
        (relation seed T) (relation r T) (relation ok T)
        (seed A :source "(1)") (seed B :source "(2)") (r B :source "(3)")
        """)
        plan = _plan(kb, "gate")
        ((g,),) = plan.naf_guards
        world = World(kb)
        assert world.absent(g, {"x": "A"}) is True      # no (r A)
        assert world.absent(g, {"x": "B"}) is False     # (r B) exists
        assert world.admits((g,), {"x": "A"}) is True
        assert world.first_failing((g,), {"x": "B"}) is g
        assert world.first_failing((g,), {"x": "A"}) is None

    def test_world_carries_its_commitment(self):
        kb = _kb("(relation r T)")
        assert World(kb).commitment == ()
        assert World(kb, (("r", ("A",)),)).commitment == (("r", ("A",)),)


class TestTwoPhaseLoop:
    def test_guarded_candidates_are_parked_not_queued(self):
        kb = _kb("""
        (rule gate () :match (and (seed ?x) (absent (r ?x)))
          :assert (ok ?x) :why "g" :priority 100)
        (relation seed T) (relation r T) (relation ok T)
        (seed A :source "(1)")
        """)
        sat = Saturator(kb)
        sat._enqueue_pass()
        assert sat._queue == []                  # nothing purely positive
        assert len(sat._parked) == 1
        # ... and the boundary then admits it against the quiesced world.
        list(sat.saturate())
        assert [f.args for f in kb.facts if f.relation_name == "ok"] == [("A",)]
        assert sat.naf_admitted == 1
        assert sat.naf_dropped == 0

    def test_a_failed_monotone_guard_retires_the_candidate(self):
        kb = _kb("""
        (rule gate () :match (and (seed ?x) (absent (r ?x)))
          :assert (ok ?x) :why "g" :priority 100)
        (relation seed T) (relation r T) (relation ok T)
        (seed A :source "(1)") (r A :source "(2)")
        """)
        sat = Saturator(kb)
        list(sat.saturate())
        assert sat.naf_retired == 1
        assert sat._parked == []                 # not re-asked forever
        assert sat.naf_admitted == 0

    def test_is_stalled_consults_the_boundary(self):
        # A parked candidate whose guards pass is an available firing; a
        # saturator that answered from the positive queue alone would claim
        # to be stalled one round early.
        kb = _kb("""
        (rule gate () :match (and (seed ?x) (absent (r ?x)))
          :assert (ok ?x) :why "g" :priority 100)
        (relation seed T) (relation r T) (relation ok T)
        (seed A :source "(1)")
        """)
        sat = Saturator(kb)
        assert sat.is_stalled() is False
        list(sat.saturate())
        assert sat.is_stalled() is True


class TestNegativeProvenance:
    def test_a_boundary_admitted_firing_records_what_was_absent(self):
        kb = _kb("""
        (rule gate () :match (and (seed ?x) (absent (r ?x)))
          :assert (ok ?x) :why "g" :priority 100)
        (relation seed T) (relation r T) (relation ok T)
        (seed A :source "(1)")
        """)
        list(Saturator(kb).saturate())
        ok = kb._fact_by_id("ok", ("A",))
        prov = ok.provenance
        assert prov.premises_raw == (("seed", ("A",)),)     # positive deps
        assert prov.absent_premises == (("r", ("A",)),)     # negative deps
        # Deps(Y) = PositiveDeps(Y) u NegativeDeps(Y) — both are now visible.

    def test_free_positions_are_recorded_as_none(self):
        kb = _kb("""
        (rule gate () :match (and (seed ?x) (absent (r ?x ?y)))
          :assert (ok ?x) :why "g" :priority 100)
        (relation seed T) (relation r T T) (relation ok T)
        (seed A :source "(1)")
        """)
        list(Saturator(kb).saturate())
        prov = kb._fact_by_id("ok", ("A",)).provenance
        # `?y` ranged free — the query was "no (r A _)", not "no (r A y)".
        assert prov.absent_premises == (("r", ("A", None)),)

    def test_a_purely_positive_firing_records_no_negative_premises(self):
        kb = _kb("""
        (rule chain () :match (seed ?x) :assert (ok ?x) :why "c" :priority 100)
        (relation seed T) (relation ok T)
        (seed A :source "(1)")
        """)
        list(Saturator(kb).saturate())
        assert kb._fact_by_id("ok", ("A",)).provenance.absent_premises == ()

    def test_an_alternative_justification_carries_its_own_negative_premises(self):
        # REGRESSION. Provenance is per *derivation* (S1.21.7), so a
        # re-derivation admitted through the boundary depends on what ITS
        # guards found missing. The redundant-firing path used to record the
        # alternative without guards, silently dropping that half.
        kb = _kb("""
        (rule r-pos () :match (other ?x) :assert (target ?x)
          :why "p" :priority 100)
        (rule r-naf () :match (and (seed ?x) (absent (block ?x)))
          :assert (target ?x) :why "n" :priority 500)
        (relation seed T) (relation other T) (relation block T)
        (relation target T)
        (seed A :source "(1)") (other A :source "(2)")
        """)
        list(Saturator(kb).saturate())
        target = kb._fact_by_id("target", ("A",))
        by_rule = {p.rule: p for p in kb.justifications(target)}
        assert by_rule["r-pos"].absent_premises == ()      # purely positive
        assert by_rule["r-naf"].absent_premises == (("block", ("A",)),)

    def test_nested_guard_records_both_levels(self):
        kb = _kb("""
        (rule undefeated ()
          :match (and (player ?p)
                      (forall ?q (and (player ?q) (neq ?p ?q)) (beats ?p ?q)))
          :assert (undefeated ?p) :why "u" :priority 100)
        (relation player T) (relation beats T T) (relation undefeated T)
        (player Alice :source "(1)") (player Bob :source "(2)")
        (beats Alice Bob :source "(3)")
        """, macros=True)
        list(Saturator(kb).saturate())
        alice = kb._fact_by_id("undefeated", ("Alice",))
        assert alice is not None
        rels = {r for r, _args in alice.provenance.absent_premises}
        # The whole query had to fail: the guard's own scan AND the nested
        # absent's — both are part of what "undefeated" depends on.
        assert rels == {"player", "beats"}
