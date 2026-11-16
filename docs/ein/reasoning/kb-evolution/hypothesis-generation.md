# Hypothesis generation: the ladder that builds H

Saturation decides what a program's facts and rules entail. What they leave undecided is guessed: a **candidate**
(a hypothesis) is a ground fact that a commitment may add, and **H**, the universe, is the set of them. Every node of
the answer's lattice is root with a subset of H committed, so H fixes what the answer can contain
([solution-semantics.md](solution-semantics.md) defines R over it; [finding-r.md](finding-r.md) says how each route
walks it).

This page states how H is built: once, at root, by one of three generators, chosen by what the program states. The
choice is called the **ladder**.

## 1. When H is built

**H is proposed at root's fixpoint**: the program's facts saturated to quiescence by its rules, with nothing
hypothesised ([Q19](../../../decisions/hypotheses/Q19-hypotheses-are-proposed-at-a-fixpoint-after.md)). It is built
once, and every node, every route and the oracle commit to subsets of that one set. Nothing proposes a candidate
while a saturation runs, and nothing a node derives adds one: propagation decides everything it can before anything
is supposed, and a candidate is proposed against what the rules have already derived.

**No option changes H.** It is built with no configuration switch applied, so no `(config …)` field, command-line
option or environment name reaches it, and with it none reaches R
([Q-M1e2.30](../../../decisions/answer/Q-M1e2.30-no-configuration-option-lever-or-flag-changes-r.md)).

**A program may state that there is nothing to guess.** `(solve :no-search)` says the lattice is root alone: no rung
runs, H is empty, and R is {∅} where root is alive and discharged and empty otherwise. It relaxes no conflict between
the mechanisms of § 6 ([Q-M1f.254](../../../decisions/hypotheses/Q-M1f.254-solve-no-search-relaxes-no-conflict.md)).

## 2. The ladder

| the program states | H comes from |
|---|---|
| `(solve :no-search)` | nothing: H is empty |
| any `(hrule …)` | **the hrules** (§ 3), an override |
| an obligation activated at root, and no hrule | **the obligations rung** (§ 4): the facts that would pay what root owes, and what a rule can bring a commitment to owe |
| neither | **the blind enumerator** (§ 5) |

The rung is decided once, at root, by one reading of the program, which every reader of the ladder shares
([Q-M1f.308](../../../decisions/hypotheses/Q-M1f.308-the-ladders-second-rung-is-taken-where-root.md)). Three
things about the middle row decide what it means:

- **It is about what the program states, not what root owes.** An obligation is *activated* at root where root's
  facts hold its activator, so that its plan compiles there; an obligation rule with no parameters is activated
  always. It is read as activated whether or not root owes anything under it
  ([Q-M1f.243](../../../decisions/hypotheses/Q-M1f.243-an-obligation-is-read-as-activated-owed-or-not.md)). Under an
  activated obligation a root that owes nothing proposes nothing, and is complete because it is discharged. Falling
  back to the blind enumerator wherever nothing was owed would make exhaustion, not discharge, decide when a state is
  finished, at exactly the states the obligation exists to judge.
- **It is about what the program activates, not what it loads.** Every import brings its module whole, obligation
  rules among it ([Q-M1f.247](../../../decisions/imports/Q-M1f.247-every-import-brings-its-module-whole.md)). An
  obligation rule nothing activates chooses no rung, so a module imported for another of its names leaves the
  program's own open relations to the blind enumerator, and a module imported for one rule takes the rung of that
  rule written out ([Q-M1f.249](../../../decisions/imports/Q-M1f.249-the-answer-does-not-depend-on-the-import-form.md)).
- **The obligations rung may decline** (§ 4.3), and then the blind enumerator builds H. It never proposes part of H
  and leaves the rest to another generator.

**Every rung's candidate is a fact root does not settle.** Whatever proposes it, a candidate root's fixpoint already
holds, or whose negation root's fixpoint stores, is not in H, and a fact proposed twice is in H once. H is a set: what
the answer depends on is which candidates it holds.

