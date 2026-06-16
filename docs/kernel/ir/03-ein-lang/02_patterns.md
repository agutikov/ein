# Pattern sub-language

The mini-language used inside rule `:match` / `:assert` clauses. The
**surface** form of the three rule types from
[`../01-ein-graph/02_rules.md`](../01-ein-graph/02_rules.md).

This was [`docs/ir.md` §3](../../README.md) before the kernel-
documentation split.

---

## Closure

The pattern language is **positive conjunctive** + `:where` filters +
a registry of **named structural predicates** (the rewrite-DSL +
named-predicate fallback from
[Q4](../../../../plans/m1_core_graph_reasoning/open_questions.md#q4)).
**Kernel meta-primitives** (`and`, `or`, `not`, `neq`, `instance`)
are shape-pinned in the grammar — wrong arity is a parse error, not
a validator error. Relation patterns (`(?r ?a ?b)`,
`(co-located ?a ?b)`) stay generic at the grammar level; the
validator (S1.1.2) enforces well-formedness against the rules below.

| construct                | example                                | reserved? | meaning                                         |
|--------------------------|----------------------------------------|-----------|-------------------------------------------------|
| variables                | `?a`, `?house`                          | —         | bound by the match; reused across sub-clauses   |
| ground atoms             | `Red`, `House-1`                        | —         | match literally                                 |
| relation pattern         | `(?r ?a ?b [?c …])`                     | —         | VAR head binds the relation name; args bind positions |
| named relation pattern   | `(co-located ?a ?b)`                    | —         | match a specific relation's instances                  |
| head wildcard            | `(_ ?a ?b)`                             | —         | match any binary list head                      |
| **conjunction**          | `(and <p1> <p2> …)`                     | ✓ AND    | conjunctive match (kernel primitive)             |
| **disjunction**          | `(or  <p1> <p2> …)`                     | ✓ OR     | disjunctive match (grammar-reserved; engine semantics in P1.3) |
| **negation**             | `(not <p>)`                             | ✓ NOT    | wrapped premise must not hold                    |
| **equality**             | `(= ?a ?b)`                             | ✓ EQ     | bind / match equality                            |
| **instance check**       | `(instance ?a ?T)`                      | ✓ INSTANCE | instance-of pattern                            |
| `:where` filter          | `:where (transitive ?r) (neq ?a ?b)`    | NEQ inside | type / inequality / structural-predicate filters |
| named structural pred.   | `(unique-remaining ?slot ?type)`        | —         | aggregate-style premise; see §Predicate registry  |

The ✓-marked heads have dedicated grammar rules with fixed arities;
typos like `(instnce ?a ?T)` or `(neq ?a)` are caught at parse time.

## What is NOT in the pattern language

- **Negation-as-failure outside `:where`** — at premise level, use a
  named structural predicate (`no-remaining-option`,
  `forbidden-by-exclusion`) so the trace planner can name the firing.
  `(not <p>)` is permitted as an *assertion* (rule conclusion) but
  acts as a positive negative fact, not as failure-to-prove.
- **Universal quantifiers / aggregates as expressions** — lift to
  named structural predicates.

The line is governed by trace fidelity: anything the matcher can see,
the trace planner can name. Opaque Python fallbacks would render as
black-box firings, failing the
[M1 acceptance §3](../../../../plans/m1_core_graph_reasoning/README.md)
explanation-completeness criterion.

## Predicate registry (initial)

Names and Python implementations are registered in P1.3 S1.3.1. The
M1 starter set (T3 structural / aggregate predicates from
[`../01-ein-graph/02_rules.md` §2.3](../01-ein-graph/02_rules.md)):

| predicate                       | meaning                                       |
|---------------------------------|-----------------------------------------------|
| `(transitive ?r)`               | the named relation is transitive              |
| `(symmetric ?r)`                | the named relation is symmetric               |
| `(neq ?a ?b)`                   | the bindings refer to distinct entities       |
| `(unique-remaining ?slot ?type)` | only one slot of `?type` is unassigned        |
| `(no-remaining-option ?x)`      | every candidate value for `?x` is excluded    |
| `(forbidden-by-exclusion ?a ?b ?r)` | `?r(?a, ?b)` is excluded by an `allDifferent`-style constraint |

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
  :why "From {0} and {1}, since {?r} is transitive, {?a} {?r} {?c}."
  :priority 10)
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
  :why "{?rel} is transitive."
  :priority 5)
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
- [`../../inference/`](../../inference/) — the P1.3 pattern matcher
  + saturation loop.
