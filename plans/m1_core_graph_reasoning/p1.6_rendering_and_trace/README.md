# P1.6 — Rendering + markdown trace

**Estimate:** 1-2 weeks.
**Depends on:** P1.2 (graph + provenance), P1.3 (saturator emits
firings), P1.5 (search tree).
**Blocks:** P1.7 (the Zebra acceptance test reads the markdown
trace and asserts the rule firings match the human walkthrough).

## Goal

Render the engine's work as a *story* — DOT diagrams for state
snapshots and structures, plus a markdown narrative that threads
the diagrams together.

The acceptance criterion is set by
[`docs/ideas/08-human-style-deductive-trace.md`](../../../docs/ideas/08-human-style-deductive-trace.md):
"the solver should reproduce a deductive trace of the kind a
human would write". Every named reasoning move in the Zebra
walkthrough must surface as a named rule firing with its premises
and a one-sentence English explanation.

## Stages

| ID      | Title                                  | Duration |
|---------|----------------------------------------|----------|
| S1.6.1  | DOT — rules + constraints              | 2 days   |
| S1.6.2  | DOT — state + state transitions        | 2-3 days |
| S1.6.3  | DOT — search tree                      | 2 days   |
| S1.6.4  | Markdown trace renderer                | 4-5 days |
| S1.6.5  | idea-08 trace acceptance               | 2-4 days |

## Defaults — compact view, project-wide; Levi only by flag

User direction recorded 2026-05-27: the canonical Levi-bipartite
graph representation reflects ein-lang structure faithfully
(atoms/names are nodes with arrows *to*, relations as
list-nodes with arrows *from*), but it's not readable as a
default view. **Compact rendering is the default for every
phase that touches DOT output;** Levi-bipartite stays available
on request via a `--levi` flag (or `EIN_RENDER_LEVI=1` env).

The mode is set per top-level form:

- **ontology / facts / reasoning** — compact (entity-style:
  instances + arrows, the abstract view).
- **rules** — `rule-mode=a` (Side-by-side LHS | RHS clusters)
  with `rankdir=LR`. The two existing modes ((a) clusters,
  (c) overlay) get folded into one default diagram per rule
  rather than the current cross-product; the `(c)` overlay
  variant moves behind a `--rule-mode=overlay` flag.
- **trace** — `trace-view=a` (per-step DOT). The aggregate
  and derivation-DAG views stay available behind
  `--trace-view={aggregate,dag}`.

## render_examples.sh — collapse the matrix

[`utils/render_examples.sh`](../../../utils/render_examples.sh)
currently produces six variants per input file
(`rule-mode ∈ {a, c} × trace-view ∈ {a, b, c}`). Update it
to render **one variant per file** under the new defaults:
compact rule mode (a, LR), per-step trace mode (a). The other
modes are addressable via env vars / flags for the rare cases
that need them. Note: example puzzles under `examples/`
don't carry `(trace …)` blocks, so the trace dimension is a
no-op for most of them anyway — only the `(rule …)`
output changes.

## VSCode ein syntax highlighting

Owned by P1.6 since it lives next to the DOT renderer work
and shares the IR grammar surface. Lightweight TextMate /
LSP-free grammar definition (`.tmLanguage.json`) covering:

- S-expr structure (`( … )` nesting, atom vs string literal)
- Keywords (`relation`, `rule`, `match`, `assert`, `is-a`,
  `functional`, `injective`, `co-located`, `adjacent-via`,
  `not`, `absent`, `forall`, `open`, `query`, `facts`,
  `ontology`, `config`)
- Variables (`?var` form)
- Comment lines (`;` prefix)
- Optional: brace-matching, snippet for `(rule … :match …
  :assert … :why … :priority …)`.

Land as a separate sub-stage if the TextMate file grows
non-trivial; otherwise fold into S1.6.1 (rules rendering)
since both edit the same set of rule keywords.

## Acceptance

- `ein-bot solve zebra.ein --trace=out.md --diagrams=out/`
  writes a complete trace bundle.
- `out.md` reads as a coherent narrative: each step names the rule,
  quotes the source condition, and links to the matching `out/*.svg`
  snapshot.
- The trace's named rule firings match
  [the target walkthrough](../../../docs/ideas/08-human-style-deductive-trace.md#the-target-trace-paraphrased)
  one-to-one (P1.7 enforces this).

## Connections

- [Idea 08](../../../docs/ideas/08-human-style-deductive-trace.md) —
  the whole phase is about delivering its acceptance criterion.
- [Idea 03 §The implicit fourth class](../../../docs/ideas/03-three-task-classes.md) —
  the *explanation* task class falls out of this rendering work.
