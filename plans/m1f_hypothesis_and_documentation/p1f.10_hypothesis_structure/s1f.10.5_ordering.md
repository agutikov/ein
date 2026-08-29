# S1f.10.5 — Ordering by structure

**Phase:** [P1f.10](README.md)
**Estimate:** 2.5 days
**Depends on:** [S1f.10.2](s1f.10.2_groups.md) (the cover) and
[S1f.10.3](s1f.10.3_the_restricted_join.md) (so the order is measured against
a join that is already restricted, not against the old one).
**Blocks:** nothing.

## Context

> So hypothesis selection could be ordered by the structure of hypothesis set,
> not by simple lexical order (what else ordering do we have btw?)

### The inventory — every order point the engine has today

Answering the parenthesis first, because the stage is about adding to this
list and the list is shorter than it looks. **Eight** places impose an order;
**one** of them is structural.

| # | where | over what | the choices | notes |
|---|---|---|---|---|
| 1 | [`apriori::order_candidates`](../../../ein.rs/crates/ein-infer/src/apriori.rs) `:214` | a layer's **commitment sets** | `lex` (default) · `score-sum` | `lattice-order`, and `-o` on the CLI. `lex` is `cmp_set` — the canonical tuple sort |
| 2 | [`hypgen::score_hypothesis`](../../../ein.rs/crates/ein-infer/src/hypgen.rs) `:815` | one **hypothesis**, as a number | `popularity` (default) · `most-constrained` | `hypgen-scoring`, with `hypgen-rel-weight` / `hypgen-obj-weight`. **`most-constrained` returns `0.0`** — a name with no implementation since S1.5a.7 |
| 3 | [`hypgen::by_participation`](../../../ein.rs/crates/ein-infer/src/hypgen.rs) `:701` | the blind rung's **focal objects** | fixed | most-participating first, symbol rank as tiebreak. **The one structural order in the engine**, and it is a degree count |
| 4 | `hypgen::relation_plan` | **relations** within a focal object | fixed | the `:hypothesis-relations` / `:no-hypothesis` scoping, then declaration order |
| 5 | [`oblgen::Choice`](../../../ein.rs/crates/ein-infer/src/oblgen.rs) | the **owed instances** | `rule-order` (default) · `fail-first` · `off` | `EIN_OBLIGATION_CHOICE`. `fail-first` is smallest-candidate-set-first — and it is **measured inert** under a lattice, because the rung emits the union and #1 re-sorts it |
| 6 | `candidate-order-seed` | one branch's **candidates** | content sort (`-1`) · seeded permutation | S1.5a.1a; a probe, not a heuristic |
| 7 | `lattice-order-seed` / `--shuffle` / `--seed` | a layer's sets | random permutation | exists to **prove** the answer is order-invariant, not to speed anything |
| 8 | `plans_for` / the saturation agenda | **rule firings** | fixed | `(priority, load order)`. Not a search order at all, but it is where a reader looks next |

Read together: of the eight, **five are fixed**, two exist to probe
invariance, and the one heuristic that was named for structure (#2's
`most-constrained`) was never written. Nothing in the list knows that
`(nation-loc Norwegian H1)` and `(nation-loc Norwegian H2)` are the same
question asked twice.

### Why order matters at all, given the answer does not depend on it

It does not change *what* is found — `--shuffle` exists to keep that true —
but it changes *when*, and `ein solve` defaults to `-n 1`. Under the default
stop policy the search stops at the first model, so the order **is** the cost.
And under `-e` it still matters: a model found early makes every later layer's
`stop_after` check and no-good store richer.

## What structure buys, and the one honest doubt

The textbook heuristic is **most-constrained-variable first**: branch on the
group with the fewest live members, because it is the one most likely to fail
fast and it has the smallest branching factor. That is exactly what
`most-constrained` was named for, exactly what `oblgen`'s `fail-first` does
for owed instances, and — the doubt — **`fail-first` was measured inert.**

The reason it was inert is specific and does not carry over: the obligations
rung emits the **union** of every instance's candidates, and #1 re-sorts the
whole layer canonically, so no order the rung imposes survives. A group order
applied at #1 — over the sets the layer will actually enter — is not in that
position. The stage should say so before it measures, and then measure.

## Acceptance

- A third `lattice-order` mode exists and is measured against `lex` on all
  five [instances](README.md#the-instances) plus the corpus: enterings to
  first model (`-n 1`), enterings to exhaustion (`-e`), and wall for both.
- **The answer is unchanged**, and the check is the existing one:
  `--shuffle`'s invariance property already asserts that a permutation of the
  layer does not move the verdict, so a new order is a permutation and the
  existing test covers it. Say which test.
- **`most-constrained` is resolved**: implemented with the group cover behind
  it, or deleted from `FIELDS` with a load error naming the two live modes.
  A third state — a documented mode that returns a constant — does not
  survive this stage. It is [MA-M1](../../m1e_review_processing/README.md#the-findings)'s shape (dead
  scaffolding with a suppressor) in a user-facing flag.
- The default does **not** change unless the corpus sweep says it should, and
  if it does, every baseline that quotes an entering count moves with it and
  is named first.

## Tasks

### Task T1f.10.5.1 — The order, as a comparator over sets

Group-aware `cmp_set`: order a candidate set by the *smallest live group* any
of its members belongs to, then by group size, then fall through to `lex` so
the order stays total and canonical. Determinism is not optional here —
design/02's rule is that anything reaching a traversal is canonically ordered
and that a tie is broken by content, never by hash.

### Task T1f.10.5.2 — Implement `most-constrained`, or remove it

The mode has been a documented choice returning `0.0` since S1.5a.7. With the
cover it has an obvious meaning: `score_hypothesis(h)` = the reciprocal of the
smallest group `h` is in. Either write that, or delete the mode — and if it is
deleted, the error message that replaces it goes in
[`defined_behaviour.md`](../../../docs/kernel/defined_behaviour.md)'s
diagnostics list, because that page is the one statement of what the loader
refuses.

### Task T1f.10.5.3 — Measure, on the two questions separately

`-n 1` and `-e` are different questions and a heuristic can win one and lose
the other. Report both, per entry, and name every entry where the new order is
**worse** — a heuristic with no losing cell has not been measured on enough
programs.

The instrument is `utils/bench_env.sh` plus the existing corpus sweep, and the
numbers go beside S1f.10.1's census.

## Notes

There is a fourth ordering question this stage does not open: **within** a
group, which member first? For a bijection every member is symmetric until a
clue breaks the symmetry, and choosing well is the value-ordering half of the
CSP pair (least-constraining-value). It is a separate heuristic, it needs the
group structure this phase builds, and it belongs to whoever picks the phase
up next — named here so the omission is deliberate.
