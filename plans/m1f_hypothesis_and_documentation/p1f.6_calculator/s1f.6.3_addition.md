# S1f.6.3 — Addition, and the machinery every circuit shares

**Phase:** [P1f.6](README.md)
**Status:** planned 2026-11-08; re-planned 2026-11-11 to [the rulings](design.md#the-rulings-2026-11-11).
**Depends on:** [S1f.6.2](s1f.6.2_digits_and_tables.md).

## What it builds

`stdlib/arith.ein`, imported as `std.arith`, begins here: the number, the protocol, the machinery, and the first
operation.

**The number** (the user: *"reading order"*). A natural number is `z`, zero, or `(n PREFIX DIGIT)`: ten times
the prefix, plus the digit. 123078 is `(n (n (n (n (n (n z 1) 2) 3) 0) 7) 8)`. The normal form has no high
zero. Any length; no width and no overflow.

**The protocol** (the user: *"ok, result facts like (sum-of X Y S) can be reused for other purposes, the goal
then has to be (sum-of x y ?s) with a variable on the result place"*). A request fact, `(add X Y)`, is answered
by a result fact that repeats its operands, `(sum-of X Y S)`. A program reads the answer with a goal that has a
variable in the result's place.

**The machinery,** design K of
[design.md § Round 3](design.md#k-an-accumulator-in-the-other-order-and-a-continuation-on-every-call):
- **One walk from the units,** each column through the table, its digit put in front of an accumulator. The
  accumulator holds the answer most significant digit outermost, a second shape, `(b DIGIT REST)`, that only the
  theory writes.
- **High zeros are dropped there,** where they are outermost.
- **Then the answer is turned round** into the public shape and handed over as `(done TAG ANSWER)`.
- **Every call carries its continuation** as a tag, a nested fact holding what the caller needs next.

**The rule of the theory: no rule joins two facts on a number.** A rule has one premise that holds numbers; its
other premises are table rows looked up by digits. This is what makes a batch of requests cost in proportion to
their count, where an ordinary join on a nested value scans its relation (E-3 of [`findings.md`](findings.md)).
It is stated in the module's header and held by a test (task 4).

**Addition:** `(add X Y)` asks for the walk with the tag `(sum X Y)`, and `(done (sum X Y) S)` is
`(sum-of X Y S)`. Operands with high zeros are read by value; the answer is in normal form.

## Tasks

1. **The module's start:** `n`, `z`, the accumulator shape, the strip, the turn, the walk for addition, the
   import of `std.digits`, and one closing rule that closes every relation of the theory and names every rule,
   so `(import std.arith :symbols (arith))` brings the theory.
2. **Named constants** (the user: *"(let NAME NUMBER) is ok"*). `(let price N)` names a number, and a request
   may name its operand: `(add price qty)` is answered by `(sum-of price qty S)`. The stage's default for what
   that means is in [the rulings § what is open](design.md#what-is-open-after-the-rulings), item 2, and it is the
   user's to overrule before this task is written.
3. **The tests.**
   - One program per rule under `tests/stdlib/arith/`, each with its `:expect`.
   - A harness test against the integers: random operands of every length up to a stated bound, operands with
     high zeros, and every answer in normal form.
   - **The search-free test,** which every later operation joins: H empty, k_s 1, R {∅}, and the same answer
     under `ein solve`, `ein saturate` and `ein test`.
4. **The test that holds the rule of the theory.** Two parts.
   - Static: every rule of `std.arith` has at most one premise whose relation is not a table of `std.digits`.
   - Measured: a batch of 250 and of 1 000 requests, and a deterministic count of the matcher's work per request
     (a counter of `ein_core::counters`, which the stage picks and names, not the wall clock) that does not grow
     with the batch.
5. **The circuit's measure:** firings and depth of one addition, with the command that re-takes them.
6. **The example:** `examples/arith/` gains the addition program of `design.md`, respelt, with its corpus entry.

## Acceptance

- 123078 + 4567 is 127645, and 999999999 + 1 has an answer of ten digits.
- The harness test, the search-free test and the test of the rule of the theory pass.
- No `absent`, no `not`, one stratum.
