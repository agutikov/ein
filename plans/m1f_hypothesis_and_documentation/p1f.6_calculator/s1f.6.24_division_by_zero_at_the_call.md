# S1f.6.24 — Division by zero is flagged at the call, whoever made it

**Phase:** [P1f.6](README.md)
**Status:** planned 2026-11-12, on the user's reply of that day and round 2's reading of it. Not started.
**Depends on:** [S1f.6.6](s1f.6.6_division.md) (the flag as built), [S1f.6.8](s1f.6.8_rational_numbers.md)
(`std.rat`'s own rule). It edits `stdlib/arith.ein`, so it runs after
[S1f.6.12's build](s1f.6.12_malformed_numbers.md#the-build-as-ruled-2026-11-12). Independent of the stdlib
layout.

## The ruling

[Question 48](questions.md#q48) asked the user to confirm two defaults: that only the public request is
flagged, `(division-by-zero (div X Y))`, so that *a theory that calls division internally with a zero divisor
gets nothing and writes a rule of its own*; and that `std.rat` uses the same flag for any zero denominator.
The user, 2026-11-12:

> I don't understand the question: (div ?x 0) must emit division-by-zero no matter where it happens (except
> theories where it is allowed - like wheels, meadows, transreal arithmetic, etc...).

[Round 2, item 48](questions_round2.md#q48) reads it as two decisions and takes both meanwhile:

1. **A division called from inside another theory,** `(div-call TAG X z)`, derives nothing today; by the
   reply this is wrong. *The flag moves to the division itself, so any call with a zero divisor derives
   `(division-by-zero CALL)` and `(false)`, whoever made it.*
2. **A rational written over zero,** used in a product or a comparison, is flagged as built, *and stays*. A
   `let` that names such a rational and that no request uses is not flagged.

And: *a theory in which division by zero is allowed (a wheel, a meadow) would be a module of its own that
does not import the flag's rule.* That is recorded and not built.

## What it changes

Today (S1f.6.6): `div-by-zero` derives `(division-by-zero (div X Y))`, the request as written, for a divisor
that is zero by value, and `division-by-zero-is-false` derives `(false)` from the flag, the one rule of the
theory that concludes it. `(div-call TAG X Y)` with a zero divisor derives nothing, and each theory above
writes a rule of its own; `std.rat` does (Q-M1f.163, Q-M1f.182).

After: the detection is at `div-call`. Every division, a program's `(div X Y)` and every theory's tagged
call, passes through it, so one rule flags all of them.

**What the flag names is the stage's design question,** shown on real programs before anything ships:

- **(i) the call,** `(division-by-zero (div-call TAG X Y))`, as round 2's sentence has it. One rule. The tag
  names the caller, so the flag of a public request reads `(division-by-zero (div-call (…the request's tag…) X
  Y))` where it read `(division-by-zero (div X Y))`: every surface S1f.6.6's T2 recorded moves (the unsat
  core, `refuted by`, `--trace`, `ein saturate --dump`), and the programs that claim on the flag.
- **(ii) the call, and the public request as today.** The public flag keeps its shape, derived from the
  call's; two flags for one public division.
- **(iii) the caller's request, through the tag.** Each theory states which request a tag belongs to;
  `std.int` and `std.rat` already use the request as the tag, so there the call's flag *is* about the
  request.

**Default: (ii),** because it moves no existing answer's text: a program that divides by zero in public
reads as it does today, and a theory's internal division by zero is flagged where it was silent. The record
shows the three on the same three programs, and the user may pick another.

**`std.rat`'s own rule, reconsidered.** `std.rat` derives the flag itself for a request with a
zero-denominator operand and for a `rat-div` by a rational that is zero. With the flag at the call, the task
reads which of those rules a division at the call would now also flag, and which never reach a division at
all (a product or a comparison with 3/0 performs none). The second kind stays as built, by item 2. The first
kind is removed only if the flag a program sees is the same fact as before; otherwise it stays and the
record says why two rules flag one request.

## Tasks

1. **T1f.6.24.1 — the three designs on real programs,** on a copy of the stdlib under `build/s1f.6.24/` and
   the release binary: a public `(div X z)`; a rule of a program's own that calls `(div-call (mine) X z)`;
   a `std.int` division by zero (after S1f.6.22, or the prototype's if that stage has not run); a `rat-div`
   by zero; a product with 3/0. For each design and program: the flag facts, the core, `refuted by`, the
   trace's steps. The table is the record's first section. A zero written with digits, `(n z 0)`, is zero by
   value and is among the cases.
2. **T1f.6.24.2 — the witness of today's silence,** as a test program before the change: a tagged call with
   a zero divisor that derives nothing and exits 0. **It is not a register row by default:** the register
   holds the engine's wrong answers, and here the engine answers the program as the stdlib's rules say; what
   is wrong is a rule of a theory. If the user wants a theory's wrong answers in the register, that is a
   question under the next free id, and the row is filed `open` before the fix.
3. **T1f.6.24.3 — the rule moved** in `stdlib/arith.ein`, the closing rule's lists, and `std.rat`'s rules as
   T1 decided.
4. **T1f.6.24.4 — the tests.** `arith_theory.rs`: a tagged call with a zero divisor derives the flag and
   `(false)`, under every tag shape the modules use. `arith_search_free.rs`: its three lists, `DIVIDES`,
   `RESTS_ON` and `FLAGGERS`, restated to where the flag now comes from. `rat_theory.rs` and `int_theory.rs`:
   every division by zero still one flag a request, or two under (ii), pinned. A program per changed rule
   under `tests/stdlib/`; the mutation census.
5. **T1f.6.24.5 — the callers swept.** Every program that writes `div-call` (`std.int`, `std.rat`,
   `examples/binary_arithmetic/conv.ein`, the eight of `examples/beyond_arith/` that compute through the
   tagged calls, the word problems): none divides by zero today unless it claims `(false)`; the sweep, old
   stdlib against new over every tracked `.ein` file, shows it. Every golden expected to move is named
   before blessing.
6. **T1f.6.24.6 — the pages:** the module's header and [`stdlib/README.md`](../../../stdlib/README.md)
   (*a division by zero is flagged wherever it is called*; a theory that allows it is a module that does not
   import the rule); S1f.6.6's record gets a pointer, not a rewrite; `AGENTS.md`; Q-M1f.163 marked overruled
   and Q-M1f.182 confirmed, with the user's words.

## Acceptance

- `(div-call TAG X Y)` with a divisor that is zero by value derives a `division-by-zero` flag and `(false)`,
  whoever asserted the call.
- A rational over zero in a request is flagged as before; a `let` nobody asks about is not.
- A public division by zero reads as it did on every surface, or the user chose a design under which it does
  not and the moved goldens were named first.
- No program that does not divide by zero moves.

## What it must not do

- **Not build a wheel, a meadow or transreal arithmetic.** The reply names them as theories where the flag
  does not apply; none is planned.
- **Not change what `(false)` from the flag means:** root is dead, R is empty, every command exits 0.
- **Not touch lowest terms in `std.rat`,** which waits on round 2's item 54.

## The standing rules

A wrong answer of the engine met on the way is a row in `corpus/wrong_answers.toml`, filed `open` before its
fix. Findings go to [`findings.md`](findings.md) under the next free id; questions to
[`open_questions.md`](../open_questions.md) under the next free id (which fact carries the flag is one, with
its default). Config never changes an answer. The gate is `./run_tests.sh`, then `./run_tests.sh --nightly`
after the code commit; `utils/ci_container.sh` is not needed.
