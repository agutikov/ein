# S1a.0.3 — Shared stdlib and examples

**Phase:** P1a.0 (Conformance harness + shared assets)
**Estimate:** 2 days
**Depends on:** [S1a.0.1](s1a.0.1_parity_contract_and_corpus.md)
**Implements design:** [design/11](../design/11_shared_assets.md)

## Context

Both implementations must read the *same* `.ein` library. Today the
stdlib lives inside the Python package
(`ein.py/src/ein/stdlib/`, resolved as
`Path(ein.__file__).parent / "stdlib"`) because that is what makes a
wheel work. A Rust binary cannot resolve that, and a second copy would
make every parity result meaningless.

This stage moves the stdlib to repo-root `stdlib/`, gives both
implementations the same three-step resolution chain, and adds the drift
checks that keep the packaged and embedded copies honest.

## Acceptance

- `stdlib/` at repo root holds the seven modules + `README.md` +
  `MANIFEST.sha256`; `ein.py/src/ein/stdlib/` is gone from git and
  `.gitignore`d.
- ein.py resolves `$EIN_STDLIB` → repo checkout → packaged copy, and
  `stdlib_macro_names()` still reads `macro.ein` through that chain.
- `pip install ein.py/` still works and `ein solve examples/zebra2.ein`
  succeeds from a directory outside the repo.
- `EIN_STDLIB=/tmp/empty ein solve examples/zebra2.ein` fails with the
  existing "module not found" message (no hidden fallback).
- `stdlib-check` is in CI and fails on a corrupted copy.
- Editing `stdlib/algebra.ein` in a checkout changes behaviour with no
  rebuild or reinstall.

## Tasks

### Task T1a.0.3.1 — Move and manifest

`git mv ein.py/src/ein/stdlib/*.ein stdlib/` (+ `README.md`). Generate
`stdlib/MANIFEST.sha256`. Add a small `utils/stdlib_manifest.py` that
regenerates and verifies it.

### Task T1a.0.3.2 — Python resolution chain

Rewrite `imports._stdlib_root()` as the three-step chain. Detect the
checkout by walking up from the package location for a directory
containing `MANIFEST.sha256`. Keep the `functools.lru_cache` on
`stdlib_macro_names()` but make sure the cache cannot outlive an
`EIN_STDLIB` change within a process (tests set it) — key the cache on
the resolved root, or clear it in a fixture.

### Task T1a.0.3.3 — Packaging

Add a `build_py` hook that copies `stdlib/` into
`ein.py/src/ein/stdlib/` at build time; keep the existing
`[tool.setuptools.package-data]` entry so the copy ships. Verify with
`python -m build` + install into a fresh venv + run from `/tmp`.

### Task T1a.0.3.4 — Rust side

`ein-ir` gets a `StdlibSource` trait with two implementations: an
on-disk root and an `include_dir!`-embedded tree. Resolution order
matches Python's; release builds prefer the embedded copy unless
`EIN_STDLIB` is set, dev builds prefer the checkout. A build-time
`const` digest is compared against `MANIFEST.sha256` in a test.

### Task T1a.0.3.5 — Drift checks in CI

One job: manifest matches files; the built Python copy matches; the Rust
embedded digest matches. Plus the corpus-completeness check from
[S1a.0.1](s1a.0.1_parity_contract_and_corpus.md).

## Notes

- The stdlib README is user-facing documentation for `std.*` modules; it
  moves with them, and
  [`docs/kernel/ir/03-ein-lang/07_stdlib_api.md`](../../../docs/kernel/ir/03-ein-lang/07_stdlib_api.md)
  gets its path reference updated.
- `examples/` does **not** move. Both suites already reach it from the
  repo root.
- Goldens stay put for now — Q-M1a.9.
