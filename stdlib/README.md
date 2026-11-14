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

## The layout (M1f S1f.6.31)

A module's name is its path under this directory, a dot a directory, and it
resolves the same way from each of the three places above: `std.algebra` is
`algebra.ein`, `std.math.decimal-arith` is `math/decimal-arith.ein`. A full
name is the module's name and the short name, `std.math.decimal-arith.add`
(M1f S1f.6.35). `MANIFEST.sha256` lists every module by that path, at any
depth, and `ein-ir`'s `the_checkout_has_no_module_the_manifest_does_not_list`
and `ein-infer`'s `the_inventory_reads_every_module_the_manifest_lists` fail on
a `.ein` here that the manifest or the coverage gate does not see.

The calculator is under `math/` (the user's replies 16, 38, 42 and 52 to
P1f.6's questions, and round 2's layout, 2026-11-12); the zebra modules stay at
the top; the suite mirrors it, `tests/stdlib/math/<module>/`.

| module | file | what it is | entry |
|---|---|---|---|
| `std.math.digits` | [`math/digits.ein`](math/digits.ein) | the decimal tables, facts and no rule, generated ([§](#stdmathdigits-a-library-of-facts-m1f-s1f62)) | none |
| `std.math.decimal-arith` | [`math/decimal-arith.ein`](math/decimal-arith.ein) | natural numbers of any length, `(n PREFIX DIGIT)` ([§](#stdmathdecimal-arith-school-arithmetic-computed-by-saturation-alone-m1f-p1f6)) | `decimal-arith` |
| `std.math.int` | [`math/int.ein`](math/int.ein) | integers over it, `(int P N)` ([§](#stdmathint-integers-as-a-positive-and-a-negative-part-m1f-s1f67)) | `int-arith` |
| `std.math.rat` | [`math/rat.ein`](math/rat.ein) | rationals over both, `(rat Z D)` ([§](#stdmathrat-rational-numbers-in-lowest-terms-m1f-s1f68)) | `rat-arith` |
| `std.math.num9` | [`math/num9.ein`](math/num9.ein) | nine digits in one fact, by wide rules, generated ([§](#stdmathnum9-nine-digits-in-one-fact-and-wide-rules-m1f-s1f613)) | `arith9` |
| `std.algebra`, `std.bijection`, `std.closure`, `std.elim`, `std.macro`, `std.slots`, `std.typing` | the top level | the relation algebra, the puzzle libraries and the macros ([§ Modules](#modules)) | |

Until S1f.6.31 the five were `std.digits`, `std.arith` (entry `arith`),
`std.int`, `std.rat` and `std.num9`, at the top. No alias is kept: an old name
is the loader's *module not found*. No relation was renamed.

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

## `std.math.digits`: a library of facts (M1f S1f.6.2)

The first module here that holds **facts and no rule**: the *ontology* half
of
[`solution_semantics.md` § 1.1](../docs/kernel/inference/solution_semantics.md#11-context-and-problem),
where every other module is theory. Ein has no numbers and no arithmetic (the
user's ruling of 2026-11-06), so the ten digits `0` to `9` are plain names,
and what school arithmetic knows about them is written down once, as tables,
the way hardware holds it. The arithmetic theories (`std.math.decimal-arith`, P1f.6's
S1f.6.3 onward, and `std.math.num9`) are the wiring; every step of theirs is a
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
(import std.math.digits :symbols (fa nz __closed__))
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
the ones it reads in its own closing rule, as `std.math.decimal-arith` does, and its
importer lists nothing of this module.
[`tests/stdlib/math/digits/01_tables.ein`](../tests/stdlib/math/digits/01_tables.ein)
is the program that holds it: every table imported, a row of each claimed,
and R = {∅} claimed on its `(solve …)` form.

**What it costs.** A program that does not import the module pays nothing: a
module is read when an import names it. One that does pays the parse of the
whole file, 28.8 KB, whichever tables it lists: `ein test
tests/stdlib/math/digits/01_tables.ein` takes 19 ms where a `std.macro` test takes
6 ms (best of ten, release, 2026-11-11). **A core that passes through a table
names the row**, since a row is a premise like any other fact:
[`tests/stdlib/math/digits/02_a_core_names_its_rows.ein`](../tests/stdlib/math/digits/02_a_core_names_its_rows.ein)
refutes `(column 7 5 3)` and its unsat core is that fact and `(fa 7 5 0 2 1)`.

## `std.math.decimal-arith`: school arithmetic, computed by saturation alone (M1f P1f.6)

A theory written **in** Ein over the ten digit names: `std.math.digits`' tables are
the gates and this module's rules are the wiring. Every step is a table lookup
whose outcome the table decides, so nothing is guessed: on a program that
imports the theory and states requests the universe has no candidate, R is
{∅}, and the answer is root's fixpoint under `ein solve`, `ein saturate` and
`ein test` alike. No `not` premise, and **one `absent`, over `let`**, which a
program writes and no rule derives (S1f.6.12; none until then): the theory
closes `let`, so the premise is decided at root, the static check calls a
program that imports the theory monotone, and it has two strata. Three rules
conclude `(false)`, each from a flag: a division by zero, a malformed
request, an undeclared name. Ein still has no
numbers (the user's ruling of 2026-11-06): nothing in the engine knows what
`7` is.

**Addition** (S1f.6.3), **subtraction and comparison** (S1f.6.4),
**multiplication** (S1f.6.5) and **division** (S1f.6.6): the four operations
and comparison; the check of what a program wrote (S1f.6.12); and the
division by zero flagged at the call (S1f.6.24): 115 rules and the closing
rule (`grep -c '^(rule ' stdlib/math/decimal-arith.ein`, less one, 2026-11-13).

```lisp
(import std.math.decimal-arith :symbols (decimal-arith))

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
| **division** | `(div X Y)` is answered by `(quot-of X Y Q R)`, the quotient and the remainder: X = Q × Y + R, and R less than Y |
| **division by zero** | a contradiction, **in two steps** (the user's ruling of 2026-11-11): a request whose divisor is zero, however written, derives the flag `(division-by-zero (div X Y))`, and the flag derives `(false)`. Root is dead, R is empty, and no query of the program has an answer; `ein solve` names the request in the unsat core and `division-by-zero-is-false` as what refuted it. A program that must not die compares its divisor with zero first and derives its `div` request by a rule over `(cmp-of Y z gt)`. **It is flagged wherever it is called** (S1f.6.24, the user's ruling of 2026-11-12: *"(div ?x 0) must emit division-by-zero no matter where it happens"*): every division goes through the tagged call `(div-call TAG X Y)`, and a call whose Y is zero by value derives `(division-by-zero (div-call TAG X Y))`, the call as it was asserted, and `(false)` from it, whoever made the call, a rule of a program's own included, which until that stage derived nothing. A public request therefore holds two flags, its own and its call's under the tag `(quotient X Y)`; `ein solve` reads as it did, and `--trace` and `ein saturate --dump` show the second flag. `std.math.int`, `std.math.rat` and `std.math.num9` see a zero divisor before they call, so theirs is one flag, the request's. **A theory in which division by zero is allowed** (a wheel, a meadow, transreal arithmetic) would be a module of its own that does not import the flag's rules; none is built |
| **named constants** | `(let NAME NUMBER)` names a number, and a request may name an operand: `(add price qty)` is answered by `(sum-of price qty S)`. The same sum written with the numbers is another request with its own answer. A name stands for one number: a name no `let` states, a name stated twice with two numbers, and `z` or a digit as a name are each flagged (below) |
| **a malformed request** | a contradiction, **in two steps**, as a division by zero is (S1f.6.12, the user's rulings of 2026-11-11 and 2026-11-12). `(malformed REQUEST)`, the request as written, where an operand is not a number; `(malformed (let NAME NUMBER))` where a `let` cannot be read; `(undeclared NAME REQUEST)` where the request names an operand no `let` states. Each flag derives `(false)` by a rule of its own. Root is dead, R is empty, every command exits 0, and `ein solve` names the request in the unsat core. **A flag means malformed; no flag does not mean well-formed** ([§ Malformed requests](#malformed-requests-what-is-flagged-and-what-stays-silent-m1f-s1f612)) |
| **the one entry** | `decimal-arith`, the closing rule: it names every rule of the theory and closes every relation of it and each table it reads, so one name brings all of it and leaves nothing of it to the search. A relation the importing program declares is the program's to close |
| **the tagged calls** | what a theory built on the calculator calls, below: an interface, held to its shapes by a test |

**The tagged calls, an interface** (M1f
[S1f.6.30](../plans/m1f_hypothesis_and_documentation/p1f.6_calculator/s1f.6.30_name_clashes_between_imported_theories.md)'s
T1; the user's reply *"(a)"* of 2026-11-12 to
[round 2's item 41](../plans/m1f_hypothesis_and_documentation/p1f.6_calculator/questions_round2.md#q41)).
A result repeats its request's operands, so a rule of a program that wants an
answer for a purpose of its own must find it by its operands, a join on a
number, which the engine answers by a scan (E-3). Inside, the theory calls
itself with a **tag**, a nested fact that names the caller, and hands the
answer back under it, so the rule that reads has one premise. Those calls are
what `std.math.int`, `std.math.rat`, the binary conversions and the programs under
`examples/beyond_arith/` call:

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
adds its own values (`std.math.int` answers `(operand TAG (int P N))` itself, so an
integer stands for itself and a name goes through `let`). A walk starts with `z`, no
digits yet; a subtraction with `0`, no borrow, and a comparison with `eq`,
nothing yet decided; those are the values a caller writes. An addition's
carry in is the caller's: `0` for X + Y, or a bit, as
[`conv.ein`](../examples/binary_arithmetic/conv.ein) doubles a number and adds
a bit in one walk, `(add-walk TAG X X BIT z)`. The tag is a fact of the caller's own
relation, `(cnt K M)` in
[`casting_out_nines.ein`](../examples/beyond_arith/casting_out_nines.ein); a
tag of the theory's own, `(sum X Y)` or `(quotient X Y)`, is read back as one
of its requests (E-39). Checking the laws of arithmetic over the digits took
16.8 s through requests and 0.42 s through these calls (`build/s1f.6.10/gen4.py`
against `gen4t.py`, 2026-11-12).
[`arith_theory.rs`](../ein.rs/crates/ein-infer/tests/arith_theory.rs)'s
`the_tagged_calls_keep_their_shapes` holds the declarations of the eleven
relations, and of `std.math.int`'s and `std.math.rat`'s calls below, to these shapes;
holds each module's header and this page to naming them as written here; and
reads every module of the stdlib and every program under `examples/` and
`tests/` that brings the theory and writes one of them, so a call written
with another count of arguments, or a walk started from another place, fails
per commit.

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

**Division is school long division, the quotient's digit by bisection.** The
divisor's multiples by 1 to 9 are computed once a division, each a row of
multiplication, in a chain whose tags hold the multiples already known, and
travel on as one fact, `(multiples M1 … M9)`; the first, the divisor times 1,
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
by the digit alone. 127645 ÷ 25 is 220 firings, one chain
(`the_circuit_of_one_division`); 4 000 divisions of numbers of up to nine
digits saturate in 2.6 s (`python3 build/s1f.6.4/batch.py div`, 2026-11-11).

**The counts above are each circuit's own chain, and the times are of
2026-11-11.** Since S1f.6.12 a request also checks its operands beside the
chain, a firing a digit of each operand that is found: 123078 + 4567 is 31
firings and still 21 deep, the subtraction 38, the comparison 23, 123 × 456
is 78 and 127645 ÷ 25 is 228 (the same tests pin both numbers), and the
batches take what [§ What a program
pays](#what-a-program-pays-the-tables-the-parse-the-calculation) says.

**The rule of the theory: no rule joins two facts on a number.** A rule has
one premise that holds numbers; its other premises are rows of `std.math.digits`
looked up by digits. The engine's fact index does not key on a nested fact
(E-3 of the phase's
[`findings.md`](../plans/m1f_hypothesis_and_documentation/p1f.6_calculator/findings.md)),
so a join on a number scans a relation for every fact of the other and a
batch costs the square of its count; written this way it costs in proportion.
Two tests hold it, and **whoever adds a rule keeps to it**:
[`ein-infer/tests/arith_theory.rs`](../ein.rs/crates/ein-infer/tests/arith_theory.rs)
on the module's text, and
[`arith_batch.rs`](../ein.rs/crates/ein-infer/tests/arith_batch.rs) on the
matcher's own count of its work, under `--features counters`. The first is in
the gate; the second is **run by hand** since M1f S1f.6.21 (`cargo test -p
ein-infer --features counters --test arith_batch`), by whoever changes a rule
of the calculator, and no tier runs it.
The stated exceptions join on a *name*: `operand-name`, an operand with its
`let`, behind a `(named ?x)` check that keeps a number operand from walking
the `let` facts; and, since S1f.6.12, `let-twice`, two `let`s of one name.
Since that stage a rule may also hold guards, `eq` and `neq`, which compare
and look nothing up, in three named rules, and a probe, `(nz ?m)` or
`(named ?m)`, one lookup whatever `?m` is; and `arith_batch.rs` holds a
second count beside the matcher's, **the values the guards were decided
for**, which the first does not see (E-27 of the phase's `findings.md`).

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
and [`div_by_zero.ein`](../examples/arith/div_by_zero.ein); the three generated
examples are the next section's.

### What a program pays: the tables, the parse, the calculation

The user, 2026-11-11: *"add examples with long numbers and many calculations
so we can compare load time with calculation time"* (M1f
[S1f.6.15](../plans/m1f_hypothesis_and_documentation/p1f.6_calculator/s1f.6.15_load_against_calculation.md)).
`ein saturate FILE` prints three times of its own: `parse:`, the program's
text; `kb load:`, macros, imports and facts; and `saturate:`, the calculation.
Three generated examples make a program long in three ways,
[`long_numbers.ein`](../examples/arith/long_numbers.ein),
[`many_calculations.ein`](../examples/arith/many_calculations.ein) and
[`factorial.ein`](../examples/arith/factorial.ein), and one script measures
them and a sweep of sizes past them. Everything below the next line is its
output, taken on 2026-11-13 through `utils/bench_env.sh` on one P-core of an
i9-14900HX under the `powersave` governor (`utils/bench_env.sh python3
utils/arith_cost.py`; `--quick` is the first two sections). It was first
taken on 2026-11-11 and re-taken when S1f.6.12 added the check of each
operand, which is in every number below.
*wall* is the whole process and *rss* its peak resident memory.

`ein 0.1.0`, best of 5, seed 20261111; `python3 utils/arith_cost.py`

#### What a program pays before its first request

| program | text | parse ms | load ms | saturate ms | firings | facts | rss MB | wall ms |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| no import: one relation | 46 B | 0.92 | 0.12 | 0.01 | 0 | 2 | 17 | 3.94 |
| `std.math.digits` alone, the seven tables | 54 B | 0.92 | 13.9 | 0.01 | 0 | 1 678 | 18 | 18.2 |
| `std.math.decimal-arith`, the theory and the tables | 36 B | 0.90 | 21.5 | 0.31 | 1 | 2 022 | 18 | 26.6 |
| `std.math.decimal-arith` and 17 + 5 | 64 B | 0.91 | 21.4 | 0.46 | 17 | 2 041 | 18 | 26.9 |

#### The three examples

`ein saturate FILE`, the program as text (`add.ein`, one sum and one by names, for scale):

| example | text | parse ms | load ms | saturate ms | firings | facts | rss MB | wall ms |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| `add.ein` | 3 KB | 1.13 | 21.6 | 0.52 | 50 | 2 079 | 18 | 27.1 |
| `long_numbers.ein` | 6 KB | 2.92 | 21.2 | 5.66 | 3 254 | 5 296 | 21 | 34.6 |
| `many_calculations.ein` | 8 KB | 4.55 | 21.2 | 14.7 | 8 120 | 10 460 | 26 | 47.6 |
| `factorial.ein` | 4 KB | 1.27 | 21.1 | 5.49 | 3 087 | 5 286 | 22 | 33.0 |

The same programs saved by `ein kb save FILE OUT.einb`, under `ein saturate OUT.einb`, which opens the container where the text is parsed and loaded:

| example | .einb | open ms | saturate ms | firings | wall ms | wall ms as text |
|---|---:|---:|---:|---:|---:|---:|
| `add.ein` | 218 KB | 22.1 | 0.50 | 50 | 26.5 | 27.1 |
| `long_numbers.ein` | 266 KB | 23.9 | 5.41 | 3 254 | 34.4 | 34.6 |
| `many_calculations.ein` | 238 KB | 25.6 | 14.4 | 8 120 | 47.3 | 47.6 |
| `factorial.ein` | 218 KB | 22.1 | 5.29 | 3 087 | 32.2 | 33.0 |

The whole process, by command:

| example | saturate | solve | test | saturate .einb | solve .einb |
|---|---:|---:|---:|---:|---:|
| `add.ein` | 27.1 | 27.4 | 27.3 | 26.5 | 27.4 |
| `long_numbers.ein` | 34.6 | 83.3 | 83.0 | 34.4 | 84.0 |
| `many_calculations.ein` | 47.6 | 96.9 | 96.3 | 47.3 | 96.3 |
| `factorial.ein` | 33.0 | 52.4 | 52.2 | 32.2 | 52.0 |

#### One request, by the length of its operands

A dividend has twice the digits of its divisor. A written number nests a level a digit, and the parser nests to 1 000, so 900 digits is near the longest number a program can write.

| request | text | parse ms | load ms | saturate ms | firings | facts | rss MB | wall ms |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| add, 9 digits | 154 B | 0.96 | 21.1 | 0.48 | 46 | 2 070 | 20 | 26.4 |
| add, 30 digits | 406 B | 1.12 | 21.1 | 0.59 | 130 | 2 154 | 19 | 26.8 |
| add, 100 digits | 1 KB | 1.62 | 21.2 | 0.92 | 410 | 2 434 | 20 | 27.7 |
| add, 300 digits | 4 KB | 3.18 | 21.1 | 1.93 | 1 210 | 3 234 | 22 | 30.3 |
| add, 900 digits | 11 KB | 7.49 | 21.5 | 4.90 | 3 610 | 5 634 | 23 | 38.4 |
| sub, 9 digits | 154 B | 0.97 | 21.0 | 0.51 | 56 | 2 081 | 20 | 26.3 |
| sub, 30 digits | 406 B | 1.12 | 21.1 | 0.64 | 161 | 2 186 | 20 | 26.8 |
| sub, 100 digits | 1 KB | 1.64 | 21.2 | 1.08 | 511 | 2 536 | 20 | 27.7 |
| sub, 300 digits | 4 KB | 3.18 | 21.1 | 2.37 | 1 511 | 3 536 | 20 | 30.7 |
| sub, 900 digits | 11 KB | 7.49 | 21.4 | 6.61 | 4 511 | 6 536 | 23 | 40.5 |
| cmp, 9 digits | 154 B | 0.99 | 21.1 | 0.49 | 35 | 2 059 | 18 | 26.3 |
| cmp, 30 digits | 406 B | 1.14 | 21.2 | 0.56 | 98 | 2 122 | 19 | 26.8 |
| cmp, 100 digits | 1 KB | 1.63 | 21.1 | 0.85 | 308 | 2 332 | 18 | 27.5 |
| cmp, 300 digits | 4 KB | 3.16 | 21.2 | 1.71 | 908 | 2 932 | 20 | 30.2 |
| cmp, 900 digits | 11 KB | 7.46 | 22.5 | 4.58 | 2 708 | 4 732 | 18 | 39.1 |
| mul, 9 digits | 154 B | 1.11 | 21.9 | 1.27 | 508 | 2 532 | 18 | 28.5 |
| mul, 30 digits | 406 B | 1.15 | 21.1 | 7.65 | 4 833 | 6 857 | 23 | 35.1 |
| mul, 100 digits | 1 KB | 1.73 | 21.2 | 82.5 | 51 090 | 53 114 | 67 | 123 |
| mul, 300 digits | 4 KB | 3.30 | 21.2 | 802 | 453 275 | 455 299 | 434 | 972 |
| div, 18 by 9 | 208 B | 1.05 | 21.0 | 2.28 | 1 185 | 3 209 | 19 | 28.6 |
| div, 60 by 30 | 586 B | 1.26 | 21.2 | 16.4 | 9 439 | 11 463 | 29 | 46.3 |
| div, 200 by 100 | 2 KB | 2.18 | 21.2 | 171 | 91 959 | 93 983 | 117 | 229 |
| div, 600 by 300 | 5 KB | 4.57 | 21.4 | 1 563 | 769 555 | 771 579 | 876 | 1 918 |

#### Many requests on nine-digit operands, by their count

| requests | text | parse ms | load ms | saturate ms | firings | facts | rss MB | wall ms | firings a request |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| 100 x add | 12 KB | 7.95 | 21.4 | 6.81 | 4 501 | 6 822 | 23 | 41.0 | 45.0 |
| 1 000 x add | 115 KB | 71.9 | 25.4 | 71.4 | 45 001 | 50 022 | 64 | 184 | 45.0 |
| 10 000 x add | 1.1 MB | 737 | 58.8 | 829 | 450 001 | 482 022 | 451 | 1 782 | 45.0 |
| 100 x sub | 12 KB | 7.97 | 21.4 | 8.25 | 5 501 | 7 922 | 24 | 42.9 | 55.0 |
| 1 000 x sub | 115 KB | 71.4 | 25.4 | 91.6 | 55 001 | 61 022 | 74 | 208 | 55.0 |
| 10 000 x sub | 1.1 MB | 736 | 59.5 | 1 163 | 550 001 | 592 022 | 771 | 2 232 | 55.0 |
| 100 x cmp | 12 KB | 8.00 | 21.5 | 5.79 | 3 401 | 5 722 | 23 | 40.0 | 34.0 |
| 1 000 x cmp | 115 KB | 71.7 | 25.4 | 63.2 | 34 001 | 39 022 | 57 | 174 | 34.0 |
| 10 000 x cmp | 1.1 MB | 736 | 59.0 | 702 | 340 001 | 372 022 | 419 | 1 629 | 34.0 |
| 100 x mul | 12 KB | 8.10 | 21.5 | 81.0 | 50 483 | 52 804 | 64 | 129 | 504.8 |
| 1 000 x mul | 115 KB | 72.6 | 25.4 | 977 | 504 301 | 509 322 | 519 | 1 255 | 504.3 |
| 4 000 x mul | 461 KB | 294 | 36.1 | 4 397 | 2 017 067 | 2 031 088 | 1987 | 5 476 | 504.3 |
| 100 x div | 12 KB | 8.07 | 21.6 | 114 | 60 653 | 62 974 | 83 | 167 | 606.5 |
| 1 000 x div | 115 KB | 71.8 | 25.2 | 1 299 | 606 598 | 611 619 | 702 | 1 711 | 606.6 |
| 4 000 x div | 461 KB | 289 | 36.3 | 5 872 | 2 425 850 | 2 439 871 | 2949 | 7 830 | 606.5 |

#### The factorial chain, by its last number

| chain | text | parse ms | load ms | saturate ms | firings | facts | rss MB | wall ms |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| 25!, 26 digits | 4 KB | 1.29 | 21.1 | 5.38 | 3 087 | 5 286 | 24 | 32.9 |
| 50!, 65 digits | 4 KB | 1.43 | 21.1 | 19.6 | 12 351 | 14 725 | 34 | 49.8 |
| 100!, 158 digits | 5 KB | 1.94 | 21.3 | 93.1 | 54 723 | 57 447 | 68 | 137 |
| 200!, 375 digits | 6 KB | 2.82 | 21.4 | 667 | 352 773 | 356 197 | 395 | 845 |

#### `ein solve` beside `ein saturate`

The same program, the whole process. `ein solve` saturates once, as `ein saturate` does, and then reads the state out. A run past 120 s is not waited for.

| program | firings | facts | saturate wall ms | solve wall ms | solve / saturate |
|---|---:|---:|---:|---:|---:|
| `examples/arith/add.ein` | 50 | 2 079 | 27.0 | 27.5 | 1.0 |
| `examples/arith/long_numbers.ein` | 3 254 | 5 296 | 34.5 | 82.0 | 2.4 |
| `examples/arith/many_calculations.ein` | 8 120 | 10 460 | 46.0 | 94.3 | 2.1 |
| `examples/arith/factorial.ein` | 3 087 | 5 286 | 32.7 | 52.3 | 1.6 |
| add, 100 digits | 410 | 2 434 | 27.7 | 46.4 | 1.7 |
| add, 300 digits | 1 210 | 3 234 | 30.2 | 208 | 6.9 |
| add, 900 digits | 3 610 | 5 634 | 38.8 | 1 937 | 50.0 |
| mul, 30 digits | 4 833 | 6 857 | 34.9 | 144 | 4.1 |
| mul, 100 digits | 51 090 | 53 114 | 126 | 4 072 | 32.4 |
| div, 60 by 30 | 9 439 | 11 463 | 45.0 | 643 | 14.3 |
| div, 200 by 100 | 91 959 | 93 983 | 232 | 20 866 | 90.1 |
| 1 000 x add, nine digits | 45 001 | 50 022 | 187 | 413 | 2.2 |
| 10 000 x add, nine digits | 450 001 | 482 022 | 1 785 | 4 233 | 2.4 |
| 100 x div, nine digits | 60 653 | 62 974 | 165 | 1 115 | 6.8 |
| 1 000 x div, nine digits | 606 598 | 611 619 | 1 707 | 11 441 | 6.7 |
| 50! | 12 351 | 14 725 | 49.9 | 248 | 5.0 |
| 100! | 54 723 | 57 447 | 132 | 2 479 | 18.8 |

**What it says, each with its number from the tables above.**

- **Up to about a hundred requests the tables are the cost.** A program that
  imports the theory pays 21.5 ms of `kb load` before its first request, 13.9
  of it the 1 664 rows of `std.math.digits` and the rest the theory's 116 rules and
  74 relations, where a program with no import loads in 0.12 ms and the whole
  process takes 3.9 ms. One sum adds 0.15 ms of saturation to that.
  `many_calculations.ein`, a hundred requests, is the first size where the
  calculation, 14.7 ms, is of the load's order.
- **A `.einb` does not buy the load back.** `ein kb save` caches a whole
  program with its imports resolved, and `add.ein` saved so opens in 22.1 ms
  where its text parses and loads in 22.7: the container holds the program's
  2 000 facts, and opening it interns each again. It did buy it back while the theory read two
  tables: on the stdlib of `277cfc44`, addition alone, a sum's `.einb` solved
  in 6.7 ms against 18.1 as text (`ein kb save`, `ein solve`, best of 20,
  2026-11-11). A library has no cache of its own (E-15 and E-25 of the
  phase's
  [`findings.md`](../plans/m1f_hypothesis_and_documentation/p1f.6_calculator/findings.md)).
- **For a sum, parsing the program's numbers costs about what adding them
  does.** 10 000 nine-digit sums are 1.1 MB of text: 737 ms to parse and
  829 ms to saturate, where the saturation was 502 ms before S1f.6.12's
  check. A written number is one nested form a digit, the parser's worst
  input so far (E-16). For a product or a quotient the calculation leads from
  the start: 4 000 nine-digit products parse in 294 ms and saturate in 4.4 s.
- **A batch costs in proportion to its count,** as the rule of the theory
  promises: 45.0, 55.0, 34.0, 504 and 607 firings a request for a sum, a
  difference, a comparison, a product and a quotient of nine-digit numbers,
  the same at 100 requests and at 10 000, and about two microseconds a
  firing. **Eighteen of each are the check of the two operands** (S1f.6.12),
  a firing a digit: 27, 37, 16, 486 and 589 without it.
- **By length:** a sum, a difference and a comparison cost a few firings a
  digit, 3 610, 4 511 and 2 708 at 900 digits, under 7 ms each, where the
  number's own text takes 7.5 ms to parse; 1 800 of each are the check. A
  product costs the two lengths multiplied, 453 275 firings and 0.80 s at
  300 digits by 300, and a quotient of 600 digits by 300 is 769 555 firings
  and 1.6 s.
- **A chain needs no text.** `factorial.ein`'s program for 200! is 6 KB and
  parses in 2.8 ms; its 352 773 firings take 0.7 s and end on a number of 375
  digits that no line of the program writes.
- **Memory is a kilobyte a stored fact.** Every step of a circuit is a fact
  that holds its whole state, and nothing is forgotten: 4 000 products are
  2.03 million facts and 2.0 GB resident, and 10 000 were 4.9 million and
  6.0 GB (2026-11-11, the first run of the sweep, which now stops at 4 000
  for a product and a quotient; E-26).
- **`ein solve` writes the state, and the state as text is what explodes.**
  `ein solve` saturates once, as `ein saturate` does, and then reads the state
  out; `ein test` does the same. Stored, a circuit's facts share their
  numbers; written, each fact repeats its own. A 900-digit sum is 38.8 ms
  under `saturate` and 1.9 s under `solve`, where it was 1.0 s before the
  check, whose every fact holds what is left of an operand and the request;
  a 100-digit product 126 ms and 4.1 s; a quotient of 200 digits by 100,
  232 ms and 20.9 s (E-24). It is why the shipped examples are the sizes they
  are: every corpus program is solved, dumped and reloaded per commit.

#### The biggest calculation

The user, 2026-11-12: *"Let's have biggest solvable arithmetic calculation in
let's say 100+-50 seconds on this host. make a generator in python that takes
some size arguments and generate an example"*; told that memory ends a
calculation before time does, *"Ok, then pick the one that fits into memory,
say ~16GB; do not call it in tests and in CI, just for manual solve call"*
(M1f
[S1f.6.29](../plans/m1f_hypothesis_and_documentation/p1f.6_calculator/s1f.6.29_the_biggest_calculation.md)).
`python3 utils/gen_arith_examples.py --op …` writes such a program. It goes
under `build/`, is never checked in, and no test, gate step, nightly step or
dev-host step writes or runs one. `utils/bench_env.sh python3
utils/arith_cost.py --biggest` searched the sizes on 2026-11-13, on cpu4, a
P-core, under the `powersave` governor. Each run was one process in a cgroup
of its own, capped at 16.0 GB with no swap (`systemd-run --user --scope -p
MemoryMax=16000000000 -p MemorySwapMax=0`,
[Q-M1f.227](../plans/m1f_hypothesis_and_documentation/open_questions.md#q-m1f227--how-a-run-of-the-biggest-calculation-is-held-to-about-16-gb-and-how-long-it-may-take))
and killed past 600 s. The search doubled a size, then narrowed to 3 %, and
measured each size once. The binary was the release build at `2ef68530`, with
that commit's stdlib. Five of the sweep's smaller programs, one of each shape
and two chains, give the same firings and facts on the stdlib of
`63955507`. Another stage's agent worked on the machine while
`ein solve`'s half ran. The user stopped the sweep before it ended (*"runs way
too long than I expected - just write down generated examples and close"*), so
the table holds what was measured, and no more.

| command | shape | the program | wall | peak RSS | firings | facts | in 100 ± 50 s | what bound it |
|---|---|---|---:|---:|---:|---:|---|---|
| `ein saturate` | product | 999 digits by 999 | 14.9 s | 6.13 GB | 5 000 895 | 5 002 922 | no | the parser: an operand of 1 000 digits nests past its limit |
| `ein saturate` | quotient | 998 digits by 499 | 5.8 s | 2.94 GB | 2 099 934 | 2 101 961 | no | the parser |
| `ein saturate` | requests | 43 430 on nine-digit numbers, the five operations in turn | 60.4 s | 15.97 GB | 16 384 266 | 16 525 264 | **yes** | the cap: 44 732 killed at it |
| `ein saturate` | chain | 1 081!, a number of 2 813 digits | 47.0 s | 15.98 GB | 16 270 571 | 16 280 163 | no | the cap: 1 113 killed at it |
| `ein solve` | product | 547 digits by 547 | 500.8 s | 2.26 GB | not taken | not taken | no | the time limit: 563 killed at 600 s |
| `ein solve` | quotient | 626 digits by 313 | 586.4 s | 1.37 GB | not taken | not taken | no | the time limit: 322 killed at 600 s |
| `ein solve` | requests | 30 379, as above | 277.2 s | 15.98 GB | not taken | not taken | no | searched to here, stopped by the user: 35 075 was killed at the cap, and 32 643 was running when the user stopped the sweep |
| `ein solve` | chain | not measured | | | | | | the user stopped the sweep before it |

*Firings* and *facts* are what `ein saturate` prints. `ein solve` prints
neither, and the saturations that would have counted its rows were not run.
`ein saturate` prints no answer. On every `ein solve` row, the script held the
answers it printed to Python's: the product's answer, 1 094 digits, is past
the parser's nesting limit for a claim, so the file states none and the
printed answer is what was checked; the other two rows' claims hold.
*Which limit bound*: under `ein saturate`, the parser bounds a written
operand before memory or time does, and the 16 GB cap bounds a program that
asks for many results or derives its numbers, each in about a minute; under
`ein solve`, which writes the state as text (E-24), the 600 s limit bounds a
long product or quotient while it holds 2.3 GB or less, and the cap bounds the
batch.

The commands, the program's first and the run's second (the date of each is
2026-11-13):

```sh
python3 utils/gen_arith_examples.py --op mul --digits 999 --count 1 --seed 20261111 -o build/s1f.6.29/biggest/product-999.ein
ein saturate build/s1f.6.29/biggest/product-999.ein

python3 utils/gen_arith_examples.py --op div --digits 499 --dividend 998 --count 1 --seed 20261111 -o build/s1f.6.29/biggest/quotient-499.ein
ein saturate build/s1f.6.29/biggest/quotient-499.ein

python3 utils/gen_arith_examples.py --op mix --digits 9 --dividend 18 --count 43430 --seed 20261111 -o build/s1f.6.29/biggest/requests-43430.ein
ein saturate build/s1f.6.29/biggest/requests-43430.ein

python3 utils/gen_arith_examples.py --op factorial --n 1081 -o build/s1f.6.29/biggest/chain-1081.ein
ein saturate build/s1f.6.29/biggest/chain-1081.ein

python3 utils/gen_arith_examples.py --op mul --digits 547 --count 1 --seed 20261111 -o build/s1f.6.29/biggest/product-547.ein
ein solve build/s1f.6.29/biggest/product-547.ein

python3 utils/gen_arith_examples.py --op div --digits 313 --dividend 626 --count 1 --seed 20261111 -o build/s1f.6.29/biggest/quotient-313.ein
ein solve build/s1f.6.29/biggest/quotient-313.ein

python3 utils/gen_arith_examples.py --op mix --digits 9 --dividend 18 --count 30379 --seed 20261111 -o build/s1f.6.29/biggest/requests-30379.ein
ein solve build/s1f.6.29/biggest/requests-30379.ein
```

### How long a number may be (M1f S1f.6.9)

The theory has no width and no overflow: a number is as long as its digits.
What bounds a run is outside the theory, in three places, each measured on
the shipped modules on 2026-11-12
([S1f.6.9](../plans/m1f_hypothesis_and_documentation/p1f.6_calculator/s1f.6.9_width.md)
has the probes; E-4, E-24 and E-26 of the phase's
[`findings.md`](../plans/m1f_hypothesis_and_documentation/p1f.6_calculator/findings.md)
are the engine's side of them).

**1. A program can write a number of 999 digits, and no longer.** A written
number nests a level a digit, and the parser refuses a program nested past
1 000 parentheses, with a line: *this `(` nests deeper than the limit of 1000
parentheses* (`ein_ir::grammar::NESTING_LIMIT`; `python3
build/s1f.6.9/depth.py`). The limit is the parser's alone: a rule can derive
a longer number, 999 nines plus 1 is answered with its thousand digits, and
the engine saturates a program whose facts are 396 000 levels deep in half a
second (`python3 build/s1f.6.9/deeper.py`).

**2. A state can be written that cannot be read back.** `ein solve
--dump-answer` writes a state as a program, one fact a line, and the parser
that reads it back has the same limit. A step of a circuit holds its operands
**inside its tag**, so its facts are deeper than the request that started it:

| operation | the deepest fact of its state | the longest operand a request can hold | the longest whose state file reloads |
|---|---|---:|---:|
| `add`, `sub`, `cmp` | the operand's digits + 2 | 999 | 998 |
| `mul`, `div` | digits + 3 | 999 | 997 |
| `int-add` | digits + 4 | 998 | 996 |
| `int-mul` | digits + 5 | 998 | 995 |
| `rat-add`, `rat-cmp`, by a numerator's digits | digits + 7 | 997 | 993 |

The third column is the parser's limit less the request's own nesting; the
fourth is 1 000 less the first. Both ends were run for a sum, a difference, a
comparison, a product by one digit and a sum of integers, and the rest follow
from the depth, which was measured at 10, 20 and 40 digits (`python3
build/s1f.6.9/depth.py --boundary add sub cmp "mul by one digit" int-add`).
Past the fourth column every answer is still right and still printed: `ein
solve` and `ein saturate --dump` write a deeper state without a word, and
only the state file's reload is refused, with the line above.
Which of three things to do about it is the user's question,
[Q-M1f.185](../plans/m1f_hypothesis_and_documentation/open_questions.md#q-m1f185--a-state-file-nested-past-the-parsers-limit);
until it is ruled, this paragraph is the answer.

**3. Each step holds its numbers whole.** One request of `std.math.decimal-arith`, its
operands drawn at random, on one P-core through `utils/bench_env.sh` (`python3
build/s1f.6.9/cost.py`); *the state's own text* is the state file less the
173 KB every program of the theory writes for the tables, and *derived* are
the facts the request added:

| request | firings | saturate ms | `ein saturate`, the process, ms | peak RSS MB | `ein solve --dump-answer`, the process | the state's own text | text a derived fact |
|---|---:|---:|---:|---:|---:|---:|---:|
| add, 10 digits | 30 | 0.3 | 26 | 19 | 48 ms | 18 KB | 0.6 KB |
| add, 100 digits | 210 | 0.5 | 26 | 19 | 97 ms | 927 KB | 4.4 KB |
| add, 998 digits | 2 006 | 2.9 | 36 | 22 | 6.7 s | 85.5 MB | 42.6 KB |
| add, 999 digits | 2 008 | 3.0 | 37 | 21 | 6.7 s | 85.6 MB | 42.6 KB |
| sub, 10 digits | 41 | 0.3 | 26 | 19 | 48 ms | 26 KB | 0.6 KB |
| sub, 100 digits | 311 | 0.7 | 26 | 19 | 118 ms | 1.4 MB | 4.6 KB |
| sub, 998 digits | 3 005 | 4.3 | 38 | 23 | 9.8 s | 127.7 MB | 42.5 KB |
| sub, 999 digits | 3 008 | 4.3 | 37 | 24 | 9.8 s | 127.9 MB | 42.5 KB |
| cmp, 10 digits | 18 | 0.3 | 26 | 19 | 48 ms | 10 KB | 0.6 KB |
| cmp, 100 digits | 108 | 0.5 | 26 | 19 | 71 ms | 457 KB | 4.2 KB |
| cmp, 998 digits | 1 006 | 2.2 | 35 | 19 | 3.5 s | 40.4 MB | 40.2 KB |
| cmp, 999 digits | 1 007 | 2.2 | 35 | 19 | 3.4 s | 40.5 MB | 40.2 KB |
| mul, 10 digits by 10 | 595 | 1.1 | 26 | 19 | 71 ms | 528 KB | 0.9 KB |
| mul, 100 digits by 100 | 50 698 | 72.7 | 111 | 75 | 18.4 s | 353.4 MB | 7.0 KB |
| mul, 300 digits by 300 | 452 676 | 662.0 | 801 | 470 | not finished in 300 s | | |
| mul, 998 digits by 1 | 2 013 | 3.2 | 37 | 21 | 5.9 s | 77.7 MB | 38.6 KB |
| mul, 998 digits by 10 | 38 139 | 56.3 | 94 | 71 | 152.2 s | 1.72 GB | 45.0 KB |
| mul, 998 digits by 100 | 408 300 | 610.2 | 753 | 430 | not run: the text is past 2 GB | | |
| div, 10 digits by 10 | 679 | 1.4 | 27 | 19 | 103 ms | 1.1 MB | 1.7 KB |
| div, 100 digits by 100 | 33 347 | 57.8 | 93 | 73 | 29.3 s | 544.8 MB | 16.3 KB |
| div, 600 digits by 300 | 766 926 | 1287.6 | 1573 | 871 | not run: the text is past 2 GB | | |
| div, 998 digits by 1 | 21 173 | 32.9 | 67 | 46 | 39.1 s | 491.0 MB | 23.2 KB |
| div, 998 digits by 10 | 74 613 | 123.0 | 177 | 126 | 165.1 s | 1.90 GB | 25.4 KB |
| div, 998 digits by 100 | 525 905 | 887.9 | 1089 | 759 | not run: the text is past 2 GB | | |

And the longest product and quotient a program can ask, under `ein saturate`
alone (`python3 build/s1f.6.9/longest.py`):

| request | firings | facts | saturate | the process | peak RSS |
|---|---:|---:|---:|---:|---:|
| mul, 998 digits by 998 | 4 988 896 | 4 990 871 | 9788.63 ms | 13.6 s | 5.9 GB |
| div, 998 digits by 499 | 2 071 748 | 2 073 723 | 3868.68 ms | 4.6 s | 2.5 GB |

**Where a run stops being practical, by what stops it:**

- **A sum, a difference or a comparison: nowhere a program can reach.** At
  999 digits each is a few thousand firings and 3 or 4 ms; the number's own
  text takes longer to parse.
- **A product or a quotient under `ein saturate`: memory.** The facts are the
  two lengths multiplied, and a stored fact costs about a kilobyte (E-26), so
  the longest product a program can write, 998 digits by 998, is five
  million facts and 5.9 GB, and saturates in ten seconds; a quotient of 998
  digits by 499 is two million and 2.5 GB.
- **Anything long under `ein solve`, `ein test` or `--dump-answer`: the
  state as text.** A derived fact of a 998-digit computation is 40 KB of
  text, since each repeats its numbers and stored they are shared (E-24).
  A 998-digit sum's state is 86 MB and 6.7 s; a product of two 100-digit
  numbers, 354 MB and 18 s, where it saturates in 73 ms; a 998-digit number
  times one of 10 digits, 1.7 GB and two and a half minutes. Past a hundred
  digits a product or a quotient is `ein saturate`'s to compute and a query's
  to read, not a state to write out.

**Words are not built.** Four-digit words, a number as a list of digits four
at a time, were measured in the design's round 3 and lost their gain to the
conversions on a single operation. The one case that would reopen them is a
long multiplication or division that stays in words throughout, a chain
whose intermediate numbers are never converted back, which is what the
phase's S1f.6.10 would ask for if it asks for long numbers.

**The names it occupies.** Imported flat, the module's names mean what the
module says, and its header has the list, in three parts.
**What a program states is the requests and `let`**: `add`, `sub`, `cmp`,
`mul`, `div` and `let`, over numbers written with `n`, `z` and the ten
digits; **and, from a rule, the tagged calls**
([above](#stdmathdecimal-arith-school-arithmetic-computed-by-saturation-alone-m1f-p1f6)).
**What it reads**: the results `sum-of`, `diff-of`, `cmp-of`, `product-of`
and `quot-of`; the flags `division-by-zero`, `malformed` and `undeclared`; the
orders `lt`, `eq` and `gt`; the tagged calls' answers, `done`, `short`,
`order-is`, `div-done` and `operand-is`. **The circuit's own**, which is in the state to be
seen and is no promise: `b`, some forty-five names of its walks, its check
and its tags, `arith-rule`, and the seven tables of `std.math.digits`. A second *declaration* of any of them is refused at
load, with the name in the message. A written *fact* of a result or of the
circuit is not: it is a fact like a derived one and the theory computes with
it, so a `(sum-of X Y S)` written by hand is an answer nobody computed. A
module does not reserve its names (the user's reply of 2026-11-12); what a
program that states false facts means is M1g's
[S1g.10.6](../plans/m1g_logic/p1g.10_school_algebra/s1g.10.6_a_program_that_states_false_facts.md).
**A number or a request with the wrong count of arguments
is refused at load** since S1f.6.23, `(n 1)`, `(n z 1 2)`, `(add (n z 5))`,
each a form of a relation the module declares
([`defined_behaviour.md` § 4.26](../docs/kernel/defined_behaviour.md#426-a-form-of-a-declared-relation-has-the-declared-count-of-arguments--m1f-s1f623)).

### Malformed requests: what is flagged, and what stays silent (M1f S1f.6.12)

The user, 2026-11-11: *"if theory can detect malformed numbers - do it"*.
Until this stage every request the theory could not read was answered by
silence: exit 0, one state, and a query with no row. Now the theory flags
what it can say without a negation, and one thing it can say only with one
(the stage's
[record](../plans/m1f_hypothesis_and_documentation/p1f.6_calculator/s1f.6.12_malformed_numbers.md#build-record-2026-11-13);
the investigation is
[`malformed_numbers.md`](../plans/m1f_hypothesis_and_documentation/p1f.6_calculator/malformed_numbers.md)).

| what the program wrote | the flag | by |
|---|---|---|
| a digit's place that holds a name that is no digit, `(n (n z 1) x)`, `z` and `10` among such names, or a fact, `(n z (n z 1))` | `(malformed REQUEST)` | `check-digit`: ten `neq` guards, which hold of every other name and of a fact |
| a number that ends in a digit where `z` goes, `(n 1 2)` | `(malformed REQUEST)` | `check-prefix-digit`, `check-prefix-zero` |
| a number that ends in a named constant, `(n price 1)`; an operand that names another name, `(let alias price)` | `(malformed REQUEST)` | `check-prefix-named` |
| a digit where a number goes, `(add 7 X)`: seven is `(n z 7)` | `(malformed REQUEST)` | `operand-digit`, `operand-digit-zero` |
| any of those behind a name, `(let broken (n z x))`, where a request uses the name | `(malformed REQUEST)` | the same rules: `operand-name` hands what it finds to the check |
| a name stated twice with two numbers | `(malformed (let NAME NUMBER))`, each of the two | `let-twice`, a join on the name. Until S1f.6.12: **two answers to one request** |
| `z` or a digit as a constant's name, `(let z N)`, `(let 7 N)` | `(malformed (let NAME NUMBER))` | `let-zero`, `let-digit`, `let-digit-zero`. Until S1f.6.12: `(add z X)` had two answers |
| an operand that is a name no `let` states, `(add foo X)` | `(undeclared foo REQUEST)` | `operand-undeclared`: the theory's one `absent`, over `let` |

Each flag derives `(false)` by a rule of its own, `malformed-is-false` and
`undeclared-is-false`, so a trace shows the flag as one step and the
contradiction as the next:

```text
  states (k_s)      0
  empty             every node is dead — root's own fixpoint derives (false)
  unsat core        (add (n (n z 1) x) (n z 5))
  refuted by        malformed-is-false
```

A request has one flag, the request as written, and no result fact; the
well-formed requests beside it are computed in root's state, which is dead
and so no answer. Such a program can claim only `(solve :expect (false))`.
A `let` that is added answers an undeclared request and leaves nothing of
the flag: root is the only state, and it has either the `let` or the flag.

**A flag means malformed; no flag does not mean well-formed.** There is no
general report of a request no rule completes (the user's reply of
2026-11-12). These stay silent, neither refused nor answered, and
`arith_theory.rs`'s `what_stays_silent` holds that the list is this one:

- **a number that ends in a name that is neither a digit nor a named
  constant**, `(n end 1)`. Saying *a name* costs a walk of the program's
  names, at every place of every number;
- **a fact of another relation where a number goes**, as an operand or
  inside a number, `(add (pair 1 2) X)`, the accumulator's own shape `(b 1
  z)` among them. No rule can say *not of this shape*;
- **a name that stands for either**;
- **in `std.math.int` and `std.math.rat`**: a natural number where an integer goes and
  an integer where a rational goes, and a malformed number *inside* an
  integer or a rational, whose parts go straight to `std.math.decimal-arith`'s walks and
  never through its operand. An undeclared name or a digit as an operand of
  theirs is seen by `std.math.decimal-arith`'s rules, `(bad-operand TAG FAULT)` in root's
  state, and **not flagged**: no rule of those modules turns their tag into
  a request. A `let` that cannot be read is flagged there as here;
- **every malformation of `std.math.num9`** (the user's reply of 2026-11-12: that
  module is kept as an example of another representation).

**The check is a pass of its own, at the operand.** `operand-number` and
`operand-name` hand each number they find to `check`, which goes one place
down a firing; nothing is added to a walk. It was measured against guards
on the walks themselves before it was chosen (the record's table): the pass
costs a firing a digit of each operand, whatever the operation, and the
mixed placement costs none for a sum, a difference and a comparison and
more for a product and a quotient, with twice the rules. 123078 + 4567 is 31
firings where it was 21, and still 21 deep, since the check runs beside the
walk (`arith_theory.rs`'s `the_circuit_of_one_addition`).

**What the undeclared name costs.** To say that an operand is a *name*, and
not a number, which is a fact, `operand-undeclared` ranges a variable over
the program's names and keeps the one that is the operand: two comparisons a
name, for each operand of each request, some 800 a request in a program of
the theory and numbers alone. It does not grow with the batch; **it grows
with the program's names**, so the same thousand sums take 69.7 ms beside no
named constant and 140.7 ms beside 4 000, where the theory before this stage
took 36.6 and 41.3 (`utils/bench_env.sh python3
build/s1f.6.12/build/cost3.py`, 2026-11-13).
`arith_batch.rs` pins it by the guards' own count, and E-36 of the phase's
`findings.md` says what would remove it.

## `std.math.num9`: nine digits in one fact, and wide rules (M1f S1f.6.13)

The calculator's second module, and the user's own number (the instruction of
2026-11-08; the ruling of 2026-11-11, *"yes, ship num9 in stdlib, why not"*):
`(num9 0 0 0 1 2 3 0 7 8)` is 123078, nine digit names, the most significant
first, a fixed width. The theory is the same circuit over the same tables of
`std.math.digits`, wired the other way: where `std.math.decimal-arith` walks a number one column
a firing, a rule here has **a premise a column**, so a sum is one firing.
Computed by saturation alone: no candidate, R {∅}, no `absent`, no `not`, no
guard, one stratum.

```lisp
(import std.math.num9 :symbols (arith9))

(add9 (num9 0 0 0 1 2 3 0 7 8) (num9 0 0 0 0 0 4 5 6 7))

(query :goal (sum9-of ?x ?y ?s))      ; ?s = (num9 0 0 0 1 2 7 6 4 5)
```

**The file is generated,** by
[`utils/gen_num9.py`](../utils/gen_num9.py), as `std.math.digits` is: nine columns
of variables twenty times over are not for typing or for reviewing by eye.
`--check` is in the gate (`ein-infer/tests/num9_theory.rs`); `--width W
--stdout` prints the same theory at another width, which is how the harness
holds it exhaustively, and no other width is shipped. 47 rules and 22
relations (`grep -c '^(rule' stdlib/math/num9.ein`, `grep -c '^(relation'`,
2026-11-11). **A wide rule is wide in registers too:** a rule binds its
variables to a file of 256 (`ein_infer::MAX_REGS`), and where the widest rule
of every other module binds 16, a sum here binds 35 and `div9-start` 162.
`ein-infer/tests/compile_limits.rs` holds that number, and the question it
raises about the bound is the user's (Q-M1f.173).

| | |
|---|---|
| **the protocol** | as `std.math.decimal-arith`'s, under names of its own, so the two import flat into one program: `(add9 X Y)` answered by `(sum9-of X Y S)`; `(sub9 X Y)` by `(diff9-of X Y D R)`, two numbers, one of them zero; `(cmp9 X Y)` by `(cmp9-of X Y O)`, O one of `lt`, `eq`, `gt`; `(mul9 X Y)` by `(product9-of X Y P)`; `(div9 X Y)` by `(quot9-of X Y Q R)`. An operand is a number written out: the module has no named constants |
| **overflow** | **a fact, and no number.** A sum or a product that does not fit nine digits derives `(overflow REQUEST)`, the request as written, and no result fact: never a truncated number. Root stays alive, the program's other requests are answered, and a rule of the program may match the flag. A difference, a comparison and a quotient cannot overflow |
| **division by zero** | a contradiction, as in `std.math.decimal-arith` and **by its rule**: `div9-by-zero` derives `(division-by-zero (div9 X ZERO))`, and `std.math.decimal-arith`'s `division-by-zero-is-false`, which this module imports, derives `(false)`. Root is dead and R is empty |
| **conversions** | a request each way, one firing: `(to9 X)`, X a number of `std.math.decimal-arith`, is answered by `(as9 X W)`, or by `(big X)` where X is written with more than nine digits; `(from9 W)` by `(of9 W X)`, X in normal form. Nothing converts a number because it exists, and no rule of either module hands a request to the other: a program that wants both writes the rules that ask ([`examples/arith/num9_both_ways.ein`](../examples/arith/num9_both_ways.ein)) |
| **the one entry** | `arith9`, the closing rule: it names every rule and closes every relation of the module, the six tables it reads, and the two relations it takes from `std.math.decimal-arith` |

**How it computes.** `add9-fits` has a premise for the request and nine rows
of the full adder's table, the carry handed from column to column in a
variable, the carry out of the top written `0`; its twin `add9-overflows`
writes `1` there. A difference is two such rules, one each way round, and the
one that does not borrow out of the top answers, as in hardware: the borrow
is the comparison. A comparison is one rule over the comparator's table. **A
product is eleven firings:** a start, a step for each digit of the multiplier
from its top, an end. A step is one rule with eighteen table premises, the
multiplicand times the digit and that row added to the sum so far shifted one
place; where the digit is 0 a second rule shifts and reads no table. Three
rules derive the overflow, one for each place a digit can fall off. **A
quotient is eleven firings:** `div9-start` computes the divisor's multiples by
2 to 9 in one firing, 72 rows of the digit multiplier's table, and puts all
nine into the state; then a step a digit of the dividend, by ten rules, one
for each digit the quotient can have, each reading its own multiple from the
state and holding two subtractors: the multiple taken from the number must not
borrow, and the divisor taken from what is left must. Exactly one fires, and
the tables decide which.

**No rule joins two facts on a number,** `std.math.decimal-arith`'s rule, kept here and
held by the same two tests (`num9_theory.rs` on the text, per commit, and
`arith_batch.rs` on the matcher's count, by hand since M1f S1f.6.21). It is what the design's prototype did not do: its step
rule joined a division's state with the divisor's multiples on the divisor, a
nested fact, and 4 000 divisions took 8.1 s where 250 took 0.19
([`design.md` § 4](../plans/m1f_hypothesis_and_documentation/p1f.6_calculator/design.md#4-the-table)).
With the multiples in the state, and a product's rows computed inside its
step, a batch costs in proportion to its count: 362, 362, 901, 10 409 and
26 889 candidates a request for a sum, a difference, a comparison, a product
and a quotient, the same at 250, 1 000 and 4 000 (`cargo test -p ein-infer
--features counters --test arith_batch -- --nocapture`, 2026-11-11).

### When to use which module

The same requests asked of both, in batches, the saturator's own
milliseconds, best of five on one P-core (`python3 build/s1f.6.13/batch.py`,
2026-11-11; two numbers of up to nine digits for a sum that fits, a
difference and a comparison, two of up to four digits for a product, up to
nine digits by 1 to 999 for a quotient):

| operation | requests | std.math.num9: firings a request | ms | std.math.decimal-arith: firings a request | ms |
|---|---:|---:|---:|---:|---:|
| add | 250 | 1.0 | 1.9 | 21.9 | 6.8 |
| add | 1000 | 1.0 | 6.3 | 21.7 | 27.3 |
| add | 4000 | 1.0 | 23.6 | 21.7 | 113.2 |
| sub | 250 | 1.0 | 1.9 | 29.4 | 10.2 |
| sub | 1000 | 1.0 | 6.4 | 29.2 | 40.9 |
| sub | 4000 | 1.0 | 23.8 | 29.2 | 170.7 |
| cmp | 250 | 1.0 | 2.9 | 13.5 | 5.2 |
| cmp | 1000 | 1.0 | 10.0 | 13.4 | 20.6 |
| cmp | 4000 | 1.0 | 38.1 | 13.4 | 82.1 |
| mul | 250 | 10.9 | 21.3 | 53.5 | 17.6 |
| mul | 1000 | 10.7 | 82.8 | 54.1 | 71.8 |
| mul | 4000 | 10.2 | 331.2 | 53.4 | 300.0 |
| div | 250 | 11.0 | 78.3 | 253.6 | 98.2 |
| div | 1000 | 11.0 | 308.1 | 249.1 | 411.2 |
| div | 4000 | 11.0 | 1243.3 | 247.6 | 1776.3 |

- **`std.math.num9` where every number fits nine digits and there are many of
  them.** One firing for a sum against 22: 4 000 sums in 24 ms against 113,
  differences in 24 against 171, comparisons in 38 against 82, quotients in
  1.24 s against 1.78. Products are level, 331 ms against 300: a product
  here always takes its nine steps, and `std.math.decimal-arith` takes as many as the
  multiplier has digits. A trace of a sum is one step with its nine table
  rows, where `std.math.decimal-arith`'s is twenty-one.
- **`std.math.num9` where a fixed width is what is modelled:** a register, a
  counter, a field of a form, with the overflow as a fact a rule can read.
- **`std.math.decimal-arith` for numbers of any length, for named constants, and for a
  program with a request or two.** A program that imports `std.math.num9` pays
  29.3 ms of `kb load` where one that imports `std.math.decimal-arith` pays 19.6: 14.3 for
  `std.math.digits`, about ten for the module's own 46 KB of wide rules, and five
  for the text of `std.math.decimal-arith`, read whole for the three names taken from it
  (`ein saturate tests/stdlib/math/num9/01_add.ein`, best of five, 2026-11-11).
  So one sum is 36 ms as a process here and 26 there.
- **Both,** flat in one program, with a conversion request where a number
  crosses. Round 3's design that routed between the two by itself was the
  slowest measured, and is not shipped.

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
`num9-rule` and the entry `arith9`. From `std.math.decimal-arith` it takes `n`, with `z`,
and `division-by-zero` with the rule from it to `(false)`; from `std.math.digits`,
`fa`, `fs`, `md`, `cl`, `nz` and `adj`, with the three orders. A `num9` whose
count of digits is not nine is **refused at load** since S1f.6.23, the
loader holding a form to its declaration's count; one with a digit's place
that holds something else is answered by silence, and stays so: `std.math.decimal-arith`
flags such a number since S1f.6.12, and this module does not (the user's
reply of 2026-11-12: it is kept as an example of another representation of
a number, and is not carried further).

## `std.math.int`: integers as a positive and a negative part (M1f S1f.6.7)

The calculator's first extension, and the user's proposal of 2026-11-08:
*"numbers containing 2 parts - positive and negative, that can be simplified
until one of them is zero"*. An integer is `(int P N)`, two natural numbers of
`std.math.decimal-arith`, and its value is P less N. The module is written over `std.math.decimal-arith`,
which it calls and does not edit, and is computed by saturation alone: no
candidate, R {∅}, and no `absent`, no `not` and no guard of its own (a
program that imports it holds `std.math.decimal-arith`'s one `absent`, and two strata).

```lisp
(import std.math.int :symbols (int-arith))

(int-add (int z (n (n z 1) 2)) (int (n (n z 4) 0) z))     ; -12 + 40

(query :goal (int-sum-of ?x ?y ?s))      ; ?s = (int (n (n z 2) 8) z)
```

| | |
|---|---|
| **the integer** | `(int P N)`. 7 is `(int (n z 7) z)`, −7 is `(int z (n z 7))`, zero is `(int z z)`, and `(int (n z 5) (n z 3))` is 2, not simplified. The **normal form** has one part `z` and the other in `std.math.decimal-arith`'s normal form; every integer has exactly one. An operand is read by value, whatever its parts and their high zeros, and every answer is in normal form, so a rule of a program tells a negative answer by its shape, `(int z (n ?p ?a))`, with no negation written |
| **the protocol** | `(int-simplify X)` answered by `(int-simple-of X S)`; `(int-neg X)` by `(int-neg-of X S)`; `(int-add X Y)` by `(int-sum-of X Y S)`; `(int-sub X Y)` by `(int-diff-of X Y S)`, **one integer**, where `std.math.decimal-arith`'s `diff-of` answers two numbers: they are its two parts; `(int-mul X Y)` by `(int-product-of X Y S)`; `(int-cmp X Y)` by `(int-cmp-of X Y O)`, O one of `lt`, `eq`, `gt`, by value |
| **an operand** | an integer written out, or a name a `(let NAME (int P N))` states: `let` is `std.math.decimal-arith`'s. A natural number is **not** an integer operand, and such a request is never answered (Q-M1f.175). **Nothing of an operand is flagged here** but a `let` that cannot be read: a malformed number inside an integer, an undeclared name and a digit as an operand are answered by silence ([`std.math.decimal-arith` § Malformed requests](#malformed-requests-what-is-flagged-and-what-stays-silent-m1f-s1f612)) |
| **division** | **names its convention** (M1f S1f.6.22, the user's ruling of 2026-11-12 on [Q-M1f.179](../plans/m1f_hypothesis_and_documentation/open_questions.md#q-m1f179--the-remainder-of-an-integer-division)); there is no bare `int-div`. Each request is answered by one fact with a quotient and a remainder, X = Q·Y + R: `(int-div-euclid X Y)` by `(int-quot-euclid-of X Y Q R)`, 0 ≤ R < \|Y\|, Q an integer and R a **natural number** of `std.math.decimal-arith`; `(int-div-trunc X Y)` by `(int-quot-trunc-of X Y Q R)`, Q rounded toward zero and R with the dividend's sign, both integers; `(int-div-floor X Y)` by `(int-quot-floor-of X Y Q R)`, Q rounded down and R with the divisor's sign, both integers. The three agree where neither operand is negative, and with `std.math.decimal-arith`'s `div` there |
| **division by zero** | a divisor that is zero by value, however written or named, derives `std.math.decimal-arith`'s flag, `(division-by-zero REQUEST)`, the request as written, and `std.math.decimal-arith`'s rule derives `(false)`: root is dead and R is empty, as everywhere in the calculator. No rule of the module concludes `(false)` |
| **the one entry** | `int-arith`, the closing rule: it closes every relation of the module, names every rule, and names `decimal-arith`, so the one name brings `std.math.decimal-arith` and the tables too |

**How it computes.** Each operation works on the parts with `std.math.decimal-arith`'s
walks and then simplifies. Simplifying is subtraction both ways at once, P − N
and N − P: the walk that ends with no borrow hands over the part that is left.
A sum is the positive parts added and the negative parts added; a difference
is a sum with the second operand's parts exchanged, and a negation a
simplification with them exchanged. A product is (P1·P2 + N1·N2, P1·N2 +
N1·P2), four products of `std.math.decimal-arith` and two sums, and **no rule looks at a
sign**. A comparison adds across, P1 + N2 against N1 + P2, and compares two
natural numbers; it builds no integer. **A division** simplifies both
operands, so that each shows its sign by its shape, and hands the two sizes to
`std.math.decimal-arith`'s `div-call` with the signs in the tag; that much is written once
for the three conventions. Truncated then only puts the signs on. Floored and
Euclidean adjust two of the four combinations of signs where a remainder is
left: one more in the quotient's size, an addition walk, and the divisor's
size less the remainder, a subtraction walk.

| X ÷ Y | Euclidean Q, R | truncated Q, R | floored Q, R |
|---|---|---|---|
| 7 ÷ 2 | 3, 1 | 3, 1 | 3, 1 |
| −7 ÷ 2 | −4, 1 | −3, −1 | −4, 1 |
| 7 ÷ −2 | −3, 1 | −3, 1 | −4, −1 |
| −7 ÷ −2 | 4, 1 | 3, −1 | 3, −1 |

−7 ÷ 2 is 124 firings under Euclidean and floored and 112 under truncated
(`cargo test -p ein-infer --test int_theory the_circuit`, 2026-11-12).
**A call's tag is its request**:
`(int-add X Y)` is the tag of its own computation. **The tagged calls a later
theory makes are an interface,** as [`std.math.decimal-arith`'s](#stdmathdecimal-arith-school-arithmetic-computed-by-saturation-alone-m1f-p1f6)
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
those. A batch costs in proportion to its count: 476, 480,
3 931 and 870 candidates a request for a sum, a difference, a product and a
comparison at 250 requests, and 4 606, 4 557 and 4 610 for a Euclidean, a
truncated and a floored division of four-digit parts, and no more at 1 000
and 4 000 (`cargo test -p ein-infer --features counters --test arith_batch
a_batch_of_integer -- --nocapture`, 2026-11-12; run by hand since S1f.6.21).

**What it costs beside `std.math.decimal-arith`.** The same operation asked of both, the
integers' on operands of up to nine digits and either sign (four digits for a
product), `std.math.decimal-arith`'s on their sizes; the saturator's own milliseconds, best
of five (`python3 build/s1f.6.7/batch.py`, 2026-11-12):

| operation | requests | std.math.int: firings a request | ms | std.math.decimal-arith on the sizes: firings a request | ms |
|---|---:|---:|---:|---:|---:|
| add | 250 | 55.5 | 19.3 | 21.8 | 7.0 |
| add | 1000 | 55.9 | 76.0 | 21.8 | 26.3 |
| add | 4000 | 56.2 | 357.3 | 21.9 | 116.7 |
| sub | 250 | 56.0 | 19.5 | 29.2 | 10.3 |
| sub | 1000 | 56.2 | 77.1 | 29.2 | 41.8 |
| sub | 4000 | 56.2 | 362.2 | 29.3 | 183.7 |
| cmp | 250 | 38.1 | 14.2 | 13.4 | 5.3 |
| cmp | 1000 | 38.3 | 56.3 | 13.4 | 20.1 |
| cmp | 4000 | 38.3 | 255.9 | 13.4 | 81.4 |
| mul | 250 | 112.8 | 39.6 | 51.1 | 16.3 |
| mul | 1000 | 118.1 | 181.8 | 54.8 | 72.1 |
| mul | 4000 | 120.9 | 864.2 | 57.6 | 327.6 |

An integer operation is two and a half to three times the natural one: a sum
is two addition walks and the two subtraction walks of the simplification. A
program that imports the module pays 24.4 ms of load where `std.math.decimal-arith`'s pays
19.8 (`ein saturate examples/arith/int.ein` and `add.ein`, the `kb load:`
line, best of five, 2026-11-12, re-taken with S1f.6.22's division in the
module; 22.4 before it).

**The names it occupies** are in the module's header: `int`, the nine requests
and nine results, the four calls and `int-done`, a division's hand-over
`int-div-done`, twenty-one tags, the signs `pos` and `neg`, `int-rule` and
`int-arith`; everything of `std.math.decimal-arith` comes with it.

## `std.math.rat`: rational numbers, in lowest terms (M1f S1f.6.8)

The calculator's second extension, and the user's proposal of 2026-11-08:
rational numbers *"containing integer numerator and natural denominator"*. A
rational is `(rat Z D)`, an integer of `std.math.int` over a natural number of
`std.math.decimal-arith` that is not zero. The module is written over the two theories
below it, which it calls and does not edit, and is computed by saturation
alone: no candidate, R {∅}, and no `absent`, no `not` and no guard of its
own (a program that imports it holds `std.math.decimal-arith`'s one `absent`, and two
strata).

```lisp
(import std.math.rat :symbols (rat-arith))

(rat-add (rat (int (n z 1) z) (n z 2)) (rat (int (n z 1) z) (n z 3)))     ; 1/2 + 1/3

(query :goal (rat-sum-of ?x ?y ?s))      ; ?s = (rat (int (n z 5) z) (n z 6))
```

| | |
|---|---|
| **the rational** | `(rat Z D)`. 1/2 is `(rat (int (n z 1) z) (n z 2))`, −3/4 is `(rat (int z (n z 3)) (n z 4))`. The **normal form** is in lowest terms: the numerator in `std.math.int`'s normal form, the denominator with no high zero, the two with no common divisor but 1, and zero written 0 over 1, `(rat (int z z) (n z 1))`. Every rational has exactly one. An operand is read by value, 2/4 as 1/2, and every answer is in normal form |
| **the protocol** | `(rat-simplify X)` answered by `(rat-simple-of X S)`; `(rat-neg X)` by `(rat-neg-of X S)`; `(rat-add X Y)` by `(rat-sum-of X Y S)`; `(rat-sub X Y)` by `(rat-diff-of X Y S)`; `(rat-mul X Y)` by `(rat-product-of X Y S)`; `(rat-div X Y)` by `(rat-quot-of X Y S)`, **one rational, exact, and no remainder**; `(rat-cmp X Y)` by `(rat-cmp-of X Y O)`, O one of `lt`, `eq`, `gt`, by value |
| **an operand** | a rational written out, `(rat (int P N) D)`, or a name a `(let NAME (rat (int P N) D))` states. An integer or a natural number is **not** a rational operand (Q-M1f.181), and nothing of an operand is flagged here but a `let` that cannot be read, as in `std.math.int` |
| **a zero denominator** | **a division by zero, and a contradiction**, as in `std.math.decimal-arith` and by its rule. A request any operand of which has a denominator of zero, however the zero is written, and a `rat-div` by a rational that is zero, derive `(division-by-zero REQUEST)`, the request as written, and `std.math.decimal-arith`'s `division-by-zero-is-false` derives `(false)`. Root is dead and R is empty (Q-M1f.182) |
| **the one entry** | `rat-arith`, the closing rule: it closes every relation of the module, names every rule, and names `int-arith` and `decimal-arith`, so the one name brings all three theories and the tables |

**How it computes.** The operands are looked up and each denominator is
compared with zero by `std.math.decimal-arith`'s walk. A sum of P1 less N1 over D1 and P2
less N2 over D2 is the **cross products**, P1·D2 + P2·D1 less N1·D2 + N2·D1,
over D1·D2: five products and two sums of natural numbers; a difference is
the same with the second numerator's parts exchanged. **A comparison is a
difference's numerator and nothing more**: its positive part against its
negative part, since the denominators are positive. A product is the
numerators' product, `std.math.int`'s, over the denominators'; a quotient is a
product with the divisor turned over, its sign kept in the numerator, and a
divisor that is zero has no turning over. Then **lowest terms**: the numerator
simplified by `std.math.int`, the greatest common divisor of its size and the
denominator by **Euclid's chain**, gcd(A, B) = gcd(B, A mod B) until the
remainder is zero, each step a division of `std.math.decimal-arith`, and both divided by
what it finds. **It does not need a division of integers**, so it does not
wait for Q-M1f.179. A call's tag is its request, as in `std.math.int`. **The tagged
calls a later theory makes are an interface,** held to these shapes by the
same test as `std.math.decimal-arith`'s; the denominators must not be zero:

```text
(rat-norm TAG P N D)                  asked: P less N over D, in lowest terms
                                      answered by (rat-done TAG RATIONAL)
(rat-mul-call TAG P1 N1 D1 P2 N2 D2)  asked: P1 less N1 over D1, times P2 less N2 over D2
                                      answered by (rat-done TAG RATIONAL)
(rat-cross TAG P1 N1 D1 P2 N2 D2)     asked: P1 × D2 + P2 × D1, and N1 × D2 + N2 × D1
                                      answered by (rat-cross-is TAG POSITIVE NEGATIVE)
```

**Every rule has one premise.** `ein-infer/tests/rat_theory.rs` holds that on
the text, and holds the answers to exact fractions: every pair of the 28
fractions with a numerator from −3 to 3 over a denominator from 1 to 4 under
each of the five operations, samples to twelve digits, and operands spelt
with a common factor, an unsimplified numerator and high zeros.

**What it costs.** One request, in firings (`cargo test -p ein-infer --test
rat_theory the_circuit`, 2026-11-12):

| request | firings |
|---|---:|
| 2/4 in lowest terms | 334 |
| 1071/462 in lowest terms, a chain of three divisions | 867 |
| −3/4 negated | 435 |
| 1/2 + 1/3 | 492 |
| 1/2 − 1/3 | 417 |
| 2/3 × 3/4 | 459 |
| 1/2 ÷ −3/4 | 494 |
| 1/2 against 1/3 | 76 |
| 123/456 + 789/1011 | 2 640 |
| 123/456 against 789/1011 | 270 |

Most of a request is lowest terms, and most of that is `std.math.decimal-arith`'s
division computing its divisor's nine multiples before its first digit, on
every step of the chain and on the two divisions after it (Q-M1f.183). **A
comparison reduces nothing** and costs a sixth of a sum: a program that only
needs the order should ask for the order. In batches, two fractions with a
numerator from 1 to 9 of either sign over a denominator from 2 to 9, the
saturator's own milliseconds, best of three (`python3 build/s1f.6.8/batch.py`,
2026-11-12):

| operation | requests | firings a request | ms |
|---|---:|---:|---:|
| add | 100 | 611.4 | 103.2 |
| add | 400 | 630.6 | 425.0 |
| add | 1200 | 639.8 | 1348.8 |
| sub | 100 | 644.1 | 109.5 |
| sub | 400 | 627.6 | 424.2 |
| sub | 1200 | 636.8 | 1366.5 |
| mul | 100 | 592.9 | 96.4 |
| mul | 400 | 604.0 | 411.2 |
| mul | 1200 | 599.6 | 1253.7 |
| div | 100 | 587.5 | 96.7 |
| div | 400 | 595.9 | 405.1 |
| div | 1200 | 599.7 | 1245.1 |
| cmp | 100 | 78.0 | 11.4 |
| cmp | 400 | 78.2 | 44.4 |
| cmp | 1200 | 78.1 | 133.9 |

A batch costs in proportion to its count: 7 511, 7 470, 6 957, 6 942 and 641
candidates a request for a sum, a difference, a product, a quotient and a
comparison at 100 requests, and within a few per cent of that at 400 and
1 200 (`cargo test -p ein-infer --features counters --test arith_batch
a_batch_of_rational -- --nocapture`, 2026-11-12). The batches are smaller
than the other modules' because a stored fact costs about a kilobyte (E-26):
1 200 sums are 760 000 facts. A program that imports the module pays
29.8 ms of load where `std.math.int`'s pays 24.4 and `std.math.decimal-arith`'s 19.8 (`ein saturate
examples/arith/rat.ein`, `int.ein` and `add.ein`, the `kb load:` line, best
of five, 2026-11-12, re-taken with S1f.6.22's division in `std.math.int`, which
this module brings; 27.0 and 22.4 before it).

**Nothing limits the length.** Cross products are as long as their operands
together, and nested numbers hold them: two fractions of twelve-digit terms
are added and compared in `rat_theory.rs`. What limits a run is what limits
`std.math.decimal-arith`'s division, the size of the state (§ [How long a number may
be](#how-long-a-number-may-be-m1f-s1f69)).

**The names it occupies** are in the module's header: `rat`, the seven
requests and seven results, the three calls and their two hand-overs, the
circuit's own five relations, the signs `pos` and `neg`, twenty-two tags,
`rat-rule` and `rat-arith`; everything of `std.math.int` and `std.math.decimal-arith` comes with
it.

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
them activated — some seconds, and **368 of 368** on 2026-11-14 since
`std.math.decimal-arith`'s 116 (M1f S1f.6.3 to S1f.6.6, S1f.6.12's checks and
S1f.6.24's call), `std.math.num9`'s 47 (S1f.6.13), `std.math.int`'s 71
(S1f.6.7, and S1f.6.22's division) and `std.math.rat`'s 57 (S1f.6.8) joined
the 77. *Of 368* is the command's, `find stdlib -name '*.ein' -exec cat {} +
| grep -c '^(rule '`, which reads every depth (the calculator's modules are
under `stdlib/math/` since S1f.6.31, and `cat stdlib/*.ein` would count 77);
*all of them* is `every_stdlib_rule_is_activated_by_a_program`'s, which fails
the day it stops being true. `python3 utils/stdlib_census.py --check` is the same census with
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
| `std.math.decimal-arith` | [`math/decimal-arith.ein`](math/decimal-arith.ein) | school arithmetic on natural numbers of any length, `z` and `(n PREFIX DIGIT)`, **computed by saturation alone** over `std.math.digits`' tables: a request `(add X Y)` answered by `(sum-of X Y S)`, named constants `(let NAME NUMBER)`; no rule joins two facts on a number; one entry, the closing rule `decimal-arith`. Addition, subtraction with its two numbers `(diff-of X Y D R)`, comparison `(cmp-of X Y lt|eq|gt)`, multiplication `(product-of X Y P)` and division `(quot-of X Y Q R)`, a division by zero a flag and `(false)` from it, raised at the tagged call whoever made it; a malformed request and an undeclared name a flag and `(false)` likewise, `(malformed REQUEST)` and `(undeclared NAME REQUEST)` ([§ `std.math.decimal-arith`](#stdmathdecimal-arith-school-arithmetic-computed-by-saturation-alone-m1f-p1f6)) | M1f S1f.6.3 to S1f.6.6, S1f.6.12, S1f.6.24 |
| `std.math.num9` | [`math/num9.ein`](math/num9.ein) | school arithmetic on numbers of nine digits in one fact, `(num9 d8 … d0)`, **computed by saturation alone**, by wide rules, a premise a column: `add9`/`sum9-of`, `sub9`/`diff9-of`, `cmp9`/`cmp9-of`, `mul9`/`product9-of`, `div9`/`quot9-of`; `(overflow REQUEST)` and no number where an answer does not fit; conversions to and from `std.math.decimal-arith`'s numbers, `to9`/`as9` and `from9`/`of9`. **Generated** by `utils/gen_num9.py`. One entry, `arith9` | S1f.6.13 |
| `std.math.int` | [`math/int.ein`](math/int.ein) | integers as a positive and a negative part, `(int P N)`, over `std.math.decimal-arith`'s natural numbers, **computed by saturation alone**: `int-simplify`/`int-simple-of`, `int-neg`/`int-neg-of`, `int-add`/`int-sum-of`, `int-sub`/`int-diff-of`, `int-mul`/`int-product-of`, `int-cmp`/`int-cmp-of`, every answer in normal form, one part `z`; every rule has one premise; integer division names its convention, `int-div-euclid`/`int-quot-euclid-of` (the remainder a natural number), `int-div-trunc`/`int-quot-trunc-of` and `int-div-floor`/`int-quot-floor-of`, since M1f S1f.6.22 ([§ `std.math.int`](#stdmathint-integers-as-a-positive-and-a-negative-part-m1f-s1f67)); imported by its one entry, `(import std.math.int :symbols (int-arith))`, which brings `std.math.decimal-arith` too | M1f S1f.6.7, S1f.6.22 |
| `std.math.rat` | [`math/rat.ein`](math/rat.ein) | rational numbers, `(rat Z D)`, an integer of `std.math.int` over a natural number that is not zero, **computed by saturation alone** and answered in lowest terms: `rat-simplify`/`rat-simple-of`, `rat-neg`/`rat-neg-of`, `rat-add`/`rat-sum-of`, `rat-sub`/`rat-diff-of`, `rat-mul`/`rat-product-of`, `rat-div`/`rat-quot-of` (one rational, no remainder), `rat-cmp`/`rat-cmp-of`; the greatest common divisor by Euclid's chain over `std.math.decimal-arith`'s division; a zero denominator is a division by zero, `std.math.decimal-arith`'s flag and its rule to `(false)`; every rule has one premise ([§ `std.math.rat`](#stdmathrat-rational-numbers-in-lowest-terms-m1f-s1f68)); imported by its one entry, `(import std.math.rat :symbols (rat-arith))`, which brings `std.math.int` and `std.math.decimal-arith` too | M1f S1f.6.8 |
| `std.math.digits` | [`math/digits.ein`](math/digits.ein) | **facts, no rule**: the calculator's seven digit tables, `fa` / `fs` / `md` / `cl` / `nz` / `mid` / `adj`, 1 664 rows, each table closed by a written `(__closed__ R)`; generated by `utils/gen_arith_tables.py` ([§ `std.math.digits`](#stdmathdigits-a-library-of-facts-m1f-s1f62)) | M1f S1f.6.2 |
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

Every import brings its module **whole**, with the module's own imports, one
copy per program; the three forms differ only in **names** (M1f S1f.6.35, the
user's ruling of 2026-11-14, *"in all cases we import everything, the
difference only in names"*):

```lisp
(import std.math.decimal-arith)                              ; the file writes std.math.decimal-arith.add, std.math.decimal-arith.sum-of
(import std.math.decimal-arith :as a)                        ; the file writes a.add, a.sum-of
(import std.math.decimal-arith :symbols (decimal-arith add n sum-of)) ; the file writes add, n, sum-of
```

Every name a module declares has one **full name**, `std.math.decimal-arith.add`, the same
whoever imports the module; it is what a state prints where the program gives
the name no spelling of its own (a circuit's step, `std.math.decimal-arith.strip`). **A
program lists every short name it writes**, from the module that declares it:
`std.math.int`'s programs list `int-add`, `int`, … from `std.math.int` and `n` from
`std.math.decimal-arith`, since `std.math.int` only imports `n` and nothing is re-exported. A
short name a program writes and lists from no import is the program's own,
whatever a module calls the same (shadowing); one short name bound to two full
names in one file is refused. A module's entry rule (`decimal-arith`, `int-arith`,
`rat-arith`, `arith9`) may still be listed, and brings nothing a whole import
does not. `std.math.digits` is the one `std.*` module that holds facts
([§ `std.math.digits`](#stdmathdigits-a-library-of-facts-m1f-s1f62)), which an import
brings with the module. `:as` and `:symbols` are mutually exclusive. A
module's name is its path: `std.x.y` under this directory, any other name a
path from the **program file's** directory, a dot per directory, `..`
refused. The whole rule, the refusals and what prints are
[`01_grammar.md` § Import](../docs/kernel/ir/03-ein-lang/01_grammar.md#import)
and `defined_behaviour.md` § 4.28.

To inline a puzzle's imports into a single standalone file — **there is no CLI
for this.** `ein ir parse --resolve` did it from P1.8 until **P1.11**
(2026-06-16, `8378ad7`), which dropped the whole `ein ir` / `ein kb`
inspection family; what survived is the library call, and it is three
functions:

```rust
let forms = ein_ir::parse(&mut ast, &text, Some(path))?;
let inlined = ein_ir::resolve_imports(&mut ast, &forms, path.parent())?;
print!("{}", ein_ir::dump_canonical(&ast, &inlined));
```

The inlined text is the program as resolved, every module whole under its full
names; a name the program declares keeps its name, so the text loads with
nothing left to resolve against. Until M1f S1f.6.35 the route was
`resolve_and_minimize`, which shook out an imported declaration no live name
or premise reached (S1f.6.25's W91 kept a listed one); every import brings its
module whole since, and nothing is shaken. `dump_canonical` is the canonical
`.ein` printer, and the parser's unescape is its inverse, so the text
re-parses.

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
