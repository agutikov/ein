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
| `std.algebra` | [`algebra.ein`](algebra.ein) | the full relation-algebra signature: relative (`converse` / `compose` / `identity`), Boolean (`meet` / `join` / `difference` / `complement` / `top` / `empty`), cardinality checks (`functional` / `injective` / `total` / `surjective` + the `bijective` fan-out — S1.8a.f20; `total`/`surjective` need `forall`), property checks (`irreflexive` / `antisymmetric` / `asymmetric` / `connex` / `difunctional`), property **closures** (`symmetric` / `transitive` / `includes` — the universal kernel rules, S1.8.A5), `imply1` / `imply2-fwd` / `imply2-reverse`, the equational lemmas (`symmetric`⟺`converse R R`, Schröder `compose-negative-{r,s}`, contravariance, converse-over-join) + `converse-illtyped-{dom,ran}` signature typecheck (generic; lemmas use reflective rule-implication) | S1.8.A7 + A12 + A5 + f20 |
| `std.typing` | [`typing.ein`](typing.ein) | `(type-hierarchy ?isR*)` one-knob converse-typecheck driver + `(reflexive R)` closure (non-generic fan-out; pairs with `std.algebra`'s `converse-illtyped-*`) | S1.8.A10 |
| `std.bijection` | [`bijection.ein`](bijection.ein) | closed-world bijection inference, **signature-driven** (types read from `(relation R A B)`) and is-a-free: `bijective-setup` / `typecheck-setup` glue fan a `(bijective R)` + two hierarchy knobs into `domain-elimination` / `range-elimination` (survivor forcing), `functional-negative` / `injective-negative` (d=0 negative completion), `typecheck-arg-{0,1}`. The signature-driven counterpart of `std.elim`'s positional form; needs `forall`. The zebra2 formulation, generalised | S1.8a.f20 |

`std.algebra`'s ops split **intrinsic** (read existing edges: `compose` / `meet`
/ `difference` / `converse` / `join` / `difunctional`) vs **extensive** (range
over the `Dom×Ran` universe to reach absent pairs: `complement` / `top` /
`identity` / `connex`). The extensive ops take the puzzle's instance-type
relation + argument types `(?isa Dom Ran)` (the A10 universe) as parameters and
inherit the closed-world soundness caveat — sound only when the operand is
saturation-determined (the `std.closure` caveat), so not for branching puzzles.

The universal kernel rules (`symmetric` / `transitive` / `includes`) now live
here as the property-closure section, and the `zebra2*` fixtures + the two
backprop `branching/` demos import them
(`(import std.algebra :symbols (symmetric transitive includes))`) rather than
inlining — the S1.8.A5-tail. Example files whose inline copy is *byte-identical*
were migrated; files carrying a **variant** copy (a different `:why` text) and
the `saturation/{symmetric,transitive}/*` showcase demos deliberately keep the
rule inline.

*Planned (not yet shipped):* the **division residuals** (`R\S` / `R/S` / `syq` —
an allegory extension beyond Tarski's RA, S1.8.A12 T5) need a `forall` over the
universe and have no M1 consumer; design is in the stage doc.

**Rule modules, auto-closure, and `forall`.** Rule modules (`std.elim`,
`std.bijection`, the `std.algebra` rule families) are *generic* (parametrised
over a relation), so a puzzle imports them **flat** (`:symbols`) to keep the
bare names their activator facts reference. `:symbols` import is **auto-closure
within the module** (S1.8a.f20, superseding A1 D7's explicit-only rule): a listed
declaration drags in every *other* declaration of that module it references by
name, so you list only the **entry** rules (the ones the puzzle's facts activate)
and the machinery they assert/match follows. Names referenced *across* modules
(`forall` in std.macro; `total`/`functional` in std.algebra when a std.bijection
rule names them) are **not** fabricated — import those modules too:

```lisp
(import std.macro     :symbols (forall))                 ; cross-module dep, named explicitly
(import std.algebra   :symbols (bijective-properties))   ; +closure → functional/injective/total/surjective
(import std.bijection :symbols (bijective-setup typecheck-setup))  ; +closure → elim / negatives / typecheck
```

## Importing

Three tiers (Python-style — see the [A1 decision record](../../../../plans/m1_core_graph_reasoning/p1.8_ein_lang_modules/s1.8.a1_module_system_design.md#decision-record-2026-06-04)):

```lisp
(import std.macro)                        ; → (std.macro.forall …)   fully qualified
(import std.macro :as m)                  ; → (m.forall …)           aliased
(import std.macro :symbols (forall open)) ; → (forall …)             flat-selective + auto-closure
```

`:symbols` keeps the listed names **plus their within-module dependency closure**
(S1.8a.f20): an entry rule pulls what it asserts/matches without the importer
enumerating it. `:as` and `:symbols` are mutually exclusive. Names are logical and dotted
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
