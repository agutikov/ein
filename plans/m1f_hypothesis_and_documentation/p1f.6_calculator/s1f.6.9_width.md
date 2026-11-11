# S1f.6.9 — What is left of the width: how long a number may be

**Phase:** [P1f.6](README.md)
**Status:** planned 2026-11-08; **restated 2026-11-11**: the representation question it was to put to the user is ruled
([the rulings](design.md#the-rulings-2026-11-11)), and most of the stage dissolved with it.
**Depends on:** [S1f.6.6](s1f.6.6_division.md). It may run beside S1f.6.7 and S1f.6.8.

## What dissolved

The user, 2026-11-08: *"with overflow detection or width extension if it is possible, I'm not sure now but maybe
relation could have variadic arguments then can write numbers in reverse order e.g. (num 8 7 0 3 2 1) ==
123078"*.

- **The representation** is ruled: nested, `(n PREFIX DIGIT)`, any length. No variadic relation is needed, so
  no grammar change is asked for.
- **Width extension** has nothing to extend: a number is as long as its digits.
- **Overflow** exists in `std.num9` only, and its detection and its test are
  [S1f.6.13](s1f.6.13_num9.md)'s.

## What is left

1. **The nesting limit.** The parser nests to 1 000, so a request can hold a number of 999 digits, while a rule
   can derive a longer one, and a `--dump-answer` state file holding such a fact does not load back (E-4 of
   [`findings.md`](findings.md); 999 nines + 1 on the prototype). The stage re-takes it on the shipped theory
   and puts the choice to the user:
   - **(a)** say it on the page and leave it: a number past 999 digits is answered and cannot be reloaded;
   - **(b)** the theory flags an answer longer than a stated bound, as `std.num9` flags overflow;
   - **(c)** an engine change, so that what a rule can derive the parser can read.
   The stage's recommendation is (a) until a program needs more.
2. **The size of a step fact.** Each step of a circuit holds its operands whole, so the text of a state grows
   with the square of the length: the 1 000-digit sum's state file was 32 MB. The stage measures facts, bytes
   and time by length for the four operations (10, 100 and 999 digits) and states where a run stops being
   practical.
3. **Words.** Four-digit words were measured in round 3 and lost their gain to the conversions on a single
   operation. They are not built. The stage states the one case that would reopen them, a multiplication or a
   division that stays in words throughout, with its measurement if S1f.6.10 asks for long numbers.

## Tasks

1. The limit re-taken, the three options shown with their probes, and the user's ruling applied.
2. The cost by length, as a table with its command.
3. The pages: the limit and the cost in the module's header and in `stdlib/README.md`.

## Acceptance

- The limit and the cost by length are stated with the commands that re-take them.
- E-4 is ruled or owned.
