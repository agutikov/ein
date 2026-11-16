# Q-M1e2.18 — A keyword pair is no guard's argument, and a fact names no relation

Two shapes that would read nothing are refused before anything runs:

- **A keyword pair as a guard's argument**, `(eq ?x :k a)`: a guard compares two values, and a keyword pair is
  none, so the shape is a parse error at the keyword.
- **A relation parameter bound to a fact.** `(rule r (?R) :match (box (?R b)) …)` with the activator `(r (a b))`
  names a relation by a fact, which names none. An activator that does so is refused at load where it is written, a
  fact or a literal in a rule's `:assert`; where a rule could derive it through a variable, the program is refused
  at load too ([Q-M1f.1](Q-M1f.1-a-rule-that-could-derive-an-activator-binding.md)).

**Why.** Each shape loads and matches nothing in silence otherwise: a guard on a keyword pair never holds, and a
pattern whose relation is a fact never matches, so a rule that reads one is a rule that never fires, with no word
said. What a guard may compare is [Q-M1e2.26](Q-M1e2.26-eq-and-neq-are-the-relations-of-equality-and-non.md)'s and
[Q-M1f.30](Q-M1f.30-a-guard-argument-holds-no-variable-inside-a-fact.md)'s.

**Held by.** `ein-cli/tests/cli_semantics.rs`: `a_rule_shape_that_reads_nothing_is_refused_at_load`, and
`ein-ir/src/from_ir.rs`' unit test of the same name; `ein-infer/tests/compile_semantics.rs`:
`a_relation_parameter_a_rule_binds_to_a_nested_fact_is_refused`; the grammar's table,
`ein-ir/tests/grammar_pages.rs`.
