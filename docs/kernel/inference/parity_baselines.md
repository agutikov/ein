# Branching demos — tree-vs-monotonic parity baselines (2026-05-28)

Recorded by [S1.5b.9](s1.5b.9_monotonic_branching_parity.md)
T1.5b.9.1 + T1.5b.9.3. Each row was captured by running both
`bench_solve.py` (tree) and `bench_monotonic.py` (monotonic) on
the laptop reference under PyPy. The parity test
[`tests/inference/monotonic/test_monotonic_parity.py`](../../../ein.py/tests/inference/monotonic/test_monotonic_parity.py)
pins these per-fixture expectations as a regression target.

## Result table

| fixture                                  | depth | tree verdict       | mono verdict | bindings              | tree wall  | mono wall | parity |
|------------------------------------------|------:|--------------------|--------------|-----------------------|-----------:|----------:|--------|
| `01_saturate_only.ein`                   |     1 | Solution           | Solution     | `c=Blue`              |  30.5 ms   |   7.1 ms  | ✓ |
| `02_one_dead_one_alive.ein`              |     5 | Solution           | Solution     | `c=Blue`              |  36.2 ms   |  12.8 ms  | ✓ |
| `03_five_hyps_one_alive.ein`             |     5 | Solution           | Solution     | `h=H5`                | 239.2 ms   | 152.7 ms  | ✓ |
| `04_two_levels.ein`                      |     5 | **Ambiguity** (2)  | **Solution** | `c=Blue` (mono)       | 395.4 ms   |  54.5 ms  | divergent — see below |
| `05_mini_zebra.ein`                      |     5 | Solution           | Solution     | `n=Bob, p=Dog`        | 313.4 ms   |  86.2 ms  | ✓ |
| `06_lookahead_on.ein`                    |     5 | Solution           | Solution     | `h=H5`                | 249.2 ms   | 212.2 ms  | ✓ |
| `07_lookahead_off.ein`                   |     5 | Solution           | Solution     | `h=H5`                | 488.4 ms   | 165.6 ms  | ✓ |
| `08_hypothesis_relation_whitelist.ein`   |     5 | Solution           | Solution     | `h=H3`                | 100.6 ms   |  53.2 ms  | ✓ |
| `09_hrule.ein`                           |     5 | Solution           | Solution     | `h=H3`                |  71.9 ms   |  34.6 ms  | ✓ |
| `10_backprop_on.ein`                     |     5 | Solution           | Solution     | `p=Dave`              | 289.4 ms   |   9.1 ms  | ✓ |
| `11_backprop_off.ein`                    |     5 | Solution           | Solution     | `p=Dave`              | 378.2 ms   |   7.5 ms  | ✓ |

Total combined wall (all 11 fixtures, sequential, PyPy):

- tree:      ~2.6 s
- monotonic: ~0.8 s

Both well inside the S1.5b.9 T1.5b.9.4 < 60 s budget; the
parametrised pytest runs both engines for all 11 fixtures in
~3.5 s on the laptop reference (3.47 s observed).

## Known divergences

### 04_two_levels.ein — Tree Ambiguity vs Monotonic Solution

By design, per
[Q1.5b.7 — monotonic-vs-lattice equivalence](open_questions.md#q15b7--termination--completeness--mode-handling)
and the algorithm spec
[`algorithm_layer_n.md` §3d.vii](algorithm_layer_n.md):

- The fixture has two satisfying branches (Blue↔H3 or Green↔H3).
- Tree depth-first search at the depth cap leaves both branches
  open, packages them as `Ambiguity(branches=(blue, green))`.
- Monotonic SOLVE mode terminates on the **first** alive
  entering whose fork satisfies `is_solved`. Lex order picks
  `(co-located Blue H3)` first; monotonic returns
  `Solution(kb=fork)` with `c=Blue`.

This is not a bug: SOLVE mode for monotonic is "give me **a**
solution"; checking uniqueness requires GAPS-mode exhaustive
search (the lattice engine, not monotonic). The tree's Ambiguity
artefact is a tree-specific shape — depth-cap halt with multiple
open satisfying leaves.

Both verdicts are equally valid under their respective contracts.
The parity test asserts this divergence explicitly rather than
either treating it as a failure or papering over it with `xfail`.

### Other expected divergences

None — the 10 uniquely-solvable demos converge to the same
binding via both engines.

## Engine-side observations

The wall-time ratio table is consistent across fixtures:

| fixture            | tree / monotonic ratio |
|--------------------|-----------------------:|
| 04_two_levels      |     7.3× (mono faster) |
| 10_backprop_on     |    31.8× (mono faster) |
| 11_backprop_off    |    50.4× (mono faster) |
|                    |                        |
| 06_lookahead_on    |     1.2× (≈ parity)    |
| 03_five_hyps_…     |     1.6× (parity-ish)  |

When the puzzle has a unique solution and lookahead pre-filters
the alive set down to one or two productive commitments,
monotonic is 1–2× faster (06, 03). When the puzzle has a wide
search tree that tree must traverse but monotonic short-circuits
via the fork-side `is_solved` check, monotonic can be
30–50× faster (10, 11).

This is the d!-redundancy point of [Q1.5b.0 / P1.5b README's
Motivation](README.md#motivation) playing out at small scale —
monotonic doesn't pay the ordering cost the tree's depth-first
search pays.

## Test surface this baseline locks in

- 11 parametrised cases in `test_monotonic_parity.py`.
- Each case asserts: tree verdict type, monotonic verdict type,
  monotonic verdict bindings, and (when both engines return
  Solution) that monotonic's binding row appears in the tree
  verdict's leaf-kb binding set.
- Combined parity-test wall: < 5 s on laptop reference.

## Cross-links

- Stage doc:
  [S1.5b.9](s1.5b.9_monotonic_branching_parity.md).
- Sibling acceptance (zebra2):
  [S1.5b.8](s1.5b.8_monotonic_acceptance.md).
- Equivalence framing:
  [Q1.5b.7 / open_questions.md](open_questions.md#q15b7--termination--completeness--mode-handling).
- Per-engine details:
  [P1.5b README — Acceptance for the phase](README.md#acceptance-for-the-phase).
