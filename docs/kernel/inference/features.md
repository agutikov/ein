# Engine feature × config matrix

Which `SolverConfig` knobs are load-bearing for solving `zebra2`, with
measured impact. The companion to the *definitional* config table in
[`docs/api/inference.md`](../../api/inference.md) (what each knob does) and
the engine narrative in
[`architecture_and_algorithms.md`](architecture_and_algorithms.md) (how
each feature works).

> **Audience: engine contributors / advanced authors.** Most puzzle
> authors only need the takeaway below.

## Takeaway

**On `zebra2`, the shipped fast path is robust — keep the defaults and
don't worry about these knobs.** With `stop_after=1` (the default solve),
disabling *any single* lever still finds the correct unique answer in ~2 s.
The levers earn their keep in **exhaustive** search (proving uniqueness /
unsatisfiability), where one is decisive:

- **`enable_singleton_writeback` is load-bearing for exhaustive solves.**
  Without it, exhaustive `zebra2` blows up (≥7× the commitments explored)
  and does **not** finish within a 90 s budget. Keep it on.
- Every other lever is ≤1.3× on `zebra2`, and two are effectively inert on
  this puzzle (`enable_forced_positive` never fires; `enable_symmetric_mirror`
  has a transparent rule fallback). They may matter more on larger or
  differently-shaped puzzles — re-measure with the harness below.

**No single lever is *correctness*-load-bearing on `zebra2`**: every
flag-off run that terminated returned the identical solution.

## Method

Measured by [`utils/feature_matrix.py`](../../../utils/feature_matrix.py)
(re-run to regenerate; raw artifact at `utils/feature_matrix_results.json`).
Each cell solves `examples/zebra2.ein` in a fresh PyPy subprocess with one
lever flipped off the all-on baseline, in two modes:

- **fast** — `stop_after=1` (the shipped default; stops at the first
  complete model), 30 s budget.
- **exhaustive** — `stop_after=None` (explores the whole commitment
  lattice; a disabled prune shows its full blow-up), 90 s budget.

A cell exceeding its budget returns an `Aborted` verdict — the
"won't-finish-if-off" sentinel (`∞`). Counts are `MonotonicStats`
enterings; `×base` is wall-time vs the all-on baseline.

*Provenance: `ein` at `8575bcc` + the S1.20.I2 flag-gating; PyPy
(`.venv-pypy`); single-run, machine-specific — read the **factors**, not
the absolute seconds.*

## Fast path (`stop_after=1`) — robust

Every lever-off run matches the baseline: **Solution, k=1, correct answer,
~2 s**.

| lever off | verdict | enterings | wall (s) | ×base |
|-----------|---------|-----------|----------|-------|
| *(baseline — all on)* | Solution | 11 | 2.0 | 1.0× |
| `enable_pre_branch_lookahead` | Solution | 11 | 1.9 | 1.0× |
| `enable_lookahead_kill_cache` | Solution | 11 | 1.9 | 1.0× |
| `enable_path_nogoods` | Solution | 11 | 1.9 | 1.0× |
| `enable_symmetric_mirror` | Solution | 11 | 1.9 | 1.0× |
| `enable_singleton_writeback` | Solution | 11 | 2.0 | 1.0× |
| `enable_forced_positive` | Solution | 11 | 2.0 | 1.0× |
| `hypgen_scoring="most-constrained"` | Solution | 11 | 1.9 | 1.0× |
| `lattice_order="score-sum"` | Solution | 13 | 1.7 | 0.9× |

## Exhaustive (`stop_after=None`) — where the levers bite

Baseline: Solution, k=1, **101 enterings (67 dead), 12.2 s**.

| lever off | verdict | enterings | wall (s) | ×base | note |
|-----------|---------|-----------|----------|-------|------|
| `enable_singleton_writeback` | **Aborted** | **790+** | **≥90 (∞)** | **≥7.4×** | **load-bearing** — does not finish |
| `lattice_order="score-sum"` | Solution | 134 | 16.3 | 1.3× | mild reordering cost |
| `enable_pre_branch_lookahead` | Solution | 111 | 14.0 | 1.1× | small |
| `enable_lookahead_kill_cache` | Solution | 101 | 12.8 | 1.0× | inert here |
| `enable_path_nogoods` | Solution | 101 | 11.9 | 1.0× | inert here |
| `enable_symmetric_mirror` | Solution | 101 | 12.6 | 1.0× | rule fallback (see below) |
| `enable_forced_positive` | Solution | 101 | 12.4 | 1.0× | never fires on zebra2 |
| `hypgen_scoring="most-constrained"` | Solution | 101 | 12.5 | 1.0× | inert here |

## Per-lever notes

- **`enable_singleton_writeback`** — caching a refuted singleton's `(not h)`
  at root lets later layers drop `h` in O(1). Without it the exhaustive
  search re-derives those refutations and the commitment count explodes
  (67 → 352+ dead enterings, still climbing at 90 s). The single knob a
  uniqueness-proving author must keep on.
- **`enable_symmetric_mirror`** — the native `__symmetric__` arg-swap is a
  *fast-path over* the stdlib `symmetric` rule. `zebra2` imports that rule
  (`std.algebra`), so disabling the mirror falls back to it transparently —
  same answer, same cost here. The mirror's benefit shows only on puzzles
  where the matcher cost of the rule dominates.
- **`enable_forced_positive`** — `zebra2` records `forced_positives = 0`
  with it on, so the puzzle never triggers a forced-positive cascade;
  disabling it is a no-op here. Expected to matter on puzzles with
  backbone singletons.
- **`enable_pre_branch_lookahead` / `enable_lookahead_kill_cache` /
  `enable_path_nogoods`** — pruning aids whose payoff scales with branch
  depth; `zebra2` is shallow (the human solution never branches past
  depth 1), so they save little here.

## Refresh

These numbers drift as the engine evolves. Regenerate with
`PYTHONPATH=ein.py/src .venv-pypy/bin/python utils/feature_matrix.py` and
update the provenance SHA. The *definitional* knob list lives in
[`docs/api/inference.md`](../../api/inference.md); add a row there and in
[`config.py`](../../../ein.py/src/ein/inference/config.py) for any new flag.
