# S1d.3.1 — What the 32 models actually differ in

**Phase:** [P1d.3](README.md) (Model sets without enumeration)
**Estimate:** 3 days
**Depends on:** [P1d.2](../p1d.2_obligations/README.md) — a state can say what
it owes, which is what makes "the candidate sets *are* the answer" a testable
sentence rather than a hope.
**Status: done 2026-08-25.** The instrument, the sweep and the probe — banked
in [`model_set_census.md`](model_set_census.md). See § What it found.

## What it found

| claim | asked for | measured |
|---|---|---|
| the instrument | `utils/model_set_census.py`, the two censuses' conventions | **exists**, the twenty-second script; `--json`, `-k`, `$EIN_BIN`, argv mirroring `plan.rs`, ~9 min for the corpus |
| the factorisation, everywhere | every multi-model entry, not the reconnaissance's one | **13 entries**, not the nine the phase README counted — a `-m 2` count against *the depth that finds every model*. **10 of 13 exhausted** |
| does anything factor | a number, in the form that can be false | **by relation 0 of 20 independent · by partition 2 of 13 · by basis 5 of 13**, and no entry has `Π dom == k` |
| the coupling, made of what | components, and for the one interesting entry what relates the variables | `zebra2-minus-15`: **one** component, K₂₃ minus five edges, **minimum vertex separator 17**; within-relation 42/42 by `injective`, across 206/211 by eleven clues |
| the leftover-open count | measured, or declined in P1d.2's form | **measured.** `hypgen::generate_blind` + `--json-summary`'s `leftover` block under `EIN_LEFTOVER=1`, on a discarded fork. 244 states probed; `zebra2`'s unique model leaves **3 678** |
| the census, banked | with the size of the thing being described | [`model_set_census.md`](model_set_census.md) — 32 × 435 = **13 920 fact lines** against `solve -e`'s **516** |

**Three things the stage found that nothing asked for.**

- **The product form is real and it dies at three objects.**
  `saturation/type-exclusivity/{colors,nationalities}` — two instances — split
  9 models as 3 × 3 over two independent blocks and are the only entries in the
  corpus that partition. `pets.ein` is the same program with three instances:
  one component, 35 models. So the free-by-product path is not a fantasy, it is
  a property of fixtures too small to need it.
- **Two variables are in every minimum key of `zebra2-minus-15`** —
  `pet-loc:Horse` and `pet-loc:Zebra` — and `pet-loc:Zebra` is also the
  variable with *all five* pairwise-independent partners. Pairwise independence
  is not joint independence, and the variable that looks freest edge by edge is
  the one no description can omit. It answers
  [S1d.3.2](s1d.3.2_representations.md)'s objection to form (b) — *why these
  four* — with something better than "minimality".
- **The 3 678 leftover facts are not what the phrase suggests.** Not one is an
  attribute arrow: all five `*-loc` relations are decided to the last well-typed
  pair, and the whole count sits on `is-a`, `is-a*`, `next-to` and `right-of` —
  exactly the four the obligations rung calls `uncovered` — where most
  candidates are ill-typed, because the kernel imposes no type system by
  design. The literal open-world reading is not merely large, it is unusable,
  and that is a stronger input to
  [S1d.3.3](s1d.3.3_the_verdict.md) than the count alone.

**One task did not need doing the way the plan drew it**, recorded rather than
left looking done: T1d.3.1.2 asked the instrument to *refuse to count* a
two-model entry's trivial factorisation. It reports `degenerate` for an entry
with fewer than two varying variables, and **no corpus entry is degenerate** —
even the `k = 2` toys have 2–8 varying variables, so the two-model factorisation
the task worried about does not arise. What does arise is the opposite: a
`k = 2` entry with 8 varying variables and one degree of freedom, which the
*basis* column catches and the partition column cannot.

## Context

The phase turns on one sentence in its own README:

> When those choices are **independent**, that state is exactly the compact
> answer: the model count is the product of the candidate-set sizes, and no
> search is needed to report it.

**A reconnaissance taken 2026-08-25 says they are not.** One entry, one run
(`solve -e -m 3 examples/zebra2-minus-15.ein`, 25.5 s, all 32 models), and
three tests of independence at three granularities:

| granularity | what was tested | result |
|---|---|---:|
| by relation | is `color-loc`'s projection independent of `pet-loc`'s? | **every one of the 10 pairs is coupled** |
| by attribute | 23 varying decision variables `(relation, value) → House` | product of domains = **9.95 × 10¹³** against 32 models |
| by any partition | connected components of the coupling graph | **one component, all 23 variables** |

So the product form does not exist here, and there is no partition of the
decision variables that recovers it. The stage's headline question is
therefore already answered *for the phase's own case*, and what this stage is
for is everything that answer leaves standing.

**Three things the reconnaissance did find**, and each is a lead rather than a
result, because each rests on n = 1:

- **78 % of every model is shared.** 340 of 435 facts are in all 32 (312
  positive, 28 negative); 190 vary, and they vary in an exact mirror — 95
  positive `*-loc` arrows and the 95 `(not …)` that negative completion writes
  beside them. A "certain core plus a varying frontier" is a compact form that
  costs nothing to compute and is *lossy*: it says which facts are settled, not
  which combinations are possible.
- **The minimum determining set is four variables.** 22 of the 8 855 quadruples
  fix all 23; none of the 1 771 triples does. And a key does not compress the
  way independence would: `(Red, Japanese, Horse, Zebra)` ranges over **32 of
  the 320** combinations its own domains allow, so the description is still a
  32-row table — four columns wide instead of twenty-five.
