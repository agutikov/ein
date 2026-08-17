# C2 — `zebra.ein`: the ontology gap, and closing it

**Stage:** [S1.22.1a](../s1.22.1a_zebra_ein_modernisation.md), tasks
T1.22.1a.1 (investigate) and T1.22.1a.2 (execute). **Date:** 2026-08-17.
**Tree:** post-S1.22.1b (`0227e44`). **Interpreter for every timing:** the
project PyPy venv (`./ein_pypy.sh`), single core.

The user's brief set the order of work: *hypothesis filtering + surface syntax
+ NL support first; then dump the reasoning and analyse what this ontology can
and cannot conclude; then add rules on top of the **existing** relations,
introducing new relations only as a last resort.* This report follows that
order, and records the answer to the last part up front: **no new relation was
needed.** `co-located`, `right-of`, `next-to`, `instance` and `type` — the five
relations the file already had — are sufficient. What was missing was a
*property* of `co-located` that the language could not previously state.

## 0. Headline

| | before | after | zebra2 (reference) |
|---|---|---|---|
| verdict | *no solve* | **Solution, k=1, exhausted** | Solution, k=1, exhausted |
| root candidates | 2255 | 56 | 56 |
| root saturation | 145 derived facts | 431 derived / 497 total | 369 total |
| `solve` (stop-after-1) | — | 2.8 s | 1.6 s |
| `solve --exhaustive` | — | **21.4 s** | 9.3 s |
| enterings (exhaustive) | — | 111 / 2 layers | 101 / 2 layers |
| rules defined in the file | 8 | **0** (all imported) | 12 |

Scope-4's bar was "same verdict within the same order of magnitude, under
~30 s". 21.4 s against zebra2's 9.3 s, with a search of 111 enterings against
101 — the two encodings now explore *the same shape of lattice*, and the
residual 2.3× is per-node saturation cost, not extra search.

## 1. Load-and-saturate baseline (T1.22.1a.1 §1)

`zebra.ein` loaded and saturated fine before any change. It derived 145 facts
in 164 firings — and the firing breakdown is the whole diagnosis:

```
type-exclusivity   120 / 120     ← the only rule doing real work
symmetric           19 /  38
implies              5 /   5
square-unique        1 /   1
transitive           0 /   0     ← never fired
square-fwd           0 /   0     ← never fired
square-bwd           0 /   0     ← never fired
hypothesis-contradiction  0/0    ← dormant by design
```

Of the eight rules, **three never fired at all** and one (`type-exclusivity`)
produced 120 of the 145 derived facts — every one of them a `(not …)`. The
single positive conclusion the whole file reached was
`(co-located Blue House-2)`, from `square-unique`. That is one step of the
human walkthrough, and then nothing.

Why `transitive` never fired: the ten authored cross-attribute clues share no
endpoint, so `(co-located A B)` ∧ `(co-located B C)` has no instance beyond the
symmetric mirror, which `(neq ?a ?c)` correctly blocks. Why `square-fwd` /
`square-bwd` never fired: both need a `co-located` fact bridging the *spatial*
leg to the *attribute* leg, and no attribute was ever placed at a house, so the
bridge never existed. The rules were not wrong — they were downstream of an
inference that never happened.

**Quantifying "too many hypotheses".** With no `(hrule …)`, hypgen falls back
to the blind combinatorial enumerator, which fills *every slot of every open
relation, type-blind* (`hypgen.py:_raw_candidates`, S1.7.23 — deliberately no
kernel `is-a` walk):

```
root hyps  2255 candidates across 2 relations
  next-to      1301  (57.7%)
  co-located    954  (42.3%)
raw 5328 → emitted 2255
```

Against zebra2's 56. Note *what* is being guessed: 1301 of the candidates are
`next-to` facts — the engine speculating about the spatial structure itself.
And `right-of` contributes zero, because `closed.emit_closed` correctly infers
`(__closed__ right-of)` (no rule asserts it). So the file was not merely
over-generating; it was generating in the wrong space.

## 2. Property-gap census (T1.22.1a.1 §2)

For every zebra2 elimination/bijection rule, why it cannot fire here:

