# S1f.10.19 — A dead node stopped at its first contradiction

**Phase:** [P1f.10](README.md), track R (the search)
**Status:** planned 2026-11-16, on the user's *"Yes, both"* of that day. **T1 to T3 ran 2026-11-16**: [§ Record](#record-2026-11-16-t1-to-t3). **T4 done under FIFO, 2026-11-16**: [§ Record: T4](#record-2026-11-16-t4), on two defaults the user may revise (a core over the node's fixpoint; a container saved saturated may count other facts). **The agenda built 2026-11-16**, on the user's ruling of Q-M1f.313, (c): the `pairs` half first is the engine's order everywhere, [§ Record: the agenda](#record-2026-11-16-the-agenda).
**Depends on:** [S1f.10.13](s1f.10.13_the_price_list.md) (the rule for *go*). **Built after
[S1f.10.17](s1f.10.17_describe_by_borders.md) is merged**, since both move every route's `work`; its measurement does
not wait.

## The idea, as the user took it up

The user, 2026-11-16: *"Maybe we can also optimize saturation in general by firing first rules that emit owed
relations"*. The main session's answer: firing order changes nothing today, because a node runs to quiescence and is
never stopped at its first contradiction (`region.rs`, § What a node is), and whether it is discharged is read off
the quiescent KB. What would save work is the reverse: **stop a dead node at its first contradiction on the routes,
and fire first what leads to `(false)`**, so a dead node costs only the derivation of its contradiction. The user:
*"Yes, both"*.

## What changes, and what may not

- **R may not.** A dead node is outside R whatever else it derives, and L1 kills its supersets either way.
- **What a dead node reports does**: its facts beyond root (the `work` row), its debts (read on dead nodes too), its
  closure, and possibly its core (an explanation, which may vary: Q-M1f.6's ruling). So the early stop is for the
  routes alone; **the oracle keeps the definition**, a node run to quiescence. Every surface that prints something
  of a dead node a route stopped early is inventoried, and says what it printed is the node as far as its first
  contradiction, or takes it from elsewhere.
- **The agenda's order is ruled FIFO in written order** (Q-M1f.6, 2026-11-08: *"plain FIFO, core is an
  explanation, F22 makes it canonical"*). The early stop under FIFO needs no new ruling. **A fail-first order inside
  a node reopens that ruling**: it is measured here and put to the user as Q-M1f.313, and not built before the reply.

## The rule for *go*, written before the table is taken

[S1f.10.13](s1f.10.13_the_price_list.md#the-rule-for-go-written-before-the-table-is-taken)'s rule, read as its
[§ Defaults taken](s1f.10.13_the_price_list.md#defaults-taken) read it, applied to *a way* that here is **a dead node
a route computes stopped at its first contradiction**, under an agenda order:

- **go** for a question when, over the corpus programs that search, the total work of that question with the early
  stop is lower than without it, **and no program's is higher by more than one tenth**;
- **no** otherwise, with the programs that lose named.

How each of its defaults reads here, written before any cell is looked at:

1. **The units are the ones the stop moves, and a *go* needs both: facts beyond root and firings.** An early stop
   visits the nodes a route visits today and saturates each once, so a cell's saturations are the same by
   construction; they are given and decide nothing. Its facts beyond root (the `work` row's second unit, what the
   budget counts) and its firings (every application the saturator returns, a redundant one and the mirror's
   included: the work inside a node, which no row counts today) are what it changes.
2. **A stopped cell counts with what it had spent**: the stop is priced on the nodes the run visited. With the stop
   a budget-stopped run would reach further, since its dead nodes cost it less; that is not simulated, and the
   verdicts over the cells no budget stopped are given beside.
3. **A flagged program is in every total and the stop is not applied to it**: its route is the cube, and the
   detector reads every node's fixpoint there, a dead one's included. Its dead nodes are measured and shown
   beside.
4. **A unit in which a cell is 0** loses by any positive net.
5. **A dead node whose reader needs its derivation whole is not stopped**: the whole node the tree's order reads
   (`explore::Space::charge_whole_node`), whose couplings are read off every application. It is counted at
   quiescence with and without.
6. **The first contradiction** is the first point of the node's saturation, the commitment written and then each
   firing in order, at which the fork holds a contradiction of either of `contradiction::detect`'s shapes; the stop
   falls after the firing that completes it, every conclusion of that firing written, or before any firing where the
   commitment written already holds one.
7. **Two orders, three applications of the rule.** Under FIFO, the agenda as ruled (Q-M1f.6), against today. Under a
   fail-first order, through an internal switch and no route, against today. And fail-first with the stop against
   FIFO with the stop, which is what the order adds and what Q-M1f.313 asks; its losers are named as the others'
   are.

## Tasks

1. **T1f.10.19.1 — the inventory**: every reader of a dead node's facts, debts, closure and core on every surface
   and in every route (the memory of what is known dead, L8's denials, the coupling's whole node, the border
   route's dead sets), and what an early stop would change of each.
2. **T1f.10.19.2 — measured with no route changed**: on every corpus program that searches, every question, for
   each dead node a route computes, the firings and facts up to its first contradiction against those to
   quiescence, under FIFO; and under a fail-first order (the rules whose conclusion is `(false)` or leads to one,
   read off the plans, first), the same. The rule for *go* written here before the table.
3. **T1f.10.19.3 — Q-M1f.313**, *may the agenda inside a route's node fire first what leads to `(false)`?*, with
   T2's numbers, the options and a default, put and not acted on.
4. **T1f.10.19.4 — built where T2 says *go***, after S1f.10.17 is merged: the early stop under FIFO on the routes,
   behind the internal with/without switch and its test, the goldens named before they move, the pages
   (`read_out.md`'s `work` row, `finding_r.md`, `solution_semantics.md` § 1.5's node, `region.rs`' header). The
   fail-first order only on the user's ruling of Q-M1f.313.
5. **T1f.10.19.5 — the record.**

## Acceptance

- The oracle's node is unchanged; R, N and every count identical on every route, with and without, against the
  oracle.
- Every surface that prints something of a dead node says what it printed, and no golden moves that is not on T1's
  list.
- The work delta per program and question, losers named.
- Q-M1f.313 put with its numbers; no fail-first order before the user's reply.

## Record (2026-11-16): T1 to T3

**T1 to T3 ran to their end; T4 waits for S1f.10.17.** No route, no answer and no surface changed: the stage
added two internal switches to the saturator, which no route takes, and an example that takes them. Base `71f4adc3`;
the rule for *go* committed as `bd28e8eb` before any cell was taken.

**In one paragraph.** A dead node a route computes is saturated to quiescence today, and every dead node the routes
visit on the corpus holds the facts of its whole fixpoint, so `region.rs`' header is true and `explore.rs`'
`charge_whole_node` doc is not (K-51). Stopped at its first contradiction under FIFO, a dead node keeps from 10 % to
49 % of its firings and from 52 % to 78 % of its facts beyond root, by question, and **the stop is *go* for all six
questions** in both units, with no program higher at all: the monotone programs' firings fall by 29 % (describe) to
68 % (`--maximal`), their facts by 3 % to 22 %. **No answer reads a dead node's fixpoint beyond its dead bit**; what
moves is the run's accounting (the `work` row, the budget, the tree's order, which reads its share against the
facts), what the surfaces that list visited nodes print of a dead one (debts, closure, core, narration), and N's cores
wherever they are taken. A **fail-first agenda** adds a little more and is *go* against FIFO with the stop too, under
either reading: `--single` 13 % fewer firings, `-n 1` 15 %, `--maximal` 9 % to 11 %, `--minimal` 1 % to 4 %,
describe under 1 %, nearly all of it the two SEND programs; it moved **no fixpoint and no firing count to
quiescence of any of the 1 872 755 nodes read**. The `pairs` reading moves no core of any dead node, stopped or not,
and is higher than FIFO on no cell; `negations` saves more under `--minimal` and `--maximal` and moves 538 cores at
quiescence and 3 012 at the stop. It is put as
[Q-M1f.313](../open_questions.md#q-m1f313--may-the-agenda-inside-a-routes-node-fire-first-what-leads-to-false),
default (b) with `pairs`, not acted on.

### What was built

- **Two switches in [`saturator.rs`](../../../ein.rs/crates/ein-infer/src/saturator.rs)**, its header's § A
  fail-first agenda, each a thread-local read once when a `Saturator` is built (`new`, `resume`), in the style of
  `known_dead::from_deaths_alone` and `order::with`, and neither a lever (no `(config …)` field, option of `ein` or
  `EIN_*` name reaches them):
  - **`fail_first(FailFirst, f)`**: a candidate's rank in the closure's queue and in the boundary's parked set is its
    tiebreaker with a bit `LATE = 1 << 62` set where its plan's rule is not among the first, so the first half fires
    before the rest and FIFO holds within each; a resumed saturation re-ranks what the snapshot holds. Which rules are
    first is read off the compiled plans by `monotone::writes_of`: **`Pairs`**, a conclusion that is `(false)`, a
    stored negation, or a fact of a relation some plan concludes the negation of or the KB holds one of (the task's
    *a fact a contradiction can be made of*: either side of a pair); **`Negations`**, `(false)` or a stored negation
    alone. The boundary's events print the tiebreaker without the bit.
  - **`stopping_at_first_contradiction(f)`**: `Saturator::saturate` returns before any firing where the KB already
    holds a contradiction (`contradiction::has_contradiction`), and otherwise after the first productive firing one
    of whose conclusions makes one (`contradiction::contradicts`), without marking the KB saturated.
  - **Why neither changes anything untaken.** Untaken, `late` returns 0 at its first match arm, so a rank is the
    tiebreaker it was, and `first` is never filled; the stop's two checks are each behind `self.stop_first &&`,
    false. The `Snapshot` carries neither flag. Two unit tests,
    `a_fail_first_agenda_moves_the_order_and_not_the_fixpoint` (both readings, the fixpoint the same, FIFO again
    after the switch) and `a_saturation_stopped_at_its_first_contradiction`; and the whole of `ein-infer`'s suite,
    `lever_invariance`, `config_reference` and `id_order_invariance` green on the final tree ([§
    Verification](#verification)).
- **The example**, [`ein-infer/examples/early_stop.rs`](../../../ein.rs/crates/ein-infer/examples/early_stop.rs),
  and its row in [`utils/README.md`](../../../utils/README.md)'s measurement set. Each of the six questions is asked
  once per program at the default budget with a `Watch` on the nodes, the route as it is; each distinct node visited
  is saturated again through `region::Nodes::with_fixpoint_traced`, the node function's own fork of root: a dead one
  to quiescence and stopped, under FIFO and both agendas, and read at both points (firings, facts beyond root,
  closure, denials, debts, core); an alive one to quiescence under each, for its firings and its fixpoint; the whole
  node to quiescence under each.

### T1: the inventory

**Where a dead node is computed.** `region::node_in_region`, through `fan_out`, is every route's node: the tree's
and the cube's (`explore::Space::eval`), the state route's (`states.rs` calls `fan_out` itself), route 1's
(`downset.rs`, in production only on a flagged program whose root is dead), the oracle's (`region::compute_prepared`)
and the cores taken after a walk (`region::cores_after`). It saturates through `fixpoint_firing`, which
`Nodes::{with_fixpoint(s), with_fixpoint_traced, refutation, support, by_parent, by_split}` share. The whole node the
tree's order reads is the one dead node saturated by a loop of its own (`Nodes::with_fixpoint_supported_within`,
`structure::NodeCoupling`). Each runs to quiescence.

**Which of the two statements is true: `region.rs`'**, *"never stopped at the first contradiction, so a dead node's
debts are its fixpoint's"*. `explore.rs`' `charge_whole_node` doc, *"the cache's nodes stop at their first
contradiction"*, is false. Read: `Space::eval` → `fan_out` → `node_in_region` → `fixpoint_firing` →
`Saturator::saturate(…, None, …)`, which returns only at the two-phase fixpoint. Measured: for every one of the
251 733 distinct dead nodes the routes visit on the corpus, the facts beyond root the watch is handed are its FIFO
fixpoint's (`dead node's watched facts ≠ its FIFO fixpoint's`: 0), and 95 429 of them hold more facts at quiescence
than at their first contradiction. Filed as [K-51](findings.md); `explore.rs` is not edited here.

**The readers, and what a stop on the routes' dead nodes would change of each.** From the code of `ein-infer`,
`ein-render` and `ein-cli` (a read-only survey of the three crates' `src/`; the rows that decide where the stop may
go, the node function, `Space::eval`, the state route's dead record, the tree's `read_structure` and `check_owed`,
`charge_whole_node` and the summary's `root_owes`, read again against the code). *Monotone* is where the stop would
apply (default 3).

| reader | what it reads of a dead node | under a stop |
|---|---|---|
| **decided by the dead bit alone** | | |
| `known_dead` (L1): `died`, `died_set`, `tree.rs`' `record_dead`, `states.rs` | the commitment that died and the state it extended | nothing: a stopped node is dead exactly when its fixpoint is (the KB only grows) |
| L8: `KnownDead::denies`, `denied_through` | an **alive** state's denials; `Seen.denied` is emptied for a dead node | nothing |
| `tree.rs`: leaves, `extend`, `check_owed`, `register`; `states.rs`' `by_state` and `by_closure` | alive nodes alone (`register` and `check_owed` run on alive ones; dead nodes are not interned) | nothing |
| R, N, `k_s`, `k_c`, the lines, the merged states, the shape (`described.rs`, `states.rs`, `explore.rs`) | N is the minimal dead **sets**; every count reads alive classes | nothing |
| `ein test` and `solve :expect` (`claim.rs`) | R; root's node for `(false)`; a dead root's node is root's fixpoint, at 0 firings | nothing, but where a budget stops a run (below) |
| the read-out's spot check (`Nodes::by_parent`) | the parent's dead bit | nothing |
| **the run's accounting** | | |
| `Space::eval`, `states.rs`, `downset.rs`: `spent`, `Layer.facts` | a dead node's facts beyond root | the `work` row (`Answer.facts`; the read-out's `work` row, `--json-summary`'s and `answer.json`'s `work` block, `--events`' `region` line) falls by the dead nodes' facts after their stop |
| the budget: `Space::afford`, `affordable`; the state route's `per_node` projection; route 1's `projected` | the same, per node | a run the budget stops today reaches further, and says so in other numbers (`Stopped::Budget`'s `spent` and `projected`); one that finishes still finishes |
| `tree.rs`' `read_structure` and the order's gates | `sp.work()`'s facts: the whole node is read within `facts / READ_SHARE`, and retried at four times the facts | **a route decision**: the coupled order reads the whole node at another point of the run, so on the programs it reads it (the two SENDs here) the tree may branch otherwise. Work, never R; the states `-n N` and `--single` list first can move. Not simulated: the stage holds the route |
| `Stepped.facts` (`watch_step`) | the same | `-v` and `--layer-progress`'s step lines, `nodes.json`'s `steps` |
| `structure.rs`' pair layer (`step`, `layer`) | `affordable`, the work's delta | `ein render h-space`'s chunking and its budget stop, `PairLayer.work` |
| **what a dead node reports** | | |
| `narrate_node` → `Visited`, the `enter` event | `owes`, `owed`, `closure`, `facts`, `core`, `core_status`, `refuted_by`, and the node's own replayed events | `--dump-lattice`'s `nodes.json` and `nodes.md`, and `--events`' `enter` lines and the node's `fire` and `owe` lines, of every dead node visited: each the node as far as its first contradiction |
| `states::Dead{owes, core, core_status, refuted_by}`, `downset::Point` | N's members' cores, walk-eager under `--events` and `--dump-lattice` | N's cores in `nogoods.json` (`--dump-answer`) and `--trace`; measured below |
| `region::cores_after`, `AfterWalk::cores` | a core taken after the walk, by the same `node_in_region` | **must stop as the walk's node does**, or `ein-cli/tests/answer_round_trip.rs`' `eager_cores` and `ein-infer/tests/explore.rs`' `a_run_that_prints_no_core_takes_none_and_answers_alike` fail: they hold an after-walk core to the walk's |
| `Space::classes` | a dead node's state key is interned beside the alive ones | other dense ids; only equality among alive classes is read, and `Space::states()` by `structure.rs`' theory check, which no command asks |
| `owing_met` (`described.rs`) | debts of every node met | moves; no surface prints it (`ein-infer/tests/obligation_reports.rs` alone) |
| L9's premise check, `unheld::breach` in `node_in_region` | every fact of a node's fixpoint, a dead one's too | a partial check on a dead node: a breach its later derivations would show is not seen. It refuses runs, and answers nothing |
| **flagged programs alone (the stop not applied, default 3)** | | |
| `detector::stale_facts` | every node's fixpoint | the warning's stale rows, `--json-summary`'s `monotonicity.state`, `ein test`'s warning |
| `Space::records` → `status_if_whole` → `detector` (`dead_sites`, `with_fixpoints`, `owing_sites`) | `first_rules`, `absent_premises`, OWING on a dead node | the attribution of `Status::Exercised` and its witness; E where the cube is whole |
| **unchanged by construction** | | |
| the oracle (`region::compute`, `Topology::of`, `ein oracle --dump`, `ein render region`) | OWING on dead nodes (E, `discharged_not_upward_closed`), every node's class, closure, excluded, beyond, core | nothing: the oracle keeps the definition |
| the whole node (`node_coupling_in`, `charge_whole_node`) | every application | nothing: its own loop, and not stopped (default 5) |
| `Nodes::{refutation, support, with_fixpoints, by_split}`; `ein render h-space --supports`; the detector's re-fixpoints | the fixpoint through `fixpoint_firing` | nothing, if the stop is placed in the routes' node and not in `fixpoint_firing` |
| S1f.10.17's border route (not merged) | its dead sets | to be read when it is merged (T4) |

**What a stopped dead node reports, measured** (distinct dead nodes, the stop against the node's fixpoint;
`analyse.py`, [§ The analysis script](#the-analysis-script)). On the **121 105 dead nodes of the monotone programs**,
under FIFO: fewer facts on 95 395 (78.8 %), another closure on 4 120, other denials on 3 559, **other debts on
2 300**, **another core on 6 268** (5.2 %). On the 130 628 of the flagged programs: fewer facts on 34, another core on
4 791. No dead node a route visits dies before its first firing (`zero_firing` is 0 in every cell): the routes do not
saturate an extension a state they have met denies, root's denials among them (L8). A core moves with the facts
unchanged where the redundant firings after the contradiction would have recorded alternative derivations the core's
search reads: on the flagged cubes the contradiction is nearly always the last productive firing (`broken-chair`
completes 65 532 of their first contradictions), and their dead nodes' 1 870 046 firings are 222 501 with the stop.

### T2: the table

**Commands.** 2026-11-16, the release example at `03b0d857`, on AC, at `--jobs 8` and again at
`--jobs 3`, the two runs' `early_stop.jsonl` one digest:

```sh
cargo build --manifest-path ein.rs/Cargo.toml --release -p ein-infer --example early_stop
ein.rs/target/release/examples/early_stop --out build/early_stop/final --jobs 8 --alive-orders --cores \
    > build/early_stop/final/stdout.md 2> build/early_stop/final/progress.log
python3 build/early_stop/scripts/analyse.py build/early_stop/final/early_stop.jsonl
```

**200 programs search** (corpus files and register witnesses, by `corpus_ops::searches`), **171 monotone**; 1 200
cells, none an error, 17 describes and `--count`s, 4 `--minimal`s and 7 `--maximal`s stopped (the `--maximal`s and
seven of the describes are flagged cubes past the limit). 251 733 distinct dead nodes, 2 whole nodes (the two SENDs'),
1 621 022 distinct alive nodes.

**What held, each over every node** (`checks` in every line is `{}`):

| | |
|---|---|
| the stop by the hook = the stop read off the quiescent trace, firings and facts, every dead node under every order | 251 733 × 3 |
| a dead node's watched facts = its FIFO fixpoint's (so the routes' nodes run to quiescence) | 251 733 |
| every fact beyond root timed, the commitment's or a firing's | every dead node, every order |
| **the fixpoint the same under both fail-first agendas as under FIFO** | 251 733 dead and 1 621 022 alive nodes |
| **the firings to quiescence the same under both agendas** | the same nodes |
| no alive node dead under any order; the universe the routes built = the reader's | every cell |
| the watched nodes = the run's saturations | every cell |

**Monotone programs, where the stop applies.** Per question: the saturations and the dead nodes' share of them; the
facts beyond root and the firings today, with the stop under FIFO (and how much less), and under each agenda.

| question | stopped | saturations | dead (share) | facts today | FIFO stop | `pairs` | `negations` | firings today | FIFO stop | `pairs` | `negations` |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| describe | 10 | 1 234 630 | 16 237 (1.3 %) | 10 286 321 | 9 993 816 (2.8 % less) | 9 983 351 | 9 971 043 | 17 245 101 | 12 294 482 (28.7 % less) | 12 284 015 | 12 200 271 |
| `--count` | 10 | the same as describe | | | | | | | | | |
| `--single` | 0 | 37 664 | 32 567 (86.5 %) | 480 833 | 380 624 (20.8 % less) | 355 465 | 355 391 | 392 264 | 194 729 (50.4 % less) | 169 570 | 169 010 |
| `-n 1` | 0 | 34 587 | 29 100 (84.1 %) | 423 020 | 340 827 (19.4 % less) | 318 148 | 318 140 | 283 525 | 155 717 (45.1 % less) | 133 038 | 132 866 |
| `--minimal` | 4 | 367 000 | 88 805 (24.2 %) | 4 361 094 | 3 774 714 (13.4 % less) | 3 709 448 | 3 690 506 | 13 308 267 | 5 291 083 (60.2 % less) | 5 225 817 | 5 088 188 |
| `--maximal` | 0 | 41 121 | 35 225 (85.7 %) | 573 448 | 447 012 (22.0 % less) | 421 590 | 421 086 | 892 608 | 287 853 (67.8 % less) | 262 449 | 256 254 |

**The dead nodes alone** (monotone): what each keeps at its stop of what it derives to quiescence.

| question | dead nodes | facts at quiescence | at the FIFO stop | `pairs` | `negations` | firings at quiescence | at the FIFO stop | `pairs` | `negations` |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| describe, `--count` | 16 237 | 612 932 | 320 427 (52.3 %) | 309 962 | 297 654 | 5 527 658 | 577 039 (10.4 %) | 566 572 | 482 828 |
| `--single` | 32 565 | 436 988 | 336 779 (77.1 %) | 311 620 | 311 546 | 334 790 | 137 255 (41.0 %) | 112 096 | 111 536 |
| `-n 1` | 29 098 | 378 324 | 296 131 (78.3 %) | 273 452 | 273 444 | 248 795 | 120 987 (48.6 %) | 98 308 | 98 136 |
| `--minimal` | 88 805 | 1 579 307 | 992 927 (62.9 %) | 927 661 | 908 719 | 9 044 394 | 1 027 210 (11.4 %) | 961 944 | 824 315 |
| `--maximal` | 35 223 | 515 402 | 388 966 (75.5 %) | 363 544 | 363 040 | 800 084 | 195 329 (24.4 %) | 169 925 | 163 730 |

**Flagged programs, beside** (the stop not applied): describe, `--count` and `--maximal` are the same cube cells,
455 810 saturations of which 130 628 dead (28.7 %); their dead nodes' facts 1 033 559 → 1 033 525 under FIFO and
967 991 under either agenda, firings 1 870 046 → 222 501 under FIFO and 130 636 under either. `--single`, `-n 1` and
`--minimal` meet 22, 2 and 64 dead nodes there.

**Where the savings are** (firings, monotone; `analyse.py`'s § The largest savings). Under describe and `--minimal`,
the zebra family: `zebra2-minus-15.ein` describe 3 125 401 → 1 111 643 (budget-stopped), `zebra.ein` 582 506 →
118 966, `zebra2.ein` 231 191 → 42 045. Under `--single`, `-n 1` and `--maximal`, the two SENDs, then the zebras:
`send_more_money.ein --single` 136 970 → 65 442 firings and 221 803 → 173 444 facts, `zebra.ein --single` 22 756 →
3 342, `zebra2.ein` 8 023 → 932. A dead node of a zebra dies at a `(false)` its `std.algebra` or the puzzle's
rules derive at the boundary, after the closure has done most of its work, and then derives on: what completes the
first contradiction on the monotone programs, of 121 105 (`direct` 117 927, `pair` 3 178), is `injective` 87 059,
`column-wrong` 21 118, `leading-zero` 4 560, `functional` 2 084, `std.algebra.surjective` 1 050, `wrong` 997,
`transitive` 895, and the rest under 500 each.

**What the agenda adds, and where** (both with the stop). The two SENDs carry most of it: `send_more_money.ein
--single` 65 442 → 52 882 firings and 173 444 → 160 884 facts under either agenda. The zebra family takes little:
`zebra.ein` describe 118 966 → 118 966 (`pairs`) and 110 377 (`negations`). **`pairs` is higher than FIFO on no
cell; `negations` is higher on two programs**, `zebra2.ein` and `zebra2-obligations.ein`, by less than one percent
under five questions (describe 42 045 → 42 226 firings and 11 266 → 11 268 facts; `--single` and `--maximal` 932 →
942 firings).

**What the agenda moves of what a dead node reports** (distinct dead nodes, an agenda against FIFO, each with the
stop and each to quiescence). On the 121 105 of the monotone programs, `pairs`' stop against FIFO's: fewer or more
facts on 89 990, another closure on 2, other debts on 1, other denials on none, **another core on none**;
`negations`': facts on 92 848, closure on 2 761, denials on 2 879, debts on 2 709, **cores on 3 012**. To quiescence
the fixpoints are FIFO's, and so are their closures and debts; **`pairs` moves no core there either, `negations` 538**.
On the 130 628 of the flagged programs, both agendas' stops differ from FIFO's in facts on 65 534 and in debts on
39 201, and in no core, stopped or not.

#### The verdicts

By the rule written before the table, over every cell (the flagged programs' cells in every total, unchanged), in
facts beyond root then firings; a cell is the verdict, then the programs lower and those higher by more than a tenth
in each unit (`build/early_stop/final/stdout.md`, `verdicts.json`).

| comparison | describe, `--count` | `--single` | `-n 1` | `--minimal` | `--maximal` |
|---|---|---|---|---|---|
| FIFO stop, against today | **go** · 12 423 178 → 12 130 673 · 19 218 237 → 14 267 618 · 38/0, 56/0 | **go** · 480 998 → 380 789 · 392 351 → 194 816 · 27/0, 38/0 | **go** · 423 046 → 340 853 · 283 538 → 155 730 · 9/0, 19/0 | **go** · 4 361 641 → 3 775 261 · 13 308 721 → 5 291 537 · 19/0, 30/0 | **go** · 2 710 305 → 2 583 869 · 2 865 744 → 2 260 989 · 42/0, 59/0 |
| `pairs` stop, against today | **go** · → 12 120 208 · → 14 257 151 · 43/0, 60/0 | **go** · → 355 630 · → 169 657 · 32/0, 42/0 | **go** · → 318 174 · → 133 051 · 11/0, 19/0 | **go** · → 3 709 995 · → 5 226 271 · 21/0, 31/0 | **go** · → 2 558 447 · → 2 235 585 · 49/0, 65/0 |
| `negations` stop, against today | **go** · → 12 107 900 · → 14 173 407 · 43/0, 60/0 | **go** · → 355 556 · → 169 097 · 32/0, 42/0 | **go** · → 318 166 · → 132 879 · 11/0, 19/0 | **go** · → 3 691 053 · → 5 088 642 · 21/0, 31/0 | **go** · → 2 557 943 · → 2 229 390 · 49/0, 65/0 |
| `pairs` stop, against FIFO's | **go** · 14/0, 14/0 | **go** · 11/0, 11/0 | **go** · 7/0, 7/0 | **go** · 9/0, 9/0 | **go** · 23/0, 24/0 |
| `negations` stop, against FIFO's | **go** · 19/0, 19/0 | **go** · 16/0, 16/0 | **go** · 9/0, 9/0 | **go** · 12/0, 12/0 | **go** · 28/0, 34/0 |

**Every verdict is *go*, with no loser**, and over the cells no budget stopped they are the same (describe 384 126 →
329 815 facts and 3 788 238 → 2 872 255 firings under FIFO; the table is in `stdout.md`). No program is higher at
all under the FIFO stop or the `pairs` stop, against today or against FIFO; `negations` is higher than FIFO's stop
on the two programs above, by less than the tenth.

**How much of each verdict is the rule's and how much the measurement's.** Against today, a stop cannot lose: it
saturates the same node and stops at or before its end, so the facts and firings of a cell can only fall, and a
*go* is certain wherever a monotone cell has a dead node that derives after its first contradiction (56 of the 88
describe cells with a dead node, 38 of 66 under `--single`). What the table adds is how much. Against FIFO's stop,
an agenda can lose, and the table is the evidence that it does not by a tenth anywhere.

**What was not simulated** (default 2 and the route held): a budget-stopped cell would reach further with the stop,
and the tree's coupled order reads the whole node against the facts spent so far (`tree.rs`' `read_structure`), so
on the SENDs the stop can move when the order is read and so the order's work, either way. Both are T4's to measure
on the built route.

### T3: Q-M1f.313

Filed in [`open_questions.md`](../open_questions.md#q-m1f313--may-the-agenda-inside-a-routes-node-fire-first-what-leads-to-false)
with its index row: *may the agenda inside a route's node fire first what leads to `(false)`?* It reopens Q-M1f.6's
(a), the agenda and the boundary FIFO in written order with no band; the options are (a) FIFO as ruled, the stop
alone; (b) a fail-first agenda in the routes' nodes on a monotone program, the oracle's node, root's saturation and a
flagged program's cube FIFO; (c) fail-first everywhere. **Default (b), with `pairs`**: *go* against FIFO with the
stop on all six questions, higher than FIFO on no cell, and moving no core of any dead node of the corpus, stopped or
not; `negations` saves more under `--minimal` and `--maximal` and moves cores. **Put and not acted on**: no route
takes either switch.

### Defaults taken

Each the user's to revise. The first seven are the rule's reading, committed before the table (`bd28e8eb`).

1. to 7. As [§ The rule for *go*](#the-rule-for-go-written-before-the-table-is-taken) writes them.
8. **The fail-first agenda is two halves**, the rules first and the rest, FIFO within each, applied to the closure's
   queue and to the order the boundary walks its parked candidates in, not a ranking by how far a rule is from a
   contradiction. Two readings of *the rules whose conclusion a contradiction can be made of* are measured: `pairs`,
   the task's (either side of a pair), and `negations` (the side a rule supplies against a fact already there).
9. **The agenda is measured from root's FIFO snapshot**: every node resumes the same root, re-ranked, as option (b)
   would have it; root's own saturation under an agenda (option (c)) is not measured, root being alive on every
   program here and its fixpoint read as unmoved by the check on its nodes.
10. **A firing is every application the saturator returns**, a redundant one included: the stop cuts mostly the
    redundant tail of a dead node (describe's dead nodes: 10.4 % of their firings kept, 52.3 % of their facts).
11. **The first contradiction is read two ways and they must agree**, by the switch and off the quiescent run's
    trace; the table reads the switch.
12. **Cores are taken with the explanation search's default budget**, as a node's are, and compared as sorted text.
13. **The whole node is counted at the facts the run charged** (its applications' derived facts, as
    `charge_whole_node` is handed them) and at its firings to quiescence under each order.
14. **Nothing is taken out of a total for the flagged programs**: they are in it, unchanged, as S1f.10.13's
    default 3 has it; their own measurements are beside.

### What T4 would need

- **Where the stop goes**: in the routes' node, `node_in_region` as `fan_out` calls it for `Space::eval`, the state
  route and `cores_after` alike, and only on a monotone program (the axes carry no stale check there:
  `Axes::stale.is_none()`, or the monotonicity itself); not in `fixpoint_firing`, which the oracle, the detector and
  `Nodes`' readers share, and not in the whole node's loop. An internal with/without switch, as the plan says, and
  its test holding R, N, every count and every claim's outcome to the oracle with and without.
- **The goldens and tests to name before they move**: the `work` row and block of every searching program's
  read-out, summary and dump (`read_out_examples.md`, regenerated; `ein-cli`'s read-out goldens; the `work` and
  `steps` of `corpus_shapes.md5`'s lines that digest them), `--dump-lattice`'s dead nodes (owes, owed, closure,
  facts, core), `--events`' per-node lines, N's cores where taken (2 300 dead nodes' debts and 6 268 cores differ
  at the stop over the corpus), `corpus_cores.rs`' cores, `answer_round_trip.rs`' `eager_cores` and `explore.rs`'
  `a_run_that_prints_no_core_takes_none_and_answers_alike` (both paths stop alike, so they hold); the budget-stopped
  cells' reach (the 10 monotone describes and 4 `--minimal`s).
- **The pages**: `read_out.md`'s `work` row (a dead node's facts are those to its first contradiction),
  `finding_r.md` § 1 (`st(C)` for a dead `C` read as far as its first contradiction on a route, the oracle's to
  quiescence), `solution_semantics.md` § 1.5's node, `region.rs`' header (§ What a node is; and K-52), and
  `explore.rs`' `charge_whole_node` doc (K-51).
- **The tree's order re-measured**: the coupled order reads its share against the facts, which the stop lowers, so
  the SENDs' `--single`, `-n 1` and `--maximal` work is T4's to take on the built route, not this table's.
- **S1f.10.17's border route**: its dead sets read against this inventory once it is merged.
- **The agenda only on Q-M1f.313's ruling**; the switch is there.

### Verification

On the final tree, each green:

```sh
cargo test --manifest-path ein.rs/Cargo.toml --profile gate -p ein-infer
cargo test --manifest-path ein.rs/Cargo.toml --profile gate -p ein-cli --test lever_invariance --test config_reference
cargo test --manifest-path ein.rs/Cargo.toml --profile gate -p ein-render --test id_order_invariance
cargo fmt --manifest-path ein.rs/Cargo.toml --all --check
cargo clippy --manifest-path ein.rs/Cargo.toml --workspace --all-targets -- -D warnings
RUSTDOCFLAGS="-D warnings" cargo doc --manifest-path ein.rs/Cargo.toml --workspace --no-deps
python3 utils/check_hashmap_iteration.py
python3 utils/doc_audit.py --links --check docs plans README.md AGENTS.md corpus/README.md tests/README.md \
    examples/README.md stdlib/README.md utils/README.md c/README.md
```

### What was not done

- **T4**, which waits for S1f.10.17; and so the stop on a built route, a budget-stopped run's reach with it, and the
  tree's order under it.
- **Root under an agenda** (option (c)'s root) and **the oracle's cube under an agenda**: the checks above read every
  node a route visits, which on the flagged programs within the limit is their whole cube.
- **A graded agenda**, a rule ranked by how many derivations it is from a contradiction: two halves were measured.
- **A program whose negative cycle crosses the two halves**: the corpus's unstratifiable fixture,
  `features/07_unstratifiable.ein`, puts both its rules in one half, so the agenda could not change which one the
  boundary admits; the fixpoint check would report a program that did.
- **Walls**: none is reported; the machine was shared.
- `./run_tests.sh`, the nightly and `utils/ci_container.sh`: the batch's.

### The analysis script

`build/early_stop/scripts/analyse.py`, which writes the tables above from `early_stop.jsonl` (`python3
build/early_stop/scripts/analyse.py build/early_stop/final/early_stop.jsonl`), verbatim:

```python
#!/usr/bin/env python3
"""Tables for S1f.10.19's record, read off build/early_stop/run/early_stop.jsonl.

python3 build/early_stop/scripts/analyse.py build/early_stop/run/early_stop.jsonl
"""
import json
import sys
from collections import Counter

rows = [json.loads(l) for l in open(sys.argv[1]) if l.strip()]
rows = [r for r in rows if "cells" in r]
Q = ["describe", "count", "single", "n1", "minimal", "maximal"]
O = ["fifo", "pairs", "negations"]


def today(c):
    return c["facts"], c["firings_alive"]["fifo"] + c["firings_whole"]["fifo"] + c["dead_quiet"]["fifo"][0]


def withs(c, o, applied):
    if not applied:
        return today(c)
    saved = c["facts_dead"] - c["dead_stop"][o][1]
    return c["facts"] - saved, c["firings_alive"][o] + c["firings_whole"][o] + c["dead_stop"][o][0]


def n(x):
    return f"{x:,}".replace(",", " ")


def pct(a, b):
    return "–" if b == 0 else f"{100.0 * a / b:.1f} %"


print("## Per question: monotone programs, where the stop applies\n")
print("| question | cells stopped | saturations | dead (share) | facts today | FIFO stop | pairs | negations | firings today | FIFO stop | pairs | negations |")
print("|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|")
for q in Q:
    st = sat = dead = 0
    f = [0] * 4
    g = [0] * 4
    for r in rows:
        if not r["monotone"]:
            continue
        c = r["cells"][q]
        st += c["stopped"] is not None
        sat += c["saturations"]
        dead += c["dead"] + c["whole"]
        t = today(c)
        f[0] += t[0]
        g[0] += t[1]
        for i, o in enumerate(O):
            w = withs(c, o, True)
            f[i + 1] += w[0]
            g[i + 1] += w[1]
    print(f"| {q} | {st} | {n(sat)} | {n(dead)} ({pct(dead, sat)}) | {n(f[0])} | {n(f[1])} ({pct(f[0]-f[1], f[0])} less) | {n(f[2])} | {n(f[3])} | {n(g[0])} | {n(g[1])} ({pct(g[0]-g[1], g[0])} less) | {n(g[2])} | {n(g[3])} |")

print("\n## Per question: flagged programs, beside (the stop not applied)\n")
print("| question | cells stopped | saturations | dead (share) | dead nodes' facts today | FIFO stop | pairs | negations | dead nodes' firings today | FIFO stop | pairs | negations |")
print("|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|")
for q in Q:
    st = sat = dead = 0
    f = [0] * 4
    g = [0] * 4
    for r in rows:
        if r["monotone"]:
            continue
        c = r["cells"][q]
        st += c["stopped"] is not None
        sat += c["saturations"]
        dead += c["dead"] + c["whole"]
        f[0] += c["facts_dead"]
        g[0] += c["dead_quiet"]["fifo"][0]
        for i, o in enumerate(O):
            f[i + 1] += c["dead_stop"][o][1]
            g[i + 1] += c["dead_stop"][o][0]
    print(f"| {q} | {st} | {n(sat)} | {n(dead)} ({pct(dead, sat)}) | {n(f[0])} | {n(f[1])} | {n(f[2])} | {n(f[3])} | {n(g[0])} | {n(g[1])} | {n(g[2])} | {n(g[3])} |")

print("\n## Per question: the dead nodes alone, monotone programs\n")
print("| question | dead nodes | facts at quiescence | at the FIFO stop | pairs | negations | firings at quiescence | FIFO stop | pairs | negations | stopped before any firing |")
print("|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|")
for q in Q:
    d = z = 0
    f = [0] * 4
    g = [0] * 4
    for r in rows:
        if not r["monotone"]:
            continue
        c = r["cells"][q]
        d += c["dead"]
        z += c["zero_firing"]
        f[0] += c["facts_dead"]
        g[0] += c["dead_quiet"]["fifo"][0]
        for i, o in enumerate(O):
            f[i + 1] += c["dead_stop"][o][1]
            g[i + 1] += c["dead_stop"][o][0]
    print(f"| {q} | {n(d)} | {n(f[0])} | {n(f[1])} ({pct(f[1], f[0])}) | {n(f[2])} | {n(f[3])} | {n(g[0])} | {n(g[1])} ({pct(g[1], g[0])}) | {n(g[2])} | {n(g[3])} | {n(z)} |")

print("\n## Where a fail-first agenda does worse than FIFO, both with the stop (any amount)\n")
for o in ["pairs", "negations"]:
    for q in Q:
        worse = []
        for r in rows:
            if not r["monotone"]:
                continue
            c = r["cells"][q]
            b = withs(c, "fifo", True)
            w = withs(c, o, True)
            if w[0] > b[0] or w[1] > b[1]:
                worse.append(f"`{r['program']}` facts {b[0]}→{w[0]}, firings {b[1]}→{w[1]}")
        if worse:
            print(f"- {o}, {q}: {len(worse)}: " + "; ".join(worse))

print("\n## The largest savings under FIFO, firings, per question (monotone)\n")
for q in Q:
    best = []
    for r in rows:
        if not r["monotone"]:
            continue
        c = r["cells"][q]
        t = today(c)
        w = withs(c, "fifo", True)
        p = withs(c, "pairs", True)
        ng = withs(c, "negations", True)
        if t[1] - w[1] > 0:
            best.append((t[1] - w[1], r["program"], t, w, p, ng, c["stopped"]))
    best.sort(reverse=True)
    print(f"\n### {q}\n")
    print("| program | stopped | facts today → FIFO / pairs / negations | firings today → FIFO / pairs / negations |")
    print("|---|---|---|---|")
    for s, prog, t, w, p, ng, stp in best[:8]:
        print(f"| `{prog}` | {stp or ''} | {n(t[0])} → {n(w[0])} / {n(p[0])} / {n(ng[0])} | {n(t[1])} → {n(w[1])} / {n(p[1])} / {n(ng[1])} |")

print("\n## What completes the first contradiction under FIFO, distinct dead nodes\n")
for mono in [True, False]:
    by = Counter()
    kinds = Counter()
    for r in rows:
        if r["monotone"] != mono:
            continue
        by.update(r["first_contradiction"]["by"])
        kinds.update(r["first_contradiction"]["kind"])
    print(f"- {'monotone' if mono else 'flagged'}: {dict(kinds)}; " + ", ".join(f"{k} {v}" for k, v in by.most_common(12)))

print("\n## What a stopped dead node reports that its fixpoint does not, distinct dead nodes\n")
for mono in [True, False]:
    rep = Counter()
    d = 0
    for r in rows:
        if r["monotone"] != mono:
            continue
        rep.update(r["reports_moved"])
        d += r["nodes"]["dead"]
    print(f"- {'monotone' if mono else 'flagged'}, {n(d)} dead nodes: " + ", ".join(f"{k} {n(v)}" for k, v in sorted(rep.items())))

print("\n## Monotone programs with dead nodes, and those the stop saves nothing on\n")
for q in Q:
    with_dead = saving = 0
    for r in rows:
        if not r["monotone"]:
            continue
        c = r["cells"][q]
        if c["dead"] > 0:
            with_dead += 1
            if withs(c, "fifo", True)[1] < today(c)[1]:
                saving += 1
    print(f"- {q}: {with_dead} cells with a dead node, {saving} where the FIFO stop saves firings")
```

## T4: the inventory, before anything moved (2026-11-16)

Committed before the code and before any golden moved. Base `f70552d5` (master), S1f.10.17 not merged on this
branch; its border route is no question's route, so nothing here reads it. The stop is built under FIFO; **the agenda
is not touched** (Q-M1f.313 is the user's): the stop is a setting of a built saturator
(`Saturator::stop_at_first_contradiction`), an agenda is read when one is built, and the two share no code.

### Where the stop goes, and where it does not

- **In the node function, `region::node_in_region`, through `fan_out`, where the axes say so**
  (`Axes::stopping_at_first_contradiction`), for every node but root's (its commitment is empty). Two places set it,
  both on a monotone program alone: the modes' routes (`explore::Space::stopping_dead_nodes`, called where
  `run_with` opens the tree's space; a flagged program's space is the cube's and is not stopped) and the state route
  (`states::compute_resumable`, which declines a flagged program before it builds its axes). `cores_after` reads
  with `Axes::after_walk`, a clone, so a node taken after the walk stops where the walk's does.
- **Not stopped**: root's node; every node of a flagged program (the detector reads each fixpoint); the oracle
  (`region::compute_prepared` builds its own axes); route 1 (`downset.rs`, in production only on a flagged program
  whose root is dead, and held node for node to the oracle with its `OWING` bit by `region.rs`'
  `route_1_is_the_oracle`); **a space a reader of the structure opens** (`structure::with_space`, `ein render
  h-space`); the whole node the tree's order reads (`Nodes::with_fixpoint_supported_within`); every reader of
  `region::Nodes`; `claim.rs`' `fixpoints`.
- **The internal switch**: `region::to_quiescence(f)`, a thread-local read when a route builds its axes, as
  `known_dead::from_deaths_alone` is; no `(config …)` field, option or `EIN_*` name reaches it, and
  `config_reference.rs` gains no row. `ein-infer/examples/early_stop.rs` asks its routes under it, so it measures
  against the quiescent routes as it did.

### Two readings of *What T4 would need* that the built route departs from, and why

1. **A core is taken over the node's fixpoint, not over the stop.** Where a surface prints a core (`--dump-lattice`,
   `--events`, `--dump-answer`'s `nogoods.json`, `--trace`), the stopped saturation goes on from where it stopped,
   silently, firing what it would have fired, and the core is taken there; everything else the node reports is
   read at the stop first. Built first as the record had it (the core over the stopped node), three things failed
   that the record did not list: `corpus/wrong_answers.toml`'s register, whose rows hold N's cores as answers (the
   **fixed** rows `W21/typed-blind-solve` and `W21/blind-enumeration` no longer answered their right answers: e.g.
   `(seats Ann Sortal)`'s core became `(seats Ann C1) (seats Ann Sortal)`, the minimal core over every derivation
   lost because the stop records fewer; and the open `W10/fork-delta-order` and `W10/second-preparation` moved);
   `corpus_cores.rs`' `no_core_is_printed_unqualified_from_a_cut_search`, which holds a printed core to the core
   search over the node function's fixpoint; and `ein solve`'s N agreeing with `ein oracle`'s, which W21's fix
   states. Over the fixpoint, every core is what it was: the register, `corpus_cores` and N's cores do not move.
   The core's cost, the rest of the node's saturation and the search, is counted in no `work` row, as the
   search's never was.
2. **The structure's readers keep the whole node.** Built as the record had it (every `Space::eval` stopped), the
   theory's check (`structure.rs`, `theory_share.rs`' `the_readings_agree_on_the_declared_control`) found a dead
   check node holding a fact the program's dead node did not, both stopped at different points: it reads a dead
   node's facts, `beyond`. And the pair layer's budget scenarios (`coupling.rs`'
   `a_layer_that_read_its_singles_has_its_holders_read`, `h_space_view.rs`' `a_layer_read_whole_has_its_holders_read`
   and the page's read-out) moved. The structure is no question's route, so its spaces are not stopped and
   `ein render h-space` prints what it printed.

### The goldens and contracts that move, by name

- **`ein-render/tests/golden/corpus_shapes.md5`**: 351 lines over 43 programs, every one monotone (`ein solve FILE
  --no-search`'s `lattice` row), and only these ops: `read-out[q]` and `cores[q]` for the same questions `q` per
  program (the `work` row; the `region` block's work, and `nodes.json`'s dead nodes' `facts`, `owes`, `owed` and
  `closure`), `region-trace` where `describe` moved (the trace prints describe's read-out), and `region-lattice` on
  three programs whose describe reaches further at the sweep's budget of 2¹⁴ facts. No `states`, `oracle`, `plan`,
  `saturate`, `hyp`, `explain`, `dot[…]` or `ir[…]` line moves, and no line of a flagged program.

  | program | `read-out[q]`, `cores[q]` | also |
  |---|---|---|
  | `examples/beyond_arith/cryptarithm.ein` | describe maximal minimal n2 single | region-trace |
  | `examples/beyond_arith/equation_by_search.ein` | maximal |  |
  | `examples/beyond_arith/equation_two_unknowns.ein` | describe | region-trace |
  | `examples/beyond_arith/send_more_money.ein` | describe maximal minimal n2 single | region-trace |
  | `examples/beyond_arith/send_more_money_written.ein` | describe maximal minimal n2 single | region-trace |
  | `examples/binary_logic/equivalent.ein` | describe minimal | region-trace |
  | `examples/binary_logic/knights.ein` | describe maximal minimal | region-trace |
  | `examples/binary_logic/not_equivalent.ein` | describe maximal minimal | region-trace |
  | `examples/binary_logic/satisfiable.ein` | describe maximal minimal n2 single | region-trace |
  | `examples/binary_logic/synthesis.ein` | describe maximal | region-trace |
  | `examples/binary_logic/truth_table.ein` | describe maximal minimal | region-trace |
  | `examples/binary_logic/unsatisfiable.ein` | describe maximal minimal n2 single | region-trace |
  | `examples/branching/01_saturate_only.ein` | describe maximal n2 single | region-trace |
  | `examples/branching/02_one_dead_one_alive.ein` | describe maximal | region-trace |
  | `examples/branching/03_five_hyps_one_alive.ein` | describe maximal n2 single | region-trace |
  | `examples/branching/04_two_levels.ein` | describe maximal n2 single | region-trace |
  | `examples/branching/05_mini_zebra.ein` | describe maximal n2 single | region-trace |
  | `examples/branching/06_lookahead_on.ein` | describe maximal n2 single | region-trace |
  | `examples/branching/07_lookahead_off.ein` | describe maximal n2 single | region-trace |
  | `examples/branching/08_hypothesis_relation_whitelist.ein` | describe maximal | region-trace |
  | `examples/branching/09_hrule.ein` | describe maximal n2 single | region-trace |
  | `examples/branching/10_kill_cache_on.ein` | describe maximal n2 single | region-trace |
  | `examples/branching/11_kill_cache_off.ein` | describe maximal n2 single | region-trace |
  | `examples/branching/12_typed_blind_solve.ein` | describe maximal n2 single | region-trace |
  | `examples/ein-bugs/wrong-answers/forall-core-elim.ein` | describe maximal n2 single | region-trace |
  | `examples/ein-bugs/wrong-answers/oblig-derived-domain.ein` | describe maximal minimal n2 single | region-trace |
  | `examples/ein-bugs/wrong-answers/oblig-fresh-name-two.ein` | describe maximal minimal n2 single | region-trace |
  | `examples/ein-bugs/wrong-answers/oblig-fresh-name.ein` | describe maximal minimal n2 single | region-trace |
  | `examples/lattice/synth/merged_states.ein` | describe maximal | region-trace |
  | `examples/saturation/type-exclusivity/colors.ein` | maximal |  |
  | `examples/saturation/type-exclusivity/nationalities.ein` | maximal |  |
  | `examples/saturation/type-exclusivity/pets.ein` | describe maximal | region-trace |
  | `examples/word_problems/boxes13.ein` | describe maximal | region-trace |
  | `examples/zebra.ein` | describe maximal minimal n2 single | region-lattice region-trace |
  | `examples/zebra2-hints.ein` | describe maximal n2 single | region-trace |
  | `examples/zebra2-minus-15-obligations.ein` | describe maximal minimal n2 single | region-trace |
  | `examples/zebra2-minus-15.ein` | describe maximal minimal n2 single | region-trace |
  | `examples/zebra2-obligations.ein` | describe maximal minimal n2 single | region-lattice region-trace |
  | `examples/zebra2.ein` | describe maximal minimal n2 single | region-lattice region-trace |
  | `tests/region/02_merged_single.ein` | describe maximal | region-trace |
  | `tests/stdlib/bijection/06_blind_enumeration.ein` | describe maximal n2 single | region-trace |
  | `tests/stdlib/bool/inputs/01_in_drives.ein` | describe maximal minimal | region-trace |
  | `tests/stdlib/bool/inputs/02_required.ein` | describe maximal minimal n2 single | region-trace |

  The **reach at 2¹⁴** is the budget-stopped cells' reach in this golden: `zebra2.ein` and `zebra2-obligations.ein`
  describe exhaust where they stopped before layer 3, and `zebra.ein` describe is complete up to states of two
  candidates where it was one.
- **`docs/kernel/inference/read_out_examples.md`**, regenerated by `python3 utils/read_out_examples.py --bin
  ein.rs/target/gate/ein`: five `work` rows, X1's `13 saturations · 60` → `45 facts`, X5's `18 saturations · 46` →
  `45`, and X10's, zebra2's describe `263 saturations · 22 503` → `11 576` (twice) and its `--single` `7 · 666` →
  `278`.
- **`ein-render/tests/golden/trace_{ambiguous,one-hypothesis,two-level}.md`**: their `work` rows (608 → 328, 1 048 →
  536, 794 → 656 facts; saturations the same). `trace_{dead,owing,unconditional}.md` do not move.
- **The transcripts**: `README.md`'s and `docs/guide/04_solving_the_whole_puzzle.md`'s `ein solve examples/zebra2.ein
  --single`, its `work` row, `7 saturations · 666 facts` → `278` (`guide_transcripts.rs`).
- **`examples/README.md`'s anchor list** gains `ein-infer/tests/first_contradiction.rs`, which names the two zebra
  puzzles (`world_anchors.rs`).
- **`ein-cli/tests/einb_cli.rs`' `a_container_saved_again_is_the_container`**, a contract, not a golden: a container
  saved `--saturate` prints its source's read-out but for the facts its `work` row counts (`examples/zebra2.ein`:
  11 266 against 11 576, the same 263 saturations). Root's derivations are in the container before root's
  saturation runs, so it compiles the plans in another order, a node's FIFO order follows the plan list, and the
  place of a dead node's first contradiction follows the order. `kb.rs`' `cmd_save` already calls such a container
  *a different starting point*; a container saved with the default, the loaded KB, stays byte-identical (the test
  beside it, four programs). The test now masks that one number for the saturated save, a default (§ Defaults
  taken), the user's to revise; past the budget such a container can also spend otherwise
  (`zebra2-minus-15.ein`'s describe: 427 887 facts spent against the source's 426 729, the same layer reached).
- **Not moving**: `docs/api/rust.md` and `embedding.rs` (they print saturations, 7); `corpus/wrong_answers.toml`;
  `corpus_cores.rs`; every member of N's core (`nogoods.json`, `--trace`); the oracle's dumps;
  `docs/kernel/inference/hypothesis_space.md`'s read-out; `corpus_exits.txt`; `cost_ms` and the slow set, **not
  re-taken**: the machine was not idle (other agents' load average 3 to 10), so `utils/corpus_cost.py` was not run.
- **Prose that quotes the moved numbers**: `finding_r.md` § 7's SEND row (18 030 → 18 031 saturations) and three
  `corpus/corpus.toml` notes (`cryptarithm.ein`, `send_more_money.ein`, `send_more_money_written.ein`: 18 031
  saturations, 173 415 facts, the couplings read at saturation 722 where it was 712).
- **What a dead node reports, on the surfaces that list visited nodes**: `--dump-lattice`'s `nodes.json` and
  `nodes.md` (`facts`, `owes`, `owed`, `closure` at the stop; `core`, `core_status`, `refuted_by` over the
  fixpoint), `--events`' node lines (`fire` up to the firing that makes the contradiction, `owe` read at the stop,
  `enter`'s `owes` at the stop and its core over the fixpoint), `-v` and `--layer-progress`' step facts. No golden
  pins them beyond `corpus_shapes.md5`'s `cores[q]`.
- **The nightly**: a sharded step for the new sweep's slow half (`deep-corpus-shards`, four shards), in
  `.github/workflows/nightly.yml` and `run_tests.sh`'s `--nightly` and `--dev-host` copies (`nightly_steps.rs`).

## Record (2026-11-16): T4

**The stop is built under FIFO, and every golden that moved is the inventory's.** On the base `f70552d5`, the
inventory committed first (`d92c7c8b`), then the code with its goldens and pages (`a761d181`), then this record.

### Where the stop is

- **`region::node_in_region`** saturates through `region::saturated`, which hands back the saturator; where the axes
  carry the stop (`Axes::stopping_at_first_contradiction`) and the commitment is not empty, it calls
  `Saturator::stop_at_first_contradiction`, and `saturate` returns after the firing whose conclusions make the
  fork's first contradiction (or before any firing, where the commitment written makes one). Everything the node
  reports is read there: its facts beyond root (`NodeOut::state`, the run's `work` and its budget), debts, closure,
  denials, the detector's `first_rules` and `absent_premises`, L9's breach, and its narration. **Then, where a core
  is taken, the same saturator goes on** (`Saturator::run_to_quiescence`), silently, and the core is taken over the
  fixpoint: the same firings in the same order, so the same core, status, rules and sources as before.
- **Set by** `explore::Space::stopping_dead_nodes` (the tree's space in `run_with`, monotone only) and
  `states::compute_resumable` (monotone by its decline); **kept off** the oracle, root, flagged programs, route 1,
  `structure::with_space`, the whole node and `region::Nodes`, by construction: none of them builds axes with it.
- **The switch**: `region::to_quiescence`, a thread-local read when a route builds its axes; no lever.
- **The agenda** is untouched: `saturator::fail_first` is read at a saturator's construction and no route takes it;
  ruling Q-M1f.313 (b) is a `fail_first(FailFirst::Pairs, …)` around the route's node, where the stop is set.

### What held, and the test that holds it

**`ein-infer/tests/first_contradiction.rs`**, `every_question_answers_alike_with_the_stop_and_without`: the six
questions on every corpus program and register witness that searches, with the stop and under `to_quiescence`,
through `ein_corpus::pool::map`, held to the definition written in its header before the cells were read. On every
program (`EIN_CORPUS_SLOW=1`, 202 programs, 1 212 cells, 1 038 of them monotone with root alive): no failure; a
program the stop does not reach equal field for field, the work included; every finished describe and `--count`
equal field for field but the work, **N with every member's core** (4 086 members) among the fields; `owing_met`
moved on none; no listing other states; no cell stopped with the stop where it finished without; every finished
describe the same saturations. Per commit, 188 programs, 1 128 cells, in 11 s to 26 s at 16 workers. Its slow half
whole was 332 s on four P-core threads with four workers, its shards 166, 125, 35 and 32 s (`EIN_CORPUS_SLOW=1
EIN_TEST_SHARD=k/4 EIN_TEST_JOBS=4 utils/bench_env.sh --cores PT:4 --require-ac cargo test … --test
first_contradiction`, on AC, load average 2 to 3 from other agents): a step of `deep-corpus-shards`.

Beside it, with no expectation edited: `explore.rs` (every mode against the oracle), `region.rs`, `claim_routes.rs`,
`known_dead.rs`, `order.rs`, `corpus_cores.rs`, `lever_invariance.rs`, `route_invariance.rs`, `answer_round_trip.rs`
(its `eager_cores` among them), `jobs_invariance.rs`, `import_invariance.rs`, `id_order_invariance.rs`'s permutation
and `wrong_answers.rs`, per commit and, for the first eight named in the task, with `EIN_CORPUS_SLOW=1`
(§ Verification).

### What moved

The inventory's list and nothing else: 351 lines of `corpus_shapes.md5` (the filter in § Verification), five
`work` rows of `read_out_examples.md`, three trace goldens, the two transcripts, the anchor list, `einb_cli.rs`'
saturated comparison, `finding_r.md` § 7's SEND row and three `corpus.toml` notes. The pages: `read_out.md`'s two
`work` rows, `answer_dump.md`'s `facts` counter and N's cores, `lattice_dump.md`, `events.md` (the node's lines,
the `owe` row and bullet, K-61), `finding_r.md` § 1 and § 8, `solution_semantics.md` § 1.5, `region.rs`' header
(K-52), `explore.rs`' module header and `charge_whole_node` (K-51), `saturator.rs`' header, `events.rs`' `Visited`,
`region_lattice.rs`' header.

### The work, per question, against the record's simulation

Monotone programs with root alive, every question at the default budget, a stopped cell with what it spent
(`EIN_CORPUS_SLOW=1 cargo test … --test first_contradiction -- --nocapture`, 2026-11-16); the simulation is § T2's
*FIFO stop* column.

| question | saturations, with / without | facts without | facts with the stop | simulated |
|---|---|---:|---:|---:|
| describe, `--count` | 1 234 652 / 1 234 652 | 10 286 374 | 9 996 689 (2.8 % less) | 9 993 816 |
| `--single` | 37 670 / 37 668 | 480 836 | 380 571 (20.9 % less) | 380 624 |
| `-n 1` | 34 689 / 34 589 | 423 020 | 341 397 (19.3 % less) | 340 827 |
| `--minimal` | 394 981 / 367 002 | 4 361 094 | 4 307 684 (1.2 % less) | 3 774 714 |
| `--maximal` | 41 140 / 41 138 | 573 494 | 446 769 (22.1 % less) | 447 012 |

Within a few hundredths of a percent of the simulation but in two places, each the record's *not simulated*:
**`--minimal`'s four budget-stopped cells reach further** and spend their budget there (`send_more_money.ein`
205 166 → 217 607 saturations, `send_more_money_written.ein` 131 134 → 140 834, both `zebra2-minus-15` files
13 102 → 16 021; the same states listed), and **the coupled order reads the whole node at another moment** on the
SENDs (below). The ten budget-stopped describes reach the same layer with the same saturations, their spend lower.
No program does more work with the stop in facts; in saturations only where the order moved (+1 on the SENDs'
`--single` and `--maximal`, +100 on `send_more_money_written.ein -n 1`).

### The coupled order, re-measured

`tree.rs`' `read_structure` reads the whole node within a share of the facts the run has derived, and the run
derives fewer:

| program, question | without the stop | with it |
|---|---|---|
| `send_more_money.ein --single`, `--maximal` | couplings at saturation 712; 18 030 saturations, 221 803 facts | at 722; 18 031, 173 415 |
| `send_more_money.ein -n 1` | at 652; 15 855, 192 722 | at 652; 15 855, 151 744 |
| `send_more_money_written.ein --single`, `--maximal` | at 712; 18 030, 221 803 | at 722; 18 031, 173 415 |
| `send_more_money_written.ein -n 1` | at 1 872; 17 542, 199 031 | at 1 972; 17 642, 158 962 |

The same verdicts and the same one state listed; the two files still take the same work to the fact under
`--single` (`order.rs`' `the_two_sends_take_the_same_work`). Ten more saturations before the read under
`--single`, one more in all.

### Defaults taken

Each the user's to revise.

1. **A core is taken over the node's fixpoint**, the stopped saturation going on from the stop, where the record
   read it over the stopped node (§ T4's inventory, *Two readings*): the register's fixed W21 rows, `corpus_cores`
   and the oracle's agreement hold N's cores to the fixpoint's. The stop saves no work on a run that prints cores
   (`--dump-lattice`, `--events`, `--dump-answer`, `--trace`), and that run's `work` row is still the run's
   without them.
2. **The structure's readers are not stopped** (`ein render h-space`, the theory's check): no question's route,
   and the check reads a dead node's facts.
3. **Route 1 is not stopped**: in production it runs only on a flagged program with a dead root, and its test
   holds it to the oracle node for node, the `OWING` bit of dead nodes included.
4. **A container saved `--saturate` may count other facts in its `work` row than its source**, and past the budget
   spend otherwise; `einb_cli.rs`' saved-again test masks that number for the saturated save. The alternative is a
   saturated container that reproduces its source's plan order on load, or no stop; the default container, the
   loaded KB, stays byte-identical.
5. **The printed rows keep their words**: the `work` row prints what it printed, and `read_out.md`,
   `answer_dump.md` and `lattice_dump.md` define a dead node's facts on a monotone program as those to its first
   contradiction; no surface says *stopped* beside a node.
6. **`owing_met`** (no surface) is left out of the sweep's comparison and reported: it moved on no program.
7. **`cost_ms` and the slow set were not re-taken**: the machine was not idle.

### Verification

On the final tree, each green: `cargo test --manifest-path ein.rs/Cargo.toml --profile gate -p ein-infer -p
ein-cli -p ein-render -p ein-corpus`; `EIN_CORPUS_SLOW=1` once for `explore`, `known_dead`, `order`,
`corpus_cores`, `id_order_invariance`, `answer_round_trip` and `first_contradiction`; `cargo fmt --all --check`;
`cargo clippy --workspace --all-targets -- -D warnings`; `RUSTDOCFLAGS="-D warnings" cargo doc --workspace
--no-deps`; `python3 utils/check_hashmap_iteration.py`; `python3 utils/doc_audit.py --links --check docs plans
README.md AGENTS.md corpus/README.md tests/README.md examples/README.md stdlib/README.md utils/README.md
c/README.md`. The golden filter: `git diff -U0 -- ein.rs/crates/ein-render/tests/golden/corpus_shapes.md5`, every
moved line's program monotone and its op one of `read-out[q]`, `cores[q]`, `region-trace` and `region-lattice`,
listed by `build/t4/shapes.py` (kept under the worktree's `build/`).

### Not done

- The fail-first agenda (Q-M1f.313).
- `./run_tests.sh`, the nightly and `utils/ci_container.sh`: the batch's.
- `cost_ms` (default 7).
- S1f.10.17's border route was not merged on this branch; its dead sets are no question's.

## The agenda: the inventory, before anything moved (2026-11-16)

On the user's ruling of Q-M1f.313 that day, **(c)**: *"non-monotone programs still out of scope, we know about
them, can detect them, can solve them somehow, but don't claim about solution soundness, correctness or completeness
or anything."* Built after the stop under FIFO was committed and verified (`a761d181`, `24f6b478`), and committed
before the code and before any golden moved.

### What changes

- **The engine's order is the agenda with the `pairs` half first**, `saturator::AGENDA`: first the rules whose
  conclusion a contradiction can be made of, `(false)`, a stored negation, or a fact of a relation some plan
  concludes the negation of or the KB holds a negation of; then the rest; FIFO in the written and arrival order
  within each half; the boundary walks its parked candidates in the same order. Not behind a switch: every
  saturation takes it, root's, every route's node, the oracle's, the whole node, every `region::Nodes` reader,
  `ein saturate`, the stdlib's. `Saturator::new` takes the engine's; `Saturator::resume` its snapshot's (the
  snapshot carries it, so a node on a worker thread follows its root).
- **The switch** that stays: `saturator::fifo(f)`, the written order alone on the calling thread, and
  `saturator::fail_first(w, f)` for a measurement's halves; no lever reaches either.
- **The test**: `ein-infer/tests/agenda.rs`, every question and every claim of every monotone corpus program and
  register witness alike under the agenda and under `fifo`, its definition written in its header before the cells
  were read; flagged programs counted and not held.

### What moves, by name

Taken from a run of the built order before this commit, and checked against it: every program a line below moves
for is monotone (`ein solve FILE --no-search`'s `lattice` row), and no flagged program's rendering moves.

- **`ein-render/tests/golden/corpus_shapes.md5`**: 435 lines over 125 programs, only these ops: `saturate` (root's
  event stream, the narration: 103), `dot[kb-since]` (the KB drawn by the step that derived each fact: 85),
  `region-trace` (root's trace and the describe's `work` row: 115), `read-out[q]` (the `work` row, and the SENDs'
  `route` row, which says at which saturation the couplings were read: 64), `cores[q]` (the run's `work`, and the
  `facts`, `owes` and `closure` of the dead nodes a route stops: 66), and `states` on two programs
  (`examples/zebra.ein`, `examples/zebra2-hints.ein`: the comment beside a fact naming the rule of its first
  derivation, on 2 and 24 lines; every fact the same, read by a probe under both orders). No `hyp`, `hyp+closed`,
  `explain`, `explain-noalts`, `plan`, `match`, `naf`, `oracle`, `load`, `ir[…]` or other `dot[…]` line moves.
  By program, the questions `q` and the other ops:

  | program | `read-out[q]` | `cores[q]` | also |
  |---|---|---|---|
  | `examples/arith/div_by_zero.ein` |  |  | dot[kb-since] region-trace saturate |
  | `examples/beyond_arith/cryptarithm.ein` | describe maximal minimal n2 single | describe maximal minimal n2 single | region-trace |
  | `examples/beyond_arith/equation_by_search.ein` | maximal | maximal |  |
  | `examples/beyond_arith/send_more_money.ein` | maximal minimal n2 single | maximal minimal n2 single |  |
  | `examples/beyond_arith/send_more_money_written.ein` | maximal n2 single | maximal n2 single |  |
  | `examples/binary_arithmetic/div_by_zero.ein` |  |  | dot[kb-since] region-trace saturate |
  | `examples/binary_logic/knights.ein` | maximal minimal | maximal minimal |  |
  | `examples/binary_logic/not_equivalent.ein` | maximal | maximal |  |
  | `examples/binary_logic/satisfiable.ein` | describe maximal minimal n2 single | describe maximal minimal n2 single | region-trace |
  | `examples/binary_logic/synthesis.ein` | describe maximal minimal n2 single | describe maximal minimal n2 single | region-trace |
  | `examples/binary_logic/truth_table.ein` | maximal | maximal |  |
  | `examples/binary_logic/unsatisfiable.ein` | describe maximal minimal n2 single | describe maximal minimal n2 single | region-trace |
  | `examples/branching/08_hypothesis_relation_whitelist.ein` |  |  | region-trace saturate |
  | `examples/branching/12_typed_blind_solve.ein` |  |  | region-trace saturate |
  | `examples/domain_elim/ab.ein` |  |  | region-trace saturate |
  | `examples/domain_elim/b_only.ein` |  |  | region-trace saturate |
  | `examples/ein-bugs/alive-set-fresh-name-declared.ein` | describe maximal n2 single | describe maximal n2 single | region-trace |
  | `examples/ein-bugs/alive-set-fresh-name.ein` | describe maximal n2 single | describe maximal n2 single | region-trace |
  | `examples/ein-bugs/alt-cap-core-reordered.ein` |  |  | dot[kb-since] region-trace saturate |
  | `examples/ein-bugs/alt-cap-core.ein` |  |  | dot[kb-since] region-trace saturate |
  | `examples/ein-bugs/fork-delta-order.ein` | describe maximal | describe maximal | region-trace |
  | `examples/ein-bugs/wrong-answers/alt-flag-two-derivations.ein` |  |  | dot[kb-since] region-trace saturate |
  | `examples/ein-bugs/wrong-answers/closure-shared-given-control.ein` |  |  | region-trace saturate |
  | `examples/ein-bugs/wrong-answers/closure-shared-given.ein` |  |  | region-trace saturate |
  | `examples/ein-bugs/wrong-answers/fresh-name-default.ein` | describe maximal n2 single | describe maximal n2 single | region-trace |
  | `examples/ein-bugs/wrong-answers/match-only-declared.ein` | maximal | maximal |  |
  | `examples/ein-bugs/wrong-answers/match-only.ein` | maximal | maximal |  |
  | `examples/ein-bugs/wrong-answers/oblig-derived-domain.ein` | maximal | maximal |  |
  | `examples/ein-bugs/wrong-answers/or-one-key-twin.ein` |  |  | region-trace saturate |
  | `examples/ein-bugs/wrong-answers/underscore-core-two-ways-twin.ein` |  |  | region-trace saturate |
  | `examples/ein-bugs/zebra2-bad.ein` |  |  | dot[kb-since] region-trace saturate |
  | `examples/ein-bugs/zebra2-hints.ein` |  |  | dot[kb-since] region-trace saturate |
  | `examples/features/09_adjacent_via_same_house.ein` |  |  | dot[kb-since] region-trace saturate |
  | `examples/features/10_expect.ein` |  |  | region-trace saturate |
  | `examples/features/13_mixed_solution_and_open.ein` | describe maximal minimal n2 single | describe maximal minimal n2 single | region-trace |
  | `examples/lattice/synth/merged_states.ein` | describe maximal | describe maximal | region-trace |
  | `examples/zebra.ein` |  | describe minimal | dot[kb-since] region-trace saturate states |
  | `examples/zebra2-hints.ein` |  |  | dot[kb-since] region-trace saturate states |
  | `examples/zebra2-minus-15-obligations.ein` |  |  | dot[kb-since] region-trace saturate |
  | `examples/zebra2-minus-15.ein` |  |  | dot[kb-since] region-trace saturate |
  | `examples/zebra2-obligations.ein` |  |  | dot[kb-since] region-trace saturate |
  | `examples/zebra2.ein` |  |  | dot[kb-since] region-trace saturate |
  | `tests/region/02_merged_single.ein` | describe maximal | describe maximal | region-trace |
  | `tests/stdlib/algebra/22_symmetric_negative.ein` |  |  | region-trace saturate |
  | `tests/stdlib/bijection/01_setup_and_negatives.ein` |  |  | region-trace saturate |
  | `tests/stdlib/bijection/02_domain_elimination.ein` |  |  | dot[kb-since] region-trace saturate |
  | `tests/stdlib/bijection/03_range_elimination.ein` |  |  | dot[kb-since] region-trace saturate |
  | `tests/stdlib/bijection/06_blind_enumeration.ein` |  |  | region-trace saturate |
  | `tests/stdlib/bool/circuit/02_one_value.ein` |  |  | dot[kb-since] region-trace saturate |
  | `tests/stdlib/bool/inputs/01_in_drives.ein` | maximal | maximal |  |
  | `tests/stdlib/bool/inputs/02_required.ein` | describe maximal minimal n2 single | describe maximal minimal n2 single | region-trace |
  | `tests/stdlib/math/binary/02_a_name_in_a_bits_place.ein` |  |  | dot[kb-since] region-trace saturate |
  | `tests/stdlib/math/binary/03_ends_in_a_bit.ein` |  |  | dot[kb-since] region-trace saturate |
  | `tests/stdlib/math/binary/04_ends_in_zero.ein` |  |  | dot[kb-since] region-trace saturate |
  | `tests/stdlib/math/binary/05_ends_in_a_name.ein` |  |  | dot[kb-since] region-trace saturate |
  | `tests/stdlib/math/binary-arith/08_div_by_zero.ein` |  |  | dot[kb-since] region-trace saturate |
  | `tests/stdlib/math/binary-arith/10_a_bit_as_an_operand.ein` |  |  | dot[kb-since] region-trace saturate |
  | `tests/stdlib/math/binary-arith/14_let_twice.ein` |  |  | dot[kb-since] region-trace saturate |
  | `tests/stdlib/math/binary-arith/15_let_zero.ein` |  |  | dot[kb-since] region-trace saturate |
  | `tests/stdlib/math/binary-arith/16_let_bit.ein` |  |  | dot[kb-since] region-trace saturate |
  | `tests/stdlib/math/binary-arith/17_let_the_bit_zero.ein` |  |  | dot[kb-since] region-trace saturate |
  | `tests/stdlib/math/binary-arith/18_let_to_a_name.ein` |  |  | dot[kb-since] region-trace saturate |
  | `tests/stdlib/math/binary-arith/19_let_to_a_bit.ein` |  |  | dot[kb-since] region-trace saturate |
  | `tests/stdlib/math/binary-arith/20_let_to_the_bit_zero.ein` |  |  | dot[kb-since] region-trace saturate |
  | `tests/stdlib/math/decimal/02_in_a_let_nobody_asks_about.ein` |  |  | dot[kb-since] region-trace saturate |
  | `tests/stdlib/math/decimal/03_in_a_fact_of_the_programs_own.ein` |  |  | dot[kb-since] region-trace saturate |
  | `tests/stdlib/math/decimal/04_in_a_rules_conclusion.ein` |  |  | dot[kb-since] region-trace saturate |
  | `tests/stdlib/math/decimal-arith/20_div_by_zero.ein` |  |  | dot[kb-since] region-trace saturate |
  | `tests/stdlib/math/decimal-arith/21_div_by_zero_spelt_and_named.ein` |  |  | dot[kb-since] region-trace saturate |
  | `tests/stdlib/math/decimal-arith/22_malformed_digit.ein` |  |  | dot[kb-since] region-trace saturate |
  | `tests/stdlib/math/decimal-arith/23_malformed_fact_as_digit.ein` |  |  | dot[kb-since] region-trace saturate |
  | `tests/stdlib/math/decimal-arith/24_malformed_ends_in_digit.ein` |  |  | dot[kb-since] region-trace saturate |
  | `tests/stdlib/math/decimal-arith/25_malformed_ends_in_zero.ein` |  |  | dot[kb-since] region-trace saturate |
  | `tests/stdlib/math/decimal-arith/26_malformed_ends_in_a_name.ein` |  |  | dot[kb-since] region-trace saturate |
  | `tests/stdlib/math/decimal-arith/27_malformed_digit_as_number.ein` |  |  | dot[kb-since] region-trace saturate |
  | `tests/stdlib/math/decimal-arith/28_malformed_zero_digit_as_number.ein` |  |  | dot[kb-since] region-trace saturate |
  | `tests/stdlib/math/decimal-arith/31_malformed_through_a_name.ein` |  |  | dot[kb-since] region-trace saturate |
  | `tests/stdlib/math/decimal-arith/32_malformed_name_for_a_name.ein` |  |  | dot[kb-since] region-trace saturate |
  | `tests/stdlib/math/decimal-arith/33_let_twice.ein` |  |  | dot[kb-since] region-trace saturate |
  | `tests/stdlib/math/decimal-arith/36_let_digit.ein` |  |  | dot[kb-since] region-trace saturate |
  | `tests/stdlib/math/decimal-arith/37_let_zero_digit.ein` |  |  | dot[kb-since] region-trace saturate |
  | `tests/stdlib/math/decimal-arith/39_div_call_by_zero.ein` |  |  | dot[kb-since] region-trace saturate |
  | `tests/stdlib/math/int/12_div_euclid_by_zero.ein` |  |  | dot[kb-since] region-trace saturate |
  | `tests/stdlib/math/int/13_div_trunc_by_zero.ein` |  |  | dot[kb-since] region-trace saturate |
  | `tests/stdlib/math/int/14_div_floor_by_zero.ein` |  |  | dot[kb-since] region-trace saturate |
  | `tests/stdlib/math/int/15_letter_in_a_part.ein` |  |  | dot[kb-since] region-trace saturate |
  | `tests/stdlib/math/int/16_letter_in_a_negative_part.ein` |  |  | dot[kb-since] region-trace saturate |
  | `tests/stdlib/math/int/17_letter_in_the_second_operand.ein` |  |  | dot[kb-since] region-trace saturate |
  | `tests/stdlib/math/int/18_part_ends_in_a_digit.ein` |  |  | dot[kb-since] region-trace saturate |
  | `tests/stdlib/math/int/19_part_ends_in_a_named_constant.ein` |  |  | dot[kb-since] region-trace saturate |
  | `tests/stdlib/math/int/23_digit_as_a_part.ein` |  |  | dot[kb-since] region-trace saturate |
  | `tests/stdlib/math/int/24_letter_through_a_name.ein` |  |  | dot[kb-since] region-trace saturate |
  | `tests/stdlib/math/int/25_let_twice.ein` |  |  | dot[kb-since] region-trace saturate |
  | `tests/stdlib/math/int/26_let_zero.ein` |  |  | dot[kb-since] region-trace saturate |
  | `tests/stdlib/math/int/29_named_constant_as_a_part.ein` |  |  | dot[kb-since] region-trace saturate |
  | `tests/stdlib/math/int/30_letter_in_a_division.ein` |  |  | dot[kb-since] region-trace saturate |
  | `tests/stdlib/math/int/31_digit_as_the_negative_part.ein` |  |  | dot[kb-since] region-trace saturate |
  | `tests/stdlib/math/int/32_digit_zero_as_a_part.ein` |  |  | dot[kb-since] region-trace saturate |
  | `tests/stdlib/math/int/33_digit_zero_as_the_negative_part.ein` |  |  | dot[kb-since] region-trace saturate |
  | `tests/stdlib/math/int/34_named_constant_as_the_negative_part.ein` |  |  | dot[kb-since] region-trace saturate |
  | `tests/stdlib/math/int/35_undeclared_divisor.ein` |  |  | dot[kb-since] region-trace saturate |
  | `tests/stdlib/math/num9/09_div_by_zero.ein` |  |  | dot[kb-since] region-trace saturate |
  | `tests/stdlib/math/rat/10_div_by_zero_rational.ein` |  |  | dot[kb-since] region-trace saturate |
  | `tests/stdlib/math/rat/11_zero_denominator_first.ein` |  |  | dot[kb-since] region-trace saturate |
  | `tests/stdlib/math/rat/12_zero_denominator_second.ein` |  |  | dot[kb-since] region-trace saturate |
  | `tests/stdlib/math/rat/13_zero_denominator_one_operand.ein` |  |  | dot[kb-since] region-trace saturate |
  | `tests/stdlib/math/rat/14_letter_in_the_numerator.ein` |  |  | dot[kb-since] region-trace saturate |
  | `tests/stdlib/math/rat/15_letter_in_the_denominator.ein` |  |  | dot[kb-since] region-trace saturate |
  | `tests/stdlib/math/rat/16_denominator_ends_in_a_digit.ein` |  |  | dot[kb-since] region-trace saturate |
  | `tests/stdlib/math/rat/20_letter_through_a_name.ein` |  |  | dot[kb-since] region-trace saturate |
  | `tests/stdlib/math/rat/21_let_twice.ein` |  |  | dot[kb-since] region-trace saturate |
  | `tests/stdlib/math/rat/24_digit_as_the_numerator.ein` |  |  | dot[kb-since] region-trace saturate |
  | `tests/stdlib/math/rat/25_digit_zero_as_the_numerator.ein` |  |  | dot[kb-since] region-trace saturate |
  | `tests/stdlib/math/rat/27_digit_as_the_denominator.ein` |  |  | dot[kb-since] region-trace saturate |
  | `tests/stdlib/math/rat/28_digit_zero_as_the_denominator.ein` |  |  | dot[kb-since] region-trace saturate |
  | `tests/stdlib/math/rat/29_named_constant_as_the_denominator.ein` |  |  | dot[kb-since] region-trace saturate |
  | `tests/stdlib/slots/01_partition.ein` |  |  | dot[kb-since] region-trace saturate |
  | `tests/stdlib/slots/02_negative.ein` |  |  | region-trace saturate |
  | `tests/stdlib/slots/03_fill.ein` |  |  | dot[kb-since] region-trace saturate |
  | `tests/stdlib/slots/04_elimination.ein` |  |  | dot[kb-since] region-trace saturate |
  | `tests/stdlib/slots/05_no_room.ein` |  |  | region-trace saturate |
  | `tests/stdlib/slots/06_no_fill.ein` |  |  | region-trace saturate |
  | `tests/stdlib/slots/07_spatial_prune.ein` |  |  | region-trace saturate |
  | `tests/stdlib/slots/08_spatial_adjacent.ein` |  |  | region-trace saturate |
  | `tests/stdlib/slots/13_adjacent_bwd_neg_direction.ein` |  |  | region-trace saturate |

- **`docs/kernel/inference/read_out_examples.md`**, regenerated: X5's `work` row, `18 saturations · 45` → `44 facts`.
- **`ein-render/tests/golden/trace_{ambiguous,one-hypothesis,unconditional}.md`**: root's steps in the agenda's
  order (and with them the step numbers the prose cites). `trace_two-level.md`, `trace_dead.md` and
  `trace_owing.md` do not move.
- **`examples/README.md`'s anchor list** gains `ein-infer/tests/agenda.rs` (`world_anchors.rs`).
- **The SENDs**: `send_more_money.ein` and `send_more_money_written.ein` under `--single` and `--maximal`, 18 031 →
  18 032 saturations, 173 415 → 160 834 facts, the couplings read at saturation 722 → 732; `send_more_money.ein -n 1`
  15 855 saturations either way, 151 744 → 140 324 facts, read at 652 → 722; `send_more_money_written.ein -n 1`
  17 642 saturations, 158 962 → 147 725 facts, read at 1 972. So `finding_r.md` § 7's SEND row and three
  `corpus/corpus.toml` notes.
- **A gate test over a flagged program, named with why**: `ein-infer/tests/region.rs`'
  `fact_grain_on_programs_written_to_test_it` holds each fact-grain prototype's cube to what it was written to show.
  `even-depth` was written to show a hole the written order opens: `all-p`, written first, is admitted at the
  boundary before `bad`, whose conclusion is `(false)`. Under the agenda `bad` goes first, its guard `(absent (done
  W))` holds at that moment, and the node dies where it lived: no hole. The program is flagged, so the engine claims
  nothing of its answer; the test computes these cubes under `saturator::fifo`, as written.
- **The standing rule and pages**: `AGENTS.md`'s *the agenda is FIFO in written order*, restated in place;
  `saturator.rs`' header; `finding_r.md` § 1 and § 8; every `docs/kernel/` page that states the order FIFO as the
  engine's today (`inference/README.md`, `architecture_and_algorithms.md`, `absent_semantics.md`,
  `implementation.md`, `monotonicity.md`, `events.md`, `glossary.md`, `architecture.md`, `ir/03-ein-lang/01_grammar.md`,
  `ir/01-ein-graph/02_rules.md`; the names *the FIFO gap* and *FIFO race* of § 7 of `monotonicity.md` are a
  phenomenon's, kept); `defined_behaviour.md` defines no order. Q-M1f.313's *Built* paragraph and index row,
  Q-M1f.6's row.
- **The nightly**: the new sweep's slow half, a step of `deep-corpus-shards`, in the workflow and `run_tests.sh`.
- **Cores**, explanations the order moves (Q-M1f.6's ruling stands for them): N's cores move on 103 of the 3 804
  members the per-commit describes take (`agenda.rs`' line), on the surfaces that print them (`nogoods.json`,
  `--trace`, `cores[q]` above); `corpus_cores.rs` holds each still to its commitment and its contradiction.
- **Not moving**: `corpus/wrong_answers.toml` (no open or fixed row's answer, cores among them), the README and
  guide transcripts, `docs/api/rust.md`, `corpus_exits.txt`, every claim of the corpus (`ein test`), every lever
  cell, the `.einb` tests, `cost_ms` (not re-taken: the machine was not idle).

## Record (2026-11-16): the agenda

**The agenda with the `pairs` half first is the engine's order, and every golden that moved is the agenda
inventory's.** The inventory committed first (`bbbfe80c`), then the code with its goldens and pages (`1df62fce`),
then this record, all after the stop under FIFO (`a761d181`, `24f6b478`), which stands as committed.

### Where it is

- **`saturator.rs`**: `AGENDA = FailFirst::Pairs`, read off the compiled plans by `monotone::writes_of` as T2
  measured it; a candidate's rank is its tiebreaker with the `LATE` bit set where its rule is not among the first,
  in the closure's queue and in the boundary's parked set alike. `Saturator::new` takes the agenda a switch on the
  calling thread names, else `AGENDA`; `Saturator::resume` the switch's, else **its snapshot's** (`Snapshot` carries
  it, so a node a worker thread computes follows the root the calling thread saturated). A resumed saturation
  ranks again what its snapshot holds whenever either has an agenda.
- **The switches**: `saturator::fifo(f)`, the written order alone, for the with/without test; `fail_first(w, f)`,
  a measurement's halves. No lever reaches them. `ein-infer/examples/early_stop.rs` runs each program under
  `fifo`, so its baseline is what it measured.
- **The stop and the agenda stay two things**: the stop is `Saturator::stop_at_first_contradiction` on a built
  saturator; the agenda is a rank. A dead node is stopped at the first contradiction the agenda's order makes.

### What held

**`ein-infer/tests/agenda.rs`**, `every_question_and_claim_answers_alike_under_the_agenda_and_the_written_order`:
every corpus program and register witness, five questions (`--count` is describe's by construction) and its
claims under the agenda and under `fifo`, held to the definition in its header, written before the cells were
read. Under `EIN_CORPUS_SLOW=1`: **612 monotone programs, 3 060 cells, no failure**; the work moved on 60 cells; N's
cores on 103 of 4 234 members (explanations); 29 flagged programs counted and not held; 387 programs' claims read
under the question `ein test` asks first and the describing one, 774 readings, one outcome each; 156 s at 16
workers. Per commit, 600 programs, 3 000 cells, 385 with claims, 19 s to 28 s. Its slow half is a step of
`deep-corpus-shards`: 177, 116, 47 and 29 s a shard on four P-core threads with four workers (`EIN_CORPUS_SLOW=1
EIN_TEST_SHARD=k/4 EIN_TEST_JOBS=4 utils/bench_env.sh --cores PT:4 --require-ac cargo test … --test agenda`, on
AC, other agents loading the machine, load average 4 to 16; with six questions shard 1 took 263 s at a load of 17
to 22, which is why `--count` is not asked again).

Beside it, with no expectation edited: `explore.rs`, `region.rs` (but the one test named below), `claim_routes.rs`,
`known_dead.rs`, `order.rs`, `corpus_cores.rs`, `first_contradiction.rs`, `lever_invariance.rs`,
`route_invariance.rs`, `answer_round_trip.rs`, `jobs_invariance.rs`, `import_invariance.rs`, `wrong_answers.rs`,
`stdlib_coverage.rs`, `naf_semantics.rs`, `einb_cli.rs`, and the corpus's `ein test` exits (`corpus_cli.rs`).

### What moved

The agenda inventory's list and nothing else: 435 lines of `corpus_shapes.md5` over 125 monotone programs (the
same filter as T4's: every moved line's program monotone, its op one of `saturate`, `dot[kb-since]`,
`region-trace`, `read-out[q]`, `cores[q]` and `states`), `read_out_examples.md`'s X5 row, `trace_ambiguous.md`,
`trace_one-hypothesis.md`, `trace_unconditional.md`, the anchor list, `finding_r.md`'s SEND row and three
`corpus.toml` notes; and **one gate test over a flagged program**, `region.rs`'
`fact_grain_on_programs_written_to_test_it`, whose cubes are computed under `saturator::fifo`, with why in the test:
under the agenda the prototype `even-depth` admits `bad`, whose conclusion is `(false)`, before `all-p`, and the
hole it was written to show does not open (K-62). Pages: `AGENTS.md`'s rule; `saturator.rs`' header;
`finding_r.md` § 1 and § 8; the kernel pages that stated the order FIFO (the inventory's list), their sentences
that the order decides nothing on a stratified rule set now saying so of a monotone program; Q-M1f.313's index row
and *Built* paragraph, Q-M1f.6's row.

### The work, per question

Monotone programs, every question at the default budget, a stopped cell with what it spent, the stop in both
(`EIN_CORPUS_SLOW=1 cargo test … --test agenda -- --nocapture`, 2026-11-16):

| question | saturations, agenda / written | facts, written order | facts, the agenda |
|---|---|---:|---:|
| describe, `--count` | 1 235 091 / 1 235 091 | 9 996 689 | 9 986 224 (0.1 % less) |
| `--single` | 38 111 / 38 109 | 380 571 | 355 370 (6.6 % less) |
| `-n 1` | 35 128 / 35 128 | 341 397 | 318 718 (6.6 % less) |
| `--minimal` | 406 626 / 395 420 | 4 307 684 | 4 307 561 |
| `--maximal` | 41 581 / 41 579 | 446 769 | 421 305 (5.7 % less) |

The firings, which no row counts, are T2's to read (`pairs` against FIFO, both with the stop: `--single` 13 % fewer,
`-n 1` 15 %, `--maximal` 9 %); the engine's units fall as T2's facts did, nearly all on the two SENDs. `--minimal`'s
budget-stopped cells reach further again (`send_more_money.ein` 217 607 → 221 899 saturations,
`send_more_money_written.ein` 140 834 → 147 748). The SENDs, both files alike: `--single` and `--maximal` 18 032
saturations, 160 834 facts, the couplings read at saturation 732; `send_more_money.ein -n 1` 15 855 and 140 324,
read at 722; `send_more_money_written.ein -n 1` 17 642 and 147 725, read at 1 972.

### Defaults taken

1. **A resumed saturation takes its snapshot's agenda**, so a switch on the calling thread reaches the nodes its
   root's run fans out to workers.
2. **The boundary walks its parked candidates in the agenda's order**, as T2 measured it and Q-M1f.313's (c) says.
3. **`region.rs`' fact-grain prototypes' cubes are read in the written order**, the order they were written to
   expose, rather than their expectations changed; the program is flagged and nothing is claimed of it.
4. **`FailFirst::Negations` stays** as a measurement's reading.

### Verification

On the final tree, each green: `cargo test --manifest-path ein.rs/Cargo.toml --profile gate -p ein-infer -p
ein-cli -p ein-render -p ein-corpus`; `EIN_CORPUS_SLOW=1` once for `explore`, `known_dead`, `order`,
`corpus_cores`, `id_order_invariance`, `answer_round_trip`, `first_contradiction`, `agenda` and `region`;
`utils/check_test_scratch.py` on their scratch; `cargo fmt --all --check`; `cargo clippy --workspace --all-targets
-- -D warnings`; `RUSTDOCFLAGS="-D warnings" cargo doc --workspace --no-deps`; `python3
utils/check_hashmap_iteration.py`; `python3 utils/doc_audit.py --links --check docs plans README.md AGENTS.md
corpus/README.md tests/README.md examples/README.md stdlib/README.md utils/README.md c/README.md`.

### Not done

- `./run_tests.sh`, the nightly and `utils/ci_container.sh`: the batch's. The agenda moves every program's root
  narration, so the nightly's `stdlib_mutants` census (`tests/mutation_survivors.txt`) was not re-run here.
- `cost_ms`: not re-taken.
- A stratum-ordered boundary, which would make a flagged program's answer independent of the order (K-62): F21's.
