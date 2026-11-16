# Q-M1f.30 — A guard argument holds no variable inside a fact

A guard compares two values, each a variable, a name, or a fact that holds no variable: `(eq ?x (f a))` compares
`?x` with the value `(f a)`. A form that holds a variable, `(eq ?x (f ?y))`, is a pattern and no value, and is
refused at load, in a rule and in a query's goal, one line per form: *… and the form (f ?y) holds a variable — a
guard compares two values, each a variable, a name, or a fact that holds no variable*. The same question written as
a premise, `(p (f ?y))`, is answered.

**Why.** `eq` binds a free side to the other side's value, and here that value is a fact the match would build: the
matcher reads the fact store and never adds to it, so it could answer only where the store happened to hold the
fact, and would miss a row elsewhere in silence.

**Held by.** `ein-cli/tests/cli_semantics.rs`: `a_rule_shape_that_reads_nothing_is_refused_at_load`, and
`ein-ir/src/from_ir.rs`' unit test of the same name. What a guard is:
[Q-M1e2.26](Q-M1e2.26-eq-and-neq-are-the-relations-of-equality-and-non.md).
