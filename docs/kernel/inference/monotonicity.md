# Monotone and non-monotone programs — the two kinds of region compared

**What the answer is when adding a fact can take one away.** The answer to a
program is the region R of its hypothesis lattice, the commitments whose state
is alive and discharged ([`solution_semantics.md`](solution_semantics.md)).
The lattice is the domain, and it is never itself monotone or not: what can be
are the functions on its nodes, a node's state and its status (§ 7).
That page is written for a **monotone** program, where growth never takes a
conclusion away and never adds a debt. This page sets the two cases side by
side: what each means, what causes the second, how each looks on the lattice,
how it is detected, what each property lets the engine simplify, and how to
compute R stably, provably and effectively in each. M1e2
[S1e2.1b.6](../../../plans/m1e2_correctness/p1e2.1b_non_monotone_programs/s1e2.1b.6_monotone_and_non_monotone_programs_compared.md)
wrote it, at the user's request of 2026-10-16, for a reader who needs R **as a
whole**: a complete answer with the provenance of its topology, the region's
properties studied to improve a theory, or a search over theories for one
with a single solution.

**What is ruled and what is not.** The monotone column is settled. Three cells
of the non-monotone column are **open**: what a node's state is, what is
reported when R has holes, and what a search may learn from a refutation that
rests on an `absent`. On 2026-11-02 the user deferred every decision about a
non-monotone program to
[F21](../../../plans/followups/f21_non_monotone_programs_decided.md), and
M1e2's [P1e2.1b](../../../plans/m1e2_correctness/p1e2.1b_non_monotone_programs/README.md)
detects, documents and warns (§ 8). Until a decision, the engine builds every
node by fork from root. Every other claim carries a test, a dated command, or
a theorem whose premise a test enforces
([`standard_of_proof.md`](../standard_of_proof.md)), and § 7 labels each of
its claims by that standard.

---

## 1. Where it starts: R is its node set

