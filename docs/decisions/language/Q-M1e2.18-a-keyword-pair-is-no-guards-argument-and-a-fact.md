# Q-M1e2.18 — A keyword pair is no guard's argument, and a fact names no relation

Two shapes read nothing:

- **A keyword pair as a guard's argument**, `(eq ?x :k a)`: a guard compares two objects, and a keyword pair is
  none, so the shape is a parse error at the keyword.
- **A relation parameter bound to a fact.** `(rule r (?R) :match (box (?R b)) …)` with the activator `(r (a b))`
  names a relation by a fact, which names none, so the activator activates nothing: it is held as the fact it is,
  the rule never fires on it, and nothing says so, whether the program writes it or a rule derives it
  ([Q-M1f.1](Q-M1f.1-a-rule-that-could-derive-an-activator-binding.md)).

**Why.** A guard on a keyword pair never holds and is no expression of the language, so the grammar refuses it. An
activator that names no relation is a fact a program may hold, so it is held, and it activates nothing, as an
activator of another count does ([Q-M1f.115](Q-M1f.115-an-activator-is-a-fact-of-its-rules-name-and-one.md)). What
a guard may compare is [Q-M1e2.26](Q-M1e2.26-eq-and-neq-are-the-relations-of-equality-and-non.md)'s and
[Q-M1f.30](Q-M1f.30-a-guard-argument-holds-no-variable-inside-a-fact.md)'s.

**Held by.** `ein-cli/tests/cli_semantics.rs`: `a_rule_shape_that_reads_nothing_is_refused_at_load`, which loads
`examples/ein-bugs/wrong-answers/nested-fact-activator.ein` on every command and reads no line of its activator, and
`ein-ir/src/from_ir.rs`' unit test of the same name; `ein-infer/tests/compile_semantics.rs`:
`a_relation_parameter_a_rule_binds_to_a_nested_fact_activates_nothing`; the grammar's table,
`ein-ir/tests/grammar_pages.rs`; the regression `nested-fact-activator`.
