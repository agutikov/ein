# Obligation expression forms — the menu S1d.2.3 chooses from

**Phase:** [P1d.2](README.md) — this is [S1d.2.3](README.md#stages)'s input, written
before its stage file so the decision is the user's and the plan follows it.
**Status:** **a menu, not a decision.** Seven forms, each stated as syntax +
mechanism + what it cannot say. Written 2026-08-24; **G arrived the same
day**, the user's follow-up proposal, quoted verbatim in
[§ G](#g--a-verdict-atom-assert-open-the-dual-of-false).
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
| **the bounds** `L`, `U` | `0`, `1`, `n`, `*` | `RANGE` is already a lexer token — though § G's revision argues numerals are the wrong currency for any bound past `0`/`1`/`*`, and `n` is really a reference set |
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

### G — a verdict atom: `:assert open`, the dual of `(false)`

**The user's proposal (2026-08-24, verbatim):**

> now we have upper limit of relations in 2 forms (assert false and assert
> negative fact that then can conflict and assert false finally); also the KB
> can be in one of 3 states: open, false (contradiction) and satisfy (model);
> So I propose to introduce new special atom to assert ":assert open" that
> will indicate KB is in open state; we count open assertions and it should be
> equal to the number of sum of obligated facts multiplied to it's arity; So
> engine has this number for every KB, also it see relations and objects used
> in matcher and can build indexes and thus domain and co-domain

and the example, from the same session:

```lisp
(rule :match (and (is-a ?who Nationality) (absent co-loc House_1 ?who))
      :assert open
      :why "somebody must live in the House_1")
```

and the revision, later the same day, after the first write-up (verbatim):

> L >= 2 : yes, G form can't say that, but remember, the ein language does
> not use numbers, so it will define domain/comdomain size by rules that use
> other relations and objects, not just by simple "2" number that has no
> connections to relations and objects; the obligations mechanism also has to
> supersed the hrule and :hrules, so if no :hrules in query - then hypothesis
> must be generated from obligations

**The symmetry it starts from** is one this page had not stated. The upper
bounds exist in *two* forms — a **verdict atom**
([`std.algebra`](../../../stdlib/algebra.ein)'s `functional` / `injective`
fire `(false)` directly, priority 250) and a **stored fact**
([`std.bijection`](../../../stdlib/bijection.ein)'s `functional-negative` /
`injective-negative` store `(not (?R ?a ?b'))` at 240, which a later positive
conflicts into `(false)`). G claims the lower bounds deserve the same two
duals — and the menu already held the other one without saying so:

| | upper bound `≤` — exists | lower bound `≥` — proposed |
|---|---|---|
| **verdict atom** | `:assert (false)` on violation | `:assert open` while unmet — **G** |
| **stored fact** | `:assert (not …)`, conflicts later | a stored positive clause, discharged later — **E** |

So G and E are not rivals for one slot: they are the two halves the upper
bounds already have, and a design can take both.

**What the example counts — the guard's side of the `absent` decides.** Read
literally, the rule binds `?who` *outside* the NAF, so it fires once per
Nationality not present in House_1 — **four** ways in a *solved* state (the
four who correctly live elsewhere; a stored `(not …)` still passes `absent`),
five in an empty one. That tally is `|domain| − #witnesses`: per-candidate
slack, bottoming out at four, not zero. Moving the guard *inside* the
`absent` makes it the obligation count:

```lisp
(rule house-1-inhabited ()
  :match  (absent (and (is-a ?who Nationality) (co-loc House_1 ?who)))  ; ∄who
  :assert open
  :why    "somebody must live in the House_1")
```

— fires exactly once while House_1 has no inhabitant, and not at all once it
has one. Both spellings are legal today up to the atom (`forall`'s own
expansion is an `absent` over an internally-bound variable). The proposal
tests *equality* against a predicted number rather than zero, which either
spelling can satisfy — but the ledger whose satisfied state reads 0 needs no
prediction, so the arithmetic below is the ∄ spelling's. Its generic form is
`std.algebra`'s `total`, one modality down — scan *absence* where `total`
scans stored `(not …)`, and say *unfinished* where it says *dead*:

```lisp
(rule total-owed (?R ?isa)
  :match  (and (relation ?R ?A ?B) (?isa ?a ?A)
               (absent (?R ?a ?b)))            ; ∄b — no witness yet
  :assert (open ?R 0 ?a)                       ; owed: some (?R ?a _)
  :priority 500
  :why    "{?R} owes {?a} a {?B}")
```

`(open ?R 0 ?a)` borrows B's positional spelling (with B's ugliness) so a
tally line has an identity and a slot; the proposal's bare `:assert open`
stays as the anonymous degenerate. Priority is the probe band (500): an
obligation read before negative-completion (240) and elimination (400) have
run would report debts the same quiescence was about to pay.
[`bijective-setup`](../../../stdlib/bijection.ein) fans out two more
activators (`total-owed` / `surjective-owed`) and no puzzle changes a line.

**Mechanism — and the one thing it must not be.** `(false)` can live as an
ordinary stored fact because contradiction is extension-stable: a dead state
stays dead under any addition. Openness is the opposite — it exists to be
destroyed by an extension — so **an `open` conclusion must not enter the
store**: a fork inheriting the root's `open` facts would still carry the debt
after paying it. The atom is a **per-quiescence verdict, not a fact**: at
each KB's NAF boundary the engine evaluates the open-rules against *that* KB,
counts the matches, and stores nothing — the discipline `absents_still_pass`
already applies to every NAF premise, one band later and read by the engine
instead of written back. Terminal like `(false)`: no rule matches on it. The
read-out is then the proposal's three states, each locally decidable at one
quiescent KB:

| the quiescent KB has | it is |
|---|---|
| `(false)` | **false** — contradiction, exactly as today |
| no `(false)`, tally > 0 | **open** — unfinished; the firing rules' `:why`s *are* the outstanding-obligations report |
| no `(false)`, tally = 0 | **satisfy** — a model, by **discharge** rather than by exhaustion |

**The count, and the number that already checked it.** The proposal's
invariant — open assertions = obligated facts × arity — is the ledger's
*size*: a `bijective` n×n relation owes each of its n arrows from both ends,
n × 2 slots. On `zebra2-minus-15` that is 5 relations × 5 facts × 2 = **50 —
the exact "obligations they imply" row of
[§5](#5-what-this-looks-like-on-zebra2-minus-15)** — and the 2 arrows already
true at root discharge 2 × 2 = 4 of them: tally **46 = §5's 23 forward + 23
backward**, measured by hand before this form had a name. The equality is a
**conservation audit** in the
[`layer_census`](../p1d.10_exhaustive_search/layer_census.md) style: the
engine can predict the ledger from the declarations and diff what the rules
emit, and a mismatch is an encoding bug with a number attached.

**Candidates, with no payload anywhere.** The proposal's last clause — the
engine "can build indexes and thus domain and co-domain" — is where the
candidate set lives: nowhere. A still-firing open names a slot through its
rule's compiled matcher and bindings — `(co-loc House_1 _)` — and candidates
are a join taken on demand: `alive ∩ slot`. §5's per-obligation histograms
(23 forward at 3–5 candidates each) are exactly that join, taken manually.
Nothing is stored, so nothing needs invalidating; the cost moved into the
read.

**Cardinality without numerals — the revision's first clause.** The
"cannot say `L ≥ 2`" verdict below and in §6 scores G against the note's
numeric form, and the revision rejects the yardstick: **the domain of
discourse has no numerals.** [`zebra2.ein`](../../../examples/zebra2.ein)'s
houses carry no positions — order is `right-of` / `next-to`, House×House —
and the only integers in the file are `:priority` metadata. A bound written
`2` names nothing any relation touches; a bound the language can own is a
**relation to a reference extent**, and for that the four properties are
already the complete comparison vocabulary — Cantor's ordering, not Peano's
arithmetic:

| the numeric claim | said with relations and objects |
|---|---|
| `#W ≥ 1` | one obligation — **G, this page** |
| `#W ≤ 1` | `functional` on the witness relation — **exists** |
| `#W ≥ #S` | an injection S → W: a pairing relation, `total` on S (G's obligations, one per member) + `functional` + `injective` (the existing `(false)` checks) |
| `#W ≤ #S` | an injection W → S — the same three, the other way |
| `#W = #S` — the note's `same-count-as` | a bijection — **the stdlib's one word** |

"At least two" is then an injection from a declared two-object reference set —
the `2` connected to relations and objects, as the revision demands — and
`L ≤ # ≤ U` decomposes back into `functional ∧ injective ∧ total ∧
surjective` on pairings: the note's opening equivalence is not the first
example of an obligation, it is the **complete basis for bounds**. Two honest
consequences. The cost is ontology — every bound is an extra relation with
its own obligations and checks, declared by the puzzle, not a token. And the
re-scoring cuts across the whole menu: A, B, C and D all carry numeral
operands (`1..1`, `… 1 1`, `:between 1..1`, `at-least 1`) — the currency the
language otherwise refuses — so under this clause the parsed-and-meaningless
`RANGE` token reads as the anomaly, not the missing feature. (The `0` in this
page's own `(open ?R 0 ?a)` sketch is the same foreign currency — a point for
the unbound-hole spelling in the sub-decision below. `odd` stays exotic in
any basis, and the phase's keyword rule already gates it.)

**Superseding `hrule` / `:hrules` — the revision's second clause.** The
restraint claimed below ("hypotheses still come from `alive`") is, per the
revision, only half the design: *the obligations mechanism has to supersede
`hrule` and `:hrules` — no `:hrules` in the query means hypotheses are
generated from obligations.*
[design/07](../../../docs/history/m1a_rust/design/07_search_layer.md) already
says "two modes, and hrule presence *is* the switch" — hrule-driven when the
query's `:hrules` activators light one, blind otherwise. The supersession
makes the switch a ladder:

| the query has | hypotheses come from |
|---|---|
| `:hrules (…)` | the user's hrules — an **override**, as today |
| no `:hrules`, open obligations | **the obligations**: pick an open slot, branch on `alive ∩ slot` — mutually exclusive, jointly exhaustive at that node |
| no `:hrules`, tally 0 | nothing to guess — the state is already judged; the blind generator survives only for programs that state no obligations at all |

Three consequences, one of them the settling of an old complaint:

- **The hypothesis space stops being an input.** The idea block at the
  [phase README](README.md)'s head objects to hrules in exactly these words —
  *"while it is not part of the theory (rules + ontology)"* — and this rung
  closes it: declarations → obligations → slots is *derived from the theory*,
  and [`zebra2.ein`](../../../examples/zebra2.ein)'s `hrule guess` with its
  per-relation `:hrules` activators demotes to an optional override.
  [§5](#5-what-this-looks-like-on-zebra2-minus-15)'s arithmetic is this
  rung's value measured in advance: a 5-candidate obligation is a 5-way
  branch complete by construction, layer 2's 318 same-obligation pairs are
  never formed, and the milestone README's sentence — *a requirement is a
  choice point* — is this line of the table.
- **It is
  [Q-M1d.4](../open_questions.md#q-m1d4--may-an-obligation-driven-generator-change-the-traversal),
  spent deliberately.** Branching from obligations changes the traversal, the
  counters, the no-goods and the discovery order. So the strata ship
  separately: the tally line first (nothing moves), the generator rung behind
  the explicit decision (everything moves, on purpose).
- **F is subsumed.** `hrule :choose` attached obligations to the choice
  construct; the ladder derives choices from obligations, leaves `hrule`
  unextended as the override, and — unlike F — a *saturated* state still
  knows what it owes, which was F's disqualifier.

And the completeness condition, stated rather than assumed: obligations are an
exhaustive branch source **iff obligations + saturation determine every
remaining open fact** — true on the zebra family, where the obligated arrows
are the decision variables and everything else propagates. Where it fails,
the leftover open facts at a discharged, consistent state are the model
family's free arrows: [`ideas.md`](../ideas.md)'s closed-world sentence ("все
оставшиеся open считаются отсутствующими") is one legal reading, and
[P1d.3](../p1d.3_model_sets/README.md)'s compact model set is the other —
arriving early, as a *state* rather than a data structure. The stage that
flips the generator owes the corpus a measured answer to which entries sit on
which side.

**What it buys, that A–F do not.**

- **[Q-M1d.2](../open_questions.md#q-m1d2--where-does-a-requirement-live)
  answered at (c)** — a rule shape, the answer under which "the phase is much
  smaller than it looks": one reserved atom, a tally per KB, rows in
  [`06_reserved_names.md`](../../../docs/kernel/ir/03-ein-lang/06_reserved_names.md);
  no data-model object, nothing in `.einb`, nothing in the renderers' types.
- **The verdict is the content.** G is the only form whose primary output is
  the three-state read-out —
  [Q-M1d.6](../open_questions.md#q-m1d6--may-contradiction-be-said-with-exhausted--false)'s
  candidate (c) mechanised. The ten `Contradiction, exhausted=False` entries
  partition measurably: owes-something ⇒ *incomplete*; owes-nothing ⇒ the
  vacuous edge below.
- **Additive and reversible — in the report stratum.** §8's six scans stay
  untouched and gain two duals; the tally ships as a report line (`--events`,
  `--json-summary`, the trace) with hypotheses still from `alive`, so **no
  [Q-M1d.4](../open_questions.md#q-m1d4--may-an-obligation-driven-generator-change-the-traversal)
  exposure, every counter standing, no verdict word moved** — the phase's
  "every existing verdict is unchanged" acceptance holds until Q-M1d.6 is
  *deliberately* spent. The generator rung (§ Superseding `hrule` /
  `:hrules`, above) is the opposite, by design and on purpose.
- **No domain freeze.** Re-evaluated per state, so a late `(is-a b₆ House)`
  is seen — the assumption E bakes in at materialisation time, G never makes.
- **It is the instrument.** An openness census per corpus entry is S1d.2.6's
  table, available *before* S1d.2.4/5 commit to machinery — measure before
  designing, the discipline that put P1d.10 first.

**Cannot say, and costs.**

- **Numeric `L ≥ 2` and `U`** — though § Cardinality without numerals, above,
  argues the numeric form is mis-posed for *every* form on this page, and its
  numeral-free decomposition is pairings that G plus the existing checks
  already state (`U = 1` stays `functional` / `injective`, where it already
  works). What G alone does not do is hold the pairing for you: the reference
  set and its map are ontology the puzzle must declare, one relation per
  bound.
- **No propagation.** G never narrows, forces, or refutes:
  `domain-elimination` still forces at one, `total` still kills at zero, by
  the same `forall` scans — §8's six-into-one collapse is **not available**
  under G alone. G is the *record + report* half of the phase goal; *narrow /
  close / refute* stay where they are, or arrive with E.
- **A re-query per quiescence** — §1's 72 % cost shape. But it is
  boundary-shaped: the exact cost P1a.6 spent twelve stages engineering down,
  not a new kind, and the tally is one band of it.
- **The vacuous edge.** An entry stating no obligations has tally ≡ 0, so its
  consistent quiescent states read *satisfy* — where today's `complete(kb)`
  ("does the generator propose anything") says otherwise.
  [`ideas.md`](../ideas.md) § "Когда fixed point является решением" endorses
  exactly this under closed-world completion; the entries where the two
  definitions disagree become tests, as the
  [phase acceptance](README.md#acceptance-for-the-phase) already requires. G
  forces that question early, because G's content *is* the model criterion.

**The sub-decision, and it is mechanical, not aesthetic.** `open` is
[`std.macro`](../../../stdlib/macro.ein)'s, arity-1, match-side — and macros
expand in `:assert` too (§2). So today `:assert (open)` is an arity error and
`:assert (open X)` expands into an `absent`-conjunction that is illegal in
assert position. The ways out are a menu of their own — § The naming menu,
next, measured against the corpus. One shape question rides along: the
slot spelling above is positional because assert-side variables come bound
from the match, so a hole has no spelling there today — relaxing that *for
the verdict head only* (`(open (?R ?a ?b))`, unbound `?b` as the hole) is
prettier and bends a rule that is otherwise uniform. Both are S1d.2.3's,
beside E's fact-or-kernel-object.

**The naming menu — probe and verdict, measured against the corpus**
(2026-08-24, the user asked for the variants). Two different things need
names, and the collision hides that they are different:

- **the probe** — match-side, *fact*-level, exists: `(open P)` ⟺ P is
  neither asserted nor negated. This is [`ideas.md`](../ideas.md)'s third
  fact-state (`present` / `forbidden` / `open`) — the note owns that word
  *for facts*.
- **the verdict** — assert-side, *KB*-level, proposed: this state has an
  unmet obligation. **Not the same notion one level up**: a KB with tally 0
  and a hundred open *facts* is *satisfy* (the vacuous edge above), so a KB
  is not "open" because its facts are — one word for both would rebuild
  [Q-M1d.6](../open_questions.md#q-m1d6--may-contradiction-be-said-with-exhausted--false)'s
  confusion (`alive ≠ ∅` versus "owes something") inside the language itself.

The measured footprint, before choosing. The probe is used by **12
programs** — 11 under `tests/stdlib/`, where it is one of the suite's three
idioms, plus [`examples/features/04_open.ein`](../../../examples/features/04_open.ein)
— so renaming it touches those, [`stdlib/macro.ein`](../../../stdlib/macro.ein),
the manifest, and the pages that teach the idiom. And the suite has already
voted on the probe's semantic field: **seven of those programs name their
observable `undecided`** — `:assert (undecided B C)` as the witness that the
probe passed — which both recommends the word and **blocks it as the macro's
own name** (a 2-ary userspace `(undecided B C)` under a 1-ary `undecided`
macro is an arity collision at expansion). `pending` is taken by
`examples/branching/14`; `owe`, `due`, `must`, `need`, `unknown`,
`undetermined`, `incomplete`, `missing`, `debt`, `unmet` are free
corpus-wide.

**For the probe** — the name must mean *neither asserted nor negated*:

| candidate | the case |
|---|---|
| `open` (keep) | zero migration, and the note's own fact-state word |
| `unknown` | Kleene's third truth value — the standard name for exactly this |
| `undetermined` | the stdlib's own phrase ("saturation-determined"), negated |
| `undecided` | the suite's word for it — and blocked, above |
| ~~`free`~~ | the note itself warns «свободные слоты» conflates *may appear* with *must appear* |
| ~~`possible`~~ | that is `absent (not P)` alone — one conjunct of two |
| ~~`pending`~~ | taken; and obligation-flavoured, re-inviting the confusion from the other side |

**For the verdict** — the name must mean *this state owes*:

| candidate | the case |
|---|---|
| `owe` | the phase's own prose ("what it still owes"); a verb that takes the slot — `(owe co-loc House_1)`; `due` / `debt` are its siblings |
| `must` | the obligation said as itself, deontic; free since B dissolved into G's tally |
| `incomplete` | [`ideas.md`](../ideas.md)'s outcome word and Q-M1d.6 candidate (c); maximal `(false)` symmetry — but an adjective that carries no slot |
| ~~`open`~~ | the two-notions point above — though pair P3 keeps it by paying elsewhere |
| ~~`unknown`~~ | SMT's word for *gave up*; this state is not unknown, it is known-unfinished |
| ~~`goal`~~ | the query keyword |
| ~~`missing`~~ | absence is not debt — a fact can be missing and owed by nobody |

**The pairs that survive**, each a coherent vocabulary:

| pair | probe | verdict | migration | the argument |
|---|---|---|---|---|
| **P1** | `open` (keep) | **`owe`** | **zero** — one reserved-name row | fact-states stay the note's; debt words match the ledger/discharge mechanism; the slot argument is natural |
| **P2** | `open` (keep) | `incomplete` | zero | verdict-adjective symmetry with `(false)`; but instances want a slot and an adjective holds none |
| **P3** | `unknown` | `open` | 12 programs + macro + manifest + docs | the proposal's original spelling for the KB state; costs the note's fact-word and re-invites the two-opens confusion |
| **P4** | `unknown` or `undetermined` | `incomplete` | the same 12 — and bare `open` becomes a **free userspace name**, the `closed` → `__closed__` precedent ([`06_reserved_names.md`](../../../docs/kernel/ir/03-ein-lang/06_reserved_names.md)) | the clean slate: standard words both sides, the overloaded word evacuated |
| **P5** | `open` | `open`, position-split | zero files, one fragile expander rule | dual positions get dual *words* elsewhere on this page — `forall` (match) / `at-least` (assert) is D's own precedent — and the fact/KB conflation stays; still rejected on sight |

One decoupling softens P2's loss and P4's cost: **the atom's name need not be
the printed verdict's.** Rules assert instances; the engine prints the
aggregate — `(owe …)` atoms under an `Incomplete (owes 46)` verdict line are
one design, not two. So `ideas.md`'s outcome word is available downstream of
*any* pair, which removes the strongest argument for spending the atom on an
adjective.

## 4. The two axes

The seven are not seven independent choices. They separate cleanly:

**Axis 1 — the surface** (how an author writes one): A (declarator metadata),
B (a bare reserved fact), C (a new head), D (an assert form), F (an `hrule`
clause), G (a verdict atom in `:assert`, derivable only by a rule).

**Axis 2 — the payload** (what the engine holds for an *open* one): a
**counter** over a re-queried candidate set (B/C/D), a **clause** over a
materialised one (E), or **nothing** — a per-quiescence tally that re-derives
instead of persisting (G).

E is therefore orthogonal: it is an answer to axis 2 that any of B/C/D can
carry. A is orthogonal again — it is sugar, and it can desugar to whatever
the other two axes settle on. And G is the corner of axis 2 that needs no
axis-1 carrier under it: the tally has no fact to ride on, which is what the
other six pay for somewhere.

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
| **G** `:assert open` verdict | **none** (one reserved atom) | **only** a rule can | yes — φ is the rule's match | yes (∀-side; the witness side names no domain) | recomputed — `alive ∩ slot` | as pairings, not numerals (§ G) | **per-KB by construction** |

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

**Where G lands in that stack** (added 2026-08-24, with the form; revised the
same day): it takes the **verdict stratum** away from B — the tally needs no
carrier fact — and the revision then empties B's residue too: `L ≥ 2` / `U`,
the one claim B still held, is mis-posed as numerals (§ G — Cardinality
without numerals) and decomposes into pairing relations if a corpus entry
ever asks. The composed shape becomes **G-report first** (the three-state
read-out and the outstanding-obligations report, as an `--events` / summary
line, no verdict word moved), then **G-generate behind the
[Q-M1d.4](../open_questions.md#q-m1d4--may-an-obligation-driven-generator-change-the-traversal)
decision** — the supersession ladder, `:hrules` as override, obligations as
the default hypothesis source — with **E as the branch representation when
S1d.2.5 wants it materialised**, **D as the surface**, A as sugar. G's
openness census is the number that says whether E's machinery is needed at
all, and the verdict word itself moves only when
[Q-M1d.6](../open_questions.md#q-m1d6--may-contradiction-be-said-with-exhausted--false)
is decided, never as a side effect.

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

**G reads the table the other way**: the six stay as written and gain two
duals (`total-owed` / `surjective-owed`) — the middle *word* added without
the endpoint *mechanisms* absorbed. Additive and reversible where the
collapse is neither; if S1d.2.1 finds the six less redundant than claimed, G
is the one form that does not care.

## 9. What this leaves for the stage files

- **S1d.2.1** — the audit, which decides whether §8's table is true.
- **S1d.2.2** — the domain: what closes it, and whether `?isa` is enough.
- **S1d.2.3** — this decision, plus the sub-decisions E raises (is a stored
  clause a fact or a kernel object?) and G raises (the naming menu — § G,
  pairs P1–P5; the slot spelling).
- **S1d.2.4** — the saturator: invalidation, and the per-quiescence cost §1
  warns about.
- **S1d.2.5** — hypotheses from obligations: the supersession ladder
  (`:hrules` → obligations → blind), its completeness condition, and
  Q-M1a.18's shape.
- **S1d.2.6** — verdicts, counters, corpus — and G's openness census: the
  tally per entry, and which of Q-M1d.6's ten it reads as *incomplete*
  versus *vacuously satisfied*.
