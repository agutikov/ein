# P1f.30 — the inverse of `ein render`: a labelled graph read as a knowledge base

**Milestone:** [M1f](../README.md)
**Status:** created 2026-11-18 on the user's instruction (below); **not started without the user's word**.
**Depends on:** P1f.23's S1f.23.1, which makes the canonical form `ein render`'s default and the labelled arrow a fold
on request (Q-M1f.419): the two drawings this phase reads back.
**Blocks:** the close of M1f.

## The instruction

The user, 2026-11-18, verbatim:

> Btw, what about inverse of render? We can take arbitrary directed 2-graph (only arrows, not hypergraph) with
> labelled nodes and labelled arrows (labels required) and convert it into ein KB, right? p1f.30

## What it is

Two readings of a drawing, as the main session put them that day:

1. **Any labelled digraph, an arrow a fact** (the fold read backwards): an arrow `a -[r]-> b` is the fact
   `(r a b)`, a node's label an atom's name, an arrow's label a relation. What it cannot keep or must decide: two
   nodes of one label are one atom (an object's identity is its name, Q-M1f.380); two parallel arrows of one label
   are one fact (a knowledge base is a set); a node no arrow touches is lost (a knowledge base is its facts); a
   label must be a name (an ASCII run of the grammar's allowed characters,
   [`01-grammar.md`](../../../docs/ein/user/04-reference/01-grammar.md)) and no word a stored fact may not have (`eq`, `neq`, `_`,
   `not` of two arguments); a node's label that is also an arrow's label is one atom, the relation, which Ein allows
   and the drawing may not have meant.
2. **`ein render`'s own drawing, the graph as it is** (the canonical form read backwards): a node with no arrow out
   is an atom, one with arrows out a fact, its arrows ordered by their numbers, its first at its relation's atom; a
   frame is layout. It is the exact inverse of `ein render kb`, facts of any count and facts under arrows included,
   and gives a round trip a test can hold: a knowledge base drawn and read back is the knowledge base. A folded
   drawing reads back by 1 for its labelled arrows and by 2 for the rest.

## Questions the survey puts, each with a default

- **The command**: a subcommand that reads a drawing and writes Ein text, beside `ein show` (which writes Ein text
  a `.ein` file could hold, and nothing else); or every command that takes a `.ein` takes a `.dot`, dispatched as a
  `.einb` is by its first bytes.
- **The formats**: DOT first, the format `ein render` writes; GraphML or JSON on request.
- **What cannot be read**: a label that is no name, two nodes of one label, a node no arrow touches, a node with
  arrows out under reading 1: refused with its line, renamed, or written with a note.
- **The round trips**: under 2, every corpus program's KB view read back equals its knowledge base (a test over the
  corpus); under 1, a graph read and drawn with the fold is the graph, but for what 1 cannot keep.

## Stages

| stage | what | state |
|---|---|---|
| S1f.30.1 | **The survey and the design**: the two readings on real drawings (corpus KB views in both forms, hand-made graphs with each case 1 cannot keep), what each command would print, the questions above with their defaults | not started |
| S1f.30.2 | **The build**, on the user's rulings | not started |
| S1f.30.3 | **The close** | last |

## Rules of the phase

The standing ones ([`AGENTS.md`](../../../AGENTS.md)): a stage per subagent; a new surface shown on real programs with
its output before it is built; a claim about the engine is a run; no estimate is written.
