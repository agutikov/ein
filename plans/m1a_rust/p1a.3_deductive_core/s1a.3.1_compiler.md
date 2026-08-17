# S1a.3.1 — The pattern compiler

**Phase:** P1a.3 (Deductive core)
**Estimate:** 4 days
**Depends on:** [P1a.2](../p1a.2_kb_core/README.md)
**Implements:** `ein/inference/compile.py`,
[design/05](../design/05_matcher.md) §2

## Context

Lower a `(rule, activator)` pair to a plan: `Scan` / `Join` / `Guard`
steps, per-disjunct `NafGuard`s lifted out by `split_naf`, and
`assert_templates`. The compiler is small but its *edge cases are
semantics* — S1.22.0 turned four silent `return []` paths into
`CompileError`s precisely because dropping a premise is unsound in one
direction and incomplete in the other.

Two additions here are pure metadata: register numbering and the
candidate `Probe` ([design/05](../design/05_matcher.md) §2).

## Acceptance

- Plan-shape parity: for every `(rule, activator)` in the corpus, the
  step sequence, relation names, slot kinds, disjunct split, guard
  scopes, `watched` sets, `monotone` flags and assert templates match
  ein.py's — compared through the `compile` event.
- Every `CompileError` message byte-identical, with fixtures for all
  four cases: unbound relation head, empty `(absent …)` sub-plan, nested
  `(or …)`, activator arity mismatch.
- `asserted_relation` / `negated_relation` / `naf_relation_refs` agree
  (they feed `closed.producible_relations` and `naf_deps`).
- Compiling all 19 zebra2 plans under 100 µs total.

## Tasks

### Task T1a.3.1.1 — Slot lowering

`_slot`: bound param `Var` → `Const`; free `Var` → `Reg`; `Atom`/`Int` →
`Const`; `SForm` → `Nested` (with a head var substituted from the
activator binding when bound). Keep the "unrecognised shape returns the
node as-is" safety net's *effect* — an opaque slot compared by equality.

### Task T1a.3.1.2 — Premise compilation

`and` flattening; `(absent P)` → `AbsentGuard` (with the empty-sub-plan
error); predicate dispatch by head name; everything else a relation
pattern, including `(not P)` as an ordinary relation `not` with a nested
arg. Top-level `(or …)` split into disjuncts by `compile_rule`, nested
`(or …)` an error. `KwPair` premises silently dropped (Q32).

**Do not** special-case `forall` / `open`: they are macros, already
expanded ([S1a.1.3](../p1a.1_ir_frontend/s1a.1.3_macros_and_imports.md)).

### Task T1a.3.1.3 — `split_naf`

Lift top-level `AbsentGuard`s per disjunct, recording each guard's
`scope` (variables bound by *preceding* positive premises, **seeded with
the rule's parameters** — the reason is in the Python docstring and it is
load-bearing for guards containing predicates), `watched` (every relation
the sub-plan reads, through nested guards), and `monotone` (no nested
`AbsentGuard`). Nested guards are *not* lifted.

### Task T1a.3.1.4 — Assert lowering

Top-level `(and …)` → one template per conjunct (A13 multi-assert);
`(not (R …))` → a `Nested("not", [Nested("R", …)])`. `assert_template`
(the first) stays available for the single-assert readers.

### Task T1a.3.1.5 — Register numbering and probes

Number the distinct free vars per disjunct; record the name for
provenance rendering. Compute the `Probe` for each `Scan`/`Join` — the
first slot whose value is known at that point, replicating
`_candidates`' left-to-right scan and its two skip rules (a `Reg` bound
to a nested fact is not keyed; a `Nested` slot is not keyed). Add a
debug assertion that the runtime probe choice equals what a live
`_candidates`-style scan would pick.

### Task T1a.3.1.6 — Plan memo

The process-wide `(rule, activator) → PlanId` memo
([design/06](../design/06_saturation.md) § Win A), plus the per-engine
ordered `Vec<PlanId>` that reproduces ein.py's `cache` iteration order.
The memo is append-only and will need `Sync` in
[P1a.7](../p1a.7_parallelism/README.md) — build it behind that seam now.

## Notes

- `_activators_for` filters activators by **arity match** (S1.22.0) — a
  rule name and a property relation may coincide, and a fact that cannot
  bind the parameters does not authorise anything. Port the filter, not
  just the compile-time error.
- The cache key stringifies *all* activator args while
  `plan.activator_args` keeps only the string ones. That asymmetry is
  Q-M1a.8; reproduce it exactly and do not "fix" it here.
