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
*Status:* 📅 parked (awaiting activation signal) · ❌ rejected
for M1 (parked here for visibility; will not promote without a
fundamental scope change).
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

The R1-R4 rejected entries stay in the README catalog only.

## Catalog

### Closure refinements (Q-S1.5.4.B descendants)

| ref | idea | mechanism | effort | value | references |
|-----|------|-----------|--------|-------|------------|
| 📅 E1 | `(functional R)` activator       | weaker than `single-parent`; just declares R is a function on its slot-0 | S | M | DL `funcProp` |
| 📅 E2 | `(at-most-one R slot)` activator | per-slot cardinality declaration; closure derives when every leaf at that slot is occupied | M | M | CSP cardinality |
| 📅 E3 | `(no-hypotheses R)` activator    | distinct from `(closed R)`: R stays open-world but the engine never *guesses* on it. For observational data | S | L | engineering convenience |
| 📅 E4 | `(symmetry-class R T)`           | declare T-instances interchangeable under R; collapse to canonical representative at gen-time, replacing the A5 emit-both hack | M | M | CSP value-symmetry breaking ([docs/index/02](../../../docs/index/02-solvers-csp-sat-smt.md)) |
| 📅 E5 | Static rule-conflict pre-analysis | precompute `(relation, arg-position)` mutex pairs from the rule set; drop hypotheses violating mutex at gen-time | M | M | rule-set sufficiency, [F7 §C](../../followups/f7_rule_induction.md) |

### Conflict-driven learning (SAT/CDCL-inspired)

