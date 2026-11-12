# Binary arithmetic: the prototype, and three ways to have it

**Stage:** [S1f.6.16](s1f.6.16_binary_arithmetic.md), task T1. **Phase:** [P1f.6](README.md).
**Status:** prototype, 2026-11-11. Nothing here ships; it is shown to the user, who rules. **Way (b) banked as
examples 2026-11-12**, [`examples/binary_arithmetic/`](../../../examples/binary_arithmetic/README.md); the
questions stay open.

The user, 2026-11-11: *"arithmetic with binary numbers, I suspect simpler rules"*.

**The answer, measured:** the suspicion is right for two operations of five. Addition, subtraction and
comparison keep **every rule they have**, unchanged to the letter: nothing in those rules says *ten*, and the
shipped `std.arith` computes them in binary from binary tables without an edit. Multiplication loses its digit
multiplier (a row is nothing or the multiplicand) and division loses its nine multiples and its bisection (one
subtraction decides a quotient bit): 36 division rules become 17. The tables shrink from 1 664 rows to 28. What
gets worse is the length of everything: a number is 3.3 times longer, so an addition is 2.5 times the firings
and the text of a program three times the parse.

Every number on this page is from a run of 2026-11-11; the command beside it re-takes it. The prototype is
under `build/s1f.6.16/` (gitignored): `python3 build.py` writes the three theories from the shipped one,
`python3 measure.py`, `bench.py`, `counts.py` and `conv/run.py` take the measurements, and the raw results are
`out/measure.json`, `out/bench.json`, `out/counts.json`.

**Where it is now (2026-11-12).** The recommended way, (b), is banked as examples, on the user's instruction
of that day, *"bank binary arithmetic and logic prototypes into examples too"*:
[`examples/binary_arithmetic/`](../../../examples/binary_arithmetic/README.md). The tables are
[`bits.ein`](../../../examples/binary_arithmetic/bits.ein), the theory
[`bin.ein`](../../../examples/binary_arithmetic/bin.ein), both written from the shipped `stdlib/arith.ein` by
[`gen_binary.py`](../../../examples/binary_arithmetic/gen_binary.py), which is this prototype's `build.py`
cut down to (b); the conversions are [`conv.ein`](../../../examples/binary_arithmetic/conv.ein); and five
programs carry claims, `first`, `five_operations`, `div_by_zero`, `both_bases` and `conversions`. They are
examples and not a stdlib module, and the generator is run by hand, not in the gate. Ways (a) and (c), the
batches and the long conversions are not banked: they stay here and under `build/s1f.6.16/`. **The banking
is not a ruling: every question at the foot of this page stays open.** The banked files were run on the
release binary and the stdlib of 2026-11-12, and the answers and the counts this page reports for (b) stand:
67 rules and 28 rows (`python3 examples/binary_arithmetic/gen_binary.py --count`); 43, 61, 120, 245, 24 and 10
firings for the six targets, 826 and 76 for 255 to binary and back, 2 317 and 202 for 123078; the 243 sampled
requests right against Python, 243 of 243, and 60 conversions of 60 (`python3
build/s1f.6.16-17/chk_arith.py`, 2026-11-12). `bin.ein` as banked differs from the prototype's `b/bin.ein`
in its header comment and in one word of its closing rule's `:why`.

**What was run.** The release binary as built at 19:17 on 2026-11-11, copied to `build/s1f.6.16/tools/ein`
(sha256 `bfaa78d8855a7027…`, `ein 0.1.0`), against the stdlib of `a621dc75` (`git archive HEAD stdlib`), so
that another agent's work in the tree could not move a number. `examples/arith/div.ein` of that commit answers
on it, 5105 remainder 20.

## The three ways

| | what it is | a number | a request | how it was built |
|---|---|---|---|---|
| **(a)** | the same shape and the same names; the tables binary, in the module `std.arith` already imports | `(n (n z 1) 0)` | `(add X Y)` | `std.arith`'s rules of addition, subtraction, comparison and multiplication **verbatim**, and a division of base two |
| **(b)** | a theory of its own: its own shape, every name its own | `(b2 (b2 z 1) 0)` | `(add2 X Y)` | (a), with multiplication and the strip rule simplified for two digits, and every relation and rule renamed |
| **(c)** | one generic theory; every request, state and table row carries its base | `(n (n z 1) 0)` | `(add 2 X Y)` | `std.arith`'s rules with a base argument added mechanically, and a division that works in any base |

