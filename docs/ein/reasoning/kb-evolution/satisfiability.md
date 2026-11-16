# Satisfiability: what a program can say of its answer

Saturation answers *what follows*. A puzzle also asks *which worlds are possible*: whether the program has a model,
and which. Ein's answer to the second is the region R, the commitments whose state is alive and discharged
([solution-semantics.md](solution-semantics.md)). This page states what a program can say about that answer from the
inside, in its rules, and where that stops:

- each rule is a sentence about the one state it fires in, and the two bounds on R are made of such sentences (§ 1);
- a sentence about the set of states is not a rule, and why no form of the language can be one (§ 2);
- such a sentence is a claim, stated at the meta level and checked against R (§ 3);
- an obligation's alternatives are jointly exhaustive, which is what lets a search branch on one debt (§ 4);
- a state of R is a model of the program as H scopes it, and the answer says where that scope ends (§ 5);
- a set of models has a compact exact form, and every count says what it guarantees (§ 6).

## 1. Two bounds, each a sentence about one state

**A restriction** is a rule that concludes `(false)`. A state that derives it is **dead**, and since a fork only adds
facts, a dead state stays dead under any addition on a program the static check calls monotone
([monotonicity.md](monotonicity.md)). Restrictions bound R from above: they set its **maximal line**.

**An obligation** is a rule whose whole `:assert` is `(open ?R)`, and whose `:match` holds while a witness is missing
([Q-M1d.2](../../../decisions/hypotheses/Q-M1d.2-an-obligation-is-a-rule-that-concludes-open-r.md)). A state at which
it matches **owes** that instance; a state that owes nothing is **discharged**. Obligations bound R from below: they
set its **minimal line**. `open` is written nowhere else
([Q-M1f.66](../../../decisions/language/Q-M1f.66-open-is-written-only-as-a-rules-verdict.md)).

The two are not symmetric, and the asymmetry is why the debt is not a fact. `(false)` can be stored, because a dead
state stays dead under any addition. A debt exists to be paid by an addition: a stored debt would survive its own
payment in the fork that paid it. So an obligation rule derives nothing and is in no saturation agenda; **one pass
over each quiescent state** reads it, and what it matches is state of the node, beside its dead bit. Whether a state
owes is therefore decided by that state alone, as whether it is dead is.

## 2. The second-order boundary

### 2.1 The question

The same s-expression means two things in two keywords:

| where | `(or A B)` means | quantifies over |
|---|---|---|
| a rule's `:match` | this state satisfies A or B | the facts of **one** state |
| a claim, `:expect` | the answer's states, or classes, are exactly those listed | the set of **states** |

A claim may say the second. May a program's rules: *there is exactly one answer*, *at most k*, *the same answers as
that other file*? **No** ([Q-M1d.7](../../../decisions/language/Q-M1d.7-a-rule-is-a-sentence-about-the-state-it-fires.md)).
A rule reads and concludes facts of the one state it fires in, and no form of the language reads another state, the
number of states, or the search's own progress. Three readings of the question show why, each refuted on different
grounds.

### 2.2 Three readings, and why each fails

**(a) A guard that is an `absent` over models: refuted by what a guard reads.** `absent` asks whether this state holds
a row matching a pattern, and every guard, present or future, is evaluated against the rows of the state the rule fires
over. *Another model exists* is a fact of no state and cannot be one: another model is another state. Incompleteness of
`R` *in this state*, which `(open ?R)` states, is decided by one pass over this state. Closure of the **set of
states** is a property of no state at all, only of the lattice of them.

**(b) A rule reading the search's own state: refuted on grounds, not on feasibility.** An engine could expose how many
states a run has found, or whether it has finished, as facts. A rule that read one would make **what is derived depend
on how far the search went**: the same program and facts would derive one set under one budget and another under a
larger one, so a budget would change what is provable. The answer is a property of the program, invariant under every
option, every order of the search and every number of workers
([Q-M1e2.30](../../../decisions/answer/Q-M1e2.30-no-configuration-option-lever-or-flag-changes-r.md),
[Q-M1e2.32](../../../decisions/answer/Q-M1e2.32-no-two-commands-disagree-and-the-oracle-is.md)). Where the search's
state is read and reported is downstream of every derivation: the read-out, which may say how much of R a run
classified. That separation is the design, and (b) would erase it.