| ref | idea | mechanism | effort | value | references |
|-----|------|-----------|--------|-------|------------|
| 📅 E6 | Transitive premise walk for "unconditional" | walk `Provenance.justified_by` chain; "unconditional" iff *no* transitive premise has `kind="hypothesis"` (the safer version of the original T1.5.4.3.a shallow spec) | S | **H** (correctness) | the *promoted* part of E6 ships as [S1.5.7 T1.5.7.1](../p1.5_hypothesis_loop/s1.5.7_back_prop_unconditional.md#task-t1571--transitive-premise-walk-e6); this entry tracks the broader applicability (e.g. unsat-core minimisation E19, learned-clause E7) |
| 📅 E7 | Learned-clause from unsat-core   | the conjunction of source facts that produced this contradiction is itself a "learned constraint"; future hypotheses are tested against it without re-running rules | L | H (long-term) | CDCL ([docs/index/02](../../../docs/index/02-solvers-csp-sat-smt.md)); ATMS justification cache ([docs/index/09](../../../docs/index/09-cognitive-architectures-neurosymbolic.md)) |
| 📅 E8 | Watched-fact rule applicability   | maintain SAT-style "watched literals" per rule's match premise; re-fire only when a watched fact changes | L | M | DPLL watched literals |

### Search heuristics

| ref | idea | mechanism | effort | value | references |
|-----|------|-----------|--------|-------|------------|
| 📅 E9  | Least-constraining-value (LCV)   | within an object's hypotheses, prefer fillers whose addition prunes the *least* of the remaining domain (counterpart to most-constrained-object) | M | M | CSP textbook |
| 📅 E10 | Iterative deepening              | start `max_depth=1`, deepen on `Ambiguity`-with-open-leaves; cheap puzzles bottom out shallow | S | M | IDA* idiom |
| 📅 E11 | Goal-driven hypothesis filter    | only emit `h` that could unify with the query goal (or a sub-goal via rule unification); backward chaining trim | M | M | Prolog SLD-resolution; [docs/index/03](../../../docs/index/03-theorem-proving-formal-methods.md) |
| 📅 E12 | Hypothesis ordering by "informativeness" | prefer hypotheses with high expected pruning (alive: max-constraint; dead: max-back-prop) | M | L (heuristic gain) | CSP value ordering |
| 📅 E13 | Per-hypothesis saturation budget | abort the fork's saturate after K steps; treat as alive (depth-N open leaf) | M | L (UX) | branch-and-bound |

### CSP-style pre-processing

| ref | idea | mechanism | effort | value | references |
|-----|------|-----------|--------|-------|------------|
| 📅 E14 | Arc-consistency pre-pass         | before main saturation, run AC-3 over the relation graph to prune trivially-impossible facts | L | M | CSP AC-3 ([docs/index/02](../../../docs/index/02-solvers-csp-sat-smt.md)) |
| 📅 E15 | Path-consistency (k-consistency) | generalises AC to k-tuples; expensive but tightens the alive set further | L | L | k-consistency |

### Engineering / UX

| ref | idea | mechanism | effort | value | references |
|-----|------|-----------|--------|-------|------------|
| 📅 E16 | Lazy `root_alive` materialisation | keep the generator stream-based instead of frozenset-materialised; saves memory on huge puzzles | S | L (memory) | T1.5.4.5 alternative |
| 📅 E17 | Engine-level branch budget       | move `bench_solve --max-nodes` into `solve()` itself; surface as `Verdict.aborted` flag | S | L (UX) | bench_solve already has the wrapper |
| 📅 E18 | Rule-applicability pruning       | per puzzle, drop rules whose `:match` references relations absent from the KB | S | L | trim engine work |
| 📅 E19 | Unsat-core minimisation          | shrink the unsat-core to a minimal subset before storing on the dead leaf — purely for trace readability | M | L (UX only) | P1.6 trace renderer; composes with E6 |
| 📅 E20 | Conflict-cache cross-call        | persist `(not h)` learnings across `solve()` calls when the same puzzle re-runs | M | L | session-scoped speedup; composes with [S1.5.7](../p1.5_hypothesis_loop/s1.5.7_back_prop_unconditional.md) |

### Mode taxonomy + state-hash design (added 2026-05-24)

| ref | idea | mechanism | effort | value | references |
|-----|------|-----------|--------|-------|------------|
| 📅 E21 | `solve` vs `prove` mode split  | `Mode.SOLVE` returns on first alive verdict (uniqueness undetermined; faster); `Mode.PROVE` keeps the current exhaustive semantics (every alive branch fully explored, returns `Solution \| Ambiguity \| Contradiction` with uniqueness witness). Current `Mode.SOLVE` is *operationally* PROVE under [S1.5.3 alive-branch termination](../p1.5_hypothesis_loop/s1.5.3_canonicalisation.md). Split surfaces the user's "find any answer" intent | M | M (UX) | [Idea 03 — three task classes](../../../docs/ideas/03-three-task-classes.md); current `Mode` enum in `inference/solver.py` |
| 📅 E22 | Alive-hyps in canonical state hash | Open question: can two different paths reach the same post-saturation KB but carry **different alive hypothesis sets**? If yes, S1.5.3's state-hash dedup is **unsound** for the alive-inherit optimisation (T1.5.4.5/8). Fix: extend `state_hash` to include the alive set; collapse only paths whose `(state, alive)` pair matches. Effort is the analysis + the fix together — the analysis may show the case is impossible under M1's monotonic semantics, in which case ship as a paper-form proof rather than a code change | M | M (correctness) | [S1.5.3 canonicalisation](../p1.5_hypothesis_loop/s1.5.3_canonicalisation.md); T1.5.4.5 alive-inherit |
| 📅 E23 | Prove speedup — replace exhaustive with what? | Once E21 lands, `Mode.PROVE` is the bottleneck. Open question: is there a way to speed up *exhaustive* search without giving up the uniqueness guarantee? Candidates: learned-clause caching (composes with E7), goal-driven pruning (composes with E11 + [F7 §C rule-set sufficiency](../../followups/f7_rule_induction.md)), arc-consistency pre-pass (E14). The framing is "not doing full search" — but uniqueness is a global property; some form of full coverage is required. The interesting design question is which form | L | M (perf) | E7 / E11 / E14 / F7 §C |

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
