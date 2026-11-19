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
with runs ([review item 34](review/part-2/34-Q-M1f.549/README.md)). The same day the user ruled Q-M1f.399
([below](#the-answer-on-the-names)): no prefix, `(false)` a convention, and what ends a state a rule's key `:ein`,
its word ruled `inconsistent` (Q-M1f.550), S1f.21.10 unblocked to build it; and **started S1f.21.6 and S1f.21.8**
(*"start s1f.21.6 and s1f.21.8"*). **S1f.21.6, S1f.21.8 and S1f.21.10 are done** (the same day), the second merged but
for `std.debts`, held for part 2. **S1f.21.11 is next, on the user's word.**
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
  a run gives ([review item 34](review/part-2/34-Q-M1f.549/README.md)).

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
  wait with it, and review items 32 and 34.

**One thing of the second part was built in the first**: the third module of
[S1f.21.8](s1f.21.8_rules_the_standard_library_states.md), the requirement, the payment and `discharged` as rules a
program imports (`std.debts`). It is additive, read by nothing unless a program imports and activates it, and it
decides neither question. Whether it is merged with the stage was put to the user; with no answer when the stage
reported, **it is held**: the stage's first commit is merged, and `std.debts` waits on its branch, built and tested,
for the user's word or for part 2.

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
record gives a reason for it. [S1f.21.16](s1f.21.16_hrules_lists_activators.md) builds it, planned and not started.

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
| [S1f.21.8](s1f.21.8_rules_the_standard_library_states.md) | D | **Rules the standard library states.** The elimination of a double negation and the contradiction, each a module a program imports; and the requirement, the payment and `discharged` | **done 2026-11-19; merged in part**: `std.negation` and `std.contradiction` (`b0ee99f4`) by the merge `ca98dec2`; `std.debts` (`6b76c164`) built and **held on the stage's branch for part 2**. Its readings: Q-M1f.555 (two modules for the negation, not the default), Q-M1f.556. The batch's container gate runs on the merge |
| [S1f.21.10](s1f.21.10_the_kernels_prefix.md) | F | **The engine's key, `:ein inconsistent`.** A rule states `:assert` or `:ein`; a rule with the key that fires ends the state; `(false)` a proposition, and one rule of a module every program gets from it to the key; no prefix, no word renamed | **done 2026-11-19**: `68c3d5d0`, `946ee788`; no answer moved over the 913 corpus files; its readings and the four consequences of an always-loaded module are Q-M1f.562 and Q-M1f.563, review item 41. **The gate in the runner's image is green on `946ee788`** (165 test binaries) **and the nightly there on `901f9fa3`**, the same code under plan commits (82 steps, exit 0) |
| [S1f.21.15](s1f.21.15_forbid_becomes_a_warning.md) | | ***Forbid* is removed, and nothing validates a non-monotone program's answer.** The check and its words go from the code and the pages; a non-monotone program loads under a loud warning; no `:expect`, regression row, golden line of an answer or assertion is left on a flagged program | **started 2026-11-19** on the user's word (*"Remove this *forbid* from the code and docs completely"*), Q-M1f.564; in the main tree; before S1f.21.11 |
| [S1f.21.16](s1f.21.16_hrules_lists_activators.md) | | **The `:hrules` clause lists activators as a rule's are written.** `(solve :hrules ((guess chosen Drink) (guess served Drink)))`; every program that writes the clause re-spelled, no answer moving | planned 2026-11-19 on the user's ruling (Q-M1f.565), **not started**; after S1f.21.15 |
| [S1f.21.11](s1f.21.11_kill_and_three_conventions.md) | G | **`not` and `relation` as conventions: the contradiction as rules.** Its examples, then the build in four commits | planned, **not started**; waits on S1f.21.15, Q-M1f.557 (how the rules reach every relation; proposed (d)) and Q-M1f.447 |
| [S1f.21.2](s1f.21.2_the_pages.md) | B | **The pages of the first part.** *Dead*, the contradiction and the key on the reasoning pages and in Ein on the user's; `(false)` as a proposition; the part's rulings as decisions | planned, **not started**; after S1f.21.8, S1f.21.10 and S1f.21.11 |

**Part 2 — `open`: what a state owes, and the hypotheses.** Not started before part 1 is done; its two questions,
Q-M1f.390 and Q-M1f.549, wait with it.

| stage | group | what | state |
|---|---|---|---|
| [S1f.21.7](s1f.21.7_the_debts_term.md) | C | **The debt's term.** An obligation's instance written as one fact, on the `owe` event and in the lattice dump | planned, **not started**; waits on Q-M1f.390 |
| [S1f.21.12](s1f.21.12_hypotheses_as_facts.md) | H | **Hypotheses as facts.** Planned under Q-M1f.549 (b): a candidate is a fact root holds, and $H$ that extent less what root decides | planned, **not started**; waits on Q-M1f.549, S1f.21.7 |
| [S1f.21.13](s1f.21.13_the_search_and_rules.md) | I | **The search and rules.** Planned under the recommendation: the lattice as rules kept as an example, the engine's search not changed, what is declined and why | planned, **not started**; waits on Q-M1f.549, S1f.21.12 |
| [S1f.21.9](s1f.21.9_the_requirement_and_the_payment_as_facts.md) | E | **The requirement and the payment as the kernel's facts.** | planned, **run only on a ruling for Q-M1f.390 (c)** |
| [S1f.21.14](s1f.21.14_the_pages_of_open.md) | B | **The pages of the second part.** *Owed* and *discharged* as rules, the remark on `(open R)`, $R$ with no search, what a hypothesis is; the part's rulings as decisions | planned, **not started**; after S1f.21.7, S1f.21.12 and S1f.21.13 |
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
