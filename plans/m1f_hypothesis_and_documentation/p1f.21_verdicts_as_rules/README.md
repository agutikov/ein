# P1f.21 — the kernel's verdicts as rules

**Milestone:** [M1f](../README.md)
**Status:** created 2026-11-17 on the user's instruction (below). [S1f.21.1](s1f.21.1_the_survey_and_the_design.md)
is done: [`design.md`](design.md) is its survey and design, and its question is put as
[Q-M1f.390](../open_questions.md#q-m1f390--what-an-obligation-emits-the-verdict-with-its-justification-or-as-a-fact),
**not ruled**. The phase stops here for the user's ruling. **2026-11-18**: the user's second instruction
([below](#the-second-instruction)) takes the design further: the kernel's verdict a fact a rule concludes,
`(ein.kill)`; the open facts the hypotheses themselves, which rules read, moved out of the open context as a state
decides them; and the search as rules. Worked out in [`open_facts.md`](open_facts.md); S1f.21.4 is planned to
take it onto programs, **not started**; S1f.21.2 and S1f.21.3 wait on it, and the close is S1f.21.5.
**2026-11-19**: on the user's fifth instruction ([below](#the-fifth-instruction)) each group of
[`modifications.md`](modifications.md) is planned as a stage ([§ Stages](#stages)), S1f.21.3 and S1f.21.4 are
folded into them, [`overview.md`](overview.md) is the distilled overview **for review**, and the instruction's
idea, hypotheses as ordinary facts, is put as
[Q-M1f.549](../open_questions.md#q-m1f549--hypotheses-as-facts-held-in-which-reading-and-how-far-the-search-goes-into-rules)
with runs ([review item 34](q-m1f.549_hypotheses_as_facts/README.md)). The same day the user ruled Q-M1f.399
([below](#the-answer-on-the-names)): no prefix, `(false)` a convention, and what ends a state a rule's key `:ein`,
its word ruled `inconsistent` (Q-M1f.550), S1f.21.10 unblocked to build it; and **started S1f.21.6 and S1f.21.8**
(*"start s1f.21.6 and s1f.21.8"*). **S1f.21.6, S1f.21.8 and S1f.21.10 are done** (the same day), the second merged, its
`std.debts` later the same day on the user's ruling (`edbe6c3b`). **S1f.21.11 is next, on the user's word.**
**Depends on:** nothing now. Its second stage writes into pages of `docs/ein/reasoning/02-kb-evolution/` that
S1f.20.10, the last stage of P1f.20, was still moving (the diagrams into files, the formulas onto several
lines); that stage is done and P1f.20 is closed (2026-11-17), so the pages stand as S1f.21.2 will find them. What
P1f.20 found on those pages and left is [the milestone's findings page](../findings.md), L-82 and L-83 among it.
Its first stage reads the engine and writes here. P1f.22, nested facts, is done and its folder deleted: what this
phase reads of it, the user's notes verbatim, a contradiction in a context, the double negation's elimination as a
rule, the names of `(false)` and `(open)` and questions as facts, is
[`open_facts.md` § 10](open_facts.md#10-what-p1f22-worked-out-that-this-phase-reads), copied from its page at its
close, and its finding that is this phase's is [V-7](findings.md#v-7--a-rule-can-conclude-a-fact-of-a-relation-the-program-does-not-count).
**Blocks:** the close of M1f, which was started on the user's acceptance of the documentation tree and paused
on the user's word; it resumes when this phase is done.

---

**Review material:** [`review/`](review/README.md): **every question of the phase that is open** (written there
2026-11-19 on the user's word), sorted by the phase's two parts, each with its options, its default where one
stands, and runs on programs where a run shows it; in two folders, a part each, with an answers' file each ([`part-1/ANSWERS.md`](review/part-1/ANSWERS.md),
[`part-2/ANSWERS.md`](review/part-2/ANSWERS.md)). Moved
here from the milestone's review bundle on the user's word (2026-11-18).

**The modifications, listed:** [`modifications.md`](modifications.md), written 2026-11-19 on the user's word:
every modification the phase's pages propose, 37 items in nine groups, the simplest and least affecting first,
each with the page it is from, its state and what it waits on. A reading; it rules nothing and runs no stage.

**The overview, for review:** [`overview.md`](overview.md), written 2026-11-19 on the user's word: the phase in one
sentence; where a negated fact, a refuted commitment, a hypothesis and a debt are held today, each on a run; what
the phase would change in the language, the kernel and the standard library; hypotheses as facts, with a
recommendation; the stages; what is the user's to rule.

## The instruction

The user, 2026-11-17, verbatim:

> in docs/ein/reasoning/02-kb-evolution - add contradiction rule defined as ein rule from fact and not fact infer
> (false); also can define owed/discharged and alive/dead rules; can say that if treat (open ?R) as facts then
> they are only facts that (1) duplicates (2) can be removed from KB; but maybe better would be to emit (open)
> facts as (open ?R <justification>) ? create new phase in M1f for that

## The second instruction

The user, 2026-11-18, verbatim:

> meanwhile - update p1f.21: (false) controls not search but saturation, right? if we have dead node then
> somebody has to kill it, maybe (ein.kill) or (ein.die) or (ein.saturation.kill) ? Also - instead of emitting
> (open ?R) emit (open (?R ?a ?b)) - the actual hypothesis. There is no problem with kernel read them, count and
> use. Then program with obligations also would have all hypotheses in the KB also (but under open). What kind of
> rules then could work with open facts? Can we completely replace/reimplement search with rules? But they have
> to be removed from KB during discharging. How removing of open facts can look like? Can it be flexible or
> transactional move from open into global true context or into not context?

Each question is answered in [`open_facts.md`](open_facts.md), on runs where the engine is concerned:

- **`(false)`** acts on a saturation, and the search reads only the verdict on the state.
- **The kernel's fact** is a verb a rule concludes, `(ein.kill)` the leaning.
- **`(open F)`** is $H$, and a debt is a disjunction of open facts. So the form that keeps the count is
  `(open F J)`, J the debt.
- **The move** is the definition of *open* read on each state, made in one step per node after its fixpoint,
  into held or into not by whatever decides F.
- **Rules over open facts** run in a stratum after the held fixpoint and conclude no held fact.
- **The search**: every decision of it can be a rule, but the split cannot; it stays the kernel's one primitive,
  or each state becomes a context.

## The third instruction

The user, 2026-11-18, verbatim:

> also ideas for p1f.21: what reserved names will be left? false - not anymore; rule - yes; relation - is not
> actually, it's just a convention already now - any new head of tuple will be new relation atom; eq,neq - neq
> needed, but eq not so much because we use one variable for equal objects in rules; not - will become a
> convention also with contradiction rule; what else? what is read by kernel from KB to make decisions about
> saturation or search?

Answered in [`open_facts.md` § 8](open_facts.md#8-the-reserved-names-that-are-left-and-what-the-kernel-reads),
with a census of every name the kernel reads, and from where. Each name of the instruction is as it says, but for
two points:
- `eq` has one use substitution does not keep: `std.math`'s test that an operand is a name and not a fact.
- `relation`'s declaration keeps the count of arguments and the `:why`.

The rest: `hrule`, `__closed__`, `__symmetric__`, `open`, `is-a`, and `solve`'s two lists.

Then, the same day:

> is-a is just some random relation, or maybe not even a relation; what for is it read by kernel read-out? Ok, so
> eq still required, leave as it is. Do eq/neq correctly compare nested facts? they should I guess.

- **`is-a`** is read by its name for one read-out row (Q-M1e.12), which decides nothing:
  [V-6](findings.md#v-6--the-read-out-reads-is-a-by-its-name), with the leaning to drop it.
- **`eq` stays.**
- **On nested facts, both compare by the object**: at any depth, a negation, a derived fact against a written one,
  and a false `eq` concluded kills.
  [`open_facts.md` § 8.4](open_facts.md#84-eq-and-neq-on-nested-facts) has the run. A free `neq` ranges over the
  program's atoms, never over a fact, and `eq` does not take a fact apart: a side with a variable inside is refused
  at load.

## The fourth instruction

The user, 2026-11-18, verbatim:

> p1f.21: `not` and `relation` became an stdlib convention, not kernel

and, the same day:

> p1f.21: `false` also becomes stdlib convention; argument-count check - will think about it in p1f.21, record as
> open question;

and:

> p1f.21: `eq` and `neq` stay kernel

It rules § 8.2's two rows of [`open_facts.md`](open_facts.md#82-each-name-under-this-phases-direction) that the
third instruction left as a direction (*"relation - is not actually, it's just a convention already now"*, *"not -
will become a convention also with contradiction rule"*): **`not` and `relation` are the standard library's
conventions, not the kernel's, and so is `false`.** **`eq` and `neq` stay the kernel's.** The loader's argument-count check, which reads `relation`'s
declarations, is **an open question of this phase**, put in `open_questions.md` as [Q-M1f.447](../open_questions.md#q-m1f447--the-loaders-argument-count-check-once-relation-is-a-convention) (the user will think about it here).
What reads any of the three today, and what each reader becomes, is S1f.21.4's
([§ 9](open_facts.md#9-what-s1f214-answers), item 6). The same day, on P1f.23's review, the user ruled that the
kernel reads no theory's name and dropped the built-in symmetry (`__symmetric__`); P1f.23's S1f.23.4 built that.

## The fifth instruction

The user, 2026-11-19, verbatim:

> plans/m1f_hypothesis_and_documentation/p1f.21_verdicts_as_rules/modifications.md - plan sections from this list:
> A, B, C, ... as stages; make a distilled overview of proposed modification of the main parts: language, kernel,
> stdlib; and write down it for review; also explain is now negated/refuted facts are stored in kb under (not ...)
> context or they are stored somehow outside of the kb? the same about hypotheses; Now generally this p1f.21 phase
> is about one more step of unification, kernel minimization, making ein more flexible - for instance we made (did
> we?) negated facts just conventionally hidden in (not ...) context, what if we would make hypotheses just
> ordinary facts hidden in (open ...) context? it is different case because kernel does search and has to make
> commitment sets from them, but kernel anyway has access to all facts, but maybe if we will get benefits from
> having hypotheses as facts? E.g. I see 2 potential options: 1. search control from inside ein 2. implement search
> with rules inside one stage, have search artefeacts seen by rules and infer facts from search. This is only an
> raw idea. But we can establish the foundation for this in this phase or can decline it for some pragmatic reason.

Each part, and where it is answered:

- **The groups as stages**: [§ Stages](#stages), a file each.
- **The overview**: [`overview.md`](overview.md), [§ 3](overview.md#3-the-modifications-distilled) by part.
- **Where a negated fact is held**: in the knowledge base, as the held fact `(not F)` that points at F, which is
  not held. A refuted commitment is in no knowledge base: its fork is dropped, and the search's memory and the
  node's record keep the death. A candidate is in none either, a tuple in a list beside root, until a commitment
  holds it ([`overview.md` § 2](overview.md#2-where-things-are-held-today), on runs).
- **Did we**: for the representation and for a rule's premise, yes
  ([Q-M1f.394](../../../docs/decisions/language/Q-M1f.394-a-fact-is-in-the-global-context-in-a-facts.md)); the
  kernel still reads `not` by its name for six decisions, which is group G.
- **Hypotheses as ordinary facts**: it runs today at no change of answer in the reading *asked*, and cannot in the
  reading *still open*; put as [Q-M1f.549](../open_questions.md#q-m1f549--hypotheses-as-facts-held-in-which-reading-and-how-far-the-search-goes-into-rules),
  with the recommendation to establish that foundation here and to decline the two uses for now, each for a reason
  a run gives ([review item 34](q-m1f.549_hypotheses_as_facts/README.md)).

## The answer on the names

The user, 2026-11-19, in the bundle's answers' file, item 33, verbatim:

> no need to reserver `ein.` prefix; make (false) an stdlib convention; add rules detecting contradictions infereing
> (false) and then - we can make another key, not :assert, but special engine control key `:ein false` which
> argument is not a fact, it can remain even the same word `false` but see
> plans/m1g_logic/p1g.40_proof_theory/ideas2.md and maybe propose better name here;

It rules [Q-M1f.399](../open_questions.md#q-m1f399--the-names-of-what-controls-the-search-false-and-open), where the reading is filed:

- **No prefix**, and no fact that is the kernel's: `(ein.kill)`, the leaning of the second instruction, is replaced.
- **`(false)` stays**, a proposition and a convention of the standard library, inferred by rules, the
  contradiction's among them.
- **What ends a state is a rule's key**: `:ein` *word* where another rule states `:assert`, the word the engine's
  and no fact.
- **The word**, [Q-M1f.550](../open_questions.md#q-m1f550--the-word-a-rule-gives-the-engine-to-end-a-state-the-argument-of-ein): on the user's *"maybe propose better name"*, `inconsistent` was proposed, the
  property of a set of premises that derives $\bot$, which is what a state is; the page the answer names sorts
  `false` with a formula in a model and `inconsistent` with a set of premises, and asks that they not be mixed.
  **Ruled the same day**, verbatim: *"Yes, :ein   inconsistent"*, and with it *"unblock S1f.21.10"*.

In the plan: [S1f.21.10](s1f.21.10_the_kernels_prefix.md), planned as the prefix, is re-planned to build the key and
waits on no decision; S1f.21.11 keeps the rest of its group, the contradiction as rules and `relation`.

And on the module that holds the rule from `(false)` to the key, the same day, verbatim: *"std.core always loaded -
ok now, later will decide how it could be customized"* ([Q-M1f.551](../open_questions.md#q-m1f551--the-module-every-program-gets-stdcore-always-loaded)); and
on the order: *"start s1f.21.10 after s1f.21.6 and s1f.21.8 merged"*.

And, when the coordinator briefed S1f.21.10 that `std.core` must declare nothing, verbatim: *"there is no
requirement "std.core must declare nothing" - it may declare; The elimination rule cannot go into the always-loaded
module as it stands - what is it about, wrong std.core? examples?"*. The brief was corrected: `std.core` may
declare, and a claim that a declaration moves is shown and not re-banked
([V-14](findings.md#v-14--a-declaration-an-import-brings-is-not-inert)). The second is answered with a program run
four ways in [V-12](findings.md#v-12--the-elimination-cannot-be-given-to-every-program-as-it-stands): it is nothing
about `std.core`, it is what S1f.21.11 would add to it.

## The order: two parts

The user, 2026-11-19, verbatim:

> what else stages not related to `open` and hypotheses? I want to leave open facts and hypotheses to the second
> part of the phase and finishe everything else before

So the phase runs in two parts ([§ Stages](#stages)):

- **Part 1, the verdict *dead***: the surfaces (done), the rules the standard library states, the engine's key
  `:ein inconsistent`, `not` and `relation` as conventions with the contradiction as rules, and the pages of all
  that. What it still needs of the user is S1f.21.11's: the choice of how the contradiction's rules reach every
  relation and count, which has no default, and
  [Q-M1f.447](../open_questions.md#q-m1f447--the-loaders-argument-count-check-once-relation-is-a-convention), each
  shown on the corpus by that stage's examples before it is asked.
- **Part 2, `open`**: the debt's term, hypotheses as facts, the search and rules, and their pages.
  [Q-M1f.390](../open_questions.md#q-m1f390--what-an-obligation-emits-the-verdict-with-its-justification-or-as-a-fact)
  and [Q-M1f.549](../open_questions.md#q-m1f549--hypotheses-as-facts-held-in-which-reading-and-how-far-the-search-goes-into-rules)
  waited with it and are answered; what they left is review items 44, 45 and 46
  ([part 2 of the bundle](review/part-2/README.md)).

**One thing of the second part was built in the first**: the third module of
[S1f.21.8](s1f.21.8_rules_the_standard_library_states.md), the requirement, the payment and `discharged` as rules a
program imports (`std.debts`). It is additive, read by nothing unless a program imports and activates it, and it
decides neither question. Whether it is merged with the stage was put to the user; with no answer when the stage
reported, it was held: the stage's first commit merged, and `std.debts` on its branch. **Merged 2026-11-19**, the user's
answer being (b), *merged now*: `edbe6c3b`.

**The user on part 2, the same day**, verbatim: *"regarding part 2: all hypotheses generation engines should emit (open H) facts and add them into KB? I think yes. Now already possible to emit new hypotheses at L1+ (dependent hypotheses) of search making search space still been lattice but not hypercube anymore. But it is non-monotone program by definition, so not to be decided in this phase."* So Q-M1f.549's main call is answered, (b): every rung
emits its candidates as facts `(open H)` the knowledge base holds. A candidate proposed above root is parked, not
this phase's ([V-21](findings.md#v-21--dependent-hypotheses-a-candidate-proposed-above-root)).

The pages' stage is cut along the same line: [S1f.21.2](s1f.21.2_the_pages.md) closes part 1, and
[S1f.21.14](s1f.21.14_the_pages_of_open.md) part 2.

## The sixth instruction

The user, 2026-11-19, verbatim:

> Also, still in Part 1, analyze semantic of hrules and obligation rules, I think they are 100% equivalent, so we
> can remove hrules completely from kernel, syntax, docs, everything; And replace them with obligation rules. Can
> we? Then we still have 2 approaches to hypotheses generation - positive and negative, the second one is blind
> generator with filters. Let's leave only one valid way for each. Obligations is positive way, what would be the
> negative (filtering) one?

Analysed in [`one_way_each.md`](one_way_each.md), on runs, with three censuses of the corpus kept beside it:

- **Not equivalent, by one thing**: an obligation is a hypothesis rule plus a requirement, *at least one of these*.
  A hypothesis rule says *may*, an obligation *must*.
- **Can we**: not into obligations alone. *Must* is the obligation's; *may* is the hypothesis rule's, or the
  filtering way's, the blind enumerator with what is not meant taken out. 12 corpus programs need only the rule
  deleted, 66 say *may* with it. The user, on reading it: *"Yes, true, hrules and obligations are not the same, easy there could be the program that has obligations, restrictions, but is not solvable without hrules because nothing generate "MAY" hypotheses"*; `examples/lattice/13_witness_of_two.ein`
  is one, and the proposal is now that the hypothesis rule stays.
- **The filtering way's one mechanism**: the closure, the fact `(__closed__ R)`; the two lists of `(solve …)` are
  the same statement written twice more, and give the same answer as closures in 99 of 100 programs.

Put as [Q-M1f.561](../open_questions.md#q-m1f561--one-way-to-say-must-and-one-to-say-may-hypothesis-rules-removed)
and **ruled the same day: declined**, verbatim: *"Ok, just decline this proposal - we leave all existing options to control hypotheses as is."* Every existing way to control the
hypotheses stays, and nothing is planned or built on it.

## The ruling on *forbid*

The user, 2026-11-19, verbatim:

> I tell you what I expect - it loads, ein print huge WARNING, and then we guarantee nothing about it's output. NO!
> Q-M1e2.5 IS WRONG! What the hell this `forbid` is? Where did it came from?

Said of review item 37, where the coordinator had proposed that *forbid* refuse seven more programs once the
contradiction is rules. Filed as [Q-M1f.564](../open_questions.md#q-m1f564--a-non-monotone-program-loads-with-a-warning-forbids-refusal-goes):
**a non-monotone program loads, under a loud warning, and nothing is guaranteed of its output**; Q-M1e2.5's refusal
at load is reversed. The ledger's entry has where *forbid* came from: the direct shape and the derived case were
ruled by the user on 2026-11-02 and 2026-11-04, on a question put as *forbid or respect*; that it refuses and does
not warn was a stage's default; and it was widened once more by a stage as a fix.

And once that history was read back, verbatim: *"Ok, I cancel those my listed decisions regarding absent -> false.  We allow to load non-monotone programs, but we don't claim about solution. That means - no tests validate the result of the solution of non-monotone programs. Remove this *forbid* from the code and docs completely."*
The two rulings behind Q-M1e2.5 are cancelled by the user who gave them; no test validates a non-monotone program's
result; *forbid* goes from the code and the pages. [S1f.21.15](s1f.21.15_forbid_becomes_a_warning.md) builds it,
**started the same day**.

## The ruling on `:hrules`

The user, 2026-11-19, verbatim: *":hrules argument looks bad. How do we activate rules normally? (rule R (?args) ....)  then somewhere (R args) right? So why :hrules look different? it has to be (solve :hrules ((guess chosen Drink) (guess served Drink)))"*

Filed as [Q-M1f.565](../open_questions.md#q-m1f565--the-hrules-clause-lists-activators-as-a-rules-are-written): the clause is a list of
activators, each a form headed by its hrule. The grouped form is the first spelling, of 2026-05-22, and nothing on
record gives a reason for it. S1f.21.16 was planned to build it and is dropped: the user's ruling of
Q-M1f.567, the next day, removes the clause.

## The rulings of the answers' file

The user's answers to part 1's six items, 2026-11-19 (*"read and process …/review/part-1/ANSWERS.md"*), each
verbatim, each filed under its question in [the ledger](../open_questions.md).

| item | question | the user, verbatim | what it rules |
|---|---|---|---|
| 35 | [Q-M1f.557](../open_questions.md#q-m1f557--how-the-contradictions-rules-reach-every-relation-and-count) | *"(d) if any program writes a negation and wants KB with P and not P dead - then it should also import std.contradiction. Otherwise in this program's theory there is no contradictions. Or it/they are defined different. For example: Kleene K3, Priest's LP, Belnap–Dunn FDE, Relevant logic, Paraconsistent logic."* | **(d)**: the contradiction is a theory a program imports, `std.contradiction`; a program that does not import it has none, or its own. The kernel's check goes (S1f.21.11) |
| 36 | [Q-M1f.447](../open_questions.md#q-m1f447--the-loaders-argument-count-check-once-relation-is-a-convention) | *"(c) and I accept program may be not well-formed intensionally or occasionally and be silent while not binding relations with different args count. More than that - let's consider this as relations overloading but in only meaning possible in ein - in structural meaning, that means different number of args, because there is no types in ein and will not ever be."* | **(c)**: the argument-count check is dropped, in silence; a relation at several counts is overloading by count, the one kind Ein has (S1f.21.11) |
| 38 | [Q-M1f.559](../open_questions.md#q-m1f559--the-elimination-and-the-module-every-program-gets) | *"forbid becomes a warning, fix :hrulse activators syntax"* | the `:hrules` clause lists activators whole (Q-M1f.565, S1f.21.16); and, **as read**, the refusal at load for what a rule could derive becomes a warning (S1f.21.17) |
| 40 | Q-M1f.552 to .556 | *"as built"* | the five readings of the finished stages stand |
| 41 | [Q-M1f.563](../open_questions.md#q-m1f563--what-a-module-every-program-gets-brings) | *"import std.core works like duplicated import; $EIN_STDLIB allows to replace std.core is in $EIN_STDLIB there is no std.core - print error about std.core not found in $EIN_STDLIB; There is still only a single user of ein - me, so I don't care about backward compatibility. Forget about it until I explicitely ask. Containers not load - delete and create new."* | `(import std.core)` is an import like any other; `$EIN_STDLIB` may replace `std.core`, and a library there without one is an error that says so (S1f.21.17); no backward compatibility |
| 42 | the findings | *"OK"* | each disposition of [`findings.md`](findings.md) as proposed |

**Read off the answers by the coordinator, said back to the user, and built only after:**

1. item 38's *"forbid becomes a warning"* is about that item's refusal, the loader's of a program in which a rule
   could derive an activator binding a relation to a fact, and reverses Q-M1f.1 (b) (S1f.21.17's task 1);
2. under item 35, the import is the switch and the fact `(clash-of-declared)` goes, though item 40's *"as built"*
   keeps it; the library's modules that conclude a negation import the module; a corpus program that relies on the
   death gains the import (S1f.21.11's task 1.1);
3. under item 36, a relation may be declared once a count (S1f.21.11's task 1.4).

**Confirmed by the user, 2026-11-20**, verbatim: *"3 readings: a - yes, b - yes, c - yes, it may because relation declaration is ordinary fact, and it may appear multiple times, rules may define what it means and how those multiple declarations used. Start stages, if anything can be done in parallel - do it."* The third is wider than it was put: a declaration is an
ordinary fact, so a relation may be declared more than once, at one count or at several, and rules say what the
several mean. On the same word the stages are started, those that can run side by side together: S1f.21.17,
S1f.21.11 and the examples that open S1f.21.12.

## The answers to part 2's items

The user, 2026-11-19 (*"process …/review/part-2/ANSWERS.md"*), each verbatim, each under its question in
[the ledger](../open_questions.md). The second part is not started by them: they are answers to its questions.

**Item 32**, [Q-M1f.390](../open_questions.md#q-m1f390--what-an-obligation-emits-the-verdict-with-its-justification-or-as-a-fact):

> obligation emits (open H) without justification, engine see where it was emitted anyway, so can record
> anythin it wants. But inside ein we just have (open H). btw, absent see it the same way as other facts. We
> have to analyze and strictly define interaction between absent, open and not. not is just an ordinary
> relation, absent is kernel primitive, open is both. draw a table 3x3 of now they behave together and where
> it is defined (not (not P)) is simple, (absent (absent P)) probably compilation error, (open (absent P))
> also compilation error. While (absent (open P)) possible. Maybe kernel primitives can't be nested because
> it's argument always a fact?

**Item 34**, [Q-M1f.549](../open_questions.md#q-m1f549--hypotheses-as-facts-held-in-which-reading-and-how-far-the-search-goes-into-rules):

> 1st (open ...) is a context for hypotheses. Not only obligations. Obligations MUST, hypotheses MAY be. As I
> understand now our obligation rules emit facts that are hypotheses and obligation options at the same time.
> How we can count the exact obligations and how we can define and emit them? Maybe in form of questions? How
> to define them? Maybe something like (owe (?R ?a ?_x)) where ?R and ?a binded and ?_x not - so the emitted
> thing is actually a query. Also a question about emitted open structure: the implied connector between KB
> facts is `and` - all facts are true so ein search is truth maintenance system. What is a connector between
> facts inside (open ...) ? Between new invented (owe ...) it is also `and` because we need to discharge all
> obligations. But what about (open ...) ? Can we say anything about them or not? Probably not - they are
> limited by restriction rules, not by obligation rules. If we will emit (open H) facts as pure hypotheses,
> and obligations would be emitted different way, can we then drop hrules as special form according to
> plans/m1f_hypothesis_and_documentation/p1f.21_verdicts_as_rules/one_way_each.md and use rules that emit open
> facts in :hrules?

**Item 39**, [Q-M1f.560](../open_questions.md#q-m1f560--stddebts-with-its-stage-or-with-the-second-part): *"(b)"*

| what | ruled, or done |
|---|---|
| what an obligation emits | **ruled**: `(open H)`, no justification in it; the engine's record of where it was emitted is its own; `absent` reads it as any fact. S1f.21.7 is not run |
| the table of `absent`, `open` and `not` | **drawn**, on runs: [`absent_open_not.md`](absent_open_not.md). Its open cells are Q-M1f.568 |
| `open` is *may*, an obligation is *must* | **ruled**; the four questions asked with it are answered on runs in [`open_and_owe.md`](open_and_owe.md), and leave Q-M1f.566 (how the *must* is emitted) and Q-M1f.567 (the `hrule` form dropped), each with a default |
| `std.debts` | **ruled (b)**, merged now, and **merged**: `edbe6c3b`, on S1f.21.15's close; no program it adds is flagged, and no other program's answer moves |

## The answers to items 44, 45 and 46

The user, 2026-11-19, in the answers' file of part 2, each verbatim, each under its question in
[the ledger](../open_questions.md). Processed on the user's word of what processing is: *"when I ask process
answers that means you consume them and leave only not answered and maybe add new - all with corresponding
sections in ANSWERS.md"*. The second part is not started by them.

**Item 44**, [Q-M1f.566](../open_questions.md#q-m1f566--how-a-requirement-is-emitted-the-must-beside-open), how the *must* is emitted:

> Obligations by (open R) and absent facts is already good. Let's leave this machinery. (open R) is obligation and
> (open (fact)) is an explicit hypothesis. Also leaving hypotheses auto-generation from absent facts in match that
> are built on relation R from (open R). All hypotheses generated by obligation rules also go into (open ...)
> context. Here for obligations everythin remain as is. We only add possibility to emit hypotheses explicitely.
> (open R) also saved as fact. Not duplicate (open R) because they are ordinary facts, but kernel still counts
> number of (open R) assertions as before. (open) is refused at load. Now let's decide what to do with open
> context when fork. One option is to erase it and re-generate after fixed point. Another options is that kernel
> will delete facts from it when they proved or refuted, but refutation now is already an stdlib convention - (not
> ...) context, so it has to be done by rules, so it will be rules that delete facts, but I'm not ready to refuse
> the only stable thing in the whole project - that we don't have rules that delete facts. So lit's pick option
> when fork removes all open facts from KB. Under contexts, like (think Ann (open (= 5 (+ 2 2)))) it is a simple
> fact, special form works only in global context. The same with `absent` - it is allowed but doesn't work in
> context it became simple fact.

**Item 45**, [Q-M1f.567](../open_questions.md#q-m1f567--the-hrule-form-dropped-for-a-rule-that-concludes-open-f), the `hrule` form dropped:

> I already decided that all mechanisms that work with hypotheses - work with facts under (open ...) constext. The
> hrule form initially was producing hypotheses - now it can do any rule that assert open facts. So there is no
> more need for :hrules special place - we just activate hypotheses generating rules anywhere. The special form
> (hrule ...) is not required anymore so we remove it and keyword :hrules.

And of *"anywhere"*, the same day: *"by "Activate anywhere" I meant in program, not necessary in :hrules list"*.

**Item 46**, [Q-M1f.568](../open_questions.md#q-m1f568--absent-open-and-not-nested-the-cells-that-are-the-users), four cells of the table:

> 4 - yes, 3 - yes, 2,1 and (open (absent P)) - refused at load because both `absent` and `open` are special
> kernel primitives (what are they now btw?) and their argument must be a fact, (absent P) is not a fact, while
> (open P) is, because `open` is a special kernel promitive AND a fact.

| what | ruled |
|---|---|
| the obligation | stays as it is: a rule with its trigger and its `absent`, concluding `(open R)`; the candidates are read off its `absent` as today |
| `(open R)` | held as a fact, once; the kernel counts the rule's instances as before |
| `(open F)`, F a fact | a hypothesis. Any rule may conclude one; the obligations' candidates are written as such |
| the bare `(open)` | refused at load |
| a fork | drops every `open` fact; they are made again at the fixpoint. No rule deletes a fact |
| the `hrule` form, the keyword `:hrules` | both go. A rule that concludes `(open F)` is activated by a fact the program writes, anywhere in it |
| nesting | a written `(open P)`, `(not (open P))` and `(open (not P))` legal; `(absent (absent P))`, `(open (open P))` and `(open (absent P))` refused at load; under another relation `open` and `absent` are plain facts |

**What follows for the stages.** S1f.21.16 is not started: the user, *"not start S1f.21.16, will drop it after
:hrules removed"*. S1f.21.9 is not run: today's count is kept. S1f.21.12 is planned again once the five questions
below are answered.

**What they leave**, each a review item with a default ([part 2 of the bundle](review/part-2/README.md)):

| item | question | what it asks |
|---|---|---|
| 47 | [Q-M1f.569](../open_questions.md#q-m1f569--what-may-read-an-open-fact-and-when) | what may read an `open` fact, and when: a fork drops the `open` facts and keeps what a rule concluded from them |
| 48 | [Q-M1f.570](../open_questions.md#q-m1f570--a-rule-that-concludes-open-f-beside-an-obligation-override-or-union) | a rule that concludes `(open F)` beside an obligation: override, as today, or union |
| 49 | [Q-M1f.571](../open_questions.md#q-m1f571--what-absent-and-open-take-and-what-each-is-under-an-ordinary-relation) | what `absent` and `open` take, and what each is under an ordinary relation |
| 50 | [Q-M1f.572](../open_questions.md#q-m1f572--whether-a-states-open-facts-are-shown-and-counted-with-its-facts) | whether a state's `open` facts are shown and counted with its facts |
| 51 | [Q-M1f.573](../open_questions.md#q-m1f573--open-as-an-obligation-or-as-a-hypothesis-and-which-hypotheses-are-candidates) | a variable under `open`; a hypothesis root has decided; $H$ as root's |

## The answers to items 47 to 51

The user, 2026-11-20 (*"process …/review/part-2/ANSWERS.md"*), each verbatim, each under its question in
[the ledger](../open_questions.md). The second part is not started by them.

**Item 47**, [Q-M1f.569](../open_questions.md#q-m1f569--what-may-read-an-open-fact-and-when), what may read an `open` fact:

> 1. Let's allow rules reading open facts. But it's automatically non-monotone program - with warning, no claims,
> no tests. That allows experimenting. (b) 2. (b) dropped with the rest, hand-written open fact present only in
> root. 3. Yes, monotone program step by step proves or refutes hypotheses, not adding new, right? If it does -
> then R is not upward-closed, search lattice is not hypercube, program is not monotone, and maybe even lattice is
> not finite (not sure about that). Maybe it's good point for non-monotonicity detection during search, in
> addition to static rules analysis that is performed before search starts, we could detect non-monotonicity
> in-flight and WARN user (if it was not done yet).

**Item 48**, [Q-M1f.570](../open_questions.md#q-m1f570--a-rule-that-concludes-open-f-beside-an-obligation-override-or-union), override or union:

> Rule can assert obligations (which imply hypotheses) and pure hypotheses independently. It can assert (open R) -
> and then it works as before, plus it cant assert (open (fact)) with whatever other fact, in the same assert.
> Identical open facts deduplicate naturally. Exactly (b).

**Item 49**, [Q-M1f.571](../open_questions.md#q-m1f571--what-absent-and-open-take-and-what-each-is-under-an-ordinary-relation), what `absent` and `open` take: *"All defautls."*

**Item 50**, [Q-M1f.572](../open_questions.md#q-m1f572--whether-a-states-open-facts-are-shown-and-counted-with-its-facts), shown with a state's facts:

> (a) hypotheses are valid part of the state; but now maybe we need separate flags/commands to show only positive
> facts of KB when we want to see only them.

**Item 51**, [Q-M1f.573](../open_questions.md#q-m1f573--open-as-an-obligation-or-as-a-hypothesis-and-which-hypotheses-are-candidates), obligation or hypothesis: *"1 - (a); 2 - (a); 3 - yes"*

| what | ruled |
|---|---|
| a rule that reads an `open` fact | allowed; its program is not monotone automatically: warned, nothing claimed, no test |
| a written `(open P)` | dropped at a fork with the rest: at root alone |
| the rules that conclude `(open …)` | every one read at every state; $H$ is root's |
| the writers of `(open F)` | the union: a rule's, an obligation's, identical ones one fact |
| `absent`, `open`, nesting | `absent` takes a premise and the universal stays; `(absent (absent P))`, `(open (open P))`, `(open (absent P))` refused; under an ordinary relation both are ordinary names |
| a state | its `open` facts are part of it: listed, drawn, counted |
| under `open` | a relation's name or parameter is the obligation; a fact, or a variable the `:match` binds, a hypothesis; one root has decided is no candidate |

**What follows for the stages.** [S1f.21.12](s1f.21.12_hypotheses_as_facts.md) is planned again on the eight
rulings, Q-M1f.566 to Q-M1f.573, and [S1f.21.18](s1f.21.18_the_hrule_form_removed.md) is planned for the sweep that
removes the `hrule` form and the keyword `:hrules`. Neither is started.

**What they leave**, each a review item with a default ([part 2 of the bundle](review/part-2/README.md)):

| item | question | what it asks |
|---|---|---|
| 52 | [Q-M1f.574](../open_questions.md#q-m1f574--a-hypothesis-first-held-above-root-how-the-program-is-found-not-monotone) | a hypothesis first held above root: detected statically, in flight, or both. Today's static check calls such a program monotone |
| 53 | [Q-M1f.575](../open_questions.md#q-m1f575--what-one-assert-holds-beside-open-r-and-beside-open-f) | what one `:assert` holds beside `(open R)` and beside `(open F)`: *"cant"* reads two ways |
| 54 | [Q-M1f.576](../open_questions.md#q-m1f576--the-unions-edges-an-obligation-whose-candidates-cannot-be-read-and-the-blind-enumerator) | what an obligation that declines and the blind enumerator do beside a rule |
| 55 | [Q-M1f.577](../open_questions.md#q-m1f577--a-view-of-a-state-without-its-open-facts) | a view of a state without its `open` facts |

## The answers to items 52 to 55

The user, 2026-11-20 (*"process …/review/part-2/ANSWERS.md"*), each verbatim, each under its question in
[the ledger](../open_questions.md). The second part is not started by them.

**Item 52**, [Q-M1f.574](../open_questions.md#q-m1f574--a-hypothesis-first-held-above-root-how-the-program-is-found-not-monotone), a hypothesis first held above root: *"1 - (c); 2 - (a) Warning shown immediately when detected, once;"*

**Item 53**, [Q-M1f.575](../open_questions.md#q-m1f575--what-one-assert-holds-beside-open-r-and-beside-open-f), what one `:assert` holds: *"(a)"*

**Item 54**, [Q-M1f.576](../open_questions.md#q-m1f576--the-unions-edges-an-obligation-whose-candidates-cannot-be-read-and-the-blind-enumerator), the union's edges:

> 1 - not exactly, "guard or trigger reads what a commitment can change" is not declined anymore, it is allowed,
> detected as non-monotone programs, but allowed. 2 - (a); 3 - What? Explain in more details

**Item 55**, [Q-M1f.577](../open_questions.md#q-m1f577--a-view-of-a-state-without-its-open-facts), a view without the `open` facts: *"(a)"*

| what | ruled |
|---|---|
| a hypothesis first held above root | found two ways: a site of the static check, for a rule that concludes `(open F)` and reads what a commitment can change; and in flight. Found in flight, the answer stands, nothing is claimed, and the warning is shown at once, once |
| one `:assert` | `(open R)` is a rule's whole `:assert`, as today; `(open F)` stands in a conjunction like any fact |
| the blind enumerator | writes only where no rule concludes `(open F)` at root and no obligation is activated |
| a view without the `open` facts | `-P` leaves them out as it leaves out the `(not …)` |

**What they leave**, each a review item with a default ([part 2 of the bundle](review/part-2/README.md)):

| item | question | what it asks |
|---|---|---|
| 56 | [Q-M1f.578](../open_questions.md#q-m1f578--the-two-lists-of-solve-and-a-closure-once-the-hypotheses-are-open-facts) | the two lists of `(solve …)` and a closure, once the hypotheses are `open` facts: the explanation the user asked for, and the question again |
| 57 | [Q-M1f.579](../open_questions.md#q-m1f579--an-obligation-whose-guard-or-trigger-reads-what-a-commitment-can-change) | an obligation whose guard or trigger reads what a commitment can change: the user's answer, put back, since *declined* is no refusal today and the answer to the letter takes a claim that holds from six corpus programs |

## The answers to items 56 and 57

The user, 2026-11-20 (*"process …/review/part-2/ANSWERS.md"*), each verbatim, each under its question in
[the ledger](../open_questions.md). **With them no question of the second part is open.** The part is not started
by them.

**Item 56**, [Q-M1f.578](../open_questions.md#q-m1f578--the-two-lists-of-solve-and-a-closure-once-the-hypotheses-are-open-facts), the two lists and a closure:

> all 3 (:hypothesis-relations, :no-hypothesis, __closed__) are configuration for blind enumerator, white list and
> black list; blind enumerator runs ONLY if at the root fixpoint (open ...) context is empty - no facts. If any of
> them present together with emitted open facts, but disagree - it's an error, invalid program. e.g. rule emits
> closed relation hypotheses - it is an error. :no-hypothesis forbids existing open facts - error,
> :hypothesis-relations lists relations that are not in open facts - also error. Do not ever run blind enumerator
> together with hypotheses emitted by rules. (a)

**Item 57**, [Q-M1f.579](../open_questions.md#q-m1f579--an-obligation-whose-guard-or-trigger-reads-what-a-commitment-can-change), an obligation whose guard or trigger reads what a commitment can change:

> (b) implement it and double check examples/ein-bugs/wrong-answers/oblig-derived-domain.ein, I expect it will
> still be solvable, despite non-monotonicity. This is my expectation, not requirement. Let's review it after
> implementation.

| what | ruled |
|---|---|
| `:hypothesis-relations`, `:no-hypothesis`, `(__closed__ R)` | the blind enumerator's configuration, a white list and a black list |
| the blind enumerator | runs only where root's fixpoint holds no `open` fact; never beside hypotheses a rule emits |
| one of the three beside `open` facts it disagrees with | an error, the program invalid: a hypothesis of a closed relation; a `:no-hypothesis` that names an `open` fact's relation; a `:hypothesis-relations` that lists a relation no `open` fact is of |
| an obligation whose guard or trigger reads what a commitment can change | never steps aside: it writes its guard's candidates at root, and the program is flagged |
| `oblig-derived-domain.ein` | run again once that is built, and shown: the user expects it still solvable, *"my expectation, not requirement. Let's review it after implementation."* |

## The word to run to the end

The user, 2026-11-20, of the examples of S1f.21.12 (review items 58 to 65), verbatim:

> plans/m1f_hypothesis_and_documentation/p1f.21_verdicts_as_rules/review/part-2 - pick defaults, implement the phase to the end but not close; after implementation complete add renders with the final ein version. I will review questions after.

So:

- **Each of the eight items takes its default**, and the six questions among them, Q-M1f.595 to Q-M1f.600, theirs.
  A default taken so is no ruling: the items stay in the bundle, with their sections of the answers' file blank,
  for the user to review after the build.
- **Every stage left is run, to the phase's end, with no further word**: S1f.21.17 and S1f.21.11, running;
  then S1f.21.2, S1f.21.12's build, S1f.21.18, S1f.21.13 and S1f.21.14; with their gates.
- **The phase is not closed**: its findings are not handed on, its folder is not deleted, and nothing of the
  milestone's close is resumed.
- **Once everything is built, the review items gain their renders on the final engine**
  ([S1f.21.19](s1f.21.19_the_review_items_on_the_built_engine.md)): each program in the spelling ruled, run on the
  built binary, with its drawings, beside what the stand-in showed.

## What it is

Three things the kernel decides by itself, each a verdict about a state, none written anywhere as a rule:

| verdict | what the kernel does today | as a rule |
|---|---|---|
| **contradiction** | a state that holds a fact and its stored negation is dead, as one that holds `(false)` is | from a fact and its negation, `(false)` |
| **alive / dead** | a state is dead where it holds `(false)` or a fact beside its negation, alive otherwise | *dead* from `(false)`; *alive* from its absence |
| **owed / discharged** | an obligation rule concludes `(open R)`; the kernel reads every such rule once on a quiescent state and counts the instances it matches; a state that owes none is discharged | *owed* where a trigger holds and no witness does; *discharged* from the absence of anything owed |

The reasoning pages state each as a formula. The instruction is to state each **as a rule**, in the rule form
[`01-static-ir/02-rewriting-rules.md`](../../../docs/ein/reasoning/01-static-ir/02-rewriting-rules.md) defines,
so that what the kernel does reads as rewriting like everything else, and what is special about it is said and
not hidden.

**And `(open R)`.** An obligation's conclusion is no fact of the knowledge base today: it is routed out of the
agenda and counted. The instruction's remark is what makes that so: read as facts, the `(open R)`s would be the
only facts that **(1) duplicate**, since two instances an obligation owes over one relation are the same tuple,
and one node, while the count needs two; and **(2) can be removed**, since a state above pays what a state below
owes, while every other fact a state holds, every larger state holds. The instruction's question is whether to
emit them as `(open R <justification>)`: with what is owed in the tuple, two instances are two facts, and (1)
goes; what then becomes of (2) is the design.

## Stages

Planned 2026-11-19 on the fifth instruction: each group of [`modifications.md`](modifications.md) is a stage.
**In two parts since the same day**, on the user's word ([§ The order: two parts](#the-order-two-parts)): the
stages that do not touch `open` or the hypotheses first, and those that do after. Within a part the rows are in
the order the stages run. [`overview.md` § 5](overview.md#5-the-stages) has the same tables with the items.

| stage | group | what | state |
|---|---|---|---|
| [S1f.21.1](s1f.21.1_the_survey_and_the_design.md) | | **The survey and the design.** What the kernel does for each verdict; each verdict as a rule; `(open R)` as a fact; `(open R <justification>)` designed; the question | **done**: [`design.md`](design.md), [Q-M1f.390](../open_questions.md#q-m1f390--what-an-obligation-emits-the-verdict-with-its-justification-or-as-a-fact), [`findings.md`](findings.md) |

**Part 1 — the verdict *dead*: `false`, `not`, `relation`, the key.**

| stage | group | what | state |
|---|---|---|---|
| [S1f.21.6](s1f.21.6_surfaces.md) | A | **The surfaces.** `-p`'s printer, the dump's rule sections, *derives (false)*, the plan's own numbers | **done 2026-11-19**: `d35408cb`, `8d8327ce`, `c9ea9597`, `d1c9d4b3`, `984137bf`; V-2, V-3 and V-4 fixed, V-9 entered; its three readings took their defaults (Q-M1f.552 to .554); no answer moved. The batch's container gate is the coordinator's, with S1f.21.8's merge |
| [S1f.21.8](s1f.21.8_rules_the_standard_library_states.md) | D | **Rules the standard library states.** The elimination of a double negation and the contradiction, each a module a program imports; and the requirement, the payment and `discharged` | **done 2026-11-19, and merged whole**: `std.negation` and `std.contradiction` (`b0ee99f4`) by the merge `ca98dec2`; `std.debts` (`6b76c164`) as `edbe6c3b`, on the user's ruling Q-M1f.560 (b). Its readings: Q-M1f.555, Q-M1f.556, each ruled *as built* |
| [S1f.21.10](s1f.21.10_the_kernels_prefix.md) | F | **The engine's key, `:ein inconsistent`.** A rule states `:assert` or `:ein`; a rule with the key that fires ends the state; `(false)` a proposition, and one rule of a module every program gets from it to the key; no prefix, no word renamed | **done 2026-11-19**: `68c3d5d0`, `946ee788`; no answer moved over the 913 corpus files; its readings and the four consequences of an always-loaded module are Q-M1f.562 and Q-M1f.563, review item 41. **The gate in the runner's image is green on `946ee788`** (165 test binaries) **and the nightly there on `901f9fa3`**, the same code under plan commits (82 steps, exit 0) |
| [S1f.21.15](s1f.21.15_forbid_becomes_a_warning.md) | | ***Forbid* is removed, and nothing validates a non-monotone program's answer.** The check and its words go from the code and the pages; a non-monotone program loads under a loud warning; no `:expect`, regression row, golden line of an answer or assertion is left on a flagged program | **done 2026-11-19**: `1f063ed9` (*forbid* gone), `15689488` (nothing validates a flagged program's answer), `1b9f87f0` (the warning), record `e2c95caf`. Six findings, V-22 to V-27, two of them the user's calls. The gate and the nightly in the runner's image are run on the batch's last commit |
| S1f.21.16 | | **The `:hrules` clause lists activators as a rule's are written.** Planned on the user's ruling of Q-M1f.565 | **dropped**: the user, *"not start S1f.21.16, will drop it after :hrules removed"*; the keyword is removed (S1f.21.18) and the stage's file with it, in `git log` |
| [S1f.21.17](s1f.21.17_the_loader_warns_and_std_core_by_import.md) | | **A warning where the loader refused for what a rule could derive; `std.core` by import and by `$EIN_STDLIB`.** The first as review item 38's answer is read; the other two the user's answer to item 41 | **done 2026-11-20**: `24491e80` (the build), `6d5204be` (its record); the gate green on its branch. The five programs load, four of them flagged; five defaults (Q-M1f.580 to .584, review item 66 of part 1); V-32 to V-34 |
| [S1f.21.11](s1f.21.11_kill_and_three_conventions.md) | G | **`not` and `relation` as conventions: the contradiction as rules.** Its examples, then the build in four commits | **done 2026-11-20**: `0345e799`, `d6773d76`, `663ca7d1`, `3617f009`, `7f69b9ac`, `04665c38`, record `ddf09492`; rebased onto S1f.21.17; the gate green on `04665c38`. No corpus answer moved; 36 programs and three library modules gained the import. Ten defaults (Q-M1f.585 to .594, review item 67 of part 1); V-35 to V-39; the cost measurement owed |
| [S1f.21.2](s1f.21.2_the_pages.md) | B | **The pages of the first part.** *Dead*, the contradiction and the key on the reasoning pages and in Ein on the user's; `(false)` as a proposition; the part's rulings as decisions | **done 2026-11-20**: `558bf086`, `7b375380`, `a5e4bc26`, `28aa1ce8`, record `ab184ba2`, `459b37a2`; the gate green on its branch. Nine decisions written and three rewritten; V-16, V-20 and V-31 fixed; five defaults (Q-M1f.601 to .605, review item 68 of part 1); V-43 to V-45 |

**Part 2 — `open`: what a state owes, and the hypotheses.** Not started before part 1 is done; its two questions,
Q-M1f.390 and Q-M1f.549, wait with it.

| stage | group | what | state |
|---|---|---|---|
| [S1f.21.7](s1f.21.7_the_debts_term.md) | C | **The debt's term.** An obligation's instance written as one fact, on the `owe` event and in the lattice dump | **not run**: set aside 2026-11-19 by the user's answer to Q-M1f.390, no justification is written into Ein |
| [S1f.21.12](s1f.21.12_hypotheses_as_facts.md) | H | **`open` facts: the hypotheses held, and the obligation's verdict a fact.** `(open R)` held where a state owes, `(open F)` a hypothesis any rule may conclude, written by a pass at each state's fixpoint and dropped at a fork; $H$ root's `open` facts, the union of the writers; a rule that reads one makes its program not monotone; the nesting rule; the `open` facts part of a state. Its examples first | **done 2026-11-20**: its examples (`f529cde2`, `bfeedc50`, `ace51632`) and its build, `e1f37012`, `e46c4c89`, `21865cbc`, `f0bbd066`, `04d780fb`, `375e3d4f`, `2788c84d`, `247dd3c3`, record `c59fdfde`, `08ed0ce7`; rebased onto the first part's pages; the gate green on `247dd3c3`. The corpus moved as the examples' item 58 said. Fifteen defaults (Q-M1f.606 to .620, review item 69 of part 2); V-46 to V-55. The container gate and the nightly are the coordinator's, at the end |
| [S1f.21.18](s1f.21.18_the_hrule_form_removed.md) | H | **The `hrule` form and the keyword `:hrules` removed.** Every program, page and test written with a rule that concludes `(open F)` and its activators as facts; the form and the keyword deleted | **done 2026-11-20**: `766514f6`, `5f04ade4`, `5cba9872`, `86f5f8d7`, `e44e1402`, `6a14f656`, `c306224b`, records `b766115a`, `a8551a18`; rebased onto S1f.21.13; the gate green on `c306224b` with four test workers. 120 programs written again, each answering as before; the form and the keyword deleted. The sweep found three faults of S1f.21.12's build, one a wrong answer, and fixed them. Ten defaults (Q-M1f.621 to .630, review item 71 of part 2); V-56 to V-60 |
| [S1f.21.13](s1f.21.13_the_search_and_rules.md) | I | **The search and rules.** Planned under the recommendation: the lattice as rules kept as an example, the engine's search not changed, what is declined and why | **done 2026-11-20**: `2b63821b`, `54cc0002`, `4b2a3026`, records `54adb21a`, `162e99e9`, `59986c93`, `1e60859d`; the gate green on `59986c93` with four test workers. The example is a pair, `examples/lattice/18_two_candidates.ein` and `18_two_candidates_as_rules.ein`, monotone and held by `ein test`; what is declined stays declined, each reason checked again. Three defaults (Q-M1f.631 to .633, review item 70 of part 2); V-61 to V-63, which S1f.21.20 fixes; V-8 still holds |
| [S1f.21.20](s1f.21.20_what_the_first_uses_of_open_found.md) | H | **What the first uses of `open` found.** Three defects of S1f.21.12's build, none moving a corpus answer: a written `(open F)` and the saved ∅ (V-61), a note before the `WARNING` (V-62), three reports that do not see the `open` writers (V-63). And from [the cost measurement](costs/README.md): **the pass runs at root and, above it, only at a state that is printed, saved or used**, the user's ruling of Q-M1f.655 (V-64); and, bounded, the gate's build of one program (V-65) | **done 2026-11-20**: `2dff3ce7`, `70a3572d`, `cff1522b`, `91453f8d`, `ebe3797f`, `a7b7105c`, `8efef891`, `48405ab5`, `fc297858`, records to `0a6e6e3b`; the gate green on `96d919fe` at sixteen workers, the two slow render sweeps green. All seven tasks: the three defects fixed, with `examples/features/24_written_hypotheses.ein` in the corpus; the pass as the user ruled, with the flag `--in-flight`; the agenda's tables kept; the gate's debug check found and bounded. Nine defaults (Q-M1f.636 to .644, review item 72 of part 2); V-66 to V-68 |
| [S1f.21.21](s1f.21.21_the_contradictions_cost.md) | H | **The contradiction's cost.** The user's ruling of Q-M1f.657, *everything recommended* by [the cost page](costs/README.md): a rule of the pair's shape matched by one probe for each new fact; the relation of a nested fact a key of the indexes; the instance of no argument made free. Then the cost measured again, a table of three columns | **done 2026-11-20**: `953372da`, `070d5088`, `0f382012`, records to `988d8805`; the gate green on `f484294e` at sixteen workers, the two slow render sweeps green. A pair matched by one probe; the nested relation a key of the participation index and of the saturator's index of plans; the set of negated facts gone (V-36); and, beyond the plan, a watermark and a shared plan list, which saved more than the two tasks. **No firing moved**: `solve -p` and the events' digest identical on all 942 corpus files. Six defaults (Q-M1f.658 to .663, review item 74 of part 2); V-78 to V-80 |
| [S1f.21.22](s1f.21.22_what_the_pages_found_in_the_build.md) | H | **What the pages found in the build.** The binary short of two rulings and two contracts, found by S1f.21.14: the kb and constraints renders draw no `open` fact of root (V-72); a written `(open …)` is not judged against the lists and a closure (V-73); a printed claim over `open` does not read back (V-74); the hypothesis found in flight and the count of workers (V-77); comments in retired words (V-75) | **done 2026-11-20**: eight commits and its records, cherry-picked onto master as `d1e17637` to `302fb1c5`; the gate green on its tip at sixteen workers, the two slow render sweeps green. All six: the kb and constraints renders draw root's `open` facts; a written `(open …)` against the lists and a closure is a load error; the claim row says where a claim is not printed; the finding in flight is one at any count of workers, a race shown by a probe and fixed; the `outside` row; the comments. Seven defaults (Q-M1f.666 to .672, review item 75 of part 2); V-84, V-85 |
| [S1f.21.23](s1f.21.23_the_delta_of_a_resumed_state.md) | H | **The delta of a resumed state, read off its own layers.** The user's ruling of Q-M1f.674, *"of course build"*: at every state the resume walks every fact the state holds to find the few the commitment wrote (V-78); read off the layers above root's, the delta costs its own size | **done 2026-11-20**: `e211bde3`, `30371a5e`, records to `6f709d66`; the gate green at sixteen workers, the two slow render sweeps green. A layer carries a stamp, a snapshot root's mark, and the delta is what the state holds above it. **No firing moved**: `solve -p` and the events' digest identical on all 942 corpus files, on the release build and on the gate's, where a check holds the new read to the walk at every state. Four defaults (Q-M1f.675 to .678, review item 76 of part 2); V-86 to V-88 |
| [S1f.21.9](s1f.21.9_the_requirement_and_the_payment_as_facts.md) | E | **The requirement and the payment as the kernel's facts.** | **not run**: the user's answer to Q-M1f.566 keeps the obligation and its count as they are |
| [S1f.21.14](s1f.21.14_the_pages_of_open.md) | B | **The pages of the second part.** *Owed* and *discharged* as rules, the remark on `(open R)`, $R$ with no search, what a hypothesis is; the part's rulings as decisions | **done 2026-11-20**: nine commits, cherry-picked onto master as `fc9173c6..903e1779`; the gate green on its tip with four test workers, after its rebase onto S1f.21.20. The four items on their pages; every page of `docs/ein/` and the READMEs audited against the binary; two decisions written, 27 rewritten, twelve files renamed (V-55); Q-M1f.655 and Q-M1f.656 cited and `--in-flight` named; V-76, V-67 and V-68 fixed. Ten defaults (Q-M1f.645 to .654, review item 73 of part 2); V-72 to V-77 |
| [S1f.21.19](s1f.21.19_the_review_items_on_the_built_engine.md) | | **The review items on the built engine.** Items 58 to 65 run again on the final binary, in the spelling ruled, with their renders, for the user's review of the questions | planned 2026-11-20 on the user's word (*"after implementation complete add renders with the final ein version"*); after every build of the phase |
| S1f.21.5 | | **The close.** | last |

Folded, their ids kept:

| stage | | what | state |
|---|---|---|---|
| S1f.21.3 | | *The engine*, as planned 2026-11-17 | **folded** 2026-11-19 into S1f.21.7 to S1f.21.13, a stage a group |
| S1f.21.4 | | *Open facts and `(ein.kill)`: the design on programs*, as planned 2026-11-18 | **folded** 2026-11-19: its seven questions ([`open_facts.md` § 9](open_facts.md#9-what-s1f214-answers)) are the examples tasks of the stages that build them: 1 and 6 S1f.21.11's, 2 to 4 S1f.21.12's, 5 S1f.21.13's, 7 the ledger's (Q-M1f.390, Q-M1f.399, with Q-M1f.549) |

## The question it puts

**Should an obligation emit `(open R <justification>)` as a fact of the knowledge base?** The user's *maybe*,
put as [Q-M1f.390](../open_questions.md#q-m1f390--what-an-obligation-emits-the-verdict-with-its-justification-or-as-a-fact)
with four options: (a) as today; (b) the verdict carries its justification, the instance, and stays a verdict,
which S1f.21.1 recommends; (c) the requirement and its payment are facts and *owed* is their difference; (d)
`(open R j)` is a fact of the state that owes it, in a part of the node no fork inherits.
[`design.md` § 5](design.md#5-the-question-and-a-recommendation) is the page to rule from. The ruling is the
user's, and the stages that build wait on it (S1f.21.7 and S1f.21.9). The user's second instruction stands beside
the four options: it is (d), with rules reading the open facts and the hypothesis in the fact; Q-M1f.549 puts it
again, in two readings, with Q-M1f.399, the name. S1f.21.2 waits for the rulings and the builds, since the pages
state the kernel as built; until the second instruction it did not wait, and would have stated today's kernel as
rules.

**And, since the fifth instruction: are the hypotheses facts?**
[Q-M1f.549](../open_questions.md#q-m1f549--hypotheses-as-facts-held-in-which-reading-and-how-far-the-search-goes-into-rules):
(a) no, as today; (b) held as *asked*, from root upward, nothing removed, which is recommended; (c) held as *still
open* and moved, the second instruction's form as [`open_facts.md` § 5](open_facts.md#5-the-move-open-held-not)
designed it; (d) the search itself as rules. It fits Q-M1f.390's default: the debt stays a verdict that is counted,
and the candidates that would pay it are held.

## Rules of the phase

The standing ones ([`AGENTS.md`](../../../AGENTS.md)): a stage per subagent; what a stage finds outside its scope
goes to a findings page of the phase's own; a claim about the engine is a run, not an argument; a new syntax is
shown on real corpus programs with their answers computed before it is built; no estimate is written.

**Every subagent of the phase runs on Opus**, from 2026-11-20 to the phase's end (the user: *"till the end of this
phase run subagents with Opus"*).

**No backward compatibility is kept** (the user, 2026-11-19: *"There is still only a single user of ein - me, so I
don't care about backward compatibility. Forget about it until I explicitely ask."*): a retired spelling gets no
line of its own, and a saved file or a container of an older format is deleted and made again.
