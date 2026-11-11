# S1g.6.3 — Seeing nested facts: the graph views, readable numbers and formulas, and rules as formulas

**Phase:** [P1g.6](README.md) — candidates C-5, C-6, C-7
**Status:** planned 2026-11-11, not started. Provisional until P1g.1's stop.
**Depends on:** P1g.1's page, which shows the views and traces as they are today
([S1g.1.1](../p1g.1_survey_and_design/s1g.1.1_what_ein_represents_today.md)'s T4); for C-7,
[S1g.4.2](../p1g.4_first_order_logic/s1g.4.2_relation_algebra.md)'s three-forms table, the hand-written
target.
**Pulls in:** [F15](../../followups/f15_math_formulae/ideas.md), rules exported as mathematical formulas
(the user's note, in Russian: an export of rules *"в виде математических формул"*, as a second level of
explanation between `ein-lang` and the human reading);
[idea 02](../../ideas/02-graph-as-formal-substrate.md), the graph as the substrate;
[idea 08](../../ideas/08-human-style-deductive-trace.md), the human-style trace;
[`09_natural_language.md`](../../../docs/kernel/ir/03-ein-lang/09_natural_language.md);
[`04_dot_rendering.md`](../../../docs/kernel/ir/03-ein-lang/04_dot_rendering.md).
**Stops for the user:** its first task. What a formula should look like is a matter of taste and of the
user's standing rule that anything built for review carries every artefact, raw and rendered.

## Why the logic needs it

Ein explains itself in two ways: drawings of the graph, and text rendered from each fact's justification.
Both were built when a value was a name. In this milestone almost every interesting value is a nested fact,
and both ways go blind on it:

- **C-5.** `ein render kb` and `ein render rule` draw a nested fact as **one opaque labelled node** (the
  coordinator's finding of 2026-11-11). A formula's tree, a proof term's steps and a number's digits are a
  string in a box. Two formulas that share a subformula share nothing in the drawing. Idea 02's claim, that
  the graph is the formal substrate, stops at the first parenthesis.
- **C-6.** A `:why` template fills `{?x}` with the value as Ein text, so a trace of the calculator reads
  `(n (n z 1) 7)` where a person reads 17 (the same finding), and a trace of a proof will read
  `(Imp (And p q) p)` where a person reads *(p ∧ q) → p*. S1f.5.14 measured how much of an answer reads as
  English; nested values are where it reads worst.
- **C-7.** A rule has no rendering as mathematics at all. F15 asks for three: the logical form, the
  relational form and the algebraic property, with a lowering between them and not a mechanical
  transliteration.

None of the three moves an answer. All three decide whether the milestone's pages can *show* what its
programs did, which is the user's first demand of every stage here.

## The three parts

**C-5, the views.** To be designed on real cases and shown before anything is built:

- A nested fact unfolded: a node per constructor application, an edge per argument, shared subterms shared
  (they are interned, so sharing is free to detect). Then a formula is a DAG, and a proof term a proof tree.
- The cost: a KB view of an arithmetic program already holds thousands of edges, and no layout finishes on a
  library of facts (E-10). Unfolding multiplies nodes. So unfolding is **selective**: a view of one fact, or
  of one query's answers, or to a depth; a `--view` of its own under `ein render`, not a change to the
  default drawing, whose goldens stay.
- A proof as a drawing: S1g.3.1's five lines as a graph, lines as nodes, citations as edges, each node its
  formula's tree or its text. This is the view the milestone's proof pages most want.
- Every DOT writer is held by `dot_wellformed.rs`, and a stage that changes one is not done until
  `utils/ci_container.sh` is green (S1f.5.29's rule).

**C-6, readable values.** The question is where the reading lives, and there are two answers that fit the
language as ruled:

- **(a) In the theory.** A module says how its values read: a relation's `:why` already is its fact's
  sentence, so `(relation Imp Formula Formula :why "if {?1} then {?2}")` makes a nested `Imp` render as its
  own sentence wherever a template prints it, recursively. Whether the renderer does that today for a
  *nested* fact, or prints it as Ein text, is **to be tried** on the binary; if it does not, making it do so
  is the small change, with every trace golden that holds a nested value named before it moves. A number
  would still read *"ten times (ten times 0 plus 1) plus 7"*, which is correct and unreadable.
- **(b) A rendering the theory declares for a shape:** a decimal number as its digits in a row, a formula
  in infix. That is a new declaration, so a grammar question for the user, and it must not be arithmetic: a
  number's digits are concatenated as text, never computed.
- The stage shows both on the calculator's trace and on a proof's, and the user picks. Strings stay
  natural-language text and nothing else (S1f.5.2); non-ASCII symbols such as ∧ are legal inside a string
  only (§ 1.6), which is exactly where a template is.

**C-7, rules as formulas.** `ein render` gains a view that writes a rule as a formula:

- **The logical form** is mechanical from the rule: universally quantify the variables, the `:match` as a
  conjunction, an implication, the `:assert`; `absent` as a negated existential marked as negation as
  failure and not as ¬; an obligation as an existential; `(false)` as ⊥. S1g.4.1's table *is* its
  specification, and the view is tested against the table's rows.
- **The relational form** needs the analysis F15 calls semantic lowering: recognise that a rule's variable
  pattern is a composition, a converse, an intersection. For rules of three variables it is a finite case
  analysis (Tarski and Givant); beyond three there is none, and the view says so.
- **The algebraic property** is a lookup of recognised shapes (transitive, symmetric, functional).
- Output as text and as LaTeX in markdown, since the review pages are markdown. Tested against S1g.4.2's
  hand-written table for every `std.algebra` rule: the exporter is done when it reproduces the table.

## Tasks

1. **T1g.6.3.1 — the page for the user:** today's drawings and traces of a formula, a proof and a number,
   beside mock-ups of each proposed view and reading, on the same programs; (a) against (b) for C-6.
2. **T1g.6.3.2 — the unfolded view,** as ruled, with its goldens, the DOT sweep and the container run.
3. **T1g.6.3.3 — readable values,** as ruled; the natural-language census (`utils/nl_census.py`) before and
   after on the calculator's and the logic's programs.
4. **T1g.6.3.4 — the formula view,** logical form first; relational and algebraic for the shapes recognised;
   held to S1g.4.2's table.
5. **T1g.6.3.5 — the pages:** `04_dot_rendering.md`, `09_natural_language.md`, the CLI reference for the new
   views, and the review bundle if it should draw them.

## Acceptance

- A formula, a proof and a number each have a drawing that shows their structure and a trace line a person
  can read, on the milestone's banked programs, with the raw artefact beside each.
- No existing view's golden moves unless named first; every new DOT is well-formed under the gate's Graphviz
  and the container's.
- The formula view reproduces S1g.4.2's logical form for every stdlib rule and its relational form for every
  rule of three variables or fewer, held by a test.
- No answer moves: R, N, H and every exit code are untouched by a view, held by the existing sweeps.

## What it may find

- **That the renderer already recurses** through a nested fact's `:why`, and C-6 is mostly a matter of
  writing good templates. Then the stage is smaller and says so.
- **That unfolded views do not lay out** past a small size under any Graphviz engine. Then the view is for
  single facts and proofs, and the page says where it ends.
