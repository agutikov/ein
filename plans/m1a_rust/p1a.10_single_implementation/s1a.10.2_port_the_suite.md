# S1a.10.2 — Port the Python test suite

**Phase:** P1a.10 (One implementation)
**Estimate:** 5 days
**Depends on:** [S1a.10.1](s1a.10.1_bank_the_oracle.md)

## Context

**1 517 pytest tests and 21 acceptance tests**, against ~312 in the ein.rs
workspace. The ratio is not a coverage gap: a large share of the Python suite
tests *the Python implementation* — its dataclasses, its `__repr__`s, its
module layout, its `argparse` wiring — and has no referent once that code is
gone. Another share tests the *semantics*, and every one of those has to
survive in some form.

Sorting them is the stage. Porting a test whose subject no longer exists is
worse than deleting it: it manufactures a green check with nothing behind it.

## Acceptance

- Every Python test file has a disposition and the dispositions are written
  down per file, not per suite: **ported**, **already covered** (naming the
  Rust test), or **deleted with its subject** (naming the subject).
- The **acceptance gate** — `ein.py/acceptance/`, the three zebra2 task-class
  fixtures — is ported in full. It is the one suite that asserts the *answer*
  rather than the agreement, which is exactly what a single-implementation
  repo has least of. `ein-infer/tests/acceptance.rs` is where it goes; part of
  it is already there.
- No test is ported by translating a Python internal into a Rust internal that
  does not exist. Where the Rust design differs, the *behaviour* is what gets
  asserted.
- `cargo test --workspace` runtime stays inside the gate's current budget, or
  the slow tests are marked and the budget is restated.

## Tasks

### Task T1a.10.2.1 — Classify

By directory, because they cluster: `tests/inference/` is mostly semantics,
`tests/render/` and `tests/trace/` are mostly goldens (S1a.10.1 may already
have banked them), `tests/test_ir_ast.py` is the IR contract, and the files
that name `_helpers`, `monotonic/` internals or `python_impl` are
implementation tests. Produce the table before porting anything.

### Task T1a.10.2.2 — Port the semantics tests

The bulk. Prefer one Rust test per *behaviour*, not per Python test: the
Python suite grew over M1's phases and has duplication a port should not
inherit.

### Task T1a.10.2.3 — Port the acceptance gate

`--acceptance-only` is 21 tests and ~44 s under PyPy; under ein.rs the same
work is well under a second, so these stop being a separate phase and become
ordinary tests. Say so in the runner's replacement.

### Task T1a.10.2.4 — Delete with the subject

Follow the project's own rule: **removing a special case removes its tests** —
delete or re-point, never restore behaviour to keep a stale test green. Here
the "special case" is an entire implementation, so the rule applies at scale
and needs the same discipline: each deletion names what it tested.

### Task T1a.10.2.5 — The fixtures the Python suite owns

`ein.py/tests/golden/`, `ein.py/tests/fixtures/` and any `.ein` file living
under `ein.py/` rather than `examples/`. A fixture that is corpus-worthy moves
to `examples/` with a `corpus.toml` entry; one that is not, dies with its test.

## Notes

- Expect the ported suite to be **smaller** than 1 517 and to say why. A port
  that reports the same count has almost certainly translated implementation
  tests into implementation tests.
- Two dispositions deserve suspicion in review: "already covered" without a
  named test, and "deleted with its subject" for anything under
  `tests/inference/`.