- **Exactly two of the 25 decision variables are fixed**, and they are the
  puzzle's two stated arrows: `Milk@House-3` and `Norwegian@House-1`. That is
  the same asymmetry [S1d.2.4](../p1d.2_obligations/s1d.2.4_obligations_in_the_saturator.md)
  found from the other end — `nation-loc` and `drink-loc` owe 8 at root where
  the other three owe 10 — arriving at the answer instead of the question.

**And the corpus has one interesting case.** Nine entries report `k > 1` under
`solve -e -m 2`; seven are two- or three-model toys, and the other two are
`zebra2-minus-15.ein` and its obligations twin. **A phase deciding "print or
describe" is deciding it on n = 1**, which is a fact about the corpus this
stage must state rather than discover late — it bears directly on
[S1d.3.3](s1d.3.3_the_verdict.md)'s willingness to ship anything.

## Tasks

### Task T1d.3.1.1 — the instrument

`utils/model_set_census.py`, the third census after
[`layer_census.py`](../../../utils/layer_census.py) and
[`openness_census.py`](../../../utils/openness_census.py), and following their
conventions: `$EIN_BIN` / `--bin`, argv mirroring `plan.rs`, a `--json` machine
copy, `-k` for one entry, and **no counter re-derived from a stream the engine
already reports**. Its transport is `--json-summary`'s `verdict.solutions`.

Per multi-model entry it reports:

| column | what it is |
|---|---|
| `k` | models found, and at what `-m` |
| `core` / `varies` | facts common to all models / facts in some and not others, each split by polarity |
| `vars` | decision variables — `(relation, first-arg)` pairs whose value differs — and how many are fixed |
| `components` | connected components of the coupling graph, with each component's projection count |
| `product` | Π of per-variable domains, against `k` — the independence claim as a ratio |
| `key` | the minimum determining set's size, and how many of its domain's combinations are realised |

**The decision-variable extraction is the part that can be wrong**, and it must
not be hand-tuned to the zebra shape. `(relation, first-arg) → second-arg` is
right for a functional binary relation and meaningless for anything else, so
the instrument reads the **program's** `(functional R)` / `(bijective R)`
declarations to decide what a decision variable is, and reports the facts it
could not bucket rather than dropping them. An entry whose variation is not
functional is a finding, not an unsupported input.

### Task T1d.3.1.2 — the factorisation test, over every multi-model entry

The reconnaissance, re-taken properly and extended: all nine entries, each at
the depth that finds every model it has, and the three granularities above.
`zebra2-minus-15` at `-m 3` is 25 s and is the only expensive one.

The claim to settle is the phase README's, in the form that can be false:
**does any multi-model entry factor?** A "no" everywhere is the useful answer
and closes the free-by-product path with a number. A "yes" on one of the
two-model toys is not evidence — a two-model set factors trivially — and the
instrument should say so rather than count it.

### Task T1d.3.1.3 — what the coupling is *made of*

The measurement that tells [S1d.3.2](s1d.3.2_representations.md) whether any
representation can exploit anything. One coupling component of 23 variables is
a *result*; the reason is a set of rules, and they are nameable.

For `zebra2-minus-15`: which rules relate two decision variables, and how many
variables does each relate? `co-located` couples the five attributes of one
house; `next-to` / `right-of` couple adjacent houses; `injective-negative`
couples the five values of one attribute. The question is whether the coupling
graph has **structure** — a tree-width, a chain, a small separator — or is the
K₅-minus-two-edges of [`c/README.md` § Circular dependencies between
levels](../../../c/README.md), which is what the puzzle's constraint graph is
known to be. **A small separator is the one finding that would make a decision
diagram cheap**, and its absence is what makes "enumerate" the likely answer.

### Task T1d.3.1.4 — the per-state leftover-open count

The number [P1d.2 handed forward
explicitly](../p1d.2_obligations/hypotheses_from_obligations.md): *how many
facts would the blind enumerator still propose at a node the rung called
complete?* It is the open-world half of the same question — a model with n
leftover open facts is 2ⁿ models under one reading and one model under
closed-world completion — and P1d.2 declined it because the probe is not a
read:

> `enable_lookahead_kill_cache` writes `(not h)` into the KB it walks, which
> would change the node's `state_key` and therefore the model dedup. A probe
> that had to disable a config flag to avoid changing the answer is a probe
> that is measuring a different engine.

So the task is first to find a probe that *is* a read — a generation pass on a
fork whose writes are discarded, priced against the node count — and then to
take the number. If no such probe exists at acceptable cost, that is the
finding, and [S1d.3.3](s1d.3.3_the_verdict.md) decides the closed-world
question without it.

### Task T1d.3.1.5 — the measurement, banked

`model_set_census.md` beside this file, the third of the milestone's censuses
and re-takable like the other two. It carries the tables, the reconnaissance
superseded by the full take, and — because this is the number every later
decision leans on — **the size of the thing being described**: 32 models ×
435 facts is 13 920 fact lines, against `solve -e`'s current 516 lines of
output on the same run.

## Acceptance

- `utils/model_set_census.py` exists, follows the two censuses' conventions,
  and takes the corpus in one command.
- **The factorisation claim is answered with a number, on every multi-model
  entry** — not with the reconnaissance's single case. If nothing factors, the
  phase README's "why it might already be free" paragraph is rewritten to say
  so, with the ratio.
- The coupling structure is reported, not just its existence: components,
  and for the one interesting entry what relates the variables.
- The leftover-open count is either measured or **declined with a reason of
  the same shape P1d.2's was** — a probe that changes the answer is not a
  probe.
- `model_set_census.md` banked, and the two claims [S1d.3.2](s1d.3.2_representations.md)
  needs are in it: what a compact form would have to represent, and what it
  would be competing against.
