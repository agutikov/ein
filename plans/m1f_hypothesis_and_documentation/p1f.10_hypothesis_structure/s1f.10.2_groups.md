# S1f.10.2 — From the exclusion graph to *pick at most one*

**Phase:** [P1f.10](README.md), track S (the structure)
**Status:** re-planned 2026-11-15. Not started.
**Depends on:** [S1f.10.1](s1f.10.1_exclusion_census.md), the excluded pairs.
**Read by:** [S1f.10.4](s1f.10.4_bounded_groups.md), [S1f.10.11](s1f.10.11_coupling.md),
[S1f.10.12](s1f.10.12_the_structure_shown.md), and of track R [S1f.10.5](s1f.10.5_ordering.md) and
[S1f.10.14](s1f.10.14_exactly_one_spent.md).

## Context

The instruction says the hypothesis set *"can be split into groups of hypothesis to pick one from (who lives in
House2, where is tee drinked, etc.)"*. That is the right object, and **split** is the word the corpus will not
grant. Three candidate definitions of a group over the exclusion graph `G`:

| definition | *pick at most one* holds? | on `zebra2`'s `nation-loc` |
|---|---|---|
| **connected component** of `G` | **no**: two vertices of a component need not be adjacent | one component swallowing all five `*-loc` relations, because the clues link them |
| **maximal clique** of `G` | **yes**, by construction | the houses left to one nationality are a clique, and the nationalities left to one house another |
| **class of a declared key** | yes, but only where the program declares it | `(bijective nation-loc)`, which I-Z2 has and I-B06 does not |

Cliques are the only one that makes the phrase true, and their price is that they **overlap**:
`(color-loc Red House-3)` is in *where is Red?* and in *what colour is House-3?*. Every member of a
bijection's candidate set is in two, the rows and the columns of one grid. So a group is a set, a candidate is
in several, and what the engine holds is a **cover** of H, not a partition.

**At root the grids are ragged.** The givens and the clues have already decided some candidates, so `zebra2`'s
H is 56 candidates of the 125 its five relations span, and a row is the houses *still open* to a value. The
definition is over H as root builds it, and the stage says so wherever it prints a size.

## Tasks

1. **T1f.10.2.1 — the three definitions against the corpus.** Components, maximal cliques and declared-key
   classes for every program S1f.10.1 read; where they differ, the program is named. Expected, each a row of
   the record: components ≠ cliques on every puzzle whose clues link attributes; cliques = declared classes on
   I-Z2, **which is the control**: if they disagree, S1f.10.1's pairs are wrong and the stage stops; cliques
   where nothing is declared on I-B06.
2. **T1f.10.2.2 — which cliques, in what order.** All maximal ones is the complete answer and can be
   exponential in a dense graph. A bijection's graph is a rook's graph, whose maximal cliques are its rows and
   columns; nothing guarantees every corpus program's graph is so benign, and the puzzle's own exclusions add
   edges across grids. **Bound the enumeration and print what the bound cut.** The order is canonical: a group
   is its members in rendered-text order, and groups are ordered by that, so the cover is a function of the
   program and not of the enumeration.
3. **T1f.10.2.3 — beside what the program declares.** For each program, the owed instances root holds with
   their alternatives (`oblgen::Alternatives`, licensed or not), and the relations its hrules propose and its
   closures take out (`mechanisms.rs`). Three columns of the record: a discovered group that is an owed
   instance's alternatives; a discovered group nothing owes; an owed instance whose alternatives are **not**
   pairwise excluding. S1f.10.4 starts from the three.
4. **T1f.10.2.4 — the owner.** The pairs and the cover live in one module of `ein-infer`, computed once per
   program from root's nodes, which the census, the bundle and, if track R goes ahead, the routes read. The
   stage names the module and gives it the two maps a reader needs, group to members and member to groups. It
   takes the program's prepared space, not a `&Kb` of some node: a representation that takes a node invites the
   next reader to recompute it per fork.

## Acceptance

- The definition of **group** is written in this file, with the corpus program that falsifies each rejected
  alternative named.
- The cover is computed for every program S1f.10.1 read and recorded beside its census: the number of groups,
  their sizes, how many groups a candidate is in (the most and the mean), and the candidates in **no** group.
- **The in-no-group count is the honest column.** A candidate nothing excludes is one no group says anything
  about, and I-L02's three are exactly that.
- **On I-Z2 the cover is the rows and the columns of the five declared bijections**, over the candidates root
  leaves, plus whatever the clues add, each addition listed.
- The enumeration is deterministic and its order pinned by a test: the cover of a program is the same under a
  permutation of the interner's ids.
- No answer, route or surface changes.

## What it must not do

- **Not assume a partition**, anywhere, including in the names of fields.
- **Not read `is-a`** or any relation by name to form a group: a group is cliques of dead pairs and nothing
  else. What the program declares goes beside it, in its own columns.

## Notes

The two clique families of a bijection are the two projections of a bipartite relation, and *exactly one from
each row and each column* is a perfect matching. That is the vocabulary
[S1f.10.4](s1f.10.4_bounded_groups.md) needs; this stage does not assume it, since a cover is well defined
whether or not two families happen to be a grid, and I-Z1's single `co-located` relation is not obviously one.
