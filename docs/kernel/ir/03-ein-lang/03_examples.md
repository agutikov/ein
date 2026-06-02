# Examples

Worked ein-lang fragments. The complete puzzles live in
[`examples/`](../../../../examples/) and exercise the engine's
acceptance tests.

This was [`docs/ir.md` §5](../../../ir.md) before the kernel-
documentation split.

---

## The four-token Zebra sentence

> *"The Norwegian lives in the first house."*

```lisp
(lives-in Norwegian House-1 :source "condition (10)")
```

The shortest meaningful IR — one fact whose `:source` derives the
FACT layer.

## A larger Zebra fragment

```lisp
(rule transitive (?rel)
  :match  (and (?rel ?a ?b) (?rel ?b ?c) :where (neq ?a ?c))
  :assert (?rel ?a ?c)
  :why    "{?rel} is transitive.")

;; Schema + implicit assumptions (no :source → ONTOLOGY layer)
(type Attribute)
(type House Attribute) (type Color Attribute) (type Nationality Attribute)
(relation co-located Attribute Attribute)
(relation right-of   Attribute Attribute)
(relation position   House House)                ; structural; a right-of derivation is a rule
;; Implicit: instance enumeration
(instance House-1 House) (instance House-2 House) (instance House-3 House)
(instance Red Color) (instance Green Color) (instance Ivory Color)
(instance Norwegian Nationality) (instance Englishman Nationality)
;; Implicit: rule-application meta-facts
(transitive co-located)

;; Explicit puzzle conditions (each :source → FACT layer)
(co-located Englishman Red    :source "condition (2)")
(right-of   Green Ivory       :source "condition (6)")
(co-located Norwegian House-1 :source "condition (10)")

(query :mode solve :goal (co-located ?nationality Water))
```

The complete puzzle — 15 conditions + ten rule families — lives in
[`examples/zebra.ein`](../../../../examples/zebra.ein) (created
alongside this spec; see
[M1 acceptance §1-2](../../../../plans/m1_core_graph_reasoning/README.md)).

## The two encodings — classic vs unified is-a

The same puzzle is encoded **two ways** in `examples/`. The choice
between them is **deferred to P1.7 S1.7.2 T1.7.2.5** — both stay
valid through every M1 stage (memory: project — IR encoding choice
deferred).

### Classic (`zebra.ein`)

Uses the kernel `(type …)` and `(instance …)` declarations:

```lisp
(type Nationality Attribute)
(instance Norwegian Nationality)
(instance Japanese  Nationality)
```

`(type …)` / `(instance …)` are ordinary facts (S1.7.6); the kernel
builds **no** `Type` / `Instance` entity-view over them (S1.7.23), so
a type projection — if a puzzle wants one — is a user-space rule over
these facts. `zebra.ein` is a non-solving demonstrator; `zebra2.ein`
below is the canonical encoding.

### Unified is-a (`zebra2.ein`)

Uses only the relation `is-a` with two recurring rules
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
type/instance view (S1.7.23); anything that needs "the type-like
nodes" reads the `is-a` facts directly (e.g. the renderer's
`_schema_nodes`) or via a user-space rule.

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
  :match  (and (instance ?a ?T)
               (instance ?b ?T)
               :where (neq ?a ?b))
  :assert (not (co-located ?a ?b))
  :why    "{?a} and {?b} are distinct instances of {?T} — distinct slots."
  :priority 10)
```

`symmetric`, `transitive`, `implies` are **T2** rules
([`../01-ein-graph/02_rules.md` §2.2](../01-ein-graph/02_rules.md));
`type-exclusivity` is **T1**
([`../01-ein-graph/02_rules.md` §2.1](../01-ein-graph/02_rules.md))
— literal relation names (`instance`, `co-located`) appear in the
LHS and RHS.

## Reasoning-layer dump

After saturation, an engine dump of the derived facts looks like
(flat forms; each carries `:rule` / `:using`, so it re-classifies to
the REASONING layer on reload):

```lisp
;; The engine derived (co-located House-1 Norwegian) from
;; condition (10) via the symmetric rule.
(co-located House-1 Norwegian :rule symmetric
                              :using ((co-located Norwegian House-1)))

;; Type-exclusivity: Norwegian and Japanese are distinct
;; Nationality instances, so they're not co-located.
(not (co-located Norwegian Japanese) :rule type-exclusivity
                                     :using ((instance Norwegian Nationality)
                                             (instance Japanese  Nationality)))
```

> **Note** — the `:using` IR syntax above isn't yet round-trippable
> through the current grammar; the engine populates rule provenance
> programmatically. See
> [`01_grammar.md` §Reasoning](01_grammar.md) for the deferral.

## See also

- Full encoded puzzles:
  [`examples/zebra.ein`](../../../../examples/zebra.ein),
  [`examples/zebra2.ein`](../../../../examples/zebra2.ein).
- [`examples/README.md`](../../../../examples/README.md) — the
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
