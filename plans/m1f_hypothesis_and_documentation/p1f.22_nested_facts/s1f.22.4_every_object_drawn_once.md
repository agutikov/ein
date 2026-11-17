# S1f.22.4 — `ein render` draws every object once

**Phase:** [P1f.22](README.md)
**Status:** planned 2026-11-17; **not started**: it follows [S1f.22.3](s1f.22.3_the_negation_of_a_negation.md).
**Rulings:** [Q-M1f.395](../open_questions.md#q-m1f395--what-ein-render-draws-of-a-fact-under-an-arrow) (every
object is drawn once; a fact of three arrows is a labelled arrow only when nothing points at it) and
[Q-M1f.398](../open_questions.md#q-m1f398--how-the-graph-is-drawn-to-show-the-global-context-and-a-facts-context)
(a fact is filled where held, and inside a dashed frame labelled with the context's relation where it is in a
context).

## What it builds

1. **`ein render kb`** (and every view that draws a knowledge base with it): a fact that a fact points at is
   drawn as **its node**, an octagon with the relation's name in it and its argument arrows numbered, held or
   not, once; filled where the knowledge base holds it; inside a dashed frame `R ctx` for the relation `R` of the
   facts that point at it, one frame naming every such relation, frames side by side. No node is labelled with a
   fact's text, and no fact is drawn both as an arrow and as a node (N-3, N-12).
2. **`ein render rule`**: a nested pattern the same way, its variables and atoms drawn, so that what it binds is
   seen (N-4).
3. **The pages, in the same commits**: the row *a fact under an arrow of another* of the knowledge-base page's
   table of the labelled-arrow form; `12-rendering.md`'s legend; the decision Q-M1f.395.
4. **The goldens**: the shape golden's lines of every program whose drawing moves, re-banked for every corpus
   program (`EIN_BLESS=1 EIN_CORPUS_SLOW=1`), with the count of programs that moved in the record.

## Acceptance

`./run_tests.sh` exit 0, and **`utils/ci_container.sh` exit 0 on the stage's last commit**, as for every change
to a DOT writer: `dot_wellformed.rs` is the one authority on a view being well-formed, and the runner's Graphviz is
part of what it checks. A drawing of a program with a fact in two contexts is in the record, as a file.

## Record

Not started.