| zebra2 rule | fires in zebra.ein? | precise reason |
|---|---|---|
| `functional` / `injective` (checks) | no | needs `(functional co-located)`. **Would be false** — `co-located` is symmetric and each value has five partners |
| `total` / `surjective` | no | same, plus they read `(relation R A B)` = `Attribute × Attribute`, so the `forall` quantifies over all 30 values instead of one type |
| `functional-negative` / `injective-negative` | no | as above. **Would be unsound** if forced: from `(co-located Englishman Red)` they would derive `¬(co-located Englishman House-1)` — Red and House-1 are both `Attribute` |
| `domain-elimination` / `range-elimination` | no | guarded on `(functional R)` ∧ `(total R)`, which do not hold |
| `typecheck-arg-0/1` | no | `(bijective R)`-gated via `typecheck-setup` |
| `co-located` (the 4-ary propagation rule) | n/a | zebra2 needs it because a clue's two attributes have no shared argument to join on. Here they do — the clue *is* a `co-located` fact |
| `adjacent-via-*` (8 rules + 3 fan-outs) | n/a | same: zebra2 restates each spatial clue as a 5-argument activator; here `(right-of Green Ivory)` is the clue |

The pattern is uniform, and it is exactly the user's diagnosis. Every one of
these properties is a property **of a relation**, and `co-located` is not the
kind of relation any of them describe. It is an *equivalence relation whose
classes hold exactly one member of each attribute type*. Restricted to one
ordered pair of types it is a bijection — which is why zebra2, whose relations
*are* those restrictions, works — but `(bijective co-located)` cannot say
"between `Nationality` and `House`". `bijective` has no place to put the type
pair.

## 3. Option weighing (T1.22.1a.1 §3)

The stage doc offered three. What shipped is a fourth, and the comparison is
the point:

**(a) Type-scoped relation properties** — `(bijective co-located Nationality
House)`. Faithful to the diagnosis, but the arity is per *ordered type pair*:
six attribute types give 30 declarations for one relation, and every
`std.bijection` rule has to thread two extra type parameters through its
premises and its `forall`. Rejected on the declaration count — 30 facts saying
one thing is not a property, it is a table.

**(b) Derived typed projections** — rules deriving `(nation-loc ?n ?h)` from
`(co-located ?n ?h)` ∧ `(instance ?n Nationality)` ∧ `(instance ?h House)`,
then running the zebra2 machinery on the projections. Cheapest to write, and
rejected on the grounds the stage doc itself gives: the reasoning would happen
on derived typed relations, so it answers "can the zebra2 machinery be reached
from this surface" rather than "can the inference be expressed over the generic
relation". It also needs five new relations, which the user's brief puts last.

**(c) Reflective property rules** deriving the type-scoped properties from
`is-a` structure. This is half of what shipped — the activation story does work
with a 5-ary property fact (see §4) — but on its own it still lands in (a)'s
30-declaration space; it just hides the table behind a rule.

**(d) — shipped: scope the property by the type FAMILY, not by the type pair.**

```
(slot-partition co-located instance type Attribute House)
```

*`co-located` is an equivalence relation; its classes are slots; each slot holds
exactly one `instance` of every type that is a direct `type`-child of
`Attribute`; and the `House` member of a slot is its name.* One fact, five
arguments, no per-pair table — and it is a statement about the relation's
*shape*, in the same register as `(bijective color-loc)`. It ships as
`std.slots`, the type-scoped counterpart of `std.bijection`, with a second
entry point for spatial relations:

```
(slot-spatial co-located right-of instance House)
(slot-spatial co-located next-to  instance House)
```

*`right-of` relates two values exactly when it relates their slots, and the
slot type carrying the structure is `House`.* This is what lets one relation
name do double duty: `(right-of Green Ivory)` is a constraint between two
Colors, `(right-of House-2 House-1)` is the structure it resolves against, and
the rules tell them apart by `instance`-membership of `House`, never by name.

**Ripple.** One new file, `ein.py/src/ein/stdlib/slots.ein`; no change to any
existing stdlib module, no change to the kernel, no property arity changed, so
no existing puzzle is affected. `zebra2.ein` is untouched and its verdict,
bindings and runtime are unchanged.

## 4. What this ontology concludes, and what it cannot (T1.22.1a.1 §2 + the user's ask 2)

