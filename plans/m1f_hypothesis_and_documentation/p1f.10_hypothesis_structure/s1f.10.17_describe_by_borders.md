# S1f.10.17 — Describe by borders

**Phase:** [P1f.10](README.md), track R (the search)
**Status:** planned 2026-11-16, on the user's ruling of Q-M1f.311 that day. Stopped after T2, *no* for describe on the corpus; resumed the same day on the user's ruling on the measurement, and **T3 to T5 done as ruled**: describe and `--count` take the tree and the classes by default and the state route on `--describe-route states`. [§ Record](#record-2026-11-16).
**Depends on:** [S1f.10.3](s1f.10.3_the_restricted_join.md) (the memory `known_dead`, L8),
[S1f.10.5](s1f.10.5_ordering.md) (the tree's order), [S1f.10.14](s1f.10.14_exactly_one_spent.md) (L9; Q-M1f.311's
numbers), [S1f.10.13](s1f.10.13_the_price_list.md) (the rule for *go*).

## The ruling

[Q-M1f.311](../open_questions.md#q-m1f311--a-describe-the-budget-would-stop-answer-what-the-tree-can-reach-instead),
option (e), put by the main session on 2026-11-16 in answer to the user's *"how describe could work the same way as
--single but when there are potentially R contains many nodes?"* and *"what is more generic approach, independent
from k_c = k_s?"*. The user, the same day:

> YES! a describe that visits each predicate's borders instead of every alive state - lets pick this variant

## The ruling on the measurement (2026-11-16)

T2 measured the whole border route *no* for describe on the corpus, and the cheap part, the tree and the classes,
exact on R where the state route stops. The user, shown the table:

> S1f.10.17 - 2 modes: state route (existing) and tree plus classes, second by default, cmdline option to force one
> or another, verbose warning about budget so user knows about forecast and options.

So **T3 is redefined**: describe and `--count` have two routes, the state route as it is and **the tree and the
classes**, which is the default; a command-line option forces either; where a route's forecast passes the budget the
run says so, with the forecast and the options. N by dualize and advance, the part that did not pay, is not a route.
This ruling is the user's, and it supersedes the plan's *no lever chooses it*: the option chooses a route and never
R.

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
   [`finding_r.md`](../../../docs/ein/reasoning/kb-evolution/finding-r.md) § 6 and the borders' formulas,
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
20 programs are higher by more than a tenth, five of the six zebra files among them. **`--count` is *go* on the corpus
with 8 losers**: 626 673 saturations and 9 154 455 facts against 1 234 630 and 10 286 321, `send_more_money.ein`
and its twin **decided in 18 127 saturations** where the state route stops after 336 101. The plan says a *no* for
describe on the corpus is the user's to see before anything else is decided, so **nothing is built**: no question of
`ein solve` takes the border route, no golden moved, and T3, T4 and T5 are not done. Every field of the two
descriptions is identical on every program both routes finish, and on every program within the oracle's limit the
border route's is the cube's.

#### What was built to measure, and what takes it

The borders past the state route's reach cannot be simulated from its nodes (on SEND and `zebra2-minus-15.ein` it
never finishes), so **T2 runs the route's own code, unwired** ([§ Defaults taken](#defaults-taken)). It is at
`580e79fc`, and T3 as ruled renamed and cut it (`tree_classes.rs`, the global border removed):

- `ein-infer/src/borders.rs`, the border route: `R`'s states by
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
  is as it was. `ein-infer/tests/borders.rs`, two tests per
  commit on seven programs (4.5 s): the border route's description is the state route's, `N` taken and not, and
  the cube's where the oracle reaches. Probed on the final tree and reverted: a maximal line read as *at most two
  alive extensions* and a class's transversal taken for a generator one candidate short both fail both tests; a
  maximal line read as *at most one* passes, no state of the seven having exactly one alive extension.
- `price_list --borders`, in `examples/shared/borders.rs`: per program, describe
  and `--count` by both routes at the default budget with the options `ein solve` gives them (describe takes `N`,
  `--count` does not, no core), the work of each border and their sizes, the two descriptions compared where both
  finish and the border route's against the cube within `--region-limit`, the global borders' sizes off the state
  route's walk and off the cube, and the verdicts by the rule.

#### The command

```sh
git checkout 580e79fc   # the measured code; T3 removed the global border and the mode
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

### T3 as ruled: the inventory again, for the default route, before it changes

On the user's ruling ([§ The ruling on the measurement](#the-ruling-on-the-measurement-2026-11-16)), with `master`
merged at `1f61ec37`. Describe and `--count` on a monotone program whose root is alive get two routes, **the state
route** as it is and **the tree and the classes**, the default; an option of `ein solve` forces either. T2's global
border, `N` by dualise and advance over `H`, is removed with what only it read (`borders::global`, `Global`, `Met`,
`known_dead`, the `global_*` counters).

**Where the default route is the state route.** The tree and the classes differ from the state route only where the
tree has something to branch on: root owes an instance L5 or L9 licenses. Where `∅ ∈ R` the tree is root alone and
its climb is every alive state; where root owes and no instance is licensed the tree branches on every extension;
where there is no candidate root is all of it. In each the walk visits the state route's states and extensions, in another order (T2: equal work on
every such program the corpus has that both finish, 143 cells of each question), and reads every field the state
route reads.
So **the default takes the state route there, read off the program before any node** (`oblgen::alternatives` at
root's fixpoint, no saturation), and the `route` row names it; the tree and the classes are taken where root owes a
licensed instance. A dead root keeps the state route's one saturation.

**What the tree and the classes compute, against the inventory of T1** (every other field of that table is
unchanged):

| field | the tree and the classes | words |
|---|---|---|
| `nogoods`, `dead` | **not computed**, unless ⊤ is in `R` (then nothing is dead, `N` is empty, as the state route reads it); the members met are the candidates dead alone, each a minimal dead set since root is alive | the `N` row: *not computed by the tree and the classes; `--describe-route states` computes it*; summary `region.nogoods` as for `N` not taken |
| `frontier`, where `R` is empty | **not computed**: the maximal alive sets are the global border; the members met are the owing states extended by every candidate with none alive | the listing says the frontier is not computed and lists the members met; a region claim over it is not checked by this route |
| a partial run | stopped in `R`'s states, in the singles or in the classes, with how many classes were found | each row a stopped run qualifies, and the stderr note with the forecast and the options |
| everything else | as the state route's: `R`'s states, each class (generators, `rep`, commitments), `k_c`, both lines, the shape, the candidates dead alone, `dead_met`, root | — |

**`ein test`.** A claim needs `N` nowhere (`claim.rs` reads `R`'s states, their classes and an empty `R`'s
frontier). A region claim over an empty `R` of the owing kind needs the frontier, so where the default route leaves
it uncomputed the claim is not checked by that run and `ein test` asks the describing question again by the state
route, as it asks a second question where the first leaves a claim undecided.

**The goldens that will move**, named before the route changes; one that moves and is not here is a stop:

1. `ein.rs/crates/ein-render/tests/golden/corpus_shapes.md5`: for each corpus program on which root owes an
   instance L5 or L9 licenses, `read-out[describe]` (the `route` and `work` rows, the `N` row, an empty `R`'s
   listing, and where the sweep's budget 2^14 stops either route, the rows a stopped run qualifies and its stderr
   note), `cores[describe]` (the summary's `route`, `licences`, `stopped`, `work`, `nogoods`, `frontier`;
   `nogoods.json`; `nodes.json`, the nodes visited), `region-lattice` (the nodes visited, the route's words) and
   `region-trace` (its route, work and `N`); `states` only where the sweep's describe stops otherwise. **And on
   every program whose describe the sweep's budget stops, by either route, `read-out[describe]`'s stderr note**,
   which now carries the forecast and the options. No mode's line, no oracle line, no frontend op.
2. `docs/kernel/inference/read_out_examples.md`, regenerated: each example whose describe or `--count` takes the
   tree and the classes, and each example of a stopped describe (the note).
3. `ein.rs/crates/ein-render/tests/golden/trace_*.md`: a trace whose program takes the tree and the classes.
4. `ein.rs/crates/ein-cli/tests/golden/help_shape.txt`: `ein solve`'s new option.
5. `ein.rs/crates/ein-cli/tests/region_modes.rs`, `model_set_report.rs`, `test_cli.rs` and `cli_semantics.rs`
   literals that print a describe's `route`, `work` or `N` row, or a stopped describe's note, of a program that
   takes the tree and the classes or stops.
6. `ein.rs/crates/ein-cli/tests/golden/corpus_exits.txt`: a cell whose claim a stopped describe left unchecked and
   the default route decides; an exit code moves only if such a claim is refuted.
7. `corpus/corpus.toml`'s `cost_ms` and the slow set, where describe's cost moves; re-taken only on an idle
   machine.
8. The pages: `read_out.md` (the `route`, `N`, `exhausted` rows, the partial forms, the stderr note),
   `finding_r.md` § 6, `configuration.md`, `implementation.md`, `answer_dump.md` where a key's words change,
   `AGENTS.md`'s command block.

### T3 as ruled: the two routes, built

**What describe does now.** On a monotone program whose root is alive, `explore::run` asks `tree_classes::branches`
whether root owes an instance L5 or L9 licenses, off root's fixpoint and the obligation pass's table, with no node.
Where it does and `Options::describe_route` is `Tree` (the default; `ein solve --describe-route tree`), describe and
`--count` take **the tree and the classes** (`ein_infer::tree_classes`): `R`'s states by `tree::find` with no `N`,
root's one-up extensions for the candidates dead alone, each state's class by dualise and advance inside `2^s`. Where
it does not, or under `--describe-route states`, they take **the state route** as it was. A flagged program, a dead
root and every mode are as they were. `Answer::describe_alt` names the other route where there is one, which a
stopped run's note reads.

**What the tree and the classes say they did not compute** (`described::of_tree_classes`):

- **`N`**: `nogoods_skipped`, the members met the candidates dead alone. Where ⊤ is in `R` it is empty and known, and
  where every state the run met was extended by every candidate it is read off the states as the state route reads
  it, no node asked: the duality, not the dualisation T2 measured. The `N` row says *not computed by the tree and the
  classes — ≥ k minimal dead sets, the candidates dead alone; --describe-route states computes N*; the summary's
  `region.nogoods` is `null`; `nogoods.json` holds the members met; `--trace`'s heading says what they are.
- **An empty `R`'s owing frontier**: `frontier_skipped`, the members met the owing states extended by every candidate
  with no alive extension. The listing says so, lists them, and prints no `(region (owing …))`; the summary's
  `region.frontier` is `null`; a region claim over an empty `R` is not checked (`read_out::check_claim`), and `ein
  test` asks the describing question again with `--describe-route states` where that left the program's solve claim
  unchecked (`ein-cli/src/test.rs`; the row's route is `describe --describe-route states`).
- **A stopped run** (`described::TreePartial`): the part it stopped in. In `R`'s states: a lower bound, the maximal line
  those met with every extension dead. In the singles: every state of `R`, the maximal line exact. In the classes:
  the same and the commitments of the states whose class was found, `k_c` a lower bound counting one for each other.
  A query over all of `R` or its maximal line is exact past `R`'s states.

**The warning, by default.** A describe or `--count` the budget stops writes, on stderr, `note: FILE: partial —` with
the forecast, what was established, what was not, and the options. The state route's:

```text
note: examples/zebra2-minus-15.ein: partial — the state route stopped before the states of 3 candidates at
--region-budget 1048576: their 12 289 extensions project 921 675 facts on top of the 545 596 spent. Established:
every alive state of up to 2 candidates, R's 32 met among them; not established: R's other states, k_c, the lines,
the shape and N. --region-budget 1 467 271 or more takes the next layer; --describe-route tree finds R's states by
the obligation tree and their classes, and not N
```

and the tree and the classes':

```text
note: examples/zebra2-minus-15.ein: partial — the tree and the classes stopped in the classes, those of 18 of R's 32
states found at --region-budget 1048576: the next step projects 93 facts on top of the 1 048 538 spent. Established:
R's 32 states, every one, its maximal line, and the commitments of 18 of them; not established: k_c, the minimal line
and the shape; N is not computed by this route. A larger --region-budget takes more steps; --describe-route states
visits every alive state and computes N
```

(Each one line on stderr; wrapped here.) The `exhausted` row says the same in short. `note:` and not `warning:`:
the read-out's stderr has no `warning:` line and every partial run's line is a `note:`
([`read_out.md` § 10](../../../docs/kernel/inference/read_out.md#10-stderr)).

**What was removed**: T2's global border (`borders::global`, `Global`, `Met`, `known_dead`, the `global_*` and
`maximal_alive` counters), `Route::Borders` (now `Route::TreeClasses`, with the words *the tree and the classes*),
`explore::BorderWork` (now `TreeWork`, the three parts' work), `price_list --borders` (now `--describe-routes`).

### T4: what holds it

| test | what it holds |
|---|---|
| `ein-infer/tests/tree_classes.rs`' `every_field_both_routes_compute_is_one_on_the_corpus` | on every corpus program and register witness, monotone with an alive root, both routes forced: every field both compute is the same (`Described::differences`; where one computed `N` or the frontier and the other did not, the members met are members), `N` asked and not; where the state route stops and the tree does not, what it met of `R` is the tree's. Per commit without the `slow` entries; with them under `EIN_CORPUS_SLOW=1` on the nightly and `--dev-host` (`nightly.yml`, `run_tests.sh`): 150 s on four P-core threads at `EIN_TEST_JOBS=4` (`utils/bench_env.sh --cores PT:4`, load average 20), one step, no shard |
| `…both_routes_are_the_oracle_where_it_reaches` | each route forced against the cube on every such program within `--region-limit` |
| `…the_default_takes_the_tree_where_it_branches` | the route `explore::run` takes, and `describe_alt`, both ways |
| `…a_stopped_run_says_what_it_established_and_no_more` | the tree and the classes at budgets 1 to 2^11 on six programs: every state listed in `R`, `k_c` no more than the whole, no minimal line, no shape, `N` not as if computed; past `R`'s states, every state, the maximal line and each class found the whole run's; past the singles, the candidates dead alone. It stops in `R`'s states and in the classes among them |
| `ein-infer/tests/explore.rs`' `every_mode_is_the_oracle` | describe and `--count` by the default and by `--describe-route states`, every cell against the cube; `N` and the frontier where computed, their members met where not |
| `…a_description_asked_without_n_is_the_description_less_n` | as before, the tree and the classes computing no `N` asked or not |
| `ein-infer/tests/claim_routes.rs`' `a_region_claim_the_tree_leaves_is_decided_by_the_state_route` | every program's region claim within the oracle's limit: the default's word where it decides is the oracle's; where it does not, `R` is empty, the frontier not computed, and the state route's word is the oracle's (three programs need it) |
| `ein-cli/tests/lever_invariance.rs`' `the_describe_route_moves_no_part_of_r` | through the binary, describe and `--count` under both values on every program not `slow`: every key of `region` that is `R`'s the same where both exhausted, `nogoods` and `frontier` where both wrote one, every claim's outcome but one over a frontier the tree did not compute |
| `ein-infer/tests/unheld.rs` | describe and `--count` with L9 and without: one `R`, the route moving with L9 (it decides whether the tree branches on two programs) |
| `ein-cli/tests/answer_round_trip.rs`, `route_invariance.rs`, `read_out_reference.rs`, `read_out_examples.rs`, `config_reference.rs`, `help_surface.rs`, `wrong_answers.rs`, `register_region.rs` | as they were, on the default; a register row that records `N` asks `--describe-route states` |

**Agreement on the corpus** (`price_list --describe-routes`, below): every field both routes compute is the same on
the 163 programs both finish, and each route's description is the cube's on the 142 within `--region-limit`.

### What the default costs, against the state route

```sh
cargo build --manifest-path ein.rs/Cargo.toml --release -p ein-infer --example price_list
mkdir -p build/s1f.10.17/t3
EIN_TEST_JOBS=8 utils/bench_env.sh --cores P:8 ein.rs/target/release/examples/price_list --describe-routes \
    --out build/s1f.10.17/t3 > build/s1f.10.17/t3/routes.out 2> build/s1f.10.17/t3.env
```

2026-11-16 15:38, `40ecb75d`, eight P-cores, on AC, load average 9 (other work on the machine): its files under the
untracked `build/s1f.10.17/t3/describe/`. **Taken twice**, the counts one digest with the walls left out (`jq -c -S
'del(.describe.default.wall, .describe.states.wall, .count.default.wall, .count.states.wall)' routes.jsonl | md5sum`:
`f77675e19484ea9d75b83e78f3a878a5`, this run and one on the tree before the last fixes to the read-out).

Over the 173 programs that search and that the static check calls monotone with an alive root, saturations · facts
beyond root; `--count`'s columns are describe's to the saturation, neither route spending a node on `N`:

| | the state route | the default | programs on the tree and the classes | lower | higher by more than a tenth |
|---|---:|---:|---:|---:|---:|
| describe | 1 234 652 · 10 286 374 | **605 068 · 8 828 073** | 37 | 19 | 6 |
| `--count` | 1 234 652 · 10 286 374 | **605 068 · 8 828 073** | 37 | 19 | 6 |

**The six higher** (both questions): `zebra2-minus-15.ein` and its `-obligations` twin, 7 261 · 545 818 stopped →
11 192 · 1 048 538 stopped in the classes (R's 32 states exact, 18 classes); `lattice/07_two_pairs_two_tables.ein`
177 · 448 → 149 · 556 (facts); `binary_logic/not_equivalent.ein` 30 · 197 → 27 · 248 (facts);
`binary_logic/truth_table.ein` 30 · 179 → 39 · 450; `tests/stdlib/bool/inputs/01_in_drives.ein` 11 · 38 → 13 · 62.
Every other program of the 37 is lower or the same; the other 136 take the state route as before.

| program | the state route | the default |
|---|---|---|
| `beyond_arith/send_more_money.ein`, `…_written.ein` | 336 101 · 1 422 943, **stopped**, no state of R met | **18 127 · 222 055, one state**, `k_c` = 1, both lines; `N` not computed |
| `zebra2-minus-15.ein`, `…-obligations.ein` | 7 261 · 545 818, **stopped**, ≥ 32 states | 11 192 · 1 048 538, **stopped in the classes**: 32 states exact, 18 classes, `k_c` ≥ 138 621 243 |
| `zebra2.ein`, `zebra2-obligations.ein` | 263 · 22 503 | 120 · 8 786 |
| `zebra.ein` | 314 · 55 346 | 120 · 17 428 |
| `beyond_arith/cryptarithm.ein` | 417 · 1 301 | 141 · 747 |
| `beyond_arith/equation_two_unknowns.ein` | 1 466 · 60 223 | 1 191 · 64 546 |
| `lattice/06_friends_tables_everyone_sits.ein` | 345 · 969 | 207 · 881 |

`N` is not computed on 26 of the 37, and the owing frontier on three (`equation_no_solution.ein`,
`binary_logic/equivalent.ein`, `binary_logic/unsatisfiable.ein`, whose region claims `ein test` then decides by the
state route, and `ein solve` leaves not checked with the option named).

### The forecasts' accuracy

Each step a route takes is projected before it is taken, its nodes times the facts per node so far, and the step's
facts are read off the run's watch (`shared/describe_routes.rs`' `forecasts`); over every step of every cell of each
route, both questions:

| route | steps | facts projected | facts spent | spent ÷ projected, per step: p10 · median · p90 · max | steps spending more than twice / less than half |
|---|---:|---:|---:|---|---:|
| the state route, a layer a step | 2 428 | 22 668 218 | 32 935 562 | 0.0 · 1.5 · 3.0 · 166 | 590 / 638 |
| the tree and the classes, 1, 2, 4, … nodes a step | 76 080 | 4 896 762 | 5 290 932 | 1.03 · 1.07 · 1.45 · 88 | 470 / 650 |

So the tree's forecast is good step by step (its steps are small and its nodes alike), and the state route's is
rough: a layer's nodes grow with its states' size, so it spends half again what it projects at the median and up to
three times at the ninetieth percentile, and a layer of dead extensions known with no fact spends none. The note
quotes the projection as the run makes it and calls it that; `--region-budget N` with `N` the spent and projected
facts takes the state route's next layer only where the forecast holds, which the record above says it often does
not. Neither forecast reaches past the step: no route knows the cost of the whole run before it.

### Defaults taken (T3 to T5)

1. **The option's name, `--describe-route tree|states`**, the main session's example; the user's to change.
2. **Where the tree has nothing to branch on, the default takes the state route** and names it: `∅` in `R`, no
   licensed instance, no candidate. The tree's walk is the state route's there (T2: equal work on every such program
   both finish), so the `route` row, `N` and every golden of those 136 programs stay as they were. `branches` reads the
   obligation pass's table a second time, off root's fixpoint, before any node.
3. **`N` is read off the states where the run met every alive state** (every state met extended by every candidate),
   as the state route reads it, and is empty and known where ⊤ is in `R`; the dualisation over `H` is removed, as
   ruled. The frontier likewise.
4. **`note:`, not `warning:`**, for the forecast: the read-out's convention for a partial run, by default and not only
   under `-v`; the modes' stopped runs are as they were.
5. **The state route's note proposes `--region-budget` at the spent and projected facts**, which takes its next layer
   where the forecast holds; the tree's proposes *a larger* budget, its next step being a node or a few.
6. **`ein solve` does not fall back** to the state route for a region claim over an uncomputed frontier: it says not
   checked, with the option; `ein test` falls back, as ruled.
7. **A register row that records `N`** runs its describe with `--describe-route states` (`wrong_answers.rs`); the
   register's `[case.region]` route vocabulary gains `tree and classes`, four rows re-banked to it.
8. **A query over all of `R` or the maximal line is exact past `R`'s states** on a stopped tree run, which lets a
   claim be decided where `R`'s states are whole and `k_c` is not.
9. `cost_ms` and the slow set are **not re-taken**: the machine was loaded (load average 9 to 20) throughout. The
   slow set's per-commit check passes; `utils/corpus_cost.py` on an idle machine would show SEND's cells cheaper.

### What moved

Named in `94bb05cc`'s message, with the filter: the shape golden's lines of the 37 programs that take the tree and
the classes (`cores[describe]`, `region-lattice`, `region-trace`, `read-out[describe]`), `read-out[describe]` of 11
whose describe the sweep's budget stops (the note), `states` of 4 (the sweep's budget stops them otherwise);
`read_out_examples.md`; `help_shape.txt`; four register rows' route word; three test literals. Not moved: any
answer, any mode's line, `corpus_exits.txt`, the traces, `README.md` and the guide (their transcripts are
`--single`'s).

**Tests that read `N` off a describe** (`8d1d9001`): `pair_layer.rs`, `coupling.rs` and `corpus_cores.rs` ask
`--describe-route states`, so they read the programs they read before rather than skipping the 26 whose `N` the
default does not compute; `known_dead.rs` holds a describe's work with and without the denials where its order is
fixed, the state route, and on the tree and the classes holds `R`'s states found whole both ways to be one set (its
floor taken over the cells both runs finish).

### Verification

On the final tree (`8d1d9001` and this record), the worktree, the gate profile:

- `cargo test --profile gate -p ein-infer -p ein-cli -p ein-render -p ein-corpus`, whole: 135 binaries, 1 082
  tests, green (on `20f5202a`; the four test files changed after it were run again whole, below).
- `EIN_CORPUS_SLOW=1`, each green: `explore` (102 s), `order` (176 s), `corpus_cores`, `known_dead` (133 s),
  `pair_layer`, `coupling`, `-p ein-render --test id_order_invariance` (68 s), `-p ein-cli --test
  answer_round_trip` (90 s), `tree_classes` (60 s; 150 s on four P-core threads under load, above).
- `cargo fmt --all --check`, `cargo clippy --workspace --all-targets -- -D warnings`, `RUSTDOCFLAGS="-D warnings"
  cargo doc --workspace --no-deps`, `utils/check_hashmap_iteration.py`, `utils/doc_audit.py --links --check` over the
  listed pages (582, 0 findings): clean.
- **`EIN_CORPUS_SLOW=1 -p ein-cli --test corpus_cli` is red, and is the nightly's**: its 4× band on `cost_ms` finds
  `zebra2.ein` and `zebra2-obligations.ein` swept in 3.6 s and 3.3 s against 19 083 and 19 067 ms recorded. Their
  six describes (among them `solve --trace`, which took the cores of `N`'s 366 members) are the tree and the classes
  now and take no `N`; both stay slow (over a second). `cost_ms` is re-taken on an idle machine only, and this one was not (another run at 380 % of
  a core, load average 8 to 20), so it is left: **the batch's nightly needs `utils/bench_env.sh --cores PT python3
  utils/corpus_cost.py` re-taken for those entries first** (and `send_more_money.ein`'s, whose describe moved too and
  which that sweep found inside the band).

### What is not done

- The machine was not idle: no wall here is a measurement, and `cost_ms` and the slow set were not re-taken, which
  leaves the nightly's `corpus_cli` red until they are (above).
- `./run_tests.sh`, the nightly and `utils/ci_container.sh`: the main session's, on the batch.

**Findings outside the stage's scope**: none; K-30 stays the last.
