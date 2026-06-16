# Stdlib API reference (`std.*`)

The **per-symbol** reference for the package-shipped standard library. The
**module-level** overview — what each module is for, the import tiers,
auto-closure, and the package layout — lives in
[`ein.py/src/ein/stdlib/README.md`](../../../../ein.py/src/ein/stdlib/README.md);
this page documents each rule/macro's *activator form* and *effect*. For
the kernel atoms these build on (`and`/`absent`/`not`/`neq`/`relation`/…),
see [`06_reserved_names.md`](06_reserved_names.md).

> **Import recap.** Rule modules are *generic* (parametrised over a
> relation), so import them **flat** so the bare names their activator
> facts reference resolve: `(import std.algebra :symbols (symmetric transitive))`.
> Auto-closure (S1.8a.f20) drags in every declaration an entry rule
> references, so you list only the *entry* symbols. Modules self-import
> their own deps (`forall`, the cardinality checks) idempotently. (One
> exception, A1 D7: a symbol you invoke in your *own* inline rule —
> `forall`/`open` — you must import explicitly.)

## Three distinctions that govern soundness

Read these before importing — they decide whether a rule is safe for your
puzzle:

- **intrinsic vs extensive** *(std.algebra)*. **Intrinsic** ops read only
  existing edges (pure joins / NAF): `compose`, `meet`, `difference`,
  `converse`, `join`, `difunctional`. **Extensive** (⊙) ops range over the
  `Dom×Ran` universe to reach *absent* pairs, so they take the puzzle's
  instance-type relation + arg types `(?isa ?Dom ?Ran)` as parameters:
  `complement`, `top`, `identity`, `connex`.
- **check vs closure**. A **check** asserts `(false)` on *violation*
  (`functional`, `injective`, `total`, `surjective`, `empty`,
  `irreflexive`, `antisymmetric`, `asymmetric`, `connex`). A **closure**
  *derives the edge* that makes the property hold (`symmetric`,
  `transitive`, `includes`, `difunctional`, `reflexive`).
- **closed-world caveat**. Anything reading *absence* (the extensive ops,
  `difference`, `compose-negative-*`, `infer-closure`) is sound **only when
  the operand is saturation-determined** — never when it needs hypothesis
  **branching** (the Zebra attribute relations). Opt in per use.

---

## `std.macro` — pattern-macro sugar

Macros (load-time AST rewrites), not rules. A puzzle invoking one in its own
inline rule must import it.

| form | expands to | meaning |
|------|------------|---------|
| `(forall ?b G B)` | `(absent (and G (absent B)))` | guarded universal ∀b. G(b)→B(b) (the bound `?b` must appear in `G`) |
| `(open P)` | `(and (absent P) (absent (not P)))` | third state: `P` is neither asserted nor negated |

---

## `std.algebra` — the relation-algebra signature

