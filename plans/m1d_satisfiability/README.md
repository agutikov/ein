# M1d — From saturation to satisfiability

**Estimate:** ~2.5 months focused — 4 phases, 17 stages, ~9.5 weeks of stage
estimates. Only [P1d.10](p1d.10_exhaustive_search/README.md) is written to stage
depth; see § How deep this plan is.
**S1d.10.1 is done and P1d.10 moved to the end** (2026-08-24): the census the
milestone opened with is [taken](p1d.10_exhaustive_search/layer_census.md), and
what it found — 96.7 % of the corpus's search work is an *exact* powerset walk —
is [P1d.2](p1d.2_obligations/README.md)'s input rather than a question P1d.10
can answer on its own. See [§ Phases](#phases).
**P1d.4 arrived 2026-08-24**, from M1c: building
[`:expect`](../../docs/history/m1c_external_validation/README.md#s1c12--how-a-program-states-what-it-expects)
produced a form that can *state* "these are all the models" and can only
*verify* it by exhausting a search that does not finish — which is this
milestone's opening measurement, met from the other side.
**Status:** **created 2026-08-21** at the user's direction, out of two pieces
that were already in the repo and had never been put next to each other:
M1a's ex-P1a.12, now [P1d.10](p1d.10_exhaustive_search/README.md), and the
"saturation vs satisfiability" note that was followup F14, now
[`ideas.md`](ideas.md).
**Depends on:** [M1a](../../docs/history/m1a_rust/README.md) — the engine. Specifically the
search layer ([design/07](../../docs/history/m1a_rust/design/07_search_layer.md)) and the
speed P1a.6 bought, without which every experiment here costs a coffee break.
P1d.10 was written against [P1a.7](../../docs/history/m1a_rust/README.md#p1a7--parallelism),
which is **paused after one stage**, so that dependency is now a decision
rather than a wait — see the phase.
**Blocks:** nothing on the critical path. [M20](../m20_gui/README.md) displays
whatever verdict vocabulary this milestone lands, so if the GUI ships first it
follows this rather than the other way round.

---

## The two halves of one question

**Operationally**, from [P1d.10](p1d.10_exhaustive_search/README.md): `examples/zebra2-minus-15.ein`
has 32 models, **every one of them is found by depth 3**, and the run does not
finish — because depths 4 and 5 exist only to prove there are no more.

| depth cap | enterings | models found | wall |
|---|---:|---:|---:|
| `-m 1` | 96 | 0 | 24 ms |
| `-m 2` | 4 656 | 28 | 1.4 s |
| `-m 3` | 48 745 | **32 — all of them** | 25.3 s |
| `-m 4` | 205 470 | 32 | — |
| `-m 5` (`-e`) | **618 076** | **32** | **416 s** — it finishes |
| **`-m 38`** | **17 204 592** | **32** | **1 496 s** at `-j16` — and it **exhausts** |

> **It finishes, and the number was predicted before it was run.** S1d.10.1's
> census row is emitted on every way out of a layer, so `-E` stops a run
> *after* the next layer is generated and the row still reports what the
> generation proposed: layers 4 and 5 came back at **156 725** and **412 606**
> candidates from two budget probes, summing to 618 076 — which is exactly what
> the full `solve -e` then entered, in **6 min 56 s**
> ([`layer_census.md` §4](p1d.10_exhaustive_search/layer_census.md#4-zebra2-minus-15-all-five-layers)).
> The "killed at 30 min" of 2026-08-20 stands as a record of that session and
> not of this engine.
>
> **What it reports is `Ambiguity k=32, exhausted=false`** — the frontier at
> depth 5 is not empty, so the cap stopped it, not the lattice. Every model was
> found at depth 3 and **569 331 of the 618 076 enterings — 92.1 % — happen
> after the last new model.** That gap is the milestone, stated in one run:
> finding is cheap, *proving there is nothing left* is the whole cost, and
> `exhausted` is still false at the end of seven minutes.
>
> **And at `-m 38` it is true — measured 2026-08-26.** The obligations twin at
> a cap deep enough not to bind stops at **22 layers with the frontier empty**,
> which is the lattice ending rather than the budget: `k = 32`, `exhausted =
> true`, **17 204 592** enterings (27.8× the depth-5 run) in **24 min 56 s** on
> sixteen threads. So the milestone's first acceptance bullet is **met** —
> `solve -e` finishes with all 32 models and a stated exhaustion claim — and
> the sentence above keeps its point rather than losing it: the extra 16.6 M
> enterings buy **no new model**, only the proof — and they kill **eight
> commitments**. `dead_post` is 19 121 at depth 5 and 19 129 at depth 22, with
> `dead_pre` 0 throughout, so
> [S1d.10.1](p1d.10_exhaustive_search/s1d.10.1_why_it_does_not_finish.md)'s
> *a layer that kills nothing learns nothing* holds to the end of the lattice:
> 16 586 516 enterings, 8 deaths.

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
[design/07](../../docs/history/m1a_rust/design/07_search_layer.md): the engine's completeness
test is `complete(kb)` — *"does the generator propose anything?"*.
Completeness by **exhaustion of candidates**, never by **discharge of
requirements**. The two coincide when prohibitions alone pin every arrow; they
diverge exactly where a lower bound is the only thing that would force one.

## What that means for this engine

The engine's search is a **powerset ordered by cardinality**
([design/07](../../docs/history/m1a_rust/design/07_search_layer.md) §1): layer 1 is the
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

That is why the two halves are one milestone: **the exponent P1d.10 measures is
what the missing vocabulary costs.** It also explains the shape of P1d.10's
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
   [S1d.10.1](p1d.10_exhaustive_search/s1d.10.1_why_it_does_not_finish.md)'s
   census is what measures it.

   **Measured 2026-08-24 — and the answer is no, by a wide margin**
   ([`layer_census.md`](p1d.10_exhaustive_search/layer_census.md)). The
   declarations are enough on **4** of the 49 corpus entries that search at
   all — `zebra`, `zebra2`, `zebra2-hints`, `branching/07`, which are the four
   the engine was tuned on. On the other **45** layer 1 kills nothing, and on
   **25** the number of commitments entered is *exactly* `Σₖ C(alive, k)`:
   `features/01_not_and_absent -e` enters `C(35, 1..5) = 384 167`, term for
   term. Those 25 cells are **96.7 %** of the corpus's search work. The
   powerset in § What that means for this engine is not an upper bound or a
   worst case — it is what 96.7 % of the search *is*.
2. **Branching on obligations changes the traversal, therefore the counters.**
   [design/08](../../docs/history/m1a_rust/design/08_parallelism.md) §7 rejected parallel
   depth-first on exactly that ground, and
   [Q-M1a.18](../../docs/history/m1a_rust/open_questions.md#q-m1a18--may-a-fork-stop-re-narrating-the-roots-fixpoint)
   is the shape of the decision it takes to change one anyway. Nothing here
   ships on-by-default without that decision being taken explicitly.

There is also a piece of external evidence available cheaply, and
[M10](../m10_external_benchmarks/README.md) is already going to collect it:
ASP states this vocabulary natively — `1 { p(X) : q(X) } 1` *is* the note's
`L ≤ # ≤ U` — and Datalog does not. If Clingo expresses the zebra family
directly while Soufflé needs an extension or a generate-and-test encoding,
that is the note's thesis measured in someone else's language.

## Phases

| phase | title | stages | est. | gate |
|---|---|---|---|---|
| [P1d.2](p1d.2_obligations/README.md) | Obligations — the half of the vocabulary that says *must* | 6 (**done 2026-08-25**) | 3.5 w | **met**: a puzzle states a requirement, a state says what it owes, the search branches on it, and the verdict reports it — [the phase ledger](p1d.2_obligations/README.md) |
| [P1d.3](p1d.3_model_sets/README.md) | Model sets without enumeration — the compact answer | 3 (**done 2026-08-26**) | 1.5 w | **met, and with both**: `ein solve --models key` is the compact representation, and the enumeration now states its own guarantee — [the phase ledger](p1d.3_model_sets/README.md#the-ledger) |
| [P1d.4](p1d.4_model_set_closure/README.md) | Closing the model set — the claim nothing can state | 3 (**done 2026-08-26**) | 1.5 w | **met, and with no keyword**: the answer is *no* three times over, and M1c's sentence is rewritten — [the phase ledger](p1d.4_model_set_closure/README.md#the-ledger) |
| [P1d.10](p1d.10_exhaustive_search/README.md) | Exhaustive search over many models — why an under-determined puzzle does not finish | 5 (1 done) | 3 w | `solve -e zebra2-minus-15` finishes with all 32 models, or the reason is measured |

17 stages, 47 days of stage estimates ≈ 9.5 weeks — and **the table is now in
id order, because the work order caught up with it.**

**P1d.10 ran first and now runs last**, moved 2026-08-24 at the user's
direction, and the two facts are the same fact. It came first for
[S1a.6.1](../../docs/history/m1a_rust/README.md#s1a61--fresh-profile-and-bench-baseline)'s and
[S1a.7.0](../../docs/history/m1a_rust/README.md#s1a70--the-speculation-audit)'s
reason — measure before designing, because both of those phases found their
premise wrong in a stage that cost days rather than weeks. **It did exactly
that and then it was done**:
[S1d.10.1](p1d.10_exhaustive_search/s1d.10.1_why_it_does_not_finish.md)'s
census ([`layer_census.md`](p1d.10_exhaustive_search/layer_census.md),
2026-08-24) is the measurement the rest of the milestone needed, and it is
taken.

What is left of P1d.10 — the depth accounting, the stopping criterion, conflict
mining, the `exhausted` contract — is **about the search obligations are going
to change**, and answering those questions against today's traversal would be
answering them twice.
[S1d.10.3](p1d.10_exhaustive_search/s1d.10.3_stopping_criterion.md)'s own text
already leaned that way ("may well hand its answer forward to P1d.2 rather than
finding one itself"); the census settled it, because two of its three candidates
are decided by numbers P1d.2 will move: (b) depends on `alive` shrinking, and
`alive` shrinks in **3 of 46** multi-layer cells today — an obligation-driven
generator is the one thing that would change that.

> **And that prediction now has an answer, which is *not by itself*.**
> [S1d.2.5](p1d.2_obligations/s1d.2.5_hypotheses_from_obligations.md) shipped
> the obligation-driven generator on 2026-08-25, and on the zebra family its
> `alive` **is** the hrule path's `alive` — the same 56 at layer 1, the same 23
> at layer 2, the same 618 076 enterings at depth 5. What it changes is the
> *floor*: against the blind enumerator on the same file, layer 1 is 56 against
> 3 734. So the rung buys a search where a puzzle would otherwise have had to
> hand-write one, and buys nothing where the puzzle already did. What is still
> unspent is the **traversal** — the rung proposes the union of the owed
> instances because a breadth-first lattice over a fixed `alive` cannot take a
> per-node branch, and the choice heuristic that a depth-first one would need
> is built and
> [measured inert](p1d.2_obligations/hypotheses_from_obligations.md) waiting
> for it. That is the shape of what P1d.10 inherits, and it is a smaller
> inheritance than this paragraph expected.

## How deep this plan is

**P1d.10 is at stage depth** — five stage files, written when it was P1a.12,
moved unchanged then and renumbered (not rewritten) on 2026-08-23. **P1d.2
reached stage depth on 2026-08-24**: six stage files, written *after* the
user took the decisions the phase README had reserved — the form (G), the
naming (P3, probe rename executed), numeral-free bounds, the supersession
ladder — all recorded on
[`obligation_forms.md`](p1d.2_obligations/obligation_forms.md). **P1d.3 reached stage depth on 2026-08-25**, three stage files, and it did so
the way this section said a phase should: not by turning a discussion into task
ids, but because a **measurement** made the decisions concrete. The
reconnaissance over `zebra2-minus-15`'s 32 models found they do not factor at
any granularity — one coupling component of all 23 varying decision variables
— which falsifies the phase README's central hope and leaves the phase with
real work instead of a by-product
([P1d.3 § Why it is not free](p1d.3_model_sets/README.md)). The decisions the
stages still reserve for the user are named as such: whether anything ships,
and whether closed-world completion is adopted.

> **S1d.3.1 landed 2026-08-25 and the shape it predicted held**: the
> reconnaissance's three granularities became four over 13 entries rather than
> one, and the extra one is where the answer changed sign. The product form
> *does* exist — on `saturation/type-exclusivity/{colors,nationalities}`,
> two-fact demos whose 9 models are 3 × 3 over two independent blocks — and the
> same program with a third instance has one component and 35 models. So the
> free-by-product path closes between two objects and three, which is a sharper
> statement than "it never happens" and a worse one for shipping anything.
> `zebra2-minus-15`'s coupling graph is K₂₃ minus five edges with a minimum
> vertex separator of 17, which is what prices a decision diagram
> ([`model_set_census.md`](p1d.3_model_sets/model_set_census.md)). The stage
> also **took** the leftover-open probe P1d.2 handed forward rather than
> declining it again: run on a discarded fork it is a read, and `zebra2`'s
> unique model leaves **3 678** facts the blind enumerator would still propose
> — none of them an attribute arrow, and most of them ill-typed, because the
> kernel imposes no type system. That is
> [S1d.3.3](p1d.3_model_sets/s1d.3.3_the_verdict.md)'s closed-world question
> with a number on it.
>
> **S1d.3.3 closed the phase on 2026-08-26, and the half that was broken was
> the one nobody had put on the ballot.** The user's answer to Q-M1d.5 was
> **both** — ship (b), *and* make the enumeration honest — and writing the
> guarantee rule down found that it was not: `ein solve -e
> examples/saturation/type-exclusivity/colors.ein` printed `solutions (k) 5`
> for a file with **nine** models, and **5 of the 10** corpus entries that
> answer `Ambiguity` under their declared runs do it with `exhausted = false`.
> `Solution` has hedged its `k = 1` since ein.py; the verdict whose `k` is a
> *number* hedged nothing. What ships is that qualifier on three surfaces and
> `--models key` as additional output — 49 lines against 516 on
> `zebra2-minus-15`, the shipped table identical to the census's on all 32
> rows, and a golden diff of **5 of 8 171 renderings**, every one a
> `trace[answer]` on an `Ambiguity` entry with no line added or removed
> ([`the_verdict.md`](p1d.3_model_sets/the_verdict.md)). The semantics question
> P1d.2 handed forward is **stated and not adopted**: a reported model is a
> *state*, `k` counts states, and closure is per relation and opt-in — which
> the language already has twice, in `:expect`'s *naming a relation closes it*
> and in the obligation domain contract.
>
> **S1d.3.2 followed the same day and reversed its own prediction.** It called
> the certain core *"the candidate to beat"*; printed, that core turns out to
> be 2 facts of answer and 338 of scaffolding, it cannot say how many models
> there are, and the arithmetic it invites over-states by 3.11 × 10¹². The
> determining key wins instead — 2 506 bytes, 72 columns, and **exact in the
> operational sense**: every one of the 32 key rows reconstructs its model to
> the fact, 30 of them by saturation alone with no commitment entered
> ([`representations.md`](p1d.3_model_sets/representations.md)). A decision
> diagram is priced out by `k = 32` rather than by the coupling — bounded in
> [24, 737] nodes under any variable order — and the disjunctive store is
> deferred with a three-clause trip-wire the corpus does not trip.

**P1d.4 reached stage depth on 2026-08-25**, the same day and the same way:
three stage files, written because a reconnaissance replaced an assumption with
a number. Its finding is that the closure claim `:expect (or …)` is written
**twice in the whole corpus**, both feature demos, and that
`examples/zebra2-minus-15.ein` — the one puzzle
[M1c's thesis](../../docs/history/m1c_external_validation/README.md#splitting-them-did-not-split-the-pipeline)
names — carries no expectation at all
([P1d.4 § What the corpus says](p1d.4_model_set_closure/README.md)). The debt
is not merely unverifiable; it is unwritten, which is a cheaper problem and a
different one.

> **S1d.4.1 took that reconnaissance apart on 2026-08-26, and the way it did is
> the discipline this section is about.** The stage's own T1d.4.1.1 demanded a
> census *parsed rather than grepped* — "a grep cannot tell a keyword from a
> comment about one" — and nothing in the engine could parse it, so the stage
> added `ein test --json-report`, one row per `(query …)` of a selection. The
> first thing it printed contradicted the paragraph above: the closure claim is
> written **once**, because `examples/features/10_expect.ein`'s `(or …)` is
> line 12 of its *header comment* and its `:expect` is a `(model …)`. **59 of
> 124 queries** state a claim, **1 of 124** states one about a set, and 59 of
> 59 hold under a search that exhausted
> ([`closure_census.md`](p1d.4_model_set_closure/closure_census.md)).
>
> Two numbers it added that nobody had: the claim's **write** cost, which is
> `k × |goal extent| / |file|` and is worst not on the puzzle but on a
> 95-line feature demo — `branching/06_lookahead_on` at **4.28×**, against the
> zebra's 0.96× — and the **counterfactual `NOT CHECKED` set**, which is what
> the empty column in the verifiability table actually means: **10 of the 121
> entries that reach a fixpoint** do not exhaust at `ein test`'s depth, so a
> closure claim on any of them would come back unchecked. `NOT CHECKED` never
> fires in the corpus because the entries where it would have have never
> written a claim.
>
> **S1d.4.2 and S1d.4.3 closed the phase the same day, and it grew nothing.**
> The rule-shape test that answered Q-M1d.2 *yes* one level down answers
> Q-M1d.7 **no** three times over, and the third refusal is the one the plan
> did not have: a verdict atom `(closed)` fails on **evaluability**, because
> `(open ?R)` costs one pass over the quiescent KB and `(closed)` costs the
> whole lattice — so the affordability problem is not downstream of the
> vocabulary, it is *why the vocabulary cannot exist*
> ([`the_boundary.md`](p1d.4_model_set_closure/the_boundary.md); Alloy is the
> nearest miss, and `run … for 5` is ein's `--max-set-size`, not ein's
> `:expect`). The user then declined all three candidate vocabularies, and
> S1d.4.3 measured the one that had a number in it: the obligation-derived
> bound is **1.244 × 10¹⁴ against 32** — and *absent* on **10 of the 12**
> entries that have a model set, because they state no obligation at all.
> What shipped is one line of stderr, which is what let
> `examples/features/11_expect_ambiguity.ein` declare the plain `solve` run
> that finally gives `Outcome::NotChecked` a corpus cell.

The caution that kept these two phases at README depth still holds and is
worth restating: the note they come from opens with *"no code, no changes —
here we only read and discuss ideas"*, and turning a discussion into task ids
would put decisions in the plan that the user has not made. What unlocked both
was **measurement, not decision** — the stage files record what is now known
and name the decisions still reserved: whether P1d.3 ships a representation,
whether closed-world completion is adopted, and whether P1d.4 grows a
keyword.

> **All three were taken on 2026-08-26 and the discipline held.**
> P1d.3 **does** ship a representation — the user chose *both*, so
> `--models key` is additional output beside a qualified count — and
> closed-world completion is **not** adopted: S1d.3.3 states what the engine
> means today (a reported model is a *state*, `k` counts states, closure is
> per relation and opt-in) so that adopting it later is a diff against a
> written specification rather than an argument from scratch. And the third:
> **P1d.4 does not grow a keyword.** Tests stay exhaustive by default,
> `:expect` stays closed by default, there is no extra syntax, and a claim too
> slow to check at the runner's depth stays out of the corpus — which the gate
> already enforces, because `NOT CHECKED` takes a failing exit code. What is
> left is not a vocabulary problem but a *search* problem, and it is
> [P1d.10](p1d.10_exhaustive_search/README.md)'s.

## Acceptance for the milestone

- **`solve -e examples/zebra2-minus-15.ein` finishes** with all 32 models and
  a stated exhaustion claim — or the milestone records, with numbers, why it
  cannot and what the honest verdict is instead.
  **Met 2026-08-26**, on the obligations twin at `-m 38`: `k = 32`,
  `exhausted = true`, 17 204 592 enterings, 22 layers, 24 min 56 s at `-j16`
  ([§ The two halves of one question](#the-two-halves-of-one-question)). What
  is *not* settled is the cost — 92 % of the run is proof — which is
  [P1d.10](p1d.10_exhaustive_search/README.md)'s remaining four stages.
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
  process boundary. [M10](../m10_external_benchmarks/README.md) measures the
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
Q-M1a.21, arrived with P1d.10) asks whether the search may stop before the
lattice is exhausted; Q-M1d.2 to Q-M1d.5 come from the note — where a
requirement lives (kernel or stdlib), what closes a domain, and whether an
obligation-driven generator changes the answer or only the path.

**[Q-M1d.6](open_questions.md#q-m1d6--may-contradiction-be-said-with-exhausted--false)
arrived 2026-08-22, from a release chore.** M1a
[S1a.9.0](../../docs/history/m1a_rust/README.md#s1a90--the-slow-corpus-re-priced) re-priced the
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

> **It did dissolve it, 2026-08-25, and one step further along than this
> paragraph guessed.** The word exists — `Open — owes n`,
> [S1d.2.6](p1d.2_obligations/s1d.2.6_verdicts_counters_corpus.md) — and
> twelve corpus entries now report it. **The ten are not among them.** Measured
> before anything moved, all ten declare zero obligation rules, so all ten are
> out of the read-out's scope and keep `Contradiction`
> ([the census §5](p1d.2_obligations/openness_census.md)). The word those ten
> entries "actually want" turns out to be one they have to *ask* for: a
> requirement is something a program states, and none of them states one.
> What is left of the question is the depth cap wearing a refutation's word,
> which is [Q-M1d.1](open_questions.md#q-m1d1--may-the-search-stop-before-the-lattice-is-exhausted)'s
> and [P1d.10](p1d.10_exhaustive_search/README.md)'s.

## Cross-links

- [`ideas.md`](ideas.md) — the note, verbatim. Authoritative on intent
- [`stdlib/algebra.ein`](../../stdlib/algebra.ein),
  [`bijection.ein`](../../stdlib/bijection.ein),
  [`elim.ein`](../../stdlib/elim.ein) — where the halves that exist today live
- [design/06](../../docs/history/m1a_rust/design/06_saturation.md) ·
  [design/07](../../docs/history/m1a_rust/design/07_search_layer.md) — the loop and the search
  this milestone changes
- [M1c](../../docs/history/m1c_external_validation/README.md) — the sibling created the same
  day: [S1c.1.1](../../docs/history/m1c_external_validation/README.md#s1c11--what-the-stdlib-promises-and-what-is-exercised)'s
  promise inventory is P1d.2's first input, and the ASP/Datalog cells of
  [M10](../m10_external_benchmarks/README.md) are
  external evidence for its premise
- [F9](../followups/f9_e_catalog.md) — the rejected search optimisations;
  read before proposing one
- [F4](../followups/f4_cross_cutting.md) — where a heuristic that changes the
  answer goes if it does not earn a flag here
