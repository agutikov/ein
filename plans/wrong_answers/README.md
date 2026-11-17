# The wrong answers

Every known wrong answer of the engine that is still open is a row of
[`register.toml`](register.toml): a well-formed program, a run the CLI accepts,
the answer it gives today and the answer the definitions give. A wrong answer
is temporary, so it is tracked here, on the process side; when it is fixed it
becomes a regression, a row of the project's
[`corpus/regressions.toml`](../../corpus/regressions.toml), and its witness a
corpus program.

Its counts are `grep -c '^\[\[case\]\]' plans/wrong_answers/register.toml`
(open) and `grep -c '^\[\[case\]\]' corpus/regressions.toml` (fixed).

## The rules

- **A wrong answer is filed as a row of `register.toml`**, `status = "open"`,
  owned by the followup or the stage that fixes it, with its witness, in a
  commit of its own. A new witness lives in [`witnesses/`](witnesses/); a
  project program that shows the wrong answer (an example another test or page
  reads too) is the witness where it is, and nothing in the project names the
  row: a project test or page that has to say the program answers wrong says
  so in words, a known gap.
- **The format is the project's**: `ein_corpus::regressions`, whose field
  reference is `corpus/regressions.toml`'s header. A row's `[case.now]` is what
  the engine answers today, and it does **not** answer its `[case.right]`; the
  register's own keys beside it (`case`, `class`, `owner`, `question`,
  `status`, `open_because`, `look_at`, `note`, `[case.wrong]`,
  `[case.control]`, `[[case.twin]]`, `[case.review]`, and `reading`, `derived`
  and `[[case.right.alternative]]` in `[case.right]`) are this file's header's.
- **The gate holds every row**: `python3 utils/wrong_answers.py`, a step of
  `run_tests.sh` and of the per-commit workflow, checks the register's own keys
  and then runs the project's runner on the file, `cargo run -p ein-cli
  --example held_answers -- --ein ein.rs/target/gate/ein --open
  plans/wrong_answers/register.toml`: each row's `[case.now]` through the
  binary, each `[case.region]` through the library, and no row answered right
  by the method. A run that changes answer fails the gate, so a fix nobody
  intended, or a new wrong answer, is seen in the commit that makes it.
- **The fix moves the row**, in the fixing commit: out of `register.toml` and
  into `corpus/regressions.toml`, stripped of the register's keys (a name, the
  title, the settings, the decisions it rests on, the witness, `[case.right]`,
  `[case.region]` and `[case.now]`, which now answers the right one); its
  witness into `examples/ein-bugs/wrong-answers/` with a corpus entry; and any
  `:expect` that stated the wrong answer flips with it. A witness's comment
  names no row and no plan.
- **A wrong answer the agent revisits first**: the user reads by hand only what
  stays ambiguous.

[`fixed_rows.tsv`](fixed_rows.tsv) maps the register's ids of the rows fixed
before the split to their regressions' names, for a project comment that still
names a row by its old id.

The review bundle, `python3 utils/review_bundle.py`, builds one page per row of
this register.
