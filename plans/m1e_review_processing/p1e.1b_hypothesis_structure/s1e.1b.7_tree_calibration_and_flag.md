# S1e.1b.7 — Why the tree wins on `zebra2-minus-15`, and the flag that replaces `EIN_TRAVERSAL`

**Phase:** [P1e.1b](README.md) (The structure of the hypothesis set)
**Estimate:** 3 days — 2 for the measurement, 1 for the flag.
**Depends on:** [S1e.1b.3](s1e.1b.3_the_restricted_join.md) for the third task
(the comparison arm needs the restricted join landed). T1 and T2 depend on
nothing and are **most useful before** S1e.1b.3, because what they produce is
this phase's target number. The flag depends on
[S1e.2.1](../p1e.2_high/s1e.2.1_correctness.md) T3 — see § The flag may not
ship first.
**Blocks:** nothing. It calibrates the phase and ships one CLI option.
**Source:** the user's instruction of 2026-08-28 — *investigate why tree search
works so good on minus-15 multi-model solution; add a CLI flag to use tree
search instead of the env var.*

## Context

M1d [S1d.10.6](../../../docs/history/m1d_satisfiability/README.md#s1d106--the-traversal)
measured this and did not explain it:

> **86 enterings, 0.07 s, 32 models identical fact for fact**, against the
> lattice's 17 204 592 and 1 496 s.

That is **200 053×** in enterings on `examples/zebra2-minus-15-obligations.ein`
— the corpus's under-determined entry, 32 models, 23 varying variables in
**one** coupling component
([model_set_census](../../../docs/history/m1d_satisfiability/model_set_census.md)).
It is the largest unexplained number in the repo, and it is unexplained in a
place that matters to this phase: the tree gets it by **branching on structure
the lattice has and is not told about**, which is this phase's whole thesis
arriving from the other direction.

Two arithmetic observations, free, and both shape the investigation:

- **The wall ratio is not the entering ratio.** 1 496 s / 17 204 592 = **87 µs**
  per lattice entering; 0.07 s / 86 = **814 µs** per tree entering. A tree
  entering costs roughly **9×** a lattice one — it re-generates — so the win is
  entirely in the *count*, and any change that buys enterings at the cost of
  per-entering work has to clear that bar.
- **The two published statements of this measurement disagree.**
  `CLAUDE.md` says 0.083 s where the M1d README says 0.07 s. The re-take
  settles it, and the drift is [MA-M2](../README.md#the-findings)'s class at a
  fourth site.

## The five hypotheses

The stage exists to attribute the 200 053×, not to admire it. Each row is
measurable, and the last two are the ones that would *reduce* the number.

| | hypothesis | how it is measured |
|---|---|---|
| **H1 — powerset against product** | the lattice enumerates `Σₖ C(alive, k)` subsets of a fixed `alive`; the tree walks one owed instance's alternatives and recurses, which is a **product of small sets** | compare `Σₖ C(alive, k)` against `Π nᵢ` from the `rung`/`hyp` streams. S1d.10.1 already found the lattice's enterings are *exactly* `Σₖ C(alive, k)` on 25 of 49 entries — is this one of them? |
| **H2 — re-generation cadence** | the lattice recomputes `alive` once per **layer barrier**; the tree recomputes it at **every node**, so it sees each commitment's consequences immediately and the lattice pays for stale candidates for a whole layer | count candidates the lattice entered that the tree's post-commitment `alive` would not have proposed |
| **H3 — intra-group pairs** | most of what the lattice joins is a set holding two alternatives of **one** instance — impossible by construction, forked anyway | the share of joined candidates whose members share a group. **This is the share [S1e.1b.3](s1e.1b.3_the_restricted_join.md) recovers without changing the traversal**, and it is the phase's own number |
| **H4 — the tree answers a weaker question** | it terminates by *discharge* and reports `exhausted = false`; the lattice's 17.2 M includes proving there is nothing else | how many lattice enterings happen **after** the 32nd model is recorded. If it is most of them, the headline compares an answer with a proof |
| **H5 — not learning** | the tree emits no no-good and no writeback at all ([CO-H3](../README.md#the-findings)(b)), and the lattice's learned clauses removed **1.4 %** corpus-wide ([layer_census](../../../docs/history/m1d_satisfiability/layer_census.md)) | already answered by those two facts; the stage states it so the reader stops looking there |

**H4 is the one to run first**, because it is the only one that could make the
headline number smaller rather than explain it — and a phase that adopts a
target it has not audited adopts the wrong one.

## The flag may not ship first

Promoting the traversal from an environment variable to a CLI option makes it a
**discoverable, documented surface**, and three of its defects are still open:

- `--traversal tree` would ignore `-n` — `tree_node` consults only
  `check_budget`, so it records the entire tree while being asked for one
  model ([CO-H3](../README.md#the-findings)(a));
- its `Contradiction` read-out prints *refuted so far (0 facts)* over an empty
  dead list (CO-H3(b));
- and what a tree reports where a lattice reports layers is still
  [T1d.10.6.4](../../../docs/history/m1d_satisfiability/README.md#s1d106--the-traversal)'s
  open question — `layers_explored` carries the deepest node, a different
  quantity wearing the same name.

Shipping the flag before those land is exactly
[Q-M1e.5](../open_questions.md#q-m1e5--is-experimental-a-licence-to-ship-a-lying-surface)
— *is "experimental" a licence to ship a surface whose read-out is false?* —
answered in the affirmative by accident. **So T3 of this stage runs after
[S1e.2.1](../p1e.2_high/s1e.2.1_correctness.md) T3**, and if that stage's
recommendation holds (*refuse*: the tree's `Contradiction` declines to print a
core rather than printing an empty one), the flag ships on top of a surface
that has stopped lying.

### What the flag is, and what it is not

**`--traversal {lattice,tree}`, a CLI option, default `lattice`.**

It is **not** a `(config …)` field, and the reason is the one already written
for `EIN_OBLIGATION_CHOICE`: `SolverConfig` is rendered into the KB-shape
digest, so a knob added there re-blesses every shape golden in the corpus. The
traversal is a *how*, not a *what*, and it belongs with the 52 CLI options.

`EIN_TRAVERSAL` stays, at the env tier, which is what the documented
precedence — CLI over env over `(config …)` — already means. Two arguments and
the stage picks in writing: keeping it costs one row in
[`configuration.md`](../../../docs/kernel/configuration.md)'s `EIN_*` census
and keeps every existing `EIN_TRAVERSAL=tree` invocation working (`CLAUDE.md`,
the M1d record, the test harness); removing it is a cleaner surface and a
breaking change to documents this milestone does not otherwise touch.
**Recommended: keep it**, and let the flag be what a reader finds first.

## Acceptance

- **The 200 053× is attributed**, per hypothesis, with a number each and a
  named remainder. *"We do not know what the last N % is"* is an acceptable
  line; *"the tree is faster because it is depth-first"* is not.
- **H4 has an answer before the phase quotes the number again.** If most of
  the lattice's enterings post-date the 32nd model, every later citation of
  the headline says so in the same sentence.
- **H3's share is stated as this phase's recoverable win**, and
  [S1e.1b.3](s1e.1b.3_the_restricted_join.md)'s measured delta is compared
  against it. The two agreeing is the phase's best evidence that its structure
  is the tree's structure, derived instead of walked.
- **The measurement is re-takable and its cost is stated.** The lattice arm is
  **~25 minutes** and narrates tens of millions of events; like
  `layer_census.py` it writes `--events` to a FIFO, and it is not in the gate.
  No fourth copy of the two-config diff — it extends `layer_census.py` or
  `model_set_census.py` ([AR-M1](../README.md#the-findings), and
  [D7](../p1e.1_open_questions/s1e.1.1_search_soundness_probes/d7_the_diff_instrument.md)
  is where that was last argued).
- **`--traversal {lattice,tree}` exists**, is in `--help`, is in
  `configuration.md`'s CLI table with its *does it change the answer* column
  filled (**yes** — it changes `exhausted` and can change `k`), and
  `ein-cli/tests/config_reference.rs` passes with the option count moved from
  52 to 53.
- **Not one answer moves** on the default path: every corpus entry's model set
  identical fact for fact with the flag absent, which is the phase's standing
  acceptance.

## Tasks

### Task T1e.1b.7.1 — Re-take the headline, and settle the two numbers

Half a day. Re-run both traversals on `zebra2-minus-15-obligations.ein`,
record enterings, wall, RSS and the model set, and state which of 0.07 s /
0.083 s was right — or that neither is, on today's machine, with
`utils/bench_env.sh` output beside it. Fix the loser wherever it is written.

### Task T1e.1b.7.2 — Attribute the difference

One day, the five hypotheses above, **H4 first**. The instrument is the event
stream both traversals already emit; the output is a table with a row per
hypothesis and a named remainder, filed as
`plans/m1e_review_processing/p1e.1b_hypothesis_structure/traversal_calibration.md`
while M1e is unshipped and moved to `docs/history/` with the milestone.

### Task T1e.1b.7.3 — Compare against the restricted join

Half a day, **after S1e.1b.3**. Re-run the lattice arm with the restricted
join on: how much of H3's share did it actually remove, and is what remains
H1 (the shape of the walk) or H2 (the cadence)? If it is H2, that is a finding
worth its own id — *the lattice recomputes `alive` once per layer and could
recompute it per entering* — and it is not this phase's to take.

### Task T1e.1b.7.4 — The flag

One day, **after S1e.2.1 T3**. Add `--traversal {lattice,tree}` to `ein-cli`,
default `lattice`, CLI tier above `EIN_TRAVERSAL`; update `--help`,
[`configuration.md`](../../../docs/kernel/configuration.md) (the CLI table, the
`EIN_*` census row, and the precedence paragraph), and
`ein-cli/tests/config_reference.rs`'s counts. Add the corpus cell that
exercises it — one entry, `traversal-tree`, on the obligations fixture — so the
surface has a reader in `cargo test` rather than only in a document.

## Notes

**Why this belongs to P1e.1b and not to P1e.2.** P1e.2 owns the tree's
*defects*; this stage owns the tree as **evidence**. The 200 053× is the best
existing statement of what structural branching is worth, and this phase's
claim is that the same structure can be handed to the lattice as a restriction
rather than walked by a second traversal. Calibrating against the tree is how
the phase learns whether its ladder is worth climbing — and if H4 eats most of
the ratio, that is the phase discovering early that its target was smaller
than advertised, which is cheaper here than at S1e.1b.5.
