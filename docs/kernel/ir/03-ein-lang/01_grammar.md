# Grammar — lexical rules and top-level forms

> **Source of truth for syntax: [§3, the complete grammar](#3-the-complete-grammar-ebnf),
> below.** §1 and §2 explain intent and structure; §3 is what parses.
>
> It was `ein.py/src/ein/ir/grammar.lark` — 244 lines of Lark, read by an
> Earley parser — until M1a
> [S1a.10.5](../../../../plans/m1a_rust/p1a.10_single_implementation/s1a.10.5_removal.md).
> The engine that is left parses by **recursive descent**
> ([`ein-ir/src/{lex,parse}.rs`](../../../../ein.rs/crates/ein-ir/src/parse.rs)),
> which is an implementation and not a specification, so the grammar was
> transcribed to EBNF here before the file went. §3 is checked against that
> parser rather than against the Lark it came from, and
> [§3.5](#35-what-keeps-this-document-honest) is what pins it: a 78-case
> decision table blessed while both parsers still ran, so the answers are
> lark's. That is the last thing the two implementations were used for.

This was [`docs/ir.md` §1 + §2](../../README.md) before the kernel-
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

The regexes above are the readable form. [§3.1](#31-lexical-grammar) is the
exact one, including the three lookaheads this table cannot show: `SYMBOL`'s
reserved-word exclusion, `WILDCARD`'s, and `STRING`'s refusal of an escaped
newline.

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

### Where a fact came from — its provenance annotation

A fact's origin is read off the annotation it carries, and nothing else
(see [`../01-ein-graph/01_kb.md` §3](../01-ein-graph/01_kb.md)):

| the fact carries… | it is… | meaning |
|---|---|---|
| `:rule` / `:using` | **derived** | the engine produced it (a rule firing) |
| `:source "(N)"` | **given** | an explicit, numbered problem statement |
| neither | **background** | an implicit assumption (schema, `is-a` enumeration, property tag) |

The distinction is **presentation only** — the engine treats every fact
alike, and fact identity is `(relation_name, args)` regardless. There is
no way to override it: `:layer ontology|fact|reasoning` used to, and
was removed in S1.22.1b (the loader **rejects** it, because the layer it
set fed a contradiction-detector restriction that silently accepted
inconsistent puzzles). Write the annotation that is true — a numbered
condition gets `:source`, a property tag gets nothing. See
[`../02-data-model/01_entities.md` §1.5](../02-data-model/01_entities.md)
and [`plans/ideas/04-nlp-to-graph-to-solver-pipeline.md` §Ontology
deduction by common sense](../../../../plans/ideas/04-nlp-to-graph-to-solver-pipeline.md)
for how the NL frontend recovers the ONTOLOGY-vs-FACT split from context.

### Relation declarator

```lisp
(relation <name> [<T1> <T2> ...]         ; signature: zero or more type atoms
  [:why <STRING>] [:cardinality <RANGE>]  ; optional metadata kw-pairs
  [...])
```

`relation` declares a relation-type node + its arg-type signature
(structural / spatial relations are plain `(relation …)`; a pattern-based
derivation is an ordinary `rule`; `a-priori` removed in S1.7.6). The loader
auto-stores each declaration as an ordinary fact `(relation R T1 T2 …)` so
rules can introspect signatures via a `(relation ?R ?A ?B)` pattern in
`:match`. Because of that, `relation` is **not** SYMBOL-excluded, so the
malformed wrapped-arg form `(relation R (T1 T2))` parses but is rejected at
LOAD time (not parse time).

**Arity — name + *zero or more* type atoms.** The grammar is
`relation_decl: "(" "relation" SYMBOL SYMBOL* kw_pair* ")"`
([§3](#3-the-complete-grammar-ebnf)), and the
loader requires exactly the same. So `(relation adult Person)` is a legal
**unary** declaration and `(relation opaque)` a legal **bare** one —
a relation node with no declared arg types. Only the *name* is mandatory;
`(relation)` is rejected at load. The wrapped-arg form
`(relation R (T1 T2))` stays rejected too (P1.3 R10) — the loader checks
that every arg after the name is a type atom or a kw-pair, so the inner
group cannot pass itself off as a bare declaration.

A bare declaration is **not a hypothesis target**: signature *presence* is
the kernel's "declared domain relation" signal (the table below), and an
empty signature deliberately fails it. Write `(relation R T)` — the
don't-care atom — when you want a guessable relation with no meaningful
type.

The blind enumerator fills **arity 1 and 2**
([`inference/hypgen.py`](../../../../ein.py/src/ein/inference/hypgen.py),
`_fill_slot`): a unary relation yields one candidate per focal object,
a binary one yields |objects| per focal object. Arity ≥ 3 declarations
load, store facts and saturate normally but are never guessed — the one
remaining M1 arity cut, and an *engine* limit rather than a grammar rule.
See [`../01-ein-graph/03_ein_model.md` §5.1](../01-ein-graph/03_ein_model.md)
for unary relations as subsets.

**The membership fact.** Because matching is arity-coupled,
`(relation ?R ?A ?B)` sees only *binary* declarations. Every declaration
therefore also stores the arity-1 fact `(relation R)` — the
arity-independent "is `?R` a declared relation?" pattern:

```lisp
(relation likes Person Drink)
;; stores BOTH  (relation likes Person Drink)   ← the signature mirror
;;        and   (relation likes)                ← the membership fact
(rule needs-a-relation () :match (relation ?R) :assert (…))
```

For a bare declaration the two coincide, so exactly one fact is stored.
Auto-vivified relations — property-tag carriers like `symmetric`, which
have no declaration — get neither, so `(relation ?R)` means precisely
*declared* relation.

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
and enumerations carry no annotation, so they read as background):

```lisp
(is-a House Attribute) (is-a Color Attribute)
(relation co-located Attribute Attribute)
(is-a Norwegian Nationality)
(is-a House-1 House)
(symmetric  co-located)
(transitive co-located)
(right-of House-2 House-1 :source "condition (1)")  ; "five in a row" — structural, but it *is* condition (1)
```

#### What the signature means — userspace types, kernel structure

The kernel imposes **no type system**
(S1.7.23),
yet a signature is not inert. It plays **two unrelated roles**, and they
never mix: rules read the signature's *content* as types; the kernel reads
its *shape* as structure. This subsection is the definitive statement of
that split.

**As types — userspace only.** The mirror fact is the whole mechanism:
everything type-like keys off `(relation ?R ?A ?B)` **in rules**, never in
the engine — [`std.bijection`](../../../../stdlib/bijection.ein)'s
typecheck stack (`(and (relation ?R ?A ?B) (bijective ?R)
(typecheck-hierarchy ?isa))`),
[`std.algebra`](../../../../stdlib/algebra.ein)'s converse
domain/range check, `std.typing`'s hierarchy wiring, zebra2's
`disjunctive-prune`. The kernel never resolves a signature atom to
anything: `Relation.signature` holds *opaque atoms*, and since S1.7.23
there are no `Type` entities to resolve them **to**
([`../02-data-model/01_entities.md` §1.1](../02-data-model/01_entities.md)).

**As structure — kernel, three signals.** The engine reads the
signature's shape only, and only in these places:

| signal | site | effect |
|---|---|---|
| signature **non-empty** | `inference/hypgen.py` (`_raw_candidates`), `inference/closed.py` (`emit_closed`) | marks a *declared domain relation* — eligible for hypothesis generation and for `__closed__` auto-inference. Property / rule-name relations (auto-vivified, empty signature) are skipped by both. |
| signature **length** | `inference/hypgen.py` (`_fill_slot`) | length 1 → one candidate per focal object; length 2 → the pairwise fill; length ≥ 3 → unenumerated (the M1 arity cut noted above). |
| signature **atoms** | `inference/hypgen.py` (`_candidate_objects`) | the declared type-role names (`Attribute`, `House`, `T`, …) are subtracted from the candidate-**object** pool, so the enumerator never guesses *about* a type node. |

Two further reads are ergonomic rather than semantic: the declaration's
`:why` render template (above) and the signature column of `ein saturate`'s
relation table (`cli/saturate.py`).

So "are the signature's type atoms used explicitly through rules, or
implicitly in the kernel?" — **both, in different roles.** As *types* they
exist only for userspace rules; as *structure* (present / length-2 /
name-set) they steer hypothesis generation. No kernel site ever interprets
an atom like `Attribute` as a type. See also
[`08_self_describing.md` §3](08_self_describing.md) for the userspace half
in its own right, and
[`06_reserved_names.md` §Not reserved](06_reserved_names.md) for why `T` is
a convention rather than a kernel atom.

**Why `relation` is a kernel declarator and not a stdlib word** (decided
2026-08-17): because demoting it would remove no kernel *interpretation* —
the three structural signals above are what makes hypothesis generation
terminate, so they survive any demotion, and the kernel would go on
hardcoding the name `relation`, merely as a reserved *fact head* (the
`__closed__` / `not` / `false` category) rather than a declarator. The
reserved set would be renamed, not shrunk. What demotion *would* cost is
concrete: the malformed / shadowed-name / duplicate-declaration checks lose
their load-time `loc` (a conflicting re-declaration would become two stored
facts and an ambiguous signature lookup feeding the enumerator, with no
diagnostic), and `:why` would be silently dropped, since facts discard
unrecognised kw-pairs. Userspace gives up nothing for this: the
declaration is *already* published as the mirror fact — and, since the
same decision, as the **membership fact** above, which is what the
question "can a rule check that `?R` is a relation?" was really after. The
friction was arity-coupling in the *published* schema, not the declarator;
publishing more fixed it without demoting anything.

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

The origin of each fact follows the
[annotation rule](#where-a-fact-came-from--its-provenance-annotation): a
`:source` makes it a given problem statement; `:rule` / `:using` make it
an engine derivation; no annotation makes it a background assumption.
Rule-application facts (`(symmetric co-located)`,
`(implies right-of next-to)`) carry no annotation → background: the
puzzle text never says "co-located is
symmetric"; that's universal context, the *meta* of the relation, while a
`rule` is the meta of the *engine*.

`all-different` is **not** a kernel primitive; pairwise distinctness within
a category is derived by `type-exclusivity` from the `(is-a X T)`
facts. Genuinely puzzle-specific structural shapes (parity, budget, …) just
take their own head: `(budget-total X Y)`.

A flat explicit-conditions example (each numbered condition is a given
via its `:source`; the un-annotated property tag is background):

```lisp
(lives-in Norwegian House-1 :source "condition (10)")
(co-located Englishman Red  :source "condition (2)")
(symmetric  co-located)                              ; property tag — implicit, background
```

### Derived facts — engine working memory

The engine *derives* facts at runtime and dumps them as flat forms
annotated with `:rule` (which rule fired) + `:using` (which premises it
consumed) instead of `:source` — so they read back as derivations:

```lisp
(<name> <arg>* :rule <RuleName> :using (<premise-id>+))
(not <expr>    :rule <RuleName> :using (<premise-id>+))
```

A hand-authored puzzle has none; they appear in engine dumps, which
round-trip through `parse` / `dump` — exactly, since S1.22.1b: the dump
carries the provenance and the provenance *is* the origin, so there is
no residue needing a `:layer` patch. Example derived facts:

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
> S1.2.3 T1.2.3.4.
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
[Q15](../../../../plans/open_questions.md#q15)
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
| `(absent P)`       | negation-as-failure — holds iff no fact matches P in the world at the closure boundary (see below) | S1.5.8c.1   |
| `(open P)`         | `std.macro` macro for `(and (absent P) (absent (not P)))` — the third-state match: P is neither asserted nor negated | S1.5.8c.3b |
| `(forall ?b (G) (B))` | `std.macro` macro for `(absent (and G (absent B)))` — for every binding of `?b` satisfying guard G, body B must hold | S1.5.8c.3a |

The three-state model: at any moment, a potential fact P is
**asserted** (matched by `(P)`), **negated** (matched by `(not P)`),
or **open** (matched by `(open P)`). The earlier overloaded
`(not P)` meaning (default NAF) was dropped in S1.5.8c — NAF must
now be written explicitly as `(absent P)`. `forall` and `open` are
load-time `(macro …)` expansions in terms of `absent` (the
[`std.macro`](../../../../stdlib/macro.ein) module since
S1.5.9 — import them; see
[`06_reserved_names.md` §macro sugar](06_reserved_names.md#pattern-macro-sugar-forall--open--not-reserved));
the compiler itself sees only `absent` + nested patterns, and lifts
each *top-level* one out of the match plan (below).

#### NAF evaluation timing — the closure/world boundary

`(absent P)` is a **query over the current fork-local world, answered
at a positive fixpoint** (S1.21.8). The compiler *lifts* every
top-level `(absent …)` out of the rule's match plan
([`compile.split_naf`](../../../../ein.py/src/ein/inference/compile.py)),
so what the matcher runs is purely positive and a match whose disjunct
carries guards is **parked** instead of fired. When the positive
closure quiesces, the saturator builds a
[`World`](../../../../ein.py/src/ein/inference/world.py) over that
fixpoint and asks the guards there — once, decisively — admitting the
firing iff every one passes. That is the *only* evaluation point: the
old fire-time re-check (`match.absents_still_pass`, S1.5a.1) is
**deleted, not bypassed**, because exactly one candidate is admitted
per boundary round into an empty queue, so it fires against precisely
the world its guard was judged in.

For a rule author, three consequences:

- **`(absent P)` asks "is `P` missing from the finished positive
  derivation?", not "has `P` not been derived *yet*?"** A rule that
  used to fire because its watched fact simply had not been produced
  yet no longer does.
- **`:priority` no longer decides what is derivable.** On a stratified
  rule set the result is priority-independent; priority still orders
  firings (hence the trace), but the priority-band discipline zebra2
  needed for soundness — every producer of a watched relation at a
  strictly lower number than every watcher — is now **advisory**.
- **Non-stratified rule sets are still answered by operational order.**
  `p ← absent q` together with `q ← absent p` has two models; the
  engine picks one (by boundary-admission order) and does not report
  that the other exists. `(config :warn-derived-naf true)` warns on the
  shape that can cause it — NAF over a *rule-derived* relation — and a
  static stratification check remains future work.

A **nested** `(absent …)` — what `forall` desugars to,
`(absent (and G (absent B)))` — is not lifted separately: the whole
double negation is one query, evaluated as a unit against that same
world.

The normative definition — worlds, the single evaluation point E1, the
corollaries the engine relies on, and what is explicitly *not*
provided (stratification, stable models, retraction) — is
[`inference/absent_semantics.md`](../../inference/absent_semantics.md);
the operational narrative is in the
[inference README](../../inference/README.md).

#### Premise order around a guard

Lifting a guard to the boundary is **not** reordering it. Where an
`(absent …)` sits among the positive premises still decides *what it
asks* — a rule of the language, unchanged by S1.21.8:

| written                        | asks                                                              |
|--------------------------------|-------------------------------------------------------------------|
| `(and (absent (P ?x)) (Q ?x))` | is there **no `P` at all**? — `?x` is still free where the guard stands |
| `(and (Q ?x) (absent (P ?x)))` | is there **no `P` for this `?x`**? — `?x` was bound by the preceding `(Q ?x)` |

The compiler records the variables bound by the premises that
*preceded* each guard
([`NafGuard.scope`](../../../../ein.py/src/ein/inference/compile.py)),
and the boundary projects the completed bindings back down to that set
— so a lifted guard is exactly as strong as it was in place, no more
and no less. Write the binding premise **before** the guard when you
mean "for this `?x`".

### Query

```lisp
(query
  :goal <expr>                             ; what to find / verify
  [:goal-text <STRING>]                    ; NL headline template (optional)
  [:hrules (<activator> …)])               ; hypothesis-generator activators
```

The engine answers idea 03's three task classes —
[`docs/ideas/03-three-task-classes.md`](../../../../plans/ideas/03-three-task-classes.md)
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

Per [Q21](../../../../plans/open_questions.md#q21),
`(trace …)` is the **same IR** as input — same parser, same AST,
same dumper. The engine can reason about its own traces; rules can
match `(step …)` forms ([TMS/ATMS analogue](../../../lib/09-cognitive-architectures-neurosymbolic.md)).
Per [Q18](../../../../plans/open_questions.md#q18)
each derived edge's provenance tuple `(rule, premise_edges, source)`
is literally a `(step …)` form — provenance and trace are the same
data structure under different views.

---

## §3 The complete grammar (EBNF)

**Notation** — W3C EBNF, as in the XML specification:

| | |
|---|---|
| `A ::= B` | production |
| `'x'` | literal, matched over the *character* stream |
| `A B` | concatenation · `A \| B` alternation · `( … )` grouping |
| `A?` `A*` `A+` | zero-or-one, zero-or-more, one-or-more |
| `[a-z]` `[^"\\]` | character class, and its complement |
| `A - B` | matches `A` but not `B` |
| `/* … */` | comment |

Two layers, and the boundary between them is not the usual one — see
[§3.3](#33-why-this-is-not-a-plain-cfg-over-a-token-stream) before assuming
a separate scanner pass.

### §3.1 Lexical grammar

Terminals, over the character stream. Every one is ASCII-anchored: a
non-ASCII character can appear only inside a `STRING` or a comment.

```ebnf
Char          ::= /* any character */

/* Trivia — skipped everywhere between tokens, never a token itself. */
Trivia        ::= Space | LineComment | BlockComment
Space         ::= /* one character with Unicode White_Space, plus U+001C-U+001F */
LineComment   ::= ';' [^#xA]*
BlockComment  ::= '#|' ( Char* - ( Char* '|#' Char* ) ) '|#'   /* non-nesting */

/* Atoms. */
SYMBOL        ::= ( '__' )? [A-Za-z] [A-Za-z0-9_*.-]*
                  /* …and the match is rejected outright when the input at
                     this position begins with one of the eleven RESERVED
                     words followed by a non-word character (or by end of
                     input). The lookahead is START-anchored, so `std.rule`
                     is one SYMBOL and `rule.x` is not a SYMBOL at all. */
RESERVED      ::= 'not' | 'and' | 'or' | 'neq' | 'rule' | 'hrule'
                | 'query' | 'config' | 'trace' | 'macro' | 'import'
                  /* `relation` is deliberately NOT here: rules match
                     `(relation ?R ?A ?B)` patterns, so it must lex as an
                     ordinary SYMBOL. `(relation R (T1 T2))` therefore parses
                     — as a fact — and the LOADER rejects it. */
WordChar      ::= [A-Za-z0-9_]                    /* Python's `\w`, as `\b` uses it */

VAR           ::= '?' [A-Za-z] [A-Za-z0-9_*-]*    /* no '.', unlike SYMBOL */
KEYWORD       ::= ':' [a-z] [A-Za-z0-9_-]*        /* lower-case first character */
WILDCARD      ::= '_'
                  /* …only where the next character is not [A-Za-z0-9_], so
                     the leading `__` of a dunder atom is never two wildcards. */
EQ            ::= '='
RANGE         ::= [0-9]+ '..' ( [0-9]+ | '*' )    /* digit-anchored */
INT           ::= '-'? [0-9]+
STRING        ::= '"' ( [^"\\] | '\\' ( Char - #xA ) )* '"'
```

`(` and `)` carry no terminal name: they appear as literals in §3.2, which is
the only place they occur.

Four terminal facts that are load-bearing and easy to lose:

1. **`SYMBOL`'s reserved-word exclusion is a negative lookahead with a word
   boundary**, not a set difference. `neq` is not a `SYMBOL`; `neq_test`
   is. `rule-x` is not (the `-` is not a word character, so the boundary
   holds and the exclusion fires); `rulex` is.
2. **`__dunder__` atoms lex as `SYMBOL`.** `(?:__)?` still requires a letter
   after it, so a bare `_` stays `WILDCARD` and `_x` is neither. `WILDCARD`'s
   own lookahead is what stops `__closed__` from starting as two wildcards.
3. **`RANGE` is tried before `INT`** — both start on a digit — and it never
   collides with a dotted `SYMBOL`, which is letter-anchored.
4. **`STRING` takes a raw newline but not an escaped one.** `[^"\\]` accepts
   `#xA`; `'\\' ( Char - #xA )` refuses it, because the `.` of the original
   `\\.` was Python's and Python's `.` does not match a newline. Unescaping
   is *minimal*: `\n` `\t` `\r` map, every other `\X` is `X`, and there is no
   `\xNN`, no `\uNNNN`, no octal — so `"\d"` is `d`.

### §3.2 Phrase grammar

`Program` is the start symbol. Trivia may appear between any two terminals.

```ebnf
Program       ::= Form*

Form          ::= RelationDecl
                | RuleDecl | HruleDecl
                | QueryForm | ConfigForm
                | TraceForm
                | MacroDecl | ImportForm
                | FactForm                        /* anything else */

/* ── Declarators (the closed reserved set) ───────────────────────── */
RelationDecl  ::= '(' 'relation' SYMBOL SYMBOL* KwPair* ')'
RuleDecl      ::= '(' 'rule'  SYMBOL RuleParams KwPair+ ')'
HruleDecl     ::= '(' 'hrule' SYMBOL RuleParams KwPair+ ')'
RuleParams    ::= '(' VAR* ')'                    /* MANDATORY; `()` when non-generic */
QueryForm     ::= '(' 'query'  KwPair+ ')'
ConfigForm    ::= '(' 'config' KwPair* ')'        /* `(config)` is valid: all defaults */
MacroDecl     ::= '(' 'macro' SYMBOL MacroParams Value ')'
MacroParams   ::= '(' VAR+ ')'
ImportForm    ::= '(' 'import' SYMBOL KwPair* ')' /* SYMBOL is a dotted logical name */

/* ── Facts: the flat default, i.e. any non-declarator head ───────── */
FactForm      ::= EqFact | NotForm | GenericFact
EqFact        ::= '(' EQ Value Value KwPair* ')'
GenericFact   ::= '(' SYMBOL Value* KwPair* ')'

/* ── Trace: engine output, same IR as input ──────────────────────── */
TraceForm     ::= '(' 'trace' TraceEvent* ')'
TraceEvent    ::= StepDecl | BranchOpen | BranchClose
                | BranchRef | ContradictionDecl | SymmetryDecl
StepDecl          ::= '(' 'step'            SYMBOL KwPair* ')'
BranchOpen        ::= '(' 'branch-open'     SYMBOL KwPair* ')'
BranchClose       ::= '(' 'branch-close'    SYMBOL KwPair* ')'
BranchRef         ::= '(' 'branch-ref'      SYMBOL ')'
ContradictionDecl ::= '(' 'contradiction'   SYMBOL KwPair* ')'
SymmetryDecl      ::= '(' 'symmetry-class'  SYMBOL KwPair* ')'

/* ── The value sub-language (the interior of every KwPair) ───────── */
KwPair        ::= KEYWORD Value                   /* KEYWORD is never a Value */
Value         ::= NotForm | AndForm | OrForm | NeqForm
                | GenericList
                | SYMBOL | VAR | WILDCARD | INT | RANGE | STRING
GenericList   ::= '(' ')'
                | '(' ListHead ListItem* ')'
ListHead      ::= SYMBOL | VAR | WILDCARD | EQ    /* never KEYWORD */
ListItem      ::= Value | KwPair

/* ── Kernel meta-primitives (shape-pinned) ───────────────────────── */
NotForm       ::= '(' 'not' Value KwPair* ')'
NeqForm       ::= '(' 'neq' Value Value ')'
AndForm       ::= '(' 'and' Value+ KwPair* ')'
OrForm        ::= '(' 'or'  Value+ KwPair* ')'
```

`NotForm` carries trailing `KwPair*` so that a *derived* `(not …)` fact's
provenance (`:rule type-exclusivity :using (c10)`) attaches naturally; in
`:match` / `:assert` position the loader rejects them.

### §3.3 Why this is not a plain CFG over a token stream

The lexical and phrase layers are **not** separated by a scanner pass. Both
implementations tokenize *contextually*: at each position the parser asks
"does the terminal I want start here?", and a word may lex differently
depending on what is being parsed.

- Lark ran an Earley parser with a dynamic lexer, which explores every
  tokenization and keeps the ones that parse.
- `ein-ir` commits by recursive descent, and recovers the same language by
  trying the **literal reading before the `SYMBOL` reading** at every head
  position where both could match — the reason `LITERALS` is a data table in
  `lex.rs` and not a `match` arm.

The consequence to keep in mind when reading §3.2: a quoted literal like
`'rule'` is a *character* match, not a token class, and the input `(rule.x A)`
is a **parse error** rather than a fact with head `rule.x` — `rule.x` is not a
`SYMBOL` (start-anchored exclusion), and the literal `rule` matches instead,
after which `.x` scans as nothing. Symmetrically, `(std.rule A)` is a fact:
the exclusion never fires because the input does not *begin* with a reserved
word.

### §3.4 What the grammar deliberately does not enforce

Everything below parses and is rejected later, by the loader
([`from_ir.rs`](../../../../ein.rs/crates/ein-ir/src/from_ir.rs)) or by the
compiler. A parse error and a load error are different exit paths with
different messages, and the corpus has a fixture directory for each
([`examples/broken/`](../../../../examples/broken/)).

- **Which keywords each form requires** — `:match` / `:assert` in a rule,
  `:goal` in a query, `:rule` / `:using` / `:derives` in a step.
- **Arity, ground-vs-pattern, and type-checking** of a fact's arguments
  against its `(relation …)` signature.
- **Unbound variables**: a `:assert` variable that no `:match` premise binds.
- **`(relation R (T1 T2))`** — the wrapped-signature form, which parses as a
  `GenericFact` because `relation` is a `SYMBOL`.
- **`:layer`** — removed in S1.22.1b. It parses as an ordinary `KwPair` and
  the loader rejects it.
- **The named structural-predicate registry**, and the `__closed__` /
  `__symmetric__` dunder triggers, which are ordinary atoms here and kernel
  meaning at load
  ([`06_reserved_names.md`](06_reserved_names.md)).

### §3.5 What keeps this document honest

A specification with no test is a wish. Two checked-in artefacts pin §3, and
both were blessed while the second parser still existed — which is what makes
them lark's answers rather than ein.rs agreeing with itself:

| | |
|---|---|
| [`ein-ir/tests/grammar_decisions.rs`](../../../../ein.rs/crates/ein-ir/tests/grammar_decisions.rs) | a **78-case decision table**, one line each, against `tests/golden/grammar_decisions.txt`. It is where every sharp edge in §3.1 is nailed down: `(rule-x A)` vs `(rulex A)` vs `(std.rule X)` vs `(neq_test X)`, `_` vs `_x` vs `__closed__`, `1..5` / `1..*` / `1..`, `"a⏎b"` vs `"a\⏎b"`, `()` vs `(x ())`, an unterminated block comment. Plus the four `examples/broken/*.ein` messages, whose `.expected` files hold ein.py's text |
| `corpus_shapes.md5`'s `ir[parse]` lines | every corpus file's parse, digested — the *structure* of an acceptance, not just the fact of one |

**Corpus coverage of §3.2, counted** (files under `examples/` + `stdlib/`
containing at least one): `relation` 90 · `rule` 88 · `query` 67 · `and` 65 ·
`import` 42 · `not` 43 · `neq` 39 · `hrule` 23 · `config` 18 · `or` 10 ·
`macro` 5 · `trace` 3 · `=` 3 · `step` 1. Terminals: `VAR` 105 · `STRING` 95 ·
`INT` 84 · dunder `SYMBOL` 11 · `RANGE` 4 · `WILDCARD` 3.

**Five productions have no exerciser at all**, and it is better to write that
down than to let a reader assume otherwise: `BranchOpen`, `BranchClose`,
`BranchRef`, `ContradictionDecl` and `SymmetryDecl` appear in no `.ein` file
in the tree, and nothing in the engine emits them — they are reachable only
through the parser's own unit tests. They are the trace vocabulary the
renderer has not needed yet ([Q21](../../../../plans/open_questions.md#q21)),
kept in the grammar so that a `(branch-open …)` never silently misclassifies
as a fact.

### §3.6 Derived artefacts

M2's GBNF for constrained LLM decoding is derived from this grammar
([`plans/ideas/01`](../../../../plans/ideas/01-self-modifying-constraint-language.md)).
The route was a `Lark → GBNF` translator over `grammar.lark`; with the Lark
file gone, the source is §3.1 + §3.2 and the translation is by hand or by a
generator that reads this document.

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
