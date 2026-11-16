# S1f.10.17 — Describe by borders

**Phase:** [P1f.10](README.md), track R (the search)
**Status:** planned 2026-11-16, on the user's ruling of Q-M1f.311 that day. **Stopped after T2, 2026-11-16: *no* for describe on the corpus**, as the plan says to stop and show it; T3 to T5 not done. [§ Record](#record-2026-11-16).
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

### The rule for *go*, written before T2's table is taken

[S1f.10.13](s1f.10.13_the_price_list.md#the-rule-for-go-written-before-the-table-is-taken)'s rule, as its
[*Defaults taken*](s1f.10.13_the_price_list.md#defaults-taken) read it, applied to the border route as one way of
answering, for **describe** and for **`--count`** apart (`--count` takes no `N`):

- **the programs**: every corpus program and register witness that searches, by the determinism sweeps' one
  definition (`corpus_ops::searches`), whose static check calls it monotone and whose root is alive. A flagged
  program takes the cube and a dead root is decided in one saturation either way, so neither is compared;
- **the cell**: the question asked at the default `--region-budget` (2^20 facts), once by the state route and once
  by the border route, on the same binary, each with the options `ein solve` gives it (describe takes `N`, `--count`
  does not). Its work is the run's own `work` row: saturations, and facts beyond root. **Every saturation the border
  route spends is in its cell**, the tree's, the classes', the singles' and the global border's alike; nothing is
  simulated;
- **go** when the total over the programs is lower with the border route **in saturations and in facts beyond
  root**, and **no program's cell is higher by more than one tenth** in either unit; **no** otherwise, with the
  programs that lose named;
- **a stopped cell counts with what it had spent**, either route's; a unit in which a cell is 0 loses by any
  positive net;
- **"go on the corpus, no on some programs"** means the first clause holds and the second fails. Then describe
  chooses between the two routes by something the run can see before it has the answer, the rule is applied
  again to the route so chosen, and the choice is listed under *Defaults taken*;
- **beside the rule, not in it**: what each cell established (`R` exact or not, `N` exact or not), and the walls
  (the border route's transversal searches cost time and no saturation, as a core did in S1f.10.13), each said
  where it bears.

### T2: the borders measured — *no* for describe on the corpus, so the stage stops here

**The verdict in one paragraph.** By the rule above, **describe is *no* on the corpus**: over the 171 monotone
programs that search with an alive root, the border route's total is lower in saturations (817 794 against
1 234 630, all of it the two SEND cells) and **higher in facts beyond root** (11 058 806 against 10 286 321), and
20 programs are higher by more than a tenth, the four zebra puzzles among them. **`--count` is *go* on the corpus
with 8 losers**: 626 673 saturations and 9 154 455 facts against 1 234 630 and 10 286 321, `send_more_money.ein`
and its twin **decided in 18 127 saturations** where the state route stops after 336 101. The plan says a *no* for
describe on the corpus is the user's to see before anything else is decided, so **nothing is built**: no question of
`ein solve` takes the border route, no golden moved, and T3, T4 and T5 are not done. Every field of the two
descriptions is identical on every program both routes finish, and on every program within the oracle's limit the
border route's is the cube's.

#### What was built to measure, and what takes it

The borders past the state route's reach cannot be simulated from its nodes (on SEND and `zebra2-minus-15.ein` it
never finishes), so **T2 runs the route's own code, unwired** ([§ Defaults taken](#defaults-taken)):

- [`ein-infer/src/borders.rs`](../../../ein.rs/crates/ein-infer/src/borders.rs), the border route: `R`'s states by
  `tree::find` with no `N` (the tree, then every state of `R` extended), root's one-up extensions for the candidates
  dead alone, each state's class by dualise and advance inside `2^s`, and `N` with the maximal alive sets by dualise
  and advance over `H`; each transversal closed under what the run knows first (`known_closure`: the closure of
  every state met whose first commitment it holds), so the memory of what is known dead (L1, L8) decides it where it
  can. The module header has the arguments.
- `described::of_borders`, the description read off the borders; `explore::describe_borders`, its answer;
  `Route::Borders` with its words in the read-out, the summary, `--dump-lattice` and `ein test`;
  `Answer::borders` (what each border cost and how large it is, which no surface prints); three helpers of the node
  cache (`owed_of`, `root_node`, `cores_after`), `KnownDead::is_empty`, and the tree's search opened to the crate
  (`St::path_bits`, `Search::order_took`).
- **Taken by no question**: `explore::describe_by(…, Route::Borders)` and the example below alone call it; `run`
  is as it was. [`ein-infer/tests/borders.rs`](../../../ein.rs/crates/ein-infer/tests/borders.rs), two tests per
  commit on seven programs (4.5 s): the border route's description is the state route's, `N` taken and not, and
  the cube's where the oracle reaches; each fails when a description is mutated (tried on the final tree with three
  mutations of the maximal line and of a class's generators, and reverted).
- [`price_list --borders`](../../../ein.rs/crates/ein-infer/examples/price_list.rs), in
  [`examples/shared/borders.rs`](../../../ein.rs/crates/ein-infer/examples/shared/borders.rs): per program, describe
  and `--count` by both routes at the default budget with the options `ein solve` gives them (describe takes `N`,
  `--count` does not, no core), the work of each border and their sizes, the two descriptions compared where both
  finish and the border route's against the cube within `--region-limit`, the global borders' sizes off the state
  route's walk and off the cube, and the verdicts by the rule.

#### The command

```sh
cargo build --manifest-path ein.rs/Cargo.toml --release -p ein-infer --example price_list
EIN_TEST_JOBS=8 utils/bench_env.sh --cores P:8 ein.rs/target/release/examples/price_list --borders \
    --out build/s1f.10.17 > build/s1f.10.17/borders.out 2> build/s1f.10.17/borders.env
```

2026-11-16 from 12:37, the worktree over `7981400e` with this stage's code, the release example, eight P-cores, on
AC, load average 5.8 at the start (other work on the machine), the default budget of 2^20 facts. Its files, under
the untracked `build/s1f.10.17/borders/`: `borders.jsonl` (a line a program), `verdicts.json`, `borders.md`.
**Taken twice**: the counts of the two runs are one digest, the walls and two counters added between them left
out (`jq -c -S 'del(.describe.states.wall, .describe.borders.wall, .count.states.wall, .count.borders.wall) |
del(.describe.borders.borders.classes, .describe.borders.borders.global_met, .count.borders.borders.classes,
.count.borders.borders.global_met)' borders.jsonl | md5sum`: `cfc332bd993163f901427201458f6d12` both). The walls
were taken eight programs at a time on a loaded machine and are no measurement.

#### What holds the two readings to each other

| | |
|---|---|
| corpus programs and register witnesses that search | 200: 29 flagged (the state route refuses them; the cube is theirs), 171 compared, none with a dead root |
| both routes finish, describe and `--count` | 161 programs: **every field of `R` or around it identical**, 322 descriptions compared (`differ`) |
| the border route against the cube | the 140 within `--region-limit`: **identical** |
| the borders' sizes, off the border route, the state route's walk and the cube | `R`'s states, the maximal closed proper subsets, the generators, `N` and the maximal alive sets agree on all 161; the walk and the cube agree on the maximal owing states too, on all 140 |
| the same work both ways | 143 of the 161: every program with `∅ ∈ R`, where the border route's walk is the state route's and both borders are read off it with no node asked, and 23 where root owes and the border route's nodes and readings come to the state route's |

#### The totals, by the rule

| question | the state route | the border route | lower | higher by more than a tenth | verdict |
|---|---:|---:|---:|---:|---|
| describe | 1 234 630 · 10 286 321 | 817 794 · **11 058 806** | 2 | 20 | **no**: higher in facts on the corpus |
| … the 161 cells no budget stopped | 34 462 · 383 473 | 39 031 · 573 186 | 0 | 18 | no |
| `--count` | 1 234 630 · 10 286 321 | 626 673 · 9 154 455 | 16 | 8 | **go on the corpus, no on 8 programs** |
| … the 161 cells no budget stopped | 34 462 · 383 473 | 33 048 · 321 879 | 14 | 6 | no |

Saturations · facts beyond root. **Describe's losers**: `zebra.ein` (314 · 55 346 → 486 · 159 912), `zebra2.ein` and
`zebra2-obligations.ein` (263 · 22 503 → 368 · 53 443), the two `zebra2-minus-15` (7 261 · 545 818 → 11 192 ·
1 048 538, both stopped), `beyond_arith/cryptarithm.ein` (417 → 1 371), `equation_by_search.ein` (251 → 827),
`equation_two_unknowns.ein` (1 466 → 3 606), `lattice/05`, `06`, `07` and `14`, six of `binary_logic/`, and
`tests/stdlib/bool/inputs/01` and `02`. **`--count`'s**: the two `zebra2-minus-15` (as describe's),
`lattice/07_two_pairs_two_tables.ein` (177 · 448 → 149 · 556, in facts), four of `binary_logic/` (`equivalent`
30 · 215 → 38 · 281, `not_equivalent` 30 · 197 → 27 · 248 in facts, `truth_table` 30 · 179 → 39 · 450,
`unsatisfiable` 28 · 156 → 37 · 232) and `tests/stdlib/bool/inputs/01_in_drives.ein` (11 · 38 → 13 · 62). Where
describe's facts go: the two `zebra2-minus-15` cells spend the whole budget, 502 720 facts each beyond the state
route, which stops at half of it on its layer's projection; the six `saturation/square-*` demos, where `∅ ∈ R` and
both routes stop, about 54 300 each; the finished programs where root owes, 189 713; against SEND's two cells, 374 366
fewer each.