The user asked for the reasoning to be dumped and read before any rules were
added on top. `ein saturate examples/zebra.ein --dump` at d=0, i.e. with **no
hypothesis at all**, now reaches:

| derived positive | rule | walkthrough step |
|---|---|---|
| `(co-located Blue House-2)` | `slot-adjacent-fwd` | (15) + House-1 is a corner ⟹ Blue is next door |
| `(co-located House-1 Yellow)` | `slot-fill` | H1's colour seat: Red✗ Blue✗ Green✗ Ivory✗ ⟹ Yellow |
| `(co-located Kools House-1)` | `slot-locate` | (8) Kools↔Yellow, Yellow@H1 ⟹ Kools@H1 |
| `(co-located Horse House-2)` | `slot-adjacent-fwd` | (12) + Kools@H1 is a corner ⟹ Horse next door |
| `(co-located House-1 Water)` | `slot-fill` | H1's drink seat: Coffee✗ Tea✗ Milk✗ Juice✗ ⟹ Water |

431 derived facts in 880 firings (410 *productive firings* — the setup
rules' multi-fact `:assert (and …)` counts once each). The last line is worth
dwelling on: **one of
the puzzle's two questions is answered before the search starts.** The chain
that gets there is the human one, and every link in it is now a rule firing
with a `:why`:

```
(10) Norwegian@H1
  → slot-occupied   ¬(H1 ↔ Englishman)          H1's nationality seat is taken
  → slot-negative   ¬(Red ↔ H1)                 via (2) Englishman↔Red
  → slot-endpoint-bwd ¬(Green ↔ H1)             (6): Green needs a house to its left
  → slot-adjacent-fwd  Blue@H2                  (15) + H1 has one neighbour
  → slot-occupied   ¬(H1 ↔ Blue)                Blue's slot is taken
  → slot-adjacent-fwd-neg ¬(Ivory ↔ H1)         ¬Green@H2 + H2's only left-neighbour is H1
  → slot-fill       Yellow@H1                   four of five colours excluded
```

**What is not feasible at d=0**, and correctly so: everything downstream of a
genuine disjunction. Conditions (11) and (12) place Chesterfields/Fox and
Kools/Horse *next to* each other, and an interior house has two neighbours, so
`slot-adjacent-fwd`'s uniqueness guard fails and the conclusion stays
disjunctive. That is what the hypothesis search is for, and it is where the 111
enterings go. The three rules that fire zero *productive* times at the root —
`slot-elimination`, `slot-prune-fwd`, `slot-prune-bwd` — are not dead: they are
the rules that need a placement to bite on, and they carry the branches.

Two things this ontology genuinely cannot do, recorded rather than fixed:

1. **Cross-attribute elimination.** "Every Colour but Red is excluded for the
   Englishman ⟹ Englishman↔Red" is expressible but never fires, because the
   index anchoring (§5) keeps derived negatives on the Attribute×House
   rectangle. It costs nothing here — the puzzle is determined by the 25
   placements — but a puzzle whose clues are *only* cross-attribute would need
   the unanchored form and would pay §5's price for it.
2. **Naming which slot-mate answers a question.** `(co-located Water House-1)`
   and `(co-located Water Norwegian)` are the same kind of fact, so the query
   has to say which one it wants: the `:goal` carries
   `(instance ?who_water Nationality)` conjuncts that zebra2 gets for free from
   `nation-loc`'s signature. This is the honest cost of one generic relation,
   and it is confined to the query.

## 5. The four things that made it fast

Getting a verdict was the easy half; §0's 21.4 s took four separate findings,
each measured. They are recorded in order of size because the *first* one is
not about this puzzle at all.

**(i) The no-good explanation search, not saturation, was the bottleneck —
50 % of a whole solve.** cProfile on a stop-after-1 run:

```
   ncalls  tottime  cumtime  function
        3    0.000   68.128  frontier.py:66  smallest_contradiction_frontier
        3    0.072   68.102  explain.py:280  _propagate
    84226   8.676   62.941  explain.py:231  _minimise
    30020   0.013   66.203  saturator.py:354 saturate   ← called FROM _minimise
```

Three dead branches, **22.7 s each**, against ~2 s for a branch's own
saturation. This is S1.21.7's multi-justification provenance: on by default, a
dead branch's no-good is explained by searching the AND/OR proof graph for a
minimal frontier. The search is affordable when facts have one or two
derivations. Here they have many — one generic `co-located` edge is reachable
by `symmetric`, `slot-locate`, `slot-occupied`, `slot-fill` and both spatial
rules — so the graph is densely multi-justified and minimisation explodes.
`(config :record-alternative-justifications false)` took stop-after-1 from
30.8 s to 4.4 s. **This is a property of the ontology, not of the engine:**
five typed relations give each fact essentially one derivation, which is why
zebra2 can afford the default and does not think about this knob. It is the
clearest instance in the whole exercise of a cost that is invisible until you
write the second encoding.

**(ii) Index anchoring — don't enumerate the equivalence closure.**
`(transitive co-located)` materialises every pair inside a class: 6 members × 5
ordered partners × 5 slots = 150 positive edges where zebra2 holds the same
information in 25, and the negative side is the complement of a 30-element
square — 900 ordered pairs against zebra2's 125. Because the negative rules
join positives *against* negatives, the cost is the product; measured at ~4.3 s
per hypothesis, and a 239 s exhaustive run. `slot-locate` is transitivity with
its conclusion pinned to the slot's index, and `slot-negative` likewise, which
puts every derived fact on the same Attribute×House rectangle zebra2's typed
relations occupy. Transitivity still *holds* — it is what justifies both rules
— it is simply not enumerated off the rectangle.

The one thing this cost: with the full closure, the first candidate tried
happened to complete the entire grid, so stop-after-1 finished in 1 entering
(1.15 s). Anchored, it takes 13. The exhaustive numbers are what matter, and
there anchoring wins by an order of magnitude.

**(iii) Splitting the elimination rule.** A single `slot-elimination`
quantifying over all ordered pairs of family types evaluates 900 `forall`
guards per pass. Only pairs involving the index can ever succeed (that is what
(ii) guarantees), so the rule splits into `slot-elimination` (place a value
among slots — zebra2's `domain-elimination`) and `slot-fill` (fill a slot's
type-seat — `range-elimination`), each pinned to the index: 275 guards, same
431 conclusions. Exhaustive 37.1 s → 21.4 s.

Both directions are needed, and they are *not* each other's mirror: R's
symmetry makes the two conclusions interchangeable, but the rules quantify over
different domains and therefore fire at different times. The Zebra opening
needs `slot-fill` ("H1's colour seat has only Yellow left"); the endgame needs
`slot-elimination` ("Zebra has only House-5 left").

**(iv) The negative symmetric mirror is load-bearing, not decoration.**
`(import std.algebra :symbols (symmetric-negative-setup))`. A symmetric
relation has a symmetric complement, and this encoding derives most negatives
in one argument order only; without the mirror,
`(not (co-located House-2 Green))` never reaches a rule matching
`(not (co-located Green House-2))`, and the *Blue@H2 ⟹ ¬Green@H2 ⟹ ¬Ivory@H1 ⟹
Yellow@H1* chain stalls at its first step — root derivations went from 145 to
239 with `std.slots` alone, and to 431 once the mirror was added. zebra2 needs
no mirror because its `*-loc` relations are directed Attribute→House, so there
is only one order to begin with.

## 6. Surface syntax and NL-readiness (T1.22.1a.1 §4, scope 1–2)

**Hypothesis filtering.** The `(hrule guess (?R ?T1 ?T2) …)` is byte-for-byte
zebra2's — the guessed relation rides in as a parameter, so the body names only
kernel primitives and variables. The *only* difference between the two
encodings is the activator list, and that difference is the ontology in one
line:

```
zebra2   :hrules (guess (color-loc Color House) (nation-loc Nationality House) …)
zebra    :hrules (guess (co-located Color House) (co-located Nationality House) …)
```

Five different typed relations, versus the same generic one five times. Both
give 125 raw candidates → 56 emitted. The `(config …)` block carries five
knobs, four of them at their defaults and written out because this is the
encoding that stresses them, plus §5(i)'s.

**Rules: 8 → 0.** The file now defines no rules of its own. `symmetric` /
`transitive` / `implies` were hand-rolled copies of std.algebra rules
(`implies` is `includes` under its stdlib name). `type-exclusivity` is
std.slots' `slot-exclusive`, with the membership relation lifted to a parameter
so the body is `instance`-free. `square-fwd` / `square-bwd` / `square-unique`
are all three the special case of `slot-adjacent-fwd` / `slot-adjacent-bwd`
whose uniqueness guard is vacuous — and the guard says better what the
square rules' comments had to say in prose ("square applies ONLY to
directional relations"): it *admits* the corner cases of a symmetric relation,
which `square-fwd` had to refuse wholesale and `square-unique` re-implemented
with a hand-written NAF clause. One mechanism replaced three.
`hypothesis-contradiction` was dormant — nothing in the engine emits
`(hypothesis ?h)` / `(contradiction-under ?h)`; the rule shape stays exercised
by `examples/saturation/hypothesis-contradiction/`.

**NL support.** Every rule carries a `:why` (they are now the stdlib's, all
written as sentences about slots rather than about relation algebra); every
given clue keeps its `:source "condition (n)"`; the four relations that read as
sentences carry positional `:why` render templates; the query carries
`:goal-text`. `ein solve examples/zebra.ein` prints

```
    query facts                       rendered
    (co-located Water House-1)        Water and House-1 are in the same house
    (co-located Norwegian House-1)    Norwegian and House-1 are in the same house
    …
  result
    The Norwegian drinks water in House-1; the Japanese owns zebra in House-5
```

The gap that remains, and it is inherent: `co-located` is symmetric and
untyped, so its render template cannot say "*Water is drunk in* House-1" the
way `drink-loc`'s can — it does not know which argument is the house. For the
M2 round-trip this is the interesting half of the comparison, not a defect to
paper over: the typed encoding carries more NL structure in its signatures, and
the generic one carries more of it in its rules' `:why` strings.

## 7. Acceptance status

1. ✅ modern syntax, `:why`/`:source`/`:goal-text`-annotated, **solves** to the
   same model as `zebra2.ein`, 21.4 s exhaustive against a ~30 s bar.
2. ✅ inference runs over the **original** relations — `co-located`,
   `right-of`, `next-to`, `instance`, `type`. No relation was added, and no
   projection was derived. What was added is a *property*.
3. ✅ both encodings in the acceptance suite
   (`acceptance/test_zebra_two_ontologies.py`, which also pins that the two
   models *agree* cell by cell); `examples/README.md` reframed. Gating is at
   full parity with zebra2's, tier for tier — Phase 2 exhaustive solve, unit
   root-saturation, walkthrough-rules-defined, walkthrough-rules-fire — see
   the stage doc's Outcome table. Writing the firing tier surfaced one thing
   worth recording: **every** inference rule `std.slots` provides fires on
   this encoding's solution path, the two ⊥-rules aside, whereas zebra2's
   firing target is a strict subset of its own library.
4. ✅ `std.slots` documented in `docs/kernel/ir/03-ein-lang/` (the property
   vocabulary) and `docs/kernel/inference/` (the rules and their bands),
   alongside `std.bijection`.

## 8. Handed onward

- **`(config :record-alternative-justifications false)` is a workaround, not a
  fix.** §5(i) is an engine cost that scales with proof-graph density, and a
  densely-justified graph is the normal case for any equivalence-relation
  encoding. `explain._minimise` re-saturates per candidate frontier (30 020
  `saturate()` calls for three branch deaths); a memoised or
  incrementally-maintained frontier would let this encoding keep minimal
  explanations. Worth a P1.23 stage.
- **The residual 2.3× per-node cost.** 111 enterings against zebra2's 101 says
  the search shape is right; 190 ms per entering against ~90 ms says each
  saturation still does about twice the work. The candidate is
  `slot-exclusive`: it materialises 120 same-type negatives extensively (no
  positive witness), and for an index-typed `hrule` — which can never propose a
  same-type pair — they are inert. Making it opt-in, the way std.algebra splits
  intrinsic from extensive operators, is the obvious next measurement.
- **The `is-a` question left open.** `zebra.ein` keeps `instance` / `type`
  split where zebra2 unifies them as `is-a`. That is a real ontological
  difference and the file keeps it deliberately (`std.slots` takes both as
  parameters, so the module is neutral). Whether the split pays for itself is a
  separate comparison from this one.
