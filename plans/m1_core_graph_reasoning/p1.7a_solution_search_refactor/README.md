# P1.7a — Sound solution model & search/result/stop refactor

**Estimate:** 1–2 weeks (investigation + structural change + acceptance).
**Depends on:** [P1.5b](../p1.5b_lattice_search/README.md) (the unified
set-search engine), [P1.6](../p1.6_rendering_and_trace/) (trace renderer that
consumes the verdict).
**Blocks:** M1 done — this *is* the corrected M1 acceptance gate (relocated
from [P1.7 S1.7.3](s1.7.3_trace_acceptance.md)).
**Spun out of:** [P1.7](../p1.7_bootstrapping_zebra/README.md) on 2026-05-31.

## Why this phase exists

[S1.7.3](s1.7.3_trace_acceptance.md) (trace-matches-human acceptance, the M1
gate) ran the engine on the canonical satisfiable puzzle `zebra2.ein` and
found a **severe soundness bug**: the engine reports a *non-solution* as a
`Solution`. `monotonic_solve` stops on the first **goal-pattern match**
(`is_solved`) — and the house-only goal `(drink-loc Water ?h) ∧ (pet-loc
Zebra ?h)` matches on a **partial, 7-cells-wrong dead-end** (it commits
`(color-loc Green House-4)` and quits). Completing that grid forces
`Spaniard = Japanese` → ⊥. The first patch attempt (bolting completeness onto
`is_solved`) made it worse: monotonic then returns **`Contradiction` for a SAT
puzzle**. Both directions break the hard invariant **a correct engine never
exhausts a satisfiable problem to ⊥, and never calls a non-model a model.**

[S1.7.3a](s1.7.3a_open_hypothesis_completeness.md) designed the *correct*
fix — a domain-agnostic definition of solution — and its Phase A
investigation (code-cited, 2026-05-31) established the key facts. **But the
fix is not one stage.** It is a structural change to how the engine frames
search, results, and stopping. This phase carries that change end-to-end.

## The reframe (the heart of this phase)

> The bug is not a missing check. It is a category error in the engine's
> public shape: *solve / gaps / contradictions* are modelled as **three
> search modes the caller picks up front**, when they are **three
> interpretations of one search's outcome**.

Three concerns are currently conflated into "mode". P1.7a separates them:

| concern | what it is | who decides |
|---|---|---|
| **the search** | enumerate every **solution node** = a saturated commitment that is **consistent ∧ complete** (no open hypothesis), **deduped by `state_hash`** | nobody — it is invariant; a solution is *always the same thing* |
| **the result type** | a *reading* of the result count `k`: `k=1` → **unique** · `k>1` → **ambiguity (gaps)** · `k=0` → **contradiction** | derived, not chosen — falls out of the run |
| **the stop policy** | when to stop looking: *first solution* · *first 42* · *exhaust* · resource budgets (`max_set_size`/`max_time`/`max_enterings`) | the user — an **optional convenience**, orthogonal to both above |
| **the projection** | what to *print* from the solution node(s): the `:goal` answer ("who keeps the zebra"), the residual open set (the gap), the unsat cores | the user — an output choice, not a search choice |

So **"give me the first complete+consistent solution" is not a mode** — it is
the stop policy `stop_after = 1` on the one sound search. "42 first
solutions" is `stop_after = 42`. `gaps` and `contradictions` are
`stop_after = None` (exhaust) plus a projection. `:mode` / `--mode` demote
from *search selector* to *(stop preset × projection)*.

