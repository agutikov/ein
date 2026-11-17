# Q-M1a.23 — Ein has no binding for another language

Ein ships a binary and Rust crates, and no module for Python or any other language; `pip install` installs
nothing of it. A program in another language runs the binary and reads what it writes: `ein solve …
--json-summary FILE` for the answer and the counters as JSON, `--events FILE` for the narration, `--dump-answer
DIR` for every state as Ein text. A Rust program links the crates: `ein-ir` to load, `ein-infer` to solve,
`ein-render` to explain, `ein-einb` to cache.

**Why.** Nothing that drives Ein needs a binding. Process start-up is a small fraction of any run, so a batch of
programs is a batch of processes; a caller that wants the loader's refusals as data links `ein-ir` and has them
with no boundary; and a benchmark compares Ein with other systems as processes. A binding is added when a
consumer appears that the binary cannot serve: a loop over items so many or so small that start-up dominates,
state that must persist across many small queries in one process, or a Python program outside this project that
expects a module.

**Held by.** `ein-cli/tests/embedding.rs`: `the_worked_example_runs` and `the_page_quotes_this_file`, the
embedding example of [`docs/ein/ein.rs/04-api.md`](../../ein/ein.rs/04-api.md) run as a test. The pages:
[`docs/ein/ein.rs/04-api.md`](../../ein/ein.rs/04-api.md) § 7;
[`docs/ein/user/01-install.md`](../../ein/user/01-install.md) § There is no Python package.
