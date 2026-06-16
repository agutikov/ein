# Completeness of inference rules

## The open question

The 5-year-old `Ein` defines two inference rules:

- **Triangle** (transitive composition).
- **Square** (spatial propagation).

The user asks: *is this enough?* If it's enough for Zebra
specifically, what other rules exist, and for which task classes do
they apply?

## User's own words

> The second half [of the problems with the design] — inference rules.
> It's unclear whether these two are enough. If they're enough for this
> task, what other rules might exist and for which tasks?

## Why this matters

The number and shape of inference rules determines:

- **What classes of problems** the reasoner can solve at all.
- **How human-readable** the proof traces are (a small named set of
  rules → readable; an unnamed implicit set → opaque).
- **Whether the system has any chance of being formally
  characterised** — without an explicit calculus, completeness /
  soundness arguments are out of reach.

## A rough taxonomy of inference rules used in practice

This is a *map*, not a claim of completeness — these are candidate next
rules to add.

| family | example | source / kind |
|---|---|---|
| Composition / transitivity | `A→B, B→C ⇒ A→C` | categorical composition |
| Equality substitution | `A=B, B→C ⇒ A→C` | congruence closure / e-graph |
| Exclusivity / `allDifferent` | `H1=Yellow ⇒ H1≠Red, H2≠Yellow, …` | global constraint |
| Elimination by exhaustion | `A∉{x,y,z}, dom(A)={x,y,z,w} ⇒ A=w` | classic puzzle move |
| Hypothesis + contradiction | `assume A=x → derive ⊥ ⇒ A≠x` | natural deduction, ATMS, CDCL |
| Constraint propagation | `B = A+1, A∈{1..5} ⇒ B∈{2..6}` | CSP arc consistency |
| Path / arc consistency | `A next B, A=1 ⇒ B=2` | CSP |
| Spatial / interval | `A right_of B, A immediate ⇒ pos(A)=pos(B)+1` | Allen / RCC |
| Global cardinality | "exactly one X per Y" | CP global constraint |
| Forced-by-unique-position | "only this slot can hold X" | dual of exclusivity |

The user's "triangle" covers row 1; "square" partially covers rows 8
and 7. Rows 3, 4, 5 in particular are clearly needed for any
human-style Zebra walkthrough (see
[08-human-style-deductive-trace.md](08-human-style-deductive-trace.md))
and are currently absent or implicit.

## What "enough" should mean

Several non-equivalent answers:

1. **Functional sufficiency** — the rule set can derive the unique
   model whenever one exists (modulo hypothesis branching). This is
   what backtracking + a few rules already achieves; brute force is
   "complete" trivially.
2. **Propagation-complete** — the rule set discovers everything
   derivable *without* branching, only branching when the puzzle is
   genuinely under-constrained at that step.
3. **Explanation-complete** — every reasoning step in a canonical
   human walkthrough can be matched to a named rule firing.
4. **Domain-complete** — the rule set characterises an entire family
   of problems (logic grids, Sudoku, Einstein puzzles, …) and the
   missing rules expose the boundary of what the reasoner can do.

The user's framing is closest to (3) and (4): the question presumes
*more* rules are wanted, not fewer.

## Open sub-questions

1. **What is the right rule-presentation language?** Free Python
   functions? A `(pattern → conclusion)` graph-rewrite DSL? A
   declarative Horn-clause / Datalog encoding?
2. **How is rule provenance recorded?** Each derived edge tagged with
   the rule that produced it — required for the explanation graph.
3. **How is rule selection ordered?** Forward-chaining queue?
   Priority by cheapness / informativeness? Random?
4. **Are rules per-puzzle or universal?** A "right-of" rule is
   spatial-puzzle-specific. Does the engine come with batteries
   included or do puzzles ship their own rules?
5. **Where does rule learning come from?** Hand-written, library,
   LLM-suggested, or learned by observing human walkthroughs?

## Connections (context, not answers)

- Rules as graph rewrites (DPO/SPO):
  [05-category-theory.md](../../docs/lib/05-category-theory.md).
- CSP-side propagation rules (arc / path consistency,
  `allDifferent`): [02-solvers-csp-sat-smt.md](../../docs/lib/02-solvers-csp-sat-smt.md), §7.
- Categorical equivalence of "triangle" with composition:
  [07-categorical-formulation.md](07-categorical-formulation.md).
- TMS / ATMS — natural home for the hypothesis-contradiction rule:
  [09-cognitive-architectures-neurosymbolic.md](../../docs/lib/09-cognitive-architectures-neurosymbolic.md).
