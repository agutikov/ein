# P1g.12 — Contexts: facts inside facts, and worlds the real one does not hold

**Milestone:** [M1g](../README.md)
**Status:** planned 2026-11-16, on the user's words of that day ([§ The instruction](#the-instruction)). Not
started. No stage file yet: the phase is this README, its stages are sections below, written out as files when
the phase is taken up. Provisional until [P1g.1](../p1g.1_survey_and_design/README.md)'s stop.
**Depends on:** [P1g.1](../p1g.1_survey_and_design/README.md) (what Ein represents today, the user's page), and
the nested-fact requirements of [P1g.6](../p1g.6_engine_requirements/README.md):
[S1g.6.1](../p1g.6_engine_requirements/s1g.6.1_a_saturation_that_does_not_stop.md) (a saturation that does not
stop, which nesting can cause), [S1g.6.2](../p1g.6_engine_requirements/s1g.6.2_nested_facts_at_scale.md) (nested
facts at scale), [S1g.6.3](../p1g.6_engine_requirements/s1g.6.3_seeing_nested_facts.md) (how a view draws them),
[S1g.6.4](../p1g.6_engine_requirements/s1g.6.4_witnesses_and_fresh_names.md) (hypotheses about nested facts) and
[S1g.6.5](../p1g.6_engine_requirements/s1g.6.5_rules_as_data_and_refutations.md) (rules as data).
**Its twin in M1h:** [P1h.5](../../m1h_beyond_word_problems/p1h.5_contexts_in_a_text/README.md), contexts in a
text: what a character says, believes and knows, against what the story makes true.

## The instruction

The user, 2026-11-16, after two example programs showed facts as other facts' arguments
([`examples/beliefs/`](../../../examples/beliefs/)) and that a rule can pack and unpack them, verbatim:

> Ok, so p1g.6 - add there ideas about nested facts - there is an option to create a sub-kb/sub-program with
> nested facts, sub-context, that contains only sub-facts of every context (knows Ann ...) (knows Bob ...) and
> what it could be inside? same rules or different? they are available from outer context, what about outer
> facts inside inner context? recursion...; I don't like them be a simple objects, but also don't like to put
> into the same KB, maybe we can make isolated sections of KB, or it can be done via rules only - model of the
> world where somebody can believe something but those beleives do not affect "real" facts. It looks like this
> topic deserves at least separate phase in m1g and m1h

## What Ein does today

- **A fact can be another fact's argument**: a value is a name or a fact a state could hold (the value rule, M1f
  P1f.7). `(believes Ann (likes Dan Soup))` stores one fact, the belief; the content is a value no state holds,
  neither true nor false, identified by its structure, and no rule sees it as a fact.
- **A rule can unpack** a content into a fact of its own, one rule per shape of content (a conclusion cannot be a
  bare variable): [`01_common_belief.ein`](../../../examples/beliefs/01_common_belief.ein), *what every person
  believes is true*.
- **A rule can pack** a fact the state holds into another fact's argument, making no new name:
  [`02_packing.ein`](../../../examples/beliefs/02_packing.ein).
- **A rule that packs a fact into itself saturates without end** ([S1g.6.1](../p1g.6_engine_requirements/s1g.6.1_a_saturation_that_does_not_stop.md)).
- **No context exists**: there is one KB per state, and a "belief world" is only the set of a relation's nested
  values. Nothing derives the consequences of Ann's beliefs *inside* her beliefs unless a rule over `believes`
  facts does.

## The three ways the user names

| | way | what the user said of it |
|---|---|---|
| **A** | **nested facts as simple values**, today's | *"I don't like them be a simple objects"* |
| **B** | **isolated sections of the KB**: a sub-KB, a sub-program, per context, holding *"only sub-facts of every context (knows Ann ...) (knows Bob ...)"* | *"maybe we can make isolated sections of KB"* |
| **C** | **rules only**: a model of the world in the rule language, *"where somebody can believe something but those beleives do not affect \"real\" facts"* | named as the alternative to B; *"also don't like to put into the same KB"* is about A's single KB |

## The user's questions, and the ones they imply

