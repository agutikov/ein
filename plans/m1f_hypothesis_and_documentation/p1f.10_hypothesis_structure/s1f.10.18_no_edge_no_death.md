# S1f.10.18 — A state that closes no coupling cannot die

**Phase:** [P1f.10](README.md), track R (the search)
**Status:** planned 2026-11-16, on the user's *"Yes, both"* of that day. Not started.
**Depends on:** [S1f.10.11](s1f.10.11_coupling.md) (the coupling, its edges), [S1f.10.5](s1f.10.5_ordering.md)
(`structure::NodeCoupling`, the edges read off the whole node in one saturation), [S1f.10.13](s1f.10.13_the_price_list.md)
(the rule for *go*). **Built after [S1f.10.17](s1f.10.17_describe_by_borders.md) is merged**, since both change which
nodes the routes saturate; its measurement does not wait.

## The idea, as the user took it up

The main session, 2026-11-16, answering *"It really make sense that we don't need to saturate owed states if no rule
can discharge it … saturating it to false may prune sub-lattice under it, right?"*: a saturation of an owing state
buys only its *dead* bit, which is what prunes, so the useful states to saturate are those that **can** die; and

> a state `S ∪ {b}` that closes no edge through `b`'s question cannot die, so it is alive without a saturation.

The user, the same day: *"Yes, both"*.

## The claim, and what it must survive

On a monotone program, let `S` be an alive state and `b` a candidate alive alone. Every contradiction `st(S ∪ {b})`
holds and `st(S)` does not is derived with `b` among its supports, and its supports' questions form an edge of the
coupling read off the whole node (`st` of every candidate alive alone, which holds `st(S ∪ {b})` by monotonicity). If
no edge through `b`'s question has every one of its questions met by `S ∪ {b}`, no such contradiction exists, and
`S ∪ {b}` is alive. **This is an argument, not yet a licence.** What it must survive, each checked against the code
and the corpus:

- the edges are read with bounds (`NODE_APPLICATIONS`, the ways and descriptions a derivation is cut at) and are
  coarse where a derived fact's label is a union of classes: a cut reading proves nothing;
- a contradiction the node derives through a fact a debt or an absent reads, or the kernel's mirror;
- a candidate dead alone, which the whole node does not commit;
- L8's denials, which are contradictions of two facts;
- a closure that adds candidates, so that `S ∪ {b}`'s state is not the set itself.

If it holds, its premise is checked on every node a route computes, a node that breaks it refusing the run, as L5's,
L8's and L9's are.

## Tasks

1. **T1f.10.18.1 — the argument**, written in this file under
   [`standard_of_proof.md`](../../../docs/kernel/standard_of_proof.md)'s rule 2, with every item above settled, or
   the program that breaks it banked under `examples/` with a corpus entry, which ends the stage at a written *no*.
2. **T1f.10.18.2 — measured with no route changed**: on the visited nodes of every corpus program that searches,
   every question, the nodes that close no edge through their last candidate; how many of them were alive (the
   saturations the licence would save) and **how many were dead (each one a counterexample)**; the cost of the
   whole node where a route did not already read it. Against the oracle on every cube within its limit. The rule
   for *go* written here before the table.
3. **T1f.10.18.3 — built where T2 says *go***, after S1f.10.17 is merged: a licence of its own name in the style of
   L1–L9, through `known_dead`'s owner of what a route may skip, with the internal with/without switch and its test,
   the goldens named before they move, and `read_out.md`'s `route` row, `finding_r.md` § 4 and the routes' headers.
4. **T1f.10.18.4 — the record.**

## Acceptance

- The argument is written whether it holds or not; a counterexample is banked.
- R and every field of the description identical with and without, every mode against the oracle.
- The work delta per program and question, losers named, the whole node's cost included.
- No lever; no relation read by name.

## What it must not do

- **Not skip a node whose state a surface prints** without saying where its content comes from.
- **Not ship a licence whose premise is argued and not checked at every node.**