## 3. The hrules: what the program says to guess

An `(hrule …)` is a rule whose conclusion is a guess. At root it proposes, for each match of its `:match`, the one
fact its `:assert` makes; a conclusion of more than one fact is refused at load
([Q-M1f.271](../../../decisions/language/Q-M1f.271-an-hrules-conclusion-is-one-fact.md)). An hrule without parameters
is active always; one with parameters is active once for each activator the `(solve :hrules …)` list gives it. An
instance of `eq` or `neq` is no candidate, since a state stores neither
([Q-M1f.32](../../../decisions/hypotheses/Q-M1f.32-an-hrules-eq-or-neq-instance-is-no-hypothesis.md)), and an hrule
that reads a negation reads the state it was asked about, root, and no write made while it is being read
([Q-M1f.129](../../../decisions/engine/Q-M1f.129-an-hrule-that-reads-not-reads-the-state-it-was.md)).

**The hrules are an override.** A program that says what to guess is never second-guessed by what it owes: under a
declared hrule no other rung runs, and the two `(solve …)` lists filter nothing (§ 6). What an hrule proposes is what
its `:match` finds at root. A match only a node's state would have is in no candidate, since H is root's.

## 4. The obligations rung: what would pay a debt

### 4.1 An instance, and the facts that would pay it

An obligation rule's whole `:assert` is `(open ?R)` and its `:match` is the requirement's guard
([Q-M1d.2](../../../decisions/hypotheses/Q-M1d.2-an-obligation-is-a-rule-that-concludes-open-r.md)): `std.algebra`'s
`total-owed` matches `(absent (and (?isa ?b ?B) (?R ?a ?b)))`, *no witness exists*. An **instance** is a match at
root's fixpoint, and an instance is **owed** while its guard holds, that is, while no witness is there.

**The facts that would pay an owed instance are read off its own guard**, with the witness step skipped: for the
guard above, `{(R a b) : (isa b B) holds at root}`. Nothing restates the domain: the branch set is the guard, read at
root, and nothing stores it ([Q-M1d.3](../../../decisions/hypotheses/Q-M1d.3-an-obligation-needs-no-closed-domain.md)).
A witness whose step the loader cannot place, or two candidate witness steps in one guard, is refused at load where
it does not depend on the activator, and declines the rung where it does (§ 4.3).

### 4.2 H is the union

**H on this rung is the union, over the owed instances, of the facts that would pay each.** One instance's payments
are jointly exhaustive: the obligation says some witness must hold, so every member of R above a state that owes the
instance holds one of them ([satisfiability.md](satisfiability.md) gives the argument and its conditions). But H is
one set, read by every route and the oracle, so the rung proposes every instance's payments together, of which each
instance's own set is a subset. A route that branches on one instance at a time reads that instance's alternatives
inside H ([finding-r.md](finding-r.md)); nothing is lost by the union, and nothing outside it is proposed.

### 4.3 When it declines

A choice among an instance's payments is exhaustive only while the set of them cannot grow beneath it. Where it could,
**the rung declines and the blind enumerator builds H**
([Q-M1e2.10](../../../decisions/hypotheses/Q-M1e2.10-the-obligations-rung-declines-where-its-domain.md)):

- **a guard scans a relation the rung itself proposes**: a payment of one debt could add a member to another debt's
  domain;
- **a trigger, or a guard beside its witness step, reads a relation a commitment can change**: the relations a
  commitment can change are read off the rules, seeded with what the rung would propose and closed under the
  saturation rules, at relation grain, so a program that already names the value at root declines too. A domain value
  only a commitment derives is in no payment scanned at root, and an H built from those scans would lack the model
  that value licenses;
- **an obligation names no relation to pay**: a bare `(open)` owes and is reported, and there is nothing to branch
  on;
- **a witness step does not resolve under its activator.**

