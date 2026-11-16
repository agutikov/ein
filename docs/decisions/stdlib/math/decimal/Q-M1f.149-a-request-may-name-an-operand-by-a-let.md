# Q-M1f.149 — A request may name an operand by a `let`

`(let NAME NUMBER)` names a number, and a request may name its operands: with `(let price …)` and `(let qty …)`,
`(add price qty)` is answered by `(sum-of price qty S)`, the names repeated as written; the same sum written with
the numbers is another request, with its own answer. Every operand is looked up, `(operand TAG X)` answered by
`(operand-is TAG N)`: a number stands for itself, and a name for the number its `let` states. A name stands for a
number, not for another name, and for one number; a `let` that breaks either is flagged
([Q-M1f.207](Q-M1f.207-a-let-that-cannot-be-read-is-flagged-whether.md),
[Q-M1f.281](Q-M1f.281-a-let-that-names-a-digit-is-flagged.md)), and so is a name no `let` states
([Q-M1f.317](Q-M1f.317-a-name-no-let-states-is-flagged-as-undeclared.md)).

**Why.** A goal can then ask by the program's own names, `(sum-of price qty ?s)`. The look-up joins on a name,
which the fact index keys, so it keeps the rule of the calculator
([Q-M1f.362](../Q-M1f.362-no-rule-of-the-calculator-joins-two-facts-on.md)).

**Held by.** `ein-infer/tests/arith_theory.rs`: `named_operands_work_for_every_request`;
`tests/stdlib/math/decimal-arith/06_named.ein`, `10_sub_named.ein` and `19_div_named_and_chained.ein`.
