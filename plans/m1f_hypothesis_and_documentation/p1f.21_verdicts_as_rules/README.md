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

It rules § 8.2's two rows of [`open_facts.md`](open_facts.md#82-each-name-under-this-phases-direction) that the
third instruction left as a direction (*"relation - is not actually, it's just a convention already now"*, *"not -
will become a convention also with contradiction rule"*): **`not` and `relation` are the standard library's
conventions, not the kernel's.** What reads either today, and what each reader becomes, is S1f.21.4's
([§ 9](open_facts.md#9-what-s1f214-answers), item 6). The same day, on P1f.23's review, the user ruled that the
kernel reads no theory's name and dropped the built-in symmetry (`__symmetric__`); P1f.23's S1f.23.4 builds that.

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

| stage | what | state |
|---|---|---|
| [S1f.21.1](s1f.21.1_the_survey_and_the_design.md) | **The survey and the design.** What the kernel does for each verdict, where, and what shows it; each verdict written as a rule, and whether Ein can load and run that rule today; `(open R)` today, the two properties shown on programs; `(open R <justification>)` designed: what the justification is, what holds a discharged debt, what changes in the engine, the stdlib, hypothesis generation, the read-out and the tests, on real corpus programs with their answers computed. Ends with the question for the user | **done**: [`design.md`](design.md), [Q-M1f.390](../open_questions.md#q-m1f390--what-an-obligation-emits-the-verdict-with-its-justification-or-as-a-fact), [`findings.md`](findings.md) |
| S1f.21.4 | **Open facts and `(ein.kill)`: the design on programs.** The user's second and third instructions, from [`open_facts.md`](open_facts.md): the verdict's name and the module that concludes it, with the contradiction rules and V-1; the form of an open fact and its justification for each rung; the move and the stratum; the rules over open facts the standard library states and what the routes read of them; the search as rules; the reserved names that are left and what the kernel reads (§ 8). On real corpus programs with their answers computed, each put as a question with a default ([`open_facts.md` § 9](open_facts.md#9-what-s1f214-answers)) | **next**, planned 2026-11-18, **not started** |
| S1f.21.2 | **The reasoning pages.** In `docs/ein/reasoning/02-kb-evolution/`: the contradiction rule, alive and dead, owed and discharged, each as a rule with its graph and its formula, and the remark on `(open R)` as facts; the user's pages where the same is said in Ein, which S1f.21.1 finds for the contradiction rule of one relation or one count of arguments, for *dead*, and for the requirement, the payment and *discharged*, and not for the rule of every fact, for *alive* or for a stored debt ([`design.md` § What S1f.21.2 writes under each ruling](design.md#what-s1f212-writes-under-each-ruling)) | after S1f.21.4's rulings, and after S1f.21.3 where they change the kernel: the pages state the kernel as it is |
| S1f.21.3 | **The engine**, if the user rules for a change: planned from S1f.21.1's and S1f.21.4's designs when the rulings are given. Under (a) of Q-M1f.390, and under (b) with nothing printed, with no `(ein.kill)` and no open fact, there is nothing to build and the stage is not run | waits for the rulings |
| S1f.21.5 | **The close.** | last |

## The question it puts

**Should an obligation emit `(open R <justification>)` as a fact of the knowledge base?** The user's *maybe*,
put as [Q-M1f.390](../open_questions.md#q-m1f390--what-an-obligation-emits-the-verdict-with-its-justification-or-as-a-fact)
with four options: (a) as today; (b) the verdict carries its justification, the instance, and stays a verdict,
which S1f.21.1 recommends; (c) the requirement and its payment are facts and *owed* is their difference; (d)
`(open R j)` is a fact of the state that owes it, in a part of the node no fork inherits.
[`design.md` § 5](design.md#5-the-question-and-a-recommendation) is the page to rule from. The ruling is the
user's, and S1f.21.3 is planned only on it. The user's second instruction stands beside the four options: it is
(d), with rules reading the open facts and the hypothesis in the fact. S1f.21.4 puts it again, with Q-M1f.399,
the name. S1f.21.2 waits for both rulings, since the pages state the kernel as built; until that instruction it
did not wait, and would have stated today's kernel as rules.

## Rules of the phase

The standing ones ([`AGENTS.md`](../../../AGENTS.md)): a stage per subagent; what a stage finds outside its scope
goes to a findings page of the phase's own; a claim about the engine is a run, not an argument; a new syntax is
shown on real corpus programs with their answers computed before it is built; no estimate is written.
