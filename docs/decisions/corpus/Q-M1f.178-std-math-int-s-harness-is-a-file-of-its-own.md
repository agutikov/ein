# Q-M1f.178 — `std.math.int`'s harness is a file of its own, exhaustive on one-digit parts

`std.math.int` is held by a test of its own, beside `std.math.decimal-arith`'s, with its own table of operations
against `i128`. Its exhausting set is every integer written with parts from 0 to 9, `(int P N)`, one hundred
spellings of −9 to 9, each against every other, under every operation and every combination of signs, simplified
and not; past it, a sample of operands of every length and sign.

**Why.** An integer operand is two natural numbers, not one `u128`, and each module's static checks read that
module's text alone, so the integers need a harness and an oracle of their own. Parts of one digit already give
every combination of signs and every spelling of a value, which is what the integer rules distinguish.

**Held by.** `ein-infer/tests/int_theory.rs`: `every_pair_of_small_parts_is_right`,
`requests_of_every_length_and_sign_are_right`, `an_operand_is_read_by_value`; its constant `PART`.
