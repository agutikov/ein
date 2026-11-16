# Q-M1e.24 — `-n` takes a count of one or more

`-n`/`--solutions N` asks for N states of the chosen part of R, and N is 1 or more. `-n 0` and every negative are a
usage error, exit 2, refused before the run with a line that says what the option takes and that leaving `-n` out
describes all of R. A value that is not an integer keeps the parser's own message.

**Why.** A stop-after of zero has no reading a user could mean: *record nothing* is no question about R, and *no
limit* is leaving `-n` out. Accepting it would make `-n 0` another spelling of some other count.

**Held by.** `ein-cli/tests/cli_semantics.rs`: `solutions_takes_a_count_of_one_or_more_and_nothing_else`. The page:
[cli.md](../../ein/user/reference/cli.md#5-what-is-fixed-about-the-command-line) § 5.
