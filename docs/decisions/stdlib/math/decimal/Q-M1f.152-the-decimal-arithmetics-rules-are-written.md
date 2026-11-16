# Q-M1f.152 — The decimal arithmetic's rules are written by hand

`std.math.decimal-arith`'s rules, and the rules of `std.math.positional` it shares with binary, are written by
hand, with a comment beside each; only the tables they read are generated
([Q-M1f.366](../Q-M1f.366-the-calculators-tables-are-generated-facts-every.md)). A family of rules that differ by a
digit is written out, and a test holds its rules alike
([Q-M1f.164](Q-M1f.164-divisions-families-of-nine-rules-are-written.md)).

**Why.** The module is the file a reader follows to see how a sum is computed.

**Held by.** `ein-infer/tests/arith_theory.rs`, which holds the rules' answers to Rust's integers and their text to
the rule of the calculator.
