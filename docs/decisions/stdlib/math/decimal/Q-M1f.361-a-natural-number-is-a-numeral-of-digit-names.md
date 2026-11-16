# Q-M1f.361 — A natural number is a numeral of digit names, in reading order

A natural number is `z`, zero, or `(n PREFIX DIGIT)`, ten times the prefix plus the digit, nested in the order it
is written: 123078 is `(n (n (n (n (n (n z 1) 2) 3) 0) 7) 8)`. A digit is one of the ten names `0` to `9`. A
numeral has any length, and the arithmetic has no width and no overflow. A program may write high zeros,
`(n (n z 0) 7)` is 7, and an operand is read by value; every answer is in normal form, with no high zero, so zero
is `z`.

**Why.** Ein has no numbers ([Q-M1e2.24](../../../language/Q-M1e2.24-ein-has-no-numbers-and-no-ranges.md)): a
number is a term of names, and the reading order lets a program write it as it is read, at any length.

**Held by.** `stdlib/math/decimal.ein`; `ein-infer/tests/arith_theory.rs`: `requests_of_every_length_are_right`,
`high_zeros_are_read_by_value`; `tests/stdlib/math/decimal-arith/03_lengths.ein` and `05_high_zeros.ein`.
