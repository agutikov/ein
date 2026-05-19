# Grammar — lexical rules and top-level forms

> **Source of truth for syntax:**
> [`src/ein_bot/ir/grammar.lark`](../../../../src/ein_bot/ir/grammar.lark).
> This document explains intent and structure; the grammar file is
> canonical for what parses.

This was [`docs/ir.md` §1 + §2](../../../ir.md) before the kernel-
documentation split.

---

## §1 Lexical rules

| terminal   | regex                          | examples                                | role                                              |
|------------|--------------------------------|-----------------------------------------|---------------------------------------------------|
| `SYMBOL`   | `[A-Za-z][A-Za-z0-9_-]*`       | `has-color`, `next-to`, `House_1`        | atoms; list heads in patterns; rule / type / step names |
| `VAR`      | `\?[A-Za-z][A-Za-z0-9_-]*`     | `?a`, `?house`, `?T`, `?Type`            | pattern variables — bound by `:match`, reused in `:assert`. Uppercase allowed for type-shaped vars (`?T`). |
| `KEYWORD`  | `:[a-z][A-Za-z0-9_-]*`         | `:rule`, `:where`, `:cardinality`        | argument markers; **always** followed by a value  |
| `WILDCARD` | `_`                            | `_`                                      | head / arg wildcard in patterns                   |
| `INT`      | `-?[0-9]+`                     | `0`, `42`, `-7`                          | integer atoms (e.g. `:priority 10`)               |
| `RANGE`    | `[0-9]+\.\.([0-9]+|\*)`        | `0..1`, `1..1`, `1..*`                  | UML-style cardinality                             |
| `STRING`   | `"…"` with `\\` escape          | `"condition (10)"`, `"{?r} is transitive"` | source-sentence provenance + `:why` templates only |

**Comments** — SMT-LIB-compatible: `; line` to end of line, `#| block |#`
non-nesting.

**Naming convention** — hyphenated lowercase for relations and rule
names (`has-color`, `triangle-composition`); PascalCase or `Foo_N`
for types and instances (`Person`, `House_1`, `Norwegian`). Convention
only; the grammar accepts either.

---

## §2 Top-level forms

The kernel has **6 reserved heads**. Anything else at the top level
fails to parse. Three of them — `ontology`, `facts`, `reasoning` —
are the *three knowledge layers* from
[`../01-ein-graph/01_kb.md` §3](../01-ein-graph/01_kb.md); the
block name carries the **provenance** of the contained items.

| head        | layer / role                                                 | reserved sub-form heads                              |
|-------------|--------------------------------------------------------------|------------------------------------------------------|
| `ontology`  | **implicit** assumptions: schema + reader-supplied context   | `type` · `relation` · `a-priori` · any fact form     |
| `facts`     | **explicit** problem statements (numbered conditions)        | `=` · `instance` · `not` + generic `(NAME args*)`    |
| `reasoning` | **derived** facts — engine working memory after a solve      | same as `facts`; provenance is `:rule` / `:using`    |
| `rules`     | inference-rule definitions (meta over ontology + facts)      | `rule`                                               |
| `query`     | what to ask the engine                                       | keyword-args only (`:mode`, `:goal`)                 |
| `trace`     | engine output — derivation log + branches                    | `step` · `branch-open` · `branch-close` · `contradiction` · `symmetry-class` |

### Ontology — schema + implicit assumptions

```lisp
(ontology
  ;; Schema —
  (type <Name> [<Parent>])                         ; declare a type, optional parent
  (relation <name> (<T1> <T2> [<T3> ...])          ; relation signature, arity ≥ 2
    [:cardinality <RANGE>] [...])                   ; optional metadata kw-pairs
  (a-priori <name> (<T1> <T2> [...])               ; structural / spatial relation
    :pattern <pattern>)

  ;; Implicit assumptions —
  (instance <Ent> <Type>)                          ; instance enumeration
  (<rule-name> <relation> ...)                     ; rule-application meta-facts
  (<relation> <args> ... [:source "..."])           ; pairwise structural facts derived
                                                    ;   from background context
  )
```

The ontology accepts **two populations**:

1. **Schema** — `type`, `relation`, `a-priori`. Describes the universe
   of discourse.
2. **Implicit-but-true assertions** — anything the puzzle treats as
   background truth without literally stating it. Three recurring
   shapes: instance enumeration, rule-application meta-facts, and
   pairwise structural facts derived from a cardinality / ordering
   statement.

