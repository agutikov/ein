# Grammar — lexical rules and top-level forms

> **Source of truth for syntax: [`00_ebnf.md`](00_ebnf.md)** — the complete
> grammar, in EBNF. This document explains intent and structure; that one is
> what parses.
>
> It was `ein.py/src/ein/ir/grammar.lark` until M1a
> [S1a.10.5](../../../history/m1a_rust/README.md#s1a105--the-removal),
> and the engine that is left parses by recursive descent — an implementation,
> not a specification — so the grammar was transcribed before the file went.

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

The regexes above are the readable form.
[`00_ebnf.md` §1](00_ebnf.md#1-lexical-grammar) is the exact one, including
the three lookaheads this table cannot show: `SYMBOL`'s reserved-word
exclusion, `WILDCARD`'s, and `STRING`'s refusal of an escaped newline.

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
| `query` | what to ask the engine — and, with `:expect`, what the answer should be | `(query :goal … [:goal-text …] [:hrules …] [:expect …])`; several per file |
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
alike, and fact identity is `(relation_name, args)` regardless. There is no
way to override it, and no need for one: write the annotation that is true —
a numbered condition gets `:source`, a property tag gets nothing. See
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
([`00_ebnf.md`](00_ebnf.md)), and the
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
([`hypgen.rs`](../../../../ein.rs/crates/ein-infer/src/hypgen.rs),
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
| signature **non-empty** | `ein-infer/hypgen.rs` (candidate enumeration), `closed.rs` (`emit_closed`) | marks a *declared domain relation* — eligible for hypothesis generation and for `__closed__` auto-inference. Property / rule-name relations (auto-vivified, empty signature) are skipped by both. |
| signature **length** | `ein-infer/hypgen.rs` (the slot fill) | length 1 → one candidate per focal object; length 2 → the pairwise fill; length ≥ 3 → unenumerated (the M1 arity cut noted above). |
| signature **atoms** | `ein-infer/hypgen.rs` (`candidate_objects`) | the declared type-role names (`Attribute`, `House`, `T`, …) are subtracted from the candidate-**object** pool, so the enumerator never guesses *about* a type node. |

Two further reads are ergonomic rather than semantic: the declaration's
`:why` render template (above) and the signature column of `ein saturate`'s
relation table (`ein-cli/saturate.rs`).

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
round-trip through `parse` / `dump` **exactly**: the dump carries the
provenance, the provenance *is* the origin, and no further annotation is
needed to reproduce it. Example derived facts:

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
[Q15](../../../../plans/open_questions.md#q15--rule-ordering)
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
| `(unknown P)`      | `std.macro` macro for `(and (absent P) (absent (not P)))` — the third-state match: P is neither asserted nor negated | S1.5.8c.3b |
| `(forall ?b (G) (B))` | `std.macro` macro for `(absent (and G (absent B)))` — for every binding of `?b` satisfying guard G, body B must hold | S1.5.8c.3a |

The three-state model: at any moment, a potential fact P is
**asserted** (matched by `(P)`), **negated** (matched by `(not P)`),
or **open** (matched by `(unknown P)`). The earlier overloaded
`(not P)` meaning (default NAF) was dropped in S1.5.8c — NAF must
now be written explicitly as `(absent P)`. `forall` and `unknown` are
load-time `(macro …)` expansions in terms of `absent` (the
[`std.macro`](../../../../stdlib/macro.ein) module since
S1.5.9 — import them; see
[`06_reserved_names.md` §macro sugar](06_reserved_names.md#pattern-macro-sugar-forall--unknown--not-reserved));
the compiler itself sees only `absent` + nested patterns, and lifts
each *top-level* one out of the match plan (below).

#### Conclusion forms in `:assert`

A conclusion is ordinarily a fact pattern, with `(and …)` concluding several
at once (A13 multi-assert). Two heads are **verdicts about the state** rather
than facts, and neither is stored as one:

| conclusion | semantics | added in |
|---|---|---|
| `(false)` | direct ⊥ — this branch is contradictory | S1.5.8c |
| `(open)` / `(open ?R)` | this state is **unfinished**: it owes a witness, and `(open ?R)` names the relation whose extent is incomplete | M1d S1d.2.3 |

`(false)` is stored — a contradiction survives any extension, so a dead state
stays dead — and `open` is not, because openness exists to be *destroyed* by
an extension and a stored one would survive its own discharge. It is a tally
on the search-lattice node instead, read once per quiescent KB **after** the
fixpoint; a rule that asserts it is an *obligation* and never enters the
saturation agenda. `(open ?R)` names the incomplete relation and nothing
else: the witness domain and the slot to fill are projected out of the rule's
own `(absent …)`, and five shapes are refused at load rather than guessed.
The form, the refusals and their messages are in
[`06_reserved_names.md` § the verdict atom](06_reserved_names.md#the-verdict-atom--open-m1d-p1d2-s1d23-read-since-s1d24)
and [`defined_behaviour.md` §4.2](../../defined_behaviour.md).
**The tally is read twice.** Since S1d.2.4
[`obligations::tally`](../../../../ein.rs/crates/ein-infer/src/obligations.rs)
reports it — on `--events` as one `owe` line per undischarged instance, in
`--json-summary`'s `owes` block, and as the trace's *Outstanding obligations*
section — and since S1d.2.6 the **verdict** reads it: a state that is
consistent, quiescent and complete by the generator's test while still owing a
witness is `Open`, not `Solution`. Since S1d.2.5 the search reads it a third
way, as the middle rung of the hypothesis ladder: the facts that would
discharge what the state owes are what it branches on.

#### NAF evaluation timing — the closure/world boundary

`(absent P)` is a **query over the current fork-local world, answered
at a positive fixpoint** (S1.21.8). The compiler *lifts* every
top-level `(absent …)` out of the rule's match plan
([`compile.rs`](../../../../ein.rs/crates/ein-infer/src/compile.rs)),
so what the matcher runs is purely positive and a match whose disjunct
carries guards is **parked** instead of fired. When the positive
closure quiesces, the saturator builds a
[the boundary phase](../../../../ein.rs/crates/ein-infer/src/saturator.rs) over that
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
([`NafGuard::scope_of`](../../../../ein.rs/crates/ein-infer/src/plan.rs)),
and the boundary projects the completed bindings back down to that set
— so a lifted guard is exactly as strong as it was in place, no more
and no less. Write the binding premise **before** the guard when you
mean "for this `?x`".

### Query

```lisp
(query
  :goal <expr>                             ; what to find / verify
  [:goal-text <STRING>]                    ; NL headline template (optional)
  [:hrules (<activator> …)]                ; hypothesis-generator activators
  [:hypothesis-relations (<rel> …)]        ; speculate only these
  [:no-hypothesis (<rel> …)]               ; …or never these
  [:expect <expectation>])                 ; what the answer should be (M1c)
```

**The keywords are an allow-list.** Those six, plus the obsolete `:mode`, are
what a `(query …)` may carry; anything else is a **load error**. It used to be
a silent no-op, which was survivable while a query only asked a question and
became untenable the moment one could carry a *test*: a mistyped `:expect`
that loaded, checked nothing and said nothing is the exact failure the keyword
exists to prevent.

**A file may carry several `(query …)` blocks**, each an independent question
over the same ontology, rules and facts. Before M1c
[S1c.1.2](../../../history/m1c_external_validation/README.md#s1c12--how-a-program-states-what-it-expects)
the **last** one silently won and the rest were discarded at load. `ein solve`
now runs every one of them in order, printing `query <i> of <n>` above each;
the flags that name a single output path — `--events`, `--trace`,
`--json-summary`, `--dump-states` — are refused on a file that asks more than
one question, because one path cannot hold two runs and overwriting quietly is
the discard again under another name. `(config …)` keeps last-wins: a config is
a *setting*, a query is *content*, and the two want opposite rules.

The engine answers idea 03's three task classes —
[`docs/ideas/03-three-task-classes.md`](../../../../plans/ideas/03-three-task-classes.md)
— *solve* (a unique model), *gaps* (under-determined: many models), and
*contradictions* (inconsistency + provenance). But these are **three answers
to one problem, read off the result `k`** (the count of distinct complete
models: `k = 1` / `k > 1` / `k = 0`) — **not** chosen up front. There is one
`ein solve`, no `--mode` flag, and the engine's goal test is hardwired to
SOLVE. A `:mode` keyword on the query is **obsolete** (vestigial; the engine
ignores it) — omit it.

**`:expect` — what the answer should be (M1c
[S1c.1.2](../../../history/m1c_external_validation/README.md#s1c12--how-a-program-states-what-it-expects)).**
A query may state its own answer, and the engine checks it: `ein solve` exits
**1** when the claim is false, and prints what disagreed. Three shapes, and the
verdict is *implied* by which one is used rather than asserted beside it:

```lisp
:expect (model <fact>*)                 ; exactly one model — Solution
:expect (or (model …) (model …) …)      ; that SET of models — k is the count
:expect (false)                         ; Contradiction — the kernel's own ⊥
```

Three rules, and the third is the one that gives the form its teeth:

1. **The goal's relations are mandatory.** An expectation that does not pin
   what the query asked is not an expectation, and does not load.
2. **More is allowed.** Any ground fact may be listed, including a `(not …)`,
   so a test can pin the consequence at a distance rather than restating the
   rule it is about.
3. **Naming a relation closes it.** If `:expect` mentions `pet-loc` at all,
   the listed `pet-loc` facts are that relation's *complete* extent in the
   model. Relations it never mentions are unconstrained.

Rule 3 sits between the two useless extremes: a per-fact assertion cannot
catch a **surplus** fact, and a whole-state golden pins 250 facts of `is-a*`
and activator noise that no test means to assert. Two clarifications it needs:
**stored negatives are not closed** — closing `pet-loc` says nothing about the
extent of `(not (pet-loc …))`, and a listed `(not …)` is checked for presence
like any other fact — and `(or …)` compares model **sets**, never sequences,
because the order a search finds models in is not observable.

The value is checked at *load*: an expectation that is not one of the three
shapes, that names a relation no declaration or fact makes, that omits the
goal's relations, or that contains a `?var` (an expectation is an answer, not
a pattern) is a load error. `examples/features/10_expect.ein` is the worked
fixture and `examples/broken/load/expect_*.ein` are the four refusals.

**`ein test` is the runner** (M1c
[S1c.1.3](../../../history/m1c_external_validation/README.md#s1c13--ein-test)).
`ein solve` checks an `:expect` because ignoring one would be worse than not
having the keyword; `ein test <file|dir>…` exists so that a corpus of them is a
**status code** rather than something to read:

```sh
ein test examples/features/          # every .ein under it, sorted
ein test puzzle.ein -v               # …and say what held, with the verdict and k
```

Three differences from `solve`, and each is the form's semantics rather than a
preference:

- **It exhausts.** An expectation is a claim about the *exhausted* answer —
  `Solution` means one model *and no other* — so a search stopped at `-n`
  establishes only a lower bound on `k` and confirms nothing. There is no `-n`
  and no `--exhaustive` on `test`; exhausting is the behaviour.
- **It runs only what the expectations need.** A query with no `:expect` states
  nothing and is never solved, so a directory of programs costs one load each
  plus a search for each claim.
- **Exit 1 means a claim is false**, so a *load* error takes **2** — the code
  that already means "this run is not a verdict". A runner that cannot tell a
  broken file from a false claim is the one failure a test runner must not
  have, and for the same reason a selection that checked nothing exits 2 as
  well.

The verdict a stopped search cannot confirm has one more spelling under either
command: **`NOT CHECKED`**, which is not a pass and takes the same exit code a
false claim does. Under `test` the only thing that can still produce it is the
lattice depth cap, `--max-set-size` — a `k = 0` from a capped search is "no
model within the cap", not "no model".

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

Per [Q21](../../../../plans/open_questions.md#q21--ir--dot-structural-isomorphism),
`(trace …)` is the **same IR** as input — same parser, same AST,
same dumper. The engine can reason about its own traces; rules can
match `(step …)` forms ([TMS/ATMS analogue](../../../lib/09-cognitive-architectures-neurosymbolic.md)).
Per [Q18](../../../../plans/open_questions.md#q18--provenance-granularity)
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
- [`../02-data-model/`](../02-data-model/) — the entities the
  loader produces from these forms.
