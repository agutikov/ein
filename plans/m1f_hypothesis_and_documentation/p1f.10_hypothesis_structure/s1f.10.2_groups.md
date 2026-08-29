# S1f.10.2 — From the exclusion graph to *pick at most one*

**Phase:** [P1f.10](README.md)
**Estimate:** 2.5 days
**Depends on:** [S1f.10.1](s1f.10.1_exclusion_census.md) — the graph, and the
disagreement column being clean.
**Blocks:** [S1f.10.3](s1f.10.3_the_restricted_join.md) (which refuses a
candidate per group) and [S1f.10.4](s1f.10.4_bounded_groups.md) (which asks
when a group is *exactly* one rather than *at most* one).

## Context

The instruction says the hypothesis set *"can be split into groups of
hypothesis to pick one from (who lives in House2, where is tee drinked,
etc.)"*. That is the right object. This stage's whole content is that the word
**split** is the part the corpus will not grant, and getting the definition
right before S1f.10.3 depends on it is cheaper than getting it wrong.

Three candidate definitions of a group over the exclusion graph `G`:

| definition | *pick at most one* holds? | on `zebra2`'s `nation-loc` |
|---|---|---|
| **connected component** of `G` | **no** — two vertices in a component need not be adjacent | one component swallowing all five `*-loc` relations, because `next-to` clues link them |
| **maximal clique** of `G` | **yes**, by construction | 5 nationalities × 1 house = a 5-clique, and 5 houses × 1 nationality = another |
| **equivalence class** of some declared key | yes, but only where the program declares it | `(bijective nation-loc)` — which [I-Z2](README.md#the-instances) has and [I-Z1](README.md#the-instances) does not |

Cliques are the only one of the three that makes the phrase true, and the
price of cliques is that they **overlap**.

## Overlap is the structure, not a defect

`(color-loc Blue H1)` belongs to two cliques:

- *where is Blue?* — `{(color-loc Blue H1) … (color-loc Blue H5)}`
- *what colour is H1?* — `{(color-loc Red H1) … (color-loc Blue H1)}`

Every member of a bijection's candidate set is in exactly two, and the two
families are the rows and the columns of the same grid. A design that assumes
a **partition** — each hypothesis in one group — is wrong on the second corpus
entry it reads, and would silently drop one of the two constraints.

So a group is a set, a hypothesis belongs to many, and the object the engine
holds is a **cover**, not a partition. Stating that here is most of the
stage's value; [S1f.10.4](s1f.10.4_bounded_groups.md) is where the two
families being *the same grid* becomes a bijection.

## Acceptance

- A written definition of **group** in the phase's notes, with the corpus
  entry that falsifies each rejected alternative named — not argued.
- The group cover computed for every corpus entry that searches, and reported
  beside S1f.10.1's census: number of groups, size distribution, how many
  groups a hypothesis is in (max and mean), and the count of hypotheses in
  **no** group.
- **The in-no-group count is the honest column.** A hypothesis excluded by
  nothing is one the restricted join cannot help with, and
  [I-L02](README.md#the-instances)'s three are expected to be exactly that.
- Cliques are found **deterministically**. Maximal-clique enumeration has no
  canonical order for free, and this repo's rule is that anything reaching a
  traversal is canonically ordered
  ([design/02](../../../docs/history/m1a_rust/design/02_determinism_and_order.md)).
  The stage states the order and pins it.
- No engine change. The cover is computed by the census script.

## Tasks

### Task T1f.10.2.1 — Enumerate, and check the three definitions against the corpus

Compute components, maximal cliques and declared-key classes for every entry;
report where they differ. The expected findings, each of which is a row in the
notes:

- **components ≠ cliques** on any puzzle with spatial clues (`next-to`,
  `right-of` link otherwise-independent attributes);
- **cliques = declared classes** on [I-Z2](README.md#the-instances), which is
  the control — if the discovered cliques disagree with the five declared
  `(bijective *-loc)` structures, S1f.10.1's oracle is wrong and this stage
  stops;
- **cliques exist where nothing is declared** on
  [I-Z1](README.md#the-instances) and [I-B06](README.md#the-instances), which
  is the phase's reason to exist.

### Task T1f.10.2.2 — Decide *maximal* vs *maximum*, and the determinism of it

Maximal-clique enumeration (Bron–Kerbosch) returns a set of sets whose order
depends on the pivot choice. Two decisions:

1. **Which cliques are kept?** All maximal ones is the complete answer and can
   be exponential in a dense graph. The bijection case is benign — the graph
   is a rook's graph and its maximal cliques are exactly the rows and columns
   — but the corpus contains
   [`branching/06`](../../../examples/branching/06_lookahead_on.ein), whose
   untyped candidate set includes the type atoms, and nothing guarantees its
   graph is benign. **Bound it, and log what was dropped** — the phase's own
   *no silent caps* rule.
2. **In what order?** Canonical, by the same `cmp_set` the lattice orders
   commitments with ([`apriori.rs`](../../../ein.rs/crates/ein-infer/src/apriori.rs)),
   so that the cover is a function of the program and not of the enumeration.

### Task T1f.10.2.3 — The cover, as a thing the engine could hold

Sketch — **not** implement — the representation S1f.10.3 will need:
group id → sorted member list, plus member → group ids. Two constraints from
the phase README's acceptance:

- **one owner.** `apriori` needs member→groups to filter a join; `hypgen`
  would want group→members to order; `oblgen` already has an equivalent
  object in its per-instance `Branch` list. Three readers, one owner, and the
  stage names which crate holds it.
- it is computed **once per program**, not per node — that is the whole claim
  of [S1f.10.1](s1f.10.1_exclusion_census.md), and a representation that
  takes a `&Kb` invites the next reader to recompute it per fork.

## Notes

The overlap structure has a name — the two clique families are the two
projections of a **bipartite** relation, and *pick exactly one from each row
and each column* is a perfect matching. That is the vocabulary
[S1f.10.4](s1f.10.4_bounded_groups.md) needs and it is worth writing the notes
in it, but this stage should not assume it: a group cover is well-defined
whether or not the two families happen to be a grid, and
[I-Z1](README.md#the-instances)'s single `co-located` relation is not
obviously one.
