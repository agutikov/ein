# Q-M1f.172 — `std.math.num9` is held exhaustively at width 2, on a twin that does not ship

`std.math.num9` is held to the integers at two widths. **Exhaustively at width 2**: the same theory generated at two
digits, the fixture `num2_theory.ein`, is asked every pair of operands from 00 to 99 under all five operations, a
division's divisor never zero. **At the shipped width, on a sample**: operands of every pair of lengths drawn from a
fixed seed, equal operands and operands one apart, and the corners where a sum or a product is the last that fits
or the first that does not. One width ships: the width-2 theory is a fixture of the test and no module of the
library, so no program of the conformance suite is asked of it.

**Why.** Nine digits cannot be exhausted, and two can. The theory is the same rules at any width, a premise a column,
with the same places a product overflows and the same step rules of a division, so at two every row of every table
it reads is used and every way a carry, a borrow or an overflow meets another is met. A second shipped module would
be a second library to test and to document, for the sake of a check.

**Held by.** `ein-infer/tests/num9_theory.rs`: `width_two_is_right_on_every_pair`, `width_nine_is_right_on_a_sample`;
the fixture `ein-infer/tests/golden/num2_theory.ein`, generated as the module is, at a width of 2. The module:
[Q-M1f.320](../stdlib/math/num9/Q-M1f.320-std-math-num9-is-a-second-representation.md) and
[nine-digit numbers](../../ein/user/04-reference/04-stdlib/16-nine-digit-numbers.md).
