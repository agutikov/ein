# S1f.10.3 — At most one, spent: the extension step

**Phase:** [P1f.10](README.md), track R (the search)
**Status:** ran 2026-11-16 ([§ Record](#record-2026-11-16)). **Built: what a state denies, for every route and
question**, through one memory of what is known dead and a licence of its own, L8; R is the oracle's in every
mode. **Not built: the pair of a death**, which the re-pricing on top of it still calls *go* for describe,
`--count` and `--minimal`, and the layer by need, *go* for `--minimal`. Re-planned 2026-11-15: built only where
[S1f.10.13](s1f.10.13_the_price_list.md) says *go*.
**Depends on:** [S1f.10.13](s1f.10.13_the_price_list.md) (which questions, which source);
[S1f.10.2](s1f.10.2_groups.md) (the owner of the pairs).
**Was:** *the join, restricted*. The join was the walk's (`generate_layer`, `apriori`), deleted on 2026-11-03.
The file keeps its name so that links to it hold.

## Context

No route joins candidate sets any more. Every route **extends**: it takes a state `S` and a candidate `b` and
asks for the node `S ∪ {b}`. Before it saturates, it asks whether a set already known dead lies inside
(`tree.rs`'s `pruned`, the state route's L1 step). What is known dead is what the run has itself seen die, and
each death is remembered as the **whole commitment** (`record_dead`). So the death of a 24-candidate set
teaches that 24-candidate set, and the pair inside it that was dead at root is paid for again above every
other state: 2 234 of the 2 346 saturations of `zebra2-minus-15 --maximal`, by the re-plan's probe.

The stage gives that memory the excluded pairs:

> **a set that holds an excluded pair is dead, and is not saturated.**

**It needs no new licence.** An excluded pair is a dead node of the program's own lattice, computed by the node
function every route uses, and L1 says its supersets are dead. The stage adds no theorem; it fills L1's memory
earlier, or from less. So it holds exactly where L1 holds, on a program the static check calls monotone, and a
flagged program gets nothing.

## What a node not saturated owes the surfaces

Listed before anything is built, each with what the stage does about it. This is the part that is not a
one-liner.

| surface | what changes | what the stage does |
|---|---|---|
| the `work` row | fewer saturations; and the pairs' own saturations are work | both counted in the one row; the `route` row names the licence in words, as it names L1 to L5 |
| `N`, `dead alone`, `k_s`, `k_c`, the lines, the shape | nothing: they are read off the states | held by the acceptance |
| `--dump-lattice`, `ein render lattice` | a dead node not saturated is not *visited*; the pair nodes are | the dump says by which pair a set was known dead, where it lists what L1 pruned today |
| a dead node's core, where a surface prints one | a node not saturated has no derivation | inventory which surfaces print a core of a node the routes would now skip; where one does, the pair's own node is the explanation, and is what is printed |
| `--events` | fewer `saturate` narrations | [`events.md`](../../../docs/kernel/inference/events.md) restated in the same commit if a line's meaning moves |
| the goldens | `read_out_examples.md`, the shape digests, corpus entries' `cost_ms` and the slow set | each named in this file before it moves |

## Tasks

1. **T1f.10.3.1 — the inventory**: the table above checked against the code, every reader of a dead node
   named, and the goldens that will move listed here. A golden that moves and is not on the list is a stop.
2. **T1f.10.3.2 — one place.** Every route asks one owner whether a set is known dead, and the excluded pairs
   are in it. The owner is S1f.10.2's; the routes do not each keep a copy. If the source S1f.10.13 chose is
   *the pair of a death*, the same owner is what a death writes its pair to, instead of the whole commitment
   or beside it.
3. **T1f.10.3.3 — with and without.** An internal switch for a test, on the calling thread, in the way
   `forbid::studying` lifts a refusal for a census: **not** a lever, not in `(config …)`, not on the command
   line, not an `EIN_*` name. The test: every corpus program that searches, every question, with the pairs and
   without, one `Described` and one verdict; the sweep through `ein_corpus::pool::map`.
4. **T1f.10.3.4 — the work delta**, per program and per question, recorded here with the command and the date;
   the programs that got slower named, and held to S1f.10.13's rule: a question that loses by more than a
   tenth on some program does not get the source.
5. **T1f.10.3.5 — the fixture**: a small bijection under `tests/` or `examples/`, with an `:expect`, on which a
   test pins how many extensions were known dead by a pair. It is what fails if a later change drops the step.
6. **T1f.10.3.6 — the pages**: [`read_out.md`](../../../docs/kernel/inference/read_out.md)'s `route` and
   `work` rows, and the routes' module headers, stating what the engine does, with no stage id and no date.

## Acceptance

- **R is identical on every corpus program in every mode.** `ein-infer/tests/explore.rs` and `region.rs`
  against the oracle, `lever_invariance`, `route_invariance`, `claim_routes` and `answer_round_trip` green with
  no expectation edited; T3's test green.
- **N and `dead alone` are unchanged** on every program a describe exhausts.
- The work delta is recorded per program and per question, with the losers named; **I-L02's is zero**.
- The pairs' own cost is in the `work` row.
- Every golden that moved is on T1's list.
- No lever of any kind.
- `./run_tests.sh` green on the batch's last commit, and `./run_tests.sh --nightly` once, since a route
  changed.

## What it must not do

- **Not spend a pair on a flagged program.**
- **Not spend a pair of the theory that is not a node of this program** (S1f.10.10's small KB). What is spent
  is S1f.10.1's.
- **Not add a route, and not change the order of one.** The order is S1f.10.5's.
- **Not skip a node whose content a surface prints without saying where the content now comes from.**

## Notes

**Why this is not simply a saturation rule.** `domain-elimination` and `no-room-left` write the same
knowledge as stored negatives, per node, by firing rules; they are already in every number S1f.10.13 took. What
is spent here is what remains after them: a node the route would have built only to watch it die.

**The saving compounds with depth**: a set is known dead if *any* pair in it excludes, so the share removed
grows with the size of the states a route extends, which is where the `--maximal` run's 24-candidate deaths
are.

## Record (2026-11-16)

### T1: the inventory, written before any route changed

**The scope, from [S1f.10.13](s1f.10.13_the_price_list.md#t3-the-verdicts).** First *what a state denies*, for
every route and question, at no saturation. Then the pair of a death and the layer by need, re-priced on top of
it and built only where still *go*. The death's core is re-priced and not built. Committed at `a3c719bf` plus
this section, before `ein.rs/` is touched.

**Where a death is remembered today.** Two copies of one memory, and one cache beside them:

| where | what it holds | who asks it |
|---|---|---|
| `tree.rs`, `Search::dead_under` | per candidate `b`, the states `S` whose `S ∪ {b}` a saturation found dead | `extend` (an extension), `pruned` (any set: the dualisation's transversals and `eval_set`), `branch_rest` (how many of an instance's alternatives are not known dead, which chooses the instance), `climb` (a top's every extension must be known dead, or the run refuses) |
| `states.rs`, `dead_under` | the same, a local of `compute_resumable` | the layer loop, before an extension is saturated |
| `explore.rs`, `Space::seen` | every node computed, by commitment | `eval`: a set asked twice is saturated once. It is no licence: it answers for the set itself |

Route 1 (`downset.rs`) keeps its own frontier and is reached by `ein solve` only where root is dead, one
saturation; the cube (`region.rs`, `Space::cube_in_order`) skips nothing. `structure.rs` reads pairs through
`Space::eval` and decides them by its own L1 and L3; no route reads it.

**Every reader of a dead node**, checked against the code:

| reader | what it reads of a dead node | what a dead node that is known and not saturated owes it |
|---|---|---|
| `states.rs`, `StateRoute::dead` | every dead extension saturated: its set, its debts, its core where the walk takes cores | a record of the set, with no core and no debts |
| `states::describe_with`, `nogoods_met` | on a run its budget stopped, or under `--count`: the members of `N` met, each a dead set of `StateRoute::dead` whose one-down subsets are all known alive | the record above, or `N … met` loses members on a partial run |
| `described::of_states`, `dead_alone_set` | root's extensions among `StateRoute::dead` | the record above, were root ever to deny a candidate (it does not: a candidate root denies is not in `H`) |
| `described::of_states`, `dead` | `N`'s members, each with the core of the record of the same set where the walk took one | nothing new: a member the walk did not saturate has no core today, and `explore::describe_states` takes it after the walk where a surface prints `N`'s cores (`--dump-answer`, `--trace`). More members take that path |
| `described::of_states`, `owing_met`, `dead_met` | whether any dead node owes; whether any node is dead | a dead node's debts are its alive subsets' on a monotone program, so `owing_met` holds from the states; `dead_met` reads the count of what was known dead |
| `tree.rs`, `climb`, `shrink`, `next_top` | that a top's extensions are known dead; that no subset of a commitment of `R` is dead | the same memory, so the same answer |
| `events::Watch`, the `enter` line (`--events`, `--dump-lattice`, `-v`) | each node **visited**, its core where cores are taken | nothing: a node not saturated is not visited, and is in neither. `region_modes.rs` holds a dump's `visited` to the `work` row's saturations |
| `ein render region … --visited` (`ein-render/src/region_doc.rs`, `Unvisited`) | **infers** why a node of the cube was not visited: *above a dead node* the run visited (L1), else *the run stopped first* or *not needed* | a second inference: the cube's own `excluded` mask of a visited alive node says the node above it was denied. Without it such a node reads *not needed* |
| the read-out's `N`, `universe … dead alone`, `k_s`, `k_c`, the lines, the shape | read off the states | nothing, held by the acceptance |
| the read-out's `route` and `work` rows, the summary's `region.licences` and `work` | the licences used; saturations and facts | the licence named when a run rested on it; fewer saturations |
| `detector.rs`, `Space::partial_cube`, `explore::of_cube` | a flagged program's nodes | nothing: a flagged program gets no licence |

**The programs that can move.** A run changes only where an alive node it visits denies a candidate, so the
price list's count of such nodes names them
(`jq 'select([.cells[] | .ways.denials.counts["alive nodes that deny a candidate"]] | add > 0) | .path'
/home/user/work/ein/build/structure/prices/prices.jsonl`): **ten of the 188 programs that search**:
`examples/zebra.ein`, `zebra2.ein`, `zebra2-hints.ein`, `zebra2-obligations.ein`, `zebra2-minus-15.ein`,
`zebra2-minus-15-obligations.ein`, `branching/12_typed_blind_solve.ein` and
`saturation/type-exclusivity/{colors,nationalities,pets}.ein`. A program outside them has the same nodes, rows
and bytes with the step and without.

**The goldens that will move**, each with what re-banks it. One not on this list that moves is a stop.

| golden | which part | re-banked by |
|---|---|---|
| [`docs/kernel/inference/read_out_examples.md`](../../../docs/kernel/inference/read_out_examples.md) | X10 (`zebra2.ein`) and X13 (`type-exclusivity/colors.ein`): the `route` and `work` rows, the summary's `licences` and `work`, a partial run's rows where the budget now reaches further | `python3 utils/read_out_examples.py` |
| `ein-render/tests/golden/corpus_shapes.md5` | the ten programs' lines `read-out[…]`, `region-trace`, `states`, `region-lattice` and `cores[…]`; no other program's line and none of their other ops | `EIN_BLESS=1 EIN_CORPUS_SLOW=1 cargo test … -p ein-render --test id_order_invariance` |
| `ein-render/tests/golden/trace_one-hypothesis.md`, `trace_ambiguous.md` | `branching/12_typed_blind_solve.ein` under `--maximal` and described: the `route` and `work` rows | `EIN_BLESS=1 cargo test … -p ein-render --test golden_trace` |
| `README.md` and `docs/guide/04_solving_the_whole_puzzle.md`, the marked transcript | `ein solve examples/zebra2.ein --single`: the `route` and `work` rows | `EIN_BLESS=1 cargo test … -p ein-cli --test guide_transcripts` |
| `docs/api/rust.md`'s output block and `ein-cli/tests/embedding.rs`'s expected text | `saturations: 45` of the same run | the test's text edited, the page's block pasted from it |
| `ein-cli/tests/region_modes.rs`, `the_count_question_is_describes_counts` | `type-exclusivity/pets.ein --count`: the block's `route` and `work` rows (`11 351 saturations`) | the expected block edited |
| `corpus/corpus.toml`, `cost_ms`; `ein-corpus`'s `the_slow_set_is_exactly_these_fifteen` | only if one of the ten crosses 1 s: `zebra.ein` is the nearest, at 2 074 ms | `utils/bench_env.sh --cores PT python3 utils/corpus_cost.py` |

**Not goldens, and stale after the step** (prose with a date or a command, which nothing checks): `README.md`'s
*374 saturations, 0.54 s* (2026-11-03); `ein-infer/tests/obligation_rung.rs`'s doc comment, *45 saturations* and
*2 346 a side*; `corpus.toml`'s notes. `examples/beyond_arith/send_more_money.ein`'s *17 747 saturations* stands
unless the pair of a death is built for `--single`.

**Tests that state a count and do not move**: `explore.rs`'s `max.saturations <= 21` and `region.rs`'s
`the_state_route_never_does_more_than_route_1` are upper bounds; `arith_search_free.rs`, `no_search.rs` and
`region_modes.rs`' *1 saturation* are root alone; `acceptance_cli.rs`' control, `zebra2.ein --region-budget
4096` is not exhausted, stays so while a describe of it costs more than 4 096 facts.

### What was built

**Ran to its end for *what a state denies*; the pair of a death and the layer by need are re-priced and not
built** ([§ What is not done](#what-is-not-done-and-the-next-way)). R is the oracle's in every mode on every
corpus program, with no expectation edited.

- **One memory of what is known dead**, [`ein_infer::known_dead`](../../../ein.rs/crates/ein-infer/src/known_dead.rs).
  Per candidate `b`, the states `S` with `S ∪ {b}` known dead, from a **death** (a saturation found it dead) or a
  **denial** (`S` is an alive state whose fixpoint stores `(not b)`). `tree.rs`'s `Search::dead_under` and
  `states.rs`'s local of the same name are gone: both routes ask it and write to it.
- **The node function reports what a node denies**: `region::NodeOut::denied`, the candidates whose `(not b)`
  the fixpoint stores, at any `n` (`Node::excluded` was the same as a mask under 64). `explore::Seen` keeps it
  for an alive node.
- **The state route** (describe, `--count`): an extension its own state denies is not saturated and is kept as a
  dead node met (`states::Dead::denied`), each set once.
- **The modes' routes** (`--single`, `-n`, `--minimal`, `--maximal`): every state met writes its denials when it
  is met, and an extension, a transversal of the dualisation and a subset a shrink tries all ask the memory.
- **Through a closure.** The node of `S ∪ {b}` holds every state `cl(P ∪ {b})` already met for a state `P` one
  edge under `S`, so it is dead when one of those and `S`, or two of those, meet in a candidate one holds and the
  other denies (`states.rs`' and `tree.rs`' `denied_through`). This is the half of the price list's `denials`
  way that the set alone does not show: without it `zebra2.ein`'s describe is 318 saturations, with it 263,
  which is the simulated number to the saturation and the fact.
- **`Licence::Denied`, `L8 what a state denies is dead above it`**, named in the `route` row and in
  `region.licences` exactly where an answer rested on a denial; `Answer::denials` counts the sets so taken for
  dead. No surface prints the count.
- **The drawing of a run on its cube** (`ein render region … --visited`) names the reason a node was not
  saturated: *above a state's denial*, with the state and the fact, read off the cube's own `excluded`, and marks
  `≠ the cube` a node so explained that the cube says is alive.
- **The switch**, `known_dead::from_deaths_alone(f)`: every memory opened on the calling thread while `f` runs
  learns from deaths alone. A thread-local as `forbid::studying` is; no `(config …)` field, no option and no
  `EIN_*` name (`config_reference.rs` gained no row).
- **The fixture**, [`tests/region/10_denied_extensions.ein`](../../../tests/region/10_denied_extensions.ein),
  three guests and three chairs whose two rules store what a seating excludes; and
  [`ein-infer/tests/known_dead.rs`](../../../ein.rs/crates/ein-infer/tests/known_dead.rs), four tests.
- **`price_list --deaths-alone`**, so that the price list can be taken both ways on one binary.

### The licence

**It needed its own name.** The stage file says *it needs no new licence*, of an excluded pair: a dead node the
node function computed, whose supersets L1 kills. A denial is no computed node. `S` is alive and `{b}` may be,
so L1 has no dead set to start from. What decides `S ∪ {b}` is an argument, in three facts
([`known_dead.rs`](../../../ein.rs/crates/ein-infer/src/known_dead.rs), § L8):

1. *a node's fixpoint holds its commitment*: the node function writes the commitment into root's fork before it
   saturates, so `b` is in the fixpoint of `S ∪ {b}`;
2. *the state is monotone*: on a program with no site each node's fixpoint contains its subsets'
   (`monotone.rs`, § The theorem), and a stored negation is a fact of the fixpoint like any other, so `(not b)`,
   which `S`'s fixpoint holds, is in the fixpoint of `S ∪ {b}`;
3. *a fact beside its stored negation is a contradiction*: `contradiction::detect`'s first shape, a pair, for
   every relation, closed or not, however either fact got there (*how a fact got into the KB is irrelevant*).

So that node is dead, and L1 closes it upward. **L8 is a second corollary of the theorem L1 and L3 rest on,
the monotone state, and holds exactly where they hold.** A flagged program gets nothing: its route is the cube,
which asks the memory nothing. That it must get nothing is shown and not only argued: a program whose rule
concludes `(not (p B))` from `(p A)` and `(absent (p B))` has an alive node above another's denial
(`a_flagged_program_spends_no_denial`).

**Checked against how the engine defines dead, and against the oracle.** On the 561 monotone cubes the oracle
enumerates over the corpus and the register, 8 486 pairs of an alive node and a candidate it denies: the node
above is dead in every one, and no alive node holds a candidate it denies
(`what_an_alive_node_denies_is_dead_above_it_on_every_monotone_cube`). No class of monotone program fails it.

**The premise is enforced on every run**, by something that fails: a state met alive that holds a set known
dead refuses the run, in both routes (`inside_of_alive`: until this stage nothing asked that of L1 either); so
does a denied extension that is a state already met alive, and an alive fixpoint that holds a candidate it
denies.

**What it does not read.** `std.algebra`'s `functional` and `injective` conclude `(false)` from the two facts
and store nothing, so `send_more_money.ein` denies nothing and its work is what it was, to the fact. That is
K-13's second half and stays open; `stdlib/` is untouched.

### Where the owner lives

In a module of its own under the routes, `known_dead.rs`, and not in `structure.rs`, where the stage file put
it. `structure` reads its nodes through `explore::Space`, and `explore` calls `tree` and `states`; a memory
those two ask cannot live in a module that sits above them without the three importing each other in a circle.
So the order is `known_dead` ← `states`, `tree` ← `explore` ← `structure`. The routes write what they learn,
deaths and denials; a stage that spends a pair the structure found writes it the same way, `died(S, b)` with
`S = {a}`, from wherever it opens the route, and the routes still ask one place.

### With and without

`ein-infer/tests/known_dead.rs`, `every_question_answers_alike_with_the_denials_and_from_deaths_alone`: every
corpus file and register witness that searches, six questions each, asked twice at the default budget, the
second time inside `from_deaths_alone`; through `ein_corpus::pool::map`. Per commit every program but the
`slow` entries, and the two zebra puzzles; under `EIN_CORPUS_SLOW=1` the rest, a step of the nightly's
`deep-corpus` job and of the dev host's tier (`nightly.yml`, `run_tests.sh`; `nightly_steps.rs` holds the
three lists to each other).

**What *one answer* means was written in the test's header before the cells were read**:

- both finished, describe and `--count`: the two descriptions are equal field for field, `N`'s cores among
  them, but the work;
- both finished, any other question: one verdict and one `exhausted`; a part of R both list whole is one set
  of states and of minimal commitments; a listing a question stopped as asked (`-n 1`, a `--single` that found
  two) is as many states from each, and every state either lists is held to R as the describe lists it;
- either stopped by a budget: a describe's order does not depend on what is known dead, so the run with the
  denials has met every state, every certified minimal commitment and every member of `N` that the run from
  deaths alone met, counts no fewer commitments and is complete at least as far; a verdict both reach is one
  verdict; and **a run with the denials is never stopped where the run from deaths alone finished**;
- the work: a describe saturates no more nodes and derives no more facts with the denials, on any program.

**What it found**, `cargo test … -p ein-infer --test known_dead -- --nocapture`, 2026-11-16: per commit 176
programs that search and 1 056 cells, none answering otherwise; under `EIN_CORPUS_SLOW=1` 189 programs and 1 134 cells: 51 rest on a denial, 45 saturate fewer nodes and four more,
3 447 472 saturations against 3 453 542 from deaths alone, and 42 cells are stopped both ways. **No cell
is stopped with the denials and finished without, and none finished with them that a budget stopped without:**
the cells a budget or a limit stops are the same either way, and the interesting case the plan names does not
occur in the corpus. **Four cells list other states**, as the definition allows: `zebra2-minus-15.ein` and its
`-obligations` twin under `--single` and `-n 1`, where the tree branched on another instance and met another of
R's 32 states first. Each
state so listed is on the maximal line, which both runs list whole; no cell the denials touched lists a state
that neither a finished describe nor that line holds, and the test fails on one that does. The price list taken from deaths alone on this binary is the price list banked
before the stage, **one digest over every count of its 1 128 cells** (below), so the switch is the routes as
they were.

### T4: the work, by a real run, against the simulation

```sh
cargo build --manifest-path ein.rs/Cargo.toml --release -p ein-infer --example price_list
utils/bench_env.sh --require-ac --core 6  ein.rs/target/release/examples/price_list --no-nodes \
    > build/structure/prices/verdicts.md 2> build/structure/prices/prices.env
utils/bench_env.sh --require-ac --core 10 ein.rs/target/release/examples/price_list --no-nodes --deaths-alone \
    --out build/alone > build/alone/prices/verdicts.md 2> build/alone/prices/prices.env
jq -c -S -f build/digest.jq build/alone/prices/prices.jsonl | md5sum     # 9c307cdb07076cd9146052ac0dbf36a9
jq -c -S -f build/digest.jq /home/user/work/ein/build/structure/prices/prices.jsonl | md5sum   # the same
```

2026-11-16, the worktree at this stage's commit, the release example, one P-core a run, on AC. `digest.jq` is
S1f.10.13's filter (the walls and the `denials` way out) with the fixture's line left out; `9c307cdb…` is the
digest that record gives for its own two runs. **Saturations and facts are the numbers. No wall is stated**: two
other stages were building in their worktrees throughout and the load average was above 20.

**Per question, over the 189 programs** (the 188 and the fixture), from deaths alone and with the denials, real
runs both (`jq -r -n --slurpfile with build/structure/prices/prices.jsonl --slurpfile old
build/alone/prices/prices.jsonl -f build/totals.jq`):

| question | saturations | facts beyond root | over the monotone cells no budget stopped: saturations | … facts | cells lower | higher | higher by more than a tenth |
|---|---|---|---|---|---:|---:|---:|
| describe, `--count` | 1 350 727 → 1 348 056 | 11 563 658 → 10 967 141 | 28 592 → 28 181 | 445 180 → 350 383 | 11 | 0 | 0 |
| `--single` | 19 696 → 19 579 | 292 907 → 261 426 | 19 576 → 19 459 | 292 746 → 261 265 | 4 | 2 | 0 |
| `-n 1` | 17 214 → 17 206 | 227 758 → 224 352 | 17 171 → 17 163 | 227 736 → 224 330 | 4 | 0 | 0 |
| `--minimal` | 231 846 → **235 969** | 3 406 320 → 3 312 963 | 4 815 → 4 426 | 263 116 → 170 447 | 4 | 2 | 2, both stopped both ways |
| `--maximal` | 483 332 → 478 606 | 3 521 322 → 2 488 588 | 27 524 → 22 798 | 1 384 469 → 351 735 | 11 | 0 | 0 |

**The cells that are higher, all four**: `zebra2-minus-15.ein` and its `-obligations` twin, under `--single`
39 → 40 saturations with 3 843 → 2 688 facts, a tree that branched on another instance; and under `--minimal`
10 846 → 13 102 saturations with 1 047 055 → 1 046 711 facts, a cell the budget stops both ways. The budget is
in facts, a denied node is one of the dear ones, and so the same budget buys 2 256 more nodes. By the rule's
letter that is a cell higher by a fifth in saturations; it is read here as a cell that spent its budget and got
further, and `--minimal` keeps the step ([§ Defaults taken](#defaults-taken), 2). Over the cells no budget
stopped no program is higher under any question but the `--single` pair, by one node.

**The six instances, and the simulation beside** (`jq -r -n --slurpfile with …/prices.jsonl --slurpfile old
<the banked prices.jsonl> -f build/t4.jq`). A cell: saturations · facts.

| | question | from deaths alone | with the denials, real | as S1f.10.13 simulated |
|---|---|---|---|---|
| **I-Z1** | describe, `--count` | 445 · 106 829 | **314 · 55 346** | 314 · 55 346 |
| | `--single` | 49 · 17 620 | **9 · 1 900** | 9 · 1 900 |
| | `-n 1` | 6 · 721 | 5 · 328 | 5 · 328 |
| | `--minimal` | 451 · 109 187 | 318 · 56 918 | 318 · 56 918 |
| | `--maximal` | 49 · 17 620 | 9 · 1 900 | 9 · 1 900 |
| **I-Z2** | describe, `--count` | 374 · 42 705 | **263 · 22 503** | 263 · 22 503 |
| | `--single` | 45 · 7 582 | **9 · 892** | 7 · 666 |
| | `-n 1` | 5 · 302 | 5 · 164 | 5 · 302 |
| | `--minimal` | 377 · 43 251 | 267 · 23 231 | 266 · 23 049 |
| | `--maximal` | 45 · 7 582 | 9 · 892 | 7 · 666 |
| **I-Z2M** | describe, `--count`, stopped | 8 391 · 796 678 | 7 261 · 545 818 | 7 260 · 545 596 |
| | `--single` | 39 · 3 843 | 40 · 2 688 | 29 · 1 623 |
| | `-n 1` | 36 · 3 466 | 35 · 2 122 | 27 · 1 468 |
| | `--minimal`, stopped | 10 846 · 1 047 055 | 13 102 · 1 046 711 | 9 397 · 725 377, to the frontier the run from deaths alone reached |
| | `--maximal` | 2 346 · 509 140 | **104 · 10 850** | 100 · 10 528 |
| **I-B06** | all six | 133, 8, 1, 1, 258 | the same | the same |
| **I-L02** | all six | 8, 2, 1, 1, 7 | **the same: its delta is zero** | the same |
| **I-SEND** | all six | 336 101, 17 747, 15 854, 205 166, 17 754 | **the same, to the fact** | the same |

The four other programs that move: `branching/12_typed_blind_solve.ein` describe 56 → 52 and `--maximal` 84 →
80 (simulated 77); `saturation/type-exclusivity/colors.ein` and `nationalities.ein` describe 173 → 169,
`--maximal` 54 and 56 → 46 (45); `pets.ein` describe 11 351 → 11 339, `--maximal` 430 → 394 (311);
`zebra2-hints.ein` describe 78 → 62, `--maximal` 123 → 84 (71). `zebra2-obligations.ein` is `zebra2.ein`'s row.

**Where the real run and the simulation differ, and why.**

- **A describe is the simulation**, to the saturation and the fact, on every program whose describe finishes;
  the stopped `zebra2-minus-15` pair is one node and 222 facts off, the layer the budget cuts.
  That holds only with the closure step. The simulation took a dead node for removed when `K`, *the commitment
  with the closures of the alive nodes inside it*, held a denied candidate. The set alone holds fewer: reading
  only what the state under the set denies, `zebra2.ein`'s describe is 318 and `zebra.ein`'s 375 (the first run
  of this stage, kept in `build/first/`). The 55 and 61 between are extensions whose denial sits one derivation
  away, in a state the node holds and the set does not show.
- **A tree is not the simulation**, in both directions, for three reasons. *Its order moved*: it branches on the
  instance with the fewest alternatives not known dead, and an alternative a state denies is now known dead, so
  on `zebra2.ein -n 1` the same five saturations are other nodes, 164 facts where the simulation, its order
  held, has 302; and `zebra2-minus-15.ein --single` is 40 where the order held gives 29. *It closes a set only
  under the states it reached the state from*, where the state route has every state under it: the tree meets
  states out of size order. *The dualisation's sets are not closed at all*: a transversal over a seed is asked
  as a set, which is where `pets.ein --maximal`'s 394 against 311 and `zebra2-hints.ein`'s 84 against 71 are.
- **No cell is dearer than the simulation said it could not be**: the simulation's premise, *no alive node holds
  a candidate a node under it denies*, is now the oracle test's, on 561 cubes.

**Losers, held to the rule.** No program loses by more than a tenth under describe, `--count`, `--single`,
`-n 1` or `--maximal`, in either unit, and each total is lower: *go*, and built. Under `--minimal` the two
stopped `zebra2-minus-15` cells are the whole of what is higher; over the 157 cells that finish it is 4 815 →
4 426 with no cell higher. **Every question has the step.**

### The pair of a death, the layer by need and a death's core, on top of the denials

Priced by the same example on the nodes the routes visit now (`jq -r -f …/matrix.jq
build/structure/prices/verdicts.json`). A cell: the verdict by S1f.10.13's rule in both units, the question's
total with the way, the programs lower / higher by more than a tenth.

| way | describe, `--count` | `--single` | `-n 1` | `--minimal` | `--maximal` |
|---|---|---|---|---|---|
| *the question's total* | *1 348 056* | *19 579* | *17 206* | *235 969* | *478 606* |
| `denials`, what the simulation's reading would still remove | 1 348 054 · 2 / 0 | nothing | nothing | 235 937 · 2 / 0 | 478 505 · 5 / 0 |
| `layer-by-need` | no · 1 347 810 · 9 / 2 | no · 12 993 · 2 / 16 | no · 11 916 · 1 / 15 | **go** · 225 775 · 10 / 0 | no · 473 247 · 23 / 49 |
| `death-pairs` | **go** · 1 347 623 · 9 / 0 | no · 12 233 · 3 / 14 | no · 11 160 · 2 / 9 | **go** · 225 728 · 10 / 0 | no · 471 369 · 6 / 19 |
| `death-core-pair` | go in the work's units · 1 347 612 · 11 / 0 | 9 968 · 5 / 0 | 9 029 · 3 / 0 | 225 499 · 10 / 0 | 468 172 · 40 / 0 |
| `death-core` | go in the work's units · 1 346 697 · 11 / 0 | 9 956 · 6 / 0 | 9 029 · 3 / 0 | 221 065 · 10 / 0 | 468 018 · 46 / 0 |

**The verdicts did not move, and what is behind them did.** Before the denials the pair of a death took
describe from 1 350 675 to 1 348 260; on top of them it takes 1 348 056 to 1 347 623, **433 saturations on nine
programs**: `zebra.ein` 314 → 266 and `zebra2.ein` 263 → 228 (13 and 8 nodes asked, 61 and 43 removed: the
pairs of the *puzzle*, which no state denies), the stopped `zebra2-minus-15` pair 7 261 → 7 155,
`branching/08_hypothesis_relation_whitelist.ein` 335 → 271, `branching/02_one_dead_one_alive.ein` 193 → 163,
`branching/04_two_levels.ein` 71 → 63. On the zebras **the theory's pairs are spent**: the `theory-shared`
bound, the theory's pairs for nothing, removes no node of `zebra.ein` or `zebra2.ein` under any question now.
Under `--minimal` it is 10 241 saturations, 9 516 of them the stopped `send_more_money.ein` cell, 109 on
`lattice/06_friends_tables_everyone_sits.ein`, 36 on `lattice/07_two_pairs_two_tables.ein`. Under the three
small questions it still loses: `zebra2.ein --single` 9 → 16.

**A death's core**, on the six instances, with the denials built: `zebra2.ein` describe 263 → 216, `--single` 9
→ 9; `zebra.ein` describe 314 → 253; `zebra2-minus-15.ein` `--maximal` 104 → 102 (it was 2 346 → 341: the
denials took what the core would have); `send_more_money.ein` `--single` 17 747 → 8 220, `-n 1` 15 854 → 7 760,
`--maximal` 17 754 → 8 222, unchanged, since nothing there is denied. Over the corpus `--single` 19 579 → 9 956
and `--maximal` 478 606 → 468 018. **So what a core is still worth is SEND**, where a core costs 20 µs against
49 µs a saturation (S1f.10.13), and the puzzle's pairs on the zebras, where it costs ten saturations' time.
K-7 stands.

**None of the three is built.** [§ What is not done](#what-is-not-done-and-the-next-way).

### What a node not saturated owes the surfaces, as built

The table at the head of this file, row by row.

| surface | what was done |
|---|---|
| the `work` row | fewer saturations; a denial costs none, so nothing is added. Its page says a set a licence decided is no saturation |
| the `route` row, `region.licences` | `L8 what a state denies is dead above it`, named where an answer rested on a denial, as L5 is named where a branch used it |
| `N`, `dead alone`, `k_s`, `k_c`, the lines, the shape | unchanged on every program, by the with-and-without test and the oracle's. A denied extension is a dead node the state route met, so a partial run's `N … met` and `dead alone` count it as they counted the saturated one |
| a dead node's core | no surface prints the core of a node that is now not saturated but `N`'s: `--dump-answer`'s `nogoods.json` and `--trace`'s *N*. Those take a member's core **after the walk**, through the node function against the walk's snapshot, as they already did for every member the walk read off the states; more members take that path, and the cores are the same (the test compares them) |
| `--dump-lattice`, `--events` | a node not saturated is not visited: it is in neither, and `visited` is still the `work` row's saturations. The dump lists nothing a licence pruned, today or before; [`lattice_dump.md`](../../../docs/kernel/inference/lattice_dump.md) says so |
| `ein render region … --visited` | the one surface that says *why* a node was not visited. It infers *above a dead node* from the dead nodes the run visited; it now infers *above a state's denial* the same way, from the cube's `excluded`, and checks it against the cube |
| `--events`' lines | no line's meaning moved: `enter` is a node visited and `layer`'s `dead` the dead nodes visited. [`events.md`](../../../docs/kernel/inference/events.md) is as it was |

### The goldens that moved

Every one is on T1's list, and each was re-banked by the command the list names:

- [`read_out_examples.md`](../../../docs/kernel/inference/read_out_examples.md), 11 lines: X10's `route` and
  `work` rows under describe (twice), `-n` and `--single`, with the state's `from {…}`, the commitment the tree
  first met it by; X13's two rows.
- `ein-render/tests/golden/corpus_shapes.md5`: 88 lines, all of the ten programs and of no other: for each its
  `read-out[…]`, `region-trace`, `region-lattice` and `cores[…]` lines where the question's work moved, six for
  the four programs whose `--single`, `-n` and `--minimal` are root's, eleven or twelve for the zebras. No
  `states` line moved, which T1 allowed for.
- `trace_one-hypothesis.md` and `trace_ambiguous.md`: the `route` and `work` rows, 84 → 80 and 56 → 52
  saturations.
- `README.md`'s and `docs/guide/04_solving_the_whole_puzzle.md`'s transcript: the two rows, 45 → 9, and the
  `from {…}`.
- `docs/api/rust.md` and `ein-cli/tests/embedding.rs`: `saturations: 45` → `9`.
- `ein-cli/tests/region_modes.rs`, the block on `pets.ein --count`: the `route` row names L8, 11 351 → 11 339
  saturations and 76 743 → 76 683 facts.
- `corpus.toml`'s `cost_ms`: **not touched**. No entry crosses 1 s. The five of the ten that are `slow` stay so:
  `zebra.ein`, the nearest, takes 4.1 s for its `render lattice` run alone on this binary and 6.7 s on the binary
  built before the stage (by hand, the machine at a load average of 22), so its recorded 2 074 ms is a number of a
  quieter machine and is not re-taken on a shared one. The five others only got cheaper.

**Three banked lists gained lines for files this stage adds, which T1 did not list.** None is a rendering that
moved; each names a new file: `ein-cli/tests/golden/corpus_exits.txt`, the fixture's two cells (`test`,
`saturate`, both 0); `corpus_shapes.md5`, the fixture's 41 lines; and the anchor list in `examples/README.md`,
which gains `ein-infer/tests/known_dead.rs`, since the test reads the two zebra puzzles by name on every run.

### Verification

2026-11-16, in the worktree, each green unless said:

```sh
cargo test --manifest-path ein.rs/Cargo.toml --profile gate -p ein-infer
cargo test --manifest-path ein.rs/Cargo.toml --profile gate -p ein-cli --test lever_invariance --test route_invariance \
    --test answer_round_trip --test read_out_reference --test corpus_cli --test config_reference --test nightly_steps \
    --test gate_steps --test wrong_answers --test region_render --test region_modes --test embedding \
    --test guide_transcripts --test world_anchors --test read_out_examples
cargo test --manifest-path ein.rs/Cargo.toml --profile gate -p ein-render --test id_order_invariance --test jobs_invariance \
    --test golden_trace
EIN_CORPUS_SLOW=1 cargo test --manifest-path ein.rs/Cargo.toml --profile gate -p ein-infer --test known_dead --test explore
cargo fmt --manifest-path ein.rs/Cargo.toml --all --check
cargo clippy --manifest-path ein.rs/Cargo.toml --workspace --all-targets -- -D warnings
RUSTDOCFLAGS="-D warnings" cargo doc --manifest-path ein.rs/Cargo.toml --workspace --no-deps
python3 utils/check_hashmap_iteration.py
python3 utils/doc_audit.py --links --check docs plans README.md AGENTS.md corpus/README.md tests/README.md \
    examples/README.md stdlib/README.md utils/README.md c/README.md
```

All green on the tree as committed, with one reading that is the machine's: `corpus_cli` under `EIN_BLESS=1`,
which reads the slow entries too, re-banked its table and failed `the_slow_flag_still_describes_the_sweep` on
`send_more_money.ein`, *recorded cost_ms = 8404, swept in 36630 ms*, with two other stages building and the
load average above 20. That program's work is what it was to the fact, and the same binary per commit is
green. Under `EIN_CORPUS_SLOW=1`: `known_dead`, `explore`, and `region`, whose
`the_state_route_is_route_1_on_zebra2_minus_15_through_layer_6` holds the state route with the denials to
route 1 without them.

The whole workspace was run once unblessed before anything was re-banked, to see every expectation that moved:
the nine failures were the goldens above and the three lists, and nothing else.

### Defaults taken

Each the user's to revise.

1. **The licence is a new one, L8**, and not L1 with a node known dead by construction: L1 has no dead set under
   `S ∪ {b}` to start from. L6 is `solve_interface.md`'s trivial lines and L7 the cube, so the number is the next
   free one.
2. **A cell a budget stops both ways is not a loser for doing more nodes.** Under `--minimal` the two
   `zebra2-minus-15` cells spend the same 2²⁰ facts on a fifth more saturations. S1f.10.13's default counts a
   stopped cell *with what it had spent, to the same frontier*; a real run does not stop at the same frontier, so
   the unit the budget is in is the one compared, and the cells that finish decide the question.
3. **The tree's choice of instance reads the memory as it is**, so a denied alternative is known dead and is not
   counted. The rule of the order is the one `tree.rs` states and is untouched; what it reads grew, the nodes a
   tree visits moved with it, and one cell is a node dearer for it. Holding the order to deaths alone would make
   every tree cell the simulation's or lower, and is the other reading.
4. **Route 1 reads no denial.** No command reaches it past a dead root, where it saturates root alone, and
   `describe_by`'s tests hold the state route to it: it stands as the route that does not have the step.
5. **The count is in the library and on no surface**: `Answer::denials`. The `route` row names the licence; the
   `work` row has no new number, since a denial costs none.
6. **A set is counted once** however many of its states deny it, and a set taken for dead inside a larger one
   counts as the set that was denied.
7. **A denied extension has no core in the state route's record**, and `Dead::owes` is 0 there: a dead node's
   debts are its alive subsets' on a monotone program, so nothing a surface prints reads them.
8. **The fixture is under `tests/region/`**, with a query's claim over R, beside the other region fixtures;
   `tests/README.md`'s table has its row.
9. **The with-and-without test's slow half is a step of the nightly**, by name, beside `pair_layer`'s and
   `coupling`'s. On four P-core threads with `EIN_TEST_JOBS=4` it took 277 s (`utils/bench_env.sh
   --cores PT:4 --require-ac`, 2026-11-16, the machine shared): inside the five minutes and close to them, so it
   is the first candidate for shards if the runner is slower.
10. **`README.md`'s dated sentence** on `zebra2.ein`'s describe was restated with today's count and the old one
    kept beside; it is prose and nothing checks it.
11. **The three lists that gained lines for new files were re-banked and are named**, and were not read as the
    stop T1 sets for a golden that moves unlisted.
12. **The closure step is part of *what a state denies*** and not a way of its own: S1f.10.13 priced `denials`
    on the set closed under the closures the run holds, and a describe is that simulation only with it. It
    spends L3 on the set asked about, a licence both routes already hold.

### What is not done, and the next way

- **The pair of a death is *go* for describe, `--count` and `--minimal` on top of the denials, and is not
  built**; nor is the layer by need, *go* for `--minimal`. This is the stage's one departure from its scope,
  which said to build what is still *go*. What stands against building it here: under describe it is worth 433
  saturations of 1 348 056, on nine programs, and it is no memory's write but a change to how the state route
  walks. The route computes a layer's extensions before it saturates any, and a pair learned *after a death*
  prunes later extensions of the same layer, which is where the simulation finds its removals; so the fan-out
  has to be cut into steps that re-ask the memory, and every pair or single asked becomes a visited node of the
  run on every surface: the dump, `--events`, the states by closure, `N … met`. The owner is ready for it:
  `KnownDead::died(S, b)` with `S = {a}` is the pair.
- **A death's core** is not built and was not in scope: above.
- **The tree is not the simulation**: it closes a set under the states it reached the state from, and the
  dualisation's transversals not at all. What a full closure there would still remove is the `denials` row of
  the table above: nothing under `--single` and `-n 1`, 32 saturations under `--minimal`, 101 under `--maximal`
  (83 of them `pets.ein`).
- **A set the memory holds under another of its states** can be saturated once by a tree where the denial is
  one-sided; the state route has the case (`denied_again`). Not measured apart.
- **`send_more_money.ein` gets nothing**, by K-13's second half; the cheapest candidate is still a
  `functional` that stores the negation it uses, which is the stdlib's and nobody's yet.
- **No wall was taken**: the machine was shared.
- `./run_tests.sh`, the nightly and `utils/ci_container.sh` were not run: the batch's. A DOT writer changed
  (`region_dot.rs`, a tooltip and a table cell), so the container is part of the batch's done.

**For [S1f.10.5](s1f.10.5_ordering.md), the order.** The tree's choice already reads the denials through the
memory, so an order derived from the structure starts from a route that counts a denied alternative as dead.
On the zebras there is little left to order: `--single` is 9 against a floor of 3. On I-SEND nothing moved,
17 747 against 9, and that is the instance. Its pairs are not denied; a core gives them (8 220), the pair of a
death half of them (10 078), each with the order held. An order measured there should be measured with one of
the two in place, or it will be credited with what a nogood would have given.

**For [S1f.10.14](s1f.10.14_exactly_one_spent.md), *exactly one*.** A denial is *at most one* arriving from
the rules; an owed group one of whose members a state holds has every other member denied wherever the rules
store the negation, so *an owed group left no member* is now visible to the tree as an instance with no
alternative not known dead, before any of them is saturated. Whether that is spent as L5 already spends it,
the branch ending, or earlier, is its to measure. Under describe the alive and owing states are untouched:
5 530 of `zebra2-minus-15.ein`'s were and are the bulk of its stopped describe.

### The `jq` programs
The price list's two runs of this stage are under the untracked `build/` of the worktree it ran in, `build/structure/`
with the denials read and `build/alone/` from deaths alone, and the first run, before the closure step, under
`build/first/`; the command above re-takes each. The programs the tables were read with, verbatim:

`build/digest.jq`:

```jq
# jq -c -S -f build/digest.jq PRICES.jsonl | md5sum
# A price list's lines less what is the machine's and what reads the denials: the walls, the
# `denials` way and its counts. The fixture this stage adds is left out, so that a run from deaths
# alone can be held to the price list banked before it.
select(.path != "tests/region/10_denied_extensions.ein")
| del(.denials_read)
| .structure |= (del(.wall) | del(.root_layer.wall))
| .cells |= map_values(del(.wall) | del(.wall_watched) | del(.ways.denials)
    | .detail |= with_entries(select(.key | test("denied") | not)))
```

`build/totals.jq`:

```jq
# jq -r -n --slurpfile with WITH.jsonl --slurpfile old ALONE.jsonl -f build/totals.jq
# Per question over the programs both files hold: the totals from deaths alone and with the denials,
# in saturations and facts, over every cell and over the monotone cells no budget stopped either way;
# the cells lower and higher; and each cell higher at all, with by how much.
($with | map({key: .path, value: .}) | from_entries) as $new
| [$old[] | select($new[.path])] as $olds
| ("describe", "count", "single", "n1", "minimal", "maximal") as $q
| [$olds[] | {path, monotone, o: .cells[$q], w: $new[.path].cells[$q]}] as $cells
| ($cells | map(select(.monotone and .o.finished and .w.finished))) as $fin
| ([$q, "all", ($cells | length),
    ($cells | map(.o.work.saturations) | add), ($cells | map(.w.work.saturations) | add),
    ($cells | map(.o.work.facts) | add), ($cells | map(.w.work.facts) | add),
    ($cells | map(select(.w.work.saturations < .o.work.saturations)) | length),
    ($cells | map(select(.w.work.saturations > .o.work.saturations)) | length),
    ($cells | map(select(.w.work.saturations > .o.work.saturations * 1.1 or .w.work.facts > .o.work.facts * 1.1)) | length)] | @tsv),
  ([$q, "finished", ($fin | length),
    ($fin | map(.o.work.saturations) | add), ($fin | map(.w.work.saturations) | add),
    ($fin | map(.o.work.facts) | add), ($fin | map(.w.work.facts) | add),
    ($fin | map(select(.w.work.saturations < .o.work.saturations)) | length),
    ($fin | map(select(.w.work.saturations > .o.work.saturations)) | length),
    ($fin | map(select(.w.work.saturations > .o.work.saturations * 1.1 or .w.work.facts > .o.work.facts * 1.1)) | length)] | @tsv),
  ($cells[] | select(.w.work.saturations > .o.work.saturations or .w.work.facts > .o.work.facts)
   | ["  higher", .path, $q, .o.work.saturations, .w.work.saturations, .o.work.facts, .w.work.facts,
      (if .o.finished then "" else "stopped" end), (if .w.finished then "" else "stopped" end)] | @tsv),
  ($cells[] | select(.o.finished != .w.finished)
   | ["  finished otherwise", .path, $q, (.o.finished | tostring), (.w.finished | tostring)] | @tsv)
```

`build/t4.jq`:

```jq
# jq -r -n --slurpfile with WITH.jsonl --slurpfile old BANKED.jsonl -f build/t4.jq
# Each cell that moved, and every cell of the six instances: the work from deaths alone (the banked
# price list), with the denials read (a real run), and as the price list simulated the `denials` way.
def inst: {"examples/zebra.ein": "I-Z1", "examples/zebra2.ein": "I-Z2", "examples/zebra2-minus-15.ein": "I-Z2M",
  "examples/branching/06_lookahead_on.ein": "I-B06", "examples/lattice/02_genuine_3set_death.ein": "I-L02",
  "examples/beyond_arith/send_more_money.ein": "I-SEND"};
($with | map({key: .path, value: .}) | from_entries) as $new
| $old[] | . as $o | select($new[$o.path]) | $new[$o.path] as $w
| ("describe", "count", "single", "n1", "minimal", "maximal") as $q
| $o.cells[$q] as $oc | $w.cells[$q] as $wc
| select($oc.work.saturations != $wc.work.saturations or $oc.work.facts != $wc.work.facts or inst[$o.path] != null)
| [(inst[$o.path] // "-"), $o.path, $q,
   $oc.work.saturations, $wc.work.saturations, ($oc.ways.denials.with.saturations // "n/a"),
   $oc.work.facts, $wc.work.facts, ($oc.ways.denials.with.facts // "n/a"),
   (if $oc.finished then "" else "stopped" end), (if $wc.finished then "" else "stopped" end)]
| @tsv
```

`build/ways_after.jq`:

```jq
# jq -r -f build/ways_after.jq build/structure/prices/prices.jsonl
# The six instances and every program a way would still move, on top of the denials as built: per
# question the cell, and the cell with each way the stage re-prices, in saturations.
def inst: {"examples/zebra.ein": "I-Z1", "examples/zebra2.ein": "I-Z2", "examples/zebra2-minus-15.ein": "I-Z2M",
  "examples/branching/06_lookahead_on.ein": "I-B06", "examples/lattice/02_genuine_3set_death.ein": "I-L02",
  "examples/beyond_arith/send_more_money.ein": "I-SEND"};
select(inst[.path]) | inst[.path] as $i
| ("describe", "single", "n1", "minimal", "maximal") as $q | .cells[$q] as $c
| [$i, $q, $c.work.saturations,
   $c.ways.denials.with.saturations, $c.ways["layer-by-need"].with.saturations,
   $c.ways["death-pairs"].with.saturations, $c.ways["death-core-pair"].with.saturations,
   $c.ways["death-core"].with.saturations, $c.ways["theory-shared"].with.saturations,
   (if $c.finished then "" else "stopped" end)]
| @tsv
```

`build/movers.jq`:

```jq
# jq -r --arg q describe --arg w death-pairs -f build/movers.jq build/structure/prices/prices.jsonl
# The programs a way would move under a question: the cell, the cell with the way, what the way
# adds and removes, in saturations, and whether a budget stopped the cell.
select(.monotone) | .cells[$q] as $c | $c.ways[$w] as $x
| select($x.applies and (($x.removed.saturations // 0) > 0 or ($x.added.saturations // 0) > 0))
| [.path, $c.work.saturations, $x.with.saturations, $x.added.saturations, $x.removed.saturations,
   (if $c.finished then "" else "stopped" end)]
| @tsv
```
