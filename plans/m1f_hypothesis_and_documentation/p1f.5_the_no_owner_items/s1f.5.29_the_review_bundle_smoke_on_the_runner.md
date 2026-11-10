# S1f.5.29 — The review bundle's smoke on the runner's Graphviz: the per-commit job red since `00c4ed34`

**Phase:** [P1f.5](README.md) (The no-owner items)
**Status:** specified 2026-11-10 on the user's instruction of that day ([§ The instruction](#the-instruction)).
**Done 2026-11-10**, in the working tree for the coordinator's commit: T1f.5.29.1, the cause, which was not S1f.5.17's
([§ T1 record](#t1-record-2026-11-10)): the runner's Graphviz has no `fdp` layout, a package `apt-get install graphviz`
does not pull on Ubuntu 26.04, so every KB view, which suggests `fdp`, was refused; T1f.5.29.2, the fix in CI's
provisioning and the bundle's check ([§ T2 record](#t2-record-2026-11-10)); T1f.5.29.3, the rule and the pages
([§ T3 record](#t3-record-2026-11-10)). Three choices taken by default:
[Q-M1f.77](../open_questions.md#q-m1f77--when-is-utilsci_containersh-part-of-done) (the stage's),
[Q-M1f.78](../open_questions.md#q-m1f78--where-the-runners-missing-fdp-is-fixed) and
[Q-M1f.79](../open_questions.md#q-m1f79--does-a-refused-dot-fail-the-whole-bundle) (the coordinator's). Two runs on
the commit remain ([§ Acceptance, re-taken](#acceptance-re-taken-2026-11-10)): `utils/ci_container.sh`, the
coordinator's, and the per-commit job on the user's next push.
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

**Confirmed by T1, 2026-11-10: the cause was not S1f.5.17's.** The two paragraphs above are the record of what the
stage presumed when it was written. Drawn on the runner's build with its stderr kept, each refused DOT says *There is
no layout engine support for "fdp"*: every KB view suggests `fdp` (`kb_dot::to_dot`), and Ubuntu 26.04's `graphviz`
links `fdp` to `dot` without the plugin behind it, `libgvplugin-neato-layout8`, a package it neither depends on nor
recommends. Both 26.04 builds refuse the same DOTs, and no DOT is at fault. The whole bundle does not fail the same
way: it exits 0 with 612 KB views and trace blocks refused, a refusal being a line on its page, so the GitHub nightly's
bundle step had drawn no KB view since 2026-11-06 and passed ([§ T1 record](#t1-record-2026-11-10)). The coordinator took
the fix in CI's provisioning, not in a view ([Q-M1f.78](../open_questions.md#q-m1f78--where-the-runners-missing-fdp-is-fixed)),
and a refused DOT now fails the whole bundle too
([Q-M1f.79](../open_questions.md#q-m1f79--does-a-refused-dot-fail-the-whole-bundle)).

## What it changes

*As specified, before T1. What T1 found moved the fix from a view to CI's provisioning (Q-M1f.78), and
added the whole bundle's check (Q-M1f.79): [§ T2 record](#t2-record-2026-11-10).*

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

## T1 record, 2026-11-10

T1f.5.29.1, at `28da7eb5`. **The cause is not the one the stage presumed, and the stage stopped here for a choice**
([§ What a fix can be](#what-a-fix-can-be)), which the coordinator took the same day: (a)
([Q-M1f.78](../open_questions.md#q-m1f78--where-the-runners-missing-fdp-is-fixed)). Logs and probes are under
`build/s1f.5.29/` (gitignored), the probes under `build/s1f.5.29/tools/`, and each drawing's outcome in its `m.*.tsv`;
the bundle itself, `build/s1f.5.29/bundle-v3/`, was deleted once counted.

**Reproduced.** In `utils/ci_container.sh`'s default image, `ein-ci:ubuntu-26.04-amd64v3-rust1.97.1` (built
2026-11-08 and still present, `docker image inspect`; Ubuntu 26.04, Graphviz 14.1.2-1ubuntu1, `libpathplan4` amd64v3,
the packages the runner unpacks by its apt log, `gh run view 42766075844 --log`), `python3 utils/review_bundle.py
--smoke`, run on this checkout mounted at its own path with the host's debug `ein` (it needs GLIBC 2.34 and the image
has 2.43), exits 1 on **six** DOTs (`build/s1f.5.29/smoke-v3-HEAD.log`). They are the CI log's seven but
`query-goal-free-head`'s root KB view: since `4e3c0c9d` (S1f.5.21, W64) that program is refused at load, so `ein render
kb` writes no view of it.

**Graphviz's own words**, the same for each of the six, each drawn as `render_svgs` draws it (the engine a `// suggested
layout:` comment names, else `dot`; `-Tsvg`) with its stderr kept (`build/s1f.5.29/tools/drawall.py`,
`build/s1f.5.29/smoke-host.v3.tsv`): exit 1, no SVG, and

```
There is no layout engine support for "fdp"
 Use one of: dot
```

**The cause: the runner's Graphviz has no `fdp` layout.** Every KB view `ein-render` writes (`kb_dot::to_dot`, since
S1a.5.1) carries `// suggested layout: fdp`, the trace block's `digraph state` among them, and the bundle lays such a DOT
out with the engine it names whenever `shutil.which` finds it (`utils/review/util.py`, `layout_for`). On Ubuntu 26.04
`/usr/bin/fdp` is a link to `dot`, but the plugin behind it, `libgvplugin_neato_layout.so.8`, is a package of its own,
`libgvplugin-neato-layout8`, which `graphviz 14.1.2-1ubuntu1` neither depends on nor recommends (`apt-cache depends
graphviz` in the image: `libcgraph8`, `libgvc7`, `libgvpr2`, `libgvplugin-gd8`, `libgvplugin-pango8`, recommends
`fonts-liberation`). So `apt-get install -y graphviz`, the workflows' line, installs `dot`'s layout and no other:
`echo 'digraph{}' | dot -Kxyz` lists *Use one of: dot* in the image, and *circo dot fdp neato nop nop1 nop2 osage
patchwork sfdp twopi* with the plugin added. On 24.04, `libgvc6` carries the plugin itself. **It is not the x86-64-v3
build**: the baseline build refuses the same DOTs, and no DOT the bundle draws today raises S1f.5.17's `shortest.c`
lines on either build, by `fdp` or by `dot`.

Every DOT the whole bundle writes, drawn on each Graphviz (`build/s1f.5.29/tools/matrix.sh`, `build/s1f.5.29/m.*.tsv`;
*refused* is an exit other than 0 or no SVG, and every other drawing printed nothing on stderr):

| Graphviz | image | `fdp` | the smoke's 31 DOTs | the whole bundle's 3 885 |
|---|---|---|---:|---:|
| 14.1.2 amd64v3, as the runner installs it | `ein-ci:ubuntu-26.04-amd64v3-rust1.97.1` | no | 6 refused | **612 refused** |
| 14.1.2 baseline | `ein-ci:26.04-amd64` (26.04 with no variant, S1f.5.17's probe image) | no | 6 refused | 612 refused |
| 14.1.2 amd64v3, `libgvplugin-neato-layout8` (amd64v3) added | `ein-ci-probe:v3-neato` (`tools/Dockerfile.neato`) | yes | 0 | **0** |
| 14.1.2 amd64v3, the `fdp` line taken out of every DOT, so `dot` lays each out | `ein-ci:ubuntu-26.04-amd64v3-rust1.97.1` | — | 0 | **0** |
| 2.42.2-9ubuntu0.1 | `ein-ci:ubuntu-24.04-rust1.97.1` | yes | 0 | 0 |
| 16.0.0 | this machine | yes | 0 | 0 |

The smoke itself, at HEAD: exit 1 in both 26.04 images (`smoke-v3-HEAD.log`, `smoke-base-HEAD.log`), exit 0 with the
plugin added (`smoke-v3neato-HEAD.log`), on 24.04 (`smoke-2404-HEAD.log`) and here (`smoke-host-HEAD.log`), 31 SVGs
and the link check exit 0 in each.

**The whole bundle in the image.** `python3 utils/review_bundle.py --out build/s1f.5.29/bundle-v3`, run in the amd64v3
image with the host's release `ein`: **191 cases, 0 failed, exit 0, 354.2 s** (`build/s1f.5.29/bundle-v3.log`). It
wrote 3 890 DOTs and 3 273 SVGs. Of the 617 DOTs with no SVG, **612 are refused, every DOT in the bundle that suggests
`fdp`**: 475 KB views and 137 trace blocks over 130 cases. The other five are lattices past `DAG_NODES_DRAWN` (1 000
nodes), which the bundle keeps as a DOT and does not lay out by design (`utils/review/run.py`): `W8/corner-house`,
`W15/equality`, `W27/past-limit-lines-minimal` and `-maximal`, and `W42/class-label-04-open`. The smoke's *every DOT
has its SVG* check would count such a lattice as refused too, though none of its four builds writes one.

**What it means beyond the smoke.**
- **The nightly is not red on it, and its bundle has drawn no KB view since the runner moved to 26.04** (2026-11-06,
  S1f.5.17's table): the whole bundle writes a refused drawing on its page as *Graphviz refused this graph* with
  Graphviz's message, and a case fails only on an exception, so the run above exits 0. The stage's expectation that
  the whole bundle *fail the same way* does not hold. S1f.5.17's in-container nightly at `76ede241`, *137 cases, 0
  failed*, ran in the image above, so its bundle drew no KB view either.
- **Nothing else checks `fdp`.** `dot_wellformed` lays every view out with `dot` (`fn graphviz`), so a KB view's
  suggested engine is exercised only by the bundle, and refused there only since `00c4ed34`'s check.
- **`utils/ci_container.sh` would have caught it**: its image lacks the plugin exactly as the runner does.

### What a fix can be

No DOT is at fault: every one of the 612 draws on the same build once a layout plugin for it is there, or once `dot`
lays it out. Three fixes, each shown above to draw all 3 885 on the runner's build:

- **(a) The runner installs the plugin.** `libgvplugin-neato-layout8` beside `graphviz` on the workflows' apt line
  (`per-commit.yml`, `nightly.yml`, and `release.yml`'s two for parity), in `utils/ci_container.sh`'s image, and in
  the two lines `gate_steps.rs` and `nightly_steps.rs` name as provisioning. No view, golden, answer or drawing moves,
  and the CI lays out what the user's bundle lays out. The cost: a 26.04 package name on the apt line, which a later
  image renames loudly (the install fails), and which 24.04 does not have, so `ci_container.sh --image ubuntu:24.04`
  installs it only where it exists. `run_tests.sh` could check for `fdp` as it checks for `dot` (exit 127), so a
  machine without it fails at the start and not at the smoke.
- **(b) The bundle falls back to `dot`** where the suggested engine does not load (one probe a run), and says so on
  the page. No golden moves, but on the runner the smoke checks a drawing the user's bundle does not make.
- **(c) The KB view stops suggesting `fdp`** (`kb_dot.rs`), so every KB view is laid out by `dot` everywhere: the
  stage's own kind of fix, a view in `ein-render`. It moves the four DOT goldens that carry the line
  (`tests/golden/dot/kb_render_to_dot.dot`, `slice_render_solution.dot`, `slice_render_state.dot`,
  `tests/golden/kb_zebra_unified.dot`), at least the 2 346 `dot[kb…]` lines of `corpus_shapes.md5` (six KB views
  × 391 files), the worked fragment in
  [`04_dot_rendering.md`](../../../docs/kernel/ir/03-ein-lang/04_dot_rendering.md), and every KB drawing the user
  reviews, from force-directed to ranked.

The stage asked for a fix of (c)'s kind on the presumption of S1f.5.17's cause. The cause is a package, so the choice
goes back to the coordinator. The agent's recommendation is (a): it is the cause, it moves nothing, and it keeps the
CI drawing what the user reviews. The smoke printing Graphviz's stderr (T2) and the `ci_container.sh` rule (T3, a
Q-M1f row) do not depend on the choice.

## T2 record, 2026-11-10

T1f.5.29.2, in the working tree on `28da7eb5`, after the coordinator took Q-M1f.78's (a) and Q-M1f.79's (a).

**CI's Graphviz has `fdp`** ([Q-M1f.78](../open_questions.md#q-m1f78--where-the-runners-missing-fdp-is-fixed)):
- `per-commit.yml`, `nightly.yml` and `release.yml` (its two lines, for parity) install `graphviz
  libgvplugin-neato-layout8`, and `gate_steps.rs`' `NOT_GATE_STEPS` and `nightly_steps.rs`' `NOT_NIGHTLY_STEPS` name
  that line as provisioning.
- `run_tests.sh` checks, after `dot`, that `fdp` lays a graph out (`echo 'graph { a }' | fdp -Tsvg`), and exits 127
  with one line where it does not: `error: Graphviz's 'fdp' lays out no graph — install its layout plugin
  (libgvplugin-neato-layout8 on Ubuntu 26.04).` Run in each image, the check exits 127 with that line in the one
  without the plugin and passes in the one with it.
- `utils/ci_container.sh`'s image installs the plugin wherever its archive has the package (24.04's has none: its
  `libgvc6` carries the plugin), and its tag carries a hash of the Dockerfile, so the image built before, which has no
  plugin, is not reused. The new one, `ein-ci:ubuntu-26.04-amd64v3-rust1.97.1-eca9ef96`, has
  `libgvplugin-neato-layout8 14.1.2-1ubuntu1` amd64v3; built, with the checkout cloned, in 105 s
  (`utils/ci_container.sh --shell`, `build/s1f.5.29/ci-container-image-build.log`). Its banner prints the layouts
  Graphviz offers: `── layouts: circo dot fdp neato nop nop1 nop2 osage patchwork sfdp twopi`. On 24.04
  (`--image ubuntu:24.04 --variant none`) the same Dockerfile builds without the package and the banner lists the same
  layouts (`build/s1f.5.29/ci-container-2404-image-build.log`).

**A refused DOT fails the bundle, with what Graphviz said**
([Q-M1f.79](../open_questions.md#q-m1f79--does-a-refused-dot-fail-the-whole-bundle)):
- `review.util.render_svgs` keeps Graphviz's stderr beside a refused DOT as `<name>.refused.txt` (the engine, its
  exit, its message), after clearing a stale SVG and note. `keep_undrawn` writes a DOT kept undrawn by design with a
  `<name>.undrawn.txt` saying why, and the one such writer, `run.py`'s lattice past `DAG_NODES_DRAWN`, uses it.
  `refused_dots` reads both: a DOT with a refusal note, or with no SVG and no undrawn note.
- `cmd_bundle`, the smoke's and the nightly's, exits 1 on a refused DOT among the cases it built, printing each path
  with Graphviz's message indented under it, and its summary line counts them (`…, 0 failed, 0 DOTs refused; …`). The
  smoke's own scan, `00c4ed34`'s, is gone: the smoke builds through `cmd_bundle`. In the image without the plugin:

  ```
  error: Graphviz refused W2/forced-2/runs/now/img/root/01_kb.dot
      fdp -Tsvg exit 1
      There is no layout engine support for "fdp"
       Use one of: dot
  ```
- `--smoke --keep DIR` builds into DIR, which must be new or empty (exit 2 otherwise), and leaves it there.

**Verified in the new image**, on the working tree with the host's `ein`, mounted at its own path (logs under
`build/s1f.5.29/`):

| run | before: the image without the plugin, at `28da7eb5` (T1) | after: `…-eca9ef96`, the working tree |
|---|---|---|
| `review_bundle.py --smoke` | exit 1, 6 DOTs refused, no message (`smoke-v3-HEAD.log`) | **exit 0**: 0 DOTs refused, 31 SVGs, link check exit 0 (`smoke-v3-plugin-T2.log`) |
| `review_bundle.py`, every register row | 191 cases, 0 failed, exit 0, and 612 DOTs refused that nothing counted (`bundle-v3.log`, `m.v3.tsv`) | **191 cases, 0 failed, 0 DOTs refused, exit 0**, 359.6 s: 3 890 DOTs and 3 885 SVGs, the five lattices past `DAG_NODES_DRAWN` each with its `.undrawn.txt` (`bundle-v3-T2.log`) |
| `doc_audit.py --links --check` on that bundle | — | 6 857 pages, 0 findings |

The check bites where the plugin is missing: in the image without it, the smoke on the working tree exits 1 printing
the six with Graphviz's message (`smoke-v3-noplugin-T2.log`), and `review_bundle.py W2` exits 1 on 6 DOTs refused
(`bundle-noplugin-W2.log`), where the whole bundle exited 0 before. Here, on Graphviz 16.0.0, the smoke exits 0
(`smoke-host-T2.log`). `build/s1f.5.29/bundle-v3-T2/` was deleted once counted.

**Nothing else moved.** No view and no `ein-render` file changed, so no DOT golden, `corpus_shapes.md5` line, answer,
exit code or register row moved: the register reads 177 fixed, 13 open and 1 not wrong (`grep '^status'
corpus/wrong_answers.toml | sort | uniq -c`, 2026-11-10). `dot_wellformed` passes in the gate below.

**The gate**: green, 16 min 36 s (`./run_tests.sh`, `build/gate-s1f.5.29.log`): the new `fdp` check passed, 1 004
tests and 0 failed, `dot_wellformed` among them, and the smoke read 191 rows and built its four with 0 DOTs refused, 31
SVGs, link check exit 0. The workflows' comment lines and pages edited after it started were re-checked on the final
tree: `gate_steps` and `nightly_steps` pass, `doc_audit.py --links --check` over the gate's roots finds 0 in 399 pages.

## T3 record, 2026-11-10

T1f.5.29.3:
- [Q-M1f.77](../open_questions.md#q-m1f77--when-is-utilsci_containersh-part-of-done), the stage's default: a stage
  that changes a DOT writer, the review bundle or what CI installs is not done until `utils/ci_container.sh` is green
  on its commit. [`AGENTS.md`](../../../AGENTS.md)'s *Running the gate* says so in a paragraph of its own, beside the
  Graphviz one. [Q-M1f.78](../open_questions.md#q-m1f78--where-the-runners-missing-fdp-is-fixed) and
  [Q-M1f.79](../open_questions.md#q-m1f79--does-a-refused-dot-fail-the-whole-bundle), the coordinator's, recorded
  with their alternatives.
- [`utils/review_bundle.py`](../../../utils/review_bundle.py)'s docstring: what the smoke checks, in order, `--keep`,
  and every DOT drawn in any bundle, with the plugin a KB view needs; `run_tests.sh`'s header names `fdp`;
  [`utils/README.md`](../../../utils/README.md)'s rows for the bundle and `ci_container.sh`.
- [S1f.5.19](s1f.5.19_the_review_bundle_and_runs_that_are_not_a_solve.md#t2-record-2026-11-09)'s T2 record: a line
  that its *0 failed* and the smoke's were local.
- This file: [§ The case](#the-case-why-it-fails) says what T1 confirmed, keeping the presumption as its record, and
  the records.

## Acceptance, re-taken 2026-11-10

On the working tree at `28da7eb5`, item by item:

- **`utils/review_bundle.py --smoke` passes in `utils/ci_container.sh`'s default image and locally, and so does the
  whole bundle in that image.** The default image is now `…-eca9ef96`, the runner's x86-64-v3 Graphviz 14.1.2 with the
  plugin the workflows now install: the smoke exits 0 there and here, and the whole bundle refuses 0 DOTs, exit 0
  ([§ T2 record](#t2-record-2026-11-10)). **Met.**
- **A refused DOT's line carries Graphviz's stderr.** Its path, then the engine, its exit and its message, indented.
  **Met.**
- **The gate is green locally and in `utils/ci_container.sh` on the stage's commit. The per-commit job on GitHub is
  green on the user's next push, and a note records that run once it exists.** Local: green, 16 min 36 s. On the commit:
  `utils/ci_container.sh` checks HEAD as committed, so it is the coordinator's run after the commit, and the GitHub job
  is the user's next push. **The container: met.** `utils/ci_container.sh` on `dbfd7862` exited 0 (2026-11-10,
  `build/ci-container-dbfd7862.log`). Its image is Ubuntu 26.04.1 with Graphviz 14.1.2, `libpathplan4 14.1.2-1ubuntu1
  amd64v3`, and layouts `circo dot fdp neato …`. Its smoke read 191 register rows, 0 failed, drew 31 SVGs with no
  DOT refused, and the link check exited 0. **The GitHub job: open until the user's next push.**
- **No answer, register row or golden moved but those named.** None was named and none moved. **Met.**
