# Q-M1e2.34 — Stdout's listings share 100 lines, and a dump holds them whole

What `ein solve` lists on stdout, R's states, a line's members, each query's classes and the claim to paste, shares
100 lines, in that order, each listing in whole items while the lines last and at least one each. The forms after
the read-out (`--models key|grid`, `-p`, `-P`, `-f`, `--diff-models`, `-H`) are listings too and share what the
read-out leaves. A cut listing says where the rest is, and a cut region is not wrapped in `(region …)`, which would
claim it is all of R. The summary rows are never cut. Nothing is written to disk unasked: `--dump-answer DIR`
writes the read-out uncut as `DIR/answer.txt` and each form uncut beside it.

**Why.** An exhausted R can be tens of thousands of lines, and a screen that holds them is no longer read; the
answer whole is one explicit flag away, and a file appears only where its reader asked for one.

**Held by.** `ein_render::read_out::LIST_LINES`; `ein-cli/tests/forms_cut.rs`:
`on_pets_each_form_prints_what_fits_and_the_dump_holds_it_uncut`, `where_nothing_is_cut_stdout_is_the_file`;
`docs/ein/user/04-reference/08-read-out.md`, the rows and what each counts.
