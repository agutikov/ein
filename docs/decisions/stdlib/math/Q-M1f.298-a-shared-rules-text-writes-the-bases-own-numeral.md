# Q-M1f.298 — A shared rule's text writes the base's own numeral

`operand-digit` and `let-to-digit`, two of `std.math.positional`'s rules, take the base's constructor as a
parameter and write it in their `:why`: a digit where a number goes reads *the number is (n z 7)* under decimal,
and *(b2 z 1)* under binary.

**Why.** A shared rule's text is read under each base that activates it, and a numeral of the other base would be
wrong in a trace.

**Held by.** `stdlib/math/positional.ein`; the programs that activate the two rules under each base,
`tests/stdlib/math/decimal-arith/27_malformed_digit_as_number.ein`, `42_let_to_a_digit.ein`,
`tests/stdlib/math/binary-arith/10_a_bit_as_an_operand.ein` and `19_let_to_a_bit.ein`.
