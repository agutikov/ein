# Q-M1f.196 — Each module of the word theory is a corpus entry of its own

Each module of the word theory is a `positive` corpus entry with runs of its own, as the stdlib's modules are:
`words.ein` and `readings.ein` are solved, saturated and drawn alone, each with R {∅}; `gaps.ein` and
`listed_is_all.ein` are saturated and drawn. A module states no claim: the problems that import it state theirs.

**Why.** Every `.ein` under `examples/` is a corpus program, and a module read alone is how a fault of the module is
found in the module and not only through a problem that imports it.

**Held by.** The four entries in `corpus/corpus.toml`; `ein-cli/tests/corpus_cli.rs`:
`every_cell_reproduces_its_exit_code`; `ein-cli/tests/answer_round_trip.rs`, which reloads every state a solve
dumps.
