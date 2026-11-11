# S1f.6.1 — The design proposal: representations, the request protocol, and target programs computed on a prototype

**Phase:** [P1f.6](README.md)
**Status:** **done 2026-11-11.** Three rounds on a prototype, [`design.md`](design.md), and the user's rulings of
that day, recorded in [§ The rulings](design.md#the-rulings-2026-11-11). The phase was re-planned to them the same
day. Planned 2026-11-08.
**Depends on:** S1f.5.5's swap, so its programs are written in the final grammar, with digits as names.

## What it delivers

A page, `p1f.6_calculator/design.md`, and a prototype under `build/s1f.6/prototype/` (gitignored). The page
shows the user **a couple of target programs**, each the way a program would import the theory and calculate,
with its answer computed on the prototype. Then it asks the questions below, each with a recommendation.
Nothing under `stdlib/`, `examples/` or `tests/` is written before the user rules.

**The target programs:**
- **One per operation** on the user's own numbers. 123078 + 4567, 123078 − 4567 and 4567 − 123078 (the second
  number non-zero), 123 × 456, and 127645 ÷ 25 (quotient 5105, remainder 20).
- **One operation that overflows**, and one division by zero.
- **One program that chains operations:** a division whose dividend is a product, the product computed by the
  same run.
- **For each target program, every artefact a run writes:**
  - `ein solve`'s read-out, showing H empty, k_s 1, R {∅} and no fork;
  - `ein saturate --dump`, with the firings and the rounds;
  - `ein test` on the program's `:expect`;
  - `--trace`.

The prototype may be narrow, a width of three or four, as long as each target program runs. Each operation it
cannot yet compute is named on the page as such. No answer on the page is written by hand.

A sketch of the shape, to be replaced by programs whose answers are computed:

```
;; sketch only: the names, the protocol and the import are S1f.6.1's to propose
(import std.arith)

(add (num9 0 0 0 1 2 3 0 7 8) (num9 0 0 0 0 0 4 5 6 7))
(div (num9 0 0 0 1 2 7 6 4 5) (num9 0 0 0 0 0 0 0 2 5))

(query :goal (sum-of ?x ?y ?s)
       :expect (model (sum-of (num9 0 0 0 1 2 3 0 7 8) (num9 0 0 0 0 0 4 5 6 7)
                              (num9 0 0 0 1 2 7 6 4 5))))
```

## The questions it puts to the user, each with the stage's recommendation

1. **The number's shape.**
   - **(a)** The user's fixed-width big-endian nested fact, `(num9 d8 … d0)`, one relation per width.
   - **(b)** A little-endian nested list, `(d 8 (d 7 (d 0 …)))`, of any width up to the nesting limit.
   - **(c)** A variadic relation. That is a grammar change, which S1f.6.9 takes up.
   - The default is (a), which the user wrote. S1f.6.9 comes back to (b) and (c).
2. **The protocol.** How a program asks and reads: a request fact, `(add X Y)`, answered by a result fact,
   `(sum-of X Y S)`, or something else. Also whether the result fact names its request.
3. **Subtraction's two numbers.**
   - **(a)** A pair `(d r)` where d = a − b when a ≥ b, otherwise 0, and r = b − a when b > a, otherwise 0.
     Exactly one of the two is zero unless a = b, and the pair is already the normal form of S1f.6.7's negative
     numbers.
   - **(b)** Hardware's convention: the difference modulo 10^width, and a borrow flag.
   - The default is (a): it is the user's *"simplified until one of them is zero"* computed directly.
4. **The width of a result.**
   - **(a)** Every operation's result has its operands' width, with a flag where it does not fit.
   - **(b)** Hardware's double-width registers: an addition gives a carry and a sum, and a multiplication a high
     and a low half. Division then takes a high and a low half for its dividend.
   - The default is (a) for the four operations. S1f.6.9 takes up width extension.
5. **Overflow and division by zero:** a fact (a flag) or `(false)`. `(false)` refutes root, so R would be empty
   and the answer would say no more than that. The default is a fact, `(overflow …)` or `(division-by-zero …)`,
   naming its request.
6. **How H stays empty.**
   - **(a)** The theory marks every relation it derives `(__closed__ R)`. A program may write a dunder there,
     the user's ruling of 2026-11-08 on the reserved names.
   - **(b)** Every program writes `(solve :no-hypothesis …)`.
   - **(c)** The arithmetic programs run only under `ein saturate` and `ein test`.
   - The default is (a): it keeps `ein solve` the one tool, and a program that imports the theory need not know
     it.
7. **The tables.**
   - **(a)** Generated once by a script under `utils/` with a `--check` in the gate, as
     `examples/gen_zebra2_variants.py` is.
   - **(b)** Written by hand.
   - Either way, a test holds every row to the harness's integers. The default is (a).
8. **Where the theory lives.**
   - **(a)** `stdlib/arith.ein`, imported as `std.arith`, with a program per rule under `tests/stdlib/arith/`.
   - **(b)** A module under `examples/arith/`, imported by a relative path.
   - The default is (a): the user's *"so the program in ein can import this"*.
9. **How a circuit is written** (measured, not asked). A ripple-carry adder is either one rule with a premise
   per column, or one rule per column over position names. The page gives both on the prototype, with firings
   and rounds, and recommends one.

## Acceptance

- The page, with every target program's answer computed and every artefact linked.
- The questions, each with its recommendation.
- **The search-free check on the prototype:** H empty, k_s 1 and R {∅} on every target program. If any program
  needs a hypothesis, that is the stop's first item.
- The commit is docs only. The prototype is under `build/`.
