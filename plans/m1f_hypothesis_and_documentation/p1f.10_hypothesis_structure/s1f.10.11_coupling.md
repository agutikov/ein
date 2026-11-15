# S1f.10.11 — Coupling: what ties the groups together

**Phase:** [P1f.10](README.md), track S (the structure)
**Status:** planned 2026-11-15, at the re-plan. Not started.
**Depends on:** [S1f.10.1](s1f.10.1_exclusion_census.md) (*interacts*), [S1f.10.2](s1f.10.2_groups.md) (the
groups).
**Read by:** [S1f.10.12](s1f.10.12_the_structure_shown.md); of track R, [S1f.10.5](s1f.10.5_ordering.md),
whose witness needs it.

## Context

Groups say which candidates answer one question. They do not say which questions bear on each other, and two
of the phase's instances turn on exactly that:

- **I-L02**: three candidates, no pair excludes, the three together are dead. Pairs see nothing.
- **I-SEND**: *different letters, different digits* is a matter of pairs. *D + E gives Y* is not: no pair of
  `(val D d)`, `(val E e)`, `(val Y y)` is dead, and most triples are. The by-column declaration order ends in
  17 747 saturations because it guesses the three letters of a column next to each other; the written order
  does not end.

A **coupling** is a set of groups that some conflict or some derivation needs a member of each of. With the
groups as nodes it is a hyperedge, and the result is the groups' **hypergraph**: a CSP's constraint graph, with
a reason on every edge. What a reader gets from it: which questions are independent of which (its components,
which is where R factors), which statement of the program ties which questions (a condition of `zebra2`, a
column of SEND), and what an order should keep together.

The exact object exists already for a program whose describe exhausts: N, the minimal dead sets, and the
members of N with three candidates or more are the couplings no pair shows. But a describe is the expensive
thing, and on I-SEND and I-Z2M it does not finish. So the stage's work is to find a **cheap reading**, and to
say honestly what each one misses.

## The readings to try, cheapest first

| | reading | finds | misses |
|---|---|---|---|
| **1** | **pairs that interact**: S1f.10.1 already has, for every alive pair, whether its fixpoint holds a fact neither single's does. An edge between the groups of the two | every coupling of two groups: `zebra2`'s `co-located` conditions; D with E in SEND, if the pair derives the column's sum, as its `col` relation suggests | a third group that only joins once two are known: Y |
| **2** | **the support of what a pair derives**: for each fact a pair's node adds, the rules that can read it next and the candidate relations those rules still wait for | the third member, as *what this pair's conclusion is waiting for* | anything two steps away |
| **3** | **one node with every alive candidate in**: saturated to quiescence, dead at once and read anyway, as a dead node's debts are; every firing's support in H is a hyperedge of any arity | every coupling one firing shows, in one saturation | couplings that need a chain of firings through derived facts, unless supports are followed back; and its size is unknown until measured |
| **4** | **N**, where a describe exhausts | exactly the minimal conflicts | every program where it does not exhaust |

Reading 4 is the reference the others are held to on the programs it reaches.

## Tasks

1. **T1f.10.11.1 — reading 1, free.** From S1f.10.1's census: the graph of groups joined by an interacting or
   excluding pair of members in different groups, its components, and per edge the stated facts the
   interaction rests on (S1f.10.10's support). For I-Z2 the expected picture is the twelve conditions that
   name two attributes, each an edge or a small set of edges between the groups it names.
2. **T1f.10.11.2 — the controls.** I-L02 must show its triple. I-SEND must show its columns: {D, E, Y},
   {N, R, E} with the carry, and so on to the leading M. Reading 1 is expected to fail both; readings 2 and 3
   are tried in that order until the controls appear, each one's cost in saturations and facts recorded.
3. **T1f.10.11.3 — held to N.** On every corpus program whose describe exhausts within the default budget:
   every member of N with three candidates or more lies inside one hyperedge of the reading taken (no minimal
   conflict spans groups the hypergraph calls unrelated). The converse is not claimed: a coupling may never
   produce a death.
4. **T1f.10.11.4 — the components against R.** Where a describe exhausts, the read-out's `components` row and
   the hypergraph's components are compared, and the stage says what the relation is. If the hypergraph's
   components are finer than R's the reading has missed a coupling, and that is a stop.
5. **T1f.10.11.5 — the record**: the reading taken, what it costs per program, what it provably misses, and
   the hypergraph of every instance under `build/structure/<program>/`, raw and drawn.

## Acceptance

- The definition of a coupling and of the hypergraph is written here, with the reading that computes it and
  that reading's stated blind spot.
- **I-L02 shows one coupling of its three candidates. I-SEND shows its columns.** Or the stage says which of
  the two no reading short of N finds, with the attempts recorded; that is a result, and S1f.10.5 then knows
  what it cannot read.
- T3's test is in the gate, reads nothing past the default budget, and is green.
- The cost of the reading is recorded per program, and beside the pair layer's.
- No answer, route or surface changes.

## What it must not do

- **Not run a describe to get a coupling**, except as T3's and T4's reference on programs where one already
  finishes.
- **Not read a relation by name.** A coupling is what nodes and supports show; `column` and `co-located` are
  the programs' words, not the engine's.
- **Not order anything.** What an order does with the hypergraph is S1f.10.5's.

## Notes

In the literature this is the constraint hypergraph, its components are the independent subproblems, and the
heuristics that read it are the degree heuristics (Brélaz) and the weighted-degree family (*dom/wdeg*), which
learn the weights from failures. [`docs/lib/`](../../../docs/lib/README.md) is where they are catalogued; the
second is the *learned* counterpart of reading 3 and belongs to track R if anywhere.
