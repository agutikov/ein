# Q-M1f.154 — The calculator's harness is exhaustive to 120 and sampled to 30 digits

`std.math.decimal-arith` is held to Rust's integers, `u128`, which hold 38 decimal digits. Every operation is asked
of every pair of operands from 0 to 120; of every pair of lengths from 0 to 30 digits, on a sample drawn from a
fixed seed; with a carry or a borrow through every length to 30; and on operands padded with high zeros, which must
answer as their value does. No answer leaves the oracle: a sum of two 30-digit numbers has at most 31 digits.

**Why.** The theory has no width: a column at the thirtieth place is computed by the rules that compute the third,
so a length is not a boundary of it, and the exhaustive range and the sampled lengths cover every case the rules
distinguish. An oracle that is itself a column adder would test the theory against a second copy of its own idea.

**Held by.** `ein-infer/tests/arith_theory.rs`: `every_small_request_is_right`,
`requests_of_every_length_are_right`, `a_carry_or_a_borrow_runs_the_whole_length`,
`high_zeros_are_read_by_value`; its constants `SMALL` and `MAX_DIGITS`.
