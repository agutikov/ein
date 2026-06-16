# Self-describing KB types (design sketch)

> **Design-only for M1.** This file *sketches* a way to express the KB's
> own type schema — the four levels of [`05_four_level_kb.md`](05_four_level_kb.md)
> — *in ein-lang itself*, so the kernel becomes introspectable in its own
> surface language. M1 ships this as a **doc**; the *operational* version
> (a `(meta …)` block the engine respects at load + run time) is
> [F5](../../../../plans/followups/f5_rules_as_data.md)'s work. Nothing
> here changes the M1 engine.

## 1. The `(meta …)` sketch

A block that declares the L0–L3 levels using ein-lang, so the same
language that authors puzzles can describe the kind-of-thing each puzzle
node is:

```lisp
(meta
  ;; L0 — objects (the kernel's "named atom" notion)
  (level L0
    :name objects
    :nodes atoms          ;; an L0 element is an atom
    :inhabits L1)         ;; L0 elements inhabit L1 facts

  ;; L1 — facts (propositions over L0 objects)
  (level L1
    :name facts
    :shape (?head ?arg ...)
    :slots-from L0
    :typed-by L2)

  ;; L2 — relations + properties
  (level L2
    :name relations
    :declares (relation ?name ?type-args ...)
    :properties (symmetric transitive functional total closed
                  bijective converse ...)
    :acts-on L1)

  ;; L3 — rules
  (level L3
    :name rules
    :shape (rule ?name (?params ...) :match (...) :assert (...))
    :patterns-on (L1 L2)
    :asserts L1))
```

This is a **schema for the kernel**: each `(level …)` says what its
elements *are*, what shape they take, and which levels they relate to.

## 2. Mapping to the current kernel

Each `(level …)` declaration has a concrete inhabitant in zebra2:

| level | `(meta …)` clause     | zebra2 inhabitant |
|-------|-----------------------|-------------------|
| L0    | `:nodes atoms`        | `Red`, `House-1`, `Englishman` |
| L1    | `:shape (?head ?arg …)` | `(color-loc Red House-1)` |
| L2    | `:declares (relation …)` + `:properties (…)` | `(relation color-loc Color House)`, `(bijective color-loc)` |
| L3    | `:shape (rule …)`     | `(rule symmetric (?R) :match (?R ?a ?b) :assert (?R ?b ?a))` |

The schema is **descriptive of what already exists** — the loader and
matcher already enforce these shapes; the `(meta …)` block would just
*name* the enforcement in one place instead of scattering it across
`kb.from_ir` checks.

## 3. What the self-describing form unlocks

- **Validation.** Load a puzzle, check it against the meta schema in one
  pass. Today this is scattered loader checks
  ([`../03-ein-lang/06_reserved_names.md`](../03-ein-lang/06_reserved_names.md)
  lists the shape-pinned declarators); the meta schema centralises them.
- **Introspection.** A rule can pattern-match on *what kinds of things
  exist* — the rules-on-rules case of
  [`05_four_level_kb.md` §4](05_four_level_kb.md) (F5).
- **Code-generation.** An author could write a rule that *generates* new
  rules conforming to the meta schema — rule induction
  ([F7](../../../../plans/followups/f7_rule_induction.md)) with a schema
  to generate against.

## 4. The M1-vs-F5 split

| concern | M1 | F5 |
|---------|----|----|
| the schema | shipped as **this doc** | shipped as an operational `(meta …)` module |
| who enforces it | the loader / matcher, hard-coded | the engine, *reading the meta block* at load + run time |
| can rules read it | no | yes (rules-on-rules; introspection) |

M1's job is only to **write the schema down** so F5 has a target; the
engine does not consult a `(meta …)` block in M1.

## Open questions

- **`(meta …)` as a new top-level form?** Or a stdlib module, or part of
  an ontology grouping? Decide when F5 makes it operational; the
  flat-form grammar (P1.7c) would admit `(meta …)` as one more declarator
  in the closed set.
- **Cyclicity.** The meta block declares L3 rules, but L3 rules also
  describe the meta block's own rules — reflexive, exactly the
  *instance-of-instance* fixed point of [`03_ein_model.md` §1](03_ein_model.md).
- **Why ein and not a metalanguage (BNF / Z / Python types)?** Because
  ein is meant to be expressive enough to describe its own kernel — that
  is the F5 thesis, and the
  [F1b logical formulation](../../../../plans/followups/f1b_logical_formulation.md)
  is its formal sibling.

## See also

- [`05_four_level_kb.md`](05_four_level_kb.md) — the L0–L3 schema this
  expresses in ein-lang.
- [`03_ein_model.md`](03_ein_model.md) — the reflexive root that makes a
  self-describing schema coherent.
- [F5 — rules as data](../../../../plans/followups/f5_rules_as_data.md)
  — the implementation half of this design.
- [F1b — logical formulation](../../../../plans/followups/f1b_logical_formulation.md)
  — the formal-fragment sibling.
