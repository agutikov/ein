# S1d.2.5 — Hypotheses from obligations

**Phase:** [P1d.2](README.md) (Obligations)
**Estimate:** 3 days
**Depends on:** [S1d.2.4](s1d.2.4_obligations_in_the_saturator.md) (the tally
exists), [S1d.2.2](s1d.2.2_domains.md) (the open-extent inventory)
**Status: done 2026-08-25.** The ladder, the two fixtures, the completeness
test, and the enterings comparison — banked in
[`hypotheses_from_obligations.md`](hypotheses_from_obligations.md). See § What
it found; the headline is that **no counter moved**.

## What it found

| claim | asked for | measured |
|---|---|---|
| `zebra2-obligations` vs `zebra2` | same model, enterings compared | **every counter identical** — 101 enterings, 34 alive, 67 dead-post, 2 layers, 67 no-goods, 56 at layer 1, one model, the same fact set |
| `zebra2-minus-15-obligations` vs the 618 076 / 416 s baseline | all 32 models, enterings compared, `exhausted` honest | **identical at full depth**: 618 076 enterings, 598 955 alive, 19 121 clauses, 5 layers, 32 models, equal model sets, `exhausted = false` on both — 422.2 s against 429.9 s |
| the branch, against the blind generator | the milestone's central claim | **56 against 3 734** at layer 1 — 66.7×, and the blind arm does not finish |
| the choice heuristic | measured before either is defaulted | **inert, 0 difference on every counter**, and the reason is the traversal rather than the fixtures ([§4](hypotheses_from_obligations.md)) |
| the determinate corpus | identical, verdicts *and* counters | **identical.** 21 new exit cells and one modified (`render rules` on the one file whose text changed); of 8 081 shape digests, 90 are new, 42 are the `--hyp-stats` previews of the 21 programs that reach the rung, 36 are that same file, and **0** are anything else |
| the cost | `zebra -e` inside the P1a.6 baseline | **45.8 ms** against a `5b6feb8` build's 46.2; the rung is off for every program that declares no obligation rule |

**Two things the stage did not build the way the plan drew them**, both
recorded with their arguments in
[the record §1](hypotheses_from_obligations.md):

- **the rung proposes the union of the accepted obligations' candidates**, not
  one chosen obligation's. "Choose one and branch" is a *depth-first* move; the
  engine's search is a breadth-first lattice over root's `alive`, where layer 2
  is pairs drawn from layer 1's set — so one obligation's candidates alone make
  every model that needs a second requirement's arrow unreachable at every
  depth. The choice heuristic is built and measured anyway, because it is the
  interface a depth-first traversal needs on day one.
- **a declined obligation declines the whole call.** The domain contract's C4
  allows falling through "to another obligation"; that loses completeness
  silently, since the declined obligation's witnesses are then proposed by
  nobody. Declining wholesale — to the blind enumerator, narrated — is what
  makes the rung's exhaustiveness claim unconditional.

**And one thing it found that nothing asked for**: the pre-existing corpus
reaches the new rung's *generating* mode **nowhere**. 114 of the 156 loadable
files declare no obligation rule, 19 override with `:hrules`, 11 are *stuck*
(they owe, and `:no-hypothesis` names the relation they owe), 9 owe nothing,
and 1 declines. There was no third case — which is precisely why T1d.2.5.4
asked for a fixture, and why `06_blind_enumeration.ein` had to grow a second
obligation to keep exercising the blind enumerator at all: under the ladder, a
program that declares `(bijective …)` no longer reaches it.

## Context

