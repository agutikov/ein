# S1f.6.33 — `std.rat` reduces only when asked

**Phase:** [P1f.6](README.md)
**Status:** planned 2026-11-12, on the user's reply of that day in round 2. **Done 2026-11-15** ([§ Record](#record-2026-11-15)): T1's table first, the design being ruled, then T2 to T6.
**Depends on:** [S1f.6.32](s1f.6.32_gcd_and_lcm.md), since `rat-simplify` then calls `std.math.gcd`;
[S1f.6.31](s1f.6.31_the_stdlib_layout.md), the layout; and [S1f.6.24](s1f.6.24_division_by_zero_at_the_call.md),
which is the other stage that edits `std.rat`'s rules.

## The ruling

[Question 54](questions.md#q54) asked whether lowest terms by Euclid's chain over division should be left,
though it is costly. The user, 2026-11-12:

> gcd cost is log2? of what, of number length or value? Euclid's chain complexity? I think we don't need to
> reduce every rational, can ask explicitely for it?

[Round 2, item 54](questions_round2.md#q54) answers the cost (the count of steps is linear in the number's
*length*; over a whole chain the work is about the square of the length, with a large constant, since a
long division's nine multiples are computed again for every step) and puts three options: (a) leave it,
every answer in lowest terms; **(b) reduce only on request:** *the operations answer unreduced,
`rat-simplify` reduces, `rat-cmp` is the equality. The tests that hold "every answer is in lowest terms"
are restated*; (c) both, by request name. The user's reply there (`dc88520b`): *"(b)"*.

## What changes, in round 2's words

- *An answer is no longer canonical: 1/2 + 1/2 is `4/4`, and two equal results are two different facts. A
  rule that wants "the same number" must ask `rat-cmp`, which already compares by cross products and never
  needed lowest terms.*
- *A chain of operations grows: denominators multiply at every step unless something reduces.*
- *A sum falls to about a fifth of its firings.* Today 1/2 + 1/3 is 492 firings, where the comparison of
  1/2 with 1/3, which reduces nothing, is 76 (`questions.md` 54).
- It *removes the reason gcd lives inside `std.rat`, since `rat-simplify` can then call `std.math.gcd`.*

## What it builds

- `rat-add`, `rat-sub`, `rat-mul`, `rat-div` and `rat-neg` answer **unreduced**: the cross products over
  the product of the denominators, as computed, with the sign in the numerator's integer in normal form
  and the denominator a natural number that is not zero. What *unreduced* fixes and what it leaves is
  stated: the integer parts are simplified by `std.int` as they are today (an `(int P N)` with one part
  `z`); only the common factor of numerator and denominator stays.
- `rat-simplify`, answered by `rat-simple-of`, reduces, and is the one request that does. It calls
  `std.math.gcd` and one division of each part; `std.rat`'s own Euclid chain goes.
- `rat-cmp` is the equality and the order, unchanged.
- **Zero:** today zero is written 0 over 1. Unreduced, 0/3 is an answer; `rat-simplify` gives 0 over 1.
- **A zero denominator** is flagged as built (round 2, item 48, point 2).

## Tasks

1. **T1f.6.33.1 — shown first, on real programs:** the banked `examples/arith/rat.ein` and the thirteen
   programs under `tests/stdlib/rat/` as they answer today and as they would, side by side, on a copy of
   the stdlib under `build/`; every claim that names a reduced answer listed, with what it becomes (the
   unreduced answer, or the same claim after a `rat-simplify` the program now asks for). The word problems
   and every other program that imports the rationals are in the table. Firings a request before and after.
2. **T1f.6.33.2 — the rules,** in `std.math.rat`: the reduction taken out of each operation's chain;
   `rat-simplify` over `std.math.gcd`; the closing rule's lists, which now name `gcd`'s entry.
3. **T1f.6.33.3 — the harness restated.** `rat_theory.rs` holds every answer to an exact fraction **by
   value** (cross-multiplied against the oracle), and holds *in lowest terms* of `rat-simplify`'s answers
   alone; *exactly one answer a request* stays. A new case: two requests whose values are equal and whose
   answers are different facts, and `rat-cmp` saying `eq` of them. The sizes stay small for the reason they
   were (a stored fact costs a kilobyte, E-26), and the task says whether unreduced answers let them grow.
4. **T1f.6.33.4 — the programs and the census:** `tests/stdlib/rat/` rewritten to T1's table, a program per
   rule; `stdlib_coverage.rs`; `arith_search_free.rs`'s lists; the mutation census with no unexplained
   survivor; the batch by hand.
5. **T1f.6.33.5 — the sweep and the goldens:** old stdlib against new over every tracked `.ein` file;
   every program that moves is one that asked for a rational, and each is in T1's table; the goldens named
   before blessing.
6. **T1f.6.33.6 — the pages:** [`stdlib/README.md`](../../../stdlib/README.md)'s section (*an answer is not
   canonical; ask `rat-simplify` for lowest terms and `rat-cmp` for equality*), the module's header,
   `AGENTS.md`'s S1f.6.8 paragraph, Q-M1f.183 marked ruled with the user's words.

## Acceptance

- No operation of the module reduces but `rat-simplify`; every answer is right by value, and
  `rat-simplify`'s are in lowest terms.
- A sum's firings are measured before and after and reported; round 2's *about a fifth* is confirmed or
  corrected with the figure.
- `std.rat` holds no gcd of its own.
- Every program that claimed a reduced answer claims what T1's table says.

## What it must not do

- **Not keep a reduced family beside the raw one** (option (c) was not chosen).
- **Not change `rat-cmp`,** nor what a zero denominator is.

## The standing rules

A wrong answer of the engine met on the way is a row in `corpus/wrong_answers.toml`, filed `open` before its
fix. Findings go to [`findings.md`](findings.md) under the next free id; questions to
[`open_questions.md`](../open_questions.md) under the next free id. A design change shows real computed
examples first (T1). Config never changes an answer. The gate is `./run_tests.sh`, then `./run_tests.sh
--nightly` after the code commit; `utils/ci_container.sh` is not needed.

## Record, 2026-11-15

**Status: built.** T1's table first, on a copy of the stdlib, and then T2 to T6 without a stop, the design being
the user's *"(b)"*; every claim rewritten as the table says. Commits: `a4771ea4` (T2 to T5: the module, the harness,
the programs, the goldens), `fc0e5541` (T6, the pages), and this record's, on S1f.6.32's (`ea217198`).

