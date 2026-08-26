# P1d.10 — Exhaustive search over many models

**Status: CLOSED 2026-08-27**, as it stood, at the user's direction — three of
six stages shipped and the rest dropped rather than deferred.
[§ The ledger](#the-ledger--closed-2026-08-27) is what it bought and what it
left, including eight measurements with no owner.
**Milestone:** [M1d — From saturation to satisfiability](../README.md)
**Estimate:** 3 weeks — 15 days of stages when it was written, **14.5 across
six** after the 2026-08-26 reshape, of which 3 are spent
**Id:** **P1d.10** since 2026-08-23 — P1d.1 before that, and M1a's P1a.12
before that.
**Runs last**, since 2026-08-24, and that is the first time the id and the
order have agreed ([§ Phases](../README.md#phases)). It ran *first* long enough
to take its census — [S1d.10.1](s1d.10.1_why_it_does_not_finish.md) is **done**,
2026-08-24, and § What the census settled is its result — and the four stages
that remain were held because they are all about a search that
[P1d.2](../p1d.2_obligations/README.md) was going to change. Answering them
against that traversal would have been answering them twice.
**Begun 2026-08-26**, with the other three phases shipped and the reconnaissance
in § What the reconnaissance found. What it changes: the phase's headline
question is **answered** — `solve -e` finishes and exhausts — so what is left is
not *whether* the proof can be had but *what it costs*, and the cost has a
measured alternative that is five orders of magnitude cheaper. Two of the four
stages had a premise that is now false, and one of them is half-shipped by a
phase that ran after it was written.
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

> **It finishes, as of 2026-08-26, so the goal has moved one clause along.**
> `-m 38` exhausts the lattice in 24 min 56 s (§ What is already measured), and
> the question that is left is the one that sentence was hiding: **what does
> the proof cost, and is there a cheaper argument for the same claim?** Every
> number in this phase is now a ratio against 17 204 592 enterings rather than
> against a run nobody had seen end.

## What is already measured

From the 2026-08-20 session that found the `disjunctive-prune` bug, with an
independent brute force as ground truth (control: restore condition (15), get
exactly one model, the canonical grid):

| depth cap | enterings | models found | wall |
|---|---:|---:|---:|
| `-m 1` | 96 | 0 | 24 ms |
| `-m 2` | 4 656 | 28 | 1.4 s |
| `-m 3` | 48 745 | **32 — all of them** | 25.3 s |
| `-m 5` (the default, i.e. `-e`) | 618 076 | 32 | **416 s** — it finishes, `exhausted = false` |
| **`-m 38`** | **17 204 592** | **32** | **1 496 s** at `-j16` — and **`exhausted = true`** |

The last two rows are later than the session above — `-m 5` from
[S1d.10.1](s1d.10.1_why_it_does_not_finish.md)'s census (2026-08-24, and the
"killed at 30 min" this row used to carry was a record of the 2026-08-20
session rather than of this engine), and `-m 38` from 2026-08-26, on
`examples/zebra2-minus-15-obligations.ein`.

> **`-m 38` is the phase's headline question answered, and the answer is that
> it takes 25 minutes.** The cap was 38 and the search stopped at **22 layers
> with the frontier empty**, so it is the lattice that ended and not the
> budget. `k` does not move — all 32 models are still found by depth 3, and the
> extra **16.6 M** enterings buy nothing but the proof.
>
> **And the deaths are eight.** The same file at `-m 5` reproduces this
> census's counters to the digit — 618 076 enterings, `dead_post` 19 121,
> `dead_pre` 0 — so with `dead_post` at **19 129** after 22 layers, layers 6
> through 22 kill **8** of **16 586 516** enterings. `dead_pre` is 0
> throughout: not one candidate is dropped by the no-good store before
> entering, at any depth. *A layer that kills nothing learns nothing*, carried
> to the end of the lattice.
>
> (Both runs are `-j16`, and the `--jobs` contract holds at this scale: 44 s
> against the census's 416 s at `--jobs 1`, with every counter identical.
> **Re-taken 2026-08-26 the `-m 5 -j16` row is 50 181 ms**, 14 % above the 44 s
> recorded here — same binary, same file, same counters to the digit. The
> difference is machine state rather than engine, and the numbers in
> § What the reconnaissance found §5b all come from the re-take so that the
> ratios in it are taken within one minute of each other.)
>
> So [the milestone's first acceptance bullet](../README.md#acceptance-for-the-milestone)
> is **met**, and what is left for this phase is unchanged and sharper: not
> *whether* it finishes but *why it costs 17 M enterings to prove what 48 745
> found*. [S1d.10.2](s1d.10.2_depth_required.md) and
> [S1d.10.3](s1d.10.3_stopping_criterion.md) are that question, and they now
> have a terminating run to measure against instead of a truncated one.

Ground truth: **32 models.** Three readings, and the third is the phase:

1. **Nothing prunes at layer 1.** All 96 candidates come back alive — no
   death, so no learned clause and no singleton `(not h)` writeback — and
   layer 2 is therefore the full `C(96,2)`. The `alive=96` a reader sees in the
   `-v` header is the count of live hypothesis *facts*, and it never shrinks,
   because nothing is ever refuted.
2. **Growth is ~11× a layer** and the wall clock with it: 96 → 4 656 → 48 745.
   Layers 4 and 5 were the run nobody had seen finish; **layer 22 is where it
   ends**, and the whole of layers 4–22 is 17.2 M enterings.
3. **Every model is found by depth 3. Every layer after it exists only to
   certify that there are no more.** The cost is not *finding*, it is *proving
   there is nothing left* — and the engine's only proof of that is exhausting
   the lattice. Both halves are now numbers rather than a prediction:
   **48 745** enterings find all 32, and **17 204 592** prove it.

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

## What the reconnaissance found — 2026-08-26

The phase was begun the way [P1d.3](../p1d.3_model_sets/README.md) and
[P1d.4](../p1d.4_model_set_closure/README.md) were: by measuring what its
stage files assume, before running any of them. **All four remaining stages had
a premise that had moved** — one of them had a measurement pointing the wrong
way by a factor of 8.5, one had its mechanism measured empty, one was half
shipped by a phase that ran after it was written, and one names a documentation
target that has since become history. And the largest number in the phase now
has an alternative that nothing in the plan had priced.

**The instruments.** A re-take of
[S1d.10.1](s1d.10.1_why_it_does_not_finish.md)'s census on today's engine
(`utils/layer_census.py --layers --json`, 197 entries, 9 min) with two crosses —
against `corpus.toml`'s declared runs and against `--json-summary`'s
`owes.declared` — all three banked as
[`layer_census.md` §10](layer_census.md#10-the-re-take--2026-08-26-and-what-p1d2-and-p1d3-moved); two `-j16` runs of the phase's own entry at `-m 5` and `-m 6`;
and an out-of-process **emulation of the per-obligation depth-first branch**
[P1d.2 deferred](../p1d.2_obligations/hypotheses_from_obligations.md), which is
described in §1 and is the finding the phase now turns on.

### 1. The proof costs 83 517× what the answer does

[S1d.2.5 §1](../p1d.2_obligations/hypotheses_from_obligations.md) shipped the
obligations rung and recorded one deviation from its own plan: the rung
proposes the **union** of every owed instance's candidates, where the plan said
*one chosen obligation's*, because

> **The rung proposes the union of every accepted obligation's candidates,
> where the plan said "one chosen obligation's".** … Branch on obligation *O*'s
> candidates alone and layer 1 is *O*'s alternatives — correct, mutually
> exclusive, jointly exhaustive. Layer 2 is then **pairs of them**, every one of
> which is two witnesses for a slot that needs one … "Choose one obligation,
> branch, recurse *at that node*" is a **depth-first** move, and the traversal
> that could take it is P1d.10's subject.

That traversal has now been run — outside the engine, as six lines of policy
over the CLI. At each node: append the node's committed facts to the program,
`ein solve -m 0 --json-summary`, and read the verdict and the `owes` block. A
non-empty unsat core is a dead branch; `owes.total == 0` on a consistent state
is a **model**; otherwise take one owed instance and branch over the full
extent of its witness type. Nothing else — no engine change, no flag, a fresh
process and a fresh load per node.

On `examples/zebra2-minus-15-obligations.ein`, against the lattice's
**17 204 592 enterings and 24 min 56 s** for the same claim:

| instance choice | nodes | dead | models | max depth | wall |
|---|---:|---:|---:|---:|---:|
| **most-owed relation first** | **171** | 105 | **32** | 6 | **2.1 s** |
| fewest-owed relation first | 196 | 125 | **32** | 7 | 2.4 s |
| report order (the rung's default) | 206 | 133 | **32** | 6 | 2.6 s |
| report order, reversed | 316 | 221 | **32** | 6 | 4.0 s |

**The model set is the same 32, verified fact for fact** — each leaf
re-solved and its `*-loc` extent compared against the 32 the lattice reports at
`-e -m 3`; the two sets are equal, with nothing on either side alone. The
hrule original `zebra2-minus-15.ein` gives the identical table, because the
obligations are the same and the branch does not read `:hrules`.

The heading's ratio is the **conservative** one: 17 204 592 / 206 = **83 517×**,
taking the rung's own default policy rather than the best of the four. The best
is 17 204 592 / 171 = 100 611×, and neither number is the interesting part —
what matters is that both are five orders of magnitude and the spread between
policies is 1.85× (§2).

Two things make the number **conservative**, and a third is why the shape is
different rather than the size:

- **Every node is a fresh process and a fresh load.** 2.1 s is 171 parses,
  171 imports and 171 root saturations from scratch, where the engine forks a
  layered COW KB. The in-engine figure is not 2.1 s; it is below it by whatever
  a load costs.
- **The branch is coarser than the rung's.** The emulator enumerates the
  witness type's whole extent; the shipped rung runs the obligation's own
  `absent` guard with the witness step skipped, which is a subset. Every extra
  candidate is a node that dies immediately, so the true count is **≤** these.
- **The depth is 6, not 38.** Each commitment discharges at least one instance
  and no commitment creates one, so the tree's depth is bounded by the 46
  instances root owes — a termination argument that does not mention
  `max_set_size` at all.

The determinate control does not regress: `zebra2.ein` and its obligations twin
are **16–26 nodes** against the lattice's **101 enterings**.

**And one of the five entries it could not do**, which is worth more than the
four it could. `examples/zebra.ein` is the B0 `co-located` encoding — its
obligations are `std.slots`' `slot-owed-room` / `slot-owed-fill` pair, and the
same relation is owed from **both** argument positions. Its tree had not closed
after 900 s where `zebra2`'s closes in 26 nodes. That is either the encoding or
the emulator's coarse branch (the witness type's whole extent, where the rung
scans a guard), and the emulator cannot tell which — so it is
[S1d.10.6](s1d.10.6_the_traversal.md)'s first regression case rather than a
footnote on this table.

> **This is not an optimisation of the lattice, and the census already ruled
> that class out.** [§8](layer_census.md#8-where-the-time-goes-in-this-regime--the-lattice-is-12-)
> measured the whole lattice machinery at **1.2 %** of the run: a perfect
> prefix join and a perfect clause index are bidding for a hundredth of the
> cost. The only lever with room behind it is *entering fewer commitments*, and
> a choice point is how the milestone said that would happen — **committing to
> one alternative excludes its siblings without anybody having to refute
> them.**

### 2. The heuristic P1d.2 measured inert is live

[S1d.2.5 §4](../p1d.2_obligations/hypotheses_from_obligations.md) built the
instance-choice heuristic, measured **0 difference on every counter** under the
breadth-first lattice, recorded it as inert per [F9](../../followups/f9_e_catalog.md)'s
rule, and kept it with a note:

> What would make it live is the deviation in §1 being closed — a per-node
> branch on one obligation — at which point "which one" is the whole question.
> That is P1d.10's to answer, and this row is the note it inherits.

The table above is the answer: **171 … 316 nodes, a 1.85× spread**, on one
puzzle and four policies. The order that is erased by `apriori::order_candidates`
under a lattice survives under a tree, which is the mechanism
[S1d.2.5 §4](../p1d.2_obligations/hypotheses_from_obligations.md) named. It is
also the smallest of the phase's numbers and the one most likely to move on a
second puzzle — there is exactly one to move it on (§3).

### 3. The vocabulary reaches five of fifty-one

The census re-take, crossed against `owes.declared`
([banked as `layer_census.md` §10.1](layer_census.md#101-the-number-the-first-census-could-not-report)):

| | cells | of which **declare an obligation** |
|---|---:|---:|
| entries that reach the search | **51** | **5** — and all five are the zebra family |
| …whose enterings are exactly `Σₖ C(alive, k)` | 25 | **0** |
| …barren (layer 1 kills nothing) | 46 | 2 |

So the milestone's thesis — *the engine enumerates a powerset because it has no
way to say that something is required* — has a corollary the milestone did not
state and this reconnaissance did not expect: **the corpus's powerset walkers do
not say it either, and the reason is that they have nothing to say.** They are
rule demos. `examples/saturation/square-fwd/houses.ein` is three facts and one
rule, asking whether `co-located` projects across `right-of`;
`examples/features/04_open.ein` demonstrates the `(unknown P)` sugar;
`examples/features/01_not_and_absent.ein` contrasts `(not P)` with `(absent P)`
over three people. None of the three has a requirement to state, because none
of the three is a puzzle. Their `Σₖ C(alive, k)` is not a search failing — it is
what *undecided* means when a demo has undecided slots and somebody asks it for
every model.

**Which is the first thing the phase has to decide, and it is a scope
decision rather than a technical one:** a traversal that branches on
requirements helps a program that states one, and after
[P1d.2](../p1d.2_obligations/README.md) the corpus has **two** such programs
that search — `zebra2-minus-15.ein` and its twin — plus three determinate
zebras. That is P1d.10's whole measurable surface.

### 4. The corpus's declared exhaustive search is one feature demo

The census sweeps `solve -e` over every entry, including the 146 that never
reach a layer and the ones whose manifest declares no `solve` at all — which it
says, and which its classifier is right to do, because *a regime is a property
of a puzzle, not of a flag*. Crossed against what `corpus.toml` actually
declares, the picture changes shape
([banked as `layer_census.md` §10.2](layer_census.md#102-the-sweep-against-the-manifest)):

| | enterings |
|---|---:|
| the sweep, over all 51 searching cells | 2 249 873 |
| **under a declared `solve -e`** (33 cells) | **408 108 — 18.1 %** |
| …of which `examples/features/01_not_and_absent.ein` | **384 167 — 94.1 %** |
| …of which `examples/zebra2-minus-15.ein` | **0** — its entry excludes `solve -e` |

**94.1 % of the exhaustive search this repository performs is one feature demo
about negation**, and the puzzle the phase is named after contributes nothing,
because the run that would cost something is the run its corpus entry excludes.

And the 35 facts that demo's 384 167 subsets are drawn from are **18 `likes`
and 17 `is-a`** — the second half being the blind enumerator proposing
`(is-a Alice Bob)` and `(is-a Alice explicitly-dislikes)`, because the kernel
imposes no type system and nothing in that file tells it not to. So the largest
exhaustive search the gate performs spends about half its width on arrows
nobody meant. That is the same phenomenon
[S1d.3.1](../p1d.3_model_sets/model_set_census.md)'s leftover probe found from
the other end — `zebra2`'s unique model leaves 3 678 proposable facts, *"none of
them an attribute arrow, and most of them ill-typed"* — and it is a fact about
the **generator**, not about the traversal. Neither this phase nor a tree fixes
it; a puzzle that states its obligations sidesteps it, which is what
[S1d.2.5 §3](../p1d.2_obligations/hypotheses_from_obligations.md)'s control arm
already showed on a different file — the rung proposes **96** where the blind
enumerator proposes **3 774**, and 1 946 of the blind arm's are `is-a*` and
`next-to` arrows the puzzle never intended anyone to guess about.
The phase's second acceptance bullet — *"one under-determined entry in the
corpus is not a regime, it is an anecdote"* — is therefore unmet twice over, and
no stage below can meet it, because meeting it needs **corpus entries that do
not exist**. [F13](../../followups/f13_puzzles_beyond_zebra/ideas.md) is where
they would come from and it is a link file, not a plan.

### 5. Two things the phase believed, measured false

**(a) Deaths do not live deeper.** [S1d.10.4](s1d.10.4_conflict_mining.md)'s
whole premise is that *"deaths live deeper, where enough hypotheses are
committed to contradict"*, and that a dive would reach them directly. Measured
on the phase's own entry, at `-j16`, with `dead_post` read off each run:

| layers | enterings | deaths |
|---|---:|---:|
| 1–5 | 618 076 | 19 121 |
| **6** | **865 757** | **8** |
| **7–22** | **15 720 759** | **0** |

`dead_post` is 19 129 at `-m 6` and 19 129 at `-m 38`, so **the deep half of
this lattice contains no contradiction at all** — fifteen and a half million
commitments entered, not one refuted, `dead_pre` 0 throughout. A dive whose only
product is clauses would come back with nothing, and it would come back with
nothing *because there is nothing there*. The stage is not merely
unmotivated; its mechanism is measured empty on the one entry it was written
for.

**(b) Deep enterings are not cheaper — the 7.9× is a `--jobs` artefact.**
[Layer census §4.1](layer_census.md#41-the-depth-10-probe--2026-08-25-and-depths-610-add-nothing)
filed a hypothesis: an entering costs 0.674 ms at depth 5 and 0.085 ms at
depth 10, *"7.9× cheaper"*, plausibly because deep enterings die at their first
firing. Reading (a) already refutes the mechanism — nothing dies down there —
and the arithmetic refutes the observation. The 0.674 ms is the census's
`--jobs 1` run; the 0.085 ms is not. At a **constant `-j16`**:

| run | enterings | wall | ms / entering |
|---|---:|---:|---:|
| `-m 5` | 618 076 | 50 181 ms | **0.0812** |
| `-m 6` | 1 483 833 | 123 361 ms | 0.0831 |
| `-m 38` (banked) | 17 204 592 | 1 496 000 ms | 0.0869 |
| — layer 6 alone | 865 757 | 73 179 ms | **0.0845** |
| — layers 7–22 alone | 15 720 759 | 1 372 639 ms | **0.0873** |

The per-entering cost **rises** monotonically with depth, by 7.5 % over
seventeen layers. It does not fall by 87 %. So the barren regime's cost is not
concentrated anywhere: it is `Σₖ C(alive, k)` enterings at a flat price, which
is the least interesting and most damning shape it could have had.

### 6. And `exhausted` was wrong, at one cap — **fixed 2026-08-26**

> **Closed by [T1d.10.5.0](s1d.10.5_contract.md#task-t1d1050--a-cap-of-zero-is-a-truncation--done-2026-08-26)**,
> the same day and out of order, because it depended on nothing in the phase
> and was a defect rather than a design question. A cap of zero is a
> **truncation**: `exhausted = false`, and the S1d.3.3 rule then applies to it
> unchanged. Measured over all 197 manifest entries, the 150 that load went
> **150 / 0** `exhausted true`/`false` to **99 / 51** — the 51 being exactly the
> cells that reach the search, and the 99 unmoved to the field. The two
> questions the task refused to assume are answered there: it answers rather
> than refuses (a refusal would have broken the reconnaissance's own
> instrument, which asks `ein solve -m 0 --json-summary` once per node), and an
> empty `unsat_core` stays constructible because **12 corpus entries already
> report one under their ordinary `solve` run**, all twelve at
> `exhausted = false`. `corpus_exits.txt` did not move.
>
> The read-out below is what it looked like before, kept because the argument
> is the record.

Found by probing the boundary S1d.10.5 owns:

```
$ ein solve -m 0 -s examples/zebra.ein ; echo "exit $?"
  solutions (k)   0
  verdict         No solution — the constraints are contradictory

  unsat core (0 facts)

stats
  solutions (k)    0
  exhausted        true
  enterings        0 (alive=0 dead_pre=0 dead_post=0)
  layers_explored  0
exit 0
```

**`-m 0` refutes every program that has anything to guess**, with an empty
unsat core, a certified exhaustion claim and a success exit code. The layer loop
is `for layer in 1..=max_set_size`, so at 0 it never runs, `truncated` is never
set, and `exhausted = !truncated` is `true` over a frontier that is the *entire*
alive set. The boundary is exact rather than sampled: a program whose root is
already complete answers correctly at `-m 0` — `tests/stdlib/algebra/23_total_owed`
says `Open — owes 1` and `branching/01_saturate_only` says `Solution`, both with
`exhausted true`, and both are right because there is no lattice to exhaust. It
is the **51 cells that reach the search** that are refuted, and all of them.

`-m 1` says `exhausted false` correctly and `-E 0` says `aborted` correctly, so
the budget paths are honest and the depth cap at zero is the one door with no
guard on it. It violates the rule
[S1d.3.3](../p1d.3_model_sets/the_verdict.md) made normative in
[`defined_behaviour.md` §5](../../../docs/kernel/defined_behaviour.md) the same
day — `exhausted = true` may say *these are the models* — by saying *there are
none* about a puzzle with one.

Beside it, and the same subject: **`Contradiction` and `Open` carry no
exhaustion qualifier**, where [S1d.3.3](../p1d.3_model_sets/the_verdict.md) gave
one to `Solution` and `Ambiguity`. That was deliberate — the page says a
refutation under a depth cap is a question about a *word*, not a *count* — and
it is [Q-M1d.1](../open_questions.md#q-m1d1--may-the-search-stop-before-the-lattice-is-exhausted)'s,
with `examples/saturation/type-exclusivity/pets.ein` as the fixture: `k = 0`,
*the constraints are contradictory*, at `-m 5` through `-m 8`, and **35 models**
at `-m 10`.

### And one thing it settled the wrong way round — 2026-08-26

[T1d.10.6.1](completeness.md) was written and part (3) of the completeness
argument came back **false** — *"the leaves are models iff saturation determines
everything no obligation owes"* does not hold, and the reason is not the tree.
Since [S1d.2.5](../p1d.2_obligations/hypotheses_from_obligations.md) a program
that declares an obligation has its candidates generated by the **rung**,
whichever way the search then walks them, so a tree and the lattice search the
*same candidate space*. Whatever `uncovered` names, both exclude.

The measurement is `EIN_LEFTOVER=1` split by relation:
`examples/zebra2-obligations.ein`'s unique model leaves **3 678** proposable
facts and they are **exactly the four `uncovered` relations, no `*-loc` among
them** — which is the argument holding. A 25-line fixture in
[`completeness.md` § 3c](completeness.md) is where it does not: `ein solve -e`
says `k = 3`, `exhausted = true`, and adding one fact of the uncovered relation
yields **three more** consistent exhausted models.

So the risk this phase filed as *"a cheap search that answers a different
question"* — **"the trap is that on this corpus it would not fire"** — fires,
and it fires for the **lattice**. It is a property of the rung, it is shipped,
and it is the same shape as the defect
[T1d.10.5.2b](s1d.10.5_contract.md#task-t1d1052b--contradiction-and-what-a-cap-may-say)
just fixed one level down: a claim printed without the qualifier that licenses
it.

### What the reconnaissance did not settle

- **Whether the traversal ships**, and behind what.
  [design/08 §7](../../../docs/history/m1a_rust/design/08_parallelism.md) rejected
  parallel depth-first because *going depth-first changes which no-goods exist
  when, i.e. the pruning, i.e. the counters* — and a per-obligation branch
  changes more than that: it changes what a *layer* is. Every golden that pins
  `layers_explored` or `enterings_total` moves.
- **What licenses `exhausted = true` under it.** The tree terminates by
  **discharge**, not by exhaustion of a lattice, and those are different
  guarantees. `uncovered` — the rung's count of hypothesis-eligible relations no
  obligation names — is the structural half of the condition and is **not 0**
  on the zebra family: 4 as the search sees it, 2 as `--hyp-stats` does, because
  [`closed.rs`](../../../ein.rs/crates/ein-infer/src/closed.rs) runs the
  closed-relation pass on a fork for the preview and not for `solve`. Both
  numbers are right about their own state and the stage has to say which one the
  argument uses — [T1d.10.6.2](s1d.10.6_the_traversal.md) carries the
  reconciliation and the same caveat for every other number `-H` prints.
- **Anything about the 46 barren cells that state no obligation.** The traversal
  cannot reach them, and §3 says why they do not want it.

## Stages

Five ids and a sixth added 2026-08-26, listed in **run order rather than id
order** — the same licence the phase itself took when it moved from first to
last, and for the same reason: two of the four remaining stages are about a
search the sixth would change, and one of them turned out to be closable on its
own terms.

| runs | stage | title | est. |
|---:|---|---|---|
| 1st | [S1d.10.1](s1d.10.1_why_it_does_not_finish.md) | Why it does not finish — **done 2026-08-24** ([`layer_census.md`](layer_census.md)) | 3 d |
| 2nd | [S1d.10.2](s1d.10.2_depth_required.md) | What depth is required, and for what — **mostly taken**; the remainder is the predictors and the re-take | 1 d |
| 3rd | [S1d.10.4](s1d.10.4_conflict_mining.md) | Conflict mining when a layer is barren — **closed on its own terms**: 0 deaths in 15 720 759 deep enterings | 0.5 d |
| 4th | [S1d.10.6](s1d.10.6_the_traversal.md) | **The traversal — one obligation per node** — **3 of 6 done 2026-08-26**: the argument, the `-H` reconciliation, and **the branch — 86 enterings against 17 204 592, same 32 models** | 5 d |
| 5th | [S1d.10.3](s1d.10.3_stopping_criterion.md) | Is there a stopping criterion? — re-aimed at what licenses `exhausted` under a tree | 3 d |
| 6th | [S1d.10.5](s1d.10.5_contract.md) | What `exhausted` means — **five of six tasks done 2026-08-26**; all four verdicts qualify themselves now, and what is left is the sentence for a search that is not a lattice | 2 d |

**S1d.10.5 is all but closed, out of order, and its last task is
[S1d.10.6](s1d.10.6_the_traversal.md)'s to unblock.** Five of its six tasks
shipped 2026-08-26 — the `-m 0` boundary, the verdict surface,
[Q-M1d.1](../open_questions.md#q-m1d1--may-the-search-stop-before-the-lattice-is-exhausted)'s
word, the docs and the corpus note — and the sixth is *"a search that is not a
lattice needs the sentence rewritten"*, which needs the tree to exist. So the
phase's vocabulary is settled for the search it has and open only for the search
it might build.

**11.5 days remain against the 12 the original four carried**, so the
reconnaissance moved work between stages rather than adding it: S1d.10.2 loses a
day to its own predecessor, S1d.10.4 loses three and a half to being closed,
S1d.10.3 loses one to a dead candidate, and the five that come back are the
traversal. One task in the last stage did not wait for any of the others and has been
taken: **`T1d.10.5.0`, the `-m 0` boundary (§6 above), is done 2026-08-26** —
an hour, out of order, and the phase's first shipped change. The 11.5 days are
otherwise untouched: it was costed inside S1d.10.5's two.

**Two ids changed subject rather than title, and the titles are kept because
the arguments are.** [S1d.10.4](s1d.10.4_conflict_mining.md) still asks whether
a barren layer can be mined for conflicts — the answer is now *there are no
conflicts down there*, which is an answer to the question it asked and not a
different question. [S1d.10.3](s1d.10.3_stopping_criterion.md) still asks for a
stopping criterion; what moved is that the most promising candidate is no
longer a criterion *over the lattice* but a **termination argument for a tree**,
and its three original candidates become the ledger F9's discipline asks for.

## The ledger — **closed 2026-08-27**

**Closed as it stands, at the user's direction**, with three stages shipped and
three left where they were. The phase was estimated at 14.5 days and spent
about five; what follows is what it bought, what it dropped, and — the part a
closed phase is most likely to lose — **what it found and did not act on.**

### What shipped

| | |
|---|---|
| [S1d.10.1](s1d.10.1_why_it_does_not_finish.md) | **done** — [`layer_census.md`](layer_census.md), the `layer` event and its sixteen counters, 96.7 % of corpus search proved an exact powerset |
| [T1d.10.5.0](s1d.10.5_contract.md) | `-m 0` is a truncation. 51 of the 150 loading entries stopped claiming a certified refutation over a frontier they had not looked at |
| [T1d.10.5.2 / .2b](s1d.10.5_contract.md) | **[Q-M1d.1](../open_questions.md)'s word.** `Contradiction` and `Open` carry the exhaustion qualifier; the unsat core is `refuted so far` when truncated. 26 cells, 13 files, no exit code |
| [T1d.10.5.3 / .4](s1d.10.5_contract.md) | the four-row table in `defined_behaviour.md` §5, `-e`'s help corrected, both corpus notes re-priced |
| [T1d.10.6.1](completeness.md) | the completeness argument, before the code — two parts hold, the third is false **and not the tree's doing** |
| [T1d.10.6.2](s1d.10.6_the_traversal.md) | `-H` against the search: the gap is one whole relation, and `emit_closed` runs before saturation |
| [T1d.10.6.3](s1d.10.6_the_traversal.md) | **the tree.** 86 enterings and 0.07 s against 17 204 592 and 1 496 s, same 32 models fact for fact, behind `EIN_TRAVERSAL=tree` |
| — | `--layer-progress`, the 51st CLI option, which the phase turned out to need to see its own subject |

### What was dropped, and it is a decision rather than an oversight

`T1d.10.2.3` (predictors at layer *d*), `T1d.10.4.5` (S1d.10.4's disposition —
the refutation *is* written, only the F9 row is not), `T1d.10.6.4` (what a tree
reports), `T1d.10.6.5` (measure both regimes), `T1d.10.6.6` (the ship
decision), all of [S1d.10.3](s1d.10.3_stopping_criterion.md)'s ledger, and
`T1d.10.5.1`'s second half — the sentence for a search that is not a lattice.

The last five are one question wearing five names: **may a tree say
`exhausted = true`?** The tree ships answering *no* — `truncated` is set, the
verdict says *models found* — which is the phase's own rule (*never a quiet
`exhausted = true`*) taken as the safe default rather than as an answer.

### Acceptance, honestly

| bullet | |
|---|---|
| `solve -e` finishes with a stated exhaustion claim | **met** — and replaced by *is there a cheaper argument?*, which is **met**: 200 053× fewer enterings for the same 32 models |
| the under-determined regime is a named part of the measurement set | **not met, and not meetable here.** Two under-determined searching entries exist and neither declares `solve -e`; 94.1 % of the exhaustive search the repo performs is one negation demo. It needs corpus entries [F13](../../followups/f13_puzzles_beyond_zebra/ideas.md) would supply |
| nothing changes what the engine proves | **held.** Every change is additive or behind `EIN_TRAVERSAL`; the gate is green with no golden re-blessed by the traversal |
| every proposal measured against F9's discipline | **held** — S1d.10.4 closed by measurement (0 deaths in 15 720 759 deep enterings), the choice heuristic re-measured live, the tree's own hrule regression measured *before* the guard |
| the determinate puzzles do not regress | **held** — `zebra`/`zebra2` are 111 and 101 enterings, unchanged, and the tree declines on both |

### What it found and did not act on

Eight things, kept here because a closed phase is where they would otherwise be
lost. Nothing below is a task; each is a measurement with no owner.

1. **A tree run narrates nothing.** 0 `enter` and 0 `layer` events while
   `enterings_total` says 86, so `--events` and `utils/layer_census.py` see an
   empty run. This was `T1d.10.6.4`'s subject and is the first thing anyone
   resuming the traversal will hit.
2. **`emit_closed` runs before saturation**, so it closes `nation-loc` on
   evidence `co-located-fanout` then invalidates. Its stated criterion — *no
   rule can positively conclude an R-fact* — is not the one it computed.
3. **…and that closure is worth 40 %.** `(__closed__ nation-loc)` takes
   `zebra2-minus-15-obligations -e -m 3` from 48 745 enterings and 26.0 s to
   **29 144 and 15.3 s**, with the model set identical fact for fact. Sound
   generalisation or ordering bug — undetermined.
4. **`exhausted = true` over-claims when `uncovered ≠ 0`** and those relations
   are neither closed nor determined. The 25-line fixture in
   [`completeness.md` § 3c](completeness.md) reports `k = 3, exhausted = true`
   where at least 6 models exist. Shipped behaviour, and
   [T1d.10.5.2b](s1d.10.5_contract.md)'s pattern one level up.
5. **Two root read-outs disagree in one binary** — `--json-summary`'s
   `root.hypgen` says `raw 230, emitted 96`; `-H` says `190 / 81`.
6. **`commitment.rs`'s doc is stale**: *"`resume` is `None` on every shipping
   path"* is false — `resume_forks()` is true by default and all four lattice
   call sites pass the snapshot. It misled this phase's own tree into
   re-deriving root's fixpoint per node (`fe34095`).
7. **7 256 complete forks collapse to 4 new models** at layer 3 of the phase
   entry — 99.94 % duplicates. That is the evidence
   [S1d.10.3](s1d.10.3_stopping_criterion.md)'s candidate (a) and its prose
   about re-walking known neighbourhoods never had.
8. **The blind-arm comparison is unaffordable.** `EIN_OBLIGATION_CHOICE=off
   … -e` on `zebra2-obligations` did not finish in **10 minutes** where the
   rung arm answers in 0.03 s. Any future rung-vs-blind re-take needs a cap and
   must say so.

## Acceptance for the phase

- ~~**`solve -e examples/zebra2-minus-15.ein` finishes**, with all 32 models and
  a stated exhaustion claim~~ — **met 2026-08-26**, on the obligations twin at
  `-m 38`: `k = 32`, `exhausted = true`, 17 204 592 enterings, 22 layers,
  24 min 56 s at `-j16`. What replaces it is the second half of the same
  sentence: **the proof has a cheaper argument or the phase records why it does
  not.** § What the reconnaissance found §1 is the candidate and the number it
  has to beat is its own — 171 nodes, out of process, against 17 204 592.
- The under-determined regime is a **named part of the measurement set**, the
  way [P1a.7](../../../docs/history/m1a_rust/README.md#p1a7--parallelism) had to re-aim its scaling target.
  One under-determined entry in the corpus is not a regime, it is an anecdote.
  > **This bullet cannot be met by any stage below, and the reconnaissance is
  > why.** Under declared runs the corpus has **two** under-determined searching
  > entries — `zebra2-minus-15.ein` and its twin — and neither declares
  > `solve -e`; 94.1 % of the exhaustive search the repository actually performs
  > is `examples/features/01_not_and_absent.ein`, a demo about negation. Meeting
  > it needs **new corpus entries**, which is
  > [F13](../../followups/f13_puzzles_beyond_zebra/ideas.md)'s subject and
  > F13 is a link file. The bullet stays, restated as what it now is: **a
  > prerequisite the phase does not own**, and the honest form of "we could not
  > meet it" is to say which entries would have.
- **Nothing changes what the engine proves.** A sound criterion makes the same
  proof cheaper; an unsound one changes the answer. Anything in the second
  class ships behind a flag and reports a *different* verdict word — never a
  quiet `exhausted = true`. **A traversal is in the first class or it does not
  ship**: a depth-first branch on an obligation is complete at its node because
  the alternatives are jointly exhaustive, and the stage that builds it owes
  that argument in writing before it owes a number.
- Every proposal is measured against F9's discipline: **a mechanism that is
  inert on the corpus is recorded as inert and not shipped**, with the number.
  S1d.10.4 is the first to be closed by it in this phase rather than merely
  judged against it.
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
  > **And a per-obligation tree changes more than the counters — it changes
  > what a *layer* is.** `layers_explored`, `enterings_total` and every `layer`
  > census row are statements about a cardinality-ordered lattice, and a tree
  > has none of them. The event protocol, the JSON summary, `--stats`, the
  > shape digests and `utils/layer_census.py` all read those fields. Whatever
  > [S1d.10.6](s1d.10.6_the_traversal.md) builds is therefore **a second
  > traversal beside the first**, selected explicitly, or it is a re-baseline of
  > the whole corpus — and those are different-sized decisions.
- **An unsound stopping rule is worse than a slow search.** "No new model for
  k layers, so stop" is a heuristic wearing a proof's clothes. If it ships it
  reports `Ambiguity (not certified)`, and the word `exhausted` stays false.
- **A cheap search that answers a different question is the same failure in a
  better disguise.** The reconnaissance's tree terminates by *discharge*, and
  the lattice terminates by *exhaustion*; they agree on `zebra2-minus-15`'s 32
  models because `uncovered`'s four relations happen to be saturation-determined
  there ([S1d.2.5 §6](../p1d.2_obligations/hypotheses_from_obligations.md)). A
  puzzle where they are not is a puzzle where the tree is **complete for the
  obligations and incomplete for the models**, and the only thing that would
  catch it is the model-set comparison S1d.2.5 already ran once. The trap is
  that on this corpus it would not fire.
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
- [S1d.2.5 — hypotheses from obligations](../p1d.2_obligations/hypotheses_from_obligations.md)
  — §1 is the deferral this phase inherits and §4 is the heuristic it makes
  live; read both before [S1d.10.6](s1d.10.6_the_traversal.md)
- [`closed.rs`](../../../ein.rs/crates/ein-infer/src/closed.rs) — *"where it is
  called from matters, and it is not `solve`"*: why `--hyp-stats`' numbers are
  not the search's
- [S1d.3.3 — the verdict](../p1d.3_model_sets/the_verdict.md) — the rendering
  rule this phase's [S1d.10.5](s1d.10.5_contract.md) finishes, and the two
  verdicts it deliberately left uncovered
