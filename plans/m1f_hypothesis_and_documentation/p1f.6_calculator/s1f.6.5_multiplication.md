# S1f.6.5 — Multiplication

**Phase:** [P1f.6](README.md)
**Status:** **done 2026-11-11**: 16 rules and 13 relations added to `stdlib/arith.ein`, 53 rules in all; `(mul X Y)`
answered by `(product-of X Y P)`, held to Rust's integers, search-free, and under the rule of the theory on the
text and on the counter; four decisions taken by default, Q-M1f.159 to .162, and no finding
([§ T1 record](#t1-record-2026-11-11) onward). Planned 2026-11-08; re-planned 2026-11-11 to
[the rulings](design.md#the-rulings-2026-11-11).
**Depends on:** [S1f.6.3](s1f.6.3_addition.md).

## What it builds

`(mul X Y)` answered by `(product-of X Y P)`, the school method from the multiplier's top digit:

1. **The multiplier is turned round once,** so its most significant digit comes first.
2. **A row per multiplier digit:** X times one digit, a walk over the `md` table.
3. **The sum so far times ten, plus the row.** Times ten is one constructor in the public shape, `(n ACC 0)`,
   and the addition is S1f.6.3's walk, called with a tag that holds the multiplier's remaining digits.
4. **The product** when no digit is left. A product has no width to overflow.

Every call carries its tag, so the rule of the theory holds: the rule that goes on after a row reads its whole
state out of `(done (TAG …) ROW)`.

## Tasks

1. **The row, the step and the answer.**
2. **The tests,** as for addition: one program per rule, the harness test (zero on either side, a multiplier
   with inner zeros, operands with high zeros, long operands), the search-free test and the test of the rule
   of the theory.
3. **The circuit's measure:** firings and depth of one multiplication, and how they grow with the two lengths.
4. **The example:** 123 × 456, and 123078 × 123078, which round 1's `num9` could not hold.

## Acceptance

- 123 × 456 is 56088 and 123078 × 123078 is 15148194084.
- The three tests pass.

## T1 record, 2026-11-11

Sixteen rules, in the order a product runs through them.

- **The operands,** `mul-ask`, `mul-ask-second`, `mul-start`, as every request looks them up, and then **a
  call**: `(mul-call (product X Y) U V)`. Multiplication is tagged throughout
  ([Q-M1f.159](../open_questions.md#q-m1f159--multiplication-called-with-a-tag), defaulted), where the
  prototype's carried the request's operands: `mul-end` hands over `(done TAG P)` and `mul-answer` reads
  `(done (product X Y) P)`. One firing more a request, and any later theory can multiply.
- **The multiplier turned round,** `mul-turn`, then `top-first-step` a digit and `top-first-end`: `(top-first TAG
  NUMBER DIGITS)` is the other turn, a number into digits with the top one outermost, handed over as
  `(top-first-is TAG DIGITS)`. S1f.6.6 turns its dividend with the same three relations.
- **A row,** `mul-row`: `(mul-walk (mul-row-of TAG U REST SUM) U DIGIT 0 z)`, a walk from the units over `md`
  (`mul-column`). It ends with no carry and goes to the shared `strip` (`mul-walk-end`), or with a carry, which
  is the top digit and not zero, and goes straight to the turn (`mul-carry-out`). S1f.6.6 computes the divisor's
  multiples with the same walk.
- **The step,** read off the row's tag: the first row is the sum so far (`mul-first-row`); a later one is added
  to ten times the sum, `(add-walk (mul-sum TAG U REST) (n SUM 0) ROW 0 z)` (`mul-add-row`), and the answer
  comes back as the sum so far (`mul-next`). `mul-first-row` matches a sum that is `z` and `mul-add-row` one
  that is `(n ?p ?a)`, so the two never both fire and no high zero is written.
- **A zero digit of the multiplier is a row like any other**
  ([Q-M1f.160](../open_questions.md#q-m1f160--a-zero-digit-of-the-multiplier), defaulted): a row of zeros,
  stripped to `z`, and added. No rule tells it apart.
- **The product,** `mul-end`, when no digit is left.

The closing rule closes the 13 relations and `md` and names the 16 rules; the import reads `(fa fs md cl nz)`.

## T2 record, 2026-11-11

- **Three programs:** `13_mul` (123 × 456 = 56088, and the operands exchanged, each claimed under its own),
  `14_mul_zeros_and_carries`, `15_mul_long_and_named` (123078 × 123078 = 15148194084, products by name, and
  `done`'s extent for the public products). `stdlib_coverage.rs`: 130 of 130 over the stdlib (`cat stdlib/*.ein |
  grep -c '^(rule '`, 2026-11-11).
- **The harness:** a row of `OPS` in `arith_theory.rs`, `mul` and `product-of` against `a * b`. Its operands are
  sampled to 19 digits where the others' are to 30
  ([Q-M1f.162](../open_questions.md#q-m1f162--the-longest-sampled-operand-of-a-product), defaulted): a product
  of two such numbers has at most 38 digits, which `u128` holds. Every case of the other operations is asked:
  every pair to 120, every pair of lengths to 19 digits twice, equal operands and operands one apart, 99…9 and
  100…0 at every length, operands padded with high zeros, and requests by name. Each request has one answer,
  in normal form.
- **The search-free test** reads `product-of`: `RESULTS` in `arith_search_free.rs`.
- **The rule of the theory:** the static test passed unchanged. `arith_batch.rs`, a row of `REQUESTS`:

  | requests | firings | a request | candidates | a request |
  |---:|---:|---:|---:|---:|
  | 250 | 121 711 | 486.84 | 2 400 967 | 9 603.87 |
  | 1 000 | 486 597 | 486.60 | 9 601 183 | 9 601.18 |
  | 4 000 | 1 945 430 | 486.36 | 38 394 803 | 9 598.70 |

  (`cargo test -p ein-infer --features counters --test arith_batch -- --nocapture`, 2026-11-11.) A product of
  two nine-digit numbers is nine rows of nine columns, and a column of `md` offers the 90 rows its first digit
  keys.
- **The mutation sweep** kills every mutant of the 16 rules, 13 of 13: 216 of 266 where 203 of 253, the bank's 50
  survivors unmoved (`python3 utils/stdlib_mutants.py --check`, 2026-11-11).

## T3 record, 2026-11-11

`arith_theory.rs`'s `the_circuit_of_one_multiplication` pins the measure (`cargo test -p ein-infer --test
arith_theory the_circuit`). A multiplication is one chain, so its depth is its size:

| request | firings and depth |
|---|---:|
| 123 × 456 | 72 |
| 123078 × 123078 | 227 |
| 999999 × 9 (six digits by one) | 28 |
| 9 × 999999 (one by six) | 128 |
| 999999 × 999999 (six by six) | 238 |
| 999999999999 × 999999 (twelve by six) | 370 |
| 999999 × 999999999999 (six by twelve) | 556 |

**How it grows.** With a multiplicand of m digits and a multiplier of k: the turn is k firings; each of the k
rows is a walk of m columns with its end and turn, about 2m; and each row after the first is an addition whose
length is the sum so far, which grows from m to m + k, about 2(m + k) with its turn. So the size is about
4mk + k², the product of the lengths: doubling the multiplicand takes 238 to 370, and doubling the multiplier
takes it to 556. The multiplier's digits cost more than the multiplicand's, so the shorter number is the better
multiplier; the theory does not exchange them.

On the release binary (`python3 build/s1f.6.4/batch.py mul`, 2026-11-11), random products of two numbers of up
to nine digits in one program: 250, 1 000 and 4 000 saturate in 58, 267 and 1 135 ms, 35 270, 145 707 and
567 620 firings, the processes 102, 392 and 1 606 ms.

**What a program pays to load** grew with `md`'s 900 rows, which every program that imports the theory now
holds: `ein test tests/stdlib/arith/01_add.ein` takes 24.5 ms where 20.7 ms, and `ein test tests/` 485 ms for
fifteen arithmetic programs where 377 ms for twelve (the same script).

## T4 record, 2026-11-11

`examples/arith/mul.ein`: 123 × 456, 123078 × 123078, a product of two named constants, and a rule of the
program's own that derives an `add` request from that product. Corpus group `positive`.

**The DOT sweep runs several files at a time**
([Q-M1f.161](../open_questions.md#q-m1f161--the-dot-sweep-several-files-at-a-time), defaulted, with Q-M1f.147
and Q-M1f.153 for the user to confirm). With `md` the KB view of every program that imports the theory is some
9 000 edges, and the bounded layout takes 2.2 s on one where `sfdp` dumps core, `neato` takes 4.6 s and `circo`
does not finish in a minute (2026-11-11); each program has six. In one thread the sweep's `dot` processes take
441 s of processor time for this stage's eighteen programs, where the whole sweep took 219 s at S1f.6.4.
`dot_wellformed.rs` now shares the corpus files out among the machine's cores, each file's views in order and
the report gathered in the files' order: 28 s here on sixteen cores. What it holds is unchanged, every view of
every file put to Graphviz and a complaint a failure.

**The sweep,** `python3 build/s1f.6.4/sweep.py` (2026-11-11), one engine and two stdlibs, twice. Against the
stdlib of `277cfc44`, over the 681 files that commit tracks: the eight programs that imported the theory then,
`ein saturate --dump` only. Against the stdlib of `088a1f87`, S1f.6.4's commit, over its 687 files (`--old
build/s1f.6.5/old/stdlib --ref 088a1f87`): the fourteen programs that import the theory, `ein saturate --dump`
only, each by `md`'s 900 rows, 14 `(__closed__ …)` and 16 `(arith-rule …)` facts and the new declarations, and
nothing gone. **`ein solve` moved on no program** in either. The fifteenth file each time is
`import_module_not_found.ein`, whose error names the stdlib's directory.

**The goldens that move,** named before `EIN_BLESS=1`: `corpus_exits.txt`, 21 new cells (the example's six, the
three programs' five each, `render rules` exiting 1 on the three); `corpus_shapes.md5`, the four new files'
lines and the lines of the fifteen files that held the theory before, whose KBs now hold `md`;
`alive_invariant.rs`'s list gains three programs' `z`.

Pages: `stdlib/README.md`, `07_stdlib_api.md`, `tests/README.md`, `examples/README.md`, `corpus/corpus.toml`,
`AGENTS.md`, the questions Q-M1f.159 to .162, the phase README and this record.

**The gate** is green (`./run_tests.sh`, `build/gate-s1f.6.5.log`, 2026-11-11), the DOT sweep 30 s in it.
`python3 utils/stdlib_census.py --check`, `python3 utils/nl_census.py --check` and `python3
utils/stdlib_mutants.py --check` pass on the tree. The register is unmoved: 305 fixed, 16 open and 1 not wrong
(`grep '^status' corpus/wrong_answers.toml | sort | uniq -c`, 2026-11-11). Not run for this stage: the runner's
image and the nightly, which S1f.6.6 runs for the three stages.

## Acceptance re-taken, 2026-11-11

- **123 × 456 is 56088 and 123078 × 123078 is 15148194084:** `tests/stdlib/arith/13_mul.ein` and
  `15_mul_long_and_named.ein`, each claim held by `ein test`.
- **The three tests pass:** `arith_theory.rs`, `arith_search_free.rs`, and `arith_batch.rs` under the counters.
