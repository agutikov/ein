# The property audit — what each stdlib rule enforces today

**Stage:** [S1d.2.1](s1d.2.1_property_audit.md) · **Phase:**
[P1d.2](README.md) · **Taken:** 2026-08-25, against `stdlib/` at `29543e2`
(73 rules) and the corpus at 180 entries.
**Instruments:** the rule text itself (parsed, not read — every row's band,
`:match` and `:assert` come from the file), `utils/stdlib_census.py` twice
(scoped to `tests/`, and all 180 entries), and four hand-built fixtures for
the findings, each quoted with what the engine actually printed.

The stage's question, under the decided form (**G** — additive, the scans
stay), is not "can the six scans collapse into one mechanism" but **what must
the two new duals not disturb**. The answer is in §4; §1 is what the audit
found on the way there and is the more interesting half.

---

## 1. The headline — the `≥` half has fifteen rules and no middle

Every one of the 73 rules classified by which bound it serves:

| | rules | what they are |
|---|---:|---|
| **`≤`** — forbid a second arrow | **20** | 12 fire `(false)`, 8 store a `(not …)` that a later positive conflicts into one |
| **`≥`** — require a first arrow | **15** | see below |
| neither | **38** | derivations, definitions, activator fan-outs, type checks |

And the fifteen, split by *how many candidates they are about*:

