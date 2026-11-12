# P1g.7 — Unsatisfiable against owing: what an open state's explanation is, and what a finished search may call it

**Milestone:** [M1g](../README.md)
**Status:** planned 2026-11-12, on the user's replies of that day to the calculator's questions. Not started.
No stage file yet: the phase is this README, and its stages are sections below, written out as files when
[P1g.1](../p1g.1_survey_and_design/README.md)'s stop has settled the rest of the milestone.
**Depends on:** P1g.1's stop; of M1f,
[S1f.6.26](../../m1f_hypothesis_and_documentation/p1f.6_calculator/s1f.6.26_what_root_owes.md), the read-out
row for what root owes, which is the small part of the subject that was ruled and is built first; and
[P1f.8's S1f.8.4](../../m1f_hypothesis_and_documentation/p1f.8_school_algebra_and_geometry/s1f.8.4_linear_equations.md#three-methods-for-one-equation-2026-11-12),
which shows the same equation dead by one method and open by another.
**Needed by:** [S1g.2.2](../p1g.2_propositional_logic/s1g.2.2_valuations_as_hypotheses.md), where
*unsatisfiable* is read off R, and [S1g.3.3](../p1g.3_proof_systems/s1g.3.3_natural_deduction.md), where a
refuted assumption is read off N.

## Where it comes from

The question was asked three times in M1f's calculator phase (a circuit, an equation, and this milestone's
own probe 1b) and merged as question 24 of
[`questions.md`](../../m1f_hypothesis_and_documentation/p1f.6_calculator/questions.md#q24): *an
unsatisfiable search reads as "owing", not as "dead": is that the wanted reading?* When the unknowns are
hypotheses that each owe a value, every complete assignment is dead but the partial ones are alive and
merely cannot be completed. So R is empty as *every alive node owes*, `ein solve --single` answers `Open`
with *decided: yes*, and the read-out prints the owing nodes and **no row for N**, the minimal dead sets
with their cores, which is the explanation. Its three options were (a) keep the words and add the N row;
(b) a program can say that a debt no extension can pay is a dead state; (c) leave everything.

The user, 2026-11-12, chose none and said why:

> solution of `x × 7 = 862` belongs yet to arithmetic or already to algebra? this question open vs. false is
> deep.
> What would give more benefits, I expect `open` will, but why? Can we change it quickly later? We already
> have core for dead state, do we have anything like this for open? What it could be used for, I remember we
> already use it for hypothesis generation? It is not the case here - we would not generate hypothesis like
> (let x <number>) for every numbers already present in KB. It looks like an entire separate phase.

and, on how a missing quantity is detected (question 28): *"see my answer in question 24; in general - need
to discuss, analyze, needs a separate stage."*

This is that phase.
[Round 2, items 24, 28 and 37](../../m1f_hypothesis_and_documentation/p1f.6_calculator/questions_round2.md#q24)
answers the reply's questions in outline and proposes it: *a phase of M1g, "unsatisfiable against owing":
the open state's core, the read-out row for N, and whether a finished search may call an unpayable debt
dead. M1g's first probes already met it.* The outline, which the phase starts from and re-takes on
programs:

- **Why the same question is `false` one way and `Open` another.** Where the program's own facts contradict
  each other the root is dead. Where each digit is a hypothesis, no fact contradicts another at the root:
  root is alive and merely owes. Every complete way of paying is dead, so R is empty, but root is not.
  `Open` was introduced (M1d) to mean *not yet a model*; here the search has shown more, *no way of paying
  exists among the candidates*, and the word does not say it.
- **What explains an open state, as a core explains a dead one.** Two things exist today: *what is owed*,
  the obligation instances, which the hypothesis generator reads; and *why it cannot be paid*, the minimal
  dead sets N, each with its core. Together they are the proof. They are written to `nogoods.json`, are not
  in the read-out, and **nothing assembles them into one object, *this debt, and for each alternative the
  refutation that kills it*. That object would be the open state's core.**
- **Which gives more.** `Open` keeps more: the partial state stays inspectable, the gap is named, and a
  text that contradicts itself is told apart from a requirement nothing satisfies. `false` gives one proof
  object and propagates: a hypothesis whose every completion is dead becomes dead itself. They are not
  exclusive.
- **Can it be changed later: yes, cheaply. R is the same set under both readings, empty.** Only the label
  on states *outside* R and the explanation printed would change, so no answer, claim or register row about
  R moves. *That is the reason not to decide now.*

## What it is not

- **Not a change to R.** Whatever is ruled here, the region of every program is the set it is today. The
  acceptance says so and a sweep holds it.
- **Not F21.** Whether a refutation may rest on an `absent` and what a non-monotone program means are
  [F21](../../followups/f21_non_monotone_programs_decided.md)'s. This phase's programs are monotone: the
  debt is the engine's, not a fact a rule reads
  ([round 2, item 30](../../m1f_hypothesis_and_documentation/p1f.6_calculator/questions_round2.md#q30)).
- **Not a SAT solver.** Propagating *dead because every completion is dead* is what a solver learns from;
  the phase says what it would cost and what it would buy, and
  [S1g.6.6](../p1g.6_engine_requirements/s1g.6.6_exactly_one_value_per_object.md) is where a cheaper search
  for one common shape is planned.
- **Not built before the user reads it.** Every stage below ends in programs with computed answers shown
  first; the second and third change what a surface prints or what a word means, and wait for a ruling.

## The stages

### S1g.7.1 — The witnesses, on one page

The programs on which the two readings part, each banked with every surface it prints today (the read-out,
`--single`, `--trace`, `nogoods.json`, the summary): `x × 7 = 862` by search (28 dead sets in the dump,
`questions.md` 24) and by saturation (a dead root, a three-row core); the banked `unsatisfiable.ein` and
`equivalent.ein` of `examples/binary_logic/`, which claim the owing frontier and `(false)` for the query
(E-23 of the calculator's
[`findings.md`](../../m1f_hypothesis_and_documentation/p1f.6_calculator/findings.md)); this milestone's
probe 1b, `(And p (Not p))` required; a word problem whose gap is a debt; and E-18's pair, on which a
query's `:expect (false)` holds though root only owes. The user's four questions are each answered on these
programs and not in prose: what `Open` keeps, what `false` would give, what exists for an open state today,
and what the obligations are used for (hypothesis generation, and why it does not propose `(let x NUMBER)`
for every number in the KB: the rung proposes what would pay an *owed instance*, digit by digit).

### S1g.7.2 — The open state's core

The object round 2 names: for an alive node that owes and cannot pay, the debt, and for each alternative
that would pay it, the refutation that kills it, assembled from the owed instances and N. Designed on the
witnesses first: its shape as data (a tree, since killing an alternative may itself rest on a debt that
cannot be paid), where it is written (`--dump-answer`, beside `nogoods.json`), what the read-out prints of
it (the N row of question 24's option (a), or the assembled object, within the read-out's line budget), and
what taking it costs. *Minimum work per question* applies: it is taken only where a surface prints it.
A core is an explanation and may vary with rule order (the user's ruling of 2026-11-08); the object
inherits that.

It changes what `ein solve` prints for an R that is empty by owing, so
[`read_out.md`](../../../docs/kernel/inference/read_out.md) and its reference test move in the same commit,
and the user sees the rows on real programs before the printer moves.

### S1g.7.3 — May a finished search call an unpayable debt dead?

The ruling the user deferred ([Q-M1g.13](../open_questions.md#q-m1g13--may-a-finished-search-call-an-unpayable-debt-dead)).
The reading round 2 offers: *a debt that provably cannot be paid within the candidates is a refutation*,
which turns `Open` into a dead state **exactly where the search has finished**. The stage prices it and
shows it; it does not assume it:

- what *finished* means for each mode (`--single`, `-n`, describe, a run cut at its budget), and that a run
  which did not finish may never say it;
- what *within the candidates* means: H is over the names the program has, so *no way of paying among the
  candidates* is not *no way of paying*, and the word must not claim the second. A word problem whose
  missing amount is simply not stated owes for a different reason than an equation with no root, and the
  page shows whether the engine can tell them apart (the first has no candidate that would pay; the second
  has candidates, all dead);
- what moves: the verdict words of `--single`, the claim forms (`(false)` on a query and on `(solve …)`,
  `(region (owing …))`), E-18's question, every corpus program whose read-out says *owes*, and the
  `tests/stdlib/` programs that report `Open`. Each listed with today's output and the output under the
  reading, **before** anything is built;
- whether propagation (a node whose every completion is dead is dead) is part of the reading or a separate
  optimisation.

Whichever way the user rules, the acceptance is the same: R unmoved on every program.

### S1g.7.4 — What the logic reads off it

With the ruling: [S1g.2.2](../p1g.2_propositional_logic/s1g.2.2_valuations_as_hypotheses.md)'s table row
*F is unsatisfiable: require F; R is empty, and N's cores say why* restated to what the read-out now shows;
the bridge to [S1g.3.3](../p1g.3_proof_systems/s1g.3.3_natural_deduction.md) and
[Q-M1g.5](../open_questions.md#q-m1g5--is-a-refuted-assumption-a-fact) (a refuted assumption is reported and
not concluded), which is the same question one level up: there a *hypothesis* is dead and root holds
nothing of it; here a *debt* is unpayable and root is not called dead.

## Acceptance

- The witnesses are banked, each with every surface, and the user's four questions are answered on them.
- The open state's core exists as data and on the surfaces ruled, or the page says why it was not built.
- The user has ruled on Q-M1g.13, with the list of what moves in front of them.
- **R, N and H are the same sets on every corpus program before and after,** held by a sweep and by
  `ein oracle`; every claim's outcome is the same, or the user's ruling names the claim forms it changes
  and each changed outcome is a register row first.
- The kernel pages that describe a changed word or row change in the same commit.

## The discipline

[P1g.6 § The discipline](../p1g.6_engine_requirements/README.md#the-discipline) applies whole: a wrong
answer is a register row before it is fixed; config never changes an answer; every answer exits 0, and a
new or changed outcome word is placed by the user; one tool, consistent answers; minimum work per question;
every sweep names the goldens it expects to move before blessing.
