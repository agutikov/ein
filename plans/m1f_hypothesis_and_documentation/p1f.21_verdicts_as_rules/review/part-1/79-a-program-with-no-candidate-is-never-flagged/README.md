# 79 · A program with no candidate is never flagged

[← part 1](../README.md) · [← the phase](../../../README.md) · [your answers](../ANSWERS.md) ·
[the finding](../../../findings.md#v-22--a-program-with-no-candidate-is-never-flagged)

A finding of the phase that is **yours to call**. It was on the findings page alone; it is here so that everything
that waits for you is in one place. Nothing waits on it: the row is in the register either way until you say.

## What it is

`W8/fifo-gap-root`, a witness of the wrong-answer register, states `:no-search` and has no candidate. The static
check looks for an `absent` a commitment can change, and with no commitment finds no site: the read-out says the
lattice is monotone and prints no warning. But the defect is at root: two rules that wait on an `absent` are
admitted in written order, and root holds a fact its own fixpoint's premises deny. So your ruling, *a
non-monotone program loads under a warning and nothing is claimed of it*, does not reach this program: the engine
claims its answer, and the answer is wrong. S1f.21.15 removed the row with the register's other rows of
non-monotone programs; the coordinator put it back.

## The question

- **(a)** It stays a row of the register: a wrong answer of a program the engine claims, the non-monotone milestone's to fix. **Default.**
- **(b)** It goes: the program is counted non-monotone, and what is owed is a detection that warns of it, that milestone's too.

## On the built engine

**How it was taken.** The final binary, `build/review-bin/ein-final-7b72bd84`, the release build of `7b72bd84`, with
the checkout's `stdlib/`; its `--version` is `runs/version.txt`. `tools/run.sh` re-takes every run into `runs/`
(both ignored by git). For each program: the read-out (`X.solve.txt`), `-p`, `--monotone`, `--no-search`,
`ein show hyps`, `ein saturate --dump`, the oracle with every node's state, the events and the summary
(`X.events.jsonl`, `X.summary.json`), the answer and the visited lattice as files, and the knowledge base, the
lattice (both views) and the region drawn (`X.kb.svg`, `X.lattice.svg`, `X.lattice-full.svg`, `X.region.svg`, each
beside its `.dot`). The register's check is run twice: `python3 utils/wrong_answers.py`, the gate's step as it is
(`runs/wrong-answers.txt`), and its runner again with the final binary given (`runs/held-answers-final.txt`).

**The programs.** Every one states `(solve :no-search)` and has no candidate, so its lattice is root alone and the
oracle's cube is one node.

- [`fifo-gap-root.ein`](programs/fifo-gap-root.ein), the witness, `r2` written before `r1`;
  [`fifo-gap-root-r1-first.ein`](programs/fifo-gap-root-r1-first.ein), `r1` written first and nothing else changed;
  [`fifo-gap-root-negation.ein`](programs/fifo-gap-root-negation.ein), the witness's order with `r1` also concluding
  `(not (w ?x))` for a `w` nothing reads. Each is the register's file under `plans/wrong_answers/witnesses/` with a
  header line before it, and nothing else: `runs/X.copy-diff.txt` diffs the two, exit 0, no line.
- **Two added**, because the third twin no longer answers what the register says it does (below):
  [`fifo-gap-root-negation-imported.ein`](programs/fifo-gap-root-negation-imported.ein), the negation twin with
  `(import std.contradiction)` added, and
  [`fifo-gap-root-negation-written.ein`](programs/fifo-gap-root-negation-written.ein), the negation twin with
  `(import std.contradiction :symbols (clash-1))` and the activator `(clash-1 w)` written. Their copy-diffs show the
  rest is the twin's.

### The witness: what the finding said, and what the binary says today

| | the finding's figure, or the register row's (as it was) | **the built engine** | the run |
|---|---|---|---|
| flagged? | not flagged: `lattice monotone`, no warning (V-22) | **not flagged**: `lattice monotone (the static check) — R is convex: no hole`; no `WARNING` row; nothing on stderr | `runs/fifo-gap-root.solve.txt` |
| the static check alone | — | `monotone yes — no site`, `universe 0 candidates` | `runs/fifo-gap-root.monotone.txt` |
| the summary | `monotonicity.order` and `.state` empty (the row's note) | `"monotone": true`, `"warning": null`, `"sites": []`, `"order": []`, `"state": null` | `runs/fifo-gap-root.summary.json` |
| what root holds | `(p one :rule r2)` beside `(q one :rule r1)` (`[case.wrong]`) | **the same two derived facts**, and `Strata — stratified, 3 strata`, `r1` stratum 1, `r2` stratum 2 | `runs/fifo-gap-root.saturate-dump.txt` |
| the order the boundary admits | `r2` before `r1`, as written (the row) | `admit` of `r2` round 1, then of `r1` round 2 | `runs/fifo-gap-root.events.jsonl` |
| R | `k_s = 1`, minimal `[[]]` (`[case.region]`) | `states (k_s) 1`, `∅ in R — root is alive and owes nothing` | `runs/fifo-gap-root.solve.txt` |
| the query `(p ?x)` | `?x = one`, printed `as :expect (model (p one))` (`[case.now]`) | **`?x = one`**, `as :expect (model (p one))` | `runs/fifo-gap-root.solve.txt` |
| the right answer, by the row | no row: `as :expect (model)` (`[case.right]`, the perfect model `(a one) (b one) (q one)`) | not what the witness answers; the `r1`-first twin answers it (below) | |
| `--no-search` | — | `?x = one`; `as :expect not printed: these classes are a lower bound` | `runs/fifo-gap-root.no-search.txt` |
| H | no candidate | `ein show hyps`: nothing on stdout; on stderr ``note: …: H, from `no-search`: no candidate: the program states :no-search`` | `runs/fifo-gap-root.show-hyps.txt` |
| the oracle | — | `states (k_s) 1`, the query `?x = one`, and `the cube 0 dead nodes with an alive superset · 0 discharged nodes with an owing superset — the static verdict holds` | `runs/fifo-gap-root.oracle.txt` |
| the register's check | the gate step, green | `wrong_answers: 9 open rows, 0 defects in the register's keys`; the runner: `9 rows, 9 runs, 9 regions, each still answered wrong`, the same line with the final binary given | `runs/wrong-answers.txt`, `runs/held-answers-final.txt` |

So, on the built engine: **the witness is still not flagged, and it still answers what its row says is wrong**, `(p
one)` beside `(q one)` at root, the query's row `?x = one`.

### The twins

The register row's note says the answer is not a function of the program: the first twin differs in the written
order of two rules, the second in a conclusion nothing reads, *and both answer the perfect model*. On the built
engine the second does not.

| program | the register row's twin (as it was, at `78a5fadd`) | **the built engine** | the run |
|---|---|---|---|
| `fifo-gap-root-r1-first.ein` | `(q one)` only; the query has no row | **the same**: derived `(q one :rule r1)` alone; `the goal has no row`, `as :expect (model)`; `admit` of `r1` round 1 | `runs/fifo-gap-root-r1-first.saturate-dump.txt`, `.solve.txt`, `.events.jsonl` |
| `fifo-gap-root-negation.ein` | `(q one)` only; the query has no row: *a rule that concludes a stored negation is in the agenda's first half (Q-M1f.313), so `r1` is admitted before `r2`* | **`(p one)` beside `(q one)` and `(not (w one))`**; `?x = one`, `as :expect (model (p one))`, as the witness; `admit` of `r2` round 1, of `r1` round 2 | `runs/fifo-gap-root-negation.saturate-dump.txt`, `.solve.txt`, `.events.jsonl` |
| `fifo-gap-root-negation-imported.ein` (added) | — | **`(p one)`** beside `(q one)` and `(not (w one))`; `?x = one`: `r2` admitted first still | `runs/fifo-gap-root-negation-imported.saturate-dump.txt`, `.solve.txt`, `.events.jsonl` |
| `fifo-gap-root-negation-written.ein` (added) | — | **`(q one)` and `(not (w one))`, no `p`**; `the goal has no row`, `as :expect (model)`; `admit` of `r1` round 1 | `runs/fifo-gap-root-negation-written.saturate-dump.txt`, `.solve.txt`, `.events.jsonl` |

All five are monotone by the static check, with no `WARNING` row (`runs/X.monotone.txt`, `runs/X.solve.txt`).

**Why the negation twin moved.** The contradiction is now a theory a program imports
([Q-M1f.557](../../../../../../docs/decisions/stdlib/Q-M1f.557-the-contradiction-is-a-theory-a-program-imports.md)),
and the agenda's first half holds a rule that concludes a stored negation only where a rule refutes the pair:
*a program with no such rule has no such pair, and a stored negation is a fact of the second half*
([Q-M1f.313](../../../../../../docs/decisions/engine/Q-M1f.313-the-agenda-fires-first-what-leads-to-false.md)). The
twin imports nothing, so `r1` waits behind `r2` in the second half, in written order, as in the witness. The row was
taken before that change and its twin is not run by the register's check, which holds each row's `[case.now]`
alone; so the check stays green.

**Why the import alone does not move it back.** With `(import std.contradiction)` the activators `(clash-1 w)` and
the rest are derived at root by `clash-of-1` from the declarations; in the events, `r2` and `r1` are enqueued before
`clash-1` is compiled for `w`, and `r2` is admitted first (`runs/fifo-gap-root-negation-imported.events.jsonl`).
With `(clash-1 w)` written, `r1` is admitted first and root holds no `p`. So on a program the static check calls
monotone, the answer moves with whether an activator of `std.contradiction` is written or derived (V-90, a finding of
this stage, in its record), which is one more face of the same defect: the boundary's order decides root's fixpoint.

What this gives the question: the program is unflagged and answers what its row says is wrong, as the finding said;
and the row's claim that the twins answer the perfect model holds today for the `r1`-first twin and for the negation
twin with its activator written, and not for the negation twin as the register has it.
