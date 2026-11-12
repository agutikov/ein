# S1f.6.21 — The gate reduced: big graphs skipped, and no counters step

**Phase:** [P1f.6](README.md)
**Status:** planned 2026-11-12, on the user's replies of that day. Not started.
**Depends on:** nothing. It runs first of the stages planned on 2026-11-12
([`replies_2026-11-12.md` § The order](replies_2026-11-12.md#the-order)), since every later stage runs the
gate it changes.

## The rulings

The user, 2026-11-12, on the three defaults taken for the per-commit drawing test
([reply 45](questions.md#q45)):

> don't test graphviz itself - that is the main point here. just skip big graphs, run tests only on small

and on the gate step that builds `ein-infer` a second time with the work counters for one test
([reply 46](questions.md#q46), option (b), *no extra gate step: the static check of the rules' text alone, and
the measured test run by hand*):

> (b) - we don't need this proportional gate

So two defaults of the phase are overruled:
[Q-M1f.147](../open_questions.md#q-m1f147--the-dot-sweep-and-a-view-no-layout-finishes) and
[Q-M1f.153](../open_questions.md#q-m1f153--the-dot-sweeps-bound-under-the-smallest-table) (a view past the
bound checked under `fdp` stopped after one iteration), and
[Q-M1f.150](../open_questions.md#q-m1f150--the-counter-and-how-the-measured-test-is-in-the-gate) (the counters
step). [Q-M1f.161](../open_questions.md#q-m1f161--the-dot-sweep-several-files-at-a-time), the corpus files
shared among the cores, changed nothing the test holds and is not overruled.

## What it changes

**1. `ein-render/tests/dot_wellformed.rs` skips a view past the edge bound.** Today a view of more than 1 000
edges is put to Graphviz under `fdp` stopped after one iteration (`questions.md` 45). After this stage such a
view is **not put to Graphviz at all**. The bound stays 1 000 (no view the corpus had before the tables is
past it: the largest has 361 edges, `questions.md` 45). The test reports how many views it skipped, so a skip
is visible in its output and a bound that starts skipping a small view is noticed.

What is given up, stated on the page and not argued away: nothing checks that the DOT text of a skipped view
is well-formed. The writers are the same functions that write the small views, which is the reason the user's
ruling is safe to take; the stage does not add a parser of its own to make up for it, since *"don't test
graphviz itself"* is about what the gate spends its time on.

**2. The `arith_batch` counters step leaves the gate.** `cargo test -p ein-infer --features counters --test
arith_batch` is removed from `run_tests.sh`, from `.github/workflows/per-commit.yml` and from
`ein-cli/tests/gate_steps.rs`'s list. The test file, the `counters` feature and the bounds stay as they are,
and the command is run by hand by a stage that changes a rule of the calculator. The static half stays in the
gate: `arith_theory.rs`'s check of the rule of the theory on the module's text (*no rule joins two facts on a
number*), and its siblings for `std.num9`, `std.int` and `std.rat`.

It joins no other tier by default: not the nightly, and not `./run_tests.sh --dev-host`. If the user wants it
on the dev host it is one line of `nightly_steps.rs`'s `DEV_HOST_ONLY`; the stage does not add it unasked.

## Tasks

1. **T1f.6.21.1 — read what uses `fdp` besides the bounded check.** S1f.5.29 made `run_tests.sh` exit 127
   where `fdp` lays out no graph, and the workflows install `libgvplugin-neato-layout8`, because KB views are
   laid out by `fdp`. If the small views still need it, the install and the exit stay; if only the bounded
   check did, they go with it, and `utils/ci_container.sh` is then part of done (what CI installs changed,
   [Q-M1f.77](../open_questions.md#q-m1f77--when-is-utilsci_containersh-part-of-done)). The task's output is
   which of the two holds, with the grep that shows it.
2. **T1f.6.21.2 — the skip.** `dot_wellformed.rs`: a view past the bound is counted and skipped; the bounded
   `fdp` branch is deleted; the count of skipped views is printed and pinned (a number with its command, per
   the repo's rule on exact counts). The sharing among the cores is kept unless the measurement of the same
   task shows the sweep no longer needs it, in which case it stays anyway: it changes nothing the test holds.
3. **T1f.6.21.3 — the step removed.** `run_tests.sh`, `per-commit.yml`, `gate_steps.rs`. The header comment of
   `arith_batch.rs` says how it is run and that no tier runs it.
4. **T1f.6.21.4 — the pages restated,** in the same commit as the change each describes:
   - `AGENTS.md`: § Running the gate's command block and its paragraph on the second run of one test; the
     S1f.6.2, S1f.6.3 and S1f.6.5 paragraphs where they say *under `fdp` stopped after one iteration* and *a
     gate step of its own*;
   - [`stdlib/README.md`](../../../stdlib/README.md), where it says a batch's cost is held per commit;
   - [`tests/README.md`](../../../tests/README.md), where it names the counters step;
   - [`04_dot_rendering.md`](../../../docs/kernel/ir/03-ein-lang/04_dot_rendering.md), where it describes the
     bounded layout;
   - this phase's README, § Acceptance, whose line *a batch of requests costs in proportion to its count, by
     a deterministic measure of the matcher's work* becomes *held by a test run by hand*;
   - `open_questions.md`: Q-M1f.147, .150 and .153 marked ruled, with the user's words.
5. **T1f.6.21.5 — E-10's row** of [`findings.md`](findings.md) restated: no view of a library of facts is
   drawn *or checked*; what a view should draw of a table stays M1g's
   [S1g.6.3](../../m1g_logic/p1g.6_engine_requirements/s1g.6.3_seeing_nested_facts.md) (row 60 of
   `questions.md`).

## Acceptance

- `./run_tests.sh` runs no `--features counters` build and puts no view past the bound to Graphviz;
  `gate_steps.rs` holds the script's list to `per-commit.yml`'s as before.
- The skipped views are counted in the test's output.
- `cargo test -p ein-infer --features counters --test arith_batch` passes when run by hand on the stage's
  commit.
- No page describes the bounded `fdp` check or the counters step as live.

## What it must not do

- **Not change a DOT writer.** No view's bytes move; `corpus_shapes.md5` is unmoved.
- **Not delete `arith_batch.rs` or the `counters` feature.** S1f.6.12's build uses both, with E-27's second
  measure.
- **Not weaken the static check** of the rule of the theory.
- **Not touch the nightly's steps** beyond what `nightly_steps.rs` requires to stay consistent.

## The standing rules

Findings go to the phase's [`findings.md`](findings.md) under the next free id; a question to
[`open_questions.md`](../open_questions.md) under the next free id. The gate is `./run_tests.sh`, then
`./run_tests.sh --nightly` after the code commit. `utils/ci_container.sh` only if T1 finds that what CI
installs changes. No answer of the engine is touched, so no register row is expected; if one is met it is a
row in `corpus/wrong_answers.toml`, filed `open` before any fix.
