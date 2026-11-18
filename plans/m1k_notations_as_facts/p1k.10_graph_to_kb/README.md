# P1k.10 — the inverse of `ein render`: a labelled graph read as a knowledge base

**Milestone:** [M1k](../README.md)
**Status:** created 2026-11-18 on the user's instruction (below) as M1f's P1f.30, and **moved the same day to M1k as
P1k.10** on the user's word (*"M1k: p1f.30 -> p1k.10"*); the user's notes below say *p1f.30*, as written. S1k.10.1's
examples are done; its design and the rest are **not started without the user's word**.
**Depends on:** M1f's P1f.23, S1f.23.1, which makes the canonical form `ein render`'s default and the labelled arrow a
fold on request (Q-M1f.419): the two drawings this phase reads back.
**Blocks:** nothing.

## The instruction

The user, 2026-11-18, verbatim:

> Btw, what about inverse of render? We can take arbitrary directed 2-graph (only arrows, not hypergraph) with
> labelled nodes and labelled arrows (labels required) and convert it into ein KB, right? p1f.30

and, the same day:

> p1f.30: DOT input only, ein show reads it

and:

> p1f.30: isolated node becomes a unary fact

and, during S1k.10.1's examples:

> p1f.30 - each generated ein is loaded, but no need to saturate it, because it has no rules; undirected arcs can be
> replaced with 2 directed, main requirement are labels, but for experiment we can allow default labels - options
> are a single one or all different.

## What it is

Two readings of a drawing, as the main session put them that day:

1. **Any labelled digraph, an arrow a fact** (the fold read backwards): an arrow `a -[r]-> b` is the fact
   `(r a b)`, a node's label an atom's name, an arrow's label a relation. What it cannot keep or must decide: two
   nodes of one label are one atom (an object's identity is its name, Q-M1f.380); two parallel arrows of one label
   are one fact (a knowledge base is a set); a node no arrow touches is **a fact of its own atom, `(Dog)`** (ruled 2026-11-18, *"isolated node becomes a
   unary fact"*: a fact whose one arrow is at the atom, which the knowledge-base page allows, $n = 0$; so no node is
   lost and no relation's name is made up); a
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

- **The command**: **ruled 2026-11-18, `ein show` reads it** (*"ein show reads it"*): it reads a drawing and
  writes the knowledge base as Ein text, as it writes what it lists today, and nothing else on stdout. How it is
  asked (a subcommand of `ein show`, or `ein show` dispatching on a `.dot` file) is the survey's, with a default.
- **The format**: **ruled 2026-11-18, DOT only** (*"DOT input only"*).
- **What cannot be read**: a label that is no name, two nodes of one label, a node with arrows out under reading 1
  (a node no arrow touches is ruled: a fact of its atom alone): refused with its line, renamed, or written with a note.
- **The round trips**: under 2, every corpus program's KB view read back equals its knowledge base (a test over the
  corpus); under 1, a graph read and drawn with the fold is the graph, but for what 1 cannot keep.

## Stages

| stage | what | state |
|---|---|---|
| S1k.10.1 | **The survey and the design**: the two readings on real drawings (corpus KB views in both forms, hand-made graphs with each case 1 cannot keep), what each command would print, the questions above with their defaults | **its examples started 2026-11-18 on the user's word** (*"Find some examples from internet or draw yourself, e.g. state machine, an algorithms, class diagram, C4 diagrams, https://graphviz.org/gallery/ ... whatever; and try convert them into Ein KB, record what succeded, what failed and why."*): [`s1k.10.1_examples.md`](s1k.10.1_examples.md); the design after it. **Record, 2026-11-18**: the examples are commit `795b5331`: 25 drawings read by reading 1 (17 downloaded, kept in `build/p1f30-external/`, and 8 drawn here) and 3 of `ein render kb`'s read by reading 2, with the prototype, [`s1k.10.1_examples/graph2ein.py`](s1k.10.1_examples/graph2ein.py). Strictly, reading 1 loads 2 of the 25; with the labels repaired and a default label for an unlabelled arrow, all 25 load and all 1,438 two-argument facts come back as their arrows; one default label is favoured over one each (the same facts, a KB independent of the order arrows are written in, a relation a rule can read). Reading 2 reads Ein's drawings back but for a rule's activator, a negation no fact points at, and a loop, which today's drawing draws alike for `(r x)` and `(r x x)`. The defaults the examples suggest for the questions above are the page's last two sections |
| S1k.10.2 | **The build**, on the user's rulings | not started |
| S1k.10.3 | **The close** | last |

## Rules of the phase

The standing ones ([`AGENTS.md`](../../../AGENTS.md)): a stage per subagent; a new surface shown on real programs with
its output before it is built; a claim about the engine is a run; no estimate is written.
