# S1a.3.2 — The register matcher

**Phase:** P1a.3 (Deductive core)
**Estimate:** 5 days
**Depends on:** [S1a.3.1](s1a.3.1_compiler.md)
**Implements:** `ein/inference/{match,firing,resolve,predicates}.py`,
[design/05](../design/05_matcher.md) §§3–5

## Context

The 46 %. Replace the recursive generator + per-bind dict copy with a
register file, a backtrack trail, explicit cursors, and `u32` equality.
The *result sequence* must be identical — same matches, same order, same
premise order, same binding order.

## Acceptance

- T2 firing-sequence parity on every saturation fixture (the matcher's
  signature is the firing order).
- `Provenance.bindings` order identical (it is the trail order, and it
  is printed in the trace).
- `premises` order identical, including through `run_seeded`'s
  position-preserving rebuild.
- Zero heap allocations in the inner loop.
- `criterion`: `match::run` over the saturated zebra2 root at least 30×
  faster than ein.py's equivalent work, measured against the same call
  counts.
- `MAX_REGS` overflow is a clean `CompileError` with a fixture.

## Tasks

### Task T1a.3.2.1 — The driver

`MatchCtx` (registers, trail, premises, cursors) and the step loop with
explicit backtracking. Unification per slot kind: `Const` → `u32`
compare; `Reg` → bind-if-unbound else compare; `Nested` → arg must be a
fact value, relation must match, arity must match, args unify pointwise.

### Task T1a.3.2.2 — Candidates

Use the compile-time `Probe` to select the participation-index bucket,
falling back to the full relation extent when no slot is known. Iterate
base-then-delta ([design/03](../design/03_data_model.md) §5) so order
matches the Python tuple's. **Keep the full re-check** of every slot
after narrowing — the narrowing is only behaviour-preserving because of
it.

### Task T1a.3.2.3 — Entry points

`run`, `run_guarded`, `run_seeded`, `run_seeded_guarded`, `run_steps` —
each with a callback rather than a materialised result. `run_guarded`
pairs every match with *its disjunct's* guards (the S1.21.8 D5 fix);
`run_seeded` seeds at **each** matching step index and rebuilds premises
at the seeded step's position.

### Task T1a.3.2.4 — Predicates

The `eq` / `neq` registry with `is_predicate` / `get` / `names` /
`register`. Guard args are **raw AST slots**, not compiled ones — they
resolve against the runtime environment including the activator seeds
([design/05](../design/05_matcher.md) §5). Getting this wrong makes a
guard inside an `(absent …)` silently resolve a parameter to nothing.

### Task T1a.3.2.5 — Firing

`build_fact` (walk a template, resolve registers, construct nested facts
recursively), `fire` (one `Prov` shared by all conclusions, premises in
step order, `absent_premises` threaded through), and the unbound-var
error with ein.py's message text.

### Task T1a.3.2.6 — Binding keys

`_binding_key` as `(rule, activator_args, [Value; n])` hashed with
FxHash, semantically equal to the Python `frozenset` (unbound registers
carry a sentinel distinct from every `Value`). `engine._fired` as an
`FxHashSet` of that key. Drop `_hashable` — its `list`/`dict` cases are
unreachable.

## Notes

- Write the T2 diff harness against a *single* fixture first
  (`examples/saturation/transitive/taxonomy.ein` is small and exercises
  a self-join), then widen. A first-diff on a 40 k-event log is much
  harder to read than a first-diff on 40.
- Join reordering, WCOJ, and dedup-in-the-matcher are all out of scope
  and stay out — [design/05](../design/05_matcher.md) §6 lists why.
