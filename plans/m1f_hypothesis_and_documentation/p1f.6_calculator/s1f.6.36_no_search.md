# S1f.6.36 — Judging root without a search: `(solve … :no-search)` and `--no-search`

**Phase:** [P1f.6](README.md)
**Status:** planned 2026-11-14, on the user's words of 2026-11-13 and 2026-11-14. Not started. **An engine stage
and a language addition.**
**Depends on:** [S1f.6.34](s1f.6.34_hypothesis_mechanisms_in_conflict.md), whose conflict check it joins and
whose corpus corrections (Q-M1f.246) it revisits; [S1f.6.35](s1f.6.35_imports_by_full_name.md), which it runs
after, since both move corpus files. Independent of the stdlib layout; it runs before
[S1f.6.31](s1f.6.31_the_stdlib_layout.md).

## The ruling

S1f.6.34 refused a program whose `:no-hypothesis` lists a relation its own obligations owe (*"filtered obligations
are nonsense by definition"*, the user, 2026-11-13). Twenty-four corpus programs did that on purpose, to switch the
search off and let the obligations judge root alone; S1f.6.34 corrected them, three with an hrule `as-stated` that
re-proposes only what root holds. Asked whether Ein should have a sanctioned way instead, the user, 2026-11-13:

> maybe some explicit flag in (solve ) ? instead of abusing bugs; or cmdline flag that disable search; or both

and, on the coordinator's objection that a flag may not change R (the ruling of 2026-10-15), 2026-11-14:

> No conflicts! flag that disables search doesn't change R, it disables discovering of R. R is not an output, it
> is complete result when we do full R search.

So **both**: a keyword the program writes, and a flag a command takes.

- **`(solve … :no-search)`** is the program's statement that its lattice is root alone: no candidate from any
  rung, whatever hrules, obligations or lists it also declares, so R is {∅} where root is alive and discharged
  and empty otherwise. It is part of the program, as `:hypothesis-relations` is, and every command reads it.
- **`--no-search`** is a command's choice not to discover R: it saturates root and reports root (its facts, alive
  or dead, discharged or what it owes), and says that R was not searched. It changes no R: a run without it
  discovers the R it would have declined to.

## Tasks

1. **T1f.6.36.1 — the design on real programs, shown before anything is built.** The 24 programs S1f.6.34
   corrected, each written with `:no-search` in place of its correction where its intent was *judge root alone*
   (the record says which were and which were not), with what every command answers; E-47's witness and the
   conflict-check fixtures with `:no-search` beside each mechanism. And for `--no-search`, what each command
   prints on a sample of the corpus: `ein solve` (its read-out rows, the `R` row saying *not searched*), `ein
   test` (which claims root decides, a `(solve :expect (false))` on a dead root among them, and which stay NOT
   CHECKED, saying why), `ein oracle` (refused, or the root alone: the stage proposes), the `--json-summary` and
   `--dump-answer` artefacts. Recorded as the record's first section.
2. **T1f.6.36.2 — the keyword.** The grammar's `solve` gains `:no-search` (a flag keyword, no argument), a row of
   the grammar page's table; the loader reads it; the candidate generator proposes nothing under it. S1f.6.34's
   check: `:no-search` beside obligations is silent (the intended use); beside an hrule, `:hypothesis-relations`
   or `:no-hypothesis`, the other declaration filters or proposes nothing, a `note:` as § 4.27's dead lists have.
3. **T1f.6.36.3 — the flag.** `--no-search` on `ein solve` and `ein test` (and on any other command where T1
   shows it means something), in `configuration.md`'s census with *does it change the answer: no — it declines to
   discover it*; `lever_invariance.rs` holds that every run with the flag reports root as the run without it
   reaches it, and never states an R.
4. **T1f.6.36.4 — the corpus.** The programs T1 names rewritten to `:no-search`, S1f.6.34's workarounds removed
   (the `as-stated` hrules among them), each claim held; the mutants S1f.6.34's corrections killed stay killed
   (`utils/stdlib_mutants.py --check`). Goldens counted before blessing.
5. **T1f.6.36.5 — the pages and the records:** the grammar, `defined_behaviour.md`, `configuration.md`,
   `read_out.md` (the row that says R was not searched), `tests/README.md`'s idioms (*judge root alone*: write
   `:no-search`), Q-M1f.246 restated, the phase README's row, the stage's record.

## Acceptance

- A program with obligations and `(solve :no-search)` loads, proposes nothing, and is judged at root; the 24
  fixtures say what they meant with it, and none carries a workaround.
- `--no-search` changes no R: on every corpus program, what it reports of root is what the full run reaches at
  root, and it states no R.
- Every page that lists the `solve` keywords or the CLI's options lists the two.

## What it must not do

- Not make `--no-search` a lever under which a complete search reports another R: it declines to search, and
  says so.
- Not relax S1f.6.34's conflicts: `:no-hypothesis` over an owed relation stays refused; `:no-search` is the way to
  say *no search*.

## The standing rules

A wrong answer of the engine met on the way is a row in `corpus/wrong_answers.toml`, filed `open` before its
fix. Findings go to [`findings.md`](findings.md) under the next free id; questions to
[`open_questions.md`](../open_questions.md) under the next free id. Every answer exits 0. The gate runs once over
the batch this stage belongs to (the user's cadence of 2026-11-13).
