# M1d — From saturation to satisfiability

**2026-08-21 → 2026-08-27.** Four phases, eighteen stages, ~9.5 weeks of stage
estimates; spent about seven days. **Shipped**, and `plans/m1d_satisfiability/`
was deleted the day it closed (`git log --diff-filter=D -- plans/m1d_satisfiability`
has all 12 256 lines of it).

This README is the milestone, its four phases and their stages, in one file.
Beside it are the fifteen documents that are still *read* — as evidence, as a
specification, or as the reason something is the way it is — which is the rule
that put them here rather than leaving them in git.

## What it was for

Two pieces of the repo that had never been put next to each other:

- **M1a's ex-P1a.12** — `examples/zebra2-minus-15.ein`, the canonical zebra
  with one condition removed, exhaustively solvable in principle and
  uncompletable in practice.
- **[`ideas.md`](ideas.md)** (ex-followup F14) — *saturation is not
  satisfiability*: the engine could say a state was **consistent** and could say
  it was **false**, and had no way to say a program **requires** something. So
  it could not distinguish *no model* from *not yet a model*.

The link between them is the milestone's thesis: **the engine enumerates a
powerset because it has no way to say that something is required.** A
requirement is a choice point, not a subset — committing to one alternative
excludes its siblings without anybody having to refute them — and
[S1d.10.1](#s1d101--why-it-does-not-finish)'s census made that measurable
before anything was built.

## What shipped

| | |
|---|---|
| **A program can state a requirement** | four stdlib obligation rules — `total-owed` / `surjective-owed` in `std.algebra`, `slot-owed-room` / `slot-owed-fill` in `std.slots` — as a **rule shape** asserting a reserved verdict atom `(open ?R)` |
| **A state can say what it owes** | one pass over the quiescent KB, never in the saturation agenda; `owes.total`, `by_relation`, `instances` in `--json-summary` and the `owe` event |
| **The search branches on it** | the obligations rung of the generation ladder — `(hrule …)` if declared, else what the state owes, else the blind enumerator |
| **The verdict reports it** | **`Open — owes n (rel: n, …)`**, the fourth verdict word, exit 0 like the other three |
| **A model set has a compact form** | `ein solve --models key` — the smallest set of slots that tells the models apart, 49 lines against 516 on `zebra2-minus-15` |
| **A count states its own guarantee** | *these are the models* against *these are models **found***, on all four verdicts — normative in [`defined_behaviour.md` §5](../../kernel/defined_behaviour.md) |
| **A second traversal** | `EIN_TRAVERSAL=tree`, one obligation per node: **86 enterings and 0.07 s** against the lattice's 17 204 592 and 1 496 s, same 32 models fact for fact |
| **Four censuses** | `utils/openness_census.py`, `model_set_census.py`, `closure_census.py`, `layer_census.py` — and `ein test --json-report`, the read-out the third needed |

## The phases

| phase | stages | outcome |
|---|---|---|
| [P1d.2](#p1d2--obligations) | 6, done 2026-08-25 | a puzzle states a requirement, a state says what it owes, the search branches, the verdict reports |
| [P1d.3](#p1d3--model-sets) | 3, done 2026-08-26 | the compact representation, and the enumeration's own guarantee |
| [P1d.4](#p1d4--closing-the-model-set) | 3, done 2026-08-26 | **no keyword** — the answer is *no* three times over |
| [P1d.10](#p1d10--exhaustive-search) | 6, **closed 2026-08-27 as it stood**; 3 shipped, 3 dropped | the proof costs 17 204 592 enterings and the tree costs 86 |

---

## P1d.2 — Obligations

*The half of the vocabulary that says **must**.* Six stages in one day.

**The gap it closed**, stated as a table the phase built: the stdlib could say
`functional` and `injective` (two conflicting facts ⇒ `(false)`), and could say
`total` and `surjective` only in the **open-world-safe** form — *every candidate
is excluded, so this state is dead*. What no rule could say was *one candidate is
still missing, so this state is **unfinished***. That sentence is the milestone.

### S1d.2.1 — The property audit
[`property_audit.md`](property_audit.md). The `≥` half has **fifteen rules and
no middle**: everything the stdlib expresses is either an upper bound enforced by
refutation or a completion once alternatives run out.

### S1d.2.2 — The domain contract
[`domain_contract.md`](domain_contract.md), C1–C4. **12 of 49** searching entries
propose `is-a` arrows, which is exactly where C4 declines: *a branch is jointly
exhaustive only while the candidate set cannot grow underneath it*, so an
obligation whose guard scans a relation the rung itself proposes is declined and
the call falls back to the blind generator.

### S1d.2.3 — The form
[`obligation_forms.md`](obligation_forms.md) is the full menu, A through G, and
**form G won**: a rule shape asserting a reserved atom. Not a kernel primitive
(form A), not a first-class object (form E), not a stdlib convention. **Nine**
load refusals with no Python counterpart. The user settled two sub-questions:
the atom is `(open ?R)` and not a bare `(open)` — *"why does `open` take
arguments when `(false)` does not?"* — and obligation rules run **after** the
fixpoint, as one pass, never in the agenda.

### S1d.2.4 — Obligations in the saturator
The report stratum. `zebra2-minus-15` owes **46**, split 10/8/8/10/10, and the
hand census reproduced it. Cost: two stored activator facts per declaration, 50
across the corpus's 13 such entries. No puzzle changed a line.

### S1d.2.5 — Hypotheses from obligations
[`hypotheses_from_obligations.md`](hypotheses_from_obligations.md) — the
generator **ladder**, and the phase's largest number: layer 1 is **56 candidates
against the blind enumerator's 3 734**, and **not one counter moved** against the
hrule path. `(bijective color-loc)` now tells the search what to guess, and
`:hrules` became an override rather than the only way in.

Two things it recorded up front, and both mattered later:

- **The rung proposes the union** of every owed instance's candidates where the
  plan said *one chosen obligation's* — because a breadth-first lattice over a
  fixed `alive` cannot take a per-node branch. *"Choose one obligation, branch,
  recurse at that node"* is a depth-first move, and it was handed to P1d.10,
  which [built it](#s1d106--the-traversal).
- **The instance-choice heuristic** was built, measured at **0 difference on
  every counter** under the lattice, recorded inert per
  [F9](../../../plans/followups/f9_e_catalog.md), and kept — because the
  traversal that would make it live was already named.

### S1d.2.6 — Verdicts, counters, corpus
`Open` shipped. **12** words moved, **0** exit codes, **0** counters, and **92**
of the 121 entries that reach a fixpoint report exactly what they did before —
because the read-out is **scoped**: a state is judged by discharge when it has
been told what it owes and by exhaustion when it has not.
[`openness_census.md`](openness_census.md) is the evidence, and its third column
is the one that did not exist: **`declared`**, how many obligation rules a
program *states*, because `owes = 0` is equally true of a debt paid and a debt
never stated.

### The decisions, and who took them

| decision | taken |
|---|---|
| a requirement is a **rule shape** asserting a reserved atom (form G) | user, 2026-08-24 |
| the atom is **`(open ?R)`**, not a bare `(open)` | user, 2026-08-25 |
| obligation rules run **after** the fixpoint, one pass, never in the agenda | user, 2026-08-25 |
| obligations **supersede** `:hrules` as the generator, as a ladder | user, 2026-08-24 |
| the verdict word is **`Open — owes n`** | user |
| the read-out is **scoped** — discharge where stated, exhaustion where not | 2026-08-25 |
| closed-and-owing reports **`Open`**, not `(false)` | user, 2026-08-25 |

### What it deferred, each with its trip-wire

**Form E** (the obligation as a named object) — un-deferred by a rule that must
reason about another rule's debt. **Form A** (a kernel `(require …)`) — by a
requirement no rule shape can express; none has appeared. **Numeric bounds**
(`L ≤ # ≤ U`, and the `odd` / `same-count-as` family) — by *a corpus entry that
cannot be stated without it*; every entry today needs only `≥ 1`. **The compound
witness** (two positive steps bearing free variables) — it is *refused at load*
rather than mis-compiled, so the trip-wire is a user hitting the refusal.
**`:expect` for an open verdict** — twelve entries changed word without one.

---

## P1d.3 — Model sets

*The compact answer.* Three stages.

### S1d.3.1 — What the models differ in
[`model_set_census.md`](model_set_census.md), and the first census whose subject
is the **answer**. It turns `--json-summary`'s solutions into decision variables
and asks whether the set *factors*, at four granularities.

**It does not.** 13 corpus entries have a model set (four more than a `-m 2`
count sees); **2** partition, and both are two-object demos whose three-object
siblings do not; 5 are a free grid and every one has `k ≤ 4`. On
`zebra2-minus-15`, **248 of 253** variable pairs are coupled — **one** component,
a graph that is K₂₃ minus five edges, minimum vertex separator **17 of 23**.

It also carries the probe P1d.2 declined: `EIN_LEFTOVER=1`, and its number —
`zebra2`'s **unique** model leaves **3 678** facts the blind enumerator would
still propose, *none of them an attribute arrow*.

### S1d.3.2 — Representations
[`representations.md`](representations.md) prices five forms on four columns —
produce · size · exact · read — and **reverses the stage's own prediction**. The
certain core, the favourite, cannot say how many models there are and invites
arithmetic that over-states by 3.11 × 10¹². The **determining key** wins at
**2 506 bytes**, and it is *verified* exact: all **32 of 32** key rows
reconstruct their model to the fact, 30 of them without entering a commitment.

### S1d.3.3 — The verdict
[`the_verdict.md`](the_verdict.md). Two things shipped, and the second is the
one that outlived the phase.

`ein solve --models key` — the 49th CLI option, read by the `Ambiguity` arm
alone, changing nothing recorded.

And a **rendering rule made normative**: `exhausted = true` may say *these are
the models*; `exhausted = false` may say only *these are models **found***. It
mattered because the corpus was full of the case —
`saturation/type-exclusivity/colors.ein` printed **5** for a file with **9**
models, and 5 of the 10 entries answering `Ambiguity` did it unexhausted.
`Contradiction` and `Open` were deliberately left out, because there the problem
is a *word* and not a qualifier on a count; P1d.10 finished them.

The same stage split two numbers that had always agreed: `verdict.k` counts
**models**, `stats.solution_nodes` counts what the **search** recorded.

---

## P1d.4 — Closing the model set

*The claim nothing can state.* Three stages in one day, and it shipped **no
keyword** — the outcome its own Risks section argued for.

### S1d.4.1 — What closure costs
[`closure_census.md`](closure_census.md), whose transport is
`ein test --json-report` — a read-out the same stage added, because `:expect` had
**no machine-readable surface at all**. The reason it insisted the census be
*parsed*: the reconnaissance grepped `:expect (or`, found two users, and one of
them was a **header comment documenting the form**.

Parsed, the corpus states **59 claims over 124 queries and exactly one about a
set**; all 59 hold and all 59 exhausted. Two new numbers: the **write** cost
under *naming a relation closes it* — worst at **4.28×** on a 95-line feature
demo and **0.96×** on the puzzle — and the counterfactual `NOT CHECKED` set:
**10 of 121** entries do not exhaust at `ein test`'s depth, so a claim written on
any of them could not be checked.

### S1d.4.2 — The second-order boundary
[`the_boundary.md`](the_boundary.md). **May a program require its own model
count? No**, three independent times: on compilation, on grounds (`-m` is a
budget, not a semantics), and on **evaluability** — `(closed)` costs 17 204 592
enterings where `(open ?R)` costs one pass over a quiescent KB. Q-M1d.7 closed.

### S1d.4.3 — The vocabulary
[`the_vocabulary.md`](the_vocabulary.md). The user decided: **no keyword.** Tests
stay exhaustive by default, `:expect` stays closed by default, and a claim too
slow to check at the runner's depth stays out of the corpus — which needs no
mechanism, because `NOT CHECKED` takes a failing exit code inside `cargo test`.

What shipped is one line of stderr: a failing `:expect` under `ein solve` now
says why on **stderr** as well, because an exit 1 with an empty stderr is a run
nobody can diagnose from a pipeline. stdout is unchanged — *a false claim is a
result, not a refusal of the input.*

---

## P1d.10 — Exhaustive search

*Why an under-determined puzzle does not finish.* Six stages; **closed
2026-08-27 as it stood**, three shipped and three dropped rather than deferred.

### S1d.10.1 — Why it does not finish
[`layer_census.md`](layer_census.md), and the `layer` event's sixteen counters —
of which `dropped_nogood`, what the learned clauses removed from the next layer's
join, is the one nothing reported before.

Its answer: of 2 232 330 joined candidates corpus-wide, **0** were dropped for a
dead element and **31 303 — 1.4 %** by a clause. For **25 of the 49** entries
that search at all, the enterings are *exactly* `Σₖ C(alive, k)` — **96.7 %** of
the corpus's search work, with nothing dying, nothing learned, nothing filtered.
Layer 1 kills something in **4 of 49** entries. And the whole lattice machinery
— prefix join plus a filter walking 11 577 clauses per candidate — is **1.2 %**
of the run, which rules out every proposal that makes the lattice cheaper.

### S1d.10.2 — What depth is required
Mostly answered by its predecessor. `d_found` is 3 on the phase entry and
`d_stop` is whatever `-m` says: 92.1 % of the run is past the last new model at
`-m 5`, **99.54 %** at `-m 10`, **99.72 %** at `-m 38`. Two beliefs were measured
**false**: deaths do not live deeper (layers 7–22 are 15 720 759 enterings and
**zero** deaths), and deep enterings are not cheaper — the reported 7.9× was a
`--jobs` artefact, and at a constant `-j16` the per-entering cost *rises* 7.5 %
over seventeen layers.

### S1d.10.4 — Conflict mining when a layer is barren
**Closed on its own terms.** The premise was *deaths live deeper, where enough
hypotheses are committed to contradict*. On the phase's own entry they live at
depths 2 and 3 and stop: `dead_post` is 19 129 at `-m 6` and 19 129 at `-m 38`.
The mechanism is sound and its fuel is absent.

### S1d.10.5 — What `exhausted` means
Five of six tasks. **`-m 0` was a defect**: the layer loop is `1..=max_set_size`,
so at zero it never ran, `truncated` was never set, and **51 of the 150** loading
corpus entries stated a refutation with an empty unsat core, a certified
exhaustion claim and a success exit code. A cap of zero is a **truncation**.

Then [Q-M1d.1](open_questions.md)'s word. `saturation/type-exclusivity/pets.ein`
said *the constraints are contradictory* at `-m 5` and `-m 8` and has **35
models** at `-m 10`. Now `exhausted = false` prints *No model found — the search
did not exhaust the lattice*, and the core block is *refuted so far* rather than
*unsat core*, because a core explains why a program has no model and a stopped
search has not shown that. **26 cells, 13 files, no exit code and no counter.**
The claim channel had been answering `NOT CHECKED` on the same run since M1c, so
the two read-outs stopped contradicting each other.

### S1d.10.6 — The traversal
[`completeness.md`](completeness.md) is the argument, written before the code as
the stage required. Three parts were asked for:

1. **jointly exhaustive** — holds, by the obligation's meaning, with C4 as its
   precondition.
2. **terminates** — holds, and by measurement on every edge: the owed count
   strictly decreased on **65 of 65, 70 of 70 and 72 of 72** edges under three
   walk policies.
3. **the leaves are models** — **false**, and *not the tree's doing*. A program
   that declares an obligation has its candidates generated by the rung
   whichever way the search walks them, so tree and lattice search the same
   space. `uncovered ≠ 0` means the **rung** does not propose those relations.

Then the branch itself: **86 enterings, 0.07 s, 32 models identical fact for
fact**, against the lattice's 17 204 592 and 1 496 s. Behind `EIN_TRAVERSAL=tree`,
reporting `exhausted = false` on purpose.

**And a guard, which is the sharpest finding of the phase.** Built without one,
the tree reproduced the engine this repo deleted: an hrule's candidates are not
one owed instance's alternatives, so branching on them walks hypothesis *paths*
and reaches a size-`d` set by `d!` routes — `examples/zebra2.ein` at **7 877**
enterings against 101, `zebra.ein` at **11 083** against 111, same models. That
is the kernel README's own sentence about the tree solver P1.5b removed in
`8d77b02` (2026-05-29), arriving 89 days later. `tree()` now probes the rung at
root and declines on anything but the obligations one.

### What P1d.10 was closed without

`T1d.10.2.3` (predictors at layer *d*), `T1d.10.4.5` (the F9 disposition row —
the refutation is written, the row is not), `T1d.10.6.4` (what a tree reports),
`T1d.10.6.5` (measure both regimes), `T1d.10.6.6` (the ship decision), all of
S1d.10.3's stopping-criterion ledger, and `T1d.10.5.1`'s second half.

The last five are **one question wearing five names**: *may a tree say
`exhausted = true`?* The tree ships answering **no** — `truncated` is set and the
verdict says *models found* — which is the phase's own rule (*never a quiet
`exhausted = true`*) taken as the safe default rather than as an answer.

### Eight measurements with no owner

Kept because a closed phase is where a finding gets lost.

1. **A tree run narrates nothing** — 0 `enter` and 0 `layer` events while
   `enterings_total` says 86, so `--events` and `utils/layer_census.py` see an
   empty run.
2. **`emit_closed` runs before saturation**, so it closes `nation-loc` on
   evidence `co-located-fanout` then invalidates. Its stated criterion — *no rule
   can positively conclude an R-fact* — is not the one it computed.
3. **…and that closure is worth 40 %.** `(__closed__ nation-loc)` takes
   `zebra2-minus-15-obligations -e -m 3` from 48 745 enterings and 26.0 s to
   **29 144 and 15.3 s**, model set identical fact for fact. Sound
   generalisation or ordering bug — undetermined.
4. **`exhausted = true` over-claims when `uncovered ≠ 0`** over relations that
   are neither closed nor determined. The 25-line fixture in
   [`completeness.md` § 3c](completeness.md) reports `k = 3, exhausted = true`
   where at least 6 models exist. Shipped behaviour.
5. **Two root read-outs disagree in one binary** — `--json-summary`'s
   `root.hypgen` says `raw 230, emitted 96`; `-H` says `190 / 81`.
6. **`commitment.rs`'s doc is stale**: *"`resume` is `None` on every shipping
   path"* is false, and it misled this milestone's own tree into re-deriving
   root's fixpoint per node (`fe34095`).
7. **7 256 complete forks collapse to 4 new models** at layer 3 of the phase
   entry — 99.94 % duplicates, which is the evidence S1d.10.3's candidate (a)
   never had.
8. **The blind-arm comparison is unaffordable** — `EIN_OBLIGATION_CHOICE=off
   … -e` on `zebra2-obligations` did not finish in **10 minutes** where the rung
   arm answers in 0.03 s.

---

## Acceptance for the milestone

| bullet | |
|---|---|
| `solve -e examples/zebra2-minus-15.ein` finishes with a stated exhaustion claim | **met** 2026-08-26 on the obligations twin at `-m 38`: `k = 32`, `exhausted = true`, 17 204 592 enterings, 22 layers, 24 min 56 s at `-j16`. And the cost is settled too — the tree reaches the same 32 in **86** |
| the engine can state a requirement | **met** — `total` and `surjective` with the force their names claim; the general `L ≤ # ≤ U` form deferred, because no corpus entry needs it |
| a saturated state can report what it still owes | **met** — `Open — owes n`, with the live candidate sets |
| nothing changes what the engine proves, silently | **held** — every change additive or behind a lever; the traversal re-blessed no golden |
| the determinate corpus does not regress | **held** — `zebra -e` and `zebra2 -e` at 111 and 101 enterings, unchanged; the tree declines on both |

**One bullet of P1d.10's own was not met and was never meetable there**: *the
under-determined regime as a named part of the measurement set*. Two
under-determined searching entries exist and neither declares `solve -e`; 94.1 %
of the exhaustive search this repository performs is
`examples/features/01_not_and_absent.ein`, a demo about negation. It needs corpus
entries [F13](../../../plans/followups/f13_puzzles_beyond_zebra/ideas.md) would
supply, which is a followup and not a stage.

## The documents beside this one

**Evidence, and re-takable** — [`openness_census.md`](openness_census.md) (what
the corpus owes, `utils/openness_census.py`),
[`model_set_census.md`](model_set_census.md) (`utils/model_set_census.py`),
[`closure_census.md`](closure_census.md) (`utils/closure_census.py`),
[`layer_census.md`](layer_census.md) (`utils/layer_census.py`), and
[`hypotheses_from_obligations.md`](hypotheses_from_obligations.md) (what the
ladder cost).

**Arguments and specifications** —
[`property_audit.md`](property_audit.md), [`domain_contract.md`](domain_contract.md)
(C1–C4), [`obligation_forms.md`](obligation_forms.md) (the menu A–G and why G),
[`representations.md`](representations.md) (five forms priced),
[`the_verdict.md`](the_verdict.md), [`the_boundary.md`](the_boundary.md),
[`the_vocabulary.md`](the_vocabulary.md), [`completeness.md`](completeness.md).

**And the two the milestone began with** — [`ideas.md`](ideas.md), the note that
is authoritative on intent, and [`open_questions.md`](open_questions.md), seven
`Q-M1d.<n>` of which **Q-M1d.1** is open on purpose: its vocabulary half is
answered and its *stopping* half — may a search stop early on purpose, and what
licenses it — is what the milestone did not settle.
