# S1d.10.1 — Why it does not finish

**Phase:** P1d.10 (Exhaustive search over many models)
**Estimate:** 3 days

## Context

The instrument stage, and it comes first for the reason
[F9](../../followups/f9_e_catalog.md) gives: a search proposal judged against
the wrong regime is judged wrong. What is known is one puzzle's numbers; what
is needed is the shape of the regime.

The one mechanism already identified: **a layer that kills nothing learns
nothing**. Pruning in this engine comes from deaths — the learned clause and,
for a singleton commitment, the `(not h)` writeback. Layer 1 of
`zebra2-minus-15` has 96 candidates and 0 deaths, so the next layer is the
full `C(96,2)` with no clause to filter it. On zebra2, layer 1 kills 67 of 101
and that is what makes it tractable.

So the question is not "why is it slow" but **"what does a layer have to
produce for the next one to be affordable, and what happens when it produces
none of it?"**

## Acceptance

- A **clause-yield census**, per layer, per corpus entry: candidates entered,
  deaths, clauses emitted, clauses subsumed, writebacks, and **how many
  candidates the next layer's generation was filtered by them**. That last
  column is the one nothing currently reports, and it is the phase's core
  measurement.
- The corpus split into regimes by that census — not by hand. The expectation
  is at least two (determinate, under-determined) and possibly a third
  (deep-but-pruning: `branching/07 -e` is 11 501 enterings over 5 layers and
  finishes in a second).
- **Growth rate per layer** and how much of it the clause store removes. On
  zebra2-minus-15 the answer at layer 2 appears to be "nothing"; that should be
  a number.
- The memory profile alongside the time, because
  [baseline.md §15](../../../docs/history/m1a_rust/measurements/baseline.md) says the wall is likely
  to be RAM.

## Tasks

### Task T1d.10.1.1 — The census instrument

Per-layer counters exist in part (`nogoods_emitted`, `nogoods_subsumed`,
`enterings_*`); what is missing is the *effect*: how many candidates
`generate_layer` produced, and how many `filter_candidate` rejected against
the store. Add those, behind the same discipline
[`ein_core::counters`](../../../ein.rs/crates/ein-core/src/counters.rs) uses —
compiled out unless asked, because this is the hottest loop in the engine.

### Task T1d.10.1.2 — The corpus census

Run it over every entry, depth-capped where the entry does not finish. Report
the table.

### Task T1d.10.1.3 — Classify the regimes

From the census, not from intuition. The classifier should be something a
reader can apply to a new puzzle — "deaths per entering below x at layer 1" —
because [S1d.10.5](s1d.10.5_contract.md) may want to *report* it.

### Task T1d.10.1.4 — Where the time goes in this regime

A profile of `zebra2-minus-15 -m 3`, bucketed like
[`profile_ein_rs.py`](../../../utils/profile_ein_rs.py) does, against the
determinate profile in [baseline.md §3](../../../docs/history/m1a_rust/measurements/baseline.md). The
question is whether the under-determined regime is the *same* engine costs at
a larger count or a different mix — if `generate_layer` and `filter_candidate`
dominate where the determinate profile has the matcher and the boundary, the
optimisation targets are different ones.

## What the census found — **taken 2026-08-24**, [`layer_census.md`](layer_census.md)

**Done.** T1d.10.1.1 … T1d.10.1.4, 180 entries, 360 child processes.

- **The powerset is not a metaphor.** For **25 of the 49** corpus entries that
  search at all, `entered` is exactly `Σₖ C(alive, k)` — `C(35, 1..5) =
  384 167` for `features/01_not_and_absent`, `C(153, 1..3) = 597 057` for
  `square-unique/terminus`. Those 25 cells hold **96.7 %** of the corpus's
  search work. Nothing died, so nothing was learned, so nothing was filtered.
- **Two regimes, and F9 measured the smaller one.** Layer 1 kills something in
  **4 of 49** cells — `zebra`, `zebra2`, `zebra2-hints`, `branching/07`, i.e.
  exactly the puzzles the search was tuned on. The other **45** are barren.
- **The corpus-wide filter split**: of 2 232 330 joined candidates, **0** were
  dropped for a dead element and **31 303 (1.4 %)** by a learned clause. The
  first is [structural](layer_census.md#6-one-of-the-two-filter-arms-cannot-fire)
  — the inter-layer retain gets there first, so **the clause store is the only
  thing that can shrink a layer** — and it is now a test.
- **The premise needs one correction.** Layer 2's filter rate is 0 %, but not
  because the store is inert: every layer-1 clause is a singleton and the
  writeback already removed its element, so layer 3 is the store's *first
  possible* contribution. On `zebra2-minus-15` it is **26.8 %** there and
  **36.2 %** at layer 4 — rising, while the layer's growth decelerates
  `47.5× → 13.2× → 4.1×`.
- **A budget is now a probe, and the run that "does not finish" does.** The
  census row is emitted on every way out of a layer, so `solve -e -m 4 -E 48746`
  *generates* layer 4 and reports it without entering it — 245 612 joined,
  88 887 dropped, **156 725 candidates**; the same trick gave layer 5 at
  **412 606**. The five columns sum to 618 076, and a full `solve -e` then
  entered **exactly** that, in **416 s**, with all 32 models and
  `exhausted=false`. "Killed at 30 min" (2026-08-20) is a record of that
  session, not of this engine.
- **The lattice is 1.2 % of the run** (T1d.10.1.4). Bucketed against
  [baseline.md §3](../../../docs/history/m1a_rust/measurements/baseline.md)'s
  determinate profile, the under-determined regime is the *same* mix at a larger
  count — match/bind 47.7 %, saturate 40.7 %, `apriori/elim` **1.2 %**. So under
  [F9](../../followups/f9_e_catalog.md)'s discipline a whole class of proposal
  is ruled out before it is written: **making the lattice cheaper cannot help,
  because the lattice is not the cost.** The only lever is entering fewer
  commitments.

### What it hands forward

- **[S1d.10.2](s1d.10.2_depth_required.md)** gets `d_found`/`d_stop` for every
  cell, and **T1d.10.2.4 answered**: `type-exclusivity/{colors,nationalities}`
  find one model at depth 4 and **four more at depth 5**, so `-m 5` is not a
  default doing nothing, and lowering it would change answers.
- **[S1d.10.3](s1d.10.3_stopping_criterion.md)**'s candidate (b) is **dead
  before it is designed**, exactly as its own text predicted: `alive` shrinks in
  **3 of 46** multi-layer cells and never once in the barren regime. Its (a) and
  its "coverage of the residual lattice" note are untouched — and the number
  that sharpens them is 11 577 clauses buying 24.9 %.
- **[S1d.10.4](s1d.10.4_conflict_mining.md)** gets its trigger's operand
  measured, and a correction to its shape: "barren" is better read as *this
  layer's join was filtered by nothing* than as *this layer had no deaths* —
  the two differ by exactly the structural layer-2 zero above.
- **[P1d.2](../p1d.2_obligations/README.md)** gets §3 as evidence: 25 cells
  where the engine proposes every subset of the open arrows in turn because a
  fixpoint that is *incomplete* has no other vocabulary.

## Notes

- Resist proposing anything in this stage. Its output is a table and a
  classification; the proposals are [S1d.10.3](s1d.10.3_stopping_criterion.md)
  and [S1d.10.4](s1d.10.4_conflict_mining.md), and both are better arguments
  for having this first.
