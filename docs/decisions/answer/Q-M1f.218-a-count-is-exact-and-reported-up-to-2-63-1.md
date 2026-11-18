# Q-M1f.218 — A count is exact, and reported up to 2^63 − 1

The engine counts commitments in an exact integer of any size, so no count fails for being large. A surface
reports a count no greater than 2^63 − 1, 9 223 372 036 854 775 807, and a reported count equal to it reads *this
many or more*: the read-out and `--count` print *or more* and why, `ein test -v` the same words, and
`--json-summary`, `answer.json`, `index.json` and `ein test --json-report` write the number, which their pages say
is a floor at that value.

**Why.** One bound every surface can carry, JSON and TOML among them, with no second form for a larger number;
nothing reads a count that large as a number, so *this many or more* says all a reader can use of it.

**Held by.** `ein_infer::count::Count`, exact, and `ein_infer::count::CAP`; `ein-cli/tests/region_modes.rs`:
`a_count_past_the_cap_is_an_answer_on_every_surface`.
