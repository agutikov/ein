# S1f.10.19 — A dead node stopped at its first contradiction

**Phase:** [P1f.10](README.md), track R (the search)
**Status:** planned 2026-11-16, on the user's *"Yes, both"* of that day. Not started.
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
