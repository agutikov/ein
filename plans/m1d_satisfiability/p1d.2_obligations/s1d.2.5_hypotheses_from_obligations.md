# S1d.2.5 — Hypotheses from obligations

**Phase:** [P1d.2](README.md) (Obligations)
**Estimate:** 3 days
**Depends on:** [S1d.2.4](s1d.2.4_obligations_in_the_saturator.md) (the tally
exists), [S1d.2.2](s1d.2.2_domains.md) (the open-extent inventory)

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
