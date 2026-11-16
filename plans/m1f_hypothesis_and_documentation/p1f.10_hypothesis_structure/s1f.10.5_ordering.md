# S1f.10.5 — The order on the obligation tree

**Phase:** [P1f.10](README.md), track R (the search)
**Status:** re-planned 2026-11-15. Not started.
**Depends on:** [S1f.10.2](s1f.10.2_groups.md) and [S1f.10.11](s1f.10.11_coupling.md), whose output is the
order's input; [S1f.10.13](s1f.10.13_the_price_list.md), for the ceiling. It does not depend on
[S1f.10.3](s1f.10.3_the_restricted_join.md), and where both are built the order is measured with it on.
**Waited for by:** `std.math.crypt`, the cryptarithm theory (reply 42 of the calculator's questions), whose
example's cost hangs on declaration order until an order is derived.
**Owns:** E-29 of the calculator's [`findings.md`](../p1f.6_calculator/findings.md).

## Context

The user, in the instruction of 2026-08-28:

> So hypothesis selection could be ordered by the structure of hypothesis set, not by simple lexical order
> (what else ordering do we have btw?)

and on 2026-11-12, to *should the search choose which owed thing to branch on by the state?*:

> Yes! Hypothesis structure phase. The concrete strategy that defines hypothesis order can be different, but
> it definitely will depend on the hyp structure. So first need to automate analysis/discovery of the
> structure itself, and then use it as input to hyp ordering strategy.

### The inventory: every order point the routes have

Answering the parenthesis, against the engine as it stands. The old plan's eight points were the walk's, and
six of them went with it.

| # | where | over what | today |
|---|---|---|---|
| 1 | `tree.rs`, `branch_rest` | which **owed instance** an owing state branches on | the licensed instance with the fewest alternatives not known dead; on a tie, the first in report order |
| 2 | the obligation pass's report order | the **instances**, as #1's tie-break reads them | obligation rules in written order, then each rule's matches, which for `total-owed` is the order of the `(is-a X T)` facts |
| 3 | `tree.rs`, the tree's queue | which **state** is extended next | owing states by size, then by closure: every state of *k* candidates before any of *k* + 1 |
| 4 | `tree.rs`, `step` | one instance's **alternatives** | ascending candidate index, which is rendered-text order; 1, 2, 4, … at a time |
| 5 | `tree.rs`, `climb` | the candidates tried on top of a state of R | ascending index |
| 6 | `states.rs` | describe's **states** | by size; every extension of each |
| 7 | the universe | the **candidates**' indices | rendered text, so no order depends on interner ids |
| 8 | the saturation agenda | **rule firings** | FIFO in written order. Not a search order, and where a reader looks next |

**Why E-29 happens**, as read from the table; T1 confirms it on the two programs' dumps before anything rests
on it. #1 counts only alternatives the run has already seen die, so on a
fresh state every instance of SEND + MORE ties at ten and #2 decides: the order the letters are declared in.
Then #3 takes every assignment of the first *k* letters before any of *k* + 1. Declared by column, a column's
three letters are adjacent in #2, a wrong column dies at size three, and each layer of #3 stays narrow.
Declared as written, S E N D come first, no column is complete until Y, and the layers grow by a factor near
ten each.

So two points decide the cost, #1 with #2, and #3; and the stage measures them separately, because a better
instance choice under a by-size queue and a depth-first queue under the written instance order are different
remedies.

### The doubt, stated before measuring

A table of excluded pairs gives #1 a real count: an instance's alternatives **not excluded by anything the
state holds**, with no saturation. On a bijection puzzle that is forward checking for free. On SEND it still
ties: *different letters, different digits* removes one alternative from every letter alike. What separates D
E Y from S E N D is that they share a column, which is a **coupling**
([S1f.10.11](s1f.10.11_coupling.md)). So the order this stage derives reads groups and couplings both, and if
S1f.10.11 found no cheap reading of SEND's columns, this stage says what an order from pairs alone reaches
and E-29 stays open, shown.

## Tasks

1. **T1f.10.5.1 — the inventory verified, and I-SENDW banked.** Each row of the table checked against the
   code. `examples/beyond_arith/send_more_money_written.ein`, the program of
   `build/s1f.6.10/p/p3_send_written.ein` in the language as it stands: I-SEND with the letters declared
   S E N D M O R Y and nothing else changed, generated from I-SEND by a script with a `--check` in the gate, as
   the zebra2 variants are, so that the two cannot drift. Its corpus entry runs `--single` at the default
   budget and records today's answer, *not decided*; whether it is `slow` is measured.
2. **T1f.10.5.2 — the strategies, as functions of the structure.** Each is a pure function from (the state,
   the groups, the hypergraph) to a choice at #1 or #3, with a canonical tie-break by rendered text so that no
   choice reads declaration order:
   - **fewest live alternatives**, live by the excluded pairs against the state's closure;
   - **most coupled to what the state holds**: the instance sharing a hyperedge with the most instances
     already paid; and its product with the first (the degree and *dom/deg* family);
   - **the queue**: by size as today; depth-first; best-first by the count above;
   - **learned weights**: an instance weighted by the deaths it took part in (*dom/wdeg*), which needs no
     structure in advance and is the control for whether the discovered structure earns its cost.
3. **T1f.10.5.3 — measured**, without changing the shipped route: each strategy through an internal switch for
   the measurement, as S1f.10.3's T3 has. Over the instances and every corpus program whose `--single` or
   `-n 1` takes the tree: saturations and facts for `--single`, `-n 1`, `--minimal` and `--maximal`. Every
   program on which a strategy is **worse** than today is named; a strategy with no losing cell has not been
   measured on enough programs. Against S1f.10.13's ceiling.
4. **T1f.10.5.4 — the order, built.** One strategy becomes the tree's own order. No lever chooses it. The
   `route` row of the read-out says in words what order the run took, since E-29's second half is that
   *nothing in the read-out says the order mattered*;
   [`read_out.md`](../../../docs/kernel/inference/read_out.md) and its examples in the same commit, and
   `tree.rs`'s § Which instance restated.
5. **T1f.10.5.5 — the invariances**, as tests in the gate:
   - **every order gives one answer**: the tree under the written order, its reverse, the order built and
     seeded permutations of #1's choice, each held to the describe or the oracle on every program a cube or a
     describe reaches;
   - **the work does not read the order a program is written in**: I-SEND and I-SENDW, which differ only in
     the order of their declarations, take the same number of saturations under `--single`; or the stage says
     what still reads written order and why.

## Acceptance

- **`ein solve --single` on I-SENDW is *Solution — R is one state* within the default `--region-budget`**, and
  its corpus entry's recorded answer is flipped in the commit that makes it so.
- I-SEND is no worse than today by more than S1f.10.13's tenth, or the stage says by how much and why the
  trade is taken.
- T5's two tests are in the gate and green, within the default budget.
- The verdict, R, N and H are the same under every order on every corpus program: the phase's standing
  acceptance, by the sweeps its README names.
- The read-out says what order a run took, and the page defines the words.
- E-29 is closed in the calculator's `findings.md` with what was done, or left open with what an order from
  the discovered structure does not reach.
- `./run_tests.sh` green on the batch's last commit, and `--nightly` once, since a route changed.

## What it must not do

- **Not make the order a lever**: no `(config …)` field, no CLI option, no `EIN_*` name. `-o/--lattice-order`
  is retired and stays retired.
- **Not read a relation, a type or a module by name** to order anything. The order reads the structure.
- **Not let an order decide an answer**, including under a budget: a run the budget stops says what it
  established and no more, under any order.

## Notes

**Within a group, which member first** (#4) is the value-ordering half of the CSP pair, *least constraining
value*. For a bijection every member is symmetric until a clue breaks the symmetry. The stage measures #4 only
where T3 shows it matters and otherwise leaves it as it is, named here so the omission is deliberate.

**`most-constrained`**, the scoring mode that has returned `0.0` since it was named, is not this stage's any
more: no command reaches it, and [S1f.10.16](s1f.10.16_what_the_walk_left.md) deletes it or says who reads it.

## Record (2026-11-16)

### T1: the inventory, verified before any route changed

Against the code at `e664a561`, the tree as [S1f.10.3](s1f.10.3_the_restricted_join.md#record-2026-11-16) left it.
The eight rows of [the plan's table](#the-inventory-every-order-point-the-routes-have), and five it did not have.

| # | where | as the code has it | the plan's row |
|---|---|---|---|
| 1 | `tree.rs`, `branch_rest` | Among the instances of L5's table the state owes, the licensed ones; the least `(open, i)`, `open` the alternatives `a` for which `KnownDead::knows_extension(state, a)` is false and `i` the instance's place in the table. **Chosen once a state**, at its first step (`St::branch`), and never again: an instance chosen when nothing was known stays chosen. **Since S1f.10.3 `knows_extension` answers for a denial too**, so an alternative the state's own fixpoint denies, or a state under it does, is not counted | holds; *not already known dead* now reads the denials |
| 2 | `oblgen::alternatives`, the table's order | `obligations::tally`'s report order: the obligation plans in the order the rules are written, each plan's matches in the matcher's order. For `(total-owed val is-a)` that is the order of the `(is-a X T)` facts. Read by #1 as its tie-break and by nothing else | holds |
| 3 | `tree.rs`, `tree_q` | a `BTreeSet` of `(closure size, closure)`, its first taken: the smallest **closure**, then the closure's candidates. Not the commitment's size: a state whose closure is larger than its commitment waits with the states of its closure's size | holds, *size* being the closure's |
| 4 | `tree.rs`, `step` | the branch's alternatives not in the state and not tried, ascending index; 1, 2, 4, … at a time up to 1 024, the state staying first in the queue until none is left | holds |
| 5 | `tree.rs`, `climb` | every candidate outside the state, ascending, each extended once | holds |
| 6 | `states.rs`, `compute_resumable` | states by closure size, a layer a size, every extension of each | holds; not this stage's |
| 7 | `region.rs`, the universe | the candidates in rendered-text order, so an index is a place in that order | holds |
| 8 | the saturation agenda | FIFO in written order | holds; not a search order |
| 9 | `tree.rs`, `alive_above` | a state of `R`'s untried extensions, ascending, 1, 2, 4, … until one is alive: `--single`'s last step | not in the plan's table |
| 10 | `tree.rs`, `shrink` | a leaf's commitment, one candidate dropped at a time in ascending order, one pass | not in the table |
| 11 | `tree.rs`, `next_top` and `maximal` | the seeds by `(closure size, closure)`; above a seed the minimal transversals in `transversal::minimal_transversals`' order, the first alive one climbed | not in the table |
| 12 | `tree.rs`, `find`'s second phase | the states of `R` by `(closure size, closure)`, each by its untried extensions ascending, 1, 2, 4, … | not in the table |
| 13 | `known_dead.rs` | the entries under a candidate in the order learned, a death answering before a denial: it decides which denial is counted as spent and no answer | not in the table |

Rows 9 to 13 are over states of `R`, sets a question already holds, or a count; none chooses among owing states,
and the stage leaves them as they are. **Rows 1 and 2 together and row 3 are the order of the tree.**

**E-29, on the two programs' dumps** (`ein solve FILE --single --dump-lattice DIR`, the release binary at
`e664a561`, 2026-11-16; `jq '.nodes | group_by(.commitment | length) | map(…)'` over `nodes.json`). Visited nodes
by the size of the commitment, with the letters committed:

| size | I-SEND, declared D E Y N R O S M | visited | dead | owing | I-SENDW, declared S E N D M O R Y | visited | dead | owing |
|---:|---|---:|---:|---:|---|---:|---:|---:|
| 0 | | 1 | 0 | 1 | | 1 | 0 | 1 |
| 1 | D | 10 | 0 | 10 | S | 10 | 1 | 9 |
| 2 | D E | 100 | 10 | 90 | S E | 90 | 9 | 81 |
| 3 | D E **Y** | 900 | 828 | 72 | S E N | 810 | 162 | 648 |
| 4 | D E Y N | 720 | 216 | 504 | S E N D | 6 480 | 1 944 | 4 536 |
| 5 | D E Y N **R** | 5 040 | 4 750 | 290 | S E N D M | 45 360 | 21 168 | 24 192 |
| 6 | D E Y N R **O** | 2 900 | 2 717 | 183 | S E N D M O | 78 383, **stopped** | 39 191 | 39 192 |
| 7 | D E Y N R O S | 1 830 | 1 208 | 622 | | | | |
| 8 | all eight | 6 220 | 6 219 | 0, and 1 in `R` | | | | |
| 9 | the leaf's extensions (#9) | 26 | 26 | 0 | | | | |
| | | **17 747** | | | | **131 134**, not decided | | |

So the plan's reading holds to the node. On I-SEND a column closes at sizes 3, 5 and 6 (in bold: the letter
that completes one) and the owing states fall from 90 to 72, 504 to 290 and 290 to 183; on I-SENDW the only
deaths through size 6 are a letter's second digit, a digit's second letter and a leading zero, since no column
has its letters until `R`, the seventh, and the owing states grow by a factor of seven to nine a layer until the
budget stops the run. On a fresh state every letter has ten alternatives not known dead, so #1 ties and #2, the
declared order, decides at every state of both runs; #3 then takes each layer whole.

**What decides the cost, then**: which instance (#1 with #2). #3 decides how soon a question that can stop early
reaches a leaf, and on a tree a `--single` exhausts it visits the same nodes in another order. Both are measured,
separately ([T3](#t3-measured)).

**The goldens an order can move, by kind**, before any is touched; the programs are named, from T3's measurement,
[below](#what-moves-named-before-the-route-changed), before the route changes. An order changes only a run that
takes the tree and branches on an instance (`route … the obligation tree`), so every row below is a program
whose `--single`, `-n`, `--minimal` or `--maximal` names L5:

| golden | which part | re-banked by |
|---|---|---|
| [`docs/kernel/inference/read_out_examples.md`](../../../docs/kernel/inference/read_out_examples.md) | every example under a mode whose route is the tree: the `route` row, which gains the order's words on every one of them, and the `work` row and the state's `from {…}` where the order moves the nodes | `python3 utils/read_out_examples.py` |
| `ein-render/tests/golden/corpus_shapes.md5` | for each such program its `read-out[…]`, `region-trace`, `region-lattice` and `cores[…]` lines of the modes, and no `states` line and no describe's line | `EIN_BLESS=1 EIN_CORPUS_SLOW=1 cargo test … -p ein-render --test id_order_invariance` |
| `ein-render/tests/golden/trace_*.md` | a trace of a mode that takes the tree | `EIN_BLESS=1 cargo test … -p ein-render --test golden_trace` |
| `README.md` and `docs/guide/04_solving_the_whole_puzzle.md`, the marked transcript | `ein solve examples/zebra2.ein --single`: the `route` row, and the `work` row and `from {…}` if they move | `EIN_BLESS=1 cargo test … -p ein-cli --test guide_transcripts` |
| `docs/api/rust.md`'s output block and `ein-cli/tests/embedding.rs` | `saturations:` of the same run, if it moves | the test's text edited, the page's block pasted from it |
| `ein-cli/tests/region_modes.rs` | a block that quotes a mode's `route` row | the expected block edited |
| `corpus/corpus.toml`, I-SENDW's entry and `cost_ms`; `ein-corpus`'s slow set | the entry's recorded answer, its runs and its flag | by hand, and `utils/corpus_cost.py` |
| `ein-cli/tests/golden/corpus_exits.txt` | I-SENDW's cells, when its runs change | `EIN_BLESS=1 EIN_CORPUS_SLOW=1 cargo test … -p ein-cli --test corpus_cli` |

**Not goldens, and stale after the step**: `examples/beyond_arith/send_more_money.ein`'s header, *in 17 747
saturations* and its paragraph on the declared order, with the generator's anchor on it; `corpus.toml`'s notes on
the two SENDs; `tests/README.md`'s mention of the program.
