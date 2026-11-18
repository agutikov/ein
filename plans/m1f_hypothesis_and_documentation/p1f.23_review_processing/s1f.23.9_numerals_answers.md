# S1f.23.9 — the user's answers on the numerals' check, built

**Phase:** [P1f.23](README.md)
**Status:** started 2026-11-18 on the user's word (*"process plans/m1f_hypothesis_and_documentation/review/02-numerals/ANSWERS.md"*),
in a worktree beside S1f.23.6 and S1f.23.8; merged after them, and one gate, `utils/ci_container.sh` and one nightly
hold the three. **A stage that changes a rule of the calculator runs the batch by hand** (`AGENTS.md`).
**The answers:** the user's file as written, `93f30a91`
(`git show 93f30a91:plans/m1f_hypothesis_and_documentation/review/02-numerals/ANSWERS.md`), and the removed items'
pages beside it in that commit (`…/02-numerals/NN-…/README.md`), with their runs' programs; the topic left the bundle
(`d21d05ec`).

## The answers

| item | the question | the user's answer |
|---|---|---|
| Q-M1f.405 | which numerals are *taken from the input program*, and *once* | *"(a) the difference is between numerals produced by operation from valid numerals, and all other numerals from any source"* |
| Q-M1f.409 | an integer and a rational: their check and their label | (a); and, asked how L-101's rule reads a rational whose part is a natural numeral, **"Verify the natural"**: a rational built from a natural numeral checks it as a numeral, as an operation checks its operands, so `(rat (n z x) (n z 2))` is dead again (the option (d) for that case) |
| Q-M1f.406, .407, .410, .411 | the label; an untaken numeral; no join on a number; two faults one flag | (a) each |
| Q-M1f.408 | where the check's rules live | **(b)**: *the walk in `std.math.positional`, the representation keeping the flags* |
| L-101 | `int-check-parts` / `rat-check-parts` ask non-numerals | *"integer built from invalid decimal is invalid, rational built from invalid integers is invalid, if you want to build rational from something else it has to provide verification, the same way as addition, multiplication, comparison etc."* |

## What it builds

1. **The rule of Q-M1f.405**: a numeral an operation produces from valid numerals is valid as made, and **every other
   numeral, from any source, is checked**: one the program writes in a request, one it hands straight to a tagged
   call (Q-M1f.405's case (e), dead again), one a rule of the program makes up. What an operation produces is not
   walked again. Within the calculator's laws: no rule joins two facts on a number (`AGENTS.md`); where the rule
   cannot be had without one, the stage says so and stops for the user.
2. **Q-M1f.408 (b)**: the numeral's walk moves into `std.math.positional`, its relations parameters as the module's
   are, each base connecting it by its macro's call; `std.math.decimal` and `std.math.binary` keep their flags
   (`malformed`, `not-decimal`, …). The closing rules' lists, the macro's body and parameters, every base's call.
3. **L-101 and the natural part**: validity composes: an integer is valid where its parts are valid decimals, a
   rational where its parts are valid integers; a rational whose part is a natural numeral checks the numeral, as an
   operation checks its operands; the check is asked only of the shapes it is for (a rule a shape), so no fact like
   `(check 7 7)` is left that no rule reads.
4. **The ledger**: each answer filed as ruled with the user's words (`open_questions.md`, Q-M1f.405 to .411; the
   milestone's `findings.md`, L-101); the decisions that state the check (Q-M1f.404 and those the build amends)
   brought to the rulings; the pages (`04-stdlib/` calculator pages, regenerated rule regions and drawings,
   `stdlib/README.md`).

## Acceptance

- Every corpus program's answer as before, but those the rulings move, each named with today's answer and the new
  one: Q-M1f.405's case (e) and Q-M1f.409's case (d) dead again, and any other the census finds; their `:expect`
  brought to the ruling.
- `cargo test -p ein-infer --features counters --test arith_batch`, by hand; `arith_theory.rs`, `int_theory.rs`,
  `rat_theory.rs` and the rest of the calculator's tests; `tests/stdlib/math/` (`ein test`); `stdlib_coverage.rs`.
- The gate, `utils/ci_container.sh` and the nightly, once, with the phase's other stages, on the merged tree.

## Record

Not started.
