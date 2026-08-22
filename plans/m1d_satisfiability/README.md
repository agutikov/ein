# M1d — From saturation to satisfiability

**Estimate:** ~2 months focused — 3 phases, 14 stages, ~8 weeks of stage
estimates. Only [P1d.1](p1d.1_exhaustive_search/README.md) is written to stage
depth; see § How deep this plan is.
**Status:** **created 2026-08-21** at the user's direction, out of two pieces
that were already in the repo and had never been put next to each other:
M1a's ex-P1a.12, now [P1d.1](p1d.1_exhaustive_search/README.md), and the
"saturation vs satisfiability" note that was followup F14, now
[`ideas.md`](ideas.md).
**Depends on:** [M1a](../m1a_rust/README.md) — the engine. Specifically the
search layer ([design/07](../m1a_rust/design/07_search_layer.md)) and the
speed P1a.6 bought, without which every experiment here costs a coffee break.
P1d.1 was written against [P1a.7](../m1a_rust/p1a.7_parallelism/README.md),
which is **paused after one stage**, so that dependency is now a decision
rather than a wait — see the phase.
**Blocks:** nothing on the critical path. [M1b](../m1b_gui/README.md) displays
whatever verdict vocabulary this milestone lands, so if the GUI ships first it
follows this rather than the other way round.

---

## The two halves of one question

**Operationally**, from [P1d.1](p1d.1_exhaustive_search/README.md): `examples/zebra2-minus-15.ein`
has 32 models, **every one of them is found by depth 3**, and the run does not
finish — because depths 4 and 5 exist only to prove there are no more.

| depth cap | enterings | models found | wall |
|---|---:|---:|---:|
| `-m 1` | 96 | 0 | 24 ms |
| `-m 2` | 4 656 | 28 | 1.4 s |
| `-m 3` | 48 745 | **32 — all of them** | 25.3 s |
| `-m 5` (`-e`) | — | — | **killed at 30 min** |

**Semantically**, from the note: saturation computes what *must follow*; it has
no vocabulary for what *must exist*. `bijective ≡ functional ∧ injective ∧
total ∧ surjective`, and only the first two are enforced in the sense their
name suggests.

The claim this milestone rests on is that these are the same fact seen from two
sides: **the engine enumerates a powerset because it has no way to say that
something is required, and a requirement is a choice point.**

## What the note says the engine is missing

[`ideas.md`](ideas.md) is the user's note, kept verbatim (RU). Its argument,
in the order it makes it:

1. **Upper bounds vs lower bounds.** `functional` and `injective` are `≤ 1`:
   they *forbid* a second arrow. `total` and `surjective` are `≥ 1`: they
   *require* a first one. A relation declared bijective in Ein gets the
   prohibitions with force and the requirements only in a degenerate form.
2. **A free slot is two different things** — an arrow that *may* still appear,
   and an arrow that *must*. The engine records the first: `alive` is exactly
   the set of arrows still open, recomputed between layers. Nothing anywhere
   records the second.
3. **Three states, and they are states of knowledge, not of the model.**
   `present` (true in every continuation), `forbidden` (false in every
   continuation), `open` (both continuations exist). Saturation is a
   computation over the three-valued partial state; satisfiability asks
   whether a two-valued completion exists.
4. **The general form** a constraint should take:
   `L ≤ #{ȳ | R(x̄, ȳ) ∧ φ(x̄, ȳ)} ≤ U`, with some arguments fixed and the
   rest quantified — of which `functional` (`U=1`), `total` (`L=1`) and
   `bijective` (`L=U=1` in both projections) are special cases, and of which
   "at least one of", "as many as", "an odd number of" are the cases nobody
   can state today.
5. **Three outcomes at a fixpoint, not one:** *contradiction* (`#present > U`,
   or `#present + #open < L` — the requirement is no longer reachable),
   *incomplete* (`#present < L ≤ #present + #open` — a witness must still be
   chosen), *complete* (every `L` met, no `U` violated).
6. **What that still does not buy.** Obligations alone are not a decision
   procedure: the engine also needs explicit quantification **domains** (what
   is in `D`, is `D` closed, may new objects appear), **witness choice** when
   several candidates remain, **backtracking**, and **termination
   guarantees** — finite domain, finite ground facts, no unbounded object
   creation.
7. **The headline, and it is a design instruction:** existence requirements
   should be **first-class obligations**, not generators of arrows. Saturation
   then narrows an obligation's candidate set; a single surviving candidate
   closes it automatically; an empty one is UNSAT; and only a set of two or
   more raises a hypothesis.

### Where the note needs sharpening against the code

The stdlib is further along than "the requirements are absent", and the
difference matters, because it is exactly the difference the note's headline
names.

| property | what the stdlib has | what it can say | what it cannot |
|---|---|---|---|
| `functional`, `injective` | `std.algebra`: two conflicting stored facts ⇒ `(false)` | "that second arrow is illegal" | — |
| `total`, `surjective` | `std.algebra`, in the **open-world-safe** formulation: `(forall ?b (?isa ?b ?B) (not (?R ?a ?b)))` ⇒ `(false)` | "**every** candidate is excluded, so this state is dead" | "one candidate is still missing, so this state is **unfinished**" |
| the one-candidate case | `std.elim` / `std.bijection`: `domain-elimination` / `range-elimination` assert the positive once all alternatives are excluded | "exactly one candidate left, so it holds" | — |

