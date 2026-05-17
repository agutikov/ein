# M3 — Open questions

Milestone-scoped. Cross-milestone questions live in
[`../open_questions.md`](../open_questions.md).

## Index

| Q   | Title                                                                       | Resolved in |
|-----|-----------------------------------------------------------------------------|-------------|
| Q25 | Primary backend — Z3, CVC5, or both?                                        | P3.1 S3.1.1 |
| Q26 | Type mapping — bit-vectors, integers, enums?                                 | P3.2 |
| Q27 | When does the graph engine hand off? Static threshold or learned heuristic?  | P3.5 |
| Q28 | Model extraction — full assignment or only goals?                           | P3.3 |
| Q29 | Unsat-core lift — minimum-cardinality or smallest-text-explanation?         | P3.4 |

---

## Q25 — Primary backend

**Options:**

- **Z3** — Python bindings, fast, model + unsat-core support.
- **CVC5** — also strong, the existing `smt/CVC4` submodule
  suggests prior interest.
- **Both** — pluggable backend interface; choose at runtime.

**Working answer**: Z3 primary (most mature Python bindings,
strong on the theories Zebra needs — integers, equality). CVC5
behind an abstract `SmtBackend` interface, used in P3.5 as an
oracle cross-check on the first few real puzzles. Final P3.1 S3.1.1.

## Q26 — Type mapping

For Zebra, three plausible sort choices for "position":

- **Integers** with bounds (`pos ∈ {1..5}` as `1 ≤ pos ∧ pos ≤ 5`).
- **Bit-vectors** of length `ceil(log2(N))`.
- **Enum / datatype** (`(declare-datatype Pos ((H1) (H2) ...))`).

**Working answer**: integers + bounds. Cleanest for the human
trace; SMT solvers handle small bounded ints in milliseconds.
Final P3.2.

## Q27 — Handoff threshold

When does the M1 engine declare a slice solver-shaped?

**Working answer**: explicit IR annotation only for now —
`(hard-slice …)` forms in the IR mark sub-problems for the
SMT path. No automatic heuristic in M3; revisit if the trace shows
the engine doing visibly-poor work that SMT would crush. The
[F4 followup](../followups/f4_cross_cutting.md) parks the
learned-heuristic question.

## Q28 — Model extraction granularity

**Working answer**: query goals only by default; full assignment on
`--full-model` flag. Per-goal extraction lets the trace stay
focused on what the user actually asked.

## Q29 — Unsat-core lift

**Working answer**: minimum-cardinality core via Z3's `with
unsat-cores enabled` + a follow-up MUS refinement. Smallest-text
is post-processing concern (the trace renderer adds it). Final
P3.4.
