# M1p — Ein to natural language

**Status:** created 2026-11-20 on the user's instruction (below); **not started**. Its three phases are named by
the instruction and drafted here as their subjects and their open questions; none is planned into stages, and none
runs without the user's word.
**Depends on:** M1f, which is running: its last phase changes what a fact and a hypothesis are (`open` facts,
the contradiction as an imported theory), and this milestone reads the language as that phase leaves it.
**Where it sits** against M1g, M1h and the other milestones after M1f is not ruled; the user has said the work stops
after M1f.

## The instruction

The user, 2026-11-20, verbatim:

> new milestone M1p improve ein to NL translation: we removed inheritance of :why from relation declaration to relation facts, now want to replace the removed mechanic to make (1) all ein facts, inference, conclusions translated to NL (2) add custom fast formatting, convertion to strings, of variables, e.g. decimal numbers. 3 phases: custom formatting, facts formatting, reasoning formatting

## What it is

**What was removed.** A relation's declaration carries a `:why`, and a fact of the relation used to read by it.
That went: a `:why` reads the form it is written on, so a declaration's text is the declaration's own statement,
and a fact of the relation takes no text from it
([Q-M1f.58](../../docs/decisions/read-out/Q-M1f.58-a-fact-reads-in-english-in-the-trace.md)). Today a fact
reads by its reason, a given's `:source` or the deriving rule's `:why` filled with that firing's bindings, and
where it has none, as written, an s-expression. The page of how Ein reads as English today is
[`14-natural-language.md`](../../docs/ein/user/03-syntax-and-semantics/14-natural-language.md); its § 5 lists
where a text has nowhere to live.

**What this milestone puts in its place**, by the instruction's two aims:

1. **Every fact, every inference and every conclusion of Ein is translated to natural language**: no line a reader
   is shown falls back to an s-expression for want of words.
2. **Custom, fast formatting of what stands in a variable**, its conversion to a string: a decimal number of the
   calculator is the instruction's example, a numeral that is a nested term today and reads as one.

| phase | what | state |
|---|---|---|
| [P1p.1](p1p.1_custom_formatting/README.md) | **Custom formatting**: how the value a variable holds is written as text, said by the program or the library that owns the value, and applied fast; a decimal number first | created 2026-11-20; **not started** |
| [P1p.2](p1p.2_facts_formatting/README.md) | **Facts formatting**: every fact reads in natural language, what replaces the text a fact once took from its relation's declaration | created 2026-11-20; **not started** |
| [P1p.3](p1p.3_reasoning_formatting/README.md) | **Reasoning formatting**: an inference, a conclusion, a refutation and an answer read in natural language, built on the two before | created 2026-11-20; **not started** |

The order is the instruction's: a value's text first, since a fact's text is made of its arguments', and a line of
reasoning is made of facts'.

## What it must square with

These are standing, each a ruling or a decision, and none is reopened by creating the milestone. A phase that
needs one changed puts the question first.

- **Ein has no numbers** and none is planned; arithmetic is a theory in the standard library, a numeral a term of
  it. So a number's text is the text of a term a theory owns.
- **The kernel reads no theory's name**
  ([Q-M1f.436](../../docs/decisions/engine/Q-M1f.436-the-kernel-reads-no-theorys-name.md)): what formats a
  decimal number cannot be the kernel knowing what a decimal number is.
- **A string is natural-language text and never an object**: it is written after `:why`, `:source` or
  `:goal-text`. A conversion to a string is then something a surface does when it prints, or the rule changes.
- **A `:why` reads the form it is written on** (Q-M1f.58, above): what gives a fact its words is to be another
  thing than a second meaning of the declaration's text, or that decision is the one revised.
- **A claim is Ein**: the read-out's `(model …)` blocks and what a claim is pasted from stay s-expressions.
- **Names are ASCII**; a text is not.
- **The project documents its current state**, and a new surface is shown on real corpus programs, with their
  output, before it is built.

## Beside it

- [M2](../m2_nl_to_ir/README.md), natural language to Ein: this milestone is the other direction, and the two meet
  in a round trip, whose target is the annotated walkthrough of the guide
  ([`06-zebra-walkthrough.md`](../../docs/ein/user/02-guide/06-zebra-walkthrough.md)).
- The user's ideas [08, a human-style deductive trace](../ideas/08-human-style-deductive-trace.md), and
  [04, the pipeline from language to graph to solver](../ideas/04-nlp-to-graph-to-solver-pipeline.md).

## Rules of the milestone

The standing ones ([`AGENTS.md`](../../AGENTS.md)): a phase per subject, a stage per subagent; a new syntax or a
new surface shown on real corpus programs with their output before it is built; a claim about the engine is a run;
no configuration changes an answer, and a text is no answer; no estimate is written. **A phase is planned into
stages and run only on the user's word.**
