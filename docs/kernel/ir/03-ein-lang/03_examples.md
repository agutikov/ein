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

The shortest meaningful IR — one fact whose `:source` annotation makes it a
**given** rather than background ([`01_kb.md` §3](../01-ein-graph/01_kb.md)).

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
M1 acceptance §1-2).

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

Cross-attribute clues are *ordinary facts* here
(`(co-located Englishman Red)`), and so are spatial ones
(`(right-of Green Ivory)` = "Green's house is immediately right of
Ivory's"); `zebra2.ein` has to restate each of those as a 4- or
5-argument activator fact, because its two attribute relations share no
argument to join on.

For a long time `zebra.ein` did not solve, and the reason was
ontological rather than linguistic. `bijective` and friends are
properties **of a relation**, declared per relation — and one universal
`co-located` is not the kind of relation any of them describe. It is an
*equivalence relation whose classes hold exactly one member of each
attribute type*. Restricted to one ordered pair of types it is a
bijection (which is why `zebra2.ein`, whose relations *are* those
restrictions, works), but `(bijective co-located)` has nowhere to put the
type pair, so the elimination rules had no structure to bite on and the
hypothesis space did not close.

What closed it (S1.22.1a) is a property scoped by the type **family**
rather than by the relation — [`std.slots`](../../../../stdlib/slots.ein),
covered under [§Type-scoped relation
properties](#type-scoped-relation-properties-slot-partition--slot-spatial)
below. No relation was added: the file still reasons over `co-located`,
`right-of`, `next-to`, `instance` and `type`.

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

## Type-scoped relation properties (`slot-partition` / `slot-spatial`)

A property vocabulary that does not fit "a property of one relation".
Where `(bijective color-loc)` equips *one* relation, these equip *one
relation over a family of types* — the shape a generic link relation
has. Both are ordinary facts, consumed by
[`std.slots`](../../../../stdlib/slots.ein):

```lisp
(slot-partition co-located instance type Attribute House)
```

> *`co-located` is an equivalence relation; its classes are **slots**;
> each slot holds exactly one `instance` of every type that is a direct
> `type`-child of `Attribute`; and the `House` member of a slot is its
> name.*

Five arguments, and each carries weight. The membership relation
(`instance`) and the subtype relation (`type`) arrive as *parameters*, so
the rule bodies name no `is-a` literal and the module works for either
membership convention (§Two ontologies above). `Attribute` names the
family, so adding an attribute category is adding a `(type T Attribute)`
fact — not a declaration per type *pair*, which is what scoping
`bijective` per pair would have cost (six categories → 30 declarations
for one relation). `House` is the **index**: the member that names a
slot, which is what lets the derived facts be anchored on the
value × index rectangle instead of enumerating the whole equivalence
closure.

```lisp
(slot-spatial co-located right-of instance House)
(slot-spatial co-located next-to  instance House)
```

> *`right-of` relates two values exactly when it relates their slots, and
> the slot type carrying the structure is `House`.*

This is what lets one relation name do double duty:
`(right-of Green Ivory)` is a **constraint** between two Colors, and
`(right-of House-2 House-1)` is the **structure** it resolves against.
The rules tell them apart by `instance`-membership of `House`, never by
name — so a puzzle states a spatial clue in the vocabulary it is written
in, rather than re-encoding it as an activator.

Both facts drive *reflective* rule activation
([`../01-ein-graph/02_rules.md`](../01-ein-graph/02_rules.md)): a
non-generic setup rule reads the property fact and derives the operational
activators, which light up the generic inference rules on the next
saturation pass. So at load time those rules have no applications at all
— see [`../../inference/README.md`](../../inference/README.md) for the
rule list and its priority bands.

## Worked rule library

Neither encoding defines property rules of its own any more; both import
them, which is what `:symbols` flat import is for
([`../../../../stdlib/README.md`](../../../../stdlib/README.md)):

```lisp
;; zebra.ein
(import std.algebra :symbols (symmetric symmetric-negative-setup includes))
(import std.slots   :symbols (slot-partition-setup slot-spatial-setup))
```

The imported closures are **T2** rules
([`../01-ein-graph/02_rules.md` §2.2](../01-ein-graph/02_rules.md)) —
parameterised over the relation, activated by a property fact:

```lisp
;; std.algebra
(rule symmetric (?rel)
  :match  (?rel ?a ?b)
  :assert (?rel ?b ?a)
  :why    "{?rel} is symmetric: {?a} ↔ {?b}."
  :priority 100)

;; std.slots — the "all-different within a category" constraint, with the
;; membership relation lifted to a parameter so the body stays is-a-free.
;; ?T is a FREE match var, not a parameter: the rule fires once per type
;; the membership relation mentions, so a puzzle adds a category by adding
;; its members.
(rule slot-exclusive (?R ?isa)
  :match  (and (?isa ?a ?T)
               (?isa ?b ?T)
               (neq ?a ?b))
  :assert (not (?R ?a ?b))
  :why    "{?a} and {?b} are distinct {?T}s — they cannot share a slot under {?R}."
  :priority 240)
```

`slot-exclusive` is the generic form of what `zebra.ein` used to spell
`type-exclusivity` with a hardcoded `instance` head and a `co-located`
literal in the conclusion. The **T1** shape (literal relation names in
the LHS and RHS,
[`../01-ein-graph/02_rules.md` §2.1](../01-ein-graph/02_rules.md)) is
still legal and still what a genuinely puzzle-specific rule looks like;
it is simply not what either Zebra encoding needs.

## Derived-fact dump

After saturation, an engine dump of the derived facts looks like
(flat forms; each carries `:rule` / `:using`, so it re-classifies to
derivations on reload):

```lisp
;; The engine derived (co-located House-1 Norwegian) from
;; condition (10) via the symmetric rule.
(co-located House-1 Norwegian :rule symmetric
                              :using ((co-located Norwegian House-1)))

;; slot-exclusive: Norwegian and Japanese are distinct Nationality
;; members, so they cannot share a slot.
(not (co-located Norwegian Japanese) :rule slot-exclusive
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
