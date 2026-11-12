# S1f.6.33 — `std.rat` reduces only when asked

**Phase:** [P1f.6](README.md)
**Status:** planned 2026-11-12, on the user's reply of that day in round 2. Not started.
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
are restated*; (c) both, by request name. The user's reply there, as it stood in the working copy when this
plan was written (2026-11-12, not yet committed; **the stage re-reads it before it starts**): *"(b)"*.

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
- **Not start before the reply is committed and re-read.**

## The standing rules

A wrong answer of the engine met on the way is a row in `corpus/wrong_answers.toml`, filed `open` before its
fix. Findings go to [`findings.md`](findings.md) under the next free id; questions to
[`open_questions.md`](../open_questions.md) under the next free id. A design change shows real computed
examples first (T1). Config never changes an answer. The gate is `./run_tests.sh`, then `./run_tests.sh
--nightly` after the code commit; `utils/ci_container.sh` is not needed.