Zero is `z` in all three, and the bits are the names `0` and `1`, which decimal's digits already are.

The tables are the generator's own logic with the base a parameter (`lib.py`'s `tables(base)`, a copy of
[`utils/gen_arith_tables.py`](../../../utils/gen_arith_tables.py)'s): at base 10 it writes the shipped 1 664
rows, at base 2 these, all of them:

```lisp
(fa 0 0 0 0 0) (fa 0 1 0 1 0) (fa 0 0 1 1 0) (fa 0 1 1 0 1) (fa 1 0 0 1 0) (fa 1 1 0 0 1) (fa 1 0 1 0 1) (fa 1 1 1 1 1)
(fs 0 0 0 0 0) (fs 0 1 0 1 1) (fs 0 0 1 1 1) (fs 0 1 1 0 1) (fs 1 0 0 1 0) (fs 1 1 0 0 0) (fs 1 0 1 0 0) (fs 1 1 1 1 1)
(md 0 0 0 0 0) (md 0 1 0 0 0) (md 1 0 0 0 0) (md 1 1 0 1 0)
(cl lt 0 0 lt) (cl lt 0 1 lt) (cl lt 1 0 gt) (cl lt 1 1 lt) (cl eq 0 0 eq) (cl eq 0 1 lt) (cl eq 1 0 gt) (cl eq 1 1 eq)
(cl gt 0 0 gt) (cl gt 0 1 lt) (cl gt 1 0 gt) (cl gt 1 1 gt)
(nz 1)
(mid 0 10 1)
(adj 0 1) (adj 1 10)
```

A table of base B has 2B² rows of `fa` and of `fs`, B²(B − 1) of `md`, 3B² of `cl`, B − 1 of `nz`, B(B − 1)/2
of `mid` and B of `adj`. The base's own name is `10` in every base, which is what `mid` and `adj` call their
upper bound.

## First: does the shipped wiring work unchanged?

**For four operations of five, yes.** `build/s1f.6.16/a0/stdlib/` is a copy of the stdlib in which
`arith.ein` is the shipped file byte for byte (`cmp` says so) and `digits.ein` is the eight lines above under
the shipped names. Programs import `std.arith` as they do today.

| request, in binary | answer | right |
|---|---|---|
| 123078 + 4567 | 127645 | yes |
| 123078 − 4567 | 118511, short by 0 | yes |
| 4567 − 123078 | 0, short by 118511 | yes |
| 123 × 456 | 56088 | yes |
| 123078 against 4567 | `gt` | yes |
| 127645 ÷ 0 | the flag, then `(false)`; R empty | yes |
| **127645 ÷ 25** | **no answer** | **no** |

