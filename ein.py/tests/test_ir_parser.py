"""Grammar acceptance + rejection tests for the IR kernel (S1.1.1).

The kernel is Level B with generic-facts:
  · Ontology holds only schema (types + relation signatures).
  · Facts are generic `(NAME args*)` — relation instances, instance
    declarations, property applications, all the same shape.
  · Rule parameter lists are mandatory (empty `()` for non-generic).
  · `=` is the one reserved fact-head; everything else is generic.
"""
import pytest
from lark.exceptions import LarkError

from ein.ir import IRParseError, parse_tree


def _ok(text: str):
    tree = parse_tree(text)
    assert tree is not None
    return tree


def _bad(text: str):
    with pytest.raises((LarkError, IRParseError)):
        parse_tree(text)


# ═══════════ Ontology (schema only) ═══════════

def test_ontology_empty():
    _ok("(type Person)")


def test_ontology_type_with_parent():
    _ok("(type Person) (type Engineer Person)")


def test_ontology_relation_binary():
    _ok("""
    (type Person) (type House)
    (relation lives-in Person House :cardinality 1..1)
    """)


def test_ontology_relation_nary():
    _ok("""
    (type Attribute)
    (relation between-three Attribute Attribute Attribute)
    """)


def test_apriori_is_now_a_plain_symbol():
    # `a-priori` was a vestigial alias of `relation`; S1.7.6 T1.7.6.1
    # removed it from the kernel. It is no longer a reserved declarator
    # — `a-priori` now lexes as an ordinary SYMBOL, so `(a-priori …)`
    # parses as a plain generic ontology fact, like any domain head.
    _ok("(a-priori right-of House House)")


# ═══════════ Facts — generic (NAME args*) ═══════════

def test_facts_empty():
    _ok("(lives-in Norwegian House-1)")


def test_facts_eq():
    _ok("(= (color House-1) Red)")


def test_facts_instance():
    """Instance is a fact, not an ontology decl."""
    _ok("(instance Norwegian Nationality)")


def test_facts_relation_with_source():
    _ok("""
    (lives-in Norwegian House-1 :source "condition (10)")
    (drinks Milk House-3        :source "condition (9)")
    """)


def test_facts_nary():
    _ok("(next-to Norwegian Englishman Spaniard)")


def test_facts_property_application():
    """Rule applications are facts about relations."""
    _ok("""
    (symmetric co-located)
    (transitive co-located)
    (reflexive co-located)
    (implies right-of next-to)
    """)


def test_facts_meta_relation():
    """All-different / constraints are just facts with the constraint
    name as head — no `constraint` wrapper."""
    _ok("(all-different House-1 House-2 House-3 House-4 House-5)")


# ═══════════ Reserved kernel meta-primitives ═══════════
# `not`, `and`, `or`, `neq` are shape-pinned: they have fixed arity and
# a dedicated grammar rule. `instance` left the reserved set in S1.7.6
# (now a plain relation). Domain relations stay generic SYMBOL-headed.

def test_instance_fact_arity_2():
    _ok("(instance Norwegian Nationality)")


def test_instance_with_kwargs():
    _ok('(instance Norwegian Nationality :source "(8)")')


def test_instance_in_pattern():
    """`(instance ?a ?T)` works in :match patterns too."""
    _ok("""
    (rule type-exclusivity ()
      :match (and (instance ?a ?T) (instance ?b ?T) :where (neq ?a ?b))
      :assert (not (co-located ?a ?b)))
    """)


def test_instance_arity_now_unchecked():
    """S1.7.6: `instance` is no longer a reserved declarator, so the
    grammar no longer pins its arity. `(instance X)` (1 arg) and
    `(instance X Y Z)` (3 args) both parse as ordinary generic facts
    now — arity is a loader/validator concern, not a parse error."""
    _ok("(instance Norwegian)")
    _ok("(instance Norwegian Nationality Spaniard)")


def test_reject_and_at_top_level():
    """`(and …)` is a pattern combinator, never a fact. Post-P1.7c (flat
    top level) the test is at top level: `and` is SYMBOL-excluded so it
    can't head a `generic_fact`, and `and_form` is not a top-level `?form`."""
    _bad("(and a b)")


