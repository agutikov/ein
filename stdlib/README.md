# Ein standard library (`std.*`)

The canonical standard library, and the **single source of truth**. The import
resolver maps a logical module name `std.<path>` to `<stdlib-root>/<path>.ein`
(P1.8 S1.8.A1 §D4 / S1.8.A3), and `<stdlib-root>` is found by
[`ein-ir`](../ein.rs/crates/ein-ir/src/stdlib.rs)'s `stdlib::resolve`, in
three steps:

1. `$EIN_STDLIB` — an explicit override, always wins;
2. **this directory**, found by walking up for a `stdlib/` carrying
   `MANIFEST.sha256`. A checkout is authoritative, so editing a module below
   takes effect with no rebuild and no reinstall;
3. the **embedded copy** — the `include_dir!` tree inside the Rust binary, so
   `ein` is one self-contained file wherever it is installed.

`MANIFEST.sha256` is what makes a fork detectable. Two readers, since M1a
[S1a.10.4](../docs/history/m1a_rust/README.md#s1a104--utils-and-the-fuzzer):
step 2 above tests only that the file is *present* (that is what identifies a
directory as the stdlib), and `ein-ir`'s
`the_embedded_copy_matches_the_manifest` digests the embedded tree against it
— which is not stale-able, because `include_dir!` makes each module a build
dependency. `utils/stdlib_manifest.py` writes the manifest and verifies it
per-module without a toolchain; nothing else writes it, so editing a module
here is two steps and the Rust test is what fails if you take only the first.

This matters more than tidiness: the stdlib is not test data but part of the
semantics under test, so a second, drifting copy would make every result
meaningless — a result would be blamed on the engine when in fact the
*program* differs
([design/11](../docs/history/m1a_rust/design/11_shared_assets.md)).

## The obligation duals (M1d S1d.2.4)

Four rules that say what a state still **owes**, rather than what is illegal
in it. `std.algebra`'s `total-owed` / `surjective-owed` sit beside the
totality scans they mirror and are fanned out by `bijective-setup`;
`std.slots`' `slot-owed-room` / `slot-owed-fill` sit beside `slot-no-room` /
`slot-no-fill` and are fanned out by `slot-partition-setup`. **No puzzle
changes a line** — a `(bijective R)` or a `(slot-partition …)` already in a
file picks them up, and the only visible difference is two more stored
activator facts per declaration (50 across the corpus's 13 such entries).

Each pair is one property at its two ends. The scan is about a candidate set
of size **0** — every partner carries a stored negative, so the requirement
has become unreachable and the state is dead. The dual is about a set that is
still empty — nothing is stored either way, so the requirement is **unmet**,
not unmeetable. `(false)` becomes `(open ?R)`; *dead* becomes *unfinished*.

They derive nothing and are not in the saturation agenda; what reads them is
one pass over the quiescent KB, reported through `--events`' `owe` lines,
`--json-summary`'s `owes` block and the trace's *Outstanding obligations*
section
([`06_reserved_names.md` § the verdict atom](../docs/kernel/ir/03-ein-lang/06_reserved_names.md)).

## `std.digits`: a library of facts (M1f S1f.6.2)

The first module here that holds **facts and no rule**: the *ontology* half
of
[`solution_semantics.md` § 1.1](../docs/kernel/inference/solution_semantics.md#11-context-and-problem),
where every other module is theory. Ein has no numbers and no arithmetic (the
user's ruling of 2026-11-06), so the ten digits `0` to `9` are plain names,
and what school arithmetic knows about them is written down once, as tables,
the way hardware holds it. The arithmetic theories (`std.arith`, P1f.6's
S1f.6.3 onward, and `std.num9`) are the wiring; every step of theirs is a
lookup here, so nothing is guessed.

| table | reads | gives | rows |
|---|---|---|---:|
| `fa` | a, b, carry in (0 or 1) | sum digit, carry out | 200 |
| `fs` | a, b, borrow in (0 or 1) | difference digit, borrow out | 200 |
| `md` | a, b, carry in (0 to 8) | product digit, carry out | 900 |
| `cl` | the order below (`lt`, `eq`, `gt`), a, b | the order: a higher differing digit overrules | 300 |
| `nz` | a digit | it is not zero | 9 |
| `mid` | two bounds from 0 to 10, at least two apart | the digit to try between them | 45 |
| `adj` | two bounds | nothing lies between them | 10 |

**Generated.** [`utils/gen_arith_tables.py`](../utils/gen_arith_tables.py)
writes `digits.ein`; nobody types 1 664 rows and nobody reviews them by eye.
Edit the script, run it, then `python3 utils/stdlib_manifest.py --write`.
Two things hold the file, both in `cargo test`
([`ein-ir/tests/digit_tables.rs`](../ein.rs/crates/ein-ir/tests/digit_tables.rs)):
the script's `--check`, which fails when the file is not what the script
writes, and a second computation of every row from Rust's integers, none
missing and none twice. `stdlib_coverage.rs` has no rule here to ask about and
names the module as rule-free, beside `std.macro`.

**Importing.** Flat, like any library of facts. A listed table brings its
declaration and its rows (M1f S1f.6.11), and a table not listed is not loaded:

```lisp
(import std.digits :symbols (fa nz __closed__))
```

**Closed, and why `__closed__` is in that list.** A table is complete, so a
row is never a hypothesis, and the module says so with one written
`(__closed__ R)` fact per table. Those are facts of the relation
`__closed__`, and a selection brings a fact when its relation's name is in
the closure
([Q-M1f.142](../plans/m1f_hypothesis_and_documentation/open_questions.md#q-m1f142--a-stored-negation-and-a-fact-of-a-kernel-relation-under-a-selection)):
listed, as above, they come, all seven, and the universe has no candidate
among the tables. Not listed, they stay behind, and `nz` and `adj`, of one
and two arguments, are relations the search may guess rows of: on a program
that imports `(fa nz adj)` alone, `ein solve` stops with *a state's
commitments pass 2^128* (2026-11-11). A theory that reads the tables closes
the ones it reads in its own closing rule, as `std.arith` does, and its
importer lists nothing of this module.
[`tests/stdlib/digits/01_tables.ein`](../tests/stdlib/digits/01_tables.ein)
is the program that holds it: every table imported, a row of each claimed,
and R = {∅} claimed on its `(solve …)` form.

**What it costs.** A program that does not import the module pays nothing: a
module is read when an import names it. One that does pays the parse of the
whole file, 28.8 KB, whichever tables it lists: `ein test
tests/stdlib/digits/01_tables.ein` takes 19 ms where a `std.macro` test takes
6 ms (best of ten, release, 2026-11-11). **A core that passes through a table
names the row**, since a row is a premise like any other fact:
[`tests/stdlib/digits/02_a_core_names_its_rows.ein`](../tests/stdlib/digits/02_a_core_names_its_rows.ein)
refutes `(column 7 5 3)` and its unsat core is that fact and `(fa 7 5 0 2 1)`.

## `std.arith`: school arithmetic, computed by saturation alone (M1f P1f.6)

A theory written **in** Ein over the ten digit names: `std.digits`' tables are
the gates and this module's rules are the wiring. Every step is a table lookup
whose outcome the table decides, so nothing is guessed: on a program that
imports the theory and states requests the universe has no candidate, R is
{∅}, and the answer is root's fixpoint under `ein solve`, `ein saturate` and
`ein test` alike. No `absent`, no `not` premise, one stratum. Ein still has no
numbers (the user's ruling of 2026-11-06): nothing in the engine knows what
`7` is.

**Addition** (S1f.6.3), **subtraction and comparison** (S1f.6.4),
**multiplication** (S1f.6.5); division is the phase's next stage and joins
this module.

```lisp
(import std.arith :symbols (arith))

(add (n (n (n (n (n (n z 1) 2) 3) 0) 7) 8) (n (n (n (n z 4) 5) 6) 7))

(query :goal (sum-of ?x ?y ?s))      ; ?s = (n (n (n (n (n (n z 1) 2) 7) 6) 4) 5)
```

| | |
|---|---|
| **the number** | `z`, zero, or `(n PREFIX DIGIT)`, ten times the prefix plus the digit, so it reads as written: 42 is `(n (n z 4) 2)`. Any length: no width, no overflow. The normal form has no high zero; an operand written with high zeros is read by value, and every answer is in normal form |
| **the protocol** | a request fact `(add X Y)` is answered by a result fact that repeats its operands, `(sum-of X Y S)`. A goal reads it with a variable in the result's place, and a rule of the program may match it. Nothing is computed that was not asked for |
| **subtraction** | `(sub X Y)` is answered by `(diff-of X Y D R)`, **two numbers**: D is X − Y where X is not the smaller and `z` otherwise, and R is what X is short by, Y − X, where X is the smaller and `z` otherwise. One of the two is always `z`, both when X = Y. The theory has no negative number, and R is its place: a rule that matches `(diff-of X Y z (n ?p ?a))` holds exactly where X is the smaller, with no negation written |
| **comparison** | `(cmp X Y)` is answered by `(cmp-of X Y O)`, O one of the names `lt`, `eq`, `gt`, by value: 5 and 005 are `eq` |
| **multiplication** | `(mul X Y)` is answered by `(product-of X Y P)`. A product has no width to overflow: 123078 × 123078 is the eleven digits of 15148194084 |
| **named constants** | `(let NAME NUMBER)` names a number, and a request may name an operand: `(add price qty)` is answered by `(sum-of price qty S)`. The same sum written with the numbers is another request with its own answer. A name no `let` states is never answered |
| **the one entry** | `arith`, the closing rule: it names every rule of the theory and closes every relation of it and each table it reads, so one name brings all of it and leaves nothing of it to the search. A relation the importing program declares is the program's to close |

**How it computes** (design K of the phase's
[`design.md`](../plans/m1f_hypothesis_and_documentation/p1f.6_calculator/design.md#k-an-accumulator-in-the-other-order-and-a-continuation-on-every-call)):
each operand is looked up; one walk from the units takes each column through
the full adder's table and puts its digit in front of an accumulator, `(b
DIGIT REST)`, which so holds the answer most significant digit outermost;
high zeros are dropped there; the answer is turned round into the public
shape and handed over as `(done TAG ANSWER)`. Every call carries its
continuation as the TAG, a nested fact holding what the caller needs next.
123078 + 4567 is 21 firings, each the next step of one chain, so the circuit
is 21 deep (`arith_theory.rs`'s `the_circuit_of_one_addition`); 4 000 sums of
numbers of up to nine digits saturate in 110 ms (`python3
build/s1f.6.3/batch.py`, 2026-11-11).

**Subtraction is two walks side by side,** X − Y and Y − X over the full
subtractor's table, since which number is the smaller is not known before the
top column and nothing is guessed. The walk that ends with no borrow hands
its answer over; the other derives `(short TAG)` and no number. 123078 − 4567
is 28 firings and 21 deep, and so is the reverse. **Comparison is one walk**
over the comparator's table that builds no number and hands over `(order-is
TAG O)`: 13 firings for the same pair (`arith_theory.rs`'s
`the_circuit_of_one_subtraction_and_one_comparison`). 4 000 subtractions
saturate in 150 ms and 4 000 comparisons in 74 ms (`python3
build/s1f.6.4/batch.py`, 2026-11-11).

**Multiplication is the school method from the multiplier's top digit.** The
multiplier is turned round once (`top-first`, the other turn); each of its
digits is a row, the multiplicand times one digit, a walk over the digit
multiplier's table; and the sum so far times ten, one constructor, plus the
row is the addition walk, called with a tag that holds the digits still to
come. It is tagged throughout, `(mul-call TAG X Y)` answered by `(done TAG
P)`, so a later theory can call it. 123 × 456 is 72 firings and 123078 ×
123078 is 227, each one chain; the size grows as the product of the two
lengths (`arith_theory.rs`'s `the_circuit_of_one_multiplication`). 4 000
products of numbers of up to nine digits saturate in 1.1 s (`python3
build/s1f.6.4/batch.py mul`, 2026-11-11).

**The rule of the theory: no rule joins two facts on a number.** A rule has
one premise that holds numbers; its other premises are rows of `std.digits`
looked up by digits. The engine's fact index does not key on a nested fact
(E-3 of the phase's
[`findings.md`](../plans/m1f_hypothesis_and_documentation/p1f.6_calculator/findings.md)),
so a join on a number scans a relation for every fact of the other and a
batch costs the square of its count; written this way it costs in proportion.
Two tests hold it, and **whoever adds a rule keeps to it**:
[`ein-infer/tests/arith_theory.rs`](../ein.rs/crates/ein-infer/tests/arith_theory.rs)
on the module's text, and
[`arith_batch.rs`](../ein.rs/crates/ein-infer/tests/arith_batch.rs) on the
matcher's own count of its work, under `--features counters`, a gate step.
The one stated exception is `operand-name`, a join with a `let` on the
*name*, behind a `(named ?x)` check that keeps a number operand from walking
the `let` facts.

**What holds the answers.** `arith_theory.rs` holds every operation to Rust's
integers, a row of its `OPS` each: all pairs up to 120, a sample of every
pair of lengths up to 30 digits with equal operands and operands one apart at
every length, carries and borrows through every length, operands padded with
high zeros, and requests by name, each request answered exactly once and in
normal form.
[`ein-cli/tests/arith_search_free.rs`](../ein.rs/crates/ein-cli/tests/arith_search_free.rs)
holds the search-free claim on every program under `tests/stdlib/arith/` and
`examples/arith/`. [`examples/arith/add.ein`](../examples/arith/add.ein) is
the program to read first, then [`sub.ein`](../examples/arith/sub.ein) and
[`mul.ein`](../examples/arith/mul.ein).

**The names it occupies.** Imported flat, `z`, `n`, `b`, `let`, the requests
`add`, `sub`, `cmp` and `mul`, the results `sum-of`, `diff-of`, `cmp-of` and
`product-of`, the orders `lt`, `eq` and `gt`, the circuit's `named`,
`operand`, `operand-is`, `add-walk`, `sub-walk`, `cmp-walk`, `mul-walk`,
`strip`, `turn`, `top-first`, `top-first-is`, `done`, `short`, `order-is`,
`mul-call`, `mul-step`, the tags (`sum`, `difference`, `shortfall`, `order`,
`product`, `mul-digits`, `mul-row-of`, `mul-sum`, and each request's `-first`
and `-second`), `arith-rule`, and the tables `fa`, `fs`, `md`, `cl` and `nz`
mean what the module says; the module's header has the list.
Only `n`, `z`, `let`, the requests, the results and the three orders are its
interface. What a clash does, and what a malformed number does, is the
phase's S1f.6.12.

## What tests these modules

[**`tests/stdlib/`**](../tests/README.md) — one program per rule or tight
family (`find tests/stdlib -name '*.ein' | wc -l`), each stating what it
should and should not derive: the 45 one-per-rule-or-family added by M1c
[S1c.1.4](../docs/history/m1c_external_validation/README.md#s1c14--the-stdlib-corpus),
the closed-and-owing pair M1d S1d.2.2 banked under `closure/`, and the nine
M1d S1d.2.4 added with the obligation duals — four firing/satisfied pairs plus
`bijection/06_blind_enumeration.ein`, which is the one program in the
directory that leaves the blind enumerator on. `ein test tests/stdlib/` runs
the lot in 0.04 s and exits 1 if a claim is false.

Before them, **38 of the 73 rules below had never fired** in any of 400 corpus
runs — 33 never even loaded — and 20 more were reached by `examples/zebra.ein`
alone, so a change to that one puzzle's encoding could have untested a whole
module silently
([`stdlib_census.md`](../docs/history/m1c_external_validation/stdlib_census.md)).
The claim that every rule is now activated is *measured*, not read off the
directory, and since M1c
[S1c.1.5](../docs/history/m1c_external_validation/README.md#s1c15--in-the-gate)
it is **in the gate**: `ein-infer/tests/stdlib_coverage.rs` solves every
program under `tests/stdlib/` with `--events` on and fails on any rule none of
them activated — half a second, and **130 of 130** since `std.arith`'s 53
joined the 77 (M1f S1f.6.3 to S1f.6.5), which is a number this page may
state exactly because `every_stdlib_rule_is_activated_by_a_program` fails
the day it stops being true (`cat stdlib/*.ein | grep -c '^(rule '` is the
denominator). `python3 utils/stdlib_census.py --check` is the same census with
the numbers, over every corpus entry rather than the suite.

**"Activated" is two events since M1d S1d.2.4**, and the second is not an
optimisation of the first. A saturation rule reaches the agenda and emits
`fire`. An **obligation** rule — one whose `:assert` is the verdict atom
`open` — never can: it derives nothing and is deliberately kept out of the
agenda, so its evidence is the `owe` its post-fixpoint pass emits. Both the
gate and the census read both kinds. Counting only `fire` would have put the
four duals below permanently in the zero set, which is why
[S1d.2.3](../docs/history/m1d_satisfiability/README.md#s1d23--the-form)
deferred shipping them until the pass existed.

Adding a rule here therefore means adding a program there, and the gate says so
rather than a README. It is the same discipline as the corpus's growth rule,
one level up: a rule with no activating program is not tested, it is merely not
contradicted.

## Location decision (S1.8.A4 — closes [Q30](../plans/open_questions.md#q30--universal-rule-library--import-mechanism))

**Q30 → (c) hybrid.** Puzzle-*agnostic* vocabulary (the pattern macros today;
the relation-algebra / type rule families as they land) lives here as
importable modules; puzzle-*specific* content (a puzzle's activator facts,
its bespoke spatial/typecheck rules) stays inline in the puzzle file. A
puzzle pulls the library in with one `(import …)` and declares only its own
facts.

Why not `examples/`: that is user content and is not installed, so an
install-relative import could not find it.

## Modules

| module | file | provides | stage |
|--------|------|----------|-------|
| `std.macro` | [`macro.ein`](macro.ein) | the `forall` / `unknown` pattern macros | S1.5.9 |
| `std.arith` | [`arith.ein`](arith.ein) | school arithmetic on natural numbers of any length, `z` and `(n PREFIX DIGIT)`, **computed by saturation alone** over `std.digits`' tables: a request `(add X Y)` answered by `(sum-of X Y S)`, named constants `(let NAME NUMBER)`; no rule joins two facts on a number; one entry, the closing rule `arith`. Addition, subtraction with its two numbers `(diff-of X Y D R)`, comparison `(cmp-of X Y lt|eq|gt)` and multiplication `(product-of X Y P)` so far ([§ `std.arith`](#stdarith-school-arithmetic-computed-by-saturation-alone-m1f-p1f6)) | M1f S1f.6.3 to S1f.6.5 |
| `std.digits` | [`digits.ein`](digits.ein) | **facts, no rule**: the calculator's seven digit tables, `fa` / `fs` / `md` / `cl` / `nz` / `mid` / `adj`, 1 664 rows, each table closed by a written `(__closed__ R)`; generated by `utils/gen_arith_tables.py` ([§ `std.digits`](#stddigits-a-library-of-facts-m1f-s1f62)) | M1f S1f.6.2 |
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
uses `std.slots`, and they reach the same model
([`examples/README.md` § Zebra puzzle](../examples/README.md), which also has
the two encodings side by side and where to read the full design report).

**Why `std.slots` anchors its conclusions at the `Index` type** instead of
enumerating the equivalence closure is a cost, and it is the argument the
deleted `C2` report carried: `(transitive co-located)` materialises every pair
inside a class — 6 members × 5 ordered partners × 5 slots = **150** positive
edges, where the typed encoding holds the same information in **25** — and the
negative side is the complement of a 30-element square, **900** ordered pairs
against 125. The negative rules join positives *against* negatives, so the cost
is the **product** of the two. `slot-locate` is transitivity with its
conclusion pinned to the slot's index, and `slot-negative` likewise, which puts
every derived fact on the same Attribute × House rectangle the typed relations
occupy. Transitivity still *holds* — it is what justifies both rules — it is
simply not enumerated off the rectangle.

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

(A puzzle invoking a `std.macro` macro — `forall` / `unknown` — in its *own*
inline rule must still import it; the loader flags an unexpanded `(forall …)` as
a missing import rather than letting the rule silently never fire.)

## Importing

Three tiers (Python-style — see the A1 decision record):

```lisp
(import std.macro)                        ; → (std.macro.forall …)   fully qualified
(import std.macro :as m)                  ; → (m.forall …)           aliased
(import std.macro :symbols (forall unknown)) ; → (forall …)             flat-selective + auto-closure
```

`:symbols` keeps the listed names **plus their dependency closure** (S1.8a.f20),
following name references across the modules a symbol's own module imports; an
entry rule pulls what it asserts/matches without the importer enumerating it.
**It keeps facts too** (M1f S1f.6.11, 2026-11-11): every top-level fact of the
module whose relation is a name of that closure comes with it, as written, so
a library of facts, an ontology or a table, is imported flat like a library
of rules, and a listed name may be a relation the module only states facts
of. `std.digits` is the one `std.*` module that holds facts
([§ `std.digits`](#stddigits-a-library-of-facts-m1f-s1f62)). The whole rule,
and what a qualified import does with a module's facts, is
[`01_grammar.md` § Import](../docs/kernel/ir/03-ein-lang/01_grammar.md#import).
Importing the same declaration twice (the diamond a module's self-imports
create) is idempotent — identical collapses, a same-name conflict errors.
`:as` and `:symbols` are mutually exclusive. Names are logical and dotted
(`.` is a normal atom character); the `.ein` suffix is implied. A
file-relative import (a non-`std` name) resolves against the importing file's
directory.

To inline a puzzle's imports into a single standalone file (resolving +
tree-shaking unused library symbols) — **there is no CLI for this.** `ein ir
parse --resolve` did it from P1.8 until **P1.11** (2026-06-16, `8378ad7`),
which dropped the whole `ein ir` / `ein kb` inspection family; what survived is
the library call, and it is three functions:

```rust
let forms = ein_ir::parse(&mut ast, &text, Some(path))?;
let inlined = ein_ir::resolve_and_minimize(&mut ast, &forms, path.parent())?;
print!("{}", ein_ir::dump_canonical(&ast, &inlined));
```

`resolve_and_minimize` keeps an imported declaration that a live name reaches
*or* that a live match pattern activates — the `*-setup` glue rules are fired
by their pattern and never by name, so keeping only what is named would drop a
whole activator-driven module and leave a file that no longer solves.
`dump_canonical` is the canonical `.ein` printer, and the parser's unescape is
its inverse, so the text re-parses.

That the three compose into the **same program** is
[`the_inlining_route_the_stdlib_readme_documents_round_trips`](../ein.rs/crates/ein-ir/tests/imports_semantics.rs):
`zebra2.ein` inlined, re-parsed with **no** base directory to resolve against,
and the same relations, rules and facts. Putting the command back is a decision
about the shipping surface — the same one
[`utils/README.md`](../utils/README.md) records for `ein ir dot` / `ein kb dot`,
whose renderers are likewise alive and unreachable — not a `stdlib/` cleanup.

## One layout-detail per concern

One file per coherent concern (`macro.ein`, future `algebra.ein`,
`types.ein`) rather than one monolith — `:symbols` selective imports and the
tree-shaking dump both reward small, focused modules. A `README.md` here is
ignored by the resolver (only `*.ein` files are modules).

Each shipped stdlib symbol is exercised by a test (e.g. `forall` / `unknown`
by `tests/stdlib/macro/01_forall.ein` / `02_unknown.ein`, under the
stdlib-conformance gate). The full per-symbol API reference is deferred to
S1.20.C.
