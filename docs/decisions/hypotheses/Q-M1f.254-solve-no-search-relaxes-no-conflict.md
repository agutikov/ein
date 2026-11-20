# Q-M1f.254 — `(solve :no-search)` relaxes no conflict

Under `(solve :no-search)` nothing is proposed and H is empty, yet root holds the `open` facts its rules and its
obligations write, as on any program, and every conflict between the hypothesis mechanisms stands, with the same
line: the lists and a closure are judged against those `open` facts, whatever the search does. `:no-search` beside a
rule that concludes a hypothesis is one `note:`, *its :no-search states the lattice is root alone, so rule 'guess'
proposes no candidate*, and beside each list one, *its :hypothesis-relations filters no candidate*, which replace
the notes those declarations print otherwise; beside an obligation or a closure it is silent, judging root alone
being what an obligation is for there.

**Why.** One reading of a declaration for every pair and every question: a program that contradicts itself does so
whether or not it asks for a search.

**Held by.** `ein-cli/tests/mechanism_conflicts.rs`: `no_search_relaxes_no_conflict`,
`no_search_is_silent_beside_an_obligation_and_a_note_beside_what_it_makes_dead`;
[`07-diagnostics.md`](../../ein/user/04-reference/07-diagnostics.md#226-a-programs-hypothesis-mechanisms-do-not-contradict-one-another) § 2.26.