Tarski's `⟨∪, ∩, ¬, 0, 1, ;, ⁻¹, 1'⟩` as generic rules. RA ≡ FOL with ≤3
variables, so most ≤3-var constraints are expressible here.

### Relative (composition) layer

| activator | effect | class |
|-----------|--------|-------|
| `(converse R1 R2)` | R2 = R1°: mirror `R1(a,b)`→`R2(b,a)` | intrinsic |
| `(imply1 R1 R2)` | `(R1 a)`⇒`(R2 a)` — 1-arg/property implication (e.g. `(imply1 functional __closed__)`) | intrinsic |
| `(imply2-fwd R1 R2)` | `(R1 a b)`⇒`(R2 a b)` | intrinsic |
| `(imply2-reverse R1 R2)` | `(R1 a b)`⇒`(R2 b a)` — ergonomic alias of `converse` | intrinsic |
| `(compose R1 R2 R3)` | R3 ⊇ R1;R2: `R1(a,b)∧R2(b,c)`→`R3(a,c)`. `(compose R R R)` = transitive closure | intrinsic |
| `(identity R isa Dom)` | self-loop every `Dom` element → `R(a,a)` | **extensive** |

### Boolean (lattice) layer

| activator | effect | class |
|-----------|--------|-------|
| `(meet R1 R2 R3)` | R3 ⊇ R1∩R2 (pairs in both) | intrinsic |
| `(difference R1 R2 R3)` | R3 ⊇ R1∖R2 (in R1, NAF on R2) | intrinsic · closed-world |
| `(join R1 R2 R3)` | R3 ⊇ R1∪R2 — **import `derive-join join-l join-r`** (fan-out idiom, not a single `(or …)` rule) | intrinsic |
| `(empty R)` | **check**: any `R(a,b)` ⟹ ⊥ | check |
| `(top R isa Dom Ran)` | the full `Dom×Ran` rectangle | **extensive** |
| `(complement R1 R2 isa Dom Ran)` | R2 ⊇ ¬R1 over the universe | **extensive** · closed-world |

### Cardinality (checks; ⊥ on violation)

| activator | effect |
|-----------|--------|
| `(functional R)` | right-unique: two distinct images of the same `a` ⟹ ⊥ |
| `(injective R)` | left-unique: one `c` reached from two distinct `a` ⟹ ⊥ |
| `(total R ?isa)` | left-total: an `a` with every `b` explicitly excluded ⟹ ⊥ (uses `forall`; open-world-safe) |
| `(surjective R ?isa)` | right-total: dual of `total` |
| `(bijective R)` | shorthand — fans out into `functional`+`injective`+`total`+`surjective` (rule `bijective-properties`) |

### Other property checks (⊥ on violation)

| activator | effect |
|-----------|--------|
| `(irreflexive R)` | self-loop `R(a,a)` ⟹ ⊥ |
| `(antisymmetric R)` | distinct mutual pair `R(a,b)∧R(b,a)`, `a≠b` ⟹ ⊥ |
| `(asymmetric R)` | *any* mutual pair (incl. self-loop) ⟹ ⊥ (= antisymmetric ∧ irreflexive) |
| `(connex R isa Dom)` | two distinct incomparable `Dom` elements ⟹ ⊥ (**extensive** · closed-world) |

### Property closures (derive the edge)

| activator | effect |
|-----------|--------|
| `(symmetric R)` | mirror every edge `R(a,b)`→`R(b,a)`. Pair with `symmetric-negative-setup` + `symmetric-negative` for the `¬`-mirror (`¬R(a,b)`⟹`¬R(b,a)`) |
| `(transitive R)` | transitive closure (guarded `neq a c` keeps an irreflexive closure irreflexive) |
| `(includes R S)` | subrelation lifting: copy each `R`-edge into `S` (e.g. `(includes is-a is-a*)`) |
| `(difunctional R)` | closure: `R(a,b)∧R(c,b)∧R(c,d)`→`R(a,d)` |

### Converse typecheck + equational lemmas

| activator | effect |
|-----------|--------|
| `(converse-illtyped-dom R1 R2 ?isR*)` / `(converse-illtyped-ran …)` | ⊥ if the converse pair's signatures don't reverse-match under the hierarchy `?isR*` |
| *auto (parameterless, reflective)* | `symmetric-is-self-converse`, `self-converse-is-symmetric`, `converse-pair-symmetric`, `compose-contravariant` (B7), `join-converse` (B8) — interrelate operator facts; converge via dedup |
| `(compose-negative-s R S T)` / `(compose-negative-r R S T)` | Schröder (B10): for a **closed** composite `T=R;S`, a missing composite forces a missing factor (`¬T(a,c)∧R(a,b)`⟹`¬S(b,c)`). Closed-world; opt in per triple |

---

## `std.bijection` — closed-world bijection inference (signature-driven)

Types are read from the `(relation R A B)` declaration; is-a-free (the
membership relation rides in as `?isa`). The generalised zebra2 formulation.
Drive it by declaring `(bijective R)` per relation plus two hierarchy knobs.

| symbol | form | effect |
|--------|------|--------|
| knob | `(bijection-hierarchy ?isa)` | the **direct** membership relation for elimination/totality/negatives |
| knob | `(typecheck-hierarchy ?isa)` | the **transitive** closure for arg typecheck |
| glue | `(bijective-setup)` | `(bijective R)`+hierarchy ⟹ `total`/`surjective`/`domain-elimination`/`range-elimination`/`functional-negative`/`injective-negative` activators |
| glue | `(typecheck-setup)` | `(bijective R)`+typed+hierarchy ⟹ `typecheck-arg-0/1` activators |
| `(functional-negative R ?isa)` | d=0 | `R(a,b)` ⟹ `¬R(a,b_other)` for every other `b_other` of `b`'s type |
| `(injective-negative R ?isa)` | d=0 | dual |
| `(domain-elimination R ?isa)` | survivor forcing | functional+total: every `b` but one excluded for `a` ⟹ force `R(a,b)` |
| `(range-elimination R ?isa)` | survivor forcing | injective+surjective: dual |
| `(typecheck-arg-0 R ?isa ?Dom)` / `(typecheck-arg-1 R ?isa ?Ran)` | check | arg not an `?isa`-member of `Dom`/`Ran` ⟹ ⊥ |

One-line pull of the whole stack: `(import std.bijection :symbols (bijective-properties bijective-setup typecheck-setup))`.

---

## `std.elim` — closed-world elimination (positional variant)

The same elimination/typecheck as `std.bijection` but **positional** — you
name the arg types explicitly and use the positional property markers
`(functional R 0 1)` / `(total R 0)`. Coexists with `std.bijection`; **don't
import both for the same relation** (the `domain-elimination` /
`typecheck-arg-*` names collide with different arities).

| activator | effect |
|-----------|--------|
| `(typecheck-arg-0 R ?isa ?Dom)` / `(typecheck-arg-1 R ?isa ?Ran)` | arg 0/1 not an `?isa`-`Dom`/`Ran` ⟹ ⊥ |
| `(domain-elimination R ?isa ?OT ?VT)` | functional+total: one `VT` survivor for `a` ⟹ force it |
| `(no-room-left R ?isa ?OT ?VT)` | functional+total: every `VT` excluded for `a` ⟹ ⊥ |

---

## `std.typing` — type-hierarchy convention

One-knob auto-wiring for converse typecheck + a reflexive-closure rule.

| symbol | form | effect |
|--------|------|--------|
| knob | `(type-hierarchy ?isR*)` | name your subtype relation once; every `(converse R1 R2)` is then typechecked against it |
| glue | `(type-hierarchy-converse)` | `(converse R1 R2)`+`(type-hierarchy isR*)` ⟹ the `converse-illtyped-dom/ran` activators |
| `(reflexive R)` | closure | self-loop every node `R` touches in either position (makes an irreflexive `is-a*` a genuine preorder) |

---

## `std.closure` — operational closure inference

| activator | effect |
|-----------|--------|
| `(infer-closure)` | `functional ∧ total` ⟹ `(__closed__ R)` (stop guessing on `R`) |

> ⚠️ **Completeness hazard.** `functional ∧ total` is an *operational*, not
> semantic, witness — safe only when `R` is saturation-determined. **Do not
> import into a puzzle that solves `R` by branching** (e.g. the Zebra
> attribute relations): closing `R` prunes the branches the search needs. The
> import *is* the gate (no config flag).

## See also

- [`ein.py/src/ein/stdlib/README.md`](../../../../ein.py/src/ein/stdlib/README.md)
  — module overview, import tiers, auto-closure, package layout.
- [`06_reserved_names.md`](06_reserved_names.md) — the kernel atoms these build on
  (+ the author quick-reference card).
- [`02_patterns.md`](02_patterns.md) — the pattern sub-language (`absent` / `forall` / `not`).
- [`examples/zebra2.ein`](../../../../examples/zebra2.ein) — the worked
  consumer (`bijective` / `domain-elimination` / `symmetric` / `includes`).
