# Q-M1f.169 — A division's multiples are computed into the state by one rule

`div9-start` computes the divisor's multiples by 2 to 9 in one firing and puts them into the state; ten step rules,
one a digit the quotient can have, each read their own multiple from there, and exactly one of them fires. A
product's step computes its own row. No rule joins two facts on a number.

**Why.** With the multiples in the state a division is a fixed count of firings, and a batch costs in proportion to
its count ([Q-M1f.362](../Q-M1f.362-no-rule-of-the-calculator-joins-two-facts-on.md)); a rule of many premises is
what a generated module is for.

**Held by.** `ein-infer/tests/num9_theory.rs`: `no_rule_joins_two_facts_on_a_number`,
`the_circuit_of_each_operation`; `tests/stdlib/math/num9/08_div_every_digit.ein`.
