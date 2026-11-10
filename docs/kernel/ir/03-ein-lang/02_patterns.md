# Pattern sub-language

The mini-language used inside rule `:match` / `:assert` clauses. The
**surface** form of the three rule types from
[`../01-ein-graph/02_rules.md`](../01-ein-graph/02_rules.md).

This was [`docs/ir.md` §3](../../README.md) before the kernel-
documentation split.

---

## Closure

The pattern language is **positive conjunctive** + two computed relations,
`eq` and `neq`, each written as a premise, `(neq ?a ?c)` (two computed
predicates that bound nothing until M1f S1f.5.7, § Predicate registry). (The rewrite-DSL +
named-predicate fallback of
[Q4](../../../../plans/open_questions.md#q4--rule-presentation-language)
chose the first half; the *named structural predicate* registry the second
half proposed was never built — § Predicate registry below.)

A rule takes `:match`, `:assert` and `:why` and no other keyword, and a term
takes none: `:where`, the filter clause this page described until M1f
[S1f.5.11](../../../../plans/m1f_hypothesis_and_documentation/p1f.5_the_no_owner_items/s1f.5.11_closed_keywords.md),
is refused at the parse wherever it is written (the user's ruling of
2026-11-07,
[Q-M1e2.28](../../../../plans/m1e2_correctness/open_questions.md#q-m1e228--which-keywords-does-each-form-take)).
[Q32](../../../../plans/open_questions.md#q32--where-semantics) dropped
`:where` for the positional premise on 2026-05-20, and a pair written anyway
was dropped by the compiler, in silence, until S1f.5.5's swap refused it;
S1f.5.11's T1f.5.11.2 retired the drop.

**Where a malformed primitive is caught** is one of two places, and the split
is not stylistic: the grammar, where a rule writes the primitive, and the
loader, after macro expansion, where a macro's body or a call's argument does
(the compiler until M1f S1f.5.24, below). Either way every command refuses the
program before any path compiles, fires, proposes or owes the rule.

| primitive | arity | refused by |
|---|---|---|
| `not` | 1 | the **grammar** — `Connective ::= '(' 'not' Pattern ')'` in a premise, `Conclusion ::= '(' 'not' Pattern ')'` in a conclusion: one pattern, and no keyword after it |
| `eq` / `neq` | 2 | the **grammar** — `Guard ::= '(' 'eq' Term Term ')' \| '(' 'neq' Term Term ')'` |
| `absent` | 1 | the **grammar** — `'(' 'absent' Premise ')'` |
| `and` / `or` | 1+ | the **grammar** — `Premises ::= Premise \| Premises Premise`, so `(and)` and `(or)` are parse errors, each with a note naming the rule and the clause (M1f S1f.5.9), and `(and X)` is legal, if degenerate; the empty conjunction is `:match ()`, below |
| `false` | 0 | the **grammar** in a conclusion, `'(' 'false' ')'`; as a premise's head `false` is a position word, a pattern over the relation `false` |

**The empty conjunction is `()`**, written as a whole `:match`, and it is
true: the rule fires once, or once per activator fact, and an hrule proposes
its conclusion likewise (M1f
[S1f.5.9](../../../../plans/m1f_hypothesis_and_documentation/p1f.5_the_no_owner_items/s1f.5.9_an_empty_match_fires.md), the user's
ruling of 2026-11-07 on F12). `()` as a whole `:assert` is the empty
conclusion, which asserts nothing. `(and)` means what `()` means, and is
refused, since `and` takes a premise; `()` is no premise inside another, the
grammar writing it only as a whole clause. Until S1f.5.9 `()` was a premise
over the parser's synthetic `#empty`, which no fact holds, so the rule never
fired (W31), and `:assert ()` stored a fact under that head (W34).

**Since M1f S1f.5.5's swap** (2026-11-09) each row is a production of
[`ein.y`](../../../../ein.rs/crates/ein-ir/src/ein.y), the grammar
[`00_ebnf.md` § 2](00_ebnf.md#2-phrase-grammar) prints. A macro's body is a
`Template`, which takes any head at any arity, and a call takes any premise as
an argument, so the grammar never reads the expanded rule: `(macro two (?a ?b)
(absent ?a ?b))`, `(macro app (?R ?a) (?R ?a))` called `(app eq ?x)`, `(macro
neg (?a) (not ?a))` called with a conjunction, `std.macro`'s `unknown` called
with one, and as a conclusion `(eq ?a)`, `(false ?a)`, an `(or …)` or a body
that is a bare parameter. **Since M1f
[S1f.5.24](../../../../plans/m1f_hypothesis_and_documentation/p1f.5_the_no_owner_items/s1f.5.24_a_macro_body_checked_where_a_rule_is.md)
the loader reads it** (`ein_ir::from_ir`'s `refuse_malformed_primitives`): a
rule's, an hrule's and an obligation's clauses, expanded, are held to the shapes
the table gives a written rule, by position (a premise, a pattern's argument, a
value, the pattern a `not` negates, a conclusion, which is a form), and a form
in another shape is refused at load, on every command, the views and `kb save`
among them, one line each in the compiler's words since
[S1f.5.21](../../../../plans/m1f_hypothesis_and_documentation/p1f.5_the_no_owner_items/s1f.5.21_a_compile_error_in_eins_words.md):
the place where the expanded form is written, the rule, the form as expanded
and what to write, `examples/broken/load/eq_in_a_macro_body.ein:23:18: rule
'r': its :match writes (eq ?x), and eq takes exactly 2 arguments, not 1 — it
compares two values, so write one guard per pair` (`defined_behaviour.md`
§ 4.16; `ein-cli/tests/macro_bodies.rs`). A macro no rule calls is not read,
so it loads whatever its body: the user's ruling of 2026-11-10 on
[Q-M1f.67](../../../../plans/m1f_hypothesis_and_documentation/open_questions.md#q-m1f67--where-is-a-macro-body-that-no-rule-could-use-refused),
*"the ultimate goal in this case is not allow execution of (eq <arg>)"*.
**A query's `:goal` is a premise, and since M1f
[S1f.7.5](../../../../plans/m1f_hypothesis_and_documentation/p1f.7_findings_outside_scope/s1f.7.5_a_macro_called_in_a_goal.md)
a macro call in one is expanded and read the same way**: `(query :goal (unknown
(paid ?g)))` asks what `(and (absent (paid ?g)) (absent (not (paid ?g))))`
asks, the expanded goal is held to the table's shapes, and a `std.macro` call
the program did not import is refused as it is in a rule. Until then only a
rule's clauses were expanded, and a call in a goal was a pattern over a
relation of the macro's name, which matched nothing
([`defined_behaviour.md` § 4.21](../../defined_behaviour.md#421-a-goal-is-held-to-what-a-rules-match-is-held-to--m1f-s1f75-s1f711-s1f71)).
A macro call written as a fact, or listed in a claim, is not expanded.
**Until S1f.5.24** the compiler refused `eq` and `absent` at another arity
where it compiled the plan (M1e
[S1e.2.1](../../../history/m1e_review_processing/README.md#s1e21--correctness-the-panic-the-guard-the-traversal),
[Q-M1e.18](../../../history/m1e_review_processing/open_questions.md#q-m1e18--three-kernel-primitives-are-not-shape-pinned-and-drop-their-extra-arguments)),
so a rule only a commitment activated, an hrule, an obligation and a rule no
activator reached loaded and some or every command answered (W66), the views
and `kb save` accepting the program; and the rest went unchecked, a `not` of no
fact and a connective as a value matched (W67), `(eq k)`, `(false k)`, an `(or
…)`, `(not k)` and a conjunction as a value concluded or proposed (W68), and a
conclusion that is no form stopped only the firing that met it (W69). Until
S1f.5.21 the compiler's line quoted the Python IR's repr and placed it `at
Loc(file=…, line=…, col=…)`. **An `(or …)` written as a premise** is no
primitive at another arity but a shape, and the loader refuses it since
S1f.5.21, a macro's expansion included (`defined_behaviour.md` § 4.15).
**Until the swap** the hand-written grammar pinned `not` (`NotForm ::= '('
'not' Value KwPair* ')'`), `neq` (`NeqForm`) and `and` / `or` (`Value+`),
and `eq` and `absent` were plain `SYMBOL`s a rule could pattern-match on,
checked by the compiler alone; `false` took any arguments, ignored by design.
Relation patterns (`(?r ?a ?b)`, `(co-located ?a ?b)`) stay generic at the
grammar level; the loader and compiler enforce well-formedness against the
rules below.

| construct                | example                                | reserved? | meaning                                         |
|--------------------------|----------------------------------------|-----------|-------------------------------------------------|
| variables                | `?a`, `?house`                          | —         | bound by the match; reused across sub-clauses   |
| ground atoms             | `Red`, `House-1`                        | —         | match literally                                 |
| relation pattern         | `(?R ?a ?b [?c …])`                     | —         | the VAR head is one of the rule's parameters, which its activator binds to a relation's name; args bind positions. A head variable no parameter binds, at the top level or inside a pattern, in either clause, is **refused at load** since M1e2 S1e2.2.24 (W24): this row read *VAR head binds the relation name*, and a premise binding it, `(and (rel ?r) (box (?r b)))`, bound nothing — the pattern inside matched nothing, in silence, and one at the top level stopped compilation. A parameter its activator binds to a nested fact is refused too, at load where the activator is written and by the compiler where a rule derives it |
| named relation pattern   | `(co-located ?a ?b)`                    | —         | match a specific relation's instances                  |
| ~~argument wildcard~~    | `(holds _)`, `(pair ?a _)`              | —         | **refused at the parse** since M1f S1f.5.5's swap, in every position (S1f.5.1, [Q-M1e2.22](../../../../plans/m1e2_correctness/open_questions.md#q-m1e222--does-ein-need-a-wildcard)): write a variable used once, `(holds ?any)`, which matches any value and is read nowhere else. This row read *match any value, binding nothing; two are independent* (M1e2 S1e2.2.20 until the swap), and `(eq ?x _)` was refused at load from S1e2.2.24; S1f.5.1's T1f.5.1.3 retired the slot (`ef5374a1`) |
| ~~head wildcard~~        | `(_ ?a ?b)`                             | —         | **refused at the parse** since M1f S1f.5.5's swap, as every bare `_` is (S1f.5.1), and at load from M1e2 S1e2.2.20 (Q-M1e2.14 (c)): a relation is a name or a parameter its activator binds. This row read *match any binary list head*, which no engine did: the compiler stopped on it (W20) |
| **conjunction**          | `(and <p1> <p2> …)`                     | ✓ AND    | conjunctive match (kernel primitive)             |
| **disjunction**          | `(or  <p1> <p2> …)`                     | ✓ OR     | disjunctive match (grammar-reserved; engine semantics in P1.3) |
| **negation**             | `(not <p>)`                             | ✓ NOT    | wrapped premise must not hold                    |
| equality fact            | `(= ?a ?b)`                             | —         | matches a **stored** `(= …)` fact — see below; `=` is a name since M1f S1f.5.5's swap (S1f.5.4's name rule), and this row's mark was ✓ EQ, the hand-written grammar's reserved head, until then |
| **computed equality**    | `(eq ?a ?b)` / `(neq ?a ?b)`            | —         | the relations of equality and non-equality, computed and never stored: a test where both sides are bound, and a binding where one is free (since M1f S1f.5.7; until then two predicates that compared the bindings and bound nothing) |
| membership check         | `(is-a ?a ?T)`                          | —         | ordinary relation pattern; this row's mark was ✓ RELATION until M1f S1f.5.5's swap, though no production was ever `is-a`'s |
| ~~`:where` filter~~      | `:where (transitive ?r) (neq ?a ?b)`    | —         | **refused at the parse** since M1f S1f.5.5's swap, as every keyword a rule does not take and every keyword pair inside a term is (S1f.5.11, Q-M1e2.28): write each guard as a premise beside the patterns, `(and (?r ?a ?b) (?r ?b ?c) (neq ?a ?c))`. This row read *type / inequality / structural-predicate filters*; Q32 (2026-05-20) dropped `:where` for the positional premise, and the compiler dropped a pair written anyway, in silence, until the swap; S1f.5.11's T1f.5.11.2 retired the drop |
| relation-property tag    | `(transitive ?r)`                       | —         | an ordinary premise matching a stored property fact, a join and not a test; this row wrote it `:where (transitive ?r)` until S1f.5.11 |

**A premise is a form** — a pattern, a guard, an `(absent …)`, or an
`(and …)` or `(or …)` of premises. An atom (a run of digits among them, an
integer until M1f S1f.5.4), a variable, `_` or a string (the last two at the
parse since M1f S1f.5.5's swap, S1f.5.1 and S1f.5.2) written where a premise goes is **refused at load** since M1e2
S1e2.2.24 (W24, [`defined_behaviour.md` § 4.9](../../defined_behaviour.md#49-a-rule-shape-that-reads-nothing--m1e2-s1e2224)):
until then the compiler dropped it in silence, so `:match _` fired with no
premise at all and `(and (hit ?y) foo)` on `(hit ?y)` alone. **A keyword
pair anywhere in a premise is refused at the parse** since M1f S1f.5.5's swap
(S1f.5.11): a term takes no keyword. Until then one as a premise of an `(and
…)`, Q32's `:where`, was dropped, as was one at a pattern's top level, and
one inside a pattern inside a pattern, `(box (a b :k v))`, matched nothing
until S1e2.2.24 refused it at load.

The ✓-marked heads, and `eq`, `neq` and `absent`, have productions with
fixed arities, so `(neq ?a)` is a parse error (the table above). **A misspelled relation is not**: `instance` has
not been a grammar-reserved head since S1.7.6, so `(instnce ?a ?T)` is a
perfectly good generic pattern over a relation that auto-vivifies, and a rule
containing it loads, compiles and never fires. Measured, M1e S1e.2.2: exit 0,
`solutions (k) 1`. Nothing refuses it, because the loader is deliberately
**open-world tolerant** — an undeclared head auto-vivifies a
`Relation(declared=false)`
([`../02-data-model/02_store.md` §3](../02-data-model/02_store.md)). What
catches a typo'd head is the *symptom*: a rule that never fires. Ask
[`--events`](../../inference/events.md) for the `fire` stream, the way
[`utils/stdlib_census.py`](../../../../utils/stdlib_census.py) does for the
whole stdlib.

### `=` is a fact head, not a unifier

`=` is a name like any other since M1f S1f.5.5's swap (S1f.5.4's name
rule): `(= a b c)` is a fact of the relation `=`, and until the swap the
equality row's ✓ marked a reserved head of the hand-written grammar, two
arguments and a `:source`, not an engine step. The distinction catches
people out because the two spellings look interchangeable and are not:

- **`(= ?a ?b)`** is an ordinary relation pattern whose head happens to be
  `=`. It matches the `(= …)` *facts the KB holds* — including ones a
  rule derived, since `:assert (= ?a ?a)` is a legal conclusion — and nothing
  else. A rule `:match (and (p ?a ?b) (= ?a ?b))` does **not** fire on
  `(p x x)`; it fires once `(= x x)` is in the KB.
- **`(eq ?a ?b)`** is the computed test. It fires on `(p x x)` with no stored
  fact at all, because it compares the bindings.

Equality has no *semantics* in M1 beyond that: a stored `(= a b)` licenses no
substitution and joins no congruence closure. The engine carries an
`EqClasses` union-find and does not drive it —
[`../02-data-model/02_store.md` §8](../02-data-model/02_store.md) records the
seam as deliberate, for a future e-graph promotion (F4 Q30). What `=` *does*
have is a rendering: an arity-2 equality fact draws as a `doublecircle`
equality-class node with an edge to each side
([`04_dot_rendering.md`](04_dot_rendering.md)), which is why
[`examples/syntax/equality.ein`](../../../../examples/syntax/equality.ein)
exists at all — no puzzle in the corpus writes one.

## What is NOT in the pattern language

- **Negation-as-failure spelled as `(not …)`** — `(not <p>)` in a `:match`
  matches a **stored** `(not p)` fact, uniform with every other pattern; it is
  not failure-to-prove. The NAF operator is `(absent <p>)`, which is a *query*
  answered once at the closure/world boundary
  ([`../../inference/absent_semantics.md`](../../inference/absent_semantics.md)).
- **Universal quantifiers / aggregates as expressions** — `forall` is
  available as a **macro** over nested `absent`s
  ([`std.macro`](../../../../stdlib/macro.ein)), not as a primitive. A
  counting or cardinality aggregate has no spelling at all, and none is
  planned: Ein has no numbers (the user, 2026-11-06,
  [Q-M1e2.24](../../../../plans/m1e2_correctness/open_questions.md#q-m1e224--are-there-numbers-in-ein)).

The line is governed by trace fidelity: anything the matcher can see, the
trace planner can name. An opaque host-language fallback would render as a
black-box firing, failing the M1 acceptance §3 explanation-completeness
criterion — which is the argument that kept the registry below at two
computed predicates rather than opening it.

## Predicate registry — **two**, and why

[`predicates.rs`](../../../../ein.rs/crates/ein-infer/src/predicates.rs) is
the whole registry, and its first line says so: *"the built-in predicate
registry — `eq` and `neq`, and nothing else."*

| predicate       | arity | meaning                                | engine site |
|-----------------|:-----:|----------------------------------------|-------------|
| `(eq ?a ?b)`    | 2     | the two values are one; a free side is bound to the other | matcher `Guard` opcode |
| `(neq ?a ?b)`   | 2     | the two values differ; a free side ranges over the program's names but the other | matcher `Guard` opcode |

**`eq` and `neq` are relations** since M1f
[S1f.5.7](../../../../plans/m1f_hypothesis_and_documentation/p1f.5_the_no_owner_items/s1f.5.7_eq_and_neq_are_relations.md), on the user's
ruling of 2026-11-07 (Q-M1e2.26): `(eq x x)` for every value, `(neq x y)` for
every two different values, computed and never stored, in a goal, a premise
and a conclusion alike. **A guard's argument is a value**: a variable, a name
(a run of digits is a name, an integer until M1f S1f.5.4), or a fact that holds
no variable, `(f a)`, as in `(p (f a))`. A form that holds a variable is
**refused at load** (Q-M1f.30), and `_` and a keyword pair at the parse (M1f
S1f.5.5's swap, S1f.5.1 and S1f.5.11). A premise means the same in any order
of its conjuncts: a guard waits for the relation premise that binds its
variable, and one whose variable nothing in its conjunction binds binds it —
`eq` to the other side, `neq` (and `eq` with both sides free) over the
program's names, the list H's candidates are built from (Q-M1e2.9 (a)); an
`(absent …)` after it reads that variable (Q-M1f.34). `(not (eq ?a ?b))` is
`(neq ?a ?b)` (Q-M1f.31). A conclusion of either is checked against its
definition: a true one adds nothing, a false one is `(false)`. **From M1e2
S1e2.2.24 to S1f.5.7** (W24) the loader refused a form as a guard's argument
and a guard on a variable no premise before it binds
([`defined_behaviour.md` § 4.9](../../defined_behaviour.md#49-a-rule-shape-that-reads-nothing--m1e2-s1e2224)),
and **until S1e2.2.24** each read nothing: a raw node was equal to no value
and an unbound variable compared as a null, so `(eq ?x (f a))` never held,
even where `?x` was the fact `(f a)`, and `(and (neq ?x a) (p ?x))` fired for
`a`. A range was refused there too, from S1e2.2.20, until M1f S1f.5.3 retired
the token: `1..3` is a parse error since.

Q33 caps it there for a stated reason: *a predicate's truth is **computed**
from the bindings, where a relation's truth is **data***. Ein has no numbers
and none is planned (the user, 2026-11-06, Q-M1e2.24), so no numeric,
cardinality or aggregation primitive is either.

`(transitive ?r)` and `(symmetric ?r)` are **not** predicates and never
were — they are ordinary relation patterns matching a stored property fact
that the puzzle or the stdlib asserted. Written as a premise, what one does
is a join, not a computation. Written in a `:where`, as this page showed until
M1f S1f.5.11, it was dropped with the pair (Q32), so it gated nothing, until
S1f.5.5's swap refused it at the parse.

> **The aggregate registry was designed and never built.** An earlier draft of
> this page listed `unique-remaining`, `no-remaining-option` and
> `forbidden-by-exclusion` as *"the M1 starter set"*, and
> [`../01-ein-graph/02_rules.md` §2.3](../01-ein-graph/02_rules.md) sketches
> four more (`elimination-by-exhaustion`, `arc-consistency-propagate`,
> `global-cardinality`, `forced-by-unique-position`) plus `in-domain`. **None
> of the eight exists** — not in the crates, not in `stdlib/`, not in any
> `.ein` file in the tree (checked M1e S1e.2.2). This is a different failure
> from the rest of the tree's stale pages: those describe machinery that was
> **removed**, this described machinery that was **planned**, and a reader was
> being credited with aggregate reasoning the engine does not have.
>
> What ships instead is *elimination as ordinary rules*:
> [`std.bijection`](../../../../stdlib/bijection.ein)'s `domain-elimination` /
> `range-elimination` and [`std.slots`](../../../../stdlib/slots.ein)'
> `slot-elimination` / `slot-fill` express "only one candidate remains" with a
> `forall` macro over nested `absent`s, decided at the closure boundary — no
> aggregate primitive required
> ([`07_stdlib_api.md`](07_stdlib_api.md),
> [`../../inference/README.md` § d=0 negative-completion](../../inference/README.md#d0-negative-completion-s15a19)).

## Triangle rule — two forms

Both forms produce the same conclusions from the same working memory;
they differ in which **rule type** they instantiate (per
[`../01-ein-graph/02_rules.md`](../01-ein-graph/02_rules.md)).

### As a T3 structural rule — which Ein does not load

```lisp
(rule triangle-composition ()
  :match (and (?r ?a ?b)
              (?r ?b ?c)
              (transitive ?r))
  :assert (?r ?a ?c)
  :why "From {0} and {1}, since {?r} is transitive, {?a} {?r} {?c}.")
```

Non-generic (`()` params), meant to fire universally, the premise
`(transitive ?r)` restricting it to the relations an application fact tags.
It does not load: a relation is named by a name or by a parameter its
activator binds (Q29), so the loader refuses `?r` in each of the three
patterns, since M1e2 S1e2.2.24 (W24, `3ab9a231`):

```text
kb load error: rule 'triangle-composition': its :match writes (?r ?a ?b) with ?r as its relation, which no parameter binds — a relation is named by a name or by a parameter its activator binds; …
```

With `?r` a parameter the rule is the T2 form below. Until M1f S1f.5.11 this
section wrote the tag `:where (transitive ?r)`, a keyword pair the compiler
dropped (Q32), so the guard read nothing even before the loader refused the
relation variable; since S1f.5.5's swap that spelling is a parse error at
`:where`.

### As a T2 relation-polymorphic property-rule (gated by parameter)

```lisp
(rule transitive (?rel)
  :match (and (?rel ?a ?b)
              (?rel ?b ?c)
              (neq ?a ?c))
  :assert (?rel ?a ?c)
  :why "{?rel} is transitive.")
```

Applied via `(transitive co-located)` in ontology; fires only on
relations explicitly tagged. The T2 form is the one the stdlib ships,
[`std.algebra`](../../../../stdlib/algebra.ein)'s `transitive`, word for word:
with `(co-located a b)` and `(co-located b c)` it derives `(co-located a c
:rule transitive)` (`ein saturate FILE --dump`, 2026-11-09). Its guard is the
premise `(neq ?a ?c)`; until M1f S1f.5.11 this page wrote it `:where (neq ?a
?c)`, which the compiler dropped (Q32), so the rule fired unguarded, and which
is a parse error since S1f.5.5's swap.

The two forms were meant to differ in *trace shape*: the T3 form would name
`triangle-composition` in the firing, the T2 form names `transitive`. Only the
T2 form loads.

## See also

- [`01_grammar.md`](01_grammar.md) — the rule form's surrounding
  grammar.
- [`../01-ein-graph/02_rules.md`](../01-ein-graph/02_rules.md) —
  what a rule *is* in graph-rewriting terms.
- [`../../inference/`](../../inference/) — the pattern matcher and the
  saturation loop that run these patterns.
