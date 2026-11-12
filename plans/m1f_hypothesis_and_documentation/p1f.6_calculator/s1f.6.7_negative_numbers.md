# S1f.6.7 — Extension: negative numbers as a positive and a negative part, simplified until one is zero

**Phase:** [P1f.6](README.md)
**Status:** **built but for division, 2026-11-12** (`f1fe6918`)**:** `stdlib/int.ein`, the stdlib's eleventh module, 36 rules and
its closing rule: the pair, its simplification, negation, addition, subtraction, multiplication and comparison,
held to Rust's signed integers, search-free, and under the rule of the theory on the text and on the counter;
five decisions taken by default, Q-M1f.174 to .178, and no finding ([§ T1 record](#t1-record-2026-11-12) onward).
**Integer division is the stage's open task:** its remainder convention is the user's question,
[Q-M1f.179](../open_questions.md#q-m1f179--the-remainder-of-an-integer-division), and
[§ T2 record](#t2-record-2026-11-12-the-remainder-convention-three-ways-for-the-user) has each of the three as
rules and computed examples. Planned 2026-11-08; restated 2026-11-11 to
[the rulings](design.md#the-rulings-2026-11-11): the numbers under it are nested, of any length, and its rules
follow S1f.6.3's rule of the theory.
**Depends on:** [S1f.6.6](s1f.6.6_division.md).

## What it builds

The user's proposal: *"numbers containing 2 parts - positive and negative, that can be simplified until one of
them is zero"*. An integer is a pair of natural numbers `(int P N)` whose value is P − N, each part a number of `std.arith`,
`z` or `(n PREFIX DIGIT)`. Simplifying a pair is
S1f.6.4's subtraction of N from P, whose two numbers under S1f.6.1's question 3 (a) are already the simplified
pair. So the normal form has one part zero, zero itself is `(int z z)`, and every integer has exactly one normal
form.

Each operation works on the parts with the natural-number circuits, then simplifies:
- **Addition:** (P1 + P2, N1 + N2).
- **Negation:** (N, P), the parts swapped.
- **Subtraction:** add the negation.
- **Multiplication:** (P1·P2 + N1·N2, P1·N2 + N1·P2). On simplified operands, only the sign rules remain.
- **Division:** on simplified operands, the magnitudes are divided by S1f.6.6 and the signs set by a rule.
  Which remainder convention it uses is the stage's question to the user:
  - **truncated:** the remainder has the dividend's sign;
  - **floored:** the remainder has the divisor's sign;
  - **Euclidean:** 0 ≤ remainder < |divisor|.

  The recommendation is Euclidean, the only one whose remainder is always a natural number.

It is a module over the natural-number theory, which it does not edit, and it is search-free as the theory is. Its
rules carry tags as the theory's do, and S1f.6.3's test of the rule of the theory covers the module. If the
expression layer is ruled in ([S1f.6.14](s1f.6.14_expressions.md)), a `sub` whose second operand is the larger
has a value here.

## Tasks

1. **The pair, its simplification and the four operations,** with negation and comparison.
2. **The remainder convention,** put to the user with the recommendation, before division is written.
3. **The tests:**
   - as for the natural numbers, against the harness's signed integers;
   - every sign combination at the exhausting width;
   - an unsimplified operand, such as (5, 3), which must give the same answers as (2, 0).
4. **The example:** S1f.6.1's programs with negative operands.

## Acceptance

- Every answer is a simplified pair and agrees with the signed integers.
- Search-free, held by the same test.
- The natural-number theory is unchanged.

## T1 record, 2026-11-12

**`std.int`,** [`stdlib/int.ein`](../../../stdlib/int.ein), written by hand: 34 relations, 36 rules and the
closing rule (`grep -c '^(relation ' stdlib/int.ein`, `grep -c '^(rule ' stdlib/int.ein`, 2026-11-12). It imports
`std.arith` by its entry and edits nothing of it.

- **The integer** is `(int P N)`, P less N, each part a number of `std.arith`. The normal form has one part `z`
  and the other in `std.arith`'s normal form; zero is `(int z z)`. An operand is read by value, whatever its
  parts and their high zeros; every answer is in normal form.
- **The names**
  ([Q-M1f.174](../open_questions.md#q-m1f174--the-names-of-stdint), defaulted, the user's to revise): the
  requests `int-simplify`, `int-neg`, `int-add`, `int-sub`, `int-mul` and `int-cmp`, answered by `int-simple-of`,
  `int-neg-of`, `int-sum-of`, `int-diff-of`, `int-product-of` and `int-cmp-of`; the entry `int-arith`. A
  difference is **one integer**: `std.arith`'s two numbers are its two parts.
- **The operands**
  ([Q-M1f.175](../open_questions.md#q-m1f175--what-stdint-reads-as-an-operand), defaulted): an integer written
  out, or a name that `std.arith`'s `(let NAME (int P N))` states. One rule, `int-operand`, says that an integer
  stands for itself, and `std.arith`'s `operand-name` does the rest unchanged. A natural number is not an
  integer operand and is answered by silence, as every malformed operand of the calculator is today (S1f.6.12).
- **Simplification** is `(int-norm TAG P N)`: `std.arith`'s subtraction walk both ways at once, under the tags
  `(int-plus TAG)` and `(int-minus TAG)`. The walk that ends with no borrow hands over the part that is left,
  `int-norm-plus` or `int-norm-minus`; where P = N both hand over zero and both rules say `(int z z)`. It is
  the stage file's *S1f.6.4's subtraction, whose two numbers are already the simplified pair*.
- **A sum** is the positive parts added, the negative parts added, and the pair simplified: three rules,
  `int-add-plus`, `int-add-minus`, `int-add-norm`. **A difference** is a sum with the second operand's parts
  exchanged, one rule of its own beside its ask and its answer; **a negation** is a simplification with the
  parts exchanged.
- **A product** is (P1·P2 + N1·N2, P1·N2 + N1·P2), the stage file's formula: four products of `std.arith`, two
  sums and the simplification, seven rules in a chain, and **no rule looks at a sign**
  ([Q-M1f.176](../open_questions.md#q-m1f176--a-product-of-integers-without-a-rule-that-looks-at-a-sign),
  defaulted). On simplified operands three of the four products have a zero operand and cost a few firings each.
- **A comparison** adds across, P1 + N2 against N1 + P2, and hands the two sums to `std.arith`'s comparison
  walk under the request's own tag, so the answer arrives as `(order-is (int-cmp X Y) O)`. It builds no integer.
- **A call's tag is its request**
  ([Q-M1f.177](../open_questions.md#q-m1f177--a-calls-tag-is-its-request-and-what-an-extension-writes-of-stdariths),
  defaulted): `(int-add X Y)` is the tag of its own computation, and `int-add-answer` reads `(int-done (int-add
  ?x ?y) ?s)`. So the module declares no tag relation for a result, and its tagged calls, `int-norm`,
  `int-add-call`, `int-mul-call` (answered by `(int-done TAG INTEGER)`) and `int-cmp-call` (answered by
  `std.arith`'s `order-is`), are what S1f.6.8 calls.
- **Every rule has one premise.** No rule reads a table of `std.digits` itself, and none joins two facts. No
  `absent`, `not`, `or`, guard or `(false)`.
- **The closing rule,** `int-arith`, closes the 34 relations, names the 36 rules and names `arith`,
  `std.arith`'s entry, which is what brings the lower theory through a selection (E-20). The module writes its
  closing rule's name as a fact, `(int-rule int-arith)` (E-11).

**The circuits,** pinned by `int_theory.rs`'s `the_circuit_of_each_operation` (`cargo test -p ein-infer --test
int_theory the_circuit`, 2026-11-12):

| request | firings | depth |
|---|---:|---:|
| simplify 123078 less 4567 | 28 | 21 |
| negate −4567 | 22 | 17 |
| 123078 + −4567 | 59 | 52 |
| 123078 − −4567 | 50 | 43 |
| 123078 against −4567 | 33 | 33 |
| 123 × −456 | 142 | 136 |
| −123078 × −123078 | 342 | 330 |
| (200 less 77) × (1000 less 1456), not simplified | 413 | 406 |

The simplification's two walks run side by side, so a depth is less than its size. `std.arith`'s 123078 + 4567
is 21 firings and its 123 × 456 is 72: a sum of integers costs two addition walks and two subtraction walks, and
a product four products, of which three are by zero where the operands are simplified.

## T2 record, 2026-11-12: the remainder convention, three ways, for the user

**Integer division is not built.** The question is
[Q-M1f.179](../open_questions.md#q-m1f179--the-remainder-of-an-integer-division): for X divided by Y, both
integers and Y not zero, which Q and R with X = Q·Y + R does `(int-div X Y)` answer?

- **truncated:** Q is rounded toward zero, and R has the dividend's sign (C, Rust's `/` and `%`);
- **floored:** Q is rounded down, and R has the divisor's sign (Python's `//` and `%`);
- **Euclidean:** 0 ≤ R < |Y|, whatever the signs.

**Each was written as rules over the shipped `std.int` and `std.arith` and run**, `python3
build/s1f.6.7/intdiv_proto.py` (2026-11-12; the script writes the three prototypes under
`build/s1f.6.7/intdiv/`, gitignored, and checks every answer against Python). 924 requests each, every dividend
from −25 to 25 by every divisor from −9 to 9 but zero and some longer ones: all right, each answer in normal
form, one answer a request.

**What all three share,** nine rules. The operands; both simplified by `int-norm`; a divisor that is `(int z z)`
derives `(division-by-zero (int-div X Y))`, the request as written, and `std.arith`'s rule derives `(false)`
(shown on the prototype: `(int-div -7 (int 3 3))`); four rules, one a combination of signs, hand the two sizes to
`std.arith`'s `div-call` with the signs in the tag; and the answer rule. Then, on `(div-done TAG q r)`, the sizes'
quotient and remainder:

| | dividend ≥ 0, divisor > 0 | ≥ 0, < 0 | < 0, > 0 | < 0, < 0 | rules of its own |
|---|---|---|---|---|---:|
| **truncated** | q, r | −q, r | −q, −r | q, −r | 4 |
| **floored** | q, r | r = 0: −q, 0; else −(q+1), −(\|Y\|−r) | r = 0: −q, 0; else −(q+1), \|Y\|−r | q, −r | 9 |
| **Euclidean** | q, r | −q, r | r = 0: −q, 0; else −(q+1), \|Y\|−r | r = 0: q, 0; else q+1, \|Y\|−r | 9 |

Truncated needs no arithmetic after the sizes' division: four rules that put the signs on, and a negated zero is
`(int z z)` already, since `(int z ?q)` with `?q` zero is the normal form of zero. Floored and Euclidean each
adjust two of the four cases where a remainder is left: one more in the quotient, an addition walk, and the
divisor's size less the remainder, a subtraction walk. That is two exact-case rules, two that start the
adjustment, one between its walks and two answers, with the two plain cases: nine.

**Computed, the three side by side** (the script's table):

| X | Y | truncated: Q, R | floored: Q, R | Euclidean: Q, R |
|---:|---:|---|---|---|
| 7 | 2 | 3, 1 | 3, 1 | 3, 1 |
| −7 | 2 | −3, −1 | −4, 1 | −4, 1 |
| 7 | −2 | −3, 1 | −4, −1 | −3, 1 |
| −7 | −2 | 3, −1 | 3, −1 | 4, 1 |
| 6 | 3 | 2, 0 | 2, 0 | 2, 0 |
| −6 | 3 | −2, 0 | −2, 0 | −2, 0 |
| 6 | −3 | −2, 0 | −2, 0 | −2, 0 |
| −6 | −3 | 2, 0 | 2, 0 | 2, 0 |
| 0 | −5 | 0, 0 | 0, 0 | 0, 0 |
| 1 | 4 | 0, 1 | 0, 1 | 0, 1 |
| −1 | 4 | 0, −1 | −1, 3 | −1, 3 |
| 127645 | −25 | −5105, 20 | −5106, −5 | −5105, 20 |
| −127645 | 25 | −5105, −20 | −5106, 5 | −5106, 5 |

**What each costs,** firings of one request on the prototype (the same script):

| | rules in all | 7 ÷ 2 | −7 ÷ 2 | −127645 ÷ 25 |
|---|---:|---:|---:|---:|
| truncated | 13 | 112 | 112 | 259 |
| floored | 18 | 112 | 124 | 279 |
| Euclidean | 18 | 112 | 124 | 279 |

Most of a request is `std.arith`'s division of the sizes (84 firings for 7 ÷ 3 there) and the two
simplifications; the adjustment is twelve to twenty firings where it applies.

**What bears on the choice.**

- **The remainder's type.** Under Euclidean the remainder is never negative, so `(int-quot-of X Y Q R)` could
  answer R as a **natural number**, the one convention under which it can. Under the other two R is an integer.
  The prototype answers an integer in all three, so that the table compares like with like.
- **Agreement with `std.arith`.** All three agree with `(div X Y)` where neither operand is negative.
- **What S1f.6.8 needs:** nothing. The rationals reduce by Euclid's chain over `std.arith`'s division of sizes
  and never divide integers ([S1f.6.8](s1f.6.8_rational_numbers.md)).
- **What a reader expects.** Truncated is what C, Rust and a pocket calculator answer; floored is Python's;
  Euclidean is the mathematician's division with remainder, and the only one in which *R = 0 exactly where Y
  divides X, and otherwise R counts up from a multiple of Y* holds for a negative X.
- **Cost of building:** four rules against nine, and the tests the same.
- **More than one** is possible: three request relations, `int-div-trunc`, `int-div-floor` and `int-div-euclid`,
  are 9 + 4 + 9 + 9 rules less what the last two share, about 27.

**The recommendation is Euclidean,** as the stage file had it: the remainder is a natural number, which is what
the theory below has and what number theory (S1f.6.10: divisibility, gcd, congruences) reads; and the rule a
program writes on it, *the remainder is `z`*, means *divides* under every sign. If the user's reader is a
programmer first, truncated is the cheaper and more familiar one.

**What is prepared.** Once the user rules: the rules above move into `stdlib/int.ein` under the stage's names,
`(int-div X Y)` answered by `(int-quot-of X Y Q R)`; a row of `int_theory.rs`'s `OPS` with a second result
argument, against `i128`'s `div_euclid` and `rem_euclid` (or `/` and `%`); a row of `arith_batch.rs`; two
programs under `tests/stdlib/int/`, one of them the division by zero; and the closing rule's lists.

## T3 record, 2026-11-12: the tests

- **Eight programs** under [`tests/stdlib/int/`](../../../tests/stdlib/int/), each a corpus entry of group
  `stdlib` with its `:expect` and `(solve :expect (region (model)))`: the normal form, negation, the four
  operations each in every combination of signs, an unsimplified operand answering as its normal form does (the
  stage's task 3), and operands by name with two rules of the program's own. Every claim was computed by
  Python's integers when the programs were drafted (`build/s1f.6.7/draft_tests.py`), and `ein test
  tests/stdlib/int` holds 23 of 23. `stdlib_coverage.rs`: 250 of 250 over the stdlib (`cat stdlib/*.ein | grep
  -c '^(rule '`, 2026-11-12).
- **The harness against the signed integers,**
  [`ein-infer/tests/int_theory.rs`](../../../ein.rs/crates/ein-infer/tests/int_theory.rs), a sibling of
  `arith_theory.rs` with an `OPS` table of its own
  ([Q-M1f.178](../open_questions.md#q-m1f178--the-harness-of-stdint-a-sibling-file-exhaustive-on-one-digit-parts),
  defaulted). The oracle is `i128`. It passed on its first run.
  - **The exhausting width:** every integer written with parts from 0 to 9, 100 spellings of −9 to 9, against
    every other, 10 000 requests an operation: every combination of signs and of simplified and unsimplified
    operands, with exactly one answer a request, in normal form.
  - **Every length:** every pair of lengths to 30 digits (18 for a product) in the four combinations of signs,
    equal operands, operands one apart, opposite operands, and 200 pairs with both parts long.
  - **By value:** 1 400 pairs of operands written with a common amount added to both parts and high zeros.
  - Simplification and negation, on every pair of parts to 30 and at every length; requests by name; a natural
    number in an operand's place, unanswered.
  - On the text: every rule has exactly one premise; no `absent`, `not`, `or`, guard or `(false)`; the closing
    rule closes every relation, names every rule and `arith`, and the module writes its one fact.
- **The batch,** `arith_batch.rs`'s `a_batch_of_integer_requests_costs_in_proportion_to_its_count`, operands
  with parts of six digits (four for a product), every third not simplified (`cargo test -p ein-infer --features
  counters --test arith_batch a_batch_of_integer -- --nocapture`, 2026-11-12):

  | operation | requests | firings a request | candidates a request |
  |---|---:|---:|---:|
  | `int-add` | 250 / 1 000 / 4 000 | 61.52 / 61.72 / 61.70 | 475.68 / 477.52 / 477.16 |
  | `int-sub` | 250 / 1 000 / 4 000 | 61.97 / 61.65 / 61.53 | 480.02 / 477.12 / 475.90 |
  | `int-mul` | 250 / 1 000 / 4 000 | 303.68 / 300.06 / 299.20 | 3 930.60 / 3 881.28 / 3 863.53 |
  | `int-cmp` | 250 / 1 000 / 4 000 | 43.93 / 43.61 / 43.52 | 870.22 / 867.32 / 865.36 |

- **On the release binary,** beside `std.arith` asked the same operation on the sizes (`python3
  build/s1f.6.7/batch.py`, 2026-11-12; integers of up to nine digits and either sign, four for a product; the
  saturator's own milliseconds, best of five): 4 000 sums in 357 ms against 117, differences 362 against 184,
  comparisons 256 against 81, products 864 against 328, and 56, 56, 38 and 121 firings a request against 22, 29,
  13 and 58. An integer operation is two and a half to three times the natural one.
- **The search-free test,** `arith_search_free.rs`: `tests/stdlib/int` joins its roots, `(import std.int ` its
  theories and the six results its result relations.
- **The mutation sweep** kills every mutant of the module, 42 of 42: 317 of 367 where 275 of 325, the bank's 50
  survivors unmoved (`python3 utils/stdlib_mutants.py --check`, 2026-11-12). No program asks a pair both ways
  round, `std.num9`'s lesson, and `03_add.ein` says why.

## T4 record, 2026-11-12: the example, the pages, and what moved

- **The example,** [`examples/arith/int.ein`](../../../examples/arith/int.ein): −12 + 40, 4567 − 123078 as one
  integer, −123 × 456, a simplification and a negation, and three named temperatures compared with freezing by
  two rules of the program's own. Seven claims.
- **The sweep.** One engine and two stdlibs, the release binary and stdlib of `f30fb51d` against the tree's,
  `ein saturate --dump` and `ein solve` over the 719 `.ein` files `f30fb51d` tracks: no answer moved; one file's
  error line prints the stdlib's directory, which the sweep sets (`python3 build/s1f.6.7/sweep.py`, 2026-11-12).
  No engine source moved, and `stdlib/arith.ein`, `digits.ein` and `num9.ein` are byte for byte what they were.
- **A program that imports `std.int`** pays 22.4 ms of load where one that imports `std.arith` pays 19.9 (`ein
  saturate examples/arith/int.ein` and `add.ein`, the `kb load:` line, best of five, 2026-11-12). **Re-taken:**
  this line first read 24.0 against 19.8, a single run of each; S1f.6.8 re-took both best of five
  ([its T4 record](s1f.6.8_rational_numbers.md)), and S1f.6.10 corrected the line to those figures.
- **The goldens that move,** named before `EIN_BLESS=1`: `ein-cli/tests/golden/corpus_exits.txt`, 50 new cells, the
  ten new files'; `ein-render/tests/golden/corpus_shapes.md5`, 420 new lines, their 42 renderings each, and no
  other line (`git diff --numstat`, 2026-11-12).
  `alive_invariant.rs`'s list gains the ten files' `z`.
- **The register** is unmoved: 305 fixed, 16 open and 1 not wrong (`grep '^status' corpus/wrong_answers.toml |
  sort | uniq -c`, 2026-11-12). No finding for `findings.md`.

Pages: `stdlib/README.md` (§ `std.int`, new), `tests/README.md` (§ `stdlib/int/`, new), `07_stdlib_api.md`
(§ `std.int`, new), `examples/README.md`, `corpus/corpus.toml`, `AGENTS.md`, the questions Q-M1f.174 to .179,
the phase README and this record.

**The gate** is green (`./run_tests.sh`, `build/gate-s1f.6.7.log`, 2026-11-12). `python3 utils/stdlib_census.py
--check`, `python3 utils/nl_census.py --check` and `python3 utils/stdlib_mutants.py --check` pass on the tree.
The nightly was run after the last code commit of this run, S1f.6.9's, and is recorded [there](s1f.6.9_width.md#after-the-commit-2026-11-12-the-nightly-for-s1f67-s1f68-and-s1f69).

## Acceptance re-taken, 2026-11-12

- **Every answer is a simplified pair and agrees with the signed integers:** `int_theory.rs`, through `hold`.
  **But for division, which is not built** and waits for Q-M1f.179.
- **Search-free, held by the same test:** `arith_search_free.rs` over `tests/stdlib/int/` and
  `examples/arith/int.ein`.
- **The natural-number theory is unchanged:** `git diff f30fb51d -- stdlib/arith.ein stdlib/digits.ein
  stdlib/num9.ein` is empty.
