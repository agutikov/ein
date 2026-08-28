# D7 — The two-config diff instrument: build it, or borrow it a fourth time?

**Touches:** [T1e.1.1.3](README.md#task-t1e113--q5-derive-lattice02-by-hand-against-the-ruling)
(Q5, lookahead on vs off) — and, since Q6 left this stage on 2026-08-28, two
customers in another phase:
[S1e.1b.6](../../p1e.1b_hypothesis_structure/s1e.1b.6_obligations_under_hypothesis.md)
T3 (tree vs lattice on the flip probe) and
[S1e.1b.7](../../p1e.1b_hypothesis_structure/s1e.1b.7_tree_calibration_and_flag.md)
(tree vs lattice on `zebra2-minus-15`). **Three customers across two phases is
a stronger case for one script than this file was written against.**
**Cheap** — but it is the one place this stage can commit
[AR-M1](../../README.md#the-findings) rather than merely cite it.

## The instrument

*Run the same program two ways and diff the model sets, fact for fact.* Both
of this stage's remaining probes need it, and it already exists **three
times**:

| # | where | shape |
|---|---|---|
| 1 | [`utils/model_set_census.py`](../../../../utils/model_set_census.py) | reads `--json-summary`'s `verdict.solutions`, turns them into decision variables |
| 2 | [`ein-infer/tests/tree_traversal.rs`](../../../../ein.rs/crates/ein-infer/tests/tree_traversal.rs) | in-process, `BTreeSet<Vec<String>>` per run, `lattice.difference(&tree)` |
| 3 | the `--jobs` invariance sweep | 20 712 cells, byte-identical verbose streams |

The stage's own Notes already say a fourth copy is not this stage's job, and
[S1e.1b.1](../../p1e.1b_hypothesis_structure/s1e.1b.1_exclusion_census.md)
says the same for P1e.1b. Two stages declining the same work is how the fifth
copy gets written by whoever needs it third.

## What the probes actually need

Less than the three above:

- Q5: two `--json-summary` files, compare `verdict.k`, `verdict.exhausted`,
  and the sorted fact lists. Ten lines of Python.
- Q6: the same, plus a scan of `--events` for `rung` lines whose `mode` is not
  `obligations` — which is the assertion that survives even when the fact sets
  happen to agree ([D2](d2_q6_which_decline_to_construct.md)).

Neither needs decision variables, determining keys, or in-process running.

## The complication D4 just added

[D4](d4_q_m1e9_upward_closure.md) showed that a comparison of **recorded fact
sets** is sensitive to levers that do not change the model:
`-K` alters `lattice/02`'s recorded facts while `k` and the verdict hold, and
under [Q-M1e.6](../../open_questions.md#q-m1e6--what-is-a-solution-and-what-is-a-model)
the *model* is the positive part minus the positive initial KB — which nothing
computes ([Q-M1e.7](../../open_questions.md#q-m1e7--the-read-out-prints-the-solution-kb-and-calls-it-a-model)).

So whatever this stage writes has to say **which object it compares**. For
these two probes the right one is the model: strip `(not …)` and subtract the
initial KB. That is four more lines, and it is also the first implementation
of Q-M1e.6's `model` — which is an argument for putting it somewhere reusable.

## Options

| | what the stage does | consequence |
|---|---|---|
| **A — throwaway, in the stage folder** | ~15 lines of Python under `probes/`, used by T3 and T4, not installed in `utils/` | no fourth catalogued copy; the evidence lives with the plan; the next stage that needs it writes a fifth |
| **B — a real `utils/` script** | `utils/model_diff.py`, catalogued, `$EIN_BIN`, re-takable, and the first thing that computes Q-M1e.6's `model` | fixes the habit instead of citing it, and gives P1e.1b's three model-set comparisons an owner. Costs perhaps half a day and grows the stage's scope |
| **C — borrow `model_set_census.py`** | add a `--form diff` to the existing script | no new file, but it bends a *census* into a *comparison*, and that script's subject is one program's model set, not two runs of one |

**Recommended: A for this stage, and let
[S1e.3.4](../../p1e.3_medium/s1e.3.4_architecture.md) decide B.** S1e.3.4 owns
AR-M1 and is where the unify-or-diff ruling belongs; pre-empting it here means
the architecture stage inherits a decision it was meant to make. But **name
the throwaway in S1e.3.4's inputs**, so the fourth copy is on the record as
one when that stage counts them.
