# S1f.6.22 — Integer division, under three conventions

**Phase:** [P1f.6](README.md)
**Status:** planned 2026-11-12, on the user's replies of that day. Not started. **It is the last task of
[S1f.6.7](s1f.6.7_negative_numbers.md),** which was built but for division and waited for this ruling; it has
a file of its own because S1f.6.7's file is that stage's record, and because the ruling asks for more than
the one request that record prepared.
**Depends on:** [S1f.6.7](s1f.6.7_negative_numbers.md) (`std.int`, shipped) and
[S1f.6.6](s1f.6.6_division.md) (`std.arith`'s tagged `div-call`). Independent of the stdlib layout; written
against today's names, which [S1f.6.31](s1f.6.31_the_stdlib_layout.md) moves later.

## The rulings

[Question 1](questions.md#q1), *which remainder does a division of integers answer*, with (d) *more than one,
each under a request of its own (`int-div-trunc`, `int-div-floor`, `int-div-euclid`), about 27 rules*. The
user, 2026-11-12: *"(d)"*.

[Question 2](questions.md#q2), *under the Euclidean convention, is the remainder a natural number or an
integer*, with (a) *a natural number*. The user, 2026-11-12: *"(a)"*.

They answer [Q-M1f.179](../open_questions.md#q-m1f179--the-remainder-of-an-integer-division).

## What it builds

Three requests in `std.int`, each answered by one fact with a quotient and a remainder, X = Q·Y + R:

| request | convention | Q | R |
|---|---|---|---|
| `(int-div-euclid X Y)` | 0 ≤ R < \|Y\| | an integer | **a natural number** of `std.arith`, `z` or `(n PREFIX DIGIT)` |
| `(int-div-trunc X Y)` | Q rounded toward zero, R takes the dividend's sign | an integer | an integer |
| `(int-div-floor X Y)` | Q rounded down, R takes the divisor's sign | an integer | an integer |

The three differ only where an operand is negative (`questions.md` 1):

| X ÷ Y | Euclidean Q, R | truncated Q, R | floored Q, R |
|---|---|---|---|
| −7 ÷ 2 | −4, 1 | −3, −1 | −4, 1 |
| 7 ÷ −2 | −3, 1 | −3, 1 | −4, −1 |
| −7 ÷ −2 | 4, 1 | 3, −1 | 3, −1 |

All three were written as rules and agree with Python on 924 requests (`python3
build/s1f.6.7/intdiv_proto.py`, 2026-11-12); −7 ÷ 2 is 124 firings under Euclidean and floored and 112 under
truncated (`questions.md` 1). The rules, their shared part and their case tables are in
[S1f.6.7 § T2 record](s1f.6.7_negative_numbers.md#t2-record-2026-11-12-the-remainder-convention-three-ways-for-the-user).
The prototype answers the Euclidean remainder as an integer; the ruling makes it a natural number, so its two
answer rules hand over the size and not `(int R z)`.

**The names of the results** are not ruled and are taken here by default, the user's to revise:
`int-quot-euclid-of`, `int-quot-trunc-of` and `int-quot-floor-of`, each `(… X Y Q R)` with the operands
repeated as written, after `std.arith`'s `quot-of`. There is **no bare `int-div`**: a request that does not
say its convention would be a fourth name for one of the three.

**Division by zero** is as everywhere in the calculator: a divisor that is zero by value derives
`(division-by-zero REQUEST)`, the request as written, and `std.arith`'s rule derives `(false)`. Which fact
carries the flag once [S1f.6.24](s1f.6.24_division_by_zero_at_the_call.md) has moved it to the call is that
stage's; this one writes the flag as the shipped modules do today.

## Tasks

1. **T1f.6.22.1 — the rules into `stdlib/int.ein`,** from the prototype, under the rule of the theory: every
   rule one premise, a call's tag its request, no rule that joins two facts on a number, no `absent`, `not`
   or guard. What the three share is written once (the operands, both simplified, the sizes handed to
   `div-call`); the count of rules is recorded with its command.
2. **T1f.6.22.2 — the closing rule's two lists** (`int-arith` closes every new relation and names every new
   rule); `arith_theory.rs`'s sibling test fails on one forgotten.
3. **T1f.6.22.3 — the harness.** Three rows of `int_theory.rs`'s `OPS`, each with a second result argument,
   against `i128`'s `div_euclid`/`rem_euclid`, `/`/`%`, and the floored pair computed from them; asked every
   case the other rows are (every pair of one-digit parts, every pair of lengths, operands by name,
   unsimplified operands answering as their normal forms do). The Euclidean row checks that R is a natural
   number in normal form.
4. **T1f.6.22.4 — the programs** under `tests/stdlib/int/`, one per rule or tight family, each with its
   `:expect`: every combination of signs under each convention with and without a remainder, and a division
   by zero under each claiming `(solve :expect (false))`. `stdlib_coverage.rs` then holds every new rule
   activated; `arith_search_free.rs`'s lists (`DIVIDES`, `RESTS_ON`, `FLAGGERS`) take the module's new
   requests.
5. **T1f.6.22.5 — the mutation census:** `python3 utils/stdlib_mutants.py`, every mutant of the new rules
   killed or the survivor named in `tests/mutation_survivors.txt` with its reason; `--check` green.
6. **T1f.6.22.6 — the batch,** by hand (S1f.6.21 took the counters step out of the gate): a row of
   `arith_batch.rs` per request, candidates a request flat across its batch sizes.
7. **T1f.6.22.7 — the example and the pages:** `examples/arith/int.ein` gains the three divisions of one pair
   of operands side by side; [`stdlib/README.md`](../../../stdlib/README.md)'s `std.int` section, the
   module's header, `AGENTS.md`'s S1f.6.7 paragraph (*integer division is not built* goes), S1f.6.7's status
   line, Q-M1f.179 marked ruled.

## Acceptance

- Each of the three requests answers one fact a request, in normal form, and agrees with the harness's
  integers on every case of T3.
- The Euclidean remainder is a natural number; the other two are integers.
- Search-free, held by the same test as the rest of the module; a division by zero is an empty R decided by
  root.
- `std.arith` and `std.rat` are unchanged, and no existing program's `ein solve` moves (the sweep of
  S1f.6.7's record, old stdlib against new).
- Every new rule is activated by a program under `tests/stdlib/int/`, and the mutation census has no
  unexplained survivor.

## What it must not do

- **Not add a division of integers that does not name its convention.**
- **Not promote a natural number to an integer** in an operand's place: [reply 12](questions.md#q12) keeps
  each module to its own shape until M1g's types theory.
- **Not touch `std.rat`,** which needs no division of integers and waits on round 2's item 54.

## The standing rules

A wrong answer of the engine met on the way is a row in `corpus/wrong_answers.toml`, filed `open` before its
fix. Findings go to [`findings.md`](findings.md) under the next free id; a question to
[`open_questions.md`](../open_questions.md) under the next free id (the result names above are one). Config
never changes an answer; every answer exits 0. The gate is `./run_tests.sh`, then `./run_tests.sh --nightly`
after the code commit; `utils/ci_container.sh` is not needed (no DOT writer, review bundle or CI install
changes).
