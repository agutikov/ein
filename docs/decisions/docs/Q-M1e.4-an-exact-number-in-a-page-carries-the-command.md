# Q-M1e.4 — An exact number in a page carries the command or the test that checks it

An exact number may stand in a page, a comment or a manifest note only beside what makes it checkable: the command
that re-takes it, or the test that pins it. Which one follows from what the number is:

| the number is | it carries |
|---|---|
| a **size**: how many files, rules, entries, options or cells a place holds | the **command** that counts it, one of Ein's own: an `ein` command, a `cargo test`, or a shell count over the tree |
| a **count a test asserts**: every rule activated, every flag with a row | the **test's name** |

A number that can carry neither is not written, and the page names what prints it instead. A measurement, a time or
a cost taken on some machine, is not stated in a page at all
([Q-M1f.349](Q-M1f.349-a-page-of-the-project-states-no-measurement.md)).

**Why.** A count that nothing re-takes drifts as the tree around it changes, silently, and reads as current; every
count a test pins stays exact. A number beside its command is checked by a reader in one step, and one beside its
test is checked on every run of the tests. The warrant is right only because it is run: the number is trusted for
the command or the test, not for the sentence.

**Held by.** `docs/ein/user/reference/config.md`, whose option counts `ein-cli/tests/config_reference.rs` takes from the
binary (`the_cli_counts_come_from_the_golden`); `corpus/README.md`, whose slow set is named by `ein-corpus`'s
`the_slow_set_is_exactly_these_fifteen`; `docs/ein/ein.rs/api.md`, whose worked example `ein-cli/tests/embedding.rs`
diffs against itself (`the_page_quotes_this_file`).
