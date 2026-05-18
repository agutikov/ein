"""Grammar acceptance + rejection tests for the IR kernel (S1.1.1).

The kernel is Level B with generic-facts:
  · Ontology holds only schema (types + relation signatures + a-priori).
  · Facts are generic `(NAME args*)` — relation instances, instance
    declarations, property applications, all the same shape.
  · Rule parameter lists are mandatory (empty `()` for non-generic).
  · `=` is the one reserved fact-head; everything else is generic.
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


# ═══════════ Ontology (schema only) ═══════════

def test_ontology_empty():
    _ok("(ontology)")


def test_ontology_type_with_parent():
    _ok("(ontology (type Person) (type Engineer Person))")


def test_ontology_relation_binary():
    _ok("""
    (ontology
      (type Person) (type House)
      (relation lives-in (Person House) :cardinality 1..1))
    """)


def test_ontology_relation_nary():
    _ok("""
    (ontology
      (type Attribute)
      (relation between-three (Attribute Attribute Attribute)))
    """)


def test_ontology_apriori():
    _ok("""
    (ontology
      (a-priori right-of (House House) :pattern (right-of ?a ?b)))
    """)


# ═══════════ Facts — generic (NAME args*) ═══════════

def test_facts_empty():
    _ok("(facts)")


def test_facts_eq():
    _ok("(facts (= (color House_1) Red))")


def test_facts_instance():
    """Instance is a fact, not an ontology decl."""
    _ok("(facts (instance Norwegian Nationality))")


def test_facts_relation_with_source():
    _ok("""
    (facts
      (lives-in Norwegian House_1 :source "condition (10)")
      (drinks Milk House_3        :source "condition (9)"))
    """)


def test_facts_nary():
    _ok("(facts (next-to Norwegian Englishman Spaniard))")


def test_facts_property_application():
    """Rule applications are facts about relations."""
    _ok("""
    (facts
      (symmetric co-located)
      (transitive co-located)
      (reflexive co-located)
      (implies right-of next-to))
    """)


def test_facts_meta_relation():
    """All-different / constraints are just facts with the constraint
    name as head — no `constraint` wrapper."""
    _ok("(facts (all-different House_1 House_2 House_3 House_4 House_5))")


# ═══════════ Rules ═══════════

def test_rules_empty():
    _ok("(rules)")


def test_rules_empty_params():
    """Non-generic rule: empty `()` parameter list, fires universally."""
    _ok("(rules (rule foo () :match a :assert b :why \"test\"))")


def test_rules_generic_param():
    """Generic rule: non-empty parameter list. The rule is applied
    by matching `(symmetric REL)` facts and substituting ?rel."""
    _ok("""
    (rules
      (rule symmetric (?rel)
        :match  (?rel ?a ?b)
        :assert (?rel ?b ?a)
        :why    "{?rel} is symmetric: {?a} ↔ {?b}."
        :priority 1))
    """)


def test_rules_two_params():
    """Generic rule with two parameters (e.g., implies)."""
    _ok("""
    (rules
      (rule implies (?p ?q)
        :match  (?p ?a ?b)
        :assert (?q ?a ?b)
        :why    "{?p} implies {?q}."))
    """)


def test_rules_explicit_guard():
    """Explicit `:where` form — alternative to generic params."""
    _ok("""
    (rules
      (rule triangle-composition ()
        :match (and (?r ?a ?b)
                    (?r ?b ?c)
                    :where (transitive ?r))
        :assert (?r ?a ?c)
        :why "From {0} and {1}, since {?r} is transitive."
        :priority 10))
    """)


def test_rules_head_wildcard_pattern():
    """Head-wildcard pattern from T1.1.1.2: (_ x y ...)"""
    _ok("(rules (rule any-binary () :match (_ ?a ?b) :assert ?a))")


def test_rules_equality_pattern():
    """`=` as a list head inside a pattern body."""
    _ok("(rules (rule eq-elim () :match (= ?a ?b) :assert ?a))")


# ═══════════ Query ═══════════

def test_query_solve():
    _ok("(query :mode solve :goal (drinks Water ?h))")


def test_query_gaps():
    _ok("(query :mode gaps :goal (lives-in _ House_1))")


def test_query_contradictions():
    _ok("(query :mode contradictions :goal (lives-in Englishman ?h))")


# ═══════════ Trace ═══════════

def test_trace_empty():
    _ok("(trace)")


def test_trace_step_only():
    _ok("""
    (trace
      (step s1 :rule from-condition
               :using (c10)
               :derives (lives-in Norwegian House_1)
               :source "condition (10)"))
    """)


def test_trace_with_branch_and_contradiction():
    _ok("""
    (trace
      (step s1 :rule from-condition :using (c10)
               :derives (lives-in Norwegian House_1))
      (step s2 :rule exclusivity :using (s1)
               :derives (not (lives-in Norwegian House_2)))
      (branch-open s3 :on (lives-in Englishman ?h)
                      :choices (s3_1 s3_2 s3_3 s3_4 s3_5))
      (step s3_1 :rule hypothesis
                 :assumes (lives-in Englishman House_1)
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
    (facts (lives-in a b))
    """)
    assert len(tree.children) == 1


# ═══════════ Multiple top-level forms ═══════════

def test_all_five_top_level_forms():
    tree = _ok("""
    (ontology (type Person))
    (facts (lives-in a b))
    (rules (rule x () :match a :assert b))
    (query :mode solve :goal X)
    (trace)
    """)
    assert len(tree.children) == 5


# ═══════════ Rejection: structural errors ═══════════

def test_reject_double_keyword():
    """`(?var :key :key)` — KEYWORD can't be a value."""
    _bad("(rules (rule x () :match (?var :key :key) :assert ?var))")


def test_reject_keyword_as_list_head():
    """KEYWORD as a list head is rejected."""
    _bad("(rules (rule x () :match (:foo :bar value) :assert _))")


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
    _bad("(query :mode :solve :goal X)")


def test_reject_unknown_ontology_head():
    """Ontology accepts only `type`, `relation`, `a-priori`."""
    _bad("(ontology (foo Person))")


def test_reject_instance_in_ontology():
    """`instance` was moved to facts."""
    _bad("(ontology (instance Norwegian Nationality))")


def test_reject_unknown_trace_head():
    _bad("(trace (foo s1 :rule x))")


def test_reject_rule_missing_params():
    """Rule parameter list is mandatory — `()` for non-generic."""
    _bad("(rules (rule x :match a :assert b))")


def test_reject_rule_without_kw_pairs():
    _bad("(rules (rule x))")
    _bad("(rules (rule x ()))")


def test_reject_query_without_kw_pairs():
    _bad("(query)")


def test_reject_step_at_top_level():
    """`step` is only valid inside (trace ...)."""
    _bad("(step s1 :rule x)")


def test_reject_branch_open_at_top_level():
    _bad("(branch-open s1 :on X)")


def test_reject_unclosed_paren():
    _bad("(rules (rule x () :match a")


def test_reject_bare_close_paren():
    _bad(")")


# ═══════════ Integration: bundled examples ═══════════

def test_examples_zebra_parses():
    """The full Zebra puzzle (examples/zebra.ein) parses under the
    Level B kernel — S1.1.1 acceptance smoke test."""
    from pathlib import Path

    repo_root = Path(__file__).resolve().parent.parent
    zebra = repo_root / "examples" / "zebra.ein"
    assert zebra.exists(), f"missing: {zebra}"
    tree = parse(zebra.read_text(encoding="utf-8"))
    # 4 top-level forms: rules, ontology, facts, query.
    assert len(tree.children) == 4
