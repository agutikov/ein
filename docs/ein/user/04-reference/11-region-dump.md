# The region dump — `ein oracle FILE --dump DIR`

**The answer as the region of the lattice, computed without a search.** The answer to a program is the set of
commitments that are alive and owe nothing, `R`, a region of its hypothesis lattice
([Q-M1e2.7](../../../decisions/answer/Q-M1e2.7-the-answer-is-a-region-r-of-the-hypothesis.md)). `ein solve` finds `R`
by a route that visits as little of the lattice as its question needs; `ein oracle` computes it by brute force: every
subset of the root candidate list, each saturated by the program's rules alone. It is the reference every `ein solve`
question is held to ([Q-M1e2.32](../../../decisions/answer/Q-M1e2.32-no-two-commands-disagree-and-the-oracle-is.md)),
and `--dump DIR` writes what it computed.

```sh
ein oracle examples/branching/15_lookahead_two_step_on.ein --dump out/
ein oracle examples/lattice/04_obligation_trigger_hole.ein --dump out/ --states   # + every node's state
```

`ein render region DIR` draws the directory, and `ein render region DIR --visited … --answer …` draws an `ein solve`
run on the cube, from that run's own `--dump-lattice` and `--dump-answer` ([`12-rendering.md`](12-rendering.md)).

## The directory

```text
out/
  region.json   every node of the cube, its two bits and their reasons, R's lines,
                holes, components, sub-cubes, N, E, the duality, the violations,
                and the static check (schema ein-region/1)
  region.md     the same for a reader, and what the computation cost
```

The directory is made **before** the run, and written once the cube is computed; over the limit it holds
`region.json` with `refused` and a `region.md` that says why. Every commitment in it is rendered fact text, so it does
not depend on the order names were met in.

## The cube

The lattice over `n` candidates is the `n`-cube. A node is an `n`-bit mask, an edge adds or removes one fact, and layer
`k` holds the `C(n, k)` masks with `k` bits set. Bit `i` is the `i`-th entry of `region.json`'s `candidates`, in
**rendered-text order**.

**The universe** is the generator's list at root's fixpoint with **no config field applied**: no option may change the
problem's lattice ([Q-M1e2.30](../../../decisions/answer/Q-M1e2.30-no-configuration-option-lever-or-flag-changes-r.md)),
and every route of `ein solve` takes the same list.