#### The six programs, border by border

Saturations · facts beyond root; the border route's work split by border (root's node, one saturation, apart).

| program | describe, the state route | describe, the border route | `R`'s states | singles | classes | `N` | `--count`, the border route |
|---|---:|---:|---:|---:|---:|---:|---:|
| `beyond_arith/send_more_money.ein` | 336 101 · 1 422 943, **stopped**, no state met | 110 696 · 1 048 577, **stopped in `N`** | 18 029 · 221 803 | 70 · 72 | 27 · 180 | 92 569 · 826 522 | **18 127 · 222 055, decided** |
| `…_written.ein` | the same | the same | the same | the same | the same | the same | the same |
| `zebra2-minus-15.ein` | 7 261 · 545 818, **stopped**, ≥ 32 states | 11 192 · 1 048 538, **stopped in the classes** | 107 · 11 512 | 94 · 2 230 | 10 990 · 1 034 796 | — | the same, stopped |
| `zebra2-minus-15-obligations.ein` | the same | the same | the same | the same | the same | — | the same |
| `zebra2.ein` | 263 · 22 503 | 368 · 53 443 | 6 · 666 | 54 · 4 816 | 59 · 3 304 | 248 · 44 657 | 120 · 8 786 |
| `zebra.ein` | 314 · 55 346 | 486 · 159 912 | 8 · 1 900 | 54 · 9 144 | 57 · 6 384 | 366 · 142 484 | 120 · 17 428 |

