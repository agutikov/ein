# Definitions — from a fact to a region

The user asked for this table on 2026-10-16. It has four columns:

- **what it is**: the definition this phase works to.
- **example**: taken from
  [`07_two_pairs_two_tables.ein`](../../../examples/lattice/07_two_pairs_two_tables.ein)
  where it can be. That program has two pairs of friends, (Ann, Bob) and
  (Cat, Dan), and two tables of two seats, T1 = {S1, S2} and T2 = {S3, S4}.
- **status**:
  - **engine**: the shipped engine does this today.
  - **ruled**: the user has ruled it, and the engine does not do it yet.
  - **proposed**: the plan proposes it, and the user has not ruled.
- **source**: where the term is defined or ruled.

**Scope.** The rows are written for a **monotone** program, which is what
P1e2.1a solves. Where a definition changes for a non-monotone program, the row
says so and points at
[P1e2.1b](../p1e2.1b_the_non_monotone_lattice/README.md).

**Moved 2026-10-16.** S1e2.1a.3 rewrote
[`solution_semantics.md`](../../../docs/kernel/inference/solution_semantics.md)
from this table. The page is the definition now, and this file is the plan's
record of where each term came from and what its status was.

## Context and problem

Added at the user's request on 2026-10-16. The user's words:

> theory = relations and their properties defined by rules, ontology = simple
> facts like (is-a Tee Drink) required for solution but not stated in the
> problem because it is obvious for humans living on Earth; theory + ontology
> = context; problem = facts + query + hypothesis hints, in context

The examples in this section come from
[`examples/zebra2.ein`](../../../examples/zebra2.ein) as well as the seating
programs, because zebra2 already marks the boundary informally. Its clue facts
carry `:source "condition (n)"`, the numbered sentences of the puzzle. Its
`(is-a Tea Drink)` carries none, because no sentence of the puzzle says it.

| term | what it is | example | status | source |
|---|---|---|---|---|
| **theory** | relations and their properties, defined by rules: the relation declarations, the property rules, the declarations that give a relation a property, and the rules that define what a relation means | `(relation drink-loc Drink House)`; `(bijective drink-loc)`, `(symmetric next-to)` and the stdlib rules behind them; zebra2's `co-located` and `adjacent-via` rules; the seating programs' `(functional sits)`, `(injective sits)` | ruled 2026-10-16 (vocabulary) | the user |
| **ontology** | simple facts the solution needs that the problem does not state, because they are obvious to people living on Earth | `(is-a Tea Drink)`, `(is-a Red Color)`, `(is-a Drink Attribute)` | ruled 2026-10-16 (vocabulary) | the user |
| **context** | theory + ontology | everything in zebra2 that carries no `:source` and is not a hint or the query | ruled 2026-10-16 (vocabulary) | the user |
| **problem** | facts + query + hypothesis hints, stated **in a context** | zebra2's `(co-located nation-loc Englishman color-loc Red :source "condition (2)")`, its `(query :goal …)` and its `:hrules (guess …)` | ruled 2026-10-16 (vocabulary) | the user |
| **problem fact** (clue) | a fact the problem states | `condition (2)`, above; in the seating program, `(member Ann Pair)`, `(at S1 Small)`, `(avoids-ac Eve)` | ruled (the *facts* of *problem*) | the user |
| **hypothesis hint** | what the problem says may be guessed: an `hrule` and its activators, and `:no-hypothesis` | zebra2's `(guess (drink-loc Drink House) …)`; the seating program's `seat` | ruled (the *hints* of *problem*); under (b) they move to `(solve …)` | the user; (b) |
| **stdlib** | the theory every program may share, imported by name | `std.algebra`, `std.bijection` | engine | `stdlib/` |
| **program** | context + problem, written as Ein text. Its imports bring theory in from the stdlib | one `.ein` file and its imports | engine. **The boundary is not in the language**: one file mixes all four, and nothing but zebra2's `:source` comments marks which part is which | — |

**What the split is for.**

