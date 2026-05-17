"""Grammar acceptance + rejection tests for the IR kernel (S1.1.1).

The grammar is Level B — structurally typed kernel, generic interiors
inside kw_pair values. Acceptance tests cover every kernel form with a
realistic interior; rejection tests cover the obvious shapes the
grammar should refuse.
"""
import pytest
from lark.exceptions import LarkError

from ein_bot.ir import parse


def _ok(text: str):
    tree = parse(text)
    assert tree is not None
    return tree


def _bad(text: str):
    with pytest.raises(LarkError):
        parse(text)


# ═══════════ Ontology ═══════════

def test_ontology_empty():
    _ok("(ontology)")


def test_ontology_type_with_parent():
    _ok("(ontology (type Person) (type Engineer Person))")


def test_ontology_instance():
    _ok("(ontology (type Person) (instance Norwegian Person))")


def test_ontology_relation():
    _ok("""
    (ontology
      (type Person)
      (type House)
      (relation lives-in Person House :cardinality 1..1))
    """)


def test_ontology_apriori():
    _ok("""
    (ontology
      (a-priori right-of House House :pattern (rel right-of ?a ?b)))
    """)


# ═══════════ Facts ═══════════

def test_facts_empty():
    _ok("(facts)")


def test_facts_eq():
    _ok("(facts (= (color House_1) Red))")


def test_facts_rel_with_source():
    _ok("""
    (facts
      (rel lives-in Norwegian House_1 :source "condition (10)")
      (rel drinks Milk House_3        :source "condition (9)"))
    """)


def test_facts_hrel():
    _ok("(facts (hrel next-to Norwegian Englishman Spaniard))")


def test_facts_constraint():
    _ok("(facts (constraint all-different House_1 House_2 House_3 House_4 House_5))")


# ═══════════ Rules ═══════════

def test_rules_empty():
    _ok("(rules)")


def test_rules_triangle():
    _ok("""
    (rules
      (rule triangle-composition
        :match (and (rel ?r ?a ?b)
                    (rel ?r ?b ?c)
                    :where (transitive ?r))
        :assert (rel ?r ?a ?c)
        :why "From {0} and {1}, since {?r} is transitive, {?a} {?r} {?c}."
        :priority 10))
    """)


def test_rules_head_wildcard_pattern():
    # head-wildcard pattern from T1.1.1.2: (_ x y ...)
    _ok("(rules (rule any-binary :match (_ ?a ?b) :assert ?a))")


def test_rules_equality_pattern():
    # = as a list head inside a pattern body
    _ok("(rules (rule eq-elim :match (= ?a ?b) :assert ?a))")


# ═══════════ Query ═══════════

def test_query_solve():
    _ok("(query :mode solve :goal (rel drinks Water ?h))")


def test_query_gaps():
    _ok("(query :mode gaps :goal (rel _ _ House_1))")


def test_query_contradictions():
    _ok("(query :mode contradictions :goal (rel lives-in Englishman ?h))")


# ═══════════ Trace ═══════════

def test_trace_empty():
    _ok("(trace)")


def test_trace_step_only():
    _ok("""
    (trace
      (step s1 :rule from-condition
               :using (c10)
               :derives (rel lives-in Norwegian House_1)
               :source "condition (10)"))
    """)


def test_trace_with_branch_and_contradiction():
    _ok("""
    (trace
      (step s1 :rule from-condition :using (c10)
               :derives (rel lives-in Norwegian House_1))
      (step s2 :rule exclusivity :using (s1)
               :derives (not (rel lives-in Norwegian House_2)))
      (branch-open s3 :on (rel lives-in Englishman ?h)
                      :choices (s3_1 s3_2 s3_3 s3_4 s3_5))
      (step s3_1 :rule hypothesis
                 :assumes (rel lives-in Englishman House_1)
                 :derives (contradiction-with s1))
      (contradiction c-branch :using (s3_1) :assumption s3_1)
      (branch-close s3 :choose s3_2)
      (symmetry-class sc1 :over (House_1 House_2) :note "numbering irrelevant"))
    """)


# ═══════════ Comments ═══════════

def test_line_comment():
    tree = _ok("""
    ; this is a comment
    (ontology (type Person))
    ; trailing comment
    """)
    assert len(tree.children) == 1


def test_block_comment():
    tree = _ok("""
    #|
       multi-line block comment
       per SMT-LIB convention
    |#
    (facts (rel a b c))
    """)
    assert len(tree.children) == 1


# ═══════════ Multiple top-level forms ═══════════

def test_all_five_top_level_forms():
    tree = _ok("""
    (ontology (type Person))
    (facts (rel a b c))
    (rules (rule x :match a :assert b))
    (query :mode solve :goal X)
    (trace)
    """)
    assert len(tree.children) == 5


# ═══════════ Rejection: structural errors ═══════════

def test_reject_var_as_list_head():
    """The user's example — (?var :key :key) must not parse."""
    _bad("(rules (rule x :match (?var :key :key) :assert ?var))")


def test_reject_keyword_as_list_head():
    _bad("(rules (rule x :match (:foo :bar) :assert _))")


def test_reject_unknown_top_level_head():
    _bad("(unknown-head a b c)")


def test_reject_top_level_bare_atom():
    _bad("Norwegian")


def test_reject_top_level_keyword():
    _bad(":rule")


def test_reject_top_level_var():
    _bad("?house")


def test_reject_top_level_int():
    _bad("42")


def test_reject_top_level_string():
    _bad('"hello"')


def test_reject_keyword_followed_by_keyword():
    """:mode :solve — KEYWORD can't be a value."""
    _bad("(query :mode :solve :goal X)")


def test_reject_unknown_fact_head():
    _bad("(facts (foo a b c))")


def test_reject_unknown_ontology_head():
    _bad("(ontology (foo Person))")


def test_reject_unknown_trace_head():
    _bad("(trace (foo s1 :rule x))")


def test_reject_rule_without_kw_pairs():
    _bad("(rules (rule x))")


def test_reject_query_without_kw_pairs():
    _bad("(query)")


def test_reject_step_at_top_level():
    """step is only valid inside (trace ...)."""
    _bad("(step s1 :rule x)")


def test_reject_branch_open_at_top_level():
    _bad("(branch-open s1 :on X)")


def test_reject_instance_without_type():
    _bad("(ontology (instance Norwegian))")


def test_reject_rel_with_one_arg():
    _bad("(facts (rel lives-in Norwegian))")


def test_reject_unclosed_paren():
    _bad("(rules (rule x :match a")


def test_reject_bare_close_paren():
    _bad(")")
