# Q-M1f.256 — Under `--no-search`, a claim is decided as far as root decides it

`--no-search` builds no node past root, and a claim is decided as a description stopped after root would decide it
under `ein test`'s refute rule ([Q-M1f.222](Q-M1f.222-under-ein-test-a-state-found-refutes-a-claim.md)). On a dead
root every claim is decided, as the full run decides it, since that run too reads root alone. On an alive root a
`(solve :expect (false))` is refuted, and where ∅ is in R a query whose class at ∅ the claim does not list is
refuted; every other claim is NOT CHECKED, the line saying whether ∅ is in R.

**Why.** Root is a node of the lattice and its standing is known; what follows from it alone is decided, and
nothing that needs another node is guessed.

**Held by.** `ein-cli/tests/no_search.rs`: `test_decides_what_root_decides_and_a_claim_not_checked_exits_0`;
`ein-cli/tests/lever_invariance.rs`: `no_search_reports_root_as_the_full_run_reaches_it_and_states_no_r`.
