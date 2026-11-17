# 3 — The rule families

The toy rules in Chapter 2 found one new fact each. The real Zebra puzzle
needs rules that *eliminate* possibilities fast enough to avoid blind
guessing. They split into two families — and the split is the most useful
lesson you'll take to your own puzzles:

| family | you… | examples |
|--------|------|----------|
| **stdlib property machinery** | *tag* a relation and *import* the logic | `(symmetric next-to)`, `(transitive is-a*)`, `(bijective color-loc)` |
| **puzzle-specific rules** | *write* the rules unique to this puzzle | `co-located`, `adjacent-via-*`, `disjunctive-prune-*` |

`zebra2.ein` opens with imports —

```lisp
(import std.algebra   :symbols (symmetric transitive includes))
(import std.bijection :symbols (bijective-setup typecheck-setup))
```

— and then *activates* them per relation:

```lisp
(symmetric  next-to)        ; next-to works both ways
(transitive is-a*)          ; is-a* is the transitive closure of is-a
(bijective  color-loc)      ; each colour ↔ exactly one house (and back)
(bijective  nation-loc) (bijective drink-loc) (bijective smoke-loc) (bijective pet-loc)
```

One tag pulls in a whole family of imported logic:

```dot
digraph { rankdir=LR; node [shape=box];
  tag [label="(bijective color-loc)", shape=note];
  tag -> "functional"; tag -> "injective";
  tag -> "total"; tag -> "surjective";
  tag -> "domain- and range-elimination"; tag -> "negative-completion";
}
```

You never wrote `symmetric`/`bijective`'s logic: the stdlib did
([the standard library](../04-reference/04-stdlib.md)). Each `(bijective R)` pulls
in four cardinality properties, `functional`, `injective`, `total` and
`surjective`, and the inference they license: **negative-completion** and
**domain- and range-elimination**.

## domain- and range-elimination — "must be, because nothing else can"

The workhorses. On a bijective relation, once every value but one is
excluded, the survivor is *forced*. There is one rule for each direction of
`(color-loc Color House)`:

- **`range-elimination`** fixes a house and forces its colour. **English:**
  "House-1 isn't green, ivory, blue or red, therefore it's yellow."
- **`domain-elimination`** fixes a colour and forces its house. **English:**
  "green isn't in houses 1 to 4, therefore it's in House-5."

Both ship in `std.bijection` (activated by `(bijective color-loc)`); you
don't write them. On `zebra2.ein`, before anything is guessed, the first one
places yellow ([the walkthrough](06-zebra-walkthrough.md), step 1):

```
before:  (not (color-loc Green  House-1))   (not (color-loc Blue  House-1))
         (not (color-loc Ivory  House-1))   (not (color-loc Red   House-1))
after:   + (color-loc Yellow House-1)        ← the only survivor, forced
```

```dot
digraph { rankdir=LR; node [shape=box];
  Green  -> "House-1" [label="✗", style=dotted];
  Blue   -> "House-1" [label="✗", style=dotted];
  Ivory  -> "House-1" [label="✗", style=dotted];
  Red    -> "House-1" [label="✗", style=dotted];
  Yellow -> "House-1" [label="color-loc (forced)", style=dashed];
}
```

`ein saturate examples/zebra2.ein --dump` lists the fact as
`(color-loc Yellow House-1 :rule std.bijection.range-elimination)`: a rule a
module brings is printed by its full name. Elimination needs negatives to
eat, which is where the next family comes in.

## negative-completion — turning one positive into many negatives

A bijective relation is exclusive both ways, so a single placement forbids a
lot. `(bijective R)` gives the *property* side; zebra2 adds
`co-located-negative` to carry those negatives across shared houses:

```lisp
(rule co-located-negative (?R1 ?V1 ?R2 ?V2)
  :match  (not (?R1 ?V1 ?h))
  :assert (not (?R2 ?V2 ?h))
  :why    "co-located: (not ({?R1} {?V1} {?h})) ⟹ (not ({?R2} {?V2} {?h})).")
```

```dot
digraph { rankdir=LR;
  subgraph cluster_m { label="match (a negative)";  "?V1" -> "?h" [label="not ?R1", style=dotted]; }
  subgraph cluster_a { label="assert (a negative)"; "?V2" -> "?h" [label="not ?R2", style=dotted]; }
}
```

**English:** "the Norwegian is in House-1, so the Englishman isn't; the
Englishman is the red house, so House-1 isn't red." Each `(not …)` it
produces is fuel for the eliminations above.

## co-located — propagate a partner through a shared house

The positive counterpart, and Chapter 2's `co-located`, now a rule of its own
in its real two-attribute form ([`zebra2.ein`](../../../../examples/zebra2.ein)):

