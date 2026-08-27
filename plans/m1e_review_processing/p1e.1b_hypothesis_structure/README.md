# P1e.1b — The structure of the hypothesis set

**Estimate:** ~3 weeks — 5 stages, 14 days.
**Depends on:** [P1e.1](../p1e.1_open_questions/README.md)
[S1e.1.1](../p1e.1_open_questions/s1e.1.1_search_soundness_probes.md) only —
for [Q-M1e.6](../open_questions.md#q-m1e6--what-is-a-solution-and-what-is-a-model)'s
ruling, which this phase is the first consumer of. Nothing else.
**Blocks:** nothing in M1e. It is a *new capability*, and every stage of it is
answer-preserving by construction — the acceptance is *the same model sets,
fact for fact*, on every corpus entry.
**Source:** the user's instruction of 2026-08-28, recorded verbatim in
[§ The instruction](#the-instruction).

---

## What this phase is, and the honest note about its placement

M1e is a milestone that **processes a review**. This phase does not: it is
engine work, it was not asked for by any of the 63 findings, and it would sit
as comfortably in an M-number of its own. It is here because the user put it
here, and because the review's own [Q5](../review/open-questions.md) turned
out to terminate in the same question — *what does it mean for a hypothesis to
be decided?* — that this phase answers structurally.

Two consequences a reader should hold:

- **M1e's acceptance is unchanged.** The 63 dispositions and the ten
  questions are the milestone; this phase is additive and may be cut whole
  without touching them.
- **This phase may not change an answer.** Not one model, not one fact, not
  one verdict word. Every stage's acceptance is a fact-for-fact model-set
  comparison across the corpus, and the win is measured in *enterings*.

## The instruction

> set of L1 hyps has mutually inconsistent hyps, e.g. (Norvegian lives in
> House1) and (Norvegian lives in House2). It can be checked on small fact set
> of this 2 facts, without the rest of KB — taking rules, ontology and this 2
> facts and check. So hyp set can be split into groups of hypothesis to pick
> one from (who lives in House2, where is tee drinked, etc.). So at least hyp
> set generator not enumerate all possible N-sets of hypothesis, but select
> already maybe compatible subsets. This is at least. Next step is somehow
> functionally find that there are bounded hyp groups, e.g. permutations of
> pets, permutations of nationalities. So hypothesis selection could be
> ordered by the structure of hypothesis set, not by simple lexical order.

## Why it is worth a phase

The search enumerates **subsets of a fixed `alive` set**. Layer *L* enters the
*L*-subsets that survive the apriori prefix-join and the no-good filter
([`apriori.rs`](../../../ein.rs/crates/ein-infer/src/apriori.rs)), and the only
thing that removes a candidate before it is entered is a **learned clause** —
which is to say, something the search already paid a death for.

Nothing removes a candidate for a reason the *theory* could have stated in
advance. `{(nation-loc Norwegian H1), (nation-loc Norwegian H2)}` is a
2-subset the join proposes, the engine forks, saturates, derives `(false)`
from `functional`, and learns a width-2 clause — **per pair, per puzzle, every
run**. There are 250 such pairs in `zebra2`'s five bijections before a single
clue is read, and every one of them is a property of `(bijective nation-loc)`
and the ontology alone.

The measured shape of the waste is already in the tree. M1d
[S1d.10.1](../../../docs/history/m1d_satisfiability/layer_census.md)'s census
found that for **25 of the 49** entries that search at all, the enterings are
*exactly* `Σₖ C(alive, k)` — the full powerset, with the no-good machinery
removing **1.4 %** corpus-wide and dead-element pruning removing **0**. The
lattice is, on those entries, an unfiltered binomial walk.

And the comparison this repo already keeps is the sharpest statement of the
gap: [`c/README.md`](../../../c/README.md)'s three baselines differ by
**3 668 465×** and *"the difference is not an algorithm — it is one integer
per clue, the level at which every attribute it names is bound."* This phase
asks the same question of the hypothesis set: what does the search already
have, structurally, that it is not being told?

## The ladder

Five stages, and each one is a strictly larger claim about the same object.

| rung | the claim | who checks it |
|---|---|---|
| **exclusion** | `h₁` and `h₂` cannot both hold, and the proof needs **rules + ontology + the two facts** and nothing else | [S1e.1b.1](s1e.1b.1_exclusion_census.md) |
| **groups** | the exclusion relation partitions — or covers — the hypothesis set into *pick at most one from each* | [S1e.1b.2](s1e.1b.2_groups.md) |
| **the join** | a candidate set containing two members of one group is refused **before** the fork | [S1e.1b.3](s1e.1b.3_the_restricted_join.md) |
| **bounded groups** | two overlapping group families whose members exhaust each other are a **bijection**, so the space is `n!` and not `2^(n²)` | [S1e.1b.4](s1e.1b.4_bounded_groups.md) |
| **order** | the traversal order follows the structure — most-constrained group first — instead of the canonical tuple sort | [S1e.1b.5](s1e.1b.5_ordering.md) |

### The one distinction the whole phase rests on

**Exclusion is state-independent; the lookahead is state-dependent.**

`Lookahead::dies_immediately` ([`hypgen.rs:440`](../../../ein.rs/crates/ein-infer/src/hypgen.rs))
simulates one rule firing **against the saturated root KB** — so its answer is
a property of *this* state and has to be recomputed at every node, and it is
the costliest of the four filters for exactly that reason.

The exclusion this phase computes takes `rules + ontology + {h₁, h₂}` and
**nothing else**, which means:

- it is a property of the *program*, computed once at load;
- it is valid at **every** node of the search, root and depth 7 alike;
- it cannot flip under a hypothesis — which is the property
  [Q6](../p1e.1_open_questions/s1e.1.1_search_soundness_probes.md) found the
  tree's rung probe *lacks*.

That last line is why this phase is not merely an optimisation. A
state-independent branch structure is one a tree traversal may cache at root
**soundly**, and it is the first candidate this repo has for jointly
exhaustive branches on a program that declares no `(open …)` rule at all.

### What it does not do

Pairwise exclusion does **not** subsume `complete`, and
[`examples/lattice/02_genuine_3set_death.ein`](../../../examples/lattice/02_genuine_3set_death.ein)
is the counterexample the corpus already carries: its `(false)` needs all
three of `a-prop`, `b-prop`, `c-prop`, so no pair excludes and no group forms.
Its own header says so — *"the genuine combinatorial-core case … no amount of
subset elimination can avoid the size-3 fork."* Q-M1e.6's definition of a
solution still needs the general test; this phase makes the common case free,
not the general one.

## The instances

Named here and used by every stage.

| tag | file | why it is the one |
|---|---|---|
| **I-Z1** | [`examples/zebra.ein`](../../../examples/zebra.ein) | one `co-located` equivalence relation; the group structure is **implied** by `sibling-exclusive` + `functional` and declared nowhere. The subject |
| **I-Z2** | [`examples/zebra2.ein`](../../../examples/zebra2.ein) | five `*-loc` projections, each `(bijective …)`. The structure is **declared**, so it is the control: whatever S1e.1b.1 discovers here it must agree with |
| **I-B06** | [`examples/branching/06_lookahead_on.ein`](../../../examples/branching/06_lookahead_on.ein) | I-Z1 in miniature — five colours, five houses, one `co-located` — and already the corpus's most expensive cell that is not `slow` (377 ms) |
| **I-L02** | [`examples/lattice/02_genuine_3set_death.ein`](../../../examples/lattice/02_genuine_3set_death.ein) | the negative control: a 3-way conflict with no pairwise exclusion. Every stage asserts it is **unchanged**, which is what keeps the phase honest about its own reach |
| **I-Z2M** | `examples/zebra2-minus-15.ein` | 32 models, 23 varying variables in **one** coupling component ([model_set_census](../../../docs/history/m1d_satisfiability/model_set_census.md)). The under-determined case, where a group order should matter most |

## Stages

| ID | title | est. | ends with |
|---|---|---:|---|
| [S1e.1b.1](s1e.1b.1_exclusion_census.md) | The exclusion relation, measured before it is used | 3 d | `utils/exclusion_census.py`; per corpus entry, how many L1 pairs exclude, how many of them the **minimal** KB proves without the puzzle's clues, and what fraction of the join they would have removed |
| [S1e.1b.2](s1e.1b.2_groups.md) | From the exclusion graph to *pick at most one* | 2.5 d | a written definition of a **group** that the corpus does not falsify — cliques, not components — plus the finding that groups **overlap** and why that is the bijection rather than a defect |
| [S1e.1b.3](s1e.1b.3_the_restricted_join.md) | The join, restricted | 3 d | `generate_layer` refuses a candidate containing two members of one group; every corpus model set identical fact for fact; the enterings delta measured per entry |
| [S1e.1b.4](s1e.1b.4_bounded_groups.md) | Bounded groups — rediscovering the bijection | 3 d | a program with no `(bijective …)` and no `(open …)` gets the same branch structure as one that declares it, or the stage says exactly which of the two it cannot recover and why |
| [S1e.1b.5](s1e.1b.5_ordering.md) | Ordering by structure | 2.5 d | a third `lattice-order` mode, measured against `lex` on the five instances — and `most-constrained`, which has returned `0.0` since it was named, either implemented or deleted |

## Acceptance

- **Not one answer moves.** Every corpus entry's model set is identical fact
  for fact before and after, at the same `-m` and `-e`, and the check is the
  comparison [`tree_traversal.rs`](../../../ein.rs/crates/ein-infer/tests/tree_traversal.rs)
  already makes — *"never of `k`. Two searches that agree on a count and
  disagree on a model are exactly the failure"*.
- **Every rung is measured before the next is built.** S1e.1b.1's census is
  the phase's gate on itself: if the minimal-KB check disagrees with the
  full-KB one on any corpus entry, the structure is state-dependent after all
  and S1e.1b.3 does not ship as written.
- **The cost is stated in enterings, per entry**, and the entries where it is
  **zero** are named — I-L02 is expected to be one of them, and a phase that
  cannot name its own null cases has not measured.
- **No new hand-maintained parallel copy** ([AR-M1](../README.md#the-findings)).
  The group structure has one owner; if `apriori`, `hypgen` and `oblgen` all
  need it, they read it from that owner.
- `./run_tests.sh` green at every stage boundary; every golden this phase
  moves is named in its stage file before it moves. **The expectation is that
  it moves none**: enterings counters are not golden, but
  `corpus_shapes.md5` and the event goldens are, and a candidate the join
  never proposes is an `enter` event that never happens.

## Risks

- **It is a CSP in disguise, and this repo has said so before.**
  [`docs/lib/`](../../../docs/lib/README.md) catalogues the constraint-solving
  literature precisely so this rewrite can borrow from it rather than
  rediscover it. Groups are variables, exclusion is an all-different, and
  most-constrained-first is a textbook heuristic that has been in
  `hypgen_scoring` **as a name with no implementation** since S1.5a.7. The
  risk is not that the idea is wrong — it is that the stage writes a solver
  instead of a *structure*, and the engine acquires a second search. Every
  stage here changes what the existing search is **told**, never how it walks.
- **The minimal KB is not obviously well-defined.** *Rules + ontology* is
  clear in `zebra2` — `(relation …)`, `(is-a …)`, the `(bijective …)`
  declarations — and much less clear where a "clue" is itself an `is-a` fact.
  S1e.1b.1 owns the definition and the corpus is what falsifies it; getting
  this wrong in the permissive direction produces an exclusion that is really
  a consequence of the clues, and a group that is wrong on a *different*
  puzzle with the same ontology.
- **Overlapping groups are the normal case, not the exception.**
  `(color-loc Blue H1)` is excluded by *Blue is elsewhere* and by *H1 is
  another colour*: two groups, one member. A design that assumes a partition
  will be wrong on the first bijection it meets, which is the second corpus
  entry it reads. S1e.1b.2 exists to get this stated before S1e.1b.3 depends
  on it.
- **The win may already be taken on the entries that matter.** `zebra.ein`
  solves in 111 enterings and `zebra2.ein` in 101; the powerset behaviour
  S1d.10.1 measured is on the *under-determined* entries. The phase should be
  honest that its headline is `zebra2-minus-15`-shaped work, not the solved
  puzzles, and S1e.1b.1's census is what will say so with a number.

## Connections

- [`docs/history/m1d_satisfiability/completeness.md`](../../../docs/history/m1d_satisfiability/completeness.md)
  — why an obligation's alternatives are jointly exhaustive. A group is the
  same claim, derived instead of declared.
- [`docs/history/m1d_satisfiability/domain_contract.md`](../../../docs/history/m1d_satisfiability/domain_contract.md)
  — C1 and C4. C4 in particular: *a branch is jointly exhaustive only while
  the candidate set cannot grow underneath it*. A state-independent group
  cannot grow underneath anything, which is the point.
- [`docs/kernel/inference/domain_elim_vs_hypothesis.md`](../../../docs/kernel/inference/domain_elim_vs_hypothesis.md)
  — the same structure as a **saturation rule** (`domain-elimination`,
  `no-room-left`) rather than as a generator restriction, measured in 2025.
  This phase must say why it is not simply that.
- [`docs/history/m1d_satisfiability/layer_census.md`](../../../docs/history/m1d_satisfiability/layer_census.md)
  — the `Σₖ C(alive, k)` finding, which is this phase's motivating number.
- [`c/README.md`](../../../c/README.md) § Circular dependencies between levels
  — the 3 668 465× and what one integer per clue bought.
- [`plans/ideas/06-inference-rules-completeness.md`](../../ideas/06-inference-rules-completeness.md)
  — the user's own framing of what the rule set owes.
