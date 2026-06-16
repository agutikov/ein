# Domain-elimination rule vs explicit hypothesis exploration

> **S1.5b.32 T32.2 / T32.3.** The same positive answer (e.g.
> `(color-loc Blue H1)`) is reachable two ways under the unified
> engine: **(A)** a `domain-elimination` saturation rule that fires
> at d=0 once every alternative is excluded, or **(B)** the
> hypothesis loop forking on each candidate and refuting the wrong
> ones. This page measures both with the single
> [`solve`](README.md#set-indexed-search--monotonic-engine-p15b-s15b0-10)
> engine — the verdict read off `k` (0/1/>1 → contradiction / solution /
> gaps), and the two **views** (`proof.solutions` / `proof.dead_commitments`)
> read off the `store_lattice` proof of one exhaustive run — and records
> the audit conclusion + recommendation the stage owed.
>
> **Historical numbers.** The tables below were measured (S1.5b.32, mid-2025)
> through the engine's then *three public entries* `solve` / `gaps_solve` /
> `contradictions_solve` — the last two were removed 2026-06-16 when the
> verdict became result-driven, so the measurement harness that produced the
> tables below (`utils/s1_5b_32_measure.py`, since removed) is a period
> record, not a live one. The work
> counts — *how far the lattice is walked* per fixture — are what the
> A-vs-B comparison turns on and are unchanged by the merge; the per-entry
> `verdict` columns simply pre-date `verdict_of` reading the one verdict
> from `k` (so the duplicate `gaps`/`contradictions` rows on a fixture are
> now two views of that fixture's single exhaustive solve). Re-running over
> the [`examples/domain_elim/`](../../../examples/domain_elim/) fixtures
> today means `solve(..., stop_after=N, store_lattice=True)` and reading
> the views off the proof. Long-form stage notes:
> [`s1.5b.32_domain_elim_vs_hyp_exploration.md`](../../../plans/m1_core_graph_reasoning/p1.5b_lattice_search/s1.5b.32_domain_elim_vs_hyp_exploration.md).

## The fixtures

A minimal exactly-one puzzle: one bijective relation `color-loc`
between `Color = {Red, Green, Blue}` and `House = {H1, H2, H3}`, two
anchor facts (`Red@H2`, `Green@H3`). By injective-completion Blue is
excluded from H2 and H3, leaving exactly one house — H1. Goal
`(color-loc Blue ?h)`; answer `?h = H1`.

The three fixtures **hold the hypothesis generator constant**
(`:hrules (guess (color-loc Color House))`) and vary only the rule
library — so each comparison moves a single variable:

| fixture | elimination rules (A) | negative-completion | what *can* solve it |
|---------|:---:|:---:|---------------------|
| [`ab.ein`](../../../examples/domain_elim/ab.ein) | ✓ | ✓ | pathway A at root saturation |
| [`b_only.ein`](../../../examples/domain_elim/b_only.ein) | — | ✓ | forced-positive promotion |
| [`b_branch.ein`](../../../examples/domain_elim/b_branch.ein) | — | — | real forking + refutation |

> **Fixture gotcha worth remembering.** A parameterised rule
> `(rule R-name (?R) …)` only fires for relations it is *activated*
> for: it needs an activator fact in `_rule_apps_by_rule[R-name]`,
> produced by a `derive-…` meta-rule (e.g. `(functional ?R) ⟹
> (functional-negative ?R)`). A first draft of these fixtures
> omitted `derive-functional-negative` / `derive-injective-negative`;
> the negative-completion rules never activated, pathway A was
> silently starved of its fuel, and all three fixtures collapsed to
> the same B-style solve. The audit table below is the guard
> against that class of mistake — it checks A *actually fires*.

## A. Pure-saturation audit (T32.3 — does pathway A fire?)

A plain `Saturator(kb).saturate()` — no solver loop, no branching:

| fixture | firings | neg-completion firings | elim (A) firings | answer @ root saturation |
|---------|--------:|-----------------------:|-----------------:|--------------------------|
| `ab` | 26 | 12 | 6 | **`rule:range-elimination`** |
| `b_only` | 14 | 8 | 0 | ABSENT |
| `b_branch` | 4 | 0 | 0 | ABSENT |

**Conclusion:** pathway A fires correctly inside per-set / root
saturation — `ab` derives `(color-loc Blue H1)` at d=0 from the
negatives, no branching, with provenance `range-elimination`. No
commitment-set spec change is needed (the open worry in
[T32.3](../../../plans/m1_core_graph_reasoning/p1.5b_lattice_search/s1.5b.32_domain_elim_vs_hyp_exploration.md)
about A failing to observe the integrated negatives does not
materialise). `b_only` derives the negatives but, lacking the
elimination rule, cannot assert the positive; `b_branch` derives
nothing — both need the hypothesis machinery.

## B. Default config (shipped — pre-branch lookahead ON)

The `run` column names the historical entry the row was measured
through (the single-solution `solve`, or the exhaustive sweep read as
the `gaps` / `contradictions` *view*); `verdict` is that entry's
*old per-entry* verdict. Under the unified `solve`, the verdict is
read from `k` instead, so every fixture here resolves to one verdict —
all three have a model, so all three are **Solution** (`k=1`); the
`Ambiguity` / `Contradiction` cells below are the removed entries'
forced labels, kept to show the work each *view* did, not a verdict
`solve` would now return.

| fixture | run (historical entry) | verdict (pre-merge) | ent | dead | fp | sat | sets | answer-prov |
|---------|------------------------|---------------------|----:|-----:|---:|----:|-----:|-------------|
| `ab` | single (`solve`) | Solution | **0** | 0 | **0** | **1** | — | `range-elimination` |
| `ab` | gaps view | Ambiguity | 0 | 0 | 0 | 1 | **0** | — |
| `ab` | contradictions view | Contradiction | 0 | 0 | 0 | 1 | **0** | — |
| `b_only` | single (`solve`) | Solution | 0 | 0 | 1 | 2 | — | `<forced-positive>` |
| `b_only` | contradictions view | Contradiction | 1 | 0 | 1 | 2 | 1 | — |
| `b_branch` | single (`solve`) | Solution | 0 | 0 | 1 | 2 | — | `<forced-positive>` |
| `b_branch` | contradictions view | Contradiction | 1 | 0 | 1 | 2 | 1 | — |

**Pathway A is strictly cheaper.** When the elimination rule is
present (`ab`), the answer falls out of root saturation: **0
enterings, 0 forced-positives, 1 saturation pass, and the exhaustive
lattice enumerates 0 sets** — A solves at root and there is nothing
left for the powerset loop to visit. Without A (`b_only` /
`b_branch`), the engine reaches the same answer via **forced-positive
promotion** — the alive-set shrinks to a singleton and is promoted —
at a measurable surcharge: an extra saturation pass, a forced-positive
event, and one lattice `SetNode`.

So pathway A **fully preempts** pathway B: there is no double-counting
(the stage's sub-question 1), because A leaves the lattice empty.

## C. The real lever — `enable_pre_branch_lookahead`

Toggling the [pre-branch lookahead](../../../ein.py/src/ein/inference/config.py)
on the exhaustive sweep (`solve(..., stop_after=None, store_lattice=True)`,
read as the contradictions view) isolates what is doing
the elimination when A is absent:

| config | fixture | ent | dead | fp | sets | nogoods |
|--------|---------|----:|-----:|---:|-----:|--------:|
| lookahead **ON** | `ab` | 0 | 0 | 0 | 0 | 0 |
| lookahead **ON** | `b_only` | 1 | 0 | 1 | 1 | 0 |
| lookahead **ON** | `b_branch` | 1 | 0 | 1 | 1 | 0 |
| lookahead **OFF** | `ab` | 0 | 0 | 0 | **0** | 0 |
| lookahead **OFF** | `b_only` | 1 | 0 | 1 | 1 | 0 |
| lookahead **OFF** | `b_branch` | **7** | **6** | 0 | **7** | **6** |

Three regimes emerge:

1. **`ab` solves at root regardless of lookahead** (0 sets in both
   rows). Pathway A is robust — it does not depend on the
   control-loop's speculative refutation.
2. **`b_only` rides negative-completion → forced-positive** (1 set in
   both rows), independent of lookahead: the authored
   `functional-/injective-negative` rules supply the negatives, the
   alive-set collapses to a singleton, done.
3. **`b_branch` depends entirely on lookahead.** With it on, the
   lookahead refutes the wrong candidates cheaply and feeds
   forced-positive (1 set, no recorded forks). With it **off**, the
   engine finally pays the genuine **pathway-B cost the stage
   describes: 7 commitment sets entered, 6 contradicted, 6 CDCL
   nogoods learned** — N branches with N−1 refutations.

The headline: on this puzzle the engine has **three** elimination
mechanisms — the declarative `domain-elimination` rule (A), the
authored negative-completion + forced-positive promotion (B-side,
procedural), and the pre-branch lookahead + forced-positive (B-side,
speculative). Any one of them reaches d=0; the expensive
fork-and-refute search only surfaces when **all three** are removed.

## D. Answers to the stage's questions

- **Q1 — per-set analog / double-counting?** Pathway B *is* the
  lattice (powerset enumeration is structural). When A fires it
  prunes the lattice to **0 sets** — no double-counting. When A is
  absent the lattice does real work (1 set with a shortcut, 7 sets
  without).
- **Q2 — does A fire under commitment-set semantics?** Yes — §A and
  the `ab` rows of §B/§C show `range-elimination` firing inside
  saturation and solving at root, on the single-solution run and in
  both proof views alike.
- **Q3 — is B's per-set saturation wasted when A would have fired?**
  Yes, relative to A: `b_branch` enters 7 sets where `ab` enters 0.
  But A eliminates that work entirely whenever its rule is in scope,
  so the waste is avoidable, not inherent.
- **Q4 — recommendation:** **Pathway A as default, B as fallback.**
  When the rule library carries `domain-/range-elimination` + the
  negative-completion they consume, A solves at root saturation —
  fewest saturations, no forced-positive round-trip, no lattice
  nodes, robust to config. The engine keeps B as the safety net: with
  the elimination rule absent it still solves (via forced-positive
  promotion, or via real branching when negative-completion is also
  absent), just at higher cost. This matches the
  [d=0 negative-completion](README.md#d0-negative-completion-s15a19)
  design — push elimination into saturation rules and let branching
  catch only what the rules cannot pre-determine.

### Soundness coupling (Q-S1.5b.32.A)

Pathway A is dormant when negative-completion is incomplete (`b_only`
vs a hypothetical "elim rule present but no negatives": A cannot fire
without the negatives in its `forall`). Pathway B is more robust — it
manufactures the negatives by refutation as it forks. This argues for
the static NAF-dependency warning parked at
[S1.7.4](../../../plans/m1_core_graph_reasoning/p1.7_bootstrapping_zebra/s1.7.4_naf_dependency_map.md):
warn at load time when an elimination rule's negatives are not
derivable, so authors know A will silently fall back to B.

### Kernel-minimisation note (F5)

`domain-elimination` **stays an ein-lang stdlib rule**, not kernel
code: the rule shape (`forall alternative excluded ⇒ survivor`)
carries the intent legibly, the measurement shows it is strictly
cheaper than the procedural equivalent when present, and authors opt
in by declaring `(bijective R)`. Feeds
[F5 § kernel minimisation](../../../plans/m1_core_graph_reasoning/followups/f5_rules_as_data.md#kernel-minimisation--which-inference-features-belong-in-ein-lang-vs-kernel-code).

## Generalisation (Q-S1.5b.32.C)

The same A-vs-B duality applies to every "forall ⇒ ∃!" rule —
`range-elimination` (measured here, the dual direction),
`no-room-left`, `cardinality-bound`-style constraints. The conclusion
transfers: a declarative elimination rule, when its premises are
derivable, preempts the hypothesis search at root and prunes the
lattice to nothing; absent it, the engine's forced-positive /
lookahead machinery reaches the same d=0 answer at a small surcharge,
degrading to full branch-and-refute only when every elimination path
is removed.

## Cross-links

- Harness: `utils/s1_5b_32_measure.py` (removed — it drove the now-deleted
  `gaps_solve` / `contradictions_solve`; the numbers above are a period
  record, see the *Historical numbers* note).
- Fixtures: [`examples/domain_elim/`](../../../examples/domain_elim/).
- Stage: [`s1.5b.32_…`](../../../plans/m1_core_graph_reasoning/p1.5b_lattice_search/s1.5b.32_domain_elim_vs_hyp_exploration.md).
- Engine overview + the d=0 rules: [README](README.md#d0-negative-completion-s15a19).
- The lookahead lever: [`SolverConfig.enable_pre_branch_lookahead`](../../../ein.py/src/ein/inference/config.py).