From the instruction, each the user's:

1. **What could be inside a context?** The sub-facts of every `(knows Ann …)`, `(believes Bob …)`: one context per
   agent and attitude, or one per fact's head?
2. **Same rules or different?** Does Ann's belief world saturate under the program's rules, under rules of its
   own, or under none?
3. **Visibility, inward and outward.** The inner facts *"are available from outer context"*; **what about outer
   facts inside inner context?** Does Ann believe what is true, everything that is true, or only what she is told?
4. **Recursion.** `(believes Ann (believes Bob (likes Dan Soup)))`: contexts inside contexts, and where it stops.

What they imply, for the stages to settle:

5. **A contradiction inside a context.** If Ann believes two things that clash, is her context dead, and does that
   leave the real state alive? Under B a section's death need not be the state's; under A and C it is a fact
   pattern a rule must read.
6. **Absent and closure per context.** "Everything Ann believes" needs Ann's beliefs closed, as
   `01_common_belief.ein` needed `person` closed; under B each section has its own closure boundary.
7. **Hypotheses in a context.** May a commitment be about a context (`Ann believes p`, guessed), or inside one
   (guessing within Ann's world), and what is H then?
8. **What R is.** One region over the real state with contexts as values, or a region per context?
9. **Monotonicity.** A context that reads the outer world through `absent` is a source of non-monotonicity of its
   own; the static check would have to see it.
10. **How a view draws a context**, and how a trace says *"inside Ann's beliefs"* ([S1g.6.3](../p1g.6_engine_requirements/s1g.6.3_seeing_nested_facts.md)).

## What to read before designing

- John McCarthy, *Notes on Formalizing Context* (IJCAI 1993): `ist(c, p)`, *p is true in context c*, lifting
  axioms between contexts. R. V. Guha, *Contexts: A Formalization and Some Applications* (1991) and Cyc's
  microtheories: contexts as sections of a knowledge base, which is the user's way B.
- Jaakko Hintikka, *Knowledge and Belief* (1962); Fagin, Halpern, Moses and Vardi, *Reasoning About Knowledge*
  (1995): epistemic logic, possible worlds, the KD45 and S5 systems, the muddy children.
- Luís Monteiro and António Porto, *Contextual Logic Programming* (1989): contexts as units of a logic program.
- Michael Gelfond, epistemic specifications in answer set programming (1991, 1994).
- Alchourrón, Gärdenfors and Makinson (1985): belief revision, for when a context changes.
- The user's own [idea 09](../../ideas/09-puzzles-beyond-zebra.md): *"Should the engine grow an epistemic layer
  (muddy children, hat puzzles), or is that a separate sub-project?"*, unanswered until this phase.

## Stages, in outline

1. **S1g.12.1 — The three ways on real programs, and a stop.** Way A as it is (the two `examples/beliefs/`
   programs); way C written today in the rule language, a belief world as a relation with rules that derive inside
   it and never into the real facts, as far as it goes; way B sketched as the program a user would write, with
   what the engine would have to do. Each of the ten questions answered by each way, side by side. **Stops for the
   user**: which way, or which mix.
2. **S1g.12.2 — The way ruled, designed**: what a context is, what is inside, which rules apply inside, what crosses
   in and out (lifting), how a contradiction inside is reported, closure per context. Real examples first.
3. **S1g.12.3 — Recursion**: contexts inside contexts, a bound, and its relation to [S1g.6.1](../p1g.6_engine_requirements/s1g.6.1_a_saturation_that_does_not_stop.md)'s.
4. **S1g.12.4 — Hypotheses and R with contexts**: what H holds, what a state is, what the oracle computes.
5. **S1g.12.5 — The puzzles as tests**: muddy children, knights and knaves (already `examples/binary_logic/knights.ein`
   in one encoding), a hat puzzle; each held to an answer computed outside Ein.

## What it must not do

- **Not design before the stop**: the user picks the way on S1g.12.1's programs.
- **Not make a belief change a real fact** except through a rule the program states.
- **Not break the alive-set invariant** (no rule asserts a name the program lacks) or the value rule silently: a
  change to either is a ruling.