So Ein has the lower bounds **in their refutation form** and lacks them **in
their obligation form**. Both endpoints of the note's arithmetic are
implemented — `#present + #open = 0` is a contradiction, `= 1` forces the
witness — and the middle, `≥ 2`, is where nothing is recorded at all. That
middle is precisely where a search happens.

The consequence shows up in one line of
[design/07](../m1a_rust/design/07_search_layer.md): the engine's completeness
test is `complete(kb)` — *"does the generator propose anything?"*.
Completeness by **exhaustion of candidates**, never by **discharge of
requirements**. The two coincide when prohibitions alone pin every arrow; they
diverge exactly where a lower bound is the only thing that would force one.

## What that means for this engine

The engine's search is a **powerset ordered by cardinality**
([design/07](../m1a_rust/design/07_search_layer.md) §1): layer 1 is the
singletons of `alive`, layer *k* is an Apriori prefix-join over layer *k−1*,
filtered by the no-good store. A commitment is an arbitrary *set* of
hypothesis facts, because an arbitrary set is the only thing the engine knows
how to commit to.

An obligation is the missing structure: `1 ≤ #{h | (pet-loc Zebra h)} ≤ 1` over
five houses is **five alternatives, exactly one of which holds** — mutually
exclusive and jointly exhaustive. Branching on *that* partitions the space
five ways and is complete by construction at that node; a search over subsets
has to reach cardinality *k* before it can say the same thing, and on
`zebra2-minus-15` reaching cardinality *k* means generating from `C(96, k)`.

That is why the two halves are one milestone: **the exponent P1d.1 measures is
what the missing vocabulary costs.** It also explains the shape of P1d.1's
first finding — a layer that kills nothing learns nothing — because a
subset-lattice only prunes through death, whereas a choice point prunes by
construction: committing to one alternative excludes its four siblings without
anybody having to refute them.

### Two ways this argument is wrong

Written down now, so the phases test it rather than assume it:

1. **The declarations may already be enough.** `zebra2.ein` declares
   `(bijective …)` on every attribute relation, and `bijective-setup` fans that
   out into activators. If negative-completion plus `domain-elimination`
   already collapse most candidate sets to singletons before any hypothesis is
   raised, then obligations would buy structure the corpus does not need, and
   the win is much smaller than the arithmetic above suggests.
   [S1d.1.1](p1d.1_exhaustive_search/s1d.1.1_why_it_does_not_finish.md)'s
   census is what measures it.
