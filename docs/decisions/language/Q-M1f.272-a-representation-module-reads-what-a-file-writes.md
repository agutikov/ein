# Q-M1f.272 — A representation module reads what a file writes through `written`

The loader states each fact nested in a fact a file writes as `(written T)`, a kernel relation, beside the facts it
loads ([Q-M1f.275](Q-M1f.275-written-states-every-fact-a-file-nests.md)). A representation module's rules read it,
`:match (written (n ?p ?a))`, so a module checks its own terms where they are written: a malformed numeral of
`std.math.decimal` is flagged at root, wherever the program writes it, with no request asked and no calculation
started.

**Why.** A malformed value is its representation's problem, found as soon as it is written. No rule can find every
place a term is nested, since a rule reads relations it names; the loader, which reads every fact a file writes,
can state each one, and a rule then reads it as any fact.

**Held by.** `ein-cli/tests/written_facts.rs`: `what_the_loader_states`; under `tests/stdlib/math/decimal/`,
`03_in_a_fact_of_the_programs_own.ein` and `04_in_a_rules_conclusion.ein`, each a numeral flagged at root where it
is written. A program does not state `written` itself: [Q-M1f.280](Q-M1f.280-a-program-does-not-write-written.md).
