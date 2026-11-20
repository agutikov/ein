# Q-M1f.567 — A hypothesis rule is a rule that concludes `(open F)`

A rule whose `:assert` concludes `(open F)` states a hypothesis
([Q-M1f.566](Q-M1f.566-what-a-state-owes-is-open-r-a-fact-and-open-f.md),
[Q-M1f.573](Q-M1f.573-under-open-a-relation-is-owed-and-a-fact.md)), and it is the one way to write one: there is no
`hrule` form and no `(solve :hrules …)` clause. Such a rule is activated as every rule is, by a fact of its name the
program writes anywhere in it, `(guess color-loc Color House)`, or by none where it has no parameter; an activator of
another count than the rule's parameters activates nothing. A program that writes `(hrule …)` is refused at the parse,
since `hrule` is a name and the form a fact whose arguments are no facts, and one that writes `:hrules` is refused at
the keyword, which `(solve …)` does not take.

```lisp
(rule guess (?R ?T)
  :match  (is-a ?x ?T)
  :assert (open (?R ?x))
  :why    "guess: is {?x} {?R}?")
(guess chosen Drink)
(guess served Drink)
```

The checks the `:hrules` list made of its own went with it: that a name it lists is a declared hypothesis rule, that an
activator gives the rule as many arguments as it has parameters, and that an argument at a parameter that names a
relation is a relation of the program. An activator is a fact as any rule's is, read by the checks every activator is
read by.

**Why.** Every mechanism that works with hypotheses works with facts under `open`: the form once produced hypotheses,
and any rule that concludes `(open F)` now does. So neither the form nor a special place in `(solve …)` for its
activators is needed: one way to write a rule, and one way to activate it.

**Held by.** `ein-ir/src/ein.y`, which has no `hrule` and no `:hrules`; the grammar's table
(`ein-ir/tests/grammar_pages.rs`), its rows *a fact's argument, the removed hrule form* and *a solve's keyword,
the removed :hrules*; `ein_infer::hypothesis_rules`, which compiles one plan per activator the state holds; the
corpus, every hypothesis rule written so, `examples/zebra2.ein` among them;
[hypotheses](../../ein/user/03-syntax-and-semantics/09-hypotheses.md).
