# Obligation expression forms — the menu S1d.2.3 chooses from

**Phase:** [P1d.2](README.md) — this is [S1d.2.3](README.md#stages)'s input, written
before its stage file so the decision is the user's and the plan follows it.
**Status:** **a menu, not a decision.** Six forms, each stated as syntax +
mechanism + what it cannot say. Written 2026-08-24.
**Reads:** [`../ideas.md`](../ideas.md) (authoritative on intent),
[`../p1d.10_exhaustive_search/layer_census.md`](../p1d.10_exhaustive_search/layer_census.md)
(what the corpus actually does today).

---

## 1. What the form has to carry

The note's shape ([`ideas.md`](../ideas.md) § "Удобная общая форма"):

```
L ≤ #{ ȳ | R(x̄, ȳ) ∧ φ(x̄, ȳ) } ≤ U
```

Four operands, and two structural questions the existing code forces:

| | what it is | why it is not obvious |
|---|---|---|
| **the counted slot** | which argument positions are fixed (`x̄`) and which are counted (`ȳ`) | `std.elim` already uses a positional convention — `(functional R 0 1)`, `(total R 0)` — so there is precedent, and it is ugly |
| **the domain** of `ȳ` | the set the count ranges over | **The hard one.** See below |
| **the bounds** `L`, `U` | `0`, `1`, `n`, `*` | `RANGE` is already a lexer token |
| **the side condition** `φ` | a guard on which `ȳ` count | whatever expresses it must be as expressive as a `:match` |
| *derived or static?* | can a **rule** produce an obligation? | `bijective` fans out into its four markers *through a rule* (`stdlib/algebra.ein` `bijective-properties`, priority 100). A form a rule cannot produce cannot participate in that, and every stdlib property is written that way |
| *who holds the candidate set?* | the engine, or a re-run query | `_admit_from_boundary`'s re-query was **72 %** of an exhaustive `zebra2` before P1a.6. An obligation index rebuilt at every quiescence is the same cost shape |

**The domain is the constraint that eliminates the obvious answers.** The stdlib
is deliberately **is-a-free in rule bodies** (S1.8.A10): the type-membership
relation arrives as the parameter `?isa`, never as an `is-a` literal, so a
puzzle may pass `is-a` or `is-a*` and the kernel commits to no type system
(S1.7.23). An obligation form that says "quantify over the declared type's
extent" puts type-directed quantification **in the kernel** — which is exactly
the line M1a held.

## 2. What exists today

Both endpoints of the arithmetic are implemented. The middle is empty.

| candidates left for a required arrow | today | where |
|---|---|---|
| **0** | `(false)` — the state is dead | `std.algebra`'s `total` / `surjective`, open-world-safe: `(forall ?b (?isa ?b ?B) (not (?R ?a ?b)))` ⇒ `(false)` |
| **1** | the positive is forced | `std.elim` / `std.bijection`'s `domain-elimination` / `range-elimination` |
| **≥ 2** | **nothing is recorded** | — |

And two facts about the language that decide half the design space:

- **`:cardinality <RANGE>` already parses and means nothing.** `RANGE` is a
  lexer terminal (`[0-9]+\.\.([0-9]+|\*)`), `:cardinality` is in the `relation`
  declarator's grammar
  ([`01_grammar.md` § Relation declarator](../../../docs/kernel/ir/03-ein-lang/01_grammar.md#relation-declarator)),
  and `ir_semantics.rs`'s round-trip suite pins `0..0`, `0..*`, `9999..*`.
  **No engine code reads it.**
- **Macros expand in `:assert`, not only in `:match`.**
  `ein_ir::macros::expand_rule_clauses` expands both clauses of a `rule` /
  `hrule`, so a new obligation *surface* can ship as a `(macro …)` in
  `std.macro` beside `forall` and `open` — **with no parser change at all.**

## 3. The forms

### A — `:cardinality`, given the meaning it already has syntax for

```lisp
(relation pet-loc Pet House :cardinality 1..1)
```

**Mechanism.** The loader reads the range and, for every `a` in the extent of
`Pet`, records `1 ≤ #{b : (pet-loc a b)} ≤ 1`. No matcher cost to *state* one.
The dual — surjectivity, "every house has a pet" — needs a second range, so the
form grows a `:co-cardinality 1..1` or a two-range `:cardinality 1..1/1..1`.

**Cannot say.** Anything conditional; anything a rule derives; any `φ`. And it
cannot name `?isa`, so the domain is *the declared type* — §1's forbidden
answer. It is also stuck at arity 2: `(relation R A B C)` has three positions
and one range.

**Verdict.** Not the mechanism. Possibly free sugar *over* the mechanism —
see §7.

### B — an obligation is a fact

```lisp
(rule bijective-obligations ()
  :match  (and (bijective ?R) (relation ?R ?A ?B) (?isa ?a ?A))
  :assert (must ?R 0 ?a ?isa ?B 1 1)
  :priority 100)
```

`must` joins `not`, `false`, `relation` and `__closed__` as a **reserved
relation name** the kernel interprets. Read it as: *count the completions of
`(?R ?a _)`; the counted slot ranges over `{?b | (?isa ?b ?B)}`; between 1 and 1.*

**Mechanism.** An obligation is an ordinary fact, so it gets — for free, with no
new machinery — **provenance**, an `--events` line, trace rendering, and
copy-on-write fork-locality: an obligation derived inside a fork dies with the
fork because the KB layer does. At quiescence the saturator scans the `(must …)`
extent, computes each candidate set, and takes one of the note's three outcomes
(`ideas.md` § "Когда fixed point является решением"): `#present > U` or
`#present + #open < L` ⇒ contradiction; `#present < L ≤ #present + #open` ⇒
**incomplete**, report it; otherwise discharged.

**Zero grammar change.** `06_reserved_names.md` gains a row; nothing else moves.

**Costs.** The candidate set is *not* in the fact, so each quiescence re-runs a
query per open obligation — §1's 72 % cost shape, in full. And the argument
list is positional and long: seven arguments is not a form anyone will write by
hand, which is why it wants a surface (D or A) on top.

### C — a new top-level declarator

```lisp
(require pet-placement (?R ?isa)
  :forall  (?a (?isa ?a Pet))
  :count   (?b (?isa ?b House))
  :of      (?R ?a ?b)
  :between 1..1
  :why     "every pet is in exactly one house")
```

Activated by a fact, the way every generic stdlib rule is:
`(pet-placement pet-loc is-a)`.

**Mechanism.** A declaration, but its *instances* are one per `?a` the
`:forall` binds — so it is as conditional as a rule, and `φ` goes in `:count`'s
guard or a `:where`. It reads exactly like §1's formula, which is its whole
argument: a reader who knows the mathematics can read the puzzle.

**Costs.** A new reserved head is the most expensive change on this page. It
touches the recursive-descent parser, `00_ebnf.md`, the loader's classifier, the
dumper (and therefore the round-trip property), `06_reserved_names.md`, the
TextMate grammar in `utils/vscode-ein/` — **and [M2](../../m2_nl_to_ir/README.md)'s
GBNF lift, which reads the grammar to constrain a model's output.** P1d.2's
README lists that as the phase's first risk, and this is the form that incurs it.

### D — an `:assert` form: the dual of `forall`

```lisp
(rule total (?R ?isa)
  :match  (and (relation ?R ?A ?B) (?isa ?a ?A))
  :assert (at-least 1 ?b (?isa ?b ?B) (?R ?a ?b))
  :priority 110)

(rule bijective-obligations ()
  :match  (and (bijective ?R) (relation ?R ?A ?B) (?isa ?a ?A))
  :assert (between 1 1 ?b (?isa ?b ?B) (?R ?a ?b)))
```

**`forall` is `(forall ?b G B)` in `:match`. This is `(between L U ?b G B)` in
`:assert` — the same shape in the dual position.** The bound variable, its
guard (the domain, `?isa`-parameterised and therefore is-a-free) and the body
are exactly `forall`'s three operands; only the quantifier changes.

**Mechanism.** The rule's `:match` supplies `x̄` and the fixed part of `φ`; the
assert form supplies `ȳ`, its domain and the bounds. Because macros already
expand in `:assert` (§2), this can ship as a `std.macro` entry desugaring to
whichever carrier is chosen — **so D is a surface and B is its carrier, and
they are one decision with two faces, not two decisions.**

**What it buys the stdlib.** `total` and `surjective` become one line each, and
their current open-world-safe `(false)` rules stop being separately authored:
`#present + #open = 0` is the *consequence* of the obligation, computed once by
the engine, rather than a `forall` scan the author had to get right. Likewise
`domain-elimination` / `no-room-left` become the `= 1` and `= 0` cases of one
mechanism.

**Costs.** Whatever the carrier costs. If the carrier is B, a re-query per
quiescence; if E, a shrinking object.

### E — obligations as **positive clauses**

```lisp
(rule total (?R ?isa)
  :match  (and (relation ?R ?A ?B) (?isa ?a ?A))
  :assert (some ?b (?isa ?b ?B) (?R ?a ?b)))
```

which materialises, at firing time, the clause `(R a b₁) ∨ … ∨ (R a bₙ)` over
the domain's *current* extent, and stores it.

**Mechanism — unit propagation.** As each `(not (R a bᵢ))` arrives the clause
shortens. At **one** survivor, assert it: that *is* `domain-elimination`,
generalised, and it no longer needs a `forall` scan over the whole domain to
notice. At **zero**, `(false)`: that is `no-room-left`. At **two or more** it is
an open clause, and the search branches on it — **mutually exclusive and jointly
exhaustive**, which is precisely what a subset lattice cannot be, and the
property the milestone README's argument turns on.

**The unification is the argument.** The no-good store already holds *negative*
clauses and `apriori::filter_candidate` already prunes with them. This adds
*positive* ones: one data structure, two signs. The candidate set lives **in the
object**, so there is no per-quiescence re-query — invalidation becomes
watched literals, which has a forty-year literature.

**And the census supports it specifically.** `alive` never shrinks in the barren
regime — 3 of 46 multi-layer cells, all three in the pruning four — so **the
clause store is the only thing that can shrink a layer**
([layer_census.md §6](../p1d.10_exhaustive_search/layer_census.md#6-one-of-the-two-filter-arms-cannot-fire)).
A positive clause is the one object that shrinks it *by construction* rather
than by waiting for a death.

**Cannot say.** `L ≥ 2`, and any `U`. It is exactly the `∃` case — `U` stays
with the existing `functional` / `injective` refutations. It also freezes the
domain at firing time: a `(is-a b₆ House)` that arrives later does not extend an
already-materialised clause. Sound under the note's §6 finite-closed-domain
premise, which this milestone assumes anyway (§ Non-goals: no object creation) —
but it is an assumption the form makes and the others do not.

**One thing to check before choosing it.** `(or …)` is reserved as a `:match`
primitive; a *stored* `(or …)` fact is a shape nothing in the KB has today, so
either the clause is a fact of a new reserved head or it is a kernel object
beside the no-good store. That is a real sub-decision, not a detail.

### F — `hrule :choose`

```lisp
(hrule pet-placement (?R ?isa)
  :match  (and (bijective ?R) (relation ?R ?A ?B) (?isa ?a ?A))
  :choose ?b :from (?isa ?b ?B) :exactly 1
  :assert (?R ?a ?b))
```

**Mechanism.** Obligations live on the one declarator that is *already* about
choice, so a changed traversal is expected rather than a regression — which
makes [Q-M1a.18](../../../docs/history/m1a_rust/open_questions.md#q-m1a18--may-a-fork-stop-re-narrating-the-roots-fixpoint)'s
decision cheaper than it is for any other form here. Kernel fact semantics are
untouched: an obligation is a **search** object, not a saturation object.

**Cannot do.** Make a *saturated state* know what it owes — which is P1d.2's
goal in one sentence. The phase README's own idea note names this as `hrule`'s
weakness: *"while it is not part of the theory (rules + ontology)"*. With F,
`complete` can never mean **discharged**; it stays "the generator proposes
nothing".

## 4. The two axes

The six are not six independent choices. They separate cleanly:

**Axis 1 — the surface** (how an author writes one): A (declarator metadata),
B (a bare reserved fact), C (a new head), D (an assert form), F (an `hrule`
clause).

**Axis 2 — the payload** (what the engine holds for an *open* one): a
**counter** over a re-queried candidate set (B/C/D), or a **clause** over a
materialised one (E).

E is therefore orthogonal: it is an answer to axis 2 that any of B/C/D can
carry. And A is orthogonal again — it is sugar, and it can desugar to whatever
the other two axes settle on.

## 5. What this looks like on `zebra2-minus-15`

Measured at root, 2026-08-24 (`ein solve -H -m 1 -e`, and the `hyp` event
stream):

| | |
|---|---|
| `(bijective R)` declarations | **5** — `color-loc`, `nation-loc`, `drink-loc`, `smoke-loc`, `pet-loc` |
| obligations they imply | **50** = 5 relations × 5 values × 2 directions |
| raw candidate arrows | 125 = 5 × 5 × 5 |
| decided at root by negative completion | 29 — 27 refuted, 2 already true |
| **open arrows** (`alive`) | **96** |
| forward obligations still open | **23**, with **3, 4 or 5** candidates each (5 × 3, 9 × 4, 9 × 5) |
| backward obligations still open | **23**, with **2 – 5** candidates (1 × 2, 2 × 3, 12 × 4, 8 × 5) |

**What the search does with that today**: layer 1 enters all 96 singletons and
kills none; layer 2 enters all `C(96, 2) = 4 560` pairs; the whole exhaustive
run is **618 076 enterings and 416 s**, of which **92.1 % happen after the last
new model is found**
([layer_census.md §4](../p1d.10_exhaustive_search/layer_census.md#4-zebra2-minus-15-all-five-layers)).

**What a choice point is instead**: one obligation with 5 candidates is a
5-way branch that is complete at that node by construction. Nothing has to
refute the four siblings — committing to one excludes them.

Two arithmetic consequences worth having before the design starts:

- **318 of layer 2's 4 560 pairs are two candidates of the same obligation**
  (`Σ C(k,2)` over both histograms above = 159 + 159). Every one of them
  violates a `U = 1` bound *by construction* and is a set an obligation-driven
  search can never form. At layer 2 that is 7 %; the fraction of `k`-subsets
  containing a same-obligation pair rises with `k`.
- It is **not** the whole win, and the honest version of the claim is smaller
  than the arithmetic suggests: layer 2 killed 1 428 commitments, so only 22 %
  of its deaths are the ones a choice point removes for free. The other 78 %
  come from the puzzle's own rules and would still have to be discovered.
  **What obligations change is the shape of the space, not the strength of the
  propagation** — and the census is what says which of those the corpus needs.

## 6. Comparison

| | new grammar | rule can derive it | carries `φ` | domain via `?isa` | candidate set | `L ≥ 2` / `U` | fork-local free |
|---|---|---|---|---|---|---|---|
| **A** `:cardinality` | **none** (token exists) | no | no | **no** — declared type | re-query | yes | n/a |
| **B** `(must …)` fact | **none** | yes | yes | yes | re-query | yes | **yes** |
| **C** `(require …)` | new head — M2's GBNF too | yes | yes | yes | re-query | yes | no |
| **D** `at-least` in `:assert` | **none** (a `std.macro`) | yes | yes | yes | its carrier's | yes | via B |
| **E** positive clause | none, if via D | yes | yes | yes | **in the object** | **no** | yes |
| **F** `hrule :choose` | new kw on `hrule` | yes | yes | yes | in the search | yes | n/a |

## 7. A recommendation

**D as the surface → B as the carrier → E as the representation of the `L = 1`
case**, with **A as free sugar** on top.

- Write `(at-least 1 ?b G B)` / `(between L U ?b G B)` in `:assert`. It is
  `forall`'s dual, it ships as a `std.macro` entry, and it costs **no parser
  change**.
- It desugars to a reserved `(must …)` fact, which costs **no grammar change**
  and inherits provenance, `--events`, the trace and fork-locality because it
  is a fact like any other.
- The saturator represents an *open* obligation whose `L = 1` as a **positive
  clause over its live candidates**, sharing the no-good store's machinery —
  which is the half the census argues for, because the clause store is the only
  thing that can shrink a layer today.
- `(relation R A B :cardinality 1..1)` then desugars to the same thing for the
  common declaration-site case, and the dead slot in the grammar finally means
  something.

**What that leaves open, deliberately**: `L ≥ 2` and `U` have no clause
representation, so they stay counters — and by
[P1c.1](../../../docs/history/m1c_external_validation/README.md#p1c1--stdlib-conformance)'s
rule, they arrive **when a corpus entry cannot be stated without them**, not
because the form would be more general with them. Today no corpus entry needs
either: every `bijective` is `L = U = 1`, and `U = 1` is already enforced by
`functional` / `injective`.

## 8. What each form does to the stdlib

Whichever is chosen, the same four rules are the test of it — they are what
`bijective` currently means:

| rule | today | under an obligation form |
|---|---|---|
| `functional` (`std.algebra`) | `(false)` on two images | unchanged — it is the `U = 1` half and already works |
| `injective` | `(false)` on two preimages | unchanged |
| `total` | `forall`-scan ⇒ `(false)` when *every* partner is excluded | **one obligation**; the `(false)` becomes its `#present + #open = 0` case |
| `surjective` | the dual scan | the dual obligation |
| `domain-elimination` (`std.elim`, `std.bijection`) | `forall`-scan ⇒ force the survivor | the obligation's `= 1` case |
| `no-room-left` | `forall`-scan ⇒ `(false)` | the same as `total`'s |

Six hand-written scans collapse into one mechanism with three outcomes. That is
the strongest argument for doing this at all, and it is also the strongest
argument for **S1d.2.1 running first**: if the six are not as redundant as this
table claims, the collapse is not available.

## 9. What this leaves for the stage files

- **S1d.2.1** — the audit, which decides whether §8's table is true.
- **S1d.2.2** — the domain: what closes it, and whether `?isa` is enough.
- **S1d.2.3** — this decision, plus the sub-decision E raises (is a stored
  clause a fact or a kernel object?).
- **S1d.2.4** — the saturator: invalidation, and the per-quiescence cost §1
  warns about.
- **S1d.2.5** — hypotheses from obligations, and Q-M1a.18's shape.
- **S1d.2.6** — verdicts, counters, corpus.
