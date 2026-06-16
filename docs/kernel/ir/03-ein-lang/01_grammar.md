# Grammar — lexical rules and top-level forms

> **Source of truth for syntax:**
> [`src/ein/ir/grammar.lark`](../../../../ein.py/src/ein/ir/grammar.lark).
> This document explains intent and structure; the grammar file is
> canonical for what parses.

This was [`docs/ir.md` §1 + §2](../../../ir.md) before the kernel-
documentation split.

---

## §1 Lexical rules

| terminal   | regex                          | examples                                | role                                              |
|------------|--------------------------------|-----------------------------------------|---------------------------------------------------|
| `SYMBOL`   | `[A-Za-z][A-Za-z0-9_*-]*`      | `has-color`, `next-to`, `House-1`, `is-a*` | atoms; list heads in patterns; rule / type / step names. `*` in tail is a character with no Kleene/multiplicative meaning (S1.5.8c.2 — supports the transitive-closure naming convention `R*`). |
| `VAR`      | `\?[A-Za-z][A-Za-z0-9_*-]*`    | `?a`, `?house`, `?T`, `?R*`              | pattern variables — bound by `:match`, reused in `:assert`. Uppercase allowed for type-shaped vars; `*` in tail allowed (same convention as SYMBOL). |
| `KEYWORD`  | `:[a-z][A-Za-z0-9_-]*`         | `:rule`, `:where`, `:cardinality`        | argument markers; **always** followed by a value  |
| `WILDCARD` | `_`                            | `_`                                      | head / arg wildcard in patterns                   |
| `INT`      | `-?[0-9]+`                     | `0`, `42`, `-7`                          | integer atoms (e.g. `:priority 10`)               |
| `RANGE`    | `[0-9]+\.\.([0-9]+|\*)`        | `0..1`, `1..1`, `1..*`                  | UML-style cardinality                             |
| `STRING`   | `"…"` with `\\` escape          | `"condition (10)"`, `"{?r} is transitive"` | source-sentence provenance + `:why` templates only |

**Comments** — SMT-LIB-compatible: `; line` to end of line, `#| block |#`
non-nesting.

**Naming convention** — hyphenated lowercase for relations and rule
names (`has-color`, `triangle-composition`); PascalCase or `Foo_N`
for types and instances (`Person`, `House-1`, `Norwegian`). Convention
only; the grammar accepts either.

---

## §2 Top-level forms

A program is a **flat sequence of forms** (P1.7c). Each top-level form is
classified by its **head** against the closed declarator set; anything
whose head is not a declarator is a **fact**. Source of truth for the set:
[`06_reserved_names.md`](06_reserved_names.md) (the parser + loader both
key on it).

| head | role | shape |
|------|------|-------|
| `relation` | declare a relation-type + its arg-type signature | `(relation <name> <T1> <T2> [<T3> …] [:kw v]*)` |
| `rule` | a saturation rewrite rule | `(rule <name> (<param-vars>*) :match … :assert … …)` |
| `hrule` | a hypothesis-generation rule (drives the blind enumerator) | same shape as `rule` |
| `query` | what to ask the engine | `(query :goal … [:goal-text …] [:hrules …])` |
| `config` | solver knobs | `(config [:flag v]*)` |
| `trace` | **engine output** — derivation log (engine-emitted, not authored) | `(trace <step\|branch-open\|…>*)` |
| *anything else* | **a fact** | `(= …)` · `(not …)` · generic `(<NAME> <args>* [:kw v]*)` |

The block wrappers `(ontology …)` / `(facts …)` / `(reasoning …)` /
`(rules …)` were **removed in P1.7c** (S1.7c.4); a former-wrapper head now
simply reads as a fact (e.g. `(facts X)` is a fact whose relation is
`facts`).

### Knowledge layer — per fact, not positional

