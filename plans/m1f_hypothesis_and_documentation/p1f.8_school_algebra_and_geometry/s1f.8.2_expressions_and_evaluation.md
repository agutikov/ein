# S1f.8.2 — Expressions as data, what a variable is, and evaluation under an assignment

**Phase:** [P1f.8](README.md)
**Status:** planned 2026-11-11. Not started. **Since 2026-11-12 it also holds the calculator's expression layer,**
`(calc E)` and `(let NAME E)`, moved here from S1f.6.14 by the user's reply of that day
([§ Moved here from S1f.6.14](#moved-here-from-s1f614-2026-11-12)).
**Depends on:** [S1f.8.1](s1f.8.1_survey_and_design.md)'s rulings: that expression data is accepted, its
spelling, and how it stands to [S1f.6.14](../p1f.6_calculator/s1f.6.14_expressions.md). If the user rules there
that the evaluator of this stage *is* S1f.6.14's answer, this stage closes that one, and says so in both files.

## The instruction's part

> variables, expressions simplifications

and, on the calculator's expression layer (the user, 2026-11-11): *"(let NAME Expression) is questionable, (calc
E) is not function call, so (let NAME (calc E)) is not the case, Ein is not Lisp, is it?"*

## What it builds

**The term language.** An expression is a value under the value rule
([`defined_behaviour.md` § 4.22](../../../docs/kernel/defined_behaviour.md)): a name, or a fact of an ordinary
relation. As planned, to be confirmed at S1f.8.1's stop:

| written | reads |
|---|---|
| `(num N)` | the number N, one of the calculator's (and, after S1f.6.7 and S1f.6.8, an integer or a rational) |
| `(var x)` | the variable named `x` |
| `(Plus A B)`, `(Minus A B)`, `(Times A B)` | A + B, A − B, A · B |
| `(Neg A)` | −A |
| `(Over A B)` | A / B, a fraction of expressions; whether it is in the first cut is S1f.8.1's to say |
| `(Pow A K)` | A to the power K, K a number written in the program: a power is repeated multiplication, and no rule computes with an exponent that is an expression |

Why `(num N)` and `(var x)` and not the bare number and the bare name: a rule must tell a number from a variable
from a compound without negation, and a pattern can only tell heads apart. `(n ?p ?d)` is a number's shape but
`z` is a name, as `x` is, and no premise says *this name is not a variable*. Whether the wrappers can go, by
declaring the variables (`(variable x)`) and matching on that, is tried and measured: it trades a constructor
for a join.

Why binary `Plus`: Ein has no variadic relation (the calculator stopped asking for one). A sum of many terms is
a nest, and its associativity is the normal form's business, [S1f.8.3](s1f.8.3_simplification_and_identities.md).

**The page on variables.** The three senses of the phase's
[README § What a variable is](README.md#what-a-variable-is), each with a program: a rule that is general over
object-level variables; the same letter in two problems of one program, and what keeps them apart (a problem's
name in the assignment, `(assign p1 x N)`, or one problem a file); and the names the theory occupies, `unknown`
among the ones it cannot have (F-6).

**Evaluation.** `(evaluate E)` is a request, answered by `(value-of E N)`, under the program's assignments
`(assign x N)`. Requests go down the tree, values come up through the calculator's requests. Division inside an
expression follows the calculator's ruling: a zero divisor derives the flag and the flag `(false)`. A variable
with no assignment gets no value, and that is reported by name, as S1f.6.20 reports a missing quantity: the
stage tries S1f.6.20's ruled mechanism first and does not invent another.

**Substitution** of an expression for a variable, `(substitute E x F)`, answered by the resulting expression:
the same walk, building a tree in place of a number. S1f.8.5 uses it to solve a system.

## Sketch

**Sketch only, to be replaced by programs whose answers are computed.** The planning agent ran this once on
2026-11-11 (probe A-7): the binary answered
`(value-of (Plus (Times (num (n z 3)) (var x)) (num (n z 4))) (n (n z 1) 9))`, no candidate, k_s 1. The rules
for `Times` are the three for `Plus` with `mul` and `product-of`.

```lisp
(import std.arith :symbols (arith))

(relation Plus Expr Expr       :why "{?1} plus {?2}")
(relation Times Expr Expr      :why "{?1} times {?2}")
(relation num Number           :why "the number {?1}")
(relation var Name             :why "the variable {?1}")
(relation assign Name Number   :why "{?1} is given the value {?2}")
(relation evaluate Expr        :why "asked: the value of {?1}")
(relation value-of Expr Number :why "the value of {?1} is {?2}")

;; "Find the value of 3x + 4 when x = 5."
(assign x (n z 5))
(evaluate (Plus (Times (num (n z 3)) (var x)) (num (n z 4))))

(rule eval-num ()
  :match  (evaluate (num ?k))
  :assert (value-of (num ?k) ?k)
  :why    "a number is its own value")
(rule eval-var ()
  :match  (and (evaluate (var ?v)) (assign ?v ?k))
  :assert (value-of (var ?v) ?k)
  :why    "{?v} stands for {?k}")
(rule eval-plus-parts ()
  :match  (evaluate (Plus ?a ?b))
  :assert (and (evaluate ?a) (evaluate ?b))
  :why    "to value a sum, value its two parts")
(rule eval-plus-ask ()
  :match  (and (evaluate (Plus ?a ?b)) (value-of ?a ?x) (value-of ?b ?y))
  :assert (add ?x ?y)
  :why    "the parts are {?x} and {?y}: add them")
(rule eval-plus ()
  :match  (and (evaluate (Plus ?a ?b)) (value-of ?a ?x) (value-of ?b ?y) (sum-of ?x ?y ?s))
  :assert (value-of (Plus ?a ?b) ?s)
  :why    "{?x} + {?y} = {?s}")

(solve :hypothesis-relations ())
(query :goal (value-of (Plus ?a ?b) ?v)
       :expect (model (value-of (Plus (Times (num (n z 3)) (var x)) (num (n z 4)))
                                (n (n z 1) 9))))
```

**What the sketch does wrong on purpose.** `eval-plus-ask` and `eval-plus` join `(evaluate (Plus ?a ?b))` with
`(value-of ?a ?x)` on the nested value `?a`, and with `(sum-of ?x ?y ?s)` on two numbers: the joins the
calculator's rule of the theory forbids, because the fact index does not key on a nested value (E-3 of the
calculator's [`findings.md`](../p1f.6_calculator/findings.md)). S1f.6.3's record adds that a premise *every slot
of which is bound* is one lookup whatever its arguments are, so `(sum-of ?x ?y ?s)` with `?s` free is the costly
one and `(evaluate ?e)` re-asked with `?e` bound is not. Task 3 measures it and rewrites with continuation tags
if the measure says so.

## Tasks

1. **The term language,** as ruled, with the wrappers kept or dropped on the measurement above; every head
   declared with a `:why` that reads as the school phrase.
2. **The page on variables,** each sense with its program.
3. **Evaluation and substitution,** each in two versions on the prototype: the natural rules (the sketch) and
   tagged rules after the calculator's design K. For each: firings and depth on one expression, and the
   matcher's candidates a request at 250, 1 000 and 4 000 requests (the calculator's `arith_batch.rs` is the
   model). The version that ships is chosen on the table, and the page shows both.
4. **The tests:** every value against a Rust evaluator that shares no code with the theory, on every expression
   up to a stated depth over a small set of numbers and one or two variables, and on a sample of deeper ones;
   the search-free test; an unassigned variable reported and not valued; a division by zero inside an
   expression dead as ruled.
5. **The examples,** each a corpus entry with an `:expect`: S1f.8.1's problem 1, a nested expression, the same
   expression at two assignments in one program.
6. **S1f.6.14, closed or left:** the stage's record says which, in the user's words.

## Acceptance

- An expression of the term language is evaluated under an assignment by saturation alone: no candidate, k_s 1,
  R {∅}, held by the search-free test.
- Every value agrees with the Rust evaluator up to the bound.
- A batch's cost is reported on the matcher's counters, and is linear, or the page says it is not and why that
  was accepted.
- A missing assignment and a zero divisor each have a stated outcome, held by a test.
- The page on variables exists, each sense with a program.

## What it may find

- **That `(evaluate E)` is `(calc E)` under another name.** It differs in what it stands on: `calc`'s tree was
  made of the calculator's own request heads, and this tree is data with heads of its own, which a rule can
  also simplify, compare and solve. Whether that difference matters to the user is S1f.8.1's question a; this
  stage does not argue it.
- **That two problems in one program share their letters.** `x` is a name of the program, so `(assign x 5)` and
  `(assign x 7)` in one file are two values of one variable. The stage says what the program reports (two
  values, both derived; no contradiction unless a rule says so) and what a problem should write.
- **Engine findings,** to [`findings.md`](findings.md).

## Moved here from S1f.6.14 (2026-11-12)

[S1f.6.14](../p1f.6_calculator/s1f.6.14_expressions.md) asked whether the calculator gets an expression layer,
`(calc E)` and `(let NAME E)`, and waited. Question 10 of the calculator's
[`questions.md`](../p1f.6_calculator/questions.md#q10) put it to the user with three options, (a) none, (b)
`(let NAME E)` only, (c) both. The user, 2026-11-12:

> both go into p1f.8, where expressions naturally belong to, with check of used names, with check of recursive
> name usage (let a (+ a 1)) and other

So **both are ruled in, and here.** That settles the second half of the README's question a (*is an evaluator
over expression data the answer to S1f.6.14?*): the expression layer is this phase's, not the calculator's
core, which stays requests and results. It does not settle how the two are spelt or whether they are one
thing with `(evaluate E)`; that is still S1f.8.1's to show on programs:

- **One layer or two.** This stage's `(evaluate E)`, answered by `(value-of E N)`, over expression *data*
  with heads of its own (`Plus`, `Times`), and S1f.6.14's `(calc E)`, whose tree was made of the calculator's
  request heads (`add`, `div`). The plan's expectation is one evaluator and one term language, with `calc`
  the request's name if the user prefers it; S1f.8.1 shows the same three programs both ways.
- **The spelling inside an expression.** The user's example writes `(let a (+ a 1))`. Whether `+` is a name
  the grammar accepts as a relation's head is checked on the release binary first (the name rule is an
  allow-list of ASCII characters); if it is, `+ − × ÷` as heads is an option beside the capitalised words,
  and the README's question b is asked with it.
