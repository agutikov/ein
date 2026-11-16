# Q-M1e.17 — `-g 0` logs no node line, and a negative is refused

`-g`/`--progress-every N` logs every N-th node under `--verbose`. `0` logs no node line, the step lines alone, as
`ein saturate --progress-every 0` does. A negative is a usage error, exit 2, in both spellings (`-g=-1` and
`-g -1`), and the `error:` line says what the option takes. No count option of `ein` reads a negative as another
number: a count option that has no reading for one refuses it, and `ein saturate --max-steps`, whose `0` or less
stops before the first firing, names the value it was given in the line it prints.

**Why.** A value silently clamped to another is a run the reader did not ask for, and a diagnostic that reports a
number nobody typed cannot be checked against the command line. Zero has a reading for this option, so it keeps it;
a negative has none.

**Held by.** `ein-cli/tests/cli_semantics.rs`: `progress_every_is_zero_for_no_node_lines_and_refuses_a_negative`,
`the_step_limit_is_one_line_in_eins_spelling_and_exits_0`. The page:
[defined_behaviour.md](../../kernel/defined_behaviour.md) § 5. `-n`'s count:
[Q-M1e.24](Q-M1e.24-n-takes-a-count-of-one-or-more.md).