The generator rung — the user's supersession decision
([`obligation_forms.md` § Superseding](obligation_forms.md), 2026-08-24)
executed, and
[Q-M1d.4](../open_questions.md#q-m1d4--may-an-obligation-driven-generator-change-the-traversal)
**spent here, explicitly**: this is the stage that changes the traversal,
the counters, the no-goods and the discovery order, and it re-baselines with
an argument instead of discovering the moves in a golden diff
(the [Q-M1a.18](../../../docs/history/m1a_rust/open_questions.md#q-m1a18--may-a-fork-stop-re-narrating-the-roots-fixpoint)
discipline).

The ladder, as decided — [design/07](../../../docs/history/m1a_rust/design/07_search_layer.md)'s
"hrule presence *is* the switch" grown one rung:

| the query has | hypotheses come from |
|---|---|
| `:hrules (…)` | the user's hrules — an override, exactly as today |
| no `:hrules`, undischarged obligations | **one chosen obligation's candidates** — branch on `{b : G(b), B neither present nor forbidden}`, mutually exclusive and jointly exhaustive at that node |
| no `:hrules`, owes nothing | the blind generator, for programs that state no obligations |

The invariants that survive any answer are S1a.7.0's: the *answer* depends on
neither entering order nor integration time. Everything else — `enterings_*`,
`layers_explored`, no-good content, model discovery order — is expected to
move, on purpose, once.

## Tasks

### Task T1d.2.5.1 — the rung

Candidate enumeration from a chosen obligation (per S1d.2.2's contract:
extent as of this quiescence), commitment = one candidate, siblings excluded
by construction. `:no-hypothesis` keeps its meaning on the new rung (a
relation listed there is not branched on even if owed — and a state owing
only such relations is *stuck*, reported, not silently complete).

### Task T1d.2.5.2 — the choice heuristic, measured

Which obligation to branch on. Fail-first (smallest candidate set) is the
default candidate; the alternative (first by rule order) is the control.
Measured on the fixtures below before either is defaulted — F9's rule: a
mechanism inert on the corpus is recorded as inert, with the number.

### Task T1d.2.5.3 — the completeness condition, as a test

The ladder is exhaustive **iff obligations + saturation determine every
remaining open fact**. On the zebra family the obligated arrows are the
decision variables, so: every model found by the hrule path is found by the
obligation path — pinned as a test, model sets compared. Where the condition
fails, a discharged consistent state carries leftover open facts — that state
is [P1d.3](../p1d.3_model_sets/README.md)'s compact-model-set territory and
[`ideas.md`](../ideas.md)'s closed-world completion; this stage *detects* and
reports it, and decides nothing about it.

### Task T1d.2.5.4 — the fixture that exercises the rung

Today's searching entries all carry `:hrules` (the override rung), so the new
rung needs its own: `examples/zebra2-obligations.ein` — zebra2 with the
`hrule guess` and the `:hrules` clause **deleted**, nothing else changed.
The theory alone drives the search: that file existing and solving is the
idea-block complaint ("not part of the theory") closed as a fixture. Corpus
entry, catalog line, goldens — the growth rule as usual. A
`zebra2-minus-15-obligations` twin gives the under-determined regime.

### Task T1d.2.5.5 — the re-baseline

The measured claim, in the stage's own record: `zebra2-obligations` vs
`zebra2` (same model, enterings compared), `zebra2-minus-15-obligations`
vs the [618 076 / 416 s baseline](../p1d.10_exhaustive_search/layer_census.md#4-zebra2-minus-15-all-five-layers)
(all 32 models, enterings compared, `exhausted` honest per S1d.2.2's
bounded-per-quiescence claim). Counters that move are listed with the
argument; goldens re-blessed once, in this stage's commit.

## Acceptance

- The ladder is the dispatch, `:hrules` untouched as override; determinate
  corpus identical (every entry with `:hrules` or no search takes the old
  paths bit-for-bit — verdicts *and* counters).
- The zebra-family completeness test passes: obligation-path model sets equal
  hrule-path model sets.
- `zebra2-obligations.ein` solves to the zebra2 model with **no hrule in the
  file**; the under-determined twin finds all 32.
- The enterings comparison is banked in this directory with the same rigour
  as the layer census — it is the milestone's central claim ("a requirement
  is a choice point") measured for the first time.
- Q-M1d.4 closed in [`open_questions.md`](../open_questions.md): decided by
  the user 2026-08-24, executed here, counters re-baselined in this commit.
