# S1a.3.3 — The saturator

**Phase:** P1a.3 (Deductive core)
**Estimate:** 5 days
**Depends on:** [S1a.3.2](s1a.3.2_matcher.md)
**Implements:** `ein/inference/{saturator,engine}.py`,
[design/06](../design/06_saturation.md)

## Context

The two-phase driver: a purely positive closure run to quiescence, then
one boundary admission, repeat. Plus the semi-naive delta enqueue, the
priority heaps, the redundant-firing path with its alternative-justification
recording, and the native `__symmetric__` mirror.

Both exact optimisations from [design/06](../design/06_saturation.md)
land here — Win A (compile once, ever) in this stage, Win B (the
semi-naive boundary) in [S1a.3.4](s1a.3.4_world_and_contradiction.md).

## Acceptance

- T2 parity for `enqueue` / `fire` / `mirror` / `quiesce` / `alt` events
  on every saturation fixture and on both zebra roots.
- Counters identical, `naf_dropped == 0`.
- Compile calls on exhaustive zebra2 down from 17 430 to one per distinct
  `(rule, activator)` pair (~170), cache order unchanged.
- `ein.rs saturate <file>` byte-identical to `ein saturate <file>`,
  `--dump` included.
- `SaturatorStepLimitError` message and behaviour identical under
  `--max-steps`.

## Tasks

### Task T1a.3.3.1 — Engine

Plan list per engine in ein.py's `compile_all` order (rules registry
order × `_rule_apps_by_rule` order), backed by the shared memo.
`_activators_for` with its arity filter. Recompute only when
`_rule_apps_by_rule` grew, tracked by a version counter — the *result*
must equal a full recompute, asserted in debug builds.

Also `Engine.step` / `Engine.saturate` — the simpler two-phase driver
that predates the `Saturator` and is still used by tests and by
`naf_dependency_map`.

### Task T1a.3.3.2 — Queues and dedup

Two `BinaryHeap`s keyed on `(priority, tiebreaker)` with the entry
payload in a side arena. `_seen` keyed on `(binding_key, guard_set)` —
the S1.22.0 fix, without which two `(or …)` disjuncts with equal
bindings but different guards collide and the whole rule becomes
order-dependent. `_priority_for` reading `rule.priority` with the
1000 default.

### Task T1a.3.3.3 — The enqueue pass

Full pass (cold start / `is_stalled`) vs delta pass. Delta pass: any
never-matched plan gets one full match first (reflective rules), then
each delta fact seeds the plans that have its relation as a positive
premise, via `pos_index`. `_matched_plan_ids` as a bitset over the plan
list. `is_stalled`'s deliberate side effect (it runs a pass, advancing
the tiebreaker) is preserved.

### Task T1a.3.3.4 — `_apply`

Build every conclusion, look each up, split redundant vs productive.
Mark fired regardless. On wholly-redundant, record the alternative
justification when any conclusion still accepts one — with the O(1)
pre-check, and with `bindings` deliberately left empty on that path (it
is display metadata no consumer of an alternative reads, and it is the
hottest path in the engine). On productive, `fire` and accumulate the
delta.

### Task T1a.3.3.5 — The native mirror

`__symmetric__`-marked relations closed under arg-swap directly. The
cold seed, the LIFO `_mirror_queue`, self-loop and existing-mirror skips,
the alternative-justification record when the mirror already exists, and
`_has_pending_mirror` for `is_stalled`. Gated by
`enable_symmetric_mirror`.

**Blocked on** [S1a.0.1](../p1a.0_conformance_harness/s1a.0.1_parity_contract_and_corpus.md)'s
hazard H1 resolution: the Python cold seed iterates a `frozenset`, so its
order may not be stable. Port the *fixed* ein.py behaviour.

### Task T1a.3.3.6 — Contradiction detector

`ContradictionDetector.detect` / `has_contradiction` (direct ⊥ first,
then pairs in extent order — the order reaches the unsat core) and the
incremental `contradicts`, which becomes two bit tests
([design/06](../design/06_saturation.md) §6).

## Notes

- Start with the naive full-pass enqueue and get T2 green, *then* switch
  on the delta pass and re-diff. The delta path is where an ordering bug
  is easiest to introduce and hardest to attribute.
- The priority bands are advisory since S1.21.8 — but they still
  determine order, and order is the gate. Do not "simplify" them.
