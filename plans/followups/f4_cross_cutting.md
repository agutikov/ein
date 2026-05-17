# F4 — Cross-cutting ideas

Theme owner: parking lot.
Trigger: opportunistic — none of these blocks any milestone.

This file collects ideas that recur across multiple
[`docs/ideas/`](../../docs/ideas/) notes but don't belong to any
one of them. Each gets a short stanza; promote individual items
to their own followup file when they grow.

## Open questions (parked)

| Q   | Title                                                                       |
|-----|-----------------------------------------------------------------------------|
| Q14 | Rule learning source — hand-written, library, LLM-suggested, learned?       |
| Q30 | Equality saturation / e-graph promotion — when does Zebra-scale benefit?    |
| Q31 | LLM as policy over the reasoning graph (M1.P1.5 search-tree choice)         |
| Q32 | 2-D / N-D spatial — when do we need beyond M1.P1.4's 1-D position lattice?  |
| Q33 | Reasoning-graph differential rendering for live agents                      |

---

## Rule learning from human walkthroughs (Q14)

Per [idea 06 §Open sub-questions point 5](../../docs/ideas/06-inference-rules-completeness.md#open-sub-questions).
M1's rules are hand-written; the user asks whether they could be
*learned* by observing human walkthroughs.

- **What it would look like**: ingest a corpus of annotated human
  walkthroughs; for each step, find the smallest pattern that
  fires the same conclusion; aggregate; propose new
  `(rule …)` definitions.
- **Why hard**: human walkthroughs collapse multiple propagation
  steps into one ("then by exclusion …"); inverting that requires
  knowing the granularity the engine uses.
- **Why interesting**: M1's rule set may turn out to be *incomplete*
  for problem classes the user hasn't yet looked at; learned rules
  would close gaps automatically.

Connection: [idea 06](../../docs/ideas/06-inference-rules-completeness.md),
[idea 08](../../docs/ideas/08-human-style-deductive-trace.md).

## E-graph promotion (Q30)

M1.P1.2 ships *equality-class IDs* as a placeholder; F4 explores
whether promoting the reasoning layer to a full e-graph
(equality saturation) buys anything on real puzzles.

- **Plausibly yes** if a puzzle has rich equational reasoning
  ("the Brit's pet is the same as the Spaniard's dog").
- **Plausibly no** for Zebra — the equality structure is shallow.

Connection: [idea 02 §What "compute directly on the graph" can
mean](../../docs/ideas/02-graph-as-formal-substrate.md#what-compute-directly-on-the-graph-can-mean),
[`docs/index/06-graphs-rewrite-systems.md §egraph`](../../docs/index/06-graphs-rewrite-systems.md).

## LLM-as-policy in search-tree (Q31)

Per [idea 09 §LLMAsPolicy](../../docs/index/09-cognitive-architectures-neurosymbolic.md):
the LLM picks *which hypothesis branch to explore first* in
M1.P1.5. AlphaZero-style guided proof search.

- **Why**: human walkthroughs make "good" branching choices —
  the kind a small policy net could learn.
- **Why hard**: serialising the search state for the LLM is
  itself a research problem; mid-search LLM calls are expensive.

Connection: [`docs/index/11-search-optimization-algorithms.md`](../../docs/index/11-search-optimization-algorithms.md) §MCTS / AlphaZero.

## 2-D / N-D spatial (Q32)

M1.P1.4's 1-D position lattice covers Zebra. *Logic-grid* puzzles
in 2-D, *chess-style* puzzles, *graph-colouring* puzzles all need
more. F4 would extend `:positional` to multi-dimensional and
attach Allen-style 2-D relations.

Connection: [M1.P1.4 S1.4.2](../m1_core_graph_reasoning/p1.4_constraints/s1.4.2_spatial.md) leaves the replacement seam explicit.

## Reasoning-graph differential rendering (Q33)

Real-time UI showing the graph evolving step-by-step. Useful for
debugging the trace planner (M1.P1.6) and for live demos. Probably
a Cytoscape.js page driven by the trace IR.

Connection: M1.P1.6, the existing
[`docs/index/knowledge-graph.cy/`](../../docs/index/knowledge-graph.cy/)
Cytoscape view as a template.

---

## How to promote

Any item that gains a concrete deliverable — a milestone scope, a
worked test puzzle, a measurable target — graduates to its own
followup file (rename `qNN_<topic>.md` and link from the F4 index).
When two related items appear in the same scope, bundle into a new
milestone folder under `plans/m_followups_<topic>/`.
