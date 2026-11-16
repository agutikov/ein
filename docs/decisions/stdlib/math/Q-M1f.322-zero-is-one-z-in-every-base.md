# Q-M1f.322 — Zero is one `z` in every base

Zero is `z`, `std.math.arith`'s, in decimal and in binary alike: every numeral of either base ends in it, and a
bare `z` is of no base. A program that writes both bases holds one zero, and nothing converts it.

**Why.** Zero is the one number whose numeral has no digit, so there is nothing to tell its bases apart by.

**Held by.** `ein-infer/tests/binary_theory.rs`: `both_bases_in_one_program_agree`;
`tests/stdlib/math/binary-arith/21_both_bases.ein`.