**One declined obligation declines the whole call.** Falling through for that obligation alone would leave its
payments proposed by nobody, and a model needing one would be in no node with nothing saying so. Declining all or
nothing is what makes the rung's claim unconditional: where it builds H, H holds every payment of every owed
instance, and where it cannot, the blind enumerator, which reads no domain, builds H instead. The decline is
reported with its reason: *an obligation scans `food`, a relation a commitment can change*.

### 4.4 A debt root is not told of

An obligation's activator is an ordinary fact, so a rule can derive one at a node, and that node owes what root never
did. **The rung also proposes, at root, what such a debt would**, read off the rules with no saturation
([Q-M1e.11](../../../decisions/hypotheses/Q-M1e.11-h-holds-what-a-debt-only-a-commitment-brings.md)):

1. the **activators a commitment can derive**: each conclusion that holds no variable and would activate an
   obligation rule, in the `:assert` of a rule that reads a relation a commitment can change, or is activated by one;
2. for each such activator root does not hold, **the rung's own scan at root** of the instances its plan matches,
   each guard read with its witness step skipped, as for a debt root owes;
3. a candidate that adds a relation widens what a commitment can change, so **both steps repeat until they add
   nothing**.

A plan read this way is held to every decline of § 4.3 beside root's own, and its payments come after what root owes.
The read-out names what H holds for each such debt. The reading is **at relation grain and inexact both ways**:
*over*, a rule that reads what a commitment can change need fire at no alive node, and H then holds a candidate for a
debt no node owes, which a state of R holds wherever nothing refutes it; *under*, an activator its rule writes with a
variable is not read, and the node that derives it owes what no candidate pays. A program this reading adds a
candidate to derives an activator from a commitment, so the static check flags it
([monotonicity.md](monotonicity.md)), and its answer is the cube's.

### 4.5 Stuck: owing, with nothing to propose

A root that owes an instance every payment of which is already refuted at root, its negation stored, proposes nothing
for it. Where that leaves the rung nothing to propose at all, the state is **stuck**: it owes, nothing can pay, and
the generator says so rather than letting an empty proposal pass for completeness. A state that held a payment would
hold a fact whose negation root stores, and be dead, so no alive state discharges the instance: R is empty, the kind of
empty where alive states exist and every one owes. A list or a closure that would take out the relation an obligation owes cannot
make a state stuck: the program is refused at load
([Q-M1f.244](../../../decisions/hypotheses/Q-M1f.244-an-obligated-relation-is-not-closed-whatever.md)).

### 4.6 What the rung leaves to saturation

**The ladder is exhaustive where obligations and saturation between them decide every remaining open fact.** The
relations a hypothesis could be about, under the blind enumerator's eligibility (§ 5), split in two: those some
obligation owes, whose payments the rung proposes, and the rest, which the rung leaves **uncovered**. Where none is
uncovered, the rung is exhaustive by construction: every fact a hypothesis could be about is one some debt can be paid
with. Where some are, the claim rests on saturation deciding them: a relation no rule concludes positively can never
be confirmed by a hypothesis, and one the program's rules derive from what the obligations settle is decided by them.

The zebra family shows the first kind of program. `examples/zebra2-obligations.ein` is `examples/zebra2.ein` with its
`(hrule …)` and its `:hrules` clause deleted and nothing else: `(bijective color-loc)` and its kin owe every attribute
arrow, and the relations no obligation names are closed or derived. The two files build the same H, so they have the
same R. Whatever rung built H, every question names the relations the program can conclude or ask about of which H
holds no fact: R holds of them only what the program states and saturation derives
([satisfiability.md](satisfiability.md)).

## 5. The blind enumerator

Where the program states neither an hrule nor an activated obligation, or the obligations rung declines, **every slot
of every open relation is filled with every object**, and the program's own rules prune what is wrong.

