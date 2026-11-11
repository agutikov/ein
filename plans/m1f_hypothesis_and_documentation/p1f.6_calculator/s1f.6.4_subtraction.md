# S1f.6.4 — Subtraction with its two numbers, and comparison

**Phase:** [P1f.6](README.md)
**Status:** **done 2026-11-11**: 18 rules and 15 relations added to `stdlib/arith.ein`; `(sub X Y)` answered
by the two numbers of `(diff-of X Y D R)`, `(cmp X Y)` by `(cmp-of X Y lt|eq|gt)`, both held to Rust's integers,
search-free, and under the rule of the theory on the text and on the counter; four decisions taken by default,
Q-M1f.155 to .158, and one finding, E-12 ([§ T1 record](#t1-record-2026-11-11) onward). Planned 2026-11-08;
re-planned 2026-11-11 to [the rulings](design.md#the-rulings-2026-11-11).
**Depends on:** [S1f.6.3](s1f.6.3_addition.md).

## What it builds

**Subtraction** (the user's ruling on question 3, (a)). `(sub X Y)` is answered by `(diff-of X Y D R)`: D is
X − Y when X is not the smaller and `z` otherwise, and R is Y − X when Y is the larger and `z` otherwise. It is
the normal form S1f.6.7's negative numbers want. The circuit subtracts both ways with S1f.6.3's walk over the
`fs` table; the walk that ends without a borrow hands its answer over, and the one that ends with a borrow
derives `(short TAG)` and no number.

**Comparison** (the user: *"what we forgot from school arithmetic is at least numbers comparison"*).
`(cmp X Y)` is answered by `(cmp-of X Y lt)`, `eq` or `gt`: one walk from the units over the `cl` table, with no
answer to build. A number that has run out counts as zeros, so the comparison is by value and holds across
spellings with high zeros.

`le`, `ge`, `max` and `min` are not built unless a later stage needs one; each is a rule over `cmp-of`.

## Tasks

1. **The two walks of subtraction, and the walk of comparison,** each under S1f.6.3's rule of the theory.
2. **The tests,** as for addition: one program per rule, the harness test against the integers (X > Y, X < Y,
   X = Y, operands with high zeros, zero on either side), the search-free test, and S1f.6.3's test of the rule
   of the theory extended to these rules.
3. **The circuit's measure:** firings and depth of one subtraction and one comparison.
4. **The example:** 123078 − 4567, 4567 − 123078, and a comparison.

## Acceptance

- 123078 − 4567 is (118511, z) and 4567 − 123078 is (z, 118511).
- Every comparison agrees with the integers, and `(cmp 5 5)` written with and without high zeros is `eq`.
- The three tests pass.

## T1 record, 2026-11-11

**Subtraction, ten rules.** `(sub X Y)` looks its operands up as `add` does (`sub-ask`, `sub-ask-second`), and
`sub-start` begins **two walks at once**, `(sub-walk (difference X Y) U V 0 z)` and `(sub-walk (shortfall X Y) V
U 0 z)`: which number is the smaller is not known before the top column, and nothing is guessed. A walk is
`add-walk`'s over the `fs` table (`sub-column`, `sub-column-first`, `sub-column-second`); it ends with no borrow
and goes to the shared `strip` (`sub-end`), or ends with a borrow and derives `(short TAG)` and no number
(`sub-borrow-out`). `sub-answer` reads `(done (difference X Y) D)` and writes `(diff-of X Y D z)`;
`sub-answer-short` reads `(done (shortfall X Y) R)` and writes `(diff-of X Y z R)`.

- **Equal numbers** ([Q-M1f.155](../open_questions.md#q-m1f155--equal-operands-of-a-subtraction-two-walks-one-fact),
  defaulted): both walks end with no borrow, both hand over `z`, and both answer rules derive the one fact
  `(diff-of X Y z z)`. One answer, by two derivations.
- **A walk callable from inside the theory.** `(sub-walk TAG X Y 0 z)` answers `(done TAG D)` or `(short TAG)`,
  which is what S1f.6.6's bisection reads: *it fits* or *it borrows*.

**Comparison, eight rules.** `(cmp X Y)` looks its operands up (`cmp-ask`, `cmp-ask-second`), and `cmp-start`
begins `(cmp-walk (order X Y) U V eq)`: one column a firing from the units over the `cl` table, the fourth
argument the order of the digits already passed (`cmp-column`, `cmp-column-first`, `cmp-column-second`).
`cmp-end` hands over `(order-is TAG O)` and `cmp-answer` writes `(cmp-of X Y O)`. No number is built, so nothing
is stripped or turned.

- **The walk carries a tag**
  ([Q-M1f.156](../open_questions.md#q-m1f156--comparison-called-with-a-tag), defaulted), where the prototype's
  carried the operands and answered at once. It costs one firing a request and makes the walk a call like the
  others, for S1f.6.7 and S1f.6.8.
- **Each request has its own three operand rules and two tags**
  ([Q-M1f.157](../open_questions.md#q-m1f157--each-request-looks-its-operands-up-by-rules-of-its-own),
  defaulted), as [S1f.6.3's last section](s1f.6.3_addition.md#for-the-stages-that-follow) says. A shared pair
  lookup would save two rules an operation and cost every request two firings, addition's pinned 21 among them.

**The closing rule** closes the 15 new relations and the tables `fs` and `cl`, and names the 18 new rules;
`arith_theory.rs`'s test of the two lists passes unchanged. The module's import reads `(fa fs cl nz)`.

## T2 record, 2026-11-11

- **Five programs** under `tests/stdlib/arith/`, each claiming its result relation's whole extent and R = {∅}:
  `08_sub` (123078 − 4567 and the reverse, and `short`'s whole extent: of four walks, the two that borrowed),
  `09_sub_borrow_and_zeros`, `10_sub_named` (with a rule of the program's own that matches the shortfall),
  `11_cmp`, `12_cmp_lengths_and_names`. `stdlib_coverage.rs` holds that the module's 37 rules are each
  activated: 114 of 114 over the stdlib (`cat stdlib/*.ein | grep -c '^(rule '`, 2026-11-11).
- **The harness against the integers,** `arith_theory.rs`, is now a table of operations, `OPS`: a request, a
  result, and what `u128` says the answer is. Every operation is asked the same cases: every pair from 0 to
  120; every pair of lengths from 0 to 30 digits, twice, with **equal operands and operands one apart at every
  length**; 99…9 and 1, 100…0 and 1, 100…0 and 99…9, both ways round, at every length; 1 700 pairs padded with
  up to three high zeros, 200 of them the same value padded differently; and
  `named_operands_work_for_every_request`, 40 named constants asked by two names and by a name and a number.
  Each request has exactly one answer, in normal form.
- **The search-free test** reads `diff-of` and `cmp-of` beside `sum-of`: `RESULTS` in `arith_search_free.rs`.
- **The rule of the theory:** the static test reads the whole module and passed unchanged, the 18 new rules
  each with one premise that is not a table row. The measured test, `arith_batch.rs`, now measures **each
  operation in batches of its own**, so one operation's proportion is not hidden in another's count:

  | requests | `add` firings | candidates a request | `sub` firings | candidates a request | `cmp` firings | candidates a request |
  |---:|---:|---:|---:|---:|---:|---:|
  | 250 | 6 751 | 181.38 | 9 251 | 362.00 | 4 001 | 901.00 |
  | 1 000 | 27 001 | 181.38 | 37 001 | 362.00 | 16 001 | 901.00 |
  | 4 000 | 108 001 | 181.39 | 148 001 | 362.00 | 64 001 | 901.00 |

  (`cargo test -p ein-infer --features counters --test arith_batch -- --nocapture`, 2026-11-11.)
  `a_number_operand_does_not_walk_the_lets` holds each operation beside 200 `let`s.
- **E-12, found by that table:** a comparison costs 100 candidates a column where a sum costs 20. The matcher
  keys a premise on its **first** bound argument (`match_.rs`, `probe_key`), and `cl`'s first is the order below,
  which a hundred rows share; `fa`'s first is a digit, which twenty do. The cost is a constant a column and the
  proportion holds; the row is in [`findings.md`](findings.md), and the table's column order is left as S1f.6.2
  shipped it ([Q-M1f.158](../open_questions.md#q-m1f158--cls-column-order-and-the-index), defaulted).
- **The mutation sweep** kills every mutant of the 18 rules, 15 of 15: 203 of 253 where 188 of 238, the bank's
  50 survivors unmoved (`python3 utils/stdlib_mutants.py --check`, 2026-11-11).

## T3 record, 2026-11-11

`arith_theory.rs`'s `the_circuit_of_one_subtraction_and_one_comparison` pins the measure (`cargo test -p
ein-infer --test arith_theory the_circuit`):

| request | firings | depth | what they are |
|---|---:|---:|---|
| 123078 − 4567 | 28 | 21 | 5 to ask and find the operands; the walk that borrows, 6 columns and its end; beside it the walk that answers, 6 columns, its end, the check for a high zero, 6 digits turned, the hand-over and the answer |
| 4567 − 123078 | 28 | 21 | the same, the two walks exchanging roles |
| 123078 − 123078 | 35 | 20 | both walks end with no borrow, each drops six zeros and hands over `z`, and both answer rules fire |
| 123078 against 4567 | 13 | 13 | 5, 6 columns, the end, the answer |

A subtraction is two chains side by side, so its depth is less than its size.

On the release binary (`python3 build/s1f.6.4/batch.py`, 2026-11-11), random requests over two numbers of up to
nine digits in one program:

| | the saturator | firings | the process |
|---|---:|---:|---:|
| 250 / 1 000 / 4 000 `add` | 6.2 / 24.1 / 103 ms | 5 335 / 20 805 / 82 093 | 37 / 87 / 297 ms |
| 250 / 1 000 / 4 000 `sub` | 9.1 / 34.8 / 150 ms | 7 133 / 27 797 / 109 743 | 40 / 98 / 351 ms |
| 250 / 1 000 / 4 000 `cmp` | 5.0 / 18.4 / 73.7 ms | 3 291 / 12 841 / 50 625 | 35 / 79 / 257 ms |

**What a program pays to load** moved a little: every program that imports the theory now holds `fs`'s and
`cl`'s 500 rows beside `fa`'s. `ein test tests/stdlib/arith/01_add.ein` takes 20.7 ms where it took 18.1 ms on
the stdlib of `277cfc44` with the same binary, and `ein test tests/` 377 ms for twelve arithmetic programs where
349 ms (the same script, with `EIN_STDLIB` on a copy of the old stdlib for the second figure).

## T4 record, 2026-11-11

`examples/arith/sub.ein`: 123078 − 4567, 4567 − 123078, a `sub` and a `cmp` by two named constants, and a rule
of the program's own, `cannot-ship`, that matches `(diff-of ?have ?want z (n ?p ?a))`, a shortfall that is not
zero. Corpus group `positive`, runs `solve`, `test`, `saturate`, `saturate --dump`, `render rules` and
`solve --trace`.

**The sweep.** `python3 build/s1f.6.4/sweep.py` (2026-11-11): the release binary saved at `277cfc44` reading a
copy of the stdlib at `277cfc44` through `EIN_STDLIB`, against the tree's, `ein saturate --dump` and `ein solve`
over the 681 `.ein` files `277cfc44` tracks. **`ein solve` moved on no arithmetic program.** `ein saturate
--dump` moved on the eight programs that import `std.arith`, and only by what the theory now brings: the state
holds `fs`'s 200 rows and `cl`'s 300, 17 more `(__closed__ …)` facts and 18 more `(arith-rule …)` facts, and the
dump declares the 15 relations, the two tables and the 18 rules; every fact it held before is there, the `sum-of` facts among
them. The ninth file that moved is `examples/broken/load/import_module_not_found.ein`, whose error names the
stdlib's directory, which the sweep itself changed.

**The goldens that move,** named before `EIN_BLESS=1`:
`ein-cli/tests/golden/corpus_exits.txt`, 31 new cells (the example's six, the five programs' five each, `render
rules` exiting 1 on the four that write no rule of their own); `ein-render/tests/golden/corpus_shapes.md5`, the
six new files' lines, and the lines of the nine files that held the theory before, `stdlib/arith.ein`,
`examples/arith/add.ein` and the seven programs: each of their KBs now holds the two tables' rows and the closing
rule's new facts, so its load digest and its KB views move, and its rule views with the module's (the first run
without `EIN_BLESS` reported exactly these three tests: `corpus_cli`, `corpus_shapes` and `alive_invariant`).
`alive_invariant.rs`'s list of names held only inside a fact gains the new programs' `z`.

Pages: `stdlib/README.md` and `07_stdlib_api.md` (§ `std.arith`), `tests/README.md` (§ `stdlib/arith/`, and the
sweep's paragraph), `examples/README.md`, `corpus/corpus.toml`, `AGENTS.md`, [`findings.md`](findings.md)
(E-12), the questions Q-M1f.155 to .158, the phase README and this record.

**The gate** is green (`./run_tests.sh`, `build/gate-s1f.6.4.log`, 2026-11-11). The DOT sweep takes 219 s where
it took 180 s: the KB view of every program that imports the theory now holds three tables, some 4 200 edges,
and goes to the bounded layout. `python3 utils/stdlib_census.py --check`, `python3 utils/nl_census.py --check`
and `python3 utils/stdlib_mutants.py --check` pass on the tree. The register is unmoved: 305 fixed, 16 open and
1 not wrong (`grep '^status' corpus/wrong_answers.toml | sort | uniq -c`, 2026-11-11). Not run for this stage:
the runner's image and the nightly, which S1f.6.6 runs for the three stages.

## Acceptance re-taken, 2026-11-11

- **123078 − 4567 is (118511, z) and 4567 − 123078 is (z, 118511):** `tests/stdlib/arith/08_sub.ein`, each
  claim held by `ein test`.
- **Every comparison agrees with the integers, and `(cmp 5 5)` written with and without high zeros is `eq`:**
  `arith_theory.rs`'s five tests over `OPS`, `high_zeros_are_read_by_value` among them, and
  `tests/stdlib/arith/12_cmp_lengths_and_names.ein`'s 5 against 005 both ways round.
- **The three tests pass:** `arith_theory.rs`, `arith_search_free.rs`, and `arith_batch.rs` under the counters.
