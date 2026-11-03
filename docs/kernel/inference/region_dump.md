# The region dump — `ein oracle FILE --dump DIR`

**The answer as the region of the lattice, computed without the search.**
The answer to a program is the set of commitments that are alive and owe
nothing, `R`, a region of its hypothesis lattice
([Q-M1e2.7](../../../plans/m1e2_correctness/open_questions.md#q-m1e27--the-answer-is-a-region-of-the-lattice)).
The search is one way to find `R`, and it is the thing under review. This
directory is `R` computed another way, by brute force: every subset of the
root candidate list, each saturated by the program's rules alone — the
reference every `ein solve` mode is held to. M1e2
[S1e2.1a.1](../../../plans/m1e2_correctness/p1e2.1a_the_solution_region/s1e2.1a.1_the_oracle.md)
added it. The computation is
[`ein_infer::region`](../../../ein.rs/crates/ein-infer/src/region.rs), and the
static check it reports is
[`ein_infer::monotone`](../../../ein.rs/crates/ein-infer/src/monotone.rs).

```sh
ein oracle examples/branching/15_lookahead_two_step_on.ein --dump out/
ein oracle examples/lattice/04_obligation_trigger_hole.ein --dump out/ --states   # + every node's state
```

**Until M1e2 S1e2.1a.5's switch it was `ein solve --dump-region DIR`**, which
wrote the cube beside the walk's own run, `walk.json`, so that a drawing could
place each entering on the cube. S1e2.1a.9 gave `ein oracle --dump DIR` the
same directory without a walk, and since the switch no command runs the walk:
`--dump-region` and `--region-states` are refused with what replaced them
(`ein oracle --dump`, `--states`). Since
[S1e2.1a.7](../../../plans/m1e2_correctness/p1e2.1a_the_solution_region/s1e2.1a.7_the_bundle_shows_r.md)
`ein render region DIR --visited … --answer …` draws a *region* run on the
cube, from that run's own `--dump-lattice` and `--dump-answer`.

## The directory

```text
out/
  region.json   every node of the cube, its two bits and their reasons, R's lines,
                holes, components, sub-cubes, N, E, the duality, the violations,
                and the static check (schema ein-region/1)
  region.md     the same for a reader, and what the computation cost
```

The directory is made **before** the run, and written once the cube is
computed; over the limit it holds `region.json` with `refused` and a
`region.md` that says why. Every commitment in it is rendered fact text, so
it does not depend on interner ids.

## The cube

Without merging, the lattice over `n` candidates is the `n`-cube. A node is
an `n`-bit mask, an edge adds or removes one fact, and layer `k` holds the
`C(n, k)` masks with `k` bits set. Bit `i` is the `i`-th entry of
`region.json`'s `candidates`, in **rendered-text order**, so the cube does not
depend on interner ids.

**The universe** is the generator's list at root's fixpoint with **no config
switch applied**: no lookahead filter, no kill cache, and the obligations rung
at its default order whatever `EIN_OBLIGATION_CHOICE` says
([`generate_config_free`](../../../ein.rs/crates/ein-infer/src/hypgen.rs)).
The user ruled on 2026-10-15 that no config option may change the problem's
lattice, so a candidate the lookahead would have dropped is still a node. The
walk started from `alive₀`, taken through the lookahead and after the
forced-positive cascade, so the two lists could differ, and `walk.json`'s
`universe` had both. Every route since the switch takes this list.

**A node** is root's fixpoint with the commitment's facts written in and the
saturation resumed to quiescence. It is never stopped at the first
contradiction, so a dead node's debts are its fixpoint's. None of the search's
machinery runs: no lookahead, no forced positive, no kill cache, no writeback,
no learned clause. Each node carries two **independent** bits:

| bit | set when | its reason, in `region.json` |
|---|---|---|
| dead | the fixpoint holds a contradiction | `core` (the search's own smallest frontier), `refuted_by` (the rules), `absent_premises` (the `absent`s its derivation rests on) |
| owing | an obligation the program states is undischarged — read on a **dead** node too | `owes`, `owed` (each instance through its `:why`) |

`status` is `R` (neither), `owing`, `dead`, or `dead+owing`. `class` is the
smallest mask whose fixpoint is the same fact set, which is how two
commitments that reach one state show (Q-M1e2.7's (e)).

Four more fields say what saturation does to a node
([S1e2.1a.2](../../../plans/m1e2_correctness/p1e2.1a_the_solution_region/s1e2.1a.2_the_lattice_drawn.md)):

- **`closure`** is the mask of the candidates its fixpoint holds. It always
  contains the node's own mask, and it is a different node when the rules
  derive a hypothesised fact. That is the node saturation *moves it to* inside
  the cube. An alive node and its closure are one state: on the corpus,
  32 074 alive nodes move and none lands on another state (2026-10-15,
  `census_for_inspection`).
- **`excluded`** is the mask of the candidates whose `(not h)` it holds.
- **`beyond`** counts the facts beyond root's that are not candidates.
- **`beyond_hypothesised`** counts the positive ones among them over a
  candidate's relation: facts a generator could have proposed and root's did
  not.

They are what a node *is* that its coordinates do not say.

## `region.json`

`schema`, `source`, `n`, `limit`, `candidates`, `monotonicity` (below), then
either `region` with `refused: null`, or `region: null` with `refused`
(`n`, `limit`, `note`). `region` holds:

| key | what |
|---|---|
| `size`, `minimal`, `maximal` | `|R|` and its two lines, each a list of fact sets |
| `holes`, `minimal_holes`, `repairs` | nodes outside `R` between two members; the minimal ones; and every edge from a hole up into `R`, with the fact that revives it |
| `components` | `R`'s connected pieces under one-fact edges, each with its size and the line members inside it |
| `subcubes` | the maximal intervals `[from … to]` inside `R`, each with its `dimension` (its free facts). Without holes these are the comparable (minimal, maximal) pairs. With holes they are searched, and `null` if that search passed its work bound |
| `nogoods`, `discharging` | `N`, the minimal dead sets, and `E`, the minimal discharging sets (dead or not) |
| `duality` | without holes, the minimal line is `E`'s alive members and the maximal line is the discharged maximal sets containing no member of `N`; each side's verdict, and what it gave |
| `local_max`, `local_min` | nodes of `R` where the one-fact test and the whole cube disagree |
| `violations` | `dead_not_upward_closed` (a dead node with an alive superset) and `discharged_not_upward_closed` (a discharged node with an owing superset), each with a count and the smallest pair |
| `edges` | every upward edge, counted by the statuses at its two ends |
| `root_dead`, `assumes_monotone` | root's own fixpoint is contradictory; the static check flags the program, and the lines and components are computed over nodes built by fork from root (`region.md`'s *(nodes by fork)*). The name is kept from before M1e2 S1e2.1b.5; it means what this row says |
| `nodes` | one record per mask: `mask`, `layer`, `facts`, `status`, `minimal`, `maximal`, `hole`, `component`, `class`, `closure`, `excluded`, `beyond`, `beyond_hypothesised`, `owes`, `owed`, `core`, `refuted_by`, `absent_premises`, and `state` (the node's facts beyond root's) under `--states` |

## `walk.json` — until the switch

`ein solve --dump-region` wrote it, and no command has since M1e2 S1e2.1a.5;
its writer in `ein-render/src/dump/region.rs` went with the walk at M1e2
S1e2.1c.5 (2026-11-03).
It is described here because a directory dumped before the switch carries one.
`ein render region` read it until S1e2.1a.7, and does not since: a build of
`7ec8a480`, the last whose `ein solve` ran the walk, draws such a directory.

`schema`, `source`, `verdict` (`type`, `k`, `exhausted`, `monotone`),
`bounds` (the run's `max_set_size` and `solutions`, `null` under `-e`),
`universe` (`proposed`, `alive0`), and from the run's proof:

- `promotions`: each forced positive the cascade wrote into root, with
  `after_enterings`, how many enterings came before it (S1e2.1a.2);
- `enterings`: every commitment entered, with its layer, its `outcome`, and
  `state_mask`, the cube node it **stood on**: the commitment plus every
  promotion before it;
- `recorded`: each model or open state, as `model` / `open`, with its debt and
  its `state_mask`, placed the same way;
- `dead`: each refuted commitment's clause and core;
- `learned_nogoods`, `alive_at_end`;
- `root_writes`: every fact the search wrote into root after its fixpoint.

A `state_mask` is `null` when a fact of the commitment is not a coordinate.

Classifying the walk against the cube was `ein render region DIR`'s work
from S1e2.1a.2 to S1e2.1a.7: it marked each node entered, reached by
saturation, pruned and why, never generated, missing or spurious, and checked
every learned clause, refuted singleton and lookahead kill against the cube.
A *region* run is classified the same way since, by state (`class`), against
the part of R it asked
([§ Region rendering](../ir/03-ein-lang/04_dot_rendering.md#region-rendering--the-cube-and-a-region-run)).

## Monotonicity — the static check

Two lines describe `R` only when `R` has no holes, and a hole needs a
commitment that grows and takes a conclusion away, or grows and adds a debt.
`ein_infer::monotone` decides from the compiled plans whether a program can
do either. Let `H` be the relations of the candidate list, and `D` the
relations a commitment can change: `H` closed under *a saturation plan that
reads `D` asserts into `D`*, with `(not (R …))` as its own `¬R`. Every read is
then:

| class | condition | growth can… | example |
|---|---|---|---|
| static | not in `D` | nothing | zebra2's `absent`s over `right-of` / `next-to` |
| dynamic, monotone polarity | in `D`: even `absent` depth in a saturation rule, odd in an obligation | only add a conclusion, or only pay a debt | `total`'s `forall`; `total-owed`'s witness |
| dynamic, non-monotone polarity | in `D`: odd depth in a saturation rule, even (the trigger) in an obligation | take a conclusion away, or add a debt | W3's `bad`, W8's `derive-p`, [`lattice/04`](../../../examples/lattice/04_obligation_trigger_hole.ein)'s `total-owed` trigger |

A **site** is a read of the third kind, or a rule whose activator relation is
in `D`. **No site means the program is monotone**: *dead* and *discharged* are
both upward-closed, `R` has no holes, and its two lines describe it. That is
a sufficient condition, and the cube is its check: `region.md` flags a
monotone verdict beside an observed violation as a defect, and
`ein-infer/tests/region.rs` fails on one.

**The grain is the relation's**, ruled by the user on 2026-11-02 (M1e2
[S1e2.1b.1](../../../plans/m1e2_correctness/p1e2.1b_non_monotone_programs/s1e2.1b.1_the_site_census.md#the-question-it-puts-to-the-user)):
a read over a relation in `D` is dynamic whatever its pattern, even where no
candidate, and nothing derived from one, can match it. A fact-grain reading
was prototyped beside it and held to the cube, and it is not what ships.

**Which property the check is about.** The verdict is about the **program**,
whose consequence operator the check approximates; the lattice is the domain,
and is never itself monotone or not. A site can break two properties of the
node functions, kept apart in
[`monotonicity.md` § 7](monotonicity.md#7-two-properties-of-the-node-functions-and-path-dependence):
the **state**'s, monotone in C and, where saturation is cumulative,
independent of the path that built the node, which only an `absent` breaks;
and the **status**'s, which an obligation's trigger or an activator breaks
with no `absent`. `region.md`'s violations are the status's. A program whose
cube shows none is a *conservative* flag.

**The warning.** Since M1e2's
[S1e2.1b.5](../../../plans/m1e2_correctness/p1e2.1b_non_monotone_programs/s1e2.1b.5_the_detector.md)
(the wording the user approved on 2026-11-03), a program with a site says so,
and says what [`ein_infer::detector`](../../../ein.rs/crates/ein-infer/src/detector.rs)
found, in these places:

- the read-out's `lattice` block, beside route 2, the cube, since route 1's
  licence is monotonicity: `not monotone: n sites (the static check) — h
  holes`, then what is assumed (`assumes`, every node a fork of root, and what
  a node is open: Q-M1e2.2, F21), each `site` with what it puts at stake, each
  `cycle` and `order` finding, the `status` (the cube's violations,
  *exercised* by the sites they are attributed to, *conservative*, or *not
  known*), the `state` (the stale facts of every node the run computed) and
  the `paths` (the spot check, at most three parents of each printed state),
  then the minimal holes as before;
- `ein solve FILE --monotone`, the static half alone, root's fixpoint and no
  node;
- a `monotonicity` event ([`events.md`](events.md)), with the static half;
- `--json-summary`'s `monotonicity` block, which every run carries, monotone
  or not: each site's `concludes` and `at_stake`, then `assumes`, `cycles`,
  `order`, `status`, `state` (every stale node, uncut) and `paths`. On a
  monotone program `assumes`, `status`, `state` and `paths` are `null` and the
  two lists empty, because `ein-summary/2` has one shape;
- `region.md`'s header and `region.json`'s `status` and `state`, which
  `ein render region` labels.

What the region *is* on such a program — which route a node is built by, what
a hole means, what may be learned from a refutation that rests on an
`absent` — is open: M1e2's
[P1e2.1b](../../../plans/m1e2_correctness/p1e2.1b_non_monotone_programs/README.md)
documents it, and the decisions are
[F21](../../../plans/followups/f21_non_monotone_programs_decided.md)'s
(2026-11-02). On one, `region.md` still computes everything, by fork from
root, and marks what is computed over the nodes so built (its words: *nodes
by fork*).

**Measured** 2026-10-15, with `EIN_REGION_CENSUS=1 cargo test -p ein-infer
--test region census_for_inspection -- --nocapture`: of the 183 corpus files
that load and saturate, **150** have a cube of at most 16 candidates and
**17** have a site. The cube observes a hole on exactly three of them:
`ein-bugs/naf-upward-closure.ein`, `ein-bugs/wrong-answers/naf-nowb.ein` and
`lattice/04_obligation_trigger_hole.ein`. The zebra family and every program
under `tests/stdlib/` have none.

## The limit

`--region-limit N`, default 16, is the largest `n` computed. The thresholds
are the user's ruling of 2026-10-15:

| `n` | behaviour |
|---|---|
| ≤ 16 | runs, silently |
| over the limit | not computed. `region.json` has `refused`, `region.md` says why, and stderr gets one `error:` line naming `--region-limit`; `ein oracle` exits 2, since its whole answer is the cube (under `ein solve --dump-region` it was a `note:` and the solve's code) |
| 17–19, raised | runs, after one `warning:` line with the projected time |
| ≥ 20, raised | runs, after a three-line `warning:`: `2ⁿ`, the cost per node measured on 64 nodes spread over every layer (saturated and discarded before the run; since S1e2.1a.8 — the first 64 in layer order were the cheapest, and projected 22.9 s for `branching/10`, n = 18, which ran past 11 minutes on 8 jobs), the projected time at one job (`--jobs` shortens it by less than the jobs: a worker hands back every node that numbers a new fact) |

## What it does not check

It checks the **routes**: every `ein solve` mode is held to it
(`ein-infer/tests/explore.rs`). It does not check the saturator, because they
share it (W7), and it does not check that the candidate list is complete (W4,
W6).
The per-subset route, [`utils/region.py`](../../../utils/region.py), is a
second implementation for small programs. It runs one `ein saturate` per
subset, and `ein-cli/tests/region_dump.rs` holds the two to the same nodes on
Q-M1e2.7's seven programs.

**From the other side**, [`utils/region_synth.py`](../../../utils/region_synth.py)
writes a program for a chosen region, carrying the target in its header
(M1e2
[S1e2.1a.8](../../../plans/m1e2_correctness/p1e2.1a_the_solution_region/s1e2.1a.8_region_to_program.md)).
`ein-infer/tests/region.rs` holds the oracle to the banked ones in
`examples/lattice/synth/` node for node. `ein-cli/tests/region_dump.rs` does
the same for 300 random targets, and requires the static check to call each
program monotone exactly when its target is. The one target it refuses, a dead
root under a non-empty R, is the fork route's limit: every node is built from
root's fixpoint.

## Cross-references

- [`answer_dump.md`](answer_dump.md): a run's answer, as files.
- [`lattice_dump.md`](lattice_dump.md): every node a run visited.
- [`04_dot_rendering.md` § Region rendering](../ir/03-ein-lang/04_dot_rendering.md#region-rendering--the-cube-and-a-region-run):
  this directory drawn, with a region run on it, `ein render region`.
- [`solution_semantics.md`](solution_semantics.md): the definitions this
  computes.
- [`monotonicity.md`](monotonicity.md): what the static check's two verdicts
  mean for R, compared, with one fixture and one drawing per cause.