**The objects** are the program's names: every name its facts hold at root's fixpoint, at any depth, and every
constant the program writes anywhere, a rule's `:match` and `:assert` included
([Q-M1e.21](../../../decisions/hypotheses/Q-M1e.21-a-rule-brings-in-no-name.md),
[Q-M1e2.9](../../../decisions/language/Q-M1e2.9-a-programs-names-are-the-constants-its-facts.md)). From them are
taken out ([Q-M1f.306](../../../decisions/hypotheses/Q-M1f.306-the-blind-enumerators-objects-are-the-programs.md)):

- every name a relation signature declares as a type;
- the reserved words of the rule language: `not`, `false`, `and`, `or`, `absent`, `eq`, `neq`;
- the kernel's two dunders, `__closed__` and `__symmetric__`, whether a rule derives a fact one heads or a program
  writes one as a value;
- every name of a relation the program declares or a file writes a fact of, of a rule, and of an obligation
  ([Q-M1f.226](../../../decisions/hypotheses/Q-M1f.226-an-obligations-name-is-no-candidate-object.md)), and the
  loader's own relation `written`.

A relation's name that only a rule's conclusion heads is neither, so it is an object; the read-out names
each candidate that binds one, and declaring the relation takes it out. Since every constant a rule writes is an
object, no rule can bring a name into a state that H lacks: the **alive-set invariant**, *no rule asserts a name the
program lacks*, holds of every program by construction ([saturation.md](saturation.md)).

**The relations** are every relation the program declares with a signature, less those the program scopes out (§ 6).

**The candidates**, for each such relation:

| arity | candidates |
|---|---|
| 1 | `(R a)` for every object `a` |
| 2 | `(R a b)` for every two distinct objects `a`, `b`: no self-edge |
| 3 or more | none: such a relation is outside the universe, and every question names it |

less what root settles (§ 2).

**The enumerator is untyped**
([Q-M1e.12](../../../decisions/hypotheses/Q-M1e.12-the-blind-enumerator-is-untyped-and-a-type-it.md)). It imposes no
type system: a candidate the program's types would reject is killed by the program's own rules, or stays in R where
none rejects it. A name a signature declares as a type is no object, so a program whose signature admits a type in a
slot is under-specified, and Ein answers it as written: beside `(is-a Red Color)` and `(relation co-located T T)`,
`(co-located Blue Color)` is a candidate. Where a candidate binds as an object a name that stands on the right of an
`is-a` fact, every question names it, and nothing else moves. The program's way out is its signature.

## 6. Scoping, and the mechanisms judged together

A program steers what a commitment may be about by five declarations: an `(hrule …)`, an obligation, the two
`(solve …)` lists `:hypothesis-relations` (only these relations) and `:no-hypothesis` (never these), and
`(__closed__ R)`, stated or derived, which says no commitment extends `R`. **The three scopings bind the obligations
rung and the blind enumerator alike**, and a relation they take out has no candidate. They do not bind an hrule: a
program that declares what to guess gets what it declared.

**The declarations do not contradict one another**, judged on root's fixpoint, at load, on every command
([Q-M1f.245](../../../decisions/hypotheses/Q-M1f.245-the-hypothesis-mechanisms-are-judged-at-load-on.md)). A pair
that says opposite things of one relation is refused: an active hrule proposing a relation a list takes out or a
closure closes; with no hrule, an obligation owing a relation a list takes out; an obligation owing a closed relation,
whatever generates ([Q-M1f.244](../../../decisions/hypotheses/Q-M1f.244-an-obligated-relation-is-not-closed-whatever.md));
a relation in both lists, or listed and closed. A declaration that contradicts nothing and filters nothing, a list
beside an hrule or a redundant name, is a note
([Q-M1f.242](../../../decisions/hypotheses/Q-M1f.242-a-hypothesis-declaration-that-filters-nothing-is.md)). The
refusals and notes, each worded, are [`defined_behaviour.md` § 4.27](../../../kernel/defined_behaviour.md).

## 7. What H does not depend on

- **The import form.** A program answers alike with its imports selective, whole or aliased, and the rung it takes is
  the same ([Q-M1f.249](../../../decisions/imports/Q-M1f.249-the-answer-does-not-depend-on-the-import-form.md)).
