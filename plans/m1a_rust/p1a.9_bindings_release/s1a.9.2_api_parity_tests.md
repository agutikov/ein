# S1a.9.2 — API parity tests

> **Amended 2026-08-21.** "Parity" here was *the module against ein.py*, and
> the stage was written as one test body parameterised over `ein` and
> `ein_rs`. [P1a.10](../p1a.10_single_implementation/README.md) removes the
> second module, and this phase now runs after it. The word keeps its shape
> and changes its operand: parity is **the module against the CLI**, two
> surfaces of one engine, plus the module against the contract `docs/api/`
> writes down. That is the same move
> [S1a.10.3](../p1a.10_single_implementation/s1a.10.3_corpus_without_an_oracle.md)
> made on the corpus manifest's `runs` column — *exercised under* rather than
> *compared under* — and for the same reason: the inventory was worth more
> than the differ.

**Phase:** P1a.9 (Bindings and release)
**Estimate:** 2 days
**Depends on:** [S1a.9.1](s1a.9.1_pyo3_surface.md)
**Implements:** the embedding half of
[design/01](../design/01_parity_contract.md)

## Context

The CLI is one surface of the engine and the PyO3 module is another, and
the second can drift on its own: a wrong default, a missing keyword, an
exception that is a `ValueError` where the documentation says `SyntaxError`.
None of that shows up in anything the workspace runs today — `corpus_cli.rs`
sweeps processes, and every library test calls Rust.

Two operands are available and neither is a second engine:

1. **the CLI** — the same five steps, the same fixtures, the same answers.
   `ein solve x.ein --json-summary` and `ein_rs`'s verdict object carry the
   same fields, so the comparison is mechanical.
2. **the documentation** — `docs/api/{ein,ir,kb,inference,trace}.md` *is* a
   contract, and `docs/api/ein.md` carries a worked example with concrete
   numbers in it. A test that executes the example turns "verified against
   commit X" from a promise into a check.

## Acceptance

- Every symbol in the five API doc pages is exercised at least once, and a
  coverage check fails when a documented symbol has no test. **This is the
  criterion that carries the stage** now that there is no second module: a
  documented surface with no test is what the phase exists to prevent.
- The module and the CLI agree on verdict, `k`, `exhausted`, goal bindings,
  the model as a fact set and every counter, over the corpus's `positive`
  and `stdlib` groups.
- Exception type, message and base class match what `docs/api/` documents
  and what `examples/broken/**/*.expected` records, for every documented
  error path.
- Keyword defaults are checked **against the documentation** rather than
  against a second module — `inspect.signature` versus the table in
  `inference.md`, which makes the doc the thing that has to be right.
- The `docs/api/ein.md` worked example is executed by the suite, so the
  documentation cannot silently go stale.

## Tasks

### Task T1a.9.2.1 — The harness

A pytest suite over `ein_rs` alone, asserting on results and never on
internals. Where a documented shape is deliberately not offered (e.g.
`ein_rs` returns opaque handles instead of typed AST nodes —
[S1a.9.1](s1a.9.1_pyo3_surface.md) T1a.9.1.2), the *documentation* says so
and the coverage check reads it there; a skip list that only the test knows
about is how a surface quietly shrinks.

**No `--impl`-shaped fixture.** A parameterisation with one value invites a
reader to look for the operand that is gone
([S1a.10.4](../p1a.10_single_implementation/s1a.10.4_utils.md)'s note, and
its lesson from `utils/`).

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

`inspect.signature` on `ein_rs`, compared against the keyword tables in
`docs/api/inference.md` and `kb.md`, so a renamed or reordered parameter
fails loudly — and so does a documentation table that drifted from the
module. This catches the class of drift that behaviour tests miss, and it
is *stronger* than the two-module comparison it replaces: two modules could
agree on a default the documentation never claimed.

### Task T1a.9.2.5 — Doc-example execution

Extract the code blocks from `docs/api/*.md` and run them, asserting the
outputs the prose claims. The `ein.md` example states concrete numbers
(`kb.facts` counts, rule/relation counts) — those become assertions, which
is how the doc's "verified against commit X" line stops being a promise and
becomes a test.

### Task T1a.9.2.6 — The CLI cross-check

The second operand. For each corpus entry in the `positive` and `stdlib`
groups: run `ein solve <file> --json-summary out.json`, drive the same file
through `ein_rs`, and compare the fields listed in the acceptance. The
corpus manifest is already the list
([`ein_corpus::manifest`](../../../ein.rs/crates/ein-corpus/src/manifest.rs)),
and `corpus_cli.rs` is the model for how to sweep it.

### Task T1a.9.2.7 — CI wiring

Add the suite to the per-commit tier once `ein_rs` builds in CI.

## Notes

- **Where the tests live is now a real question.** They were to sit in
  `ein.py/tests/api_parity/`; that tree is gone. They are Python-level
  assertions about a Python surface and must be runnable by someone who has
  only the wheels, so they want a directory of their own —
  `bindings/tests/` beside the PyO3 crate is the obvious candidate, and
  [S1a.9.3](s1a.9.3_packaging.md) has to place it anyway.
  **This is the one place in the repo where `pytest` comes back**, and it
  comes back for a Python surface rather than for a Python engine.
- If a divergence between the module and the CLI is found and *accepted*, it
  goes in [`divergences.md`](../divergences.md) like any other — the API
  surface is under the same contract as stdout. Note what that file becomes
  here: D1–D3 record where two *implementations* differed, and a new entry
  would record where two *surfaces of one implementation* do.