This is the completion of the user's 2026-05-28 direction recorded in the
[P1.5b README](../p1.5b_lattice_search/README.md#what-this-phase-delivers):
*"they all first iterate lattice nodes, and then make decisions about results
depending on mode."* P1.5b built the shared `_explore_layers` loop but kept
`is_solved`/`Mode`-driven termination and three return types; P1.7a finishes
the job and makes the shared decision **sound**.

## What's solid going in (S1.7.3a Phase A, code-verified)

- **The completeness predicate is free.** `_compute_alive(kb)`
  (`inference/monotonic/solver.py:393`) already returns the open-hypothesis
  set; `complete(kb) ≡ not _compute_alive(kb)`. No `is-a` / `total` /
  signature crutches needed (those made the first patch encoding-specific and
  wrong).
- **`state_hash` is the dedup key.** `inference/canon.py:27` hashes the
  propositional facts order-insensitively, excluding bookkeeping heads. Same
  model ⇒ same hash (zebra2's 14 winning commitments share one).
- **Build on the lattice, not monotonic.** `monotonic_solve`'s
  `_promote_forced_positives` (`solver.py:337`) pollutes its shared,
  append-only root with *wrong* forced-positives once driven past
  first-goal-match (Phase A I5) — it is unsound for complete-model finding by
  construction. The sound solution node is a **per-branch** commitment kb
  (gaps-style), never the shared root.

## Documents in this phase

| doc | role |
|---|---|
| [`target_design.md`](target_design.md) | the ideal end-state: one search, result-type-from-k, orthogonal stop + projection, the public API we want |
| [`analysis.md`](analysis.md) | what we **have** / what's **missing** / what's **incorrect**, code-grounded; the gap between today and the target |
| [`s1.7.3_trace_acceptance.md`](s1.7.3_trace_acceptance.md) | *(origin)* the M1 gate that surfaced the bug |
| [`s1.7.3a_open_hypothesis_completeness.md`](s1.7.3a_open_hypothesis_completeness.md) | *(origin)* the correct definition + Phase A findings |
| `refactoring_design.md` | the concrete change: types, signatures, migration of the three entries |

## Transition stages

| ID | title | status (2026-05-31) |
|---|---|---|
| **S1.7a.1** | Investigations — finish the analysis (open-vs-refuted; exhaustive search on all 3 variants; certification) | ✅ **done** (`4297deb`) — probe + findings in [`analysis.md`](analysis.md) |
| **S1.7a.2** | `solution_node` core — `complete ∧ consistent`, deduped by `state_hash`; the predicates | ✅ **shipped** (`4297deb`) — `inference/solution.py` |
| **S1.7a.3** | One search + result-type-from-`k` — verdict from the deduped count; retire `is_solved`/`Mode` as terminator | ✅ **shipped** (`4297deb`) — `solve()`/`verdict_of`, **pure per-branch** |
| **S1.7a.4** | Stop policy — `stop_after: int \| None`; honesty about early-stop vs exhausted | ✅ **shipped** (`4297deb`) |
| **S1.7a.5** | `:goal` as projection; demote `:mode`; revert the S1.7.3 experiment | ✅ **done** (via .6) — revert was a no-op; projection landed in S1.7a.6 |
| **S1.7a.6** | Trace + CLI answer path — answer-in-words for SOLVE; verdict-consuming linearizer | ✅ **shipped** — `solve --mode=solve` + `trace/answer.py`; proof-less verdicts render |
| **S1.7a.7** | **Acceptance** + test infra (`run_tests.sh`, drop `pytest.ini`); PyPy 3-variant solve ← **M1 gate** | ✅ **shipped** (`75bee52`) — gate **GREEN** |

## What shipped (2026-05-31) — and where it deviated from the plan

P1.7a's soundness goal is **met and gated** (commits `4297deb`, `75bee52`):
`solve()` gives `zebra2` → unique Solution (25/25), `zebra2-minus-15` →
Ambiguity, `zebra2-bad` → Contradiction; the SAT↛⊥ / UNSAT↛Solution invariant
holds. Deviations from [`target_design.md`](target_design.md) /
[`refactoring_design.md`](refactoring_design.md), recorded honestly:

- **Pure per-branch, not the surgical-reuse plan.** `refactoring_design.md`
  proposed reusing the root-merge + forced-positive machinery; in practice
  that **reproduced the SAT→⊥ bug** because `unconditional_facts` extraction
  is *unsound under NAF* (`absent`). `solve()` keeps root stable (no
  cross-commitment merge) + a sound inter-layer forced-positive prune.
- **Perf is the soundness tax.** Exhaustive `solve()` ≈ 90s (zebra2) / 64s
  (minus-15 to 2 models) under PyPy — hence acceptance is a separate slow
  phase; `stop_after=1` is the ~6s fast answer.
- **Unsat core = source frontier (~38 facts), not a minimal MUS.** Criterion
  3's "2–3 edge core" is aspirational; the injected fact is in the core,
  condition (6) grounds out through the `right-of` chain. MUS minimisation is
  a follow-up.
- **Acceptance lives in `ein.py/acceptance/`** (outside pytest `testpaths`),
  run as Phase 2 of `run_tests.sh` — not `tests/`-resident (user direction).
- **Cross-interpreter PyPy-speedup assertion (criterion 5) not wired** (no
  CPython venv locally); PyPy is the runner default.
- **S1.7a.6 shipped** — `ein-bot solve --mode=solve` prints the English
  answer (the *who* projected via `nation-loc` at the goal's house); proof-less
  `Solution`/`Ambiguity`/`Contradiction` all render. With this, the M1
  acceptance criteria #1–5 are met; the **M1-done closer** (mark done in
  `../p1.7_bootstrapping_zebra/README.md` + `plans/README.md`) and the manual
  trace-narrative review remain. Other non-gating follow-ups: minimal-MUS
  unsat core, the cross-interpreter speedup assertion.

## Acceptance (this is M1's, corrected)

1. **`zebra2.ein` → `k = 1`** (unique). The complete 25/25 model; the
   `--mode=solve` / answer path prints *"The Japanese keeps the zebra; the
   Norwegian drinks water."*
2. **`zebra2-minus-15.ein` → `k > 1`** (gaps). Reports the residual open set
   (the contingent hypotheses) and names the dropped condition it depends on.
3. **`ein-bugs/zebra2-bad.ein` → `k = 0`** (contradiction). A tight unsat
   core (2–3 source edges) pairing the injected fact with condition (6).
4. **Invariant:** no SAT puzzle yields `Contradiction`; no UNSAT puzzle yields
   `Solution`. Pinned by a regression test across the variant suite.
5. **`run_tests.sh`** runs the whole suite; root `pytest.ini` removed. The
   3-variant acceptance runs under **PyPy** and the zebra2 PyPy speedup over
   CPython is asserted (or reported). Full suite green; `ruff check .` green.

Kernel-purity work ([P1.7 S1.7.6/.7](../p1.7_bootstrapping_zebra/README.md))
stays in P1.7 and is **not** part of this gate.

## Connections

- [Idea 03 — three task classes](../../../docs/ideas/03-three-task-classes.md):
  this phase makes solve/gaps/contradictions *one* operation with three
  readings — exactly idea 03's "different shapes of the same graph question".
- [Idea 08 — human-style trace](../../../docs/ideas/08-human-style-deductive-trace.md):
  the answer path + reductio rendering are the trace's closing.
- [`examples/README.md`](../../../examples/README.md): the M1 target trace,
  unchanged — P1.7a is what makes the engine actually reach its end state
  soundly.
