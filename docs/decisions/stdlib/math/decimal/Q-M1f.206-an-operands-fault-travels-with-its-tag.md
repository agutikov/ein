# Q-M1f.206 — An operand's fault travels with its tag to the request

A fault is found at an operand's tag and named on the request. The look-up derives `(bad-operand TAG FAULT)`, FAULT
`(not-a-number THING)`, what stands where a number goes, or `(no-let NAME)`. Ten rules, alike but for the
operation, take a tag to its request, `(bad-request REQUEST FAULT)`, and `bad-request-malformed` and
`bad-request-undeclared` derive the two flags, `(malformed REQUEST)` and `(undeclared NAME REQUEST)`. A module over
the decimal arithmetic adds a rule for each tag of its own: `int-bad-first`, `rat-bad-first`, `gcd-bad-first` and
their kin.

**Why.** One relation carries both kinds of fault, so a tag needs one rule and not one a kind. A fault is a fact
and not a bare name, so no rule concludes a constant.

**Held by.** `ein-infer/tests/arith_theory.rs`: `the_rules_from_a_tag_to_its_request_are_alike`,
`a_malformed_request_is_flagged_once_and_never_answered`.
