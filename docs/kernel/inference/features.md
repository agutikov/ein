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
disabling *any single* lever still finds the correct unique answer in ~1.3 s.
The levers earn their keep in **exhaustive** search (proving uniqueness /
unsatisfiability), where two matter:

- **`enable_singleton_writeback` is load-bearing for exhaustive solves.**
  Without it, exhaustive `zebra2` blows up (33× the commitments explored)
  and does **not** finish within a 90 s budget. Keep it on.
- **`enable_fail_fast_fork` is the one plain speed knob** (S1.9.E23): it
  costs nothing to leave on, changes nothing about *what* is found (same
  verdict, same 101 enterings, same 67 deaths), and roughly halves the
  exhaustive wall-clock — the only lever here whose whole effect is price
  per branch rather than number of branches.
- Every other lever is ≤1.1× on `zebra2`, and two are effectively inert on
  this puzzle (`enable_forced_positive` never fires; `enable_symmetric_mirror`
  has a transparent rule fallback). They may matter more on larger or
  differently-shaped puzzles — re-measure with the harness below.

**No single lever is *correctness*-load-bearing on `zebra2`**: every
flag-off run that terminated returned the identical solution.

## Method

Measured by [`utils/feature_matrix.py`](../../../utils/feature_matrix.py)
(re-run to regenerate; it writes the raw per-cell artifact to
`utils/feature_matrix_results.json`, which is untracked — the tables below
are the committed record).
Each cell solves `examples/zebra2.ein` in a fresh PyPy subprocess with one
lever flipped off the all-on baseline, in two modes:

- **fast** — `stop_after=1` (the shipped default; stops at the first
  complete model), 30 s budget.
- **exhaustive** — `stop_after=None` (explores the whole commitment
  lattice; a disabled prune shows its full blow-up), 90 s budget.

A cell exceeding its budget returns an `Aborted` verdict — the
"won't-finish-if-off" sentinel (`∞`). Counts are `MonotonicStats`
enterings; `×base` is wall-time vs the all-on baseline.

*Provenance: re-measured 2026-08-17 on `ein` at `b17e1f5` + the S1.9.E23
fail-fast fork saturation (which is why every absolute number here is well
below the 2026-07 run it replaces); PyPy (`.venv-pypy`); single-run,
machine-specific — read the **factors**, not the absolute seconds.*

## Fast path (`stop_after=1`) — robust

Every lever-off run matches the baseline: **Solution, k=1, correct answer,
~1.3 s**.

| lever off | verdict | enterings | wall (s) | ×base |
|-----------|---------|-----------|----------|-------|
| *(baseline — all on)* | Solution | 11 | 1.24 | 1.0× |
| `enable_fail_fast_fork` | Solution | 11 | 1.66 | 1.3× |
| `lattice_order="score-sum"` | Solution | 13 | 1.34 | 1.1× |
| `enable_symmetric_mirror` | Solution | 11 | 1.29 | 1.0× |
| `hypgen_scoring="most-constrained"` | Solution | 11 | 1.27 | 1.0× |
| `enable_singleton_writeback` | Solution | 11 | 1.27 | 1.0× |
| `enable_lookahead_kill_cache` | Solution | 11 | 1.26 | 1.0× |
| `enable_path_nogoods` | Solution | 11 | 1.26 | 1.0× |
| `enable_forced_positive` | Solution | 11 | 1.26 | 1.0× |
| `enable_pre_branch_lookahead` | Solution | 11 | 1.10 | 0.9× |

## Exhaustive (`stop_after=None`) — where the levers bite

Baseline: Solution, k=1, **101 enterings (67 dead), 3.92 s**.

| lever off | verdict | enterings | wall (s) | ×base | note |
|-----------|---------|-----------|----------|-------|------|
| `enable_singleton_writeback` | **Aborted** | **3336+** | **≥90 (∞)** | **≥23×** | **load-bearing** — does not finish |
| `enable_fail_fast_fork` | Solution | 101 | 7.57 | 1.9× | pure price-per-branch (see below) |
| `lattice_order="score-sum"` | Solution | 134 | 4.00 | 1.0× | explores 33 more sets for the same wall |
| `hypgen_scoring="most-constrained"` | Solution | 101 | 3.93 | 1.0× | inert here |
| `enable_path_nogoods` | Solution | 101 | 3.92 | 1.0× | inert here |
| `enable_symmetric_mirror` | Solution | 101 | 3.65 | 0.9× | rule fallback (see below) |
| `enable_lookahead_kill_cache` | Solution | 101 | 3.58 | 0.9× | inert here |
| `enable_forced_positive` | Solution | 101 | 3.58 | 0.9× | never fires on zebra2 |
| `enable_pre_branch_lookahead` | Solution | 111 | 3.52 | 0.9× | 10 extra deaths, cheaper than the lookahead that prevents them |

## Per-lever notes

- **`enable_singleton_writeback`** — caching a refuted singleton's `(not h)`
  at root lets later layers drop `h` in O(1). Without it the exhaustive
  search re-derives those refutations and the commitment count explodes
  (101 → 3336+ enterings, still climbing at 90 s). The single knob a
  uniqueness-proving author must keep on.
- **`enable_fail_fast_fork`** — stops a fork's saturation at the firing that
  makes it inconsistent instead of running to quiescence and only then
  scanning. Unique among these levers in changing *nothing* about the
  search: identical enterings, deaths, solutions and clauses, because the
  KB is append-only and a fork inconsistent at firing *n* is inconsistent at
  the fixpoint. On zebra2 the clash lands after ~320 of ~2790 firings, so
  what it drops is ~88 % of every dying fork's saturation — 1.9× here, and
  2.3–2.4× in a standalone fresh-process A/B at `max_set_size=5` (8.5 s →
  3.7 s). The fast path gains less (1.3×) simply because it enters only 2
  dead forks before it stops.
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
  depth 1), so they save little here. The lookahead now measures *slightly
  negative* on the exhaustive run (0.9×, at 10 extra deaths): it pays a
  one-step rule simulation per candidate to avoid forks that fail-fast has
  made cheap. Not a reason to flip the default — the lookahead's cost is
  bounded and its benefit grows with depth, where an unpruned fork is a
  whole subtree — but a shape to re-measure on a deeper puzzle.

## Refresh

These numbers drift as the engine evolves. Regenerate with
`PYTHONPATH=ein.py/src .venv-pypy/bin/python utils/feature_matrix.py` and
update the provenance SHA. The *definitional* knob list lives in
[`docs/api/inference.md`](../../api/inference.md); add a row there and in
[`config.py`](../../../ein.py/src/ein/inference/config.py) for any new flag.
