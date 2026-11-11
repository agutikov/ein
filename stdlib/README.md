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
`ein test` alike. No `absent`, no `not` premise, one stratum; one rule
concludes `(false)`, from a division by zero. Ein still has no
numbers (the user's ruling of 2026-11-06): nothing in the engine knows what
`7` is.

**Addition** (S1f.6.3), **subtraction and comparison** (S1f.6.4),
**multiplication** (S1f.6.5) and **division** (S1f.6.6): the four operations
and comparison, 88 rules and the closing rule.

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
| **division** | `(div X Y)` is answered by `(quot-of X Y Q R)`, the quotient and the remainder: X = Q × Y + R, and R less than Y |
| **division by zero** | a contradiction, **in two steps** (the user's ruling of 2026-11-11): a request whose divisor is zero, however written, derives the flag `(division-by-zero (div X Y))`, and the flag derives `(false)`. Root is dead, R is empty, and no query of the program has an answer; `ein solve` names the request in the unsat core and `division-by-zero-is-false` as what refuted it. A program that must not die compares its divisor with zero first and derives its `div` request by a rule over `(cmp-of Y z gt)` |
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

**Division is school long division, the quotient's digit by bisection.** The
divisor's multiples by 1 to 9 are computed once a division, each a row of
multiplication, in a chain whose tags hold the multiples already known, and
travel on as one fact, `(multiples M1 … M9)`; the first, the divisor times 1,
is the divisor in normal form and shows whether it is zero. The dividend is
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
output, taken on 2026-11-11 through `utils/bench_env.sh` on one P-core of an
i9-14900HX under the `powersave` governor (`utils/bench_env.sh python3
utils/arith_cost.py`, some ten minutes; `--quick` is the first two sections).
*wall* is the whole process and *rss* its peak resident memory.

`ein 0.1.0`, best of 5, seed 20261111; `python3 utils/arith_cost.py`

#### What a program pays before its first request

| program | text | parse ms | load ms | saturate ms | firings | facts | rss MB | wall ms |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| no import: one relation | 46 B | 1.02 | 0.13 | 0.01 | 0 | 2 | 17 | 3.99 |
| `std.digits` alone, the seven tables | 54 B | 1.05 | 14.3 | 0.01 | 0 | 1 678 | 17 | 18.9 |
| `std.arith`, the theory and the tables | 36 B | 1.07 | 20.6 | 0.29 | 1 | 1 975 | 17 | 25.8 |
| `std.arith` and 17 + 5 | 64 B | 1.08 | 20.6 | 0.31 | 14 | 1 989 | 17 | 25.9 |

#### The three examples

`ein saturate FILE`, the program as text (`add.ein`, one sum and one by names, for scale):

| example | text | parse ms | load ms | saturate ms | firings | facts | rss MB | wall ms |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| `add.ein` | 3 KB | 1.32 | 20.5 | 0.34 | 37 | 2 015 | 17 | 26.2 |
| `long_numbers.ein` | 6 KB | 3.21 | 20.8 | 4.92 | 2 604 | 4 589 | 17 | 34.1 |
| `many_calculations.ein` | 8 KB | 5.00 | 20.9 | 13.7 | 7 409 | 9 503 | 21 | 46.7 |
| `factorial.ein` | 4 KB | 1.45 | 20.5 | 4.71 | 2 635 | 4 639 | 17 | 31.6 |

The same programs saved by `ein kb save FILE OUT.einb`, under `ein saturate OUT.einb`, which opens the container where the text is parsed and loaded:

| example | .einb | open ms | saturate ms | firings | wall ms | wall ms as text |
|---|---:|---:|---:|---:|---:|---:|
| `add.ein` | 209 KB | 21.9 | 0.34 | 37 | 26.2 | 26.2 |
| `long_numbers.ein` | 257 KB | 23.8 | 4.70 | 2 604 | 34.3 | 34.1 |
| `many_calculations.ein` | 230 KB | 25.7 | 13.4 | 7 409 | 46.8 | 46.7 |
| `factorial.ein` | 209 KB | 22.1 | 4.38 | 2 635 | 32.0 | 31.6 |

The whole process, by command:

| example | saturate | solve | test | saturate .einb | solve .einb |
|---|---:|---:|---:|---:|---:|
| `add.ein` | 26.2 | 26.9 | 26.8 | 26.2 | 26.9 |
| `long_numbers.ein` | 34.1 | 72.9 | 72.9 | 34.3 | 73.0 |
| `many_calculations.ein` | 46.7 | 91.6 | 92.3 | 46.8 | 91.4 |
| `factorial.ein` | 31.6 | 49.0 | 48.7 | 32.0 | 48.8 |

#### One request, by the length of its operands

A dividend has twice the digits of its divisor. A written number nests a level a digit, and the parser nests to 1 000, so 900 digits is near the longest number a program can write.

| request | text | parse ms | load ms | saturate ms | firings | facts | rss MB | wall ms |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| add, 9 digits | 154 B | 1.15 | 20.6 | 0.33 | 28 | 2 003 | 17 | 26.0 |
| add, 30 digits | 406 B | 1.30 | 20.7 | 0.38 | 70 | 2 045 | 17 | 26.3 |
| add, 100 digits | 1 KB | 1.82 | 20.6 | 0.54 | 210 | 2 185 | 17 | 27.0 |
| add, 300 digits | 4 KB | 3.36 | 20.9 | 1.13 | 610 | 2 585 | 17 | 29.7 |
| add, 900 digits | 11 KB | 7.89 | 20.9 | 2.68 | 1 810 | 3 785 | 17 | 35.6 |
| sub, 9 digits | 154 B | 1.14 | 20.6 | 0.34 | 38 | 2 014 | 17 | 26.1 |
| sub, 30 digits | 406 B | 1.31 | 20.7 | 0.43 | 101 | 2 077 | 17 | 26.4 |
| sub, 100 digits | 1 KB | 1.82 | 20.6 | 0.70 | 311 | 2 287 | 17 | 27.2 |
| sub, 300 digits | 4 KB | 3.31 | 20.9 | 1.56 | 911 | 2 887 | 17 | 30.1 |
| sub, 900 digits | 11 KB | 8.03 | 21.1 | 4.40 | 2 711 | 4 687 | 17 | 38.7 |
| cmp, 9 digits | 154 B | 1.14 | 20.6 | 0.32 | 17 | 1 992 | 17 | 26.0 |
| cmp, 30 digits | 406 B | 1.29 | 20.4 | 0.36 | 38 | 2 013 | 17 | 25.9 |
| cmp, 100 digits | 1 KB | 1.82 | 20.3 | 0.49 | 108 | 2 083 | 17 | 26.7 |
| cmp, 300 digits | 4 KB | 3.38 | 21.0 | 0.91 | 308 | 2 283 | 17 | 29.6 |
| cmp, 900 digits | 11 KB | 7.89 | 21.2 | 2.06 | 908 | 2 883 | 17 | 35.4 |
| mul, 9 digits | 154 B | 1.14 | 20.6 | 1.01 | 490 | 2 465 | 17 | 26.9 |
| mul, 30 digits | 406 B | 1.29 | 20.7 | 7.90 | 4 773 | 6 748 | 19 | 35.5 |
| mul, 100 digits | 1 KB | 1.87 | 20.7 | 83.4 | 50 890 | 52 865 | 60 | 124 |
| mul, 300 digits | 4 KB | 3.46 | 21.0 | 832 | 452 675 | 454 650 | 446 | 997 |
| div, 18 by 9 | 208 B | 1.19 | 20.5 | 2.28 | 1 158 | 3 133 | 18 | 28.4 |
| div, 60 by 30 | 586 B | 1.42 | 20.6 | 16.9 | 9 349 | 11 324 | 23 | 45.8 |
| div, 200 by 100 | 2 KB | 2.25 | 20.7 | 175 | 91 659 | 93 634 | 111 | 233 |
| div, 600 by 300 | 5 KB | 4.57 | 20.9 | 1 626 | 768 655 | 770 630 | 818 | 1 973 |

#### Many requests on nine-digit operands, by their count

| requests | text | parse ms | load ms | saturate ms | firings | facts | rss MB | wall ms | firings a request |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| 100 x add | 12 KB | 8.52 | 20.9 | 3.98 | 2 701 | 4 775 | 18 | 38.7 | 27.0 |
| 1 000 x add | 115 KB | 75.4 | 23.6 | 38.8 | 27 001 | 29 975 | 44 | 149 | 27.0 |
| 10 000 x add | 1.1 MB | 748 | 51.9 | 502 | 270 001 | 281 975 | 358 | 1 412 | 27.0 |
| 100 x sub | 12 KB | 8.33 | 20.7 | 5.79 | 3 701 | 5 875 | 22 | 40.2 | 37.0 |
| 1 000 x sub | 115 KB | 75.4 | 23.6 | 61.8 | 37 001 | 40 975 | 58 | 176 | 37.0 |
| 10 000 x sub | 1.1 MB | 741 | 52.0 | 720 | 370 001 | 391 975 | 464 | 1 648 | 37.0 |
| 100 x cmp | 12 KB | 8.52 | 20.8 | 2.78 | 1 601 | 3 675 | 23 | 36.9 | 16.0 |
| 1 000 x cmp | 115 KB | 75.6 | 23.5 | 28.0 | 16 001 | 18 975 | 33 | 135 | 16.0 |
| 10 000 x cmp | 1.1 MB | 737 | 50.7 | 312 | 160 001 | 171 975 | 230 | 1 163 | 16.0 |
| 100 x mul | 12 KB | 8.06 | 19.8 | 78.1 | 48 683 | 50 757 | 62 | 123 | 486.8 |
| 1 000 x mul | 115 KB | 73.3 | 23.2 | 952 | 486 301 | 489 275 | 505 | 1 221 | 486.3 |
| 4 000 x mul | 461 KB | 294 | 31.8 | 4 252 | 1 945 067 | 1 951 041 | 2131 | 5 314 | 486.3 |
| 100 x div | 12 KB | 8.00 | 19.7 | 95.9 | 58 853 | 60 927 | 93 | 143 | 588.5 |
| 1 000 x div | 115 KB | 71.2 | 22.6 | 1 247 | 588 598 | 591 572 | 777 | 1 643 | 588.6 |
| 4 000 x div | 461 KB | 281 | 31.6 | 5 715 | 2 353 850 | 2 359 824 | 2790 | 7 623 | 588.5 |

#### The factorial chain, by its last number

| chain | text | parse ms | load ms | saturate ms | firings | facts | rss MB | wall ms |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| 25!, 26 digits | 4 KB | 1.29 | 19.6 | 4.16 | 2 635 | 4 639 | 23 | 29.9 |
| 50!, 65 digits | 4 KB | 1.45 | 19.6 | 15.7 | 10 575 | 12 604 | 31 | 44.1 |
| 100!, 158 digits | 5 KB | 1.96 | 19.6 | 73.7 | 46 915 | 48 994 | 70 | 114 |
| 200!, 375 digits | 6 KB | 2.78 | 19.7 | 599 | 317 325 | 319 504 | 382 | 766 |

#### `ein solve` beside `ein saturate`

The same program, the whole process. `ein solve` saturates once, as `ein saturate` does, and then reads the state out. A run past 120 s is not waited for.

| program | firings | facts | saturate wall ms | solve wall ms | solve / saturate |
|---|---:|---:|---:|---:|---:|
| `examples/arith/add.ein` | 37 | 2 015 | 25.4 | 25.6 | 1.0 |
| `examples/arith/long_numbers.ein` | 2 604 | 4 589 | 33.3 | 71.9 | 2.2 |
| `examples/arith/many_calculations.ein` | 7 409 | 9 503 | 45.1 | 91.7 | 2.0 |
| `examples/arith/factorial.ein` | 2 635 | 4 639 | 31.8 | 48.0 | 1.5 |
| add, 100 digits | 210 | 2 185 | 25.5 | 35.8 | 1.4 |
| add, 300 digits | 610 | 2 585 | 27.9 | 120 | 4.3 |
| add, 900 digits | 1 810 | 3 785 | 34.5 | 1 030 | 29.8 |
| mul, 30 digits | 4 773 | 6 748 | 34.5 | 143 | 4.2 |
| mul, 100 digits | 50 890 | 52 865 | 125 | 4 021 | 32.3 |
| div, 60 by 30 | 9 349 | 11 324 | 45.9 | 638 | 13.9 |
| div, 200 by 100 | 91 659 | 93 634 | 228 | 20 817 | 91.3 |
| 1 000 x add, nine digits | 27 001 | 29 975 | 146 | 279 | 1.9 |
| 10 000 x add, nine digits | 270 001 | 281 975 | 1 386 | 2 772 | 2.0 |
| 100 x div, nine digits | 58 853 | 60 927 | 166 | 1 087 | 6.6 |
| 1 000 x div, nine digits | 588 598 | 591 572 | 1 670 | 11 189 | 6.7 |
| 50! | 10 575 | 12 604 | 47.9 | 229 | 4.8 |
| 100! | 46 915 | 48 994 | 124 | 2 278 | 18.4 |

**What it says, each with its number from the tables above.**

- **Up to about a hundred requests the tables are the cost.** A program that
  imports the theory pays 20.6 ms of `kb load` before its first request, 14.3
  of it the 1 664 rows of `std.digits` and the rest the theory's 89 rules and
  67 relations, where a program with no import loads in 0.13 ms and the whole
  process takes 4.0 ms. One sum adds 0.02 ms of saturation to that.
  `many_calculations.ein`, a hundred requests, is the first size where the
  calculation, 13.7 ms, is of the load's order.
- **A `.einb` does not buy the load back.** `ein kb save` caches a whole
  program with its imports resolved, and `add.ein` saved so opens in 21.9 ms
  where its text parses and loads in 21.8: the container holds the program's
  1 800 facts, and opening it interns each again. It did buy it back while the theory read two
  tables: on the stdlib of `277cfc44`, addition alone, a sum's `.einb` solved
  in 6.7 ms against 18.1 as text (`ein kb save`, `ein solve`, best of 20,
  2026-11-11). A library has no cache of its own (E-15 and E-25 of the
  phase's
  [`findings.md`](../plans/m1f_hypothesis_and_documentation/p1f.6_calculator/findings.md)).
- **Past a thousand sums, parsing the program's numbers costs more than
  adding them.** 10 000 nine-digit sums are 1.1 MB of text: 748 ms to parse
  and 502 ms to saturate. A written number is one nested form a digit, the
  parser's worst input so far (E-16). For a product or a quotient the
  calculation leads from the start: 4 000 nine-digit products parse in 294 ms
  and saturate in 4.3 s.
- **A batch costs in proportion to its count,** as the rule of the theory
  promises: 27.0, 37.0, 16.0, 486 and 589 firings a request for a sum, a
  difference, a comparison, a product and a quotient of nine-digit numbers,
  the same at 100 requests and at 10 000, and about two microseconds a
  firing.
- **By length:** a sum, a difference and a comparison cost a few firings a
  digit, 1 810, 2 711 and 908 at 900 digits, under 5 ms each, where the
  number's own text takes 7.9 ms to parse. A product costs the two lengths
  multiplied, 452 675 firings and 0.83 s at 300 digits by 300, and a quotient
  of 600 digits by 300 is 768 655 firings and 1.6 s.
- **A chain needs no text.** `factorial.ein`'s program for 200! is 6 KB and
  parses in 2.8 ms; its 317 325 firings take 0.6 s and end on a number of 375
  digits that no line of the program writes.
- **Memory is a kilobyte a stored fact.** Every step of a circuit is a fact
  that holds its whole state, and nothing is forgotten: 4 000 products are
  1.95 million facts and 2.1 GB resident, and 10 000 were 4.9 million and
  6.0 GB (2026-11-11, the first run of the sweep, which now stops at 4 000
  for a product and a quotient; E-26).
- **`ein solve` writes the state, and the state as text is what explodes.**
  `ein solve` saturates once, as `ein saturate` does, and then reads the state
  out; `ein test` does the same. Stored, a circuit's facts share their
  numbers; written, each fact repeats its own. A 900-digit sum is 34.5 ms
  under `saturate` and 1.0 s under `solve`; a 100-digit product 125 ms and
  4.0 s; a quotient of 200 digits by 100, 228 ms and 20.8 s (E-24). It is why
  the shipped examples are the sizes they are: every corpus program is
  solved, dumped and reloaded per commit.

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

**3. Each step holds its numbers whole.** One request of `std.arith`, its
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
module says, and its header has the list: `z`, `n`, `b`, `let`; the requests
`add`, `sub`, `cmp`, `mul` and `div`; the results `sum-of`, `diff-of`,
`cmp-of`, `product-of` and `quot-of`; the flag `division-by-zero`; the orders
`lt`, `eq` and `gt`; some sixty names of the circuit's own, its walks, its
hand-overs and its tags; `arith-rule`; and the seven tables of `std.digits`.
Only `n`, `z`, `let`, the requests, the results, the flag and the three orders
are its interface. What a clash does, and what a malformed number does, is the
phase's S1f.6.12.

## `std.num9`: nine digits in one fact, and wide rules (M1f S1f.6.13)

The calculator's second module, and the user's own number (the instruction of
2026-11-08; the ruling of 2026-11-11, *"yes, ship num9 in stdlib, why not"*):
`(num9 0 0 0 1 2 3 0 7 8)` is 123078, nine digit names, the most significant
first, a fixed width. The theory is the same circuit over the same tables of
`std.digits`, wired the other way: where `std.arith` walks a number one column
a firing, a rule here has **a premise a column**, so a sum is one firing.
Computed by saturation alone: no candidate, R {∅}, no `absent`, no `not`, no
guard, one stratum.

```lisp
(import std.num9 :symbols (arith9))

(add9 (num9 0 0 0 1 2 3 0 7 8) (num9 0 0 0 0 0 4 5 6 7))

(query :goal (sum9-of ?x ?y ?s))      ; ?s = (num9 0 0 0 1 2 7 6 4 5)
```

**The file is generated,** by
[`utils/gen_num9.py`](../utils/gen_num9.py), as `std.digits` is: nine columns
of variables twenty times over are not for typing or for reviewing by eye.
`--check` is in the gate (`ein-infer/tests/num9_theory.rs`); `--width W
--stdout` prints the same theory at another width, which is how the harness
holds it exhaustively, and no other width is shipped. 47 rules and 22
relations (`grep -c '^(rule' stdlib/num9.ein`, `grep -c '^(relation'`,
2026-11-11). **A wide rule is wide in registers too:** a rule binds its
variables to a file of 256 (`ein_infer::MAX_REGS`), and where the widest rule
of every other module binds 16, a sum here binds 35 and `div9-start` 162.
`ein-infer/tests/compile_limits.rs` holds that number, and the question it
raises about the bound is the user's (Q-M1f.173).

| | |
|---|---|
| **the protocol** | as `std.arith`'s, under names of its own, so the two import flat into one program: `(add9 X Y)` answered by `(sum9-of X Y S)`; `(sub9 X Y)` by `(diff9-of X Y D R)`, two numbers, one of them zero; `(cmp9 X Y)` by `(cmp9-of X Y O)`, O one of `lt`, `eq`, `gt`; `(mul9 X Y)` by `(product9-of X Y P)`; `(div9 X Y)` by `(quot9-of X Y Q R)`. An operand is a number written out: the module has no named constants |
| **overflow** | **a fact, and no number.** A sum or a product that does not fit nine digits derives `(overflow REQUEST)`, the request as written, and no result fact: never a truncated number. Root stays alive, the program's other requests are answered, and a rule of the program may match the flag. A difference, a comparison and a quotient cannot overflow |
| **division by zero** | a contradiction, as in `std.arith` and **by its rule**: `div9-by-zero` derives `(division-by-zero (div9 X ZERO))`, and `std.arith`'s `division-by-zero-is-false`, which this module imports, derives `(false)`. Root is dead and R is empty |
| **conversions** | a request each way, one firing: `(to9 X)`, X a number of `std.arith`, is answered by `(as9 X W)`, or by `(big X)` where X is written with more than nine digits; `(from9 W)` by `(of9 W X)`, X in normal form. Nothing converts a number because it exists, and no rule of either module hands a request to the other: a program that wants both writes the rules that ask ([`examples/arith/num9_both_ways.ein`](../examples/arith/num9_both_ways.ein)) |
| **the one entry** | `arith9`, the closing rule: it names every rule and closes every relation of the module, the six tables it reads, and the two relations it takes from `std.arith` |

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

**No rule joins two facts on a number,** `std.arith`'s rule, kept here and
held by the same two tests (`num9_theory.rs` on the text, `arith_batch.rs` on
the matcher's count). It is what the design's prototype did not do: its step
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

| operation | requests | std.num9: firings a request | ms | std.arith: firings a request | ms |
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

- **`std.num9` where every number fits nine digits and there are many of
  them.** One firing for a sum against 22: 4 000 sums in 24 ms against 113,
  differences in 24 against 171, comparisons in 38 against 82, quotients in
  1.24 s against 1.78. Products are level, 331 ms against 300: a product
  here always takes its nine steps, and `std.arith` takes as many as the
  multiplier has digits. A trace of a sum is one step with its nine table
  rows, where `std.arith`'s is twenty-one.
- **`std.num9` where a fixed width is what is modelled:** a register, a
  counter, a field of a form, with the overflow as a fact a rule can read.
- **`std.arith` for numbers of any length, for named constants, and for a
  program with a request or two.** A program that imports `std.num9` pays
  29.3 ms of `kb load` where one that imports `std.arith` pays 19.6: 14.3 for
  `std.digits`, about ten for the module's own 46 KB of wide rules, and five
  for the text of `std.arith`, read whole for the three names taken from it
  (`ein saturate tests/stdlib/num9/01_add.ein`, best of five, 2026-11-11).
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
[`tests/stdlib/num9/`](../tests/stdlib/num9/) and the three examples
search-free, and `answer_round_trip.rs` reloads each one's state.
[`examples/arith/num9.ein`](../examples/arith/num9.ein) is the program to
read first, then [`num9_overflow.ein`](../examples/arith/num9_overflow.ein)
and [`num9_both_ways.ein`](../examples/arith/num9_both_ways.ein).

**The names it occupies:** `num9`; the requests `add9`, `sub9`, `cmp9`,
`mul9`, `div9`, `to9` and `from9`; the results `sum9-of`, `diff9-of`,
`cmp9-of`, `product9-of`, `quot9-of`, `as9` and `of9`; the flags `overflow`
and `big`; the states `mul9-at` and `div9-at`, with `multiples9` and `row9`;
`num9-rule` and the entry `arith9`. From `std.arith` it takes `n`, with `z`,
and `division-by-zero` with the rule from it to `(false)`; from `std.digits`,
`fa`, `fs`, `md`, `cl`, `nz` and `adj`, with the three orders. A malformed
`num9`, a digit's place that holds something else or a count of digits that
is not nine, is answered by silence today, as in `std.arith`: the phase's
S1f.6.12.

## `std.int`: integers as a positive and a negative part (M1f S1f.6.7)

The calculator's first extension, and the user's proposal of 2026-11-08:
*"numbers containing 2 parts - positive and negative, that can be simplified
until one of them is zero"*. An integer is `(int P N)`, two natural numbers of
`std.arith`, and its value is P less N. The module is written over `std.arith`,
which it calls and does not edit, and is computed by saturation alone: no
candidate, R {∅}, no `absent`, no `not`, no guard, one stratum.

```lisp
(import std.int :symbols (int-arith))

(int-add (int z (n (n z 1) 2)) (int (n (n z 4) 0) z))     ; -12 + 40

(query :goal (int-sum-of ?x ?y ?s))      ; ?s = (int (n (n z 2) 8) z)
```

| | |
|---|---|
| **the integer** | `(int P N)`. 7 is `(int (n z 7) z)`, −7 is `(int z (n z 7))`, zero is `(int z z)`, and `(int (n z 5) (n z 3))` is 2, not simplified. The **normal form** has one part `z` and the other in `std.arith`'s normal form; every integer has exactly one. An operand is read by value, whatever its parts and their high zeros, and every answer is in normal form, so a rule of a program tells a negative answer by its shape, `(int z (n ?p ?a))`, with no negation written |
| **the protocol** | `(int-simplify X)` answered by `(int-simple-of X S)`; `(int-neg X)` by `(int-neg-of X S)`; `(int-add X Y)` by `(int-sum-of X Y S)`; `(int-sub X Y)` by `(int-diff-of X Y S)`, **one integer**, where `std.arith`'s `diff-of` answers two numbers: they are its two parts; `(int-mul X Y)` by `(int-product-of X Y S)`; `(int-cmp X Y)` by `(int-cmp-of X Y O)`, O one of `lt`, `eq`, `gt`, by value |
| **an operand** | an integer written out, or a name a `(let NAME (int P N))` states: `let` is `std.arith`'s. A natural number is **not** an integer operand, and such a request is never answered (Q-M1f.175) |
| **division** | **not built.** Which remainder a division of integers has, truncated, floored or Euclidean, is the user's question, [Q-M1f.179](../plans/m1f_hypothesis_and_documentation/open_questions.md#q-m1f179--the-remainder-of-an-integer-division); the stage's record has each of the three as rules, run and compared |
| **the one entry** | `int-arith`, the closing rule: it closes every relation of the module, names every rule, and names `arith`, so the one name brings `std.arith` and the tables too |

**How it computes.** Each operation works on the parts with `std.arith`'s
walks and then simplifies. Simplifying is subtraction both ways at once, P − N
and N − P: the walk that ends with no borrow hands over the part that is left.
A sum is the positive parts added and the negative parts added; a difference
is a sum with the second operand's parts exchanged, and a negation a
simplification with them exchanged. A product is (P1·P2 + N1·N2, P1·N2 +
N1·P2), four products of `std.arith` and two sums, and **no rule looks at a
sign**. A comparison adds across, P1 + N2 against N1 + P2, and compares two
natural numbers; it builds no integer. **A call's tag is its request**:
`(int-add X Y)` is the tag of its own computation. The calls a later theory
makes are `(int-norm TAG P N)`, `(int-add-call TAG P1 N1 P2 N2)` and
`(int-mul-call TAG P1 N1 P2 N2)`, each answered by `(int-done TAG INTEGER)`,
and `(int-cmp-call TAG P1 N1 P2 N2)`, answered by `std.arith`'s `(order-is
TAG ORDER)`.

**Every rule has one premise,** which is the rule of the theory in its
strongest form: none reads a table itself and none joins two facts.
`ein-infer/tests/int_theory.rs` holds that on the text, and holds the answers
to Rust's signed integers: every pair of integers written with parts from 0 to
9 under every operation, and samples at every length to 30 digits in every
combination of signs. A batch costs in proportion to its count: 476, 480,
3 931 and 870 candidates a request for a sum, a difference, a product and a
comparison at 250 requests, and no more at 1 000 and 4 000 (`cargo test -p
ein-infer --features counters --test arith_batch a_batch_of_integer --
--nocapture`, 2026-11-12).

**What it costs beside `std.arith`.** The same operation asked of both, the
integers' on operands of up to nine digits and either sign (four digits for a
product), `std.arith`'s on their sizes; the saturator's own milliseconds, best
of five (`python3 build/s1f.6.7/batch.py`, 2026-11-12):

| operation | requests | std.int: firings a request | ms | std.arith on the sizes: firings a request | ms |
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
program that imports the module pays 22.4 ms of load where `std.arith`'s pays
19.9 (`ein saturate examples/arith/int.ein` and `add.ein`, the `kb load:`
line, best of five, 2026-11-12).

**The names it occupies** are in the module's header: `int`, the six requests
and six results, the four calls and `int-done`, fifteen tags, `int-rule` and
`int-arith`; everything of `std.arith` comes with it.

## `std.rat`: rational numbers, in lowest terms (M1f S1f.6.8)

The calculator's second extension, and the user's proposal of 2026-11-08:
rational numbers *"containing integer numerator and natural denominator"*. A
rational is `(rat Z D)`, an integer of `std.int` over a natural number of
`std.arith` that is not zero. The module is written over the two theories
below it, which it calls and does not edit, and is computed by saturation
alone: no candidate, R {∅}, no `absent`, no `not`, no guard, one stratum.

```lisp
(import std.rat :symbols (rat-arith))

(rat-add (rat (int (n z 1) z) (n z 2)) (rat (int (n z 1) z) (n z 3)))     ; 1/2 + 1/3

(query :goal (rat-sum-of ?x ?y ?s))      ; ?s = (rat (int (n z 5) z) (n z 6))
```

| | |
|---|---|
| **the rational** | `(rat Z D)`. 1/2 is `(rat (int (n z 1) z) (n z 2))`, −3/4 is `(rat (int z (n z 3)) (n z 4))`. The **normal form** is in lowest terms: the numerator in `std.int`'s normal form, the denominator with no high zero, the two with no common divisor but 1, and zero written 0 over 1, `(rat (int z z) (n z 1))`. Every rational has exactly one. An operand is read by value, 2/4 as 1/2, and every answer is in normal form |
| **the protocol** | `(rat-simplify X)` answered by `(rat-simple-of X S)`; `(rat-neg X)` by `(rat-neg-of X S)`; `(rat-add X Y)` by `(rat-sum-of X Y S)`; `(rat-sub X Y)` by `(rat-diff-of X Y S)`; `(rat-mul X Y)` by `(rat-product-of X Y S)`; `(rat-div X Y)` by `(rat-quot-of X Y S)`, **one rational, exact, and no remainder**; `(rat-cmp X Y)` by `(rat-cmp-of X Y O)`, O one of `lt`, `eq`, `gt`, by value |
| **an operand** | a rational written out, `(rat (int P N) D)`, or a name a `(let NAME (rat (int P N) D))` states. An integer or a natural number is **not** a rational operand (Q-M1f.181) |
| **a zero denominator** | **a division by zero, and a contradiction**, as in `std.arith` and by its rule. A request any operand of which has a denominator of zero, however the zero is written, and a `rat-div` by a rational that is zero, derive `(division-by-zero REQUEST)`, the request as written, and `std.arith`'s `division-by-zero-is-false` derives `(false)`. Root is dead and R is empty (Q-M1f.182) |
| **the one entry** | `rat-arith`, the closing rule: it closes every relation of the module, names every rule, and names `int-arith` and `arith`, so the one name brings all three theories and the tables |

**How it computes.** The operands are looked up and each denominator is
compared with zero by `std.arith`'s walk. A sum of P1 less N1 over D1 and P2
less N2 over D2 is the **cross products**, P1·D2 + P2·D1 less N1·D2 + N2·D1,
over D1·D2: five products and two sums of natural numbers; a difference is
the same with the second numerator's parts exchanged. **A comparison is a
difference's numerator and nothing more**: its positive part against its
negative part, since the denominators are positive. A product is the
numerators' product, `std.int`'s, over the denominators'; a quotient is a
product with the divisor turned over, its sign kept in the numerator, and a
divisor that is zero has no turning over. Then **lowest terms**: the numerator
simplified by `std.int`, the greatest common divisor of its size and the
denominator by **Euclid's chain**, gcd(A, B) = gcd(B, A mod B) until the
remainder is zero, each step a division of `std.arith`, and both divided by
what it finds. **It does not need a division of integers**, so it does not
wait for Q-M1f.179. A call's tag is its request, as in `std.int`; the calls a
later theory makes are `(rat-norm TAG P N D)` and `(rat-mul-call TAG …)`,
answered by `(rat-done TAG RATIONAL)`, and `(rat-cross TAG …)`, answered by
`(rat-cross-is TAG POSITIVE NEGATIVE)`.

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

Most of a request is lowest terms, and most of that is `std.arith`'s
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
27.0 ms of load where `std.int`'s pays 22.4 and `std.arith`'s 19.9 (`ein saturate
examples/arith/rat.ein`, `int.ein` and `add.ein`, the `kb load:` line,
2026-11-12).

**Nothing limits the length.** Cross products are as long as their operands
together, and nested numbers hold them: two fractions of twelve-digit terms
are added and compared in `rat_theory.rs`. What limits a run is what limits
`std.arith`'s division, the size of the state (§ [How long a number may
be](#how-long-a-number-may-be-m1f-s1f69)).

**The names it occupies** are in the module's header: `rat`, the seven
requests and seven results, the three calls and their two hand-overs, the
circuit's own five relations, the signs `pos` and `neg`, twenty-two tags,
`rat-rule` and `rat-arith`; everything of `std.int` and `std.arith` comes with
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
them activated — some seconds, and **307 of 307** since `std.arith`'s 89
(M1f S1f.6.3 to S1f.6.6), `std.num9`'s 47 (S1f.6.13), `std.int`'s 37
(S1f.6.7) and `std.rat`'s 57 (S1f.6.8) joined the 77, which
is a number this page may
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
| `std.arith` | [`arith.ein`](arith.ein) | school arithmetic on natural numbers of any length, `z` and `(n PREFIX DIGIT)`, **computed by saturation alone** over `std.digits`' tables: a request `(add X Y)` answered by `(sum-of X Y S)`, named constants `(let NAME NUMBER)`; no rule joins two facts on a number; one entry, the closing rule `arith`. Addition, subtraction with its two numbers `(diff-of X Y D R)`, comparison `(cmp-of X Y lt|eq|gt)`, multiplication `(product-of X Y P)` and division `(quot-of X Y Q R)`, a division by zero a flag and `(false)` from it ([§ `std.arith`](#stdarith-school-arithmetic-computed-by-saturation-alone-m1f-p1f6)) | M1f S1f.6.3 to S1f.6.6 |
| `std.num9` | [`num9.ein`](num9.ein) | school arithmetic on numbers of nine digits in one fact, `(num9 d8 … d0)`, **computed by saturation alone**, by wide rules, a premise a column: `add9`/`sum9-of`, `sub9`/`diff9-of`, `cmp9`/`cmp9-of`, `mul9`/`product9-of`, `div9`/`quot9-of`; `(overflow REQUEST)` and no number where an answer does not fit; conversions to and from `std.arith`'s numbers, `to9`/`as9` and `from9`/`of9`. **Generated** by `utils/gen_num9.py`. One entry, `arith9` | S1f.6.13 |
| `std.int` | [`int.ein`](int.ein) | integers as a positive and a negative part, `(int P N)`, over `std.arith`'s natural numbers, **computed by saturation alone**: `int-simplify`/`int-simple-of`, `int-neg`/`int-neg-of`, `int-add`/`int-sum-of`, `int-sub`/`int-diff-of`, `int-mul`/`int-product-of`, `int-cmp`/`int-cmp-of`, every answer in normal form, one part `z`; every rule has one premise; integer division waits for the user's ruling on its remainder ([§ `std.int`](#stdint-integers-as-a-positive-and-a-negative-part-m1f-s1f67)); imported by its one entry, `(import std.int :symbols (int-arith))`, which brings `std.arith` too | M1f S1f.6.7 |
| `std.rat` | [`rat.ein`](rat.ein) | rational numbers, `(rat Z D)`, an integer of `std.int` over a natural number that is not zero, **computed by saturation alone** and answered in lowest terms: `rat-simplify`/`rat-simple-of`, `rat-neg`/`rat-neg-of`, `rat-add`/`rat-sum-of`, `rat-sub`/`rat-diff-of`, `rat-mul`/`rat-product-of`, `rat-div`/`rat-quot-of` (one rational, no remainder), `rat-cmp`/`rat-cmp-of`; the greatest common divisor by Euclid's chain over `std.arith`'s division; a zero denominator is a division by zero, `std.arith`'s flag and its rule to `(false)`; every rule has one premise ([§ `std.rat`](#stdrat-rational-numbers-in-lowest-terms-m1f-s1f68)); imported by its one entry, `(import std.rat :symbols (rat-arith))`, which brings `std.int` and `std.arith` too | M1f S1f.6.8 |
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