**The borders' sizes** on the same programs:

| program | states of `R` | `m_j`, summed | generators, summed | `N` | maximal alive | maximal owing | alive states | `N` decided by the memory |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| SEND, either | 1 | 8 | 1 | ≥ 1 131 | ≥ 5 912 | not found | ≥ 330 329 owing met by the state route | — |
| `zebra2-minus-15`, either | 32 | ≥ 259 | ≥ 7 171 | not reached | not reached | not found | not found | — |
| `zebra2.ein` | 1 | 3 | 63 | 366 | 8 | 7 | 20 | 192 |
| `zebra.ein` | 1 | 3 | 63 | 458 | 9 | 8 | 27 | 196 |

On SEND the global dualisation had found 5 912 maximal alive sets and 1 131 members of `N` when the budget stopped
it, 28 538 transversals in the round it was in, its third; on `zebra2-minus-15.ein` 18 of the 32 classes were
found, with 7 171 minimal generators between them, each asked by a saturation, when the budget stopped it there.

#### What the numbers say about the bet

- **The global border is larger than what it replaces wherever root owes.** Over the 161 programs the walk
  finishes, the alive states are 29 343 (1 434 of them owing), `N` 4 082, the maximal alive sets 997 and the
  maximal owing sets 904, and on 49 programs `|N|` and the maximal alive sets together outnumber the alive states.
  `zebra2.ein`: 20 alive states, `N` 366 and 8 maximal alive sets; 192 of the 366 decided by the memory, the rest
  each a node, and each maximal alive set a climb. The state route reads `N` off the alive states it has met for
  nothing; the border route pays for every member the memory cannot decide.
