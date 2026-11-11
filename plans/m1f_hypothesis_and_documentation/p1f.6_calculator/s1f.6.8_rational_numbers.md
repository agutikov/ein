# S1f.6.8 — Extension: rational numbers, an integer numerator over a natural denominator

**Phase:** [P1f.6](README.md)
**Status:** **done 2026-11-12:** `stdlib/rat.ein`, the stdlib's twelfth module, 56 rules and its closing rule:
the rational, lowest terms by Euclid's chain, negation, the four operations and comparison, a zero denominator
as a division by zero; held to exact fractions, search-free, and under the rule of the theory on the text and on
the counter; five decisions taken by default, Q-M1f.180 to .184, and no finding
([§ T1 record](#t1-record-2026-11-12) onward). **It did not need integer division**, so it did not wait for
Q-M1f.179. Planned 2026-11-08; restated 2026-11-11 to [the rulings](design.md#the-rulings-2026-11-11): the
numbers under it are nested, of any length, and its rules follow S1f.6.3's rule of the theory.
**Depends on:** [S1f.6.7](s1f.6.7_negative_numbers.md).

## What it builds

The user's proposal: rational numbers *"containing integer numerator and natural denominator"*. A rational is
`(rat Z D)`: Z is an integer in S1f.6.7's form and D a non-zero natural number. Its normal form has the
numerator and denominator coprime, and zero written as 0 over 1.

**Simplifying needs the gcd, and Euclid's algorithm is a chain of divisions.** gcd(a, b) is gcd(b, a mod b)
until the remainder is zero. Each step is a request that the previous step's remainder makes, answered by
S1f.6.6's division, so the algorithm runs as saturation rounds. It terminates because the remainders strictly
decrease. Dividing both parts by the gcd is two more divisions.

**The operations, each followed by simplification:**
- **Addition:** (Z1·D2 + Z2·D1) / (D1·D2).
- **Subtraction:** add the negation.
- **Multiplication:** (Z1·Z2) / (D1·D2).
- **Division,** by a non-zero rational: (Z1·D2) / (D1·|Z2|), with Z2's sign moved to the numerator.
- **Comparison:** compare Z1·D2 with Z2·D1. Each denominator is positive, so the order is the order of the
  cross products.

A zero denominator, written by a program or reached by dividing by a zero rational, is reported, as
S1f.6.1 ruled for division by zero.

**No width limits it.** Cross products are twice as long as their operands, and nested numbers hold them.
The gcd chain is the program `le/p/gcd.ein` of [design.md § C](design.md#i-round-1s-request-and-answer-facts),
rewritten with tags so that it follows the rule of the theory.

## Tasks

1. **The gcd chain and the normal form.**
2. **The operations and comparison.**
3. **The tests:**
   - against the harness's exact fractions;
   - every operand pair at the exhausting width;
   - an unsimplified operand, such as 2/4, which must answer as 1/2.
4. **The example:** a short computation over rationals, such as 1/2 + 1/3 = 5/6.

## Acceptance

- Every answer is in normal form and agrees with exact fractions.
- Search-free, held by the same test.
- The natural-number and integer theories are unchanged.
- Where overflow limits the extension, it says so with the command that shows it.

## T1 record, 2026-11-12

**`std.rat`,** [`stdlib/rat.ein`](../../../stdlib/rat.ein), written by hand: 48 relations, 56 rules and the
closing rule (`grep -c '^(relation ' stdlib/rat.ein`, `grep -c '^(rule ' stdlib/rat.ein`, 2026-11-12). It imports
`std.arith` and `std.int`, each by its entry, and edits neither.

- **The rational** is `(rat Z D)`: Z an integer written `(int P N)`, D a number of `std.arith` that is not zero.
  The normal form is in lowest terms, the numerator in `std.int`'s normal form, the denominator with no high
  zero, and zero written 0 over 1. An operand is read by value; every answer is in normal form.
- **The names**
  ([Q-M1f.180](../open_questions.md#q-m1f180--the-names-of-stdrat), defaulted, the user's to revise): the requests
  `rat-simplify`, `rat-neg`, `rat-add`, `rat-sub`, `rat-mul`, `rat-div` and `rat-cmp`, answered by
  `rat-simple-of`, `rat-neg-of`, `rat-sum-of`, `rat-diff-of`, `rat-product-of`, `rat-quot-of` and `rat-cmp-of`;
  the entry `rat-arith`. A quotient is one rational, exact, with no remainder.
- **The operands**
  ([Q-M1f.181](../open_questions.md#q-m1f181--what-stdrat-reads-as-an-operand), defaulted): a rational written
  out, or a name that `std.arith`'s `let` states. `rat-operand` says a rational stands for itself;
  `rat-operand-check` starts `std.arith`'s comparison walk of the denominator against `z`, under a tag that
  holds the operand; the order `gt` hands the operand on as `(rat-operand-is TAG P N D)`, and `eq` derives
  `(rat-zero-den TAG)`. One rule, `rat-operand-second`, asks for the second operand of every request.
- **A zero denominator is `std.arith`'s division by zero**
  ([Q-M1f.182](../open_questions.md#q-m1f182--a-zero-denominator-is-stdariths-division-by-zero), defaulted):
  three rules, one for each place an operand can stand, derive `(division-by-zero REQUEST)` from `rat-zero-den`,
  the request read out of the tag, as written; `rat-div-zero` derives it for a divisor that is zero, whose
  numerator simplifies to `(int z z)`; and `std.arith`'s `division-by-zero-is-false` derives `(false)`. No rule
  of this module concludes `(false)`.
- **The cross products,** `(rat-cross TAG P1 N1 D1 P2 N2 D2)`: seven rules in a chain, four products and two
  sums of natural numbers, handed over as `(rat-cross-is TAG POSITIVE NEGATIVE)`. A sum's numerator is that
  pair; a difference's is the same with the second numerator's parts exchanged; **a comparison is a
  difference's numerator and nothing more**, its positive part against its negative part by `std.arith`'s walk
  under the request's own tag. So the three share the seven rules, and a comparison finds no common divisor and
  divides nothing.
- **A product** is `std.int`'s `int-mul-call` on the numerators, which hands over an integer in normal form,
  `std.arith`'s `mul-call` on the denominators, and lowest terms without a second simplification. **A quotient**
  simplifies the divisor's numerator first, since its size becomes a denominator: positive, the divisor P over E
  turns into E over P; negative, of size N, into E negated over N; zero, the request divides by zero.
- **Lowest terms,** `(rat-norm TAG P N D)` and `(rat-reduce TAG INTEGER D)`: the numerator simplified by
  `std.int`'s `int-norm`; its sign taken off and kept in the tag as `pos` or `neg`; **Euclid's chain**,
  `rat-gcd-step`, `rat-gcd-next` and `rat-gcd-end`, each step one `div-call` of `std.arith`, the remainder going
  on; then two more divisions, the numerator's size and the denominator each by what the chain found, and the
  sign put back by `rat-lowest-pos` or `rat-lowest-neg`. Zero falls out of it: gcd(0, D) is D, and 0 over D is 0
  over 1.
- **It needs no division of integers.** Every division is of two natural numbers, and the sign travels beside
  them. The stage's instruction was to stop that part if it did; it did not arise.
- **Every rule has one premise,** and a call's tag is its request, as in `std.int`
  ([Q-M1f.177](../open_questions.md#q-m1f177--a-calls-tag-is-its-request-and-what-an-extension-writes-of-stdariths)).
  No `absent`, `not`, `or` or guard.
- **The closing rule,** `rat-arith`, closes the 48 relations, names the 56 rules and names `int-arith` and
  `arith`. The module writes its closing rule's name as a fact, `(rat-rule rat-arith)` (E-11).

**The gcd chain is the program `le/p/gcd.ein` of `design.md` rewritten with tags,** as the stage file asked:
there a step joined a request with its answer on the two numbers; here the divisor travels in the tag
`(rat-gcd-rem TAG DIVISOR)` and `rat-gcd-next` has one premise, `(div-done (rat-gcd-rem ?k ?b) ?q ?r)`.

## T2 record, 2026-11-12: a zero denominator on the surfaces

The four programs that claim `(false)`, each with a sum beside the request that kills it. Every command exits 0.
`ein solve tests/stdlib/rat/12_zero_denominator_second.ein`:

```text
  states (k_s)      0
  empty             every node is dead — root's own fixpoint derives (false)
  universe          no candidate: R is {∅} or empty, decided by root alone

  unsat core        (cl eq 0 0 eq)
  unsat core        (cl eq 4 0 gt)
  unsat core        (let nothing (rat (int (n z 1) z) (n (n z 0) 0)))
  unsat core        (rat-mul (rat (int (n z 3) z) (n z 4)) nothing)
  refuted by        division-by-zero-is-false

  (solve :expect)   holds
```

- **The core is the request and what its operands' denominators rest on:** the request as written; the `let` of
  a named operand; and the rows of `cl` a denominator was compared with zero through, one a digit, of the bad
  operand and of the good one before it, since the second is looked up only when the first is found. Where both
  denominators are written `z` or have no digit to compare, the core is the request alone
  (`11_zero_denominator_first.ein`, `13_zero_denominator_one_operand.ein`).
- **`refuted by` names `std.arith`'s rule,** and `ein saturate --dump` holds the flag by the module's rule,
  `(division-by-zero (rat-mul …) :rule rat-zero-second)`, and `(false :rule division-by-zero-is-false)`, with
  the sum beside it answered in root's state.
- **A divisor that is zero** (`10_div_by_zero_rational.ein`, 3/4 divided by 0 over 5): the core is the request and
  two rows of `cl`, the two denominators compared, and the flag is `rat-div-zero`'s.

**What holds it.** `arith_search_free.rs`'s `division_by_zero`, which now reads a request, the tables a zero
rests on and the rules that flag from three lists, `DIVIDES`, `RESTS_ON` and `FLAGGERS`: no candidate, k_s 0,
root dead, one saturation, the core exactly one request and otherwise `let`s and table rows, `refuted by` the one
rule, no state file, and `ein test` holding the claim. `rat_theory.rs`'s
`a_zero_denominator_is_flagged_and_false`: 68 requests with a zero denominator, written `z`, with digits or
under a name, in the first place, the second and both, under each of the seven requests, and four divisions by
a rational that is zero written four ways, each flagged as written and never answered; the requests beside them
answered and not flagged; and a rational that is zero an operand like any other of a sum and a product.

## T3 record, 2026-11-12: the tests

- **Thirteen programs** under [`tests/stdlib/rat/`](../../../tests/stdlib/rat/), each a corpus entry of group
  `stdlib` with its `:expect`, nine with `(solve :expect (region (model)))` and four with `(solve :expect
  (false))`. Every claim was computed by Python's fractions when the programs were drafted
  (`build/s1f.6.8/draft_tests.py`), and `ein test tests/stdlib/rat` holds 28 of 28. `stdlib_coverage.rs`: 307
  of 307 over the stdlib (`cat stdlib/*.ein | grep -c '^(rule '`, 2026-11-12).
- **The harness against exact fractions,**
  [`ein-infer/tests/rat_theory.rs`](../../../ein.rs/crates/ein-infer/tests/rat_theory.rs)
  ([Q-M1f.184](../open_questions.md#q-m1f184--the-harness-of-stdrat-and-the-sizes-memory-allows), defaulted).
  The oracle is a pair of `i128` reduced by Euclid's algorithm in Rust. It passed on its first run.
  - **The exhausting set:** the 28 fractions with a numerator from −3 to 3 over a denominator from 1 to 4,
    every pair under each of the five operations, 784 requests an operation (672 for a division, whose divisor
    is not zero), exactly one answer a request, in lowest terms. 250 requests a saturation.
  - **Longer:** terms of one to four digits in the combinations of signs, three pairs of twelve-digit terms, and
    the neighbours of the Fibonacci numbers to 987, on which Euclid's chain is longest.
  - **By value:** 150 pairs spelt with a common factor in both terms, a common amount in both parts of the
    numerator and high zeros in the denominator.
  - Simplification and negation, every numerator from −12 to 12 over every denominator to 12 and terms to ten
    digits; requests by name; an integer and a natural number in an operand's place, unanswered and not
    flagged; a zero denominator (§ T2).
  - On the text: every rule has exactly one premise; no `absent`, `not`, `or`, guard or `(false)`; the four rules
    that derive the flag, by name; the closing rule's lists, with `int-arith` and `arith`.
- **The circuits,** pinned by `the_circuit_of_each_operation` (`cargo test -p ein-infer --test rat_theory
  the_circuit`, 2026-11-12):

  | request | firings | depth |
  |---|---:|---:|
  | 2/4 in lowest terms | 334 | 332 |
  | 1071/462 in lowest terms | 867 | 862 |
  | −3/4 negated | 435 | 433 |
  | 1/2 + 1/3 | 492 | 490 |
  | 1/2 − 1/3 | 417 | 415 |
  | 2/3 × 3/4 | 459 | 457 |
  | 1/2 ÷ −3/4 | 494 | 490 |
  | 1/2 against 1/3 | 76 | 76 |
  | 123/456 + 789/1011 | 2 640 | 2 633 |
  | 123/456 against 789/1011 | 270 | 270 |

  **Lowest terms is most of every request but a comparison**
  ([Q-M1f.183](../open_questions.md#q-m1f183--lowest-terms-by-stdariths-division-and-what-it-costs),
  defaulted): `std.arith`'s division computes its divisor's nine multiples before its first digit, some 70
  firings for a divisor of one digit, and a reduction is a division a step of the chain and two after it.
- **The batch,** `arith_batch.rs`'s `a_batch_of_rational_requests_costs_in_proportion_to_its_count`, two
  fractions with a numerator from 1 to 9 of either sign over a denominator from 2 to 9 (`cargo test -p ein-infer
  --features counters --test arith_batch a_batch_of_rational -- --nocapture`, 2026-11-12):

  | operation | requests | firings a request | candidates a request |
  |---|---:|---:|---:|
  | `rat-add` | 100 / 400 / 1 200 | 651.42 / 630.71 / 635.28 | 7 510.67 / 7 175.38 / 7 221.83 |
  | `rat-sub` | 100 / 400 / 1 200 | 648.94 / 638.48 / 636.49 | 7 469.57 / 7 279.65 / 7 237.41 |
  | `rat-mul` | 100 / 400 / 1 200 | 597.87 / 599.12 / 594.44 | 6 957.39 / 6 916.27 / 6 821.76 |
  | `rat-div` | 100 / 400 / 1 200 | 609.10 / 595.42 / 596.71 | 6 942.28 / 6 703.77 / 6 727.01 |
  | `rat-cmp` | 100 / 400 / 1 200 | 77.72 / 77.97 / 78.00 | 640.72 / 639.93 / 640.74 |

  The batches are smaller than the other modules', and their operands one digit: with terms of two digits a
  request is some 1 450 firings and 1 600 sums are 2.4 million facts (the stage's first run of the test). The
  tolerance is 15 % where the others take 5 %, since the length of the chain varies from request to request.
- **The search-free test,** `arith_search_free.rs`: `tests/stdlib/rat` joins its roots, `(import std.rat ` its
  theories and the seven results its result relations; the four dead programs are held as § T2 says.
- **The mutation sweep** kills every mutant of the module, 44 of 44, on its first run: 361 of 411 where 317 of
  367, the bank's 50 survivors unmoved (`python3 utils/stdlib_mutants.py --check`, 2026-11-12).

## T4 record, 2026-11-12: the example, the pages, and what moved

- **The example,** [`examples/arith/rat.ein`](../../../examples/arith/rat.ein): the stage's 1/2 + 1/3 = 5/6, a
  difference, a quotient, 2/4 in lowest terms, and a recipe scaled, 3/4 × 6/4 = 9/8, compared with a cup by two
  rules of the program's own. Seven claims.
- **On the release binary** (`python3 build/s1f.6.8/batch.py`, 2026-11-12; two fractions with a numerator from 1
  to 9 of either sign over a denominator from 2 to 9; the saturator's own milliseconds, best of three): 1 200 sums
  in 1.35 s, differences 1.37 s, products 1.25 s, quotients 1.25 s, and comparisons 134 ms; 640, 637, 600, 600
  and 78 firings a request. A sum of such fractions is about a millisecond, and a comparison a tenth of one.
- **A program that imports `std.rat`** pays 27.0 ms of load where `std.int`'s pays 22.4 and `std.arith`'s 19.9
  (`ein saturate examples/arith/rat.ein`, `int.ein` and `add.ein`, the `kb load:` line, best of five,
  2026-11-12). S1f.6.7's record and page said 24.0 for `std.int`, a single run; the page now says 22.4.
- **The sweep.** One engine and two stdlibs, the release binary and stdlib of `f1fe6918` against the tree's,
  `ein saturate --dump` and `ein solve` over the 729 `.ein` files `f1fe6918` tracks: no answer moved; one file's
  error line prints the stdlib's directory, which the sweep sets (`python3 build/s1f.6.8/sweep.py`, 2026-11-12).
  No engine source moved, and `stdlib/arith.ein`, `digits.ein`, `num9.ein` and `int.ein` are byte for byte what
  they were.
- **The goldens that move,** named before `EIN_BLESS=1`: `ein-cli/tests/golden/corpus_exits.txt`, 75 new cells, the
  fifteen new files'; `ein-render/tests/golden/corpus_shapes.md5`, 630 new lines, their 42 renderings each, and
  no other line (`git diff --numstat`, 2026-11-12).
  `alive_invariant.rs`'s list gains the fifteen files' `z`, `pos` and `neg`: the two signs are names only the
  module's rules write.
- **The register** is unmoved: 305 fixed, 16 open and 1 not wrong (`grep '^status' corpus/wrong_answers.toml |
  sort | uniq -c`, 2026-11-12). No finding for `findings.md`.

Pages: `stdlib/README.md` (§ `std.rat`, new), `tests/README.md` (§ `stdlib/rat/`, new), `07_stdlib_api.md`
(§ `std.rat`, new), `examples/README.md`, `corpus/corpus.toml`, `AGENTS.md`, the questions Q-M1f.180 to .184,
the phase README and this record.

**The gate** is green (`./run_tests.sh`, `build/gate-s1f.6.8.log`, 2026-11-12). `python3 utils/stdlib_census.py
--check`, `python3 utils/nl_census.py --check` and `python3 utils/stdlib_mutants.py --check` pass on the tree.

## Acceptance re-taken, 2026-11-12

- **Every answer is in normal form and agrees with exact fractions:** `rat_theory.rs`, through `hold`.
- **Search-free, held by the same test:** `arith_search_free.rs` over `tests/stdlib/rat/` and
  `examples/arith/rat.ein`; the four programs with a zero denominator are held to an empty R decided by root
  alone.
- **The natural-number and integer theories are unchanged:** `git diff f1fe6918 -- stdlib/arith.ein
  stdlib/digits.ein stdlib/num9.ein stdlib/int.ein` is empty.
- **Where overflow limits the extension, it says so with the command that shows it:** nothing overflows; what
  limits a run is the size of the state, which is [S1f.6.9](s1f.6.9_width.md)'s to state, and Q-M1f.183 and
  Q-M1f.184 say what a reduction costs.