```lisp
(rule co-located (?R1 ?V1 ?R2 ?V2)
  :match  (?R1 ?V1 ?h)
  :assert (?R2 ?V2 ?h)
  :why    "co-located: ({?R1} {?V1} {?h}) ⟹ ({?R2} {?V2} {?h}).")
```

```dot
digraph { rankdir=LR;
  subgraph cluster_m { label="match";  "?V1" -> "?h" [label="?R1"]; }
  subgraph cluster_a { label="assert"; "?V2" -> "?h" [label="?R2", style=dashed]; }
}
```

Driven by an activator like `(co-located nation-loc Englishman color-loc Red)`
("the Englishman lives in the red house"), it copies a known placement of
one attribute onto its partner: place the Englishman, and Red lands in the
same house.

## adjacent-via — "next to" / "right of"

Spatial clues run over an adjacency relation `?S` (`next-to` or `right-of`).
One rule covers both, with a twist — it only fires the positive when the
neighbour is **unique**:

```lisp
(rule adjacent-via-fwd (?S ?R1 ?V1 ?R2 ?V2)
  :match  (and (?R1 ?V1 ?h1)
               (?S ?h2 ?h1)
               (absent (and (?S ?h_o ?h1) (neq ?h_o ?h2))))
  :assert (?R2 ?V2 ?h2)
  :why    "({?R1} {?V1} {?h1}) and unique-{?S} neighbour {?h2} ⟹ ({?R2} {?V2} {?h2}).")
```

```dot
digraph { rankdir=LR;
  subgraph cluster_m { label="match";
    "?V1" -> "?h1" [label="?R1"];
    "?h2" -> "?h1" [label="?S (unique nbr)"]; }
  subgraph cluster_a { label="assert";
    "?V2" -> "?h2" [label="?R2", style=dashed]; }
}
```

`(absent …)` is **negation-as-failure**: the clause matches only when *no*
such fact is present. Here it means "fire only if `?h1` has a single
`?S`-neighbour" — true for `right-of` always, and for `next-to` only at the
row's ends. That's how the same rule handles a deterministic clue and a
two-way one safely.

## disjunctive-prune — exclude the impossible up front

`adjacent-via` only asserts a positive when it's certain; `disjunctive-prune`
fires in *every* case, pre-emptively excluding houses a spatial clue rules
out:

```lisp
(rule disjunctive-prune-fwd (?S ?R1 ?V1 ?R2 ?V2)
  :match  (and (?R1 ?V1 ?h1) (relation ?R2 ?A ?B)
               (is-a ?h_other ?B)
               (absent (?S ?h_other ?h1)))
  :assert (not (?R2 ?V2 ?h_other))
  :why    "({?V1} at {?h1} via {?R1}) restricts {?V2} to {?S}-neighbours (fwd); {?h_other} excluded.")
```

```dot
digraph { rankdir=LR;
  subgraph cluster_m { label="match";
    "?V1" -> "?h1" [label="?R1"];
    "?h_other" -> "?h1" [label="?S absent", style=dotted]; }
  subgraph cluster_a { label="assert";
    "?V2" -> "?h_other" [label="not ?R2", style=dotted]; }
}
```

**English:** "if V2 must be a `?S`-neighbour of V1, then every house that
*isn't* a neighbour is excluded for V2." Those negatives feed
negative-completion and the eliminations: the families chain.

**Including V1's own house.** The rule has no `(neq ?h_other ?h1)` premise,
and must not have one. A value does exclude its own house here: `next-to` and
`right-of` are irreflexive, so `?h1` is not its own `?S`-neighbour, and
`(absent (?S ?h_other ?h1))` already answers that case. A guard exempting
`?h1` would let V1 and V2 sit in the *same* house and still satisfy the clue.
The canonical puzzle would not show it, since other clues pin those cells,
but the under-determined `zebra2-minus-15.ein` would count models the puzzle
does not have. The lesson generalises past this rule: a hand-written guard
that duplicates what an `absent` premise decides can disagree with it.

## The takeaway

Solving a new puzzle with Ein is mostly **declaring properties** — tag each
relation `bijective` / `symmetric` / `transitive`, import the stdlib — and
writing only the handful of rules that are genuinely specific to your
puzzle's structure (here, the spatial `adjacent-via` / `disjunctive-prune`
family). The generic logic is library code you reuse.

In [Chapter 4](04-solving-the-whole-puzzle.md) we assemble the whole thing
and watch these families solve it.

> **Reference:** the standard library, module by module:
> [stdlib](../04-reference/04-stdlib.md); the properties as concepts:
> [properties and operators](../03-syntax-and-semantics/07-properties-and-operators.md);
> each family firing in context: [the walkthrough](06-zebra-walkthrough.md).
