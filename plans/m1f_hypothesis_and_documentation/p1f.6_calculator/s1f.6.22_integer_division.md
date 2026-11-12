# S1f.6.22 — Integer division, under three conventions

**Phase:** [P1f.6](README.md)
**Status:** **done 2026-11-12** ([§ Record](#record-2026-11-12)). Planned that day, on the user's replies. **It is the last task of
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

## Record, 2026-11-12

**T1, the rules.** `stdlib/int.ein` gained 34 rules, 70 beside its closing rule (`grep -c '^(rule '
stdlib/int.ein` says 71, 2026-11-12), 13 relations, and the two names `pos` and `neg`. Every rule has one
premise, none writes `absent`, `not` or a guard, and a call's tag is its request.

| | rules | which |
|---|---:|---|
| shared by the three | 8 | `int-div-start`, `int-div-divisor`, `int-div-by-zero`, `int-div-pp`, `-pn`, `-np`, `-nn`, `int-div-rest` |
| Euclidean | 10 | `int-div-euclid-ask`, `-pp`, `-pn`, `-np-exact`, `-nn-exact`, `-np-more`, `-nn-more`, `-neg`, `-pos`, `-answer` |
| truncated | 6 | `int-div-trunc-ask`, `-pp`, `-pn`, `-np`, `-nn`, `-answer` |
| floored | 10 | `int-div-floor-ask`, `-pp`, `-nn`, `-pn-exact`, `-np-exact`, `-pn-more`, `-np-more`, `-neg`, `-pos`, `-answer` |

The stage file guessed about 27. The prototype's shape is kept and two things differ from it:

- **The shared part is entered through one tag, `(int-div-of REQUEST)`.** A request's `-ask` rule looks its first
  operand up under `(int-first (int-div-of REQUEST) Y)`; the module's own `int-operand-second` asks for the
  second; and `int-div-start` matches `(operand-is (int-second (int-div-of ?k) ?p ?n) (int ?q ?m))`, so one rule
  starts the three, where the prototype's start rule named its request. From there the request travels as `?k`
  down to `std.arith`'s `div-call`, and the convention's own rules read it back out of the tag, `(div-done
  (int-div-m (int-div-euclid ?x ?y) pos pos ?b) ?q ?r)`.
- **The Euclidean remainder is a natural number**, the user's ruling: the six Euclidean rules that hand over
  an answer conclude `(int-div-done REQUEST Q R)` with R the size itself, `?r`, `z` or `?t`, and not `(int R z)`.
  `int-div-done`'s third argument is therefore a number under one convention and an integer under two; the three
  result relations each hold one shape.

`int-div-rest`, the adjustment's second walk, is shared by the two conventions that adjust; its sign argument is
the quotient's sign under Euclidean and the remainder's under floored, which the module's comments say.

**T2, the closing rule.** `int-arith` closes the 13 new relations and names the 34 new rules;
`the_closing_rule_names_every_rule_and_closes_every_relation` holds it.

**T3, the harness.** `int_theory.rs`'s `OPS` has seven rows. A division's row answers two arguments, a quotient
and a remainder, against `i128`: `div_euclid` and `rem_euclid`, the remainder spelt as a natural number in
normal form; `/` and `%`; and the floored pair computed from the truncated one. Each is asked every case the
other rows are, a divisor of zero left out by `hold`: the 10 000 pairs of integers written with one-digit parts
less the 1 000 whose divisor is zero, every pair of lengths to 30 digits in the four combinations of signs,
equal operands and operands one apart, unsimplified operands and operands with high zeros, and operands by
name. `a_division_by_zero_is_the_flag_under_every_convention` holds the flag, the request as written, `(false)`
and no quotient, for four spellings of zero by four dividends under each convention. The circuits, pinned in
`CIRCUITS`: −7 ÷ 2 is (124, 120), (112, 108) and (124, 120) firings and depth under Euclidean, truncated and
floored, and −127645 ÷ 25 is (279, 269), (259, 249) and (279, 269) (`cargo test -p ein-infer --test int_theory
the_circuit -- --nocapture`, 2026-11-12): the firings are the prototype's.

**T4, the programs.** Six under `tests/stdlib/int/`, each with a corpus entry: `09_div_euclid.ein`,
`10_div_trunc.ein` and `11_div_floor.ein` ask the same eighteen requests, the four combinations of signs with
a remainder and without, a dividend of zero, a dividend smaller than the divisor, six digits by two and two
requests with unsimplified operands, and claim their result relation's whole extent; `12_` to `14_` divide by a
zero written `(int z z)`, 3 less 3, and a name for 0 less 00, and claim `(solve :expect (false))`. Every answer
claimed was computed in Python apart from the engine. `stdlib_coverage.rs` holds every rule activated, 341 of
341 (`cat stdlib/*.ein | grep -c '^(rule '`, 2026-11-12). `arith_search_free.rs`: `DIVIDES` took `(int-div-`,
`FLAGGERS` the rule `int-div-by-zero`, `QUOTIENTS` and `RESULTS` the three result relations, and `RESTS_ON`
took `(nz `: the core of a division by a named zero written with digits is the request, the `let`, two rows of
`fs` and `(nz 7)`, the row the dividend's simplification read.

`ein-infer/tests/alive_invariant.rs`'s list moved: `pos` and `neg` are names only a rule's `:assert` writes, in a
tag, as `std.rat`'s two signs already were. Every program that imports `std.int` now holds them, 16 more
entries of the list carry the two lines, and the rules named are `int-div-pn` and `int-div-pp`, the first to
assert them, in `std.rat`'s programs too. H is empty on every one of them, as before.

**T5, the mutation census.** 379 of 429 killed (`python3 utils/stdlib_mutants.py`, 2026-11-12), where it was
361 of 411: the 18 mutants of the new rules are all killed, and the 50 survivors are the banked ones.

**T6, the batch, by hand.** Three rows of `REQUESTS_INT`, operands with four-digit parts, a divisor of zero
left out, and a bound of 5 500 candidates a request:

| request | candidates a request at 250 | at 1 000 | at 4 000 | firings a request at 250 |
|---|---:|---:|---:|---:|
| `int-div-euclid` | 4 606.01 | 4 617.77 | 4 619.46 | 270.62 |
| `int-div-trunc` | 4 557.12 | 4 568.15 | 4 568.32 | 261.95 |
| `int-div-floor` | 4 609.68 | 4 619.64 | 4 619.55 | 271.26 |

(`cargo test -p ein-infer --features counters --test arith_batch a_batch_of_integer -- --nocapture`,
2026-11-12; the whole file under the feature, 5 passed.) Flat, within the test's tolerance.

**T7, the example and the pages.** `examples/arith/int.ein` has −7 by 2 under the three conventions side by
side, with three queries. `stdlib/README.md`'s `std.int` section and its module table, the module's header,
`07_stdlib_api.md`'s table of requests and results, `tests/README.md`, `AGENTS.md`, the phase README, S1f.6.7's
status line, Q-M1f.179 as built.

**What moved.** No engine source. `std.arith` and `std.rat` are unchanged. The sweep, the release binary
reading the stdlib of `8464f522` against the tree's, `ein saturate --dump` and `ein solve` over the 804 `.ein`
files `8464f522` tracks (`python3 build/s1f.6.22/sweep.py`, 2026-11-12): **no existing program's `ein solve`
moved** but `examples/arith/int.ein`'s, which this stage edited. `ein saturate --dump` holds more on the 23
programs that import `std.int` or `std.rat`: the new relations closed and the new rules named, and nothing they
held is gone. Two files differ by a line that is not an answer, a timing in `beyond_arith/primes.ein` and the
stdlib's path in an import error. A program that imports `std.int` pays 24.4 ms of load where it paid 22.4,
and `std.rat`'s 29.8 where it paid 27.0 (`ein saturate examples/arith/int.ein` and `rat.ein`, the `kb load:`
line, best of five, 2026-11-12). `int_theory.rs` takes 79 s where the workspace run had it shorter, three more
operations under the same cases (`cargo test -p ein-infer --test int_theory`, 2026-11-12). The register is
unmoved, and no finding was met.

**Defaults taken:** [Q-M1f.200](../open_questions.md#q-m1f200--the-names-of-an-integer-divisions-results), the
names of the three result relations, as this file had them.

## Acceptance re-taken, 2026-11-12

- Each request answers one fact a request, in normal form, and agrees with `i128` on every case of T3: held by
  `int_theory.rs`'s `hold`, which asks for exactly one answer.
- The Euclidean remainder is a natural number and the other two are integers: the rows' answers, compared as
  text.
- Search-free, and a division by zero an empty R decided by root: `arith_search_free.rs` over the six programs
  and the example.
- `std.arith` and `std.rat` unchanged, no existing program's `ein solve` moved: the sweep above.
- Every new rule activated under `tests/stdlib/int/`, no unexplained survivor: T4 and T5.

**After the commit** (`f99732e1`): `./run_tests.sh --nightly` green in every step, run on the commit as
committed, in a detached worktree of it (`build/nightly-s1f.6.22.log`, 2026-11-12); its mutation census, `--check`,
379 of 429 with the bank unmoved. The gate before the commit: `build/gate-s1f.6.22.log`. `utils/ci_container.sh`
was not run: no DOT writer, review bundle or CI install changed (Q-M1f.77).

