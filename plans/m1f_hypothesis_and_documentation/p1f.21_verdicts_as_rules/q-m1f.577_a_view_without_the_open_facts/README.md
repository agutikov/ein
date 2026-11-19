# 55 · Q-M1f.577: a view of a state without its `open` facts

[← part 2](../review/part-2/README.md) · [← the phase](../README.md) ·
[the ledger](../../open_questions.md#q-m1f577--a-view-of-a-state-without-its-open-facts)

**Ruled 2026-11-20, and out of the review bundle**, where it was item 55 of part 2. Your answer, in the part's
answers' file:

> (a)

It is filed under the question in the ledger and in [the phase's README](../README.md#the-answers-to-items-52-to-55).
It leaves nothing open. The page is kept, with its programs, as the survey the second part's stages cite; where it says
*the default*, read it as it stood before the answer. Its outputs and its tools (`runs/` and `tools/` below) are
on this machine under `build/review-archive/p1f21-part-2/55-Q-M1f.577/`, as they were run.

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

## 3. The ruling

Ruled: the answer at the head of this page.
