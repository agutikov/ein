# Q-M1f.162 — A sampled product's operands have at most 19 digits

The harness samples a product's operands to 19 digits, where every other operation's go to 30
([Q-M1f.154](Q-M1f.154-the-calculators-harness-is-exhaustive-to-120.md)): a product of two 19-digit numbers has at
most 38, which the oracle, `u128`, holds.

**Why.** For the same reason a length is not a boundary of the theory: the rules that multiply a 19-digit number are
the ones that multiply a 3-digit one. Keeping the oracle Rust's own integers keeps it independent of the theory.

**Held by.** `ein-infer/tests/arith_theory.rs`: the multiplication row of `OPS`, whose longest sampled operand is 19
digits, through `requests_of_every_length_are_right`.
