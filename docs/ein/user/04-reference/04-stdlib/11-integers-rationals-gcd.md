# Integers, rationals, gcd: `std.math.int`, `std.math.rat`, `std.math.gcd`

Three theories over [the natural numbers](08-natural-numbers.md): integers as two natural numbers, rationals as an
integer over a natural number, and the greatest common divisor that reduces them. What they share with the rest of
the calculator, requests and results, flags, the tagged calls, is [the calculator's page](07-calculator.md). Their
rules are [the integers'](12-integers-rules.md) and [the rationals' and the divisor's](13-rationals-and-gcd-rules.md).

## Using `std.math.int` — integers

An integer is `(int P N)`, two natural numbers of `std.math.decimal-arith`, whose value is P less N
([Q-M1f.352](../../../../decisions/stdlib/math/int/Q-M1f.352-an-integer-is-two-parts-a-positive.md)):

$$
\operatorname{val}\bigl((\mathsf{int}\ P\ N)\bigr) = \operatorname{val}(P) - \operatorname{val}(N)
$$

−7 is `(int z (n z 7))`, zero `(int z z)`, and `(int (n z 5) (n z 3))` is 2. The **normal form** has one part `z`
and the other in normal form, and every integer has exactly one; an operand is read by value, and every answer is
in normal form, so a rule tells a negative answer by its shape, `(int z (n ?p ?a))`, with no negation written.
Every name is `int-`
([Q-M1f.174](../../../../decisions/stdlib/math/int/Q-M1f.174-the-names-of-std-math-int-carry-the-prefix-int.md)).

```lisp
(import std.math.int :symbols (int-arith int int-add int-sum-of))
(import std.math.decimal :symbols (n))
(import std.math.arith :symbols (z))
```

The entry is `int-arith`, which names `decimal-arith`, so one name brings the natural numbers too.

| a program writes | the theory answers | meaning |
|---|---|---|
| `(int-simplify X)` | `(int-simple-of X S)` | S is X's normal form |
| `(int-neg X)` | `(int-neg-of X S)` | $S = 0 - X$ |
| `(int-add X Y)` | `(int-sum-of X Y S)` | $X + Y = S$ |
| `(int-sub X Y)` | `(int-diff-of X Y S)` | $X - Y = S$, **one integer** |
| `(int-mul X Y)` | `(int-product-of X Y S)` | $X \cdot Y = S$, computed on the parts with no case on a sign ([Q-M1f.176](../../../../decisions/stdlib/math/int/Q-M1f.176-a-product-of-integers-is-computed-on-the-parts.md)) |
| `(int-cmp X Y)` | `(int-cmp-of X Y O)` | O is `lt`, `eq` or `gt`, by value |
| `(int-div-euclid X Y)` | `(int-quot-euclid-of X Y Q R)` | $X = Q \cdot Y + R$ with $0 \le R < \lvert Y \rvert$; Q an integer, R a **natural number** |
| `(int-div-trunc X Y)` | `(int-quot-trunc-of X Y Q R)` | Q rounded toward zero, R with the dividend's sign; both integers |
| `(int-div-floor X Y)` | `(int-quot-floor-of X Y Q R)` | Q rounded down, R with the divisor's sign; both integers |

```lisp
(import std.math.int :symbols (int-arith int int-add int-sum-of int-sub int-diff-of int-mul int-product-of
                               int-neg int-neg-of int-cmp int-cmp-of int-simplify int-simple-of))
(import std.math.decimal :symbols (n))
(import std.math.arith :symbols (z))

(int-add (int z (n (n z 1) 2)) (int (n (n z 4) 0) z))     ; −12 + 40
(int-sub (int (n z 3) z) (int (n z 8) z))                 ; 3 − 8
(int-mul (int z (n z 4)) (int z (n z 6)))                 ; −4 × −6
(int-neg (int (n z 5) z))                                 ; −(5)
(int-cmp (int z (n z 2)) (int (n z 1) z))                 ; −2 against 1
(int-simplify (int (n z 5) (n z 3)))                      ; 5 less 3

(query :goal (int-sum-of ?x ?y ?s))
(query :goal (int-diff-of ?x ?y ?s))
(query :goal (int-product-of ?x ?y ?s))
(query :goal (int-neg-of ?x ?s))
(query :goal (int-cmp-of ?x ?y ?o))
(query :goal (int-simple-of ?x ?s))
```

<!-- output -->
```text
  query 1 of 6      (int-sum-of ?x ?y ?s)
  over              all of R — 1 state → 1 projection class
    class 1 of 1    1 state (1)
                      ?s = (int (n (n z 2) 8) z) · ?x = (int z (n (n z 1) 2)) · ?y = (int (n (n z 4) 0) z)
  as :expect        (model (int-sum-of (int z (n (n z 1) 2)) (int (n (n z 4) 0) z) (int (n (n z 2) 8) z)))

  query 2 of 6      (int-diff-of ?x ?y ?s)
  over              all of R — 1 state → 1 projection class
    class 1 of 1    1 state (1)
                      ?s = (int z (n z 5)) · ?x = (int (n z 3) z) · ?y = (int (n z 8) z)
  as :expect        (model (int-diff-of (int (n z 3) z) (int (n z 8) z) (int z (n z 5))))

  query 3 of 6      (int-product-of ?x ?y ?s)
  over              all of R — 1 state → 1 projection class
    class 1 of 1    1 state (1)
                      ?s = (int (n (n z 2) 4) z) · ?x = (int z (n z 4)) · ?y = (int z (n z 6))
  as :expect        (model (int-product-of (int z (n z 4)) (int z (n z 6)) (int (n (n z 2) 4) z)))

  query 4 of 6      (int-neg-of ?x ?s)
  over              all of R — 1 state → 1 projection class
    class 1 of 1    1 state (1)
                      ?s = (int z (n z 5)) · ?x = (int (n z 5) z)
  as :expect        (model (int-neg-of (int (n z 5) z) (int z (n z 5))))

  query 5 of 6      (int-cmp-of ?x ?y ?o)
  over              all of R — 1 state → 1 projection class
    class 1 of 1    1 state (1)
                      ?o = lt · ?x = (int z (n z 2)) · ?y = (int (n z 1) z)
  as :expect        (model (int-cmp-of (int z (n z 2)) (int (n z 1) z) lt))

  query 6 of 6      (int-simple-of ?x ?s)
  over              all of R — 1 state → 1 projection class
    class 1 of 1    1 state (1)
                      ?s = (int (n z 2) z) · ?x = (int (n z 5) (n z 3))
  as :expect        (model (int-simple-of (int (n z 5) (n z 3)) (int (n z 2) z)))
```

−12 + 40 is 28; 3 − 8 is −5; −4 × −6 is 24; the negation of 5 is −5; −2 is less than 1; and 5 less 3 is 2 in
normal form.

### A division names its convention

There is no bare `int-div`
([Q-M1f.179](../../../../decisions/stdlib/math/int/Q-M1f.179-integer-division-names-its-convention.md),
[Q-M1f.200](../../../../decisions/stdlib/math/int/Q-M1f.200-an-integer-divisions-result-is-named-after.md)). The
three agree where neither operand is negative, and with `div` there:

| X ÷ Y | Euclidean Q, R | truncated Q, R | floored Q, R |
|---|---|---|---|
| 7 ÷ 2 | 3, 1 | 3, 1 | 3, 1 |
| −7 ÷ 2 | −4, 1 | −3, −1 | −4, 1 |
| 7 ÷ −2 | −3, 1 | −3, 1 | −4, −1 |
| −7 ÷ −2 | 4, 1 | 3, −1 | 3, −1 |

The second row, asked three ways:

```lisp
(import std.math.int :symbols (int int-div-euclid int-quot-euclid-of int-div-trunc int-quot-trunc-of
                               int-div-floor int-quot-floor-of))
(import std.math.decimal :symbols (n))
(import std.math.arith :symbols (z))

(int-div-euclid (int z (n z 7)) (int (n z 2) z))          ; −7 ÷ 2, three ways
(int-div-trunc  (int z (n z 7)) (int (n z 2) z))
(int-div-floor  (int z (n z 7)) (int (n z 2) z))

(query :goal (int-quot-euclid-of ?x ?y ?q ?r))
(query :goal (int-quot-trunc-of ?x ?y ?q ?r))
(query :goal (int-quot-floor-of ?x ?y ?q ?r))
```

<!-- output -->
```text
  query 1 of 3      (int-quot-euclid-of ?x ?y ?q ?r)
  over              all of R — 1 state → 1 projection class
    class 1 of 1    1 state (1)
                      ?q = (int z (n z 4)) · ?r = (n z 1) · ?x = (int z (n z 7)) · ?y = (int (n z 2) z)
  as :expect        (model (int-quot-euclid-of (int z (n z 7)) (int (n z 2) z) (int z (n z 4)) (n z 1)))

  query 2 of 3      (int-quot-trunc-of ?x ?y ?q ?r)
  over              all of R — 1 state → 1 projection class
    class 1 of 1    1 state (1)
                      ?q = (int z (n z 3)) · ?r = (int z (n z 1)) · ?x = (int z (n z 7)) · ?y = (int (n z 2) z)
  as :expect        (model (int-quot-trunc-of (int z (n z 7)) (int (n z 2) z) (int z (n z 3)) (int z (n z 1))))

  query 3 of 3      (int-quot-floor-of ?x ?y ?q ?r)
  over              all of R — 1 state → 1 projection class
    class 1 of 1    1 state (1)
                      ?q = (int z (n z 4)) · ?r = (int (n z 1) z) · ?x = (int z (n z 7)) · ?y = (int (n z 2) z)
  as :expect        (model (int-quot-floor-of (int z (n z 7)) (int (n z 2) z) (int z (n z 4)) (int (n z 1) z)))
```

The Euclidean remainder is a natural number, `(n z 1)`; the other two are integers.

### A negative answer is a shape

```lisp
(import std.math.int :symbols (int int-sub int-diff-of))
(import std.math.decimal :symbols (n))
(import std.math.arith :symbols (z))

(relation overdrawn Account :why "{?1} is overdrawn")
(relation balance Account Number Number :why "{?1} had {?2} and spent {?3}")

(balance Ann (int (n (n z 3) 0) z) (int (n (n z 4) 5) z))
(balance Bob (int (n (n z 3) 0) z) (int (n (n z 1) 0) z))

(rule ask-what-is-left ()
  :match  (balance ?who ?had ?spent)
  :assert (int-sub ?had ?spent)
  :why    "what {?who} has left: {?had} less {?spent}")

(rule below-zero ()
  :match  (and (balance ?who ?had ?spent) (int-diff-of ?had ?spent (int z (n ?p ?a))))
  :assert (overdrawn ?who)
  :why    "{?who} has less than nothing left")

(solve :no-search)
(query :goal (overdrawn ?who))
```

<!-- output -->
```text
  query             (overdrawn ?who)
  over              all of R — 1 state → 1 projection class
    class 1 of 1    1 state (1)
                      ?who = Ann
  as :expect        (model (overdrawn Ann))
```

Ann's 30 less 45 is `(int z (n (n z 1) 5))`, which the second rule's `(int z (n ?p ?a))` matches; Bob's 20 is
`(int (n (n z 2) 0) z)`, which it does not.

### Operands, and what is flagged

**An operand** is an integer written out, or a name a `(let NAME (int P N))` states
([Q-M1f.175](../../../../decisions/stdlib/math/int/Q-M1f.175-an-integer-operand-is-an-integer-written-out.md)):

```lisp
(import std.math.int :symbols (int int-add int-sum-of))
(import std.math.decimal-arith :symbols (let))
(import std.math.decimal :symbols (n))
(import std.math.arith :symbols (z))

(let debt (int z (n (n z 2) 5)))        ; −25
(int-add debt (int (n (n z 4) 0) z))    ; + 40

(query :goal (int-sum-of debt ?y ?s))
```

<!-- output -->
```text
  query             (int-sum-of debt ?y ?s)
  over              all of R — 1 state → 1 projection class
    class 1 of 1    1 state (1)
                      ?s = (int (n (n z 1) 5) z) · ?y = (int (n (n z 4) 0) z)
  as :expect        (model (int-sum-of debt (int (n (n z 4) 0) z) (int (n (n z 1) 5) z)))
```

A divisor that is zero by value derives `(division-by-zero REQUEST)`, however the zero is written:

```lisp
(import std.math.int :symbols (int int-div-floor))
(import std.math.decimal :symbols (n))
(import std.math.arith :symbols (z))

(int-div-floor (int (n z 7) z) (int (n z 3) (n z 3)))     ; 7 ÷ (3 less 3)
```

<!-- output -->
```text
  unsat core        (int-div-floor (int (n z 7) z) (int (n z 3) (n z 3)))
  unsat core        (std.math.digits.fs 3 3 0 0 0)
  unsat core        (std.math.digits.fs 7 0 0 7 0)
  unsat core        (std.math.digits.nz 7)
  refuted by        std.math.arith.division-by-zero-is-false

  (false)
```

A digit or a named constant where a part goes, `(int 7 z)`, is the integer's own fault,
`(not-integer INTEGER THING)`, flagged where it is written
([Q-M1f.276](../../../../decisions/stdlib/math/Q-M1f.276-an-integers-and-a-rationals-own-shape-is-checked.md)):

```lisp
(import std.math.int :symbols (int int-neg))
(import std.math.decimal :symbols (n))
(import std.math.arith :symbols (z))

(int-neg (int 7 z))
```

<!-- output -->
```text
  unsat core        (int-neg (int 7 z))
  unsat core        (std.math.digits.nz 7)
  refuted by        std.math.int.not-integer-is-false

  (false)
```

<!-- output -->
```text
  (std.math.int.not-integer (int 7 z) 7 :rule std.math.int.int-p-digit)
```

A name no `let` states is `(undeclared NAME REQUEST)`, as in the natural numbers; here `(int-add debt …)` with no
`let`:

<!-- output -->
```text
  unsat core        (int-add debt (int (n z 4) z))
  refuted by        std.math.positional.undeclared-is-false

  (false)
```

A natural number in an integer's place is answered by silence: root is alive and the request has no answer.

```lisp
(import std.math.int :symbols (int int-neg int-neg-of))
(import std.math.decimal :symbols (n))
(import std.math.arith :symbols (z))

(int-neg (n z 7))

(query :goal (int-neg-of ?x ?s))
```

<!-- output -->
```text
  ∅                 in R — root is alive and owes nothing
  …
  query             (int-neg-of ?x ?s)
  over              all of R — 1 state → 1 projection class
    class 1 of 1    1 state (1) — the goal has no row: undetermined here
  as :expect        (model)
```

### The tagged calls

The calls a later theory makes, each part a natural number written out. A call's tag is its request
([Q-M1f.177](../../../../decisions/stdlib/math/Q-M1f.177-a-module-over-the-decimal-arithmetic-tags-each.md)).

| a rule concludes | asked | the theory answers |
|---|---|---|
| `(int-norm TAG P N)` | P less N, in normal form | `(int-done TAG INTEGER)` |
| `(int-add-call TAG P1 N1 P2 N2)` | P1 less N1, plus P2 less N2 | `(int-done TAG INTEGER)` |
| `(int-mul-call TAG P1 N1 P2 N2)` | P1 less N1, times P2 less N2 | `(int-done TAG INTEGER)` |
| `(int-cmp-call TAG P1 N1 P2 N2)` | P1 less N1, against P2 less N2 | `(order-is TAG O)` |

[`tests/stdlib/math/int/`](../../../../../tests/stdlib/math/int/) has a program for each request and each fault.

## Using `std.math.rat` — rational numbers

A rational is `(rat Z D)`: Z an integer of `std.math.int`, D a natural number that is not zero
([Q-M1f.353](../../../../decisions/stdlib/math/rat/Q-M1f.353-a-rational-is-an-integer-over-a-natural-number.md)).
1/2 is `(rat (int (n z 1) z) (n z 2))`, −3/4 is `(rat (int z (n z 3)) (n z 4))`. Every name is `rat-`
([Q-M1f.180](../../../../decisions/stdlib/math/rat/Q-M1f.180-the-names-of-std-math-rat-carry-the-prefix-rat.md)).

```lisp
(import std.math.rat :symbols (rat-arith rat rat-add rat-sum-of))
(import std.math.int :symbols (int))
(import std.math.decimal :symbols (n))
(import std.math.arith :symbols (z))
```

The entry is `rat-arith`, which names `int-arith`, `decimal-arith`, `decimal`, `arith` and `gcd`.

| a program writes | the theory answers | meaning |
|---|---|---|
| `(rat-simplify X)` | `(rat-simple-of X S)` | S is X in lowest terms: the one request that reduces |
| `(rat-neg X)` | `(rat-neg-of X S)` | $S = 0 - X$ |
| `(rat-add X Y)` | `(rat-sum-of X Y S)` | $X + Y = S$ |
| `(rat-sub X Y)` | `(rat-diff-of X Y S)` | $X - Y = S$ |
| `(rat-mul X Y)` | `(rat-product-of X Y S)` | $X \cdot Y = S$ |
| `(rat-div X Y)` | `(rat-quot-of X Y S)` | $X \div Y = S$, one rational, exact: there is no remainder |
| `(rat-cmp X Y)` | `(rat-cmp-of X Y O)` | O is `lt`, `eq` or `gt`, by value; nothing is reduced for it |

An operation answers the cross products, or the numerators' product, over the product of the denominators:

$$
\frac{a}{b} + \frac{c}{d} = \frac{a d + c b}{b d}, \qquad \frac{a}{b} \cdot \frac{c}{d} = \frac{a c}{b d}, \qquad \frac{a}{b} \div \frac{c}{d} = \frac{a d}{b c}
$$

```lisp
(import std.math.rat :symbols (rat-arith rat rat-add rat-sum-of rat-sub rat-diff-of rat-mul rat-product-of
                               rat-div rat-quot-of rat-neg rat-neg-of))
(import std.math.int :symbols (int))
(import std.math.decimal :symbols (n))
(import std.math.arith :symbols (z))

(rat-add (rat (int (n z 1) z) (n z 2)) (rat (int (n z 1) z) (n z 3)))   ; 1/2 + 1/3
(rat-sub (rat (int (n z 1) z) (n z 2)) (rat (int (n z 1) z) (n z 3)))   ; 1/2 − 1/3
(rat-mul (rat (int (n z 2) z) (n z 3)) (rat (int z (n z 3)) (n z 4)))   ; 2/3 × −3/4
(rat-div (rat (int (n z 1) z) (n z 2)) (rat (int (n z 3) z) (n z 4)))   ; 1/2 ÷ 3/4
(rat-neg (rat (int (n z 1) z) (n z 2)))                                 ; −(1/2)

(query :goal (rat-sum-of ?x ?y ?s))
(query :goal (rat-diff-of ?x ?y ?s))
(query :goal (rat-product-of ?x ?y ?s))
(query :goal (rat-quot-of ?x ?y ?s))
(query :goal (rat-neg-of ?x ?s))
```

<!-- output -->
```text
  query 1 of 5      (rat-sum-of ?x ?y ?s)
  over              all of R — 1 state → 1 projection class
    class 1 of 1    1 state (1)
                      ?s = (rat (int (n z 5) z) (n z 6)) · ?x = (rat (int (n z 1) z) (n z 2)) · ?y = (rat (int (n z 1) z) (n z 3))
  as :expect        (model (rat-sum-of (rat (int (n z 1) z) (n z 2)) (rat (int (n z 1) z) (n z 3)) (rat (int (n z 5) z) (n z 6))))

  query 2 of 5      (rat-diff-of ?x ?y ?s)
  over              all of R — 1 state → 1 projection class
    class 1 of 1    1 state (1)
                      ?s = (rat (int (n z 1) z) (n z 6)) · ?x = (rat (int (n z 1) z) (n z 2)) · ?y = (rat (int (n z 1) z) (n z 3))
  as :expect        (model (rat-diff-of (rat (int (n z 1) z) (n z 2)) (rat (int (n z 1) z) (n z 3)) (rat (int (n z 1) z) (n z 6))))

  query 3 of 5      (rat-product-of ?x ?y ?s)
  over              all of R — 1 state → 1 projection class
    class 1 of 1    1 state (1)
                      ?s = (rat (int z (n z 6)) (n (n z 1) 2)) · ?x = (rat (int (n z 2) z) (n z 3)) · ?y = (rat (int z (n z 3)) (n z 4))
  as :expect        (model (rat-product-of (rat (int (n z 2) z) (n z 3)) (rat (int z (n z 3)) (n z 4)) (rat (int z (n z 6)) (n (n z 1) 2))))

  query 4 of 5      (rat-quot-of ?x ?y ?s)
  over              all of R — 1 state → 1 projection class
    class 1 of 1    1 state (1)
                      ?s = (rat (int (n z 4) z) (n z 6)) · ?x = (rat (int (n z 1) z) (n z 2)) · ?y = (rat (int (n z 3) z) (n z 4))
  as :expect        (model (rat-quot-of (rat (int (n z 1) z) (n z 2)) (rat (int (n z 3) z) (n z 4)) (rat (int (n z 4) z) (n z 6))))

  query 5 of 5      (rat-neg-of ?x ?s)
  over              all of R — 1 state → 1 projection class
    class 1 of 1    1 state (1)
                      ?s = (rat (int z (n z 1)) (n z 2)) · ?x = (rat (int (n z 1) z) (n z 2))
  as :expect        (model (rat-neg-of (rat (int (n z 1) z) (n z 2)) (rat (int z (n z 1)) (n z 2))))
```

1/2 + 1/3 is 5/6 and 1/2 − 1/3 is 1/6; 2/3 × −3/4 is −6/12, and 1/2 ÷ 3/4 is 4/6, neither reduced.

### An answer is not reduced; `rat-simplify` reduces, and `rat-cmp` is equality

([Q-M1f.183](../../../../decisions/stdlib/math/rat/Q-M1f.183-a-rational-is-reduced-only-on-request.md)). An
operation answers as computed, its numerator in `std.math.int`'s normal form and its denominator with no high zero
([Q-M1f.302](../../../../decisions/stdlib/math/rat/Q-M1f.302-an-unreduced-answer-has-its-numerator-in-normal.md)):
1/2 + 1/2 is 4/4, and two equal answers may be two facts. Lowest terms, zero written 0 over 1, is the one normal
form every rational has, and `rat-simplify` answers it, by `std.math.gcd`'s tagged call
([Q-M1f.303](../../../../decisions/stdlib/math/rat/Q-M1f.303-lowest-terms-hands-the-numerator-to-the-greatest.md)).
A chain of operations grows unless the program asks `rat-simplify` on the way.

```lisp
(import std.math.rat :symbols (rat-arith rat rat-add rat-sum-of rat-simplify rat-simple-of rat-cmp rat-cmp-of))
(import std.math.int :symbols (int))
(import std.math.decimal :symbols (n))
(import std.math.arith :symbols (z))

(rat-add (rat (int (n z 1) z) (n z 2)) (rat (int (n z 1) z) (n z 2)))   ; 1/2 + 1/2
(rat-simplify (rat (int (n z 4) z) (n z 4)))                            ; 4/4 in lowest terms
(rat-cmp (rat (int (n z 1) z) (n z 2)) (rat (int (n z 2) z) (n z 4)))   ; 1/2 against 2/4

(query :goal (rat-sum-of ?x ?y ?s))
(query :goal (rat-simple-of ?x ?s))
(query :goal (rat-cmp-of ?x ?y ?o))
```

<!-- output -->
```text
  query 1 of 3      (rat-sum-of ?x ?y ?s)
  over              all of R — 1 state → 1 projection class
    class 1 of 1    1 state (1)
                      ?s = (rat (int (n z 4) z) (n z 4)) · ?x = (rat (int (n z 1) z) (n z 2)) · ?y = (rat (int (n z 1) z) (n z 2))
  as :expect        (model (rat-sum-of (rat (int (n z 1) z) (n z 2)) (rat (int (n z 1) z) (n z 2)) (rat (int (n z 4) z) (n z 4))))

  query 2 of 3      (rat-simple-of ?x ?s)
  over              all of R — 1 state → 1 projection class
    class 1 of 1    1 state (1)
                      ?s = (rat (int (n z 1) z) (n z 1)) · ?x = (rat (int (n z 4) z) (n z 4))
  as :expect        (model (rat-simple-of (rat (int (n z 4) z) (n z 4)) (rat (int (n z 1) z) (n z 1))))

  query 3 of 3      (rat-cmp-of ?x ?y ?o)
  over              all of R — 1 state → 1 projection class
    class 1 of 1    1 state (1)
                      ?o = eq · ?x = (rat (int (n z 1) z) (n z 2)) · ?y = (rat (int (n z 2) z) (n z 4))
  as :expect        (model (rat-cmp-of (rat (int (n z 1) z) (n z 2)) (rat (int (n z 2) z) (n z 4)) eq))
```

The sum is 4/4; its lowest terms 1/1; and 1/2 against 2/4 is `eq`.
[`tests/stdlib/math/rat/31_equal_values_two_facts.ein`](../../../../../tests/stdlib/math/rat/31_equal_values_two_facts.ein)
holds that two equal answers stay two facts.

### Operands, and what is flagged

**A zero denominator is a division by zero**
([Q-M1f.182](../../../../decisions/stdlib/math/rat/Q-M1f.182-a-zero-denominator-is-a-division-by-zero.md)): a
request any operand of which has a denominator of zero, and a `rat-div` by a rational that is zero, derive
`(division-by-zero REQUEST)`, and `(false)` from it.

```lisp
(import std.math.rat :symbols (rat rat-neg))
(import std.math.int :symbols (int))
(import std.math.decimal :symbols (n))
(import std.math.arith :symbols (z))

(rat-neg (rat (int (n z 1) z) z))       ; 1/0
```

<!-- output -->
```text
  unsat core        (rat-neg (rat (int (n z 1) z) z))
  refuted by        std.math.arith.division-by-zero-is-false

  (false)
```

**An operand** is a rational written out, `(rat (int P N) D)`, or a name a `let` states
([Q-M1f.181](../../../../decisions/stdlib/math/rat/Q-M1f.181-a-rational-operand-is-a-rational-written-out.md)); an
integer or a natural number in a rational's place is answered by silence
([Q-M1f.277](../../../../decisions/stdlib/math/Q-M1f.277-an-operand-of-another-kind-of-number-is-answered.md)). A
digit or a named constant where the integer or the denominator goes is the rational's own fault,
`(not-rational RATIONAL THING)`:

```lisp
(import std.math.rat :symbols (rat rat-neg))
(import std.math.int :symbols (int))
(import std.math.decimal :symbols (n))
(import std.math.arith :symbols (z))

(rat-neg (rat 7 (n z 2)))
```

<!-- output -->
```text
  unsat core        (rat-neg (rat 7 (n z 2)))
  unsat core        (std.math.digits.nz 7)
  refuted by        std.math.rat.not-rational-is-false

  (false)
```

<!-- output -->
```text
  (std.math.rat.not-rational (rat 7 (n z 2)) 7 :rule std.math.rat.rat-z-digit)
```

### The tagged calls

The calls a later theory makes; the denominators must not be zero.

| a rule concludes | asked | the theory answers |
|---|---|---|
| `(rat-norm TAG P N D)` | P less N over D, not reduced | `(rat-done TAG RATIONAL)` |
| `(rat-mul-call TAG P1 N1 D1 P2 N2 D2)` | the product, not reduced | `(rat-done TAG RATIONAL)` |
| `(rat-cross TAG P1 N1 D1 P2 N2 D2)` | $P_1 D_2 + P_2 D_1$ and $N_1 D_2 + N_2 D_1$ | `(rat-cross-is TAG POSITIVE NEGATIVE)` |

[`tests/stdlib/math/rat/`](../../../../../tests/stdlib/math/rat/) has a program for each request and each fault.

## Using `std.math.gcd` — greatest common divisor and least common multiple

Of two natural numbers, by Euclid's chain over the tagged division
([Q-M1f.336](../../../../decisions/stdlib/math/gcd/Q-M1f.336-gcd-and-lcm-are-a-module-of-their-own-over.md)). `gcd`
is both the request and the module's entry.

```lisp
(import std.math.gcd :symbols (gcd lcm gcd-of lcm-of))
```

| a program writes | the theory answers | meaning |
|---|---|---|
| `(gcd X Y)` | `(gcd-of X Y G)` | G divides both and is the greatest that does; $\gcd(X, 0) = X$, and $\gcd(0, 0) = 0$ ([Q-M1f.299](../../../../decisions/stdlib/math/gcd/Q-M1f.299-the-greatest-common-divisor-of-zero-and-zero-is.md)) |
| `(lcm X Y)` | `(lcm-of X Y L)` | both divide L, the least that both divide; zero where an operand is zero |
| `(gcd-call TAG X Y)`, in a rule | `(gcd-done TAG G)` | the tagged call ([Q-M1f.300](../../../../decisions/stdlib/math/gcd/Q-M1f.300-the-greatest-common-divisors-tagged-call-is.md)): X and Y numbers, never a name |

```lisp
(import std.math.gcd :symbols (gcd lcm gcd-of lcm-of))
(import std.math.decimal-arith :symbols (let))
(import std.math.decimal :symbols (n))
(import std.math.arith :symbols (z))

(let a (n (n (n (n z 1) 0) 7) 1))
(gcd a (n (n (n z 4) 6) 2))           ; gcd(1071, 462)
(lcm (n (n z 1) 2) (n (n z 1) 8))     ; lcm(12, 18)
(gcd (n z 6) z)                       ; gcd(6, 0)
(gcd z z)                             ; gcd(0, 0)

(query :goal (gcd-of ?x ?y ?g))
(query :goal (lcm-of ?x ?y ?l))
```

<!-- output -->
```text
  query 1 of 2      (gcd-of ?x ?y ?g)
  over              all of R — 1 state → 1 projection class
    class 1 of 1    1 state (1)
                      ?g = (n (n z 2) 1) · ?x = a · ?y = (n (n (n z 4) 6) 2)
                      ?g = (n z 6) · ?x = (n z 6) · ?y = z
                      ?g = z · ?x = z · ?y = z
  as :expect        (model (gcd-of (n z 6) z (n z 6))
                           (gcd-of a (n (n (n z 4) 6) 2) (n (n z 2) 1))
                           (gcd-of z z z))

  query 2 of 2      (lcm-of ?x ?y ?l)
  over              all of R — 1 state → 1 projection class
    class 1 of 1    1 state (1)
                      ?l = (n (n z 3) 6) · ?x = (n (n z 1) 2) · ?y = (n (n z 1) 8)
  as :expect        (model (lcm-of (n (n z 1) 2) (n (n z 1) 8) (n (n z 3) 6)))
```

The greatest common divisor of 1071 and 462 is 21, answered as `(gcd-of a … …)` with the name as written; of 6 and
0 it is 6, and of 0 and 0 it is `z`; the least common multiple of 12 and 18 is 36.

An operand of a request is a natural number, high zeros allowed and read by value, or a name a `let` states; an
integer or a rational in its place is answered by silence. No request of the module divides by zero. An undeclared
name and a digit as an operand are flagged, `(undeclared NAME REQUEST)` and `(malformed REQUEST)`, and `(false)`
follows:

```lisp
(import std.math.gcd :symbols (gcd))
(import std.math.decimal :symbols (n))
(import std.math.arith :symbols (z))

(gcd 6 (n z 4))
```

<!-- output -->
```text
  unsat core        (gcd 6 (n z 4))
  unsat core        (std.math.digits.nz 6)
  refuted by        std.math.positional.malformed-is-false

  (false)
```

The tagged call, under a tag of the program's own:

```lisp
(import std.math.gcd :symbols (gcd-call gcd-done))
(import std.math.decimal :symbols (n))
(import std.math.arith :symbols (z))

(relation tiles   Number Number :why "a floor of {?1} by {?2}")
(relation square  Number Number :why "the largest square tile for a floor of {?1} by {?2}")
(relation tile-is Number Number Number :why "a floor of {?1} by {?2} takes square tiles of side {?3}")

(tiles (n (n z 1) 8) (n (n z 1) 2))        ; 18 by 12

(rule ask-the-tile ()
  :match  (tiles ?w ?h)
  :assert (gcd-call (square ?w ?h) ?w ?h)
  :why    "the largest square that tiles {?w} by {?h} has their greatest common divisor as its side")

(rule read-the-tile ()
  :match  (gcd-done (square ?w ?h) ?g)
  :assert (tile-is ?w ?h ?g)
  :why    "the side is {?g}")

(solve :no-search)
(query :goal (tile-is ?w ?h ?side))
```

<!-- output -->
```text
  query             (tile-is ?w ?h ?side)
  over              all of R — 1 state → 1 projection class
    class 1 of 1    1 state (1)
                      ?h = (n (n z 1) 2) · ?side = (n z 6) · ?w = (n (n z 1) 8)
  as :expect        (model (tile-is (n (n z 1) 8) (n (n z 1) 2) (n z 6)))
```

## How it works

Each of the three is a circuit of tagged calls into the module below it, and none joins two facts on a number.

- **An integer** is computed on its parts. A sum adds the two positive parts and the two negative parts by two
  addition walks; a product is four products of parts, summed two and two; a comparison compares the cross sums
  $P_1 + N_2$ and $P_2 + N_1$; and every answer goes through one normalisation, a subtraction walk that leaves one
  part zero. A division divides the two magnitudes by the natural numbers' `div-call` and then sets the signs, and
  the remainder, by its convention. The rules are [the integers' rules](12-integers-rules.md).
- **A rational** is computed on its integer and its denominator: the cross products by the integers' and the
  natural numbers' multiplications, and nothing divided. `rat-simplify` hands the numerator's magnitude and the
  denominator to `gcd-call` and divides both by what comes back.
- **The greatest common divisor** is Euclid's chain, $\gcd(x, y) = \gcd(y, x \bmod y)$ until the second is zero,
  each remainder one `div-call`; the least common multiple is the product divided by it. Their rules, with the
  rationals', are [the next page but one](13-rationals-and-gcd-rules.md).