The three knowledge layers from
[`../01-ein-graph/01_kb.md` §3](../01-ein-graph/01_kb.md) — ONTOLOGY
(implicit assumptions), FACT (explicit problem statements), REASONING
(engine-derived) — used to be carried by the enclosing block. Flat forms
carry the layer **per fact** (P1.7c S1.7c.1): an explicit
`:layer ontology|fact|reasoning` keyword is authoritative; otherwise the
layer is **derived from provenance** — the same signal that picks the
`Provenance` kind:

| the fact carries… | layer | meaning |
|---|---|---|
| `:rule` / `:using` | REASONING | engine working-memory (a derived fact) |
| `:source "(N)"` | FACT | an explicit, numbered problem statement |
| neither | ONTOLOGY | an implicit background assumption (schema, type/instance enumeration, property tag) |

Only REASONING is inference-distinguished (the contradiction detector's
cross-layer rule); ONTOLOGY-vs-FACT is render / provenance metadata and
does not affect the search. Authored files rarely need an explicit
`:layer` — a sourced condition derives FACT, an unannotated property tag or
`relation` decl derives ONTOLOGY. Write `:layer` only where the derivation
would disagree with intent (e.g. a structural ONTOLOGY fact that *also*
wants a `:source`, or an unsourced explicit FACT). Fact identity is
`(relation_name, args)`; the layer only records origin. See
[`../02-data-model/01_entities.md` §1.5](../02-data-model/01_entities.md)
and [`docs/ideas/04-nlp-to-graph-to-solver-pipeline.md` §Ontology
deduction by common sense](../../../ideas/04-nlp-to-graph-to-solver-pipeline.md)
for how the NL frontend recovers the ONTOLOGY-vs-FACT split from context.

### Relation declarator

```lisp
(relation <name> <T1> <T2> [<T3> ...]    ; relation signature, arity ≥ 2
  [:why <STRING>] [:cardinality <RANGE>]  ; optional metadata kw-pairs
  [...])
```

`relation` declares a relation-type node + its arg-type signature
(structural / spatial relations are plain `(relation …)`; a pattern-based
derivation is an ordinary `rule`; `a-priori` removed in S1.7.6). The loader
auto-stores each declaration as an ONTOLOGY fact `(relation R T1 T2 …)` so
rules can introspect signatures via a `(relation ?R ?A ?B)` pattern in
`:match`. Because of that, `relation` is **not** SYMBOL-excluded, so the
malformed wrapped-arg form `(relation R (T1 T2))` parses but is rejected at
LOAD time (not parse time).

**`:why` render template (optional).** A `:why "<tmpl>"` string turns a fact
of this relation into natural-language text — used by `ein solve`'s result
table (the *rendered query facts* column). It reuses the rule `:why` engine
but references the fact's argument **slots positionally**: `{?1}` is arg 0,
`{?2}` is arg 1, … (a leading digit is the relation-template form;
rule/goal `:why` uses letter-led var names). A relation with **no** `:why`
renders as its raw IR s-expression `(R a b)` — there is no built-in
relation→verb vocabulary, so untemplated relations stay in IR. Example:

```lisp
(relation drink-loc Drink House :why "{?1} is drunk in {?2}")
;; (drink-loc Water House-1)  →  "Water is drunk in House-1"
(relation right-of House House)            ; no :why → renders as (right-of …)
```

A schema + implicit-assumption example (all flat forms; the property tags
and enumerations derive ONTOLOGY, so no `:layer` is needed):

```lisp
(type Attribute)
(type House Attribute) (type Color Attribute)
(relation co-located Attribute Attribute)
(instance Norwegian Nationality)
(instance House-1 House)
(symmetric  co-located)
(transitive co-located)
(right-of House-2 House-1 :source "condition (1)" :layer ontology)  ; "five in a row" — sourced but structural
```

### Facts — `(NAME args*)`, the flat default

A fact is any top-level form whose head is **not** a declarator. Three
shapes:

