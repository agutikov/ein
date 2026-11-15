# S1f.10.1 — The pair layer: what one candidate and two decide at root

**Phase:** [P1f.10](README.md), track S (the structure)
**Status:** re-planned 2026-11-15. Not started.
**Depends on:** nothing.
**Read by:** every other stage of tracks S and R.

## What it is

The first object of [§ The structure](README.md#the-structure), read from the program's own lattice. For H, the
universe of `n` candidates root builds:

| object | read from | definition |
|---|---|---|
| **dead alone** | the node `{a}` | it is dead |
| **implies** | the node `{a}`, alive | `b` is in its closure: committing `a` brings `b` |
| **equivalent** | two alive single nodes | one closure, as a candidate and its symmetric twin in `zebra.ein` |
| **excludes** | the node `{a, b}`, both alive alone, neither implying the other | it is dead |
| **interacts** | the same node, alive | its fixpoint holds a fact that neither `{a}`'s nor `{b}`'s does |

Each is a statement about a node [`region.rs`](../../../ein.rs/crates/ein-infer/src/region.rs) defines, so each
is true of the lattice R is a region of, and the excluded pairs are by definition the members of N that hold
two candidates. *Interacts* is recorded here because the node is already computed; it is
[S1f.10.11](s1f.10.11_coupling.md)'s input.

**What needs no saturation.** A pair with a member dead alone is dead (L1). A pair one of whose members implies
the other has the larger closure (L3). Both hold on a program the static check calls monotone, and the census
says for each program which licence it used; on a flagged program it saturates every pair and prints the
[domain](README.md#the-domain)'s sentence.

## The vehicle

A measurement example in the crate, `ein-infer/examples/pair_layer.rs`, beside `state_reach` and
`route1_reach` and in their shape: every corpus file and register witness that loads, or the `FILE`s given; one
JSON line per program; `--jobs`; a budget in facts beyond root. It computes a node through the function the
routes use (`explore::Space`'s), never a second one, which is what makes its answer comparable with a
describe's.

The old plan built this as a script over `ein saturate` on a rewritten copy of the program. That reads a pair
as two **stated** facts of another program, which is the from-scratch route; the lattice's node is a fork of
root, and where the two differ is F21's. The small-KB check the user's instruction describes is
[S1f.10.10](s1f.10.10_the_theorys_share.md)'s, where it is the subject.

## Tasks

1. **T1f.10.1.1 — the example.** Singles first, then the pairs the two licences leave. Per program, one line:
   `n`, the static verdict, dead alone, the implications, the equivalence classes, pairs saturated, pairs
   excluded, pairs interacting, saturations and facts beyond root spent, wall; and the lists themselves, as
   rendered text, in a file per program under `build/structure/<program>/pairs.json`.
2. **T1f.10.1.2 — held to N.** A test, in the gate: on every corpus program whose describe exhausts within the
   default budget, the excluded pairs are exactly N's members of two candidates and the candidates dead alone
   are the read-out's `dead alone`. Two computations, one answer. N is read from
   [`described.rs`](../../../ein.rs/crates/ein-infer/src/described.rs), which takes it from the maximal alive
   states by duality, so the two share no code past the node. The sweep uses `ein_corpus::pool::map`.
3. **T1f.10.1.3 — the census.** Over every corpus program that searches (`corpus_ops::searches`): the line
   above, and three derived columns: the share of all pairs that exclude; the share of the candidates in at
   least one excluded pair; and the layer's cost against the program's describe, `--single` and `--maximal`
   where they finish, in saturations. Recorded in this file, with the command and the date.
4. **T1f.10.1.4 — what it does not reach.** Every program the budget stops is named with its `n` and the
   pairs left. Every flagged program is named.

## Acceptance

- The example exists, is listed in [`utils/README.md`](../../../utils/README.md)'s measurement set, and is
  re-takable by one command.
- T2's test is in the gate and green; it reads no program past the default budget.
- The census is recorded here: per program the columns of T3, the programs T4 names, and for the
  [instances](README.md#the-instances) the lists themselves, linked.
- **I-L02 has no excluded pair**, three alive singles and no implication.
- **I-Z2's excluded pairs include every same-value and every same-house pair of each `*-loc` relation** among
  the candidates root leaves, and the census says how many more there are; those are the puzzle's, and
  S1f.10.10 says which.
- No answer, route or surface changes. The example reads.

## What it must not do

- **Not filter pairs by type or by relation.** The kernel imposes no type system; a pair is skipped only by L1
  or L3. What a cheaper way of getting the pairs would be is [S1f.10.13](s1f.10.13_the_price_list.md)'s to
  price.
- **Not write a second node function.**
- **Not add a CLI option or a subcommand.** The surface is [S1f.10.12](s1f.10.12_the_structure_shown.md)'s,
  after the user has seen the structure.

## The standing rules

Config never changes an answer. No test runs past the default `--region-budget`. A corpus sweep uses the worker
pool. The gate is `./run_tests.sh`, once on the last commit of the batch this stage's code is in.
