# Q-M1f.210 — A malformed number is its representation's fault, flagged where it is written

Whether a term is a well-formed number is asked by the module that defines its representation, of what a file
writes, wherever it is written, with no calculation reading it: `std.math.decimal` and `std.math.binary` check a
numeral ([Q-M1f.273](decimal/Q-M1f.273-the-decimal-numeral-is-std-math-decimal-s-which.md),
[Q-M1f.295](binary/Q-M1f.295-the-binary-numeral-is-std-math-binary-s-which.md)), `std.math.int` an integer's own
shape and `std.math.rat` a rational's ([Q-M1f.276](Q-M1f.276-an-integers-and-a-rationals-own-shape-is-checked.md)).
Each reads the loader's `(written T)`, stated for every fact nested in a fact a file writes and in a rule's
conclusion that holds no variable. What is wrong with an operand, a digit where a number goes or a name no `let`
states, is the fault of the module that reads the operand, flagged on the request
([Q-M1f.206](decimal/Q-M1f.206-an-operands-fault-travels-with-its-tag.md)).

**Why.** A numeral that holds a letter is no number from the start, as on paper: finding it need not wait for a
calculation to use it, and a module that computes with numbers need not know how each is spelt.

**Held by.** `tests/stdlib/math/decimal/02_in_a_let_nobody_asks_about.ein`, `03_in_a_fact_of_the_programs_own.ein`
and `04_in_a_rules_conclusion.ein`; `ein-infer/tests/int_theory.rs`:
`a_malformed_integer_and_an_operands_fault_are_flagged_as_written`; `rat_theory.rs`:
`a_malformed_rational_is_flagged_as_written`; `ein-cli/tests/written_facts.rs`: `what_the_loader_states`.
