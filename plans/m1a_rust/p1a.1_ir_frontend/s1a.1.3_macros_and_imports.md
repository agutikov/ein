# S1a.1.3 — Macro expansion and import resolution

**Phase:** P1a.1 (IR frontend)
**Estimate:** 3 days
**Depends on:** [S1a.1.2](s1a.1.2_ast_and_dumper.md),
[S1a.0.3](../p1a.0_conformance_harness/s1a.0.3_shared_stdlib_and_examples.md)
**Implements design:** [design/04](../design/04_ir_frontend.md) §§5–6

> **Instruments (M1a [S1a.10.6](../p1a.10_single_implementation/s1a.10.6_docs.md)).** This document names `ir_oracle.py`. It is gone — deleted with the second engine at S1a.10.3–S1a.10.5 — so the numbers here are a **record**, not something you can re-run. What answers each one's question now is the census in [`utils/README.md`](../../../utils/README.md#the-census).

## Context

`(import …)` is resolved at the **form** level before loading
(flatten-then-load, A1 D8), and `(macro …)` is a load-time AST rewrite —
which is how `forall` / `open` exist as ein source (`stdlib/macro.ein`)
rather than as compiler arms. Both are pure AST→AST transforms, so they
belong to the frontend phase and can be gated on `dump_canonical`
equality.

The tree-shaking pass `resolve_and_minimize` matters more than it looks:
the surviving rule set is observable through `len(engine.cache)` and
through firing order, so dropping one declaration too many or too few is
a T1/T2 failure much later.

## Acceptance

- For every corpus file, `dump_canonical(resolve_imports(parse(f)))` is
  byte-identical between implementations.
- Same for `resolve_and_minimize`, including its activation-closure walk.
- Every import/macro error message byte-identical:
  module-not-found, cycle, bare `std`, `:as` with `:symbols`, empty
  `:symbols`, unknown macro, macro arity mismatch, and the S1.8a.f20
  unimported-`std.macro` rule check.
- `stdlib_macro_names()`'s Rust equivalent reads `stdlib/macro.ein`
  through the resolution chain, not a hardcoded list.

## Tasks

### Task T1a.1.3.1 — Import spec parsing and resolution

`(import MODULE [:as A | :symbols (S+)])`. Logical module names:
`std.x.y` → `<stdlib>/x/y.ein`; anything else → `<base_dir>/x/y.ein`
(dotted → path, `.ein` implied). Recursive, bottom-up, re-qualified
under the outer namespace (D6), cycle-checked.

### Task T1a.1.3.2 — Qualification and selection

Whole-module prefixing, `:as` aliasing, `:symbols` flat selection.
`_defined_names` / `_rename_atoms` / `_qualify` / `_select` /
`_dedup_declarations` port with the same semantics, including the
deliberate strictness that a flat name pulled through two paths is a
duplicate-name error.

### Task T1a.1.3.3 — Minimisation

`resolve_and_minimize`: drop imported declarations nothing references,
following the activation closure over match heads. Diff the surviving
declaration list against ein.py's for every corpus file — this is the
task most likely to be subtly wrong and the one whose failure surfaces
latest.

### Task T1a.1.3.4 — Macro expander

Substitute `(NAME a…)` invocations with the macro body under the
parameter binding, over the arena. Iterate to a fixpoint with a depth
cap; reproduce `MacroError` conditions and messages.

### Task T1a.1.3.5 — Unexpanded-macro guard

The S1.8a.f20 check: a rule whose `:match` names a `std.macro` macro that
was not imported is a load error (unexpanded, it would silently never
fire). Read the macro names from `stdlib/macro.ein`; keep the check
deliberately narrow — an *absent optional marker relation* in a match
head is not an error.

## Notes

- Expansion happens before compilation, so by the time the compiler runs
  there are no `(forall …)` / `(open …)` heads left — they are already
  `(absent (and G (absent B)))` / `(and (absent P) (absent (not P)))`.
  Any Rust code that special-cases them is a bug.
