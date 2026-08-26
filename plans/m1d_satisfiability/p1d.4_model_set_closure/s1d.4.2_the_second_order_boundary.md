# S1d.4.2 — May a program state it? The second-order boundary

**Phase:** [P1d.4](README.md) (Closing the model set)
**Estimate:** 3 days
**Depends on:** [S1d.4.1](s1d.4.1_what_closure_costs.md) — not for the argument,
which is a language question, but for the *urgency*: a boundary drawn around a
claim nobody writes is drawn differently from one around a claim everybody
wants.
**Status: done 2026-08-26.** [Q-M1d.7](../open_questions.md#q-m1d7--may-a-program-require-its-own-model-count)
closed **no**; the argument is banked in
[`the_boundary.md`](the_boundary.md). Nothing was implemented, as planned.

## What it found

| claim | asked for | answered |
|---|---|---|
| the rule-shape test | applied, not assumed; the negative argued | **three refusals, independent** — (a) on compilation, (b) on grounds, **(c) on evaluability**, which the plan did not list |
| the traversal argument | in full, since it is the reason rather than the symptom | **`-m` is a budget**: a rule reading `exhausted` fires at `-m 38` and not at `-m 5` on the same program, so a budget would change what is provable |
| the neighbour survey | four systems, cited, contradictions recorded | **none contradicts** — and all four put the count at the *meta* level, the two closest putting it on the command line |
| Alloy | catalogued, or recorded as out | **catalogued** — [`docs/lib/03` § Alloy](../../../docs/lib/03-theorem-proving-formal-methods.md). The knowledge graph is not touched, and § 3 says why |
| the boundary | stated once, general enough for the next keyword | *a rule is a sentence about the world it fires in* — with three corollaries, one of which is a **licence** rather than a prohibition |

**Three things the stage found that nothing asked for.**

- **The third refusal is the interesting one, and it is about affordability
  rather than about semantics.** Form G invites `(closed)` as a verdict atom
  the way `(open ?R)` is one — and the reason it fails is that a verdict atom
  is only worth having if the engine can *evaluate* it. `(open ?R)` costs one
  pass over the quiescent KB; `(closed)` costs the whole lattice — 17 204 592
  enterings on `zebra2-minus-15`. **So the affordability problem is not
  downstream of the vocabulary; it is why the vocabulary cannot exist.** That
  is a stronger closing of Q-M1d.7 than "rules cannot see the search", because
  it survives someone building the mechanism.
- **Alloy is the nearest miss and ein already has its mechanism.** `run p for
  5` is a bound on the analyser's search, written in the model file — which is
  what made it look like a program constraining its own model space. It is a
  *command*, and **the ein analogue of an Alloy scope is `--max-set-size`**.
  Ein has Alloy's mechanism, in Alloy's position, spelled as a flag.
- **The boundary licenses one thing as well as forbidding three.** *How deep to
  search* is not a second-order claim — it is Alloy's scope — so a future
  program-level depth declaration is not blocked by this ruling. That matters
  because it is the shape of the only gap
  [S1d.4.1](closure_census.md) found that anybody actually wants.

**One task did not need doing the way the plan drew it.** T1d.4.2.2's scope
discipline said "no encoding is written, nothing is run" and the survey held to
it; what it did *not* anticipate is that the catalogue's **knowledge graph** is
a curated subset rather than an index (`Curry–Howard`, `Natural deduction`,
`Monte Carlo Tree Search` and a dozen more are catalogued without nodes), so
adding Alloy to `docs/lib/03` correctly means **not** re-rendering the four
SVGs. AGENTS.md's rule fires when `knowledge-graph.dot` changes; it did not.

## Context

[Q-M1d.7](../open_questions.md#q-m1d7--may-a-program-require-its-own-model-count)
in one line: `:expect (or M₁ … M_k)` says *the model set is exactly these k*, a
**test** may say that, and the question is whether a **puzzle** may.

The asymmetry that makes it a question rather than a preference is that the
**same s-expression** means two different things in two keywords:

| where | `(or A B)` means | quantifies over |
|---|---|---|
| `:match` | this world satisfies A or B | facts in *one* KB |
| `:expect` | the model set is exactly {A, B} | the set of KBs |

A rule fires **in a world**. That is not an implementation choice — it is what
`compile.rs` compiles, what `match_.rs` walks and what a firing's provenance
records — so "and there are no others" has nothing in the rule language to
attach to. The question is whether that is *a defect to fix or a boundary to
state*, and Q-M1d.7's prior is the second.

**The phase has a precedent for exactly this shape of question, one level
down.** [Q-M1d.2](../open_questions.md#q-m1d2--where-does-a-requirement-live)
asked where a *requirement* lives — kernel primitive, stdlib convention, or
rule shape — and the answer was **(c) a rule shape asserting a reserved verdict
atom**, form G, which works because a requirement *is* a sentence about one
world: `(open ?R)` says *this KB's R-extent is incomplete*. The analogous
question here has an analogous test, and this stage's job is to apply it
honestly rather than to assume the answer:

> **Is there a rule shape that expresses model-set closure?**

If there is, the answer to Q-M1d.7 is yes and P1d.2's machinery is the template.
If there is not, the reason will be that every rule shape reduces to a claim
about the KB it fires in, and **that reason is the boundary**, written once.

## The neighbours, and why they are evidence rather than decoration

Q-M1d.7's prior is that no language of this family lets a program constrain its
own model count. The stage checks it rather than repeating it, because "nobody
does this" is a claim about a literature, and the repo has most of that
literature catalogued: ASP and clingo in
[`docs/lib/02`](../../../docs/lib/02-solvers-csp-sat-smt.md) and
[`04`](../../../docs/lib/04-programming-languages.md), SMT and #SAT in `02`,
the proving end in
[`03`](../../../docs/lib/03-theorem-proving-formal-methods.md).

**Alloy is not in the catalogue**, and it is the one system whose nearest
mechanism looks most like a counterexample — so the survey either adds it to
`docs/lib/03` or records why it does not belong there. A gap found while using
the catalogue is the catalogue's own maintenance rule.

The four worth checking, and what each is expected to show:

| system | the mechanism that looks like it | why it probably is not |
|---|---|---|
| **ASP** (clingo) | `#count` aggregates, `#minimize` | aggregates count atoms **within** an answer set; optimisation ranks answer sets from outside the program's logic, and neither states *"there are exactly k"* |
| **Alloy** | `run … for N` / `check … for N` | a **scope**, and a command rather than a constraint — it bounds the search, it is not a sentence in the model |
| **SMT** (Z3) | the blocking-clause loop [M10](../../m10_external_benchmarks/README.md) uses | a *procedure* outside the formula; the formula never mentions how many models it has |
| **#SAT / projected model counting** | the count itself | an operation **on** a program, which is the meta level the question is asking about |

**The interesting one is Alloy**, because it is the closest thing to a
counterexample and it is not one: a scope is Ein's `--max-set-size`, not Ein's
`:expect`. If that reading survives the stage, it is the strongest single piece
of evidence the boundary is real and not an omission.

## Tasks

### Task T1d.4.2.1 — the rule-shape test, applied

Take Q-M1d.2's test and run it on this question. Is there a rule shape — any
shape, including ones the loader would have to grow — whose firing means *"the
model set is closed"*? Two candidate readings to refute or accept explicitly:

- **A rule that fires when no further model exists.** Its guard would be an
  `absent` over *models*, and `absent` is compiled as a sub-plan over the KB;
  there is no KB in which "another model exists" is a fact. Where the argument
  is refutable, refute it: [P1d.2's `(open ?R)`](../p1d.2_obligations/s1d.2.3_the_form.md)
  looked equally impossible until the verdict atom made it a claim about *this*
  KB.
- **A rule that fires on the search's own state**, the way the lattice's
  counters are observable. This is the one that has to be refused on *grounds*
  rather than on feasibility: the engine could expose `k` as a fact, and a rule
  reading it would make derivation depend on the traversal, which
  [S1a.7.0's invariant](../../../docs/history/m1a_rust/README.md) forbids — the
  answer depends on neither entering order nor integration time.

The second refutation is the load-bearing one and it should be written out in
full, because it is the reason the boundary is a boundary and not a cost.

### Task T1d.4.2.2 — the neighbour survey, with citations

The table above, checked and cited from `docs/lib/`, one paragraph per system
saying what its nearest mechanism actually quantifies over. Wrong entries are
the valuable output: a system that *does* let a program state its own model
count changes the answer.

**Scope discipline** — this is a survey, not a benchmark. No encoding is
written, nothing is run, and [M10](../../m10_external_benchmarks/README.md)
owns any claim that needs a solver installed.

### Task T1d.4.2.3 — the boundary, stated once

The written answer, in the form Q-M1d.2's was: the decision, the reason, and
what it implies for every future keyword rather than for this one. The shape it
should have if the answer is *no*:

> A rule is a sentence about the world it fires in. A claim about the *set* of
> worlds is a sentence about the search, and the search is not a thing rules
> may read — because a rule that read it would make derivation depend on the
> traversal. So closure claims live at the meta level, `:expect` is already
> that level, and the question is not where to put the claim but what the meta
> level can afford to check.

And the corollary that makes it useful rather than merely correct: **it settles
the same question for every second-order claim anyone proposes next** —
"exactly one model", "an even number of models", "the same models as that other
file" — none of which needs re-litigating once the reason is written.

### Task T1d.4.2.4 — Q-M1d.7 closed

With the neighbours cited, the rule-shape test's result, and — if the answer is
*no* — the hand-off to [S1d.4.3](s1d.4.3_the_vocabulary.md): the question stops
being *where does the claim live* and becomes *what can the meta level check,
and what does it say when it cannot*.

If the answer is *yes*, this stage stops and the phase re-plans: a program that
can state its own model count is a language change with reach past this
milestone, and it is not a thing to design in the stage that discovered it was
possible.

## Acceptance

- **[Q-M1d.7](../open_questions.md#q-m1d7--may-a-program-require-its-own-model-count)
  closed**, either way, with the reason written once and stated generally
  enough to cover the next second-order keyword.
- The rule-shape test is applied and its negative result argued, not assumed —
  including the traversal-dependence argument in full, since that is the reason
  rather than the symptom.
- The neighbour survey names four systems, cites `docs/lib/`, and records any
  case that contradicts the prior.
- **Alloy is either added to [`docs/lib/03`](../../../docs/lib/03-theorem-proving-formal-methods.md)
  or recorded as deliberately out of the catalogue** — it is the nearest thing
  to a counterexample and it is currently uncatalogued, which is a gap the
  survey found by using the catalogue.
- Nothing is implemented. This is a decision stage and its whole output is
  prose plus a question's closing entry.
