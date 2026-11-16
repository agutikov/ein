# Q-M1f.254 — `(solve :no-search)` relaxes no conflict

Under `(solve :no-search)` no rung runs, yet every conflict between the hypothesis mechanisms stands, with the same
line: an hrule is read as active and an obligation as its plans, whatever the search does. `:no-search` beside an
hrule is one `note:`, *the hrule proposes no candidate*, and beside each list one, *the list filters no candidate*,
which replace the notes those declarations print otherwise; beside an obligation or a closure it is silent, judging
root alone being what an obligation is for there.

**Why.** One reading of a declaration for every pair and every question: a program that contradicts itself does so
whether or not it asks for a search.

**Held by.** `ein-cli/tests/mechanism_conflicts.rs`: `no_search_relaxes_no_conflict`,
`no_search_is_silent_beside_an_obligation_and_a_note_beside_what_it_makes_dead`;
[`defined_behaviour.md`](../../kernel/defined_behaviour.md) § 4.27.