- **R is a property of a problem in a context.** The same problem in another
  context has another region. *Improving the theory* and *searching for a
  theory that gives a single solution* (the user, quoted in
  [S1e2.1b.6](../p1e2.1b_the_non_monotone_lattice/s1e2.1b.6_the_two_lattices_compared.md))
  hold the problem fixed and vary the context.
- **It is the split M2's NL → IR has to make.** The problem comes from the
  text. The ontology comes from world knowledge, which is exactly what a text
  leaves out. The theory comes from the stdlib or is written once per
  domain.

**The pattern, and its one edge.**

- **Every clue in both zebra files is a fact.** Each is `co-located`,
  `adjacent-via` or `right-of` with a `:source`, and the rules that give those
  relations their meaning are theory.
- **The seating programs follow the same pattern.** `together` and
  `air-conditioner` are general rules: any group, and any guest who avoids a
  seat facing the air-conditioner. The problem states only `(member …)`,
  `(avoids-ac Eve)` and `(faces-ac B1)`.
- **The edge is a rule that names a constant of the problem**, such as a rule
  about Eve alone. It is problem in substance and rule in form. The table does
  not yet say which side it is on, and that matters when the boundary gets
  into the language.

## The program

| term | what it is | example | status | source |
|---|---|---|---|---|
| **name** | a constant the program mentions anywhere: in a fact, **or in a rule**, since rules are part of the program. Ein never makes one at run time, so the lattice is finite | `Ann`, `S1`, `Friend` | ruled: *"we don't emit new names"* and *"rules are part of root, so rules can't assert new names"* (2026-10-16). Today the candidate list only draws on the names in facts, which is W4 | Q-M1e2.7 (h), third answer |
| **type** | a name used as the second argument of `is-a` | `(is-a Ann Friend)` | engine | `01_grammar.md` |
| **relation** | a declared predicate with a signature | `(relation sits Friend Seat)` | engine | `01_grammar.md` |
| **fact** | a ground atom. It is positive unless written `(not …)` | `(sits Ann S1)` | engine | `02-data-model/` |
| **stored negative** | a `(not …)` fact held in a KB. A program, a rule, or the search can write one. A rule that matches `(not …)` reads it | `(not (sits Bob S1))` | engine | Q-M1e.7 |
| **`absent`** | negation as failure: `(absent P)` holds while P is not in the KB | `total-owed`'s witness test | engine | `absent_semantics.md` |
| **rule** | a saturation rule: `:match` → `:assert` | `together` | engine | `01_grammar.md` |
| **restriction** | a rule that asserts `(false)`. It sets the maximal line | `together`, `functional`, `injective` | engine | Q-M1e2.7 |
| **obligation** | a rule that asserts `(open ?R)`. It states a debt and sets the minimal line. An **owed instance** is one binding of it that has no witness | `(total-owed sits is-a)`: every friend owes a seat | engine | M1d S1d.2.4 |
| **hrule** | a hypothesis generator. It proposes candidates and never asserts them | `seat` proposes `(sits ?p ?s)` | engine; under (b) it moves to `(solve …)` | Q-M1e2.7 (b) |
| **query** | today: `(query :goal … :hrules … :expect …)`, which is solved once per query. Under (b): a question about the region, asked after the program is solved | `:goal (sits ?p ?s)` | engine; ruled to change | (b), (c) |

## The knowledge base

| term | what it is | example | status | source |
|---|---|---|---|---|
| **KB** | the fact store: facts with provenance, layered copy-on-write, so that a fork shares its parent | — | engine | `02-data-model/` |
| **saturation** `sat(K)` | firing rules until nothing new is derived (quiescence). It never stops at the first contradiction | — | engine | `inference/` |
| **root** | `sat(the program's facts)`. Nothing is hypothesised at root. Root is node ∅ and is never written by the search | 4 friends, 4 seats, no `sits` | engine. *Never written* is proposed: today the forced positive and the search's negatives write into root | Q-M1e2.7, third answer |
| **state** | the saturated KB of a commitment C, `sat(root + C)`. It is built by forking root, and from scratch it is the same KB for a monotone program | — | engine | S1e2.1a.1. Fork against scratch for a non-monotone program is P1e2.1b's S1e2.1b.2 |
| **dead** | the state holds `(false)` | `{(sits Ann S1), (sits Cat S1)}` | engine | — |
| **alive** | not dead | `{(sits Ann S1)}` | engine | — |
| **owing n** | the state has n owed instances. The oracle also reads debts on dead states | `{(sits Ann S1)}` owes 3 | engine (the oracle) | S1e2.1a.1 |
| **discharged** | owes nothing | a full seating | engine | — |

