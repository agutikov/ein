# Q-M1f.639 — `ein saturate --events` narrates root's saturation and nothing after it

A stream of `ein saturate --events` is the first three parts of a run's stream, `run`, `load` and root's saturation,
and nothing after them. Root's `open` facts are written at its fixpoint as every command writes them
([Q-M1f.566](../hypotheses/Q-M1f.566-what-a-state-owes-is-open-r-a-fact-and-open-f.md)), and where a rule reads
one, the saturation resumes over them. What it fires then is root's: counted in the read-out's firings and its
per-rule list, and listed by `--dump`, but narrated by no line; the pass and the resumed saturation write no event.
`ein solve`'s stream narrates the resumed saturation, after `H` is built and root's obligations are read.

**Why.** In a region run's stream the resumed saturation follows the lines that build $H$ and read what root owes;
`ein saturate` writes none of them, and its resumed firings narrated after root's `quiesce` would read as root's
first saturation going on. Its read-out already counts and lists every firing root makes.

**Held by.** `ein-cli/src/saturate.rs`'s `root_open`, which writes root's `open` facts with the events off;
`ein-cli/tests/cli_semantics.rs`: `saturate_counts_the_firings_after_roots_open_facts`;
[`05-events.md`](../../ein/ein.rs/05-events.md#the-orders-a-stream-can-take), the orders a stream can take.