**What it leaves out** is named: each relation the program can conclude or ask about, does not scope out itself, and of
which the rung that built the list proposes no fact
([Q-M1e2.3](../../../decisions/answer/Q-M1e2.3-r-is-over-the-universe-and-the-read-out-names.md),
[`06-solution-semantics.md` § 1.4](../../reasoning/02-kb-evolution/06-solution-semantics.md#14-candidate-and-universe)).
`region.md` says it under its candidate count, as the read-out's `outside` row does, and `region.json` lists it as
`outside`.

**A node** is root's fixpoint with the commitment's facts written in and the saturation resumed to quiescence. It is
never stopped at the first contradiction, so a dead node's debts are its fixpoint's. Each node carries two
**independent** bits:

| bit | set when | its reason, in `region.json` |
|---|---|---|
| dead | the fixpoint holds a contradiction | `core` (the smallest frontier the core search finds), `core_status` (`within-budget`, or `cut-` and the caps the search stopped at; the oracle takes every dead node's core), `refuted_by` (the rules of the last step of the core's derivation: for each fact of the contradiction the core was taken over, the first recorded justification whose premises the core derives, `(given)` for a fact the core holds itself), `absent_premises` (the `absent`s its derivation rests on) |
| owing | an obligation the program states is undischarged — read on a **dead** node too | `owes`, `owed` (each instance through its `:why`) |

`status` is `R` (neither), `owing`, `dead`, or `dead+owing`. `class` is the smallest mask whose fixpoint is the same
fact set, which is how two commitments that reach one state show.

Four more fields say what saturation does to a node:

- **`closure`** is the mask of the candidates its fixpoint holds. It always contains the node's own mask, and it is a
  different node when the rules derive a hypothesised fact. That is the node saturation *moves it to* inside the cube.
  An alive node and its closure are one state.
- **`excluded`** is the mask of the candidates whose `(not h)` it holds.
- **`beyond`** counts the facts beyond root's that are not candidates.
- **`beyond_hypothesised`** counts the positive ones among them over a candidate's relation: facts a generator could
  have proposed and root's did not.

They are what a node *is* that its coordinates do not say.

## `region.json`

`schema`, `source`, `n`, `limit`, `candidates`, `later_debts` (written only where H holds a candidate for a debt root
does not owe: `--json-summary`'s `root.hypgen.later_debts`, one `{obligation, activator, relation, by, added,
candidates}` each), `outside` (one `{relation, arity, why}` per relation outside the universe, `--json-summary`'s
`root.hypgen.outside`, `[]` when none), `monotonicity` (below), then either `region` with
`refused: null`, or `region: null` with `refused` (`n`, `limit`, `note`). `region` holds:

| key | what |
|---|---|
| `size`, `minimal`, `maximal` | $\lvert R \rvert$ and its two lines, each a list of fact sets |
| `holes`, `minimal_holes`, `repairs` | nodes outside `R` between two members; the minimal ones; and every edge from a hole up into `R`, with the fact that revives it |
| `components` | `R`'s connected pieces under one-fact edges, each with its size and the line members inside it |
| `subcubes` | the maximal intervals `[from … to]` inside `R`, each with its `dimension` (its free facts). Without holes these are the comparable (minimal, maximal) pairs. With holes they are searched, and `null` if that search passed its work bound |
| `nogoods`, `discharging` | `N`, the minimal dead sets, and `E`, the minimal discharging sets (dead or not) |
| `duality` | without holes, the minimal line is `E`'s alive members and the maximal line is the discharged maximal sets containing no member of `N`; each side's verdict, and what it gave |
| `local_max`, `local_min` | nodes of `R` where the one-fact test and the whole cube disagree |
| `violations` | `dead_not_upward_closed` (a dead node with an alive superset) and `discharged_not_upward_closed` (a discharged node with an owing superset), each with a count and the smallest pair |
| `edges` | every upward edge, counted by the statuses at its two ends |
| `root_dead`, `assumes_monotone` | root's own fixpoint is contradictory; the static check flags the program, and the lines and components are computed over nodes built by fork from root (`region.md`'s *(nodes by fork)*) |
| `nodes` | one record per mask: `mask`, `layer`, `facts`, `status`, `minimal`, `maximal`, `hole`, `component`, `class`, `closure`, `excluded`, `beyond`, `beyond_hypothesised`, `owes`, `owed`, `core` and `refuted_by` (lists on a dead node, `[]` the empty set; `null` on a node that is not dead, whose core does not exist), `core_status` (`null` where no core was taken), `absent_premises`, and `state` (the node's facts beyond root's) under `--states` |

## Monotonicity — the static check

Two lines describe `R` only when `R` has no holes, and a hole needs a commitment that grows and takes a conclusion
away, or grows and adds a debt. The static check decides from the program's rules whether it can do either. Let `H` be
the relations of the candidate list, and `D` the relations a commitment can add a fact of: `H` closed under *a saturation
rule that reads `D` asserts into `D`*, with `(not (R …))` as its own `¬R`. Every read is then:

| class | condition | growth can… | example |
|---|---|---|---|
| static | not in `D` | nothing | zebra2's `absent`s over `right-of` / `next-to` |
| dynamic, monotone polarity | in `D`: even `absent` depth in a saturation rule, odd in an obligation | only add a conclusion, or only pay a debt | `total`'s `forall`; `total-owed`'s witness |
| dynamic, non-monotone polarity | in `D`: odd depth in a saturation rule, even (the trigger) in an obligation | take a conclusion away, or add a debt | a rule `derive-p` that reads `(absent (q ?x))` beside a rule `derive-q` that reads `(absent (p ?x))`; [`lattice/04`](../../../../examples/lattice/04_obligation_trigger_hole.ein)'s `total-owed` trigger |

A **site** is a read of the third kind, or a rule whose activator relation is in `D`. **No site means the program is
monotone**: *dead* and *discharged* are both upward-closed, `R` has no holes, and its two lines describe it. That is a
sufficient condition, and the cube is its check: `region.md` flags a monotone verdict beside an observed violation as
an engine defect, and `ein oracle` exits 1 on one.

**The grain is the relation's**: a read over a relation in `D` is dynamic whatever its pattern, even where no
candidate, and nothing derived from one, can match it.

**Which property the check is about.** The verdict is about the **program**, whose consequence operator the check
approximates; the lattice is the domain, and is never itself monotone or not. A site can break two properties of the
node functions, kept apart in
[`08-monotonicity.md` § 7](../../reasoning/02-kb-evolution/08-monotonicity.md#2-the-state-the-status-and-the-program):
the **state**'s, monotone in C and, where saturation is cumulative, independent of the path that built the node, which
only an `absent` breaks; and the **status**'s, which an obligation's trigger or an activator breaks with no `absent`.
`region.md`'s violations are the status's. A program whose cube shows none is a *conservative* flag.

**The warning.** A program with a site says so, and says what the check found, in these places:

- the read-out's `lattice` block, beside route 2, the cube, since route 1's licence is monotonicity:
  `not monotone: n sites (the static check) — h holes`, then what is assumed (`assumes`, every node a fork of root,
  [Q-M1e2.2](../../../decisions/answer/Q-M1e2.2-a-node-is-a-fork-of-root-an-assumption-on.md)), each `site` with
  what it puts at stake, each `cycle` and `order` finding, the `status` (the cube's violations, *exercised* by the
  sites they are attributed to, *conservative*, or *not known*), the `state` (the stale facts of every node the run
  computed) and the `paths` (the spot check, at most three parents of each printed state), then the minimal holes
  ([`08-read-out.md` § 5.1](08-read-out.md#51-under-lattice-and-monotone-the-warning));
- `ein solve FILE --monotone`, the static half alone, root's fixpoint and no node;
- a `monotonicity` event ([`05-events.md`](../../ein.rs/05-events.md)), with the static half;
- `--json-summary`'s `monotonicity` block, which every run carries, monotone or not: each site's `concludes` and
  `at_stake`, then `assumes`, `cycles`, `order`, `status`, `state` (every stale node, uncut) and `paths`. On a monotone
  program `assumes`, `status`, `state` and `paths` are `null` and the two lists empty, because `ein-summary/2` has one
  shape;
- `region.md`'s header and `region.json`'s `status` and `state`, which `ein render region` labels.

On such a program the engine answers under the reading that every node is a fork of root, warns, and claims nothing
about the program's solutions
([Q-M1e2.2](../../../decisions/answer/Q-M1e2.2-a-node-is-a-fork-of-root-an-assumption-on.md)). `region.md` still
computes everything, by fork from root, and marks what is computed over the nodes so built (its words: *nodes by
fork*).

### The `monotonicity` block, field by field

The block is the same in `--json-summary` (every run, monotone or not, `ein-summary/2` having one shape) and in
`region.json`, which adds `absents` and writes `status`, `state` and `paths` only on a flagged program. A list of
records is written `name[]`, and its rows name the fields of each element. Every field a run writes has a row here and
every row is a field some run writes; the test suite holds the table to the binary both ways.

| field | what |
|---|---|
| `monotone` | the verdict: no site, and nothing found in flight (`in_flight`, below) |
| `warning` | on a flagged program, the sentence every surface prints in capitals, that the program is not monotone and nothing is claimed of the answer ([Q-M1f.564](../../../decisions/answer/Q-M1f.564-a-non-monotone-program-loads-with-a-warning.md)); `null` on a monotone one |
| `hypothesised` | `H`, the candidates' relations, sorted |
| `dynamic` | `D`, the relations a commitment can change, sorted; `¬R` for a stored negative |
| `plans.saturation` | how many saturation plans the check read |
| `plans.obligation` | how many obligation plans it read |
| `reads.static` | every read of a relation outside `D`, in every plan, at any `absent` depth |
| `reads.dynamic_monotone` | every read of a relation in `D` whose polarity is monotone: even `absent` depth in a saturation rule, odd in an obligation |
| `reads.dynamic_non_monotone` | every read of a relation in `D` whose polarity is not: odd depth in a saturation rule, even (the trigger) in an obligation. Each is a site |
| `absent_reads.static` | as `reads.static`, counting only the reads inside an `absent` (depth 1 or more) |
| `absent_reads.dynamic_monotone` | as `reads.dynamic_monotone`, inside an `absent` |
| `absent_reads.dynamic_non_monotone` | as `reads.dynamic_non_monotone`, inside an `absent` |
| `absents[].rule` | `region.json` only: every read inside an `absent`, in site order; the rule that reads it |
| `absents[].activator` | the arguments of the activator its plan was compiled from, empty for a rule with none |
| `absents[].kind` | the plan's kind, `saturation` or `obligation` |
| `absents[].relation` | the relation read |
| `absents[].depth` | its `absent` depth |
| `absents[].class` | `static`, `dynamic-monotone` or `dynamic-non-monotone` |
| `sites[].rule` | each site, sorted and distinct: the rule |
| `sites[].activator` | its activator's arguments, empty for an activator site, which is about the rule |
| `sites[].kind` | `absent`, `obligation-trigger`, `activator`, `open-read` (a rule reads `open`), `hypothesis` (a rule that concludes `(open F)` reads a relation in `D` outside an `absent`, or inside an even number) or `obligation-guard` (an obligation's guard reads one beside its witness) |
| `sites[].relation` | the relation it reads in `D`, or its activator relation |
| `sites[].depth` | the read's `absent` depth, 0 for a top-level premise |
| `sites[].concludes` | what the rule concludes, the check's reading: `(false)`, `(not …)`, `a positive fact`, `(open …)`, or that root compiled no plan of it |
| `sites[].at_stake` | what the site puts at stake: the state, the state and death, or what the status owes, directly or through a node's plan |
| `in_flight` | the first hypothesis a state the run built and checked holds and root does not: a state printed, and every state under `--in-flight` ([Q-M1f.656](../../../decisions/cli/Q-M1f.656-a-flag-runs-the-pass-at-every-state-and-checks.md)), as its text: the program is not monotone, found in flight ([Q-M1f.574](../../../decisions/answer/Q-M1f.574-a-hypothesis-first-held-above-root-makes-its.md)), whatever its sites; `null` where the run built no such state |
| `assumes.node` | on a flagged program, how a node is built, `fork from root`; `assumes` is `null` on a monotone one |
| `assumes.open` | on a flagged program, `Q-M1e2.2`, the decision that says the answer rests on that reading of a node |
| `cycles[].relations` | each cycle through an `absent` over `D`, a strongly connected part of the rules' dependency graph with a negative edge in it: its relations, sorted |
| `cycles[].negative` | its negative edges, sorted, `rule: a →¬ b` |
| `cycles[].odd` | whether some cycle in it has an odd number of negative edges: a state with no stale fact need not exist; where none has, there can be several |
| `order[].reader` | each order finding: the rule that reads under an `absent` what another concludes |
| `order[].writer` | the rule that concludes it |
| `order[].relations` | the relations between them |
| `status.read` | what the status was read from, `the cube`, or `null` where it is not known |
| `status.bit` | whether a site bit on the cube: `exercised`, `conservative` or `not known` |
| `status.by` | the rules a violation is attributed to, on an exercised site; empty otherwise |
| `status.holes` | the cube's holes |
| `status.dead_not_upward_closed` | its dead nodes with an alive superset |
| `status.discharged_not_upward_closed` | its discharged nodes with an owing superset |
| `status.why` | where the status is not known, why: past `--region-limit`, the run did not read every node, or root is dead (L0) |
| `status.n` | there, the candidates |
| `status.limit` | there, the `--region-limit` the run took, where the reason has one |
| `state.read` | what the stale-fact check read: `the cube`, or every node the run computed |
| `state.nodes` | how many nodes that is |
| `state.stale_nodes` | how many of them hold a stale fact, a fact whose `absent` guard the node's own fixpoint falsifies |
| `state.stale[].node` | each such node, in the order the run computed it: its commitment |
| `state.stale[].class` | its class, `R`, `owing` or `dead` |
| `state.stale[].facts` | its stale facts, each with the rule and the `absent` that derived it |
| `paths.bound` | the parent-path spot check: the parents it reads per printed state, `null` under `-y`, which reads every one |
| `paths.states` | the printed states of two commitments or more |
| `paths.parents` | the parents it checked: alive, built, the fact written, saturated |
| `paths.dead_parents` | the parents it skipped as dead |
| `paths.saturations` | the saturations the check took |
| `paths.differ` | each path that reaches another state, `{state} by way of its parent without h` |
| `paths.unprinted.members` | where the read-out prints no state to check and the run met some it did not print: how many it met; `unprinted` is `null` otherwise |
| `paths.unprinted.saturations` | what checking theirs would cost at most |

## The limit

`--region-limit N`, default 16, is the largest `n` computed:

| `n` | behaviour |
|---|---|
| 16 or fewer | runs, silently |
| over the limit | not computed. `region.json` has `refused`, `region.md` says why, and stderr gets one `error:` line naming `--region-limit` and the value that would compute it; `ein oracle` exits 2, since its whole answer is the cube |
| 17–19, the limit raised | runs, after one `warning:` line with the projected time |
| 20 or more, the limit raised | runs, after a three-line `warning:`: the count of saturations, $2^n$, the cost per node measured on nodes spread over the cube and saturated before the run with the projected time at one job (`--jobs` shortens it by less than the jobs), and that the raised `--region-limit` asked for it |

## What it does not check

It checks the **routes**: every `ein solve` question is held to it. It does not check the saturation, which the two
share, and it does not check that the candidate list is complete: a relation outside the universe is named (`outside`,
above) rather than proposed.

## Cross-references

- [`10-answer-dump.md`](10-answer-dump.md): a run's answer, as files.
- [`06-lattice-dump.md`](../../ein.rs/06-lattice-dump.md): every node a run visited.
- [`12-rendering.md`](12-rendering.md): this directory drawn, with a region run on it, `ein render region`.
- [`13-solutions-and-models.md`](../03-syntax-and-semantics/13-solutions-and-models.md): what R is.
- [`08-monotonicity.md`](../../reasoning/02-kb-evolution/08-monotonicity.md): what the static check's two verdicts mean for R,
  compared, with one fixture and one drawing per cause.
