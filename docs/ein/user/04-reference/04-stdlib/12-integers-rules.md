# The integers' rules

How `std.math.int` works: every rule of the module, as it writes it and as the engine draws it. What a program
writes to use it is [the page before](11-integers-rationals-gcd.md#using-stdmathint--integers); how to read a
drawing is [the index](README.md#how-a-rule-is-shown).

None of these rules has a parameter. An integer `(int P N)` is computed on its two parts, each a natural number,
by tagged calls into [`std.math.decimal-arith`](08-natural-numbers.md#the-tagged-calls): every step below hands a
walk or a call a tag that says what the answer is for, and the next rule has that answer as its one premise. A
rule's `:why` says what one firing does, in the words a trace prints.

## The operands

An operand is an integer written out, which stands for itself, or a name, which the natural numbers' `operand`
looks up.

#### `int-operand`

<!-- rule std.math.int int-operand -->
```lisp
(rule int-operand ()
  :match  (operand ?k (int ?p ?n))
  :assert (operand-is ?k (int ?p ?n))
  :why    "an integer stands for itself")
```

![The rule int-operand of std.math.int, as the engine draws it](diagrams/12-std.math.int.int-operand.svg)
<!-- /rule -->

#### `int-operand-second`

<!-- rule std.math.int int-operand-second -->
```lisp
(rule int-operand-second ()
  :match  (operand-is (int-first ?k ?y) (int ?p ?n))
  :assert (operand (int-second ?k ?p ?n) ?y)
  :why    "the first operand is {?p} less {?n}: now the integer {?y} stands for")
```

![The rule int-operand-second of std.math.int, as the engine draws it](diagrams/12-std.math.int.int-operand-second.svg)
<!-- /rule -->

## The normal form

For $P$ less $N$, two subtraction walks, each part from the other, as a natural subtraction runs them: one of the
two has an answer, and it is the part that is left.

$$
\operatorname{norm}(P, N) = \begin{cases} (\mathsf{int}\ (P - N)\ \mathsf{z}) & P \ge N \\ (\mathsf{int}\ \mathsf{z}\ (N - P)) & P < N \end{cases}
$$

#### `int-norm-walks`

<!-- rule std.math.int int-norm-walks -->
```lisp
(rule int-norm-walks ()
  :match  (int-norm ?k ?p ?n)
  :assert (and (sub-walk (int-plus ?k) ?p ?n 0 z)
               (sub-walk (int-minus ?k) ?n ?p 0 z))
  :why    "take {?n} from {?p}, and {?p} from {?n}: one of the two has an answer")
```

![The rule int-norm-walks of std.math.int, as the engine draws it](diagrams/12-std.math.int.int-norm-walks.svg)
<!-- /rule -->

#### `int-norm-plus`

<!-- rule std.math.int int-norm-plus -->
```lisp
(rule int-norm-plus ()
  :match  (done (int-plus ?k) ?d)
  :assert (int-done ?k (int ?d z))
  :why    "the positive part is {?d}, and the negative part is zero")
```

![The rule int-norm-plus of std.math.int, as the engine draws it](diagrams/12-std.math.int.int-norm-plus.svg)
<!-- /rule -->

#### `int-norm-minus`

<!-- rule std.math.int int-norm-minus -->
```lisp
(rule int-norm-minus ()
  :match  (done (int-minus ?k) ?r)
  :assert (int-done ?k (int z ?r))
  :why    "the negative part is {?r}, and the positive part is zero")
```

![The rule int-norm-minus of std.math.int, as the engine draws it](diagrams/12-std.math.int.int-norm-minus.svg)
<!-- /rule -->

## Simplification, and negation

Simplifying is the normal form; negating is the normal form with the parts exchanged.

#### `int-simplify-ask`

<!-- rule std.math.int int-simplify-ask -->
```lisp
(rule int-simplify-ask ()
  :match  (int-simplify ?x)
  :assert (operand (int-only (int-simplify ?x)) ?x)
  :why    "{?x} in normal form is asked: first, the integer {?x} stands for")
```

![The rule int-simplify-ask of std.math.int, as the engine draws it](diagrams/12-std.math.int.int-simplify-ask.svg)
<!-- /rule -->

#### `int-simplify-start`

<!-- rule std.math.int int-simplify-start -->
```lisp
(rule int-simplify-start ()
  :match  (operand-is (int-only (int-simplify ?x)) (int ?p ?n))
  :assert (int-norm (int-simplify ?x) ?p ?n)
  :why    "simplify {?p} less {?n}")
```

![The rule int-simplify-start of std.math.int, as the engine draws it](diagrams/12-std.math.int.int-simplify-start.svg)
<!-- /rule -->

#### `int-simplify-answer`

<!-- rule std.math.int int-simplify-answer -->
```lisp
(rule int-simplify-answer ()
  :match  (int-done (int-simplify ?x) ?s)
  :assert (int-simple-of ?x ?s)
  :why    "{?x} is {?s}")
```

![The rule int-simplify-answer of std.math.int, as the engine draws it](diagrams/12-std.math.int.int-simplify-answer.svg)
<!-- /rule -->

#### `int-neg-ask`

<!-- rule std.math.int int-neg-ask -->
```lisp
(rule int-neg-ask ()
  :match  (int-neg ?x)
  :assert (operand (int-only (int-neg ?x)) ?x)
  :why    "{?x} negated is asked: first, the integer {?x} stands for")
```

![The rule int-neg-ask of std.math.int, as the engine draws it](diagrams/12-std.math.int.int-neg-ask.svg)
<!-- /rule -->

#### `int-neg-start`

<!-- rule std.math.int int-neg-start -->
```lisp
(rule int-neg-start ()
  :match  (operand-is (int-only (int-neg ?x)) (int ?p ?n))
  :assert (int-norm (int-neg ?x) ?n ?p)
  :why    "the parts change places: simplify {?n} less {?p}")
```

![The rule int-neg-start of std.math.int, as the engine draws it](diagrams/12-std.math.int.int-neg-start.svg)
<!-- /rule -->

#### `int-neg-answer`

<!-- rule std.math.int int-neg-answer -->
```lisp
(rule int-neg-answer ()
  :match  (int-done (int-neg ?x) ?s)
  :assert (int-neg-of ?x ?s)
  :why    "{?x} negated is {?s}")
```

![The rule int-neg-answer of std.math.int, as the engine draws it](diagrams/12-std.math.int.int-neg-answer.svg)
<!-- /rule -->

## Addition, and subtraction

$$
(P_1 - N_1) + (P_2 - N_2) = (P_1 + P_2) - (N_1 + N_2)
$$

The positive parts are added, then the negative parts, then the two sums are normalised. A subtraction is the
addition of the second operand with its parts exchanged.

#### `int-add-plus`

<!-- rule std.math.int int-add-plus -->
```lisp
(rule int-add-plus ()
  :match  (int-add-call ?k ?p ?n ?q ?m)
  :assert (add-walk (int-add-p ?k ?n ?m) ?p ?q 0 z)
  :why    "first the positive parts: {?p} + {?q}")
```

![The rule int-add-plus of std.math.int, as the engine draws it](diagrams/12-std.math.int.int-add-plus.svg)
<!-- /rule -->

#### `int-add-minus`

<!-- rule std.math.int int-add-minus -->
```lisp
(rule int-add-minus ()
  :match  (done (int-add-p ?k ?n ?m) ?s)
  :assert (add-walk (int-add-n ?k ?s) ?n ?m 0 z)
  :why    "the positive parts make {?s}: now the negative parts, {?n} + {?m}")
```

![The rule int-add-minus of std.math.int, as the engine draws it](diagrams/12-std.math.int.int-add-minus.svg)
<!-- /rule -->

#### `int-add-norm`

<!-- rule std.math.int int-add-norm -->
```lisp
(rule int-add-norm ()
  :match  (done (int-add-n ?k ?s) ?t)
  :assert (int-norm ?k ?s ?t)
  :why    "the negative parts make {?t}: simplify {?s} less {?t}")
```

![The rule int-add-norm of std.math.int, as the engine draws it](diagrams/12-std.math.int.int-add-norm.svg)
<!-- /rule -->

#### `int-add-ask`

<!-- rule std.math.int int-add-ask -->
```lisp
(rule int-add-ask ()
  :match  (int-add ?x ?y)
  :assert (operand (int-first (int-add ?x ?y) ?y) ?x)
  :why    "{?x} + {?y} is asked: first, the integer {?x} stands for")
```

![The rule int-add-ask of std.math.int, as the engine draws it](diagrams/12-std.math.int.int-add-ask.svg)
<!-- /rule -->

#### `int-add-start`

<!-- rule std.math.int int-add-start -->
```lisp
(rule int-add-start ()
  :match  (operand-is (int-second (int-add ?x ?y) ?p ?n) (int ?q ?m))
  :assert (int-add-call (int-add ?x ?y) ?p ?n ?q ?m)
  :why    "add {?p} less {?n} and {?q} less {?m}")
```

![The rule int-add-start of std.math.int, as the engine draws it](diagrams/12-std.math.int.int-add-start.svg)
<!-- /rule -->

#### `int-add-answer`

<!-- rule std.math.int int-add-answer -->
```lisp
(rule int-add-answer ()
  :match  (int-done (int-add ?x ?y) ?s)
  :assert (int-sum-of ?x ?y ?s)
  :why    "{?x} + {?y} = {?s}")
```

![The rule int-add-answer of std.math.int, as the engine draws it](diagrams/12-std.math.int.int-add-answer.svg)
<!-- /rule -->

#### `int-sub-ask`

<!-- rule std.math.int int-sub-ask -->
```lisp
(rule int-sub-ask ()
  :match  (int-sub ?x ?y)
  :assert (operand (int-first (int-sub ?x ?y) ?y) ?x)
  :why    "{?x} - {?y} is asked: first, the integer {?x} stands for")
```

![The rule int-sub-ask of std.math.int, as the engine draws it](diagrams/12-std.math.int.int-sub-ask.svg)
<!-- /rule -->

#### `int-sub-start`

<!-- rule std.math.int int-sub-start -->
```lisp
(rule int-sub-start ()
  :match  (operand-is (int-second (int-sub ?x ?y) ?p ?n) (int ?q ?m))
  :assert (int-add-call (int-sub ?x ?y) ?p ?n ?m ?q)
  :why    "to take {?q} less {?m} away, add {?m} less {?q}")
```

![The rule int-sub-start of std.math.int, as the engine draws it](diagrams/12-std.math.int.int-sub-start.svg)
<!-- /rule -->

#### `int-sub-answer`

<!-- rule std.math.int int-sub-answer -->
```lisp
(rule int-sub-answer ()
  :match  (int-done (int-sub ?x ?y) ?s)
  :assert (int-diff-of ?x ?y ?s)
  :why    "{?x} - {?y} = {?s}")
```

![The rule int-sub-answer of std.math.int, as the engine draws it](diagrams/12-std.math.int.int-sub-answer.svg)
<!-- /rule -->

## Multiplication

$$
(P_1 - N_1) \cdot (P_2 - N_2) = (P_1 P_2 + N_1 N_2) - (P_1 N_2 + N_1 P_2)
$$

Four products of natural numbers and two sums, with no case on a sign, then the normal form.

#### `int-mul-first`

<!-- rule std.math.int int-mul-first -->
```lisp
(rule int-mul-first ()
  :match  (int-mul-call ?k ?p ?n ?q ?m)
  :assert (mul-call (int-mul-pp ?k ?p ?n ?q ?m) ?p ?q)
  :why    "first the positive parts: {?p} x {?q}")
```

![The rule int-mul-first of std.math.int, as the engine draws it](diagrams/12-std.math.int.int-mul-first.svg)
<!-- /rule -->

#### `int-mul-second`

<!-- rule std.math.int int-mul-second -->
```lisp
(rule int-mul-second ()
  :match  (done (int-mul-pp ?k ?p ?n ?q ?m) ?a)
  :assert (mul-call (int-mul-nn ?k ?p ?n ?q ?m ?a) ?n ?m)
  :why    "the positive parts make {?a}: now the negative parts, {?n} x {?m}")
```

![The rule int-mul-second of std.math.int, as the engine draws it](diagrams/12-std.math.int.int-mul-second.svg)
<!-- /rule -->

#### `int-mul-plus`

<!-- rule std.math.int int-mul-plus -->
```lisp
(rule int-mul-plus ()
  :match  (done (int-mul-nn ?k ?p ?n ?q ?m ?a) ?b)
  :assert (add-walk (int-mul-pos ?k ?p ?n ?q ?m) ?a ?b 0 z)
  :why    "the negative parts make {?b}: the product's positive part is {?a} + {?b}")
```

![The rule int-mul-plus of std.math.int, as the engine draws it](diagrams/12-std.math.int.int-mul-plus.svg)
<!-- /rule -->

#### `int-mul-third`

<!-- rule std.math.int int-mul-third -->
```lisp
(rule int-mul-third ()
  :match  (done (int-mul-pos ?k ?p ?n ?q ?m) ?s)
  :assert (mul-call (int-mul-pn ?k ?n ?q ?s) ?p ?m)
  :why    "the positive part is {?s}: now across, {?p} x {?m}")
```

![The rule int-mul-third of std.math.int, as the engine draws it](diagrams/12-std.math.int.int-mul-third.svg)
<!-- /rule -->

#### `int-mul-fourth`

<!-- rule std.math.int int-mul-fourth -->
```lisp
(rule int-mul-fourth ()
  :match  (done (int-mul-pn ?k ?n ?q ?s) ?c)
  :assert (mul-call (int-mul-np ?k ?s ?c) ?n ?q)
  :why    "that makes {?c}: and across the other way, {?n} x {?q}")
```

![The rule int-mul-fourth of std.math.int, as the engine draws it](diagrams/12-std.math.int.int-mul-fourth.svg)
<!-- /rule -->

#### `int-mul-minus`

<!-- rule std.math.int int-mul-minus -->
```lisp
(rule int-mul-minus ()
  :match  (done (int-mul-np ?k ?s ?c) ?d)
  :assert (add-walk (int-mul-neg ?k ?s) ?c ?d 0 z)
  :why    "that makes {?d}: the product's negative part is {?c} + {?d}")
```

![The rule int-mul-minus of std.math.int, as the engine draws it](diagrams/12-std.math.int.int-mul-minus.svg)
<!-- /rule -->

#### `int-mul-norm`

<!-- rule std.math.int int-mul-norm -->
```lisp
(rule int-mul-norm ()
  :match  (done (int-mul-neg ?k ?s) ?t)
  :assert (int-norm ?k ?s ?t)
  :why    "the negative part is {?t}: simplify {?s} less {?t}")
```

![The rule int-mul-norm of std.math.int, as the engine draws it](diagrams/12-std.math.int.int-mul-norm.svg)
<!-- /rule -->

#### `int-mul-ask`

<!-- rule std.math.int int-mul-ask -->
```lisp
(rule int-mul-ask ()
  :match  (int-mul ?x ?y)
  :assert (operand (int-first (int-mul ?x ?y) ?y) ?x)
  :why    "{?x} x {?y} is asked: first, the integer {?x} stands for")
```

![The rule int-mul-ask of std.math.int, as the engine draws it](diagrams/12-std.math.int.int-mul-ask.svg)
<!-- /rule -->

#### `int-mul-start`

<!-- rule std.math.int int-mul-start -->
```lisp
(rule int-mul-start ()
  :match  (operand-is (int-second (int-mul ?x ?y) ?p ?n) (int ?q ?m))
  :assert (int-mul-call (int-mul ?x ?y) ?p ?n ?q ?m)
  :why    "multiply {?p} less {?n} by {?q} less {?m}")
```

![The rule int-mul-start of std.math.int, as the engine draws it](diagrams/12-std.math.int.int-mul-start.svg)
<!-- /rule -->

#### `int-mul-answer`

<!-- rule std.math.int int-mul-answer -->
```lisp
(rule int-mul-answer ()
  :match  (int-done (int-mul ?x ?y) ?s)
  :assert (int-product-of ?x ?y ?s)
  :why    "{?x} x {?y} = {?s}")
```

![The rule int-mul-answer of std.math.int, as the engine draws it](diagrams/12-std.math.int.int-mul-answer.svg)
<!-- /rule -->

## Comparison

$$
P_1 - N_1 \lessgtr P_2 - N_2 \iff P_1 + N_2 \lessgtr P_2 + N_1
$$

Two sums of natural numbers, and one comparison walk between them.

#### `int-cmp-left`

<!-- rule std.math.int int-cmp-left -->
```lisp
(rule int-cmp-left ()
  :match  (int-cmp-call ?k ?p ?n ?q ?m)
  :assert (add-walk (int-cmp-l ?k ?n ?q) ?p ?m 0 z)
  :why    "add across: the left side is {?p} + {?m}")
```

![The rule int-cmp-left of std.math.int, as the engine draws it](diagrams/12-std.math.int.int-cmp-left.svg)
<!-- /rule -->

#### `int-cmp-right`

<!-- rule std.math.int int-cmp-right -->
```lisp
(rule int-cmp-right ()
  :match  (done (int-cmp-l ?k ?n ?q) ?l)
  :assert (add-walk (int-cmp-r ?k ?l) ?n ?q 0 z)
  :why    "the left side is {?l}: the right side is {?n} + {?q}")
```

![The rule int-cmp-right of std.math.int, as the engine draws it](diagrams/12-std.math.int.int-cmp-right.svg)
<!-- /rule -->

#### `int-cmp-sides`

<!-- rule std.math.int int-cmp-sides -->
```lisp
(rule int-cmp-sides ()
  :match  (done (int-cmp-r ?k ?l) ?r)
  :assert (cmp-walk ?k ?l ?r eq)
  :why    "the right side is {?r}: compare {?l} with {?r}")
```

![The rule int-cmp-sides of std.math.int, as the engine draws it](diagrams/12-std.math.int.int-cmp-sides.svg)
<!-- /rule -->

#### `int-cmp-ask`

<!-- rule std.math.int int-cmp-ask -->
```lisp
(rule int-cmp-ask ()
  :match  (int-cmp ?x ?y)
  :assert (operand (int-first (int-cmp ?x ?y) ?y) ?x)
  :why    "{?x} compared with {?y} is asked: first, the integer {?x} stands for")
```

![The rule int-cmp-ask of std.math.int, as the engine draws it](diagrams/12-std.math.int.int-cmp-ask.svg)
<!-- /rule -->

#### `int-cmp-start`

<!-- rule std.math.int int-cmp-start -->
```lisp
(rule int-cmp-start ()
  :match  (operand-is (int-second (int-cmp ?x ?y) ?p ?n) (int ?q ?m))
  :assert (int-cmp-call (int-cmp ?x ?y) ?p ?n ?q ?m)
  :why    "compare {?p} less {?n} with {?q} less {?m}")
```

![The rule int-cmp-start of std.math.int, as the engine draws it](diagrams/12-std.math.int.int-cmp-start.svg)
<!-- /rule -->

#### `int-cmp-answer`

<!-- rule std.math.int int-cmp-answer -->
```lisp
(rule int-cmp-answer ()
  :match  (order-is (int-cmp ?x ?y) ?o)
  :assert (int-cmp-of ?x ?y ?o)
  :why    "{?x} against {?y}: {?o}")
```

![The rule int-cmp-answer of std.math.int, as the engine draws it](diagrams/12-std.math.int.int-cmp-answer.svg)
<!-- /rule -->

## Division: the sizes

All three conventions start alike. The dividend and the divisor are normalised; a divisor of zero is flagged; the
two sizes, the parts that are not zero, are divided by the natural numbers' division; and which of the four cases
of signs it was is kept in the tag (`pp`, `pn`, `np`, `nn`: the dividend's sign, then the divisor's).
`int-div-rest` asks for the divisor's size less the remainder, which the conventions that round away from zero
need.

#### `int-div-start`

<!-- rule std.math.int int-div-start -->
```lisp
(rule int-div-start ()
  :match  (operand-is (int-second (int-div-of ?k) ?p ?n) (int ?q ?m))
  :assert (int-norm (int-div-a ?k ?q ?m) ?p ?n)
  :why    "first the dividend, simplified: {?p} less {?n}")
```

![The rule int-div-start of std.math.int, as the engine draws it](diagrams/12-std.math.int.int-div-start.svg)
<!-- /rule -->

#### `int-div-divisor`

<!-- rule std.math.int int-div-divisor -->
```lisp
(rule int-div-divisor ()
  :match  (int-done (int-div-a ?k ?q ?m) ?a)
  :assert (int-norm (int-div-b ?k ?a) ?q ?m)
  :why    "the dividend is {?a}: now the divisor, simplified, {?q} less {?m}")
```

![The rule int-div-divisor of std.math.int, as the engine draws it](diagrams/12-std.math.int.int-div-divisor.svg)
<!-- /rule -->

#### `int-div-by-zero`

<!-- rule std.math.int int-div-by-zero -->
```lisp
(rule int-div-by-zero ()
  :match  (int-done (int-div-b ?k ?a) (int z z))
  :assert (division-by-zero ?k)
  :why    "the divisor of {?k} is zero")
```

![The rule int-div-by-zero of std.math.int, as the engine draws it](diagrams/12-std.math.int.int-div-by-zero.svg)
<!-- /rule -->

#### `int-div-pp`

<!-- rule std.math.int int-div-pp -->
```lisp
(rule int-div-pp ()
  :match  (int-done (int-div-b ?k (int ?a z)) (int (n ?b ?c) z))
  :assert (div-call (int-div-m ?k pos pos (n ?b ?c)) ?a (n ?b ?c))
  :why    "neither is negative: divide the sizes, {?a} by (n {?b} {?c})")
```

![The rule int-div-pp of std.math.int, as the engine draws it](diagrams/12-std.math.int.int-div-pp.svg)
<!-- /rule -->

#### `int-div-pn`

<!-- rule std.math.int int-div-pn -->
```lisp
(rule int-div-pn ()
  :match  (int-done (int-div-b ?k (int ?a z)) (int z (n ?b ?c)))
  :assert (div-call (int-div-m ?k pos neg (n ?b ?c)) ?a (n ?b ?c))
  :why    "the divisor is negative: divide the sizes, {?a} by (n {?b} {?c})")
```

![The rule int-div-pn of std.math.int, as the engine draws it](diagrams/12-std.math.int.int-div-pn.svg)
<!-- /rule -->

#### `int-div-np`

<!-- rule std.math.int int-div-np -->
```lisp
(rule int-div-np ()
  :match  (int-done (int-div-b ?k (int z (n ?a ?d))) (int (n ?b ?c) z))
  :assert (div-call (int-div-m ?k neg pos (n ?b ?c)) (n ?a ?d) (n ?b ?c))
  :why    "the dividend is negative: divide the sizes, (n {?a} {?d}) by (n {?b} {?c})")
```

![The rule int-div-np of std.math.int, as the engine draws it](diagrams/12-std.math.int.int-div-np.svg)
<!-- /rule -->

#### `int-div-nn`

<!-- rule std.math.int int-div-nn -->
```lisp
(rule int-div-nn ()
  :match  (int-done (int-div-b ?k (int z (n ?a ?d))) (int z (n ?b ?c)))
  :assert (div-call (int-div-m ?k neg neg (n ?b ?c)) (n ?a ?d) (n ?b ?c))
  :why    "both are negative: divide the sizes, (n {?a} {?d}) by (n {?b} {?c})")
```

![The rule int-div-nn of std.math.int, as the engine draws it](diagrams/12-std.math.int.int-div-nn.svg)
<!-- /rule -->

#### `int-div-rest`

<!-- rule std.math.int int-div-rest -->
```lisp
(rule int-div-rest ()
  :match  (done (int-div-q ?k ?s ?b ?r) ?q)
  :assert (sub-walk (int-div-r ?k ?s ?q) ?b ?r 0 z)
  :why    "the quotient's size is {?q}: now the divisor's size less the remainder, {?b} - {?r}")
```

![The rule int-div-rest of std.math.int, as the engine draws it](diagrams/12-std.math.int.int-div-rest.svg)
<!-- /rule -->

## Division: the Euclidean convention

$$
X = Q \cdot Y + R, \qquad 0 \le R < \lvert Y \rvert
$$

Where the dividend is negative and a remainder is left, the quotient's size is one more and the remainder is the
divisor's size less the sizes' remainder.

#### `int-div-euclid-ask`

<!-- rule std.math.int int-div-euclid-ask -->
```lisp
(rule int-div-euclid-ask ()
  :match  (int-div-euclid ?x ?y)
  :assert (operand (int-first (int-div-of (int-div-euclid ?x ?y)) ?y) ?x)
  :why    "{?x} divided by {?y}, the remainder never negative, is asked: first, the integer {?x} stands for")
```

![The rule int-div-euclid-ask of std.math.int, as the engine draws it](diagrams/12-std.math.int.int-div-euclid-ask.svg)
<!-- /rule -->

#### `int-div-euclid-pp`

<!-- rule std.math.int int-div-euclid-pp -->
```lisp
(rule int-div-euclid-pp ()
  :match  (div-done (int-div-m (int-div-euclid ?x ?y) pos pos ?b) ?q ?r)
  :assert (int-div-done (int-div-euclid ?x ?y) (int ?q z) ?r)
  :why    "neither is negative: the sizes' quotient {?q} and remainder {?r} are the answer")
```

![The rule int-div-euclid-pp of std.math.int, as the engine draws it](diagrams/12-std.math.int.int-div-euclid-pp.svg)
<!-- /rule -->

#### `int-div-euclid-pn`

<!-- rule std.math.int int-div-euclid-pn -->
```lisp
(rule int-div-euclid-pn ()
  :match  (div-done (int-div-m (int-div-euclid ?x ?y) pos neg ?b) ?q ?r)
  :assert (int-div-done (int-div-euclid ?x ?y) (int z ?q) ?r)
  :why    "the divisor is negative: the quotient is {?q} negated, and the remainder {?r} stays")
```

![The rule int-div-euclid-pn of std.math.int, as the engine draws it](diagrams/12-std.math.int.int-div-euclid-pn.svg)
<!-- /rule -->

#### `int-div-euclid-np-exact`

<!-- rule std.math.int int-div-euclid-np-exact -->
```lisp
(rule int-div-euclid-np-exact ()
  :match  (div-done (int-div-m (int-div-euclid ?x ?y) neg pos ?b) ?q z)
  :assert (int-div-done (int-div-euclid ?x ?y) (int z ?q) z)
  :why    "the dividend is negative and nothing is left: the quotient is {?q} negated")
```

![The rule int-div-euclid-np-exact of std.math.int, as the engine draws it](diagrams/12-std.math.int.int-div-euclid-np-exact.svg)
<!-- /rule -->

#### `int-div-euclid-nn-exact`

<!-- rule std.math.int int-div-euclid-nn-exact -->
```lisp
(rule int-div-euclid-nn-exact ()
  :match  (div-done (int-div-m (int-div-euclid ?x ?y) neg neg ?b) ?q z)
  :assert (int-div-done (int-div-euclid ?x ?y) (int ?q z) z)
  :why    "both are negative and nothing is left: the quotient is {?q}")
```

![The rule int-div-euclid-nn-exact of std.math.int, as the engine draws it](diagrams/12-std.math.int.int-div-euclid-nn-exact.svg)
<!-- /rule -->

#### `int-div-euclid-np-more`

<!-- rule std.math.int int-div-euclid-np-more -->
```lisp
(rule int-div-euclid-np-more ()
  :match  (div-done (int-div-m (int-div-euclid ?x ?y) neg pos ?b) ?q (n ?r ?d))
  :assert (add-walk (int-div-q (int-div-euclid ?x ?y) neg ?b (n ?r ?d)) ?q (n z 1) 0 z)
  :why    "a remainder is left, (n {?r} {?d}): one more in the quotient's size, {?q} + 1")
```

![The rule int-div-euclid-np-more of std.math.int, as the engine draws it](diagrams/12-std.math.int.int-div-euclid-np-more.svg)
<!-- /rule -->

#### `int-div-euclid-nn-more`

<!-- rule std.math.int int-div-euclid-nn-more -->
```lisp
(rule int-div-euclid-nn-more ()
  :match  (div-done (int-div-m (int-div-euclid ?x ?y) neg neg ?b) ?q (n ?r ?d))
  :assert (add-walk (int-div-q (int-div-euclid ?x ?y) pos ?b (n ?r ?d)) ?q (n z 1) 0 z)
  :why    "a remainder is left, (n {?r} {?d}): one more in the quotient's size, {?q} + 1")
```

![The rule int-div-euclid-nn-more of std.math.int, as the engine draws it](diagrams/12-std.math.int.int-div-euclid-nn-more.svg)
<!-- /rule -->

#### `int-div-euclid-neg`

<!-- rule std.math.int int-div-euclid-neg -->
```lisp
(rule int-div-euclid-neg ()
  :match  (done (int-div-r (int-div-euclid ?x ?y) neg ?q) ?t)
  :assert (int-div-done (int-div-euclid ?x ?y) (int z ?q) ?t)
  :why    "the quotient is {?q} negated, and the remainder is {?t}")
```

![The rule int-div-euclid-neg of std.math.int, as the engine draws it](diagrams/12-std.math.int.int-div-euclid-neg.svg)
<!-- /rule -->

#### `int-div-euclid-pos`

<!-- rule std.math.int int-div-euclid-pos -->
```lisp
(rule int-div-euclid-pos ()
  :match  (done (int-div-r (int-div-euclid ?x ?y) pos ?q) ?t)
  :assert (int-div-done (int-div-euclid ?x ?y) (int ?q z) ?t)
  :why    "the quotient is {?q}, and the remainder is {?t}")
```

![The rule int-div-euclid-pos of std.math.int, as the engine draws it](diagrams/12-std.math.int.int-div-euclid-pos.svg)
<!-- /rule -->

#### `int-div-euclid-answer`

<!-- rule std.math.int int-div-euclid-answer -->
```lisp
(rule int-div-euclid-answer ()
  :match  (int-div-done (int-div-euclid ?x ?y) ?q ?r)
  :assert (int-quot-euclid-of ?x ?y ?q ?r)
  :why    "{?x} divided by {?y} = {?q}, remainder {?r}")
```

![The rule int-div-euclid-answer of std.math.int, as the engine draws it](diagrams/12-std.math.int.int-div-euclid-answer.svg)
<!-- /rule -->

## Division: the truncated convention

The quotient is the sizes' quotient with the sign of the two signs' product, and the remainder has the dividend's
sign.

#### `int-div-trunc-ask`

<!-- rule std.math.int int-div-trunc-ask -->
```lisp
(rule int-div-trunc-ask ()
  :match  (int-div-trunc ?x ?y)
  :assert (operand (int-first (int-div-of (int-div-trunc ?x ?y)) ?y) ?x)
  :why    "{?x} divided by {?y}, rounded toward zero, is asked: first, the integer {?x} stands for")
```

![The rule int-div-trunc-ask of std.math.int, as the engine draws it](diagrams/12-std.math.int.int-div-trunc-ask.svg)
<!-- /rule -->

#### `int-div-trunc-pp`

<!-- rule std.math.int int-div-trunc-pp -->
```lisp
(rule int-div-trunc-pp ()
  :match  (div-done (int-div-m (int-div-trunc ?x ?y) pos pos ?b) ?q ?r)
  :assert (int-div-done (int-div-trunc ?x ?y) (int ?q z) (int ?r z))
  :why    "neither is negative: the sizes' quotient {?q} and remainder {?r} are the answer")
```

![The rule int-div-trunc-pp of std.math.int, as the engine draws it](diagrams/12-std.math.int.int-div-trunc-pp.svg)
<!-- /rule -->

#### `int-div-trunc-pn`

<!-- rule std.math.int int-div-trunc-pn -->
```lisp
(rule int-div-trunc-pn ()
  :match  (div-done (int-div-m (int-div-trunc ?x ?y) pos neg ?b) ?q ?r)
  :assert (int-div-done (int-div-trunc ?x ?y) (int z ?q) (int ?r z))
  :why    "the divisor is negative: the quotient is {?q} negated, and the remainder {?r} stays")
```

![The rule int-div-trunc-pn of std.math.int, as the engine draws it](diagrams/12-std.math.int.int-div-trunc-pn.svg)
<!-- /rule -->

#### `int-div-trunc-np`

<!-- rule std.math.int int-div-trunc-np -->
```lisp
(rule int-div-trunc-np ()
  :match  (div-done (int-div-m (int-div-trunc ?x ?y) neg pos ?b) ?q ?r)
  :assert (int-div-done (int-div-trunc ?x ?y) (int z ?q) (int z ?r))
  :why    "the dividend is negative: the quotient is {?q} negated, and the remainder {?r} negated")
```

![The rule int-div-trunc-np of std.math.int, as the engine draws it](diagrams/12-std.math.int.int-div-trunc-np.svg)
<!-- /rule -->

#### `int-div-trunc-nn`

<!-- rule std.math.int int-div-trunc-nn -->
```lisp
(rule int-div-trunc-nn ()
  :match  (div-done (int-div-m (int-div-trunc ?x ?y) neg neg ?b) ?q ?r)
  :assert (int-div-done (int-div-trunc ?x ?y) (int ?q z) (int z ?r))
  :why    "both are negative: the quotient is {?q}, and the remainder {?r} negated")
```

![The rule int-div-trunc-nn of std.math.int, as the engine draws it](diagrams/12-std.math.int.int-div-trunc-nn.svg)
<!-- /rule -->

#### `int-div-trunc-answer`

<!-- rule std.math.int int-div-trunc-answer -->
```lisp
(rule int-div-trunc-answer ()
  :match  (int-div-done (int-div-trunc ?x ?y) ?q ?r)
  :assert (int-quot-trunc-of ?x ?y ?q ?r)
  :why    "{?x} divided by {?y} = {?q}, remainder {?r}")
```

![The rule int-div-trunc-answer of std.math.int, as the engine draws it](diagrams/12-std.math.int.int-div-trunc-answer.svg)
<!-- /rule -->

## Division: the floored convention

The quotient is rounded down, so where the signs differ and a remainder is left its size is one more; the
remainder has the divisor's sign.

#### `int-div-floor-ask`

<!-- rule std.math.int int-div-floor-ask -->
```lisp
(rule int-div-floor-ask ()
  :match  (int-div-floor ?x ?y)
  :assert (operand (int-first (int-div-of (int-div-floor ?x ?y)) ?y) ?x)
  :why    "{?x} divided by {?y}, rounded down, is asked: first, the integer {?x} stands for")
```

![The rule int-div-floor-ask of std.math.int, as the engine draws it](diagrams/12-std.math.int.int-div-floor-ask.svg)
<!-- /rule -->

#### `int-div-floor-pp`

<!-- rule std.math.int int-div-floor-pp -->
```lisp
(rule int-div-floor-pp ()
  :match  (div-done (int-div-m (int-div-floor ?x ?y) pos pos ?b) ?q ?r)
  :assert (int-div-done (int-div-floor ?x ?y) (int ?q z) (int ?r z))
  :why    "neither is negative: the sizes' quotient {?q} and remainder {?r} are the answer")
```

![The rule int-div-floor-pp of std.math.int, as the engine draws it](diagrams/12-std.math.int.int-div-floor-pp.svg)
<!-- /rule -->

#### `int-div-floor-nn`

<!-- rule std.math.int int-div-floor-nn -->
```lisp
(rule int-div-floor-nn ()
  :match  (div-done (int-div-m (int-div-floor ?x ?y) neg neg ?b) ?q ?r)
  :assert (int-div-done (int-div-floor ?x ?y) (int ?q z) (int z ?r))
  :why    "both are negative: the quotient is {?q}, and the remainder {?r} negated")
```

![The rule int-div-floor-nn of std.math.int, as the engine draws it](diagrams/12-std.math.int.int-div-floor-nn.svg)
<!-- /rule -->

#### `int-div-floor-pn-exact`

<!-- rule std.math.int int-div-floor-pn-exact -->
```lisp
(rule int-div-floor-pn-exact ()
  :match  (div-done (int-div-m (int-div-floor ?x ?y) pos neg ?b) ?q z)
  :assert (int-div-done (int-div-floor ?x ?y) (int z ?q) (int z z))
  :why    "the divisor is negative and nothing is left: the quotient is {?q} negated")
```

![The rule int-div-floor-pn-exact of std.math.int, as the engine draws it](diagrams/12-std.math.int.int-div-floor-pn-exact.svg)
<!-- /rule -->

#### `int-div-floor-np-exact`

<!-- rule std.math.int int-div-floor-np-exact -->
```lisp
(rule int-div-floor-np-exact ()
  :match  (div-done (int-div-m (int-div-floor ?x ?y) neg pos ?b) ?q z)
  :assert (int-div-done (int-div-floor ?x ?y) (int z ?q) (int z z))
  :why    "the dividend is negative and nothing is left: the quotient is {?q} negated")
```

![The rule int-div-floor-np-exact of std.math.int, as the engine draws it](diagrams/12-std.math.int.int-div-floor-np-exact.svg)
<!-- /rule -->

#### `int-div-floor-pn-more`

<!-- rule std.math.int int-div-floor-pn-more -->
```lisp
(rule int-div-floor-pn-more ()
  :match  (div-done (int-div-m (int-div-floor ?x ?y) pos neg ?b) ?q (n ?r ?d))
  :assert (add-walk (int-div-q (int-div-floor ?x ?y) neg ?b (n ?r ?d)) ?q (n z 1) 0 z)
  :why    "a remainder is left, (n {?r} {?d}): one more in the quotient's size, {?q} + 1")
```

![The rule int-div-floor-pn-more of std.math.int, as the engine draws it](diagrams/12-std.math.int.int-div-floor-pn-more.svg)
<!-- /rule -->

#### `int-div-floor-np-more`

<!-- rule std.math.int int-div-floor-np-more -->
```lisp
(rule int-div-floor-np-more ()
  :match  (div-done (int-div-m (int-div-floor ?x ?y) neg pos ?b) ?q (n ?r ?d))
  :assert (add-walk (int-div-q (int-div-floor ?x ?y) pos ?b (n ?r ?d)) ?q (n z 1) 0 z)
  :why    "a remainder is left, (n {?r} {?d}): one more in the quotient's size, {?q} + 1")
```

![The rule int-div-floor-np-more of std.math.int, as the engine draws it](diagrams/12-std.math.int.int-div-floor-np-more.svg)
<!-- /rule -->

#### `int-div-floor-neg`

<!-- rule std.math.int int-div-floor-neg -->
```lisp
(rule int-div-floor-neg ()
  :match  (done (int-div-r (int-div-floor ?x ?y) neg ?q) ?t)
  :assert (int-div-done (int-div-floor ?x ?y) (int z ?q) (int z ?t))
  :why    "the quotient is {?q} negated, and the remainder is {?t} negated")
```

![The rule int-div-floor-neg of std.math.int, as the engine draws it](diagrams/12-std.math.int.int-div-floor-neg.svg)
<!-- /rule -->

#### `int-div-floor-pos`

<!-- rule std.math.int int-div-floor-pos -->
```lisp
(rule int-div-floor-pos ()
  :match  (done (int-div-r (int-div-floor ?x ?y) pos ?q) ?t)
  :assert (int-div-done (int-div-floor ?x ?y) (int z ?q) (int ?t z))
  :why    "the quotient is {?q} negated, and the remainder is {?t}")
```

![The rule int-div-floor-pos of std.math.int, as the engine draws it](diagrams/12-std.math.int.int-div-floor-pos.svg)
<!-- /rule -->

#### `int-div-floor-answer`

<!-- rule std.math.int int-div-floor-answer -->
```lisp
(rule int-div-floor-answer ()
  :match  (int-div-done (int-div-floor ?x ?y) ?q ?r)
  :assert (int-quot-floor-of ?x ?y ?q ?r)
  :why    "{?x} divided by {?y} = {?q}, remainder {?r}")
```

![The rule int-div-floor-answer of std.math.int, as the engine draws it](diagrams/12-std.math.int.int-div-floor-answer.svg)
<!-- /rule -->

## An operand's faults

An operand's fault is found by the natural numbers' rules, as a fact about the tag the operand was asked under.
These four carry it from this module's tags to the request, where the shared rules turn it into the flag.

#### `int-bad-only`

<!-- rule std.math.int int-bad-only -->
```lisp
(rule int-bad-only ()
  :match  (bad-operand (int-only ?request) ?f)
  :assert (bad-request ?request ?f)
  :why    "the operand of {?request} is at fault: {?f}")
```

![The rule int-bad-only of std.math.int, as the engine draws it](diagrams/12-std.math.int.int-bad-only.svg)
<!-- /rule -->

#### `int-bad-first`

<!-- rule std.math.int int-bad-first -->
```lisp
(rule int-bad-first ()
  :match  (bad-operand (int-first ?request ?y) ?f)
  :assert (bad-request ?request ?f)
  :why    "the first operand of {?request} is at fault: {?f}")
```

![The rule int-bad-first of std.math.int, as the engine draws it](diagrams/12-std.math.int.int-bad-first.svg)
<!-- /rule -->

#### `int-bad-second`

<!-- rule std.math.int int-bad-second -->
```lisp
(rule int-bad-second ()
  :match  (bad-operand (int-second ?request ?p ?n) ?f)
  :assert (bad-request ?request ?f)
  :why    "the second operand of {?request} is at fault: {?f}")
```

![The rule int-bad-second of std.math.int, as the engine draws it](diagrams/12-std.math.int.int-bad-second.svg)
<!-- /rule -->

#### `int-bad-div`

<!-- rule std.math.int int-bad-div -->
```lisp
(rule int-bad-div ()
  :match  (bad-request (int-div-of ?request) ?f)
  :assert (bad-request ?request ?f)
  :why    "the division {?request} is at fault: {?f}")
```

![The rule int-bad-div of std.math.int, as the engine draws it](diagrams/12-std.math.int.int-bad-div.svg)
<!-- /rule -->

## An integer's own shape

A digit or a named constant where a part goes, checked where the integer is written: each rule has one premise,
`(written (int P N))`, and a row of `nz` or of `named`.

#### `int-p-digit`

<!-- rule std.math.int int-p-digit -->
```lisp
(rule int-p-digit ()
  :match  (and (written (int ?x ?n)) (nz ?x))
  :assert (not-integer (int ?x ?n) ?x)
  :why    "the digit {?x} stands where a natural number goes: it is (n z {?x})")
```

![The rule int-p-digit of std.math.int, as the engine draws it](diagrams/12-std.math.int.int-p-digit.svg)
<!-- /rule -->

#### `int-p-zero`

<!-- rule std.math.int int-p-zero -->
```lisp
(rule int-p-zero ()
  :match  (written (int 0 ?n))
  :assert (not-integer (int 0 ?n) 0)
  :why    "the digit 0 stands where a natural number goes: zero is z")
```

![The rule int-p-zero of std.math.int, as the engine draws it](diagrams/12-std.math.int.int-p-zero.svg)
<!-- /rule -->

#### `int-p-named`

<!-- rule std.math.int int-p-named -->
```lisp
(rule int-p-named ()
  :match  (and (written (int ?x ?n)) (named ?x))
  :assert (not-integer (int ?x ?n) ?x)
  :why    "the name {?x} stands where a natural number goes: a name stands for a number as an operand, not inside one")
```

![The rule int-p-named of std.math.int, as the engine draws it](diagrams/12-std.math.int.int-p-named.svg)
<!-- /rule -->

#### `int-n-digit`

<!-- rule std.math.int int-n-digit -->
```lisp
(rule int-n-digit ()
  :match  (and (written (int ?p ?x)) (nz ?x))
  :assert (not-integer (int ?p ?x) ?x)
  :why    "the digit {?x} stands where a natural number goes: it is (n z {?x})")
```

![The rule int-n-digit of std.math.int, as the engine draws it](diagrams/12-std.math.int.int-n-digit.svg)
<!-- /rule -->

#### `int-n-zero`

<!-- rule std.math.int int-n-zero -->
```lisp
(rule int-n-zero ()
  :match  (written (int ?p 0))
  :assert (not-integer (int ?p 0) 0)
  :why    "the digit 0 stands where a natural number goes: zero is z")
```

![The rule int-n-zero of std.math.int, as the engine draws it](diagrams/12-std.math.int.int-n-zero.svg)
<!-- /rule -->

#### `int-n-named`

<!-- rule std.math.int int-n-named -->
```lisp
(rule int-n-named ()
  :match  (and (written (int ?p ?x)) (named ?x))
  :assert (not-integer (int ?p ?x) ?x)
  :why    "the name {?x} stands where a natural number goes: a name stands for a number as an operand, not inside one")
```

![The rule int-n-named of std.math.int, as the engine draws it](diagrams/12-std.math.int.int-n-named.svg)
<!-- /rule -->

#### `not-integer-is-false`

<!-- rule std.math.int not-integer-is-false -->
```lisp
(rule not-integer-is-false ()
  :match  (not-integer ?x ?thing)
  :assert (false)
  :why    "{?x} is not an integer: the program cannot hold it")
```

![The rule not-integer-is-false of std.math.int, as the engine draws it](diagrams/12-std.math.int.not-integer-is-false.svg)
<!-- /rule -->

## The entry

The closing rule: it closes every relation of the module, names each rule above, and names the entries
`decimal-arith`, `decimal` and `arith`. Its drawing is not shown: it has a node for each of those facts.

#### `int-arith`

<!-- rule std.math.int int-arith -->
```lisp
(rule int-arith ()
  :match  ()
  :assert (and (__closed__ int) (__closed__ nz) (__closed__ not-integer)
               (__closed__ int-simplify) (__closed__ int-simple-of)
               (__closed__ int-neg) (__closed__ int-neg-of)
               (__closed__ int-add) (__closed__ int-sum-of)
               (__closed__ int-sub) (__closed__ int-diff-of)
               (__closed__ int-mul) (__closed__ int-product-of)
               (__closed__ int-cmp) (__closed__ int-cmp-of)
               (__closed__ int-norm) (__closed__ int-done)
               (__closed__ int-add-call) (__closed__ int-mul-call)
               (__closed__ int-cmp-call)
               (__closed__ int-only) (__closed__ int-first)
               (__closed__ int-second)
               (__closed__ int-plus) (__closed__ int-minus)
               (__closed__ int-add-p) (__closed__ int-add-n)
               (__closed__ int-mul-pp) (__closed__ int-mul-nn)
               (__closed__ int-mul-pos) (__closed__ int-mul-pn)
               (__closed__ int-mul-np) (__closed__ int-mul-neg)
               (__closed__ int-cmp-l) (__closed__ int-cmp-r)
               (__closed__ int-div-euclid) (__closed__ int-quot-euclid-of)
               (__closed__ int-div-trunc) (__closed__ int-quot-trunc-of)
               (__closed__ int-div-floor) (__closed__ int-quot-floor-of)
               (__closed__ int-div-done) (__closed__ int-div-of)
               (__closed__ int-div-a) (__closed__ int-div-b)
               (__closed__ int-div-m) (__closed__ int-div-q)
               (__closed__ int-div-r)
               (__closed__ int-rule)
               (int-rule decimal-arith) (int-rule decimal) (int-rule arith)
               (int-rule int-operand) (int-rule int-operand-second)
               (int-rule int-norm-walks) (int-rule int-norm-plus)
               (int-rule int-norm-minus)
               (int-rule int-simplify-ask) (int-rule int-simplify-start)
               (int-rule int-simplify-answer)
               (int-rule int-neg-ask) (int-rule int-neg-start)
               (int-rule int-neg-answer)
               (int-rule int-add-plus) (int-rule int-add-minus)
               (int-rule int-add-norm)
               (int-rule int-add-ask) (int-rule int-add-start)
               (int-rule int-add-answer)
               (int-rule int-sub-ask) (int-rule int-sub-start)
               (int-rule int-sub-answer)
               (int-rule int-mul-first) (int-rule int-mul-second)
               (int-rule int-mul-plus) (int-rule int-mul-third)
               (int-rule int-mul-fourth) (int-rule int-mul-minus)
               (int-rule int-mul-norm)
               (int-rule int-mul-ask) (int-rule int-mul-start)
               (int-rule int-mul-answer)
               (int-rule int-cmp-left) (int-rule int-cmp-right)
               (int-rule int-cmp-sides)
               (int-rule int-cmp-ask) (int-rule int-cmp-start)
               (int-rule int-cmp-answer)
               (int-rule int-div-start) (int-rule int-div-divisor)
               (int-rule int-div-by-zero) (int-rule int-div-pp)
               (int-rule int-div-pn) (int-rule int-div-np)
               (int-rule int-div-nn) (int-rule int-div-rest)
               (int-rule int-div-euclid-ask) (int-rule int-div-euclid-pp)
               (int-rule int-div-euclid-pn)
               (int-rule int-div-euclid-np-exact)
               (int-rule int-div-euclid-nn-exact)
               (int-rule int-div-euclid-np-more)
               (int-rule int-div-euclid-nn-more)
               (int-rule int-div-euclid-neg) (int-rule int-div-euclid-pos)
               (int-rule int-div-euclid-answer) (int-rule int-div-trunc-ask)
               (int-rule int-div-trunc-pp) (int-rule int-div-trunc-pn)
               (int-rule int-div-trunc-np) (int-rule int-div-trunc-nn)
               (int-rule int-div-trunc-answer) (int-rule int-div-floor-ask)
               (int-rule int-div-floor-pp) (int-rule int-div-floor-nn)
               (int-rule int-div-floor-pn-exact)
               (int-rule int-div-floor-np-exact)
               (int-rule int-div-floor-pn-more)
               (int-rule int-div-floor-np-more) (int-rule int-div-floor-neg)
               (int-rule int-div-floor-pos) (int-rule int-div-floor-answer)
               (int-rule int-bad-only) (int-rule int-bad-first)
               (int-rule int-bad-second) (int-rule int-bad-div)
               (int-rule int-p-digit) (int-rule int-p-zero) (int-rule int-p-named)
               (int-rule int-n-digit) (int-rule int-n-zero) (int-rule int-n-named)
               (int-rule not-integer-is-false))
  :why    "integers are computed and never guessed: every relation of the theory is closed")
```
<!-- /rule -->
