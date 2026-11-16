# S1f.10.5 — The order on the obligation tree

**Phase:** [P1f.10](README.md), track R (the search)
**Status:** ran to its end, 2026-11-16 ([record](#record-2026-11-16)): the order built, I-SENDW decided and the
same work as I-SEND under `--single`, E-29 closed; by the price list's rule the order ships for six questions of
eight, and the minimal line, whole and under `--single`, keeps the written order.
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

### T2: the strategies, as functions

Each is a value of [`ein_infer::order::Order`](../../../ein.rs/crates/ein-infer/src/order.rs): a **choice**, which
licensed instance an owing state branches on; a **queue**, which owing state is branched next; and the
**structure** the choice reads. The tree takes one for the calling thread while a closure runs
(`order::with`), as `known_dead::from_deaths_alone` is taken; no `(config …)` field, no option and no `EIN_*`
name reaches it. Every key is a function of the state, the run's memory and the structure, and the last key is
always total, so no choice falls to a hash map's order or an interner's id.

| key | as built | what it reads | what that costs |
|---|---|---|---|
| **the count**, *fewest alternatives not known dead* | `Choice::Written`, `Text` and the first key of the others before a structure is read | the state and `known_dead`: an alternative a saturation found dead under the state, or one the state denies (L8). With `denials: false` only the deaths | nothing |
| **the last key** | the first the obligation pass reports (`Written`), the last (`Reversed`), the first by the instances' alternatives as rendered text (`Text`), a seeded permutation (`Shuffled`) | the table's order, which is the order the program states its facts in; or the universe's, which is rendered text | nothing |
| **closest to closing a coupling** | `Coupled::unpaid`: over the edges that hold a question of the instance, the fewest questions the state has not paid, the instance's own among them; a question is paid where the state holds one of its members | the edges of the structure, restricted to the questions some licensed instance can pay | the structure's, below |
| **learned weight** | `Choice::Learned`: the count over one more than the deaths the instance was blamed for; a death under a branch is blamed on the instance branched on (`Blame::Branch`) or on it and every licensed instance the dying state had paid (`Blame::Path`) | the run's own deaths | nothing. A blame by the death's core, which is what *dom/wdeg* weighs a constraint by, needs a core at every death, a search no route takes unless a surface prints it (K-7); it was not run |

**The structure a coupled choice reads, by what it costs.**

| source | as built | costs | the order measured with it |
|---|---|---|---|
| (a) nothing | `Structure::None` | nothing | `today`, `deaths-count`, `reversed`, `text`, the learned ones |
| (b) **the whole node alone** | `Structure::Node`: [`structure::NodeCoupling`](../../../ein.rs/crates/ein-infer/src/structure/coupling/node.rs), **an entry added to the owner by this stage**, `pub(crate) node_coupling_in` over an open `Space` | **one saturation**, its facts those of every candidate committed and saturated to quiescence; charged to the run and counted in its `work` | `node@k`, `node-first@k`, `one-step@k`, `open-first/k`, `coupled/k`, `written+coupled/k` |
| (c) the owed groups, from L5's table | inside (b): its questions **are** the alternative sets of the instances L5 licenses, read off the table the tree already holds, and every other candidate alone | nothing beyond the table | the same |
| (d) the full structure | `Structure::Given`: [`structure::coupling`](../../../ein.rs/crates/ein-infer/src/structure/coupling.rs)'s edges, read before the run by the harness | root's pair layer, the small KB's and the node: added to every cell | `full` |

**The whole node alone** is the price list's *not built*. What it is, in the owner's words
(`structure/coupling/node.rs`): the node [S1f.10.11](s1f.10.11_coupling.md#record-2026-11-15)'s reading
takes, with **every** candidate of `H` committed, since which are alive alone is a layer's to say; the labels
settled as the full reading settles them; **the questions the debts' and not the cover's groups**; and no pair
layer to drop the ways no alive state has. Its blind spots, stated in the module: a conflict of exactly two
candidates is no edge (with no cover *different letters, different digits* cannot be told from a pair the
puzzle's statements make, so pairs are counted, join the components and make no edge); a way no alive state has
is read, so an edge may be coarser than the full reading's; a candidate dead alone is committed; a group nothing
owes is no question.

**Held to the full coupling** by `ein-infer/tests/node_coupling.rs` (`cargo test --manifest-path
ein.rs/Cargo.toml --profile gate -p ein-infer --test node_coupling -- --nocapture`, 2026-11-16):

- I-SEND: 8 questions, the letters; **the edges are `{D, E}`, `{D, E, Y}`, `{D, E, N, R}`, `{D, E, N, O, R}` and
  `{D, E, M, N, O, R, S}`**, the full reading's five; one application back `{D, E}` and `{D, E, Y}`; 640 pairs;
  one saturation, 166 facts, 2 932 applications (the full reading's node is 162 facts, of the 78 candidates alive
  alone).
- **I-SEND and I-SENDW are one reading, field for field.**
- I-L02: no obligation, three questions, one edge of the three.
- On the 136 corpus programs that search and that both read whole: 486 edges from the node alone, **479 inside
  one edge of the full reading, 7 inside one of its components and no edge, none across two components**; and of
  the full reading's 57 edges of three questions or more that its node gave, every one meets some edge read here
  in each of its questions. The seven: `branching/15_lookahead_two_step_on.ein`, `16_lookahead_two_step_off.ein`,
  `ein-bugs/fork-delta-order.ein`, `ein-bugs/wrong-answers/forall-core-false.ein`, `oblig-derived-domain.ein`,
  `oblig-fresh-name.ein` and `tests/region/10_denied_extensions.ein`, one edge each, a way the full reading
  drops by the pair layer.
- The reading is the same under a permuted id space, place for place, and under every import form, as sets of
  texts, on every corpus program that searches (`structure_invariance.rs`, which gained it).

**When a structure is read** is part of a strategy, since the node is not free: `node@k` reads it at the first
choice the count leaves tied once the run has saturated `k` nodes, and starts the tree again from root, every
choice made afresh; `…/k` reads it under the same condition **within one `k`-th of the facts the run has
derived**, the node stopped where it needs more, what it derived counted, and tried again at four times the
work. **Starting again** keeps every node computed, every set known dead and every leaf found: a state the new
tree reaches again is read from memory.

### T3: measured

No route changed: the tree's order was `Order::WRITTEN` throughout, the strategies taken through `order::with`.
The harness is `price_list --orders` ([`examples/shared/orders.rs`](../../../ein.rs/crates/ein-infer/examples/shared/orders.rs)),
the price list's example extended, a file an order and a line a program:

```sh
cargo build --manifest-path ein.rs/Cargo.toml --release -p ein-infer --example price_list
utils/bench_env.sh --require-ac --core 0 ein.rs/target/release/examples/price_list --orders \
    --out build/s5/m4 --only today,deaths-count,reversed,text     # and eight more groups, a P-core each
ein.rs/target/release/examples/price_list --orders-report --out build/s5/m4 > build/s5/m4/report.md
```

2026-11-16, the release example, on AC, nine processes each pinned to one core, the machine shared with two
other stages (load average 10 to 36): **saturations and facts are the numbers and no wall is stated.** 190
programs that search, the 188 of the price list, S1f.10.3's fixture and I-SENDW; four questions; 33 orders. 36
programs are answered by a tree under `--single` today. Artefacts under the worktree's untracked
`build/s5/m4/`: `orders/<order>.jsonl`, `report.md` with every table and every loser, `env/` with the
fingerprints.

**Every answer is one.** Over every order and every cell no budget stopped either way: **0** cells with another
verdict, another `exhausted`, another maximal line or another count of the minimal line than `today`.

**The rule**, as [S1f.10.13](s1f.10.13_the_price_list.md#the-rule-for-go-written-before-the-table-is-taken) wrote
it: *go* where the total is lower with the structure's cost in it and no program's cell is higher by more than
one tenth, in saturations and in facts. Read as [S1f.10.3](s1f.10.3_the_restricted_join.md#defaults-taken)'s
default 2 has it: a cell a budget stops under both orders spent that budget either way, so the total is over
the cells **not stopped both ways**, a cell an order stops where `today` finishes sinks it, and a cell it
finishes where `today` stops counts with what `today` spent there. A cell is *saturations · facts*; *lower* and
*higher* count the cells finished both ways.

**`--single`.**

| order | not stopped both ways | `today` on those cells | finished both ways | lower | higher | by more than a tenth | stopped where `today` finishes | finishes where `today` stops | by the rule |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---|
| `today` | 19 581 · 261 430 | 19 581 · 261 430 | 19 581 · 261 430 | 0 | 0 | 0 | 0 | 0 | — |
| `deaths-count` | 19 557 · 259 676 | 19 581 · 261 430 | 19 557 · 259 676 | 4 | 0 | 0 | 0 | 0 | **go** |
| `reversed` | 174 435 · 1 103 694 | 19 581 · 261 430 | 3 911 · 55 136 | 7 | 8 | 8 | 1 | 0 | no |
| `text` | 106 006 · 1 086 416 | 19 581 · 261 430 | 1 828 · 37 845 | 4 | 1 | 0 | 1 | 0 | no |
| `node@0` | 37 349 · 503 471 | 150 715 · 1 309 989 | 19 601 · 280 381 | 2 | 24 | 20 | 0 | 1 | no |
| `node@10` | 37 330 · 502 350 | 150 715 · 1 309 989 | 19 582 · 279 260 | 2 | 7 | 4 | 0 | 1 | no |
| `node@32` | 37 329 · 502 169 | 150 715 · 1 309 989 | 19 581 · 279 079 | 2 | 6 | 3 | 0 | 1 | no |
| `node@100` | 37 325 · 501 645 | 150 715 · 1 309 989 | 19 577 · 278 555 | 4 | 3 | 1 | 0 | 1 | no |
| `node-first@0` | 37 317 · 500 927 | 150 715 · 1 309 989 | 19 569 · 277 837 | 4 | 22 | 20 | 0 | 1 | no |
| `node-first@10` | 37 316 · 500 288 | 150 715 · 1 309 989 | 19 568 · 277 198 | 4 | 5 | 4 | 0 | 1 | no |
| `node-first@32` | 37 351 · 502 627 | 150 715 · 1 309 989 | 19 603 · 279 537 | 2 | 6 | 5 | 0 | 1 | no |
| `open-first/20` | 86 564 · 1 010 581 | 150 715 · 1 309 989 | 44 196 · 524 213 | 4 | 2 | 1 | 0 | 1 | no |
| `coupled/20` | 37 888 · 481 451 | 150 715 · 1 309 989 | 19 858 · 259 648 | 4 | 2 | 0 | 0 | 1 | **go** |
| `coupled/10` | 37 888 · 481 517 | 150 715 · 1 309 989 | 19 858 · 259 705 | 4 | 2 | 0 | 0 | 1 | **go** |
| `coupled/40` | 39 326 · 486 347 | 150 715 · 1 309 989 | 20 577 · 262 096 | 4 | 2 | 0 | 0 | 1 | **go** |
| `written+coupled/20` | 39 015 · 490 957 | 150 715 · 1 309 989 | 19 582 · 261 596 | 0 | 1 | 0 | 0 | 1 | **go** |
| `written+coupled/10` | 38 205 · 485 874 | 150 715 · 1 309 989 | 19 582 · 261 633 | 0 | 4 | 0 | 0 | 1 | **go** |
| `written+coupled/40` | 40 665 · 501 362 | 150 715 · 1 309 989 | 19 582 · 261 596 | 0 | 1 | 0 | 0 | 1 | **go** |
| `coupled/20/deepest` | 38 586 · 499 174 | 150 715 · 1 309 989 | 20 094 · 267 011 | 13 | 7 | 4 | 0 | 1 | no |
| `written+coupled/20/deepest` | 37 219 · 467 716 | 150 715 · 1 309 989 | 19 003 · 238 388 | 12 | 5 | 4 | 0 | 1 | no |
| `one-step@32` | 90 181 · 1 104 278 | 19 581 · 261 430 | 1 839 · 55 693 | 2 | 5 | 3 | 1 | 0 | no |
| `full` | 115 915 · 2 298 820 | 150 715 · 1 309 989 | 91 842 · 2 061 797 | 0 | 160 | 160 | 0 | 1 | no |
| `learned-branch` | 105 982 · 1 084 940 | 19 581 · 261 430 | 1 804 · 36 369 | 4 | 1 | 0 | 1 | 0 | no |
| `learned-path` | 105 992 · 1 085 210 | 19 581 · 261 430 | 1 814 · 36 639 | 4 | 1 | 0 | 1 | 0 | no |
| `learned-branch/restarts` | 121 189 · 1 067 046 | 19 581 · 261 430 | 1 724 · 18 471 | 7 | 1 | 1 | 1 | 0 | no |
| `today/deepest` | 19 002 · 238 146 | 19 581 · 261 430 | 19 002 · 238 146 | 12 | 4 | 4 | 0 | 0 | no |
| `text/deepest` | 87 531 · 1 081 974 | 19 581 · 261 430 | 1 602 · 33 413 | 14 | 5 | 4 | 1 | 0 | no |
| `node@0/deepest` | 37 123 · 499 039 | 150 715 · 1 309 989 | 19 375 · 275 949 | 9 | 19 | 18 | 0 | 1 | no |
| `node@32/deepest` | 37 700 · 503 756 | 150 715 · 1 309 989 | 19 652 · 277 426 | 13 | 7 | 6 | 0 | 1 | no |
| `learned-branch/deepest` | 128 562 · 1 623 375 | 150 715 · 1 309 989 | 65 083 · 828 395 | 16 | 7 | 6 | 0 | 1 | no |
| `learned-path/deepest` | 87 531 · 1 081 974 | 19 581 · 261 430 | 1 602 · 33 413 | 14 | 5 | 4 | 1 | 0 | no |
| `learned-branch/deepest/restarts` | 87 569 · 1 075 399 | 19 581 · 261 430 | 1 501 · 26 825 | 16 | 6 | 5 | 1 | 0 | no |
| `learned-path/deepest/restarts` | 87 531 · 1 081 974 | 19 581 · 261 430 | 1 602 · 33 413 | 14 | 5 | 4 | 1 | 0 | no |

**`-n 1`.**

| order | not stopped both ways | `today` on those cells | finished both ways | lower | higher | by more than a tenth | stopped where `today` finishes | finishes where `today` stops | by the rule |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---|
| `today` | 17 208 · 224 356 | 17 208 · 224 356 | 17 208 · 224 356 | 0 | 0 | 0 | 0 | 0 | — |
| `deaths-count` | 17 186 · 222 964 | 17 208 · 224 356 | 17 186 · 222 964 | 2 | 2 | 2 | 0 | 0 | no |
| `reversed` | 173 013 · 1 088 588 | 17 208 · 224 356 | 2 489 · 40 030 | 6 | 11 | 8 | 1 | 0 | no |
| `text` | 105 512 · 1 078 969 | 17 208 · 224 356 | 1 334 · 30 398 | 2 | 2 | 2 | 1 | 0 | no |
| `node@0` | 32 843 · 431 672 | 148 342 · 1 272 915 | 17 101 · 240 758 | 3 | 23 | 22 | 0 | 1 | no |
| `node@10` | 32 824 · 430 551 | 148 342 · 1 272 915 | 17 082 · 239 637 | 3 | 6 | 6 | 0 | 1 | no |
| `node@32` | 32 821 · 429 926 | 148 342 · 1 272 915 | 17 079 · 239 012 | 3 | 5 | 5 | 0 | 1 | no |
| `node@100` | 32 819 · 429 846 | 148 342 · 1 272 915 | 17 077 · 238 932 | 3 | 3 | 3 | 0 | 1 | no |
| `node-first@0` | 32 807 · 429 548 | 148 342 · 1 272 915 | 17 065 · 238 634 | 3 | 23 | 22 | 0 | 1 | no |
| `node-first@10` | 32 806 · 428 909 | 148 342 · 1 272 915 | 17 064 · 237 995 | 3 | 6 | 6 | 0 | 1 | no |
| `node-first@32` | 32 821 · 429 926 | 148 342 · 1 272 915 | 17 079 · 239 012 | 3 | 5 | 5 | 0 | 1 | no |
| `open-first/20` | 82 058 · 938 782 | 148 342 · 1 272 915 | 41 696 · 484 590 | 2 | 3 | 3 | 0 | 1 | no |
| `coupled/20` | 33 382 · 409 652 | 148 342 · 1 272 915 | 17 358 · 220 025 | 2 | 3 | 2 | 0 | 1 | no |
| `coupled/10` | 33 382 · 409 718 | 148 342 · 1 272 915 | 17 358 · 220 082 | 2 | 4 | 2 | 0 | 1 | no |
| `coupled/40` | 34 820 · 414 548 | 148 342 · 1 272 915 | 18 077 · 222 473 | 2 | 3 | 2 | 0 | 1 | no |
| `written+coupled/20` | 34 751 · 423 553 | 148 342 · 1 272 915 | 17 209 · 224 522 | 0 | 1 | 0 | 0 | 1 | **go** |
| `written+coupled/10` | 33 941 · 418 470 | 148 342 · 1 272 915 | 17 209 · 224 559 | 0 | 4 | 0 | 0 | 1 | **go** |
| `written+coupled/40` | 36 401 · 433 958 | 148 342 · 1 272 915 | 17 209 · 224 522 | 0 | 1 | 0 | 0 | 1 | **go** |
| `coupled/20/deepest` | 27 864 · 364 548 | 148 342 · 1 272 915 | 14 487 · 197 851 | 19 | 4 | 3 | 0 | 1 | no |
| `written+coupled/20/deepest` | 23 116 · 289 294 | 148 342 · 1 272 915 | 13 297 · 165 999 | 20 | 3 | 3 | 0 | 1 | no |
| `one-step@32` | 89 679 · 1 096 683 | 17 208 · 224 356 | 1 337 · 48 098 | 2 | 5 | 5 | 1 | 0 | no |
| `full` | 111 395 · 2 226 479 | 148 342 · 1 272 915 | 89 328 · 2 021 632 | 0 | 160 | 160 | 0 | 1 | no |
| `learned-branch` | 105 506 · 1 079 087 | 17 208 · 224 356 | 1 328 · 30 516 | 2 | 2 | 2 | 1 | 0 | no |
| `learned-path` | 105 504 · 1 078 469 | 17 208 · 224 356 | 1 326 · 29 898 | 2 | 2 | 2 | 1 | 0 | no |
| `learned-branch/restarts` | 120 700 · 1 060 077 | 17 208 · 224 356 | 1 235 · 11 502 | 4 | 4 | 3 | 1 | 0 | no |
| `today/deepest` | 13 296 · 165 757 | 17 208 · 224 356 | 13 296 · 165 757 | 20 | 3 | 3 | 0 | 0 | no |
| `text/deepest` | 87 039 · 1 078 280 | 17 208 · 224 356 | 1 110 · 29 719 | 19 | 3 | 3 | 1 | 0 | no |
| `node@0/deepest` | 26 401 · 364 413 | 148 342 · 1 272 915 | 13 768 · 206 789 | 8 | 20 | 19 | 0 | 1 | no |
| `node@32/deepest` | 26 978 · 369 130 | 148 342 · 1 272 915 | 14 045 · 208 266 | 18 | 5 | 4 | 0 | 1 | no |
| `learned-branch/deepest` | 104 816 · 1 327 070 | 148 342 · 1 272 915 | 52 964 · 678 399 | 21 | 5 | 5 | 0 | 1 | no |
| `learned-path/deepest` | 87 039 · 1 078 280 | 17 208 · 224 356 | 1 110 · 29 719 | 19 | 3 | 3 | 1 | 0 | no |
| `learned-branch/deepest/restarts` | 60 644 · 736 408 | 148 342 · 1 272 915 | 30 684 · 373 596 | 21 | 5 | 5 | 0 | 1 | no |
| `learned-path/deepest/restarts` | 87 039 · 1 078 280 | 17 208 · 224 356 | 1 110 · 29 719 | 19 | 3 | 3 | 1 | 0 | no |

**`--minimal`.**

| order | not stopped both ways | `today` on those cells | finished both ways | lower | higher | by more than a tenth | stopped where `today` finishes | finishes where `today` stops | by the rule |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---|
| `today` | 4 601 · 170 994 | 4 601 · 170 994 | 4 601 · 170 994 | 0 | 0 | 0 | 0 | 0 | — |
| `deaths-count` | 4 599 · 170 630 | 4 601 · 170 994 | 4 599 · 170 630 | 2 | 0 | 0 | 0 | 0 | **go** |
| `reversed` | 6 476 · 182 892 | 4 601 · 170 994 | 6 476 · 182 892 | 3 | 5 | 5 | 0 | 0 | no |
| `text` | 4 599 · 170 630 | 4 601 · 170 994 | 4 599 · 170 630 | 2 | 0 | 0 | 0 | 0 | **go** |
| `node@0` | 4 622 · 189 632 | 4 601 · 170 994 | 4 622 · 189 632 | 0 | 21 | 16 | 0 | 0 | no |
| `node@10` | 4 604 · 188 904 | 4 601 · 170 994 | 4 604 · 188 904 | 2 | 5 | 2 | 0 | 0 | no |
| `node@32` | 4 602 · 188 330 | 4 601 · 170 994 | 4 602 · 188 330 | 2 | 3 | 1 | 0 | 0 | no |
| `node@100` | 4 600 · 188 250 | 4 601 · 170 994 | 4 600 · 188 250 | 2 | 1 | 1 | 0 | 0 | no |
| `node-first@0` | 4 622 · 189 632 | 4 601 · 170 994 | 4 622 · 189 632 | 0 | 21 | 16 | 0 | 0 | no |
| `node-first@10` | 4 604 · 188 904 | 4 601 · 170 994 | 4 604 · 188 904 | 2 | 5 | 2 | 0 | 0 | no |
| `node-first@32` | 4 602 · 188 330 | 4 601 · 170 994 | 4 602 · 188 330 | 2 | 3 | 1 | 0 | 0 | no |
| `open-first/20` | 4 599 · 170 630 | 4 601 · 170 994 | 4 599 · 170 630 | 2 | 0 | 0 | 0 | 0 | **go** |
| `coupled/20` | 4 599 · 170 630 | 4 601 · 170 994 | 4 599 · 170 630 | 2 | 0 | 0 | 0 | 0 | **go** |
| `coupled/10` | 4 599 · 170 742 | 4 601 · 170 994 | 4 599 · 170 742 | 2 | 2 | 0 | 0 | 0 | **go** |
| `coupled/40` | 4 599 · 170 630 | 4 601 · 170 994 | 4 599 · 170 630 | 2 | 0 | 0 | 0 | 0 | **go** |
| `written+coupled/20` | 4 601 · 170 994 | 4 601 · 170 994 | 4 601 · 170 994 | 0 | 0 | 0 | 0 | 0 | no |
| `written+coupled/10` | 4 601 · 171 152 | 4 601 · 170 994 | 4 601 · 171 152 | 0 | 4 | 0 | 0 | 0 | no |
| `written+coupled/40` | 4 601 · 170 994 | 4 601 · 170 994 | 4 601 · 170 994 | 0 | 0 | 0 | 0 | 0 | no |
| `coupled/20/deepest` | 4 932 · 173 701 | 4 601 · 170 994 | 4 932 · 173 701 | 2 | 8 | 4 | 0 | 0 | no |
| `written+coupled/20/deepest` | 4 927 · 174 394 | 4 601 · 170 994 | 4 927 · 174 394 | 0 | 7 | 4 | 0 | 0 | no |
| `one-step@32` | 4 602 · 188 330 | 4 601 · 170 994 | 4 602 · 188 330 | 2 | 3 | 1 | 0 | 0 | no |
| `full` | 52 133 · 822 371 | 4 601 · 170 994 | 52 133 · 822 371 | 0 | 157 | 157 | 0 | 0 | no |
| `learned-branch` | 4 599 · 170 630 | 4 601 · 170 994 | 4 599 · 170 630 | 2 | 0 | 0 | 0 | 0 | **go** |
| `learned-path` | 4 599 · 170 630 | 4 601 · 170 994 | 4 599 · 170 630 | 2 | 0 | 0 | 0 | 0 | **go** |
| `learned-branch/restarts` | 4 543 · 170 287 | 4 601 · 170 994 | 4 543 · 170 287 | 4 | 1 | 0 | 0 | 0 | **go** |
| `today/deepest` | 4 927 · 172 579 | 4 601 · 170 994 | 4 927 · 172 579 | 0 | 5 | 4 | 0 | 0 | no |
| `text/deepest` | 4 932 · 172 264 | 4 601 · 170 994 | 4 932 · 172 264 | 2 | 6 | 4 | 0 | 0 | no |
| `node@0/deepest` | 4 955 · 191 266 | 4 601 · 170 994 | 4 955 · 191 266 | 0 | 23 | 20 | 0 | 0 | no |
| `node@32/deepest` | 4 935 · 189 964 | 4 601 · 170 994 | 4 935 · 189 964 | 2 | 7 | 6 | 0 | 0 | no |
| `learned-branch/deepest` | 4 936 · 173 074 | 4 601 · 170 994 | 4 936 · 173 074 | 2 | 8 | 5 | 0 | 0 | no |
| `learned-path/deepest` | 4 932 · 172 264 | 4 601 · 170 994 | 4 932 · 172 264 | 2 | 6 | 4 | 0 | 0 | no |
| `learned-branch/deepest/restarts` | 4 897 · 172 842 | 4 601 · 170 994 | 4 897 · 172 842 | 3 | 7 | 6 | 0 | 0 | no |
| `learned-path/deepest/restarts` | 4 910 · 172 175 | 4 601 · 170 994 | 4 910 · 172 175 | 2 | 6 | 4 | 0 | 0 | no |

**`--maximal`.**

| order | not stopped both ways | `today` on those cells | finished both ways | lower | higher | by more than a tenth | stopped where `today` finishes | finishes where `today` stops | by the rule |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---|
| `today` | 23 016 · 352 388 | 23 016 · 352 388 | 23 016 · 352 388 | 0 | 0 | 0 | 0 | 0 | — |
| `deaths-count` | 22 996 · 350 772 | 23 016 · 352 388 | 22 996 · 350 772 | 4 | 0 | 0 | 0 | 0 | **go** |
| `reversed` | 177 854 · 1 198 815 | 23 016 · 352 388 | 7 330 · 150 257 | 8 | 9 | 7 | 1 | 0 | no |
| `text` | 109 443 · 1 178 673 | 23 016 · 352 388 | 5 265 · 130 102 | 3 | 3 | 0 | 1 | 0 | no |
| `node@0` | 40 782 · 595 712 | 154 150 · 1 400 947 | 23 034 · 372 622 | 2 | 20 | 14 | 0 | 1 | no |
| `node@10` | 40 767 · 594 607 | 154 150 · 1 400 947 | 23 019 · 371 517 | 3 | 7 | 3 | 0 | 1 | no |
| `node@32` | 40 766 · 594 426 | 154 150 · 1 400 947 | 23 018 · 371 336 | 3 | 6 | 2 | 0 | 1 | no |
| `node@100` | 40 764 · 594 346 | 154 150 · 1 400 947 | 23 016 · 371 256 | 3 | 5 | 1 | 0 | 1 | no |
| `node-first@0` | 40 774 · 594 896 | 154 150 · 1 400 947 | 23 026 · 371 806 | 2 | 20 | 14 | 0 | 1 | no |
| `node-first@10` | 40 777 · 594 273 | 154 150 · 1 400 947 | 23 029 · 371 183 | 3 | 7 | 3 | 0 | 1 | no |
| `node-first@32` | 40 812 · 596 612 | 154 150 · 1 400 947 | 23 064 · 373 522 | 3 | 6 | 4 | 0 | 1 | no |
| `open-first/20` | 90 001 · 1 102 974 | 154 150 · 1 400 947 | 47 633 · 616 606 | 3 | 4 | 1 | 0 | 1 | no |
| `coupled/20` | 41 325 · 573 844 | 154 150 · 1 400 947 | 23 295 · 352 041 | 3 | 4 | 0 | 0 | 1 | **go** |
| `coupled/10` | 41 409 · 579 788 | 154 150 · 1 400 947 | 23 379 · 357 976 | 3 | 4 | 2 | 0 | 1 | no |
| `coupled/40` | 42 847 · 584 618 | 154 150 · 1 400 947 | 24 098 · 360 367 | 3 | 4 | 2 | 0 | 1 | no |
| `written+coupled/20` | 42 457 · 582 229 | 154 150 · 1 400 947 | 23 017 · 352 742 | 0 | 3 | 0 | 0 | 1 | **go** |
| `written+coupled/10` | 41 731 · 581 952 | 154 150 · 1 400 947 | 23 101 · 357 585 | 0 | 4 | 2 | 0 | 1 | no |
| `written+coupled/40` | 44 107 · 592 446 | 154 150 · 1 400 947 | 23 017 · 352 554 | 0 | 1 | 0 | 0 | 1 | **go** |
| `coupled/20/deepest` | 42 629 · 602 044 | 154 150 · 1 400 947 | 24 115 · 369 479 | 7 | 10 | 7 | 0 | 1 | no |
| `written+coupled/20/deepest` | 41 628 · 592 002 | 154 150 · 1 400 947 | 23 388 · 362 237 | 4 | 9 | 6 | 0 | 1 | no |
| `one-step@32` | 93 628 · 1 197 547 | 23 016 · 352 388 | 5 286 · 148 962 | 3 | 5 | 4 | 1 | 0 | no |
| `full` | 119 352 · 2 391 077 | 154 150 · 1 400 947 | 95 279 · 2 154 054 | 0 | 160 | 160 | 0 | 1 | no |
| `learned-branch` | 109 429 · 1 177 477 | 23 016 · 352 388 | 5 251 · 128 906 | 5 | 1 | 0 | 1 | 0 | no |
| `learned-path` | 109 473 · 1 184 153 | 23 016 · 352 388 | 5 295 · 135 582 | 3 | 3 | 2 | 1 | 0 | no |
| `learned-branch/restarts` | 125 124 · 1 198 443 | 23 016 · 352 388 | 5 659 · 149 868 | 5 | 4 | 4 | 1 | 0 | no |
| `today/deepest` | 23 307 · 354 132 | 23 016 · 352 388 | 23 307 · 354 132 | 4 | 6 | 4 | 0 | 0 | no |
| `text/deepest` | 91 474 · 1 180 140 | 23 016 · 352 388 | 5 545 · 131 579 | 7 | 8 | 5 | 1 | 0 | no |
| `node@0/deepest` | 41 106 · 597 993 | 154 150 · 1 400 947 | 23 336 · 374 501 | 6 | 22 | 17 | 0 | 1 | no |
| `node@32/deepest` | 41 690 · 603 187 | 154 150 · 1 400 947 | 23 620 · 376 455 | 7 | 10 | 6 | 0 | 1 | no |
| `learned-branch/deepest` | 132 671 · 1 743 670 | 154 150 · 1 400 947 | 69 167 · 948 233 | 11 | 10 | 10 | 0 | 1 | no |
| `learned-path/deepest` | 91 478 · 1 180 480 | 23 016 · 352 388 | 5 549 · 131 919 | 7 | 8 | 5 | 1 | 0 | no |
| `learned-branch/deepest/restarts` | 92 082 · 1 212 035 | 23 016 · 352 388 | 5 991 · 163 494 | 10 | 10 | 8 | 1 | 0 | no |
| `learned-path/deepest/restarts` | 91 502 · 1 182 636 | 23 016 · 352 388 | 5 573 · 134 075 | 7 | 8 | 7 | 1 | 0 | no |

**What sank each *no*, and what is left.**

- **`text`, the canonical order with no structure, does not decide either SEND**: 104 178 saturations and
  stopped, on both. Alphabetically the letters are D E M N O R S Y, and no column has its letters before the
  sixth. So today's order on I-SEND was the declared order standing in for the structure, which is E-29, and a
  canonical last key alone makes the corpus's one cryptarithm of eight letters undecided.
- **The whole node read at once**, `node@0` and `node-first@0`: decides both SENDs at 17 748 · 223 090, one
  node and 166 facts over today's I-SEND, and is sunk by **the node on a small cell**: 20 programs higher by
  more than a tenth under `--single`, 22 under `-n 1`, each by the one saturation or by the node's facts
  (`zebra.ein` 9 · 1 900 → 10 · 2 293).
- **Read after `k` saturations**, `node@10`, `@32`, `@100`: the saturation is then within the tenth, and what
  is left is **the node's facts**, which no count of saturations bounds:
  `beyond_arith/equation_two_unknowns.ein`, 524 · 24 571 → 525 · 42 191, whose whole node is 17 620 facts, every
  pair of digits through the calculator; `cryptarithm.ein` 119 · 691 → 120 · 785;
  `lattice/07_two_pairs_two_tables.ein` 44 · 137 → 45 · 154.
- **Read within a share of the facts derived, every choice then made afresh**, `open-first/20`: no small cell
  pays, and **I-SEND is 42 368**. The count *not known dead* is read off the run's memory, and after a first
  tree that memory holds the deaths of whichever instances that tree tried: an instance already tried at a
  state has alternatives known dead there and an untried one has none, so the count sends the second tree back
  down the first one's order. The same key at a fresh root (`node@0`) has no such memory, which is why it
  measured well.
- **`coupled/k`**: the coupling first and the count second once the edges are read, and **each state that had
  chosen choosing again by the counts it had when it first chose**, so the edges move a choice and the memory
  does not; the tree starts again only where one falls otherwise. I-SEND and I-SENDW are both 18 030 · 221 803:
  a first tree under text, D and E and then M, 600 nodes under M before the node was within a twentieth; the
  node, read at saturation 712; and the tree by the couplings, 318 nodes smaller than the 17 747 by column,
  since the first tree's deaths of M's digits are known to it
  ([§ The order built](#the-order-built) has the dump).
- **`written+coupled/k`**, the same with the first reported among equals: a cell that never reads the node is
  today's to the node, I-SEND is today's and the node, 17 748, and **I-SENDW is 19 433**, after a first tree by
  S, E and N whose node is read at saturation 1 872.
- **The edges one application back**, `one-step@32`: both SENDs stopped. `{D, E}` and `{D, E, Y}` order three
  letters and leave five to text.
- **The full structure**, `full`: decides both SENDs at 24 073, the same order as the node alone and 6 325
  saturations of layers before it; 160 programs higher by more than a tenth, by the layers, whatever the order
  does. A count *live by root's excluded pairs*, the plan's first strategy, reads the same layers and was not
  run as an order of its own: what it could choose is not what sinks it.
- **The largest state first**, every `…/deepest`: `-n 1` over the cells finished both ways falls from 17 208 to
  13 296 under today's choice and 20 cells are lower, I-Z2M `--single` is 10 where it is 40; and three to seven
  cells are higher by more than a tenth under every question: `zebra.ein -n 1` 5 · 328 → 7 · 1 114,
  `ein-bugs/wrong-answers/oblig-fresh-name-two.ein --minimal` 39 → 150,
  `lattice/06_friends_tables_everyone_sits.ein --maximal` 213 → 273. *No* for every question, by the rule;
  the gain under `-n 1` is real and is not taken.

**The instances** (`report.md` § The instances), `--single` and `-n 1`; a cell *saturations · facts*.

| `--single` | I-Z1 | I-Z2 | I-Z2M | I-B06 | I-L02 | I-SEND | I-SENDW |
|---|---:|---:|---:|---:|---:|---:|---:|
| `today` | 9 · 1 900 | 9 · 892 | 40 · 2 688 | 8 · 34 | 2 · 1 | 17 747 · 222 924 | 131 134 · 1 048 559, stopped |
| `deaths-count` | 9 · 1 900 | 7 · 666 | 30 · 2 037 | 8 · 34 | 2 · 1 | 17 747 · 222 924 | 131 134 · 1 048 559, stopped |
| `text` | 9 · 1 900 | 7 · 666 | 39 · 2 568 | 8 · 34 | 2 · 1 | 104 178 · 1 048 571, stopped | 104 178 · 1 048 571, stopped |
| `node@0` | 10 · 2 293 | 8 · 848 | 40 · 2 790 | 8 · 34 | 2 · 1 | 17 748 · 223 090 | 17 748 · 223 090 |
| `node-first@0` | 10 · 2 293 | 8 · 848 | 24 · 1 518 | 8 · 34 | 2 · 1 | 17 748 · 223 090 | 17 748 · 223 090 |
| `open-first/20` | 9 · 1 900 | 7 · 666 | 39 · 2 568 | 8 · 34 | 2 · 1 | 42 368 · 486 368 | 42 368 · 486 368 |
| **`coupled/20`** | 9 · 1 900 | 7 · 666 | 39 · 2 568 | 8 · 34 | 2 · 1 | **18 030 · 221 803** | **18 030 · 221 803** |
| `written+coupled/20` | 9 · 1 900 | 9 · 892 | 40 · 2 688 | 8 · 34 | 2 · 1 | 17 748 · 223 090 | 19 433 · 229 361 |
| `one-step@32` | 9 · 1 900 | 7 · 666 | 43 · 2 642 | 8 · 34 | 2 · 1 | 88 342 · 1 048 585, stopped | 88 342 · 1 048 585, stopped |
| `full` | 2 175 · 250 144 | 2 070 · 101 319 | 9 243 · 570 206 | 8 900 · 24 984 | 17 · 23 | 24 073 · 237 023 | 24 073 · 237 023 |
| `learned-branch` | 9 · 1 900 | 7 · 666 | 27 · 1 830 | 8 · 34 | 2 · 1 | 104 178 · 1 048 571, stopped | 104 178 · 1 048 571, stopped |
| `learned-branch/deepest` | 9 · 1 900 | 7 · 666 | 8 · 572 | 8 · 34 | 2 · 1 | 63 479 · 794 980 | 63 479 · 794 980 |
| `learned-branch/deepest/restarts` | 9 · 1 900 | 7 · 666 | 8 · 572 | 8 · 34 | 2 · 1 | 86 068 · 1 048 574, stopped | 86 068 · 1 048 574, stopped |
| `today/deepest` | 9 · 1 900 | 9 · 892 | 10 · 690 | 8 · 34 | 2 · 1 | 17 747 · 222 924 | 76 619 · 1 048 564, stopped |
| `coupled/20/deepest` | 9 · 1 900 | 7 · 666 | 8 · 572 | 8 · 34 | 2 · 1 | 18 492 · 232 163 | 18 492 · 232 163 |

| `-n 1` | I-Z1 | I-Z2 | I-Z2M | I-SEND | I-SENDW |
|---|---:|---:|---:|---:|---:|
| `today` | 5 · 328 | 5 · 164 | 35 · 2 122 | 15 854 · 192 556 | 131 134 · 1 048 559, stopped |
| `deaths-count` | 5 · 328 | 5 · 302 | 24 · 1 288 | 15 854 · 192 556 | 131 134 · 1 048 559, stopped |
| `text` | 5 · 328 | 5 · 302 | 25 · 1 283 | 104 178 · 1 048 571, stopped | 104 178 · 1 048 571, stopped |
| `coupled/20` | 5 · 328 | 5 · 302 | 25 · 1 283 | 16 024 · 189 627 | 16 024 · 189 627 |
| **`written+coupled/20`** | 5 · 328 | 5 · 164 | 35 · 2 122 | **15 855 · 192 722** | **17 542 · 199 031** |
| `learned-branch/deepest` | 7 · 1 114 | 7 · 666 | 7 · 476 | 51 852 · 648 671 | 51 852 · 648 671 |
| `learned-branch/deepest/restarts` | 7 · 1 114 | 7 · 666 | 7 · 476 | 29 960 · 362 812 | 29 960 · 362 812 |
| `today/deepest` | 7 · 1 114 | 7 · 528 | 9 · 594 | 12 662 · 157 921 | 76 619 · 1 048 564, stopped |
| `coupled/20/deepest` | 7 · 1 114 | 7 · 666 | 7 · 476 | 13 377 · 166 697 | 13 377 · 166 697 |

Under `--minimal` both SENDs are stopped under every order, the line's sweep of every owing state being what
costs, and under `--maximal` they are `--single`'s: 18 030 · 221 803 under `coupled/20` where I-SEND is
17 754 · 223 050 today. I-B06 and I-L02 take no tree and do not move under any order.

**The structure against the learned control.** Stated plainly, since it is a result about track S's use for an
order:

- **A weight learned from the run's deaths does not do what the structure does.** Of the seven learned
  orders, under `--single` **one** decides the SENDs, `learned-branch/deepest`, in 63 479 · 794 980 on each:
  **3.5 times** the 18 030 of the order from the structure, and 3.6 times today's I-SEND, a loss of 258 % on
  the one program it had to keep. The six others are stopped on both. Why, as read off the orders' definitions
  and not traced node by node: by size a weight learned in one layer reaches no choice of that layer; by the
  path every death blames the whole path, which weighs the instances in the order they were already taken; and
  started again as it goes, the weights are those of the deepest instances, which are the ones whose
  alternatives die.
- Under `-n 1` two learned orders reach SEND's state, in 51 852 and in 29 960, against 16 024 and 15 855.
- **Under `--minimal` a learned order is *go*** by the rule, as `text` is and for the same reason: with no
  structure they are the count and text, and over the cells that finish that is two nodes fewer than today.
  It is no result about learning.
- **So the order built is the structure's**, the cheapest *go* that decides I-SENDW, and the control says what
  the structure is worth here: on the corpus's one program whose constraints join three instances or more
  through derived facts, a factor of 3.5 over the best order that reads none, and the difference between
  decided and not for every other order that reads none. On the zebras, where every coupling of two is already
  a denial the count reads, the structure and the control and today are within two nodes of each other.

**What the choice reading the denials did** (addition 4 of the stage's brief). `deaths-count` is today's order
with the count reading what a saturation found dead and not what a state denies, the choice as it was before
S1f.10.3 over the memory as it is now:

| | `--single` | `-n 1` | `--minimal` | `--maximal` |
|---|---|---|---|---|
| I-Z2 and `zebra2-obligations.ein` | 9 · 892 with, **7 · 666** without | **5 · 164** with, 5 · 302 without | 267 · 23 231, 266 · 23 049 | 9 · 892, 7 · 666 |
| I-Z2M and its `-obligations` twin | 40 · 2 688 with, **30 · 2 037** without | 35 · 2 122, **24 · 1 288** | stopped both ways | 104 · 10 850, 96 · 10 268 |
| every other program | the same | the same | the same | the same |

So **letting the choice read the memory hurt** on I-Z2M under every question it finishes, by eight to eleven
nodes a cell, and on I-Z2 under three, by one or two, 24 saturations of the corpus's 19 581 under `--single`;
and it helped one cell, `zebra2.ein -n 1`, by 138 facts in the same five nodes. The 40 against a simulated 29 is this: with the order held to the deaths the
real run is 30. Why a count that knows more chooses worse there was not looked into past the table; it is four
programs, one puzzle. The order built keeps the count reading the memory: the instance the state has all but
decided is the classical choice, the rule sinks `deaths-count` under `-n 1` by that one cell, and under the
built order's last key I-Z2 is 7 either way.

**Against the ceiling.** [S1f.10.13](s1f.10.13_the_price_list.md#the-two-shares) put `--single` at 19 620
saturations against a floor of 437 for finding the leaves, I-SEND 17 747 against 9. **An order does not move
that**: with the structure's order I-SEND is 18 030, and the same tree by column was 17 747. What stands
between is not which letter is taken next: 9 350 of those deaths hold a letter's two digits or a digit's two
letters, which every state of every tree meets again, and 6 184 are a column's, one for each wrong digit of its
last letter. Both are a nogood's to remove, the pair of a death or its core
([S1f.10.3](s1f.10.3_the_restricted_join.md#the-pair-of-a-death-the-layer-by-need-and-a-deaths-core-on-top-of-the-denials)),
and a `functional` that stores what it excludes (K-13). **What the order buys is that the cost no longer hangs
on the declaration**: over the corpus with I-SENDW in it `--single` is 37 888 where it was 150 715 and stopped
on one program, and over the 189 programs that finish both ways it is 19 858 where it was 19 581: I-SEND's 283
more, the price of not reading the order it was declared in, less six nodes on the zebra2 files.

### The order to be built, by the rule, per question

Written before T4, from T3's tables, which read the rule over the cells not stopped both ways. What decides is
[§ The rule as the price list reads it](#the-rule-as-the-price-list-reads-it-per-question), below: it keeps this
table's order for six questions and sinks the minimal line.

| question | *go*, of the orders that decide I-SENDW | taken | why |
|---|---|---|---|
| `--single` | `coupled/10`, `/20`, `/40`; `written+coupled/10`, `/20`, `/40` | **`coupled/20`** | the canonical one: I-SEND and I-SENDW the same work. Of the shares, a twentieth is the least total; a fortieth reads the node later and pays a longer first tree, 20 577 over the cells finished both ways against 19 858 |
| `-n 1` | `written+coupled/10`, `/20`, `/40` | **`written+coupled/20`** | **every order with a canonical last key is sunk by one cell**, `zebra2.ein -n 1` and its `-obligations` twin, 5 · 164 → 5 · 302: the same five saturations on other nodes, 138 facts more, which is what that cell was before S1f.10.3. The written last key keeps every cell that reads no node as it is |
| `--minimal` | none decides it: both SENDs are stopped under every order. *Go*: `text`, `open-first/20`, `coupled/10`, `/20`, `/40`, the learned by size | **`coupled/20`** | over the cells that finish it is `text`, two nodes fewer than today on the zebra2 pair and no cell higher; on the SENDs the tree ends and its state is listed before the sweep is stopped, where under `text` the tree itself is stopped and nothing is listed |
| `--maximal` | `coupled/20`, `written+coupled/20`, `/40` | **`coupled/20`** | canonical. **By one share of three**: `coupled/10` and `/40` are *no*, each by the I-Z2M pair, 104 · 10 850 → 150 · 14 474, which is 108 · 11 512 at a twentieth. That cell moves by a third with the moment the node is read; the rule's verdict here rests on it |

So **the order is one function of the question**: the coupling, then the count, then the last key, the node
read within a twentieth of the run's facts after twenty saturations; and the last key is rendered text, but
under `-n N`, where it is the reported order. The four cells measured are `--single`, `-n 1` and the two lines
whole; of the cells between, a line under `-n N` takes `-n N`'s key and a line under `--single` the canonical
one ([§ Defaults taken](#defaults-taken)).

### What moves, named before the route changed

From `build/s5/m4/orders/`, each question's order against `today` (`jq` over the two files, the cells whose
saturations, facts or listed commitments differ). **Every cell not listed is today's to the node and the
fact**, but for the `route` row's words.

| question | programs whose work moves | programs whose listed states are others |
|---|---|---|
| `--single` | `beyond_arith/cryptarithm.ein` 119 · 691 → 119 · 722; `send_more_money.ein` 17 747 · 222 924 → 18 030 · 221 803; `send_more_money_written.ein` stopped → 18 030 · 221 803; `zebra2.ein` and `zebra2-obligations.ein` 9 · 892 → 7 · 666; `zebra2-minus-15.ein` and `zebra2-minus-15-obligations.ein` 40 · 2 688 → 39 · 2 568 | `send_more_money_written.ein` (none listed → its state); `zebra.ein`, `zebra2.ein`, `zebra2-obligations.ein` (the same state, `from` another commitment); `zebra2-minus-15.ein` and its twin (two others of R's 32); `ein-bugs/wrong-answers/owed-seats-renamed.ein` (two others) |
| `-n 1` and `-n N` | `send_more_money.ein` 15 854 · 192 556 → 15 855 · 192 722; `send_more_money_written.ein` stopped → 17 542 · 199 031 | `send_more_money_written.ein` |
| `--minimal` | `send_more_money.ein` and `_written`, stopped both ways, 205 166 and 131 134 → 205 840 saturations; `zebra2.ein` and its twin 267 · 23 231 → 266 · 23 049; the `zebra2-minus-15` pair, stopped both ways | `send_more_money_written.ein` (its state, before the stop); the `zebra2-minus-15` pair (a stopped listing) |
| `--maximal` | `cryptarithm.ein` 128 · 763 → 128 · 794; `send_more_money.ein` 17 754 · 223 050 → 18 030 · 221 803; `send_more_money_written.ein` stopped → 18 030 · 221 803; `lattice/05_owed_seats_bounded.ein` 9 · 20 → 8 · 17; `zebra2.ein` and its twin 9 · 892 → 7 · 666; the `zebra2-minus-15` pair 104 · 10 850 → 108 · 11 512 | `send_more_money_written.ein` |

**The goldens that will move**, by the list of [T1](#t1-the-inventory-verified-before-any-route-changed), with
the programs now named. One not on this list that moves is a stop.

- **The `route` row of every mode's read-out that takes the tree gains the order's words**: every program one
  of whose modes names L5, 36 of the 190 under `--single`. So in `corpus_shapes.md5`, for each such program, its
  `read-out[single]`, `read-out[n2]`, `read-out[minimal]` and `read-out[maximal]` lines where that mode takes
  the tree, and `region-trace` where the trace quotes the row; in
  [`read_out_examples.md`](../../../docs/kernel/inference/read_out_examples.md) every example whose route is the
  tree; in `README.md` and `docs/guide/04_solving_the_whole_puzzle.md` the transcript's `route` row.
- **Where the work or the states move as well**, the ten programs of the table: the `work` row and the listed
  state's `from {…}` on the same lines, and those programs' `cores[…]` and `region-lattice` lines where the
  nodes visited move. `zebra2.ein --single` is in the transcript (9 → 7 saturations, the state `from` another
  commitment), in `docs/api/rust.md`'s output block and in `ein-cli/tests/embedding.rs` (`saturations: 9` → `7`).
- `ein-render/tests/golden/trace_*.md`: none, unless one is of a mode that takes the tree; the two S1f.10.3
  re-banked are a blind program's, which takes none.
- `ein-cli/tests/region_modes.rs`: a block that quotes a tree's `route` row, if there is one.
- `corpus/corpus.toml`: I-SENDW's entry, its runs, its note and its `cost_ms`; the slow set, if it leaves it;
  `corpus_exits.txt`, its cells; `send_more_money.ein`'s header and note, which state 17 747, and the
  generator's anchor on that header, with the variant regenerated.

### The order built

T4, as committed: [`ein_infer::order`](../../../ein.rs/crates/ein-infer/src/order.rs)'s `Order::of`, a function of
the question and of nothing else, which every tree a route opens takes where no `order::with` is running. An
owing state branches on one licensed instance it owes, chosen once, by three keys in turn:

1. **the closest to closing a coupling**, once the couplings are read: over the edges that hold the instance,
   the fewest questions the state has not paid, the instance's own among them;
2. **the fewest alternatives not known dead**, by the run's one memory, a death or a denial: the key the tree
   had;
3. **the first by rendered text**, the instances in the order of their alternatives; under `-n N`, of all of
   `R` or of a line, the first the obligation pass reports.

**The minimal line, whole and under `--single`, then went back to the written order**, by the rule
([below](#the-rule-as-the-price-list-reads-it-per-question)); under `-n N` it keeps `-n N`'s.

The queue is as it was, the smallest closure first. **What it reads**: the state and `known_dead`, for nothing,
as before; and [`structure::NodeCoupling`](../../../ein.rs/crates/ein-infer/src/structure/coupling/node.rs), the
coupling read from the whole node alone, **one saturation, its facts those of every candidate committed**,
charged to the run, counted in its `work` row and listed among the nodes it visited (`Space::charge_whole_node`);
a whole node found dead is also written to the run's memory as ⊤ dead, so no route of the same run saturates it
again. **When**: at the first choice the count leaves tied, once the run has saturated twenty nodes
(`order::READ_AFTER`) and within a twentieth of the facts it has derived (`READ_SHARE`), the node stopped at that
bound where it needs more, what it derived counted, and tried again at four times the work. **When the couplings
are read the choices made before are made again**, each by the counts its state had when it first chose, and the
tree starts again from root where one falls otherwise (`Search::choose_again`): T3's `open-first/20` is why.

**What the read-out says.** The `route` row of a mode the tree answers says the order the run took, after the
route's name and before the licences (`explore::TreeOrder::words`):

- *branching on the owed instance with the fewest alternatives left*, where no choice was tied;
- *…, the first by text among equals*, or *the first written among equals* under `-n N` and, after the rule, on
  the minimal line, where one was; with
  *; no coupling read, the whole node costing more than a twentieth of the run* where the node was tried and
  stopped;
- *branching on the owed instance closest to closing a coupling, then the one with the fewest alternatives
  left, then the first by text: 5 couplings read off the whole node at saturation 712, the tree started again*,
  where the couplings were read.

[`read_out.md` § 3.1](../../../docs/kernel/inference/read_out.md#31-what-order-the-obligation-tree-took) defines
the words, `hypothesis_space.md` (until the rename, `structure.md`) § The coupling a route reads the reading, and `tree.rs`'s § Which instance is
restated. No JSON field was added: `region.route` is `tree` as it was, and `explore::Answer::order` holds the
order for a library's reader.

**I-SEND and I-SENDW, before and after**, a cell *saturations · facts* (`price_list --orders --only
today,routes`, `build/s5/m6/`; `routes` is the order the routes take with no `order::with`, and is
`coupled/20`'s cells, and `written+coupled/20`'s under `-n N`, to the node):

| | I-SEND before | I-SEND after | I-SENDW before | I-SENDW after |
|---|---|---|---|---|
| `--single` | *Solution*, 17 747 · 222 924 | *Solution*, 18 030 · 221 803 | stopped, 131 134, none listed | ***Solution*, 18 030 · 221 803** |
| `-n 1` | 15 854 · 192 556 | 15 855 · 192 722 | stopped, none listed | 17 542 · 199 031 |
| `--minimal` | stopped, 205 166 | stopped, 205 840; **after the rule**, the written order's, 205 166 | stopped, 131 134, none listed | stopped, 205 840, its state listed; **after the rule**, 131 134, none listed |
| `--maximal` | 17 754 · 223 050 | 18 030 · 221 803 | stopped, none listed | 18 030 · 221 803 |

I-SEND's nodes under `--single` by the letters committed (`ein solve examples/beyond_arith/send_more_money.ein
--single --dump-lattice DIR`, then a tally of `nodes.json`, whose 18 030 entries are the `work` row's
saturations, the whole node among them):

| letters committed | visited | dead | |
|---|---:|---:|---|
| none | 1 | 0 | root |
| D | 10 | 0 | |
| D E | 100 | 10 | |
| D E M | 600 | 165 | **the first tree**, under text: M after D and E, until the node is within a twentieth of the facts derived |
| all eighty candidates | 1 | 1 | **the whole node**, read at saturation 712: five couplings, and the tree started again |
| D E Y | 900 | 828 | the tree by the couplings |
| D E Y N | 720 | 216 | |
| D E Y N R | 5 040 | 4 750 | |
| D E Y N R O | 2 900 | 2 717 | |
| D E Y N R O M | 1 512 | 890 | 318 fewer than the 1 830 of the tree by column: the first tree's deaths of M's digits are known |
| all eight | 6 220 | 6 219 | and the state, in `R` |
| the state's extensions | 26 | 26 | |
| | **18 030** | | 17 747 by column, before |

### The rule as the price list reads it, per question

T3's tables read the rule as [S1f.10.3](s1f.10.3_the_restricted_join.md#defaults-taken)'s default 2 has it, over
the cells not stopped both ways. **The rule is [S1f.10.13](s1f.10.13_the_price_list.md#defaults-taken)'s, and read
as its own defaults read it**: every cell in the total, **a cell a budget stopped counting with what it had
spent**, under either order (default 2); both units, a *go* needing both (default 1); and no program's cell higher
by more than a tenth in either unit, a stopped cell compared with what it had spent. The cost of what the order
reads is in every cell: the whole node is one of the run's saturations, with its facts, and an attempt stopped
at its share is in the run's facts. Applied after T4, on the release example at T4
(`price_list --orders --only today,routes,coupled/20,written+coupled/20,text,coupled/10,coupled/40`, then
`--orders-report`, `build/s5/m6/`, 2026-11-16, a core each, on AC, the machine shared; saturations and facts are
the numbers). The harness gained two things for it: `routes`, the order the routes take with no `order::with`, and
**the four cells between the measured ones**, each line under `-n 1` and under `--single`, so that no question the
tree answers takes its order by a default. 190 programs; the written order is the tree's of before T4.

| question | the order T4 built for it | every cell, that order | every cell, the written order | cells higher by more than a tenth | by the rule | without I-SENDW, that order against the written | the cells finished both ways |
|---|---|---:|---:|---:|---|---:|---:|
| `--single` | `coupled/20` | 37 888 · 481 451 | 150 715 · 1 309 989 | 0 | **go** | 19 858 · 259 648 against 19 581 · 261 430 | the same |
| `-n 1` | `written+coupled/20` | 34 751 · 423 553 | 148 342 · 1 272 915 | 0 | **go** | 17 209 · 224 522 against 17 208 · 224 356 | the same |
| `--minimal` | `coupled/20` | 442 447 · 4 361 988 | 367 105 · 4 361 526 | **1**: `send_more_money_written.ein`, stopped both ways, 131 134 → 205 840 saturations for 1 048 559 facts | **no** | 236 607 · 3 313 429 against 235 971 · 3 312 967 | 4 599 · 170 630 against 4 601 · 170 994 |
| `--maximal` | `coupled/20` | 496 917 · 2 710 048 | 609 742 · 3 537 151 | 0 | **go** | 478 887 · 2 488 245 against 478 608 · 2 488 592 | 23 295 · 352 041 against 23 016 · 352 388 |
| `--minimal -n 1` | `written+coupled/20` | 34 891 · 424 817 | 148 475 · 1 274 109 | 0 | **go** | 17 342 · 225 716 against 17 341 · 225 550 | the same |
| `--minimal --single` | `coupled/20` | 414 775 · 2 236 327 | 339 390 · 2 235 899 | **3**: `send_more_money_written.ein` as under `--minimal`; `zebra2-minus-15.ein` and its `-obligations` twin, 45 · 2 878 → 49 · 3 271 | **no** | 208 935 · 1 187 768 against 208 256 · 1 187 340 | 3 095 · 139 209 against 3 090 · 138 789 |
| `--maximal -n 1` | `written+coupled/20` | 491 742 · 2 572 552 | 605 302 · 3 421 330 | 0 | **go** | 474 169 · 2 372 937 against 474 168 · 2 372 771 | 18 577 · 236 733 against 18 576 · 236 567 |
| `--maximal --single` | `coupled/20` | 495 899 · 2 687 567 | 608 724 · 3 514 670 | 0 | **go** | 477 869 · 2 465 764 against 477 590 · 2 466 111 | 22 277 · 329 560 against 21 998 · 329 907 |

**So the order ships for six questions of eight, and the minimal line, whole and under `--single`, keeps the
order it had**: `Order::of` returns `Order::WRITTEN` there, the count and then the first the obligation pass
reports, no coupling read; under `-n N` the minimal line takes `-n N`'s order like every line. `routes` was
measured again after that change: on the minimal line, whole and under `--single`, it is the written order's to
the cell, and on the six others `coupled/20`'s or `written+coupled/20`'s to the cell.

**How I-SEND and I-SENDW weigh in it, plainly.** On every *go* the total is lower **because of I-SENDW alone**:
the written order spent 131 134 saturations and 1 048 559 facts on it and answered nothing, the built order
decides it in 18 030 and 221 803 (17 542 to 17 573 under `-n 1`), and the price list's default 2 counts the
stopped cell with what it spent. **Without it, the corpus is a little dearer under every question that reads the
couplings**: under `--single` 277 saturations more of 19 581, 1.4 %, and 1 782 facts fewer, which is I-SEND's
283 more, 17 747 → 18 030, 1.6 %, less two nodes on each zebra2 file and one on each `zebra2-minus-15` file;
under `--maximal` and `--maximal --single` 279 more and 347 facts fewer, I-SEND's 276, 17 754 → 18 030, with four
more on each `zebra2-minus-15` file, two fewer on each zebra2 file and one fewer on
`lattice/05_owed_seats_bounded.ein`; under `-n 1` and its two line cells one saturation and 166 facts more,
I-SEND's, 15 854 → 15 855. No cell is higher by more than a tenth under any of the six. So the rule's *go* rests
on one program going from not decided to decided, which is what the stage was for (E-29), at a price of 1.6 % on
the program that was declared in the order a search could use.

**What sank the minimal line, and the next way.** Under `--minimal` the built order's total is higher in both
units, by 75 342 saturations and 462 facts, and one cell is higher by more than a tenth: I-SENDW, stopped by the
budget both ways, 131 134 → 205 840 saturations for the same 1 048 559 facts. The written order's tree is itself
stopped in its sixth layer, nothing listed; the built order's tree ends, lists its state, and the line's sweep of
the owing states the tree left is what the budget stops, on more and smaller nodes. I-SEND is stopped both ways
too, 205 166 → 205 840; the 462 facts are the four stopped cells' (the `zebra2-minus-15` pair stops at 1 047 120
facts where it stopped at 1 046 711) less the zebra2 pair's one node each. The cells that finish are lower, 4 601 →
4 599, and none is higher. Under `--minimal --single` the same I-SENDW cell sinks it, and with it the
`zebra2-minus-15` pair, 45 · 2 878 → 49 · 3 271, nine per cent more nodes and fourteen per cent more facts, which
`text` alone does as well: it is the canonical last key, not the couplings. **No order measured decides either
SEND on the minimal line** (T3's tables: both stopped under every one of 33), because what costs there is the
sweep, the line as the alive minimal transversals of the maximal owing states' complements, every owing state
extended once; an order of the tree moves which nodes the budget is spent on. The next way is S1f.10.3's priced
and not built, a death's pair or its core as a nogood that the sweep's extensions are pruned by: 9 350 of
I-SEND's deaths under `--single` hold a letter's two digits or a digit's two letters. For the four nodes on the
`zebra2-minus-15` pair under `--minimal --single`, K-26.

### T5: the invariances

Each a test in the gate, within the default `--region-budget`, through `ein_corpus::pool::map` where it sweeps.

- **Every order gives one answer.** [`ein-infer/tests/explore.rs`](../../../ein.rs/crates/ein-infer/tests/explore.rs),
  `every_order_of_the_tree_is_the_oracle`: ten orders beside the routes' own, the written one with the denials
  counted and without, its reverse, **two seeded permutations of the choice**, one alone under the routes' queue
  (added here, seed 4) and one with the other queue started again as it goes, both queues, and the couplings read
  at the first choice and at the second, third, fourth and fifth node, whole or within a share; each against
  **the oracle**, C1 and C2, on every monotone cube some mode of which a tree answers, every mode but describe,
  unbudgeted and at the budget of C3: 3 360 cells, 2 416 of them stopped by the budget. The routes' own order is
  held to the oracle by the file's other tests. [`ein-infer/tests/order.rs`](../../../ein.rs/crates/ein-infer/tests/order.rs),
  `every_order_gives_one_answer`: `--single`, `-n 1` and both lines on every corpus program and register witness
  that searches, the routes' order against six others per commit and sixteen in the nightly, the `slow` entries
  against four; every listing **the routes' own among them** (added here) held to a finished describe or to the
  maximal line listed whole, and two finished runs to one verdict, one `exhausted` and, where a part is listed
  whole, one set of states. Per commit: 4 320 cells, 231 with other work than the routes', 40 listing other
  states of `R`, none held to nothing.
- **The work does not read the order a program is written in.** `order.rs`, `the_two_sends_take_the_same_work`:
  under `--single` and `--maximal` I-SEND and I-SENDW saturate the same number of nodes, 18 030, derive the same
  facts, list the same state and report the same order taken, five couplings read; under `-n 1` they find the one
  state; under the written order of before I-SENDW `--single` is stopped. **What still reads the written order,
  and why**: `-n N`, whose last key among instances equal in coupling and in count is the first the obligation
  pass reports, by the rule (`zebra2.ein -n 1` sinks a canonical key, K-26), so the two find the one state in
  15 855 and 17 542; and the minimal line, whole and under `--single`, which keeps the written order by the rule
  and which the budget stops on both, so the two are two runs there (held, with the `slow` entries, to the written
  order and to no coupling read). And K-23: a spelling that reorders the universe moves the work, since a
  branch's alternatives and the last key are in rendered-text order.
- **The work under a permuted id space**: [`ein-render/tests/id_order_invariance.rs`](../../../ein.rs/crates/ein-render/tests/id_order_invariance.rs)
  as it was: its digests of `read-out[single]`, `[n2]`, `[minimal]` and `[maximal]` hold the `route` row's words,
  with the saturation the couplings were read at, and the `work` row to the same bytes under a permuted interner,
  per commit on every corpus program outside `tests/` that searches and in the nightly on every program, the two
  SENDs among them.
- **The work under the import forms**: [`ein-cli/tests/import_invariance.rs`](../../../ein.rs/crates/ein-cli/tests/import_invariance.rs),
  which T4 extended: where a program's `--single` takes the tree, each form's verdict, saturations and facts are
  the program's, but on `binary_logic/synthesis.ein`, named with why (K-23); **the `slow` entries too where
  `EIN_CORPUS_SLOW=1` adds them** (added here; T4 left them to the answer's comparison), the two SENDs among them.
- **No lever.** `config_reference.rs` gained no row; `order::with` is a thread-local the tests and the harness
  take; `lever_invariance.rs` holds every mode's read-out, the `route` and `work` rows among it, under every
  config flag and declared lever.
- **On a tier, within five minutes a step.** Walls on four P-core threads with four workers, on AC, the machine
  otherwise idle (`utils/bench_env.sh --cores PT:4 --require-ac env EIN_CORPUS_SLOW=1 EIN_TEST_JOBS=4 cargo test
  … --test order`, 2026-11-16). **`order.rs`'s slow half, as T3 left it, was 554 s whole**, past the nightly's
  five minutes; it is now a step of `deep-corpus-shards`, `EIN_TEST_SHARD=k/4`, a program in one shard
  (`ein_corpus::shard::keep`), the two SENDs' test whole in shard 4, in `nightly.yml`, `run_tests.sh --nightly`
  and `--dev-host`, which `nightly_steps.rs` holds to one another. Dealt whole a program to a unit, the shards
  were 322, 249, 55 and 29 s; the sweep now asks each program's base (its describe and the routes' four answers)
  and then each other order as a unit of its own, so a long program's orders run side by side, and the shards are
  **291, 168, 51 and 55 s**. Shard 1 is near the line: it is dealt three of the four longest programs, I-SEND,
  I-SENDW and `zebra2-minus-15-obligations.ein`, and I-SEND's units run one at a time in the pool's heavy lane,
  about 185 s of it. Per commit the sweep is 19 s where it was 24, and its counts are the same, 4 320 cells.
  `import_invariance.rs`'s slow half, a step of `deep-corpus`, is 119 s with the slow entries' work compared.
  `known_dead.rs` gained nothing.

### What moved

**The work**, by question, the order built against the written one (`build/s5/m6/`, `today` against `routes`):

| question | programs whose work moved |
|---|---|
| `--single` | `send_more_money_written.ein` stopped after 131 134 → *Solution* in 18 030 · 221 803; `send_more_money.ein` 17 747 · 222 924 → 18 030 · 221 803; `cryptarithm.ein` 119 · 691 → 119 · 722; `zebra2.ein` and `zebra2-obligations.ein` 9 · 892 → 7 · 666; `zebra2-minus-15.ein` and its twin 40 · 2 688 → 39 · 2 568 |
| `-n 1`, `--minimal -n 1`, `--maximal -n 1` | `send_more_money_written.ein` stopped → 17 542, 17 549 and 17 573 saturations; `send_more_money.ein` one saturation and 166 facts more under each |
| `--maximal`, `--maximal --single` | as `--single` for the two SENDs (I-SEND 17 754 → 18 030), `cryptarithm.ein` 128 · 763 → 128 · 794, `lattice/05_owed_seats_bounded.ein` 9 · 20 → 8 · 17, the zebra2 pair 9 → 7, the `zebra2-minus-15` pair 104 · 10 850 → 108 · 11 512 |
| `--minimal`, `--minimal --single` | none: the written order |

**Programs whose listed states are others**, every one a state of `R` (held by `order.rs` to a describe or a whole
maximal line, and by `explore.rs` to the oracle on the cubes): under `--single`, `send_more_money_written.ein`
(none → its state), `zebra.ein`, `zebra2.ein` and `zebra2-obligations.ein` (the same state reached from another
commitment), `zebra2-minus-15.ein` and its `-obligations` twin (two others of R's 32) and
`ein-bugs/wrong-answers/owed-seats-renamed.ein` (two others of its R); under `-n 1` and every line under `-n 1`,
and under `--maximal`, `send_more_money_written.ein` alone; under `--maximal --single`, it and the
`zebra2-minus-15` pair. Under `--minimal` and `--minimal --single`, none. On 4 programs the tree visits as many
nodes and other ones, or the same in another order (`owed-seats.ein`, `uncovered.ein`, `uncovered-knows.ein`,
`equation_two_unknowns.ein`, read off `--dump-lattice` against the binary at T3's commit), which moves their
`cores[…]` lines and nothing they print.

**The goldens that moved**, every one on [T1's list](#t1-the-inventory-verified-before-any-route-changed), and
shown against `master` after the merge by a filter that prints nothing:

```sh
git diff master -U0 -- ein.rs/crates/ein-render/tests/golden/corpus_shapes.md5 | grep '^[-+][^-+]' \
  | awk '{print $3}' | sort -u | grep -v '^examples/beyond_arith/send_more_money_written.ein::' \
  | grep -v -E '::(read-out|cores)\[(single|n2|minimal|maximal)\]$'
```

- `ein-render/tests/golden/corpus_shapes.md5`: the lines of `send_more_money_written.ein`, the program T1
  banked; `read-out[single]`, `[n2]`, `[minimal]` and `[maximal]` of the 35 other programs a mode of which the
  tree answers, the `route` row's words, and the work and listed state where they moved (T4; under `[minimal]`
  27 of them moved again with the rule, to the written order's words and work); and `cores[single]` and
  `[maximal]` of the 13 whose visited nodes moved, with `cores[n2]` of `send_more_money.ein`. `cores[minimal]`
  is master's on every program but the new one. No describe's line, no `states`, `region-trace`,
  `region-lattice` or DOT line of a program master has.
- `docs/kernel/inference/read_out_examples.md`: `--maximal -n 1` and `--single` on `zebra2.ein`, the two
  examples whose route is the tree.
- `README.md` and `docs/guide/04_solving_the_whole_puzzle.md`: the transcript of `zebra2.ein --single`, the
  `route` row, 9 → 7 saturations and the state `from` another commitment.
- `docs/api/rust.md`'s output block and `ein-cli/tests/embedding.rs`: `saturations: 9` → `7`.
- `ein-cli/tests/golden/corpus_exits.txt`: I-SENDW's two cells, `solve --single` and `test`, exit 0.
- `corpus/corpus.toml`: I-SENDW's entry (its recorded answer flipped, `test` declared, `cost_ms` re-taken) and
  the notes of `send_more_money.ein` and `cryptarithm.ein`.
- `examples/README.md`'s anchor list did not move (`world_anchors`, blessed on the merged tree).

### The merge of `master`

`git merge master` into the branch (`06df3ac0`). Four conflicts, each resolved by keeping both sides' behaviour:
`docs/kernel/inference/structure.md`, renamed `hypothesis_space.md` on master, took the stage's edits (one object
of the structure a route reads, § The coupling a route reads) with `ein render h-space` where it said `ein render
structure`, and `read_out.md` links the new page; `ein-render/src/read_out.rs` prints master's `relation names`
row and the order's words in the `route` row; `structure_invariance.rs` holds the coupling from the whole node
beside master's `h-space.json` and listing of H; `implementation.md`'s `structure.rs` row has master's text and
the sentence on the one object a route reads. `ein-infer/src/explore.rs` merged without a conflict (`known`,
`cached` and `TreeOrder`). The generated and golden files were made again on the merged tree and not merged by
hand. The two SENDs are 18 030 saturations each on the merged tree, as before it.

### Verification

On the branch's last tree, after its last edit, in the worktree (2026-11-16), `--manifest-path ein.rs/Cargo.toml`:

- `cargo test --profile gate -p ein-infer`: 549 passed in 53 binaries; `-p ein-cli`: 406 in 65; `-p
  ein-render`: 66 in 12. None failed.
- `EIN_CORPUS_SLOW=1`, once each: `-p ein-infer --test explore --test known_dead --test order`,
  `-p ein-render --test id_order_invariance`, `-p ein-cli --test import_invariance`: all passed at the default sixteen workers (`explore` 7 tests, `known_dead` 4, `order` 2, the whole slow half unsharded, `id_order_invariance` 4, `import_invariance` 1). `order.rs`'s
  four shards, each on four P-core threads, as § T5 has them.
- `cargo fmt --all --check`; `cargo clippy --workspace --all-targets -- -D warnings` (T4's `bool::then` with a
  closure was refused and is `then_some`); `RUSTDOCFLAGS="-D warnings" cargo doc --workspace --no-deps`;
  `python3 utils/check_hashmap_iteration.py`; `python3 utils/stdlib_manifest.py`; `python3
  examples/gen_send_written.py --check`; `python3 utils/doc_audit.py --links --check docs plans README.md
  AGENTS.md corpus/README.md tests/README.md examples/README.md stdlib/README.md utils/README.md c/README.md`,
  576 pages and no finding. All green.
- Not run, the batch's: `./run_tests.sh`, `--nightly`, `utils/ci_container.sh`.

### Defaults taken

Each the user's to revise. The first three are how the measurement was read, written before the order is
built.

1. **The rule is read over the cells no budget stops both ways**, as
   [S1f.10.3](s1f.10.3_the_restricted_join.md#defaults-taken)'s default 2 has it: a cell stopped under both
   orders spent its budget either way. A cell an order finishes where `today` stops counts with what `today`
   spent; so I-SENDW, stopped today, is in every total, and the totals over the programs that finish both ways
   are given beside.
2. **Of the cells between the four measured, a line under `-n N` takes `-n N`'s last key, the reported
   order, and a line under `--single` the canonical one.** A line's first state is the tree's first leaf, which
   is `-n 1`'s question: `--maximal -n 1` on `zebra2.ein`, an example of the read-out's page, is the same
   5 · 164 → 5 · 302 under a canonical key that sinks `-n 1`.
3. **I-SENDW's `--check` is a cargo test's**, `cli_semantics.rs`, as the zebra2 variants' is; no static step
   was added to the gate's six.
4. **The rule that decided is the price list's, read as its own defaults read it** (§ The rule as the price
   list reads it): every cell in the total and in the tenth, a stopped cell with what it had spent. Defaults 1
   and 2 above are how T3's tables read it, and they gave `--minimal` *go*; the price list's reading gives it
   *no*, by a cell stopped both ways whose saturations rose for the same facts, and that is the verdict taken.
   Read otherwise, a cell stopped both ways left out of the tenth, the minimal line would be *go* under
   `--minimal` and still *no* under `--minimal --single`, by the `zebra2-minus-15` pair.
5. **Of default 2's cells, none is a default any more**: each line under `-n 1` and under `--single` was
   measured and judged by the rule. The minimal line under `-n 1` is *go* and takes `-n N`'s order; under
   `--single` it is *no* and keeps the written one.
6. **The order is a function of the question, and the last key is the first written under `-n N` and on the
   minimal line.** Under `-n 1` the rule sinks every canonical last key by `zebra2.ein`'s 138 facts in an
   unchanged five saturations (K-26). The other reading, the user's to take: one canonical order for every
   question, `-n 1` on `zebra2.ein` back at the 302 facts it was before S1f.10.3, and I-SENDW under `-n 1` the
   same work as I-SEND.
7. **The count still reads the denials** (K-24): the classical choice, and the rule sinks the count held to
   deaths under `-n 1`.
8. **The share is a twentieth and the threshold twenty saturations**, `order::READ_SHARE` and `READ_AFTER`,
   constants and no lever: of the three shares measured the least total, and the one under which `--maximal` is
   *go* (K-26 says how thin that is).
9. **The queue is as it was**, the smallest closure first: the largest first is *no* for every question (K-25).
10. **The whole node is a node the run visited**: one of the `work` row's saturations and an entry of a
    `--dump-lattice`, the node of every candidate. An attempt stopped at its share is no node, and its facts are
    in the `work` row. A whole node found dead is ⊤ known dead for the rest of the run, so no route saturates it
    a second time; an alive one is ⊤'s, through the cache.
11. **The read-out says the order in the `route` row and in no JSON field.** `region.route` and
    `region.licences` are as they were; `explore::Answer::order` holds it for a library's reader.
12. **`node_coupling_in` commits every candidate**, not those alive alone, which a layer would have to say; the
    owner's full coupling is untouched and `ein render h-space` does not show the reading a route takes.
13. **A blame by a death's core was not run** as a learned order: it needs a core at every death, a search no
    route takes unless a surface prints one. The control is the run's own deaths, by the branch and by the path,
    by size and largest first, with and without starting again.
14. **Three strategies of the plan's list were not run as written**: the count *live by root's excluded pairs*
    (its source is the pair layer, which `full` shows sinks whatever reads it); the *product* of the count and
    the coupling (they are taken in turn, either first); and a *best-first* queue by the count.
15. **I-SENDW declares `solve --single` and `test`**, and no bare `solve`: describing its R is I-SEND's
    question, to the node. It stays `slow`, its two runs 1.59 s together on an idle machine, re-taken after the
    order was built: the estimate of 0.9 s that T4's note made from a loaded machine was wrong.
16. **T5's new permutation is in `explore.rs`'s set alone**, a permutation of the choice under the routes'
    queue held to the oracle; `order.rs`'s per-commit set is the six T3 chose, the permutations of seeds 2 and 3
    in the nightly's.

### What is not done, and what it leaves

- **I-SEND is 18 030 saturations against a floor of 9**, and an order was never going to move that: what
  stands between is 9 350 deaths that hold a letter's two digits or a digit's two letters and 6 184 that are a
  column's, which a nogood removes and an order does not ([§ T3](#t3-measured), *Against the ceiling*). The pair
  of a death and a death's core are priced and not built ([S1f.10.3](s1f.10.3_the_restricted_join.md)); a
  `functional` that stores what it excludes is K-13's. The same nogoods are the next way for the minimal line,
  which no order measured decides on either SEND.
- **`-n N` and the minimal line still read the declared order among equals** (defaults 4 to 6, K-26).
- **The largest state first** is a quarter less work under `-n 1` and is not built (K-25).
- **A spelling that reorders the universe moves the work** (K-23).
- **For [S1f.10.14](s1f.10.14_exactly_one_spent.md), *exactly one*.** The tree now has the couplings of what
  root owes at one node's price, and the instances L5 does not license are in none of them: an unlicensed
  instance is no question of the reading, since its alternatives are not known to be all of what can pay it.
  If S1f.10.14 licenses one, it becomes a question and is ordered with the others for nothing more. Its witness,
  `lattice/06_friends_tables_everyone_sits.ein`, 207 saturations under `--single`, does not move under any order
  measured here: the states above it that owe unlicensed instances alone branch on every extension, and no order
  chooses among extensions. And the largest state first, which the rule sank, loses most on such programs
  (`oblig-fresh-name-two.ein --minimal` 39 → 150), so a licence there would change what the queue costs.
- **`send_more_money.ein`'s own `cost_ms`** is not this stage's and was not re-banked: on the same idle machine
  its declared runs were 14.40 s together, the bare `solve` 12.87 s, against the 8 404 ms recorded
  (`utils/bench_env.sh --cores PT --require-ac python3 utils/corpus_cost.py -k send_more_money --runs 5`,
  2026-11-16). The flag stands either way; the state page owes it a quiet re-take.
- **`order.rs`'s first shard is 291 s of the nightly's 300** on this machine (§ T5). I-SENDW belongs on
  `ein_corpus::pool::LONG` by that list's own definition, a unit of 5 s or more, and on it would be dealt away
  from I-SEND; but the list orders every sweep's start and deals every sharded step, `answer_round_trip`,
  `corpus_cores` and `id_order_invariance` among them, whose shards were measured with it as it is. Not done:
  it wants those steps measured again with it, on the runner.
- `./run_tests.sh`, the nightly and `utils/ci_container.sh` were not run: the batch's, on the merged tree. No DOT
  writer changed.
