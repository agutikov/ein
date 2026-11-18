# S1f.23.7 — a slice of the knowledge base defined by a query, and drawn

**Phase:** [P1f.23](README.md)
**Status:** planned 2026-11-18 on the user's answer to Q-M1f.426; task 1, the examples, started the same day on the user's word (*"go ahead"*); it stops for the user's look.

## The user's words

On Q-M1f.426, what the constraint view draws (the answers' file, `e4c0eaf6`), verbatim:

> So, it turns out that we have commands that render part of KB. I'm not against it - (a). Small addition - would be
> nice to formalize this KB slice with something, e.g. ein query or rule that selects all nodes where relation is
> argument (is it correct definition?), and together with `constraints` allow to render arbitrary query.

## What it asks

- **The constraint view's slice, stated in Ein**: a query (or a rule) whose answers are exactly the facts the view
  draws. The user's definition, *every fact with a relation among its arguments*, is close to the view's today
  (Q-M1f.426 (a): every un-annotated fact of a relation the program does not declare, the declarations among them) and
  not the same: a fact with a relation as an argument that the program gives with a `:source` is in the first and not
  the second, and an undeclared table of plain objects (a digit table) in the second and not the first. The stage
  shows both on the corpus and puts the definition to the user.
- **Any query's slice drawn**: `ein render kb` (or a sibling) given a goal draws the facts that answer it, with what
  they point at, in the canonical form; the constraint view becomes one such query. Q-M1f.431's query part of the
  program view, M20's *arbitrary query on a KB state* and M1h's P1h.6 (an open fact as a question) read the same.

## Stages

1. **Examples** (stops for the user): the two definitions side by side on the corpus's programs, counted; the slice
   of each as `ein render kb` would draw it, mocked from today's drawing; how a goal is given (a flag, a form in the
   program, a saved state with a query).
2. **The build**, on the user's rulings.

## Record

- Task 1 done, `ef2010ce`, waiting for the user's look: [`s1f.23.7_examples.md`](s1f.23.7_examples.md), its programs, scripts and outputs in [`s1f.23.7_examples/`](s1f.23.7_examples/).
