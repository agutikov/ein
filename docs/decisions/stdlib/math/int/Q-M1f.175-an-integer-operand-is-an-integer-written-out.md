# Q-M1f.175 — An integer operand is an integer written out, or a name

An operand of `std.math.int` is an integer written out, `(int P N)` with P and N natural numbers, simplified or
not, or a name that a `(let NAME (int P N))` of `std.math.decimal-arith` states. A natural number is no integer
operand: `(int-add (n z 5) X)` is not answered, and 5 is written `(int (n z 5) z)`.

**Why.** Each kind of number has one shape, and the program writes the integer it means
([Q-M1f.277](../Q-M1f.277-an-operand-of-another-kind-of-number-is-answered.md)).

**Held by.** `ein-infer/tests/int_theory.rs`: `named_operands_work_for_every_request`,
`a_natural_number_is_not_an_integer_operand`; `tests/stdlib/math/int/08_named_and_chained.ein`.
