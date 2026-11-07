# S1f.20.4 — `docs/ein/reasoning/` — implementation-agnostic

**Phase:** [P1f.20](README.md) (`docs/ein/`: the tree a released system would have)
**Estimate:** 6 days.
**Status:** not started. It was `T1f.5.20.4` until 2026-11-08.
**Depends on:** [S1f.20.2](s1f.20.2_the_disposition_table.md)'s table, reviewed. Its
`rule-evolution/analysis-of-rules.md` is sequenced with
[S1f.5.6](../p1f.5_documentation_and_other/s1f.5.6_rule_priority.md), whichever runs second.

## What it writes

Every page the table sends to `docs/ein/reasoning/`.

The hard third, because the source material is *not* implementation-agnostic.
`inference/architecture_and_algorithms.md` (821 lines on 2026-08-28) and the eleven design contracts describe
`ein.rs`. The split is not a move: it is a **rewrite with a question asked of every paragraph**: *would this
still be true of a second implementation?* If yes, it is `reasoning/`; if no, it is `ein.rs/`
([S1f.20.5](s1f.20.5_ein_rs.md)).

Four known traps:

- **`defined_behaviour.md`** is the diagnostics, orderings and error strings. It is both a user-facing
  reference (the diagnostics) and an implementation-agnostic contract (the orderings), so it splits.
- **The M1d pages** are the best implementation-agnostic writing in the repo (`the_boundary`, `completeness`,
  `the_verdict`, `obligation_forms`), and they are also milestone documents in voice. De-scaffolding them is
  most of the wordcount and none of the difficulty.
- **`rule-evolution/analysis-of-rules.md`** is *empty until something fills it*. S1f.5.6 fills it: the rule
  dependency graph, the strata, and what a rule set's structure decides. Sequence the two so the page is
  written once.
- **`kb-evolution/search.md`** was mapped from `inference/algorithm_layer_n.md`, a page about a walk that has
  had no code since M1e2 S1e2.1c.5. What it states now is the region R and its routes
  ([`solution_semantics.md`](../../../docs/kernel/inference/solution_semantics.md)), as the redrawn table
  says.

## How it moves a page

Each page moves with its inbound links, in one commit, and its release-voice count falls to 0 with it
([P1f.20 § Two rules](README.md#two-rules-across-the-stages)).

## The method

### Task T1f.20.4.1 — `static-ir/` and `kb-evolution/`

Each page of the table's rows, every paragraph asked the question above.

### Task T1f.20.4.2 — `rule-evolution/`

`rules-as-data.md`, `analysis-of-rules.md` with S1f.5.6's content or after it, and `transformations.md` as a
stub with its question in it.

## Acceptance

- Every row the table sends to `reasoning/` is written there.
- No paragraph under `reasoning/` names a crate, a module or a type of `ein.rs`.
- The link check is green, and the release-voice baseline has no line for a page under
  `docs/ein/reasoning/`.
- `./run_tests.sh` is green.
