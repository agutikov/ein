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
