# Examples

Worked ein-lang fragments. The complete puzzles live in
[`examples/`](../../../../examples/) and exercise the engine's
acceptance tests.

This was [`docs/ir.md` §5](../../README.md) before the kernel-
documentation split.

---

## The four-token Zebra sentence

> *"The Norwegian lives in the first house."*

```lisp
(lives-in Norwegian House-1 :source "condition (10)")
```

The shortest meaningful IR — one fact whose `:source` derives the
a given (`:source`-carrying) fact.

## A larger Zebra fragment

```lisp
(rule transitive (?rel)
  :match  (and (?rel ?a ?b) (?rel ?b ?c) :where (neq ?a ?c))
  :assert (?rel ?a ?c)
  :why    "{?rel} is transitive.")

;; Schema + implicit assumptions (no :source → background)
(relation is-a       T T)
(relation co-located Attribute Attribute)
(relation right-of   Attribute Attribute)
(relation position   House House)                ; structural; a right-of derivation is a rule
;; The attribute hierarchy and its members are ordinary `is-a` facts.
(is-a House Attribute) (is-a Color Attribute) (is-a Nationality Attribute)
(is-a House-1 House) (is-a House-2 House) (is-a House-3 House)
(is-a Red Color) (is-a Green Color) (is-a Ivory Color)
(is-a Norwegian Nationality) (is-a Englishman Nationality)
;; Implicit: rule-application meta-facts
(transitive co-located)

;; Explicit puzzle conditions (each :source → a given)
(co-located Englishman Red    :source "condition (2)")
(right-of   Green Ivory       :source "condition (6)")
(co-located Norwegian House-1 :source "condition (10)")

(query :goal (co-located ?nationality Water))
```

The complete puzzle — 15 conditions + ten rule families — lives in
[`examples/zebra.ein`](../../../../examples/zebra.ein) (created
alongside this spec; see
[M1 acceptance §1-2](../../../../plans/m1_core_graph_reasoning/README.md)).

## Two ontologies for one puzzle

The same puzzle is encoded **two ways** in `examples/`. Both are valid
Ein; what differs is the *ontology* each commits to, which is what makes
the pair worth keeping — it is the only way to see which of the engine's
reasoning power is general and which is an artefact of how `zebra2.ein`
happens to be written.

### One generic link relation (`zebra.ein`)

Declares its own membership relations and links every attribute through a
single generic `co-located`:

```lisp
(relation type     T T)
(relation instance T T)
(type Nationality Attribute)
(instance Norwegian Nationality)
(instance Japanese  Nationality)
```

Nothing here is kernel syntax: `type` and `instance` are ordinary
relations this puzzle declares, exactly like `co-located`. The kernel
special-cases no head and builds no type/instance entity-view — a type
projection, if a puzzle wants one, is a user-space rule over its own
facts.

`zebra.ein` does not currently solve, and the reason is ontological
rather than linguistic: relation properties are declared *per relation*,
so one universal `co-located` gives the elimination rules no
per-attribute structure to bite on, and the hypothesis space does not
close.

### Typed attribute relations (`zebra2.ein`)

Uses the relation `is-a` with two recurring rules
(`transitive is-a` and `asymmetric is-a`):

```lisp
(relation is-a T T)
(is-a Nationality Attribute)
(is-a Norwegian Nationality)
(is-a Japanese  Nationality)
(transitive is-a)
(asymmetric is-a)
(sibling-exclusive is-a)
```

The inheritance hierarchy is just the `is-a` fact graph (closed under
`transitive is-a` after saturation). The kernel keeps no derived
type/instance view; anything that needs "the type-like nodes" reads the
`is-a` facts directly (e.g. the renderer's `_schema_nodes`) or via a
user-space rule. Alongside it, `zebra2.ein` splits `co-located` into five
*typed* attribute relations (`nation-loc`, `drink-loc`, …), each carrying
its own `bijective` declaration — that is what the elimination rules act
on, and the substantive difference from `zebra.ein`.

The categorical motivation (T as terminal object / limit of the
order viewed as a category) is documented in `zebra2.ein`'s header.

## Worked rule library

Both encodings use the same property-rule pattern. The `(rule …)`
forms from `zebra.ein`:

```lisp
(rule symmetric (?rel)
  :match  (?rel ?a ?b)
  :assert (?rel ?b ?a)
  :why    "{?rel} is symmetric: {?a} ↔ {?b}."
  :priority 1)

(rule transitive (?rel)
  :match  (and (?rel ?a ?b) (?rel ?b ?c) :where (neq ?a ?c))
  :assert (?rel ?a ?c)
  :why    "{?rel} is transitive."
  :priority 5)

(rule implies (?p ?q)
  :match  (?p ?a ?b)
  :assert (?q ?a ?b)
  :why    "{?p} implies {?q}."
  :priority 3)

(rule type-exclusivity ()
  :match  (and (is-a ?a ?T)
               (is-a ?b ?T)
               (neq ?a ?b))
  :assert (not (co-located ?a ?b))
  :why    "{?a} and {?b} are distinct members of {?T} — distinct slots."
  :priority 10)
```

`symmetric`, `transitive`, `implies` are **T2** rules
([`../01-ein-graph/02_rules.md` §2.2](../01-ein-graph/02_rules.md));
`type-exclusivity` is **T1**
([`../01-ein-graph/02_rules.md` §2.1](../01-ein-graph/02_rules.md))
— literal relation names (`is-a`, `co-located`) appear in the
LHS and RHS.

## Derived-fact dump

After saturation, an engine dump of the derived facts looks like
(flat forms; each carries `:rule` / `:using`, so it re-classifies to
derivations on reload):

```lisp
;; The engine derived (co-located House-1 Norwegian) from
;; condition (10) via the symmetric rule.
(co-located House-1 Norwegian :rule symmetric
                              :using ((co-located Norwegian House-1)))

;; Type-exclusivity: Norwegian and Japanese are distinct
;; Nationality members, so they're not co-located.
(not (co-located Norwegian Japanese) :rule type-exclusivity
                                     :using ((is-a Norwegian Nationality)
                                             (is-a Japanese  Nationality)))
```

> **Note** — the `:using` IR syntax above isn't yet round-trippable
> through the current grammar; the engine populates rule provenance
> programmatically. See
> [`01_grammar.md` §Reasoning](01_grammar.md) for the deferral.

## See also

- Full encoded puzzles:
  [`examples/zebra.ein`](../../../../examples/zebra.ein),
  [`examples/zebra2.ein`](../../../../examples/zebra2.ein).
- [`examples/README.md`](../../inference/zebra_walkthrough.md) — the
  Wikipedia human-style Zebra solution annotated step-by-step against
  `zebra2.ein`: NL sentence ↔ firing ein rule ↔ branch-depth ↔
  premises → conclusion, with the four `d=1` hypothesis branches and
  their learnt no-good clauses spelled out. M1 target (the inference
  column) and M2 target (the full NL ⇄ IR ⇄ NL row).
- [`01_grammar.md`](01_grammar.md) — the form-level grammar for what
  you see above.
- [`02_patterns.md`](02_patterns.md) — the pattern sub-language
  inside rule bodies.
- [`04_dot_rendering.md`](04_dot_rendering.md) — how these
  fragments render visually.
