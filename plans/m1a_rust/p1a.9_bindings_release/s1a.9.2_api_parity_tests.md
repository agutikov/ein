# S1a.9.2 — API parity tests

**Phase:** P1a.9 (Bindings and release)
**Estimate:** 2 days
**Depends on:** [S1a.9.1](s1a.9.1_pyo3_surface.md)
**Implements:** the embedding half of
[design/01](../design/01_parity_contract.md)

## Context

T0–T3 compare two *processes*. The PyO3 module is a third surface — an
in-process API — and it can drift independently: a wrong default, a
missing keyword, an exception that is a `ValueError` on one side and a
`RuntimeError` on the other. None of that shows up in a CLI diff.

The fix is cheap because the documentation is already a contract:
`docs/api/{ein,ir,kb,inference,trace}.md` describe the surface, and
`docs/api/ein.md` carries a worked example that was verified end-to-end
against `examples/zebra2.ein`. Parameterise a test body over the two
modules and run it against both.

## Acceptance

- One test suite, parameterised over `ein` and `ein_rs`, green for both.
- Every symbol in the five API doc pages is exercised at least once, and
  a coverage check fails when a documented symbol has no test.
- Exception type, message and base class identical for every documented
  error path.
- Default values identical for every keyword argument (checked
  reflectively, not by hand).
- The `docs/api/ein.md` worked example is executed by the suite, so the
  documentation cannot silently go stale.

## Tasks

### Task T1a.9.2.1 — The parameterised harness

A pytest fixture yielding `(module, name)` for `ein` and `ein_rs`; every
test takes it and asserts on results, never on internals. Where the two
genuinely differ (e.g. `ein_rs` returns opaque handles instead of typed
AST nodes — [S1a.9.1](s1a.9.1_pyo3_surface.md) T1a.9.1.2), the test
skips *explicitly* with the reason, and the skip list is short and
reviewed.

### Task T1a.9.2.2 — The five steps

Parse → load → saturate → solve → read, on a small fixture and on
`zebra2.ein`. Assert on the verdict, `k`, `exhausted`, the goal bindings,
the model as a fact set, and the counters — i.e. T0 + T1 through the
API rather than through stdout.

### Task T1a.9.2.3 — Per-module pages

One test group per doc page: `ir.md` (parse / dump / round-trip /
`IRParseError`), `kb.md` (registries, fact views, provenance,
`justifications`, `unsat_core`, `derivation_dag`, `KBLoadError`),
`inference.md` (`SolverConfig` fields + defaults, budgets, `Aborted`,
`goal_bindings`, `explain`), `trace.md` (`linearize`,
`render_markdown`, `render_solution_table`).

### Task T1a.9.2.4 — Signature and default checks

Reflectively compare keyword names and defaults between the two modules
(`inspect.signature` on both), so a renamed or reordered parameter fails
loudly. This catches the class of drift that behaviour tests miss.

### Task T1a.9.2.5 — Doc-example execution

Extract the code blocks from `docs/api/*.md` and run them against both
modules, asserting the outputs the prose claims. The `ein.md` example
states concrete numbers (`kb.facts` counts, rule/relation counts) —
those become assertions, which is how the doc's "verified against commit
X" line stops being a promise and becomes a test.

### Task T1a.9.2.6 — CI wiring

Add the suite to the per-commit tier once `ein_rs` builds in CI, with the
`ein`-only half running from P1a.0 onward (it is a useful check on the
oracle by itself).

## Notes

- Keep the tests in `ein.py/tests/api_parity/` rather than in a Rust
  crate: they are Python-level assertions about a Python surface, and
  they must be runnable by someone who has only the wheels.
- If a divergence is found and *accepted*, it goes in
  [`divergences.md`](../divergences.md) like any other — the API surface
  is under the same contract as stdout.
