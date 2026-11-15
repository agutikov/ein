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