```lisp
(= <expr> <expr> [:source <STRING>])      ; equality            (reserved head `=`)
(not <expr> [:source <STRING>])           ; negative            (reserved head `not`, arity 1)
(<name> <arg>* [:source <STRING>])        ; relation instance / enumeration / property tag
```

| kind | example | semantics |
|---|---|---|
| **relation instance** | `(co-located Englishman Red :source "(2)")` | a relation holds between specific entities |
| **equality**          | `(= (color House-1) Red :source "(?)")` | equational form (reserved `=`) |
| **negative**          | `(not (drinks Spaniard Coffee) :source "(?)")` | the wrapped fact does *not* hold |

`=` and `not` are **shape-pinned reserved heads** (wrong arity is a parse
error). `and` / `or` / `neq` are kernel meta-primitives that belong inside
`:match` patterns / `:where` clauses, never a top-level fact head — and the
declarators `rule` / `hrule` / `query` / `config` / `trace` cannot be fact
heads either (they're SYMBOL-excluded). Everything else — `instance`,
`type`, `symmetric`, `co-located`, `lives-in`, … — is an ordinary generic
`(SYMBOL value*)` fact, open-world: introduce a relation by declaring it
with `(relation …)` and asserting it.

The layer of each fact follows the
[per-fact rule](#knowledge-layer--per-fact-not-positional): a `:source`
makes it a FACT-layer problem statement; `:rule` / `:using` make it a
REASONING-layer derived fact; no annotation makes it an ONTOLOGY-layer
background assumption (an explicit `:layer` overrides). Rule-application
facts (`(symmetric co-located)`, `(implies right-of next-to)`) carry no
annotation → ONTOLOGY: the puzzle text never says "co-located is
symmetric"; that's universal context, the *meta* of the relation, while a
`rule` is the meta of the *engine*.

`all-different` is **not** a kernel primitive; pairwise distinctness within
a category is derived by `type-exclusivity` from the `(instance X T)`
facts. Genuinely puzzle-specific structural shapes (parity, budget, …) just
take their own head: `(budget-total X Y)`.

A flat explicit-conditions example (each numbered condition is FACT-layer
via its `:source`; the property tag derives ONTOLOGY):

```lisp
(lives-in Norwegian House-1 :source "condition (10)")
(co-located Englishman Red  :source "condition (2)")
(symmetric  co-located)                              ; property tag — implicit, ONTOLOGY
```

### Derived facts (REASONING layer — engine working memory)

The engine *derives* facts at runtime and dumps them as flat forms
annotated with `:rule` (which rule fired) + `:using` (which premises it
consumed) instead of `:source` — so they re-classify to the REASONING
layer on reload:

```lisp
(<name> <arg>* :rule <RuleName> :using (<premise-id>+))
(not <expr>    :rule <RuleName> :using (<premise-id>+))
```

A hand-authored puzzle has none; they appear in engine dumps, which
round-trip through `parse` / `dump`. (A REASONING fact with no rule
provenance — e.g. an authored hypothesis fixture — carries an explicit
`:layer reasoning` instead.) Example derived facts:

```lisp
(co-located Blue House-2 :rule square-fwd :using (c10 c15))
(not (co-located Norwegian House-2) :rule type-exclusivity :using (c10))
```

> **`:using` IR round-trip caveat (M1):** the current grammar accepts
> `:using (atom-id-1 atom-id-2 ...)` but parses it to a shape that
> doesn't directly match the data model's `(rel, args)` premise ids.
> The compact-form `:using ((rel a b) (rel c d))` is what the data
> model uses internally but is *rejected* by the current grammar (a
> kw-pair value must be a headed list). Both forms wait on a P1.1
> grammar tweak or a `:id <atom>` annotation system — see
> [S1.2.3 T1.2.3.4](../../../../plans/m1_core_graph_reasoning/p1.2_typed_hypergraph/s1.2.3_provenance.md).
> Until then, rule-kind provenance is populated by the engine
> programmatically via `Provenance.from_rule(...)`, which works
> end-to-end — only the IR text round-trip is deferred.

### Rules

```lisp
(rule <name> (<param-vars>*)      ; parameter list — mandatory, `()` for non-generic
  :match <pattern>                ; LHS — structural pattern (see 02_patterns.md)
  :assert <conclusion>            ; RHS — what to derive
  :why <STRING>                   ; reason template for trace
  [:priority <INT>])              ; rule ordering — lower = earlier
```

A rule is a top-level `(rule …)` form (a hypothesis-generation rule is the
same shape headed `hrule`). There is no `(rules …)` block (P1.7c).

Each rule has one `:match` and one `:assert`. The pattern sub-language
is in [`02_patterns.md`](02_patterns.md). `:priority` resolves
[Q15](../../../../plans/m1_core_graph_reasoning/open_questions.md#q15)
(rule ordering): static per-rule, cheap-propagation rules at lower
numbers.

The **parameter list** is mandatory. Two cases (mapping onto the
three rule types from
[`../01-ein-graph/02_rules.md`](../01-ein-graph/02_rules.md)):

| parameters | rule type | how it fires |
|---|---|---|
| **non-empty** `(?p1 ?p2 …)` containing **relation variables** | **T2 — relation-polymorphic** | fires only when bound to a rule-application fact, e.g. `(symmetric co-located)` binds `?rel = co-located` for the `symmetric` rule |
| **empty `()`** | **T1 first-order OR T3 structural** | fires universally on every match; free vars in `:match` are bound by the matcher |

A relation-polymorphic rule with no matching application facts never
fires — the parameters are the gate. A non-generic rule needs no
application fact.

Concrete: the rule

```lisp
(rule symmetric (?rel)
  :match  (?rel ?a ?b)
  :assert (?rel ?b ?a)
  :why    "{?rel} is symmetric." :priority 1)
```

is applied via the fact `(symmetric co-located)`, which substitutes
`?rel = co-located` and then matches `(co-located ?a ?b)` against
working memory. One generic rule per property replaces N per-relation
property-rules.

#### Premise forms in `:match`

In addition to ordinary fact patterns, `:match` accepts three
NAF / quantifier-style premises:

| premise            | semantics                                           | added in    |
|--------------------|-----------------------------------------------------|-------------|
| `(not P)`          | matches a STORED `(not P)` fact in the KB           | S1.5.8c.1   |
| `(absent P)`       | negation-as-failure — fires iff no fact matches P   | S1.5.8c.1   |
| `(open P)`         | parser sugar for `(and (absent P) (absent (not P)))` — the third-state match: P is neither asserted nor negated | S1.5.8c.3b |
| `(forall ?b (G) (B))` | parser sugar for `(absent (and G (absent B)))` — for every binding of `?b` satisfying guard G, body B must hold | S1.5.8c.3a |

The three-state model: at any moment, a potential fact P is
**asserted** (matched by `(P)`), **negated** (matched by `(not P)`),
or **open** (matched by `(open P)`). The earlier overloaded
`(not P)` meaning (default NAF) was dropped in S1.5.8c — NAF must
now be written explicitly as `(absent P)`. `forall` and `open` are
parser sugars expressed in terms of `absent`; the matcher itself
sees only `absent` + nested patterns.

#### NAF evaluation timing (known limitation)

`(absent P)` is evaluated at **enqueue time**, not at fire time.
A rule's NAF premise sees the KB state at the moment the matcher
first finds a binding. Once that binding is enqueued (with
NAF=passes), the firing commits regardless of later KB state.

This means **NAF on derived facts can race**: if a rule's `(absent
(R ?x))` premise depends on a relation R that is itself populated
by another rule's firings, the NAF might pass early (before R is
fully derived), commit a firing that would be incorrect under the
fully-derived KB. The race is benign when R is fully present at
load time (e.g., enumerated facts), problematic when R is
derived by `(symmetric)` / `(includes)` / `(transitive)` rules.

Parked for engine-side resolution in
[P1.5a](../../../plans/m1_core_graph_reasoning/p1.5a_zebra_solution/README.md).
Until then, rules whose NAF depends on derived facts should
pre-declare those facts explicitly (as ONTOLOGY- or FACT-layer forms)
to avoid the race.

### Query

```lisp
(query
  :goal <expr>                             ; what to find / verify
  [:goal-text <STRING>]                    ; NL headline template (optional)
  [:hrules (<activator> …)])               ; hypothesis-generator activators
```

The engine answers idea 03's three task classes —
[`docs/ideas/03-three-task-classes.md`](../../../ideas/03-three-task-classes.md)
— *solve* (a unique model), *gaps* (under-determined: many models), and
*contradictions* (inconsistency + provenance). But these are **three answers
to one problem, read off the result `k`** (the count of distinct complete
models: `k = 1` / `k > 1` / `k = 0`) — **not** chosen up front. There is one
`ein solve`, no `--mode` flag, and the engine's goal test is hardwired to
SOLVE. A `:mode` keyword on the query is **obsolete** (vestigial; the engine
ignores it) — omit it.

**`:goal-text` headline template (optional).** A `:goal-text "<tmpl>"` string
renders the one-line natural-language answer for `ein solve`'s result table,
referencing the goal's **own variables** by name — the same `{?var}` engine as
rule `:why`, bound from the solution. The vocabulary lives entirely in the
puzzle; nothing is hardcoded in the renderer. The value is a **single** string
literal (ein-lang does not concatenate adjacent strings). Example (the Zebra
goal binds `?who_water` / `?h_zebra` / …):

```lisp
(query
  :goal (and (drink-loc Water ?h_water) (nation-loc ?who_water ?h_water)
             (pet-loc Zebra ?h_zebra)   (nation-loc ?who_zebra ?h_zebra))
  :goal-text "The {?who_water} drinks water in {?h_water}; the {?who_zebra} owns zebra in {?h_zebra}")
;;  →  "The Norwegian drinks water in House-1; the Japanese owns zebra in House-5"
```

The division of labour: a `(relation … :why)` template renders an individual
*fact* to text (the rendered-facts column); the `(query … :goal-text)`
template renders the *headline result* from the goal bindings.

### Trace

```lisp
(trace
  (step <id> :rule <name>                  ; engine derivation step
             :using (<premise-ids>)
             :derives <expr>
             [:source <ref> | :assumes <expr>])
  (branch-open <id> :on <expr>             ; open a hypothesis-driven split
                    :choices (<sub-ids>))
  (branch-close <id> :choose <sub-id>)     ; commit to a branch
  (contradiction <id> :using (<step-ids>)  ; record a contradiction
                      :assumption <step-id>)
  (symmetry-class <id> :over (<entities>)  ; mark engine-arbitrary choices
                       :note <STRING>))
```

Per [Q21](../../../../plans/m1_core_graph_reasoning/open_questions.md#q21),
`(trace …)` is the **same IR** as input — same parser, same AST,
same dumper. The engine can reason about its own traces; rules can
match `(step …)` forms ([TMS/ATMS analogue](../../../lib/09-cognitive-architectures-neurosymbolic.md)).
Per [Q18](../../../../plans/m1_core_graph_reasoning/open_questions.md#q18)
each derived edge's provenance tuple `(rule, premise_edges, source)`
is literally a `(step …)` form — provenance and trace are the same
data structure under different views.

---

## See also

- [`02_patterns.md`](02_patterns.md) — the pattern sub-language used
  inside `:match` / `:assert`.
- [`03_examples.md`](03_examples.md) — worked Zebra fragments.
- [`04_dot_rendering.md`](04_dot_rendering.md) — DOT rendering of
  each form.
- [`../01-ein-graph/`](../01-ein-graph/) — what these forms *mean*
  in graph terms.
- [`../02-data-model/`](../02-data-model/) — Python entities the
  loader produces from these forms.
