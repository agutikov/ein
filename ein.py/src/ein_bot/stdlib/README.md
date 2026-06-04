# ein-bot standard library (`std.*`)

The canonical, **package-shipped** standard library. This directory *is* the
stdlib root: the import resolver
([`ein_bot/kb/imports.py`](../kb/imports.py)) maps a logical module name
`std.<path>` to `ein_bot/stdlib/<path>.ein` (P1.8 S1.8.A1 §D4 / S1.8.A3).
It ships with the package via `pyproject.toml` `package-data`
(`ein_bot = ["stdlib/*.ein", "stdlib/**/*.ein"]`), so `(import std.…)`
resolves whether ein-bot is run from a checkout or an install.

## Location decision (S1.8.A4 — closes [Q30](../../../../plans/m1_core_graph_reasoning/open_questions.md#q30--universal-rule-library--import-mechanism))

**Q30 → (c) hybrid.** Puzzle-*agnostic* vocabulary (the pattern macros today;
the relation-algebra / type rule families as they land) lives here as
importable modules; puzzle-*specific* content (a puzzle's activator facts,
its bespoke spatial/typecheck rules) stays inline in the puzzle file. A
puzzle pulls the library in with one `(import …)` and declares only its own
facts.

Why the package (not `examples/`): `examples/` is user content and is not
installed, so an install-relative import couldn't find it; the package dir is
always present and version-locked to the engine.

## Modules

| module | file | provides | stage |
|--------|------|----------|-------|
| `std.macro` | [`macro.ein`](macro.ein) | the `forall` / `open` pattern macros | S1.5.9 |
| `std.elim` | [`elim.ein`](elim.ein) | closed-world `typecheck-arg-{0,1}` + `domain-elimination` + `no-room-left` (generic; the instance-type relation is the `?isa` param, not a hardcoded `is-a` — S1.8.A10; needs `forall`) | S1.8.A8 |
| `std.closure` | [`closure.ein`](closure.ein) | `infer-closure` — `functional ∧ total ⇒ (closed R)` (parameter-less; **opt-in, not for branching puzzles** — see the file's caveat) | S1.8.A6 |
| `std.algebra` | [`algebra.ein`](algebra.ein) | the full relation-algebra signature: relative (`converse` / `compose` / `identity`), Boolean (`meet` / `join` / `difference` / `complement` / `top` / `empty`), property checks (`irreflexive` / `antisymmetric` / `asymmetric` / `connex` / `difunctional`), `imply1` / `imply2-fwd` / `imply2-reverse`, the equational lemmas (`symmetric`⟺`converse R R`, Schröder `compose-negative-{r,s}`, contravariance, converse-over-join) + `converse-illtyped-{dom,ran}` signature typecheck (generic; lemmas use reflective rule-implication) | S1.8.A7 + A12 |
| `std.typing` | [`typing.ein`](typing.ein) | `(type-hierarchy ?isR*)` one-knob converse-typecheck driver + `(reflexive R)` closure (non-generic fan-out; pairs with `std.algebra`'s `converse-illtyped-*`) | S1.8.A10 |

`std.algebra`'s ops split **intrinsic** (read existing edges: `compose` / `meet`
/ `difference` / `converse` / `join` / `difunctional`) vs **extensive** (range
over the `Dom×Ran` universe to reach absent pairs: `complement` / `top` /
`identity` / `connex`). The extensive ops take the puzzle's instance-type
relation + argument types `(?isa Dom Ran)` (the A10 universe) as parameters and
inherit the closed-world soundness caveat — sound only when the operand is
saturation-determined (the `std.closure` caveat), so not for branching puzzles.

*Planned (not yet shipped):* the **division residuals** (`R\S` / `R/S` / `syq` —
an allegory extension beyond Tarski's RA, S1.8.A12 T5) need a `forall` over the
universe and have no M1 consumer; design is in the stage doc. When the universal
kernel rules (`symmetric` / `transitive` / …) themselves move out of inline
`zebra2.ein` into a stdlib module, that's the pending tail of S1.8.A5 (`compose`
is the keystone the `transitive` promotion rides on).

**Rule modules vs `forall`.** `std.elim`'s rules are *generic* (parametrised
over a relation), so a puzzle imports them **flat** (`:symbols`) to keep the
bare names their activator facts reference — and, because selective import is
not auto-closure (A1 D7), must also import the rules' `forall` dependency:

```lisp
(import std.macro :symbols (forall))
(import std.elim  :symbols (typecheck-arg-0 typecheck-arg-1
                            domain-elimination no-room-left))
```

## Importing

Three tiers (Python-style — see the [A1 decision record](../../../../plans/m1_core_graph_reasoning/p1.8_ein_lang_modules/s1.8.a1_module_system_design.md#decision-record-2026-06-04)):

```lisp
(import std.macro)                        ; → (std.macro.forall …)   fully qualified
(import std.macro :as m)                  ; → (m.forall …)           aliased
(import std.macro :symbols (forall open)) ; → (forall …)             flat-selective
```

`:as` and `:symbols` are mutually exclusive. Names are logical and dotted
(`.` is a normal atom character); the `.ein` suffix is implied. A
file-relative import (a non-`std` name) resolves against the importing file's
directory.

To inline a puzzle's imports into a single standalone file (resolving + 
tree-shaking unused library symbols):

```sh
ein-bot ir parse --resolve path/to/puzzle.ein
```

## One layout-detail per concern

One file per coherent concern (`macro.ein`, future `algebra.ein`,
`types.ein`) rather than one monolith — `:symbols` selective imports and the
tree-shaking dump both reward small, focused modules. A `README.md` here is
ignored by the resolver (only `*.ein` files are modules).

Each shipped stdlib symbol is exercised by a test (e.g. `forall` / `open` by
`tests/inference/test_forall.py` / `test_open.py` and
`tests/kb/test_imports.py`). The full per-symbol API reference is deferred to
[S1.20.C](../../../../plans/m1_core_graph_reasoning/p1.20_kernel_docs/s1.20.c_stdlib_api_reference.md).