2. **Branching on obligations changes the traversal, therefore the counters.**
   [design/08](../m1a_rust/design/08_parallelism.md) §7 rejected parallel
   depth-first on exactly that ground, and
   [Q-M1a.18](../m1a_rust/open_questions.md#q-m1a18--may-a-fork-stop-re-narrating-the-roots-fixpoint)
   is the shape of the decision it takes to change one anyway. Nothing here
   ships on-by-default without that decision being taken explicitly.

There is also a piece of external evidence available cheaply, and
[M1c](../m1c_external_validation/README.md) is already going to collect it:
ASP states this vocabulary natively — `1 { p(X) : q(X) } 1` *is* the note's
`L ≤ # ≤ U` — and Datalog does not. If Clingo expresses the zebra family
directly while Soufflé needs an extension or a generate-and-test encoding,
that is the note's thesis measured in someone else's language.

## Phases

| phase | title | stages | est. | gate |
|---|---|---|---|---|
| [P1d.1](p1d.1_exhaustive_search/README.md) | Exhaustive search over many models — why an under-determined puzzle does not finish | 5 | 3 w | `solve -e zebra2-minus-15` finishes with all 32 models, or the reason is measured |
| [P1d.2](p1d.2_obligations/README.md) | Obligations — the half of the vocabulary that says *must* | 6 | 3.5 w | a saturated state can report what it still owes, and a puzzle can state a requirement |
| [P1d.3](p1d.3_model_sets/README.md) | Model sets without enumeration — the compact answer | 3 | 1.5 w | either a compact representation of the 32 models, or a written argument for why enumeration is the answer |

14 stages, 40 days of stage estimates ≈ 8 weeks. **Order is not obvious and is
deliberate**: P1d.1 measures before P1d.2 designs, for
[S1a.6.1](../m1a_rust/p1a.6_performance/s1a.6.1_profile_baseline.md)'s and
[S1a.7.0](../m1a_rust/p1a.7_parallelism/s1a.7.0_speculation_audit.md)'s
reason — both phases found that the premise they were built on was wrong, and
found it in a stage that cost days rather than weeks. P1d.1's stopping-criterion
stage ([S1d.1.3](p1d.1_exhaustive_search/s1d.1.3_stopping_criterion.md)) may
well hand its answer forward to P1d.2 rather than finding one itself; that is a
legitimate outcome and it is why the census comes first.

## How deep this plan is

**P1d.1 is at stage depth** — five stage files, written when it was P1a.12,
moved unchanged. **P1d.2 and P1d.3 are phase READMEs only.** That is on
purpose: the note they come from opens with *"no code, no changes — here we
only read and discuss ideas"*, and turning a discussion into fifteen task ids
would put decisions in the plan that the user has not made. What is written is
the decomposition, the dependencies and the questions; the stage files are
written when the milestone starts.

## Acceptance for the milestone

- **`solve -e examples/zebra2-minus-15.ein` finishes** with all 32 models and
  a stated exhaustion claim — or the milestone records, with numbers, why it
  cannot and what the honest verdict is instead.
- **The engine can state a requirement.** At minimum `total` and `surjective`
  with the force their names claim; the general `L ≤ # ≤ U` form only if a
  corpus entry needs it.
- **A saturated state can report what it still owes** — outstanding
  obligations, with their live candidate sets. This is the deliverable that
  makes "is this a model or a stuck state?" a local question.
- **Nothing changes what the engine proves, silently.** A sound criterion
  proves the same thing sooner and ships. A heuristic ships behind a flag,
  reports a different verdict word, and never sets `exhausted = true`.
- **The determinate corpus does not regress** — `zebra -e`, `zebra2 -e` and
  P1a.6's targets hold their timings and their verdicts. Counters may move;
  the stage that moves them re-baselines with an argument.

## Non-goals

- **A solver back-end.** The answer to "saturation is missing existence
  constraints" is not "call Z3". M3 was dropped 2026-08-18 and this milestone
  does not reopen it; what it borrows from ASP and CP is *vocabulary*, not a
  process boundary. [M1c](../m1c_external_validation/README.md) measures the
  rivals; it does not invite them in.
- **A new search engine.** The cardinality-BFS stays until something measured
  replaces it, and [F9](../followups/f9_e_catalog.md)'s discipline applies to
  every proposal here: a mechanism that is inert on the corpus is recorded as
  inert, with the number, and not shipped.
- **A little constraint language.** The vocabulary grows one keyword at a time,
  each demanded by a puzzle that cannot be stated without it. `odd`,
  `same-count-as` and friends are in the note as *possibilities*; none of them
  enters the grammar without a corpus entry asking.
- **Object creation.** Every termination guarantee in the note depends on a
  finite domain and no fresh witnesses. If a requirement cannot be satisfied by
  an existing object, the state is UNSAT — inventing one is a different engine.
- **NL surface.** [M2](../m2_nl_to_ir/README.md) reads whatever the grammar
  ends up with; no part of this milestone is about how a requirement is said in
  English.

## Open questions

[`open_questions.md`](open_questions.md) — `Q-M1d.<n>`. **Q-M1d.1** (ex
Q-M1a.21, arrived with P1d.1) asks whether the search may stop before the
lattice is exhausted; Q-M1d.2 to Q-M1d.5 come from the note — where a
requirement lives (kernel or stdlib), what closes a domain, and whether an
obligation-driven generator changes the answer or only the path.

**[Q-M1d.6](open_questions.md#q-m1d6--may-contradiction-be-said-with-exhausted--false)
arrived 2026-08-22, from a release chore.** M1a
[S1a.9.0](../m1a_rust/p1a.9_release/s1a.9.0_slow_corpus.md) re-priced the
corpus's slow tail and found ten entries that cost the same exhaustively as on
the fast path, all of them reporting `Contradiction k=0` with
`exhausted=False` and `layers_explored == -m`: the search runs out of
commitment-set depth and says *"the constraints are contradictory"* anyway,
where the same engine answers `Aborted` — "budget cut, not proven" — for a
`-T` or `-E` budget. It is Q-M1d.1's neighbour: that one asks whether the
search may stop early, this one asks **what it is allowed to say when it
does**. [P1d.2](p1d.2_obligations/README.md) may dissolve it — a state that
knows what it owes can report *incomplete*, which is
[`ideas.md`](ideas.md)'s middle outcome and the word those ten entries
actually want.

## Cross-links

- [`ideas.md`](ideas.md) — the note, verbatim. Authoritative on intent
- [`stdlib/algebra.ein`](../../stdlib/algebra.ein),
  [`bijection.ein`](../../stdlib/bijection.ein),
  [`elim.ein`](../../stdlib/elim.ein) — where the halves that exist today live
- [design/06](../m1a_rust/design/06_saturation.md) ·
  [design/07](../m1a_rust/design/07_search_layer.md) — the loop and the search
  this milestone changes
- [M1c](../m1c_external_validation/README.md) — the sibling created the same
  day: [S1c.1.1](../m1c_external_validation/p1c.1_stdlib_conformance/s1c.1.1_what_the_stdlib_promises.md)'s
  promise inventory is P1d.2's first input, and the ASP/Datalog cells of
  [P1c.2](../m1c_external_validation/p1c.2_external_benchmarks/README.md) are
  external evidence for its premise
- [F9](../followups/f9_e_catalog.md) — the rejected search optimisations;
  read before proposing one
- [F4](../followups/f4_cross_cutting.md) — where a heuristic that changes the
  answer goes if it does not earn a flag here
