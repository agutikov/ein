# Q-M1f.155 — Equal operands of a subtraction are answered by both walks, with one fact

A subtraction runs two walks, X − Y and Y − X ([Q-M1f.364](Q-M1f.364-a-subtraction-answers-the-difference.md)).
Where X = Y neither borrows, so both hand over zero, and `sub-answer` and `sub-answer-short` derive the one fact
`(diff-of X Y z z)`: one answer, with two derivations.

**Why.** The answer is the same fact either way, and a rule that kept one walk quiet on zero would read less
plainly and change no answer.

**Held by.** `tests/stdlib/math/decimal-arith/09_sub_borrow_and_zeros.ein`, `(diff-of z z z z)` among its claims;
`ein-infer/tests/arith_theory.rs`: `every_small_request_is_right`.
