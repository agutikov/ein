# Q-M1f.301 — The greatest common divisor puts each number into normal form first

Before Euclid's chain, the call, `gcd-run` behind `gcd-call`, puts each number into normal form by an addition walk
to zero, `(add-run TAG X z 0 z)`: zero is then `z`, every divisor `(n P D)` is not zero, and the answer is in normal form.
The chain divides the larger by the smaller where they are written so; where X is the smaller, its first step's
quotient is zero, and the step turns the two round.

**Why.** A zero written with digits would otherwise be a divisor of the shape `(n P D)`, and an answer that is an
operand's value would come back as the operand was written.

**Held by.** `ein-infer/tests/gcd_theory.rs`: `high_zeros_are_read_by_value`, `zero_in_each_place`;
`tests/stdlib/math/gcd/03_high_zeros.ein`.
