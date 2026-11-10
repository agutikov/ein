# S1f.5.29 — The review bundle's smoke on the runner's Graphviz: the per-commit job red since `00c4ed34`

**Phase:** [P1f.5](README.md) (The no-owner items)
**Status:** specified 2026-11-10 on the user's instruction of that day ([§ The instruction](#the-instruction)). Not
started.
**Depends on:** nothing in code. It runs after [S1f.5.21](s1f.5.21_a_compile_error_in_eins_words.md), one stage at a
time on the main tree, and ahead of the rest of item 12 ([README § The order](README.md#the-order)), because the
per-commit CI is red until it lands.

## The instruction

The user, 2026-11-10: *"what is utils/review_bundle.py --smoke? what it does? why it is called in per-commit CI job?
why it fails? - add new stage"*.

## What `--smoke` is

[`utils/review_bundle.py`](../../../utils/review_bundle.py) builds **the review bundle** (M1e2 S1e2.1.6). It is
markdown pages under `build/review/`, one per row of
[`corpus/wrong_answers.toml`](../../../corpus/wrong_answers.toml), each carrying every artefact the row's run writes,
raw and rendered: each DOT drawn as an SVG by Graphviz, each JSON a table. It exists for the user's review of a wrong
answer. The whole bundle is a nightly step.

`--smoke` is the gate's small version of it (`cmd_smoke`), about 1.3 s:
1. **The register pass** (S1f.5.19, Q-M1f.53). Every register row, 175 today, is read through the index and the
   case pages' register-only parts, with no run.
2. **Four builds** into a temporary directory, with the debug `ein` that `cargo test` just built: the case
   `W2/forced-2`, a solve; the cases `W40/kb-save` and `W41/saturate-import`, whose runs are not a solve; and
   `examples/ein-bugs/query-goal-free-head.ein`, a program whose solve is refused, so its page must not link a dump
   it did not write.
3. **Three checks** on what was built: every SVG non-empty; **every DOT has its SVG beside it**, so a drawing
   Graphviz refused fails (added by S1f.5.19's T2, `00c4ed34`); and every link and image reference resolves
   (`doc_audit.py --links --check`).

**Why it is per commit.** The bundle is generated and gitignored, so nothing else notices it break until the
review that needs it. That happened: the nightly's whole bundle stopped on W40's and W41's row shape at `fc53818f`
and stayed red behind a green gate until S1f.5.19. The per-commit step is in both lists, `run_tests.sh` and
`.github/workflows/per-commit.yml` (`gate_steps.rs` diffs them).

## The case: why it fails

The per-commit job on GitHub has been red on both commits pushed since `00c4ed34`:

| run | commit | failed step | finding |
|---|---|---|---|
| 42676103307, 2026-11-09 | `00c4ed34` (T1f.5.19.2) | `python3 utils/review_bundle.py --smoke` | 7 DOTs refused, exit 1 |
| 42766075844, 2026-11-10 | `f454de3b` (T1f.5.14.8) | the same | the same 7 |

(`gh run list --workflow per-commit.yml`; `gh run view <id> --log-failed`, 2026-11-10.) Every other step was green,
`cargo test --workspace` and `dot_wellformed` among them. The register pass read 175 rows, 0 failed, and all four
builds succeeded. What failed is the check `00c4ed34` added:

```
error: Graphviz refused W2/forced-2/runs/now/img/root/01_kb.dot
error: Graphviz refused W2/forced-2/runs/now/img/trace/block0001.dot
error: Graphviz refused W2/forced-2/runs/now/img/models/01/01_kb.dot
error: Graphviz refused programs/examples/ein-bugs/query-goal-free-head/runs/run/img/root/01_kb.dot
error: Graphviz refused W41/saturate-import/runs/now/img/context-root/01_kb.dot
error: Graphviz refused W41/saturate-import/runs/now/img/saturated/01_kb.dot
error: Graphviz refused W40/kb-save/runs/now/img/context-root/01_kb.dot
```

Six KB views (`01_kb.dot`) and one trace block. Locally all seven draw, with Graphviz 16.0.0 (`dot -V`, 2026-11-10),
so `./run_tests.sh` was green on both commits and S1f.5.19's records say *all 175 cases, 0 failed*. The runner is
Ubuntu 26.04, which installs the **x86-64-v3 build** of Graphviz 14.1.2. S1f.5.17 found that build's
`libpathplan` contracts its orientation test into a fused multiply-add and its spline router then refuses graphs the
baseline build draws. It fixed the overlay and Levi digraphs view-side (`ranksep=0.55`,
`dot_util::POLYGON_RANKSEP`) and added [`utils/ci_container.sh`](../../../utils/ci_container.sh) to run the gate in a
container built like the runner. S1f.5.19's gate was not run there. The cause is presumably the same, in two views
S1f.5.17 did not cover, but the smoke does not print what Graphviz said, so that is unconfirmed until T1.

The GitHub nightly has not run the whole bundle since `00c4ed34`: its last run, 42605782048 of 2026-11-09, failed at
`review_bundle.py` on W40's and W41's row shape, which S1f.5.19 fixed. The whole bundle draws many more KB views
and trace blocks, so it should be expected to fail the same way.

## What it changes

- **The cause, confirmed, not presumed.** Each of the seven DOTs is drawn on the runner's Graphviz build, with its
  stderr kept.
- **The fix, view-side**, as S1f.5.17's was: the attribute or shape in the renderer that writes each view
  (`ein-render`'s KB view and trace-block writers, or the bundle's own DOT if it writes one) that the runner's
  build routes, chosen by drawing the views on both builds. No answer moves. A DOT golden that moves
  (`corpus_shapes.md5`, `golden_dot`) is named before it is re-blessed. Nothing is sent upstream (the user's
  ruling of 2026-11-08).
- **The smoke says why.** A refused DOT prints Graphviz's own stderr beside its path, so the next red says what
  Graphviz objected to.
- **The local gate sees what the runner sees.** Twice now a Graphviz refusal reached CI behind a green local gate
  (S1f.5.17, and here). Default, recorded as a Q-M1f row: a stage that changes a DOT writer or the bundle is not
  done until `utils/ci_container.sh` is green on its commit. The stage also writes this rule into AGENTS.md's
  *Running the gate*. The alternatives, for the row: install the baseline Graphviz build in CI, or run the smoke's
  drawings in the container from `run_tests.sh` itself.

## Tasks

### Task T1f.5.29.1 — the cause, on the runner's Graphviz

Rebuild the `utils/ci_container.sh` image: `build/` was removed on 2026-11-10, its image directory with it. In
it, build the smoke's bundle (`utils/review_bundle.py --smoke`, kept: a `--keep DIR` option or a copy of its
temporary directory) and draw each refused DOT with `dot -Tsvg`, keeping Graphviz's stderr. Draw the same DOTs on
the local Graphviz and on the container's `--variant none` baseline. Find the smallest change to each view that
the v3 build routes. Then run the whole bundle (`utils/review_bundle.py`, every row) in the container, to list
every DOT the nightly will be refused, not only the smoke's seven.

### Task T1f.5.29.2 — the fix, and the smoke's message

The views fixed in `ein-render`, the goldens named and re-blessed, `--smoke` printing Graphviz's stderr per refused
DOT. `./run_tests.sh` green locally, and `utils/ci_container.sh` green on the commit. The commit says both.

### Task T1f.5.29.3 — the rule, and the pages

AGENTS.md's *Running the gate* (when `ci_container.sh` is required), `utils/review_bundle.py`'s docstring (what the
smoke checks and prints), S1f.5.19's record (a line: its *0 failed* was local), the stage's records.

## Acceptance

- `utils/review_bundle.py --smoke` passes in `utils/ci_container.sh`'s default image (the runner's x86-64-v3
  Graphviz 14.1.2) and locally, and so does the whole bundle in that image.
- A refused DOT's line carries Graphviz's stderr.
- The gate is green locally and in `utils/ci_container.sh` on the stage's commit. The per-commit job on GitHub is
  green on the user's next push, and a note records that run once it exists.
- No answer, register row or golden moved but those named.