- Import resolution is the engine's only filesystem access; keep it
  behind the `StdlibSource`/`FileSource` seam so any later policy on what
  the engine may read is a one-line change rather than an audit.

---

## Outcome — 2026-08-18

`ein-ir` gains `imports.rs` (`Resolver`, 620 lines) and `macros.rs` (330).

| acceptance item | result |
|---|---|
| `dump_canonical(resolve_imports(parse(f)))` byte-identical for every corpus file | 91 files, 0 differences |
| … same for `resolve_and_minimize`, activation closure included | 91 files, 0 differences |
| every import/macro error message byte-identical | the eleven `examples/broken/load/import_*.expected` fixtures, compared against the **files**, plus `macro_arity_mismatch` |
| `stdlib_macro_names()` reads `stdlib/macro.ein` through the resolution chain | `Resolver::stdlib_macro_names()`, checked against the oracle |
| *(phase)* `zebra2.ein` parse + resolve under 2 ms | **824 µs** for parse + resolve + expand, against 618.9 ms CPython / 193.7 ms PyPy — 751× / 235× |

Macro expansion gained a third gate the stage did not ask for: an `expand` op
in `utils/ir_oracle.py` and `the_corpus_expands_identically`, so the expander
is compared on real input rather than only on its error messages.

### What is here and what waits for the loader

The stage doc lists the S1.8a.f20 unimported-macro guard (T1a.1.3.5) and the
macro registry's duplicate / reserved-name checks. Those are **loader**
checks: `_ingest_macros` and `_validate_unexpanded_macros` live in
`kb/from_ir.py` and read `kb.macros` and compiled rules. What this stage owes
them is here —

- `Resolver::stdlib_macro_names()`, read from the module rather than
  hardcoded, so the guard consults the library that is actually loaded;
- `macros::unimported_macro_errors()`, the guard's message and its narrowness
  (a *match head* naming an unimported `std.macro` macro; an absent optional
  marker relation such as `functional` or `hypothesis` is **not** an error),
  as a pure IR function;
- `collect_macros()` with first-wins semantics, which is what `_ingest_macros`
  leaves behind after it rejects a duplicate.

[P1a.2](../p1a.2_kb_core/README.md) wires all three into `load()` and adds the
two rejects. The same split applies to the macro arity message: the
`MacroError` text is pinned here, and the loader's `({head} {name}): ` prefix
is asserted by composition in `the_import_and_macro_failures_are_byte_identical`
so the remaining half is a wiring job rather than a discovery.

### `pathlib` semantics leak into an error message

`_resolve_module_path` builds `root.joinpath(*rel).with_suffix(".ein")` and
the failure message prints that path, so the port has to mangle module names
the way `pathlib` does: empty segments dropped (`std..nope` is `std/nope`),
the final component's extension *replaced* rather than appended, and a
trailing separator on `$EIN_STDLIB` normalised away. Rust's `PathBuf::push` +
`set_extension` agree on all of it; `module_paths_are_mangled_the_way_pathlib_mangles_them`
is the test that says so rather than the reading that hoped so.

### The order-sensitivity that is not one

`resolve_and_minimize` computes a fixpoint over two coupled relations, and
both the work list and the `live` set are Python `set`s — normally a
[design/02](../design/02_determinism_and_order.md) §9 hazard. It is not one
here: the loop runs to a fixpoint, so the *result* is a closure and therefore
order-independent, and the surviving forms are emitted in stream order. The
Rust uses `BTreeSet` anyway, which costs nothing at this size and removes the
question.

### Not ported: `functools.lru_cache`

`stdlib_macro_names()` is cached in ein.py, keyed on the resolved root
(S1a.0.3 fixed an argument-less cache that answered the first root forever).
`Resolver` holds its `stdlib::Source` for the life of the resolver instead, so
the cache has no work to do; if profiling ever says otherwise it is a field on
`Resolver`, not a global.