def test_reject_or_at_top_level():
    _bad("(or a b)")


def test_reject_neq_at_top_level():
    """`(neq …)` is a `:where`/`:match` predicate, not a fact."""
    _bad("(neq a b)")


def test_not_fact():
    """`(not X)` IS a permitted fact form (negative assertion)."""
    _ok("(not (co-located Spaniard Coffee))")


def test_reject_not_arity_2():
    """`not` is unary."""
    _bad("(not a b)")


def test_reject_neq_arity_1():
    _bad("(rule x () :match (and (?r ?a ?b) :where (neq ?a)) :assert ?a)")


def test_and_or_neq_in_pattern():
    """All three appear inside :match patterns."""
    _ok("""
    (rule x ()
      :match  (and (?r ?a ?b)
                   (or (drinks ?a Tea) (drinks ?a Milk))
                   :where (neq ?a ?b))
      :assert ?a)
    """)


# ═══════════ Rules (flat `(rule …)` / `(hrule …)` declarators) ═══════════

def test_rule_empty_params():
    """Non-generic rule: empty `()` parameter list, fires universally."""
    _ok("(rule foo () :match a :assert b :why \"test\")")


def test_rule_generic_param():
    """Generic rule: non-empty parameter list. The rule is applied
    by matching `(symmetric REL)` facts and substituting ?rel."""
    _ok("""
    (rule symmetric (?rel)
      :match  (?rel ?a ?b)
      :assert (?rel ?b ?a)
      :why    "{?rel} is symmetric: {?a} ↔ {?b}."
      :priority 1)
    """)


def test_rule_two_params():
    """Generic rule with two parameters (e.g., implies)."""
    _ok("""
    (rule implies (?p ?q)
      :match  (?p ?a ?b)
      :assert (?q ?a ?b)
      :why    "{?p} implies {?q}.")
    """)


def test_rule_explicit_guard():
    """Explicit `:where` form — alternative to generic params."""
    _ok("""
    (rule triangle-composition ()
      :match (and (?r ?a ?b)
                  (?r ?b ?c)
                  :where (transitive ?r))
      :assert (?r ?a ?c)
      :why "From {0} and {1}, since {?r} is transitive."
      :priority 10)
    """)


def test_rule_head_wildcard_pattern():
    """Head-wildcard pattern from T1.1.1.2: (_ x y ...)"""
    _ok("(rule any-binary () :match (_ ?a ?b) :assert ?a)")


def test_rule_equality_pattern():
    """`=` as a list head inside a pattern body."""
    _ok("(rule eq-elim () :match (= ?a ?b) :assert ?a)")


# ═══════════ Macro declarator (P1.8 S1.5.9) ═══════════

def test_macro_decl_multi_param():
    """`(macro NAME (?p…) BODY)` — the forall sugar as an ein macro."""
    _ok("(macro forall (?b ?G ?B) (absent (and ?G (absent ?B))))")


def test_macro_decl_single_param():
    """One parameter, an `(and …)` body — the open sugar."""
    _ok("(macro open (?P) (and (absent ?P) (absent (not ?P))))")


def test_macro_body_can_be_a_var():
    """The body is any `value`, including a bare var."""
    _ok("(macro id (?x) ?x)")


def test_reject_macro_no_params():
    """`macro_params` is `VAR+` — an empty param list is a parse error."""
    _bad("(macro foo () (rel ?a))")


def test_reject_macro_no_body():
    """The body `value` is mandatory."""
    _bad("(macro foo (?p))")


def test_reject_macro_bare():
    """`macro` is SYMBOL-excluded: a bare `(macro)` is a parse error, not a
    silent fact head named `macro`."""
    _bad("(macro)")


def test_macro_is_not_a_plain_symbol_head():
    """The negative-lookahead forbids `macro` as a fact head entirely."""
    _bad("(macro ?x ?y)")


# ═══════════ Import declarator + dotted atoms (P1.8 S1.8.A2) ═══════════

def test_import_bare():
    """`(import MODULE)` — whole-module, qualified-by-default."""
    _ok("(import std.macro)")


def test_import_as_alias():
    _ok("(import std.macro :as m)")


def test_import_symbols():
    _ok("(import std.macro :symbols (forall open))")


