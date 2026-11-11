# S1f.6.13 — Fixed-width numbers as a second module, `std.num9`

**Phase:** [P1f.6](README.md)
**Status:** **done 2026-11-11** (`f6243365`): `stdlib/num9.ein`, generated, 47 rules, the five operations by wide rules, overflow as a
flag, division by zero through `std.arith`'s rule, and conversions each way; held to Rust's integers, exhaustively at
width 2; no rule joins two facts on a number, so a batch is linear where the prototype's was quadratic; seven decisions
taken by default, Q-M1f.167 to .173, **the last the user's to confirm** (the register tripwire, § T7), and one finding,
E-28 ([§ T1 record](#t1-record-2026-11-11-the-generator-and-the-module) onward).
Planned 2026-11-11, on the user's ruling of that day.
**Depends on:** [S1f.6.2](s1f.6.2_digits_and_tables.md) for the tables; [S1f.6.6](s1f.6.6_division.md) for the
conversions to and from `std.arith`'s numbers.

## The ruling

The user, 2026-11-11: *"yes, ship num9 in stdlib, why not"*.

## What it builds

`stdlib/num9.ein`, imported as `std.num9`: the user's own number, `(num9 0 0 0 1 2 3 0 7 8)`, most significant
digit first, nine digits. It is round 1's theory
([design.md § How the prototype is built](design.md#how-the-prototype-is-built)): one rule with a premise per
column for a sum, a difference and a comparison, shift registers over step names for a product and a quotient.
Where it wins is a single request on numbers that fit: one firing for a sum.

**Decisions taken here, each the user's to overrule:**

- **One width ships: nine.** The module is written by a generator that takes the width, as the prototype's
  `gen.py` does, and `--check` in the gate holds the shipped file to it. Another width is the generator run
  with another number, not a shipped module; a second shipped width would double the rules the coverage gate
  asks tests for.
- **Its relations are its own:** `add9`, `sum9-of` and so on, so that a program may import both modules flat.
  The names are the stage's to propose with examples before the module is written.
- **Overflow is a fact,** `(overflow <request>)`, and no number (the design's recommendation; the user ruled
  division by zero and not this). **Division by zero** is as in `std.arith`: the flag, then `(false)` from it.
- **Conversions are requests,** one firing each way, as round 3's mix built them: `(to9 X)` answered by
  `(as9 X W)` or by `(big X)` for a number of ten digits or more, and `(from9 W)` answered in normal form. They
  live in `std.num9`, which imports `std.arith` for the number's shape. No routing between the two theories is
  shipped: round 3's was the slowest design measured.

**Its known cost.** The division's step rule joins its state with the divisor's multiples on the divisor, a
nested fact, so a batch is quadratic: 250 divisions in 193 ms and 4 000 in 8.1 s on the prototype
([design.md § 4](design.md#4-the-table)). Task 3 decides whether the multiples travel in the state, as
`std.arith`'s do.

## Tasks

1. **The generator and the module,** with `--check` in the gate, importing `std.digits` and adding its own step
   counter.
2. **The names,** shown on the design's target programs before the module is written.
3. **The division's batch cost,** measured on the shipped module; the multiples moved into the state if that
   removes the join, and the measure re-taken.
4. **The conversions,** and a program that computes one sum both ways and compares the answers with `cmp`.
5. **The tests:** one program per rule under `tests/stdlib/num9/`; the harness test, exhaustive at width 2 on
   the generator's output and sampled at width 9; overflow on every operation that can overflow, never a
   truncated number; the search-free test.
6. **The page:** when to use which module, with round 3's table as the evidence.

## Acceptance

- The eight target programs of `design.md`'s round 1 answer as that page showed, under the shipped names.
- Overflow and division by zero as stated; the stdlib's gates hold for the module.
- H empty, k_s 1, R {∅} on every program.

## T1 record, 2026-11-11: the generator and the module

[`utils/gen_num9.py`](../../../utils/gen_num9.py) writes [`stdlib/num9.ein`](../../../stdlib/num9.ein): 47 rules and 22
relations in 819 lines (`grep -c '^(rule' stdlib/num9.ein`, `grep -c '^(relation'`, `wc -l`, 2026-11-11), one
stratum, no `absent`, `not`, `or` or guard. `--check` is in the gate as `num9_theory.rs`'s
`the_file_is_what_the_generator_writes`; `--width W --stdout`, 2 to 9, prints the same theory over `(numW …)`, and
no other width is written to the stdlib.

| what | relations | rules |
|---|---|---|
| the number | `num9` | — |
| the protocol | `add9`, `sum9-of`, `sub9`, `diff9-of`, `cmp9`, `cmp9-of`, `mul9`, `product9-of`, `div9`, `quot9-of`, `overflow`, `to9`, `as9`, `big`, `from9`, `of9` | — |
| addition | — | `add9-fits`, `add9-overflows` |
| subtraction | — | `sub9-fits`, `sub9-short` |
| comparison | — | `cmp9-columns` |
| multiplication | `mul9-at` | `mul9-start`, `mul9-step`, `mul9-step-zero`, `mul9-done`, `mul9-overflows-shift`, `-row`, `-add` |
| division | `div9-at`, `multiples9`, `row9` | `div9-start`, `div9-step-0` to `div9-step-9`, `div9-done`, `div9-by-zero` |
| conversions | — | `to9-zero`, `to9-1` to `to9-9`, `to9-big`, `from9-zero`, `from9-1` to `from9-9` |
| the entry | `num9-rule` | `arith9`, the closing rule |

It is round 1's theory in what it computes and not in how it is wired; § T3 says where and why. Four smaller things
differ from the prototype's `gen.py`:

- **The tables are `std.digits`'**, imported, where the prototype stated its own by rules with an empty `:match`.
- **The step counter is `std.digits`' `adj`**, the pairs k and k + 1, where the plan said the module adds a counter
  of its own ([Q-M1f.171](../open_questions.md#q-m1f171--stdnum9s-step-counter-and-its-comparison)): no table is added,
  and the width stops at nine either way.
- **Comparison is one rule over the comparator's table**, `cl`, where round 2's `num9` used two subtractors and a
  third rule for equal numbers (Q-M1f.171).
- **`arith9` closes and names as `std.arith`'s `arith` does**, and the module writes `(num9-rule arith9)`, the one
  fact E-11 asks of a library whose entry is a rule with an empty `:match`. `answer_round_trip.rs` passes on every
  program of the module.

**From `std.arith` the module takes three names:** `n`, the number's relation, for the conversions, and
`division-by-zero` with `division-by-zero-is-false`. A division by zero is then one flag relation and one rule
that concludes `(false)` across both modules
([Q-M1f.168](../open_questions.md#q-m1f168--overflow-and-division-by-zero-in-stdnum9)), and the module concludes no
`(false)` of its own. The selection brings those three and nothing else of `std.arith`: a program that imports
`std.num9` alone loads 47 rules of its own and that one.

## T2 record, 2026-11-11: the names

([Q-M1f.167](../open_questions.md#q-m1f167--the-names-of-stdnum9), defaulted.) Each of `std.arith`'s public names
with a `9`: `add9` and `sum9-of`, `sub9` and `diff9-of`, `cmp9` and `cmp9-of`, `mul9` and `product9-of`, `div9` and
`quot9-of`; the entry `arith9`; the conversions `to9`, `as9`, `from9`, `of9`, as round 3's mix named them; and two
flags with no digit, `overflow` and `big`, which `std.arith` has no use for. So the two modules import flat into
one program, which `tests/stdlib/num9/12_both_modules.ein` does.

**The eight target programs of `design.md`'s round 1, under the shipped names**, each answer `ein solve` prints
(2026-11-11):

| target | the request | the row `ein solve` prints | where |
|---|---|---|---|
| add | `(add9 (num9 0 0 0 1 2 3 0 7 8) (num9 0 0 0 0 0 4 5 6 7))` | `?s = (num9 0 0 0 1 2 7 6 4 5)` | `tests/stdlib/num9/01_add.ein` |
| sub | `(sub9 (num9 0 0 0 1 2 3 0 7 8) (num9 0 0 0 0 0 4 5 6 7))` | `?d = (num9 0 0 0 1 1 8 5 1 1) · ?r = (num9 0 0 0 0 0 0 0 0 0)` | `03_sub.ein` |
| sub-rev | `(sub9 (num9 0 0 0 0 0 4 5 6 7) (num9 0 0 0 1 2 3 0 7 8))` | `?d = (num9 0 0 0 0 0 0 0 0 0) · ?r = (num9 0 0 0 1 1 8 5 1 1)` | `examples/arith/num9.ein` |
| mul | `(mul9 (num9 0 0 0 0 0 0 1 2 3) (num9 0 0 0 0 0 0 4 5 6))` | `?p = (num9 0 0 0 0 5 6 0 8 8)` | `05_mul.ein` |
| div | `(div9 (num9 0 0 0 1 2 7 6 4 5) (num9 0 0 0 0 0 0 0 2 5))` | `?q = (num9 0 0 0 0 0 5 1 0 5) · ?r = (num9 0 0 0 0 0 0 0 2 0)` | `07_div.ein` |
| overflow | 999999999 + 1, and 123078 × 123078 | `?request = (add9 …)` and `?request = (mul9 …)` under `(overflow ?request)`; no `sum9-of` and no `product9-of` for either | `02_add_overflow.ein`, `06_mul_overflow.ein`, `examples/arith/num9_overflow.ein` |
| div-zero | `(div9 (num9 0 0 0 1 2 7 6 4 5) (num9 0 0 0 0 0 0 0 0 0))` | `states (k_s) 0`, `unsat core (div9 …)`, `refuted by division-by-zero-is-false` | `09_div_by_zero.ein` |
| chain | the product of 123 and 456, divided by 25 through a rule of the program | `?q = (num9 0 0 0 0 0 2 2 4 3) · ?r = (num9 0 0 0 0 0 0 0 1 3)` | `13_chain.ein` |

Seven answer as the page showed. **The eighth, div-zero, differs by the ruling that came after the page:** the page's
prototype stored a flag and left root alive; the user ruled the flag and then `(false)` from it, and this stage's
plan says *as in `std.arith`*.

## T3 record, 2026-11-11: the batch, and why the wiring changed

([Q-M1f.169](../open_questions.md#q-m1f169--where-stdnum9s-multiples-and-rows-live), defaulted.) The prototype's
division asked for the divisor's ten multiples as facts of their own, `(muld-of D q P c)`, and its step rule
joined the state with them on the divisor, a nested fact the index does not key: 250 divisions in 193 ms and 4 000
in 8.1 s. Its product did the same with the rows of the multiplier's digits. The shipped module has no such fact:

- **`div9-start` computes the multiples by 2 to 9 in one firing,** 72 rows of `md` in one rule, and puts the nine
  into the state as `(multiples9 (row9 …) … (row9 …))`, each row ten digits, the carry out of the top first.
- **Ten step rules, `div9-step-0` to `-9`,** one a digit of the quotient, each reading its own multiple from the
  state: it taken from the number with no borrow, and the divisor taken from what is left with one. Exactly one
  fires. They are one rule written ten times by the generator.
- **`mul9-step` computes its row inside itself,** nine rows of `md` and nine of `fa` in one rule, and
  `mul9-step-zero` shifts where the digit is 0 and reads no table.

So **no rule of the module joins two facts on a number**, `std.arith`'s rule, held on the text by `num9_theory.rs`
and on the matcher's count by `arith_batch.rs`'s `a_batch_of_nine_digit_requests_costs_in_proportion_to_its_count`:

| request | firings | candidates a request at 250 | at 1 000 | at 4 000 |
|---|---:|---:|---:|---:|
| `add9`, two of eight digits | 1 | 362.00 | 362.00 | 362.00 |
| `sub9`, two of nine | 1 | 362.00 | 362.00 | 362.00 |
| `cmp9`, two of nine | 1 | 901.00 | 901.00 | 901.00 |
| `mul9`, two of four | 11 | 10 408.96 | 10 386.62 | 10 357.29 |
| `div9`, nine by four | 11 | 26 888.66 | 26 968.12 | 27 014.05 |

(`cargo test -p ein-infer --features counters --test arith_batch -- --nocapture`, 2026-11-11.) A sum's 362 are two rules, each nine lookups among the twenty rows a first digit
keys; a comparison's 901 are the hundred rows an order keys (E-12).

On the release binary, beside `std.arith` on the same requests (`python3 build/s1f.6.13/batch.py`, 2026-11-11, one
P-core, best of five; the prototype's batches, `design.md` § 4):

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

4 000 divisions take 1.24 s where the prototype took 8.1, and 4 000 sums 24 ms where it took 255. A request stated
twice is one fact, which is why a batch of short products reads under eleven firings a request.

## T4 record, 2026-11-11: the conversions

([Q-M1f.170](../open_questions.md#q-m1f170--what-stdnum9-reads-as-an-operand), defaulted.) `(to9 X)` is answered by
`(as9 X W)` by a rule for each count of written digits, `to9-zero` and `to9-1` to `to9-9`, whose pattern is the
whole nested number, and by `(big X)` for more than nine (`to9-big`). `(from9 W)` is answered by `(of9 W X)` by a
rule for each count of digits the number has, the first digit that is not zero read off `nz`, and `from9-zero`. One
firing each. A number is converted as written: high zeros count, so nine digits behind a zero are `big`, and a
name a `let` states is not looked up.

**One sum, both ways** (`tests/stdlib/num9/12_both_modules.ein`, `examples/arith/num9_both_ways.ein`): `(add X Y)`
by `std.arith`; `(to9 X)` and `(to9 Y)`; a rule of the program asks `(add9 …)` of the two `as9` answers; another
asks `(from9 …)` of the sum; a third asks `std.arith` to compare the two answers, and `(cmp-of … eq)` is the
program's claim. No rule of either module routes a request to the other.

## T5 record, 2026-11-11: the tests

- **Thirteen programs** under `tests/stdlib/num9/`, each a corpus entry of group `stdlib` with its `:expect` and,
  but for the division by zero, `(solve :expect (region (model)))`. `stdlib_coverage.rs` holds that the 47 rules
  are each activated by one of them: 213 of 213 over the stdlib (`cat stdlib/*.ein | grep -c '^(rule '`,
  2026-11-11).
- **The harness against the integers,** `ein-infer/tests/num9_theory.rs`
  ([Q-M1f.172](../open_questions.md#q-m1f172--the-harness-of-stdnum9-exhaustive-at-width-2), defaulted). The oracle
  is `u64`.
  - **Exhaustive at width 2:** the generator's `--width 2 --stdout`, its text put in front of the requests, every
    pair of operands from 00 to 99 under all five operations, 49 900 requests, a divisor never zero. Each request
    has **exactly one** result fact, what the integers say, **or exactly one** `(overflow REQUEST)` where the
    answer has three digits, and never both: no truncated number exists. It passed on the first run.
  - **Sampled at width 9:** four draws for every pair of lengths from 0 to 9 digits, equal operands and operands
    one apart at every length, and twenty corners, the last sum and product that fit and the first that do not.
  - The conversions at every count of digits to twelve, with high zeros; a division by zero, flagged as written
    and `(false)`; and the firings of each operation, pinned: 1 for a sum, a difference and a comparison (2 for a
    difference of equal numbers), 11 for a product and a quotient, 1 for a conversion.
  - On the text: no rule joins two facts on a number; no `absent`, `not`, `or`, `neq`, `eq` or `false`; and the
    closing rule closes every relation and names every rule, with the module's one written fact.
- **The search-free test,** `arith_search_free.rs`: `tests/stdlib/num9` joins its roots, and the module's five
  results, `overflow`, `as9`, `big` and `of9` its result relations. A program that claims `(solve :expect
  (false))` is held as `std.arith`'s are.
- **The mutation sweep** kills every mutant of the module, 41 of 41: **275 of 325** where 234 of 284, the bank's 50
  survivors unmoved (`python3 utils/stdlib_mutants.py --check`, 2026-11-11). Its first run read 273 of 327 with
  four survivors, and each was acted on:
  - `(adj ?j ?k)` exchanged in `mul9-overflows-row` and `-add`: equivalent mutants. Both rules also ask that the
    multiplier's next digit is not zero, which already says a digit is still to come, so the counter's premise
    changed nothing. **It is gone from both rules**, which is 41 mutants where 43.
  - `(sub9 X Y)` read as `(sub9 Y X)` in `sub9-fits` and in `sub9-short`: `03_sub.ein` asked 123078 − 4567 and
    4567 − 123078, and each got the other's fact. It now asks no pair both ways round, and says why.

## T6 record, 2026-11-11: the page, the examples, and what moved

**The page** is [`stdlib/README.md` § `std.num9`](../../../stdlib/README.md#stdnum9-nine-digits-in-one-fact-and-wide-rules-m1f-s1f613),
with § When to use which module: the batch table above as the evidence, where the plan named round 3's. What it
says that the plan did not expect: **`std.num9` wins at volume and loses on one request.** It is five to seven
times `std.arith` on a batch of sums or differences and 1.4 times on quotients, level on products; but a program
that imports it pays 29.3 ms of load where `std.arith`'s pays 19.6, about ten for the module's own 46 KB and five
for the text of `std.arith`, read whole for three names (Q-M1f.148), so one sum is 36 ms as a process against 26.
The plan's *where it wins is a single request* is true of firings, 1 against 21, and not of the clock.

**Three examples** under `examples/arith/`: `num9.ein`, the five operations and the chain; `num9_overflow.ein`;
`num9_both_ways.ein`. Corpus group `positive`.

**The sweep.** One engine and two stdlibs, the release binary and stdlib of `eb749c0f` against the tree's, `ein
saturate --dump` and `ein solve` over the 702 `.ein` files `eb749c0f` tracks: no answer moved. One file's output differs, by a path:
`examples/broken/load/import_module_not_found.ein`'s error line prints the stdlib's directory, which the sweep
sets (`python3 build/s1f.6.13/sweep.py`, 2026-11-11). No engine source moved; `stdlib/arith.ein` and `stdlib/digits.ein` are byte for byte what they were.

**The goldens that move,** named before `EIN_BLESS=1`: `ein-cli/tests/golden/corpus_exits.txt`, 83 new cells (the
module's four, the thirteen programs' five each, `render rules` exiting 1 on the eleven that write no rule of
their own, and the examples' fourteen); `ein-render/tests/golden/corpus_shapes.md5`, 714 new lines, the seventeen
new files' 42 renderings each, and no other line. `alive_invariant.rs`'s list gains sixteen programs' `z`: a
program of nine-digit numbers writes none, and holds it through the module's rule `to9-zero`.

**The register** is unmoved: 305 fixed, 16 open and 1 not wrong (`grep '^status' corpus/wrong_answers.toml | sort |
uniq -c`, 2026-11-11). One finding for [`findings.md`](findings.md), E-28, § T7. The wide rules, 73 premises in
`div9-start`, compile, match and draw (`dot_wellformed.rs`, 74 s with the seventeen files, 2026-11-11).

Pages: `stdlib/README.md` (§ `std.num9`, new), `tests/README.md` (§ `stdlib/num9/`, new), `07_stdlib_api.md`
(§ `std.num9`, new), `examples/README.md`, `utils/README.md` (thirty-six scripts), `AGENTS.md`, the questions
Q-M1f.167 to .173, [`findings.md`](findings.md) (E-28), the phase README and this record.

## T7 record, 2026-11-11: the register tripwire, a decision for the user

The stage's first gate failed in one place, `ein-infer/tests/compile_limits.rs`'s
`the_corpus_is_nowhere_near_the_ceiling`: *the widest corpus rule (div9-start, 162 registers) is within 8× of
MAX_REGS (256) — the bound needs re-deciding*. A rule binds its variables to a file of 256 registers, the engine's
one compile-time bound, and that test is its tripwire: the corpus's widest rule was 16, and the test holds it
under 32. A wide rule cannot be: `add9-fits` binds 35 variables, three numbers and eight carries, a product's
step about 60, a division's about 90, and `div9-start` 162.

**No engine source was changed, and none had to be:** 162 is under 256, and the module loads, compiles and answers.
What the tripwire asks for is a decision, taken by default and **the user's to confirm**
([Q-M1f.173](../open_questions.md#q-m1f173--max_regs-and-the-wide-rules-of-stdnum9); E-28 of
[`findings.md`](findings.md)): the bound stays, and the test measures `std.num9` apart. It reads the module's rule
names, holds every other rule of the corpus to an eighth of the bound as before (the widest is still `div-try-1`,
16), and holds the module's widest to its number, `div9-start` at 162, and under the bound. The module is
generated and its width fixed, so that number moves only with the generator, and then the test fails.

The alternatives are the question's: a chain of eight rules for the multiples takes the widest to about 92, still
three times the tripwire, at eight firings more a division; a larger register file is an engine change; rules
split in halves are no longer the theory that was ruled in.

## After the commit, 2026-11-11: the nightly and the runner's image, for S1f.6.15 and S1f.6.13

Both on `f6243365`, the tree clean.

- **The nightly.** `./run_tests.sh --nightly`, green in every step (`build/nightly-s1f.6.13.log`): the slow corpus
  cells, the round trip's slow and `heavy_dump` entries, the levers', the cores', `region`'s and
  `obligation_rung`'s slow halves, the scratch check, the mutation census at 275 of 325, the fuzzer at both seeds
  and the bench smoke.
- **The runner's image.** `utils/ci_container.sh`, the whole gate in a container built like the runner: Ubuntu
  26.04.1 with Graphviz 14.1.2 in its x86-64-v3 build, green (`build/s1f.6.13/ci-container.log`). The DOT sweep
  took 132 s there, with the twenty new programs' views, where it takes 75 s here; the counters' step passes.
- **Not run:** the dev-host tier, the whole review bundle among its steps.

## Acceptance re-taken, 2026-11-11

- **The eight target programs of `design.md`'s round 1 answer as that page showed, under the shipped names:** § T2's
  table; seven as shown, and the division by zero as the later ruling has it.
- **Overflow and division by zero as stated; the stdlib's gates hold for the module:** `02_add_overflow.ein`,
  `06_mul_overflow.ein` and `09_div_by_zero.ein`, and `num9_theory.rs`'s *exactly one of an answer and a flag* on
  every pair at width 2; `stdlib_coverage.rs`, the mutation sweep, and the generator's `--check`.
- **H empty, k_s 1, R {∅} on every program:** `arith_search_free.rs` over `tests/stdlib/num9/` and
  `examples/arith/`; the program that divides by zero is held to an empty R decided by root alone.
