# Q-M1f.222 — Under `ein test`, a state found refutes a claim that does not list its class

A claim is held only by an answer that lists all of what it is about. Under `ein test`, a run that did not exhaust
what a claim is about still refutes it where a state it found falls in a class the claim does not list, since every
state a run lists is a certified member of the part it was asked; and it refutes `(solve :expect (false))` where
root is alive. `ein solve` under a mode decides no such claim: it prints *not checked* with a `note:`, and exits 0.

**Why.** A found state is evidence whatever the run left unvisited, and a runner whose job is to decide claims
should use it. `ein solve` answers the question it was asked; a claim beside it is the runner's to decide.

**Held by.** `ein_infer::explore::Options::refute`, set by `ein test` and by nothing else;
`ein-cli/tests/test_cli.rs`: `a_state_found_refutes_a_claim_that_does_not_list_its_class` and
`the_regions_false_is_refuted_at_an_alive_root`.
