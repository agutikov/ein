# State model — Low

## EqClasses auto-vivifies on read: a read-shaped query mutates state that fork() copies

**Severity:** Low
**Confidence:** High
**Topic:** State model
**Classification:** design ambiguity (deliberate parity today, a determinism tripwire for the planned consumer)

**Locations**
- `ein.rs/crates/ein-core/src/kb.rs:415-481` (test at `:1909-1923` documents the behavior)

### Finding

Merely *asking* `kb.classes().equivalent(a, c)` inserts `c` into the parent map, so `classes()` output order depends on query history. Faithful to ein.py, and inert today (nothing fires equality propagation — O4 is a stub) — but a query-mutates-state API is exactly the shape the determinism rules exist to keep away from observables, and the first real consumer (the F4 e-graph seam) will inherit it silently.

### Recommendation

Either make find non-vivifying (a lookup, not a union-find insert) before wiring a consumer, or leave a loud comment at the future wiring point.