### T1f.6.33.1 — the table, on real programs

Two copies of the stdlib under `build/s1f.6.33/`, `stdlib-a` as shipped after S1f.6.32 and `stdlib-b` with
`std.math.rat` under (b) (`build/s1f.6.33/mk_rat_b.py`, the module this stage then shipped), and **every tracked
program that uses the rationals** (`git ls-files '*.ein'`, a file that imports `std.math.rat` or writes a `rat-`
request; no word problem, no Rust test's inline program but the harnesses' own, below): `ein test` of each under
both, the release binary of `f69e1b65`'s engine, which this stage does not change (`python3
build/s1f.6.33/t1.py`, 2026-11-15). **32 programs, 57 claims; under (b) 10 programs refute 16 claims**, each a
claim that names a reduced answer, and no other claim moves. The 22 others answer alike: the 19 dead ones (a zero
denominator, a malformed operand or rational; their one claim `(false)`), `01_simplify` (lowest terms is
`rat-simplify`'s, still), `07_cmp` (a comparison reduces nothing) and `tests/stdlib/math/decimal/01`, which writes
a rational and asks nothing of it.

| program | claims moved | the answers today → under (b) | rewritten to |
|---|---|---|---|
| `examples/arith/rat.ein` | 3 of 7 | 1/2 ÷ −3/4: −2/3 → −4/6; flour × scale: 9/8 → 18/16; `short-of flour`: 9/8 → 18/16 | the unreduced answers, and **two rules of the program's own ask `rat-simplify`**, of the quotient and of the flour the cook is short of, so that the recipe says 9/8 as a cook would; `rat-simple-of`'s claim gains −4/6 → −2/3 and 18/16 → 9/8 |
| `tests/stdlib/math/rat/02_neg` | 1 of 2 | −(−3/6): 1/2 → 3/6; −(2/4): −1/2 → −2/4; −(0/5): 0/1 → 0/5 | the unreduced answers |
| `03_add` | 1 of 2 | 1/4 + −3/4: −1/2 → −8/16; 1/6 + 1/3: 1/2 → 9/18; 5/7 + −5/7: 0/1 → 0/49; 7/10 + 13/15: 47/30 → 235/150; −2/3 + −1/6: −5/6 → −15/18 | the unreduced answers |
| `04_sub` | 1 of 2 | 3/4 − −1/4: 1/1 → 16/16; 5/7 − 5/7: 0/1 → 0/49; −1/2 − 1/6: −2/3 → −8/12; −2/3 − −1/6: −1/2 → −9/18 | the unreduced answers |
| `05_mul` | 1 of 2 | 12/35 × 25/18: 10/21 → 300/630; 2/3 × 3/4: 1/2 → 6/12; 4/5 × −5/8: −1/2 → −20/40; 7/1 × 1/7: 1/1 → 7/7; −1/2 × −2/7: 1/7 → 2/14; −2/3 × 3/5: −2/5 → −6/15; 0/3 × −5/9: 0/1 → 0/27 | the unreduced answers |
| `06_div` | 1 of 2 | 1/2 ÷ 3/4: 2/3 → 4/6; 1/2 ÷ −3/4: −2/3 → −4/6; 7/9 ÷ 3/6: 14/9 → 42/27; −3/8 ÷ −9/4: 1/6 → 12/72; −5/6 ÷ 2/3: −5/4 → −15/12; 0/5 ÷ −2/3: 0/1 → 0/10 | the unreduced answers |
| `08_unsimplified` | 4 of 6 | 2/4 + −3/5: −1/10 → −2/20; 2/4 − −3/5: 11/10 → 22/20; 2/4 × −3/5: −3/10 → −6/20; 2/4 ÷ −3/5: −5/6 → −10/12 (1/2's four unmoved) | the unreduced answers, and **four rules of the program's own ask `rat-simplify` of each answer**: the program's point, *2/4 and 1/2 are one rational*, is now that the two answers have one lowest terms, a sixth claim, `rat-simple-of` whole |
| `09_named_and_chained` | 2 of 4 | 5/6 × share: 5/8 → 15/24; the chain's comparison then asks 15/24 against half: `gt` | the unreduced answers: the chain carries 15/24 |
| `22_integer_where_a_rational_goes`, `23_natural_where_a_rational_goes` | 1 of 2 each | 1/2 + 1/2: 1/1 → 4/4 | the unreduced answer, round 2's own example |

**One program is new**, `tests/stdlib/math/rat/31_equal_values_two_facts.ein`: 1/2 + 1/2 = 4/4, 1/4 + 3/4 = 16/16
and 3/2 × 2/3 = 6/6, three facts of one value; a rule of the program's own asks `rat-cmp` of each against one, `eq`
three times, and `rat-simplify` of each, 1/1 three times: *a rule that wants the same number asks `rat-cmp`*.

**Firings a request, before and after** (`rat_theory.rs`' circuit, the closing rules' firings left out; before the
pinned measures of S1f.6.8, re-taken on `stdlib-a`; after on `stdlib-b`, `std.math.gcd`'s closing rule left out
too; 2026-11-15):

| request | today | under (b) | (b) / today |
|---|---:|---:|---:|
| 2/4 in lowest terms | 334 | 345 | 1.03 |
| 1071/462 in lowest terms | 867 | 888 | 1.02 |
| −3/4 negated | 435 | 25 | 0.06 |
| 1/2 + 1/3 | 492 | 101 | 0.21 |
| 1/2 − 1/3 | 417 | 104 | 0.25 |
| 2/3 × 3/4 | 459 | 81 | 0.18 |
| 1/2 ÷ −3/4 | 494 | 91 | 0.18 |
| 1/2 against 1/3 | 76 | 76 | 1 |
| 123/456 + 789/1011 | 2 640 | 386 | 0.15 |
| 123/456 against 789/1011 | 270 | 270 | 1 |

**Round 2's *a sum falls to about a fifth of its firings* is confirmed:** 492 to 101, 0.21; a product to 0.18, a
negation, which divided by itself before, to 0.06, and a sum of three-digit terms to 0.15. `rat-simplify` costs 11
and 21 firings more, `std.math.gcd`'s two walks to normal form (S1f.6.32's T5) less the one step its chain no longer
has; a comparison is unchanged.

### T1f.6.33.2 — the rules

`stdlib/math/rat.ein`, from 67 rules to 65, every one of one premise, as `build/s1f.6.33/mk_rat_b.py` writes it from
the shipped module (the copy T1 ran is the module shipped, byte for byte):

| what | before | after |
|---|---|---|
| `(rat-norm TAG P N D)` | the numerator simplified, then lowest terms (`rat-norm-numerator`, `rat-norm-reduce`) | the numerator simplified and D walked to normal form, **nothing divided** (`rat-norm-numerator`, `rat-norm-denominator`, `rat-norm-done`; [Q-M1f.302](../open_questions.md#q-m1f302--what-stdmathrats-unreduced-answer-fixes-and-rat-norm-under-b), defaulted) |
| a product, a quotient | `rat-mul-reduce` to `rat-reduce` | `rat-mul-done`: the numerators' product over the denominators', as computed |
| a sum, a difference, a negation | through `rat-norm`, reduced | through `rat-norm`, unreduced |
| `rat-simplify` | through `rat-norm` | `rat-simplify-start` simplifies the numerator under a tag of its own, `rat-low`; `rat-simplify-pos` and `-neg` take its sign off and call **`std.math.gcd`'s `(gcd-call TAG A D)`**, the numerator's size first ([Q-M1f.303](../open_questions.md#q-m1f303--rat-simplify-asks-stdmathgcd-the-numerator-first), defaulted); `rat-lowest-numerator` reads `gcd-done`; the two divisions and `rat-lowest-pos`, `-neg` as before |
| Euclid's chain | `rat-reduce-pos`, `-neg`, `rat-gcd-step`, `-next`, `-end`; relations `rat-reduce`, `rat-gcd`, `rat-gcd-is`, `rat-gcd-rem` | **gone**: `std.math.rat` holds no gcd of its own |
| the closing rule | names `int-arith`, `decimal-arith`, `decimal`, `arith` | and `gcd`; the module imports `std.math.gcd` |

`rat-cmp` is unchanged, and so is what a zero denominator is: the four flagging rules, their tags and their one flag a
request. The header says what an answer is now (*not reduced; not canonical; `rat-simplify` for lowest terms,
`rat-cmp` for equality; a chain of operations grows*) and the interface's two changed meanings, `rat-norm`'s and
`rat-mul-call`'s, their shapes unchanged (`arith_theory.rs`' `the_tagged_calls_keep_their_shapes` holds them).

### T1f.6.33.3 — the harness restated

`ein-infer/tests/rat_theory.rs`, 14 tests (12 before), `cargo test --profile gate -p ein-infer --test rat_theory`: 14
of 14, 3.5 s, on AC.

- **Every answer by value**: `hold` reads the answer as `(rat (int P N) D)` **in the module's form** (one part `z`, no
  high zero, D not zero; an answer out of form fails as a wrong one does) and compares it with the oracle's fraction by
  cross products, falling back to reduction where `i128` would overflow (two twelve-digit operands make a numerator of
  twenty-four digits). The exhausting set, the samples, the operands spelt every way and the names all go through it.
- **`rat-simplify`'s answers alone in lowest terms**, held as written; `rat-neg`'s by value.
- **`only_rat_simplify_reduces`**, new: every pair of the 28 fractions under the four operations, each answer held to
  its unreduced spelling as computed, the cross products or the numerators' product over the product of the
  denominators: an operation that reduced anything fails here.
- **`equal_values_are_two_facts_and_rat_cmp_says_eq`**, new, T3's case: five pairs of requests of one value, 1/2 + 1/2
  and 1/4 + 3/4 among them, two different facts each, `rat-cmp` saying `eq` of the two, and `rat-simplify` giving them
  one lowest terms.
- *Exactly one answer a request* stays (`hold`, and the named and the unary tests).
- The zero-denominator and the integer-operand tests read the unreduced answers, 1/2 ÷ 1/2 = 2/2, 1/2 + 1/2 = 4/4;
  the closing-rule test names `gcd` and the import of `std.math.gcd`; the circuit leaves out `std.math.gcd`'s closing
  rule too, and the circuits are re-pinned: T1's table, to the firing.
- **The sizes stay** (28 fractions, programs of 250 requests): a stored fact costs a kilobyte (E-26), and the
  measures compare. Unreduced answers would let them grow about fivefold for the same facts, a request being a fifth
  of its firings; nothing in the claim needs it, and it is not done.

`arith_batch.rs`: the rationals' bounds are the new measure's, a little above it (750 candidates a request for a sum
or a difference, 650 for a product or a quotient, 900 for a comparison, as before), where lowest terms needed 10 000
and 9 500; their doc comments say why.

### T1f.6.33.4 — the programs and the census

`tests/stdlib/math/rat/` rewritten to T1's table, claim by claim (`build/s1f.6.33/rewrite.py` replaced each refuted
fact by (b)'s answer for the same request, the three written over two lines by hand), and each header's arithmetic
with it: `02` to `06`, `09`, `22`, `23` claim the unreduced answers; `08` asks `rat-simplify` of each answer by four
rules of its own and claims the two spellings' one lowest terms; `01`'s header names the rules that now run,
`std.math.gcd`'s call among them; `31_equal_values_two_facts.ein` is new. `examples/arith/rat.ein` claims the
unreduced answers and asks `rat-simplify` of the quotient and of the flour it is short of. Every rule of the module is
activated: `rat-norm-denominator` and `-done` by every sum and negation, `rat-simplify-pos` and `-neg` by `01`,
`rat-mul-done` by `05` (`stdlib_coverage.rs`: every rule of the stdlib activated, **499 of 499**, two fewer than
S1f.6.32 left, `find stdlib -name '*.ein' -exec cat {} + | grep -c '^(rule '`, 2026-11-15). `arith_search_free.rs` needs no change: the module's results
and requests are as they were, and every program of `tests/stdlib/math/rat/` is held search-free, `31` with it.

**The mutation census** (`python3 utils/stdlib_mutants.py --jobs 16`, 6 min 41 s, on AC, 2026-11-15): **498 of 617
killed**, where S1f.6.32 left 494 of 613: the module's mutants are its new rules' too (a conclusion's or a premise's
arguments exchanged in `rat-norm-denominator`, `-done`, `rat-simplify-pos`, `-neg`, `rat-mul-done`, and the chain's
gone with its rules), every one killed on the first run, and **no new survivor**: the 119 are the banked set,
`tests/mutation_survivors.txt` unmoved.

The batch by hand, whole (`cargo test -p ein-infer --features counters --test arith_batch -- --nocapture`,
2026-11-15): 8 of 8, every module in proportion. **The rationals at 100 requests**: a sum 110.5 firings and 669.8
candidates a request, a difference 109.8 and 665.2, a product 84.5 and 501.7, a quotient 95.1 and 549.3, a
comparison 77.8 and 656.0, where S1f.6.32's tree had 651.4 and 7 526.0 for a sum and 597.9 and 6 972.7 for a product
(0.17 and 0.14 of the firings), and within a few per cent of that at 400 and 1 200. Decimal's, the integers', binary's
and `std.math.gcd`'s as S1f.6.32 left them.

### T1f.6.33.5 — the sweep, and the goldens

**Old against new over every corpus entry** (`build/s1f.6.33/sweep.py`, S1f.6.38's tool: `ein solve --json-summary`
and `ein saturate --dump` of each, the release binary, the tree of `ea217198` against this one, each on its own
stdlib; `compare --answers`, 2026-11-15): **864 programs in both; 15 answer differently**:

- **the 10 programs of T1's table whose claims moved**, by their queries' classes, as the table says, and no other
  field: R, root's standing and the claims' verdicts as before (every claim holds, rewritten);
- five of `examples/broken/load/`, whose load error names the tree's path, as at S1f.6.16;
- and `31_equal_values_two_facts.ein`, new.

**Firings moved on 33 programs, every one a program that brings `std.math.rat`**: the 32 of T1's table and the
module itself, 144 525 firings over the corpus to 127 119 (−12.0 %). The ones that compute fell most:
`03_add` 4 585 to 933, `05_mul` 3 580 to 631, `09` 1 378 to 303, `examples/arith/rat.ein` 2 370 to 1 777 (it now asks
two `rat-simplify`); `01_simplify` rose 4 385 to 4 513 and `08` 4 205 to 4 522 (four `rat-simplify` more); the
dead ones that state a sum beside their fault fell (`10` 519 to 129), and those that compute nothing rose by one,
`std.math.gcd`'s closing rule. No program that does not bring the rationals moved.

**The goldens**, named before blessing (`EIN_BLESS=1` over `corpus_cli` and `corpus_shapes`): `corpus_shapes.md5`
**775 renderings moved over 33 programs, exactly the 32 of T1's table and `stdlib/math/rat.ein`**, and 41 new, `31`'s;
`corpus_exits.txt` 5 cells new, `31`'s, and **one moved, `08_unsimplified.ein :: render rules`, 1 to 0**: `render
rules` exits 1 on a program that writes no rule (*no rule forms*), and `08` now writes four. `alive_invariant.rs`'
`SEEDED` gains `31`, which meets `pos` and `neg` in `std.math.int`'s division's rules as its siblings do; their lines
did not move.

### T1f.6.33.6 — the pages

`stdlib/README.md`: the layout's row, the section's title (*reduced on request*), a paragraph *an answer is not
canonical; ask `rat-simplify` for lowest terms and `rat-cmp` for equality*, the table's rows, how it computes, the
interface's two calls, the harness, and the costs before and after: the circuits of T1's table, the saturator's
milliseconds (below) and the batches; the modules' table; the coverage count, 499 of 499. The module's header
(T2). `docs/kernel/ir/03-ein-lang/07_stdlib_api.md`'s section. `tests/README.md`'s row and section, a line a
program, `31` among them, and the census. `examples/README.md`'s line. `AGENTS.md`: the stage file's *S1f.6.8
paragraph* is no longer in it (its history left at S1f.6.31); the line on `std.math.rat` in the stdlib's paragraph
says the answer is unreduced, `rat-simplify` reduces by `std.math.gcd`, and equality is `rat-cmp`.
[Q-M1f.183](../open_questions.md#q-m1f183--lowest-terms-by-stdariths-division-and-what-it-costs) marked ruled with
the user's *"(b)"*.

### What a sum and a product cost, before and after

The saturator's own milliseconds and every firing `ein saturate` counts (the closing rules and the numerals' checks
included), medians of eleven interleaved rounds on cpu2, a P-core, `powersave`, **on AC** (ACAD online=1;
`utils/bench_env.sh --core 2 --require-ac python3 build/s1f.6.33/cost.py 11`, 2026-11-15; *before* on `stdlib-a`,
*after* on `stdlib-b`, the shipped module; two fractions a request with a numerator from 1 to 9 of either sign over a
denominator from 2 to 9):

| program | before: ms | firings | after: ms | firings | firings after / before |
|---|---:|---:|---:|---:|---:|
| one sum, 1/2 + 1/3 | 1.77 | 497 | 1.10 | 107 | 0.215 |
| one product, 2/3 × 3/4 | 1.73 | 464 | 1.08 | 87 | 0.188 |
| one `rat-simplify`, 1071/462 | 2.37 | 872 | 2.45 | 894 | 1.025 |
| 400 sums | 469.57 | 255 594 | 71.81 | 44 007 | 0.172 |
| 400 products | 437.08 | 236 007 | 56.71 | 33 786 | 0.143 |

**Round 2's *about a fifth* is confirmed**: a sum's circuit 101 firings of 492 (0.21), a batch of sums 0.17 of the
firings and 0.15 of the milliseconds; a product less still. The load is a millisecond more, `std.math.gcd` being
brought (35.5 ms against 34.4 for the one sum, `kb load:`, median of seven, cpu2, on AC).

### Defaults, findings

- **Ruled, built:** [Q-M1f.183](../open_questions.md#q-m1f183--lowest-terms-by-stdariths-division-and-what-it-costs),
  round 2's *"(b)"*.
- **New, each defaulted, the user's to revise:**
  [Q-M1f.302](../open_questions.md#q-m1f302--what-stdmathrats-unreduced-answer-fixes-and-rat-norm-under-b) (an
  answer with no high zero; `rat-norm`'s meaning in the interface),
  [Q-M1f.303](../open_questions.md#q-m1f303--rat-simplify-asks-stdmathgcd-the-numerator-first) (`rat-simplify`
  asks the numerator first).
- **The rewrite choices** are T1's table's last column: the unreduced answer wherever the program's point is the
  operation, a `rat-simplify` the program asks where its point is a value (`08`, the rational example).
- No finding, and no wrong answer of the engine met: every program that moved is one T1's table names.

### What it did not do

No reduced family beside the raw one (option (c) was not chosen); `rat-cmp` and a zero denominator unchanged; the
harness's sizes unchanged; no engine change, so no nightly (the user's cadence) and no `utils/ci_container.sh` (no
DOT writer, review bundle or workflow touched).


### The gate (#4), for the batch of S1f.6.32 and S1f.6.33

`./run_tests.sh` once over the batch, on `fc0e5541` (both stages' code and pages; the records uncommitted beside
it), here, **on AC** (ACAD online=1 at its start and its end): **exit 0, 17 min 15 s**. The six static checks clean
(the manifest, 24 modules; the hash-map check; the link check over 563 pages, no finding; `cargo fmt --check`;
clippy and rustdoc with `-D warnings`); `cargo test --profile gate --workspace` **1 243 tests in 145 binaries, none
failing**; the scratch check, the review bundle's smoke and the bench smoke (`build/s1f.6.33/gate.log`). One gate for
the batch, the user's cadence of 2026-11-13: S1f.6.32's code ran under `cargo test --profile gate --workspace` before
its commit (one failure, `alive_invariant.rs`' `SEEDED`, which `12_what_stays_silent.ein` joined, fixed before
`8ccf48e5`), and nothing the gate found needed a second run. No nightly: neither stage changes the engine. No
`utils/ci_container.sh`: no DOT writer, review bundle or workflow touched (Q-M1f.77).