**(c) A verdict atom, `(closed)` beside `(open ?R)`: refuted on evaluability.** A verdict atom is worth having only if
the engine can evaluate the claim it makes. `(open ?R)` costs one pass over a quiescent state per obligation instance.
`(closed)` would cost all of R: the whole search, which is the expensive thing the obligation exists to steer. The
affordability is not downstream of the vocabulary; it is the reason the vocabulary cannot exist.

### 2.3 The neighbours

Every system of this family puts the count of models at the meta level:

| system | what looks like a count in the program | what it quantifies over |
|---|---|---|
| answer set programming ([ASP](../../../lib/02-solvers-csp-sat-smt.md#answer-set-programming-asp), [clingo](../../../lib/02-solvers-csp-sat-smt.md#clingo)) | `#count`, `#sum`, `1 { p(X) : q(X) } 1`; `#minimize` | atoms **within** one answer set; optimisation ranks answer sets from outside the program's logic |
| [Alloy](../../../lib/03-theorem-proving-formal-methods.md#alloy) | `run p for 5` | a **scope**, a bound on the universe the analyser searches, given as a command, not a sentence of the model; `#r` counts tuples **inside** one instance |
| SMT ([Z3](../../../lib/02-solvers-csp-sat-smt.md#z3)) | the blocking-clause loop | a procedure **outside** the formula, which never mentions how many models it has |
| #SAT, projected model counting | the count itself | an operation **on** a program |

Alloy is the nearest, and it is not a counterexample: its bound is written in the file the modeller writes, but it is a
search budget given to the analyser. Ein's analogue of Alloy's scope is a budget given to the run, such as
`--region-budget`, not a claim and not a rule.

### 2.4 The boundary, stated once

> **A rule is a sentence about the state it fires in.** A claim about the set of states is a sentence about the
> search, and the search is not a thing rules may read: a rule that read it would make derivation depend on the
> traversal, and the traversal is a budget. So claims about the set of states live at the **meta level**, where a
> claim, `:expect`, already is, and the question is not *where to put the claim* but *what the meta level can afford
> to check*.

Three corollaries:

1. **It settles every second-order claim.** *Exactly one model*, *an even number*, *the models of that other file*,
   *at most k*: each is a sentence about the set of states, none is a property of any state in it, and (b) refutes
   each alike.
2. **It forbids no budget.** *How much to search* is not a claim about the answer at all; it is a run's budget, and it
   belongs with the run.
3. **It does not weaken a claim.** The meta level may state what a run cannot verify, provided the run says so: a
   claim the run did not decide is **not checked**, and is never read as held (§ 3).

## 3. Claims about the set of states

A claim is the meta level's sentence. A query's `:expect` lists the classes of the goal's rows over the states the
query asks about; the `(solve …)` form's `:expect` lists the states of R. A claim is an answer, never a pattern: its
facts are ground, and **naming a relation closes it**, so every relation a region claim names is listed at its
complete extent in every state, and a relation it never names is unconstrained
([Q-M1c.2](../../../decisions/language/Q-M1c.2-an-expectation-is-ground-and-naming-a-relation.md)). A claim lists all of
what it is about: there is no form for *at least these*.

A claim is checked against R, which the engine computes, and its outcome is one of three:

- **held**, where the answer the run computed lists exactly what the claim lists;
- **failed**, where it does not: a false claim is a result, not a refusal of the program, and the run exits 1;
- **not checked**, where the run did not decide it, because it asked another question of R or a budget stopped it
  first. A claim not checked is never read as held. It exits 1 like a failed one, so that a claim too costly to check
  is not mistaken for one that holds; under `--no-search`, which asks root alone and so asks nothing of R, it moves no
  exit code ([Q-M1f.257](../../../decisions/cli/Q-M1f.257-under-no-search-a-claim-not-checked-moves-no.md)).

## 4. Why an obligation's alternatives are jointly exhaustive

A search need not enumerate the subsets of H to find R. Where a state owes an instance, it can **branch on the
instance's alternatives alone**, the candidates that would pay it, and lose nothing. This section is why.

**The branch is jointly exhaustive.** An owed instance of `total-owed` says *`a` has an `R`-arrow to some member of
`B`*. Its alternatives are `{(R a b) : b ∈ ext(B)}`, less what is already refuted. Every member of R above the state is
discharged, so it pays the instance, so it holds one of the alternatives: the obligation says a model must. Branching
on the alternatives and continuing from each therefore reaches every member of R above the state. No death, no depth
and no ordering enters the argument. `surjective-owed` is the same statement read from the other argument.

**Mutual exclusivity is not needed.** Where `R` is also `functional`, the alternatives exclude one another and the
branch is a partition. Where it is not, one state may hold two alternatives and be reached by two branches: that is a
duplicate, not a gap, and R is a set of states, so it holds the state once.

**It holds under two conditions**, each checked rather than assumed:

1. **The alternatives cannot grow beneath the branch.** They are read at root; a domain a commitment could extend
   would hold, at some state, a witness root never saw. The generator declines to build H from obligations wherever that
   could happen ([hypothesis-generation.md](hypothesis-generation.md),
   [Q-M1e2.10](../../../decisions/hypotheses/Q-M1e2.10-the-obligations-rung-declines-where-its-domain.md)).
2. **Every witness is a candidate, or refuted at root.** A witness that is neither could pay the instance at a state
   no branch reaches. A route takes the branch only for an instance of which this is checked, at load and on every
   node it computes, and refuses the run where a node breaks it ([finding-r.md](finding-r.md) states the licence and
   the route).

**On a monotone program the branching ends.** An instance's trigger reads only relations no commitment changes, so a
state above root owes only instances root owes; a route checks this on every node it computes, and a node that owes
one root did not refuses the run. Discharged is upward-closed, so each child pays the instance it branched on and
every instance its parent had paid. The owed set therefore shrinks strictly along every branch, and no branch is longer
than the number of instances root owes.

## 5. A state of R is a model of the program as H scopes it

A state of R decides every candidate of H: each is in it or not. It need not decide a fact outside H. So **R is over
H**, and a run that classified every commitment, `exhausted`, certifies all of R over H and nothing beyond it
([Q-M1e2.3](../../../decisions/answer/Q-M1e2.3-r-is-over-the-universe-and-the-read-out-names.md)).

A state of R is a model of the **program**, and not only of the program as H scopes it, when every relation a
hypothesis could be about and H holds no fact of is either:

- **closed**: no rule concludes it positively, so no hypothesis about it could be confirmed; or
- **decided** by saturation from what the commitment settles.

`examples/ein-bugs/wrong-answers/uncovered.ein` is a program for which neither holds. Its obligations owe `seats` from
both ends, and H holds the `seats` candidates alone. `knows` is owed by nobody and not closed, since a rule concludes
it, and nothing decides whether anyone knows anyone: R's states mention no `knows`, and the same program with
`(knows Ann Bob)` added, `examples/ein-bugs/wrong-answers/uncovered-knows.ein`, has further consistent states. R is
right over H, and the certificate is true of H. **What it leaves out, every question names**: wherever H holds no fact
of a relation the program can conclude or ask about, and the program does not scope that relation out itself, the
read-out says *knows — no candidate: no obligation owes it. R holds of it only what the program states and saturation
derives*. That sentence is the certificate's scope, printed beside the certificate.

How much a listed state leaves open can be asked as well: the facts the blind enumerator would still propose there,
outside H. The count is read on a copy of the state, so asking it changes nothing else a run reports.

## 6. A set of models, and what its count claims

**A model** is what a state of R adds to the program: the positive part of the state, less the positive part of the
program as loaded ([Q-M1e.6](../../../decisions/answer/Q-M1e.6-a-model-is-what-a-solutions-state-adds.md)). The state
is what is judged; the model is a projection of it, and decides nothing
([Q-M1e.7](../../../decisions/answer/Q-M1e.7-a-state-is-what-the-engine-judges-and-a-model-is.md)).

**What models differ in.** A set of states varies in facts, and a compact description of it is about variables, so
facts are read as variables by two rules:

1. every positive fact that is in some listed state and not in another is a **Boolean** variable;
2. where the program declares a relation `functional` (or `bijective`, which implies it), the facts `(R a ·)` exclude
   one another, so for each `a` they are **one** variable whose values are the ones it takes.

The declarations are read from the states' own facts, so a program that states `bijective`, one that states
`functional` and one that derives the marker by a rule are read alike. A negative fact is no variable: where the
program stores `(not (R a b))` beside every excluded value, it mirrors the positive facts exactly.

**The determining key** is the smallest set of those variables whose values tell every listed state apart, with the
table of the combinations that occur, one row per state. By construction a row's values pick out its state among
those listed, and each row is read off a state that exists, so a state the run did not find could add a row but never
contradict one. Finding a smallest such set is a
minimum hitting set over the pairs of states, which is NP-hard, so the key is offered under a budget and, where it is
unaffordable, declined with the reason, leaving the listing as the form. It is a form asked for after the answer and
never a second answer: no count, state, claim or exit code moves with it
([Q-M1d.5](../../../decisions/read-out/Q-M1d.5-the-read-out-lists-the-states-each-count.md)).

**Every count says what it guarantees.** A run that classified all of what it was asked reports *the* states, of R or
of the line asked; one that a budget stopped reports the states *found*, a part, complete up to where it says
([Q-M1d.5](../../../decisions/read-out/Q-M1d.5-the-read-out-lists-the-states-each-count.md),
[Q-M1d.1](../../../decisions/answer/Q-M1d.1-a-search-stops-early-only-on-a-decided-question.md)). An unqualified count
would claim a completeness the run did not establish: on a set of states that is not one, *these are the models* and
*these are models found* are different statements, and only the first licenses a count.

## What holds it

| claim | test |
|---|---|
| an obligation rule derives nothing and never reaches the firing stream; a state that owes is not a solution | `ein-infer/tests/obligation_reports.rs`: `an_obligation_rule_never_reaches_the_firing_stream`, `a_state_that_owes_is_open_and_not_a_solution` |
| no option, lever or flag changes what is derived or which states are in R | `ein-cli/tests/lever_invariance.rs`: `every_declared_lever_cell_moves_no_line_of_any_mode` |
| no order of the search, interning order or number of workers moves an answer | `ein-infer/tests/explore.rs`: `every_order_of_the_tree_is_the_oracle`; `ein-render/tests/id_order_invariance.rs`: `every_rendering_reproduces_its_digest_and_none_depends_on_the_id_order`; `ein-render/tests/jobs_invariance.rs`: `jobs_does_not_move_any_observable` |
| naming a relation closes it in every state a claim lists | `ein-infer/tests/claim_semantics.rs`: `naming_a_relation_closes_it_in_every_model` |
| a claim the run did not decide is not checked; a run a budget stopped leaves its claim not checked and says why; under `--no-search` a claim not checked moves no exit code | `ein-infer/tests/expect_semantics.rs`: `a_claim_the_run_did_not_decide_is_not_checked`; `ein-cli/tests/test_cli.rs`: `a_budget_stopped_run_is_not_checked_and_says_why`; `ein-cli/tests/no_search.rs`: `test_decides_what_root_decides_and_a_claim_not_checked_exits_0` |
| every route that branches on an owed instance's alternatives finds the oracle's R | `ein-infer/tests/explore.rs`: `every_mode_is_the_oracle` |
| on the zebra family, branching on what the obligations owe finds what the hrule finds | `ein-infer/tests/obligation_rung.rs`: `the_theory_finds_what_the_hrule_finds` |
| every question names a relation H holds no fact of, in one sentence | `ein-cli/tests/read_out.rs`: `the_universes_scope_is_one_sentence_on_every_surface` |
| what the blind enumerator would still propose at a listed state is a read, off unless asked | `ein-cli/tests/leftover_probe.rs`: `the_leftover_probe_is_off_by_default_is_a_read_and_means_what_it_says` |
| an exhausted run reports the states, an unexhausted one the states found; the key changes stdout and nothing recorded, names its size and one row per state, and an unaffordable key says why | `ein-cli/tests/model_set_report.rs`: `an_exhausted_run_reports_the_states`, `an_unexhausted_run_reports_states_found`, `the_key_form_changes_stdout_and_nothing_recorded`, `the_key_table_names_its_size_its_alternatives_and_one_row_per_state`, `an_unaffordable_key_says_why` |
