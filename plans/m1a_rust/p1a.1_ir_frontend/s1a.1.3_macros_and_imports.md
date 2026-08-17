# S1a.1.3 — Macro expansion and import resolution

**Phase:** P1a.1 (IR frontend)
**Estimate:** 3 days
**Depends on:** [S1a.1.2](s1a.1.2_ast_and_dumper.md),
[S1a.0.3](../p1a.0_conformance_harness/s1a.0.3_shared_stdlib_and_examples.md)
**Implements design:** [design/04](../design/04_ir_frontend.md) §§5–6

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
  behind the `StdlibSource`/`FileSource` seam so
  [`--sandbox`](../design/09_server_mode.md) §7 is a one-line change
  later.
