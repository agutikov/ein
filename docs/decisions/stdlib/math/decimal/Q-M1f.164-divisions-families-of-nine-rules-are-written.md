# Q-M1f.164 — Division's families of nine rules are written by hand, and a test holds them alike

Division's two families, the multiples' chain `div-multiple-2` to `div-multiple-9` and the trials `div-try-1` to
`div-try-9`, are written out in `std.math.decimal-arith`, and a test holds that the rules of a family differ by
their digit alone.

**Why.** The module stays one hand-written file a reader can follow, and the test holds what a generator would
guarantee. One rule for a family would need a premise that joins on a number
([Q-M1f.362](../Q-M1f.362-no-rule-of-the-calculator-joins-two-facts-on.md)).

**Held by.** `ein-infer/tests/arith_theory.rs`: `the_rules_of_a_family_differ_by_their_digit_alone`.
