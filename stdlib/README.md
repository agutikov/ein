# Ein standard library (`std.*`)

The standard library, and its **single source of truth**: every `std.*` module
is a file in this directory, and nothing holds a copy that can differ.
[`04-stdlib.md`](../docs/ein/user/04-reference/04-stdlib.md) is the reference a
program's author reads, module by module: what a program lists, every relation
and rule it writes or reads, what each module flags. This page is the
library's own, beside its files: where a module is read from, how the
directory is laid out, how each theory computes, and what holds it.

The import resolver maps a module name `std.<path>` to `<stdlib-root>/<path>.ein`,
and `<stdlib-root>` is found by
[`ein-ir`](../ein.rs/crates/ein-ir/src/stdlib.rs)'s `stdlib::resolve`, in
three steps:

1. `$EIN_STDLIB`, an explicit override, which always wins; a directory it
   names is refused unless it carries `MANIFEST.sha256`;
2. **this directory**, found by walking up from the `ein` binary's directory
   for a `stdlib/` carrying `MANIFEST.sha256`. A checkout is authoritative, so
   an edited module takes effect with no rebuild and no reinstall;
3. the **embedded copy**, the `include_dir!` tree inside the binary, so that
   `ein` is one self-contained file wherever it is installed.

`MANIFEST.sha256` lists every module, at any depth, with its digest, and two
things read it. Step 2 tests only that it is *present*, which is what
identifies a directory as the stdlib. `ein-ir`'s
`the_embedded_copy_matches_the_manifest` digests the embedded tree against it,
and cannot go stale, because `include_dir!` makes each module a build
dependency. The manifest is generated and never written by hand: a module
edited without the manifest written again fails that test.
`the_checkout_has_no_module_the_manifest_does_not_list` and `ein-infer`'s
`the_inventory_reads_every_module_the_manifest_lists` fail on a `.ein` here
that the manifest or the conformance test does not see.

The stdlib is not test data but part of the semantics under test: a second,
drifting copy would let two runs disagree because their *programs* differ,
and the difference would be blamed on the engine
([`02-shared-assets.md`](../docs/ein/ein.rs/03-design/02-shared-assets.md)).

## The layout

A module's name is its path under this directory, a dot a directory, and it
resolves the same way from each of the three places above: `std.algebra` is
`algebra.ein`, `std.math.decimal-arith` is `math/decimal-arith.ein`. Every
name a module declares has one full name, the module's name and the short
name, `std.math.decimal-arith.add`. The calculator is under `math/`, Boolean
functions and circuits under `bool/`
([Q-M1f.338](../docs/decisions/stdlib/Q-M1f.338-the-calculator-is-std-math-and-boolean-functions.md)),
and the relation algebra and the puzzle libraries at the top. The test suite
mirrors it: `tests/stdlib/math/<module>/`, `tests/stdlib/bool/<module>/`.

A rule family that is not about one problem is a module here, and a problem's
own rules (its activator facts, its own spatial or typing rules) stay in the
problem's file
([Q16](../docs/decisions/stdlib/Q16-general-rule-families-are-stdlib-modules.md)).
`examples/` is user content and is not installed, so an import could not find
a module there. One file is one coherent concern: every import brings its
module whole, and a small module is what keeps a program from loading what it
does not use. A `README.md` here is ignored by the resolver; only `*.ein`
files are modules.

