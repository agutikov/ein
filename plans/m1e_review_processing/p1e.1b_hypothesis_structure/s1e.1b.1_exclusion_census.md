# S1e.1b.1 — The exclusion relation, measured before it is used

**Phase:** [P1e.1b](README.md) (The structure of the hypothesis set)
**Estimate:** 3 days
**Depends on:** [Q-M1e.6](../open_questions.md#q-m1e6--what-is-a-solution-and-what-is-a-model)
— the ruling on what a solution is, which makes *"h is excluded"* a statement
about the answer rather than about the search.
**Blocks:** every other stage of this phase. Nothing is built on the exclusion
relation until it has a number.

## Context

The phase's first rung is one predicate:

```
excludes(h₁, h₂)  ≡  saturate(rules ∪ ontology ∪ {h₁, h₂})  derives (false)
```

and the whole phase turns on the parenthesis in it: **`ontology`, not the
puzzle.** If the exclusion needs a clue, it is a fact about *this* puzzle
state and belongs with the lookahead — recomputed per node, useless for the
join. If it needs only the ontology, it is a fact about the *program*,
computed once at load and valid at every node, and that is the property
S1e.1b.3 spends.

This stage does not build the predicate into the engine. It measures it,
because two things are unknown and both change what S1e.1b.3 is:

1. **How much is there?** The corpus's layer-1 hypothesis sets are large —
   `zebra2` alone has 96 singletons at root — and the number of *excluding*
   pairs among them is unmeasured. If it is small, the phase is a footnote.
2. **Does the minimal KB agree with the full one?** For each excluding pair,
   the same check run against the fully saturated root must agree. A pair the
   minimal KB calls compatible and the full KB refutes is fine — that is the
   clues doing work. A pair the **minimal KB refutes and the full KB does
   not** is a bug in the minimal KB's definition, and finding one is this
   stage's most valuable outcome.

## What *minimal KB* means, and why the stage owns the definition

`rules + ontology` is precise in [I-Z2](README.md#the-instances) — the
`(rule …)` / `(hrule …)` forms, the `(relation …)` signatures, the
`(bijective …)` declarations, the `(import …)`ed stdlib — and imprecise
everywhere a *clue* is itself an `is-a` fact. `zebra.ein` states
`(is-a Norwegian Nationality)` (ontology) beside `(co-located Norwegian H1)`
(clue) in the same syntax.

The stage's proposed rule, to be falsified against the corpus:

> The minimal KB is every form of the program **except** ground facts of a
> relation that the query's `:goal` names or that any `(hrule …)` /
> obligation proposes. Everything else — rules, imports, `(relation …)`,
> `(is-a …)`, `(config …)` — is ontology.

That is mechanical, it keeps `(is-a Norwegian Nationality)` and drops
`(co-located Norwegian H1)`, and it is wrong in at least one direction on some
corpus entry. **Finding which** is task T3.

## Acceptance

- **`utils/exclusion_census.py`** exists, is listed in
  [`utils/README.md`](../../../utils/README.md) under *checks*, names its
  binary through `$EIN_BIN` like the other twenty-three, and is re-takable.
- Its answer is recorded in
  `docs/history/…/exclusion_census.md` — or, while M1e is unshipped, in
  `plans/m1e_review_processing/p1e.1b_hypothesis_structure/exclusion_census.md`
  — with, per corpus entry that searches at all: `|alive₀|`, the number of
  L1 pairs, how many exclude under the **minimal** KB, how many under the
  **full** root, the disagreement count, and the fraction of layer 2's join
  the minimal exclusions would have removed.
- **The disagreement column is zero, or every non-zero cell is explained**
  by name. This is the stage's gate on the rest of the phase.
- **A null result is a result.** If the excluding fraction is small on every
  entry that is not a bijection puzzle, the census says so and P1e.1b's
  estimate is cut before S1e.1b.3 is written.
- Nothing in the engine changes. The census drives the shipped binary.

## Tasks

### Task T1e.1b.1.1 — The oracle, outside the engine first

Build the check as a **script**, not as a crate function, and build it out of
the CLI the engine already has:

1. emit the minimal KB as a `.ein` file (the program's forms, filtered by the
   rule above),
2. append `h₁` and `h₂` as ground facts,
3. `ein saturate` it and read whether `(false)` is present.

Slow and obviously correct, which is the right order. `zebra2`'s 96 singletons
are 4 560 pairs and each is a whole-program saturation, so the census will
want the batching of T3 — but the *first* implementation is the naive one,
because it is the thing everything later is diffed against.

The precedent is exact: `model_set_census.py` reads `--json-summary` rather
than linking the crates, and it is why its numbers survived S1d.3.3 changing
the read-out.

### Task T1e.1b.1.2 — The full-KB arm, and the disagreement column

The same pair, against the **saturated root** of the real program. Two
sub-questions, and they are not the same:

- Does the full KB refute a pair the minimal KB called compatible? *Expected,
  frequently* — that is a clue doing its job, and the count is interesting
  but not alarming.
- Does the minimal KB refute a pair the full KB does not? **This must not
  happen.** Saturation is monotone in the fact set, so a `(false)` derivable
  from a subset is derivable from the superset — unless a rule is
  non-monotone in a way `absent` makes possible, which is exactly the
  [NAF boundary](../../../docs/kernel/inference/absent_semantics.md). A hit
  here is a finding about `absent` and the stage stops to report it rather
  than continuing.

### Task T1e.1b.1.3 — Batch it, and price the whole corpus

Naive is `O(|alive₀|²)` whole-program saturations. Two reductions, in order:

1. **Type-filter first.** A pair whose facts share no argument cannot be
   refuted by any rule that does not mention both — cheap to over-approximate
   from the rules' relation sets, and it should remove most of the 4 560.
2. **One process, many pairs.** If the script is the bottleneck, the honest
   move is a `#[test]`-shaped Rust probe reusing one `Ast`/`Terms` — but only
   after the script's answer exists to diff against, per T1.

Record the wall-clock. A census that costs more than the search it optimises
is still a census, but the stage should say so.

### Task T1e.1b.1.4 — What it would have bought

For each entry, take layer 2's actual candidate count from the `layer` event
([`layer_census.py`](../../../utils/layer_census.py)'s sixteen counters) and
compute how many of those candidates contain an excluding pair. That number —
**not** the pair count — is what S1e.1b.3 removes, and it is the phase's
headline.

Do the same for layer 3, where the effect compounds: a 3-set is removed if
*any* of its three pairs excludes.

## Notes

The comparison instrument this stage needs — *run the same program two ways
and diff* — is now wanted in a fourth place
([S1e.1.1](../p1e.1_open_questions/s1e.1.1_search_soundness_probes.md)'s Notes
counted three, and [AR-M1](../README.md#the-findings) is a finding about the
habit). This stage does **not** build it either; it uses `--json-summary` and
`--events` like its siblings, and P1e.3 [S1e.3.4](../p1e.3_medium/s1e.3.4_architecture.md)
is where the fifth copy gets refused.

The predicate has a shorter name in the literature — it is a binary constraint
between two CSP variables, and the graph is a *microstructure*.
[`docs/lib/`](../../../docs/lib/README.md) is where that reading is
catalogued, and the census is worth writing in vocabulary that lets a reader
find it there.
