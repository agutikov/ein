# S1f.10.17 — Describe by borders

**Phase:** [P1f.10](README.md), track R (the search)
**Status:** planned 2026-11-16, on the user's ruling of Q-M1f.311 that day. Running: [§ Record](#record-2026-11-16).
**Depends on:** [S1f.10.3](s1f.10.3_the_restricted_join.md) (the memory `known_dead`, L8),
[S1f.10.5](s1f.10.5_ordering.md) (the tree's order), [S1f.10.14](s1f.10.14_exactly_one_spent.md) (L9; Q-M1f.311's
numbers), [S1f.10.13](s1f.10.13_the_price_list.md) (the rule for *go*).

## The ruling

[Q-M1f.311](../open_questions.md#q-m1f311--a-describe-the-budget-would-stop-answer-what-the-tree-can-reach-instead),
option (e), put by the main session on 2026-11-16 in answer to the user's *"how describe could work the same way as
--single but when there are potentially R contains many nodes?"* and *"what is more generic approach, independent
from k_c = k_s?"*. The user, the same day:

> YES! a describe that visits each predicate's borders instead of every alive state - lets pick this variant

## Context

Describe takes the state route (`states.rs`). It visits every alive state, each once with its one-up extensions,
and reads every row of its description off what it met (`described.rs`: the states with their classes, generators
and line flags, `k_c`, the minimal commitments, N, dead alone, the shape, and an empty R's owing frontier). That is
correct and costs every alive state.

Where most alive states owe, nearly all of the work is states that are not in R. Q-M1f.311's record, S1f.10.14's
T4, measured it:

| program | describe | of it, on alive owing states | what the tree reaches |
|---|---:|---:|---|
| `send_more_money.ein` | stopped at the budget, 336 101 saturations | 98.3 % | `--single` proves R is one state in 18 030 |
| `zebra2-minus-15.ein` | stopped at the budget, 7 261 | 76.2 % | `-n 1000000` lists all 32 states in 104 |

## The design ruled, to be measured before it is built

Every quantity describe reports is a **border** of a monotone predicate over the cube, or a count over one. On a
monotone program:

| what describe reports | the predicate and its border | how it is found |
|---|---|---|
| R's **states** | *in R*, up-closed among the alive | the tree exhausted (L5, L9), then the alive states above its leaves (L2, L3): `-n N`'s route with no N |
| per state, **its class** `{C ⊆ s : cl(C) = s}`: its commitments, its **generators**, `k_c` | *generates s*, an up-set inside `2^s` because cl is monotone; its minimal elements are the minimal generators, the maximal elements outside it are the maximal closed proper subsets `m_j` of `s` | dualize and advance inside `2^s` with cl as the oracle, saturating subsets of `s` only; `|class(s)| = 2^|s| − |⋃_j 2^{m_j}|` |
| the **minimal line** | R's minimal commitments: the alive minimal discharged sets | duality against the maximal owing states, `--minimal`'s route |
| the **maximal line** | R's maximal states | the climb and dualize and advance, `--maximal`'s route |
| **N** | the minimal dead sets: the border of *alive* | dualize and advance between the maximal alive sets and the minimal dead sets |
| **dead alone** | the singletons of N | the singles |
| an empty R's **owing frontier** | the maximal owing states | the same duality as the minimal line |
| the **shape** | components and sub-cubes of R | read off R's states and lines |

Each border is found at a cost polynomial in its size and its dual's, quasi-polynomial at worst (Fredman–Khachiyan),
and not in the number of alive states. **That is the bet, and it is not measured**: the global borders, N and the
maximal owing states, can themselves be large, and on a program where L5 and L9 license no instance the tree
branches on every extension and is the state route again.

## Tasks

1. **T1f.10.17.1 — the inventory.** Every field of `Described` and every surface that reads it (the read-out, the
   summary, `--dump-answer`, `--dump-lattice`, `--count`, the region claim's check, `ein test`), mapped to the
   border it needs and the route that finds it, checked against the code. The goldens that will move named here,
   in a commit of its own, before any route changes: the `work` and `route` rows move, and with them
   `read_out_examples.md`, the shape digests, `cost_ms` and the slow set. A golden that moves and is not on the
   list is a stop.
2. **T1f.10.17.2 — the borders measured.** For every corpus program that searches: the size of each border (R's
   states, each class's `m_j` and generators, N, the maximal alive sets, the maximal owing states) and what finding
   each costs in saturations and facts beyond root, against the state route's describe, in an example beside
   `price_list`. On every program within the oracle's limit the borders are read off the cube as well, and the two
   readings agree. Within the default budget; a cell the budget stops is recorded as stopped, with what it had
   spent. **The rule for *go* is S1f.10.13's**, read as its *Defaults taken* read it, applied to describe and to
   `--count`, written in this file before the table is taken.
3. **T1f.10.17.3 — the route built**, where T2 says *go*: a border route in a module of its own, which `describe`
   and `--count` take on a monotone program, every node through the node function and the memory of what is known
   dead. Where it is *no* on some programs and *go* on the corpus, the stage says how describe chooses between the
   two routes, by something the run can see before it has the answer, and **no lever chooses it**. The state route
   stays, behind an internal switch on the calling thread (as `known_dead`'s with and without), as the reference.
   If T2 says *no* for describe on the corpus, **stop and report** with the table: the user picked this variant,
   and a *no* is theirs to see before anything else is decided.
4. **T1f.10.17.4 — held.** Every field of the description identical between the border route and the state route on
   every corpus program the state route finishes; both held to the oracle where it reaches (`ein-infer/tests/explore.rs`);
   the claims (`claim_routes.rs`); `lever_invariance`, `route_invariance`, `answer_round_trip`; a run the budget
   stops says what it established and no more, its counts lower bounds. A sweep through `ein_corpus::pool::map`,
   its slow half on a nightly tier within five minutes a shard.
5. **T1f.10.17.5 — the pages and the record.** [`read_out.md`](../../../docs/kernel/inference/read_out.md)'s `route` and
   `work` rows, `read_out_examples.md` regenerated,
   [`finding_r.md`](../../../docs/kernel/inference/finding_r.md) § 6 and the borders' formulas,
   `implementation.md`'s rows; the record here with every number's command; Q-M1f.311's *Built* paragraph.

## Acceptance

- The rule for *go* is written here before T2's table is taken.
- **Every field of the description is identical** to the state route's on every corpus program both finish, and to
  the oracle's where the oracle reaches.
- The work per program for describe and `--count` is recorded against the state route's, the programs made slower
  named, and the rule applied with the borders' cost included.
- Whether `send_more_money.ein`'s and `zebra2-minus-15.ein`'s describe now finish within the default budget is
  recorded, with what each costs.
- No lever. No relation, type or module read by name. No answer changes.
- `./run_tests.sh` and `./run_tests.sh --nightly` (in the runner's container) green on the batch's last commit,
  since a route changed.

## What it must not do

- **Not drop a field of the description.** A field the border route cannot find within the budget is reported as
  what was established, a lower bound or *not computed*, as a stopped run's are today.
- **Not take the border route on a flagged program.** A flagged program takes the cube, as every mode does.
- **Not count a saved saturation without the cost that bought it.**
- **Not run a test past the default budget.**

## Record (2026-11-16)

### T1: the inventory, written before any route changed

Taken on `0686103e`, against the code: [`described.rs`](../../../ein.rs/crates/ein-infer/src/described.rs)'s
`of_states`, [`states.rs`](../../../ein.rs/crates/ein-infer/src/states.rs)' `describe_with`, `explore.rs`'
`describe_states`, and every reader below.

**Which fields describe on a monotone program fills, and what reads each.** *R* marks a field of `R` or around it,
which T4 holds identical between the routes and to the oracle; *run* a field of the run, which the route is free to
fill its own way and which the `route` and `work` rows print. A `Described` built by the state route (`Route::States`)
is the only one a monotone program's describe or `--count` makes today; a dead root is decided there in one
saturation, and a flagged program takes the cube or route 1, neither of which this stage touches.

| field | kind | the border it needs | how the border route finds it | read by |
|---|---|---|---|---|
| `candidates`, `monotonicity` | R | none: `H` and the static verdict | root's preparation, as now | every surface |
| `root` (alive, owes, owed, core, sources) | R | none: root's node | root's node, one saturation, as now | `∅`, `owed`, `empty` rows; summary `region.root`, `root_owes`, `root_owed`; `--trace`'s headline; `ein test`'s claims |
| `any_alive` | R | root's node | root alive | `empty` row; claims; summary `empty` |
| `exhausted` | R | every state of `R` found | the tree exhausted (L5, L9), then every state of `R` met extended by every candidate (L2, L3): `-n N`'s route with no `N` | `exhausted` row; the listing; claims; summary `region.exhausted` |
| `states[].closure` | R | *in R*'s states | as `exhausted` | the listing; `--dump-answer`'s state files; claims; summary `region.states` |
| `states[].generators`, `rep` | R | per state `s`, the minimal elements of *generates s* inside `2^s` | dualise and advance inside `2^s`, `cl` the oracle: the maximal closed proper subsets `m_j` of `s` against the minimal transversals of `{s ∖ m_j}`; `rep` the first generator, as now | the listing (`(merged … (or …))`); `merged` row; claims; state files' `reached by` |
| `states[].commitments`, `k_c` | R | per state, `|class(s)|` | the sets of `2^s` meeting every `s ∖ m_j`, counted from the same edges the state route counts from (`transversal::count_hitting`) | `commitments (k_c)`; `merged`; components' sizes; claims; summary `region.k_c` |
| `states[].minimal`, `minimal_commitments` | R | `R`'s minimal commitments | the minimal sets among every state's generators (∅ where root is in `R`), as `states::describe_with` reads them now: no border of its own | `minimal line` row; summary `region.minimal`; sub-cubes |
| `states[].maximal` | R | `R`'s maximal states | a state of `R` every extension of which is dead (L4), each known from the extension step above | `maximal line` row; summary `region.maximal` |
| `shape.components`, `shape.subcubes` | R | read off `R`'s states and lines | as `states::describe_with`: union over the one-up edges inside `R`, and `[a … b]` over the two lines | `components`, `sub-cubes` rows; summary |
| `shape.holes`, `shape.discharging` | R | none on a monotone program | `None`, as now (`Some((0, []))` on a dead root, which keeps the state route) | `E`, `minimal hole` rows (route 2's) |
| `nogoods`, `dead` (with cores) | R | `N`, the minimal dead sets: the border of *alive*, with its dual the maximal alive sets | where every alive state met was extended (∅ in `R`), the duality on the maximal alive states met, as now; otherwise dualise and advance between the maximal alive sets and `N`, each transversal decided by the memory of what is known dead or by its node, an alive one climbed to a new maximal alive set. Cores taken after, as now (`AfterWalk::cores`) | `N` row; summary `region.nogoods`; `--dump-answer`'s `nogoods.json`; `--trace`'s N section |
| `nogoods_skipped` | R | — | as now: `N` not taken where no surface prints it (`--count`, `ein test`) | `N` row; summary |
| `dead_alone`, `dead_alone_set` | R | the singletons of `N`: `{b}` dead with root alive | `N`'s singletons where `N` is taken; root's one-up extensions otherwise | `universe` row; `ein solve`'s candidate listing (`ein-cli`'s `modes.rs`); summary `region.universe.dead_alone` |
| `dead_met` | R | some node is dead, which on a monotone program is ⊤ dead | on an exhausted run, some node the run met is dead: a state of `R` below ⊤ has a dead extension, and an empty `R` with nothing dead has ⊤ alive | the `maximal line` row's *⊤, the trivial line*; summary `region.maximal.trivial` |
| `frontier` | R | where `R` is empty: the maximal alive sets, all owing | the maximal alive sets of the same dualisation as `N` | the listing's `owing` rows; claims over an empty `R`; summary `region.frontier`; `--dump-answer`'s `owing/` |
| `owing_met` | run | — | some node the run met owes | no surface: `ein-infer/tests/obligation_reports.rs` alone |
| `route` | run | — | a new route | `route` row; summary; the claim's route word (`ein test -v`); `--dump-lattice`'s header |
| `exhausted` qualifications: `complete_up_to`, `stopped_before`, `states_partial` | run | — | the border route's own partial form, saying which border it had finished | `states (k_s)`, `exhausted`, lines', `N` rows' qualifications; summary `region.stopped` |
| `work` | run | — | the route's saturations and facts | `work` row; summary `work` |

The answer's own fields under describe (`explore::Answer`) are read off the same description: `states` (each state's
`rep` and closure; the state route lists them in the order met, the border route lists them in the description's
canonical order), `shape`, `counts`, `exhausted`, `stopped`, and the run's `licences`, `saturations`, `facts`,
`denials`, `order`. `ein-infer/tests/explore.rs` holds these to the oracle as sets.

**What `-n N` and the lines already give, and are reused.** `-n N` over all of `R` lists every state of `R` by the
tree and the climb above its leaves (`tree::find`), so the border route's first part is that function asked for no
`N`. The maximal line is read off the states it extends (L4), as the climb's test; `--maximal`'s dualise and advance
(`Search::next_top`) and its climb (`Search::climb`) are the steps the global border's search takes. The minimal line
is read off the classes' generators, which describe needs anyway, so `--minimal`'s duality against the maximal owing
states is **not** a step of describe: the maximal owing states are measured in T2 as a border, and not searched for.

**The goldens that will move**, where describe or `--count` takes the border route on a program, and nothing else.
`--count` has no line of its own in the shape golden; the modes' routes do not change, so no mode's line moves.

1. `ein.rs/crates/ein-render/tests/golden/corpus_shapes.md5`, per such program: `read-out[describe]` (the `route`
   and `work` rows; where the sweep's budget, 2^14, stopped describe and the border route stops elsewhere or
   finishes, every row a stopped run qualifies), `cores[describe]` (the summary's `route`, `licences`, `stopped` and
   `work`; `nodes.json`, the nodes the run visited, and their cores), `region-lattice` (the nodes visited) and
   `region-trace` (its route and work), and `states` only where the sweep's describe stopped. No line of a mode, of
   the oracle or of a frontend op.
2. `docs/kernel/inference/read_out_examples.md`: the `route` and `work` rows of each example whose program takes
   the border route, regenerated by `utils/read_out_examples.py`.
3. `ein.rs/crates/ein-render/tests/golden/trace_*.md`: the route and work lines of a trace whose program takes it.
4. `ein.rs/crates/ein-cli/tests/region_modes.rs`' literal read-out (`route`, `work`), if its program takes it.
5. `ein.rs/crates/ein-cli/tests/golden/corpus_exits.txt`: only a cell whose claim a stopped describe left
   unchecked and the border route decides.
6. `ein.rs/crates/ein-cli/tests/test_cli.rs`' claim lines that name the route (`(state route, k_s = 0, k_c = 0)`),
   if their programs take it.
7. `corpus/corpus.toml`: `cost_ms` of the entries whose describe or `--count` costs otherwise, and the slow set
   (`corpus_cli`'s `the_slow_set_is_exactly_these_fifteen`), re-taken only on an idle machine.
8. The pages: `read_out.md`'s `route`, `work` and `exhausted` rows, `finding_r.md` § 6, `implementation.md`'s
   rows, `answer_dump.md` if the summary's `stopped` block gains a key.

Not moving: any answer (`R`, its counts, its lines, `N`, a claim's outcome), any mode's read-out, the oracle,
`README.md`'s and `docs/guide/04_solving_the_whole_puzzle.md`'s transcripts (`--single`) and `docs/api/rust.md`'s
block.