## The lattice

| term | what it is | example | status | source |
|---|---|---|---|---|
| **candidate** (hypothesis) | a fact the generator proposes at root, with no config switch applied: no lookahead and no forced positive | `(sits Ann S3)` | engine (the oracle's list) | S1e2.1a.1 |
| **universe** H | the candidate list, with n = \|H\|. It must contain every fact any model needs. If it does not, the program is wrong (a missing name) or the engine is (a missing candidate) | n = 16 | ruled 2026-10-16 | (h) |
| **commitment** C | a subset of H | `{(sits Ann S1), (sits Bob S2)}` | engine | — |
| **node** | a commitment, seen as a point of the cube | — | engine | — |
| **cube** (lattice) | all 2ⁿ commitments ordered by ⊆. Layer L*k* holds the commitments of k facts. An **edge** adds one candidate | 2¹⁶ = 65 536 nodes | engine (the oracle) | Q-M1e2.7 |
| **closure** cl(C) | the candidates `sat(root + C)` holds. Always C ⊆ cl(C) | here cl(C) = C everywhere | engine (the oracle) | S1e2.1a.2 |
| **transient node** | a node with cl(C) ≠ C: saturation moves it. The `sat` arrow is C ⇒ cl(C) | `synth/merged_states`: {A} ⇒ {A,B} | engine (the drawing) | S1e2.1a.2 |
| **state class** | the commitments that share one state. For a monotone program its **top** is the closed set cl(C), and its **bottoms** are the **minimal generators**: the least that must be guessed. The class is the union of the intervals [generator … top] | `synth/merged_states`: {A} and {A,B} are one state | ruled (third answer): shown as `(region (model …) …)` with `(merged (to from) …)` | (e) |
| **monotone** (program) | no read of the third kind, so adding a candidate never takes a conclusion away and never adds a debt. Then dead and discharged are both upward-closed, R is convex, and cl is a closure operator | both seating programs | engine (the static check) | `monotone.rs`; [S1e2.1b.6](../p1e2.1b_the_non_monotone_lattice/s1e2.1b.6_the_two_lattices_compared.md) for the other kind |

## The answer

| term | what it is | example | status | source |
|---|---|---|---|---|
| **solution** | a node of R. **∅ is a solution exactly when root is alive and discharged, whatever the generator proposes**, including when it proposes nothing | the 8 seatings; ∅ here owes 4 | ruled 2026-10-15 and 2026-10-16 | Q-M1e2.7 |
| **region** R | every solution: `{C : C alive and discharged}`. In general R is its whole node set, simplified only where the case allows or a query asks | R = 8 nodes, all at L4 | ruled | Q-M1e2.7 |
| **k_c** | \|R\| counted in **commitments** | 8 | ruled 2026-10-16: it replaces `k`. Today's `k` counts recorded nodes | (a), (e) |
| **k_s** | \|R\| counted in **states** (state classes) | 8. They differ on 13 corpus regions, up to 32 768 against 728 ([region_census.md § 6](region_census.md#6-commitments-against-states-ruling-a--e)) | ruled 2026-10-16: it replaces `k` | (a), (e) |
| **minimal line** / **maximal line** | R's minimal elements / R's maximal elements | both are the 8 seatings | engine (the oracle) | Q-M1e2.7 |
| **trivial line** | in a monotone program: the minimal line is {∅} when it has restrictions only, and the maximal line is {⊤} (every candidate) when it has obligations only | `synth/trivial_line` | ruled for monotone (2026-10-16) | (f) |
| **convex** | R has no **hole**, that is, no non-member between two members. Then R = ⋃ [a … b] over its lines. A monotone program is always convex | both seating programs | engine (the oracle counts holes) | Q-M1e2.7 |
| **sub-cube** [a … b] | every commitment between a and b. Its **dimension** \|b\| − \|a\| is the number of free facts | each seating is [s … s], dimension 0 | engine (the oracle) | Q-M1e2.7 |
| **component** | a maximal part of R connected by one-fact edges inside R. **It is not a model.** It is a set of models | 8: moving one friend passes through a dead or owing node | engine (the oracle) | (i) |
| **model** | what a solution adds: the positive part of its state minus root's positive facts. *Models are models* | `{(sits Ann S1), (sits Bob S2), (sits Cat S3), (sits Dan S4)}` | engine (the read-out) | Q-M1e.7 ruling A; (i) |
| **projection** | a model cut down to the rows of a query's goal | the goal `(sits ?p ?s)` sees all 4 rows | engine | — |
| **projection class** | the models with one projection, which is what a query answers; models that differ only in hidden facts fall into one class | grouped by which table each pair took: **2** classes of 4 | ruled 2026-10-16 as the name for the user's grouping. No surface computes it yet | (i), (c) |
| **N** / **E** | the minimal dead sets / the minimal discharging sets. For a monotone program the lines follow from them (the duality) | `06_friends_tables`: 68 / 675 | engine (the oracle) | Q-M1e2.7 |
| **certificate** | evidence that a claimed R is R, cheaper to check than recomputing it. For a monotone program: N, E and the static verdict | — | proposed | (g) |
| **exhausted** | the reported region is all of R. A run limited by layers, enterings or time reports a lower bound | — | ruled for monotone. Today it certifies the walk (Q-M1e.8) | (g) |

## The search, and what it is not

| term | what it is | status | source |
|---|---|---|---|
| **oracle** | R computed by saturating all 2ⁿ nodes, with no search machinery: `ein_infer::region`, `--dump-region`, n ≤ 16 by default | engine | S1e2.1a.1 |
| **walk** | what a search run did: its enterings, its recorded nodes, its learned clauses and its root writes | engine (`walk.json`) | S1e2.1a.1 |
| **entering** | one commitment the search forked and saturated | engine | — |
| **recorded node** | a model or an open state the walk reported | engine | — |
| **open state** | a state the walk recorded as alive and owing (*not yet a model*). Some of them can never be finished. On `07_two_pairs_two_tables` it records 16 of these at L2, such as `{(sits Ann S2), (sits Cat S1)}`, where Bob can no longer sit with Ann | engine | M1d S1d.2.6 |
| **complete** (today) | *the active rung proposes nothing*. That is a property of the generator, which is why today's search looks for the maximal line, and why ∅ is never recorded while any candidate exists. Under R it is replaced by **discharged** | engine; ruled to go | (b), [Q-M1e2.8](../open_questions.md#q-m1e28--where-does-the-rework-of-ein-solve-go) |
| **learned nogood** (clause) | a commitment the search marks dead together with all its supersets | engine | — |
| **kill cache**, **singleton writeback** | the stored negatives `(not h)` the search writes after it refutes {h} | engine | Q-M1e.7 |
| **forced positive** | when exactly one candidate is still alive at root, the search writes it **into root** as a fact and re-saturates. It is not entered as a commitment. Every later node holds it, and ∅ drops off the walk. In a monotone program it is never needed: all but h dead alone means the alive nodes are at most ∅ and {h} | engine (`:enable-forced-positive`); to go, with every root write | W2; Q-M1e2.7, third answer |
| **lever** / **config** | any option. It may change the speed and never R | ruled 2026-10-15 | config principle |
| **budget** | `-m`, `-E`, `-T`. It stops the walk, and the answer is then partial (not exhausted) | ruled 2026-10-16 | (g) |
| **expectation** | what a program says its answer is. Today: `:expect`. Under (c): `(solution …)` for the region, and `(query … :expect …)` over projections | engine; ruled to change | (c) |
| **verdict words** | `Solution` / `Ambiguity` / `Contradiction` / `Open`. Under (i), `Ambiguity` and `Contradiction` judge a **stated expectation** of exactly one solution, or of some solution. Without an expectation, `solve` describes R | engine; ruled to change | (i) |