| module | file | what it is | entry |
|---|---|---|---|
| `std.macro` | [`macro.ein`](macro.ein) | the pattern macros `forall` and `unknown`, expanded at load ([§](#the-relation-algebra-and-the-puzzle-libraries)) | none |
| `std.algebra` | [`algebra.ein`](algebra.ein) | the relation algebra: the relative operators (`converse`, `compose`, `identity`), the Boolean ones (`meet`, `join`, `difference`, `complement`, `top`, `empty`), the cardinality checks (`functional`, `injective`, `total`, `surjective`, and the `bijective-properties` fan-out), the property checks (`irreflexive`, `antisymmetric`, `asymmetric`, `connex`, `difunctional`), the property closures (`symmetric` with its `symmetric-negative` mirror through `symmetric-negative-setup`, `transitive`, `includes`), `imply1`, `imply2-fwd`, `imply2-reverse`, the equational lemmas (`symmetric` ⟺ `converse R R`, Schröder's `compose-negative-r` and `-s`, contravariance, converse over join), the signature typecheck `converse-illtyped-dom` and `-ran`, and the totality obligations `total-owed` and `surjective-owed` ([§](#the-relation-algebra-and-the-puzzle-libraries)) | none |
| `std.bijection` | [`bijection.ein`](bijection.ein) | closed-world bijection inference, read off the signature `(relation R A B)`: `bijective-setup` and `typecheck-setup` fan a `(bijective R)` and two hierarchy knobs into `domain-elimination` and `range-elimination` (survivor forcing), `functional-negative` and `injective-negative` (negative completion at root), and `typecheck-arg-0` and `-1`. Needs `forall`. The zebra2 encoding, generalised | none |
| `std.elim` | [`elim.ein`](elim.ein) | closed-world `typecheck-arg-0` and `-1`, `domain-elimination` and `no-room-left`, the instance-type relation a parameter `?isa`; the positional counterpart of `std.bijection`. Needs `forall` | none |
| `std.slots` | [`slots.ein`](slots.ein) | closed-world inference for **one co-location relation** whose equivalence classes are *slots*, one member per type: `slot-partition-setup` fans `(slot-partition R isa sub Super Index)` into `slot-locate`, `slot-exclusive`, `slot-occupied`, `slot-negative`, `slot-elimination` and `slot-fill`, `slot-no-room` and `slot-no-fill`, and the obligations `slot-owed-room` and `slot-owed-fill`; `slot-spatial-setup` fans `(slot-spatial R S isa PT)` into eight congruence rules, `slot-adjacent-fwd` and `-bwd` with their negatives, `slot-prune-fwd` and `-bwd`, `slot-endpoint-fwd` and `-bwd`. Needs `forall`, `symmetric` and `symmetric-negative-setup`. The zebra encoding, generalised | none |
| `std.typing` | [`typing.ein`](typing.ein) | `(type-hierarchy ?isR*)`, a converse-typecheck driver named once, and the `(reflexive R)` closure; pairs with `std.algebra`'s `converse-illtyped-*` | none |
| `std.closure` | [`closure.ein`](closure.ein) | `infer-closure`: `functional` ∧ `total` ⟹ `(__closed__ R)`. Opt-in, and **not for a relation the search decides** (the file's caveat) | none |
| `std.math.digits` | [`math/digits.ein`](math/digits.ein) | the decimal tables, facts and no rule, generated ([§](#stdmathdigits-a-library-of-facts)) | none |
| `std.math.arith` | [`math/arith.ein`](math/arith.ein) | arithmetic itself, what every base shares: `z`, and the division by zero's flag and its rule ([§](#stdmatharith-and-stdmathdecimal-arithmetic-itself-and-the-decimal-numeral)) | `arith` |
| `std.math.decimal` | [`math/decimal.ein`](math/decimal.ein) | the decimal numeral `(n PREFIX DIGIT)`, and its check where it is written ([§](#stdmatharith-and-stdmathdecimal-arithmetic-itself-and-the-decimal-numeral)) | `decimal` |
| `std.math.decimal-arith` | [`math/decimal-arith.ein`](math/decimal-arith.ein) | natural numbers of any length, school arithmetic over `std.math.decimal`'s numerals ([§](#stdmathdecimal-arith-school-arithmetic-computed-by-saturation-alone)) | `decimal-arith` |
| `std.math.positional` | [`math/positional.ein`](math/positional.ein) | the rules every positional base shares, every relation a parameter, and the macro `positional` whose one call connects a base ([§](#stdmathpositional-the-rules-every-positional-base-shares)) | the macro |
| `std.math.bits` | [`math/bits.ein`](math/bits.ein) | the binary tables, facts and no rule, generated ([§](#binary-stdmathbits-stdmathbinary-and-stdmathbinary-arith)) | none |
| `std.math.binary` | [`math/binary.ein`](math/binary.ein) | the binary numeral `(b2 PREFIX BIT)`, and its check where it is written ([§](#binary-stdmathbits-stdmathbinary-and-stdmathbinary-arith)) | `binary` |
| `std.math.binary-arith` | [`math/binary-arith.ein`](math/binary-arith.ein) | binary arithmetic, decimal's twin under `bin-` names ([§](#binary-stdmathbits-stdmathbinary-and-stdmathbinary-arith)) | `binary-arith` |
| `std.math.convert` | [`math/convert.ein`](math/convert.ein) | decimal to binary and back, as requests ([§](#stdmathconvert-decimal-to-binary-and-back)) | `convert` |
| `std.math.int` | [`math/int.ein`](math/int.ein) | integers over the natural numbers, `(int P N)` ([§](#stdmathint-integers-as-a-positive-and-a-negative-part)) | `int-arith` |
| `std.math.rat` | [`math/rat.ein`](math/rat.ein) | rationals over both, `(rat Z D)`, reduced only by `rat-simplify` ([§](#stdmathrat-rational-numbers-reduced-on-request)) | `rat-arith` |
| `std.math.num9` | [`math/num9.ein`](math/num9.ein) | nine digits in one fact, by wide rules, generated ([§](#stdmathnum9-nine-digits-in-one-fact-and-wide-rules)) | `arith9` |
| `std.math.gcd` | [`math/gcd.ein`](math/gcd.ein) | the greatest common divisor and the least common multiple of two natural numbers ([§](#stdmathgcd-the-greatest-common-divisor-and-the-least-common-multiple)) | `gcd` |
| `std.bool.gates` | [`bool/gates.ein`](bool/gates.ein) | the sixteen functions of two inputs, a table `(gate2 NAME A B OUT)` (its rows generated) and a relation each, `(And A B OUT)` ([§](#stdbool-the-sixteen-functions-of-two-inputs-and-circuits)) | `gates` |
| `std.bool.props` | [`bool/props.ein`](bool/props.ein) | thirty relations known of the sixteen, derived from the rows by rules ([§](#stdboolprops-what-is-known-of-the-sixteen-by-rules)) | `props` |
| `std.bool.circuit` | [`bool/circuit.ein`](bool/circuit.ein) | a circuit as a program, simulated by saturation ([§](#stdboolcircuit-a-circuit-as-a-program)) | `circuit` |
| `std.bool.inputs` | [`bool/inputs.ein`](bool/inputs.ein) | a circuit's inputs as hypotheses, and required values ([§](#stdboolinputs-inputs-as-hypotheses)) | `inputs` |

An **entry** is a module's closing rule: a rule with an empty `:match` that
closes every relation of the module and names every rule of it, so that one
name brings all of it and nothing of it is left for the search to guess. The
relation-algebra modules have none, since what they close is the program's to
say.

## Importing

Every import brings its module **whole**, with the module's own imports, one
copy per program; the three forms differ only in **names**
([Q-M1f.247](../docs/decisions/imports/Q-M1f.247-every-import-brings-its-module-whole.md)),
and the answer does not depend on which a file uses
([Q-M1f.249](../docs/decisions/imports/Q-M1f.249-the-answer-does-not-depend-on-the-import-form.md)):

```lisp
(import std.math.decimal-arith)                              ; the file writes std.math.decimal-arith.add, std.math.decimal-arith.sum-of
(import std.math.decimal-arith :as a)                        ; the file writes a.add, a.sum-of
(import std.math.decimal-arith :symbols (decimal-arith add sum-of)) ; the file writes add, sum-of
(import std.math.decimal :symbols (n))                       ; … and n, std.math.decimal's
```

Every name a module declares has one **full name**, `std.math.decimal-arith.add`,
the same whoever imports the module
([Q-M1f.231](../docs/decisions/imports/Q-M1f.231-every-name-a-module-declares-has-a-full-name.md));
it is what a state prints where the program gives the name no spelling of its
own (a circuit's step, `std.math.decimal-arith.strip`). **A file lists every
short name it writes**, from the module that declares it: nothing is
re-exported
([Q-M1f.232](../docs/decisions/imports/Q-M1f.232-a-module-has-no-export-list.md)),
so `std.math.int`'s programs list `int-add`, `int`, … from `std.math.int`, `n`
from `std.math.decimal` and `z` from `std.math.arith`, since `std.math.int`
only imports them. A short name a file writes and lists from no import is the
file's own, whatever a module calls the same
([Q-M1f.236](../docs/decisions/imports/Q-M1f.236-a-short-name-a-file-writes-and-lists-from-no.md));
one short name bound to two full names in one file is refused
([Q-M1f.234](../docs/decisions/imports/Q-M1f.234-one-short-name-bound-to-two-full-names-in-one.md)).
A module's entry rule (`decimal-arith`, `int-arith`, `rat-arith`, `arith9`) may
be listed, and brings nothing a whole import does not
([Q-M1f.240](../docs/decisions/imports/Q-M1f.240-a-module-keeps-its-entry-rule-which-a-program.md)).
An import brings a module's facts with it, which is how `std.math.digits`'
tables arrive. `:as` and `:symbols` are mutually exclusive. A module's name is
its path: `std.x.y` under this directory, any other name a path from the
**program file's** directory, a dot per directory, `..` refused
([Q-M1f.235](../docs/decisions/imports/Q-M1f.235-a-module-has-one-path-from-the-program-files.md)).
No import brings a module's `(query …)`, `(solve …)` or `(config …)`
([Q-M1f.215](../docs/decisions/imports/Q-M1f.215-no-import-brings-a-modules-query-solve-config.md)).
The whole rule, the refusals and what prints are
[`02-syntax.md` § Import](../docs/ein/user/04-reference/02-syntax.md#import) and
[`07-diagnostics.md` § 2.28](../docs/ein/user/04-reference/07-diagnostics.md#228-an-import-is-by-full-name-and-a-short-name-names-one-declaration-in-a-file).

**A program with its imports inlined.** No command prints it on its own;
every state file `ein solve FILE --dump-answer DIR` writes begins with it
([`10-answer-dump.md`](../docs/ein/user/04-reference/10-answer-dump.md)). The library
route is three calls:

```rust
let forms = ein_ir::parse(&mut ast, &text, Some(file))?;
let inlined = ein_ir::resolve_imports(&mut ast, &forms, Path::new(file).parent())?;
print!("{}", ein_ir::dump_canonical(&ast, &inlined));
```

The inlined text is the program as resolved, every module whole under its
full names; a name the program declares keeps its name, so the text loads with
nothing left to resolve against. `dump_canonical` is the canonical `.ein`
printer, and the parser's unescape is its inverse, so the text re-parses. That
the three compose into the **same program** is
[`the_inlining_route_the_stdlib_readme_documents_round_trips`](../ein.rs/crates/ein-ir/tests/imports_semantics.rs):
`zebra2.ein` inlined, re-parsed with **no** base directory to resolve against,
and the same relations, rules and facts.

## The relation algebra and the puzzle libraries

`std.macro` holds the pattern macros, `forall` and `unknown`, ordinary
`(macro …)` declarations expanded at load. A program that writes one in a rule
of its own imports `std.macro` and lists it; the loader refuses an unexpanded
`(forall …)` as a missing import rather than let the rule silently never match.

**Rule modules are generic**, parametrised over a relation: a puzzle writes
an activator fact, `(symmetric next-to)`, and lists the rule it names, so the
short name is the module's. A module imports its own dependencies
(`std.algebra` brings `std.macro`; `std.bijection` brings `std.algebra` and
`std.macro`), and every import brings its module whole, one copy per program,
so a diamond of imports is one copy and an importer need not know a module's
dependencies. The whole bijection stack, elimination, negatives, typecheck,
the cardinality checks and `forall`, is one line:

```lisp
(import std.bijection :symbols (bijective-setup typecheck-setup))
```

**`std.bijection` or `std.slots`: pick by how the puzzle names its links.**
Both give the same closed-world inference (negative completion, elimination,
no-room ⊥); they differ in what carries the property. `std.bijection` wants one
relation per attribute, each a bijection onto the positions, and reads the
argument types off `(relation R A B)`, so the property is per relation.
`std.slots` wants *one* relation shared by every attribute, and the property
is scoped by a type **family** (`Super`'s direct children) and the type that
names a slot (`Index`), because `(bijective R)` has nowhere to put a type
pair, and scoping per pair would need one declaration per ordered pair of
attribute types. The two Zebra encodings are the worked comparison:
[`examples/zebra2.ein`](../examples/zebra2.ein) uses `std.bijection`,
[`examples/zebra.ein`](../examples/zebra.ein) uses `std.slots`, and both are
held to the published answer
([Q-M1e.25](../docs/decisions/corpus/Q-M1e.25-both-zebra-encodings-are-held-to-the-published.md);
the [examples' catalogue](../examples/README.md) has them side by side).

**Why `std.slots` anchors its conclusions at the `Index` type** instead of
enumerating the equivalence closure is a cost. `(transitive co-located)`
materialises every pair inside a class, 6 members × 5 ordered partners × 5
slots = **150** positive edges, where the typed encoding holds the same
information in **25**; and the negative side is the complement of a
30-element square, **900** ordered pairs against 125. The negative rules join
positives *against* negatives, so the cost is the **product** of the two.
`slot-locate` is transitivity with its conclusion pinned to the slot's index,
and `slot-negative` likewise, which puts every derived fact on the same
Attribute × House rectangle the typed relations occupy. Transitivity still
*holds*, and it is what justifies both rules; it is not enumerated off the
rectangle.

**`std.algebra`'s operators are intrinsic or extensive.** An intrinsic one
reads existing edges: `compose`, `meet`, `difference`, `converse`, `join`,
`difunctional`. An extensive one ranges over the `Dom × Ran` universe to reach
absent pairs: `complement`, `top`, `identity`, `connex`. The extensive ones
take the puzzle's instance-type relation and argument types `(?isa Dom Ran)`
as parameters and inherit the closed-world caveat: sound only where the
operand is decided by saturation (the `std.closure` caveat), so not for a
relation the search decides
([`04-stdlib.md` § Three distinctions](../docs/ein/user/04-reference/04-stdlib.md#three-distinctions-that-govern-soundness)).
The module has no division residuals (`R\S`, `R/S`, `syq`), which would need
a `forall` over the universe.

**The universal closures** (`symmetric`, `transitive`, `includes`) are
`std.algebra`'s property-closure section. The Zebra encodings import them,
`(import std.algebra :symbols (symmetric transitive includes))`; smaller
examples, the `saturation/` showcase and the `branching/` demos among them,
write the rule inline.

### The obligation duals

Four rules that say what a state still **owes**, rather than what is illegal
in it. `std.algebra`'s `total-owed` / `surjective-owed` sit beside the
totality scans they mirror and are fanned out by `bijective-setup`;
`std.slots`' `slot-owed-room` / `slot-owed-fill` sit beside `slot-no-room` /
`slot-no-fill` and are fanned out by `slot-partition-setup`. A puzzle does not
name them: a `(bijective R)` or a `(slot-partition …)` already in a file picks
them up, and adds two stored activator facts per declaration.

Each pair is one property at its two ends. The scan is about a candidate set
of size **0**: every partner carries a stored negative, so the requirement
has become unreachable and the state is dead. The dual is about a set that is
still empty: nothing is stored either way, so the requirement is **unmet**,
not unmeetable. `(false)` becomes `(open ?R)`; *dead* becomes *unfinished*.

They derive nothing and are not in the saturation agenda; what reads them is
one pass over the quiescent KB, reported through `--events`' `owe` lines,
`--json-summary`'s `owes` block and the trace's *Outstanding obligations*
section
([`03-reserved-names.md` § the verdict atom](../docs/ein/user/04-reference/03-reserved-names.md#the-verdict-atom--open)).

## `std.math.digits`: a library of facts

A module of **facts and no rule**: the *ontology* half of
[`06-solution-semantics.md` § 1.1](../docs/ein/reasoning/02-kb-evolution/06-solution-semantics.md#11-context-and-problem),
where every other module is theory. Ein has no numbers and no arithmetic
([Q-M1e2.24](../docs/decisions/language/Q-M1e2.24-ein-has-no-numbers-and-no-ranges.md)),
so the ten digits `0` to `9` are plain names, and what school arithmetic knows
about them is written down once, as tables, the way hardware holds it. The
arithmetic theories, `std.math.decimal-arith` and `std.math.num9`, are the
wiring; every step of theirs is a lookup here, so nothing is guessed.

| table | reads | gives | rows |
|---|---|---|---:|
| `fa` | a, b, carry in (0 or 1) | sum digit, carry out | 200 |
| `fs` | a, b, borrow in (0 or 1) | difference digit, borrow out | 200 |
| `md` | a, b, carry in (0 to 8) | product digit, carry out | 900 |
| `cl` | the order below (`lt`, `eq`, `gt`), a, b | the order: a higher differing digit overrules | 300 |
| `nz` | a digit | it is not zero | 9 |
| `mid` | two bounds from 0 to 10, at least two apart | the digit to try between them | 45 |
| `adj` | two bounds | nothing lies between them | 10 |

**Generated.** `digits.ein`'s 1 664 rows are written by a generator and never
edited by hand
([Q-M1f.366](../docs/decisions/stdlib/math/Q-M1f.366-the-calculators-tables-are-generated-facts-every.md)):
the generator is edited and run, and the manifest written again.
[`ein-ir/tests/digit_tables.rs`](../ein.rs/crates/ein-ir/tests/digit_tables.rs)
computes every row a second time from Rust's integers, none missing and none
twice. `stdlib_coverage.rs` has no rule here to ask about and names the module
as rule-free, beside `std.macro`.

**Importing.** Like any library of facts: an import brings the module whole,
every table's declaration and its rows, and the list names what the file
writes short:

```lisp
(import std.math.digits :symbols (fa nz __closed__))
```

**Closed.** A table is complete, so a row is never a hypothesis, and the
module says so with one written `(__closed__ R)` fact per table
([Q-M1f.146](../docs/decisions/stdlib/math/Q-M1f.146-each-digit-table-is-closed-by-a-written-fact.md)).
Those are facts of the relation `__closed__`, and they come with the module,
all seven, whatever the list names, so the universe has no candidate among the
tables. A theory that reads the tables closes the ones it reads in its own
closing rule, as `std.math.decimal-arith` does, and its importer lists nothing
of this module.
[`tests/stdlib/math/digits/01_tables.ein`](../tests/stdlib/math/digits/01_tables.ein)
is the program that holds it: every table imported, a row of each claimed,
and R = {∅} claimed on its `(solve …)` form.

**What it costs.** A program that does not import the module pays nothing: a
module is read when an import names it. One that does pays the parse of the
whole file, whichever tables it lists. **A core that passes through a table
names the row**, since a row is a premise like any other fact:
[`tests/stdlib/math/digits/02_a_core_names_its_rows.ein`](../tests/stdlib/math/digits/02_a_core_names_its_rows.ein)
refutes `(column 7 5 3)` and its unsat core is that fact and `(fa 7 5 0 2 1)`.

## `std.math.arith` and `std.math.decimal`: arithmetic itself, and the decimal numeral

Two modules below `std.math.decimal-arith`, each holding what is its own. A
malformed decimal numeral is the decimal module's fault, and a malformed
binary one the binary module's, not arithmetic's
([Q-M1f.210](../docs/decisions/stdlib/math/Q-M1f.210-a-malformed-number-is-its-representations-fault.md));
zero is one `z` that both bases share
([Q-M1f.322](../docs/decisions/stdlib/math/Q-M1f.322-zero-is-one-z-in-every-base.md),
[Q-M1f.283](../docs/decisions/stdlib/math/Q-M1f.283-zero-is-a-relation-of-no-argument-and-each.md)),
and a division by zero is a contradiction of arithmetic itself, in every base
([Q-M1f.269](../docs/decisions/stdlib/math/Q-M1f.269-division-by-zero-is-a-contradiction.md)):

| module | declares | rules | entry |
|---|---|---|---|
| `std.math.arith` | `z`, zero, a relation of no argument no fact is of, so that it has one full name both bases share; the flag `division-by-zero` | `division-by-zero-is-false`, and the closing rule | `arith` |
| `std.math.decimal` | the numeral `n`; `named`, a name that stands for a number, which the module that declares `let` writes; the flag `not-decimal` | the numeral's check, five rules over `(written (n P A))`, and the closing rule | `decimal` |

`std.math.decimal-arith` imports both and lists what it writes of them, `z`,
`n` and `named`; `std.math.int`, `std.math.rat` and `std.math.num9` import
them too, each for the names it writes. **A program lists `n` from
`std.math.decimal` and `z` from `std.math.arith`** wherever it writes a
numeral, nothing being re-exported:

```lisp
(import std.math.decimal-arith :symbols (decimal-arith add sum-of))
(import std.math.decimal :symbols (n))
(import std.math.arith :symbols (z))
```

**The numeral's check reads what the loader states.** The loader states every
fact nested in a fact a file writes, and in a rule's conclusion where it holds
no variable, as `(written T)`
([`07-diagnostics.md` § 2.30](../docs/ein/user/04-reference/07-diagnostics.md#230-written-is-the-loaders-what-a-file-writes-stated-at-load),
[Q-M1f.272](../docs/decisions/language/Q-M1f.272-a-representation-module-reads-what-a-file-writes.md)),
so `std.math.decimal`'s rules see every place of every numeral a file writes,
wherever it sits, and flag a malformed one in root's first layer, with no
request reading it: § [Malformed requests and numerals](#malformed-requests-and-numerals-what-is-flagged-where-and-what-stays-silent)
has the table. `named` is `std.math.decimal`'s so that the module below
`let` can say *the prefix is a named constant* by a probe, at no cost a
well-formed numeral pays
([Q-M1f.278](../docs/decisions/stdlib/math/decimal/Q-M1f.278-a-named-constant-is-told-by-named-the-numeral.md)).

## `std.math.decimal-arith`: school arithmetic, computed by saturation alone

A theory written **in** Ein over the ten digit names: `std.math.digits`'
tables are the gates and this module's rules are the wiring. Every step is a
table lookup whose outcome the table decides, so nothing is guessed: on a
program that imports the theory and states requests the universe has no
candidate, R is {∅}, and the answer is root's fixpoint under `ein solve`,
`ein saturate` and `ein test` alike
([Q-M1f.365](../docs/decisions/stdlib/math/Q-M1f.365-the-calculator-closes-its-own-relations-so.md)).
No `not` premise, and **one `absent`, over `let`**, which a program writes and
no rule derives: the theory closes `let`, so the premise is decided at root,
the static check calls a program that imports the theory monotone, and it has
two strata. Four rules a program of the theory holds conclude `(false)`, each
from a flag: a division by zero (`std.math.arith`'s), a malformed request and
an undeclared name (`std.math.positional`'s, activated with decimal's
relations), and a malformed numeral (`std.math.decimal`'s). Nothing in the
engine knows what `7` is.

The four operations and comparison, the check of what a program wrote, and
the division by zero flagged at the call. The rules every base shares are
[`std.math.positional`](#stdmathpositional-the-rules-every-positional-base-shares)'s,
activated by one call in the closing rule: 35 rules of decimal's own and the
closing rule here, and the 77 it activates there (`grep -c '^(rule '
stdlib/math/decimal-arith.ein stdlib/math/positional.ein`).

```lisp
(import std.math.decimal-arith :symbols (decimal-arith))

(add (n (n (n (n (n (n z 1) 2) 3) 0) 7) 8) (n (n (n (n z 4) 5) 6) 7))

(query :goal (sum-of ?x ?y ?s))      ; ?s = (n (n (n (n (n (n z 1) 2) 7) 6) 4) 5)
```

| | |
|---|---|
| **the number** | `z`, zero, or `(n PREFIX DIGIT)`, ten times the prefix plus the digit, so it reads as written: 42 is `(n (n z 4) 2)`. Any length: no width, no overflow. The normal form has no high zero; an operand written with high zeros is read by value, and every answer is in normal form |
| **the protocol** | a request fact `(add X Y)` is answered by a result fact that repeats its operands, `(sum-of X Y S)` ([Q-M1f.363](../docs/decisions/stdlib/math/Q-M1f.363-a-calculation-is-a-request-fact-answered.md)). A goal reads it with a variable in the result's place, and a rule of the program may match it. Nothing is computed that was not asked for |
| **subtraction** | `(sub X Y)` is answered by `(diff-of X Y D R)`, **two numbers**: D is X − Y where X is not the smaller and `z` otherwise, and R is what X is short by, Y − X, where X is the smaller and `z` otherwise. One of the two is always `z`, both when X = Y. The theory has no negative number, and R is its place: a rule that matches `(diff-of X Y z (n ?p ?a))` holds exactly where X is the smaller, with no negation written |
| **comparison** | `(cmp X Y)` is answered by `(cmp-of X Y O)`, O one of the names `lt`, `eq`, `gt`, by value: 5 and 005 are `eq` |
| **multiplication** | `(mul X Y)` is answered by `(product-of X Y P)`. A product has no width to overflow: 123078 × 123078 is the eleven digits of 15148194084 |
| **division** | `(div X Y)` is answered by `(quot-of X Y Q R)`, the quotient and the remainder: X = Q × Y + R, and R less than Y |
| **division by zero** | a contradiction, **in two steps**: a request whose divisor is zero, however written, derives the flag `(division-by-zero (div X Y))`, and the flag derives `(false)` by `std.math.arith`'s rule ([Q-M1f.269](../docs/decisions/stdlib/math/Q-M1f.269-division-by-zero-is-a-contradiction.md)). Root is dead, R is empty, and no query of the program has an answer; `ein solve` names the request in the unsat core and `division-by-zero-is-false` as what refuted it. A program that must not die compares its divisor with zero first and derives its `div` request by a rule over `(cmp-of Y z gt)`. **It is flagged wherever it is called** ([Q-M1f.163](../docs/decisions/stdlib/math/decimal/Q-M1f.163-a-division-by-zero-is-flagged-wherever-division.md)): every division goes through the tagged call `(div-call TAG X Y)`, and a call whose Y is zero by value derives `(division-by-zero (div-call TAG X Y))`, the call as it was asserted, and `(false)` from it, whoever made the call, a rule of a program's own included. A public request therefore holds two flags, its own and its call's under the tag `(quotient X Y)` ([Q-M1f.211](../docs/decisions/stdlib/math/decimal/Q-M1f.211-a-public-division-by-zero-holds-two-flags.md)); `ein solve`'s read-out names the request, and `--trace` and `ein saturate --dump` show the second flag. `std.math.int`, `std.math.rat` and `std.math.num9` see a zero divisor before they call, so theirs is one flag, the request's. **A theory in which division by zero is allowed** (a wheel, a meadow, transreal arithmetic) would be a module of its own that does not import `std.math.arith`; none is built |
| **named constants** | `(let NAME NUMBER)` names a number, and a request may name an operand: `(add price qty)` is answered by `(sum-of price qty S)` ([Q-M1f.149](../docs/decisions/stdlib/math/decimal/Q-M1f.149-a-request-may-name-an-operand-by-a-let.md)). The same sum written with the numbers is another request with its own answer. A name stands for one number: a name no `let` states, a name stated twice with two numbers, and `z` or a digit as a name are each flagged (below) |
| **a malformed request** | a contradiction, **in two steps**, as a division by zero is ([Q-M1f.316](../docs/decisions/stdlib/math/Q-M1f.316-the-calculator-flags-each-malformed-request-it.md)). `(malformed REQUEST)`, the request as written, where a digit stands for an operand; `(malformed (let NAME NUMBER))` where a `let` cannot be read; `(undeclared NAME REQUEST)` where the request names an operand no `let` states. A malformed **numeral** is `std.math.decimal`'s, `(not-decimal NUMERAL THING)`, flagged where it is written, with no request reading it. Each flag derives `(false)` by a rule of its own. Root is dead, R is empty, every command exits 0, and `ein solve` names the fact the fault was written in, in the unsat core. **A flag means malformed; no flag does not mean well-formed** ([§ Malformed requests and numerals](#malformed-requests-and-numerals-what-is-flagged-where-and-what-stays-silent)) |
| **the one entry** | `decimal-arith`, the closing rule: it names every rule of the theory and closes every relation of it and each table it reads, so one name brings all of it and leaves nothing of it to the search. A relation the importing program declares is the program's to close |
| **the tagged calls** | what a theory built on the calculator calls, below: an interface, held to its shapes by a test |

**The tagged calls, an interface**
([Q-M1f.189](../docs/decisions/stdlib/math/Q-M1f.189-the-decimal-arithmetics-tagged-calls-are.md),
[Q-M1f.228](../docs/decisions/stdlib/math/Q-M1f.228-the-interface-of-the-tagged-calls-is-what-their.md)).
A result repeats its request's operands, so a rule of a program that wants an
answer for a purpose of its own must find it by its operands, a join on a
number, which the engine answers by a scan, since its fact index does not key
on a nested fact. Inside, the theory calls itself with a **tag**, a nested
fact that names the caller, and hands the answer back under it, so the rule
that reads has one premise
([Q-M1f.177](../docs/decisions/stdlib/math/Q-M1f.177-a-module-over-the-decimal-arithmetic-tags-each.md)).
Those calls are what `std.math.int`, `std.math.rat`, `std.math.gcd`, the
binary conversions and the programs under `examples/beyond_arith/` call:

```text
(add-walk TAG X Y C z)    asked: X + Y + C     answered by (done TAG S); C a carry in, 0 or 1
(sub-walk TAG X Y 0 z)    asked: X − Y         answered by (done TAG D) where X is not the smaller,
                                               and by (short TAG) where it is
(cmp-walk TAG X Y eq)     asked: X against Y   answered by (order-is TAG O), O one of lt, eq, gt
(mul-call TAG X Y)        asked: X × Y         answered by (done TAG P)
(div-call TAG X Y)        asked: X ÷ Y         answered by (div-done TAG Q R); flagged where Y is zero
(operand TAG X)           wanted: what X is    answered by (operand-is TAG N); X a number or a let's name
```

In the five calls X and Y are numbers as `n` and `z` build them, high zeros
allowed, never a name a `let` states: a call looks nothing up; the look-up,
`operand`, is the one that reads a `let`, and the place a theory over this one
adds its own values (`std.math.int` answers `(operand TAG (int P N))` itself, so
an integer stands for itself and a name goes through `let`). A walk starts
with `z`, no digits yet; a subtraction with `0`, no borrow, and a comparison
with `eq`, nothing yet decided; those are the values a caller writes. An
addition's carry in is the caller's: `0` for X + Y, or a bit, as
[`std.math.convert`](math/convert.ein) doubles a number and adds a bit in one
walk, `(add-walk TAG X X BIT z)`. The tag is a fact of the caller's own
relation, `(cnt K M)` in
[`casting_out_nines.ein`](../examples/beyond_arith/casting_out_nines.ein); a
tag of the theory's own, `(sum X Y)` or `(quotient X Y)`, is read back as one
of its requests.
[`arith_theory.rs`](../ein.rs/crates/ein-infer/tests/arith_theory.rs)'s
`the_tagged_calls_keep_their_shapes` holds the declarations of the eleven
relations, and of `std.math.int`'s, `std.math.rat`'s, binary's and
`std.math.gcd`'s calls below, to these shapes; holds each module's header and
this page to naming them as written here; and reads every module of the
stdlib and every program under `examples/` and `tests/` that brings the
theory and writes one of them, so a call written with another count of
arguments, or a walk started from another place, fails.

**How it computes**
([`04-stdlib.md`](../docs/ein/user/04-reference/04-stdlib.md#stdmathdecimal-arith--school-arithmetic-on-natural-numbers)
has the protocol): each operand is looked up
([Q-M1f.157](../docs/decisions/stdlib/math/decimal/Q-M1f.157-each-request-looks-its-operands-up-by-rules.md));
one walk from the units takes each column through the full adder's table and
puts its digit in front of an accumulator, `(b DIGIT REST)`, which so holds
the answer most significant digit outermost; high zeros are dropped there; the
answer is turned round into the public shape and handed over as `(done TAG
ANSWER)`. Every call carries its continuation as the TAG, a nested fact
holding what the caller needs next. 123078 + 4567 is 21 firings, each the
next step of one chain, so the circuit is 21 deep (`arith_theory.rs`'s
`the_circuit_of_one_addition`).

**Subtraction is two walks side by side,** X − Y and Y − X over the full
subtractor's table, since which number is the smaller is not known before the
top column and nothing is guessed
([Q-M1f.364](../docs/decisions/stdlib/math/decimal/Q-M1f.364-a-subtraction-answers-the-difference.md)).
The walk that ends with no borrow hands its answer over; the other derives
`(short TAG)` and no number. 123078 − 4567 is 28 firings and 21 deep, and so
is the reverse. **Comparison is one walk** over the comparator's table that
builds no number and hands over `(order-is TAG O)`: 13 firings for the same
pair (`the_circuit_of_one_subtraction_and_one_comparison`).

**Multiplication is the school method from the multiplier's top digit.** The
multiplier is turned round once (`top-first`, the other turn); each of its
digits is a row, the multiplicand times one digit, a walk over the digit
multiplier's table; and the sum so far times ten, one constructor, plus the
row is the addition walk, called with a tag that holds the digits still to
come. It is tagged throughout, `(mul-call TAG X Y)` answered by `(done TAG
P)`, so a later theory can call it. 123 × 456 is 72 firings and 123078 ×
123078 is 227, each one chain; the size grows as the product of the two
lengths (`the_circuit_of_one_multiplication`).

**Division is school long division, the quotient's digit by bisection**
([Q-M1f.324](../docs/decisions/stdlib/math/decimal/Q-M1f.324-decimal-division-is-long-division-each-digit.md)).
The divisor's multiples by 1 to 9 are computed once a division, each a row of
multiplication, in a chain whose tags hold the multiples already known, and
travel on as one fact of nine numbers, `multiples`; the first, the divisor times 1,
is the divisor in normal form and shows whether it is zero (its tag holds
the divisor as the call wrote it, which the call's flag repeats). The dividend is
turned round, and each digit is a step: it is brought down beside what is
left over, one constructor, and the quotient's digit, at least 0 and less
than 10, is found by halving the range through the `mid` and `adj` tables.
A trial takes the middle multiple from the number by subtraction's walk: `(done
TAG LEFT)`, it fits, or `(short TAG)`, it does not. At most four trials a
digit, nothing guessed. The nine trial rules, `div-try-1` to `div-try-9`, are
one rule written nine times, since a rule cannot pick the J-th of nine by a
digit without joining on a number; `arith_theory.rs` holds that they differ
by the digit alone
([Q-M1f.164](../docs/decisions/stdlib/math/decimal/Q-M1f.164-divisions-families-of-nine-rules-are-written.md)).
127645 ÷ 25 is 220 firings, one chain (`the_circuit_of_one_division`).

Each count above is pinned by the test named beside it, so that a change to
the wiring shows as a change to it, and `cargo test -p ein-infer --test
arith_theory the_circuit` re-takes them.

**The rule of the theory: no rule joins two facts on a number**
([Q-M1f.362](../docs/decisions/stdlib/math/Q-M1f.362-no-rule-of-the-calculator-joins-two-facts-on.md)).
A rule has one premise that holds numbers; its other premises are rows of
`std.math.digits` looked up by digits. The engine's fact index does not key on
a nested fact, so a join on a number scans a relation for every fact of the
other and a batch costs the square of its count; written this way it costs in
proportion. Two tests hold it, and **whoever adds a rule keeps to it**:
[`ein-infer/tests/arith_theory.rs`](../ein.rs/crates/ein-infer/tests/arith_theory.rs)
on the module's text, and
[`arith_batch.rs`](../ein.rs/crates/ein-infer/tests/arith_batch.rs) on the
work a batch does. Under `cargo test --workspace` `arith_batch.rs` holds that
the firings a request costs do not grow with the batch; the matcher's own
count of its work is compiled out of the shipped engine, and `cargo test -p
ein-infer --features counters --test arith_batch`, run by hand by whoever
changes a rule of the calculator, holds that count to the same proportion.
The stated exceptions join on a *name*: `operand-name`, an operand with its
`let`, behind a `(named ?x)` check that keeps a number operand from walking
the `let` facts; and `let-twice`, two `let`s of one name. A rule may also
hold guards, `eq` and `neq`, which compare and look nothing up, in three named
rules, and a probe, `(nz ?m)` or `(named ?m)`, one lookup whatever `?m` is;
`arith_batch.rs` holds a second count beside the matcher's, **the values the
guards were decided for**, which the first does not see.

**What holds the answers.** `arith_theory.rs` holds every operation to Rust's
integers, a row of its `OPS` each: all pairs up to 120, a sample of every
pair of lengths up to 30 digits with equal operands and operands one apart at
every length, carries and borrows through every length, operands padded with
high zeros, and requests by name, each request answered exactly once and in
normal form.
[`ein-cli/tests/arith_search_free.rs`](../ein.rs/crates/ein-cli/tests/arith_search_free.rs)
holds the search-free claim on every program under `tests/stdlib/math/decimal-arith/` and
`examples/arith/`. [`examples/arith/add.ein`](../examples/arith/add.ein) is
the program to read first, then [`sub.ein`](../examples/arith/sub.ein),
[`mul.ein`](../examples/arith/mul.ein), [`div.ein`](../examples/arith/div.ein)
and [`div_by_zero.ein`](../examples/arith/div_by_zero.ein); three generated
examples make a program long in three ways,
[`long_numbers.ein`](../examples/arith/long_numbers.ein),
[`many_calculations.ein`](../examples/arith/many_calculations.ein) and
[`factorial.ein`](../examples/arith/factorial.ein).
`ein saturate FILE` prints three times of its own: `parse:`, the program's
text; `kb load:`, macros, imports and facts, most of it the tables of
`std.math.digits`, the same for every program of the theory; and `saturate:`,
the calculation, with its firings.

### How long a number may be

The theory has no width and no overflow: a number is as long as its digits.
What bounds a run is outside the theory, in three places
([Q-M1f.185](../docs/decisions/engine/Q-M1f.185-a-state-file-nested-past-the-parsers-limit-does.md)).

**1. A program can write a number of 999 digits, and no longer.** A written
number nests a level a digit, and the parser refuses a program nested past
1 000 parentheses, with a line: *this `(` nests deeper than the limit of 1000
parentheses* (`ein_ir::grammar::NESTING_LIMIT`). The limit is the parser's
alone: a rule can derive a longer number, 999 nines plus 1 is answered with
its thousand digits, and the engine holds a fact of any depth.

**2. A state can be written that cannot be read back.** `ein solve
--dump-answer` writes a state as a program, one fact a line, and the parser
that reads it back has the same limit. A step of a circuit holds its operands
**inside its tag**, so its facts are deeper than the request that started it.
The depth of the deepest fact of the state file `ein solve FILE --dump-answer
DIR` writes, against an operand's digits:

| operation | the deepest fact of its state | the longest operand a request can hold | the longest whose state file reloads |
|---|---|---:|---:|
| `add`, `sub`, `cmp` | the operand's digits + 2 | 999 | 998 |
| `mul`, `div` | digits + 3 | 999 | 997 |
| `int-add` | digits + 4 | 998 | 996 |
| `int-mul` | digits + 5 | 998 | 995 |
| `rat-add`, `rat-cmp`, by a numerator's digits | digits + 7 | 997 | 993 |

The third column is the parser's limit less the request's own nesting, an
operand's digits and one for a natural number, two for an integer, three for
a rational; the fourth is 1 000 less the first. Past the fourth column every
answer is still right and still printed: `ein solve` and `ein saturate
--dump` write a deeper state without a word, and only the state file's reload
is refused, with the line above.

**3. Each step holds its numbers whole.** A step of a circuit is a fact that
holds its whole state, and nothing is forgotten. A sum, a difference and a
comparison cost a few firings a digit; a product's facts, and a quotient's,
grow as the product of the two lengths
(`the_circuit_of_one_multiplication` holds the growth at six and twelve
digits). Stored, a circuit's facts share their numbers; written as text, each
fact repeats its own, so the text `ein solve` and `ein test` read out, and a
state file, grow far faster than the stored state. A long product or quotient
is `ein saturate`'s to compute and a query's to read, not a state to write
out.

**The names it occupies.** Imported, the module's names mean what the module
says, and its header has the list, in three parts. **What a program states is
the requests and `let`**: `add`, `sub`, `cmp`, `mul`, `div` and `let`, over
numbers written with `n`, `z` and the ten digits; **and, from a rule, the
tagged calls**
([above](#stdmathdecimal-arith-school-arithmetic-computed-by-saturation-alone)).
**What it reads**: the results `sum-of`, `diff-of`, `cmp-of`, `product-of`
and `quot-of`; the flags `division-by-zero`, `malformed` and `undeclared`; the
orders `lt`, `eq` and `gt`; the tagged calls' answers, `done`, `short`,
`order-is`, `div-done` and `operand-is`. **The circuit's own**, which is in
the state to be seen and is no promise: `b`, the names of its walks, its check
and its tags, `arith-rule`, and the seven tables of `std.math.digits`
([Q-M1f.151](../docs/decisions/stdlib/math/decimal/Q-M1f.151-the-circuits-own-relations-are-named-with-words.md)).
A second *declaration* of any of them is refused at load, with the name in the
message. A written *fact* of a result or of the circuit is not: it is a fact
like a derived one and the theory computes with it, so a `(sum-of X Y S)`
written by hand is an answer nobody computed; no module reserves its names
([Q-M1f.319](../docs/decisions/imports/Q-M1f.319-no-module-reserves-its-names.md)).
**A number or a request with the wrong count of arguments is refused at
load**, `(n 1)`, `(n z 1 2)`, `(add (n z 5))`, each a form of a relation the
module declares
([`07-diagnostics.md` § 2.26](../docs/ein/user/04-reference/07-diagnostics.md#226-a-form-of-a-declared-relation-has-the-declared-count-of-arguments)).

### Malformed requests and numerals: what is flagged, where, and what stays silent

A fault is flagged by the module whose term it is a fault of
([Q-M1f.210](../docs/decisions/stdlib/math/Q-M1f.210-a-malformed-number-is-its-representations-fault.md)):
a numeral's by `std.math.decimal`, as soon as the numeral is written, wherever
it sits; an operand's and a `let`'s by the module that reads the operand,
`std.math.decimal-arith`, and by `std.math.int` and `std.math.rat` for their
own
([Q-M1f.316](../docs/decisions/stdlib/math/Q-M1f.316-the-calculator-flags-each-malformed-request-it.md)).

**A numeral is checked where it is written.** The loader states every fact
nested in a fact a file writes, and in a rule's conclusion where it holds no
variable, as `(written T)`
([`07-diagnostics.md` § 2.30](../docs/ein/user/04-reference/07-diagnostics.md#230-written-is-the-loaders-what-a-file-writes-stated-at-load)),
and every place of a numeral is such a fact, so `std.math.decimal`'s five
rules over `(written (n P A))` see every place of every numeral, as an
operand, inside an integer or a rational, as a `let`'s value nobody asks
about, or in a fact of the program's own relation. The flag names the
smallest `n` term at fault
([Q-M1f.274](../docs/decisions/stdlib/math/decimal/Q-M1f.274-a-malformed-numeral-is-flagged-on-its-smallest.md)),
and comes in root's first layer, before any calculation:

| what the program wrote | the flag | by |
|---|---|---|
| a digit's place that holds a name that is no digit, `(n (n z 1) x)`, `10` and a sign among such names, or a fact, `(n z (n z 1))` | `(not-decimal NUMERAL THING)` | `std.math.decimal`'s `numeral-digit`: ten `neq` guards, which hold of every other name and of a fact |
| a numeral that ends in a digit where `z` goes, `(n 1 2)`, `(n 0 2)` | `(not-decimal NUMERAL THING)` | `numeral-prefix-digit`, `numeral-prefix-zero` |
| a numeral that ends in a named constant, `(n price 1)` | `(not-decimal NUMERAL THING)` | `numeral-prefix-named`, by the probe `(named ?m)`: `named` is `std.math.decimal`'s, and `std.math.decimal-arith`, which declares `let`, writes it |
| a digit or a named constant where a part of an integer goes, `(int 7 z)`, `(int price z)` | `(not-integer INTEGER THING)` | `std.math.int`'s six shape rules, over `(written (int P N))` |
| a digit or a named constant where a rational's integer or its denominator goes, `(rat 7 (n z 2))`, `(rat (int (n z 1) z) two)` | `(not-rational RATIONAL THING)` | `std.math.rat`'s six shape rules, over `(written (rat Z D))` |
| a digit where a number goes, `(add 7 X)`: seven is `(n z 7)` | `(malformed REQUEST)` | `operand-digit`, `operand-digit-zero` |
| a name stated twice with two numbers | `(malformed (let NAME NUMBER))`, each of the two | `let-twice`, a join on the name |
| `z` or a digit as a constant's name, `(let z N)`, `(let 7 N)` | `(malformed (let NAME NUMBER))` | `let-zero`, `let-digit`, `let-digit-zero` |
| a `let` that names a digit or another named constant, `(let seven 7)`, `(let alias price)` | `(malformed (let NAME N))` | `let-to-digit`, `let-to-digit-zero`, `let-to-name`, whether or not a request names it ([Q-M1f.207](../docs/decisions/stdlib/math/decimal/Q-M1f.207-a-let-that-cannot-be-read-is-flagged-whether.md), [Q-M1f.281](../docs/decisions/stdlib/math/decimal/Q-M1f.281-a-let-that-names-a-digit-is-flagged.md)) |
| an operand that is a name no `let` states, `(add foo X)` | `(undeclared foo REQUEST)` | `operand-undeclared`: the theory's one `absent`, over `let` ([Q-M1f.317](../docs/decisions/stdlib/math/decimal/Q-M1f.317-a-name-no-let-states-is-flagged-as-undeclared.md)) |
| an undeclared name or a digit as an operand of `std.math.int` or `std.math.rat` | `(undeclared …)`, `(malformed REQUEST)` | the three rules of each module from its tags to the request, `int-bad-only`, `-first`, `-second` and `int-bad-div` (a division's tag holds `(int-div-of REQUEST)`: the request holds two flags, the tag's and its own), `rat-bad-only`, `-first`, `-second` ([Q-M1f.206](../docs/decisions/stdlib/math/decimal/Q-M1f.206-an-operands-fault-travels-with-its-tag.md)) |

Each flag derives `(false)` by a rule of its own, `not-decimal-is-false`,
`not-integer-is-false`, `not-rational-is-false`, `malformed-is-false` and
`undeclared-is-false`, so a trace shows the flag as one step and the
contradiction as the next:

```text
  states (k_s)      0
  empty             every node is dead — root's own fixpoint derives (false)
  unsat core        (add (n (n z 1) x) (n z 5))
  refuted by        std.math.decimal.not-decimal-is-false
```

The core names the fact the numeral was written in, the `written` fact being
derived from it; a numeral written whole in a rule's conclusion rests on no
fact, and its core is ∅. A request has one flag, the request as written, and
no result fact; the well-formed requests beside it are computed in root's
state, which is dead and so no answer. Such a program can claim only
`(solve :expect (false))`. A `let` that is added answers an undeclared
request and leaves nothing of the flag: root is the only state, and it has
either the `let` or the flag.

**A flag means malformed; no flag does not mean well-formed**
([Q-M1f.318](../docs/decisions/stdlib/math/Q-M1f.318-a-flag-means-malformed-and-no-flag-does-not-mean.md)).
There is no general report of a request no rule completes. These stay silent,
neither refused nor answered, and `arith_theory.rs`'s `what_stays_silent`
holds that the list is this one:

- **a numeral that ends in a name that is neither a digit nor a named
  constant**, `(n end 1)`. Saying *a name* would cost a walk of the program's
  names, at every place of every numeral;
- **a fact of another relation where a number goes**, as an operand or
  inside a number, `(add (pair 1 2) X)`, the accumulator's own shape `(b 1
  z)` among them. No rule can say *not of this shape*;
- **a name that stands for either**;
- **a numeral a rule assembles from a variable**, `(n ?p x)`: no file writes
  it, so nothing states it
  ([Q-M1f.275](../docs/decisions/language/Q-M1f.275-written-states-every-fact-a-file-nests.md)),
  and a top-level fact of `n`, which is no nested term;
- **in `std.math.int` and `std.math.rat`, an operand of another type**: a
  natural number where an integer goes, and an integer or a natural number
  where a rational goes
  ([Q-M1f.277](../docs/decisions/stdlib/math/Q-M1f.277-an-operand-of-another-kind-of-number-is-answered.md));
- **every malformation of `std.math.num9`**, which is kept as an example of
  another representation
  ([Q-M1f.320](../docs/decisions/stdlib/math/num9/Q-M1f.320-std-math-num9-is-a-second-representation.md)).

**What the check costs.** A well-formed numeral costs it no firing: its ten
`neq` guards and two probes a place, read off a `written` fact the loader
stated, and the loader's statement of each nested fact. 123078 + 4567 is 21
firings and 21 deep, the walk alone (`the_circuit_of_one_addition`).

**What the undeclared name costs.** To say that an operand is a *name*, and
not a number, which is a fact, `operand-undeclared` ranges a variable over
the program's names and keeps the one that is the operand
([Q-M1f.208](../docs/decisions/stdlib/math/decimal/Q-M1f.208-two-guards-say-that-an-operand-is-a-name.md)):
two comparisons a name, for each operand of each request. It does not grow
with the batch; **it grows with the program's names**. `arith_batch.rs` pins
it by the guards' own count.

## `std.math.positional`: the rules every positional base shares

Most of school arithmetic does not know its base: a walk takes a column a
firing and looks it up in a table, and nothing in it says *ten* or *two*.
Those rules are written once, here, for decimal and binary alike. **Every
relation a rule names is a parameter**, in capitals, and a base activates each
rule with its own relations
([Q-M1f.298](../docs/decisions/stdlib/math/Q-M1f.298-a-shared-rules-text-writes-the-bases-own-numeral.md)):

```lisp
(rule add-column (?ADD-WALK ?N ?FA ?B)
  :match  (and (?ADD-WALK ?k (?N ?p ?a) (?N ?q ?c) ?e ?acc)
               (?FA ?a ?c ?e ?s ?f))
  :assert (?ADD-WALK ?k ?p ?q ?f (?B ?s ?acc))
  :why    "{?a} + {?c} + carry {?e} is {?s}, carry {?f}: {?s} goes in front of the digits so far")
```

decimal's `add-column` walks `add-walk` over `n` and the table `fa`, binary's
`bin-add-walk` over `b2` and `bin-fa`. 77 rules (`grep -c '^(rule '
stdlib/math/positional.ein`): the operands and the `let`'s name probe; the
addition, subtraction and comparison walks; the normal form and both turns;
the five operations' ask, second operand, start and answer; multiplication's
and division's calls; the division by zero at the request and at the call,
with `std.math.arith`'s flag; and an operand's and a `let`'s faults, the ten
routes from a tag to its request, and the two request flags with their rules
to `(false)`. A numeral's own faults are no rule here: they are the
representation's, `std.math.decimal`'s and `std.math.binary`'s.

**One call connects a base**
([Q-M1f.266](../docs/decisions/stdlib/math/Q-M1f.266-a-base-is-connected-to-the-shared-rules-by-one.md)).
A rule with parameters fires for a fact headed by its own name, so 77 rules
take 77 activators; the module's macro `positional`, of the rules' 61
parameters, expands to them, and a base calls it once inside its closing
rule's conclusion, with its relations in the macro's order:

```lisp
(rule decimal-arith ()
  :match  ()
  :assert (and (__closed__ fa) … (arith-rule mul-column) …
               (positional operand operand-is n let named add-walk fa b strip turn nz done …))
  :why    "…")
```

The activators are derived in the closing rule's own firing, so connecting a
base costs no firing; each activated rule compiles when its activator appears.
A program never imports this module: it comes with a base, one copy a program,
and a program that imports both bases holds each rule once, activated twice. A
shared rule prints by its own full name whichever base activated it,
`std.math.positional.add-column` in a trace, `std.math.positional.malformed-is-false`
under `refuted by`. A base's closing rule does not name these rules, as it
names its own: a rule's name written as an argument makes every
parameterised conclusion of the rule's arity a possible activator that binds a
parameter to a fact, which the loader refuses.
[`arith_theory.rs`](../ein.rs/crates/ein-infer/tests/arith_theory.rs) holds the
macro to activating each rule once with that rule's parameters
([Q-M1f.270](../docs/decisions/stdlib/math/Q-M1f.270-the-connecting-macros-body-is-written-by-hand.md)),
each base's call to its order (`one_call_of_the_macro_connects_each_base`),
and the rule of the calculator on this module's text read as each base
activates it.

## Binary: `std.math.bits`, `std.math.binary` and `std.math.binary-arith`

Binary arithmetic as a theory of its own, beside decimal: its own numeral, its
own names, the same rules where they are the same
([Q-M1f.321](../docs/decisions/stdlib/math/binary/Q-M1f.321-binary-arithmetic-is-a-theory-of-its-own-beside.md);
[`04-stdlib.md`](../docs/ein/user/04-reference/04-stdlib.md#binary-stdmathbits-stdmathbinary-and-stdmathbinary-arith)
has every relation).

| module | what it is |
|---|---|
| `std.math.bits` | the tables, **facts and no rule**, generated beside `std.math.digits` and held to Rust's integers by `ein-ir/tests/digit_tables.rs`: `bin-fa` 8 rows, `bin-fs` 8, `bin-cl` 12, `bin-nz` 1, where decimal's seven tables have 1 664. No digit multiplier and no bisection: a row of a product is nothing or the multiplicand, and one subtraction decides a bit of a quotient |
| `std.math.binary` | the numeral `(b2 PREFIX BIT)`, twice the prefix plus the bit, and its check where it is written, on `std.math.decimal`'s pattern ([Q-M1f.295](../docs/decisions/stdlib/math/binary/Q-M1f.295-the-binary-numeral-is-std-math-binary-s-which.md)): four rules over the loader's `(written (b2 P A))` flag `(not-binary NUMERAL THING)`, a bit's place holding no bit, a bit or a named constant as the prefix, and `not-binary-is-false` derives `(false)`; `bin-named`. Entry `binary` |
| `std.math.binary-arith` | the arithmetic, `std.math.decimal-arith`'s twin: 57 relations and 14 rules of its own (`grep -c '^(relation '` and `grep -c '^(rule ' stdlib/math/binary-arith.ein`, less the closing rule), and `std.math.positional`'s 77 activated by one call in its closing rule. Entry `binary-arith` |

```lisp
(import std.math.binary-arith :symbols (binary-arith bin-add bin-mul bin-sum-of bin-product-of))
(import std.math.binary :symbols (b2))
(import std.math.arith :symbols (z))

(bin-add (b2 (b2 (b2 z 1) 0) 1) (b2 (b2 (b2 z 1) 1) 1))   ; 101 + 111: (bin-sum-of … (b2 (b2 (b2 (b2 z 1) 1) 0) 0))
(bin-mul (b2 (b2 (b2 z 1) 0) 1) (b2 (b2 (b2 z 1) 1) 1))   ; 101 x 111: 100011
```

**The protocol** is decimal's with `bin-` before every name, the numeral's
head `b2`
([Q-M1f.267](../docs/decisions/stdlib/math/binary/Q-M1f.267-binarys-names-carry-the-prefix-bin.md)):
`bin-add`/`bin-sum-of`, `bin-sub`/`bin-diff-of`, `bin-cmp`/`bin-cmp-of`,
`bin-mul`/`bin-product-of`, `bin-div`/`bin-quot-of`, and `bin-let`. Zero is
`std.math.arith`'s `z`, of no base. A division by zero is `std.math.arith`'s
flag, `(division-by-zero (bin-div X Y))` and its call's, and `(false)` from
each by that module's rule: arithmetic's own, whatever the base. **Nothing of
decimal is loaded**: a program that imports only the binary theory holds no
row of `std.math.digits` and no name of `std.math.decimal-arith`
([Q-M1f.192](../docs/decisions/stdlib/math/Q-M1f.192-a-program-that-reads-the-binary-tables-imports.md)).
A program that imports both holds both, sharing `z`, `0`, `1`,
`std.math.arith` and the shared rules, and no relation else.

**The tagged calls**, binary's twins of decimal's interface, held to their
shapes as decimal's are
([Q-M1f.297](../docs/decisions/stdlib/math/binary/Q-M1f.297-binarys-tagged-calls-are-an-interface.md)):

```text
(bin-add-walk TAG X Y C z)    asked: X + Y + C     answered by (bin-done TAG S)
(bin-sub-walk TAG X Y 0 z)    asked: X − Y         answered by (bin-done TAG D), or (bin-short TAG)
(bin-cmp-walk TAG X Y eq)     asked: X against Y   answered by (bin-order-is TAG O)
(bin-mul-call TAG X Y)        asked: X × Y         answered by (bin-done TAG P)
(bin-div-call TAG X Y)        asked: X ÷ Y         answered by (bin-div-done TAG Q R); flagged where Y is zero
(bin-operand TAG X)           wanted: what X is    answered by (bin-operand-is TAG N)
```

**Malformed requests, as far as decimal flags its own**
([Q-M1f.268](../docs/decisions/stdlib/math/binary/Q-M1f.268-binary-flags-a-malformed-request-as-far-as.md)):
a malformed numeral is `std.math.binary`'s, `(not-binary …)`, flagged where it
is written; a bit where a number goes, `(bin-add 1 X)`, and a `bin-let` that
cannot be read are `(bin-malformed …)`; a name no `bin-let` states is
`(bin-undeclared NAME REQUEST)`, by `bin-operand-undeclared`, the theory's one
`absent`, over `bin-let`. A decimal digit is a name in binary: `(bin-add 7 X)`
is undeclared, and `(b2 7 1)`, a prefix that is such a name, is silent, as
`(n end 1)` is in decimal. A fact of another relation where a number goes, a
decimal number among them, is answered by silence.

**What it costs**, in firings and depth, at decimal's example operands
(`ein-infer/tests/binary_theory.rs`' `the_circuit_of_each_operation` pins
them; `cargo test -p ein-infer --test binary_theory the_circuit` re-takes
them): 123078 + 4567, 43 and 43 (decimal 21); 123078 − 4567, 61 and 43 (28);
against, 24 (13); 123 × 456, 120 (72); 127645 ÷ 25, 245 (220); 127645 ÷ 0, 12
and 10 (12). A walk is three times as long; a product's row and a quotient's
bit are cheaper. `binary_theory.rs` holds every answer to Rust's integers,
every pair to 64 and lengths around every power of two to 64 bits, and
`arith_batch.rs` a batch to cost in proportion to its count.

## `std.math.convert`: decimal to binary and back

A theory built on both bases, a module of its own so that a program that only
computes in binary does not load decimal's tables
([Q-M1f.323](../docs/decisions/stdlib/math/Q-M1f.323-conversion-between-the-bases-is-a-module.md)):

```lisp
(import std.math.convert :symbols (convert to-bin to-dec bin-of dec-of))
(import std.math.decimal :symbols (n))
(import std.math.binary :symbols (b2))
(import std.math.arith :symbols (z))

(to-bin (n (n (n z 2) 5) 5))         ; (bin-of 255 11111111)
(to-dec (b2 (b2 (b2 (b2 z 1) 0) 1) 0))   ; (dec-of 1010 10)
```

`(to-bin X)` halves X by decimal's tagged division until nothing is left, each
remainder a bit, and binary's strip and turn make the number; `(to-dec B)`
reads B's bits from the top and doubles the number so far by one decimal
addition walk, the bit as the carry in. Eleven rules, each a step in a chain
of the two theories' own calls, and no rule joins two facts on a number. The
two directions do not cost the same: 255 to binary is 826 firings and
1111101000 to decimal 100, a halving being a whole division and a doubling one
addition walk (`ein saturate` of each request alone, the six firings of the
closing rules left out). An operand is a number; a name is not looked up.
`binary_theory.rs` holds every number to 300, and a sample to 2^40, both ways
and round trip.

## `std.math.num9`: nine digits in one fact, and wide rules

`(num9 0 0 0 1 2 3 0 7 8)` is 123078, nine digit names, the most significant
first, a fixed width. The theory is the same circuit over the same tables of
`std.math.digits`, wired the other way: where `std.math.decimal-arith` walks a
number one column a firing, a rule here has **a premise a column**, so a sum
is one firing. Computed by saturation alone: no candidate, R {∅}, no `absent`,
no `not`, no guard, one stratum.

```lisp
(import std.math.num9 :symbols (arith9))

(add9 (num9 0 0 0 1 2 3 0 7 8) (num9 0 0 0 0 0 4 5 6 7))

(query :goal (sum9-of ?x ?y ?s))      ; ?s = (num9 0 0 0 1 2 7 6 4 5)
```

**The file is generated**, as `std.math.digits` is: nine columns of variables
twenty times over are not for typing or for reviewing by eye. The generator
writes the same theory at any width, and its width-2 theory, a generated test
fixture, is how `num9_theory.rs` holds it exhaustively; no other width is shipped. 47 rules and 22 relations
(`grep -c '^(rule ' stdlib/math/num9.ein`, `grep -c '^(relation '`). **A wide
rule is wide in registers too:** a rule binds its variables to a file of 256
(`ein_infer::MAX_REGS`); every other rule of the corpus is held to an eighth
of that, and `div9-start`, the widest here, binds 162
(`ein-infer/tests/compile_limits.rs` holds both,
[Q-M1f.173](../docs/decisions/engine/Q-M1f.173-the-register-file-stays-256-wide.md)).

| | |
|---|---|
| **the protocol** | as `std.math.decimal-arith`'s, under names of its own, so the two import into one program ([Q-M1f.167](../docs/decisions/stdlib/math/num9/Q-M1f.167-the-names-of-std-math-num9-are-decimals-with-a-9.md)): `(add9 X Y)` answered by `(sum9-of X Y S)`; `(sub9 X Y)` by `(diff9-of X Y D R)`, two numbers, one of them zero; `(cmp9 X Y)` by `(cmp9-of X Y O)`, O one of `lt`, `eq`, `gt`; `(mul9 X Y)` by `(product9-of X Y P)`; `(div9 X Y)` by `(quot9-of X Y Q R)`. An operand is a number written out: the module has no named constants ([Q-M1f.170](../docs/decisions/stdlib/math/num9/Q-M1f.170-a-num9-operand-is-a-number-written-out.md)) |
| **overflow** | **a fact, and no number** ([Q-M1f.168](../docs/decisions/stdlib/math/num9/Q-M1f.168-overflow-is-a-fact-and-no-number-and-a-division.md)). A sum or a product that does not fit nine digits derives `(overflow REQUEST)`, the request as written, and no result fact: never a truncated number. Root stays alive, the program's other requests are answered, and a rule of the program may match the flag. A difference, a comparison and a quotient cannot overflow |
| **division by zero** | a contradiction, as everywhere in the calculator: `div9-by-zero` derives `(division-by-zero (div9 X ZERO))`, and `std.math.arith`'s `division-by-zero-is-false`, which this module imports, derives `(false)`. Root is dead and R is empty |
| **conversions** | a request each way, one firing: `(to9 X)`, X a natural number of `std.math.decimal`, is answered by `(as9 X W)`, or by `(big X)` where X is written with more than nine digits; `(from9 W)` by `(of9 W X)`, X in normal form. Nothing converts a number because it exists, and no rule of either module hands a request to the other: a program that wants both writes the rules that ask ([`examples/arith/num9_both_ways.ein`](../examples/arith/num9_both_ways.ein)) |
| **the one entry** | `arith9`, the closing rule: it names every rule and closes every relation of the module, the six tables it reads, and the two relations it takes from the modules below, `n` and `division-by-zero` |

**How it computes.** `add9-fits` has a premise for the request and nine rows
of the full adder's table, the carry handed from column to column in a
variable, the carry out of the top written `0`; its twin `add9-overflows`
writes `1` there. A difference is two such rules, one each way round, and the
one that does not borrow out of the top answers, as in hardware: the borrow
is the comparison. A comparison is one rule over the comparator's table
([Q-M1f.171](../docs/decisions/stdlib/math/num9/Q-M1f.171-std-math-num9-counts-and-compares-with-the-digit.md)).
**A product is eleven firings:** a start, a step for each digit of the
multiplier from its top, an end. A step is one rule with eighteen table
premises, the multiplicand times the digit and that row added to the sum so
far shifted one place; where the digit is 0 a second rule shifts and reads no
table. Three rules derive the overflow, one for each place a digit can fall
off. **A quotient is eleven firings:** `div9-start` computes the divisor's
multiples by 2 to 9 in one firing, 72 rows of the digit multiplier's table,
and puts all nine into the state
([Q-M1f.169](../docs/decisions/stdlib/math/num9/Q-M1f.169-a-divisions-multiples-are-computed-into.md));
then a step a digit of the dividend, by ten rules, one for each digit the
quotient can have, each reading its own multiple from the state and holding
two subtractors: the multiple taken from the number must not borrow, and the
divisor taken from what is left must. Exactly one fires, and the tables decide
which. `num9_theory.rs`' `the_circuit_of_each_operation` pins each count
(`cargo test -p ein-infer --test num9_theory the_circuit`).

**No rule joins two facts on a number,** `std.math.decimal-arith`'s rule, kept
here and held by the same two tests, `num9_theory.rs` on the text and
`arith_batch.rs` on the work a batch does. With the multiples in the state,
and a product's rows computed inside its step, a batch costs in proportion to
its count.

### When to use which module

Firings a request, each pinned by a test (`num9_theory.rs` and
`arith_theory.rs`' `the_circuit` tests): a sum, 123078 + 4567, is one firing
here and 21 in `std.math.decimal-arith`; a product, 123 × 456, eleven against
72; a quotient, 127645 ÷ 25, eleven against 220. A trace of a sum is one step
with its nine table rows, where `std.math.decimal-arith`'s is twenty-one.

- **`std.math.num9` where every number fits nine digits and there are many of
  them.** A product here always takes its nine steps, where
  `std.math.decimal-arith` takes as many as the multiplier has digits.
- **`std.math.num9` where a fixed width is what is modelled:** a register, a
  counter, a field of a form, with the overflow as a fact a rule can read.
- **`std.math.decimal-arith` for numbers of any length, for named constants,
  and for a program with a request or two.** A program that imports
  `std.math.num9` loads its wide rules and, since the module imports it,
  `std.math.decimal-arith` whole besides.
- **Both,** in one program, with a conversion request where a number crosses.

**What holds it.**
[`ein-infer/tests/num9_theory.rs`](../ein.rs/crates/ein-infer/tests/num9_theory.rs)
holds the theory to Rust's integers: **exhaustively at width 2**, the
generator's output over `(num2 d d)`, every pair of operands from 00 to 99
under all five operations, 49 900 requests, each with exactly one answer or
exactly one flag and never both; at width 9 on a sample of every pair of
lengths with the corners; the conversions at every count of digits; a
division by zero; and what each request costs in firings. The same file holds
the rule of the theory, the closing rule and the absence of any guard on the
module's text. `arith_search_free.rs` holds every program under
[`tests/stdlib/math/num9/`](../tests/stdlib/math/num9/) and the three examples
search-free, and `answer_round_trip.rs` reloads each one's state.
[`examples/arith/num9.ein`](../examples/arith/num9.ein) is the program to
read first, then [`num9_overflow.ein`](../examples/arith/num9_overflow.ein)
and [`num9_both_ways.ein`](../examples/arith/num9_both_ways.ein).

**The names it occupies:** `num9`; the requests `add9`, `sub9`, `cmp9`,
`mul9`, `div9`, `to9` and `from9`; the results `sum9-of`, `diff9-of`,
`cmp9-of`, `product9-of`, `quot9-of`, `as9` and `of9`; the flags `overflow`
and `big`; the states `mul9-at` and `div9-at`, with `multiples9` and `row9`;
`num9-rule` and the entry `arith9`. From `std.math.decimal` it takes `n`, from
`std.math.arith` `z`, `division-by-zero` and the rule from it to `(false)`,
and from `std.math.digits` `fa`, `fs`, `md`, `cl`, `nz` and `adj`, with the
three orders. A `num9` whose count of digits is not nine is **refused at
load**, the loader holding a form to its declaration's count; one with a
digit's place that holds something else is answered by silence: this module
flags no malformed digit
([Q-M1f.320](../docs/decisions/stdlib/math/num9/Q-M1f.320-std-math-num9-is-a-second-representation.md)).

## `std.math.int`: integers as a positive and a negative part

An integer is `(int P N)`, two natural numbers of `std.math.decimal-arith`,
and its value is P less N. The module is written over
`std.math.decimal-arith`, which it calls and does not edit, and is computed by
saturation alone: no candidate, R {∅}, and no `absent`, no `not` and no guard
of its own (a program that imports it holds `std.math.decimal-arith`'s one
`absent`, and two strata).

```lisp
(import std.math.int :symbols (int-arith))

(int-add (int z (n (n z 1) 2)) (int (n (n z 4) 0) z))     ; -12 + 40

(query :goal (int-sum-of ?x ?y ?s))      ; ?s = (int (n (n z 2) 8) z)
```

| | |
|---|---|
| **the integer** | `(int P N)`. 7 is `(int (n z 7) z)`, −7 is `(int z (n z 7))`, zero is `(int z z)`, and `(int (n z 5) (n z 3))` is 2, not simplified. The **normal form** has one part `z` and the other in `std.math.decimal-arith`'s normal form; every integer has exactly one. An operand is read by value, whatever its parts and their high zeros, and every answer is in normal form, so a rule of a program tells a negative answer by its shape, `(int z (n ?p ?a))`, with no negation written |
| **the protocol** | `(int-simplify X)` answered by `(int-simple-of X S)`; `(int-neg X)` by `(int-neg-of X S)`; `(int-add X Y)` by `(int-sum-of X Y S)`; `(int-sub X Y)` by `(int-diff-of X Y S)`, **one integer**, where `std.math.decimal-arith`'s `diff-of` answers two numbers: they are its two parts; `(int-mul X Y)` by `(int-product-of X Y S)`; `(int-cmp X Y)` by `(int-cmp-of X Y O)`, O one of `lt`, `eq`, `gt`, by value ([Q-M1f.174](../docs/decisions/stdlib/math/int/Q-M1f.174-the-names-of-std-math-int-carry-the-prefix-int.md)) |
| **an operand** | an integer written out, or a name a `(let NAME (int P N))` states: `let` is `std.math.decimal-arith`'s. A natural number is **not** an integer operand, and such a request is never answered and not flagged ([Q-M1f.175](../docs/decisions/stdlib/math/int/Q-M1f.175-an-integer-operand-is-an-integer-written-out.md), [Q-M1f.277](../docs/decisions/stdlib/math/Q-M1f.277-an-operand-of-another-kind-of-number-is-answered.md)). **An undeclared name and a digit as an operand are flagged** on the request, by the module's three rules from its tags to the request and `int-bad-div`; **a digit or a named constant where a part goes**, `(int 7 z)`, `(int price z)`, is the integer's own fault, `(not-integer INTEGER THING)`, flagged where it is written ([Q-M1f.276](../docs/decisions/stdlib/math/Q-M1f.276-an-integers-and-a-rationals-own-shape-is-checked.md)); a malformed numeral inside a part is `std.math.decimal`'s ([§ Malformed requests and numerals](#malformed-requests-and-numerals-what-is-flagged-where-and-what-stays-silent)) |
| **division** | **names its convention** ([Q-M1f.179](../docs/decisions/stdlib/math/int/Q-M1f.179-integer-division-names-its-convention.md)); there is no bare `int-div`. Each request is answered by one fact with a quotient and a remainder, X = Q·Y + R: `(int-div-euclid X Y)` by `(int-quot-euclid-of X Y Q R)`, 0 ≤ R < \|Y\|, Q an integer and R a **natural number** of `std.math.decimal-arith`; `(int-div-trunc X Y)` by `(int-quot-trunc-of X Y Q R)`, Q rounded toward zero and R with the dividend's sign, both integers; `(int-div-floor X Y)` by `(int-quot-floor-of X Y Q R)`, Q rounded down and R with the divisor's sign, both integers ([Q-M1f.200](../docs/decisions/stdlib/math/int/Q-M1f.200-an-integer-divisions-result-is-named-after.md)). The three agree where neither operand is negative, and with `std.math.decimal-arith`'s `div` there |
| **division by zero** | a divisor that is zero by value, however written or named, derives `std.math.arith`'s flag, `(division-by-zero REQUEST)`, the request as written, and `std.math.arith`'s rule derives `(false)`: root is dead and R is empty, as everywhere in the calculator. The module's one rule that concludes `(false)` is its integer's shape's, `not-integer-is-false` |
| **the one entry** | `int-arith`, the closing rule: it closes every relation of the module and `nz`, names every rule, and names `decimal-arith`, `decimal` and `arith`, the entries of the modules it imports |

**How it computes.** Each operation works on the parts with
`std.math.decimal-arith`'s walks and then simplifies. Simplifying is
subtraction both ways at once, P − N and N − P: the walk that ends with no
borrow hands over the part that is left. A sum is the positive parts added and
the negative parts added; a difference is a sum with the second operand's
parts exchanged, and a negation a simplification with them exchanged. A
product is (P1·P2 + N1·N2, P1·N2 + N1·P2), four products of
`std.math.decimal-arith` and two sums, and **no rule looks at a sign**
([Q-M1f.176](../docs/decisions/stdlib/math/int/Q-M1f.176-a-product-of-integers-is-computed-on-the-parts.md)).
A comparison adds across, P1 + N2 against N1 + P2, and compares two natural
numbers; it builds no integer. **A division** simplifies both operands, so
that each shows its sign by its shape, and hands the two sizes to
`std.math.decimal-arith`'s `div-call` with the signs in the tag; that much is
written once for the three conventions. Truncated then only puts the signs on.
Floored and Euclidean adjust two of the four combinations of signs where a
remainder is left: one more in the quotient's size, an addition walk, and the
divisor's size less the remainder, a subtraction walk.

| X ÷ Y | Euclidean Q, R | truncated Q, R | floored Q, R |
|---|---|---|---|
| 7 ÷ 2 | 3, 1 | 3, 1 | 3, 1 |
| −7 ÷ 2 | −4, 1 | −3, −1 | −4, 1 |
| 7 ÷ −2 | −3, 1 | −3, 1 | −4, −1 |
| −7 ÷ −2 | 4, 1 | 3, −1 | 3, −1 |

−7 ÷ 2 is 124 firings under Euclidean and floored and 112 under truncated;
123078 + −4567 is 59 firings, two addition walks and the two subtraction walks
of the simplification, 52 deep (`int_theory.rs`' `the_circuit_of_each_operation`
pins them; `cargo test -p ein-infer --test int_theory the_circuit` re-takes
them). **A call's tag is its request**: `(int-add X Y)` is the tag of its own
computation. **The tagged calls a later theory makes are an interface,** as
[`std.math.decimal-arith`'s](#stdmathdecimal-arith-school-arithmetic-computed-by-saturation-alone)
are, held to these shapes by the same test:

```text
(int-norm TAG P N)                 asked: P less N, in normal form     answered by (int-done TAG INTEGER)
(int-add-call TAG P1 N1 P2 N2)     asked: P1 less N1, plus P2 less N2  answered by (int-done TAG INTEGER)
(int-mul-call TAG P1 N1 P2 N2)     asked: P1 less N1, times P2 less N2 answered by (int-done TAG INTEGER)
(int-cmp-call TAG P1 N1 P2 N2)     asked: P1 less N1, against P2 less N2
                                   answered by std.math.decimal-arith's (order-is TAG O)
```

Each part is a natural number of `std.math.decimal-arith`, written out.

**Every rule has one premise,** which is the rule of the theory in its
strongest form: none reads a table itself and none joins two facts.
`ein-infer/tests/int_theory.rs` holds that on the text, and holds the answers
to Rust's signed integers: every pair of integers written with parts from 0 to
9 under every operation, and samples at every length to 30 digits in every
combination of signs, the three divisions among them, each against `i128`:
`div_euclid` and `rem_euclid`, `/` and `%`, and the floored pair computed from
those. `arith_batch.rs` holds a batch to cost in proportion to its count.

**The names it occupies** are in the module's header: `int`, the nine requests
and nine results, the four calls and `int-done`, a division's hand-over
`int-div-done`, its tags, the signs `pos` and `neg`, `int-rule` and
`int-arith`; everything of `std.math.decimal-arith` comes with it.

## `std.math.rat`: rational numbers, reduced on request

A rational is `(rat Z D)`, an integer of `std.math.int` over a natural number
of `std.math.decimal-arith` that is not zero. The module is written over the
theories below it, which it calls and does not edit, and is computed by
saturation alone: no candidate, R {∅}, and no `absent`, no `not` and no guard
of its own (a program that imports it holds `std.math.decimal-arith`'s one
`absent`, and two strata).

```lisp
(import std.math.rat :symbols (rat-arith))

(rat-add (rat (int (n z 1) z) (n z 2)) (rat (int (n z 1) z) (n z 3)))     ; 1/2 + 1/3
(rat-add (rat (int (n z 1) z) (n z 2)) (rat (int (n z 1) z) (n z 2)))     ; 1/2 + 1/2
(rat-simplify (rat (int (n z 4) z) (n z 4)))                              ; 4/4 in lowest terms

(query :goal (rat-sum-of ?x ?y ?s))      ; 5/6, and 1/2 + 1/2 = (rat (int (n z 4) z) (n z 4))
(query :goal (rat-simple-of ?x ?s))      ; ?s = (rat (int (n z 1) z) (n z 1))
```

**An answer is not canonical; ask `rat-simplify` for lowest terms and
`rat-cmp` for equality**
([Q-M1f.183](../docs/decisions/stdlib/math/rat/Q-M1f.183-a-rational-is-reduced-only-on-request.md)).
An operation answers the cross products, or the numerators' product, over the
product of the denominators, as computed: 1/2 + 1/2 is 4/4, and two equal
results are two different facts. A rule that wants *the same number* asks
`rat-cmp`, which compares by cross products, or `rat-simplify` of each. A chain
of operations grows, the denominators multiplying at every step, unless the
program asks `rat-simplify` on the way.

| | |
|---|---|
| **the rational** | `(rat Z D)`. 1/2 is `(rat (int (n z 1) z) (n z 2))`, −3/4 is `(rat (int z (n z 3)) (n z 4))`. **An answer is unreduced**: its numerator in `std.math.int`'s normal form, one part `z`, its denominator with no high zero ([Q-M1f.302](../docs/decisions/stdlib/math/rat/Q-M1f.302-an-unreduced-answer-has-its-numerator-in-normal.md)); a common factor of the two stays, and zero is 0 over any denominator, 0/6. **Lowest terms**, the two with no common divisor but 1 and zero written 0 over 1, `(rat (int z z) (n z 1))`, is the one normal form every rational has, and `rat-simplify` answers it. An operand is read by value, 2/4 as 1/2 |
| **the protocol** | `(rat-simplify X)` answered by `(rat-simple-of X S)`, X in lowest terms, the one request that reduces; `(rat-neg X)` by `(rat-neg-of X S)`; `(rat-add X Y)` by `(rat-sum-of X Y S)`; `(rat-sub X Y)` by `(rat-diff-of X Y S)`; `(rat-mul X Y)` by `(rat-product-of X Y S)`; `(rat-div X Y)` by `(rat-quot-of X Y S)`, **one rational, exact, and no remainder**; `(rat-cmp X Y)` by `(rat-cmp-of X Y O)`, O one of `lt`, `eq`, `gt`, by value ([Q-M1f.180](../docs/decisions/stdlib/math/rat/Q-M1f.180-the-names-of-std-math-rat-carry-the-prefix-rat.md)) |
| **an operand** | a rational written out, `(rat (int P N) D)`, or a name a `(let NAME (rat (int P N) D))` states. An integer or a natural number is **not** a rational operand, and is not flagged ([Q-M1f.181](../docs/decisions/stdlib/math/rat/Q-M1f.181-a-rational-operand-is-a-rational-written-out.md), [Q-M1f.277](../docs/decisions/stdlib/math/Q-M1f.277-an-operand-of-another-kind-of-number-is-answered.md)). An undeclared name and a digit as an operand are flagged on the request, by the module's three rules from its tags to the request; a digit or a named constant where the integer or the denominator goes, `(rat 7 (n z 2))`, is the rational's own fault, `(not-rational RATIONAL THING)`, flagged where it is written ([Q-M1f.276](../docs/decisions/stdlib/math/Q-M1f.276-an-integers-and-a-rationals-own-shape-is-checked.md)), as in `std.math.int` |
| **a zero denominator** | **a division by zero, and a contradiction**, as in `std.math.decimal-arith`, and by `std.math.arith`'s rule ([Q-M1f.182](../docs/decisions/stdlib/math/rat/Q-M1f.182-a-zero-denominator-is-a-division-by-zero.md)). A request any operand of which has a denominator of zero, however the zero is written, and a `rat-div` by a rational that is zero, derive `(division-by-zero REQUEST)`, the request as written, and `std.math.arith`'s `division-by-zero-is-false` derives `(false)`. Root is dead and R is empty |
| **the one entry** | `rat-arith`, the closing rule: it closes every relation of the module and `nz`, names every rule, and names `int-arith`, `decimal-arith`, `decimal`, `arith` and `gcd`, the entries of the modules it imports |

**How it computes.** The operands are looked up and each denominator is
compared with zero by `std.math.decimal-arith`'s walk. A sum of P1 less N1
over D1 and P2 less N2 over D2 is the **cross products**, P1·D2 + P2·D1 less
N1·D2 + N2·D1, over D1·D2: five products and two sums of natural numbers; a
difference is the same with the second numerator's parts exchanged. **A
comparison is a difference's numerator and nothing more**: its positive part
against its negative part, since the denominators are positive. A product is
the numerators' product, `std.math.int`'s, over the denominators'; a quotient
is a product with the divisor turned over, its sign kept in the numerator, and
a divisor that is zero has no turning over. **Nothing is divided**: the
numerator is simplified by `std.math.int` and the denominator walked to normal
form, or is a product, which has none. **Lowest terms, on request**:
`rat-simplify` simplifies the numerator, asks `std.math.gcd`'s tagged call,
`(gcd-call TAG A D)`, for the greatest common divisor of its size and the
denominator, Euclid's chain, and divides both by what it finds, two divisions
of `std.math.decimal-arith`; the module holds no chain of its own
([Q-M1f.303](../docs/decisions/stdlib/math/rat/Q-M1f.303-lowest-terms-hands-the-numerator-to-the-greatest.md)).
It needs no division of integers. A call's tag is its request, as in
`std.math.int`. **The tagged calls a later theory makes are an interface,**
held to these shapes by the same test as `std.math.decimal-arith`'s; the
denominators must not be zero:

```text
(rat-norm TAG P N D)                  asked: P less N over D, the numerator in normal form and
                                      D with no high zero, not reduced
                                      answered by (rat-done TAG RATIONAL)
(rat-mul-call TAG P1 N1 D1 P2 N2 D2)  asked: P1 less N1 over D1, times P2 less N2 over D2, not reduced
                                      answered by (rat-done TAG RATIONAL)
(rat-cross TAG P1 N1 D1 P2 N2 D2)     asked: P1 × D2 + P2 × D1, and N1 × D2 + N2 × D1
                                      answered by (rat-cross-is TAG POSITIVE NEGATIVE)
```

**Every rule has one premise.** `ein-infer/tests/rat_theory.rs` holds that on
the text, and holds the answers to exact fractions **by value**,
cross-multiplied against the oracle, each in the module's form: every pair of
the 28 fractions with a numerator from −3 to 3 over a denominator from 1 to 4
under each of the five operations, samples to twelve digits, and operands
spelt with a common factor, an unsimplified numerator and high zeros; every
answer of the four operations over the 28 held to its unreduced spelling, as
computed; and `rat-simplify`'s alone to lowest terms. Two requests of one
value answer two facts, and `rat-cmp` says `eq` of them.

**What it costs.** One request, in firings and depth, the closing rules'
left out (`rat_theory.rs`' `the_circuit_of_each_operation` pins them; `cargo
test -p ein-infer --test rat_theory the_circuit` re-takes them):

| request | firings | depth |
|---|---:|---:|
| 2/4 in lowest terms | 345 | 343 |
| 1071/462 in lowest terms, a chain of three divisions | 888 | 883 |
| −3/4 negated | 25 | 23 |
| 1/2 + 1/3 | 101 | 99 |
| 1/2 − 1/3 | 104 | 102 |
| 2/3 × 3/4 | 81 | 79 |
| 1/2 ÷ −3/4 | 91 | 87 |
| 1/2 against 1/3 | 76 | 76 |
| 123/456 + 789/1011 | 386 | 379 |
| 123/456 against 789/1011 | 270 | 270 |

Lowest terms is most of a `rat-simplify`, and most of that is
`std.math.decimal-arith`'s division computing its divisor's nine multiples
before its first digit, on every step of the chain and on the two divisions
after it. An operation divides nothing, and a comparison reduces nothing.
`arith_batch.rs` holds a batch to cost in proportion to its count.

**Nothing limits the length.** Cross products are as long as their operands
together, and nested numbers hold them: two fractions of twelve-digit terms
are added and compared in `rat_theory.rs`. What limits a run is what limits
`std.math.decimal-arith`'s division, the size of the state
(§ [How long a number may be](#how-long-a-number-may-be)).

**The names it occupies** are in the module's header: `rat`, the seven
requests and seven results, the three calls and their two hand-overs, the
circuit's own relations, the signs `pos` and `neg`, its tags, `rat-rule` and
`rat-arith`; everything of `std.math.int` and `std.math.decimal-arith` comes
with it.

## `std.math.gcd`: the greatest common divisor and the least common multiple

A theory over `std.math.decimal-arith`, on its natural numbers, which it calls
and does not edit
([Q-M1f.336](../docs/decisions/stdlib/math/gcd/Q-M1f.336-gcd-and-lcm-are-a-module-of-their-own-over.md));
computed by saturation alone, no candidate, R {∅}, and no `absent`, no `not`
and no guard of its own (a program that imports it holds
`std.math.decimal-arith`'s one `absent`, and two strata).

```lisp
(import std.math.gcd :symbols (gcd lcm gcd-of lcm-of))
(import std.math.decimal :symbols (n))
(import std.math.arith :symbols (z))

(gcd (n (n (n (n z 1) 0) 7) 1) (n (n (n z 4) 6) 2))     ; gcd(1071, 462)
(lcm (n (n z 1) 2) (n (n z 1) 8))                       ; lcm(12, 18)

(query :goal (gcd-of ?x ?y ?g))      ; ?g = (n (n z 2) 1), 21
(query :goal (lcm-of ?x ?y ?l))      ; ?l = (n (n z 3) 6), 36
```

| | |
|---|---|
| **the protocol** | `(gcd X Y)` answered by `(gcd-of X Y G)`; `(lcm X Y)` by `(lcm-of X Y L)`. The operands are repeated as written; every answer is in normal form |
| **an operand** | a natural number of `std.math.decimal-arith`, high zeros allowed and read by value, or a name a `(let NAME NUMBER)` states. **Natural numbers only**: an integer or a rational where a number goes is answered by silence. An undeclared name and a digit as an operand are flagged on the request, by the module's two rules from its tags to the request, as `std.math.int`'s are; a malformed numeral is `std.math.decimal`'s, where it is written |
| **zero** | gcd(X, 0) = gcd(0, X) = X, and **gcd(0, 0) = 0**, the usual convention ([Q-M1f.299](../docs/decisions/stdlib/math/gcd/Q-M1f.299-the-greatest-common-divisor-of-zero-and-zero-is.md)); lcm with a zero is zero, lcm(0, 0) too. **No request of this module divides by zero**: the chain divides only by a number whose shape says it is not zero, and the least common multiple divides by the greatest common divisor only where that is not zero |
| **the one entry** | `gcd`, the closing rule, which has the request's name: it closes every relation of the module, names every rule, and names `decimal-arith`, `decimal` and `arith`, the entries of the modules it imports |

**How it computes.** The operands are looked up by `std.math.decimal-arith`'s
`operand`; each is put into normal form by an addition walk to zero, so that
zero has one shape, `z`
([Q-M1f.301](../docs/decisions/stdlib/math/gcd/Q-M1f.301-the-greatest-common-divisor-puts-each-number.md));
then **Euclid's chain**, gcd(A, B) = gcd(B, A mod B) until the remainder is
zero, each step one tagged division, the tag carrying what is left of the
chain. Where A is the smaller the first step's quotient is zero and the chain
turns the two round. The least common multiple is the product divided by the
greatest common divisor, one `mul-call` and one `div-call` more. **The tagged
call a later theory makes is an interface**
([Q-M1f.300](../docs/decisions/stdlib/math/gcd/Q-M1f.300-the-greatest-common-divisors-tagged-call-is.md)),
held to its shape by the same test as `std.math.decimal-arith`'s;
`std.math.rat`'s `rat-simplify` calls it:

```text
(gcd-call TAG X Y)                    asked: the greatest common divisor of X and Y
                                      answered by (gcd-done TAG G)
```

X and Y are numbers, high zeros allowed and never a name; G is in normal form.

**Every rule has one premise.** `ein-infer/tests/gcd_theory.rs` holds that on
the text, and holds the answers to Rust's integers: both requests over every
pair from 0 to 30, operands of one to nineteen digits and pairs with a common
factor of twelve, the neighbours of the Fibonacci numbers, zero in each place,
equal operands, high zeros and names, and no division by zero in any of them.

**What it costs.** One request, in firings and depth (`gcd_theory.rs`'
`the_circuit_of_each_request` pins them; `cargo test -p ein-infer --test
gcd_theory the_circuit -- --nocapture` re-takes them):

| request | firings | depth |
|---|---:|---:|
| gcd(1071, 462), a chain of three divisions | 553 | 553 |
| gcd(462, 1071), four: the first turns the two round | 742 | 742 |
| lcm(1071, 462) | 866 | 866 |
| gcd(5, 6) | 257 | 257 |
| gcd(7, 0) | 17 | 17 |
| gcd(0, 0), and lcm(0, 0) | 14 | 14 |
| gcd(102334155, 63245986), F(40) and F(39): 38 divisions | 10 337 | 10 337 |

A step is a long division, and most of it is the divisor's nine multiples,
computed before the first digit. The count of steps is linear in the
operands' length (Lamé: at most five a decimal digit of the smaller).
`arith_batch.rs` holds a batch to cost in proportion to its count.

**The names it occupies** are in the module's header: the two requests and
their results, the call and its hand-over, `gcd-chain`, its tags, `gcd-rule`
and the entry `gcd`; everything of `std.math.decimal-arith` comes with it.

## `std.bool`: the sixteen functions of two inputs, and circuits

Four modules under `bool/`
([Q-M1f.328](../docs/decisions/stdlib/bool/Q-M1f.328-boolean-functions-and-circuits-ship-as-four-std.md);
[`04-stdlib.md`](../docs/ein/user/04-reference/04-stdlib.md#boolean-functions-and-circuits-stdbool)
has every relation). The two bits are the names `0` and `1`, which are
decimal's digits too: Ein has no truth values of its own, as it has no
numbers. Each module comes whole with what it imports, and each has an entry,
its closing rule, that closes its relations and names its rules and the entry
below it
([Q-M1f.265](../docs/decisions/stdlib/bool/Q-M1f.265-each-std-bool-module-is-entered-by-its-last-name.md)):

    std.bool.gates  ←  std.bool.props
                    ←  std.bool.circuit  ←  std.bool.inputs  →  std.algebra

Nothing but `std.bool.inputs` guesses, and it guesses only an input's value.
No rule of the four writes `absent`, `not` or `or`;
`ein-infer/tests/bool_theory.rs` holds that on their text, and every fact of
the tables and the property rules to an enumeration of the sixteen columns in
Rust. The applications, six of them, are
[`examples/binary_logic/`](../examples/binary_logic/README.md).

### `std.bool.gates`: the table and a relation a function

A function of two bits is its column of four outputs, and there are sixteen,
each with a name: `Zero And Andn Fst Randn Snd Xor Or Nor Xnor Nsnd Orn Nfst
Rorn Nand One`, capitalised because `and`, `or` and `not` are Ein's own words.
Two layouts
([Q-M1f.325](../docs/decisions/stdlib/bool/Q-M1f.325-a-boolean-function-is-a-value-of-one-table.md)):

    (gate2 Xor 0 1 1)   the table, 64 rows: a function is a value, so a rule
                        about any function needs no parameter, and a search can
                        make the function a hypothesis
    (Xor 0 1 1)         a relation a function, its four rows, derived from the
                        table by sixteen rules, `Zero-rows` to `One-rows`

with each function's column as a second name, `(column-name Xor f0110)`, and
three small tables the property rules read, `inv`, `le` and `bit`. **The
table's rows are generated**, between two lines of the file that say so, and
not edited by hand; the rest is written by hand, the sixteen rules among it.

**A function's name is the module's.** Declared as a relation, each name has
one full name, `std.bool.gates.Xor`, as the relation and as the value in the
table, so a program that writes it, as a gate's function too, lists it:
`(import std.bool.gates :symbols (Xor And))`. A name a program writes and does
not list is its own, a name no row has: a gate of that function computes
nothing, and nothing says so
([Q-M1f.264](../docs/decisions/imports/Q-M1f.264-a-gate-functions-name-a-program-writes-and-does.md)).
Every program that imports the module derives the second layout, 64 firings at
root: an import brings every rule, so the sixteen are not a program's to
select
([Q-M1f.263](../docs/decisions/stdlib/bool/Q-M1f.263-every-program-that-imports-std-bool-gates.md)).

### `std.bool.props`: what is known of the sixteen, by rules

Thirty relations, every fact derived from the 64 rows by monotone rules, in a
module of its own
([Q-M1f.326](../docs/decisions/stdlib/bool/Q-M1f.326-the-properties-of-the-sixteen-functions-are.md)):
a program that imports the module pays 405 firings at root, beside the table's
65, and one that does not pays nothing (`ein saturate` on a program that
imports it and writes nothing else). They are each function's column; five
relations between two functions, `swap-of`, `complement-of`, `dual-of`,
`left-negated` and `right-negated`; `commutative`, `associative`,
`idempotent`; which inputs a function reads or ignores; its identity and
absorbing bits, one-sided and two-sided
([Q-M1f.327](../docs/decisions/stdlib/bool/Q-M1f.327-identity-and-absorbing-elements-are-derived-one.md));
and Post's five classes from inside and from outside, with `complete-alone`,
Nand and Nor. *For every input* is a conjunction of a function's four rows and
*not in a class* one witness, so no rule needs `absent`. The programs under
`tests/stdlib/bool/props/` claim the facts, generated from the columns. The
module's header lists each relation with its definition.

### `std.bool.circuit`: a circuit as a program

`(gate INSTANCE FUNCTION IN1 IN2 OUT)` and `(val WIRE BIT)`: wires are names,
a gate is a fact, and `propagate` carries values from a gate's inputs to its
output through the table, so saturation is simulation, a firing a gate.
`one-value` makes a wire that carries both bits a contradiction.

### `std.bool.inputs`: inputs as hypotheses

`(is-a WIRE Input)`, written; `(in WIRE BIT)`, the hypothesis; `(require
WIRE BIT)`, written. An input owes exactly one value, by `std.algebra`'s
`(total-owed in is-a)` and `(functional in)`, and the obligation is what
proposes the candidates, two an input, through the obligations rung. R is the
input assignments that contradict nothing, so a truth table is a query's
classes and satisfiability is R with requirements; an unsatisfiable question
is R empty because every alive node owes. It is a demonstration, not a SAT
solver.

## What tests these modules

[**`tests/stdlib/`**](../tests/README.md): one program per rule or tight
family (`find tests/stdlib -name '*.ein' | wc -l`), each stating what it
should and should not derive. `ein test tests/stdlib/` runs the lot and exits
1 if a claim is false. The obligation duals have a firing and a satisfied
program each, and `bijection/06_blind_enumeration.ein` leaves the blind
enumerator on, where the other programs of its directory turn it off.

**Every rule is activated by a program**, and that is measured, not read off
the directory: `ein-infer/tests/stdlib_coverage.rs` solves every program under
`tests/stdlib/` with `--events` on and fails on any rule none of them
activated (`every_stdlib_rule_is_activated_by_a_program`). The rules it counts
are every depth's, `find stdlib -name '*.ein' -exec cat {} + | grep -c '^(rule '`
(`cat stdlib/*.ein` would read only the top level).

**"Activated" is two events**, and the second is not an optimisation of the
first. A saturation rule reaches the agenda and emits `fire`. An
**obligation** rule, one whose `:assert` is the verdict atom `open`, never
can: it derives nothing and is kept out of the agenda, so its evidence is the
`owe` its post-fixpoint pass emits
([`08-obligations.md`](../docs/ein/user/03-syntax-and-semantics/08-obligations.md#the-form-open-r)).
The test reads both kinds; counting only `fire` would leave the four duals
permanently unactivated.

Adding a rule here therefore means adding a program there, and the test says
so rather than a README. It is the corpus's growth rule, one level up: a rule
with no activating program is not tested, it is merely not contradicted.
