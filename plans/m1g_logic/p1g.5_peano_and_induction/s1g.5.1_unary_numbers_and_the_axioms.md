# S1g.5.1 — Unary numbers and the axioms as rules: `z`, `(s N)`, addition, multiplication and order by recursion, on request

**Phase:** [P1g.5](README.md)
**Status:** planned 2026-11-11, not started.
**Depends on:** [P1g.1](../p1g.1_survey_and_design/README.md)'s rulings. It needs nothing of P1g.2 to P1g.4
and may run early.

## What it asks

*"Natural numbers, Peano"* (the user, 2026-11-11). What each of Peano's axioms **is** when written in Ein,
and whether the arithmetic they define computes by saturation alone.

## The axioms, one by one

The coordinator's reading, each **to be tried** with its program:

| axiom | in Ein | expected |
|---|---|---|
| 0 is a number | a name, `z`; `(nat z)` if the theory wants a predicate | a fact |
| the successor of a number is a number | `(s N)`, a nested fact. As a forward rule, `(nat ?n)` gives `(nat (s ?n))`, it enumerates the numbers and **does not stop** (probe 3: 8 388 505 facts when a 20 s timeout ended `ein saturate`). So `nat` is checked on request, walking a given term down to `z`, never generated | a rule, demand-driven |
| 0 is not a successor | `(neq z (s ?n))`: true of every value, because `z` is a name and `(s N)` a nested fact, and `neq` is non-identity of values | a property of nested facts; nothing to write. To be confirmed that `eq` and `neq` compare nested facts structurally in a premise and a conclusion |
| the successor is injective | from `(eq (s ?m) (s ?n))`, `(eq ?m ?n)`: again identity of values | the same |
| x + 0 = x; x + S(y) = S(x + y) | two rules on a request, below | rules |
| x · 0 = 0; x · S(y) = (x · y) + x | two rules, calling addition | rules |
| induction | not a rule over numbers: [S1g.5.3](s1g.5.3_induction.md) | a schema |

The third and fourth rows are the observation to test and state carefully: the term algebra of nested facts
**is** a model of the first four axioms, so they are not assumptions of an Ein theory but facts about Ein's
values. What that leaves out is the standard model's other half, *nothing else is a number*, which is
exactly what induction says and what `(nat …)` checked on request approximates for the terms a program
writes: `(s Ann)` is a value and not a number, and the `nat` check derives nothing for it. A malformed number
is the calculator's S1f.6.12 again, and its ruling applies.

## The recursion

On the calculator's protocol: a request, an answer that repeats the request. A sketch, to be replaced by
programs whose answers are computed. It is probe 6's program; on the release binary at `8860d616`
(2026-11-11) it answered `(plus-of (s (s z)) (s (s (s z))) (s (s (s (s (s z))))))`, with R {∅}.

```lisp
;; sketch: addition by recursion on the second argument
(relation s Nat               :why "the successor of {?1}")
(relation plus Nat Nat        :why "{?1} + {?2} is asked for")
(relation plus-of Nat Nat Nat :why "{?1} + {?2} = {?3}")

(rule plus-zero ()                         ; x + 0 = x
  :match  (plus ?x z)
  :assert (plus-of ?x z ?x)
  :why    "{?x} + 0 = {?x}")
(rule plus-ask ()                          ; x + S(y) needs x + y
  :match  (plus ?x (s ?y))
  :assert (plus ?x ?y)
  :why    "{?x} + S({?y}) needs {?x} + {?y}")
(rule plus-succ ()                         ; x + S(y) = S(x + y)
  :match  (and (plus ?x (s ?y)) (plus-of ?x ?y ?r))
  :assert (plus-of ?x (s ?y) (s ?r))
  :why    "{?x} + {?y} = {?r}, so {?x} + S({?y}) = S({?r})")

(plus (s (s z)) (s (s (s z))))             ; 2 + 3
(solve :hypothesis-relations ())
(query :goal (plus-of (s (s z)) (s (s (s z))) ?r)
       :expect (model (plus-of (s (s z)) (s (s (s z))) (s (s (s (s (s z))))))))
```

To be settled:

- **`plus-succ` joins two facts on numbers,** the request and the smaller answer, which the calculator's
  *rule of the theory* forbids for a measured reason (E-3). Here both of the second premise's first two
  slots are bound and only the answer is free, so it is the scanning case. Built both ways, the join above
  and with a continuation, and measured on batches, as [S1g.2.1](../p1g.2_propositional_logic/s1g.2.1_formulas_and_evaluation.md)
  does for evaluation. The join is the textbook's equation almost verbatim, and that readability is worth
  stating a price for.
- **An accumulator form** (`x + S(y) = S(x) + y`) is tail-recursive, needs no join, and is not the Peano
  equation. Both ship if both are instructive; the page proves nothing about their equivalence and checks it
  up to the bound.
- **Multiplication** calls addition: a continuation, or a join on the sub-request's answer. **Order,**
  `(leq X Y)`, by walking both down together, answering one of three names as the calculator's `cmp` does.
  **Predecessor and truncated subtraction** come free from the constructor.
- **Closure:** the module closes its relations, so H is empty.
- **Functions defined by a program,** the general shape: primitive recursion as a pattern any program can
  follow (factorial, exponent, Fibonacci as examples), and where it ends (Ackermann's function is recursive
  and not primitive recursive: it nests requests, and is to be tried as the test of whether the request
  protocol is general).

## Tasks

1. **T1g.5.1.1 — the axioms table,** a program per row; in particular `eq` and `neq` on nested facts, in a
   premise and in a conclusion, each with what the binary does.
2. **T1g.5.1.2 — the theory:** `nat` on request, addition, multiplication, order; a program per rule; where
   it ships as ruled ([Q-M1g.2](../open_questions.md#q-m1g2--where-the-logic-ships)).
3. **T1g.5.1.3 — the checks:** against Rust's integers on every pair up to a bound (a row of the harness);
   the search-free test; each request answered exactly once.
4. **T1g.5.1.4 — the two shapes measured:** join against continuation, candidates per request, and firings
   against the size of the numbers (addition linear in the second argument, multiplication in the product,
   expected).
5. **T1g.5.1.5 — the examples:** primitive recursion's pattern, and Ackermann tried under a bound.

## Acceptance

- Each axiom has its program and its one-line statement of what it is in Ein, confirmed by the run.
- Addition, multiplication and order are computed by saturation alone, held by the search-free test, and
  agree with Rust on every pair up to the stated bound.
- The cost of each shape is on the page as a count, with its command.
- No rule in the module generates numbers; a test holds that the module's programs saturate, each under a
  bound.

## What it may find

- **That `eq` on nested facts has a corner** (an `eq` guard that *binds* a variable to a nested fact, S1f.5.7's
  domain being the program's names). Stated on the page; a wrong answer is a register row.
- **That the nesting limit is closer than expected** in derived facts, which may nest deeper than a program
  can write (E-4): a product over a thousand is derived and its state file does not reload. S1f.6.9's ruling
  applies, and the bound of every test stays under it.
