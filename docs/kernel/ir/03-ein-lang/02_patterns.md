# Pattern sub-language

The mini-language used inside rule `:match` / `:assert` clauses. The
**surface** form of the three rule types from
[`../01-ein-graph/02_rules.md`](../01-ein-graph/02_rules.md).

This was [`docs/ir.md` §3](../../README.md) before the kernel-
documentation split.

---

## Closure

The pattern language is **positive conjunctive** + `:where` filters + two
computed predicates. (The rewrite-DSL + named-predicate fallback of
[Q4](../../../../plans/open_questions.md#q4--rule-presentation-language)
chose the first half; the *named structural predicate* registry the second
half proposed was never built — § Predicate registry below.)

**Where a malformed primitive is caught** is one of two places, and the split
is not stylistic:

| primitive | arity | refused by |
|---|---|---|
| `not` | 1 | the **grammar** — `NotForm ::= '(' 'not' Value KwPair* ')'` |
| `neq` | 2 | the **grammar** — `NeqForm ::= '(' 'neq' Value Value ')'` |
| `and` / `or` | 1+ | the **grammar** — `Value+`, so `(and)` is a parse error and `(and X)` is legal, if degenerate |
| `eq` | 2 | the **compiler**, with a positioned `CompileError` — M1e [S1e.2.1](../../../history/m1e_review_processing/README.md#s1e21--correctness-the-panic-the-guard-the-traversal), which is where the whole class was checked ([Q-M1e.18](../../../history/m1e_review_processing/open_questions.md#q-m1e18--three-kernel-primitives-are-not-shape-pinned-and-drop-their-extra-arguments)) |
| `absent` | 1 | same |
| `false` | 0+ | nothing — extra args are ignored by design, `(false)` being a verdict and not a query |

`eq` and `absent` are plain `SYMBOL`s so a rule can pattern-match on them;
that is why the grammar has no production to pin, and why the check moved to
the one place that reads the form. Relation patterns (`(?r ?a ?b)`,
`(co-located ?a ?b)`) stay generic at the grammar level; the loader and
compiler enforce well-formedness against the rules below.

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
| **equality fact**        | `(= ?a ?b)`                             | ✓ EQ     | matches a **stored** `(= …)` fact — see below     |
| **computed equality**    | `(eq ?a ?b)` / `(neq ?a ?b)`            | —         | the two registered predicates: compare the bindings |
| **membership check**     | `(is-a ?a ?T)`                          | ✓ RELATION | ordinary relation pattern                      |
| `:where` filter          | `:where (transitive ?r) (neq ?a ?b)`    | NEQ inside | type / inequality / structural-predicate filters |
| relation-property tag    | `:where (transitive ?r)`                | —         | an ordinary premise matching a stored property fact |

**A premise is a form** — a pattern, a guard, an `(absent …)`, or an
`(and …)` or `(or …)` of premises. An atom (a run of digits among them, an
integer until M1f S1f.5.4), a variable, `_` or a string (the last two at the
parse since M1f S1f.5.5's swap, S1f.5.1 and S1f.5.2) written where a premise goes is **refused at load** since M1e2
S1e2.2.24 (W24, [`defined_behaviour.md` § 4.9](../../defined_behaviour.md#49-a-rule-shape-that-reads-nothing--m1e2-s1e2224)):
until then the compiler dropped it in silence, so `:match _` fired with no
premise at all and `(and (hit ?y) foo)` on `(hit ?y)` alone. A keyword pair
as a premise of an `(and …)` is Q32's `:where`, dropped, as is one at a
pattern's top level; one **inside a pattern inside a pattern**, `(box (a b :k
v))`, matched nothing and is refused at load since the same stage.

The ✓-marked heads have dedicated grammar rules with fixed arities, so
`(neq ?a)` is a parse error. **A misspelled relation is not**: `instance` has
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

The ✓ on the equality row marks a **grammar** rule, not an engine step, and
the distinction catches people out because the two spellings look
interchangeable and are not:

- **`(= ?a ?b)`** is an ordinary relation pattern whose head happens to be the
  reserved `=`. It matches the `(= …)` *facts the KB holds* — including ones a
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
| `(eq ?a ?b)`    | 2     | the bindings resolve to the same value | matcher `Guard` opcode |
| `(neq ?a ?b)`   | 2     | the bindings resolve to distinct values | matcher `Guard` opcode |

**A guard's argument is a value**: a variable a premise before the guard
binds (or a rule's parameter), or a name; a run of digits is a name, an
integer until M1f S1f.5.4, and a string was a name too, the one it spells, until
M1f S1f.5.5's swap refused a string there (S1f.5.2). A guard is
judged where it is written. `_` (at the parse since M1f S1f.5.5's swap,
S1f.5.1), a form, a keyword pair, or a variable no
premise before it binds is **refused at load** (since M1e2 S1e2.2.24, W24;
[`defined_behaviour.md` § 4.9](../../defined_behaviour.md#49-a-rule-shape-that-reads-nothing--m1e2-s1e2224)).
Until then each read nothing: a raw node was equal to no value and an
unbound variable compared as a null, so `(eq ?x (f a))` never held, even
where `?x` was the fact `(f a)`, and `(and (neq ?x a) (p ?x))` fired for
`a`. A range was refused there too, from S1e2.2.20, until M1f S1f.5.3
retired the token: `1..3` is a parse error since.

Q33 caps it there for a stated reason: *a predicate's truth is **computed**
from the bindings, where a relation's truth is **data***. Ein has no numbers
and none is planned (the user, 2026-11-06, Q-M1e2.24), so no numeric,
cardinality or aggregation primitive is either.

`(transitive ?r)` and `(symmetric ?r)` are **not** predicates and never
were — they are ordinary relation patterns matching a stored property fact
that the puzzle or the stdlib asserted. Writing one in `:where` works, and
what it does is a join, not a computation.

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

### As a T3 structural rule (explicit `:where` guard)

```lisp
(rule triangle-composition ()
  :match (and (?r ?a ?b)
              (?r ?b ?c)
              :where (transitive ?r))
  :assert (?r ?a ?c)
  :why "From {0} and {1}, since {?r} is transitive, {?a} {?r} {?c}.")
```

Non-generic (`()` params), fires universally; the `:where` guard
restricts to transitive relations. The structural predicate
`(transitive ?r)` introspects whether `?r` has a corresponding
`(transitive ?r)` application fact in the ontology.

### As a T2 relation-polymorphic property-rule (gated by parameter)

```lisp
(rule transitive (?rel)
  :match (and (?rel ?a ?b)
              (?rel ?b ?c)
              :where (neq ?a ?c))
  :assert (?rel ?a ?c)
  :why "{?rel} is transitive.")
```

Applied via `(transitive co-located)` in ontology; fires only on
relations explicitly tagged. The T2 form is the one used in the M1
zebra.ein rule library.

The two forms are not equivalent in *trace shape*: the T3 form names
`triangle-composition` in the firing, the T2 form names `transitive`.
The choice is a documentation question — which name reads better in
a human trace? — and is per-puzzle.

## See also

- [`01_grammar.md`](01_grammar.md) — the rule form's surrounding
  grammar.
- [`../01-ein-graph/02_rules.md`](../01-ein-graph/02_rules.md) —
  what a rule *is* in graph-rewriting terms.
- [`../../inference/`](../../inference/) — the pattern matcher and the
  saturation loop that run these patterns.
