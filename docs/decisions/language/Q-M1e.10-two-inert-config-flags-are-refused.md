# Q-M1e.10 — Two inert config flags are refused

`(config :print-alive …)` and `(config :candidate-order-seed …)` are refused at the parse, on every command, as
every keyword a `(config …)` does not take is, with a note that names why the flag went:

```text
<file>:1:9: unexpected `:print-alive`: a keyword this form does not take
  note: config flag :print-alive is retired: it was inert: no code path read it (Q-M1e.10)
```

`SolverConfig` keeps both fields at their defaults, `false` and `-1`, read by nothing, so that the `.einb`
container's layout of the configuration does not move.

**Why.** No code path ever read either flag: `print-alive`'s read-out exists under other options, and
`candidate-order-seed`'s per-branch order was never built. A flag a program can set and nothing reads tells its
reader that it changes something; refused with its reason, it tells the reader that it does not, and what to
delete.

**Held by.** `ein_core::config::RETIRED`, the table of retired flags and their reasons;
`ein-cli/tests/config_reference.rs`: `the_retired_flags_are_refused_with_why`; `ein-ir/tests/load_semantics.rs`:
`the_config_coercion_table_is_stable`, against `ein-ir/tests/golden/config_coercion.txt`;
`ein-ir/tests/grammar_pages.rs`: `every_row_of_the_grammars_table_is_refused_at_the_parse`. The page:
[`06-config.md`](../../ein/user/04-reference/06-config.md), the table of retired flags.
