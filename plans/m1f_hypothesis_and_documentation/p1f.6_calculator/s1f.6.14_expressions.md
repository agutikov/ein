# S1f.6.14 — Optional: nested expressions, `(calc E)` and `(let NAME E)`

**Phase:** [P1f.6](README.md)
**Status:** **moved to P1f.8 on 2026-11-12, by the user's reply of that day** ([§ Moved](#moved-to-p1f8-2026-11-12)).
Nothing of it is built in this phase. Planned 2026-11-11; it waited for the user until then.
**Depends on:** [S1f.6.6](s1f.6.6_division.md), and the user's ruling. [S1f.6.20](s1f.6.20_word_problems.md) is
where the need for it will show.

## Why it waits

The user, 2026-11-11: *"(let NAME NUMBER) is ok, (let NAME Expression) is questionable, (calc E) is not function
call, so (let NAME (calc E)) is not the case, Ein is not Lisp, is it?"*

So the core theory ships without an expression layer. A program computes by request facts and result facts, and
composes them with rules of its own: a rule from an answer to the next request, as `design.md`'s chained
program, its gcd and its factorial do. Named constants, `(let NAME NUMBER)`, are in the core (S1f.6.3).

Round 2 built an expression layer on the prototype and it worked
([design.md § C](design.md#ii-nested-expressions-evaluated-on-demand)): one fact holds a tree,
`(calc (add N1 (div N2 N3)))`, three rules per operator evaluate it on demand, and `(let NAME E)` names a value.
What it would be is a second way to say what rules already say, in a shape that looks like calling functions.
That is the user's objection, and this stage does not argue with it.

## What the user is asked, when the need shows

S1f.6.20 writes word problems, where *"together means sum"* is a rule that turns facts about people into
arithmetic. Its programs will show how often a quantity is the result of two or three operations, and what that
costs to write as rules. The question then has evidence:

- **(a)** No expression layer. Every composition is a rule of the program's. The smallest theory, and Ein stays
  a language of facts and rules.
- **(b)** `(let NAME E)` only, with E one level deep or any depth: named quantities, no anonymous `calc`.
- **(c)** Both, as round 2 built them.

If it is ruled in, two things are already settled by the user: **`sub` inside an expression is strict** (*"yes"*):
it has a value when the first is not the smaller, and otherwise `(negative (sub a b))` and no value; and
division is two heads, `div` for the quotient and `mod` for the remainder.

## Tasks, if ruled in

1. The layer as a module over `std.arith`, imported on its own, so a program that does not want it does not
   load it.
2. **Rewritten to the rule of the theory.** Round 2's rules join an expression with its operands' values, on
   numbers; here an evaluation carries a tag, as every circuit does.
3. The tests, the search-free test, and S1f.6.3's test extended to the module.
4. The examples of `design.md` § C, respelt.

## Acceptance

- Not started without a ruling.
- If built: the expression programs of `design.md` answer as that page showed, and a batch of them costs in
  proportion to its count.

## Moved to P1f.8 (2026-11-12)

[Question 10](questions.md#q10) put this stage's three options to the user with what the word problems had
shown (*every quantity took two rules per word*; *no rule needed two operations at once*). The user,
2026-11-12:

> both go into p1f.8, where expressions naturally belong to, with check of used names, with check of recursive
> name usage (let a (+ a 1)) and other

So `(calc E)` and `(let NAME E)` are both ruled **in**, and **in P1f.8**, not in the calculator's core: the
stage that takes them is
[S1f.8.2](../p1f.8_school_algebra_and_geometry/s1f.8.2_expressions_and_evaluation.md#moved-here-from-s1f614-2026-11-12),
with the two checks the reply names. This file stays as the record of the question; its
[§ Tasks, if ruled in](#tasks-if-ruled-in) are superseded by that stage's, and the two things already settled
(`sub` inside an expression is strict; division is two heads, `div` and `mod`) go with it.

In this phase the calculator's core stays requests and results, and `(let NAME NUMBER)` stays as built
([reply 11](questions.md#q11): *"(a) if it is already built, why asking..."*).
