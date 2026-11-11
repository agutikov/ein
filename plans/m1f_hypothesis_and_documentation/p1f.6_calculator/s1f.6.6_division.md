# S1f.6.6 — Division with remainder, and division by zero

**Phase:** [P1f.6](README.md)
**Status:** **done 2026-11-11**: 36 rules and 23 relations added to `stdlib/arith.ein`, 89 rules in all, and the module
reads all seven tables; `(div X Y)` answered by `(quot-of X Y Q R)`, held to Rust's integers, search-free, and
under the rule of the theory on the text and on the counter; division by zero a flag and `(false)` from it, each
by its own rule, every surface shown below; three decisions taken by default, Q-M1f.163 to .165, and no finding
([§ T1 record](#t1-record-2026-11-11) onward). Planned 2026-11-08; re-planned 2026-11-11 to
[the rulings](design.md#the-rulings-2026-11-11).
**Depends on:** [S1f.6.4](s1f.6.4_subtraction.md), [S1f.6.5](s1f.6.5_multiplication.md).

## What it builds

`(div X Y)` answered by `(quot-of X Y Q R)`, school long division:

1. **The divisor in normal form, and its multiples by 2 to 9,** each a row of S1f.6.5, computed once per
   division in a chain whose tags hold the multiples already known. They then travel in the state as one fact.
2. **The dividend turned round once,** so its top digit comes first.
3. **One step per dividend digit.** The digit is brought down under the remainder, which is one constructor.
4. **The quotient digit by bisection.** It lies between 0 and 10; the middle multiple is subtracted by
   S1f.6.4's walk; *it fits* or *it borrows* halves the range, through the `mid` and `adj` tables. At most four
   subtractions a digit. Nothing is guessed: each trial is a subtraction whose outcome the tables decide.
5. **The answer** when no digit is left.

**Division by zero, in two steps** (the user: *"I expected the flag derived from division by zero and then flag
cause (false), the cost is ok, program contains division by zero, why we would want other queries results?"*).
- A divisor whose normal form is `z` derives `(division-by-zero (div X Y))`, by one rule.
- A second rule derives `(false)` from any `(division-by-zero ?request)`.

So the trace shows the flag as a step of its own and the contradiction as the next, the core is the request,
and `refuted by` names the second rule. The costs are accepted as ruled: R is empty, every other query of the
program loses its answer, and only `(false)` can be claimed
([design.md § E](design.md#e-division-by-zero-as-false-question-5)). The stage shows every surface again on the
shipped theory.

## Tasks

1. **The multiples, the turn, the step, the bisection and the answer,** under the rule of the theory.
2. **Division by zero,** the two rules, and its surfaces: `ein solve`, `--trace`, `--dump-answer`,
   `ein saturate --dump`, `ein test` on `(solve :expect (false))`.
3. **The tests,** as for addition, with: X < Y, X = Y, a divisor of one digit and of many, a zero divisor
   written `z` and written with high zeros, and long operands.
4. **The circuit's measure:** firings and depth of one division, and the batch of S1f.6.3's test.
5. **The example:** 127645 ÷ 25, a division by zero, and the chained program of `design.md`: a rule of the
   program's own from `product-of` to a `div` request.

## Acceptance

- 127645 ÷ 25 is 5105 remainder 20.
- A division by zero derives the flag and then `(false)`, each by its own rule, and never a number.
- The three tests pass.

## T1 record, 2026-11-11

Thirty-four rules of division, in the order a division runs through them, and the two of division by zero.

- **The operands,** `div-ask`, `div-ask-second`, `div-start`, and then a call, `(div-call (quotient X Y) U V)`,
  answered by `(div-done TAG Q R)`: division is tagged throughout, as multiplication is
  ([Q-M1f.163](../open_questions.md#q-m1f163--division-called-with-a-tag-and-whose-division-by-zero-is-flagged),
  defaulted), and `div-answer` reads `(div-done (quotient X Y) Q R)`.
- **The multiples,** `div-multiples` and `div-multiple-2` to `div-multiple-9`: nine rows of S1f.6.5's
  `mul-walk`, each tag `div-mJ` holding the dividend and the multiples before it. The first is the divisor
  times 1, which is the divisor in normal form: the rest multiply that, so a divisor written with high zeros
  costs its zeros once, and `div-multiple-2` matches a first multiple that is `(n ?p ?a)`, not zero.
- **The turn,** `div-turn`: the nine multiples go on as one fact, `(multiples M1 … M9)`, and the dividend is
  turned round by S1f.6.5's `top-first`.
- **A step a digit,** `(div-step TAG MULTIPLES DIGITS LEFT QUOTIENT)` (`div-begin`). The digit is brought down
  beside what is left over, `(n LEFT DIGIT)`, by three rules so that no high zero is written: `div-bring`
  where something is left, `div-bring-first` where nothing is and the digit is not 0, and `div-bring-zero`
  where both are zero, which is a digit 0 with no trial.
- **The bisection,** `(div-choose TAG MULTIPLES DIGITS QUOTIENT NUMBER LOW HIGH LEFT)`: the digit is at least
  LOW and less than HIGH, 0 and 10 at first, and LEFT is NUMBER less LOW times the divisor. `div-try-J` reads
  `(mid LOW HIGH J)` and takes the J-th multiple from NUMBER by S1f.6.4's `sub-walk` under the tag `(div-trial …
  J)`, which holds the whole state. `div-fits` reads `(done (div-trial …) T)` and raises LOW to J with T left;
  `div-too-big` reads `(short (div-trial …))` and lowers HIGH to J. `div-digit-found` reads `(adj LOW HIGH)`.
  At most four trials a digit: 5; 2 or 7; then 1, 3, 6 or 8; then 4 or 9.
- **The digit goes after the quotient,** by three rules again: `div-next` for a digit that is not 0,
  `div-next-zero` for a 0 after a quotient that is not zero, `div-next-none` for a 0 before the first digit.
- **The answer,** `div-end`, when no digit is left.

**The two families are written by hand and held by a test**
([Q-M1f.164](../open_questions.md#q-m1f164--divisions-families-of-nine-rules-by-hand-or-generated), defaulted,
on Q-M1f.152's *while a rule is not a family of nine*). A rule cannot pick the J-th of nine multiples by a digit
without a second premise that joins on a number, so there are nine trial rules. `arith_theory.rs`'s
`the_rules_of_a_family_differ_by_their_digit_alone` holds that `div-try-2` to `-9` are `div-try-1` with the
digit and the multiple's variable exchanged, and `div-multiple-3` to `-9` to their one pattern.

**Division by zero,** two rules. `div-by-zero` matches `(done (div-m1 (quotient X Y) U) z)`, the divisor's
normal form being zero for a public request, and derives `(division-by-zero (div X Y))`, the request as
written. `division-by-zero-is-false` matches any flag and derives `(false)`. Nothing else of the division
runs: `div-multiple-2` does not match a first multiple of `z`.

The closing rule closes the 23 relations and `mid` and `adj`, and names the 36 rules; the import reads all seven
tables. The module's header says what a division by zero is and how a program avoids one.

## T2 record, 2026-11-11: division by zero on every surface

The shipped theory, on [`examples/arith/div_by_zero.ein`](../../../examples/arith/div_by_zero.ein): two `let`s,
`(add 2 2)`, `(div takings customers)` with `customers` zero, and `(solve :expect (false))`. Every command
exits 0.

**`ein solve`:**

```text
  states (k_s)      0
  commitments (k_c) 0
  exhausted         yes — every commitment is classified: this is all of R
  empty             every node is dead — root's own fixpoint derives (false)
  lattice           monotone (the static check) — R is convex: no hole
  universe          no candidate: R is {∅} or empty, decided by root alone
  work              1 saturation · 0 facts beyond root · 1 layer

  unsat core        (div takings customers)
  unsat core        (let customers z)
  unsat core        (let takings (n (n (n z 1) 2) 0))
  refuted by        division-by-zero-is-false

  (false)

  (solve :expect)   holds
```

- **The core is the request and what its operands rest on.** With numbers for operands and the divisor written
  `z`, `tests/stdlib/arith/20_div_by_zero.ein`, it is the request alone, `(div (n z 7) z)`, as the design said.
  A named operand adds its `let`. A zero written with digits adds the rows of `md` it was put into normal form
  by: `21_div_by_zero_spelt_and_named.ein`'s core is `(div (n z 7) none)`, `(let none (n (n z 0) 0))` and `(md 0
  1 0 0 0)`. A table row is a premise like any other fact, S1f.6.2's measured cost of plain facts.
- **`refuted by` names the second rule,** `division-by-zero-is-false`.

**`ein solve --trace`:** the flag is a step and the contradiction is the next step of that chain (steps 21 and
23 of the run; the sum's last steps come between, the agenda being first in, first out):

```text
> the divisor of takings divided by customers is zero
Derives `division-by-zero(div(takings, customers))` — (div takings customers) divides by zero

### Step 23 — `division-by-zero-is-false`
> (div takings customers) divides by zero: no number is the answer
Derives `false`.

Refuted by `division-by-zero-is-false`.
```

**`ein solve --dump-answer DIR`:** `answer.json`, `answer.txt`, `index.json` and `nogoods.json`, and no state
file: R has no state.

**`ein saturate --dump`:** root's state, saturated to its fixpoint whatever it holds, *stratified, 1 stratum*.
The flag and the contradiction are in it, each with its rule, and so is the sum:

```text
  (division-by-zero (div takings customers) :rule div-by-zero)
  (false :rule division-by-zero-is-false)
  (sum-of (n z 2) (n z 2) (n z 4) :rule add-answer)
```

**`ein test`:** `(solve :expect (false))` holds, *1 expectation: 1 held*. It is the one claim such a program can
make. The same program with a claim about its sum, `(query :goal (sum-of ?x ?y ?s) :expect (model (sum-of 2 2
4)))`, is FAILED under `ein solve` and `ein test`, exit 1, *the claim lists 1 class, the query has 0*, and the
read-out says *R is empty: no state to answer from* (`build/s1f.6.6/dz/dz.ein`): every other query of the
program loses its answer, the cost the user accepted.

**What holds it.** `arith_search_free.rs`'s `division_by_zero`, for every arithmetic program that claims
`(false)`: no candidate, k_s 0 and k_c 0, exhausted, root dead, one saturation, monotone; the core names
exactly one `(div …)` fact and otherwise `let`s and rows of `md`; `refuted_by` is `division-by-zero-is-false`;
no state file; `ein saturate --dump` holds the flag by `div-by-zero` and `(false)` by the second rule and no
`quot-of`; `ein test` holds the claim
([Q-M1f.165](../open_questions.md#q-m1f165--what-search-free-means-of-a-program-that-divides-by-zero),
defaulted). `arith_theory.rs`'s `a_division_by_zero_is_flagged_and_false`: sixteen requests whose divisor is
`z`, 0, 000 or a name for zero are each flagged, as written, and never answered; `(false)` is derived; eight
requests beside them whose divisor is 3 are answered and not flagged; and a program with no zero divisor has no
flag and no `(false)`. `the_theory_writes_no_negation` holds that `division-by-zero-is-false` is the one rule
that concludes `(false)`, and that it reads the flag alone.

## T3 record, 2026-11-11

- **Six programs:** `16_div` (127645 ÷ 25), `17_div_every_digit` (a quotient 1234567890, all nine trial rules),
  `18_div_small_and_zeros`, `19_div_named_and_chained`, `20_div_by_zero`, `21_div_by_zero_spelt_and_named`.
  `stdlib_coverage.rs`: 166 of 166 over the stdlib (`cat stdlib/*.ein | grep -c '^(rule '`, 2026-11-11).
- **The harness:** a row of `OPS`, `div` and `quot-of` against `a / b` and `a % b`, operands to 30 digits. An
  operation's answer is now an `Option`, `None` for a zero divisor, which the shared cases skip. Every case of
  the other operations is asked: every pair to 120 with a divisor that is not zero, every pair of lengths to 30
  digits twice (X shorter than Y, longer, and of one length), equal operands and operands one apart, 99…9 and
  100…0 against 1 and each other, operands padded with high zeros, and requests by name.
- **The search-free test** reads `quot-of`, and holds a program that claims `(false)` as above.
- **The rule of the theory:** the static test passed unchanged on the 36 rules. `arith_batch.rs`, a row of
  `REQUESTS`, a dividend of nine digits by a divisor of four, so that the quotient has five or six digits:

  | requests | firings | a request | candidates | a request |
  |---:|---:|---:|---:|---:|
  | 250 | 111 463 | 445.85 | 1 508 496 | 6 033.98 |
  | 1 000 | 446 591 | 446.59 | 6 030 246 | 6 030.25 |
  | 4 000 | 1 786 640 | 446.66 | 24 123 775 | 6 030.94 |

  (`cargo test -p ein-infer --features counters --test arith_batch -- --nocapture`, 2026-11-11.)
- **The mutation sweep** kills every mutant of the 36 rules, 18 of 18: 234 of 284 where 216 of 266, the bank's
  50 survivors unmoved (`python3 utils/stdlib_mutants.py --check`, 2026-11-11). `std.arith`'s 89 rules have 67
  mutants and no survivor.

## T4 record, 2026-11-11

`arith_theory.rs`'s `the_circuit_of_one_division` pins the measure (`cargo test -p ein-infer --test arith_theory
the_circuit`). A division is one chain, so its depth is its size:

| request | firings and depth |
|---|---:|
| 127645 ÷ 25 | 220 |
| 7 ÷ 3 | 84 |
| 999999 ÷ 7 (six digits by one) | 229 |
| 999999999999 ÷ 7 (twelve by one) | 394 |
| 999999999999 ÷ 777777 (twelve by six) | 655 |
| 127645 ÷ 0 | 10 |

The multiples are a fixed cost, nine rows of the divisor's length, some 70 firings for a divisor of one digit.
Each digit of the dividend is then three or four trials, a trial a subtraction the length of the divisor with
its strip and turn where it fits. So the size grows as the product of the two lengths, and a division by zero
stops after ten firings: the operands, one row, its strip, the flag and the contradiction.

On the release binary (`python3 build/s1f.6.4/batch.py div`, 2026-11-11), random divisions of two numbers of up
to nine digits in one program: 250, 1 000 and 4 000 saturate in 143, 609 and 2 644 ms, 71 914, 285 605 and
1 126 816 firings, the processes 208, 810 and 3 492 ms. The prototype's K took 2.2 s for 4 000.

**The suite's times.** `ein test tests/stdlib/arith/01_add.ein` takes 25.7 ms, where it took 18.1 ms before
S1f.6.4: every program that imports the theory holds the seven tables, 1 664 rows, and compiles 89 rules. `ein
test tests/` takes 681 ms for twenty-one arithmetic programs, where it took 349 ms for seven, and
`stdlib_coverage.rs` 1.4 s where half a second (the same script, and `cargo test -p ein-infer --test
stdlib_coverage`, 2026-11-11).

## T5 record, 2026-11-11

- `examples/arith/div.ein`: 127645 ÷ 25, and the chain of `design.md`, a rule of the program's own from the
  product of 123 and 456 to a `div` request, 56088 ÷ 25 = 2243 remainder 13.
- `examples/arith/div_by_zero.ein`: T2's program.

**The sweep,** `python3 build/s1f.6.4/sweep.py` (2026-11-11), one engine and two stdlibs, twice. Against the
stdlib of `277cfc44` over its 681 files: the eight programs that imported the theory then, `ein saturate
--dump` only. Against the stdlib of `b2c4b238`, S1f.6.5's commit, over its 691 files (`--old
build/s1f.6.6/old/stdlib --ref b2c4b238`): the eighteen programs that import the theory, `ein saturate --dump`
only, each by `mid`'s and `adj`'s 55 rows, 25 `(__closed__ …)` and 36 `(arith-rule …)` facts and the new
declarations, and nothing gone. **`ein solve` moved on no program** in either. The other file each time is
`import_module_not_found.ein`, whose error names the stdlib's directory.

**The goldens that move,** named before `EIN_BLESS=1`: `corpus_exits.txt`, 41 new cells (`div.ein`'s six,
`div_by_zero.ein`'s five, the six programs' five each, `render rules` exiting 0 on `19`, which writes a rule,
and 1 on the five others); `corpus_shapes.md5`, the eight new files' lines and the lines of the nineteen files
that held the theory before, whose KBs now hold `mid` and `adj` and the closing rule's new facts;
`alive_invariant.rs`'s list gains five programs' `z`.

Pages: `stdlib/README.md`, `07_stdlib_api.md`, `tests/README.md`, `examples/README.md`, `corpus/corpus.toml`,
`AGENTS.md`, the questions Q-M1f.163 to .165, the phase README and this record.

**The gate** is green (`./run_tests.sh`, `build/gate-s1f.6.6.log`, 2026-11-11), the DOT sweep 32 s in it.
`python3 utils/stdlib_census.py --check`, `python3 utils/nl_census.py --check` and `python3
utils/stdlib_mutants.py --check` pass on the tree. The register is unmoved: 305 fixed, 16 open and 1 not wrong
(`grep '^status' corpus/wrong_answers.toml | sort | uniq -c`, 2026-11-11). The runner's image and the nightly
are run after this commit, for the three stages, and recorded below it.

## Acceptance re-taken, 2026-11-11

- **127645 ÷ 25 is 5105 remainder 20:** `tests/stdlib/arith/16_div.ein`, held by `ein test`.
- **A division by zero derives the flag and then `(false)`, each by its own rule, and never a number:** T2, and
  the two tests named there.
- **The three tests pass:** `arith_theory.rs`, `arith_search_free.rs`, and `arith_batch.rs` under the counters.
