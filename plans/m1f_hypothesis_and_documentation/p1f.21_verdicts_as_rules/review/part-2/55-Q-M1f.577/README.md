# 55 · Q-M1f.577: a view of a state without its `open` facts

[← part 2](../README.md) · [← the phase](../../../README.md) · [your answers](../ANSWERS.md) ·
[the ledger](../../../../open_questions.md#q-m1f577--a-view-of-a-state-without-its-open-facts)

**From your answer to item 50**, 2026-11-20: *"(a) hypotheses are valid part of the state; but now maybe we need
separate flags/commands to show only positive facts of KB when we want to see only them."*

Nothing is built: every run is today's engine, the gate build of master at `edbe6c3b`
(`build/review-bin/ein-head-edbe6c3b`), on [`drinks-asked-and-denied.ein`](programs/drinks-asked-and-denied.ein),
whose states hold facts, stored negations and, under `question`, which stands for `open`, hypotheses.
`tools/run.sh` re-takes the runs into `runs/` (both ignored, so on this machine only).

## 1. The views `ein solve` has today

| flag | what it lists of each state | on the state of `{(chosen Tea)}` |
|---|---|---|
| `-p`, `--print-final-state` | its facts beyond the program as loaded | `(chosen Tea)` `(not (served Tea))` and the four `(question …)` |
| `-P`, `--print-final-positive` | the same, less the `(not …)` facts | `(chosen Tea)` and the four `(question …)` |
| `-f`, `--print-final-hfacts` | its facts of the candidates' relations | `(chosen Tea)` |

(`runs/drinks-asked-and-denied.solve-p.txt`, `.solve-P.txt`, `.solve-f.txt`.) `--dump-answer` writes each as a
file. So a view that leaves out the stored negations exists, and none leaves out the hypotheses.

## 2. The options

- **(a)** `-P` leaves out the `(open …)` facts as it leaves out the `(not …)`: *positive* is what a state holds as
  so. `-p` lists everything. No new flag. **Default.**
- **(b)** A flag of its own, beside `-p` and `-P`, that leaves out the `(open …)` facts and nothing else; and the
  same on `ein saturate --dump` and on the drawing of a knowledge base.
- **(c)** Nothing now: a state is listed whole, and a view is added when it is missed.

## 3. Your call

(a), (b) or (c), in [`ANSWERS.md`](../ANSWERS.md) under item 55. Blank keeps (a).
