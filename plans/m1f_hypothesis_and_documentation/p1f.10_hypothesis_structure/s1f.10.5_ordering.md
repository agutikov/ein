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
