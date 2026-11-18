# Nine digits in one fact: `std.math.num9`

A second representation of a number: `(num9 d8 d7 d6 d5 d4 d3 d2 d1 d0)`, nine digit names, the most significant
first, a fixed width
([Q-M1f.320](../../../../decisions/stdlib/math/num9/Q-M1f.320-std-math-num9-is-a-second-representation.md)):

$$
\operatorname{val}\bigl((\mathsf{num9}\ d_8 \dots d_0)\bigr) = \sum_{i=0}^{8} d_i \cdot 10^i
$$

123078 is `(num9 0 0 0 1 2 3 0 7 8)`. The module's names are decimal's with a 9
([Q-M1f.167](../../../../decisions/stdlib/math/num9/Q-M1f.167-the-names-of-std-math-num9-are-decimals-with-a-9.md)),
so it stands beside `std.math.decimal-arith` in one program. Its file is generated and not edited by hand. What it
shares with the rest of the calculator is [the calculator's page](07-calculator.md); its rules are
[the next page](17-nine-digit-numbers-rules.md).

## Using it

```lisp
(import std.math.num9 :symbols (arith9 num9 add9 sum9-of mul9 product9-of overflow))
```

The entry is `arith9`. A program that writes only `num9` numbers lists nothing from `std.math.decimal`.

| a program writes | the theory answers | meaning |
|---|---|---|
| `(add9 X Y)` | `(sum9-of X Y S)` | $X + Y = S$, where S fits nine digits |
| `(sub9 X Y)` | `(diff9-of X Y D R)` | two numbers, as `diff-of`, one of them zero |
| `(cmp9 X Y)` | `(cmp9-of X Y O)` | O is `lt`, `eq` or `gt` |
| `(mul9 X Y)` | `(product9-of X Y P)` | $X \cdot Y = P$, where P fits nine digits |
| `(div9 X Y)` | `(quot9-of X Y Q R)` | $X = Q \cdot Y + R$ with $0 \le R < Y$ |
| `(to9 X)` | `(as9 X W)`, or the flag `(big X)` | W is X, a number of `std.math.decimal-arith`, as nine digits; `big` where X is written with more than nine |
| `(from9 W)` | `(of9 W X)` | X is W as a number of `std.math.decimal-arith`, in normal form |

```lisp
(import std.math.num9 :symbols (arith9 num9 add9 sum9-of mul9 product9-of overflow))

(add9 (num9 0 0 0 1 2 3 0 7 8) (num9 0 0 0 0 0 4 5 6 7))    ; 123078 + 4567
(mul9 (num9 0 0 0 1 0 0 0 0 0) (num9 0 0 0 1 0 0 0 0 0))    ; 100000 × 100000

(query :goal (sum9-of ?x ?y ?s))
(query :goal (product9-of ?x ?y ?p))
(query :goal (overflow ?r))
```

<!-- output -->
```text
  query 1 of 3      (sum9-of ?x ?y ?s)
  over              all of R — 1 state → 1 projection class
    class 1 of 1    1 state (1)
                      ?s = (num9 0 0 0 1 2 7 6 4 5) · ?x = (num9 0 0 0 1 2 3 0 7 8) · ?y = (num9 0 0 0 0 0 4 5 6 7)
  as :expect        (model (sum9-of (num9 0 0 0 1 2 3 0 7 8) (num9 0 0 0 0 0 4 5 6 7) (num9 0 0 0 1 2 7 6 4 5)))

  query 2 of 3      (product9-of ?x ?y ?p)
  over              all of R — 1 state → 1 projection class
    class 1 of 1    1 state (1) — the goal has no row: undetermined here
  as :expect        (model)

  query 3 of 3      (overflow ?r)
  over              all of R — 1 state → 1 projection class
    class 1 of 1    1 state (1)
                      ?r = (mul9 (num9 0 0 0 1 0 0 0 0 0) (num9 0 0 0 1 0 0 0 0 0))
  as :expect        (model (overflow (mul9 (num9 0 0 0 1 0 0 0 0 0) (num9 0 0 0 1 0 0 0 0 0))))
```

The sum is 127645. The product does not fit, so `(overflow (mul9 …))` is derived and no `product9-of`. The other
operations:

```lisp
(import std.math.num9 :symbols (num9 sub9 diff9-of cmp9 cmp9-of mul9 product9-of div9 quot9-of))

(sub9 (num9 0 0 0 0 0 0 1 0 0) (num9 0 0 0 0 0 0 0 0 1))    ; 100 − 1
(sub9 (num9 0 0 0 0 0 0 0 0 1) (num9 0 0 0 0 0 0 1 0 0))    ; 1 − 100
(cmp9 (num9 0 0 0 0 0 0 1 0 0) (num9 0 0 0 0 0 0 0 9 9))    ; 100 against 99
(mul9 (num9 0 0 0 0 0 0 0 1 2) (num9 0 0 0 0 0 0 0 1 2))    ; 12 × 12
(div9 (num9 0 0 0 0 0 0 1 0 0) (num9 0 0 0 0 0 0 0 0 7))    ; 100 ÷ 7

(query :goal (diff9-of ?x ?y ?d ?r))
(query :goal (cmp9-of ?x ?y ?o))
(query :goal (product9-of ?x ?y ?p))
(query :goal (quot9-of ?x ?y ?q ?r))
```

<!-- output -->
```text
  query 1 of 4      (diff9-of ?x ?y ?d ?r)
  over              all of R — 1 state → 1 projection class
    class 1 of 1    1 state (1)
                      ?d = (num9 0 0 0 0 0 0 0 0 0) · ?r = (num9 0 0 0 0 0 0 0 9 9) · ?x = (num9 0 0 0 0 0 0 0 0 1) · ?y = (num9 0 0 0 0 0 0 1 0 0)
                      ?d = (num9 0 0 0 0 0 0 0 9 9) · ?r = (num9 0 0 0 0 0 0 0 0 0) · ?x = (num9 0 0 0 0 0 0 1 0 0) · ?y = (num9 0 0 0 0 0 0 0 0 1)
  as :expect        (model (diff9-of (num9 0 0 0 0 0 0 0 0 1) (num9 0 0 0 0 0 0 1 0 0) (num9 0 0 0 0 0 0 0 0 0) (num9 0 0 0 0 0 0 0 9 9))
                           (diff9-of (num9 0 0 0 0 0 0 1 0 0) (num9 0 0 0 0 0 0 0 0 1) (num9 0 0 0 0 0 0 0 9 9) (num9 0 0 0 0 0 0 0 0 0)))

  query 2 of 4      (cmp9-of ?x ?y ?o)
  over              all of R — 1 state → 1 projection class
    class 1 of 1    1 state (1)
                      ?o = gt · ?x = (num9 0 0 0 0 0 0 1 0 0) · ?y = (num9 0 0 0 0 0 0 0 9 9)
  as :expect        (model (cmp9-of (num9 0 0 0 0 0 0 1 0 0) (num9 0 0 0 0 0 0 0 9 9) gt))

  query 3 of 4      (product9-of ?x ?y ?p)
  over              all of R — 1 state → 1 projection class
    class 1 of 1    1 state (1)
                      ?p = (num9 0 0 0 0 0 0 1 4 4) · ?x = (num9 0 0 0 0 0 0 0 1 2) · ?y = (num9 0 0 0 0 0 0 0 1 2)
  as :expect        (model (product9-of (num9 0 0 0 0 0 0 0 1 2) (num9 0 0 0 0 0 0 0 1 2) (num9 0 0 0 0 0 0 1 4 4)))

  query 4 of 4      (quot9-of ?x ?y ?q ?r)
  over              all of R — 1 state → 1 projection class
    class 1 of 1    1 state (1)
                      ?q = (num9 0 0 0 0 0 0 0 1 4) · ?r = (num9 0 0 0 0 0 0 0 0 2) · ?x = (num9 0 0 0 0 0 0 1 0 0) · ?y = (num9 0 0 0 0 0 0 0 0 7)
  as :expect        (model (quot9-of (num9 0 0 0 0 0 0 1 0 0) (num9 0 0 0 0 0 0 0 0 7) (num9 0 0 0 0 0 0 0 1 4) (num9 0 0 0 0 0 0 0 0 2)))
```

100 − 1 is 99 and nothing short, and 1 − 100 is nothing with 99 short; 100 is greater than 99; 12 × 12 is 144;
and 100 ÷ 7 is 14, remainder 2.

### Overflow is a fact and no number

A sum or a product that does not fit derives `(overflow REQUEST)` and no result, never a truncated number
([Q-M1f.168](../../../../decisions/stdlib/math/num9/Q-M1f.168-overflow-is-a-fact-and-no-number-and-a-division.md)).
Root stays alive, the other requests are answered, and a rule may match the flag:

```lisp
(import std.math.num9 :symbols (num9 add9 sum9-of overflow))

(relation meter Number  :why "the meter reads {?3}")
(relation rolled-over   :why "the meter rolled over")

(meter (num9 9 9 9 9 9 9 9 9 9))

(rule one-more ()
  :match  (meter ?m)
  :assert (add9 ?m (num9 0 0 0 0 0 0 0 0 1))
  :why    "one more is counted on {?m}")

(rule too-many ()
  :match  (overflow (add9 ?m ?one))
  :assert (rolled-over)
  :why    "{?m} and one more do not fit nine digits")

(solve :no-search)
(query :goal (rolled-over))
(query :goal (sum9-of ?x ?y ?s))
```

<!-- output -->
```text
  query 1 of 2      (rolled-over)
  over              all of R — 1 state → 1 projection class
    class 1 of 1    1 state (1)
                      the goal holds: it has no variable
  as :expect        (model (rolled-over))

  query 2 of 2      (sum9-of ?x ?y ?s)
  over              all of R — 1 state → 1 projection class
    class 1 of 1    1 state (1) — the goal has no row: undetermined here
  as :expect        (model)
```

### Conversions

Nothing converts by itself. A conversion is a request, and no rule of either module hands a request to the other:
a program that wants both writes the rules that ask
([`examples/arith/num9_both_ways.ein`](../../../../../examples/arith/num9_both_ways.ein) adds two numbers by each
module and compares the sums).

```lisp
(import std.math.num9 :symbols (num9 to9 as9 from9 of9 big))
(import std.math.decimal :symbols (n))
(import std.math.arith :symbols (z))

(to9 (n (n z 4) 2))                                           ; 42
(to9 (n (n (n (n (n (n (n (n (n (n z 1) 2) 3) 4) 5) 6) 7) 8) 9) 0))   ; ten digits
(from9 (num9 0 0 0 0 0 0 0 4 2))

(query :goal (as9 ?x ?w))
(query :goal (big ?x))
(query :goal (of9 ?w ?x))
```

<!-- output -->
```text
  query 1 of 3      (as9 ?x ?w)
  over              all of R — 1 state → 1 projection class
    class 1 of 1    1 state (1)
                      ?w = (num9 0 0 0 0 0 0 0 4 2) · ?x = (n (n z 4) 2)
  as :expect        (model (as9 (n (n z 4) 2) (num9 0 0 0 0 0 0 0 4 2)))

  query 2 of 3      (big ?x)
  over              all of R — 1 state → 1 projection class
    class 1 of 1    1 state (1)
                      ?x = (n (n (n (n (n (n (n (n (n (n z 1) 2) 3) 4) 5) 6) 7) 8) 9) 0)
  as :expect        (model (big (n (n (n (n (n (n (n (n (n (n z 1) 2) 3) 4) 5) 6) 7) 8) 9) 0)))

  query 3 of 3      (of9 ?w ?x)
  over              all of R — 1 state → 1 projection class
    class 1 of 1    1 state (1)
                      ?w = (num9 0 0 0 0 0 0 0 4 2) · ?x = (n (n z 4) 2)
  as :expect        (model (of9 (num9 0 0 0 0 0 0 0 4 2) (n (n z 4) 2)))
```

A number written with ten digits has no number of nine: `big` is a fact a rule may read, and root stays alive.

### What is flagged, refused, and silent

**A division by zero** derives `(division-by-zero (div9 X Y))`, and `(false)` from it:

```lisp
(import std.math.num9 :symbols (num9 div9))

(div9 (num9 0 0 0 0 0 0 0 0 7) (num9 0 0 0 0 0 0 0 0 0))
```

<!-- output -->
```text
  unsat core        (div9 (num9 0 0 0 0 0 0 0 0 7) (num9 0 0 0 0 0 0 0 0 0))
  refuted by        std.math.arith.division-by-zero-is-false

  (false)
```

**An operand is a number written out**
([Q-M1f.170](../../../../decisions/stdlib/math/num9/Q-M1f.170-a-num9-operand-is-a-number-written-out.md)): the module
has no named constants. A `num9` with another count of digits is refused at load:

```lisp
(import std.math.num9 :symbols (num9 add9))

(add9 (num9 0 0 0 0 0 0 0 7) (num9 0 0 0 0 0 0 0 0 1))
```

<!-- output -->
```text
kb load error: n9-eight.ein:3:8: the fact (add9 (num9 0 0 0 0 0 0 0 7) (num9 0 0 0 0 0 0 0 0 1)) holds (num9 0 0 0 0 0 0 0 7), which gives num9 8 arguments, and (relation num9 Digit Digit Digit Digit Digit Digit Digit Digit Digit) declares 9 — a form headed by a declared relation has as many arguments as its declaration has types
```

One with a digit's place holding something else is answered by silence, and the module flags no malformed digit:

```lisp
(import std.math.num9 :symbols (num9 add9 sum9-of))

(add9 (num9 0 0 0 0 0 0 0 0 x) (num9 0 0 0 0 0 0 0 0 1))

(query :goal (sum9-of ?x ?y ?s))
```

<!-- output -->
```text
  ∅                 in R — root is alive and owes nothing
  …
  query             (sum9-of ?x ?y ?s)
  over              all of R — 1 state → 1 projection class
    class 1 of 1    1 state (1) — the goal has no row: undetermined here
  as :expect        (model)
```

### Which to use

`std.math.num9` where every number fits nine digits and there are many requests, since a sum is one firing where
`std.math.decimal-arith` walks a column a firing, and where a fixed width is what is modelled — a register, a
counter, a field of a form — with the overflow as a fact a rule reads. `std.math.decimal-arith` for numbers of any
length, for named constants, and for a program with a request or two. Both, in one program, with a conversion
request where a number crosses.

[`tests/stdlib/math/num9/`](../../../../../tests/stdlib/math/num9/) has a program for each request.

## How it works

The theory is the same circuit over the same tables, `std.math.digits`', wired the other way: a rule has a premise
a column, so a sum is one firing of one rule, and a product or a quotient a step a digit. Where
`std.math.decimal-arith` walks the recurrence

$$
\begin{aligned}
x_i + y_i + c_i &= 10 \cdot c_{i+1} + s_i \\
c_0 &= 0
\end{aligned}
$$

one column a firing, `add9-fits` states all nine columns in one `:match`, with the carries $c_1$ to $c_8$ as
variables shared between neighbouring rows of `fa`, and $c_9 = 0$; `add9-overflows` is the same rule with
$c_9 = 1$. Every rule is on [the next page](17-nine-digit-numbers-rules.md).
