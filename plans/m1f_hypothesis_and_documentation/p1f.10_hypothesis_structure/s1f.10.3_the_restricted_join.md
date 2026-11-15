# S1f.10.3 — At most one, spent: the extension step

**Phase:** [P1f.10](README.md), track R (the search)
**Status:** re-planned 2026-11-15. Not started. **Built only where
[S1f.10.13](s1f.10.13_the_price_list.md) says *go***; where it says *no* for every question, this file records
that and the stage is not built.
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
