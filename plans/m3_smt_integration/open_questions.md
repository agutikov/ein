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
| Q30 | Seam ↔ SMT mapping — Clark completion at the NAF boundary                   | P3.1–P3.3 |

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

## Q30 — Seam ↔ SMT mapping (Clark completion at the NAF boundary)

Added 2026-08-16 by P1.21 R6
([`r6_seam.md`](../m1_core_graph_reasoning/p1.21_review_response/reports/r6_seam.md);
edge-by-edge table in its §3). The M1 target seam
([`docs/kernel/architecture.md` §closure/worlds seam](../../docs/kernel/architecture.md#the-closureworlds-seam))
gives each side an SMT counterpart; three sub-questions to settle:

- **NAF boundary → Clark completion.** SMT has no NAF: `(absent P)`
  translates to `¬∃x̄.P` **only** under a Clark-completion axiom whose
  scope is exactly the boundary's world `W`.
  [`naf_deps`](../../ein.py/src/ein/inference/naf_deps.py)'
  `declared_only` vs `derived` split says which relations get a finite
  completion axiom vs need stratified/ASP treatment (clingo as alternate
  backend). NAF buried inside closure `JoinPlan`s makes the translation
  non-compositional — the concrete reason the seam (and
  [P1.21 S1.21.8](../m1_core_graph_reasoning/p1.21_review_response/s1.21.8_boundary_naf.md))
  must be explicit before M3's translator.
- **Worlds lattice → assumptions.** Commitments ↔ assumption literals +
  `check-sat-assuming`; `DeadCommitment.unsat_core` ↔ solver unsat cores
  over assumptions; learned `_nogoods` ↔ learned clauses; the Apriori
  layer-BFS is replaced by the solver's internal search or AllSAT
  enumeration with blocking clauses.
- **StateKey → blocking clauses.** Model dedup/enumeration builds a
  blocking clause over a model's atoms — this needs the canonical fact
  *tuple* ([`canon.StateKey`](../../ein.py/src/ein/inference/canon.py),
  shipped P1.21 R1), not a digest.

**Working answer**: recorded now so the P3.1 translator design starts
from the seam, not from the engine's leak list; decide per sub-question
in P3.1 (translation), P3.2 (completion-axiom sorts/bounds), P3.3
(assumptions + enumeration). Final P3.1–P3.3.