The split between *ontology* and *facts* is **by provenance**: did the
puzzle's text say it, or did the reader supply it from context? An
explicit numbered condition goes in `facts` with `:source "(N)"`; a
reader-supplied assumption goes in `ontology`.

Rule-application facts (`(symmetric co-located)`, `(implies right-of
next-to)`) live in `ontology` because the puzzle text never says
"co-located is symmetric" — that's universal context. They are the
*meta* of the relation, while `rules` is the meta of the *engine*.

Example:

```lisp
(ontology
  (type Attribute)
  (type House Attribute) (type Color Attribute)
  (relation co-located (Attribute Attribute))
  (instance Norwegian Nationality)
  (instance House_1 House)
  (symmetric  co-located)
  (transitive co-located)
  (right-of House_2 House_1 :source "condition (1)"))   ; from "five in a row"
```

### Facts — `(NAME args*)`, with reserved heads

```lisp
(facts
  (= <expr> <expr> :source <STRING>)                  ; equality condition   (reserved)
  (instance <Ent> <Type> :source <STRING>)            ; instance assertion   (reserved, arity 2)
  (not <expr> :source <STRING>)                       ; negative condition   (reserved, arity 1)
  (<name> <arg>* :source <STRING>))                   ; relation condition
```

The `facts` block holds **explicit problem statements** — one entry
per numbered puzzle condition, each annotated with
`:source "condition (N)"`. Implicit assumptions (instance enumerations,
rule-application meta-facts, structural facts derived from background
context) live in `(ontology …)` instead.

Three heads are **shape-pinned reserved words** at the grammar level:
`=`, `instance`, `not`. Wrong arity is a parse error, not a validator
error. `and`, `or`, `neq` are also reserved kernel meta-primitives
but they belong inside `:match` patterns / `:where` clauses, not at
the fact level — the grammar rejects them in `(facts …)`.

Domain relations (`co-located`, `lives-in`, `next-to`) stay generic
`(SYMBOL value*)` and are open-world: anyone can introduce a new
relation by declaring it in the ontology and using it in facts.

Three kinds of facts share the same syntactic family at the explicit-
condition level:

| kind | example | semantics |
|---|---|---|
| **relation instance** | `(co-located Englishman Red :source "(2)")` | a relation holds between specific entities |
| **equality**          | `(= (color House_1) Red :source "(?)")` | equational form |
| **negative**          | `(not (drinks Spaniard Coffee) :source "(?)")` | the wrapped fact does *not* hold |

`all-different` is **not** a kernel primitive; pairwise distinctness
within a category is derived by `type-exclusivity` from the
`(instance X T)` facts. Genuinely puzzle-specific structural shapes
(parity, budget, …) just take their own head: `(budget-total X Y)`.

### Reasoning — derived facts (engine working memory)

```lisp
(reasoning
  (<name> <arg>* :rule <RuleName> :using (<premise-id>+))
  (not <expr>   :rule <RuleName> :using (<premise-id>+))
  ...)
```

The `reasoning` block holds facts the engine has *derived* at runtime.
Same syntactic shapes as `(facts …)`; the provenance kw-pair is
`:rule` (which rule fired) + `:using` (which premises it consumed),
instead of `:source` (which puzzle condition originated it).

Hand-authored puzzle files typically leave this block empty — it's
populated by the engine after `solve`. The block is parseable IR, so
engine dumps round-trip through `parse` / `dump`.

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

Example:

```lisp
(facts
  (instance Norwegian Nationality)
  (instance House_1   House)
  (symmetric  co-located)
  (transitive co-located)
  (implies    right-of next-to)
  (lives-in   Norwegian House_1 :source "condition (10)"))
```

### Rules

```lisp
(rules
  (rule <name> (<param-vars>*)    ; parameter list — mandatory, `()` for non-generic
    :match <pattern>              ; LHS — structural pattern (see 02_patterns.md)
    :assert <conclusion>          ; RHS — what to derive
    :why <STRING>                 ; reason template for trace
    [:priority <INT>]))           ; rule ordering — lower = earlier
```

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

### Query

```lisp
(query
  :mode (solve | gaps | contradictions)   ; task class — idea 03
  :goal <expr>)                            ; what to find / verify
```

The three modes correspond to the three task classes from
[`docs/ideas/03-three-task-classes.md`](../../../ideas/03-three-task-classes.md):

- `solve` — derive a unique model.
- `gaps` — what cannot be derived from the given facts.
- `contradictions` — find inconsistencies + provenance.

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
match `(step …)` forms ([TMS/ATMS analogue](../../../index/09-cognitive-architectures-neurosymbolic.md)).
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
