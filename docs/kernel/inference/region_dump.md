# The region dump — `ein solve --dump-region DIR`

**The answer as the region of the lattice, computed without the search.**
The answer to a program is the set of commitments that are alive and owe
nothing, `R`, a region of its hypothesis lattice
([Q-M1e2.7](../../../plans/m1e2_correctness/open_questions.md#q-m1e27--the-answer-is-a-region-of-the-lattice)).
The search is one way to find `R`, and it is the thing under review. This
flag computes `R` another way, by brute force: every subset of the root
candidate list, each saturated by the program's rules alone. It then writes
`R` beside this run's walk over the same lattice. M1e2
[S1e2.1a.1](../../../plans/m1e2_correctness/p1e2.1a_the_solution_region/s1e2.1a.1_the_oracle.md)
added it. The computation is
[`ein_infer::region`](../../../ein.rs/crates/ein-infer/src/region.rs), and the
static check it reports is
[`ein_infer::monotone`](../../../ein.rs/crates/ein-infer/src/monotone.rs).

```sh
ein solve examples/branching/15_lookahead_two_step_on.ein -e --dump-region out/
```

## The directory

```text
out/
  region.json   every node of the cube, its two bits and their reasons, R's lines,
                holes, components, sub-cubes, N, E, the duality, the violations,
                and the static check (schema ein-region/1)
  walk.json     what this run did with the lattice: its universe, every entering,
                the recorded nodes, the learned clauses, the root writes (ein-walk/1)
  region.md     the two for a reader, and what the computation cost
```

The flag runs **after** the solve and after every other artefact is written.
It runs on **its own load** of the program, because the oracle interns every
fact of `2ⁿ` fixpoints, and doing that in the solve's interner would renumber
facts other readers compare. The cube and the walk are therefore joined by
rendered fact text, and every commitment in both files is text. A commitment
in `walk.json` also carries the cube's `mask` when every one of its facts is
a coordinate of the cube, and `null` when one is not.

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
search starts from `alive₀`, which is taken through the lookahead and after
the forced-positive cascade, so the two lists can differ. `walk.json`'s
`universe` has both, and `region.md` names what the config dropped and what
the cascade promoted to root.

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
| `root_dead`, `assumes_monotone` | root's own fixpoint is contradictory; the static check found a site |
| `nodes` | one record per mask: `mask`, `layer`, `facts`, `status`, `minimal`, `maximal`, `hole`, `component`, `class`, `closure`, `excluded`, `beyond`, `beyond_hypothesised`, `owes`, `owed`, `core`, `refuted_by`, `absent_premises`, and `state` (the node's facts beyond root's) under `--region-states` |

## `walk.json`

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

Classifying the walk against the cube is
[`ein render region DIR`](../ir/03-ein-lang/04_dot_rendering.md#region-rendering--the-cube-and-the-walk-s1e21a2)'s
work (S1e2.1a.2). It marks each node entered, reached by saturation, pruned
and why, never generated, missing or spurious, and checks every learned clause,
refuted singleton and lookahead kill against the cube. `region.md` says, for
each recorded node, where its state sits, and which members of each line no
recorded model holds. Both compare **states** (`class`), not masks: a
commitment whose saturation derives a candidate records its closure's
state.

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
in `D`. **No site means the lattice is monotone**: *dead* and *discharged* are
both upward-closed, `R` has no holes, and its two lines describe it. That is
a sufficient condition, and the cube is its check: `region.md` flags a
monotone verdict beside an observed violation as a defect, and
`ein-infer/tests/region.rs` fails on one.

**The stub.** A program with a site reaches `solve`'s `non_monotone`. It
leaves the search unchanged and the answer unchanged, and says so in three
places:

- one line under the verdict, `lattice  not monotone (n sites) — this answer
  assumes it is`;
- a `monotonicity` event ([`events.md`](events.md));
- `--json-summary`'s `monotonicity` block, which every run carries, monotone
  or not.

What the region *is* on such a program — which route a node is built by, what
a hole means, whether a refutation may rest on an `absent` — is
[P1e2.1b](../../../plans/m1e2_correctness/p1e2.1b_the_non_monotone_lattice/README.md)'s.
On one, `region.md` still computes everything and marks what assumes a
monotone lattice.

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
| over the limit | not computed. `region.json` has `refused`, `region.md` says why, the walk is written, and stderr gets one `note:` line naming `--region-limit`. The exit code is the solve's |
| 17–19, raised | runs, after one `warning:` line with the projected time |
| ≥ 20, raised | runs, after a three-line `warning:`: `2ⁿ`, the cost per node measured on the first 64, the projected time |

## What it does not check

It checks the **walk**. It does not check the saturator, because both share
it (W7), and it does not check that the candidate list is complete (W4, W6).
The per-subset route, [`utils/region.py`](../../../utils/region.py), is a
second implementation for small programs. It runs one `ein saturate` per
subset, and `ein-cli/tests/region_dump.rs` holds the two to the same nodes on
Q-M1e2.7's seven programs.

## Cross-references

- [`answer_dump.md`](answer_dump.md): the answer the search recorded, as files.
- [`lattice_dump.md`](lattice_dump.md): every commitment the search tested.
- [`04_dot_rendering.md` § Region rendering](../ir/03-ein-lang/04_dot_rendering.md#region-rendering--the-cube-and-the-walk-s1e21a2):
  this directory drawn, `ein render region`.
- [`solution_semantics.md`](solution_semantics.md): the definitions this
  checks the walk against.
