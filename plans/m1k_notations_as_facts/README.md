# M1k — other notations read as Ein facts

**Status:** created 2026-11-18 on the user's instruction (below); **not started as a milestone**. Its first phase came
from M1f (P1f.30, created and surveyed the same day); a phase runs only on the user's word.
**Depends on:** M1f's P1f.23, S1f.23.1: `ein render` draws a fact in the canonical form by default and folds a fact of
three arrows on request (Q-M1f.419), the drawing P1k.10 reads back.
**Where it sits** against M1g and M1h is not ruled; the user has said the work stops after M1f.

## The instruction

The user, 2026-11-18, verbatim:

> M1k: p1f.30 -> p1k.10; p1k.20: arbitrary S-expressions as Ein facts: Lisp, SMT, link grammar output; same
> approach - just pick random files (from repo and outside), parse as Ein and render ;

## What it is

Ein's knowledge base is a graph of atoms and facts, and its program text is S-expressions. Other notations that are
graphs or trees can be read as facts: what reads, what does not, and why, found on real files before anything is
built.

| phase | what | state |
|---|---|---|
| [P1k.10](p1k.10_graph_to_kb/README.md) | **A labelled graph read as a knowledge base**, the inverse of `ein render`: DOT only, read by `ein show`; any labelled digraph an arrow a fact, and `ein render`'s own drawing read back exactly | moved from M1f's P1f.30; S1k.10.1's examples done (28 drawings); the design and the build **not started without the user's word** |
| [P1k.20](p1k.20_s_expressions_as_facts/README.md) | **Arbitrary S-expressions as Ein facts**: Lisp, SMT-LIB, link grammar's output; random files from the repo and outside, parsed as Ein and drawn | created 2026-11-18; **not started without the user's word** |

## Rules of the milestone

The standing ones ([`AGENTS.md`](../../AGENTS.md)): a phase per subject, a stage per subagent; a new surface shown on
real files with its output before it is built; a claim about the engine is a run; third-party files are read and
never run, kept out of git where their licence is not the project's, cited by their URL; no estimate is written.
