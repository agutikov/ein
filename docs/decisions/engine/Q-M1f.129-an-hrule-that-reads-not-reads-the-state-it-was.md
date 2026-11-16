# Q-M1f.129 — An hrule that reads `not` reads the state it was asked about

Where any hrule's `:match` reads a negation, a stored `(not …)`, an `(absent (not …))` or the stdlib's `unknown`,
every hrule is matched before the first candidate is handed back, so no write a caller makes between two candidates
is visible to the enumeration. Where none reads one, each hrule is matched as it is reached, so a caller that stops
early matches no hrule it does not reach; the two orders give the same candidates. A debug build and a release
build answer such a program alike, and as `ein oracle` does.

**Why.** A candidate is a hypothesis about the state the generator was asked about; an enumeration that could see a
negation written mid-enumeration would propose about a state nobody asked about.

**Held by.** `ein_infer::hrule::Hrules::candidates`; `ein-cli/tests/hrule_reads_not.rs`:
`an_hrule_that_reads_not_is_answered_as_the_oracle_answers`, `every_hrule_reads_the_state_it_was_asked_about`.
