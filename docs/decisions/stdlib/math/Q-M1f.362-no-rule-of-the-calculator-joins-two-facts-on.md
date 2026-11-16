# Q-M1f.362 — No rule of the calculator joins two facts on a number

Every rule of the calculator's modules has at most one premise that holds numbers. Its others are rows of a table
looked up by digits, guards, which compare and look nothing up, or a probe whose argument the first premise binds.
Two rules of `std.math.positional` join on a **name**, which the fact index keys: `operand-name`, which asks
`(named ?x)` first so that a number operand stops there, and `let-twice`. A computation is a chain of such steps:
each call carries in its tag what its caller needs next, and each walk keeps its accumulator in the state, so a
rule reads one fact and a table.

**Why.** The engine's fact index does not key on a nested fact, so a join of two facts on a number would scan a
relation for every fact of the other, and a batch of requests would cost the square of its count. Written this way,
a batch costs in proportion to its count.

**Held by.** `ein-infer/tests/arith_theory.rs` and `num9_theory.rs`: `no_rule_joins_two_facts_on_a_number`, on each
module's text and on the shared rules as each base activates them; `int_theory.rs`, `rat_theory.rs` and
`gcd_theory.rs`: `every_rule_has_one_premise`; `ein-infer/tests/arith_batch.rs`, on the matcher's count.
