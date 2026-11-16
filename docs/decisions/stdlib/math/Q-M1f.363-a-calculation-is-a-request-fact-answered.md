# Q-M1f.363 — A calculation is a request fact, answered by a result fact that repeats its operands

A program asks with a fact, `(add X Y)`, and the theory answers with a fact that repeats the operands as written,
`(sum-of X Y S)`. A goal reads the answer with a variable in the result's place, `(sum-of X Y ?s)`, and a rule of
the program may match it like any fact, the next request of a chain among them. Nothing is computed that was not
asked for: no number is enumerated, and a query of a result sees only the requests the program made. Every module
of the calculator answers so: `int-add` by `int-sum-of`, `bin-add` by `bin-sum-of`, `add9` by `sum9-of`.

**Why.** A result that repeats its operands tells the requests of one program apart, and lets a rule match an
answer to its own question.

**Held by.** `ein-infer/tests/arith_theory.rs`: `every_small_request_is_right`,
`requests_of_every_length_are_right`; `tests/stdlib/math/decimal-arith/07_only_what_is_asked.ein` and
`19_div_named_and_chained.ein`.
