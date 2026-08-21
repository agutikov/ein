# S1a.0.3 — Shared stdlib and examples

**Phase:** P1a.0 (Conformance harness + shared assets)
**Estimate:** 2 days
**Depends on:** [S1a.0.1](s1a.0.1_parity_contract_and_corpus.md)
**Implements design:** [design/11](../design/11_shared_assets.md)

> **Instruments (M1a [S1a.10.6](../p1a.10_single_implementation/s1a.10.6_docs.md)).** This document names `ein-conformance`. It is gone — deleted with the second engine at S1a.10.3–S1a.10.5 — so the numbers here are a **record**, not something you can re-run. What answers each one's question now is the census in [`utils/README.md`](../../../utils/README.md#the-census).

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

---

## Outcome — 2026-08-17

The stdlib is at repo-root `stdlib/` — seven modules, `README.md` and
`MANIFEST.sha256` — and `ein.py/src/ein/stdlib/` is gone from git and
`.gitignore`d. Both implementations resolve it the same three ways
(`$EIN_STDLIB` → the checkout → the packaged/embedded copy), and both halves
are pinned: `tests/kb/test_stdlib_resolution.py` and `ein-ir`'s
`stdlib::tests`.

Every acceptance item checked by running it, not by reading:

| claim | how |
|---|---|
| a wheel still works from outside the repo | built one, installed it into a fresh venv, ran `ein solve <abs path>/zebra2.ein` from `/tmp` |
| `EIN_STDLIB=<empty> ` fails with "module not found" | `(import std.algebra) — module not found at /tmp/empty-stdlib/algebra.ein` — no hidden fallback |
| editing a module needs no rebuild | added a macro to `stdlib/macro.ein`, saw it in `stdlib_macro_names()` immediately |
| the drift check fails on a corrupted copy | appended a line to `stdlib/typing.ein`; `cargo test -p ein-ir` failed with `typing.ein: embedded copy differs from the manifest` |
| the corpus still agrees | `ein-conformance run … --tier T3`: **438 cells, 0 differences** after the move |

The harness now sets `EIN_STDLIB` for both sides itself. Each engine would
find the directory unaided, but "each resolved to the same place" is a weaker
claim than "both were told the same place" — and the stdlib is the one input
whose divergence no parity tier can diagnose, because both engines would be
*correct* about different programs.

### The bug this arrangement grows

The build copy is verbatim, manifest included, and the checkout walk starts
inside the package — so its **first** candidate is the build product. A stale
one therefore outranked the checkout it was built from, which is precisely the
failure the single-source rule exists to prevent. Found by building a wheel in
a checkout and watching `_stdlib_root()` move from `<repo>/stdlib` to
`<repo>/ein.py/src/ein/stdlib`. Fixed by skipping the packaged path in the
walk, and pinned by `test_a_packaged_copy_does_not_shadow_the_checkout`.

A second, smaller one: `stdlib_macro_names()`'s `lru_cache` was argument-less,
so a process that changed `$EIN_STDLIB` — which every test in the new module
does — would answer from the first root forever, and the S1.8a.f20
unimported-macro check would silently consult the wrong library. Keyed on the
resolved root now.

### Correction — 2026-08-18: the hook was never wired

`[tool.setuptools.cmdclass] build_py = "_build.build_py"` does not resolve.
setuptools looks a cmdclass module up under `package-dir`, which
`[tool.setuptools.packages.find] where = ["src"]` fills with `{"": "src"}`
*before* the field is expanded — so `_build.py` at the project root was
searched for at `src/_build.py`, found nowhere, and **every** install of
`ein.py` failed with `ModuleNotFoundError: _build` while pip was still
gathering build requirements. Not a regression: checked against setuptools
70.0.0, 75.8.2, 78.1.1, 80.9.0 and 84.0.0, all five identical. It surfaced
only when [S1a.0.4](s1a.0.4_workspace_skeleton_and_ci.md)'s CI ran
`pip install -e 'ein.py[dev]'` on a runner with no `ein` already present.

So the wheel this section reports building was not built by the hook, and the
packaged copy it watched shadow the checkout came from somewhere else — the
copy the `git mv` left on disk, most likely. The *finding* stands and its fix
is still pinned by `test_a_packaged_copy_does_not_shadow_the_checkout`; what
did not stand is "the packaging is verified".

The mechanism is now an in-tree PEP 517 backend — `build-backend = "_build"`
with `backend-path = ["."]`, delegating every hook to `setuptools.build_meta`
and copying the stdlib in `build_wheel` / `build_sdist` / `build_editable`.
`backend-path` puts the module on the build's import path explicitly, so
nothing depends on where the package tree is rooted. Two things the change
brings with it, both checked by running them: `MANIFEST.in` has to ship
`_build.py` (an sdist without its own backend stops at
`BackendUnavailable`), and the copy now appears in a checkout after an
editable install — which broke `tests/test_corpus_manifest.py`, whose
`_stdlib_dir()` still pointed at the package and had been passing on an empty
glob. Details in [11 — Shared assets](../design/11_shared_assets.md)
§ Packaging.

### Not moved

`examples/` stays where it is; both suites already reach it from the repo
root. Goldens stay in `ein.py/tests/golden/` — Q-M1a.9, decided at the P1a.5
gate when ein.rs starts producing them too.
