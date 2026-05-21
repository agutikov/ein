# Inference — the rule firing engine

> **Stub.** Becomes load-bearing when [P1.3](../../../plans/m1_core_graph_reasoning/p1.3_inference_rules/)
> ships. This page sketches the planned structure so the rest of the
> kernel tree can cross-link confidently.

The inference engine is what takes a populated
[`KnowledgeBase`](../ir/02-data-model/02_store.md) and produces
**reasoning-layer facts** by firing
[rules](../ir/01-ein-graph/02_rules.md). Everything else in the
kernel tree describes *what* the engine reads and writes; this
chapter describes *how* it does it.

---

## Planned structure (P1.3 — P1.5)

```text
docs/kernel/inference/
├── README.md         ← this file (stub)
├── 01_matcher.md     ← pattern matching: variable binding, predicate
│                       evaluation, multi-pattern conjunction
│                       (P1.3 S1.3.1)
├── 02_saturation.md  ← the firing loop; priority ordering; quiescence
│                       detection (P1.3 S1.3.3)
├── 03_hypothesis.md  ← branching: choose-an-undetermined-slot;
│                       fork + saturate + retract-on-contradiction;
│                       commit-on-uniqueness (P1.5)
├── 04_contradiction.md ← clash detection; unsat-core walk; trace
│                       generation for failed branches (P1.5)
└── 05_trace.md       ← step-by-step ein-lang trace generation;
│                       reordering pass; per-step / aggregate / DAG
│                       views (P1.6)
```

The five files map to plan phases; each ships when its phase does.

## What lives where today (M1 stubs)

| concept                         | M1 state                                                         |
|---------------------------------|------------------------------------------------------------------|
| Pattern matcher                 | `Pattern` dataclass holds structural view; matcher in P1.3       |
| Rule registry                   | `Rule` entity in [`02-data-model`](../ir/02-data-model/); rule definitions in `examples/rules.ein` (P1.3 T1.3.2.1-10) |
| Property-fact activation        | KB indexes `_rule_apps_by_rule` / `_rule_apps_on_relation` built at load |
| Saturation loop                 | not yet — placeholder in P1.3 S1.3.3                              |
| Hypothesis branching            | `kb.fork()` exists; full loop is P1.5                            |
| Contradiction detection         | `kb.unsat_core()` exists; clash detection in P1.5                |
| Trace generation                | `DerivationDAG.to_dot()` exists; markdown trace in P1.6           |

The data substrate (KB, entities, layer views, fork, provenance,
derivation DAG) is **complete** through P1.2. The engine that
*operates* on this substrate is the gap that P1.3 — P1.6 closes.

## Design principles (already locked in M1)

These are inherited from the graph + data-model docs and don't
change when the engine arrives:

1. **The graph is canonical, the engine is dynamic.**
   [`feedback_graph_canonical`](../../../README.md). The engine
   never replaces the KB; it appends to the reasoning layer.
2. **Rules can be higher-order.** Three rule types
   ([`../ir/01-ein-graph/02_rules.md`](../ir/01-ein-graph/02_rules.md));
   the matcher must enumerate relation variables.
3. **Every firing leaves provenance.** Rule-kind provenance with
   `premises_raw` and `bindings` is mandatory; trace fidelity
   ([idea 08](../../ideas/08-human-style-deductive-trace.md)) is an
   M1 acceptance gate.
4. **Lazy branching.** Saturate first with all propagation rules;
   branch only when no rule fires and the puzzle is not yet solved.
   ([Q19 working answer](../../../plans/m1_core_graph_reasoning/open_questions.md#q19).)
5. **Encoding-agnostic.** The engine must work over both `zebra.ein`
   (classic) and `zebra2.ein` (unified is-a) — the final IR encoding
   choice is deferred to P1.7. The matcher must consult
   `logical_types` / `logical_instances` where the encoding might
   matter.

## M1 invariant — `enable_alive_inherit` soundness

[`SolverConfig.enable_alive_inherit`](../../../ein.py/src/ein_bot/inference/config.py)
ships **on by default** as of S1.5.4 T1.5.4.8. With the flag on,
the hypothesis loop seeds the alive-candidate set **once at root
saturation** (via
[`generate_hypotheses_with_stats`](../../../ein.py/src/ein_bot/inference/hypgen.py))
and stashes it on `kb.alive`. Forks inherit `alive` through
[`kb.fork()`](../../../ein.py/src/ein_bot/kb/store.py); each
`_explore` entry re-prunes against the fork's KB and picks the
next hypothesis from what remains. `generate_hypotheses` runs
**once per `solve()` call**.

This is sound iff three pre-conditions hold across the puzzle's
rule library — collectively the **M1 invariant**:

1. **No new objects.** Rules don't `:assert` facts whose args
   introduce names that weren't already in the ontology /
   facts. (Q40 nested-Fact args are existing facts, not new
   names.)
2. **No new relations.** Rules don't `:assert (relation N S₀ S₁)`
   declarations — the relation registry is fixed by the ontology
   block.
3. **Hypotheses connect names only.** `_fill_slot` iterates
   `_instance_like_objects` and string-fills both slots; no
   nested-Fact hypothesis args.

Under these clauses, every admissible hypothesis is enumerable
from the root state; deeper branches **eliminate** candidates,
never extend the space.

**When the invariant breaks** (a rule library asserts new
`(relation …)`; F5 rules-as-data; a future puzzle's matcher
produces nested-Fact hypotheses):

- The default-on flag becomes **unsound** — alive entries may
  miss candidates introduced post-root.
- The escape hatch: set
  ``(config :enable-alive-inherit false)`` in the puzzle, or pass
  ``solve(kb, config=SolverConfig(enable_alive_inherit=False))``
  programmatically. The engine reverts to per-branch
  `generate_hypotheses(kb)` — the pre-`40b8dd4` shape — at the
  cost of re-enumerating every level.

Tracked at
[M1 Q-S1.5.4.D](../../../plans/m1_core_graph_reasoning/p1.5_hypothesis_loop/s1.5.4_hypgen_improvements.md#open-questions-parked-here)
as a long-term design seam; promote to a typed invariant check
when F5 lands.

## Where the design lives today

The complete plan, including task breakdown and acceptance criteria:

- Plan phase [P1.3 — Inference rules](../../../plans/m1_core_graph_reasoning/p1.3_inference_rules/).
- Plan phase [P1.5 — Hypothesis loop](../../../plans/m1_core_graph_reasoning/p1.5_hypothesis_loop/).
- Plan phase [P1.6 — Rendering + trace](../../../plans/m1_core_graph_reasoning/p1.6_rendering_and_trace/).
- Idea: [`docs/ideas/06-inference-rules-completeness.md`](../../ideas/06-inference-rules-completeness.md).
- Idea: [`docs/ideas/08-human-style-deductive-trace.md`](../../ideas/08-human-style-deductive-trace.md).

When P1.3 work begins, this stub becomes a hub for the
implementation reality.
