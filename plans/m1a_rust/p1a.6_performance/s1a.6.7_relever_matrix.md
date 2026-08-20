# S1a.6.7 — Re-measure the lever matrix

**Phase:** P1a.6 (Performance)
**Status:** **shipped 2026-08-20** — five tasks, all five landed, and the
stage's own instrument was the first thing it had to fix. `features.md` now
carries an ein.rs column beside the ein.py one, taken the same day by the same
harness, **plus a `control` row that states what each column can resolve** —
1.2× under PyPy, 1.0× under ein.rs, which retires four of the old table's ten
conclusions. Two levers changed sign or size on a deeper puzzle and one of
them turned out not to be a prune at all. The measurements are
[baseline.md §19](baseline.md#19-s1a67-and-s1a66--the-lever-matrix-in-two-engines-and-the-fuzzer).
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

---

## What each task did

### T1a.6.7.1 — the harness drives either implementation ✅

Not by importing the engine twice: by **shelling out to both**, one fresh
process per run, with the lever delivered through a generated `(config …)`
block appended to a copy of the puzzle. `ein solve` exposes five of the ten
knobs as flags; the IR head exposes all ten, and both loaders keep the *last*
block in the file, so the two engines read the identical bytes and each run
reads its own `summary.json` back to prove the lever it names is the one that
moved. That is [Q-M1a.16](../open_questions.md#q-m1a16--how-does-the-harness-drive-the-lever-matrix)'s
option (b), and the objection recorded against it — "the corpus entry is then
not the file in `examples/`" — is about the *corpus*, not about this harness:
here both engines are handed the same generated file. Option (a), a
`--config KEY=VALUE` flag on both CLIs, remains the recommendation for the
corpus's own lever list and is **not** built by this stage.

`wall_s` keeps its meaning — root saturation + hypothesis search — read from
`--timing` rather than timed around `solve()`, so the column is comparable to
the 2026-08-17 one. `proc_s` is new and is the whole process.

**Then the harness measured itself.** Cell-by-cell, PyPy's baseline runs first
and reads ~20 % fast, and every ratio in the table is divided by it: eight
inert levers came out at a uniform 1.2× that ein.rs measured at exactly 1.0×
with identical entering counts. Runs now go **round-robin over the cells**,
and a **`control` cell** — byte-identical to `baseline`, measured last — makes
the residue visible instead of arguable.

### T1a.6.7.2 — run and cross-check ✅

72 cells over four puzzles, both engines, same machine, same day. **Every one
agrees** on the verdict, `k`, the goal bindings and twelve counters before any
timing is compared; the single exemption is `no-singleton-writeback`, where
ein.py stops on its 90 s budget at 3 358 enterings and ein.rs finishes 3 831
in 1.53 s. A pair where one side stopped on a budget is exempted rather than
reported — the two runs did different amounts of work.

That is the acceptance's "T1 check dressed as a benchmark", and it is
mechanical rather than asserted: the counters come out of `--json-summary`,
which is the surface T0/T1 read.

### T1a.6.7.3 — the lookahead, re-examined ✅ (and it is not a prune)

`zebra`: it prunes 23 of 134 commitments and pays for itself — 1.1× to turn
off, in both engines. `zebra2`: a wash, both engines, 1.0×. The 0.9× the
2026-08-17 table called "slightly negative" was inside that method's own
noise, which the control row now prices.

`branching/06` and `lattice/02` — the deep fixtures the task named — answer
something else entirely: with the lookahead off the verdict changes from
`Ambiguity` to **`Contradiction`**, identically in both engines, because
`complete(kb)` asks the hypothesis generator whether anything is undecided and
the generator's candidates are lookahead-filtered. The fast path of
`branching/06` is **448×** in ein.rs. Parked as
[F4 Q40](../../followups/f4_cross_cutting.md); both fixtures' headers, which
claimed "same verdict either way", now carry the measurement.

**The one proposed default change**, per the stage's own Notes: `lattice_order
= "score-sum"` is **0.6× on `zebra`** (62 commitments against 111) and 1.2× on
`zebra2` (134 against 101), in both engines to the digit. Written up in
`features.md` with its counter-example; **not applied**.

### T1a.6.7.4 — the narrative ✅

`features.md` is rewritten around the two columns and the control, with the
2026-08-17 and P1a.4 tables kept and *labelled* rather than overwritten — the
arc is the interesting part, and two of the old table's conclusions did not
survive a control row. `architecture_and_algorithms.md` §7 gains what the
phase did to its own premise: the ~95 % matcher share is a property of the
implementation, and on the port the answer moved three times in one phase —
matcher, then the NAF boundary at 37.7 %, then no block above 8 %.

### T1a.6.7.5 — F11, closed against numbers ✅

**D1 measured and re-parked, for the third time and in the same direction.**
The intermediate a beta-memory would materialise is now **0.45 tuples per step
entered** (47.4 before S1a.6.3, 2.21 after), with the step count flat to 0.3 %
across all three readings — the matcher takes the same decisions and looks at
100× fewer facts to take them. **D2's cost half moved away from its trigger**:
the matcher's five hot functions are about a fifth of a 47 ms `zebra -e`,
where the phase began at 66.9 %. Both entries stay open with sharper triggers;
the milestone that was F11's "most likely promotion trigger" ran, reached the
matcher, and declined both.

## What this stage did not do

- **`--config KEY=VALUE` on both CLIs** (Q-M1a.16 option (a)) — the corpus
  still exercises four levers through flags. Adding a knob to the T3 surface
  of both implementations is a CLI-surface change, not a measurement, and the
  matrix no longer needs it.
- **Any default change.** `score-sum` has the numbers and the counter-example;
  the decision is the user's.
- **Fixing F4 Q40.** Whether a performance lever may decide what counts as a
  complete model is a design question; this stage measured it and wrote it
  down.
