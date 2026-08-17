# P1a.4 — Search layer

**Milestone:** [M1a — Rust port](../README.md)
**Estimate:** 4 weeks (19 days of stages)
**Depends on:** [P1a.3](../p1a.3_deductive_core/README.md)
**Blocks:** [P1a.5](../p1a.5_presentation/README.md)

## Goal

Everything above the fixpoint: hypothesis generation, one-step
lookahead, the Apriori commitment lattice, no-good learning, the
commitment primitive, and the three-phase `solve` loop with its verdict
synthesis. At the end of this phase `ein.rs solve <file>` returns the
right answer with the right counters on every corpus entry — **T1
corpus-wide**, T2 on the branching fixtures.

Design: [design/07](../design/07_search_layer.md).

## Stages

| stage | title | est. |
|---|---|---|
| [S1a.4.1](s1a.4.1_hypothesis_generation.md) | Hypothesis generation | 4 d |
| [S1a.4.2](s1a.4.2_lookahead_and_closure.md) | Lookahead, closure marking, NAF dependency map | 3 d |
| [S1a.4.3](s1a.4.3_apriori_and_nogoods.md) | Apriori candidate generation and the no-good store | 3 d |
| [S1a.4.4](s1a.4.4_commitment_primitive.md) | The commitment primitive | 2 d |
| [S1a.4.5](s1a.4.5_solve_loop.md) | The solve loop and verdict synthesis | 4 d |
| [S1a.4.6](s1a.4.6_explanation_and_cores.md) | Explanation and unsat cores | 3 d |

## Acceptance for the phase

- **T1 corpus-wide**: every counter in
  [design/01](../design/01_parity_contract.md) §2, on every corpus entry
  × run-matrix cell.
- **T2** on `examples/branching/**`, `examples/lattice/**`,
  `examples/domain_elim/**`: identical `hyp` / `enter` / `nogood` /
  `writeback` event sequences.
- The three acceptance fixtures (`test_zebra_two_ontologies`,
  `test_zebra_three_classes`, `test_mode_consistency`) reproduce, with
  the same models.
- `--shuffle --seed N` produces the same traversal as ein.py for the same
  seed (Q-M1a.5) or the shuffle rows are explicitly T0-only in the
  ledger.
- [`features.md`](../../../docs/kernel/inference/features.md)'s lever
  matrix regenerated against ein.rs: same verdicts, same entering counts,
  `enable_singleton_writeback` still the one load-bearing lever.

## Risks

- **`explain` is where a "cleaner" port silently changes the answer.**
  Its tie-breaks are `repr`-based and its search is a budgeted least
  fixpoint. Port it literally, diff its output on every corpus
  contradiction, and resist restructuring until T2 is green.
- **hypgen's stats are attribution, not accounting.** The filter *order*
  decides which counter a drop lands in. Any reordering — including one
  that looks like an optimisation — is a T1 failure.

## Cross-links

- [design/07 — Search layer](../design/07_search_layer.md)
- [`algorithm_layer_n.md`](../../../docs/kernel/inference/algorithm_layer_n.md)
- [F9 ledger](../../followups/f9_e_catalog.md) — the rejected
  search-layer optimisations; do not re-derive them in Rust.
