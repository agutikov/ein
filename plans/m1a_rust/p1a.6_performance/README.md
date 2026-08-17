# P1a.6 — Performance

**Milestone:** [M1a — Rust port](../README.md)
**Estimate:** 3.5 weeks (18 days of stages)
**Depends on:** [P1a.5](../p1a.5_presentation/README.md) — the byte gate
must be closed first, so every change here is measured against a green
harness.
**Absorbs:** [F11 — deductive-layer perf](../../followups/f11_deductive_layer_perf.md)
(D1 beta-memories, D2 WCOJ).

## Goal

Turn the parity build into a fast one, with T3 green at every step.
Method is fixed and non-negotiable: **profile, change one thing, re-diff,
re-measure, record.** A change that cannot be attributed is reverted.

## Targets

Against PyPy on the same machine, at `--jobs 1`:

| workload | PyPy today | target |
|---|---|---|
| `solve zebra2.ein -e` end-to-end | 4.07 s | ≤ 0.20 s (≥ 20×) |
| `solve zebra.ein -e` end-to-end | 8.15 s | ≤ 0.40 s |
| parse + load `zebra2.ein` | 0.78 s | ≤ 0.015 s (≥ 50×) |
| the acceptance gate (3 fixtures) | ~91 s | ≤ 5 s |

Much of this should already be true when the phase *starts* — the
register matcher, integer facts, O(1) forks, compile-once and the
semi-naive boundary all land in P1a.2–3. The phase exists to find what is
left, not to assume it.

## Stages

Everything after S1a.6.1 is *chosen* by the table S1a.6.1 produces. The
list below is the expected shape, not a commitment: a stage the profile
does not justify is skipped, with the reason recorded.

| stage | title | est. |
|---|---|---|
| [S1a.6.1](s1a.6.1_profile_baseline.md) | Fresh profile and bench baseline | 2 d |
| [S1a.6.2](s1a.6.2_memory_layout.md) | Memory layout | 3 d |
| [S1a.6.3](s1a.6.3_beta_memories.md) | Beta-memories (F11 D1) — **gated** | 4 d |
| [S1a.6.4](s1a.6.4_hypgen_and_lattice.md) | Hypgen and lattice hot paths | 3 d |
| [S1a.6.5](s1a.6.5_frontend.md) | Frontend and load path | 2 d |
| [S1a.6.6](s1a.6.6_differential_fuzzer.md) | The differential fuzzer | 3 d |
| [S1a.6.7](s1a.6.7_relever_matrix.md) | Re-measure the lever matrix | 1 d |

## Rules for this phase

1. **T3 stays green.** A perf change that needs a ledger entry is not a
   perf change, it is a semantics change, and it goes back to the
   relevant phase.
2. **One change per commit, with its number.** The commit message
   carries the before/after for the benchmark it targeted.
3. **A wash is a revert.** P1.8a's D3 cross-fork carry was built and
   reverted the same day; that is the standard.
4. **No search-layer re-litigation.** [F9](../../followups/f9_e_catalog.md)
   measured that cluster inert against a complete cardinality-BFS. Rust
   does not change the branch count.
5. **Record everything** in
   [design/README.md § Measured](../design/README.md#measured).

## Acceptance for the phase

- Targets met, or a written account of which one was not and why.
- T3 green on the whole corpus at every commit in the phase.
- The fuzzer has run for ≥ 24 h with no unexplained T1 divergence.
- `features.md` regenerated with an ein.rs column.
- F11 closed or updated: D1 landed, or D1 measured and parked with the
  numbers.

## Cross-links

- [design/05 §7 — beta-memories](../design/05_matcher.md)
- [design/06 §3–§4 — the two exact wins](../design/06_saturation.md)
- [`architecture_and_algorithms.md` §7](../../../docs/kernel/inference/architecture_and_algorithms.md)
  — the lever list this phase works through
