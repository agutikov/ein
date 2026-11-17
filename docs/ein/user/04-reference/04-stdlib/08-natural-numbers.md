# Natural numbers: `std.math.digits`, `std.math.arith`, `std.math.decimal`, `std.math.decimal-arith`

School arithmetic on natural numbers of any length. A program imports `std.math.decimal-arith` for the operations,
and lists the numeral `n` from `std.math.decimal` and zero, `z`, from `std.math.arith` where it writes a number.
What these modules share with the rest of the calculator, requests and results, flags, the tagged calls, is
[the calculator's page](07-calculator.md); their rules are [the next page](09-natural-numbers-rules.md) and
[the one after](10-positional-rules.md).

## Using `std.math.decimal-arith` — school arithmetic on natural numbers

Addition, subtraction, comparison, multiplication and division of natural numbers of any length, computed by
saturation alone over `std.math.digits`' tables
([Q-M1f.261](../../../../decisions/stdlib/math/decimal/Q-M1f.261-the-decimal-arithmetic-is-std-math-decimal-arith.md)).
There is no width and no overflow.

```lisp
(import std.math.decimal-arith :symbols (decimal-arith add sum-of))
(import std.math.decimal :symbols (n))
(import std.math.arith :symbols (z))
```

The entry is `decimal-arith`; a program may list it, and need not.

### The number

A natural number is `z`, zero, or `(n PREFIX DIGIT)`, ten times the prefix plus the digit, read in the order it is
written ([Q-M1f.361](../../../../decisions/stdlib/math/decimal/Q-M1f.361-a-natural-number-is-a-numeral-of-digit-names.md)):

$$
\operatorname{val}(\mathsf{z}) = 0, \qquad \operatorname{val}\bigl((\mathsf{n}\ P\ d)\bigr) = 10 \cdot \operatorname{val}(P) + d
$$

| number | numeral |
|---|---|
| 0 | `z` |
| 7 | `(n z 7)` |
| 42 | `(n (n z 4) 2)` |
| 123078 | `(n (n (n (n (n (n z 1) 2) 3) 0) 7) 8)` |

A digit is one of `0` to `9`, a prefix `z` or a numeral. Any length; a program may write high zeros,
`(n (n z 0) 7)` is 7, and the arithmetic answers in normal form, with none, so zero is `z` and never `(n z 0)`.

### The five operations

| a program writes | the theory answers | meaning |
|---|---|---|
| `(add X Y)` | `(sum-of X Y S)` | $X + Y = S$, the operands as the request wrote them, S in normal form |
| `(sub X Y)` | `(diff-of X Y D R)` | **two numbers** ([Q-M1f.364](../../../../decisions/stdlib/math/decimal/Q-M1f.364-a-subtraction-answers-the-difference.md)): D is $X - Y$ where X is not the smaller, else `z`; R is $Y - X$ where X is the smaller, else `z` |
| `(cmp X Y)` | `(cmp-of X Y O)` | O is `lt`, `eq` or `gt`, by value: `5` and `005` are `eq` |
| `(mul X Y)` | `(product-of X Y P)` | $X \cdot Y = P$, of whatever length |
| `(div X Y)` | `(quot-of X Y Q R)` | $X = Q \cdot Y + R$ with $0 \le R < Y$ |
| `(let NAME NUMBER)` | nothing: it is read | NAME names NUMBER, and a request may name an operand ([Q-M1f.149](../../../../decisions/stdlib/math/decimal/Q-M1f.149-a-request-may-name-an-operand-by-a-let.md)) |

$$
\operatorname{diff-of}(X, Y, D, R): \qquad D = \max(X - Y,\ 0), \qquad R = \max(Y - X,\ 0)
$$

One of the two is always `z`, both when $X = Y$. There is no negative number: `(diff-of X Y z (n ?p ?a))` matches
exactly where X is the smaller.

```lisp
(import std.math.decimal-arith :symbols (add sum-of sub diff-of cmp cmp-of mul product-of div quot-of))
(import std.math.decimal :symbols (n))
(import std.math.arith :symbols (z))

(add (n (n z 9) 7) (n z 5))        ; 97 + 5
(sub (n (n z 9) 7) (n z 5))        ; 97 − 5
(sub (n z 5) (n (n z 9) 7))        ; 5 − 97
(cmp (n (n z 9) 7) (n z 5))        ; 97 against 5
(mul (n (n z 9) 7) (n z 5))        ; 97 × 5
(div (n (n z 9) 7) (n z 5))        ; 97 ÷ 5

(query :goal (sum-of ?x ?y ?s))
(query :goal (diff-of ?x ?y ?d ?r))
(query :goal (cmp-of ?x ?y ?o))
(query :goal (product-of ?x ?y ?p))
(query :goal (quot-of ?x ?y ?q ?r))
```

<!-- output -->
```text
  query 1 of 5      (sum-of ?x ?y ?s)
  over              all of R — 1 state → 1 projection class
    class 1 of 1    1 state (1)
                      ?s = (n (n (n z 1) 0) 2) · ?x = (n (n z 9) 7) · ?y = (n z 5)
  as :expect        (model (sum-of (n (n z 9) 7) (n z 5) (n (n (n z 1) 0) 2)))

  query 2 of 5      (diff-of ?x ?y ?d ?r)
  over              all of R — 1 state → 1 projection class
    class 1 of 1    1 state (1)
                      ?d = (n (n z 9) 2) · ?r = z · ?x = (n (n z 9) 7) · ?y = (n z 5)
                      ?d = z · ?r = (n (n z 9) 2) · ?x = (n z 5) · ?y = (n (n z 9) 7)
  as :expect        (model (diff-of (n (n z 9) 7) (n z 5) (n (n z 9) 2) z)
                           (diff-of (n z 5) (n (n z 9) 7) z (n (n z 9) 2)))

  query 3 of 5      (cmp-of ?x ?y ?o)
  over              all of R — 1 state → 1 projection class
    class 1 of 1    1 state (1)
                      ?o = gt · ?x = (n (n z 9) 7) · ?y = (n z 5)
  as :expect        (model (cmp-of (n (n z 9) 7) (n z 5) gt))

  query 4 of 5      (product-of ?x ?y ?p)
  over              all of R — 1 state → 1 projection class
    class 1 of 1    1 state (1)
                      ?p = (n (n (n z 4) 8) 5) · ?x = (n (n z 9) 7) · ?y = (n z 5)
  as :expect        (model (product-of (n (n z 9) 7) (n z 5) (n (n (n z 4) 8) 5)))

  query 5 of 5      (quot-of ?x ?y ?q ?r)
  over              all of R — 1 state → 1 projection class
    class 1 of 1    1 state (1)
                      ?q = (n (n z 1) 9) · ?r = (n z 2) · ?x = (n (n z 9) 7) · ?y = (n z 5)
  as :expect        (model (quot-of (n (n z 9) 7) (n z 5) (n (n z 1) 9) (n z 2)))
```

97 + 5 is 102; 97 − 5 is 92 and nothing short, 5 − 97 is nothing with 92 short; 97 is greater; 97 × 5 is 485; and
97 ÷ 5 is 19, remainder 2.

**An operand is read by value** and may carry high zeros; every answer is in normal form. Zero is an operand like
any other:

```lisp
(import std.math.decimal-arith :symbols (add sum-of sub diff-of cmp cmp-of))
(import std.math.decimal :symbols (n))
(import std.math.arith :symbols (z))

(add z (n z 5))                    ; 0 + 5
(sub (n z 5) (n z 5))              ; 5 − 5
(cmp (n z 5) (n (n (n z 0) 0) 5))  ; 5 against 005

(query :goal (sum-of ?x ?y ?s))
(query :goal (diff-of ?x ?y ?d ?r))
(query :goal (cmp-of ?x ?y ?o))
```

<!-- output -->
```text
  query 1 of 3      (sum-of ?x ?y ?s)
  over              all of R — 1 state → 1 projection class
    class 1 of 1    1 state (1)
                      ?s = (n z 5) · ?x = z · ?y = (n z 5)
  as :expect        (model (sum-of z (n z 5) (n z 5)))

  query 2 of 3      (diff-of ?x ?y ?d ?r)
  over              all of R — 1 state → 1 projection class
    class 1 of 1    1 state (1)
                      ?d = z · ?r = z · ?x = (n z 5) · ?y = (n z 5)
  as :expect        (model (diff-of (n z 5) (n z 5) z z))

  query 3 of 3      (cmp-of ?x ?y ?o)
  over              all of R — 1 state → 1 projection class
    class 1 of 1    1 state (1)
                      ?o = eq · ?x = (n z 5) · ?y = (n (n (n z 0) 0) 5)
  as :expect        (model (cmp-of (n z 5) (n (n (n z 0) 0) 5) eq))
```

### Named constants

With `(let price …)` and `(let qty …)`, `(mul price qty)` is answered by `(product-of price qty P)`, the names
repeated as written. A name stands for one number, never for another name.

```lisp
(import std.math.decimal-arith :symbols (let mul product-of add sum-of))
(import std.math.decimal :symbols (n))
(import std.math.arith :symbols (z))

(let price (n (n z 1) 2))          ; 12
(let qty   (n z 3))                ; 3

(mul price qty)
(add price (n (n z 0) 8))          ; 12 + 08: a high zero is read by value

(query :goal (product-of price qty ?p))
(query :goal (sum-of price ?y ?s))
```

<!-- output -->
```text
  query 1 of 2      (product-of price qty ?p)
  over              all of R — 1 state → 1 projection class
    class 1 of 1    1 state (1)
                      ?p = (n (n z 3) 6)
  as :expect        (model (product-of price qty (n (n z 3) 6)))

  query 2 of 2      (sum-of price ?y ?s)
  over              all of R — 1 state → 1 projection class
    class 1 of 1    1 state (1)
                      ?s = (n (n z 2) 0) · ?y = (n (n z 0) 8)
  as :expect        (model (sum-of price (n (n z 0) 8) (n (n z 2) 0)))
```

**A request is answered once**, and the same request written with other spellings of its operands, a name for a
number or a high zero more, is another request with its own answer:

```lisp
(import std.math.decimal-arith :symbols (let add sum-of))
(import std.math.decimal :symbols (n))
(import std.math.arith :symbols (z))

(let five (n z 5))
(add five (n z 1))
(add (n z 5) (n z 1))
(add (n (n z 0) 5) (n z 1))

(query :goal (sum-of ?x ?y ?s))
```

<!-- output -->
```text
  query             (sum-of ?x ?y ?s)
  over              all of R — 1 state → 1 projection class
    class 1 of 1    1 state (1)
                      ?s = (n z 6) · ?x = (n (n z 0) 5) · ?y = (n z 1)
                      ?s = (n z 6) · ?x = (n z 5) · ?y = (n z 1)
                      ?s = (n z 6) · ?x = five · ?y = (n z 1)
  as :expect        (model (sum-of (n (n z 0) 5) (n z 1) (n z 6))
                           (sum-of (n z 5) (n z 1) (n z 6))
                           (sum-of five (n z 1) (n z 6)))
```

### Division by zero

`(div X Y)` with Y zero by value derives the flag `(division-by-zero (div X Y))`, and `(false)` from it: root is
dead, R is empty, no query has an answer, and `ein solve` names the request as the unsat core and
`std.math.arith.division-by-zero-is-false` under `refuted by`
([the calculator's page](07-calculator.md#a-fault-is-a-flag-and-the-flag-is-a-contradiction) shows the run). It is
flagged wherever division is called, through the tagged call too, so a public division by zero holds two flags,
the request's and its call's
([Q-M1f.163](../../../../decisions/stdlib/math/decimal/Q-M1f.163-a-division-by-zero-is-flagged-wherever-division.md),
[Q-M1f.211](../../../../decisions/stdlib/math/decimal/Q-M1f.211-a-public-division-by-zero-holds-two-flags.md)), and
is decided by root alone
([Q-M1f.165](../../../../decisions/stdlib/math/decimal/Q-M1f.165-a-program-that-divides-by-zero-is-decided.md)).

A program that must not die compares its divisor with zero first, and derives its `div` request by a rule over
`(cmp-of Y z gt)`:

```lisp
(import std.math.decimal-arith :symbols (cmp cmp-of div quot-of))
(import std.math.decimal :symbols (n))
(import std.math.arith :symbols (z))

(relation share        Number Number :why "{?1} is shared among {?2}")
(relation cannot-share Number        :why "{?1} cannot be shared among nobody")

(share (n z 7) z)
(share (n z 7) (n z 2))

(rule ask-whether-zero ()
  :match  (share ?x ?y)
  :assert (cmp ?y z)
  :why    "is {?y} zero?")

(rule divide-when-positive ()
  :match  (and (share ?x ?y) (cmp-of ?y z gt))
  :assert (div ?x ?y)
  :why    "{?y} is more than zero, so {?x} can be divided by it")

(rule refuse-when-zero ()
  :match  (and (share ?x ?y) (cmp-of ?y z eq))
  :assert (cannot-share ?x)
  :why    "{?y} is zero")

(solve :no-search)
(query :goal (quot-of ?x ?y ?q ?r))
(query :goal (cannot-share ?x))
```

<!-- output -->
```text
  query 1 of 2      (quot-of ?x ?y ?q ?r)
  over              all of R — 1 state → 1 projection class
    class 1 of 1    1 state (1)
                      ?q = (n z 3) · ?r = (n z 1) · ?x = (n z 7) · ?y = (n z 2)
  as :expect        (model (quot-of (n z 7) (n z 2) (n z 3) (n z 1)))

  query 2 of 2      (cannot-share ?x)
  over              all of R — 1 state → 1 projection class
    class 1 of 1    1 state (1)
                      ?x = (n z 7)
  as :expect        (model (cannot-share (n z 7)))
```

### What is flagged

| the theory derives | where |
|---|---|
| `(malformed REQUEST)` | a digit where a number goes, `(add 7 X)` |
| `(malformed (let NAME N))` | a `let` that cannot be read: a name stated twice with two numbers, `z` or a digit as the name, a digit or another name as the number |
| `(undeclared NAME REQUEST)` | an operand that is a name no `let` states |
| `(division-by-zero (div X Y))` | the divisor is zero by value, however written; the flag is `std.math.arith`'s |

Each is a contradiction. The runs, and what is answered by silence instead, are
[the calculator's page](07-calculator.md#malformed-numbers-what-is-flagged-and-where).

### No module reserves its names

A program may state a result by hand, and the theory computes with it: this sum is an answer nobody computed.

```lisp
(import std.math.decimal-arith :symbols (sum-of))
(import std.math.decimal :symbols (n))
(import std.math.arith :symbols (z))

(sum-of (n z 2) (n z 2) (n z 5))

(query :goal (sum-of ?x ?y ?s))
```

<!-- output -->
```text
  query             (sum-of ?x ?y ?s)
  over              all of R — 1 state → 1 projection class
    class 1 of 1    1 state (1)
                      ?s = (n z 5) · ?x = (n z 2) · ?y = (n z 2)
  as :expect        (model (sum-of (n z 2) (n z 2) (n z 5)))
```

### The tagged calls

An interface for a theory built on this one
([Q-M1f.189](../../../../decisions/stdlib/math/Q-M1f.189-the-decimal-arithmetics-tagged-calls-are.md),
[Q-M1f.228](../../../../decisions/stdlib/math/Q-M1f.228-the-interface-of-the-tagged-calls-is-what-their.md)). A call
looks nothing up: X and Y are numbers as `n` and `z` build them, high zeros allowed, never a name. TAG is a fact of
the caller's own relation, which the answer carries back; a tag of the theory's own, `(sum X Y)` or
`(quotient X Y)`, is read back as one of its requests. A tag names one call: two calls under one tag answer under
one tag, and the rule that reads cannot tell them apart.

| a rule concludes | asked | the theory answers |
|---|---|---|
| `(add-walk TAG X Y C z)` | $X + Y + C$, C a carry in of `0` or `1` | `(done TAG S)` |
| `(sub-walk TAG X Y 0 z)` | $X - Y$ | `(done TAG D)` where X is not the smaller, `(short TAG)` where it is |
| `(cmp-walk TAG X Y eq)` | X against Y | `(order-is TAG O)` |
| `(mul-call TAG X Y)` | $X \cdot Y$ | `(done TAG P)` |
| `(div-call TAG X Y)` | $X \div Y$ | `(div-done TAG Q R)`; flagged where Y is zero |
| `(operand TAG X)` | what X stands for, a number or a name a `let` states | `(operand-is TAG N)` |

A walk starts with `z`, no digits yet; a subtraction with `0`, no borrow; a comparison with `eq`, nothing decided:
those are the values a caller writes. A program that doubles a number by one addition walk under a tag of its own
is on [the calculator's page](07-calculator.md#no-rule-joins-two-facts-on-a-number), and
[`examples/beyond_arith/casting_out_nines.ein`](../../../../../examples/beyond_arith/casting_out_nines.ein) calls
`cmp-walk`, `add-walk` and `div-call` with tags of its own, to count, to add up digits and to take remainders.

### How long a number may be

The theory has no width. A program can write a number of at most 999 digits, since a numeral nests a level a digit
and the parser refuses a program nested past 1 000 parentheses ([diagnostics](../07-diagnostics.md)); a rule may
derive a longer one. A step of a circuit holds its operands inside its tag, two levels deeper than the request for
a sum, a difference and a comparison and three for a product and a quotient, so a state written by `--dump-answer`
from operands near that length is refused when it is loaded again, though every answer is right and printed. A
product's or a quotient's facts grow as the two lengths multiplied, each holding its numbers whole, so a long
product is a query's to read rather than a state to write out.

## Using `std.math.decimal` — the decimal numeral

The decimal **representation**: the numeral and what a written one must be
([Q-M1f.273](../../../../decisions/stdlib/math/decimal/Q-M1f.273-the-decimal-numeral-is-std-math-decimal-s-which.md)).
It computes nothing.

```lisp
(import std.math.decimal :symbols (n))
```

| fact | who writes it | meaning |
|---|---|---|
| `(n PREFIX DIGIT)` | the program | the numeral |
| `(named NAME)` | `std.math.decimal-arith`, for each name a `let` states | NAME stands for a number |
| `(not-decimal NUMERAL THING)` | this module: a **flag** | NUMERAL, written somewhere in the program, holds THING where a digit or a prefix goes; `not-decimal-is-false` derives `(false)` |

**A numeral is checked where it is written**
([Q-M1f.210](../../../../decisions/stdlib/math/Q-M1f.210-a-malformed-number-is-its-representations-fault.md)). The
loader states every fact nested in a fact a file writes as `(written T)`
([reserved names](../03-reserved-names.md)), so the module's rules see every place of every numeral, as an operand,
inside an integer or a rational, as a `let`'s value nobody asks about, or in a fact of the program's own relation,
and flag a malformed one in root's first layer, before any calculation, on its smallest `n` term at fault
([Q-M1f.274](../../../../decisions/stdlib/math/decimal/Q-M1f.274-a-malformed-numeral-is-flagged-on-its-smallest.md)).
A named constant is told by `named`
([Q-M1f.278](../../../../decisions/stdlib/math/decimal/Q-M1f.278-a-named-constant-is-told-by-named-the-numeral.md)).
Entry `decimal`.

A program that imports this module alone, and computes nothing, is still held to its numerals. This is
[`tests/stdlib/math/decimal/03_in_a_fact_of_the_programs_own.ein`](../../../../../tests/stdlib/math/decimal/03_in_a_fact_of_the_programs_own.ein):

```lisp
(import std.math.decimal-arith :symbols (decimal-arith))
(import std.math.decimal :symbols (n))
(import std.math.arith :symbols (z))

(relation price Item Amount :why "{?1} costs {?2}")
(__closed__ price)
(price apple (n 1 2))

(solve :expect (false))
```

<!-- output -->
```text
  unsat core        (price apple (n 1 2))
  unsat core        (std.math.digits.nz 1)
  refuted by        std.math.decimal.not-decimal-is-false
```

The programs beside it hold the rest: a well-formed numeral in every place a file can write one
(`01_well_formed_everywhere.ein`), a malformed one in a `let` nobody asks about and in a rule's conclusion (`02`,
`04`), and what stays silent (`05`).

## Using `std.math.arith` — zero, and the division by zero

What is true of a number in every base, and so is no base's own:

- **`z`, zero**, the number with no digit: every numeral ends in it, decimal's and binary's alike
  ([Q-M1f.322](../../../../decisions/stdlib/math/Q-M1f.322-zero-is-one-z-in-every-base.md)). It is declared as a
  relation of no argument that no fact is ever of, so that it has one full name
  ([Q-M1f.283](../../../../decisions/stdlib/math/Q-M1f.283-zero-is-a-relation-of-no-argument-and-each.md)), and a
  file that writes it lists it.
- **Division by zero is a contradiction**, in every base
  ([Q-M1f.269](../../../../decisions/stdlib/math/Q-M1f.269-division-by-zero-is-a-contradiction.md)): a module that
  divides derives the flag `(division-by-zero REQUEST)`, the request or the call as it was asserted, and
  `division-by-zero-is-false` derives `(false)` from it. A theory in which division by zero is allowed would be a
  module that does not import this one; there is none.

```lisp
(import std.math.arith :symbols (z division-by-zero))
```

Entry `arith`. A program lists `z` where it writes a numeral, and `division-by-zero` where a rule or a claim reads
the flag.

## Using `std.math.digits` — the digit tables

A library of facts: no rule and no entry. Each table is complete and closed by a written `(__closed__ R)` fact of
the module
([Q-M1f.146](../../../../decisions/stdlib/math/Q-M1f.146-each-digit-table-is-closed-by-a-written-fact.md)), which
every import brings, so no row is ever a hypothesis. The file is generated and not edited by hand, and every row
is held to the integers
([Q-M1f.366](../../../../decisions/stdlib/math/Q-M1f.366-the-calculators-tables-are-generated-facts-every.md)).
A theory that reads a table closes it in its own entry, and a program that imports the theory lists nothing from
here; one that reads a table itself lists it:

```lisp
(import std.math.digits :symbols (fa nz __closed__))
```

| row | reads | gives | rows |
|---|---|---|---|
| `(fa a b cin d cout)` | two digits and a carry in, `0` or `1` | $a + b + c_{in} = 10 \cdot c_{out} + d$ | $10 \cdot 10 \cdot 2 = 200$ |
| `(fs a b bin d bout)` | two digits and a borrow in, `0` or `1` | $a - b - b_{in} = d - 10 \cdot b_{out}$ | $10 \cdot 10 \cdot 2 = 200$ |
| `(md a b cin d cout)` | two digits and a carry in, `0` to `8` | $a \cdot b + c_{in} = 10 \cdot c_{out} + d$ | $10 \cdot 10 \cdot 9 = 900$ |
| `(cl below a b order)` | the order of the digits below, `lt`, `eq` or `gt`, and one digit of each number | `lt` if $a < b$, `gt` if $a > b$, else `below` | $3 \cdot 10 \cdot 10 = 300$ |
| `(nz d)` | a digit | it is not zero: `1` to `9` | $9$ |
| `(mid lo hi d)` | two bounds from `0` to `10`, at least two apart | the digit between them to try, $d = \lfloor (lo + hi) / 2 \rfloor$ | $\binom{11}{2} - 10 = 45$ |
| `(adj lo hi)` | two bounds | nothing lies between them: $hi = lo + 1$ | $10$ |

A table is complete, a row for each input, which is what the count in the last column says. A program reads a row
by a query, or by a premise of a rule:

```lisp
(import std.math.digits :symbols (fa md))

(query :goal (fa 7 5 0 ?digit ?carry))
(query :goal (md 7 8 0 ?digit ?carry))
```

<!-- output -->
```text
  query 1 of 2      (fa 7 5 0 ?digit ?carry)
  over              all of R — 1 state → 1 projection class
    class 1 of 1    1 state (1)
                      ?carry = 1 · ?digit = 2
  as :expect        (model (fa 7 5 0 2 1))

  query 2 of 2      (md 7 8 0 ?digit ?carry)
  over              all of R — 1 state → 1 projection class
    class 1 of 1    1 state (1)
                      ?carry = 5 · ?digit = 6
  as :expect        (model (md 7 8 0 6 5))
```

7 + 5 is 2, carry 1, and 7 × 8 is 6, carry 5. A core that passes through a table names the row, since a row is a
premise like any other fact:
[`tests/stdlib/math/digits/02_a_core_names_its_rows.ein`](../../../../../tests/stdlib/math/digits/02_a_core_names_its_rows.ein)
refutes a column a program states wrongly.

<!-- output -->
```text
  unsat core        (column 7 5 3)
  unsat core        (fa 7 5 0 2 1)
  refuted by        column-wrong
```

## How it works

A sum, in the columns of school arithmetic. With $x_i$ and $y_i$ the digits of the two numbers from the units, and
$c_i$ the carry into column $i$:

$$
x_i + y_i + c_i = 10 \cdot c_{i+1} + s_i, \qquad c_0 = 0
$$

and the sum's digits are the $s_i$, with the last carry in front. The table `fa` holds every solution of the
equation for one column, so a rule that takes a column has no arithmetic to do: it looks the row up.

Each operand is looked up; one walk from the units takes each column through the full adder's table and puts its
digit in front of an accumulator, `(b DIGIT REST)`; high zeros are dropped there; the answer is turned round into
the public shape and handed over as `(done TAG ANSWER)`. Every call carries its continuation as its tag. The
steps of `(add (n z 2) (n z 3))` are in the state for a query or a trace to see. `ein solve -P` prints what a state
holds beyond the program, and these are the facts of the sum among them:

```lisp
(import std.math.decimal-arith :symbols (add))
(import std.math.decimal :symbols (n))
(import std.math.arith :symbols (z))

(add (n z 2) (n z 3))
```

<!-- output -->
```text
  (std.math.decimal-arith.add-walk (std.math.decimal-arith.sum (n z 2) (n z 3)) (n z 2) (n z 3) 0 z)
  (std.math.decimal-arith.add-walk (std.math.decimal-arith.sum (n z 2) (n z 3)) z z 0 (std.math.decimal-arith.b 5 z))
  …
  (std.math.decimal-arith.done (std.math.decimal-arith.sum (n z 2) (n z 3)) (n z 5))
  (std.math.decimal-arith.operand (std.math.decimal-arith.add-first (n z 2) (n z 3)) (n z 2))
  (std.math.decimal-arith.operand (std.math.decimal-arith.add-second (n z 2) (n z 3) (n z 2)) (n z 3))
  (std.math.decimal-arith.operand-is (std.math.decimal-arith.add-first (n z 2) (n z 3)) (n z 2))
  (std.math.decimal-arith.operand-is (std.math.decimal-arith.add-second (n z 2) (n z 3) (n z 2)) (n z 3))
  (std.math.decimal-arith.strip (std.math.decimal-arith.sum (n z 2) (n z 3)) (std.math.decimal-arith.b 5 z))
  (std.math.decimal-arith.sum-of (n z 2) (n z 3) (n z 5))
  (std.math.decimal-arith.turn (std.math.decimal-arith.sum (n z 2) (n z 3)) (std.math.decimal-arith.b 5 z) z)
  (std.math.decimal-arith.turn (std.math.decimal-arith.sum (n z 2) (n z 3)) z (n z 5))
```

The listing is in the order of the names. In the order of the computation: the two operands are asked and
answered; the walk starts with carry `0` and no digit, and after the one column
holds `(b 5 z)`; `strip` finds no high zero; `turn` turns the accumulator round into `(n z 5)`; `done` hands it to
the tag `(sum …)`, and the result repeats the request's operands.

- **Subtraction** is two walks side by side, $X - Y$ and $Y - X$, since which is smaller is not known before the
  top column: the walk that ends with no borrow answers.
- **Comparison** is one walk over the comparator's table.
- **Multiplication** is the school method from the multiplier's top digit.
- **Division** is long division, each digit of the quotient found by bisection over the `mid` and `adj` tables, at
  most four trials a digit
  ([Q-M1f.324](../../../../decisions/stdlib/math/decimal/Q-M1f.324-decimal-division-is-long-division-each-digit.md)).

The circuit's own relations — the accumulator `b`, the walks, the steps, the tags, `arith-rule` — are in the
state, and are not an interface; the module's header lists every name it occupies.

The rules are on two pages. What every positional base does the same way, the walks, the requests and their
answers, the operands and their faults, is `std.math.positional`'s, written once with every relation a parameter:
[the rules every base shares](10-positional-rules.md). What is decimal's own, a row of a product by one digit and
the bisection of a quotient's digit, with the two small modules below, is
[the natural numbers' rules](09-natural-numbers-rules.md).