- **How a closure is stated.** A `(__closed__ R)` a rule derives and one a program writes close the same relation and
  give one H and one R.
- **Any option** (§ 1).

## What holds it

| claim | test |
|---|---|
| the ladder dispatches by what the program states: an hrule overrides, an activated obligation takes the second rung, a declined obligation falls to the blind enumerator, a program with neither takes the blind enumerator, and `(solve :no-search)` runs no rung | `ein-infer/tests/obligation_rung.rs`: `the_ladder_dispatches_by_what_the_program_declares` |
| under an activated obligation a discharged root proposes nothing | `ein-infer/tests/obligation_rung.rs`: `a_discharged_state_owes_nothing_and_declines_nothing` |
| a module imported for one rule takes the rung of that rule written out; the answer does not depend on the import form | `ein-cli/tests/import_invariance.rs`: `a_module_imported_for_one_rule_takes_the_rung_of_the_rule_written_out`, `every_corpus_program_answers_alike_under_every_import_form` |
| the obligations rung's candidates are its guards' scans with the witness step skipped | `ein-infer/tests/obligation_rung.rs`: `the_candidates_are_what_the_guard_scans` |
| the hrule path and the obligation path build one H, and with it one R, on the zebra family | `ein-infer/tests/obligation_rung.rs`: `the_theory_finds_what_the_hrule_finds` |
| a root that owes and can propose nothing is stuck, reports it, and is no state of R | `ein-infer/tests/obligation_rung.rs`: `owing_and_unable_to_branch_is_stuck_and_says_so` |
| H holds the payments of a debt only a commitment brings, by the static reading, and R over that H is the oracle's | `ein-infer/tests/later_debts.rs`: `the_universe_is_the_static_readings_on_every_probe`, `r_over_the_readings_universe_is_the_oracles`, `the_rung_line_counts_a_later_debt_apart` |
| every name of the program is a blind candidate object, and no run reaches a name the program lacks | `ein-infer/tests/alive_invariant.rs`: `every_name_of_the_program_is_a_candidate_object`, `no_run_reaches_a_name_the_program_does_not_have` |
| a kernel dunder is no object, and a closure by a rule answers what the closure by a fact answers | `ein-infer/tests/dunder_objects.rs`: `a_dunder_written_as_a_value_is_no_object`, `a_closure_by_a_rule_answers_what_the_closure_by_a_fact_answers` |
| a type bound as an object, and a relation's name bound as an object, are each named on every surface | `ein-cli/tests/read_out.rs`: `a_type_bound_as_an_object_is_one_sentence_on_every_surface`; `ein-cli/tests/relation_names_row.rs`: `a_relations_name_bound_as_an_object_is_one_sentence_on_every_surface` |
| a relation of arity 3 or more has no blind candidate and is named outside the universe | `ein-cli/tests/read_out.rs`: `the_universes_scope_is_one_sentence_on_every_surface` (`examples/ein-bugs/wrong-answers/arity3.ein`) |
| an hrule that reads a negation proposes what the oracle's does | `ein-cli/tests/hrule_reads_not.rs`: `an_hrule_that_reads_not_is_answered_as_the_oracle_answers`, `every_hrule_reads_the_state_it_was_asked_about` |
| an hrule's conclusion is one fact | `ein-cli/tests/conclusion_shapes.rs`: `an_hrule_concludes_one_fact` |
| the mechanisms' conflicts are refused on every command, a dead or redundant declaration is a note, and `:no-search` relaxes no conflict | `ein-cli/tests/mechanism_conflicts.rs`: `a_conflict_is_refused_on_every_command`, `a_dead_or_redundant_list_is_a_note_and_a_silent_pair_says_nothing`, `no_search_relaxes_no_conflict` |
| no option changes R | `ein-cli/tests/lever_invariance.rs`: `every_declared_lever_cell_moves_no_line_of_any_mode` |
