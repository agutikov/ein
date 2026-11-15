# S1f.10.10 — The theory's share: rules, ontology and the two facts

**Phase:** [P1f.10](README.md), track S (the structure)
**Status:** planned 2026-11-15, at the re-plan. Not started.
**Depends on:** [S1f.10.1](s1f.10.1_exclusion_census.md), the excluded pairs at root.
**Read by:** [S1f.10.12](s1f.10.12_the_structure_shown.md); [S1f.10.13](s1f.10.13_the_price_list.md), which
prices the small-KB check as one way of getting the pairs.

## Context

The user's instruction, 2026-08-28:

> It can be checked on small fact set of this 2 facts, without the rest of KB — taking rules, ontology and this
> 2 facts and check.

S1f.10.1 says which pairs exclude **at root**, with everything the program states in the KB. This stage says of
each such pair **why**: it is *of the theory* when rules, ontology and the two facts suffice, and *of the
puzzle* when it needs something the puzzle states. That is information of its own: `zebra2`'s grids are the
theory's and are the same for every puzzle over that ontology, and what the clues add is this puzzle's.

Two readings, and the stage builds both because each checks the other.

**The small KB.** The program with the puzzle's statements taken out, the two facts put in, saturated; the pair
is *of the theory* if that is dead. Its difficulty is the definition. In `zebra2.ein` a condition is one of
three things: a fact of a candidate's relation (conditions 9 and 10, `(drink-loc Milk House-3)`), the activator
of a rule with parameters (twelve of the fifteen, `(co-located nation-loc Englishman color-loc Red)`), or a
fact of the layout every rule reads (condition 1, the four `right-of` facts). The first two are the puzzle; the
third is ontology, and taking it out breaks the theory. No syntactic mark separates them: all three carry
`:source "condition (n)"`. The stage's proposed rule, to be falsified against the corpus:

> The small KB keeps every rule, every import, every `(relation …)`, every fact of a relation the program
> closes or no rule concludes and no candidate is of, and drops (a) every stated fact of a relation a candidate
> is of, and (b) every stated activator that names, as an argument, a relation a candidate is of.

It keeps `(is-a Red Color)`, `(right-of House-2 House-1)` and `(bijective color-loc)`, the last naming the
relation as its whole subject, which (b) must be worded to keep. It drops the two givens and the twelve
activators. It is wrong in some direction on some corpus program, and finding which is T2.

**The support of a death.** The node `{a, b}` is dead by a derivation, and the engine records what a
derivation rests on (provenance, the cores, [`explain.rs`](../../../ein.rs/crates/ein-infer/src/explain.rs)).
The stated facts a death rests on are its support; a pair whose support holds no statement of the puzzle is of
the theory, and otherwise the support **names** the conditions, which is more than a yes or a no: *excluded by
condition (2)*. A support is an explanation, so it may vary with rule order, as a core does; it is sufficient,
not least.

**Each holds the other.** A pair the small KB kills must have, among its supports, one that lies inside the
small KB. And any support, put with the rules and the two facts and nothing else, must be dead: the user's
check, run on the support. A disagreement is a finding and the stage stops on it.

## Tasks

1. **T1f.10.10.1 — the small KB, by the proposed rule**, on the instances first: a program written to
   `build/structure/<program>/theory.ein`, loadable, and the pairs S1f.10.1 found re-read on it. Saturation is
   monotone in the facts stated, so a pair dead on the small KB must be dead at root; one that is not is a
   refutation resting on an `absent` that the smaller KB flips, which *forbid* is meant to have refused, and
   the stage stops and shows it.
2. **T1f.10.10.2 — where the rule is wrong.** Over the corpus programs that search: every program where the
   small KB does not load, where it has a dead root, or where a pair of one of the program's declared
   bijections is not of the theory. Each named, and the rule restated until the corpus does not falsify it, or
   the residue listed as programs the rule does not fit.
3. **T1f.10.10.3 — the support.** For each excluded pair, the stated facts its death rests on, read from the
   node the pair layer already computed, with the rule that concluded `(false)`. The check that makes it
   trustworthy runs on every pair of the instances: support, rules and the two facts, saturated, dead.
4. **T1f.10.10.4 — the two against each other**, and the record: per program, pairs of the theory, pairs of
   the puzzle by condition, and disagreements, which are zero or each explained by name.

## Acceptance

- Every excluded pair of every instance is marked *of the theory* or *by* a list of stated facts, in
  `build/structure/<program>/`, raw and as a table.
- **On I-Z2 every same-value and same-house pair of the five relations is of the theory**, and no pair that
  needs a condition is.
- **I-Z2 and I-Z2M have the same theory's share**: the two programs differ by condition (15) and nothing else,
  so the pairs of the theory among the candidates both have are the same pairs. An invariance, held by a test
  on the pair.
- The small KB's rule is written here as the corpus left it, with the programs it does not fit.
- The disagreement count is zero, or every non-zero cell is explained by name.
- Each number is recorded with its command and date. No answer, route or surface changes.

## What it must not do

- **Not spend a theory's pair in the search.** Track R spends pairs that are nodes of the program's own
  lattice (S1f.10.3); this stage classifies them.
- **Not read `:source`** to decide what the puzzle states: it marks condition (1) too.
- **Not make the support canonical.** That is F22's, for cores, and the same answer will serve here.

## Notes

The predicate has a name in the literature: a binary constraint between two CSP variables, and the graph of
allowed pairs is the *microstructure*. [`docs/lib/`](../../../docs/lib/README.md) is where that reading is
catalogued.