- **A state reached by many minimal commitments costs one node a commitment.** The class's dualisation must ask
  every minimal transversal it has not decided, and a generator is decided only by its node: `zebra2-minus-15`'s
  classes take a saturation per generator. The state route counts the same class off the states below it.
- **Where `R` is small and the alive owing states are many, it pays, and `N` sinks it again.** SEND's `R` is one
  state of eight candidates, its class 27 saturations, the singles 70. With `N` not
  taken, `--count` is 18 127 saturations, the tree's. With `N` taken, the global border is at least 5 912 maximal
  alive sets, more than the budget lets it finish.
- **`∅ ∈ R` gains nothing and loses nothing**: there every alive state is in `R`, the tree is the state route's
  walk, and every border is read off it with no node asked (143 cells of each question the same to the saturation).

#### Beside the rule: what a stopped run established

- **SEND, describe**: the border route stops in `N` with `R` exact, one state, `k_c` = 1, both lines, the shape and
  the candidates dead alone; the state route stops having met no state of `R`. **`--count`** decides it.
- **`zebra2-minus-15.ein`**: the border route has `R`'s 32 states exact and 18 classes; the state route has 32
  states as a lower bound. Neither has `k_c` or the lines.
- **`saturation/square-*`**: both stop with `∅ ∈ R`; the state route saturates ⊤ once and decides `R` as the whole
  cube, which the border route as measured does not do (it would, built, as the state route does).

#### Defaults taken

Each the user's to revise.

1. **T2 ran the route's own code, unwired**, and it is committed: the borders of the programs that decide the bet
   cannot be simulated from the state route's nodes, since it does not finish there. No question takes it
   (`explore::run` is as it was), no golden moved, and `tests/borders.rs` keeps it from rotting. If the user drops
   the variant, the module, the route's variant and the example's mode go with it.
2. **The rule's programs**: the monotone corpus programs and register witnesses that search, with an alive root
   (171), as the price list read *the corpus*.
3. **The minimal line read off the classes' generators**, not by `--minimal`'s duality against the maximal owing
   states: the classes are needed anyway, and the maximal owing states were measured as a border, not searched.
4. **The singles before the classes**: describe and `--count` both need the candidates dead alone, and their
   closures are what the classes' and the global border's transversals are closed under.
5. **A transversal is closed under the closures of the states met** before it is asked (`known_closure`), for a
   generator of a class and for a member of `N`: L3 and the monotone state, read off memory. With the singles before them it
   took `zebra2.ein`'s class from 108 saturations to 59, and saved nothing on `zebra2-minus-15.ein`'s.
6. **The global seed is the maximal alive states met**, and a climb is the maximal line's (`Search::climb`,
   ascending).
7. **A stopped border run spends the budget node by node**, as every tree step is afforded; the state route stops
   before a layer whose projection passes it. Read as the rule reads it, a stopped cell with what it spent.

#### What is not done

- **T3, T4, T5**: not built, on the plan's stop. Q-M1f.311's *Built* paragraph is not written: nothing is built.
- **A choice between the routes for `--count`** is not proposed as a rule: its losers are the `zebra2-minus-15` pair
  (the classes) and six small programs where the singles and the classes cost more than the alive states (2 to 9
  saturations, or facts alone); the user may want `--count` alone first.
- `cost_ms`, the slow set and every golden: untouched, nothing moved.
- `./run_tests.sh`, the nightly and `utils/ci_container.sh`: the main session's.

**Findings outside the stage's scope**: none; K-30 stays the last.
