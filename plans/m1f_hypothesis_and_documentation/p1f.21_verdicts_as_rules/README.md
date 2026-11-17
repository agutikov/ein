# P1f.21 — the kernel's verdicts as rules

**Milestone:** [M1f](../README.md)
**Status:** created 2026-11-17 on the user's instruction (below). [S1f.21.1](s1f.21.1_the_survey_and_the_design.md)
is next; the phase stops after it for the user's ruling on its question.
**Depends on:** [S1f.20.10](../p1f.20_docs_refactor/s1f.20.10_the_review_of_the_tree.md) for its second stage,
which writes into pages of `docs/ein/reasoning/02-kb-evolution/` that S1f.20.10's remaining lanes still move
(the diagrams into files, the formulas onto several lines). Its first stage reads the engine and writes here.
**Blocks:** the close of M1f, which was started on the user's acceptance of the documentation tree and paused
on the user's word; it resumes when this phase is done.

---

## The instruction

The user, 2026-11-17, verbatim:

> in docs/ein/reasoning/02-kb-evolution - add contradiction rule defined as ein rule from fact and not fact infer
> (false); also can define owed/discharged and alive/dead rules; can say that if treat (open ?R) as facts then
> they are only facts that (1) duplicates (2) can be removed from KB; but maybe better would be to emit (open)
> facts as (open ?R <justification>) ? create new phase in M1f for that

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
| [S1f.21.1](s1f.21.1_the_survey_and_the_design.md) | **The survey and the design.** What the kernel does for each verdict, where, and what shows it; each verdict written as a rule, and whether Ein can load and run that rule today; `(open R)` today, the two properties shown on programs; `(open R <justification>)` designed: what the justification is, what holds a discharged debt, what changes in the engine, the stdlib, hypothesis generation, the read-out and the tests, on real corpus programs with their answers computed. Ends with the question for the user | next |
| S1f.21.2 | **The reasoning pages.** In `docs/ein/reasoning/02-kb-evolution/`: the contradiction rule, alive and dead, owed and discharged, each as a rule with its graph and its formula, and the remark on `(open R)` as facts; the user's pages where the same is said in Ein, if S1f.21.1 finds Ein can say it | after S1f.21.1 and S1f.20.10's lanes |
| S1f.21.3 | **`(open R <justification>)` in the engine**, if the user rules for it: planned from S1f.21.1's design when the ruling is given | waits for the ruling |
| S1f.21.4 | **The close.** | last |

## The question it will put

**Should an obligation emit `(open R <justification>)` as a fact of the knowledge base?** The user's *maybe*.
S1f.21.1 answers what it would mean and cost, and shows it on programs; the ruling is the user's, and S1f.21.3
is planned only on it. S1f.21.2 does not wait for it: it states today's kernel as rules, and says of `(open R)`
what the instruction says.

## Rules of the phase

The standing ones ([`AGENTS.md`](../../../AGENTS.md)): a stage per subagent; what a stage finds outside its scope
goes to a findings page of the phase's own; a claim about the engine is a run, not an argument; a new syntax is
shown on real corpus programs with their answers computed before it is built; no estimate is written.
