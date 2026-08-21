# Ein standard library (`std.*`)

The canonical standard library, and the **single source of truth for both
implementations**. The import resolver maps a logical module name
`std.<path>` to `<stdlib-root>/<path>.ein` (P1.8 S1.8.A1 §D4 / S1.8.A3), and
`<stdlib-root>` is found the same way in each engine
([`ein.rs`](../ein.rs/crates/ein-ir/src/stdlib.rs) `stdlib::resolve`; it was
ein.py's `kb/imports.py::_stdlib_root` too, until M1a S1a.10.5):

1. `$EIN_STDLIB` — an explicit override, always wins;
2. **this directory**, found by walking up for a `stdlib/` carrying
   `MANIFEST.sha256`. A checkout is authoritative, so editing a module below
   takes effect with no rebuild and no reinstall;
3. the **packaged copy** — `ein/stdlib/` in a Python wheel (written at build
   time by `ein.py/_build.py`), the `include_dir!`-embedded tree in the Rust
   binary. Both distribution promises stay intact: `pip install ein` works,
   and `ein.rs` is one self-contained binary.

`MANIFEST.sha256` is what makes a fork detectable. Two readers, since M1a
[S1a.10.4](../plans/m1a_rust/p1a.10_single_implementation/s1a.10.4_utils.md):
step 2 above tests only that the file is *present* (that is what identifies a
directory as the stdlib), and `ein-ir`'s
`the_embedded_copy_matches_the_manifest` digests the embedded tree against it
— which is not stale-able, because `include_dir!` makes each module a build
dependency. `utils/stdlib_manifest.py` writes the manifest and verifies it
per-module without a toolchain; nothing else writes it, so editing a module
here is two steps and the Rust test is what fails if you take only the first.

This matters more than tidiness: the stdlib is not test data but part of the
semantics under test, so a second, drifting copy would make every result
meaningless — a diff would report "the engines disagree" when in fact the
*programs* differ
([design/11](../plans/m1a_rust/design/11_shared_assets.md)).

## Location decision (S1.8.A4 — closes [Q30](../plans/open_questions.md#q30--universal-rule-library--import-mechanism))

**Q30 → (c) hybrid.** Puzzle-*agnostic* vocabulary (the pattern macros today;
the relation-algebra / type rule families as they land) lives here as
importable modules; puzzle-*specific* content (a puzzle's activator facts,
its bespoke spatial/typecheck rules) stays inline in the puzzle file. A
puzzle pulls the library in with one `(import …)` and declares only its own
facts.

Why not `examples/`: that is user content and is not installed, so an
install-relative import could not find it.

Why the repo root and not inside `ein.py/` (moved there at M1a S1a.0.3): two
implementations read this library, and a directory inside one of them would
have made the other's copy a fork. The wheel still gets its copy — as a build
product, from here.

## Modules

| module | file | provides | stage |
|--------|------|----------|-------|
| `std.macro` | [`macro.ein`](macro.ein) | the `forall` / `open` pattern macros | S1.5.9 |
| `std.elim` | [`elim.ein`](elim.ein) | closed-world `typecheck-arg-{0,1}` + `domain-elimination` + `no-room-left` (generic; the instance-type relation is the `?isa` param, not a hardcoded `is-a` — S1.8.A10; needs `forall`) | S1.8.A8 |
| `std.closure` | [`closure.ein`](closure.ein) | `infer-closure` — `functional ∧ total ⇒ (__closed__ R)` (parameter-less; **opt-in, not for branching puzzles** — see the file's caveat) | S1.8.A6 |
| `std.algebra` | [`algebra.ein`](algebra.ein) | the full relation-algebra signature: relative (`converse` / `compose` / `identity`), Boolean (`meet` / `join` / `difference` / `complement` / `top` / `empty`), cardinality checks (`functional` / `injective` / `total` / `surjective` + the `bijective` fan-out — S1.8a.f20; `total`/`surjective` need `forall`), property checks (`irreflexive` / `antisymmetric` / `asymmetric` / `connex` / `difunctional`), property **closures** (`symmetric` (+ its `symmetric-negative` mirror via `symmetric-negative-setup`, S1.9) / `transitive` / `includes` — the universal kernel rules, S1.8.A5), `imply1` / `imply2-fwd` / `imply2-reverse`, the equational lemmas (`symmetric`⟺`converse R R`, Schröder `compose-negative-{r,s}`, contravariance, converse-over-join) + `converse-illtyped-{dom,ran}` signature typecheck (generic; lemmas use reflective rule-implication) | S1.8.A7 + A12 + A5 + f20 |
| `std.typing` | [`typing.ein`](typing.ein) | `(type-hierarchy ?isR*)` one-knob converse-typecheck driver + `(reflexive R)` closure (non-generic fan-out; pairs with `std.algebra`'s `converse-illtyped-*`) | S1.8.A10 |
| `std.bijection` | [`bijection.ein`](bijection.ein) | closed-world bijection inference, **signature-driven** (types read from `(relation R A B)`) and is-a-free: `bijective-setup` / `typecheck-setup` glue fan a `(bijective R)` + two hierarchy knobs into `domain-elimination` / `range-elimination` (survivor forcing), `functional-negative` / `injective-negative` (d=0 negative completion), `typecheck-arg-{0,1}`. The signature-driven counterpart of `std.elim`'s positional form; needs `forall`. The zebra2 formulation, generalised | S1.8a.f20 |
| `std.slots` | [`slots.ein`](slots.ein) | closed-world inference for a **single generic co-location relation** whose equivalence classes are *slots*, one member per type. `slot-partition-setup` fans `(slot-partition R isa sub Super Index)` into `slot-locate` (index-anchored transitivity), `slot-exclusive` (all-different within a type), `slot-occupied` (a slot's type-seat is taken), `slot-negative` (the contrapositive), `slot-elimination` / `slot-fill` (survivor forcing, both directions) and `slot-no-room` / `slot-no-fill` (⊥). `slot-spatial-setup` fans `(slot-spatial R S isa PT)` into eight congruence rules — `slot-adjacent-{fwd,bwd}` (+ negatives), `slot-prune-{fwd,bwd}`, `slot-endpoint-{fwd,bwd}` — so one relation name can be both a constraint between values and the structure it resolves against. Needs `forall` + `symmetric` / `symmetric-negative-setup`. The zebra.ein formulation, generalised | S1.22.1a |

**`std.bijection` vs `std.slots` — pick by how the puzzle names its links.**
Both give the same closed-world inference (negative completion, elimination,
no-room ⊥); they differ in what carries the property. `std.bijection` wants one
relation per attribute, each a bijection onto the positions, and reads the arg
types off `(relation R A B)` — so the property is per relation. `std.slots`
wants *one* relation shared by every attribute, and the property is scoped by a
type **family** (`Super`'s direct children) plus the type that names a slot
(`Index`) — because `(bijective R)` has nowhere to put a type pair, and scoping
per pair would need one declaration per ordered pair of attribute types. The two
Zebra encodings are the worked comparison: [`examples/zebra2.ein`](../examples/zebra2.ein)
uses `std.bijection`, [`examples/zebra.ein`](../examples/zebra.ein)
uses `std.slots`, and they reach the same model. See
C2
for the measurements, including why `std.slots` anchors its conclusions at the
`Index` type instead of enumerating the equivalence closure.

`std.algebra`'s ops split **intrinsic** (read existing edges: `compose` / `meet`
/ `difference` / `converse` / `join` / `difunctional`) vs **extensive** (range
over the `Dom×Ran` universe to reach absent pairs: `complement` / `top` /
`identity` / `connex`). The extensive ops take the puzzle's instance-type
relation + argument types `(?isa Dom Ran)` (the A10 universe) as parameters and
inherit the closed-world soundness caveat — sound only when the operand is
saturation-determined (the `std.closure` caveat), so not for branching puzzles.

The universal kernel rules (`symmetric` / `transitive` / `includes`) now live
here as the property-closure section, and the `zebra2*` fixtures + the two
`branching/` demos import them
(`(import std.algebra :symbols (symmetric transitive includes))`) rather than
inlining — the S1.8.A5-tail. Example files whose inline copy is *byte-identical*
were migrated; files carrying a **variant** copy (a different `:why` text) and
the `saturation/{symmetric,transitive}/*` showcase demos deliberately keep the
rule inline.

*Planned (not yet shipped):* the **division residuals** (`R\S` / `R/S` / `syq` —
an allegory extension beyond Tarski's RA, S1.8.A12 T5) need a `forall` over the
universe and have no M1 consumer; design is in the stage doc.

**Rule modules, auto-closure, and self-contained dependencies.** Rule modules
(`std.elim`, `std.bijection`, the `std.algebra` rule families) are *generic*
(parametrised over a relation), so a puzzle imports them **flat** (`:symbols`)
to keep the bare names their activator facts reference. Two S1.8a.f20 mechanics
make this ergonomic:

- **Auto-closure** (superseding A1 D7's explicit-only rule): a listed
  declaration drags in every *other* declaration **of any module reachable from
  it** that it references by name. So you list only the **entry** rules — the
  ones the puzzle's facts activate — and the machinery they assert/match
  follows.
- **Self-contained modules + idempotent import:** a module `(import …)`s its own
  dependencies (`std.algebra` pulls `forall`; `std.bijection` pulls `forall` +
  the cardinality rules), and re-importing an *identical* declaration is a
  no-op (a same-name **differing** body is still a conflict). So the diamond
  collapses and the importer needn't know transitive deps.

Net: pulling the whole bijection stack — elimination, negatives, typecheck,
the cardinality checks, and `forall` — is one line:

```lisp
(import std.bijection :symbols (bijective-properties bijective-setup typecheck-setup))
```

(A puzzle invoking a `std.macro` macro — `forall` / `open` — in its *own*
inline rule must still import it; the loader flags an unexpanded `(forall …)` as
a missing import rather than letting the rule silently never fire.)

## Importing

Three tiers (Python-style — see the A1 decision record):

```lisp
(import std.macro)                        ; → (std.macro.forall …)   fully qualified
(import std.macro :as m)                  ; → (m.forall …)           aliased
(import std.macro :symbols (forall open)) ; → (forall …)             flat-selective + auto-closure
```

`:symbols` keeps the listed names **plus their dependency closure** (S1.8a.f20),
following name references across the modules a symbol's own module imports; an
entry rule pulls what it asserts/matches without the importer enumerating it.
Importing the same declaration twice (the diamond a module's self-imports
create) is idempotent — identical collapses, a same-name conflict errors.
`:as` and `:symbols` are mutually exclusive. Names are logical and dotted
(`.` is a normal atom character); the `.ein` suffix is implied. A
file-relative import (a non-`std` name) resolves against the importing file's
directory.

To inline a puzzle's imports into a single standalone file (resolving + 
tree-shaking unused library symbols):

```sh
ein ir parse --resolve path/to/puzzle.ein
```

## One layout-detail per concern

One file per coherent concern (`macro.ein`, future `algebra.ein`,
`types.ein`) rather than one monolith — `:symbols` selective imports and the
tree-shaking dump both reward small, focused modules. A `README.md` here is
ignored by the resolver (only `*.ein` files are modules).

Each shipped stdlib symbol is exercised by a test (e.g. `forall` / `open` by
`tests/inference/test_forall.py` / `test_open.py` and
`tests/kb/test_imports.py`). The full per-symbol API reference is deferred to
S1.20.C.
