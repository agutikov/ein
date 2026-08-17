# S1a.6.7 — Re-measure the lever matrix

**Phase:** P1a.6 (Performance)
**Estimate:** 1 day
**Depends on:** every other stage in the phase
**Implements:** a refresh of
[`docs/kernel/inference/features.md`](../../../docs/kernel/inference/features.md)

## Context

`features.md` records which `SolverConfig` knobs are load-bearing for
solving `zebra2`, with measured impact. It was last regenerated
2026-08-17 against ein.py under PyPy, and its conclusions are
*engine-relative*:

- `enable_singleton_writeback` is the one load-bearing lever — without
  it, exhaustive zebra2 blows up 33× in commitments and does not finish
  in 90 s;
- `enable_fail_fast_fork` is the one plain speed knob (1.9×);
- `enable_pre_branch_lookahead` measures **slightly negative** (0.9×) —
  it pays a one-step simulation to avoid forks that fail-fast made cheap;
- everything else is ≤ 1.1×, and two levers are inert on this puzzle.

At least one of those should move. The lookahead's cost is a *match*, and
matching is now much cheaper; the fail-fast win is a *saturation* saving,
and saturation is now much cheaper too. Which way the ratios land is a
measurement, and it is the last thing this phase owes.

## Acceptance

- `features.md` regenerated with an ein.rs column beside the ein.py one,
  same method (fresh subprocess per cell, one lever off the all-on
  baseline, fast + exhaustive modes, budgets), same provenance block
  (date, commit, machine).
- **Same verdicts and same entering counts** as ein.py for every cell —
  this is a T1 check dressed as a benchmark, and a mismatch here is a
  parity bug, not a performance finding.
- `enable_singleton_writeback` still the one lever whose absence fails to
  finish. If it is *not*, that is a significant finding and it goes in
  the stage log prominently.
- Any proposed default change is written up with its numbers and left as
  a decision, not applied silently.

## Tasks

### Task T1a.6.7.1 — Port the harness

`utils/feature_matrix.py` drives ein.py; extend it to drive either
implementation (it already shells out per cell). Keep the JSON artifact
shape so old and new runs are comparable.

### Task T1a.6.7.2 — Run and cross-check

Run the full matrix for both engines on the same machine on the same
day. Cross-check every cell's verdict and entering count between engines
before looking at any timing — a timing comparison across two engines
that explored different numbers of commitments is meaningless.

### Task T1a.6.7.3 — Re-examine the lookahead

`enable_pre_branch_lookahead` at 0.9× was already a "shape to re-measure
on a deeper puzzle". Measure it on `zebra` as well as `zebra2`, and on a
deliberately deeper fixture (a `--max-set-size 3+` puzzle from
`examples/lattice/`), since its benefit grows with branch depth where an
unpruned fork is a whole subtree.

### Task T1a.6.7.4 — Update the narrative

`features.md`'s takeaway section, and
[`architecture_and_algorithms.md` §7](../../../docs/kernel/inference/architecture_and_algorithms.md)'s
"where the bodies are" summary, both state costs that this phase
changed. Update them with the new split, keeping the historical numbers
labelled rather than overwritten — the arc is the interesting part.

### Task T1a.6.7.5 — Close or update F11

[F11](../../followups/f11_deductive_layer_perf.md) named the Rust port as
its own most likely promotion trigger. Record the outcome: D1 landed
(with numbers), or D1 measured and re-parked (with numbers), and D2's
trigger re-checked.

## Notes

- Resist changing a default in the same commit as the measurement. The
  measurement is a fact; the default is a decision, and the two want
  separate review.
- If a lever's sign flipped, add a fixture that makes the *new*
  behaviour visible — the matrix is a snapshot, the corpus is the
  regression net.