| candidates | rules | how |
|---|---:|---|
| **0** — the requirement is unreachable | **5** | `total`, `surjective` (`std.algebra`), `no-room-left` (`std.elim`), `slot-no-room`, `slot-no-fill` (`std.slots`) — a `forall`-scan over stored negatives ⇒ `(false)` |
| **1** — the witness is forced | **5** | `domain-elimination` ×2, `range-elimination`, `slot-elimination`, `slot-fill` — a `forall`-scan excluding all alternatives ⇒ assert the survivor |
| **a unique witness, known statically** | **4** | `identity`, `top` (`std.algebra`), `reflexive-dom`, `reflexive-cod` (`std.typing`) — they *generate* the arrow, because there is no choice to make |
| **0, but read open-world-naively** | **1** | `connex` — see [F2](#f2--connex-is-a-lower-bound-in-the-form-total-was-written-to-avoid) |
| **≥ 2** | **0** | — |

**That last row is the phase, measured.** The note's premise was that the
lower bounds are missing; the truer statement is that they are all *there*
and every one of them is about a candidate set of size 0 or 1. Where a
requirement has two or more candidates left, no rule in the stdlib records
anything — not because an author forgot, but because the language has no way
to say it, which is what [S1d.2.3](s1d.2.3_the_form.md)'s `(open ?R)` is for.

The four static-witness rules are worth their own sentence, because they are
[`ideas.md`](../ideas.md)'s named anti-pattern alive in the tree: *"existence
requirements should be first-class obligations, not generators of arrows."*
`identity`, `top` and the two `reflexive` halves are lower bounds implemented
exactly as generators — and they are sound, because their witness is unique
by construction (`a ↦ a`, or every pair). That is the dividing line the
phase turns on: **a lower bound whose witness is determined may be a
generator; a lower bound with a choice in it may not.**

## 2. The table

`ᵖ` marks `std.elim`'s positional spelling, `ᵈ`/`ʳ` the domain and range
directions of the slots family. "corpus entries firing" is over all 180
entries; the last column is the `tests/stdlib/` programs that activate the
rule, which is the claim `ein-infer/tests/stdlib_coverage.rs` gates.

### `std.algebra`

| rule | property | half | form | band | corpus entries firing | `tests/stdlib/` activator |
|---|---|---|---|---:|---:|---|
| `converse` | converse | — | derives | 100 | 3 | `algebra/01_copiers`, `algebra/05_tag_lemmas`, `typing/03_type_hierarchy_welltyped` |
| `imply1` | imply | — | derives | 100 | 1 | `algebra/01_copiers` |
| `imply2-fwd` | imply | — | derives | 100 | 1 | `algebra/01_copiers` |
| `imply2-reverse` | imply | — | derives | 100 | 1 | `algebra/01_copiers` |
| `symmetric-is-self-converse` | symmetric | — | fan-out | 90 | 1 | `algebra/05_tag_lemmas` |
| `self-converse-is-symmetric` | symmetric | — | fan-out | 90 | 1 | `algebra/05_tag_lemmas` |
| `converse-pair-symmetric` | converse | — | fan-out | 90 | 1 | `algebra/05_tag_lemmas` |
| `converse-illtyped-dom` | *typing* | — | `(false)` | 110 | 2 | `algebra/16_converse_illtyped_dom`, `typing/02_type_hierarchy_converse` |
| `converse-illtyped-ran` | *typing* | — | `(false)` | 110 | 1 | `algebra/17_converse_illtyped_ran` |
| `compose` | compose | — | derives | 100 | 3 | `algebra/02_compose`, `algebra/06_equational`, `algebra/21_transitive` |
| `identity` | identity | **≥** | **forces** | 100 | 1 | `algebra/04_extensive` |
| `meet` | meet | — | derives | 100 | 1 | `algebra/03_boolean` |
| `difference` | difference | — | derives (NAF) | 100 | 1 | `algebra/03_boolean` |
| `derive-join` | join | — | fan-out | 120 | 2 | `algebra/03_boolean`, `algebra/06_equational` |
| `join-l` | join | — | derives | 100 | 2 | `algebra/03_boolean`, `algebra/06_equational` |
| `join-r` | join | — | derives | 100 | 2 | `algebra/03_boolean`, `algebra/06_equational` |
| `empty` | empty | ≤ | `(false)` | 110 | 1 | `algebra/13_empty_violated` |
| `top` | top | **≥** | **forces** | 100 | 1 | `algebra/04_extensive` |
| `complement` | complement | — | derives (NAF) | 100 | 1 | `algebra/04_extensive` |
| `functional` | functional | ≤ | `(false)` | 250 | 2 | `algebra/14_functional_violated` |
| `injective` | injective | ≤ | `(false)` | 250 | 2 | `algebra/15_injective_violated` |
| `bijective-properties` | bijective | — | fan-out | 100 | 6 | `bijection/01_setup_and_negatives`, `bijection/02_domain_elimination`, `bijection/03_range_elimination` |
| `total` | total | **≥** | `(false)` — **0-end** | 110 | 4 | `algebra/19_total_violated` |
| `surjective` | surjective | **≥** | `(false)` — **0-end** | 110 | 3 | `algebra/20_surjective_violated` |
| `irreflexive` | irreflexive | ≤ | `(false)` | 110 | 1 | `algebra/09_irreflexive_violated` |
| `antisymmetric` | antisymmetric | ≤ | `(false)` | 110 | 1 | `algebra/10_antisymmetric_violated` |
| `asymmetric` | asymmetric | ≤ | `(false)` | 110 | 1 | `algebra/11_asymmetric_violated` |
| `connex` | connex | **≥** | `(false)` — **naive** | 110 | 1 | `algebra/12_connex_violated` |
| `difunctional` | difunctional | — | derives | 90 | 1 | `algebra/02_compose` |
| `symmetric` | symmetric | — | derives | 100 | 16 | `algebra/05_tag_lemmas`, `algebra/22_symmetric_negative`, `slots/01_partition`, `slots/02_negative`, `slots/03_fill`, `slots/04_elimination`, `slots/07_spatial_prune`, `slots/08_spatial_adjacent` |
| `symmetric-negative-setup` | symmetric | — | fan-out | 100 | 10 | `algebra/22_symmetric_negative`, `slots/01_partition`, `slots/02_negative`, `slots/03_fill`, `slots/04_elimination`, `slots/05_no_room`, `slots/06_no_fill`, `slots/07_spatial_prune`, `slots/08_spatial_adjacent` |
| `symmetric-negative` | symmetric | ≤ | stored `(not …)` | 100 | 10 | `algebra/22_symmetric_negative`, `slots/01_partition`, `slots/02_negative`, `slots/03_fill`, `slots/04_elimination`, `slots/05_no_room`, `slots/06_no_fill`, `slots/07_spatial_prune`, `slots/08_spatial_adjacent` |
| `transitive` | transitive | — | derives | 200 | 7 | `algebra/21_transitive` |
| `includes` | includes | — | derives | 100 | 8 | `algebra/01_copiers` |
| `compose-negative-s` | compose | ≤ | stored `(not …)` | 240 | 1 | `algebra/07_schroder` |
| `compose-negative-r` | compose | ≤ | stored `(not …)` | 240 | 1 | `algebra/07_schroder` |
| `compose-contravariant` | compose | — | fan-out | 90 | 1 | `algebra/06_equational` |
| `join-converse` | join | — | fan-out | 90 | 1 | `algebra/06_equational` |

### `std.bijection`

| rule | property | half | form | band | corpus entries firing | `tests/stdlib/` activator |
|---|---|---|---|---:|---:|---|
| `bijective-setup` | bijective | — | fan-out ×6 | 100 | 6 | `bijection/01_setup_and_negatives`, `bijection/02_domain_elimination`, `bijection/03_range_elimination` |
| `typecheck-setup` | *typing* | — | fan-out ×2 | 100 | 4 | `bijection/01_setup_and_negatives` |
| `functional-negative` | functional | ≤ | stored `(not …)` | 240 | 7 | `bijection/01_setup_and_negatives`, `bijection/02_domain_elimination`, `bijection/03_range_elimination`, `closure/01_infer_closure` |
| `injective-negative` | injective | ≤ | stored `(not …)` | 240 | 7 | `bijection/01_setup_and_negatives`, `bijection/02_domain_elimination`, `bijection/03_range_elimination`, `closure/01_infer_closure` |
| `domain-elimination` | total ∧ functional | **≥** | **forces — 1-end** | 400 | 7 | `bijection/01_setup_and_negatives`, `bijection/02_domain_elimination`, `bijection/03_range_elimination`, `closure/01_infer_closure` |
| `range-elimination` | surjective ∧ injective | **≥** | **forces — 1-end** | 400 | 6 | `bijection/01_setup_and_negatives`, `bijection/02_domain_elimination`, `bijection/03_range_elimination` |
| `typecheck-arg-0` | *typing* | — | `(false)` | 220 | 1 | `bijection/04_typecheck_arg0_violated` |
| `typecheck-arg-1` | *typing* | — | `(false)` | 220 | 1 | `bijection/05_typecheck_arg1_violated` |

### `std.elim`

| rule | property | half | form | band | corpus entries firing | `tests/stdlib/` activator |
|---|---|---|---|---:|---:|---|
| `typecheck-arg-0` | *typing* | — | `(false)` | 110 | 1 | `elim/03_typecheck_arg0_violated` |
| `typecheck-arg-1` | *typing* | — | `(false)` | 110 | 1 | `elim/04_typecheck_arg1_violated` |
| `domain-elimination` | total ∧ functional ᵖ | **≥** | **forces — 1-end** | 400 | 2 | `elim/01_domain_elimination` |
| `no-room-left` | total ᵖ | **≥** | `(false)` — **0-end** | 110 | 1 | `elim/02_no_room_left` |

### `std.slots`

| rule | property | half | form | band | corpus entries firing | `tests/stdlib/` activator |
|---|---|---|---|---:|---:|---|
| `slot-partition-setup` | slot-partition | — | fan-out ×8 | 100 | 7 | `slots/01_partition`, `slots/02_negative`, `slots/03_fill`, `slots/04_elimination`, `slots/07_spatial_prune`, `slots/08_spatial_adjacent` |
| `slot-spatial-setup` | slot-spatial | — | fan-out ×8 | 100 | 3 | `slots/07_spatial_prune`, `slots/08_spatial_adjacent` |
| `slot-locate` | slot-partition | — | derives | 200 | 2 | `slots/01_partition` |
| `slot-exclusive` | slot-partition | ≤ | stored `(not …)` | 240 | 9 | `slots/01_partition`, `slots/02_negative`, `slots/03_fill`, `slots/04_elimination`, `slots/05_no_room`, `slots/06_no_fill`, `slots/07_spatial_prune`, `slots/08_spatial_adjacent` |
| `slot-occupied` | slot-partition | ≤ | stored `(not …)` | 240 | 7 | `slots/01_partition`, `slots/02_negative`, `slots/03_fill`, `slots/04_elimination`, `slots/07_spatial_prune`, `slots/08_spatial_adjacent` |
| `slot-negative` | slot-partition | ≤ | stored `(not …)` | 240 | 7 | `slots/01_partition`, `slots/02_negative`, `slots/03_fill`, `slots/04_elimination`, `slots/07_spatial_prune`, `slots/08_spatial_adjacent` |
| `slot-elimination` | slot-partition ᵈ | **≥** | **forces — 1-end** | 400 | 7 | `slots/01_partition`, `slots/02_negative`, `slots/03_fill`, `slots/04_elimination`, `slots/07_spatial_prune`, `slots/08_spatial_adjacent` |
| `slot-fill` | slot-partition ʳ | **≥** | **forces — 1-end** | 400 | 7 | `slots/01_partition`, `slots/02_negative`, `slots/03_fill`, `slots/04_elimination`, `slots/07_spatial_prune`, `slots/08_spatial_adjacent` |
| `slot-no-room` | slot-partition ᵈ | **≥** | `(false)` — **0-end** | 250 | 2 | `slots/05_no_room` |
| `slot-no-fill` | slot-partition ʳ | **≥** | `(false)` — **0-end** | 250 | 2 | `slots/06_no_fill` |
| `slot-adjacent-fwd` | slot-spatial | — | derives | 200 | 2 | `slots/08_spatial_adjacent` |
| `slot-adjacent-bwd` | slot-spatial | — | derives | 200 | 2 | `slots/08_spatial_adjacent` |
| `slot-adjacent-fwd-neg` | slot-spatial | ≤ | stored `(not …)` | 240 | 3 | `slots/07_spatial_prune`, `slots/08_spatial_adjacent` |
| `slot-adjacent-bwd-neg` | slot-spatial | ≤ | stored `(not …)` | 240 | 3 | `slots/07_spatial_prune`, `slots/08_spatial_adjacent` |
| `slot-prune-fwd` | slot-spatial | ≤ | stored `(not …)` | 250 | 3 | `slots/07_spatial_prune`, `slots/08_spatial_adjacent` |
| `slot-prune-bwd` | slot-spatial | ≤ | stored `(not …)` | 250 | 3 | `slots/07_spatial_prune`, `slots/08_spatial_adjacent` |
| `slot-endpoint-fwd` | slot-spatial | ≤ | stored `(not …)` | 240 | 3 | `slots/07_spatial_prune`, `slots/08_spatial_adjacent` |
| `slot-endpoint-bwd` | slot-spatial | ≤ | stored `(not …)` | 240 | 3 | `slots/07_spatial_prune`, `slots/08_spatial_adjacent` |

### `std.typing`

| rule | property | half | form | band | corpus entries firing | `tests/stdlib/` activator |
|---|---|---|---|---:|---:|---|
| `type-hierarchy-converse` | *typing* | — | fan-out | 120 | 2 | `typing/02_type_hierarchy_converse`, `typing/03_type_hierarchy_welltyped` |
| `derive-reflexive` | reflexive | — | fan-out | 120 | 1 | `typing/01_reflexive` |
| `reflexive-dom` | reflexive | **≥** | **forces** | 110 | 1 | `typing/01_reflexive` |
| `reflexive-cod` | reflexive | **≥** | **forces** | 110 | 1 | `typing/01_reflexive` |

### `std.closure`

| rule | property | half | form | band | corpus entries firing | `tests/stdlib/` activator |
|---|---|---|---|---:|---:|---|
| `infer-closure` | *closure* | — | derives `__closed__` | 90 | 1 | `closure/01_infer_closure` |


## 3. The endpoints, verified — and three holes

[`obligation_forms.md` §8](obligation_forms.md#8-what-each-form-does-to-the-stdlib)
claimed the map: per family, candidates = 0 ⇒ `(false)`, = 1 ⇒ forced.
**Confirmed for three families of four, with the corrections below.**

| family | 0-endpoint | 1-endpoint | complete? |
|---|---|---|---|
| `std.algebra` | `total`, `surjective` | — | **no** — it has no elimination of its own; the 1-endpoint arrives from `std.bijection`, and only because `bijective-setup` activates both |
| `std.bijection` | — | `domain-elimination`, `range-elimination` | **no**, symmetrically — it has no unreachability scan of its own |
| `std.elim` | `no-room-left` (domain only) | `domain-elimination` (domain only) | **half** — [F3](#f3--stdelim-has-no-range-side) |
| `std.slots` | `slot-no-room`, `slot-no-fill` | `slot-elimination`, `slot-fill` | **yes** — both directions × both endpoints, the only family that is self-contained |

So §8's table is right about the *mechanisms* and wrong about the *families*:
the two endpoints of the bijective pair live in **different modules**, joined
by one activator fan-out. That is the audit's first input to
[S1d.2.4](s1d.2.4_obligations_in_the_saturator.md) — the duals belong beside
the 0-endpoints in `std.algebra`, and the fan-out that reaches them is
`std.bijection`'s.

### F1 — `std.elim`'s positional markers are guards that nothing checks

`(functional R 0 1)` and `(total R 0)` appear only as *premises* of
`std.elim`'s rules. `std.algebra`'s `functional` takes one parameter, so an
arity-3 activator does not match it (S1.22.0 skips activators whose arity
differs), and no rule anywhere consumes the positional form. A puzzle written
to the `std.elim` formulation therefore gets elimination **and no violation
check at all**:

```lisp
(import std.elim :symbols (domain-elimination no-room-left))
(relation r O V)  (is-a o1 O) (is-a v1 V) (is-a v2 V)
(functional r 0 1)  (total r 0)  (domain-elimination r is-a O V)
(r o1 v1)  (r o1 v2)          ; ← two images: functionality violated
```

`ein solve` → **`Solution`**. Adding the arity-1 `(functional r)` marker to
the same file → `Contradiction`, unsat core `(r o1 v1) (r o1 v2)`.

This is worse than a missing endpoint, and it is the reason it is F1:
`domain-elimination` **forces arrows** on the strength of a `(functional R 0
1)` premise that nothing in the tree verifies. Named as a finding, not filled
— filling it is a stdlib change with its own conformance pair, and it belongs
to whoever owns `std.elim`, not to an obligations phase. But the duals must
not be written to the positional spelling, because it would inherit the hole.

### F2 — `connex` is a lower bound in the form `total` was written to avoid

`total` scans for a **stored negative** per candidate
(`(forall ?b (?isa ?b ?B) (not (?R ?a ?b)))`) and so fires only when every
candidate has been *refuted*. `connex` scans for **absence**
(`(absent (?R ?a ?b)) (absent (?R ?b ?a))`) and so fires when a pair has
merely not been decided yet. On the same empty-state shape:

| | `connex` | `total` |
|---|---|---|
| two declared objects, no arrows | **`Contradiction`**, unsat core = the two `is-a` facts | `Solution` |

`std.algebra`'s module header documents the caveat — the extensive ops are
"sound only when the operand is saturation-determined … NOT when it needs
hypothesis BRANCHING" — so this is known and opt-in, not a bug. The audit's
point is narrower: **of the six `≥` refutations in the stdlib, five are
extension-safe and one is not**, and the one that is not sits at the same
band as two that are. [S1d.2.2](s1d.2.2_domains.md)'s refutation division —
"the tally may under-claim but never over-claims, and `(false)` outranks the
tally" — is sound against the five and inherits `connex`'s caveat verbatim
against the sixth. Say so there rather than discovering it later.

### F3 — `std.elim` has no range side

`std.bijection` and `std.slots` each carry both directions; `std.elim` has
`domain-elimination` and `no-room-left` and no `range-elimination`, no
`no-fill`. So the positional formulation expresses `total`-like requirements
and not `surjective`-like ones. Consequence for this phase: **the duals are
two in `std.algebra`'s formulation and two in `std.slots`'s, but only one
would have a home in `std.elim`** — which is another argument for not
writing them there.

## 4. The disturbance list

The duals do not land in a band: they run after the fixpoint, in a pass of
their own ([S1d.2.3](s1d.2.3_the_form.md) item 1). So the question is what
could reach the saturation loop from outside it.

### 4.1 What is order-sensitive, and therefore what the bit-identity check protects

**27 of the 73 rules carry a NAF premise** (`absent`, or a `forall` that
expands to one) and **16 match a stored `(not …)`**; 42 are pure positive
joins and cannot care when they run. The order-sensitive set is the whole
`≥` machinery plus the negative-completion band:

| band | rules | depends on |
|---:|---|---|
| 110 | `total`, `surjective`, `no-room-left`, `connex`, the `converse-illtyped` pair, `std.elim`'s typechecks | every stored `(not …)` that will ever exist for the scanned slot |
| 220 / 240 | `functional-negative`, `injective-negative`, `compose-negative-*`, `slot-exclusive` / `-occupied` / `-negative`, the four `slot-adjacent-*-neg` / `slot-endpoint-*`, `std.bijection`'s typechecks | the positives present when they run |
| 250 | `slot-no-room`, `slot-no-fill`, `slot-prune-*` | the 240 band having completed |
| 400 | the four eliminations | *all* of the above — they force a positive on the strength of a complete exclusion |

An obligation pass that runs **after** the fixpoint sits after every one of
these by construction, which is the whole argument: there is no band to be
wrong about, and no re-check to rely on.

### 4.2 The one thing that does reach the loop — and it is not free

**Finding, and it contradicts an acceptance bullet as written.** A rule whose
`:match` names a relation through a variable **must** get that variable from
its activator — verified:

```
CompileError: unbound relation head ?R in a premise … M1 matches relations
per activator (Q29), so the head var must be bound by the rule's activator.
```

So `total-owed` cannot be a parameter-less rule that reads `(total ?R ?isa)`
facts out of the KB; it needs a `(total-owed R isa)` **activator fact**, and
something has to assert it. That something is the existing fan-out — and an
activator fact is a *stored fact*, so:

| fan-out | fires in | `(bijective R)` / `(slot-partition …)` declarations it serves | facts added by +2 activators |
|---|---:|---:|---:|
| `bijective-setup` (asserts 6 → 8) | 6 entries | 18 | **36** |
| `slot-partition-setup` (asserts 8 → 10) | 7 entries | 7 | **14** |

Concretely: `examples/zebra2.ein`, `zebra2-minus-15.ein` and
`ein-bugs/zebra2-bad.ein` gain 10 facts each; `examples/zebra.ein` gains 2;
nine `tests/stdlib/` programs gain 2 each. **Thirteen entries, 50 facts** —
and `examples/syntax/constraint-scopes.ein` is not among them, because it
declares a `slot-partition` whose setup rule it never imports, which is why
the declaration count and the firing count differ by one.

**So S1d.2.4's "the saturation loop's firing counts and selection order are
bit-identical on every corpus entry" cannot hold as written.** What can hold,
and what that stage should claim instead:

- **the saturation rules' firing counts are unchanged** — no new rule enters
  the agenda, and the added facts activate only the obligation pass;
- **the fact store grows by exactly two activator facts per declaration**,
  predicted in advance by the table above and diffed as a number, the
  `layer_census` style of claim;
- **goldens, `--events` streams and DOT views move on those 13 entries** and
  are re-blessed once, in that stage's commit, with the diff shown to be
  exactly the activator facts.

Two smaller channels, both real and both cheap to close:

- **`alive` and hypothesis generation.** The new activator facts are facts of
  a stdlib relation, so a *blind* run could propose them. Every entry the
  fan-outs reach scopes its hypotheses today (`:hypothesis-relations` /
  `:no-hypothesis`), so nothing changes there — but the obligation activators
  should be added to the same scoping the six existing ones already sit
  behind, and that is a fixture, not an argument.
- **counters keyed on the rule count.** The stdlib goes 73 → 77 (two duals
  for the algebra formulation, two for slots), which moves anything that
  reports "rules loaded" and moves `stdlib_coverage.rs`'s own 73.

### 4.3 What stays untouched

`__closed__` (`std.closure/infer-closure` derives it from `functional` ∧
`total`) is unaffected: the obligation pass reads and asserts nothing, so a
relation's closedness is decided exactly as today. The closed-and-owing
corner — a relation both `__closed__` and owing — is real but it is
[S1d.2.2](s1d.2.2_domains.md)'s, and this audit only confirms that nothing in
the loop decides it differently.

## 5. The incomplete-candidates set

Which entries activate an unreachability scan at all, and which end without
it firing — the first look at who might owe something. From the 180-entry
census:

| scan | band | entries where it is loaded | where it **fires** | **loaded and silent** |
|---|---:|---:|---:|---:|
| `total` | 110 | 11 | 4 | **7** |
| `surjective` | 110 | 9 | 3 | **6** |
| `no-room-left` | 110 | 3 | 1 | **2** |
| `slot-no-room` | 250 | 8 | 2 | **6** |
| `slot-no-fill` | 250 | 8 | 2 | **6** |
| `connex` | 110 | 2 | 1 | 1 |

Against which the **1-endpoints** are doing nearly all of the work:

| | fires in | productive firings |
|---|---:|---:|
| `std.bijection/domain-elimination` | 7 | **1 011** |
| `std.bijection/range-elimination` | 6 | 206 |
| `std.slots/slot-elimination` | 7 | 292 |
| `std.slots/slot-fill` | 7 | 34 |
| `std.algebra/total` | 4 | 82 |
| `std.algebra/surjective` | 3 | 34 |

**The `≥` half spends its life forcing witnesses, not refuting them** — 1 543
productive elimination firings against 116 refutations. That is the shape the
obligation pass is going to report on: the states where an elimination has
not fired *yet* and a refutation never will.

One row is worth naming on its own. On
[`examples/zebra2-minus-15.ein`](../../../examples/zebra2-minus-15.ein) —
the milestone's fixture — **`total` fires and `surjective` does not**. Both
are activated (`bijective-setup` reaches it), so the difference is the
puzzle: the forward direction reaches unreachability inside dead forks, the
backward one never does. That asymmetry is the first concrete prediction the
audit hands forward, and
[T1d.2.4.5](s1d.2.4_obligations_in_the_saturator.md)'s `owes = 46` — 23
forward + 23 backward — is where it gets checked from the other side.

The seven entries where `total` is loaded and silent, and the six for
`surjective`, are the candidate set for
[S1d.2.6](s1d.2.6_verdicts_counters_corpus.md)'s census. They are almost all
`tests/stdlib/` programs — which is the honest caveat on this table: the
corpus's *puzzles* mostly either solve or die, and the entries that sit
unfinished are the ones written to sit unfinished.

## 6. What this changes upstream

| finding | what it moves |
|---|---|
| §1's empty `≥ 2` row | nothing — it confirms the phase's premise, with a number |
| §3 F1 (positional markers unchecked) | the duals are written to `std.algebra`'s formulation and `std.slots`'s, never `std.elim`'s |
| §3 F2 (`connex` naive) | [S1d.2.2](s1d.2.2_domains.md) T1's refutation contract names it as the one `≥` refutation that is not extension-safe |
| §3 the split families | the duals live beside the 0-endpoints in `std.algebra`; `std.bijection`'s fan-out reaches them |
| **§4.2 (activator facts are stored facts)** | **[S1d.2.4](s1d.2.4_obligations_in_the_saturator.md)'s bit-identity acceptance is restated** as "saturation firings unchanged, fact store +2 per declaration, predicted and diffed, goldens re-blessed once on 13 entries" |
| §5 | S1d.2.6's census has its candidate list three stages early |