(`python3 measure.py a0`; answers read back from `ein saturate --dump` and compared with Python's integers.)
On a sample of 243 requests, 196 are right and the 47 divisions are not answered.

**The division is answered by silence.** It computes the divisor times 1, then asks for the divisor times 2,
`(mul-walk (div-m2 …) … 2 0 z)`, and no row of the binary `md` has a `2`: the walk stops after 20 firings.
No `quot-of`, no flag, k_s 1, exit 0. It is the calculator's standing weakness, a request no rule completes
says nothing ([E-2](findings.md)'s family), met here on a whole operation.

**What in `std.arith` is decimal,** then, is exactly this and nothing else:

- the eight rules `div-multiple-2` to `div-multiple-9` and `div-turn`, which reads the ninth multiple;
- the nine rules `div-try-1` to `div-try-9`, alike but for the digit;
- the relations `multiples`, of nine arguments, and `div-m2` to `div-m9`;
- the tables `mid` and `adj`, the bisection of 0 to 10, and the constants `0 10` in `div-bring` and
  `div-bring-first`.

Every other rule is generic in its base. `nz` and `md` are read as tables, and a binary table answers. The
carry out of an addition is matched as `1`, which it is in every base. *Ten times the sum so far* in
`mul-add-row` is the constructor `(n SUM 0)`, which is *the base times the sum* whatever the base is. So
**decimal lives in the tables and in the division's bisection, and nowhere else.**

## The three theories

### (a) The same names, and a division of base two

`a1/stdlib/arith.ein`: 59 of the shipped 88 rules verbatim (`python3 counts.py` compares the forms), the
bisection gone, and ten new rules. One subtraction decides a bit:

```lisp
(rule div-bring ()
  :match  (div-step ?k ?d (b ?a ?rest) (n ?p ?c) ?w)
  :assert (sub-walk (div-trial ?k ?d ?rest ?w (n (n ?p ?c) ?a)) (n (n ?p ?c) ?a) ?d 0 z)
  :why    "bring down {?a} beside what is left over, and try to take the divisor from it")
(rule div-fits ()
  :match  (done (div-trial ?k ?d ?rest ?w ?u) ?t)
  :assert (div-step ?k ?d ?rest ?t (n ?w 1))
  :why    "the divisor fits, and leaves {?t}: the next digit of the quotient is 1")
(rule div-too-big ()
  :match  (short (div-trial ?k ?d ?rest (n ?p ?c) ?u))
  :assert (div-step ?k ?d ?rest ?u (n (n ?p ?c) 0))
  :why    "the divisor does not fit: the next digit of the quotient is 0")
```

The state carries the divisor where decimal's carries nine multiples. The other new rules are the ones that
keep a high zero from being written. Division by zero is the shipped pair of rules, unchanged.

**A program cannot hold both bases this way, imported flat.** Two modules that declare `fa` are two
definitions of one name, and the loader refuses the first relation that differs:

```text
kb load error: conflicting definitions of 'div-digits': same name, different body (at None)
```

(`build/s1f.6.16/q/both_flat.ein`.) **Imported qualified, it can**, and the shape separates by itself:

```lisp
(import std.arith :as d)
(import arith2 :as t)
(d.add (d.n (d.n (d.n (d.n (d.n (d.n z 1) 2) 3) 0) 7) 8) (d.n (d.n (d.n (d.n z 4) 5) 6) 7))
(t.add (t.n (t.n (t.n z 1) 0) 1) (t.n (t.n (t.n z 1) 1) 1))
```

```text
  query 1 of 2      (d.sum-of ?x ?y ?s)
                      ?s = (d.n (d.n (d.n (d.n (d.n (d.n z 1) 2) 7) 6) 4) 5) · ?x = (d.n … 8) · ?y = (d.n … 7)
  query 2 of 2      (t.sum-of ?x ?y ?s)
                      ?s = (t.n (t.n (t.n (t.n z 1) 1) 0) 0) · ?x = (t.n (t.n (t.n z 1) 0) 1) · ?y = (t.n (t.n (t.n z 1) 1) 1)
```

(`q/both_as.ein`, `out/q_both_as.txt`; trimmed.) A qualified import prefixes the relations and leaves the
digits and `z` shared, so `(t.n …)` and `(d.n …)` are two shapes from one text. That is (b)'s separation with
no second set of names, at the price of a prefix on every number a program writes.

### (b) A theory of its own

`b/bin.ein` and `b/bits.ein`: 67 rules, three tables, 28 rows. Beyond (a):

- **No digit multiplier.** A row of a binary multiplication is nothing or the multiplicand, so `md`, the
  `mul-walk` and its three rules go, and a step is one of four rules:

  ```lisp
  (rule mul-one2 ()
    :match  (mul-step2 ?k ?u (a2 1 ?rest) (b2 ?p ?a))
    :assert (add-walk2 (mul-sum2 ?k ?u ?rest) (b2 (b2 ?p ?a) 0) ?u 0 z)
    :why    "the next digit of the multiplier is 1: twice the sum so far, plus {?u}")
  (rule mul-zero2 ()
    :match  (mul-step2 ?k ?u (a2 0 ?rest) (b2 ?p ?a))
    :assert (mul-step2 ?k ?u ?rest (b2 (b2 ?p ?a) 0))
    :why    "the next digit of the multiplier is 0: twice the sum so far")
  ```

- **No `nz`.** The digit that is not zero is `1`, written in the rule.
- **The divisor's normal form** is zero plus the divisor, one addition walk, where decimal multiplies by 1.

A program may import it beside `std.arith`, flat (`q/both_b.ein`):

```text
  query 1 of 2      (sum-of ?x ?y ?s)
                      ?s = (n (n (n (n (n (n z 1) 2) 7) 6) 4) 5) · ?x = (n (n (n (n (n (n z 1) 2) 3) 0) 7) 8) · ?y = (n (n (n (n z 4) 5) 6) 7)
  query 2 of 2      (sum-of2 ?x ?y ?s)
                      ?s = (b2 (b2 (b2 (b2 z 1) 1) 0) 0) · ?x = (b2 (b2 (b2 z 1) 0) 1) · ?y = (b2 (b2 (b2 z 1) 1) 1)
```

The names of the prototype are the shipped ones with a `2` (`add2`, `sum-of2`, the accumulator `a2`); they
are placeholders, and the user's to choose.

### (c) One generic theory

`c/garith.ein` and `c/gdigits.ein`: 72 rules; the tables of both bases in one module, the base the last
column of a row (last, so that a lookup is still keyed on a digit: [E-12](findings.md)).

```lisp
(div 2  (n (n (n (n (n z 1) 1) 0) 0) 1) (n (n z 1) 1))
(div 10 (n (n (n (n (n (n z 1) 2) 7) 6) 4) 5) (n (n z 2) 5))
```

```text
  query 1 of 2      (quot-of ?g ?x ?y ?q ?r)
                      ?g = 10 · ?q = (n (n (n (n z 5) 1) 0) 5) · ?r = (n (n z 2) 0) · ?x = (n (n (n (n (n (n z 1) 2) 7) 6) 4) 5) · ?y = (n (n z 2) 5)
                      ?g = 2 · ?q = (n (n (n (n (n (n (n (n (n (n (n (n (n z 1) 0) 0) 1) 1) 1) 1) 1) 1) 0) 0) 0) 1) · ?r = (n (n (n (n (n z 1) 0) 1) 0) 0) · …
```

(`c/p/t.ein`.) The rules of the four other operations are the shipped ones with `?g` added by a script, first
in every state and request and last in every table row. **The division cannot be the shipped one**, whose
shape is ten's, so it is written for any base: the divisor is taken from what is left over until it does not
fit, and a table `succ-more` / `succ-top` counts the times, which is the quotient's digit. Twenty rules for
every base, where decimal's bisection is 36 for one.

## Right, and search-free

**Every answer is right against Python's integers** in (a), (b) and (c), at both of (c)'s bases:

- the seven targets (`python3 measure.py`);
- 243 requests in one program a variant: 40 random of each operation with operands to twelve decimal digits
  (multiplication and division to nine by six), and nine edge pairs each, zero, one, equal operands, 255 and
  256, 999 and 1000: 243 of 243 in each of the five (`out/measure.json`, `samples`);
- 250 requests of each operation with nine-digit operands, and 23 divisions whose quotients are all nines:
  every one right (`python3 bench.py`, `out/bench.json`).

**Every program is search-free:** *universe — no candidate: R is {∅} or empty, decided by root alone*, k_s 1,
one saturation, *monotone (the static check)*, on every target and every batch of every variant. No `absent`,
no `not`, and the rule of the theory holds in all three: each rule has one premise that holds numbers.

The targets under (b), `ein solve b/targets.ein` (`out/b_targets.txt`; the operands trimmed):

```text
  states (k_s)      1
  universe          no candidate: R is {∅} or empty, decided by root alone
  work              1 saturation · 0 facts beyond root · 2 layers

  query 1 of 5      (sum-of2 ?x ?y ?s)
                      ?s = (b2 (b2 (b2 (b2 (b2 (b2 (b2 (b2 (b2 (b2 (b2 (b2 (b2 (b2 (b2 (b2 (b2 z 1) 1) 1) 1) 1) 0) 0) 1) 0) 1) 0) 0) 1) 1) 1) 0) 1) · …
  query 2 of 5      (diff-of2 ?x ?y ?d ?r)
                      ?d = (b2 (b2 (b2 (b2 (b2 (b2 (b2 (b2 (b2 (b2 (b2 (b2 (b2 (b2 (b2 (b2 (b2 z 1) 1) 1) 0) 0) 1) 1) 1) 0) 1) 1) 1) 0) 1) 1) 1) 1) · ?r = z · …
                      ?d = z · ?r = (b2 (b2 (b2 (b2 (b2 (b2 (b2 (b2 (b2 (b2 (b2 (b2 (b2 (b2 (b2 (b2 (b2 z 1) 1) 1) 0) 0) 1) 1) 1) 0) 1) 1) 1) 0) 1) 1) 1) 1) · …
  query 3 of 5      (product-of2 ?x ?y ?p)
                      ?p = (b2 (b2 (b2 (b2 (b2 (b2 (b2 (b2 (b2 (b2 (b2 (b2 (b2 (b2 (b2 (b2 z 1) 1) 0) 1) 1) 0) 1) 1) 0) 0) 0) 1) 1) 0) 0) 0) · …
  query 4 of 5      (quot-of2 ?x ?y ?q ?r)
                      ?q = (b2 (b2 (b2 (b2 (b2 (b2 (b2 (b2 (b2 (b2 (b2 (b2 (b2 z 1) 0) 0) 1) 1) 1) 1) 1) 1) 0) 0) 0) 1) · ?r = (b2 (b2 (b2 (b2 (b2 z 1) 0) 1) 0) 0) · …
  query 5 of 5      (cmp-of2 ?x ?y ?o)
                      ?o = gt · …
```

11111001010011101 is 127645, 11100111011101111 is 118511, 1101101100011000 is 56088, 1001111110001 is 5105
and 10100 is 20. **A division by zero** (`b/dz.ein`, `ein solve` and `ein test`):

```text
  states (k_s)      0
  empty             every node is dead — root's own fixpoint derives (false)
  universe          no candidate: R is {∅} or empty, decided by root alone
  unsat core        (div2 (b2 (b2 … 1) z)
  refuted by        division-by-zero-is-false2
  (solve :expect)   holds
```

## The counts, side by side

`python3 counts.py`; the closing rule is not counted.

| | decimal, shipped | (a) | (b) | (c), both bases |
|---|---:|---:|---:|---:|
| rules | 88 | 69 | 67 | 72 |
| of them: operands, strip, turn | 9 | 9 | 9 | 9 |
| addition | 9 | 9 | 9 | 9 |
| subtraction | 10 | 10 | 10 | 10 |
| comparison | 8 | 8 | 8 | 8 |
| multiplication | 16 | 16 | 14 | 16 |
| division | 36 | 17 | 17 | 20 |
| rules verbatim from `std.arith` | 88 | 59 | 51, up to the names | 0: each has a base |
| rules with a second premise | 25 | 13 | 10 | 17 |
| relations | 66 | 55 | 53 | 57 |
| tables | 7 | 5 | 3 | 7 |
| table rows | 1 664 | 33 | 28 | 1 652 |
| `fa` · `fs` · `md` · `cl` · `nz` · `mid` · `adj` | 200 · 200 · 900 · 300 · 9 · 45 · 10 | 8 · 8 · 4 · 12 · 1 · 0 · 0 | 8 · 8 · 0 · 12 · 0 · 0 · 0 | 208 · 208 · 904 · 312 · 10, and `succ` 10 |
| the tables' text | 28 800 bytes | | 880 bytes | |
| facts before saturation, one request | 1 814 | 157 | 144 | 1 784 |

## Firings, depth and time, side by side

**One request.** Firings / depth; the closing rule's firing is not counted. Depth is the longest chain of
firings (`measure.py`'s reading of `ein saturate --events`), so it is the size where the computation is one
chain and less where subtraction runs its two walks side by side.

| request | decimal | (a0) shipped rules | (a) | (b) | (c) base 2 | (c) base 10 |
|---|---:|---:|---:|---:|---:|---:|
| 123078 + 4567 | 21 / 21 | 43 / 43 | 43 / 43 | 43 / 43 | 43 / 43 | 21 / 21 |
| 123078 − 4567 | 28 / 21 | 61 / 43 | 61 / 43 | 61 / 43 | 61 / 43 | 28 / 21 |
| 4567 − 123078 | 28 / 21 | 61 / 43 | 61 / 43 | 61 / 43 | 61 / 43 | 28 / 21 |
| 123 × 456 | 72 / 72 | 415 / 415 | 415 / 415 | 120 / 120 | 415 / 415 | 72 / 72 |
| 127645 ÷ 25 | 220 / 220 | 20, no answer | 245 / 245 | 245 / 245 | 279 / 279 | 173 / 173 |
| 123078 against 4567 | 13 / 13 | 24 / 24 | 24 / 24 | 24 / 24 | 24 / 24 | 13 / 13 |
| 127645 ÷ 0 | 10 / 10 | 10 / 10 | 10 / 10 | 10 / 10 | 10 / 10 | 10 / 10 |

Time, the saturation alone / the whole `ein solve` process, in ms, best of seven:

| request | decimal | (a) | (b) | (c) base 2 | (c) base 10 |
|---|---:|---:|---:|---:|---:|
| 123078 + 4567 | 0.30 / 27.6 | 0.27 / 9.8 | 0.22 / 10.9 | 0.39 / 45.6 | 0.24 / 25.7 |
| 123 × 456 | 0.39 / 25.7 | 0.64 / 15.0 | 0.34 / 9.8 | 0.82 / 31.2 | 0.33 / 26.6 |
| 127645 ÷ 25 | 0.61 / 26.2 | 0.50 / 11.5 | 0.48 / 11.5 | 0.69 / 29.6 | 0.46 / 26.1 |

**A small program is its load, and binary's load is a quarter of decimal's.** Parse, load and compile take
26.8 ms with the decimal tables, 8.0 ms under (a) and 5.8 ms under (b); under (c), which holds both bases'
rows, 26 to 41 ms. The calculation itself is under a millisecond in every column
([E-15](findings.md), [S1f.6.15](s1f.6.15_load_against_calculation.md)).

**A batch,** 250 requests of one operation in one program, the operands of nine decimal digits, which is
thirty bits; the multiplier and the divisor of four digits. Firings a request · the saturation
(`python3 bench.py`):

| | decimal | (a) | (b) | (c) base 2 | (c) base 10 |
|---|---:|---:|---:|---:|---:|
| addition | 27.0 · 9 ms | 68.4 · 21 ms | 68.4 · 26 ms | 68.4 · 24 ms | 27.0 · 10 ms |
| subtraction | 37.0 · 13 ms | 99.1 · 32 ms | 99.1 · 37 ms | 99.1 · 42 ms | 37.0 · 15 ms |
| comparison | 16.0 · 7 ms | 36.8 · 12 ms | 36.8 · 11 ms | 36.8 · 20 ms | 16.0 · 7 ms |
| multiplication | 188.3 · 69 ms | 1 726.3 · 671 ms | 524.9 · 231 ms | 1 726.3 · 802 ms | 188.3 · 81 ms |
| division | 443.7 · 188 ms | 669.9 · 273 ms | 669.9 · 295 ms | 728.6 · 312 ms | 436.8 · 167 ms |
| division, quotients all nines (23 requests) | 272.7 | 200.8 | 200.8 | 242.0 | 381.6 |
| parsing the 250 additions | 19 ms | 55 ms | 60 ms | 55 ms | 21 ms |

## What got simpler, what did not, what got worse

**Simpler:**

- **Division, by half its rules.** 36 rules to 17. No multiples, no bisection, no `mid`, no `adj`: a
  quotient's bit is one subtraction, which fits or is short. The state carries one number where it carried
  nine.
- **Multiplication, a little in rules and a lot in work.** 16 rules to 14 and no `md`. The saving is not the
  two rules: with the generic wiring a binary row is a walk through `md` that copies the multiplicand or
  writes zeros, 1 726 firings a product; without it, 525.
- **The tables.** 1 664 rows to 28, seven tables to three, 28 800 bytes to 880. A program loads in a quarter
  of the time.
- **The rules with a second premise.** 25 to 10. What is left is the nine column rules, each with its row of
  `fa`, `fs` or `cl`, and `operand-name`; every other rule matches one fact and writes the next.

**Not simpler at all:**

- **Addition, subtraction and comparison.** The same 27 rules and the 9 shared ones, letter for letter. A
  column is a column; only its table is smaller.
- **The protocol, the operands, the normal form, the turn.** None of it knows the base.
- **Division by zero.** The same two rules.

**Worse:**

- **Length.** Nine decimal digits are thirty bits. An addition is 68 firings where it was 27 (2.5 times),
  a comparison 37 where 16, a subtraction 99 where 37.
- **Multiplication and division, in work,** although their rules are simpler: 525 firings a product where
  decimal takes 188 (2.8 times), 670 a quotient where 444 (1.5 times). Fewer steps a digit, three times the
  digits, and each step an addition or a subtraction three times as long.
- **The text.** A batch of binary requests takes three times as long to parse, and the parse is already the
  larger half of such a run ([E-16](findings.md)). A read-out line of `(b2 (b2 (b2 …` is unreadable sooner
  ([E-14](findings.md)).

**A by-product about decimal.** (c)'s division, which bisects nothing and computes no multiple, is 20 rules
for every base, and at base 10 it is not slower than the shipped 36 on ordinary operands: 436.8 firings a
quotient against 443.7 on the batch, 173 against 220 on 127645 ÷ 25. It loses where the quotient's digits are
large, 381.6 against 272.7 on quotients of nines, since a 9 costs it nine subtractions and the bisection
four. It is recorded here and is not this stage's to act on.

## Conversion between decimal and binary

As requests, under (b) beside the shipped `std.arith`: `build/s1f.6.16/conv/conv.ein`, eleven rules, each a
chain of the two theories' own operations.

- **Decimal to binary:** `(to-bin X)`. The number is divided by two with decimal's division, `div-call`, until
  nothing is left; each remainder is a bit, put in front of the bits so far, which is the binary theory's
  accumulator; its `strip2` and `turn2` make the number.
- **Binary to decimal:** `(to-dec B)`. The bits from the top, `top-first2`; the number so far is doubled and
  the bit added by **one** decimal addition walk, the number plus itself with the bit as the carry in.

```lisp
(import conv :symbols (conv))
(to-bin (n (n (n z 2) 5) 5))
(to-dec (b2 (b2 (b2 (b2 z 1) 0) 1) 0))
```

`python3 conv/run.py`, ten numbers each way, read back and compared with Python (`out/conv_run.txt`,
`out/conv_solve.txt`):

```text
bin-of 0 → 0 right                  dec-of 0 ← 0 right
bin-of 1 → 1 right                  dec-of 1 ← 1 right
bin-of 2 → 10 right                 dec-of 2 ← 10 right
bin-of 10 → 1010 right              dec-of 10 ← 1010 right
bin-of 255 → 11111111 right         dec-of 255 ← 11111111 right
bin-of 256 → 100000000 right        dec-of 256 ← 100000000 right
bin-of 1000 → 1111101000 right      dec-of 1000 ← 1111101000 right
bin-of 123078 → 11110000011000110 right
bin-of 127645 → 11111001010011101 right
bin-of 1099511627777 → 10000000000000000000000000000000000000001 right
20 of 20 right
```

and as `ein solve` prints one, with its `:goal-text`:

```text
  universe          no candidate: R is {∅} or empty, decided by root alone
                      ?b = (b2 (b2 (b2 (b2 (b2 (b2 (b2 (b2 (b2 (b2 z 1) 1) 1) 1) 1) 0) 1) 0) 0) 0) · ?x = (n (n (n (n z 1) 0) 0) 0)
                        “(n (n (n (n z 1) 0) 0) 0) is (b2 (b2 (b2 (b2 (b2 (b2 (b2 (b2 (b2 (b2 z 1) 1) 1) 1) 1) 0) 1) 0) 0) 0) in binary”
```

Search-free, and no rule joins two facts on a number. **The two directions do not cost the same:** 255 to
binary is 826 firings and back is 76; 123078 is 2 317 and 202. A halving is a whole decimal division, with
its nine multiples of two computed again each time, where a doubling is one addition.

**What the conversion says about the three ways.** It needs both bases in one program, and a result that
says which base it is in. Under (b) the shapes say it. Under (a) it needs the qualified imports above. Under
(c) the two numbers have one shape and only the relation, `bin-of`, says which is which.

**It reaches into both theories:** `div-call`, `add-walk`, `done`, `strip2`, `top-first2` are the circuits'
own facts, which `std.arith`'s header says are *not a promise*. A conversion through the public requests
alone would join a state with `(quot-of …)` on a number, which the rule of the theory forbids
([E-3](findings.md)). So a theory built on the calculator needs the tagged calls to be an interface. That is
the stage's first finding as *the first use of the calculator by a theory*.

## What did not work, and what the engine said

1. **The shipped division over binary tables: silence** (above; `a0/t_div_127645_25.ein`).
2. **Two bases imported flat under (a): refused**, and the line names no place: `(at None)`.
3. **A module that builds on another, imported by its own entry alone, is half a theory, in silence.**
   `(import conv :symbols (conv))` first brought `conv`'s eleven rules, the *declarations* of the relations
   they write, `div-call` among them, and none of `std.arith`'s rules: a flat selection follows the names a
   kept declaration writes, and nothing in `conv` wrote the name `arith`. Every request loaded and none was
   answered. The repair is one fact in `conv`'s closing rule, `(arith-rule arith)`, which names the other
   theory's entry. It is [E-20](findings.md), which [S1f.6.20](s1f.6.20_word_problems.md) filed the same
   day from its own theory; this is a second witness.
4. **That half-theory then met a limit that is an exit 1.** With the theories' relations unclosed there were
   792 candidates, and `ein solve` printed one line to stderr and nothing to stdout:

   ```text
   a state's commitments pass 2^128: the state route cannot count them
   ```

   exit 1; `ein solve --count` on it had printed nothing after 20 s; `--minimal` answers in a moment
   (`build/s1f.6.16/out/witness_2p128/only_conv.ein`). The user's rule is that every answer exits 0.

2 is row E-21 of [`findings.md`](findings.md), 3 is E-20 with a note there, and 4 is E-22.

## The recommendation

**(b), a theory of its own, with (a)'s finding written into it.**

- It is the smallest and the cheapest of the three: 67 rules, 28 rows, a quarter of decimal's load, and the
  only one whose multiplication is not three times dearer than it need be.
- It can stand beside decimal in one program with a plain import, which conversion needs and
  [S1f.6.17](s1f.6.17_binary_logic.md)'s gates will want: their bits are the same two names.
- A number says which base it is in.
- What it costs is a second copy of 51 rules that differ from `std.arith`'s only in their names. The stage
  would generate that copy from `std.arith` and hold it there by a test, as this prototype's `build.py` does,
  so that the two cannot drift.

**Not (a) alone:** it keeps the generic multiplication, 3.3 times (b)'s firings, and a program must choose
its base for the whole of itself or prefix every number. Its finding stays whatever is chosen: *addition,
subtraction and comparison are base-free*, and a test can hold that by running the shipped rules over binary
tables, as `a0` does.

**Not (c):** it changes every request the user has ruled on, `(add X Y)` to `(add 10 X Y)`, makes every state
fact one argument wider, and loads both tables into every program, for a generality (*any* base) the user set
aside on 2026-11-11, *"skip different bases"*.

## Questions for the user

1. **Which of the three?** The recommendation is (b).
2. **The names under (b).** The shape `(b2 PREFIX BIT)`, the requests `add2` … `div2`, the results `sum-of2`
   … ; or another convention (`bin-add`, `badd`, a module prefix). The prototype's are placeholders.
3. **Is zero the same `z` in both bases?** It is in the prototype, and it is the one number whose numeral
   does not say its base.
4. **May `std.arith` be generated in part,** so that the binary rules that are the same up to their names are
   written once? Or is a hand-kept second file, held equal by a test, preferred?
5. **The conversions:** in the binary module, or a module of their own; and are the tagged calls they use
   (`div-call`, `add-walk` with a tag, answered by `done`) to become an interface a theory may rely on?
6. **Decimal's division.** The 20-rule division by repeated subtraction is as cheap as the 36-rule bisection
   on ordinary operands and dearer on quotients of large digits. Leave the shipped one, or is the simpler one
   wanted?
7. **Should a request no rule completes say so?** The shipped division over binary tables is the cleanest
   witness yet of a silent non-answer.