> **R = { C ⊆ H : sat(root + C) is alive and discharged }**,
> [`solution_semantics.md` § 2](solution_semantics.md#2-the-answer--the-region-r).

The user, 2026-10-16: *"In general non-monotonic case R is completely defined
only by full set of nodes … Let's go from it, and simplify the results if our
case allows or if it is requested explicitely."* That is exact
([§ 2.1 there](solution_semantics.md#21-in-general-r-is-its-node-set)):

- **Any subset of the cube is the region of some program.**
  [`utils/region_synth.py`](../../../utils/region_synth.py) writes a program
  for a chosen region, and the oracle gives it back node for node: 2 100
  random targets with 0 mismatches (2026-10-15, M1e2 S1e2.1a.8), and 300 per
  commit (`ein-cli/tests/region_dump.rs::a_random_region_written_as_a_program_comes_back`).
  The one target it refuses is a dead ∅ under a non-empty R, because the
  oracle builds every node from root's fixpoint. Which construction a node's
  state is defined by is open (Q-M1e2.2, F21's D2).
- **There are 2^(2ⁿ) regions**, so nothing shorter than the set describes all
  of them. The shortest exact form of one is the union of its maximal
  sub-cubes, which the oracle computes (`region.json`'s `subcubes`). It is
  small when R is regular, and it reaches 2ⁿ⁻¹ isolated points (parity).

## 2. The ladder

Each step down is licensed by a property the step above does not have, and
each makes a shorter description exact:

| step | licensed by | R is described by | a claimed R is certified by |
|---|---|---|---|
| **general** | nothing | the node set, or its maximal sub-cubes | every node classified: 2ⁿ saturations |
| **convex** | **no hole**, observed on the cube | the two lines: R = ⋃ [a … b] over minimal a ⊆ maximal b | as above: a hole is known only after the cube is enumerated |
| **monotone** | the **static check** says *monotone*, before any search (§ 5.1) | the two lines, computed from the alive down-set and N, or from N and E, without visiting 2ⁿ nodes | N and E, one saturation per member, and the static verdict |
| **shape** | which constraints the program states | restrictions only: the minimal line is {∅}. Obligations only: the maximal line is {⊤}. Both, and well-posed: every component a point | as monotone |

**Convex is the common non-monotone case in the corpus.** Of the 18 flagged
programs within the oracle's limit, 9 have no hole, and each of the 9 with
one was written to show it (`cargo run --release -p ein-infer --example
site_census`, 2026-11-02 at `4562bf70`; the holes by cause are M1e2
S1e2.1b.3's [hole census](../../../plans/m1e2_correctness/p1e2.1b_non_monotone_programs/hole_census.md)).
That says more about the corpus than about programs. **A conditional obligation makes holes as a matter of
course**: [`lattice/06_friends_tables.ein`](../../../examples/lattice/06_friends_tables.ein),
*a group that sits owes its members' seats*, the way a person writes *if you
come, you all sit*, has R of 21 isolated nodes and 166 holes. Its monotone twin
[`06_friends_tables_everyone_sits.ein`](../../../examples/lattice/06_friends_tables_everyone_sits.ein),
*everyone must sit*, has 8 seatings and no hole (Q-M1e2.7, 2026-10-16,
[`open_questions.md`](../../../plans/m1e2_correctness/open_questions.md#q-m1e27--the-answer-is-a-region-of-the-lattice)).

## 3. The comparison

| | monotone | non-monotone | evidence |
|---|---|---|---|
| **what it means** | adding a commitment never takes a conclusion away and never adds a debt | some commitment does one or the other | the definition in [`monotone.rs`](../../../ein.rs/crates/ein-infer/src/monotone.rs)'s module docs; the cube counts each breach as a violation (`region.json`'s `violations`) |
| **what causes it** | nothing: no read of the third kind (§ 5.1) | a **site**, from one of three sources: an `absent` at odd depth in a saturation rule over a relation a commitment can change; an obligation's **trigger** over one; a rule whose **activator** relation a commitment can change, so a node compiles a plan root never had | one fixture per cause, each its program's only site, each with the hole its cause makes: [`08_absent_witness_hole.ein`](../../../examples/lattice/08_absent_witness_hole.ein), [`04_obligation_trigger_hole.ein`](../../../examples/lattice/04_obligation_trigger_hole.ein), [`09_activator_hole.ein`](../../../examples/lattice/09_activator_hole.ein) (`ein-infer/tests/region.rs::each_cause_has_a_fixture_whose_cube_shows_its_hole`) |
| **what does not cause it** | | an `absent` over a static relation (zebra2's spatial reads, `region.rs::zebra2_is_monotone_by_construction`); an obligation's witness `absent` (`owed-seats`, `lattice/05`, `region.rs::monotone_programs_show_no_violation_and_the_non_monotone_three_are_flagged`); a rule that reads a stored negative of a hypothesised relation ([`complete-records-stale.ein`](../../../examples/ein-bugs/complete-records-stale.ein), monotone by the static check, no violation on its cube) | as listed |
| **how it looks** | a **band**. Dead is an up-set, drawn at the bottom; owing is a down-set among the alive nodes, near L0; R lies between, and components separate where dead and owing meet | **islands**: a node outside R between two members (a hole), joined to R above it by a repair edge; a dead node with an alive superset; a discharged node with an owing superset | § 4's drawings |
| **dead** | upward-closed | not, where a refutation rests on an `absent` (W3's `bad`) or on a plan a node compiled (`ACTIVATOR_REFUTES` in `region.rs`) | `region.json`'s `dead_not_upward_closed`. On no cube the static check calls monotone is it non-zero: 155 cubes, 0 (`site_census --json`'s rows, 2026-11-02 at `6743dd17`), and `region.rs::fact_grain_never_calls_a_violated_cube_monotone` sweeps them per commit |
| **discharged** | upward-closed | not, where a trigger grows (`lattice/04`), an `absent` takes a witness away (`lattice/08`) or a commitment compiles an obligation (`lattice/09`) | `discharged_not_upward_closed`, asserted on all three by `each_cause_has_a_fixture_whose_cube_shows_its_hole` |
| **R's shape** | convex | any subset of the cube | § 1's round trips |
| **the lines, decided** | locally: a node of R is maximal iff every edge up leaves R, minimal iff every edge down does | globally only. Across a hole the one-fact test calls a node a line node that is not one | `region.json`'s `local_max` and `local_min`. On the monotone corpus they never disagree with the cube ([`solution_semantics.md` § 2.4](solution_semantics.md#24-shape)). On [`synth/local_not_global.ein`](../../../examples/lattice/synth/local_not_global.ein), *A and B together or not at all*, ∅ is maximal by the one-fact test and {A,B} minimal, and neither is (`region.rs::across_a_hole_the_one_fact_test_misreads_the_lines`) |
| **N/E duality** | holds: the minimal line is E's alive members, the maximal line the discharged maximal sets containing no member of N | not guaranteed. It fails on the maximal side on `naf-upward-closure`, `synth/three_components` and `synth/trigger_hole`, which have holes, and on [`synth/maximal_not_top.ein`](../../../examples/lattice/synth/maximal_not_top.ein), which has none; it holds on `lattice/04`, `08` and `09` | `region.rs::the_duality_holds_without_holes_and_fails_on_the_hole`; `region.json`'s `duality` (`ein oracle FILE --dump DIR`, 2026-11-02) |
| **trivial lines** | restrictions only: min = {∅}. Obligations only: max = {⊤}, so the maximal line says nothing (ruled for the monotone case, 2026-10-16) | max ≠ {⊤} where a trigger grows: `synth/maximal_not_top.ein` states obligations and nothing dead, and its maximal line is {A,B} while ⊤ owes | `every_synthetic_program_has_the_region_it_was_written_for` |
| **learned nogoods, kill cache, singleton writeback** | sound: every superset of a dead set is dead | unsound where the refutation read a third-kind relation: W3, and `synth/three_components`' learned clauses {A,F} and {A,G} (S1e2.1a.8). The language is ruled, *forbid* ([S1e2.4.1 § 5](../../../plans/m1e2_correctness/p1e2.4_what_the_switch_left/s1e2.4.1_refutation_under_absent.md#5-the-ruling), 2026-11-02): a refutation may not rest on an `absent` over a relation a commitment can extend. **What a route may learn before that refusal ships is open**: F21's D6, measured by [S1e2.1b.4](../../../plans/m1e2_correctness/p1e2.1b_non_monotone_programs/s1e2.1b.4_refutation_resting_on_absent.md)'s [taint census](../../../plans/m1e2_correctness/p1e2.1b_non_monotone_programs/taint_census.md): a death that rests on no `absent` over D is dead in every superset on every flagged cube within the limit, once the taint follows each plan's activator, and not read off provenance alone (`region.rs::an_untainted_death_is_dead_in_every_superset`). Since the switch no route learns on a flagged program | the dead row's evidence |
| **saturation on the cube** | a **closure operator**: C ⊆ cl(C), cl(cl(C)) = cl(C), C ⊆ D ⇒ cl(C) ⊆ cl(D). R's states are closed sets, each the union of the intervals [g … cl] over its minimal generators g | cl need not be monotone. States still partition the commitments, but not as the closed sets of a closure system | `region.rs::a_monotone_verdict_makes_saturation_a_closure_operator`: every node of every monotone cube within the limit, dead ones included, and saturation moves a node on 15 of them. On `CL_NOT_MONOTONE`, flagged, cl is not monotone |
| **a node's state** | unique, the least fixpoint: built by fork from root, by the theorem; from scratch the same fact set except where an equal-priority race over a static read is re-run inside the node ([`lattice/11_fifo_gap_static`](../../../examples/lattice/11_fifo_gap_static.ein), § 7.6) (`ein-cli/tests/region_dump.rs` holds `utils/region.py`, the scratch route, to the oracle on Q-M1e2.7's seven programs) | depends on the route. On W8's [`stale-naf-fork.ein`](../../../examples/ein-bugs/wrong-answers/stale-naf-fork.ein), `{(q one)}` is dead by fork, because root derived `(p one)` under `(absent (q one))`, and in R from scratch. A parent path can differ from both (§ 7.2). **Which one R is defined over is open**: Q-M1e2.2, F21's D2, with [S1e2.1b.2](../../../plans/m1e2_correctness/p1e2.1b_non_monotone_programs/s1e2.1b.2_fork_or_scratch.md)'s evidence | § 6.4: the guard split reads scratch, and disagrees with the oracle on exactly W8's two witnesses; § 7 |
| **holes, reported** | there are none | the read-out names up to four minimal holes today (`ein solve examples/lattice/04_obligation_trigger_hole.ein`: `minimal hole {(seats Ann C1)} — owes 1`). **What is reported, a qualifier, the pieces or a refusal, is open**: Q-M1e2.7 (d), F21's D4, with [S1e2.1b.3](../../../plans/m1e2_correctness/p1e2.1b_non_monotone_programs/s1e2.1b.3_holes.md)'s evidence: every hole by cause and kind, and each option drafted on real programs ([hole census](../../../plans/m1e2_correctness/p1e2.1b_non_monotone_programs/hole_census.md)) | `region.json`'s `holes`, `minimal_holes`, `repairs` |
| **certificate** | N, E and the static verdict: polynomial in \|N\| + \|E\| | the full classification. With the guard split, each cell's guess and its N and E (§ 6.4, not built) | route 1 is held to the oracle (`region.rs::route_1_is_the_oracle_on_every_monotone_program_it_reaches`) |
| **detection** | static: [`monotone.rs`](../../../ein.rs/crates/ein-infer/src/monotone.rs), a sufficient condition read at **relation grain**, as the user ruled on 2026-11-02 ([S1e2.1b.1](../../../plans/m1e2_correctness/p1e2.1b_non_monotone_programs/s1e2.1b.1_the_site_census.md#the-question-it-puts-to-the-user)) | the static check flags a **site**; only the cube says whether it did harm | § 5 |

## 4. The drawings

Seven regions drawn by [`ein render region`](../ir/03-ein-lang/04_dot_rendering.md#region-rendering--the-cube-and-a-region-run),
each with a bare `ein solve` on its cube. Fill is the node's status, shape is
what the run did there, and border is the lines and the defects; a purple
border is a hole, and a `+fact` edge is the repair that takes it back into R.

The run is what makes the pairs comparable. On a monotone program `ein solve`
takes route 1, which grows the alive down-set and never enters a node above a
dead one: it visits 7 of 8 nodes on `branching/15`, 11 of 16 on `lattice/05`
and 25 of 32 on `merged_states`. On a flagged program it takes route 2, the
cube, and visits every node.

**Monotone.**

| program | what it shows |
|---|---|
| [`branching/15_lookahead_two_step_on.ein`](../../../examples/branching/15_lookahead_two_step_on.ein) | restrictions only: dead is an up-set, the minimal line is {∅}, and R is the band above it |
| [`lattice/05_owed_seats_bounded.ein`](../../../examples/lattice/05_owed_seats_bounded.ein) | restrictions and obligations: owing from above, dead from below, and R squeezed to two isolated points, the shape of a well-posed puzzle |
| [`lattice/synth/merged_states.ein`](../../../examples/lattice/synth/merged_states.ein) | saturation moves nodes: a `sat` arrow from each transient commitment to its closure, and 16 commitments in 8 states |

![branching/15](diagrams/monotonicity_15_lookahead_two_step_on.svg)

![lattice/05](diagrams/monotonicity_05_owed_seats_bounded.svg)

![synth/merged_states](diagrams/monotonicity_merged_states.svg)

**Non-monotone**, one per cause and one for dead holes.

| program | cause | what it shows |
|---|---|---|
| [`lattice/08_absent_witness_hole.ein`](../../../examples/lattice/08_absent_witness_hole.ein) | an `absent` | an owing hole: being allergic takes away the soup that paid Ann's meal |
| [`lattice/04_obligation_trigger_hole.ein`](../../../examples/lattice/04_obligation_trigger_hole.ein) | a trigger | an owing hole: seating Ann makes her a Diner, and a Diner owes a meal |
| [`lattice/09_activator_hole.ein`](../../../examples/lattice/09_activator_hole.ein) | an activator | the same cube as `lattice/04`, by another mechanism: opening the kitchen declares that every guest owes a meal. The picture cannot tell the causes apart; the static check can |
| [`lattice/synth/three_components.ein`](../../../examples/lattice/synth/three_components.ein) | `absent`s | the user's shape of 2026-10-15: three components, two of them with dead holes, each revived by a repair edge |

![lattice/08](diagrams/monotonicity_08_absent_witness_hole.svg)

![lattice/04](diagrams/monotonicity_04_obligation_trigger_hole.svg)

![lattice/09](diagrams/monotonicity_09_activator_hole.svg)

![synth/three_components](diagrams/monotonicity_three_components.svg)

Re-made with one command, from the repository root with `ein` on `PATH`:

```sh
for p in branching/15_lookahead_two_step_on lattice/05_owed_seats_bounded lattice/synth/merged_states \
         lattice/08_absent_witness_hole lattice/04_obligation_trigger_hole lattice/09_activator_hole lattice/synth/three_components; do
  f=examples/$p.ein; o=$(mktemp -d); d=docs/kernel/inference/diagrams/monotonicity_$(basename $p)
  ein oracle $f --dump $o/cube > /dev/null && ein solve $f --dump-lattice $o/lattice --dump-answer $o/answer > /dev/null &&
  ein render region $o/cube --visited $o/lattice --answer $o/answer > $d.dot && dot -Tsvg $d.dot -o $d.svg && rm -r $o
done
```

## 5. Detection

### 5.1 Static

[`ein_infer::monotone`](../../../ein.rs/crates/ein-infer/src/monotone.rs)
decides from the compiled plans, before any search
([`region_dump.md` § Monotonicity](region_dump.md#monotonicity--the-static-check)).
H is the relations of the candidate list, and D the relations a commitment
can change: H, closed under *a saturation plan that reads D asserts into D*.
A read of a relation outside D is **static**. A read of one in D is
**dynamic**, at monotone or non-monotone polarity by its `absent` depth and
its rule's kind. A **site** is a dynamic read at non-monotone polarity, or a
rule whose activator relation is in D. No site is the theorem's premise: dead
and discharged are then both upward-closed, so R has no hole.

The check reads at **relation grain**, as the user ruled on 2026-11-02. A
fact-grain reading, dynamic only where a candidate or a fact derived from one
can match the read's pattern, was prototyped and held to the cube, and it is
not what ships
([site census § 5](../../../plans/m1e2_correctness/p1e2.1b_non_monotone_programs/site_census.md#5-the-grain)).

### 5.2 On the cube

The check is sufficient, not necessary. The oracle (`ein oracle FILE --dump
DIR`, n ≤ 16) counts what a site did: holes, dead nodes with an alive
superset, discharged nodes with an owing superset. A flagged program whose
cube shows one is **exercised**, and one whose cube shows none is
**conservative**.

`cargo run --release -p ein-infer --example site_census`, 2026-11-02 at
`4562bf70` with M1e2 T1e2.1b.3.2's attribution, over the corpus and the
register's witnesses:

| | |
|---|---:|
| files that load and saturate | 208 of 271 |
| flagged | **26** |
| … within the limit | 18: **10 exercised**, **8 conservative** |
| … past it | 8 |
| sites | 37: 28 `absent`, 7 trigger, 2 activator |
| monotone cubes with a violation | **0** of 155 |
| flagged programs that would leave the set at fact grain | 2, `features/05` and `features/12`, both past the limit |
| a violation no site is named for | **0** |

Every conservative program is flagged over a relation it means to decide,
and never makes the site bite. The census's first take, at `4b822a8d` the same
day, had 22 flagged and 6 exercised, and its take at `6743dd17` 25 and 9. The
differences are the four fixtures banked for this page: `lattice/08`,
`lattice/09`, `synth/maximal_not_top` and, after `6743dd17`,
`synth/local_not_global`, flagged with two holes. One activator site was
exercised nowhere until
[`09_activator_hole.ein`](../../../examples/lattice/09_activator_hole.ein):
`features/01`'s is past the limit and static at fact grain.

**The census names a site for every violation.** For each minimal hole and
each violation's witness it names a cause and the site that made it: a dead
node by its refutation, an owing node by the debt new against the node of R
below it. The take at `6743dd17` named none on `lattice/08` and `lattice/09`
(*a violation no site is named for: 2*), whose owing holes come from an
`absent` that took a witness away and from an obligation a node compiled.
S1e2.1b.3 added a rule for each, and one for a refutation a node compiles; the
re-take above names `soup-unless-allergic` and `total-owed`'s activator
([hole census § 8](../../../plans/m1e2_correctness/p1e2.1b_non_monotone_programs/hole_census.md#8-the-attribution-gap-closed)).

### 5.3 The detector

Built by M1e2's
[S1e2.1b.5](../../../plans/m1e2_correctness/p1e2.1b_non_monotone_programs/s1e2.1b.5_the_detector.md),
in the form and the words the user approved on 2026-11-03
([`detector_proposal.md`](../../../plans/m1e2_correctness/p1e2.1b_non_monotone_programs/detector_proposal.md)):
[`ein_infer::detector`](../../../ein.rs/crates/ein-infer/src/detector.rs).
It runs on a program the static check flags, and only there: no option turns
it on or off, and on an unflagged program it does no work beyond the test of
the verdict. It decides nothing (§ 8): no finding chooses a node, refuses a
program or moves R.

**The static half**, read off root's plans at load, before any node: what each
site's rule concludes and which property it puts at stake (§ 7.6's table, per
site: an `absent` puts the state at stake, and `dead` with it when it refutes;
a trigger, the status; an activator, either, through the plan a node
compiles); each **cycle through an `absent`** over D, with its parity; and each
**order** finding, a rule that reads under an `absent` a relation in D another
`absent` rule concludes, at one priority (the written order decides which the
boundary admits first) or at an earlier one (the reader goes first in every
order). `ein-infer/tests/detector.rs` pins every finding on the corpus by
name: the cycles of `features/07` and `f07-fpoff` (even) and of
`d3-unsat-core` (odd), and the order findings of the same two and of
`lattice/10_fifo_gap` (§ 7.3).

The cycle rows state the general theory, **argued**: an even cycle through an
`absent` can have several states with no stale fact (two stable models), an
odd one need not have any. What the corpus shows is narrower, **measured**: on
every flagged corpus node, the odd cycle included, exactly one stale-free
state is reached from P
([`node_census.md` § 11.2](../../../plans/m1e2_correctness/p1e2.1b_non_monotone_programs/node_census.md#112-the-flagged-cubes),
`8a2dffe3`, 2026-11-02), because the boundary's admission order picks one.

**The dynamic half** says whether a site bit, with **the status and the state
apart**, because the cube reads only the status (§ 7.9):

- **the status**: the cube's violations (holes, dead nodes with an alive
  superset, discharged nodes with an owing superset), each attributed to the
  sites that made it by S1e2.1b.3's rule: *exercised*, *conservative*, or *not
  known* where the run did not read the cube;
- **the state**: every node the run computes is read for **stale facts**, a
  fact whose `absent` guard the node's own fixpoint falsifies (`node_census.md`
  § 4's check, with the engine's matcher), dead nodes included, because W8's
  stale fact is in a dead node no run lists;
- **the paths**: for each state the read-out prints of two commitments or
  more, at most three of its one-step parents `C \ {h}`, each built, `h`
  written, saturated, and compared with the state the route built. **Its reach
  is one step.** On the probe `via_hole` and on `ACTIVATOR_UNDER_ABSENT` a node
  of R is dead along a two-step chain through an alive parent, and one-step
  parents see one of the two path-dependent nodes (`node_census.md` § 11.2).
  The row says what was checked, *n parents of m printed states*, and claims
  nothing more.

**Where it is printed**: the read-out's `lattice` block on every run of a
flagged program, under every question and from `ein oracle`; `ein solve FILE
--monotone`, the static half alone; `--json-summary`'s `monotonicity` block;
the `monotonicity` event (the static half); `ein test`, beside a claim under
`-v` or a claim that failed; `region.md`, `region.json` and `ein render
region`'s label; the review bundle. On stdout at most four sites and four
stale nodes are named, then a `…` row naming `--dump-answer`, which writes
them whole.

**What it reports on the corpus**, `ein solve FILE --json-summary` on each of
the 26 flagged corpus programs (release build, 2026-11-03): the status
*exercised* on 10, *conservative* on 8, *not known* on 8 (§ 5.2's split, now
printed); stale facts on 10, five of them conservative by the cube
(`stale-naf-fork` and its `-warn` twin, `features/07`, `f07-fpoff`,
`lattice/10_fifo_gap`) and five past the limit, where the read-out once said
only *holes not known* (`features/01`: 24 754 of the 64 655 nodes the run
computed; `features/04`: 21 042 of 64 761; the three `square-unique` demos:
5 595, 5 835 and 4 876 of about 65 000); a parent path that differs on 2
(`lattice/08`, 1 of 7; `lattice/10_fifo_gap`, 1 of 2), and on
`d3-unsat-core`, outside the corpus, 3 of 34.

**Why it costs nothing on an unflagged program** is an argument, beside the
code in `detector.rs`, with its scope: no site means no negative edge over D,
no order finding, no stale fact and no path difference **on nodes built by
fork from root**, and on splits from a fixpoint that holds root's. It does not
hold from scratch: [`lattice/11_fifo_gap_static`](../../../examples/lattice/11_fifo_gap_static.ein)
(§ 7.6) is monotone by the check and holds a stale fact from scratch. No
shipped route builds from scratch. Under
[`standard_of_proof.md`](../standard_of_proof.md)'s rule 2 the argument holds
while its premise is enforced, and tests in the gate enforce it: a *Monotone*
verdict beside a cube violation fails
`region.rs::fact_grain_never_calls_a_violated_cube_monotone` (every corpus and
register cube within the limit) and `region_synth.py`'s check (random
programs); a node of a monotone cube with `n ≤ 10` that a split from root's
fixpoint builds differently, or that holds a stale fact by fork, fails
`detector.rs::a_monotone_verdict_is_one_state_per_node_by_fork_and_by_every_split`
(147 cubes, 1 941 nodes, 37 172 splits, 2026-11-03) and its sibling on random
programs, which pin `11_fifo_gap_static`'s `{(b one)}` as the one corpus node
where scratch departs.

**What it costs, measured** 2026-11-03, release build, the binary at
`b50b2906`, before S1e2.1b.5, against the one after it:

| | unflagged (243 entries) | flagged (26) |
|---|---|---|
| a bare `ein solve`, best of 3: the `work` row | identical on every entry | identical on every entry |
| … stdout, stderr, exit | byte-identical | the `lattice` block only (its first row less the stub's clause, then +5 to +11 rows); every other line, stderr and the exit code byte-identical |
| … wall | median ratio 1.003 over all 243; the 18 above 50 ms within −1.2 % … +5.8 % one run each; `zebra.ein`, twelve interleaved runs each, median 1 781 → 1 798 ms, sd 22–29 ms | within the limit unchanged (2–7 ms, process start); past it +27 ms (`features/05`) to +151 ms (`features/04`, 1 241 → 1 392 ms), the stale check reading ~65 000 nodes |
| the corpus runs, `utils/corpus_cost.py --runs 3`, summed per entry | no entry crosses 1 s | `features/04` 379 → 529 ms, `features/05` 828 → 879 ms, `features/01` 1 946 → 2 193 ms (slow already); none crosses 1 s, and `--check` finds 0 disagreements with the manifest |

The `work` row counts the route's saturations, as before: the detector's own
(the attribution's few, the spot check's at most six a printed state) are not
in it, so no count moves; the `paths` row says how many parents it read.

## 6. How to solve

### 6.1 Three words

The user's, as this page uses them:

- **stable**: the answer is a function of the program alone. It does not
  depend on a lever, on interner ids, on rule order or on `--jobs`. That is
  the config principle.
- **proved**: the answer comes with a certificate that a second, simpler
  check accepts.
- **effective**: the cost grows with the size of the answer's description,
  not with 2ⁿ, wherever the case allows.

### 6.2 Per question

| question | monotone | non-monotone |
|---|---|---|
| **is R non-empty** (a first solution) | any sound search. A node found is proved by one saturation, and learned nogoods are safe | a node found is proved the same way. A *no* needs care: learning from a refutation across a hole can prune all of R. On [`synth/trigger_hole.ein`](../../../examples/lattice/synth/trigger_hole.ein), with 9 nodes in R, the walk answered `Contradiction` ([S1e2.1a.8](../../../plans/m1e2_correctness/p1e2.1a_the_solution_region/s1e2.1a.8_region_to_program.md#the-fixtures-and-what-the-engine-answers-on-them), 2026-10-15, before the switch). What may be learned is open, F21's D6 |
| **the lines** | grow the alive down-set layer by layer (route 1), or enumerate N and E and dualise: the minimal line is E's alive members, the maximal line the discharged complements of the minimal transversals of N. Prior art: MARCO-style MUS/MSS enumeration (Liffiton et al., *Constraints* 2016); incremental monotone dualisation is quasi-polynomial in input plus output (Fredman & Khachiyan, *J. Algorithms* 1996) | no duality. **Split on the guards** (§ 6.4): fix the truth of each ground fact a third-kind read can see change, g of them. Inside each cell the program is monotone and has lines, and R is the union over the cells of the part each cell's guess survives. That is the Gelfond–Lifschitz reduct's move, so it presumes a stable reading of a node, which is open (F21's D2). Cost: 2ᵍ × the monotone cost |
| **\|R\|, components, sub-cubes** | from the lines: the sub-cubes, then inclusion–exclusion | per cell, then the union; or enumeration |
| **states, not commitments** | the closed sets in R and their minimal generators: formal concept analysis's closed sets and minimal generators (Ganter & Wille 1999) | classes by state, read node by node |
| **stable, how** | a node's state is unique. Output is ordered by rendered text, the candidate list is config-free, and a lever may only reorder or prune soundly (`ein-cli/tests/lever_invariance.rs`) | first the node, which is open (F21's D2, § 7), then the same |
| **proved, how** | N, E and the static verdict | per cell, its guess and its N and E; otherwise the full classification |
| **effective, how** | route 1 visits the alive down-set and never a node above a dead one (§ 4's runs), and saturation jumps over transient nodes. Dualising N and E instead, past route 1's reach, is P1e2.1c's | g, not n, sets the exponent, where g is small. § 6.4 measures g, and it is small on only part of the corpus |

### 6.3 What the engine implements

Two of these strategies, and none of the non-monotone ones:

- **The oracle** — the general case, every node classified, at n ≤ 16
  ([`region_dump.md` § The limit](region_dump.md#the-limit)). It is `ein
  oracle FILE --dump DIR`, and inside `ein solve` it is **route 2**, the
  route a flagged program takes, which past the limit stops at the budget with
  a partial answer that says so.
- **Route 1** — a monotone program's R from its alive down-set and N, with the
  static verdict as its licence
  ([`ein_infer::downset`](../../../ein.rs/crates/ein-infer/src/downset.rs)).
  It is a bare `ein solve` on a program the check calls monotone, and it is
  held to the oracle node for node
  (`region.rs::route_1_is_the_oracle_on_every_monotone_program_it_reaches`).

The lines by duality past route 1's reach are
[P1e2.1c](../../../plans/m1e2_correctness/p1e2.1c_the_region_search/README.md)'s.
The guard split is a prototype, below. Building any of it into `ein solve`
is out of this page's scope
([Q-M1e2.8](../../../plans/m1e2_correctness/open_questions.md#q-m1e28--where-does-the-rework-of-ein-solve-go));
the non-monotone search past the limit is open, F21's D7 (it was
[S1e2.1c.3](../../../plans/m1e2_correctness/p1e2.1c_the_region_search/s1e2.1c.3_the_non_monotone_search.md)'s
until 2026-11-02).

### 6.4 The guard split, prototyped

[`utils/guard_split.py`](../../../utils/guard_split.py) builds the cells as
programs and runs them through the engine:

- **The guards.** Every third-kind read is pointed at `gs-<rel>`, a static
  relation that holds the program's given facts of `<rel>` and the facts the
  guess calls present. A derivable activator is a guard too, and a cell
  compiles its rule when the guess calls it present. The guards are the facts
  one saturation of the relaxed program (every guard read empty, every
  candidate given) can hold, that the program does not give, and that a
  rewritten read's pattern can match.
- **The cells.** Each guess γ is checked by the cell's own rules, monotonically:
  a fact called present is owed, a fact called absent refutes. So R(P_γ) is
  the part of the cube on which the guess is the truth, and the split's R is
  the union. Each cell is solved by a bare `ein solve`, route 1, and read back
  from `--json-summary`: a state's commitments are the intervals [g … cl] over
  its minimal generators, which is exact because cl is a closure operator on
  a monotone cube (§ 3).
- **The check.** The union is held to `ein oracle --dump` on the original
  program, node for node.

`python3 utils/guard_split.py --corpus` (2 s), 2026-11-02, against the
binary and corpus of `6743dd17` and `synth/local_not_global.ein`, which was
banked with the script. It takes every flagged corpus and register program
and every program under `examples/lattice/synth/`. Within the limit that is
**21 programs, and 19 agree with the oracle node for node**:

| program | n | g | cells (non-empty) | \|R\| | node for node | saturations: cells / cube |
|---|---:|---:|---:|---:|---|---:|
| [`corpus/fuzz_findings/d3-unsat-core.ein`](../../../corpus/fuzz_findings/d3-unsat-core.ein) | 5 | 1 | 2 (2) | 20 | yes | 36 / 32 |
| [`branching/13_lookahead_naf_world.ein`](../../../examples/branching/13_lookahead_naf_world.ein) | 2 | 2 | 4 (2) | 4 | yes | 9 / 4 |
| [`branching/14_lookahead_unjudgeable.ein`](../../../examples/branching/14_lookahead_unjudgeable.ein) | 2 | 1 | 2 (2) | 4 | yes | 7 / 4 |
| [`ein-bugs/alive-empty-interlayer.ein`](../../../examples/ein-bugs/alive-empty-interlayer.ein) | 3 | 2 | 4 (4) | 4 | yes | 17 / 8 |
| [`ein-bugs/naf-upward-closure.ein`](../../../examples/ein-bugs/naf-upward-closure.ein) | 2 | 1 | 2 (2) | 3 | yes | 7 / 4 |
| [`ein-bugs/wrong-answers/f07-fpoff.ein`](../../../examples/ein-bugs/wrong-answers/f07-fpoff.ein) | 1 | 2 | 4 (2, 1 node in two) | 2 | yes | 6 / 2 |
| [`ein-bugs/wrong-answers/naf-nowb.ein`](../../../examples/ein-bugs/wrong-answers/naf-nowb.ein) | 2 | 1 | 2 (2) | 3 | yes | 7 / 4 |
| [`ein-bugs/wrong-answers/stale-naf-fork-warn.ein`](../../../examples/ein-bugs/wrong-answers/stale-naf-fork-warn.ein) | 1 | 1 | 2 (2) | 1 | **no**: {(q one)} extra | 4 / 2 |
| [`ein-bugs/wrong-answers/stale-naf-fork.ein`](../../../examples/ein-bugs/wrong-answers/stale-naf-fork.ein) | 1 | 1 | 2 (2) | 1 | **no**: {(q one)} extra | 4 / 2 |
| [`features/07_unstratifiable.ein`](../../../examples/features/07_unstratifiable.ein) | 1 | 2 | 4 (2, 1 node in two) | 2 | yes | 6 / 2 |
| [`lattice/04_obligation_trigger_hole.ein`](../../../examples/lattice/04_obligation_trigger_hole.ein) | 2 | 1 | 2 (2) | 3 | yes | 7 / 4 |
| [`lattice/08_absent_witness_hole.ein`](../../../examples/lattice/08_absent_witness_hole.ein) | 3 | 1 | 2 (2) | 5 | yes | 13 / 8 |
| [`lattice/09_activator_hole.ein`](../../../examples/lattice/09_activator_hole.ein) | 2 | 1 | 2 (2) | 3 | yes | 7 / 4 |
| [`lattice/synth/local_not_global.ein`](../../../examples/lattice/synth/local_not_global.ein) | 2 | 2 | 4 (2) | 2 | yes | 13 / 4 |
| [`lattice/synth/maximal_not_top.ein`](../../../examples/lattice/synth/maximal_not_top.ein) | 3 | 1 | 2 (1) | 2 | yes | 13 / 8 |
| [`lattice/synth/merged_states.ein`](../../../examples/lattice/synth/merged_states.ein) | 5 | 0 | 1 (1) | 16 | yes | 25 / 32 |
| [`lattice/synth/one_model_per_component.ein`](../../../examples/lattice/synth/one_model_per_component.ein) | 4 | 0 | 1 (1) | 4 | yes | 11 / 16 |
| [`lattice/synth/three_components.ein`](../../../examples/lattice/synth/three_components.ein) | 7 | 2 | 4 (4) | 13 | yes | 109 / 128 |
| [`lattice/synth/trigger_hole.ein`](../../../examples/lattice/synth/trigger_hole.ein) | 5 | 2 | 4 (4) | 9 | yes | 74 / 32 |
| [`lattice/synth/trivial_line.ein`](../../../examples/lattice/synth/trivial_line.ein) | 5 | 0 | 1 (1) | 24 | yes | 32 / 32 |
| [`tests/region/07_cube.ein`](../../../tests/region/07_cube.ein) | 2 | 1 | 2 (2) | 3 | yes | 7 / 4 |

**Where it disagrees.** On W8's two witnesses, `stale-naf-fork` and its
`-warn` twin, the split puts `{(q one)}` in R and the oracle does not. The
oracle builds a node by fork from root, which already holds `(p one)`, derived
under `(absent (q one))`; a cell builds it from scratch, where `(p one)` is
never derived. So **the guard split is not yet sound against the oracle's
node**. It gives the reading the scratch route gives, `{(q one)}` in R
([P1e2.1b](../../../plans/m1e2_correctness/p1e2.1b_non_monotone_programs/README.md#why-a-phase-of-its-own),
`utils/region.py`), and which reading R takes is open (F21's D2). On `features/07` and `f07-fpoff` one node lies in two cells: the
program has two stable readings there, and the union is still the oracle's R.

**g against n.** Within the limit g is at most 2 and n at most 7. g exceeds
n on the two unstratifiable programs, because a guard need not be a
candidate. On every program with a guard except `three_components`, the 2ᵍ
cells cost more saturations than the 2ⁿ of the cube. The exponent matters only
past the limit, where the split takes the guards and builds no cell:

| program | sites | n | g |
|---|---|---:|---:|
| [`features/01_not_and_absent.ein`](../../../examples/features/01_not_and_absent.ein) | absent, activator | 35 | 18 |
| [`features/04_open.ein`](../../../examples/features/04_open.ein) | absent | 81 | 28 |
| [`features/05_stdlib_domain_elim.ein`](../../../examples/features/05_stdlib_domain_elim.ein) | absent | 56 | 0 |
| [`features/12_expect_false.ein`](../../../examples/features/12_expect_false.ein) | absent | 20 | 0 |
| [`lattice/06_friends_tables.ein`](../../../examples/lattice/06_friends_tables.ein) | obligation-trigger | 19 | 19 |
| [`saturation/square-unique/corner-house.ein`](../../../examples/saturation/square-unique/corner-house.ein) | absent | 118 | 40 |
| [`saturation/square-unique/cul-de-sac.ein`](../../../examples/saturation/square-unique/cul-de-sac.ein) | absent | 114 | 37 |
| [`saturation/square-unique/terminus.ein`](../../../examples/saturation/square-unique/terminus.ein) | absent | 153 | 49 |

So g ≪ n holds on 2 of the 8, the two that fact grain unflags, where no
candidate can match the read and the split has one cell. On the others g is
18 to 49. **On `lattice/06_friends_tables`, the conditional obligation, g = n**:
its trigger reads the candidate relation itself, so every candidate is a
guard and the split is no cheaper than the cube. The case § 2 calls a
matter of course is the one the split does not shorten, which is F21's D7
to answer.

## 7. Two properties of the node functions, and path dependence

Written 2026-11-02, at the user's *"Ok, just add all of this into followup
and docs - will return to it later"*. It records an analysis and decides
nothing: every question it leaves is [F21](../../../plans/followups/f21_non_monotone_programs_decided.md)'s
(§ 8), which carries the same analysis with the user's words in full. Each
claim is labelled by its standing under
[`standard_of_proof.md`](../standard_of_proof.md): **held by a test**,
**measured** (with the command, commit and date), **argued** (with the premise
it rests on), or **unverified** (with what would check it). § 7.8 lists them
together.

### 7.1 A domain and two arrows

The user, 2026-11-02: *"lattice is a domain of non-binary truth (liveness,
dischargeness) function, or 2 binary functions; And while lattice is
set-indexed dag - then path-independence === monotonicity; Correct me if I'm
wrong"*.

```
L ──state──▶ KB ──(dead, owes)──▶ {0,1}²
```

- **The lattice** L = (2^H, ⊆), the commitments over the candidate universe
  H, is the domain: a Boolean lattice, never monotone or non-monotone itself.
- **The state**, `state(C)`, is the KB a node holds.
- **The status** is two binary functions of the state, `dead` and `owes`,
  which are `region.json`'s two bits per node ([`region_dump.md`](region_dump.md)).
  R = alive ∧ discharged.

So *monotone* is asked of an arrow: of the state (§ 7.3), of the status
(§ 7.5), and, as a third and stronger question, of the program (§ 7.6).

### 7.2 Paths, and the general form

The user, 2026-11-02: *"I guess this is a special case of more common
sat(P+C1+C2) != sat(sat(P+C1)+C2) ? so in general non-monotonic is not only
about root scratch and fixpoint"*: sat(P ∪ C1 ∪ C2) against
sat(sat(P ∪ C1) ∪ C2). A node's state can be built along a path,
each step forking the last fixpoint, writing the next facts and saturating:

| construction | form | used by |
|---|---|---|
| scratch | sat(P ∪ C) | `utils/region.py`; the guard split's cells (§ 6.4) |
| fork from root | sat(sat(P) ∪ C), the case C1 = ∅ | every shipped route: the oracle, route 1, route 2 |
| a parent path | sat(sat(P ∪ C \ {h}) ∪ {h}), and longer chains | the walk's `-y` check before 2026-10-18; any route that extends a parent's fixpoint |

Every shipped route builds a node from root with all of C at once, so C1 = ∅
is the only case a route meets. **Measured**
([`node_census.md`](../../../plans/m1e2_correctness/p1e2.1b_non_monotone_programs/node_census.md),
`cargo run --release -p ein-infer --example node_census`, `bc1dff73`,
2026-11-02):

- fork and scratch hold different facts on 4 of the 256 nodes of the 18
  flagged cubes within the limit, and on none of the 268 213 nodes of the 155
  monotone ones (`-- --control`);
- a one-step parent path, `sanity::check_commutativity`, reaches a different
  state from the direct fork on 8 of `d3-unsat-core`'s 26 checked nodes, 2 of
  `lattice/08`'s 4 and 1 of the probe `via_hole.ein`'s 4, and the direct fork is
  scratch's node on all of them. On `lattice/08` nothing is derived at root:
  `soup-unless-allergic` fires at the parent `{(seats Ann C1)}` and keeps
  `(meal Ann Soup)` after `(allergic Ann)` is added.

So on a flagged program, by today's fork, a node's state can depend on the
path that built it, and a route that extended a parent's fixpoint would build
a node other than the oracle's. **Every split C1 ⊆ C and every maximal chain
are measured** since M1e2's S1e2.1b.7
([`node_census.md` § 11](../../../plans/m1e2_correctness/p1e2.1b_non_monotone_programs/node_census.md#11-the-path-census--s1e21b7),
`cargo run --release -p ein-infer --example node_census -- --paths`,
2026-11-02, at `8a2dffe3`):

- on the 18 flagged cubes within the limit, 20 of the 256 nodes reach two
  states, and none reaches three. 10 of them depend on the path only through a
  dead parent, which keeps its `(false)`. Along alive paths a node's **status**
  moves on 3: `lattice/08`'s owing hole is in R by way of its parent, and W8's
  `{(q one)}` is in R from scratch;
- on the probe `via_hole.ein` and the written `ACTIVATOR_UNDER_ABSENT`, a node
  of R is dead by way of an alive parent, through a chain no one-step parent
  shows;
- on `d3-unsat-core`, four of § 6.1's eight differences were the `-y` direct
  path stopping at its first contradiction (`enable-fail-fast-fork`); built to
  quiescence, as the oracle builds a node, those four agree;
- on 150 monotone cubes (n ≤ 12, 6 069 nodes, 1 207 419 steps) no node depends
  on the path. The five at n = 15–16 are past the census's bound. Since
  T1e2.1b.7.3 banked § 7.6's program, one node of a monotone cube does, between
  fork and scratch (`node_census.md` § 12.2).

Which construction defines a node is open (§ 8, D2), and whether a route may
extend a parent is open too (D3).

**The traversal: a tree of paths, and what it may be memoized by** (argued,
2026-11-02, from a question of the user's).

- **The general object is the path tree.** A node is an ordered sequence of
  distinct candidates, and a step is `s → sat(s ∪ {h})`. Unmemoized it has
  ⌊e·n!⌋ nodes, ≈ 5.7·10¹³ at n = 16 against the cube's 65 536.
- **Memoizing by C** is sound exactly when the state is path-independent, and
  it collapses the tree onto the cube.
- **Memoizing by the state** (the KB) is sound whenever a step is a function of
  the KB's contents, not of its insertion history. That premise is **measured**
  for the engine on every cube the path census read: 688 809 steps taken from
  contents another history had reached, each giving the same result
  ([`node_census.md` § 11.5](../../../plans/m1e2_correctness/p1e2.1b_non_monotone_programs/node_census.md#115-what-it-settles-and-what-it-does-not)).
  It collapses the tree onto a DAG of reachable states, which is acyclic
  because a step only adds.
- **On a path-independent program**, the reachable states are the image of
  C ↦ sat(P ∪ C). Ordered by ⊆, they form a lattice whose join is
  sat(P ∪ C1 ∪ C2) and whose meet is not the intersection. That lattice is the
  read-out's `merged`: k_c commitments onto k_s states.
- **On a path-dependent program**, only the state key is sound. The DAG is not
  a lattice, since one C reaches several states, and it can be larger than the
  cube.
- **This suggests one more reading of a node, open in F21's D2 as (F):** the
  states its paths reach whose facts are not stale. That is the stability
  condition of default logic and of stable models. It was argued to give 0
  states per C on an odd cycle, 1 on a stratified program, and several on an
  even cycle. **Measured, it does not** (§ 11.5 there): on every node of every
  flagged cube within the limit, paths from P reach exactly one stale-free
  state, the odd cycle `d3-unsat-core` included. Paths from root, the tree as
  stated above, reach none on W8's four `{(q one)}` nodes, because root's
  `(p one)` survives on every one of them; and the even cycle `features/07`
  never reaches its second stable model, because the boundary admits
  `derive-p` first. On the written stratified `stratified_stale` ∅ has no
  stale-free state at all: the FIFO gap reaches root itself. So the count
  depends on the admission order and on where a path starts.

### 7.3 The state: path-independent ⇔ monotone, given cumulativity

- **Path-independent**: every path to C gives one fact set, `state(C)`.
- **Monotone**: C ⊆ C′ ⇒ `state(C)` ⊆ `state(C′)`.
- **Cumulative** saturation: X ⊆ Y ⊆ sat(X) ⇒ sat(Y) = sat(X).

**⇒ holds always (argued).** Construction is append-only: the KB never
retracts ([`absent_semantics.md`](absent_semantics.md) § Explicitly not
provided). Take C ⊆ C′ and build C′ by way of C: the result holds `state(C)`,
and by path independence it is `state(C′)`.

**⇐ needs cumulativity (argued).** Take a path ∅ = K_0 ⊆ K_1 ⊆ … ⊆ K_k = C,
with S_0 = sat(P) and each step S_i = sat(S_{i−1} ∪ K_i), and suppose
S_{i−1} = sat(P ∪ K_{i−1}). By monotonicity S_{i−1} ⊆ sat(P ∪ K_i), so
P ∪ K_i ⊆ S_{i−1} ∪ K_i ⊆ sat(P ∪ K_i), and by cumulativity
S_i = sat(P ∪ K_i). By induction every path ends at sat(P ∪ C).

**Who is cumulative.** A monotone program is, by fork: its saturation is a
closure operator (argued; § 7.6 has the test beside it), and the cumulativity
census finds no exception by fork on any monotone cube it read. From scratch
one is banked, § 7.6's. A stratified program evaluated stratum by stratum
should be, since its fixpoint is its perfect model, and it is **measured** so
under an emulation: with each `absent` rule's priority set by its stratum,
every stratified program read is cumulative on every node, and every node
built from scratch is stale-free
([`node_census.md` § 12.4](../../../plans/m1e2_correctness/p1e2.1b_non_monotone_programs/node_census.md#124-admission-in-stratum-order-emulated),
`node_census -- --stratum-order`, 2026-11-02, at `a3ac7d6f`). The census adds
each fact a node's fixpoint derived before saturating and compares the
fixpoint (§ 12.3 there).

**The engine's boundary breaks cumulativity.** It admits equal-priority
candidates one a round in (priority, FIFO) order, not in stratum order
(`node_census.md` § 4). Take r2 `a ∧ b ∧ absent(q) → p`, written before r1
`a ∧ absent(z) → q`. Built at once, {a, b} gives {a, b, p, q}, with `p`
stale; by way of {a} it gives {a, b, q}. The state function is monotone and
the state is path-dependent: {a, b, q} ⊆ sat({a, b}), and sat({a, b, q}) ≠
sat({a, b}). This example as written is **measured**: banked as
[`examples/lattice/10_fifo_gap.ein`](../../../examples/lattice/10_fifo_gap.ein),
the engine reproduces it, {a, b} holding a stale `p` built at once and not by
way of {a}, and adding the derived `q` before saturating at {a, b} takes `p`
away ([`node_census.md` § 12.1](../../../plans/m1e2_correctness/p1e2.1b_non_monotone_programs/node_census.md#121-the-fifo-gap-banked), `node_census
-- --paths --cumulative --invariance`, `a3ac7d6f`). As the engine admits
today, saturation is cumulative on every node the census read except where
this race reaches: the fixture, the reader at an earlier priority, the written
`stratified_stale` and § 7.6's program (§ 12.3 there). Its relative
`stratified_stale` (`p ← ¬q`, `q ← ¬r`, one priority, `derive-p` written
first) was measured before: its scratch root holds a stale `(p one)`, and
swapping the two rules' text changes its universe (`node_census.md` § 2.3 and
§ 7.1, `bc1dff73`). Before the fixture no corpus program showed the gap: from
scratch, no corpus node held a stale fact (§ 4 there).

So the state has **one property**, monotone ⇔ path-independent wherever
saturation is cumulative. It is broken by an odd-depth `absent` over D (§ 7.4)
and by the FIFO gap. It is
[Q-M1e2.2](../../../plans/m1e2_correctness/open_questions.md#q-m1e22--does-a-guards-absent-premise-bind-the-state-the-search-ends-in)'s
subject.

### 7.4 A state difference needs an `absent`

The user, 2026-11-02: *"Is this possible without absent? We already decide
the case with absent in s1e2.4.1"*. For the state, no.

- **Argued.** A path or scratch difference needs a fact admitted under a read
  that a later fact falsifies. A positive read, or one at even `absent` depth,
  only becomes true as facts grow, and a stored negative read positively only
  grows. An activator only adds plans as facts grow, and an obligation rule
  asserts `(open …)` and derives nothing. What is left is an odd-depth
  `absent` in a saturation rule, `SiteKind::Absent`. The premise is the read
  kinds as [`monotone.rs`](../../../ein.rs/crates/ein-infer/src/monotone.rs)
  classifies them.
- **Measured, consistent with it** (`node_census.md` § 2, § 4, § 6.1,
  `bc1dff73`): each of the 5 nodes where fork and scratch differ, 4 in the
  corpus and 1 probe, holds a fact derived under an `absent` that the
  commitment falsifies. Of the 11 parent-path differences, 7 hold such a
  fact. `d3-unsat-core`'s other 4 were dead nodes `-y` built two ways, and
  built to quiescence, as the oracle builds a node, they agree: the
  difference was `-y`'s direct path stopping at its first contradiction
  ([`node_census.md` § 11.2](../../../plans/m1e2_correctness/p1e2.1b_non_monotone_programs/node_census.md#112-the-flagged-cubes), 2026-11-02).
  Since then the path census finds every state a path reaches beyond fork
  and scratch holding such a fact, 21 of 21 (§ 11.5 there).
- **An activator can hide the `absent`** (held by a test:
  `region.rs::each_cause_has_a_fixture_whose_cube_shows_its_hole`, its
  `ACTIVATOR_REFUTES`). The rule a node compiles there reads
  `(absent (?R ?x))`; the program's only site is the activator, and a check of
  root's plans does not see the `absent`.
- **An activator can hide the `absent` from a refutation's derivation as
  well** (measured: `cargo run --release -p ein-infer --example hole_census --
  --written`, 2026-11-02 at `4562bf70`,
  [hole census § 2](../../../plans/m1e2_correctness/p1e2.1b_non_monotone_programs/hole_census.md#2-by-cause-and-kind)).
  On `ein-infer/examples/shared/written.rs`'s `ACTIVATOR_UNDER_ABSENT`, `act`
  derives the activator `(watch r)` under `(absent (q ?x))`, and the plan it
  compiles refutes `(r A)` reading no `absent`. The dead hole `{(p A), (r A)}`
  has no `absent` premise, because a derivation does not record the activator
  its plan was compiled from. Held by a test since M1e2 S1e2.1b.4:
  `region.rs::an_untainted_death_is_dead_in_every_superset`, which also holds
  that a death resting on no `absent` over D is dead in every superset on
  every flagged cube within the limit, once the walk follows each plan's
  activator.

### 7.5 The status: `dead` follows the state, `owes` does not

- **`dead`** inherits monotonicity from the state (argued): a contradiction is
  facts, and on a monotone state they stay.
- **`owes`** does not. An instance is owed when its trigger holds and no
  witness does, so a trigger, or an activator that compiles an obligation,
  adds a debt over a monotone, path-independent state. Held by a test:
  `each_cause_has_a_fixture_whose_cube_shows_its_hole` asserts, on
  `lattice/04` (a trigger), `lattice/09` (an activator) and `lattice/08`,
  that no dead node has an alive superset and some discharged node has an
  owing one. Measured: fork and scratch agree on every node of `lattice/04`
  and `lattice/09` (`node_census.md` § 2.1). **A hole needs no path
  dependence.**
- So holes are a question about the status, Q-M1e2.7 (d) (§ 8, D4), and not
  about the state.

### 7.6 The program: the third, stronger property

A **monotone program**, the logic-programming term, has a consequence
operator that is monotone on every KB. Then saturation is a closure operator,
both arrows are monotone, and R has no hole (§ 5.1's theorem). Held by a
test: `region.rs::a_monotone_verdict_makes_saturation_a_closure_operator` on
every monotone cube within the limit, and
`region.rs::monotone_programs_show_no_violation_and_the_non_monotone_three_are_flagged`.
The first holds cl, the state's projection onto H, to the closure axioms,
which is adjacent to cumulativity of the whole state and not the same claim;
the `--control` run of § 7.2 measures the whole state, fork against scratch,
on 268 213 monotone nodes.

The static check approximates this third property: a site says the operator
is not monotone over D. The node functions can still be monotone on the
program's own lattice, and those programs are the check's **conservative**
flags: 8 of the 18 flagged programs within the limit (measured, § 5.2,
`4562bf70`).

**The check's verdict is relative to root.** A read over a relation outside D
is *static* because root's fixpoint completes it and a fork never re-derives
it. So a *Monotone* verdict says the operator is monotone on root's fixpoint
extended by commitments, which is what the theorem and every shipped route
need, and not on every KB. A scratch build re-derives root inside each node,
where § 7.3's FIFO race can still reach a static read: with `a` given and
only `b` a candidate, r2 `a ∧ b ∧ absent(q) → p` written before r1
`a ∧ absent(z) → q` has no site (`q` is not in D). **Measured**, banked as
[`examples/lattice/11_fifo_gap_static.ein`](../../../examples/lattice/11_fifo_gap_static.ein):
the static check calls it monotone and `ein solve` takes route 1, and its
scratch state on {b} holds a stale `p` that its fork state does not
([`node_census.md` § 12.2](../../../plans/m1e2_correctness/p1e2.1b_non_monotone_programs/node_census.md#122-the-static-variant-banked), `a3ac7d6f`).
Before it, the corpus showed no such case (the `--control` run: fork and
scratch agreed on all 268 213 monotone nodes, `bc1dff73`). So building a node
from scratch (F21's D2 (B)) needs admission in stratum order on programs the
check calls monotone too: under the emulation of § 12.4 there, scratch's {b}
is the fork's. [`solution_semantics.md` § 1.5](solution_semantics.md#15-commitment-node-state)
carries the qualifier.

| property | of | broken by | its question |
|---|---|---|---|
| **state monotone** (⇔ path-independent, given cumulativity) | `state`: L → KB | an odd-depth `absent` over D; the FIFO gap | which KB a node holds; whether a route that extends a parent agrees with the oracle (D1–D3) |
| **status monotone** | `(dead, owes)`: KB → {0,1}² | `dead`: as the state. `owes`: a trigger, or an activator that compiles an obligation | holes, and whether two lines describe R (D4) |
| **monotone program** | the operator, on every KB | any site | the static check's verdict, and route 1's licence |

### 7.7 What *forbid* covers, and option (E)

*Forbid*, ruled 2026-11-02
([S1e2.4.1 § 5](../../../plans/m1e2_correctness/p1e2.4_what_the_switch_left/s1e2.4.1_refutation_under_absent.md#5-the-ruling)),
refuses `(absent P)` ⟹ `(false)` or `(not …)` over a relation a commitment
can still extend. The refusal is not built yet (M1e2's
[S1e2.2.5](../../../plans/m1e2_correctness/p1e2.2_wrong_answers/s1e2.2.5_naf_upward_closure.md),
with its F1–F5 open). It does not cover:

- an `absent` that concludes a **positive** fact: `derive-p` on `features/07`
  and `stale-naf-fork`, `soup-unless-allergic` on `lattice/08`. Every state
  difference of § 7.4 is of this kind;
- the **derived** refutation, S1e2.4.1's `via` class (`stale-naf-fork`'s
  `p-and-q-clash`, the probe `via_hole.ein`), unless its F2 takes it;
- a refutation **compiled at a node** through an activator
  (`ACTIVATOR_REFUTES`), unless its F3 reads the plans a node compiles;
- a refutation through an **activator derived under an `absent`**
  (`ACTIVATOR_UNDER_ABSENT`, § 7.4): the compiled plan reads no `absent`, so
  a check of the plans a node compiles misses it too, unless F2's `via`
  follows a plan's activator (measured, M1e2's
  [taint census § 7](../../../plans/m1e2_correctness/p1e2.1b_non_monotone_programs/taint_census.md#7-the-refutation-a-node-compiles-for-f3));
- obligation **triggers and activators**, which refute nothing;
- the **FIFO gap**.

**Option (E)** extends *forbid* to every `SiteKind::Absent` over a relation a
commitment can extend, whatever it concludes. *Forbid*'s own statement
applies word for word: such a read *"judges a state that has not yet decided
P as if P were false"*. Under (E), no program that loads keeps a site that
breaks the state, so the state is monotone and path-independent on every one
of them, and only status holes remain (argued, from §§ 7.3–7.5; premise: the
refusal reads D and the plans a node compiles). Its cost is **measured**: the
site census (`cargo run --release -p ein-infer --example site_census`, at
`4b822a8d` and `6743dd17`, 2026-11-02) lists **13** programs with an `absent`
site that concludes a positive fact: `d3-unsat-core`, `f07-fpoff`,
`stale-naf-fork` and its `-warn` twin, `features/01`, `/04`, `/05`, `/07`,
`/12`, the three `square-unique` demos, and `lattice/08`. Five of them
*forbid* already reaches, under F1 (b) and F2 with `via`: `features/05` and
`/12` by a refuting site of their own, `d3-unsat-core` and W8's pair through
`via` (S1e2.4.1 § 1). Since M1e2's S1e2.1b.7 banked
[`lattice/10_fifo_gap`](../../../examples/lattice/10_fifo_gap.ein), the list
is 14, and § 8.1 has (E) under each reading of F1 and F2. (E) is an option,
not a ruling (§ 8, D1).

### 7.8 The standing of each claim

| claim | standing | held by, or what would check it |
|---|---|---|
| fork ≠ scratch on 4 of 256 flagged nodes, 0 of 268 213 monotone | measured | `node_census`, `bc1dff73`, 2026-11-02. Re-taken at `a3ac7d6f` with § 7.6's program banked: 1 of 268 215 monotone nodes, that program's (`node_census.md` § 12.2) |
| a one-step parent path differs on `d3-unsat-core`, `lattice/08`, `via_hole.ein` | measured | `node_census` § 6.1, same run |
| every split C1 ⊆ C, and longer chains | measured | `node_census -- --paths`, `8a2dffe3`, 2026-11-02 (`node_census.md` § 11): 20 of 256 flagged nodes, 0 of 6 069 monotone at n ≤ 12 (1 of 6 071 once § 7.6's program was banked, `a3ac7d6f`); every maximal chain to 13 700 tree nodes |
| the path tree collapses onto the cube iff the state is path-independent, and onto a state DAG always | argued; the second half's premise measured | the premise, a step is a function of the KB's contents: 688 809 steps from contents met before, 0 conflicts, same run (§ 11.5 there) |
| (F)'s per-C state counts: 0 on an odd cycle, 1 stratified, several on an even cycle | **refuted**, measured | same run: from P, 1 on every flagged node, `d3-unsat-core`'s odd cycle included; from root, 0 on W8's four `{(q one)}`; 0 on `stratified_stale`'s ∅ (written into `node_census`); never 2 |
| a node's status moves along an alive path | measured | same run: `lattice/08`'s hole in R by way of its parent; a node of R dead by way of an alive parent on the probe `via_hole` and on `ACTIVATOR_UNDER_ABSENT` |
| path-independent ⇒ monotone | argued | premise: the KB never retracts. No test named here enforces it |
| monotone ⇒ path-independent, given cumulativity | argued, and measured consistent; by fork and by every split from root's fixpoint, **held by a test** | premise: cumulativity. By fork, every monotone cube read at n ≤ 12 is cumulative and path-independent; § 7.6's program is neither from scratch (`node_census.md` § 11.4, § 12). Since M1e2 S1e2.1b.5, `ein-infer/tests/detector.rs::a_monotone_verdict_is_one_state_per_node_by_fork_and_by_every_split` builds every node of every monotone corpus cube with n ≤ 10 by fork and by every split `sat(sat(root ∪ C1) ∪ (C \ C1))`, and its sibling the random programs' |
| a monotone program's saturation is cumulative | by fork, measured; from scratch, **refuted** | `node_census -- --cumulative --control --max-n 12`, `a3ac7d6f`, 2026-11-02 (`node_census.md` § 12.3): on 151 monotone cubes (n ≤ 12), 6 071 nodes, 0 of 12 135 fork tests change a fixpoint, and 1 of 25 124 from scratch, `11_fifo_gap_static` (§ 7.6) |
| a stratified program evaluated stratum by stratum is cumulative | measured, under an emulation | `node_census -- --stratum-order`, same commit (§ 12.4 there): every node of every stratified program read, each `absent` rule's priority set by its stratum. As the engine admits today it is **refuted** on four programs, each a FIFO race (§ 12.3) |
| the FIFO example is path-dependent in the engine as written | measured | [`examples/lattice/10_fifo_gap.ein`](../../../examples/lattice/10_fifo_gap.ein), `node_census -- --paths --cumulative --invariance`, same commit (§ 12.1 there) |
| a state difference needs an `absent` | argued, and measured consistent | `node_census` § 2, § 4, § 6.1; and § 11.5: each of the 21 states a path reaches beyond fork and scratch holds a fact derived under an `absent` its node falsifies |
| an activator can hide an `absent` from root's plans | held by a test | `each_cause_has_a_fixture_whose_cube_shows_its_hole` |
| a death resting on no `absent` over D is dead in every superset, the taint following each plan's activator (D6's (b)) | held by a test | `region.rs::an_untainted_death_is_dead_in_every_superset`, on every flagged cube within the limit |
| an activator derived under an `absent` hides it from the refutation's derivation | held by a test, and measured | `region.rs::an_untainted_death_is_dead_in_every_superset` pins the empty `absent_premises`; `hole_census -- --written`, `4562bf70` |
| `dead` follows the state | argued | premise: the state is monotone |
| `owes` breaks over a path-independent state | held by a test, and measured | `each_cause_has_a_fixture_whose_cube_shows_its_hole`; `node_census` § 2.1 |
| a monotone program has both properties and no hole | held by a test | `a_monotone_verdict_makes_saturation_a_closure_operator`, `monotone_programs_show_no_violation_and_the_non_monotone_three_are_flagged`; since S1e2.1b.5 `fact_grain_never_calls_a_violated_cube_monotone` holds the shipped verdict to no violation on every corpus and register cube within the limit, and `region_synth.py`'s check on random programs |
| a *Monotone* verdict can leave scratch ≠ fork through a FIFO race over a static read | measured, and **held by a test** | [`examples/lattice/11_fifo_gap_static.ein`](../../../examples/lattice/11_fifo_gap_static.ein), `node_census -- --paths`, same commit (§ 12.2 there); since S1e2.1b.5 the same test pins `{(b one)}` as the one corpus node where scratch departs from the fork |
| 8 conservative flags of 18 within the limit | measured | `site_census`, `4562bf70` |
| (E) leaves a monotone, path-independent state on every program that loads | argued, and measured consistent | premise: S1e2.2.5's F1 reads D and its F3 sees compiled plans. Every program the path and cumulativity censuses find path-dependent is refused by (E) or *forbid*, but `ACTIVATOR_REFUTES` (F3) and § 7.6's program (by scratch only) (§ 8.1, D1) |
| (E)'s 13 programs | measured | `site_census`, `4b822a8d` and `6743dd17`; 14 with `lattice/10_fifo_gap` (§ 8.1, `refutation_census -- --option-e`, `ed8a13f2`) |
| each of them has a rewrite that keeps R | measured: **5 of the 6** within the limit and `features/12` past it keep R; `d3-unsat-core` has none found; 6 past the limit are not comparable | § 8.2's probes, `ein test` on each, `4502e841` |

### 7.9 The causes, as one table

Written 2026-11-02 by M1e2's
[S1e2.1b.7](../../../plans/m1e2_correctness/p1e2.1b_non_monotone_programs/s1e2.1b.7_the_consequences_written_down.md)
from the census pages, one row per cause, each with a fixture. It decides
nothing: every *covers* and every *bears on* cell names an option of
[F21](../../../plans/followups/f21_non_monotone_programs_decided.md), which is
open. **Path dependence is not a row**: it is a consequence, and its column
says which causes produce it.

| | cause | fixture | breaks | the static check sees it as | the cube calls it | path dependence |
|---|---|---|---|---|---|---|
| c1 | an `absent` that concludes a positive fact | [`features/07`](../../../examples/features/07_unstratifiable.ein), [`stale-naf-fork`](../../../examples/ein-bugs/wrong-answers/stale-naf-fork.ein) (W8) | **the state**: root's `(p one)` is kept under a guard the commitment falsifies | `SiteKind::Absent` | conservative | yes: fork ≠ scratch at `{(q one)}` (`node_census.md` § 2.1); from root no path reaches a stale-free state there (§ 11.2) |
| c2 | an `absent` that refutes | [`naf-upward-closure`](../../../examples/ein-bugs/naf-upward-closure.ein) (W3's `bad`) | **the state, and `dead` with it**: `(false)` is a fact | `SiteKind::Absent`, concluding `(false)` | exercised: a dead hole | only through a dead parent: 1 of 4 nodes (`node_census.md` § 11.2) |
| c3 | a refutation through a fact derived under an `absent` (`via`) | probe [`via_hole.ein`](../../../plans/m1e2_correctness/p1e2.4_what_the_switch_left/s1e2.4.1_probes/via_hole.ein); in the corpus `stale-naf-fork`'s `p-and-q-clash` | **the state, and `dead`**; the refuting rule reads no `absent` | `SiteKind::Absent` on the deriving rule; the refuting rule is no site | exercised: a dead hole | yes: 2 of 8 nodes, through alive parents, and one of them, a node of R, is dead that way (`node_census.md` § 11.3) |
| c4 | an `absent` that takes a witness away | [`lattice/08`](../../../examples/lattice/08_absent_witness_hole.ein) | **the state, and `owes` with it** | `SiteKind::Absent` | exercised: an owing hole | yes, with nothing derived at root: 2 of 8 nodes, and along an alive path the owing hole is in R (§ 11.2 there) |
| c5 | an obligation's trigger | [`lattice/04`](../../../examples/lattice/04_obligation_trigger_hole.ein), [`06_friends_tables`](../../../examples/lattice/06_friends_tables.ein) | **the status alone**: `owes` | `SiteKind::ObligationTrigger` | exercised: an owing hole | none: one state per node by every split and chain (§ 11.2 there) |
| c6 | an activator that compiles an obligation | [`lattice/09`](../../../examples/lattice/09_activator_hole.ein) | **the status alone**: `owes` | `SiteKind::Activator` | exercised: an owing hole | none, as c5 |
| c7 | an activator that compiles a refutation, hiding its `absent` | `ACTIVATOR_REFUTES` ([`written.rs`](../../../ein.rs/crates/ein-infer/examples/shared/written.rs)) | **the state, and `dead`**, through `(absent (?R ?x))` in a plan only a node compiles | `SiteKind::Activator` only: root compiled no plan of the rule | exercised: a dead hole | only through a dead parent: 1 of 4 nodes (§ 11.3 there) |
| c8 | an even cycle through an `absent` | `features/07`, [`f07-fpoff`](../../../examples/ein-bugs/wrong-answers/f07-fpoff.ein) | **the state**, and stability: the written order of two rules at one priority moves the universe (`node_census.md` § 7.1) | two `SiteKind::Absent`; the cycle is no shipped check's (`node_census -- --strata`) | conservative | yes, as c1 |
| c9 | an odd cycle through an `absent` | [`d3-unsat-core`](../../../corpus/fuzz_findings/d3-unsat-core.ein) | **the state**: a parent path keeps `(r1 o2 o2)`, derived under an `absent` the commitment falsifies | `SiteKind::Absent`; the cycle as in c8 | conservative | yes: 4 of 32 nodes, through alive parents. § 6.1's other four were `-y`'s fail-fast stop (§ 11.2 there) |
| c10 | the FIFO gap, at one priority | [`lattice/10_fifo_gap`](../../../examples/lattice/10_fifo_gap.ein) (§ 7.3's example) | **the state**: cumulativity, on a stratified program | `SiteKind::Absent` on the reader | conservative | yes: 1 of 4 nodes, `{a, b}` at once against by way of `{a}`, through an alive parent (`node_census.md` § 12.1) |
| c11 | the FIFO gap, the reader at an earlier priority | [`detector_proposal/fifo_gap_priority.ein`](../../../plans/m1e2_correctness/p1e2.1b_non_monotone_programs/detector_proposal/fifo_gap_priority.ein) | **the state**, in every written order: no equal-priority pair exists for a reorder to move | `SiteKind::Absent` on the reader | conservative | yes, as c10, measured on the same node (§ 12.1 there) |
| c12 | an activator derived under an `absent` | `ACTIVATOR_UNDER_ABSENT` (`written.rs`) | **the state, and `dead`**: the `absent` is in the activator's derivation, which no premise records | `SiteKind::Absent` (`act`) and `SiteKind::Activator` (`watch`) | exercised: a dead hole | yes: 2 of 8 nodes, through alive parents, and a node of R is dead that way (§ 11.3 there) |
| c13 | a FIFO race over a static read (§ 7.6) | [`lattice/11_fifo_gap_static`](../../../examples/lattice/11_fifo_gap_static.ein) | **the state built from scratch**, under a *Monotone* verdict; a fork never meets it | nothing: no site | no violation (monotone) | between fork and scratch only: 1 of 2 nodes (§ 12.2 there) |

| | *forbid* covers it (S1e2.2.5's F1–F3) | option (E) (D1) | option (D) (D2) | S1e2.1b.5's detector reports (built 2026-11-03, § 5.3; each cell is the binary's `ein solve FILE` row, on the fixture of the first table) | bears on |
|---|---|---|---|---|---|
| c1 | no: it concludes no refutation. `stale-naf-fork`'s downstream refutation is c3 | refuses it | `stale-naf-fork` is stratified and stays; `features/07` see c8 | `site … concludes a positive fact — at stake: the state`; `status monotone here (conservative)`; **`state stale facts — 1 of the cube's 2 nodes`** | D1, D2, D3 |
| c2 | **yes**, under F1 (a), (b) and (c): the `direct` set ([S1e2.4.1 § 1](../../../plans/m1e2_correctness/p1e2.4_what_the_switch_left/s1e2.4.1_refutation_under_absent.md#1-the-exposed-set-re-counted)) | refuses it, as *forbid* does | stays: stratified | `site … concludes (false) — at stake: the state, and dead with it`; `status non-monotone here (exercised, by bad)` | D4, D6; ruled by *forbid* |
| c3 | only if **F2** takes `via` | refuses the deriving rule | stays | the deriving rule as the `site`; `status exercised, by derive-p`; `paths path-dependent` | D1, D6, F2 |
| c4 | no: nothing refutes | refuses it | stays: stratified | `site … at stake: the state`; `status exercised`; `state no stale fact`; only `paths` sees the path dependence | D1, D2, D3, D4 |
| c5 | no: it refutes nothing | no: not an `absent` site | stays | `site … obligation-trigger … at stake: the status: owes`; `status exercised` | D4, D7 |
| c6 | no | no | stays | `site … its activator relation is one a commitment can change; root compiled no plan of it — at stake: the status: owes, through a plan a node compiles` | D4 |
| c7 | only by **F3** at each node's compile, or by its drafted third placement; not at load ([taint census § 7](../../../plans/m1e2_correctness/p1e2.1b_non_monotone_programs/taint_census.md#7-the-refutation-a-node-compiles-for-f3)) | only if the refusal reads the plans a node compiles | stays | `site watch: its activator relation … — at stake: the state, through a plan a node compiles`; `status exercised, by watch` (on `written.rs`'s text, written to a file) | D6, F3 |
| c8 | no | refuses both rules | **refuses it** (over D) | two `site` rows, `cycle even through an absent — several states with no stale fact can exist`, `order … which the boundary admits first is the order they are written in`, `state stale facts` | D1, D2's (D) and (F) |
| c9 | only through **F2**'s `via`: `fire-0` reads `r1`, which `fire-1` derives under the `absent` | refuses `fire-1` | **refuses it** (over D) | `cycle odd through an absent — a state with no stale fact need not exist`; `paths path-dependent — 3 of 34` | D2's (D) and (F), D3 |
| c10 | no | refuses the reader | stays: stratified. Admission in stratum order closes it | `order … both at priority 200: which the boundary admits first is the order they are written in`; `state stale facts`; `paths path-dependent` | D2 (B) and stratum order, D3 |
| c11 | no | refuses the reader | stays. Stratum order closes it | `order r2 (priority 100) … r2 is admitted first, whatever the order` | as c10 |
| c12 | by neither F3 placement; only if F2's `via` follows a plan's activator | refuses `act` | stays | `site act` and `site watch`; `status exercised, by watch`; `state no stale fact`; `paths path-dependent — 1 of 6` | D1, D6's (b) with the activator walk |
| c13 | no | no: no site | stays | nothing: the detector runs on a flagged program only ([`detector_proposal.md` § 7](../../../plans/m1e2_correctness/p1e2.1b_non_monotone_programs/detector_proposal.md#7-what-the-drafting-found), item 7) | D2 (B) |

**Where the cells come from.** *Breaks*, *path dependence* and *the cube* are
measured: [`node_census.md`](../../../plans/m1e2_correctness/p1e2.1b_non_monotone_programs/node_census.md)
§ 2, § 6.1 and § 7 (`bc1dff73`) and § 11, the path census (`8a2dffe3`), the [hole census](../../../plans/m1e2_correctness/p1e2.1b_non_monotone_programs/hole_census.md)
§ 2 (`072e6d0d`), the [taint census](../../../plans/m1e2_correctness/p1e2.1b_non_monotone_programs/taint_census.md)
§ 3 and § 7 (`b6a3045b`), and the drafted detector's spot check
([`detector_proposal.md` § 5](../../../plans/m1e2_correctness/p1e2.1b_non_monotone_programs/detector_proposal.md#5-the-parent-path-spot-check-priced),
`e7bec8f0`), all 2026-11-02. *The static check sees it as* is `monotone::classify`'s
`SiteKind`, the same day. The *forbid* column is S1e2.4.1 § 1 and the taint
census § 7 read against F1–F3; (E) and (D) are F21's D1 and D2 read against
`site_census` and `node_census -- --strata`. c10, c11 and c13 are
`node_census.md` § 12 (`a3ac7d6f`).

**c2's path dependence.** Fork and scratch agree on every node of
`naf-upward-closure` (§ 2.1 there). Its one path-dependent node is reached
through the dead parent `{(p A)}`, which keeps its `(false)` when `(q A)` is
added (measured, § 11.2 there): the spot check and `-y` skip a dead parent,
and a route that learns from a dead node takes exactly that path. That is W3's
mechanism read as a path. c7 and the dead holes of `synth/three_components`
and `synth/trigger_hole` depend on the path the same way, and only that way.

**The status and the state are apart, measured.** Five programs the cube calls
*conservative* still have a broken state: `stale-naf-fork`, `features/07`,
`d3-unsat-core`, `fifo_gap` (banked since as `lattice/10_fifo_gap`) and
`stratified_stale`. Each holds a stale fact or
has a parent path that reaches another state, and `ein oracle` closes on each
with *"the flagged sites break nothing here"*, which is true of the status
only ([`detector_proposal.md` § 7](../../../plans/m1e2_correctness/p1e2.1b_non_monotone_programs/detector_proposal.md#7-what-the-drafting-found),
items 1 and 5). **Past the limit**, `features/04_open` holds a stale
`(open-slot …)` in 21 042 of the 64 761 members its describe met (item 2
there). So *exercised* and *conservative* are words about the status, and
c1, c8, c9, c10 and c11 break the state while the cube says *conservative*.

## 8. What is open

Every decision about a non-monotone program was deferred on 2026-11-02 to
[F21](../../../plans/followups/f21_non_monotone_programs_decided.md), on the
user's direction: *"prepare everything we are able to do now, without making
final decisions; And then leave further investigation of non-monotonic
lattices as followup"*. Until a decision, the engine does what the last
column says, and since M1e2's
[S1e2.1b.5](../../../plans/m1e2_correctness/p1e2.1b_non_monotone_programs/s1e2.1b.5_the_detector.md)
(2026-11-03) every surface that reports R carries a warning saying so (§ 5.3).

| decision | F21 | the evidence | until it is taken |
|---|---|---|---|
| (E): *forbid* extended to every `absent` site | D1 | § 7.7; [`site_census.md`](../../../plans/m1e2_correctness/p1e2.1b_non_monotone_programs/site_census.md) | *forbid* as ruled; the 13 load |
| what a node's state is (Q-M1e2.2): fork, scratch, guards as premises, refuse a cycle through an odd `absent`, the stable endpoints of every path (§ 7.2); (D)'s scope; admission in stratum order | D2 | § 7.2–7.3; [S1e2.1b.2](../../../plans/m1e2_correctness/p1e2.1b_non_monotone_programs/s1e2.1b.2_fork_or_scratch.md), `node_census.md` § 9–10 | fork from root |
| whether a route may build a node from a parent's fixpoint on a flagged program | D3 | § 7.2 | only on a monotone program |
| what is reported when R has holes (Q-M1e2.7 (d)) | D4 | § 7.5; [S1e2.1b.3](../../../plans/m1e2_correctness/p1e2.1b_non_monotone_programs/s1e2.1b.3_holes.md), its [hole census](../../../plans/m1e2_correctness/p1e2.1b_non_monotone_programs/hole_census.md) | the read-out names up to four minimal holes |
| refuse, qualify, or search differently | D5 | § 6 | route 2, and the warnings |
| what a route may learn from a refutation resting on an `absent` | D6 | § 3's learned-nogoods row; [S1e2.1b.4](../../../plans/m1e2_correctness/p1e2.1b_non_monotone_programs/s1e2.1b.4_refutation_resting_on_absent.md), its [taint census](../../../plans/m1e2_correctness/p1e2.1b_non_monotone_programs/taint_census.md) | nothing, on a flagged program |
| the search past the oracle's limit | D7 | § 6.4 | partial past `--region-limit`, the cube when asked |

Each decision's consequences, per program, are § 8.1, open as the table is.

**Ruled**, and not open: the static check reads at relation grain (2026-11-02,
[S1e2.1b.1](../../../plans/m1e2_correctness/p1e2.1b_non_monotone_programs/s1e2.1b.1_the_site_census.md#the-question-it-puts-to-the-user)),
and *forbid* (2026-11-02, S1e2.4.1 § 5).

### 8.1 Each option's consequences, per program

Written 2026-11-02 by M1e2's
[S1e2.1b.7](../../../plans/m1e2_correctness/p1e2.1b_non_monotone_programs/s1e2.1b.7_the_consequences_written_down.md),
T1e2.1b.7.4. **Every table in § 8.1 is open, and every decision in it is
[F21](../../../plans/followups/f21_non_monotone_programs_decided.md)'s.** A
cell says what the option would move on the program, taken from the census
page it names, or re-taken here with the command beside it, at `ed8a13f2`, the
release build. *Lines* are the program's lines of
`ein-cli/tests/golden/corpus_exits.txt` and
`ein-render/tests/golden/corpus_shapes.md5`, counted from those files at the
same commit: they bound what a build would move, and which of them move is
the build's to say. Where the agent has a recommendation it is marked as the
agent's, and it is not a ruling.

#### D2: what a node's state is

R, k_s and the states, from `node_census` (§ 9–10 there, `bc1dff73`, and
re-taken with `node_census -- --json` for the fixtures banked since), the
stratum-order column from `node_census -- --stratum-order` (§ 12.4 there),
(F) from `node_census -- --paths` (§ 11). Every program not in the table
moves under no option of D2: the other 13 of the 19 flagged corpus cubes
within the limit, and every monotone cube within it but `11_fifo_gap_static`.
Past the limit only `features/12` and `features/04` are measured.

| program | lines | (A) fork, today | (B) scratch | (C) over (A) | (C) over (B) | (D) over D · over all | stratum order | (F) from P · from root |
|---|---|---|---|---|---|---|---|---|
| `features/07`, twin `f07-fpoff` | 6 + 35, 1 + 35; `W8/features-07` | R = {∅, {(q one)}}, k_s 2; `{(q one)}` holds `(p one) (q one)`; the written order of two rules moves the universe | R the same, `{(q one)}` holds `(q one)` alone; the order still moves the universe | R = {∅}, k_s 1: `{(q one)}` is the third outcome. `[case.now]` reads `[[]]` for `[["(q one)"]]` | as (B) | **refused**, a load error | no stratum order | (B)'s states · `{(q one)}` has none: R = {∅} |
| `stale-naf-fork`, twin `-warn` | 1 + 35 each; `W8/stale-naf-fork` | R = {∅}, k_s 1: `{(q one)}` dead on the stale `(p one)` | **R = {∅, {(q one)}}, k_s 2**: the register's `right` | R = {∅}; `{(q one)}` the third outcome, dead on a stale fact | as (B) | stays: stratified | nothing moves: the fork keeps root's `(p one)` | as (B) · as (A) |
| probe `not_conclusion` | none (a probe) | R = {∅, {(q A)}}; `{(q A)}` holds the stale `(not (p A))` | `{(q A)}` loses it | R = {∅} | as (B) | stays | nothing moves | as (B) · R = {∅} |
| `d3-unsat-core` | none; `W1/d3-unsat-core`, `W11/d3-unsat-core` | R of 20 | the same: no node differs | the same: no stale node | the same | **refused**: both rows re-stated with a refusal as `right` | no stratum order | (A)'s state on every node, both ways: the other path states are stale |
| `lattice/10_fifo_gap` | 6 + 35 | R = all 4; `{a, b}` holds the stale `p` | the same: root derives nothing | **R loses `{a, b}`**, k_s 4 → 3, maximal line `{a}`, `{b}` | as (C) over (A) | stays: stratified | **`{a, b}` loses `p`**; R unchanged | `{a, b}`'s state without `p`, both ways |
| `lattice/11_fifo_gap_static`, monotone | 6 + 35 | R = 2; `{b}` holds `(q one)` | **`{b}` holds a stale `p`** | nothing moves | **R loses `{b}`**, k_s 2 → 1: a monotone program's R | stays | under (B), `{b}` is the fork's again | the fork's state, both ways |
| written `stratified_stale` | none | R = {∅, {(r one)}}; `{(r one)}` holds two stale facts | `{(r one)}` holds `(p one) (r one)`; root keeps its stale `(p one)` | **R = ∅** | R = {{(r one)}} | stays | **the universe moves**, on both routes: root becomes `{(q one)}` | R = {{(r one)}} · R = ∅: ∅ has no stale-free state either way |
| written `dead_root`, `dead_root_via` | none | R = ∅ (L0) | **R = {{(h A)}}** | R = ∅ | as (B) | stay | nothing moves | as (B) · as (A) |
| `features/04_open`, n = 81 | 5 + 35 | partial past the limit: 64 761 members of R met | not measured | **21 042 of the 64 761 members met hold a stale `(open-slot …)` and leave R** (`detector_proposal.md` § 7) | not measured | stays: stratified | not run | not run |
| `features/12`, n = 20 | 7 + 35; four `:expect`s | R = ∅ by L0 | R = ∅, measured: all 1 048 576 scratch nodes dead (§ 6.2); L0 is no longer its reason | unchanged, argued at fact grain | unchanged | stays: stratified | not run: past the limit | not run |
| three `syntax/` fixtures, n = 0 | 2 + 35 each | — | — | — | — | stay · **refused** | no stratum order | — |

What each costs, and what it settles, from the same pages:

| | stable under rule order | what it leaves stale | cost |
|---|---|---|---|
| **(A)** | no on `features/07`, `f07-fpoff`, and on `10_fifo_gap`'s state (`node_census.md` § 7.1, § 12.1) | the fork's: W8's family, `not_conclusion`, the FIFO fixture | as today |
| **(B)** | no on the same, and on `stratified_stale`, until admission follows the strata | the FIFO races: `10_fifo_gap`, `11_fifo_gap_static`, `stratified_stale` | saturation ×0.2–5.6 a node; root re-derived in every node; L0 is no longer a construction |
| **(C)** | does not settle it (§ 7.2 there) | nothing in R: a stale state is the third outcome | a matcher call per fact with an `absent` premise (`detector_proposal.md` § 5) |
| **(D)** | settles it on the corpus, by refusing the two unstable programs | as the reading it is combined with | a graph pass at load |
| **stratum order** | yes on every stratified program read (§ 12.4 there) | the fork's, as (A); from scratch nothing | a change to the saturator's admission, emulated here by priorities |
| **(F)** | depends on the order: it counts what the boundary's paths reach (§ 11.5 there) | nothing: a state that holds a stale fact is not counted | the state DAG: the census's 19 910 steps for the 256 flagged nodes, ≈ 3ⁿ against the cube's 2ⁿ |

The agent's recommendation in `node_census.md` § 10, (B) with (D) and
admission in stratum order, is unchanged by these measurements, and it is the
agent's, not a ruling: under it every node read is stale-free from scratch,
and § 12.2's monotone program needs the same admission order.

#### D1: option (E), under each reading of F1 and F2

`cargo run --release -p ein-infer --example refutation_census -- --option-e`
(new with this stage; `ed8a13f2`), and `site_census --json`'s per-site fact-grain
verdict for F1 (c). (E) refuses every saturation rule that reads at odd
`absent` depth a relation a commitment can extend, whatever it concludes.
**F2 does not move (E)'s set**: every `via` refutation on the corpus sits
downstream of an `absent` rule (E) already refuses (`stale-naf-fork`'s
`derive-p`, `d3-unsat-core`'s `fire-1`, `features/12`'s `forced`). F2 moves
what *forbid* already covers, the baseline (E) is counted against.

| F1 reading | (E) refuses | *forbid* refuses, direct · with `via` | (E) adds, over direct · over `via` | what the reading itself changes |
|---|---:|---:|---:|---|
| **(b) in D**, the static check's | **21** | 9 · 12 | 12 · **9** | — |
| **(a) eligible**, S1e.2.3's | 23 | 10 · not computed: the `via` set is read over D | 13 · — | drops `d3-unsat-core` (its `absent` reads `(not r1)`, which no generator proposes) and `lattice/10_fifo_gap` (`q` is derived, not eligible); adds `features/03_forall`, **monotone** (n = 5, `undefeated` reads `player`), and three `syntax/` fixtures with n = 0 |
| **(c) fact grain**, the prototype | 19 | 7 · 10 | 12 · 9 | `features/05` and `/12` leave: their reads ask for membership in a type, which no candidate has |

The 9 that (E) adds under F1 (b) and F2 with `via`, each with what it would
move as a refusal. Which of its cells a refusal moves is S1e2.2.5's F3 (only
the subcommands that build a lattice, or every one that loads); the lines are
the program's, all of them, 44 of `corpus_exits.txt` and 385 of
`corpus_shapes.md5` in all:

| program | n | lines | register | `:expect` |
|---|---:|---|---|---|
| `features/07_unstratifiable` | 1 | 6 + 35 | `W8/features-07` | none |
| `ein-bugs/wrong-answers/f07-fpoff` | 1 | 1 + 35 | `W8/features-07` (twin) | none |
| `features/01_not_and_absent` | 35 | 6 + 35 | | none |
| `features/04_open` | 81 | 5 + 35 | | none |
| `lattice/08_absent_witness_hole` | 3 | 6 + 35 | | none |
| `lattice/10_fifo_gap` | 2 | 6 + 35 | | none |
| `saturation/square-unique/corner-house`, `cul-de-sac`, `terminus` | 118, 114, 153 | 4 + 35 each | | none |

What each would be written as instead, and whether that keeps R, is § 8.2.

Under F2 direct alone (E) also adds `d3-unsat-core` (no lines; `W1/` and
`W11/d3-unsat-core`, both fixed, re-stated with a refusal as `right`) and W8's
`stale-naf-fork` pair (1 + 35 each; `W8/stale-naf-fork`). (E) at F1 (b)
leaves a monotone, path-independent state on every program that loads, by
§ 7.7's argument, and the path census is consistent with it: every program
§ 11 and § 12 find path-dependent is refused by (E) or by *forbid*, but two.
`ACTIVATOR_REFUTES` is caught only if the refusal reads the plans a node
compiles (F3), and `11_fifo_gap_static` depends on the path only between fork
and scratch, so its fork state is path-independent.

#### D3: path dependence as a constraint on P1e2.1c

From the path census (`node_census.md` § 11) against
[P1e2.1c](../../../plans/m1e2_correctness/p1e2.1c_the_region_search/README.md)'s
region search, which builds a node by extending a parent where it can:

| | what P1e2.1c may do | what it meets on the flagged corpus within the limit |
|---|---|---|
| **(i)** required of every program | extend a parent on any program that loads, once a reading under which a node is a function of C is taken: (B) with stratum order, or (E) | under today's fork, 20 of 256 nodes reach another state along some path; 10 along alive parents, and 3 change status that way, `lattice/08`'s hole moving into R among them. On the probe `via_hole` and on `ACTIVATOR_UNDER_ABSENT` a node of R is dead along an alive parent, so a route that extended parents would lose it |
| **(ii)** required of monotone programs only | extend parents on a monotone program only, by fork; on a flagged one, build every node from root at once (today) | no path-dependent node on the 151 monotone cubes read (n ≤ 12), by fork. § 12.2's monotone program depends on the path only between fork and scratch, so a route that extends by fork is unaffected |

**Until decided, (ii)**, and S1e2.1c.2 and .4 are scoped to monotone programs.

#### D4: what is reported when R has holes

[`hole_census.md` § 10](../../../plans/m1e2_correctness/p1e2.1b_non_monotone_programs/hole_census.md#10-the-consequences-per-option)
(`c92d204e`) has the four options' consequences per program, and they stand.
The two fixtures banked since add no hole: `lattice/10_fifo_gap` has none, and
`lattice/11_fifo_gap_static` is monotone. So (a) and (b) move the same 18
lines; (c) moves `10_fifo_gap`'s 6 read-out lines too, **156**; and (d) refuses
**27** of the 210 programs that load and saturate (26 corpus entries and
`d3-unsat-core`), 9 of them conservative.

#### D6: what a route may learn from a refutation resting on an `absent`

[`taint_census.md` § 4](../../../plans/m1e2_correctness/p1e2.1b_non_monotone_programs/taint_census.md#4-what-b-buys-over-a)
(`b6a3045b`): under (b), with the taint following each plan's activator, a
route may skip 91 of the 256 flagged nodes, 88 of them on `three_components`;
under (a), none. The path census adds the other half: the paths that depend on
a dead parent (§ 11.2, c2's note in § 7.9) are exactly the ones a route that
learned from a tainted death would take, and the taint calls every one of
those deaths tainted. `lattice/10_fifo_gap` has no dead node and moves
nothing.

#### D7: the search past the oracle's limit

The guard split (§ 6.4), re-taken with `python3 utils/guard_split.py --corpus`
at `ed8a13f2`: **22 programs within the limit, 20 agree with the oracle node for
node**, `lattice/10_fifo_gap` among them, and W8's two still disagree, because
a cell builds from scratch. Past the limit the same 8 programs, with g from
0 to 49. Under (b) of D7 nothing changes but the limit; under (c), today's,
`features/04_open` keeps its partial answer and 21 042 of its 64 761 members
met hold a stale fact (`detector_proposal.md` § 7).

### 8.2 (E)'s rewrites

Written 2026-11-02 by S1e2.1b.7, T1e2.1b.7.5, and **open** as § 8.1 is: what
(E) would make the author write is evidence for F21's D1, not a ruling. For
each of the 13 programs § 7.7 lists, and `lattice/10_fifo_gap` banked since,
the rewrite the program's own text points to, from the menu S1e2.4.1 measured
for *forbid*: the decided attribute of
[`dual_rail.ein`](../../../plans/m1e2_correctness/p1e2.4_what_the_switch_left/s1e2.4.1_probes/dual_rail.ein),
the relation closed (`:no-hypothesis`), an obligation with hrules, an explicit
choice, or *no rewrite that keeps R was found*. Each is banked as a probe in
[`s1e2.1b.7_probes/`](../../../plans/m1e2_correctness/p1e2.1b_non_monotone_programs/s1e2.1b.7_probes/)
with its re-take command in its header. Where n ≤ 16 the probe carries the
original's R, as `ein solve` prints it, as its own `(solve :expect (region …))`,
and `ein test` holds it; past the limit the header gives its verdict and
route. Taken with the release build at `4502e841`.

| program | n | the rewrite | probe | keeps R | verdict and route, the original → the rewrite |
|---|---:|---|---|---|---|
| `features/07`, twin `f07-fpoff` | 1 | **an explicit choice**: each thing takes one side, P or Q (`functional` + `total-owed`), and the side derives `p` or `q` | `features07_choice.ein` | **yes, as printed**, naming `q`: `(region (model) (model (q one)))`. The states are (B)'s: here the state with `q` holds `q` alone, where the fork's holds the stale `p` too, and the empty commitment owes the choice | flagged, route 2 → monotone, route 1 |
| `stale-naf-fork`, twin `-warn` | 1 | **the relation closed**, `(solve :no-hypothesis (q))` | `stale_naf_fork_closed.ein` | **yes**: `(region (model (p one)))`, which also holds on the original as written. The dead node `{(q one)}` leaves the cube | flagged, route 2 → monotone, route 1, no candidate |
| ″ | | **the decided attribute**: `q-val` yes or no, `p` from the decided *no* | `stale_naf_fork_dual_rail.ein` | **no**: R is `(p one)` and `(q one)`, (B)'s R and the register's `right` for W8; the original's claim fails on it | → monotone, route 1 |
| `lattice/08_absent_witness_hole` | 3 | **the decided attribute**: `allergic-val` yes or no, every guest owing it; the soup on the decided *no* | `lattice08_dual_rail.ein` | **yes, verbatim**: the original's five states. Its owing hole is still owing and no longer between two members of R, so R has no hole | flagged, route 2 → monotone, route 1 |
| `lattice/10_fifo_gap` | 2 | **the reader deleted**: r2 never fires in the perfect model | `fifo_gap_without_r2.ein` | **yes, as printed**, naming `a` and `b`; `{a, b}` loses the stale `p`, as under stratum order | flagged, route 2 → monotone, route 1 |
| `corpus/fuzz_findings/d3-unsat-core` | 5 | **no rewrite that keeps R was found**: the read made positive gives 24 states, `r1` closed gives 8 over 3 candidates and stays flagged, against the original's 20 | `d3_unsat_core_positive_read.ein`, the closer one | no | flagged, route 2 → monotone, route 1, a different R |
| `features/12_expect_false` | 20 | the relation closed, S1e2.4.1's | [`features12_scoped.ein`](../../../plans/m1e2_correctness/p1e2.4_what_the_switch_left/s1e2.4.1_probes/features12_scoped.ein) | **yes**: R = ∅ on both, `(false)` | route 1 by L0 → route 1 by L1, monotone |
| `features/05_stdlib_domain_elim` | 56 | the relation closed, S1e2.4.1's: one line, `(solve :no-hypothesis (is-a))`; it closes `domain-elimination`'s read too | [`features05_scoped.ein`](../../../plans/m1e2_correctness/p1e2.4_what_the_switch_left/s1e2.4.1_probes/features05_scoped.ein) | past the limit: the original's R is not known | flagged, route 2, partial → monotone, route 1, **exact: 32 states** over 26 candidates |
| `features/01_not_and_absent` | 35 | the relations closed, `likes` and `is-a`. `likes` alone satisfies (E) and leaves the activator site of a relation-grain name collision, on 17 candidates | `features01_closed.ein` | past the limit; root's fixpoint is unchanged | flagged, route 2, partial → monotone, route 1, **exact: R = {∅}**, the query's one class the original's largest |
| `features/04_open` | 81 | the relation closed, `house-color` | `features04_closed.ein` | past the limit; root's fixpoint is unchanged | flagged, route 2, partial → monotone, route 1, partial at `--region-budget`: 53 candidates, k_s ≥ 24 859, complete up to commitments of 3 (0.45 s) |
| `saturation/square-unique/corner-house`, `cul-de-sac`, `terminus` | 118, 114, 153 | the relation closed, `next-to` | `square_unique_{corner_house,cul_de_sac,terminus}_closed.ein` | past the limit; root's fixpoint, the corner inference among it, is unchanged (`ein saturate`'s counts are identical) | flagged, route 2, partial → monotone, route 1, partial at `--region-budget`: 78, 77 and 104 candidates, k_s ≥ 79 158, 75 851 and 186 637, complete up to commitments of 3 (1.3, 1.4 and 5.6 s) |

**What it comes to.** Of the 13: within the oracle's limit, **5 of 6 have a
rewrite that keeps R** (2 of them, W8's `features/07` pair, as `ein solve`
prints it and with (B)'s states, not the fork's), and **1, `d3-unsat-core`,
has none that was found**. Past the limit, `features/12` keeps R (∅), and the
other 6 have a monotone rewrite whose R cannot be held to the original's,
which is not known; two of them become exact. The fixture banked since keeps
R as printed. Every rewrite is monotone, so **none needs (E)'s refusal to be
read as anything but a scope or a decision**: on the corpus, an `absent`
concluding a positive fact was always one of three things, a relation the
program states and should close, an attribute the search should decide, or a
choice the program should state.

**What the rewrites cost the programs.** Each changes the candidate universe:
closing a relation removes candidates, a decided attribute adds one per value,
an explicit choice adds the sides. So *keeps R* is R as the read-out prints it,
over the relations the original names; the commitments that reach each state
differ. The two W8 programs cannot keep both their text's intent and today's
answer: the closed rewrite keeps today's R, the decided attribute gives the
register's `right`.

## Cross-references

- [`solution_semantics.md`](solution_semantics.md): R defined, for the
  monotone case; § 2.1 is this page's starting point.
- [`region_dump.md`](region_dump.md): the oracle's directory, and the static
  check it reports.
- [`04_dot_rendering.md` § Region rendering](../ir/03-ein-lang/04_dot_rendering.md#region-rendering--the-cube-and-a-region-run):
  how § 4's drawings are read.
- [`absent_semantics.md`](absent_semantics.md): what an `absent` means at fire
  time, which is why a fork and a scratch build can differ.
- [`lattice_diagrams.md`](lattice_diagrams.md): **superseded**, the 2026-05
  design's search lattice, not the cube drawn here.
- The phase: [P1e2.1b](../../../plans/m1e2_correctness/p1e2.1b_non_monotone_programs/README.md),
  its [site census](../../../plans/m1e2_correctness/p1e2.1b_non_monotone_programs/site_census.md)
  and its [node census](../../../plans/m1e2_correctness/p1e2.1b_non_monotone_programs/node_census.md).
- [F21](../../../plans/followups/f21_non_monotone_programs_decided.md): every
  decision this page leaves open, with § 7's analysis and the user's words.
