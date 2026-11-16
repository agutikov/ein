# Q-M1a.13 — The CLI's behaviour is fixed, and its help's layout is clap's

`ein` parses its arguments with `clap`, and the layout of `--help` and the wording of a usage error are `clap`'s.
Everything a script or a habit can depend on is fixed: the subcommands and the `render` and `show` views, the
delegated dispatch, every option's long name, short key, metavar, arity, default, choices, mutually-exclusive group
and help string, the accept or reject verdict on every invocation, the exit code, and which stream each byte goes
to. Free: wrapping, indentation, headings, the order within a section, and the wording of a usage diagnosis.

**Why.** No argument parser reproduces another's layout byte for byte, and a usage error carries the usage block,
so a fixed error text needs a whole formatter of its own. A reader depends on the options and their meaning, not on
where a line wraps, and a check of the structure catches a lost or renamed option on its own line.

**Held by.** `ein-cli/tests/help_surface.rs`: `the_argument_surface_is_stable`, every parser rendered as
`{command → {option → short, metavar, arity, default, choices, group, help}}` against
`ein-cli/tests/golden/help_shape.txt`; `the_extractor_finds_the_whole_surface`, the options counted from the parsers;
`a_renamed_short_key_fails`. The surface: [cli.md](../../ein/user/reference/cli.md#5-what-is-fixed-about-the-command-line) § 5.
