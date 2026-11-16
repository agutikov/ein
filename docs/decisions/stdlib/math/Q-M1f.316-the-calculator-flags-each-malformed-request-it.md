# Q-M1f.316 — The calculator flags each malformed request it can detect, and the flag is a contradiction

What the calculator can tell of a malformed request by monotone rules, it flags, and a rule of its own derives
`(false)` from each flag, a step after it, as from a division by zero: a numeral's faults
([Q-M1f.274](decimal/Q-M1f.274-a-malformed-numeral-is-flagged-on-its-smallest.md)), an integer's or a rational's
shape ([Q-M1f.276](Q-M1f.276-an-integers-and-a-rationals-own-shape-is-checked.md)), a digit where a number goes,
`(malformed REQUEST)`, a `let` that cannot be read
([Q-M1f.207](decimal/Q-M1f.207-a-let-that-cannot-be-read-is-flagged-whether.md),
[Q-M1f.281](decimal/Q-M1f.281-a-let-that-names-a-digit-is-flagged.md)), and a name no `let` states
([Q-M1f.317](decimal/Q-M1f.317-a-name-no-let-states-is-flagged-as-undeclared.md)). Root is dead, R is empty, every
command exits 0, and the core names the fact the fault was written in.

**Why.** With a theory imported its names mean what the theory says, so a request it cannot read is the program's
error; answered by silence, it would read as a request still waiting.

**Held by.** `ein-cli/tests/arith_search_free.rs`: `every_arithmetic_program_is_search_free`, which holds each
flagged program to an empty R decided by root and to the rule its core names; `ein-infer/tests/arith_theory.rs`:
`a_malformed_request_is_flagged_once_and_never_answered`, `a_let_that_cannot_be_read_is_flagged`.
