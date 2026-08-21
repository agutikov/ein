# S1a.9.1 — The PyO3 surface

**Phase:** P1a.9 (Bindings and release)
**Estimate:** 3 days
**Depends on:** [P1a.10](../p1a.10_single_implementation/README.md) — the
phase's, and there is one engine to bind
**Implements:** [`docs/api/ein.md`](../../../docs/api/ein.md)'s five
steps, in Rust

## Context

[M2](../../m2_nl_to_ir/README.md)'s NL frontend stays CPython (llama.cpp,
Python bindings), so it needs a way to call the engine. This is that way:
a `ein_rs` extension module mirroring the documented embedding contract
—  **parse** → `KnowledgeBase` → (optionally **saturate**) → **solve** →
read the verdict.

The binding is a *surface*, not a boundary. Q-M1a.1 settled that the port
is full (Boundary A); `ein_rs` exists so M2 can call the engine cheaply,
not so a Python harness can own the loop. Keep it thin: handles in,
results out, no callbacks into Python on a hot path.

## Acceptance

- The worked example in `docs/api/ein.md` runs against `ein_rs` and
  produces identical output to `ein`.
- The five entry symbols exist with the documented signatures:
  `parse(text, *, filename=None)`,
  `KnowledgeBase.from_file(path)` / `.from_ir(forms, *, base_dir=None)`,
  `Saturator(kb).saturate(*, max_steps=None)`,
  `solve(kb, *, stop_after=None, max_set_size=5, config=None, …)`,
  and the verdict readers.
- Exceptions map: `IRParseError`, `KBLoadError`, `CompileError`,
  `BudgetExceededError`, `SaturatorStepLimitError` — the names and message
  text `docs/api/` documents and the CLI already prints, with the
  inheritance that mattered in ein.py (`IRParseError` was a `SyntaxError`)
  kept because consumers were written against it.
- No GIL held during a solve; a `KeyboardInterrupt` interrupts one.
- Round-trip: `dump(parse(x))` from `ein_rs` equals the CLI's, and
  `dump → parse → dump` is a fixed point — the property
  `ein-ir/tests/fuzz_properties.rs` already asserts in-process, asserted
  again across the boundary.

## Tasks

### Task T1a.9.1.1 — Module scaffold

`ein_rs` via `maturin`, feature-gated (`python`) so the engine crates
build without PyO3. Submodules mirroring the doc pages:
`ein_rs.ir`, `ein_rs.kb`, `ein_rs.inference`, `ein_rs.trace`.

### Task T1a.9.1.2 — IR layer

`parse` returning an opaque `Forms` handle plus enough accessors for the
documented uses; `dump` / `dump_compact` / `dump_canonical`.

Decision to record: do **not** expose the full typed AST as Python
objects in v1. Reconstructing `SForm` / `Atom` / `KwPair` on the Python
side would be a second data model to keep in step with the Rust one for no
current consumer — M2 produces *text* and consumes *verdicts*. Add it when
something needs it.

### Task T1a.9.1.3 — KB layer

`KnowledgeBase` wrapping a handle: `from_file`, `from_ir`, `facts`
(paged / iterable), `relations`, `rules`, `query`, `config`, `fork`,
`snapshot`, `all_facts()` filters, `justifications(fact)`,
`derivation_dag`, `unsat_core`, `to_dot`. Facts cross the boundary as
their s-expression text plus a light record (relation, args, provenance
kind), not as a mirrored object graph.

### Task T1a.9.1.4 — Inference layer

`Saturator(kb).saturate()` yielding `Firing` records; `solve(...)`
returning `(verdict, stats)`; `SolverConfig` as a Python dataclass-like
object with the same field names and defaults; `goal_bindings`;
`explain`; the verdict types `Solution` / `Ambiguity` / `Contradiction` /
`Aborted` with the documented fields.

Release the GIL around `saturate` and `solve` (they are long and take no
Python callbacks), and check a cancellation flag at the same budget
checkpoints so `KeyboardInterrupt` works.

### Task T1a.9.1.5 — Trace layer

`linearize(verdict, …)` and `render_markdown(trace, …)`,
`render_solution_table`. These return strings, so they are cheap to bind
and immediately comparable byte-for-byte with what `ein solve --trace`
writes — and with the twelve S1a.6.11 goldens, which are the same bytes.

### Task T1a.9.1.6 — Errors and types

Map every engine error to a Python exception class with the name, message
and base class `docs/api/` records. Where the documented behaviour was a
bare `KeyError` or `TypeError` (Q-M1a.14), match the *type* and message.
The reference is the documentation plus `examples/broken/**/*.expected` —
the checked-in diagnostics, which are ein.py's own text — since the engine
that raised them is gone by the time this stage runs.

## Notes

- The module is `ein_rs`, not `ein`. The reason was side-by-side
  installation with the Python package; that package is gone, so the name
  is an open choice — see the phase README's note and
  [S1a.9.3](s1a.9.3_packaging.md).
- If a hot loop ever needs to cross the boundary per fact, that is a
  design smell: the right answer is a batch call or an `--events`
  subscription, not a faster FFI.
