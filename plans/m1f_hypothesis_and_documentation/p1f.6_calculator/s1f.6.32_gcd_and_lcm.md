# S1f.6.32 — gcd and lcm, as `std.math.gcd`

**Phase:** [P1f.6](README.md)
**Status:** planned 2026-11-12, on the user's reply of that day. Not started.
**Depends on:** [S1f.6.31](s1f.6.31_the_stdlib_layout.md), the layout, since the module is born under
`std.math`; and [S1f.6.24](s1f.6.24_division_by_zero_at_the_call.md), since it calls division with a tag.

## The ruling

[Question 38](questions.md#q38): *Do gcd and lcm go into `std.arith`?*, with (a) *into `std.arith`, as
`(gcd X Y)` answered by `(gcd-of X Y G)`*. The user, 2026-11-12:

> (a) but not into std.arith; it's time to make a std.math with subfolders;

So they ship, in a module of their own: `std.math.gcd`, `stdlib/math/gcd.ein`, entry `gcd`
([round 2's layout](questions_round2.md#layout), confirmed).

## What it builds

- `(gcd X Y)` answered by `(gcd-of X Y G)` and `(lcm X Y)` by `(lcm-of X Y L)`, on natural numbers of
  `std.math.decimal-arith`, the operands repeated as written, an operand a number or a name a `let` states.
- **Euclid's chain** over the calculator's tagged division, a step a `div-call`, each step's tag carrying
  what is left; the lcm as the product divided by the gcd. Six rules offered both on the prototype
  (`questions.md` 38; [`beyond_arithmetic.md`](beyond_arithmetic.md)'s question 5 and its banked example
  under `examples/beyond_arith/`).
- **Under the rule of the theory:** no rule joins two facts on a number; no `absent`, no `not`; search-free.
- **The edge the rules must say:** gcd with zero (gcd(X, 0) is X, gcd(0, 0) is zero by the usual
  convention, which the stage states and the harness holds), and lcm with zero, which is zero and must not
  divide by a zero gcd. No request of this module derives a division by zero; the test holds that.
- The entry's closing rule closes the module's relations, names its rules, and names `decimal-arith`, so one
  name brings the theory under it (E-20's remedy).

**`std.rat` is not touched by this stage.** It computes a greatest common divisor inside itself today.
Under option (b) of [round 2, item 54](questions_round2.md#q54), *reduce only on request*, `rat-simplify`
calls this module and the reason gcd lives inside `std.rat` is gone; the user replied *"(b)"* there
(as it stood in the working copy when this plan was written (2026-11-12, not yet committed)), and that change is
[S1f.6.33](s1f.6.33_lowest_terms_on_request.md), which runs after this stage. Here the two chains are
measured side by side, firings on the same operands, as that stage's starting figures.

## Tasks

1. **T1f.6.32.1 — the module,** from the banked example's rules, rewritten to the tagged calls the interface
   names ([S1f.6.30](s1f.6.30_name_clashes_between_imported_theories.md)'s T1); its header with the names it
   occupies; the closing rule's lists.
2. **T1f.6.32.2 — the harness,** `ein-infer/tests/gcd_theory.rs`, a sibling of `arith_theory.rs`: every pair
   to a bound and sampled long operands against Rust's integers; zero in each place; equal operands;
   operands by name; exactly one answer a request, in normal form; the rule of the theory on the text.
3. **T1f.6.32.3 — the programs** under `tests/stdlib/`, one per rule or tight family, each with its
   `:expect`; `stdlib_coverage.rs`; `arith_search_free.rs` takes the module in; the mutation census with no
   unexplained survivor; the batch by hand.
4. **T1f.6.32.4 — the example:** the banked gcd example under `examples/beyond_arith/` imports the module in
   place of its own rules, with the same claim; `examples/arith/` gains one that uses both requests.
5. **T1f.6.32.5 — the comparison with `std.rat`'s chain,** firings on the same operands, in the record.
6. **T1f.6.32.6 — the pages:** [`stdlib/README.md`](../../../stdlib/README.md), a section for the module;
   `AGENTS.md`; the manifest.

## Acceptance

- Both requests answer one fact a request and agree with the harness's integers, zero included.
- Search-free, held by the test; every rule activated by a program under `tests/stdlib/`.
- `std.rat`, `std.int` and the decimal theory are unchanged, and no existing program's answer moves.

## What it must not do

- **Not edit `std.rat`:** that is S1f.6.33's.
- **Not add inverse requests** (`solve-add`, `solve-mul`): [question 37](questions.md#q37) is not ruled in,
  and round 2 says they are *not* added to the stdlib now.
- **Not add gcd of integers or rationals.** Natural numbers only, unless the user asks.

## The standing rules

A wrong answer of the engine met on the way is a row in `corpus/wrong_answers.toml`, filed `open` before its
fix. Findings go to [`findings.md`](findings.md) under the next free id; questions to
[`open_questions.md`](../open_questions.md) under the next free id (gcd(0, 0) is one, with its default).
Config never changes an answer. The gate is `./run_tests.sh`, then `./run_tests.sh --nightly` after the code
commit; `utils/ci_container.sh` is not needed.
