# P1.9 — Hypothesis-loop follow-ups

**Estimate:** TBD per entry.
**Status:** **placeholder / catalog** — parking lot for the
post-S1.5.4 hypothesis-loop ideas that didn't make the M1
acceptance critical path. Created 2026-05-21 in response to the
S1.5.4 implementation-order call (*"All Open ideas — defer to
P1.9"*).
**Depends on:** [S1.5.4](../p1.5_hypothesis_loop/s1.5.4_hypgen_improvements.md)
ship for the empirical baseline each entry would be measured
against.
**Blocks:** nothing within M1 acceptance. Like
[P1.8](../p1.8_ein_lang_modules/README.md), P1.9 promotes a
catalog entry into a stage file only when a puzzle's signal
justifies the work.

## Scope

The full E1-E20 catalog (closure refinements, conflict-driven
learning, search heuristics, CSP-style pre-processing,
engineering/UX) plus the R1-R4 rejected entries lives here,
moved from S1.5.4 on 2026-05-21. Each entry promotes to a
`s1.9.<n>_<title>.md` stage file when activation criteria below
are met; until then this README is the authoritative spec.

**Legend.**
*Status:* 📅 parked (awaiting activation signal) · ✅ resolved
(closed by shipped work) · ⛔ superseded (overtaken by a later
design; kept for the record) · ❌ rejected for M1 (parked here for
visibility; will not promote without a fundamental scope change).
*Effort:* S (≤ ½ day) · M (1-3 days) · L (> 3 days).
*Value:* H / M / L — node-count reduction on the demo suite,
with the qualifier "after the S1.5.4 ship lands" where relevant.
Most entries are M/L because they compound on top of T1.5.4.1's
headline win and the marginal returns diminish; the H entries
are correctness-bearing (E6) or long-term-structural (E7).

## Stage files

Each E-entry has a per-stage stub file (created 2026-05-24).
The README catalog rows stay authoritative for the catalog
description + effort/value/references; the stub files add a
Tasks / Acceptance / open-questions skeleton.

| Stub file                                                                            | Entry |
|--------------------------------------------------------------------------------------|-------|
| [s1.9.e1_functional_activator.md](s1.9.e1_functional_activator.md)                  | E1    |
| [s1.9.e2_at_most_one.md](s1.9.e2_at_most_one.md)                                    | E2    |
| [s1.9.e3_no_hypotheses.md](s1.9.e3_no_hypotheses.md)                                | E3    |
| [s1.9.e4_symmetry_class.md](s1.9.e4_symmetry_class.md)                              | E4    |
| [s1.9.e5_static_rule_conflict.md](s1.9.e5_static_rule_conflict.md)                  | E5    |
| [s1.9.e6_transitive_premise_walk.md](s1.9.e6_transitive_premise_walk.md)            | E6    |
| [s1.9.e6a_tree_solver_cleanup.md](s1.9.e6a_tree_solver_cleanup.md)                  | E6 prereq |
| [s1.9.e7_learned_clause.md](s1.9.e7_learned_clause.md)                              | E7    |
| [s1.9.e8_watched_fact.md](s1.9.e8_watched_fact.md)                                  | E8    |
| [s1.9.e9_lcv.md](s1.9.e9_lcv.md)                                                    | E9    |
| [s1.9.e10_iterative_deepening.md](s1.9.e10_iterative_deepening.md)                  | E10   |
| [s1.9.e11_goal_driven_filter.md](s1.9.e11_goal_driven_filter.md)                    | E11   |
| [s1.9.e12_informativeness.md](s1.9.e12_informativeness.md)                          | E12   |
| [s1.9.e13_per_hyp_budget.md](s1.9.e13_per_hyp_budget.md)                            | E13   |
| [s1.9.e14_arc_consistency.md](s1.9.e14_arc_consistency.md)                          | E14   |
| [s1.9.e15_path_consistency.md](s1.9.e15_path_consistency.md)                        | E15   |
| [s1.9.e16_lazy_root_alive.md](s1.9.e16_lazy_root_alive.md)                          | E16   |
| [s1.9.e17_branch_budget.md](s1.9.e17_branch_budget.md)                              | E17   |
| [s1.9.e18_rule_applicability_pruning.md](s1.9.e18_rule_applicability_pruning.md)    | E18   |
| [s1.9.e19_unsat_core_min.md](s1.9.e19_unsat_core_min.md)                            | E19   |
| [s1.9.e20_conflict_cache.md](s1.9.e20_conflict_cache.md)                            | E20   |
| [s1.9.e21_solve_vs_prove.md](s1.9.e21_solve_vs_prove.md)                            | E21   |
| [s1.9.e22_alive_hyps_in_state_hash.md](s1.9.e22_alive_hyps_in_state_hash.md)        | E22   |
| [s1.9.e23_prove_speedup.md](s1.9.e23_prove_speedup.md)                              | E23   |
| [s1.9.e24_lattice_perf_optimisations.md](s1.9.e24_lattice_perf_optimisations.md)    | E24   |

The R1-R4 rejected entries stay in the README catalog only.

## Catalog

### Closure refinements (Q-S1.5.4.B descendants)

| ref | idea | mechanism | effort | value | references |
|-----|------|-----------|--------|-------|------------|
| ✅ E1 | `(functional R)` activator       | **resolved by P1.8 stdlib** — `functional`/`injective`/`bijective` ship across `std.algebra`/`std.bijection`/`std.elim`/`std.closure` ([§Resolution](s1.9.e1_functional_activator.md#resolution-2026-06-15)); `single-parent` retired | S | M | DL `funcProp` |
| ✅ E2 | `(at-most-one R slot)` activator | **resolved a different way** — at-most-one = `functional`/`injective` (incl. positional `(functional R 0 1)`) + `std.closure` saturating to `(closed R)`; no dedicated activator needed ([§Resolution](s1.9.e2_at_most_one.md#resolution-2026-06-15)) | M | M | CSP cardinality |
| ✅ E3 | `:no-hypothesis` query key       | **implemented** — query key `:no-hypothesis`, the exclusion dual of the `:hypothesis-relations` whitelist; blind-enumerator-scoped, saturation untouched ([§Implemented](s1.9.e3_no_hypotheses.md#implemented-2026-06-15)) | S | L | engineering convenience |
| ⛔ E4 | `(symmetry-class R T)`           | **superseded 2026-06-15** by the symmetric **D/A/B/C decomposition** (Phase 2a/2b) — the dedicated activator is overtaken: E4(a) gen-time pruning → a user hrule (B); E4(b) uniqueness-up-to-symmetry → the positive mirror (C subsumed by D — stdlib `symmetric` + kernel `__symmetric__`). Residual = *object*-value-symmetry (lex-leader SBP/SBDD, L-effort, unexercised) ([§Superseded](s1.9.e4_symmetry_class.md#superseded-by-the-dabc-decomposition-2026-06-15)) | M | M | CSP value-symmetry breaking ([docs/index/02](../../../docs/index/02-solvers-csp-sat-smt.md)) |
| ⛔ E5 | Static rule-conflict pre-analysis | **reframed 2026-06-15 → rule induction (F4-Q34 / F5 / F7)**: a mutex is a *negative hrule* (zebra2 already ships it as `functional-negative`), so this is `property → negative-companion rule` synthesis (cf. `symmetric → symmetric-negative`), not a hypgen table; only a *dominated* Python-table residual stays in P1.9 ([§Reframed](s1.9.e5_static_rule_conflict.md#reframed-as-rule-induction-2026-06-15)) | M | M | rule-set sufficiency, [F7 §C](../../followups/f7_rule_induction.md) |

### Conflict-driven learning (SAT/CDCL-inspired)

| ref | idea | mechanism | effort | value | references |
|-----|------|-----------|--------|-------|------------|
| ✅ E6 | Transitive premise walk for "unconditional" | **DONE 2026-06-15** — `walk_premises` (set-collecting dual of `reaches`) shipped in `provenance.py`; `store.unsat_core` refactored onto it (parity-verified); the substrate E19 / the trace consume ([§Executed](s1.9.e6_transitive_premise_walk.md#executed-2026-06-15)) | S | **H** (correctness) | — |
| ✅ E7 | Learned-clause from unsat-core   | **largely resolved 2026-06-15** — deriving half ships (`DeadCommitment.unsat_core`, now on E6); pruning half **measured vacuous** (all 49 zebra2 deaths are singletons → nogoods already Apriori-minimal) + unsound-under-NAF → closed; only minimise (E19) remains ([§Resolved](s1.9.e7_learned_clause.md#resolved-2026-06-15)) | L | H (long-term) | CDCL ([docs/index/02](../../../docs/index/02-solvers-csp-sat-smt.md)); ATMS justification cache ([docs/index/09](../../../docs/index/09-cognitive-architectures-neurosymbolic.md)) |
| ⛔ E8 | Watched-fact rule applicability   | **motivation superseded by P1.8a** — delta-driven semi-naive saturation (alpha-memory index + seeded delta join) already kills the re-iterate cost; literal watched-literals judged premature ([§Superseded](s1.9.e8_watched_fact.md#superseded-by-p18a-2026-06-15)) | L | M | DPLL watched literals |

### Search heuristics

| ref | idea | mechanism | effort | value | references |
|-----|------|-----------|--------|-------|------------|
| ❌ E9  | Least-constraining-value (LCV)   | **rejected 2026-06-15** (measured): worst ordering on zebra2 — first completer at rank 35/56 (vs lex 11); completers are heavy pruners, LCV prefers least-pruning ([§Rejected](s1.9.e9_lcv.md#rejected-measured-2026-06-15); `demo/score_hypotheses.py`) | M | M | CSP textbook |
| ⛔ E10 | Iterative deepening              | **inapplicable to the lattice BFS** — cardinality layering *is* breadth-first deepening; no DFS depth bound to re-raise ([§Inapplicable](s1.9.e10_iterative_deepening.md#inapplicable-to-the-lattice-bfs-2026-06-15)) | S | M | IDA* idiom |
| ❌ E11 | Goal-driven hypothesis filter    | **rejected 2026-06-15** (per user): can't filter a hypothesis without testing it — unsound (drops contradiction-pruning candidates); sound variant is cold on the connected corpus + changes the `solve()` contract ([§Rejected](s1.9.e11_goal_driven_filter.md#rejected-2026-06-15)) | M | M | Prolog SLD-resolution; [docs/index/03](../../../docs/index/03-theorem-proving-formal-methods.md) |
| ❌ E12 | Hypothesis ordering by "informativeness" | **rejected 2026-06-15** (measured): "max cascade" is dominated by dead-post singletons → first completer at rank 19/56 (vs lex 11); discriminating signal is irreducibly post-fork ([§Rejected](s1.9.e12_informativeness.md#rejected-measured-2026-06-15); `demo/score_hypotheses.py`) | M | L (heuristic gain) | CSP value ordering |
| ❌ E13 | Per-hypothesis saturation budget | **dropped 2026-06-15** (per user): saturation is correctness-critical — a per-fork budget aborts before quiescence, so the fork's verdict is unsound even on the fast path ([§Dropped](s1.9.e13_per_hyp_budget.md#dropped-2026-06-15)) | M | L (UX) | branch-and-bound |

### CSP-style pre-processing

| ref | idea | mechanism | effort | value | references |
|-----|------|-----------|--------|-------|------------|
| ❌ E14 | Arc-consistency pre-pass         | **rejected 2026-06-15** — **subsumed by rule-saturation**: the engine is append-only (no domains to prune) and the puzzle's elimination rules already propagate the `(not h)` negatives AC-3 would derive ([§"Prune"…](s1.9.e14_arc_consistency.md#prune-in-an-append-only-engine-2026-06-15)) | L | M | CSP AC-3 ([docs/index/02](../../../docs/index/02-solvers-csp-sat-smt.md)) |
| ❌ E15 | Path-consistency (k-consistency) | **rejected 2026-06-15** — k-tuple generalisation of the (also-rejected) E14; just **eagerly** computes multi-literal nogoods `_nogoods` already builds **lazily + Apriori-minimal** ([§…weird here](s1.9.e15_path_consistency.md#what-it-is-and-why-its-weird-here-2026-06-15)) | L | L | k-consistency |

> **Lattice re-grounding (2026-06-15).** Re-judged against the engine's actual
> search — a *complete BFS over commitment-set cardinality* (Apriori), not a
> DPLL/DFS decision tree ([architecture_and_algorithms.md](../../../docs/kernel/inference/architecture_and_algorithms.md)
> §O7) — the search/CSP entries are now **all closed** against it. **Reorderers**
> (E9, E12) are inert for the complete/uniqueness search (within-layer order
> can't change Apriori pruning) and — measured on zebra2 (`demo/score_hypotheses.py`)
> — even **worse than lex** on the fast path: LCV ranks the first completer 35th,
> informativeness 19th, vs lex's 11th, because completers are
> pre-fork-indistinguishable and the dead-post singletons dominate every cascade
> signal. **❌ rejected.** **E13 dropped** (aborting saturation is unsound).
> **E10 inapplicable** (the cardinality layering already *is* the deepening). The
> would-be **space-shrinkers** are gone too: **E11 ❌ rejected** (can't filter a
> hypothesis without testing it — unsound, and the sound variant is cold +
> changes the `solve()` contract); **E4/E5 ⛔ superseded**; **E14/E15 ❌ rejected**
> (subsumed by append-only rule-saturation + the lazy Apriori-minimal nogood
> store). Net: the whole §search-heuristics + §CSP-preprocessing cluster is
> closed — a complete cardinality-BFS over a connected corpus leaves no purchase
> for these. See each stub's status section.

### Engineering / UX

| ref | idea | mechanism | effort | value | references |
|-----|------|-----------|--------|-------|------------|
| 📅 E16 | Lazy alive-set materialisation | stream `_compute_alive` instead of materialising a `frozenset[FactId]`; saves memory on huge puzzles. **Premise re-grounded** — no `root_alive`, and `state_hash` doesn't use the alive set ([E16 § Re-grounding](s1.9.e16_lazy_root_alive.md#re-grounding-2026-06-03)) — so the win is smaller than first framed | S | L (memory) | T1.5.4.5 alternative |
| ✅ E17 | Engine-level branch budget       | **DONE 2026-06-15** — budget (`max_enterings`/`max_time`) was already shipped; the residual T1.9.E17.2 now ships: `solve(on_budget="verdict")` returns an `Aborted` verdict (partial stats, `exhausted=False`) instead of raising (opt-in; default raise unchanged) ([§Implemented](s1.9.e17_branch_budget.md#implemented-2026-06-15)) | S | L (UX) | already in the engine |
| ❌ E18 | Rule-applicability pruning       | **rejected 2026-06-15** (measured): drops 0/30 rules on zebra2 — structurally dead, because generic rules' **variable assert-heads** can produce any relation, so nothing is ever provably-unreachable ([§Rejected](s1.9.e18_rule_applicability_pruning.md#rejected-2026-06-15)) | S | L | trim engine work |
| ✅ E19 | Unsat-core minimisation          | **DONE 2026-06-15** — `min_core.minimal_unsat_core` (provenance, on E6): smallest single-witness frontier, not the witness-union (zebra2-bad 38→1, the injected culprit). Re-saturation deletion-MUS is **NAF-unsound** → not shipped ([§Implemented](s1.9.e19_unsat_core_min.md#implemented-2026-06-15)) | M | L (UX only) | P1.6 trace renderer; composes with E6 |
| 📅 E20 | Conflict-cache cross-call        | persist `(not h)` learnings across `solve()` calls when the same puzzle re-runs | M | L | session-scoped speedup; composes with [S1.5.7](../p1.5_hypothesis_loop/s1.5.7_back_prop_unconditional.md) |

### Mode taxonomy + state-hash design (added 2026-05-24)

| ref | idea | mechanism | effort | value | references |
|-----|------|-----------|--------|-------|------------|
| ⛔ E21 | `solve` vs `prove` mode split  | **Superseded by [P1.7a](../p1.7a_solution_search_refactor/README.md) (2026-05-31)** — shipped *differently*: the exhaustive-with-uniqueness side is now `solve()` (returns `Solution \| Ambiguity \| Contradiction`, 1/>1/0 verdict); the fast side is the monotonic `solve(stop_after=1)` path. `Mode` (in `inference/verdict.py`, not `solver.py`) is the orthogonal SOLVE/GAPS/CONTRADICTIONS task-class axis, not SOLVE/PROVE. See [E21 § Superseded](s1.9.e21_solve_vs_prove.md#superseded-by-p17a-2026-06-03) | M | M (UX) | [Idea 03 — three task classes](../../../docs/ideas/03-three-task-classes.md) |
| ✅ E22 | Alive-hyps in canonical state hash | **Resolved in code:** `canon.state_hash` keys dedup on the KB facts ONLY, not the alive set — the engine relies on "KB ⇒ alive-set" and [S1.7.24](../p1.7_bootstrapping_zebra/s1.7.24_dehardcode_symmetric.md)'s state-hash-keyed lattice cements it. The "extend the hash with the alive set" fix was *not* taken; residual = document the invariant. See [E22 § Resolution](s1.9.e22_alive_hyps_in_state_hash.md#resolution-2026-06-03) | M | M (correctness) | [S1.5.3 canonicalisation](../p1.5_hypothesis_loop/s1.5.3_canonicalisation.md); T1.5.4.5 alive-inherit |
| 📅 E23 | Speed up the complete (exhaustive) search | **Re-anchored** from the never-shipped `Mode.PROVE` to `solve()` (P1.7a's complete entry — the actual bottleneck). Open question: is there a way to speed up *exhaustive* search without giving up the uniqueness guarantee? Candidates: learned-clause caching (composes with E7), goal-driven pruning (composes with E11 + [F7 §C rule-set sufficiency](../../followups/f7_rule_induction.md)), arc-consistency pre-pass (E14). Uniqueness is a global property; some form of full coverage is required. The design question is which form | L | M (perf) | E7 / E11 / E14 / F7 §C |

### Deductive-layer perf (not a hypothesis-loop item; added 2026-06-15)

**Surfaced by the 2026-06-15 P1.9 review.** P1.8a's profiling established that
**~95% of a solve is the matcher inside saturation (O1+O2 — the *deductive*
layer), not search**
([architecture_and_algorithms.md](../../../docs/kernel/inference/architecture_and_algorithms.md) §7).
Every P1.9 entry above is a *search-layer* optimisation, so the catalog's
highest-leverage perf lever has **no home here**. Recorded for visibility (the
📌 marks these as not standard P1.9 parked entries):

| ref | idea | mechanism | effort | value | references |
|-----|------|-----------|--------|-------|------------|
| 📌 D1 | RETE **beta-memories** | persist partial joins across firings (the one thing P1.8a's D5 semi-naive join still recomputes); the named next step up the Datalog ladder | M | **H** (perf) | Arch §6 O1 / §7; Forgy *Rete* (1982) |
| 📌 D2 | Worst-case-optimal join | Leapfrog-Triejoin / Generic-Join — only if cyclic join patterns appear (they don't yet) | L | L (until cyclic) | AGM bound; NPRR (2012) |

These belong to the **performance arc** ([P1.8a](../p1.8a_performance/README.md),
now closed), not the hypothesis loop — promote into a reopened P1.8a tail or a
new perf phase when saturation again dominates past the P1.8a gains, *not* into
P1.9. (Cross-ref: [E23 § Re-anchor 2](s1.9.e23_prove_speedup.md#re-anchor-2-2026-06-15).)

### Rejected / out-of-scope for M1

Listed here for visibility — these would need a fundamental
scope change to promote, not a measurable activation signal.

| ref | idea | reason |
|-----|------|--------|
| ❌ R1 | Soft-constraint / probabilistic weighting | M1 is hard-constraint only; soft constraints are an [M3 SMT](../../m3_smt_integration/) concern |
| ❌ R2 | Cross-puzzle learning (failed-pattern transfer) | requires session/file persistence; out of scope until M2's NL pipeline gives us a stream of puzzles |
| ❌ R3 | Parallel branch evaluation         | engineering; M1 is single-threaded; defer to a P1.7+ profiling pass if needed |
| ❌ R4 | Domain-specific filters (e.g. spatial) | violates the canonical-zebra2 direction — all constraints should live in user rules, not engine hardcode |

## Promotion mechanics

Each entry promotes to a `s1.9.<n>_<title>.md` stage file when
work starts. Numbering follows the catalog letter — `E6` →
`s1.9.e6_*.md`, or re-numbered sequentially `s1.9.1_*.md`,
`s1.9.2_*.md`, …; pick at promotion time and cross-reference the
original E-id in the new file's header. The catalog row stays
here as the index entry; the promoted stage file owns the
detailed spec.

## Activation criteria

P1.9 entries activate on **measurable need**, not on schedule:

- **A user-facing puzzle** (Zebra, M2 NL output, M3 SMT slice)
  exceeds the current engine's ergonomic time/space envelope and
  one of the catalog entries is the demonstrable bottleneck.
- **A regression** from a downstream change re-opens the
  efficiency hole one of these entries closes.
- **An empirical study** (P1.7's Zebra acceptance, or a P1.8
  Theme C measurement) surfaces a specific catalog entry as the
  highest-leverage next move.

Without one of those signals, P1.9 stays cold.

## Cross-links

- [S1.5.4](../p1.5_hypothesis_loop/s1.5.4_hypgen_improvements.md)
  — the stage that spawned this catalog; the "Already shipped"
  / "Planned in S1.5.4" / "Deferred — own stages" sub-catalogs
  stay there.
- [S1.5.5](../p1.5_hypothesis_loop/s1.5.5_closure_auto_inference.md),
  [S1.5.6](../p1.5_hypothesis_loop/s1.5.6_one_step_lookahead.md),
  [S1.5.7](../p1.5_hypothesis_loop/s1.5.7_back_prop_unconditional.md)
  — the *promoted* S1.5.4 follow-ups (those that ship inside M1).
  P1.9 entries are the ones that didn't promote.
- [P1.8](../p1.8_ein_lang_modules/README.md) — sibling
  placeholder phase; same activation pattern.
- [F7 rule induction](../../followups/f7_rule_induction.md) — the
  long-term framing that many P1.9 entries (especially E1-E5,
  E11) are interim workarounds for.
- [docs/index/02 — solvers / CSP / SAT / SMT](../../../docs/index/02-solvers-csp-sat-smt.md)
  — external tech background for E4, E7, E14.
- [docs/index/09 — cognitive architectures / neurosymbolic](../../../docs/index/09-cognitive-architectures-neurosymbolic.md)
  — ATMS justification-cache framing for E7.

## Open questions

Per-entry questions live alongside the entry's row above as
references / commentary. Promotion to a stage file is where
Q-numbered questions get assigned (e.g. `Q-S1.9.E6.A` etc.).
