# S1f.10.19 — A dead node stopped at its first contradiction

**Phase:** [P1f.10](README.md), track R (the search)
**Status:** planned 2026-11-16, on the user's *"Yes, both"* of that day. T1 to T3 started 2026-11-16; T4 waits for S1f.10.17.
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
