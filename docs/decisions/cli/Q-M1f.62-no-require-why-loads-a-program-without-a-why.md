# Q-M1f.62 — `--no-require-why` loads a program without a `:why`

A program whose relation, rule or macro carries no `:why` is refused at load, one line per form, unless the
command is given `--no-require-why`; every command that loads a program takes it. In the library,
`LoadOptions::default()` requires a `:why`, and `require_why: false` is the option. It decides whether a program
loads, never what a loaded one answers.

**Why.** Every boolean option of the CLI is a flag that sets something, and a negative one is spelt `--no-…`. With
the requirement on by default, a positive flag would be inert, and an option that changes nothing is not offered.

**Held by.** `ein-cli/tests/config_reference.rs`: `require_why_decides_loading_only`, the option on every command
that loads and on no other, a program missing three `:why`s refused with a line each and run with the option, two
corpus programs byte for byte the same with it and without; `ein-ir/tests/load_semantics.rs`:
`the_default_requires_a_why_and_false_loads_without`. The row: [06-config.md](../../ein/user/04-reference/06-config.md).
