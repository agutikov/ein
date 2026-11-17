# Binary numbers and conversion: `std.math.bits`, `std.math.binary`, `std.math.binary-arith`, `std.math.convert`

Binary arithmetic is a theory of its own beside decimal's
([Q-M1f.321](../../../../decisions/stdlib/math/binary/Q-M1f.321-binary-arithmetic-is-a-theory-of-its-own-beside.md)):
its own numeral, its own names, every one `bin-`, and the same shared rules. Nothing of decimal is loaded with it:
a program that imports only binary holds no decimal row. A program that imports both shares `z`, `0`, `1`,
`std.math.arith` and the shared rules, and nothing else. What it shares with the rest of the calculator is
[the calculator's page](07-calculator.md); its rules are [the next page](15-binary-rules.md).

| module | what it is |
|---|---|
| `std.math.bits` | the tables, facts and no rule, generated |
| `std.math.binary` | the numeral `(b2 PREFIX BIT)`, `bin-named`, and the numeral's check where it is written ([Q-M1f.295](../../../../decisions/stdlib/math/binary/Q-M1f.295-the-binary-numeral-is-std-math-binary-s-which.md)); entry `binary` |
| `std.math.binary-arith` | the arithmetic, `std.math.decimal-arith`'s twin ([Q-M1f.267](../../../../decisions/stdlib/math/binary/Q-M1f.267-binarys-names-carry-the-prefix-bin.md)); entry `binary-arith` |
| `std.math.convert` | decimal to binary and back; entry `convert` |

## Using `std.math.binary-arith` — binary arithmetic

```lisp
(import std.math.binary-arith :symbols (binary-arith bin-add bin-sum-of))
(import std.math.binary :symbols (b2))
(import std.math.arith :symbols (z))
```

A binary number is `z` or `(b2 PREFIX BIT)`, twice the prefix plus the bit; the prefix is `z` or a numeral, the
bit `0` or `1`:

$$
\begin{aligned}
\operatorname{val}(\mathsf{z}) &= 0 \\
\operatorname{val}\bigl((\mathsf{b2}\ P\ d)\bigr) &= 2 \cdot \operatorname{val}(P) + d
\end{aligned}
$$

5 is `(b2 (b2 (b2 z 1) 0) 1)`.

| a program writes | the theory answers | meaning |
|---|---|---|
| `(bin-add X Y)` | `(bin-sum-of X Y S)` | $X + Y = S$ |
| `(bin-sub X Y)` | `(bin-diff-of X Y D R)` | **two numbers**, as decimal's `diff-of`: the difference, and what X is short by |
| `(bin-cmp X Y)` | `(bin-cmp-of X Y O)` | O is `lt`, `eq` or `gt`, by value |
| `(bin-mul X Y)` | `(bin-product-of X Y P)` | $X \cdot Y = P$ |
| `(bin-div X Y)` | `(bin-quot-of X Y Q R)` | $X = Q \cdot Y + R$ with $0 \le R < Y$ |
| `(bin-let NAME NUMBER)` | nothing: it is read | NAME names NUMBER |

```lisp
(import std.math.binary-arith :symbols (binary-arith bin-add bin-sum-of bin-mul bin-product-of))
(import std.math.binary :symbols (b2))
(import std.math.arith :symbols (z))

(bin-add (b2 (b2 (b2 z 1) 0) 1) (b2 (b2 (b2 z 1) 1) 1))   ; 101 + 111
(bin-mul (b2 (b2 (b2 z 1) 0) 1) (b2 (b2 (b2 z 1) 1) 1))   ; 101 × 111

(query :goal (bin-sum-of ?x ?y ?s))
(query :goal (bin-product-of ?x ?y ?p))
```

<!-- output -->
```text
  query 1 of 2      (bin-sum-of ?x ?y ?s)
  over              all of R — 1 state → 1 projection class
    class 1 of 1    1 state (1)
                      ?s = (b2 (b2 (b2 (b2 z 1) 1) 0) 0) · ?x = (b2 (b2 (b2 z 1) 0) 1) · ?y = (b2 (b2 (b2 z 1) 1) 1)
  as :expect        (model (bin-sum-of (b2 (b2 (b2 z 1) 0) 1) (b2 (b2 (b2 z 1) 1) 1) (b2 (b2 (b2 (b2 z 1) 1) 0) 0)))

  query 2 of 2      (bin-product-of ?x ?y ?p)
  over              all of R — 1 state → 1 projection class
    class 1 of 1    1 state (1)
                      ?p = (b2 (b2 (b2 (b2 (b2 (b2 z 1) 0) 0) 0) 1) 1) · ?x = (b2 (b2 (b2 z 1) 0) 1) · ?y = (b2 (b2 (b2 z 1) 1) 1)
  as :expect        (model (bin-product-of (b2 (b2 (b2 z 1) 0) 1) (b2 (b2 (b2 z 1) 1) 1) (b2 (b2 (b2 (b2 (b2 (b2 z 1) 0) 0) 0) 1) 1)))
```

The sum is 1100, and the product 100011. The other three operations, with a named constant and a high zero:

```lisp
(import std.math.binary-arith :symbols (bin-sub bin-diff-of bin-cmp bin-cmp-of bin-div bin-quot-of bin-let))
(import std.math.binary :symbols (b2))
(import std.math.arith :symbols (z))

(bin-let mask (b2 (b2 (b2 z 1) 1) 0))                     ; 110
(bin-sub mask (b2 (b2 z 1) 1))                            ; 110 − 11
(bin-sub (b2 (b2 z 1) 1) mask)                            ; 11 − 110
(bin-cmp mask (b2 (b2 (b2 (b2 z 0) 1) 1) 0))              ; 110 against 0110
(bin-div mask (b2 (b2 (b2 z 1) 0) 0))                     ; 110 ÷ 100

(query :goal (bin-diff-of ?x ?y ?d ?r))
(query :goal (bin-cmp-of ?x ?y ?o))
(query :goal (bin-quot-of ?x ?y ?q ?r))
```

<!-- output -->
```text
  query 1 of 3      (bin-diff-of ?x ?y ?d ?r)
  over              all of R — 1 state → 1 projection class
    class 1 of 1    1 state (1)
                      ?d = (b2 (b2 z 1) 1) · ?r = z · ?x = mask · ?y = (b2 (b2 z 1) 1)
                      ?d = z · ?r = (b2 (b2 z 1) 1) · ?x = (b2 (b2 z 1) 1) · ?y = mask
  as :expect        (model (bin-diff-of (b2 (b2 z 1) 1) mask z (b2 (b2 z 1) 1))
                           (bin-diff-of mask (b2 (b2 z 1) 1) (b2 (b2 z 1) 1) z))

  query 2 of 3      (bin-cmp-of ?x ?y ?o)
  over              all of R — 1 state → 1 projection class
    class 1 of 1    1 state (1)
                      ?o = eq · ?x = mask · ?y = (b2 (b2 (b2 (b2 z 0) 1) 1) 0)
  as :expect        (model (bin-cmp-of mask (b2 (b2 (b2 (b2 z 0) 1) 1) 0) eq))

  query 3 of 3      (bin-quot-of ?x ?y ?q ?r)
  over              all of R — 1 state → 1 projection class
    class 1 of 1    1 state (1)
                      ?q = (b2 z 1) · ?r = (b2 (b2 z 1) 0) · ?x = mask · ?y = (b2 (b2 (b2 z 1) 0) 0)
  as :expect        (model (bin-quot-of mask (b2 (b2 (b2 z 1) 0) 0) (b2 z 1) (b2 (b2 z 1) 0)))
```

110 − 11 is 11 and nothing short; 11 − 110 is nothing, with 11 short; 110 and 0110 are `eq`; and 110 ÷ 100 is 1,
remainder 10.

### What is flagged

| the flag | where | by |
|---|---|---|
| `(not-binary NUMERAL THING)` | a bit's place holds no bit, or a bit or a named constant is the prefix | `std.math.binary` |
| `(bin-malformed REQUEST)` | a bit where a number goes, a `bin-let` that cannot be read ([Q-M1f.268](../../../../decisions/stdlib/math/binary/Q-M1f.268-binary-flags-a-malformed-request-as-far-as.md)) | `std.math.binary-arith` |
| `(bin-undeclared NAME REQUEST)` | a name no `bin-let` states | `std.math.binary-arith` |
| `(division-by-zero (bin-div X Y))` | the divisor is zero by value; its call is flagged beside it. The flag is `std.math.arith`'s | `std.math.binary-arith` |

A `2` in a bit's place:

```lisp
(import std.math.binary-arith :symbols (bin-add))
(import std.math.binary :symbols (b2))
(import std.math.arith :symbols (z))

(bin-add (b2 (b2 z 1) 2) (b2 z 1))
```

<!-- output -->
```text
  unsat core        (bin-add (b2 (b2 z 1) 2) (b2 z 1))
  refuted by        std.math.binary.not-binary-is-false

  (false)
```

<!-- output -->
```text
  (std.math.binary.not-binary (b2 (b2 z 1) 2) 2 :rule std.math.binary.bin-numeral-bit)
```

A bit where a number goes, `(bin-add 1 (b2 z 1))`: one is `(b2 z 1)`.

<!-- output -->
```text
  unsat core        (bin-add 1 (b2 z 1))
  unsat core        (std.math.bits.bin-nz 1)
  refuted by        std.math.positional.malformed-is-false

  (false)
```

<!-- output -->
```text
  (std.math.binary-arith.bin-malformed (bin-add 1 (b2 z 1)) :rule std.math.positional.bad-request-malformed)
```

**A decimal digit is a name in binary**: `(bin-add 7 X)` is undeclared, and `(b2 7 1)` is silent.

<!-- output -->
```text
  unsat core        (bin-add 7 (b2 z 1))
  refuted by        std.math.positional.undeclared-is-false

  (false)
```

<!-- output -->
```text
  (std.math.binary-arith.bin-undeclared 7 (bin-add 7 (b2 z 1)) :rule std.math.positional.bad-request-undeclared)
```

```lisp
(import std.math.binary-arith :symbols (bin-add bin-sum-of))
(import std.math.binary :symbols (b2))
(import std.math.arith :symbols (z))

(bin-add (b2 7 1) (b2 z 1))

(query :goal (bin-sum-of ?x ?y ?s))
```

<!-- output -->
```text
  ∅                 in R — root is alive and owes nothing
  …
  query             (bin-sum-of ?x ?y ?s)
  over              all of R — 1 state → 1 projection class
    class 1 of 1    1 state (1) — the goal has no row: undetermined here
  as :expect        (model)
```

A division by zero, the zero written with a high zero:

```lisp
(import std.math.binary-arith :symbols (bin-div))
(import std.math.binary :symbols (b2))
(import std.math.arith :symbols (z))

(bin-div (b2 (b2 z 1) 0) (b2 z 0))                        ; 10 ÷ 0
```

<!-- output -->
```text
  unsat core        (bin-div (b2 (b2 z 1) 0) (b2 z 0))
  unsat core        (std.math.bits.bin-fa 0 0 0 0 0)
  refuted by        std.math.arith.division-by-zero-is-false

  (false)
```

### The tagged calls

Decimal's with `bin-` before each name
([Q-M1f.297](../../../../decisions/stdlib/math/binary/Q-M1f.297-binarys-tagged-calls-are-an-interface.md)):

| a rule concludes | the theory answers |
|---|---|
| `(bin-add-walk TAG X Y C z)` | `(bin-done TAG S)` |
| `(bin-sub-walk TAG X Y 0 z)` | `(bin-done TAG D)`, or `(bin-short TAG)` where X is the smaller |
| `(bin-cmp-walk TAG X Y eq)` | `(bin-order-is TAG O)` |
| `(bin-mul-call TAG X Y)` | `(bin-done TAG P)` |
| `(bin-div-call TAG X Y)` | `(bin-div-done TAG Q R)` |
| `(bin-operand TAG X)` | `(bin-operand-is TAG N)` |

[`examples/binary_arithmetic/`](../../../../../examples/binary_arithmetic/README.md) has programs over binary
arithmetic, and [`tests/stdlib/math/binary-arith/`](../../../../../tests/stdlib/math/binary-arith/) one for each
request and each fault.

## Using `std.math.binary` — the binary numeral

The binary representation: the numeral `b2`, `bin-named`, which the module that declares `bin-let` writes for
each name, and the flag `not-binary`. It computes nothing, and checks every numeral where it is written, as
`std.math.decimal` checks decimal's. A program lists `b2` from it wherever it writes a binary numeral.

```lisp
(import std.math.binary :symbols (b2))
```

[`tests/stdlib/math/binary/`](../../../../../tests/stdlib/math/binary/) holds the check: a well-formed numeral in
every place a file can write one, a name in a bit's place, a numeral that ends in a bit, in `0`, in a name, and
what stays silent.

## Using `std.math.bits` — the bit tables

Facts and no rule, generated beside the digits'. A program that reads a table imports this module
([Q-M1f.192](../../../../decisions/stdlib/math/Q-M1f.192-a-program-that-reads-the-binary-tables-imports.md)).

| row | gives | rows |
|---|---|---|
| `(bin-fa a b cin d cout)` | $a + b + c_{in} = 2 \cdot c_{out} + d$ | $2 \cdot 2 \cdot 2 = 8$ |
| `(bin-fs a b bin d bout)` | $a - b - b_{in} = d - 2 \cdot b_{out}$ | $2 \cdot 2 \cdot 2 = 8$ |
| `(bin-cl below a b order)` | `lt` if $a < b$, `gt` if $a > b$, else `below` | $3 \cdot 2 \cdot 2 = 12$ |
| `(bin-nz 1)` | one is not zero | $1$ |

```lisp
(import std.math.bits :symbols (bin-fa))

(query :goal (bin-fa 1 1 1 ?bit ?carry))
```

<!-- output -->
```text
  query             (bin-fa 1 1 1 ?bit ?carry)
  over              all of R — 1 state → 1 projection class
    class 1 of 1    1 state (1)
                      ?bit = 1 · ?carry = 1
  as :expect        (model (bin-fa 1 1 1 1 1))
```

One and one and a carry of one is one, carry one.

## Using `std.math.convert` — decimal to binary and back

A module of its own over both bases
([Q-M1f.323](../../../../decisions/stdlib/math/Q-M1f.323-conversion-between-the-bases-is-a-module.md)), so that a
program that computes in one base loads nothing of the other.

| a program writes | the theory answers | meaning |
|---|---|---|
| `(to-bin X)` | `(bin-of X B)` | B is X, a decimal number, in binary, in normal form |
| `(to-dec B)` | `(dec-of B X)` | X is B, a binary number, in decimal, in normal form |

```lisp
(import std.math.convert :symbols (convert to-bin to-dec bin-of dec-of))
(import std.math.decimal :symbols (n))
(import std.math.binary :symbols (b2))
(import std.math.arith :symbols (z))

(to-bin (n (n z 1) 3))                                    ; 13
(to-dec (b2 (b2 (b2 (b2 z 1) 0) 1) 0))                    ; 1010

(query :goal (bin-of ?x ?b))
(query :goal (dec-of ?b ?x))
```

<!-- output -->
```text
  query 1 of 2      (bin-of ?x ?b)
  over              all of R — 1 state → 1 projection class
    class 1 of 1    1 state (1)
                      ?b = (b2 (b2 (b2 (b2 z 1) 1) 0) 1) · ?x = (n (n z 1) 3)
  as :expect        (model (bin-of (n (n z 1) 3) (b2 (b2 (b2 (b2 z 1) 1) 0) 1)))

  query 2 of 2      (dec-of ?b ?x)
  over              all of R — 1 state → 1 projection class
    class 1 of 1    1 state (1)
                      ?b = (b2 (b2 (b2 (b2 z 1) 0) 1) 0) · ?x = (n (n z 1) 0)
  as :expect        (model (dec-of (b2 (b2 (b2 (b2 z 1) 0) 1) 0) (n (n z 1) 0)))
```

13 is 1101, and 1010 is 10. Zero is `z` in both bases, and a high zero is read by value:

```lisp
(import std.math.convert :symbols (to-bin to-dec bin-of dec-of))
(import std.math.decimal :symbols (n))
(import std.math.binary :symbols (b2))
(import std.math.arith :symbols (z))

(to-bin z)
(to-bin (n (n z 0) 2))                                    ; 02
(to-dec (b2 (b2 z 0) 0))                                  ; 00

(query :goal (bin-of ?x ?b))
(query :goal (dec-of ?b ?x))
```

<!-- output -->
```text
  query 1 of 2      (bin-of ?x ?b)
  over              all of R — 1 state → 1 projection class
    class 1 of 1    1 state (1)
                      ?b = (b2 (b2 z 1) 0) · ?x = (n (n z 0) 2)
                      ?b = z · ?x = z
  as :expect        (model (bin-of (n (n z 0) 2) (b2 (b2 z 1) 0)) (bin-of z z))

  query 2 of 2      (dec-of ?b ?x)
  over              all of R — 1 state → 1 projection class
    class 1 of 1    1 state (1)
                      ?b = (b2 (b2 z 0) 0) · ?x = z
  as :expect        (model (dec-of (b2 (b2 z 0) 0) z))
```

An operand is a number; a name is not looked up, so a request that names its operand is answered by silence:

```lisp
(import std.math.convert :symbols (to-bin bin-of))
(import std.math.decimal-arith :symbols (let))
(import std.math.decimal :symbols (n))
(import std.math.arith :symbols (z))

(let thirteen (n (n z 1) 3))
(to-bin thirteen)

(query :goal (bin-of ?x ?b))
```

<!-- output -->
```text
  ∅                 in R — root is alive and owes nothing
  …
  query             (bin-of ?x ?b)
  over              all of R — 1 state → 1 projection class
    class 1 of 1    1 state (1) — the goal has no row: undetermined here
  as :expect        (model)
```

## How it works

Binary's walks, requests, operands and faults are [the rules every base shares](10-positional-rules.md),
connected by one call of the macro `positional` in `binary-arith`'s closing rule, with binary's relations and
tables in place of decimal's. What is binary's own is small. Binary has no digit multiplier and no bisection: a
row of a product is nothing or the multiplicand, and one subtraction decides a bit of a quotient.

$$
\begin{aligned}
x \cdot y &= \sum_{i} y_i \cdot 2^i \cdot x \\
y_i &\in \lbrace 0, 1 \rbrace
\end{aligned}
$$

The conversions are circuits over the two bases' tagged calls. `to-bin` halves X by decimal's tagged division
until nothing is left, each remainder a bit; `to-dec` reads B's bits from the top and doubles the number so far by
one addition walk, the bit as its carry in:

$$
\operatorname{val}(b_k \dots b_0) = (\dots((b_k \cdot 2 + b_{k-1}) \cdot 2 + b_{k-2}) \dots) \cdot 2 + b_0
$$

Every rule of the three modules is on [the next page](15-binary-rules.md).
