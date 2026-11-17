# The natural numbers' rules

How `std.math.arith`, `std.math.decimal` and `std.math.decimal-arith` work: every rule of the three, as the module
writes it and as the engine draws it. What a program writes to use them is
[the page before](08-natural-numbers.md); how to read a drawing is [the index](README.md#how-a-rule-is-shown).

None of these rules has a parameter: each is active wherever its module is loaded. A rule's `:why` says what one
firing of it does, in the words a trace prints. The addition, the subtraction and the comparison of a column, the
requests and their answers, and the faults of an operand are not here: every positional base does them the same
way, and they are [`std.math.positional`'s](10-positional-rules.md), which `std.math.decimal-arith` connects by
one call of a macro in its closing rule.

## `std.math.arith`

One rule, and the entry. The flag is a fact of `division-by-zero`, which a module that divides derives.

#### `division-by-zero-is-false`

The flag is a contradiction.

<!-- rule std.math.arith division-by-zero-is-false -->
```lisp
(rule division-by-zero-is-false ()
  :match  (division-by-zero ?request)
  :assert (false)
  :why    "{?request} divides by zero: no number is the answer")
```

![The rule division-by-zero-is-false of std.math.arith, as the engine draws it](diagrams/09-std.math.arith.division-by-zero-is-false.svg)
<!-- /rule -->

#### `arith`

The entry: it closes the held extent of the flag's relation and names the module's one rule.

<!-- rule std.math.arith arith -->
```lisp
(rule arith ()
  :match  ()
  :assert (and (__closed__ division-by-zero)
               (__closed__ arith-module-rule)
               (arith-module-rule division-by-zero-is-false))
  :why    "arithmetic is computed and never guessed: its flag is closed")
```

![The rule arith of std.math.arith, as the engine draws it](diagrams/09-std.math.arith.arith.svg)
<!-- /rule -->

## `std.math.decimal`

The numeral's check. The loader states `(written T)` for every fact nested in a fact a file writes, so each rule
has one premise that holds a number, a written numeral, beside a guard or a row of `nz`.

#### `numeral-digit`

The last place of a numeral holds something that is none of the ten digits: one `neq` guard a digit.

<!-- rule std.math.decimal numeral-digit -->
```lisp
(rule numeral-digit ()
  :match  (and (written (n ?p ?a))
               (neq ?a 0) (neq ?a 1) (neq ?a 2) (neq ?a 3) (neq ?a 4)
               (neq ?a 5) (neq ?a 6) (neq ?a 7) (neq ?a 8) (neq ?a 9))
  :assert (not-decimal (n ?p ?a) ?a)
  :why    "{?a} stands where a digit goes and is none of the ten")
```

![The rule numeral-digit of std.math.decimal, as the engine draws it](diagrams/09-std.math.decimal.numeral-digit.svg)
<!-- /rule -->

#### `numeral-prefix-digit`

The prefix is a digit that is not zero, `(n 1 2)`.

<!-- rule std.math.decimal numeral-prefix-digit -->
```lisp
(rule numeral-prefix-digit ()
  :match  (and (written (n ?m ?a)) (nz ?m))
  :assert (not-decimal (n ?m ?a) ?m)
  :why    "the digit {?m} stands where a number goes: a number ends in z")
```

![The rule numeral-prefix-digit of std.math.decimal, as the engine draws it](diagrams/09-std.math.decimal.numeral-prefix-digit.svg)
<!-- /rule -->

#### `numeral-prefix-zero`

The prefix is the digit `0`, `(n 0 2)`.

<!-- rule std.math.decimal numeral-prefix-zero -->
```lisp
(rule numeral-prefix-zero ()
  :match  (written (n 0 ?a))
  :assert (not-decimal (n 0 ?a) 0)
  :why    "the digit 0 stands where a number goes: a number ends in z")
```

![The rule numeral-prefix-zero of std.math.decimal, as the engine draws it](diagrams/09-std.math.decimal.numeral-prefix-zero.svg)
<!-- /rule -->

#### `numeral-prefix-named`

The prefix is a named constant, `(n price 1)`.

<!-- rule std.math.decimal numeral-prefix-named -->
```lisp
(rule numeral-prefix-named ()
  :match  (and (written (n ?m ?a)) (neq ?m z) (named ?m))
  :assert (not-decimal (n ?m ?a) ?m)
  :why    "the name {?m} stands where a number goes: a name stands for a number as an operand, not inside one")
```

![The rule numeral-prefix-named of std.math.decimal, as the engine draws it](diagrams/09-std.math.decimal.numeral-prefix-named.svg)
<!-- /rule -->

#### `not-decimal-is-false`

The flag is a contradiction.

<!-- rule std.math.decimal not-decimal-is-false -->
```lisp
(rule not-decimal-is-false ()
  :match  (not-decimal ?x ?thing)
  :assert (false)
  :why    "{?x} is not a decimal number: the program cannot hold it")
```

![The rule not-decimal-is-false of std.math.decimal, as the engine draws it](diagrams/09-std.math.decimal.not-decimal-is-false.svg)
<!-- /rule -->

#### `decimal`

The entry: it closes the held extent of each of the module's relations, names its rules, and names `arith`.

<!-- rule std.math.decimal decimal -->
```lisp
(rule decimal ()
  :match  ()
  :assert (and (__closed__ nz)
               (__closed__ n) (__closed__ named) (__closed__ not-decimal)
               (__closed__ decimal-rule)
               (decimal-rule arith)
               (decimal-rule numeral-digit) (decimal-rule numeral-prefix-digit)
               (decimal-rule numeral-prefix-zero)
               (decimal-rule numeral-prefix-named)
               (decimal-rule not-decimal-is-false))
  :why    "a numeral is written, and never guessed: every relation of the module is closed")
```

![The rule decimal of std.math.decimal, as the engine draws it](diagrams/09-std.math.decimal.decimal.svg)
<!-- /rule -->

## `std.math.decimal-arith`

What is decimal's own: a product's row by one digit, the multiples of a divisor, and the bisection that finds a
digit of a quotient. Every other step of the five operations is a shared rule.

### Multiplication: a row is the multiplicand times one digit

A product is summed from the multiplier's top digit: the sum so far, times ten, plus the next row. A row is one
walk over the multiplicand's columns through the table `md`,

$$
\begin{aligned}
x_i \cdot m + c_i &= 10 \cdot c_{i+1} + s_i \\
c_0 &= 0
\end{aligned}
$$

with $m$ the multiplier's digit and $c_i$ a carry from `0` to `8`.

#### `mul-column`

<!-- rule std.math.decimal-arith mul-column -->
```lisp
(rule mul-column ()
  :match  (and (mul-walk ?k (n ?p ?a) ?m ?e ?acc)
               (md ?a ?m ?e ?s ?f))
  :assert (mul-walk ?k ?p ?m ?f (b ?s ?acc))
  :why    "{?a} x {?m} + carry {?e} is {?s}, carry {?f}: {?s} goes in front of the digits so far")
```

![The rule mul-column of std.math.decimal-arith, as the engine draws it](diagrams/09-std.math.decimal-arith.mul-column.svg)
<!-- /rule -->

#### `mul-walk-end`

<!-- rule std.math.decimal-arith mul-walk-end -->
```lisp
(rule mul-walk-end ()
  :match  (mul-walk ?k z ?m 0 ?acc)
  :assert (strip ?k ?acc)
  :why    "no column is left and no carry: the digits are {?acc}")
```

![The rule mul-walk-end of std.math.decimal-arith, as the engine draws it](diagrams/09-std.math.decimal-arith.mul-walk-end.svg)
<!-- /rule -->

#### `mul-carry-out`

<!-- rule std.math.decimal-arith mul-carry-out -->
```lisp
(rule mul-carry-out ()
  :match  (and (mul-walk ?k z ?m ?e ?acc) (nz ?e))
  :assert (turn ?k (b ?e ?acc) z)
  :why    "no column is left: the carry {?e} is the top digit, in front of {?acc}")
```

![The rule mul-carry-out of std.math.decimal-arith, as the engine draws it](diagrams/09-std.math.decimal-arith.mul-carry-out.svg)
<!-- /rule -->

#### `mul-row`

<!-- rule std.math.decimal-arith mul-row -->
```lisp
(rule mul-row ()
  :match  (mul-step ?k ?u (b ?a ?rest) ?acc)
  :assert (mul-walk (mul-row-of ?k ?u ?rest ?acc) ?u ?a 0 z)
  :why    "the next digit of the multiplier is {?a}: the row {?u} x {?a}")
```

![The rule mul-row of std.math.decimal-arith, as the engine draws it](diagrams/09-std.math.decimal-arith.mul-row.svg)
<!-- /rule -->

#### `mul-first-row`

<!-- rule std.math.decimal-arith mul-first-row -->
```lisp
(rule mul-first-row ()
  :match  (done (mul-row-of ?k ?u ?rest z) ?row)
  :assert (mul-step ?k ?u ?rest ?row)
  :why    "nothing was summed yet: the row {?row} is the sum so far")
```

![The rule mul-first-row of std.math.decimal-arith, as the engine draws it](diagrams/09-std.math.decimal-arith.mul-first-row.svg)
<!-- /rule -->

#### `mul-add-row`

<!-- rule std.math.decimal-arith mul-add-row -->
```lisp
(rule mul-add-row ()
  :match  (done (mul-row-of ?k ?u ?rest (n ?p ?a)) ?row)
  :assert (add-walk (mul-sum ?k ?u ?rest) (n (n ?p ?a) 0) ?row 0 z)
  :why    "ten times the sum so far, plus the row {?row}")
```

![The rule mul-add-row of std.math.decimal-arith, as the engine draws it](diagrams/09-std.math.decimal-arith.mul-add-row.svg)
<!-- /rule -->

### Division: the divisor's multiples

Long division needs the divisor times one to nine. They are computed once a division, each by one walk of the divisor times a digit,
one after another, and `div-turn` then turns the dividend's digits round so that the top one comes first.

#### `div-multiples`

<!-- rule std.math.decimal-arith div-multiples -->
```lisp
(rule div-multiples ()
  :match  (div-call ?k ?u ?v)
  :assert (mul-walk (div-m1 ?k ?u ?v) ?v 1 0 z)
  :why    "to divide {?u} by {?v}: first {?v} x 1, the divisor in normal form")
```

![The rule div-multiples of std.math.decimal-arith, as the engine draws it](diagrams/09-std.math.decimal-arith.div-multiples.svg)
<!-- /rule -->

#### `div-multiple-2`

<!-- rule std.math.decimal-arith div-multiple-2 -->
```lisp
(rule div-multiple-2 ()
  :match  (done (div-m1 ?k ?u ?v) (n ?p ?a))
  :assert (mul-walk (div-m2 ?k ?u (n ?p ?a)) (n ?p ?a) 2 0 z)
  :why    "the divisor is not zero: next, 2 times the divisor")
```

![The rule div-multiple-2 of std.math.decimal-arith, as the engine draws it](diagrams/09-std.math.decimal-arith.div-multiple-2.svg)
<!-- /rule -->

#### `div-multiple-3`

<!-- rule std.math.decimal-arith div-multiple-3 -->
```lisp
(rule div-multiple-3 ()
  :match  (done (div-m2 ?k ?u ?m1) ?m2)
  :assert (mul-walk (div-m3 ?k ?u ?m1 ?m2) ?m1 3 0 z)
  :why    "2 times the divisor is {?m2}: next, 3 times the divisor")
```

![The rule div-multiple-3 of std.math.decimal-arith, as the engine draws it](diagrams/09-std.math.decimal-arith.div-multiple-3.svg)
<!-- /rule -->

#### `div-multiple-4`

<!-- rule std.math.decimal-arith div-multiple-4 -->
```lisp
(rule div-multiple-4 ()
  :match  (done (div-m3 ?k ?u ?m1 ?m2) ?m3)
  :assert (mul-walk (div-m4 ?k ?u ?m1 ?m2 ?m3) ?m1 4 0 z)
  :why    "3 times the divisor is {?m3}: next, 4 times the divisor")
```

![The rule div-multiple-4 of std.math.decimal-arith, as the engine draws it](diagrams/09-std.math.decimal-arith.div-multiple-4.svg)
<!-- /rule -->

#### `div-multiple-5`

<!-- rule std.math.decimal-arith div-multiple-5 -->
```lisp
(rule div-multiple-5 ()
  :match  (done (div-m4 ?k ?u ?m1 ?m2 ?m3) ?m4)
  :assert (mul-walk (div-m5 ?k ?u ?m1 ?m2 ?m3 ?m4) ?m1 5 0 z)
  :why    "4 times the divisor is {?m4}: next, 5 times the divisor")
```

![The rule div-multiple-5 of std.math.decimal-arith, as the engine draws it](diagrams/09-std.math.decimal-arith.div-multiple-5.svg)
<!-- /rule -->

#### `div-multiple-6`

<!-- rule std.math.decimal-arith div-multiple-6 -->
```lisp
(rule div-multiple-6 ()
  :match  (done (div-m5 ?k ?u ?m1 ?m2 ?m3 ?m4) ?m5)
  :assert (mul-walk (div-m6 ?k ?u ?m1 ?m2 ?m3 ?m4 ?m5) ?m1 6 0 z)
  :why    "5 times the divisor is {?m5}: next, 6 times the divisor")
```

![The rule div-multiple-6 of std.math.decimal-arith, as the engine draws it](diagrams/09-std.math.decimal-arith.div-multiple-6.svg)
<!-- /rule -->

#### `div-multiple-7`

<!-- rule std.math.decimal-arith div-multiple-7 -->
```lisp
(rule div-multiple-7 ()
  :match  (done (div-m6 ?k ?u ?m1 ?m2 ?m3 ?m4 ?m5) ?m6)
  :assert (mul-walk (div-m7 ?k ?u ?m1 ?m2 ?m3 ?m4 ?m5 ?m6) ?m1 7 0 z)
  :why    "6 times the divisor is {?m6}: next, 7 times the divisor")
```

![The rule div-multiple-7 of std.math.decimal-arith, as the engine draws it](diagrams/09-std.math.decimal-arith.div-multiple-7.svg)
<!-- /rule -->

#### `div-multiple-8`

<!-- rule std.math.decimal-arith div-multiple-8 -->
```lisp
(rule div-multiple-8 ()
  :match  (done (div-m7 ?k ?u ?m1 ?m2 ?m3 ?m4 ?m5 ?m6) ?m7)
  :assert (mul-walk (div-m8 ?k ?u ?m1 ?m2 ?m3 ?m4 ?m5 ?m6 ?m7) ?m1 8 0 z)
  :why    "7 times the divisor is {?m7}: next, 8 times the divisor")
```

![The rule div-multiple-8 of std.math.decimal-arith, as the engine draws it](diagrams/09-std.math.decimal-arith.div-multiple-8.svg)
<!-- /rule -->

#### `div-multiple-9`

<!-- rule std.math.decimal-arith div-multiple-9 -->
```lisp
(rule div-multiple-9 ()
  :match  (done (div-m8 ?k ?u ?m1 ?m2 ?m3 ?m4 ?m5 ?m6 ?m7) ?m8)
  :assert (mul-walk (div-m9 ?k ?u ?m1 ?m2 ?m3 ?m4 ?m5 ?m6 ?m7 ?m8) ?m1 9 0 z)
  :why    "8 times the divisor is {?m8}: next, 9 times the divisor")
```

![The rule div-multiple-9 of std.math.decimal-arith, as the engine draws it](diagrams/09-std.math.decimal-arith.div-multiple-9.svg)
<!-- /rule -->

#### `div-turn`

<!-- rule std.math.decimal-arith div-turn -->
```lisp
(rule div-turn ()
  :match  (done (div-m9 ?k ?u ?m1 ?m2 ?m3 ?m4 ?m5 ?m6 ?m7 ?m8) ?m9)
  :assert (top-first (div-digits ?k (multiples ?m1 ?m2 ?m3 ?m4 ?m5 ?m6 ?m7 ?m8 ?m9)) ?u z)
  :why    "9 times the divisor is {?m9}: the nine multiples are known. Now the digits of {?u}, the top one first")
```

![The rule div-turn of std.math.decimal-arith, as the engine draws it](diagrams/09-std.math.decimal-arith.div-turn.svg)
<!-- /rule -->

### Division: bringing a digit down

Each digit of the dividend, from the top, is brought down beside what is left over, and the next digit of the
quotient is known to lie between `0` and `10`.

#### `div-bring`

<!-- rule std.math.decimal-arith div-bring -->
```lisp
(rule div-bring ()
  :match  (div-step ?k ?ms (b ?a ?rest) (n ?p ?c) ?w)
  :assert (div-choose ?k ?ms ?rest ?w (n (n ?p ?c) ?a) 0 10 (n (n ?p ?c) ?a))
  :why    "bring down {?a} beside what is left over: the next digit of the quotient is at least 0 and less than 10")
```

![The rule div-bring of std.math.decimal-arith, as the engine draws it](diagrams/09-std.math.decimal-arith.div-bring.svg)
<!-- /rule -->

#### `div-bring-first`

<!-- rule std.math.decimal-arith div-bring-first -->
```lisp
(rule div-bring-first ()
  :match  (and (div-step ?k ?ms (b ?a ?rest) z ?w) (nz ?a))
  :assert (div-choose ?k ?ms ?rest ?w (n z ?a) 0 10 (n z ?a))
  :why    "nothing was left over: bring down {?a}. The next digit of the quotient is at least 0 and less than 10")
```

![The rule div-bring-first of std.math.decimal-arith, as the engine draws it](diagrams/09-std.math.decimal-arith.div-bring-first.svg)
<!-- /rule -->

#### `div-bring-zero`

<!-- rule std.math.decimal-arith div-bring-zero -->
```lisp
(rule div-bring-zero ()
  :match  (div-step ?k ?ms (b 0 ?rest) z ?w)
  :assert (div-digit ?k ?ms ?rest ?w 0 z)
  :why    "nothing was left over and the digit brought down is 0: the next digit of the quotient is 0")
```

![The rule div-bring-zero of std.math.decimal-arith, as the engine draws it](diagrams/09-std.math.decimal-arith.div-bring-zero.svg)
<!-- /rule -->

### Division: a quotient's digit by bisection

A digit $q$ of the quotient is the largest with $q \cdot Y \le U$, where $U$ is what was brought down. It is kept
between two bounds, $lo \le q < hi$; the table `mid` names the digit $j$ to try between them; one subtraction walk
takes $j \cdot Y$ from $U$, and either it fits, so $lo$ becomes $j$, or it does not, so $hi$ becomes $j$; the table
`adj` says when nothing lies between the bounds. One rule a digit tried, since the multiple to take is a
different place of the `multiples` fact for each.

#### `div-try-1`

<!-- rule std.math.decimal-arith div-try-1 -->
```lisp
(rule div-try-1 ()
  :match  (and (div-choose ?k (multiples ?m1 ?m2 ?m3 ?m4 ?m5 ?m6 ?m7 ?m8 ?m9) ?rest ?w ?u ?lo ?hi ?left)
               (mid ?lo ?hi 1))
  :assert (sub-walk (div-trial ?k (multiples ?m1 ?m2 ?m3 ?m4 ?m5 ?m6 ?m7 ?m8 ?m9) ?rest ?w ?u ?lo ?hi ?left 1)
                    ?u ?m1 0 z)
  :why    "the digit is at least {?lo} and less than {?hi}: try 1, and take 1 times the divisor, {?m1}, from {?u}")
```

![The rule div-try-1 of std.math.decimal-arith, as the engine draws it](diagrams/09-std.math.decimal-arith.div-try-1.svg)
<!-- /rule -->

#### `div-try-2`

<!-- rule std.math.decimal-arith div-try-2 -->
```lisp
(rule div-try-2 ()
  :match  (and (div-choose ?k (multiples ?m1 ?m2 ?m3 ?m4 ?m5 ?m6 ?m7 ?m8 ?m9) ?rest ?w ?u ?lo ?hi ?left)
               (mid ?lo ?hi 2))
  :assert (sub-walk (div-trial ?k (multiples ?m1 ?m2 ?m3 ?m4 ?m5 ?m6 ?m7 ?m8 ?m9) ?rest ?w ?u ?lo ?hi ?left 2)
                    ?u ?m2 0 z)
  :why    "the digit is at least {?lo} and less than {?hi}: try 2, and take 2 times the divisor, {?m2}, from {?u}")
```

![The rule div-try-2 of std.math.decimal-arith, as the engine draws it](diagrams/09-std.math.decimal-arith.div-try-2.svg)
<!-- /rule -->

#### `div-try-3`

<!-- rule std.math.decimal-arith div-try-3 -->
```lisp
(rule div-try-3 ()
  :match  (and (div-choose ?k (multiples ?m1 ?m2 ?m3 ?m4 ?m5 ?m6 ?m7 ?m8 ?m9) ?rest ?w ?u ?lo ?hi ?left)
               (mid ?lo ?hi 3))
  :assert (sub-walk (div-trial ?k (multiples ?m1 ?m2 ?m3 ?m4 ?m5 ?m6 ?m7 ?m8 ?m9) ?rest ?w ?u ?lo ?hi ?left 3)
                    ?u ?m3 0 z)
  :why    "the digit is at least {?lo} and less than {?hi}: try 3, and take 3 times the divisor, {?m3}, from {?u}")
```

![The rule div-try-3 of std.math.decimal-arith, as the engine draws it](diagrams/09-std.math.decimal-arith.div-try-3.svg)
<!-- /rule -->

#### `div-try-4`

<!-- rule std.math.decimal-arith div-try-4 -->
```lisp
(rule div-try-4 ()
  :match  (and (div-choose ?k (multiples ?m1 ?m2 ?m3 ?m4 ?m5 ?m6 ?m7 ?m8 ?m9) ?rest ?w ?u ?lo ?hi ?left)
               (mid ?lo ?hi 4))
  :assert (sub-walk (div-trial ?k (multiples ?m1 ?m2 ?m3 ?m4 ?m5 ?m6 ?m7 ?m8 ?m9) ?rest ?w ?u ?lo ?hi ?left 4)
                    ?u ?m4 0 z)
  :why    "the digit is at least {?lo} and less than {?hi}: try 4, and take 4 times the divisor, {?m4}, from {?u}")
```

![The rule div-try-4 of std.math.decimal-arith, as the engine draws it](diagrams/09-std.math.decimal-arith.div-try-4.svg)
<!-- /rule -->

#### `div-try-5`

<!-- rule std.math.decimal-arith div-try-5 -->
```lisp
(rule div-try-5 ()
  :match  (and (div-choose ?k (multiples ?m1 ?m2 ?m3 ?m4 ?m5 ?m6 ?m7 ?m8 ?m9) ?rest ?w ?u ?lo ?hi ?left)
               (mid ?lo ?hi 5))
  :assert (sub-walk (div-trial ?k (multiples ?m1 ?m2 ?m3 ?m4 ?m5 ?m6 ?m7 ?m8 ?m9) ?rest ?w ?u ?lo ?hi ?left 5)
                    ?u ?m5 0 z)
  :why    "the digit is at least {?lo} and less than {?hi}: try 5, and take 5 times the divisor, {?m5}, from {?u}")
```

![The rule div-try-5 of std.math.decimal-arith, as the engine draws it](diagrams/09-std.math.decimal-arith.div-try-5.svg)
<!-- /rule -->

#### `div-try-6`

<!-- rule std.math.decimal-arith div-try-6 -->
```lisp
(rule div-try-6 ()
  :match  (and (div-choose ?k (multiples ?m1 ?m2 ?m3 ?m4 ?m5 ?m6 ?m7 ?m8 ?m9) ?rest ?w ?u ?lo ?hi ?left)
               (mid ?lo ?hi 6))
  :assert (sub-walk (div-trial ?k (multiples ?m1 ?m2 ?m3 ?m4 ?m5 ?m6 ?m7 ?m8 ?m9) ?rest ?w ?u ?lo ?hi ?left 6)
                    ?u ?m6 0 z)
  :why    "the digit is at least {?lo} and less than {?hi}: try 6, and take 6 times the divisor, {?m6}, from {?u}")
```

![The rule div-try-6 of std.math.decimal-arith, as the engine draws it](diagrams/09-std.math.decimal-arith.div-try-6.svg)
<!-- /rule -->

#### `div-try-7`

<!-- rule std.math.decimal-arith div-try-7 -->
```lisp
(rule div-try-7 ()
  :match  (and (div-choose ?k (multiples ?m1 ?m2 ?m3 ?m4 ?m5 ?m6 ?m7 ?m8 ?m9) ?rest ?w ?u ?lo ?hi ?left)
               (mid ?lo ?hi 7))
  :assert (sub-walk (div-trial ?k (multiples ?m1 ?m2 ?m3 ?m4 ?m5 ?m6 ?m7 ?m8 ?m9) ?rest ?w ?u ?lo ?hi ?left 7)
                    ?u ?m7 0 z)
  :why    "the digit is at least {?lo} and less than {?hi}: try 7, and take 7 times the divisor, {?m7}, from {?u}")
```

![The rule div-try-7 of std.math.decimal-arith, as the engine draws it](diagrams/09-std.math.decimal-arith.div-try-7.svg)
<!-- /rule -->

#### `div-try-8`

<!-- rule std.math.decimal-arith div-try-8 -->
```lisp
(rule div-try-8 ()
  :match  (and (div-choose ?k (multiples ?m1 ?m2 ?m3 ?m4 ?m5 ?m6 ?m7 ?m8 ?m9) ?rest ?w ?u ?lo ?hi ?left)
               (mid ?lo ?hi 8))
  :assert (sub-walk (div-trial ?k (multiples ?m1 ?m2 ?m3 ?m4 ?m5 ?m6 ?m7 ?m8 ?m9) ?rest ?w ?u ?lo ?hi ?left 8)
                    ?u ?m8 0 z)
  :why    "the digit is at least {?lo} and less than {?hi}: try 8, and take 8 times the divisor, {?m8}, from {?u}")
```

![The rule div-try-8 of std.math.decimal-arith, as the engine draws it](diagrams/09-std.math.decimal-arith.div-try-8.svg)
<!-- /rule -->

#### `div-try-9`

<!-- rule std.math.decimal-arith div-try-9 -->
```lisp
(rule div-try-9 ()
  :match  (and (div-choose ?k (multiples ?m1 ?m2 ?m3 ?m4 ?m5 ?m6 ?m7 ?m8 ?m9) ?rest ?w ?u ?lo ?hi ?left)
               (mid ?lo ?hi 9))
  :assert (sub-walk (div-trial ?k (multiples ?m1 ?m2 ?m3 ?m4 ?m5 ?m6 ?m7 ?m8 ?m9) ?rest ?w ?u ?lo ?hi ?left 9)
                    ?u ?m9 0 z)
  :why    "the digit is at least {?lo} and less than {?hi}: try 9, and take 9 times the divisor, {?m9}, from {?u}")
```

![The rule div-try-9 of std.math.decimal-arith, as the engine draws it](diagrams/09-std.math.decimal-arith.div-try-9.svg)
<!-- /rule -->

#### `div-fits`

<!-- rule std.math.decimal-arith div-fits -->
```lisp
(rule div-fits ()
  :match  (done (div-trial ?k ?ms ?rest ?w ?u ?lo ?hi ?left ?j) ?t)
  :assert (div-choose ?k ?ms ?rest ?w ?u ?j ?hi ?t)
  :why    "{?j} times the divisor fits, and leaves {?t}: the digit is at least {?j}")
```

![The rule div-fits of std.math.decimal-arith, as the engine draws it](diagrams/09-std.math.decimal-arith.div-fits.svg)
<!-- /rule -->

#### `div-too-big`

<!-- rule std.math.decimal-arith div-too-big -->
```lisp
(rule div-too-big ()
  :match  (short (div-trial ?k ?ms ?rest ?w ?u ?lo ?hi ?left ?j))
  :assert (div-choose ?k ?ms ?rest ?w ?u ?lo ?j ?left)
  :why    "{?j} times the divisor does not fit: the digit is less than {?j}")
```

![The rule div-too-big of std.math.decimal-arith, as the engine draws it](diagrams/09-std.math.decimal-arith.div-too-big.svg)
<!-- /rule -->

#### `div-digit-found`

<!-- rule std.math.decimal-arith div-digit-found -->
```lisp
(rule div-digit-found ()
  :match  (and (div-choose ?k ?ms ?rest ?w ?u ?lo ?hi ?left) (adj ?lo ?hi))
  :assert (div-digit ?k ?ms ?rest ?w ?lo ?left)
  :why    "nothing lies between {?lo} and {?hi}: the digit is {?lo}, and {?left} is left over")
```

![The rule div-digit-found of std.math.decimal-arith, as the engine draws it](diagrams/09-std.math.decimal-arith.div-digit-found.svg)
<!-- /rule -->

### Division: the quotient grows

#### `div-next`

<!-- rule std.math.decimal-arith div-next -->
```lisp
(rule div-next ()
  :match  (and (div-digit ?k ?ms ?rest ?w ?j ?t) (nz ?j))
  :assert (div-step ?k ?ms ?rest ?t (n ?w ?j))
  :why    "the digit {?j} goes after the quotient so far")
```

![The rule div-next of std.math.decimal-arith, as the engine draws it](diagrams/09-std.math.decimal-arith.div-next.svg)
<!-- /rule -->

#### `div-next-zero`

<!-- rule std.math.decimal-arith div-next-zero -->
```lisp
(rule div-next-zero ()
  :match  (div-digit ?k ?ms ?rest (n ?p ?c) 0 ?t)
  :assert (div-step ?k ?ms ?rest ?t (n (n ?p ?c) 0))
  :why    "the digit 0 goes after a quotient that is not zero")
```

![The rule div-next-zero of std.math.decimal-arith, as the engine draws it](diagrams/09-std.math.decimal-arith.div-next-zero.svg)
<!-- /rule -->

#### `div-next-none`

<!-- rule std.math.decimal-arith div-next-none -->
```lisp
(rule div-next-none ()
  :match  (div-digit ?k ?ms ?rest z 0 ?t)
  :assert (div-step ?k ?ms ?rest ?t z)
  :why    "the digit is 0 and the quotient so far is zero: no high zero is written")
```

![The rule div-next-none of std.math.decimal-arith, as the engine draws it](diagrams/09-std.math.decimal-arith.div-next-none.svg)
<!-- /rule -->

### An operand that is a name no `let` states

The one `absent` of the theory, over its own `let`, which no rule may derive.

#### `operand-undeclared`

<!-- rule std.math.decimal-arith operand-undeclared -->
```lisp
(rule operand-undeclared ()
  :match  (and (operand ?k ?x)
               (neq ?v z) (eq ?v ?x)
               (neq ?x 0) (neq ?x 1) (neq ?x 2) (neq ?x 3) (neq ?x 4)
               (neq ?x 5) (neq ?x 6) (neq ?x 7) (neq ?x 8) (neq ?x 9)
               (absent (let ?x ?w)))
  :assert (bad-operand ?k (no-let ?x))
  :why    "{?x} is a name, and no let states it")
```

![The rule operand-undeclared of std.math.decimal-arith, as the engine draws it](diagrams/09-std.math.decimal-arith.operand-undeclared.svg)
<!-- /rule -->

### The entry

The closing rule. It closes the held extent of every relation of the module and of the seven tables of `std.math.digits`, names the
entries `arith` and `decimal` and each rule above, and calls the macro `positional` once, with this module's
relations, which activates every shared rule for decimal. Its drawing is not shown: it has a node for each of
those facts.

#### `decimal-arith`

<!-- rule std.math.decimal-arith decimal-arith -->
```lisp
(rule decimal-arith ()
  :match  ()
  :assert (and (__closed__ fa) (__closed__ fs) (__closed__ md) (__closed__ cl)
               (__closed__ nz) (__closed__ mid) (__closed__ adj)
               (__closed__ b)
               (__closed__ let) (__closed__ add) (__closed__ sum-of)
               (__closed__ sub) (__closed__ diff-of)
               (__closed__ cmp) (__closed__ cmp-of)
               (__closed__ mul) (__closed__ product-of)
               (__closed__ div) (__closed__ quot-of)
               (__closed__ malformed) (__closed__ undeclared)
               (__closed__ bad-operand)
               (__closed__ bad-request) (__closed__ not-a-number)
               (__closed__ no-let)
               (__closed__ operand) (__closed__ operand-is)
               (__closed__ add-walk) (__closed__ strip) (__closed__ turn)
               (__closed__ done)
               (__closed__ sub-walk) (__closed__ short)
               (__closed__ cmp-walk) (__closed__ order-is)
               (__closed__ top-first) (__closed__ top-first-is)
               (__closed__ mul-walk) (__closed__ mul-call) (__closed__ mul-step)
               (__closed__ div-call) (__closed__ div-done)
               (__closed__ multiples) (__closed__ div-step)
               (__closed__ div-choose) (__closed__ div-digit)
               (__closed__ sum) (__closed__ add-first) (__closed__ add-second)
               (__closed__ difference) (__closed__ shortfall)
               (__closed__ sub-first) (__closed__ sub-second)
               (__closed__ order) (__closed__ cmp-first) (__closed__ cmp-second)
               (__closed__ product) (__closed__ mul-first) (__closed__ mul-second)
               (__closed__ mul-digits) (__closed__ mul-row-of) (__closed__ mul-sum)
               (__closed__ quotient) (__closed__ div-first)
               (__closed__ div-second) (__closed__ div-m1) (__closed__ div-m2)
               (__closed__ div-m3) (__closed__ div-m4) (__closed__ div-m5)
               (__closed__ div-m6) (__closed__ div-m7) (__closed__ div-m8)
               (__closed__ div-m9) (__closed__ div-digits)
               (__closed__ div-trial)
               (__closed__ arith-rule)
               (arith-rule arith) (arith-rule decimal)
               (arith-rule mul-column) (arith-rule mul-walk-end)
               (arith-rule mul-carry-out)
               (arith-rule mul-row) (arith-rule mul-first-row)
               (arith-rule mul-add-row) (arith-rule div-multiples) (arith-rule div-multiple-2)
               (arith-rule div-multiple-3) (arith-rule div-multiple-4)
               (arith-rule div-multiple-5) (arith-rule div-multiple-6)
               (arith-rule div-multiple-7) (arith-rule div-multiple-8)
               (arith-rule div-multiple-9) (arith-rule div-turn)
               (arith-rule div-bring)
               (arith-rule div-bring-first) (arith-rule div-bring-zero)
               (arith-rule div-try-1) (arith-rule div-try-2)
               (arith-rule div-try-3) (arith-rule div-try-4)
               (arith-rule div-try-5) (arith-rule div-try-6)
               (arith-rule div-try-7) (arith-rule div-try-8)
               (arith-rule div-try-9) (arith-rule div-fits)
               (arith-rule div-too-big) (arith-rule div-digit-found)
               (arith-rule div-next) (arith-rule div-next-zero)
               (arith-rule div-next-none) (arith-rule operand-undeclared)
               (positional operand operand-is n let named add-walk fa b strip turn nz done
                           add add-first add-second sum sum-of
                           sub-walk fs short sub sub-first sub-second difference shortfall diff-of
                           cmp-walk cl order-is cmp cmp-first cmp-second order cmp-of
                           top-first top-first-is mul-call mul-digits mul-step mul-sum
                           mul mul-first mul-second product product-of
                           div-digits div-step div-done div div-first div-second div-call
                           quotient quot-of div-m1
                           bad-operand not-a-number malformed bad-request no-let undeclared))
  :why    "arithmetic is computed and never guessed: every relation of the theory is closed")
```
<!-- /rule -->
