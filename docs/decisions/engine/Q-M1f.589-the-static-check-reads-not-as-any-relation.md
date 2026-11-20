# Q-M1f.589 — The static check reads `not` as any relation

The static check that flags a program that may not be monotone reads relations as names, `not` among them, with no
negated side to a relation: a read of `(not (p ?x))` under an `absent` is a read of `not`, and a conclusion
`(not (p a))` writes `not`. What makes a rule a refutation is read off the rules with no name: it states
`:ein inconsistent`, or concludes a fact a rule that states it matches, `(false)` in every program, or concludes the
side of a pair a rule refutes that holds the other, a stored negation where the program imports `std.contradiction`
([Q-M1f.557](../stdlib/Q-M1f.557-the-contradiction-is-a-theory-a-program-imports.md)). A site's label names a read of
`not` as `not`.

**Why.** `not` is the standard library's name and no word of the kernel's
([Q-M1f.436](Q-M1f.436-the-kernel-reads-no-theorys-name.md)); a check that kept a negated side for each relation
would read the theory the program may not take. Read at relation grain, the check flags a program where a rule's
`absent` reads what a commitment can change, as it does of any relation.

**Held by.** `ein_infer::monotone`, its `Refutes` and `leads_to_an_end`, with `ein_infer::contradiction::Pairs`;
`ein-infer/tests/detector.rs`' `the_static_half_is_empty_on_a_monotone_program_and_named_on_a_flagged_one` and
`each_site_names_what_it_concludes_and_puts_at_stake`; `ein-cli/tests/non_monotone_warning.rs`, the flag on every
corpus program. The page is [monotonicity](../../ein/reasoning/02-kb-evolution/08-monotonicity.md).
