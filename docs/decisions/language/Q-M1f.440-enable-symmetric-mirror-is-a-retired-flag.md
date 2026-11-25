# Q-M1f.440 — `enable-symmetric-mirror` is a retired flag, refused with why it went

`(config :enable-symmetric-mirror …)` is refused at the parse, on every command, as every keyword a `(config …)` does
not take is, with a note that says why the flag went:

```text
<file>:1:9: unexpected `:enable-symmetric-mirror`: a keyword this form does not take
  note: config flag :enable-symmetric-mirror is retired: it switched the kernel's native mirror, and the kernel mirrors no relation: a relation closed under swapping its two arguments is std.algebra's (symmetric R), a rule
```

`SolverConfig` keeps the field at its default, read by no engine code, so that the `.einb` container's layout of the
configuration, which writes and reads every field in order, does not move. A container whose program sets the flag is
refused where its `.ein` is.

**Why.** The flag switched a mirror the kernel no longer has: the kernel reads no theory's name and mirrors no
relation, and a relation closed under swapping its two arguments is `std.algebra`'s rule
([Q-M1f.436](../engine/Q-M1f.436-the-kernel-reads-no-theorys-name.md)). A program that sets it asks for something
no run does; refused with its reason, it tells its reader what took the flag's place and what to delete, where an
unknown keyword's refusal would say only that the flag is none.

**Held by.** `ein_core::config::RETIRED`, the table of retired flags and their reasons, and
`ein_core::config::FIELDS`, which keeps the field; `ein-cli/tests/config_reference.rs`:
`the_retired_flags_are_refused_with_why`; `ein-ir/tests/grammar_pages.rs`:
`every_row_of_the_grammars_table_is_refused_at_the_parse`, against `ein-ir/tests/golden/grammar_table.txt`. The page:
[`06-config.md`](../../ein/user/04-reference/06-config.md), the table of retired flags.
