# Q-M1f.268 — Binary flags a malformed request as far as decimal does

`std.math.binary-arith` flags what decimal flags, by the shared rules and one rule of its own over the bits: a bit
where a number goes, `(bin-malformed REQUEST)`; a `bin-let` that cannot be read,
`(bin-malformed (bin-let NAME N))`; and a name no `bin-let` states, `(bin-undeclared NAME REQUEST)`, by
`bin-operand-undeclared`. A decimal digit is a name in binary: `(bin-add 7 X)` is undeclared. Binary is silent
where decimal is, and on a decimal numeral where a binary one goes.

**Why.** A program in either base meets the same faults, flagged the same way.

**Held by.** `ein-infer/tests/binary_theory.rs`: `a_malformed_request_is_flagged_once_and_never_answered`,
`an_undeclared_name_is_flagged_and_a_let_answers_it`, `a_let_that_cannot_be_read_is_flagged`, `what_stays_silent`;
`tests/stdlib/math/binary-arith/10_a_bit_as_an_operand.ein` to `22_what_stays_silent.ein`.