- **`(let NAME E)` beside `(let NAME NUMBER)`.** The calculator's `let` names a number and is built
  (reply 11: *"(a) if it is already built, why asking..."*). Naming an expression's value is the same
  relation with an expression in the number's place, or a relation of its own; the first makes `let` a
  relation two modules write rules for, which [S1f.6.30](../p1f.6_calculator/s1f.6.30_name_clashes_between_imported_theories.md)'s
  investigation of mixing theories bears on.

**What was already settled goes with it** (S1f.6.14, the user's words of 2026-11-11): `sub` inside an
expression is strict, with a value when the first is not the smaller and otherwise `(negative (sub a b))` and
no value; division is two heads, `div` for the quotient and `mod` for the remainder. Round 2 of the
calculator's design built the layer on a prototype
([`design.md` § C](../p1f.6_calculator/design.md#ii-nested-expressions-evaluated-on-demand)), three rules an
operator, and that is the starting text.

### The checks the reply names

**A check of used names.** An expression that uses a name no `let` and no assignment states has no value.
The calculator's own case, `(add foo X)` with no `(let foo …)`, is ruled on in the calculator's phase
([S1f.6.12's build](../p1f.6_calculator/s1f.6.12_malformed_numbers.md#the-build-as-ruled-2026-11-12):
`(undeclared NAME REQUEST)` through one `absent` over `let`, then `(false)`, on round 2's default). This
stage follows whatever stands there when it runs, at every depth of an expression, and reports the name and
the expression that used it. This stage's own text above says *a variable with no assignment gets no value,
and that is reported by name, as S1f.6.20 reports a missing quantity*; the two are reconciled at S1f.8.1's
stop, since *undeclared* (a slip, dead) and *not given* (a gap, named) are different faults and an algebra
problem has both.

**A check of recursive name usage.** `(let a (+ a 1))`: a name whose value is defined through itself.
Evaluated on demand it does not answer and does not stop being asked: the request for `a` asks for `a`.
What it does on the release binary today is the task's first probe (a request that is its own
sub-request is one fact, so saturation may simply reach a fixpoint with no value; or a longer cycle,
`a` through `b` through `a`, may do otherwise). The check derives a flag that names the cycle,
`(recursive NAME)`, by a rule that follows *uses* from name to name: a transitive closure over the names a
`let`'s expression mentions, monotone, with no negation. Whether the flag is then `(false)`, as the
calculator's faults are, is asked with the degenerate equations of S1f.8.4, since *a = a + 1* is also an
equation with no solution.

**"and other".** The reply leaves the list open. What the stage adds unasked, each shown before it is built:
a name stated twice with two expressions (the calculator's twice-named constant, two answers to one request
today); an operator with the wrong count of operands (refused at load once
[S1f.6.23](../p1f.6_calculator/s1f.6.23_the_loaders_arity_check.md) has landed); a division by zero inside an
expression (the calculator's flag, at the call since [S1f.6.24](../p1f.6_calculator/s1f.6.24_division_by_zero_at_the_call.md));
a name that is both a variable of a problem and a `let` constant.

### Tasks added

7. **The two layers on the same programs,** for S1f.8.1's page: `(calc E)` over request heads and
   `(evaluate E)` over data, the examples of `design.md` § C respelt, with answers computed.
8. **`(let NAME E)`,** evaluated on demand, to any depth, under the rule of the theory (an evaluation
   carries a tag; no rule joins two facts on a number).
9. **The check of used names,** as the calculator's phase left it.
10. **The check of recursive use,** with the probe of today's behaviour first, direct and through a longer
    cycle.
11. **The other checks,** each shown first.
12. **Tests:** every check a program that trips it and a twin that does not; the Rust evaluator of task 4
    extended to named expressions; a batch's cost on the counters, by hand.

### Acceptance added

- `(calc E)` and `(let NAME E)` answer as `design.md` § C's examples showed, search-free.
- A used name nothing states, and a name defined through itself, are each reported by name, by a rule, and
  held by a test.
- The calculator's core is unchanged: no rule of `std.arith` reads an expression.
