# P1a.7 — Parallelism

**Milestone:** [M1a — Rust port](../README.md)
**Estimate:** 2.5 weeks (13 days of stages)
**Depends on:** [P1a.6](../p1a.6_performance/README.md) — parallelise a
fast engine, not a slow one, or the speedup measures the wrong thing.

## Goal

Use the cores without giving up the byte gate. `--jobs 1` stays the
default and stays T3; `--jobs N` is **also** T3 — same verdict, same
models, same counters, same stdout — via speculate-and-validate; and
`--unordered` is an explicit opt-out for throughput.

Design: [design/08](../design/08_parallelism.md).

## Stages

| stage | title | est. |
|---|---|---|
| [S1a.7.1](s1a.7.1_sync_shared_state.md) | Making the shared state `Sync` | 3 d |
| [S1a.7.2](s1a.7.2_parallel_enterings.md) | Level 1: parallel enterings | 4 d |
| [S1a.7.3](s1a.7.3_parallel_boundary.md) | Level 3: the parallel boundary round | 2 d |
| [S1a.7.4](s1a.7.4_parallel_enqueue.md) | Level 2: the parallel enqueue pass | 2 d |
| [S1a.7.5](s1a.7.5_jobs_contract.md) | The `--jobs` contract | 2 d |

## Acceptance for the phase

- `--jobs {1,2,4,8,16}` T3-identical on the whole corpus.
- A 10 000-run randomised stress of `--jobs 8` vs `--jobs 1` with no
  divergence.
- ≥ 6× on 8 cores for exhaustive zebra2's Phase 2 wall-clock.
- Re-validation rate reported and ≤ a few percent; if not, the read-set
  tracking is refined before the mode ships (Q-M1a.7).
- TSan and `loom` clean on the shared structures.

## Risks

- **The validation argument is the whole phase.** `sat(base ∪ W ∪ c) =
  sat(sat(base ∪ c) ∪ W)` holds because the KB is append-only and
  saturation is a least fixpoint. Write it down next to the code, with
  the fixture that would break if it were false (a layer-2 commitment
  whose fork reads a `(not h)` written mid-layer).
- **Memory scales with jobs.** N live forks = N deltas over one shared
  base; measure peak RSS at `--jobs 16` on the worst corpus entry
  (`enable_singleton_writeback=false`, 3 336+ enterings).
- **Speculative waste at `stop_after`.** Bounded by the job count, but
  measure it: a `-n 1` solve that speculates 16 enterings to use 1 is
  fine; one that speculates 16 layers is not.

## Cross-links

- [design/08 — Parallelism](../design/08_parallelism.md)
- [design/03 §5 — `Arc<KbCore> + Delta`](../design/03_data_model.md)