def test_reject_import_no_module():
    """`import` requires a module SYMBOL — `(import)` is a parse error."""
    _bad("(import)")


def test_dotted_atom_is_one_symbol():
    """`.` is a SYMBOL char: a dotted name lexes as a single atom (used for
    module names + qualified references)."""
    _ok("(rel a.b c.d.e)")
    _ok("(rule r () :match (std.macro.forall ?x) :assert (y ?x) :why \"w\")")


def test_dotted_atom_cannot_start_with_reserved_word():
    """The negative-lookahead is start-anchored: a dotted atom may not BEGIN
    with a reserved word (`import.x` is rejected; `std.import` would be fine)."""
    _bad("(import.foo Bar)")


def test_range_still_lexes_despite_dotted_atoms():
    """`1..5` stays a RANGE (digit-anchored), not an atom — no regression
    from adding `.` to the letter-anchored SYMBOL."""
    _ok("(relation r A B :cardinality 1..1)")


# ═══════════ Query ═══════════

def test_query_goal():
    _ok("(query :goal (drinks Water ?h))")


def test_query_goal_text():
    _ok('(query :goal (drinks Water ?h) :goal-text "who drinks at {?h}")')


def test_query_conjunctive_goal():
    _ok("(query :goal (and (drinks Water ?h) (lives-in Norwegian ?h)))")


# ═══════════ Trace ═══════════

def test_trace_empty():
    _ok("(trace)")


def test_trace_step_only():
    _ok("""
    (trace
      (step s1 :rule from-condition
               :using (c10)
               :derives (lives-in Norwegian House-1)
               :source "condition (10)"))
    """)


def test_trace_with_branch_and_contradiction():
    _ok("""
    (trace
      (step s1 :rule from-condition :using (c10)
               :derives (lives-in Norwegian House-1))
      (step s2 :rule exclusivity :using (s1)
               :derives (not (lives-in Norwegian House-2)))
      (branch-open s3 :on (lives-in Englishman ?h)
                      :choices (s3_1 s3_2 s3_3 s3_4 s3_5))
      (step s3_1 :rule hypothesis
                 :assumes (lives-in Englishman House-1)
                 :derives (contradiction-with s1))
      (contradiction c-branch :using (s3_1) :assumption s3_1)
      (branch-close s3 :choose s3_2)
      (symmetry-class sc1 :over (House-1 House-2) :note "numbering irrelevant"))
    """)


# ═══════════ Comments ═══════════

def test_line_comment():
    tree = _ok("""
    ; this is a comment
    (type Person)
    ; trailing comment
    """)
    assert len(tree.children) == 1


def test_block_comment():
    tree = _ok("""
    #|
       multi-line block comment
       per SMT-LIB convention
    |#
    (lives-in a b)
    """)
    assert len(tree.children) == 1


# ═══════════ Multiple top-level forms ═══════════

def test_flat_top_level_forms():
    """A flat program mixes declarators, facts, a query, and a trace — each
    classified by its own head (P1.7c). No block wrappers."""
    tree = _ok("""
    (relation lives-in Person House)
    (type Person)
    (lives-in a b :source "(1)")
    (lives-in c d :rule symmetric :using (s1))
    (rule x () :match a :assert b)
    (query :goal X)
    (trace)
    """)
    assert len(tree.children) == 7


def test_derived_facts_use_rule_using_provenance():
    """Derived facts carry `:rule` / `:using` provenance instead of `:source`
    (they re-classify to the REASONING layer at load)."""
    _ok("""
    (co-located Blue House-2 :rule square-fwd :using (c10 c15))
    (not (co-located Norwegian House-2) :rule type-exclusivity :using (c10))
    """)


# ═══════════ Rejection: structural errors ═══════════

def test_reject_double_keyword():
    """`(?var :key :key)` — KEYWORD can't be a value."""
    _bad("(rule x () :match (?var :key :key) :assert ?var)")


def test_reject_keyword_as_list_head():
    """KEYWORD as a list head is rejected."""
    _bad("(rule x () :match (:foo :bar value) :assert _)")


