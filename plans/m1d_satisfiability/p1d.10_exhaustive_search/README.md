# P1d.10 — Exhaustive search over many models

**Milestone:** [M1d — From saturation to satisfiability](../README.md)
**Estimate:** 3 weeks (15 days of stages)
**Id:** **P1d.10** since 2026-08-23 — P1d.1 before that, and M1a's P1a.12
before that.
**Runs last**, since 2026-08-24, and that is the first time the id and the
order have agreed ([§ Phases](../README.md#phases)). It ran *first* long enough
to take its census — [S1d.10.1](s1d.10.1_why_it_does_not_finish.md) is **done**,
2026-08-24, and § What the census settled is its result — and the four stages
that remain are all about a search that [P1d.2](../p1d.2_obligations/README.md)
is going to change. Answering them against today's traversal would be answering
them twice.
**Depends on:** [M1a](../../../docs/history/m1a_rust/README.md)'s
[P1a.7](../../../docs/history/m1a_rust/README.md#p1a7--parallelism) — cores change the
constant, not the exponent, and this phase is about the exponent. Knowing
which is which needs the parallel numbers first. **P1a.7 resumed 2026-08-22
and is two stages in**, neither of which produces a `--jobs` number — so this
is still a decision rather than a wait: either P1a.7 reaches
[S1a.7.5](../../../docs/history/m1a_rust/README.md#s1a75--the---jobs-contract)'s scaling
table first, or this phase starts without the parallel numbers and says so
where a reading would have used them.
**Was P1a.12; moved here 2026-08-21** at the user's direction, together with
the note that is the other half of its question ([`ideas.md`](../ideas.md),
ex-F14).

---

**The f14 analysis this file used to carry a TODO for is in the milestone
README** — [§ What the note says the engine is
missing](../README.md#what-the-note-says-the-engine-is-missing). Its bearing
on *this* phase is one sentence: the layer-by-layer powerset measured below is
what the engine does **because** it has no way to say that something is
*required*, and a requirement is a choice point, not a subset.
[P1d.2](../p1d.2_obligations/README.md) is that vocabulary; this phase
measures the regime first, and its census is what tells P1d.2 whether the
argument survives contact with the corpus.

## Goal

**Understand why an under-determined puzzle does not finish, and decide what
to do about it.** `examples/zebra2-minus-15.ein` is the case: the canonical
zebra2 with one condition removed, exhaustively solvable in principle,
uncompletable in practice.

## What is already measured

From the 2026-08-20 session that found the `disjunctive-prune` bug, with an
independent brute force as ground truth (control: restore condition (15), get
exactly one model, the canonical grid):

| depth cap | enterings | models found | wall |
|---|---:|---:|---:|
| `-m 1` | 96 | 0 | 24 ms |
| `-m 2` | 4 656 | 28 | 1.4 s |
| `-m 3` | 48 745 | **32 — all of them** | 25.3 s |
| `-m 5` (the default, i.e. `-e`) | — | — | **killed at 30 min** |

Ground truth: **32 models.** Three readings, and the third is the phase:

1. **Nothing prunes at layer 1.** All 96 candidates come back alive — no
   death, so no learned clause and no singleton `(not h)` writeback — and
   layer 2 is therefore the full `C(96,2)`. The `alive=96` a reader sees in the
   `-v` header is the count of live hypothesis *facts*, and it never shrinks,
   because nothing is ever refuted.
2. **Growth is ~11× a layer** and the wall clock with it: 96 → 4 656 → 48 745.
   Layers 4 and 5 are the run nobody sees finish.
3. **Every model is found by depth 3. Depths 4 and 5 exist only to certify
   that there are no more.** The cost is not *finding*, it is *proving there
   is nothing left* — and the engine's only proof of that is exhausting the
   lattice.

That third line is the phase's whole subject, and it was not visible before
this measurement.

## Why F9 does not already close this

[F9](../../followups/f9_e_catalog.md) rejected most of the search-optimisation
catalogue, and its cluster note is the reason to read it first:

> Re-judged against the engine's actual search — a *complete BFS over
> commitment-set cardinality* (Apriori), not a DPLL/DFS decision tree —
> reorderers are inert … A complete cardinality-BFS over a connected corpus
> leaves no purchase for any of them.

E10 (iterative deepening) is closed as "inapplicable — cardinality layering
already *is* breadth-first deepening". **Every one of those judgements was
measured on a puzzle with a unique model.** On zebra2, layer 1 kills **32 of
its 56** candidates and the pruning is what makes the search tractable; on
zebra2-minus-15 layer 1 kills nothing at all. Those are different regimes, and
F9 measured one of them.

> **That line read "67 of 101" until the census took it.** Both numbers are
> real and neither is layer 1's: an exhaustive `zebra2` is **101 enterings**
> over two layers, of which **67** die. Layer 1 is 56 of them and kills 32.
> The correction does not move the argument — 57 % against 66 % is the same
> regime — and it is here because [S1a.9.4](../../../docs/history/m1a_rust/README.md#s1a94--documentation)'s
> rule applies to a plan as much as to a page: a re-take that finds an error
> reports the error.

This is [S1a.6.4](../../../docs/history/m1a_rust/README.md#s1a64--hypgen-and-lattice-hot-paths)'s lesson
a third time — the phase had been measuring one shape of workload — so the
first stage here is a census, not a proposal.

## What the census settled — 2026-08-24

[S1d.10.1](s1d.10.1_why_it_does_not_finish.md) is **done**, and
[`layer_census.md`](layer_census.md) is the measurement. What it changes about
the phase above:

**The regime is the corpus, not the exception.** Layer 1 kills something in
**4 of the 49** entries that search at all — `zebra`, `zebra2`,
`zebra2-hints`, `branching/07`. The other **45** are barren, and they hold
2 189 278 of the 2 201 027 enterings. So "F9 measured one of two regimes" was
right and understated: it measured the **smaller** one, four cells wide.

**And "enumerates a powerset" is exact.** For 25 of the 49, `entered` equals
`Σₖ C(alive, k)` term for term — `features/01_not_and_absent` enters
`C(35, 1..5) = 384 167` — which is **96.7 %** of the corpus's search work.
Nothing died, nothing was learned, nothing was filtered.

**Reading 1 of § What is already measured needs a correction**, and it is the
useful kind. *"Nothing prunes at layer 1, so layer 2 is the full `C(96,2)`"* is
right, and the 0 % filter rate at layer 2 is **structural rather than
evidence**: a layer-1 death licenses a width-1 clause, and the singleton
writeback plus the inter-layer retain have already removed that element. The
clause store's first *possible* contribution is layer 3 — where it is **26.8 %**
— and at layer 4 it is **36.2 %**. The store is not inert; it is two layers
late, and by then the layer is 44 089 enterings.

**Reading 3 survives intact and is now cheap to check at depth.** The census row
is emitted on every way out of a layer, including a budget cut, so
`solve -e -m 4 -E 48746` generates layer 4 and reports it without entering it:
**245 612 joined, 88 887 clause-dropped, 156 725 candidates**, 103 s. Layers
1–4 are 205 471 enterings together, and the growth is decelerating
(`47.5× → 13.2× → 4.1×`) while the filter rate rises — which is a different
diagnosis from *"it never converges"* and is
[S1d.10.2](s1d.10.2_depth_required.md)'s to act on.

**One mechanism is inert and is now recorded as inert**, with the number F9's
discipline asks for: **0** of 2 232 330 joined candidates were dropped because
an element had left `alive`, and that is structural — the retain at the previous
barrier gets there first. **The clause store is the only thing that can shrink a
layer.**

**And a whole class of proposal is ruled out before anyone writes it.** The
profile of `zebra2-minus-15 -m 3` is the determinate mix at a larger count, not
a different one: match/bind 47.7 %, saturate 40.7 %, and the entire lattice —
the prefix join plus a filter that walks 11 577 clauses per candidate —
**1.2 %**. Making the lattice cheaper cannot help, because the lattice is not
the cost; the only lever with room behind it is **entering fewer commitments**.
That is the milestone's thesis arriving as a profile rather than as an argument,
and it is what [S1d.10.3](s1d.10.3_stopping_criterion.md) and
[S1d.10.4](s1d.10.4_conflict_mining.md) have to be judged against.

**`solve -e` finishes.** 618 076 enterings, **416 s**, all 32 models,
`exhausted=false` because the depth cap stopped it and not the lattice. The
phase's first acceptance bullet is met, on this engine, and what remains of it
is the second half of the same bullet — the *honest verdict*, which is
[S1d.10.5](s1d.10.5_contract.md)'s.

**And raising the cap buys nothing** — measured 2026-08-25 on the obligations
twin at `-m 10`: **10 587 736 enterings, 15 minutes, still 32 models**
([layer census §4.1](layer_census.md#41-the-depth-10-probe--2026-08-25-and-depths-610-add-nothing)).
Seventeen times the enterings for zero additional models, and the fraction of
the run spent past the last new model goes **92.1 % → 99.54 %** as the cap
moves 5 → 10. So the table above extends one row and the row is flat:

| depth cap | enterings | models found | wall |
|---|---:|---:|---:|
| `-m 10` | **10 587 736** | **32 — the same 32** | **905 s** |

`d_stop − d_found` is bounded by `-m` and by nothing the search knows, which
is the difference between an expensive search and a **missing termination
argument** — and is why [S1d.10.3](s1d.10.3_stopping_criterion.md) has to
produce a criterion rather than a better default.

## Stages

| stage | title | est. |
|---|---|---|
| [S1d.10.1](s1d.10.1_why_it_does_not_finish.md) | Why it does not finish — **done 2026-08-24** ([`layer_census.md`](layer_census.md)) | 3 d |
| [S1d.10.2](s1d.10.2_depth_required.md) | What depth is required, and for what | 2 d |
| [S1d.10.3](s1d.10.3_stopping_criterion.md) | Is there a stopping criterion? | 4 d |
| [S1d.10.4](s1d.10.4_conflict_mining.md) | Conflict mining when a layer is barren | 4 d |
| [S1d.10.5](s1d.10.5_contract.md) | What `exhausted` means | 2 d |

## Acceptance for the phase

- **`solve -e examples/zebra2-minus-15.ein` finishes**, with all 32 models and
  a stated exhaustion claim — or the phase records, with numbers, why it
  cannot and what the honest verdict is instead.
- The under-determined regime is a **named part of the measurement set**, the
  way [P1a.7](../../../docs/history/m1a_rust/README.md#p1a7--parallelism) had to re-aim its scaling target.
  One under-determined entry in the corpus is not a regime, it is an anecdote.
- **Nothing changes what the engine proves.** A sound criterion makes the same
  proof cheaper; an unsound one changes the answer. Anything in the second
  class ships behind a flag and reports a *different* verdict word — never a
  quiet `exhausted = true`.
- Every proposal is measured against F9's discipline: **a mechanism that is
  inert on the corpus is recorded as inert and not shipped**, with the number.
- The determinate puzzles do not regress: `zebra -e`, `zebra2 -e` and the
  P1a.6 targets hold their timings and their counters.

## Risks

- **Changing the traversal changes the counters.**
  [design/08](../../../docs/history/m1a_rust/design/08_parallelism.md) §7 rejected parallel depth-first for
  exactly this: "going depth-first changes which no-goods exist when, i.e. the
  pruning, i.e. the counters". The same is true of a sequential dive. This
  phase therefore needs the decision P1a.7 needed —
  [Q-M1a.18](../../../docs/history/m1a_rust/open_questions.md#q-m1a18--may-a-fork-stop-re-narrating-the-roots-fixpoint)'s
  shape — before anything ships on by default.
- **An unsound stopping rule is worse than a slow search.** "No new model for
  k layers, so stop" is a heuristic wearing a proof's clothes. If it ships it
  reports `Ambiguity (not certified)`, and the word `exhausted` stays false.
- **The line this phase used to sit on is now the milestone's.** Under M1a the
  rule was "anything that changes what the engine can prove belongs in a
  followup", and this phase was its named exception. In
  [M1d](../README.md) the distinction survives without the exception: a
  *sound* criterion proves the same thing sooner and is ordinary work here; a
  heuristic that changes the answer ships behind a flag with a different
  verdict word, or goes to [F4](../../followups/f4_cross_cutting.md). What the
  move does **not** relax is the second half —
  [S1d.10.5](s1d.10.5_contract.md) still owns the vocabulary, and `exhausted`
  still means the lattice was exhausted.
- **Memory before time.** An uncapped
  `saturation/square-unique/terminus.ein -e` reached 12.3 GB before being
  OOM-killed ([baseline.md §15](../../../docs/history/m1a_rust/measurements/baseline.md)).
  A deeper search may not get the chance to be slow. **The companion figure
  moved and this one has not been re-taken**:
  `features/01_not_and_absent -e` peaked at 724 MB and now peaks at
  **85–91 MB**, because
  [T1a.7.1.7](../../../docs/history/m1a_rust/README.md#s1a71--making-the-shared-state-sync)
  found most of it was a provenance arena nothing reclaimed until the run
  ended. Whether `terminus.ein`'s ~1 KB per entering was the same structure is
  unmeasured, so this bullet's *shape* survives its numbers — but re-measure
  before sizing anything by them.

## Cross-links

- [design/07 — Search layer](../../../docs/history/m1a_rust/design/07_search_layer.md)
- [F9 — the rejected search optimisations](../../followups/f9_e_catalog.md) —
  read before proposing anything here
- [`examples/zebra2-minus-15.ein`](../../../examples/zebra2-minus-15.ein) —
  the case, and `corpus/corpus.toml`'s note on why `solve -e` is not one
  of its runs
