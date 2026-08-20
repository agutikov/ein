# Open Questions — M1d (From saturation to satisfiability)

Milestone-scoped questions. Ids are **sticky** — `Q-M1d.<n>`, in the style
[M1a](../m1a_rust/open_questions.md) uses for `Q-M1a.<n>` rather than the
global `Q<n>` sequence in [`plans/open_questions.md`](../open_questions.md).
A closed id is never reused.

**Q-M1d.1 arrived with [P1d.1](p1d.1_exhaustive_search/README.md)** on
2026-08-21, where it was Q-M1a.21; the M1a entry stays as a redirect. The rest
come from [`ideas.md`](ideas.md), the note that is the milestone's other half,
and they are the questions the note leaves open rather than the ones it
answers.

## Index

| Q | title | status |
|---|---|---|
| [Q-M1d.1](#q-m1d1--may-the-search-stop-before-the-lattice-is-exhausted) | May the search stop before the lattice is exhausted? | open — [P1d.1](p1d.1_exhaustive_search/README.md); `exhausted` keeps its meaning either way *(was Q-M1a.21)* |
| [Q-M1d.2](#q-m1d2--where-does-a-requirement-live) | Where does a requirement live — kernel, stdlib, or rule shape? | open — the note says **first-class obligation**; the cost is a kernel concept |
| [Q-M1d.3](#q-m1d3--what-closes-a-domain) | What closes a domain? | open — no answer, no lower bound; `is-a` extents and `open` are what exists |
| [Q-M1d.4](#q-m1d4--may-an-obligation-driven-generator-change-the-traversal) | May an obligation-driven generator change the traversal? | open — [Q-M1a.18](../m1a_rust/open_questions.md#q-m1a18--may-a-fork-stop-re-narrating-the-roots-fixpoint)-shaped; the answer moves every counter |
| [Q-M1d.5](#q-m1d5--print-or-describe) | 32 models: print or describe? | open — [P1d.3](p1d.3_model_sets/README.md); "enumerate, and say so" is an acceptable answer |

---

## Q-M1d.1 — May the search stop before the lattice is exhausted?

[P1d.1](p1d.1_exhaustive_search/README.md)'s question, and the measurement
that raises it: on `examples/zebra2-minus-15.ein` **every one of the 32 models
is found by depth 3, and depths 4–5 exist only to prove there are no more** —
which is where the run stops finishing.

So: is there an argument that lets the search stop early?

- **A sound criterion** proves the same thing sooner. It was in scope even
  under M1a's "no new reasoning features" non-goal, because it changes the
  cost of the proof and not the proof; here it is the phase's first prize.
- **A heuristic** ("no new model for k layers") changes the answer. It ships
  behind a flag, off by default, reporting `Ambiguity (not certified)` — and
  **never sets `exhausted = true`**. The word means the lattice was exhausted;
  a second guarantee needs a second word.

The candidates and their obligations are in
[S1d.1.3](p1d.1_exhaustive_search/s1d.1.3_stopping_criterion.md), and a
written refutation is as good an outcome as a proof — that is the discipline
[F9](../followups/f9_e_catalog.md) established for this exact area, and F9's
own judgements were all measured on puzzles with a unique model, which is the
regime this question is not about.

**Moved 2026-08-21 from Q-M1a.21**, with the phase. The one thing the move
adds: [P1d.2](p1d.2_obligations/README.md) is a fourth candidate the M1a
framing did not have — a state that knows what it still owes can recognise a
model *locally*, and an enumeration that branches on requirements is complete
at a depth bounded by the number of requirements rather than by
`max_set_size`. That is not yet a stopping criterion for the *model set*, and
[S1d.1.3](p1d.1_exhaustive_search/s1d.1.3_stopping_criterion.md) should say so
carefully; it is, however, the first candidate that attacks the exponent
instead of the constant.

## Q-M1d.2 — Where does a requirement live?

The note's headline is a design instruction: existence requirements are
**first-class obligations**, not generators of arrows. Three places that could
live, and they cost in different currencies:

- **(a) A derived fact.** `(owes R a {b1 b2 b3})` asserted by a rule, read by
  rules. Costs nothing structurally — it is what the stdlib already does with
  activators — and probably cannot carry a candidate set that shrinks, because
  the store is append-only and a narrowed set is a *new* fact each time.
- **(b) A kernel object.** The saturator tracks obligations beside the fact
  store, with an index that narrows in place. Buys the shrinking candidate set
  and the quiescence report; costs a new concept in the data model, in the
  `.einb` container, in the fork's copy-on-write layer, and in every
  renderer.
- **(c) A rule shape.** `forall` already quantifies over a domain and
  `domain-elimination` already forces a singleton. Perhaps the missing middle
  is expressible without a new object at all — in which case the phase is much
  smaller than it looks.

**No recommendation yet**, and that is deliberate: the choice depends on
[S1d.2.1](p1d.2_obligations/README.md)'s audit of what the rules already do
and on whether the candidate set has to be *stored* or can be *recomputed*.
The last one is a performance question with a measured precedent —
`_admit_from_boundary`'s re-query was 72 % of an exhaustive `zebra2` before
P1a.6 — so "recompute it" is not automatically cheap.

## Q-M1d.3 — What closes a domain?

`∀x ∈ D. ∃y ∈ C. R(x,y)` is unanswerable without knowing D and C. The note
lists the sub-questions: what is in the set, is the set closed, and may new
objects appear. Ein has `is-a` extents, `is-a*` for the transitive closure,
the `open` macro, and a corpus entry
([`features/04_open.ein`](../../examples/features/04_open.ein)) whose whole
point is that an open domain makes the search unbounded.

So the question is not "does Ein have domains" — it is **where the closure is
stated and who is allowed to rely on it**. A lower bound that quantifies over
a domain the puzzle never closed is either unenforceable or wrong, and the
engine has to say which at load time rather than at quiescence.

Related: the stdlib is deliberately **is-a-free in rule bodies** — the
hierarchy relation arrives as an activator parameter. An obligation that
hard-codes `is-a` would put a type system in the kernel, which
[S1.7.23](../m1a_rust/README.md) settled it would not have.

## Q-M1d.4 — May an obligation-driven generator change the traversal?

Generating hypotheses from an obligation's candidate set instead of from
`alive` produces branches that are mutually exclusive and jointly exhaustive —
a different traversal, therefore different `enterings_*`, different no-goods,
different `layers_explored`, and a different order of discovery for the models
themselves.

This is exactly the shape of
[Q-M1a.18](../m1a_rust/open_questions.md#q-m1a18--may-a-fork-stop-re-narrating-the-roots-fixpoint),
which had to be decided before a fork was allowed to narrate less, and of
[design/08](../m1a_rust/design/08_parallelism.md) §7, which rejected parallel
depth-first because "going depth-first changes which no-goods exist when, i.e.
the pruning, i.e. the counters".

The invariants that survive any answer are the ones
[S1a.7.0](../m1a_rust/p1a.7_parallelism/s1a.7.0_speculation_audit.md) already
pinned as tests: the *answer* depends on neither the entering order nor the
integration time. What is negotiable is everything that is not the answer, and
the phase has to say so explicitly rather than discover it in a golden diff.

## Q-M1d.5 — Print or describe?

If a puzzle has 32 models, is the answer 32 models or a description of them?
[P1d.3](p1d.3_model_sets/README.md) owns it, and the reason it is a question
rather than an obvious yes is that every consumer downstream reads *models*:
the trace, `:expect`, [M1b](../m1b_gui/README.md)'s views, and the benchmark
adapters that compare Ein's answer to Clingo's.

"Enumerate, and say so" is a legitimate answer. So is "report the factorisation
and enumerate on request". What is not legitimate is a compact form that only
the engine can read.
