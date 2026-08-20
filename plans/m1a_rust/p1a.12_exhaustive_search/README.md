# P1a.12 — Exhaustive search over many models

**Milestone:** [M1a — Rust port](../README.md)
**Estimate:** 3 weeks (15 days of stages)
**Depends on:** [P1a.7](../p1a.7_parallelism/README.md) — cores change the
constant, not the exponent, and this phase is about the exponent. Knowing
which is which needs the parallel numbers first.

## Goal

**Understand why an under-determined puzzle does not finish, and decide what
to do about it.** `examples/zebra2-minus-15.ein` is the case: the canonical
zebra2 with one condition removed, exhaustively solvable in principle,
uncompletable in practice.

## What is already measured

From the 2026-08-20 session that found the `disjunctive-prune` bug, with an
independent brute force as ground truth (control: restore condition (15), get
exactly one model, the canonical grid):

| depth cap | enterings | models found | wall |
|---|---:|---:|---:|
| `-m 1` | 96 | 0 | 24 ms |
| `-m 2` | 4 656 | 28 | 1.4 s |
| `-m 3` | 48 745 | **32 — all of them** | 25.3 s |
| `-m 5` (the default, i.e. `-e`) | — | — | **killed at 30 min** |

Ground truth: **32 models.** Three readings, and the third is the phase:

1. **Nothing prunes at layer 1.** All 96 candidates come back alive — no
   death, so no learned clause and no singleton `(not h)` writeback — and
   layer 2 is therefore the full `C(96,2)`. The `alive=96` a reader sees in the
   `-v` header is the count of live hypothesis *facts*, and it never shrinks,
   because nothing is ever refuted.
2. **Growth is ~11× a layer** and the wall clock with it: 96 → 4 656 → 48 745.
   Layers 4 and 5 are the run nobody sees finish.
3. **Every model is found by depth 3. Depths 4 and 5 exist only to certify
   that there are no more.** The cost is not *finding*, it is *proving there
   is nothing left* — and the engine's only proof of that is exhausting the
   lattice.

That third line is the phase's whole subject, and it was not visible before
this measurement.

## Why F9 does not already close this

[F9](../../followups/f9_e_catalog.md) rejected most of the search-optimisation
catalogue, and its cluster note is the reason to read it first:

> Re-judged against the engine's actual search — a *complete BFS over
> commitment-set cardinality* (Apriori), not a DPLL/DFS decision tree —
> reorderers are inert … A complete cardinality-BFS over a connected corpus
> leaves no purchase for any of them.

E10 (iterative deepening) is closed as "inapplicable — cardinality layering
already *is* breadth-first deepening". **Every one of those judgements was
measured on a puzzle with a unique model.** On zebra2, layer 1 kills 67 of 101
candidates and the pruning is what makes the search tractable; on
zebra2-minus-15 layer 1 kills nothing at all. Those are different regimes, and
F9 measured one of them.

This is [S1a.6.4](../p1a.6_performance/s1a.6.4_hypgen_and_lattice.md)'s lesson
a third time — the phase had been measuring one shape of workload — so the
first stage here is a census, not a proposal.

## Stages

| stage | title | est. |
|---|---|---|
| [S1a.12.1](s1a.12.1_why_it_does_not_finish.md) | Why it does not finish | 3 d |
| [S1a.12.2](s1a.12.2_depth_required.md) | What depth is required, and for what | 2 d |
| [S1a.12.3](s1a.12.3_stopping_criterion.md) | Is there a stopping criterion? | 4 d |
| [S1a.12.4](s1a.12.4_conflict_mining.md) | Conflict mining when a layer is barren | 4 d |
| [S1a.12.5](s1a.12.5_contract.md) | What `exhausted` means | 2 d |

## Acceptance for the phase

- **`solve -e examples/zebra2-minus-15.ein` finishes**, with all 32 models and
  a stated exhaustion claim — or the phase records, with numbers, why it
  cannot and what the honest verdict is instead.
- The under-determined regime is a **named part of the measurement set**, the
  way [P1a.7](../p1a.7_parallelism/README.md) had to re-aim its scaling target.
  One under-determined entry in the corpus is not a regime, it is an anecdote.
- **Nothing changes what the engine proves.** A sound criterion makes the same
  proof cheaper; an unsound one changes the answer. Anything in the second
  class ships behind a flag and reports a *different* verdict word — never a
  quiet `exhausted = true`.
- Every proposal is measured against F9's discipline: **a mechanism that is
  inert on the corpus is recorded as inert and not shipped**, with the number.
- The determinate puzzles do not regress: `zebra -e`, `zebra2 -e` and the
  P1a.6 targets hold their timings and their counters.

## Risks

- **Changing the traversal changes the counters.**
  [design/08](../design/08_parallelism.md) §7 rejected parallel depth-first for
  exactly this: "going depth-first changes which no-goods exist when, i.e. the
  pruning, i.e. the counters". The same is true of a sequential dive. This
  phase therefore needs the decision P1a.7 needed —
  [Q-M1a.18](../open_questions.md#q-m1a18--may-a-fork-stop-re-narrating-the-roots-fixpoint)'s
  shape — before anything ships on by default.
- **An unsound stopping rule is worse than a slow search.** "No new model for
  k layers, so stop" is a heuristic wearing a proof's clothes. If it ships it
  reports `Ambiguity (not certified)`, and the word `exhausted` stays false.
- **This is the boundary of M1a's non-goals.** "Anything that changes what the
  engine can prove belongs in a followup." A *sound* criterion does not — it
  proves the same thing sooner. A heuristic mode does, and belongs in
  [F4](../../followups/f4_cross_cutting.md) unless the phase argues otherwise
  explicitly.
- **Memory before time.** `features/01_not_and_absent -e` peaks at 724 MB and
  an uncapped `saturation/square-unique/terminus.ein -e` reached 12.3 GB before
  being OOM-killed ([baseline.md §15](../p1a.6_performance/baseline.md)). A
  deeper search may not get the chance to be slow.

## Cross-links

- [design/07 — Search layer](../design/07_search_layer.md)
- [F9 — the rejected search optimisations](../../followups/f9_e_catalog.md) —
  read before proposing anything here
- [`examples/zebra2-minus-15.ein`](../../../examples/zebra2-minus-15.ein) —
  the case, and `conformance/corpus.toml`'s note on why `solve -e` is not one
  of its runs