def test_unknown_top_level_head_is_a_fact():
    """The flat contract (P1.7c): any head NOT in the closed declarator set
    {relation, rule, hrule, query, config} (+ the `trace` sibling) is a
    FACT — 'detect facts by *not* being reserved'. `(unknown-head …)` used
    to be a top-level parse error; it now parses as a generic fact."""
    from ein.ir import parse
    forms = parse("(unknown-head a b c)")
    assert len(forms) == 1
    assert forms[0].head.name == "unknown-head"


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
    _bad("(query :goal :solve)")


def test_ontology_accepts_implicit_facts():
    """Ontology holds implicit assumptions in addition to schema:
    instance enumerations and rule-application facts both live here."""
    _ok("""
    (type Nationality)
    (instance Norwegian Nationality)
    (symmetric co-located)
    (implies right-of next-to)
    """)


def test_ontology_head_is_now_a_fact():
    """Post-P1.7c: `ontology` is no longer a block wrapper with a strict
    decl interior — at top level it is an ordinary fact head (any
    non-declarator head is a fact). A body the old wrapper would have
    rejected, like `(ontology :foo bar)`, now reads as a fact
    `(ontology :foo bar)` (head `ontology`, kw-pair `:foo bar`)."""
    _ok("(ontology :foo bar)")


def test_reject_unknown_trace_head():
    """`trace` IS still shape-pinned (SYMBOL-excluded): `(trace …)` may
    only contain trace events, so an unknown interior head is rejected at
    parse time (unlike a declarator typo, which the loader catches)."""
    _bad("(trace (foo s1 :rule x))")


def test_reject_rule_missing_params():
    """Rule parameter list is mandatory — `()` for non-generic."""
    _bad("(rule x :match a :assert b)")


def test_reject_rule_without_kw_pairs():
    _bad("(rule x)")
    _bad("(rule x ())")


def test_reject_query_without_kw_pairs():
    _bad("(query)")


def test_step_at_top_level_is_a_fact():
    """Post-P1.7c flat model: trace-event heads (`step`, `branch-open`, …)
    are special ONLY inside `(trace …)`. At top level they are not in the
    closed declarator set, so they parse as ordinary facts (`step` is a
    plain SYMBOL — only the declarators + `trace` are SYMBOL-excluded)."""
    _ok("(step s1 :rule x)")


def test_branch_open_at_top_level_is_a_fact():
    _ok("(branch-open s1 :on X)")


def test_reject_unclosed_paren():
    _bad("(rule x () :match a")


def test_reject_bare_close_paren():
    _bad(")")


# ═══════════ S1.5.8c T1.5.8c.2 — * in identifier tail ═══════════
# `*` is purely a character in identifier names; no Kleene or
# arithmetic meaning. Lets `is-a*` (transitive closure of is-a)
# and `?R*` parse as ordinary atoms / vars.

def test_star_in_symbol_tail():
    _ok("(relation is-a* T T)")


def test_star_in_var_tail():
    _ok(
        "(rule lift (?R*) :match (?R* ?a ?b) "
        ":assert (alias ?a ?b) :why \"t\")",
    )


def test_star_in_both_atom_and_var():
    _ok(
        "(relation is-a* T T) "
        "(rule closure (?R ?R*) "
        ":match (?R ?a ?b) :assert (?R* ?a ?b) :why \"c\")",
    )


# ═══════════ Integration: bundled examples ═══════════

def test_examples_zebra_parses():
    """The full Zebra puzzle (examples/zebra.ein) parses under the flat
    kernel (P1.7c) — S1.1.1 acceptance smoke test. No block wrappers: a
    sequence of rule decls, relation decls, facts, and one query."""
    from pathlib import Path

    from ein.ir import parse

    repo_root = Path(__file__).resolve().parents[2]
    zebra = repo_root / "examples" / "zebra.ein"
    assert zebra.exists(), f"missing: {zebra}"
    forms = parse(zebra.read_text(encoding="utf-8"))
    heads = [f.head.name for f in forms]
    assert "query" in heads
    assert heads.count("rule") >= 5            # property / square / exclusivity rules
    assert "co-located" in heads               # a puzzle fact at top level
    assert not any(h in ("ontology", "facts", "reasoning", "rules")
                   for h in heads)             # wrappers are gone
