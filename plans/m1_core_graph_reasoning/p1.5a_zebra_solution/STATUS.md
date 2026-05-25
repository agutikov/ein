# P1.5a — status report

**Generated:** 2026-05-25.
**Phase acceptance gate:** [S1.5a.13](s1.5a.13_acceptance_zebra2_solves.md) — zebra2 cold-solve under M1 perf budget (< 60s laptop, idea-08 trace fidelity). **Not yet met.**

Smoke (depth 0): `bench_solve examples/zebra2.ein --max-depth 0` → `Ambiguity` (89 candidates across 5 relations, 0 solution branches, 1641ms). Suite: **665 tests** collected, focused P1.5a tests green (46 / 46).

## Stage status table

| ID | Title | Status | Shipped | Open work / blocker |
|----|-------|--------|---------|---------------------|
| [S1.5a.1](s1.5a.1_naf_semantic_rearch.md) | NAF semantic re-architecture | shipped | 2026-05-24 (`ccf2aa8`) | — |
| [S1.5a.1a](s1.5a.1a_branch_order_determinism.md) | Branch exploration order determinism | shipped | 2026-05-24 (`fb04f1a`) | — |
| [S1.5a.2](s1.5a.2_hypgen_pre_pruning_recovery.md) | Hypgen pre-pruning recovery (disjunctive-prune fwd/bwd) | shipped | 2026-05-24 (`00b5ed3`) | full `zebra2.ein` re-test deferred to S1.5a.13 |
| [S1.5a.2a](s1.5a.2a_statistics_determinism.md) | Statistics determinism | partial | 2026-05-24 V1 (`bbe3690`) | T1.5a.2a.2 promoted to S1.5a.16; T1.5a.2a.3 doc pending; Q-S1.5a.2a.B parked |
| [S1.5a.5](s1.5a.5_house_to_location_rename.md) | zebra2 rename family (`*-loc`, `co-located`, `adjacent-via`) | shipped | 2026-05-24 (`ef7c69e`) | — |
| [S1.5a.6](s1.5a.6_pypy_compat_perf.md) | PyPy compatibility + perf measurement | partial | 2026-05-25 runner only (`a2a802d`) | T1.5a.6.1–.4 (audit, bench, report, docs); waits on S1.5a.13 |
| [S1.5a.7](s1.5a.7_hypgen_scoring_branch_info.md) | Hypothesis scoring + branch-info ordering | partial | 2026-05-25 Idea 1 default (`10550d4`) | T1.5a.7.3 (Idea 2 impl, currently `NotImplementedError`); T1.5a.7.4/.5 (empirical compare + recommendation) await S1.5a.13/.6 |
| [S1.5a.8](s1.5a.8_naf_dependency_map.md) | Static NAF dependency map | parked | — | T1.5a.8.1–.3 (whole stage); promote when a surprising derived-NAF rule lands |
| [S1.5a.10](s1.5a.10_query_semantics_who_vs_where.md) | Query semantics: who vs where | parked | — | T1.5a.10.1–.3 (verify, `:project` design, trace text); needs zebra2 solving |
| [S1.5a.11](s1.5a.11_state_dump.md) | State dump harness | shipped | 2026-05-24 v1 + v2 (`6680956`, `691d349`) | non-blocking deferred polish (pre-backprop snapshot, JSON mirror, streaming, round-trippable `.ein`) |
| [S1.5a.12](s1.5a.12_idea08_trace_acceptance.md) | idea-08 trace acceptance | parked | — | T1.5a.12.1–.4 (build checklist, diff, resolve, stabilise); needs solve **and** P1.6 S1.6.4 markdown renderer |
| [S1.5a.13](s1.5a.13_acceptance_zebra2_solves.md) | Acceptance — zebra2 solves uniquely | planned | — | T1.5a.13.1–.4 (measure, trace, docs, verdict); composite gate — depends on every other stage's perf lever landing |
| [S1.5a.14](s1.5a.14_transitive_backprop.md) | Transitive back-prop (bubble `(not h)` to all ancestors) | partial | 2026-05-25 Phase 1 (`17bd821`) | Phase 2 absorbed into S1.5a.17; Phase 3 absorbed into S1.5a.15 |
| [S1.5a.15](s1.5a.15_dead_caching_unsat_core.md) | Dead-branch caching by unsat-core + per-level back-prop | in-progress (design) | — | T1.5a.15.1–.4 (all four phases); ready to start, gated by S1.5a.13 budget urgency |
| [S1.5a.16](s1.5a.16_branch_order_shuffle_invariance.md) | Branch-order shuffle invariance (depth-bounded) | in-progress (design) | — | T1.5a.16.1–.4 (knob + serialiser, harness, triage, doc); gates the S1.5a.17 default-on flip |
| [S1.5a.17](s1.5a.17_eager_root_bubble_outer_loop.md) | Eager root-bubble + outer re-entry loop | partial (flag off) | 2026-05-25 mechanism (`43333c5`) | T1.5a.17.1–.6 surface in code & tests; default flip + acceptance composition gated on S1.5a.16 harness |
| [S1.5a.18](s1.5a.18_path_condition_nogoods.md) | Path-condition no-good clause learning | partial (flag off) | 2026-05-25 mechanism (`2d8cac3`) | T1.5a.18.1–.4 surface in code & tests; default flip gated on empirical demo-10 + zebra2 measurement |

**Legend.** *shipped* = all tasks closed, in git, default-on if config-gated. *partial* = mechanism shipped behind an off-by-default flag, or only a subset of tasks closed. *in-progress (design)* = doc + design exists, code not yet started. *parked* = doc exists, not currently scheduled — promote when blocker lifts. *planned* = terminal acceptance stage, runs after its dependencies.

## Acceptance-gate dependency chain

```
S1.5a.13 (zebra2 solves)
  ├── S1.5a.1   ✓   (NAF re-eval)
  ├── S1.5a.2   ✓   (disjunctive-prune fwd/bwd)
  ├── S1.5a.12  ✗   parked — needs P1.6 trace renderer
  └── perf levers for cold-solve in budget:
        S1.5a.7   partial   (popularity default on; branch-info NIE)
        S1.5a.14  partial   (Phase 1 shipped; Phase 2/3 elsewhere)
        S1.5a.15  design    (not started)
        S1.5a.16  design    (gate for .17 default-on)
        S1.5a.17  partial   (flag off; awaits .16 harness)
        S1.5a.18  partial   (flag off; awaits demo-10 measurement)
        S1.5a.6   partial   (PyPy runner only)
```

The shortest path to closing the gate is: **S1.5a.16** (harness) → flip **S1.5a.17** + **S1.5a.18** defaults → land **S1.5a.15** → run **S1.5a.13** measurement → if budget met, queue **S1.5a.12** (after P1.6 S1.6.4) → final verdict.

## Open tasks by stage

### S1.5a.2a — Statistics determinism (partial)
- **T1.5a.2a.2** — *Branch-order shuffle invariance.* `SolverConfig.candidate_order` knob; shuffle-and-compare test on zebra2 + demos 03–05 at N=0,1,2. **Largely promoted to S1.5a.16.**
- **T1.5a.2a.3** — Document the invariant in `docs/kernel/inference/README.md` (Statistics determinism subsection).
- **Q-S1.5a.2a.B** — Broader entity back-pointer audit (`Type._kb` / `Relation._kb` / `Instance._kb`); decision deferred until another consumer trips the bug.

### S1.5a.6 — PyPy compatibility + perf (partial)
- **T1.5a.6.1** — Compatibility audit (pytest under PyPy3; categorise + fix/skip failures).
- **T1.5a.6.2** — Perf measurement (`bench_solve` + `bench_saturate` CPython vs PyPy cold vs warm; branching demos).
- **T1.5a.6.3** — Report (compat status + perf table + viability recommendation; bears on P1.8 Theme B urgency).
- **T1.5a.6.4** — Document caveats in `docs/kernel/inference/`.
- *Blocker:* needs S1.5a.13 (a known-correct solve to measure).

### S1.5a.7 — Hypothesis scoring + branch-info (partial)
- **T1.5a.7.3** — Implement branch-info ordering (Idea 2); currently raises `NotImplementedError`; needs post-saturation signal integrating with S1.5.7b stable-alive caching.
- **T1.5a.7.4** — Empirical comparison (all 4 scoring modes × zebra2 + 5 branching demos; tabulate tree-node count, wall time, alive-set size, unsat-core size).
- **T1.5a.7.5** — Recommendation report (default-mode choice from T1.5a.7.4).
- *Blocker:* T1.5a.7.4/.5 await S1.5a.13 + S1.5a.6 for a fair perf baseline.

### S1.5a.8 — Static NAF dependency map (parked)
- **T1.5a.8.1** — Compile-plan NAF→relation walk; `Engine.naf_dependency_map()`.
- **T1.5a.8.2** — Load-time warning (`warnings.warn` for derived-NAF rules; `warn_derived_naf` config flag).
- **T1.5a.8.3** — Tests (unit declared-only-NAF / derived-NAF + zebra2 smoke).

### S1.5a.10 — Query semantics: who vs where (parked)
- **T1.5a.10.1** — Verify option (2) end-to-end (extend `:goal` in zebra2 with `nation-loc` clauses; confirm Norwegian/Japanese bindings via `bench_solve`).
- **T1.5a.10.2** — Design `:project` for multi-hop questions (only if a 3+ hop puzzle surfaces).
- **T1.5a.10.3** — Trace text "X drinks Water" rendering (record requirement for P1.6).
- *Blocker:* needs S1.5a.13.

### S1.5a.12 — idea-08 trace acceptance (parked)
- **T1.5a.12.1** — Build firing checklist (per "Therefore X" in idea-08, identify expected rule firing + binding; ~15–20 rows → `s1.5a.12_idea08_checklist.md`).
- **T1.5a.12.2** — Run + diff (`bench_solve --trace=zebra.md`; annotate each row ✅ / ❌ / 🚧).
- **T1.5a.12.3** — Resolve mismatches (missing firing / wrong order / wrong rule classes; sub-tasks per row).
- **T1.5a.12.4** — Stabilise the checklist as a regression target.
- *Blocker:* needs P1.6 S1.6.4 markdown trace renderer in addition to S1.5a.13.

### S1.5a.13 — Acceptance — zebra2 solves uniquely (planned)
- **T1.5a.13.1** — Measurement (`bench_solve` on laptop ref; verdict, bindings, wall + RSS, tree-node + dead counts, HypGen stats; tabulate vs pre-B1 + post-S1.5a.1/2 baselines).
- **T1.5a.13.2** — Trace + checklist re-run (`--trace=zebra2.md --trace-dir=zebra2/`; re-run S1.5a.12 checklist).
- **T1.5a.13.3** — Docs (`docs/kernel/inference/README.md`: NAF invariant, workaround pattern, hypgen pre-pruning).
- **T1.5a.13.4** — Phase verdict (ship/no-ship; queue follow-ups if no-ship).
- *Blocker:* every perf-lever stage downstream of the chain above.

### S1.5a.14 — Transitive back-prop (partial)
- Phase 1 shipped. Phase 2 (outer driver) absorbed into **S1.5a.17**; Phase 3 (per-level) absorbed into **S1.5a.15**. Nothing standalone remains.

### S1.5a.15 — Dead-branch caching by unsat-core (design)
- **T1.5a.15.1** — Direct-dead unsat-core cache (`builder.dead_index: list[_DeadByCore]`; superset short-circuit in `_explore`; bench on `zebra2-hints`).
- **T1.5a.15.2** — Promoted-dead cache via `_promote_verdicts` (`aggregated_unsat_core` field; alive-fp pairing; register in `dead_index`).
- **T1.5a.15.3** — Per-level back-prop (`builder.depth_of[nid]`; `back_propagate_per_level(shallowest_required)`; per-binding analysis on every dead candidate).
- **T1.5a.15.4** — Promoted-dead back-prop (root case absorbed into S1.5a.17.3; **general per-level case (depths ≥ 1) remains here**).

### S1.5a.16 — Branch-order shuffle invariance (design)
- **T1.5a.16.1** — Shuffle knob + snapshot serialiser (`SolverConfig.candidate_order: Literal | tuple`; `snapshot_partial_info` reusing S1.5a.11 dumper).
- **T1.5a.16.2** — Diff harness + zebra2 baseline (`tests/inference/test_shuffle_invariance.py` param over puzzle × depth × seed; `demo/diff_shuffle.py`).
- **T1.5a.16.3** — Triage the violations (state-hash cache / back-prop-depth / NAF cross-branch-timing classes).
- **T1.5a.16.4** — Document invariant in `docs/kernel/inference/README.md` Determinism subsection.
- *Blocker:* needs the `candidate_order` knob promoted out of T1.5a.2a.2.

### S1.5a.17 — Eager root-bubble + outer re-entry loop (partial)
- **T1.5a.17.1** — `BubbleAbort` exception + `solve_outer` driver skeleton; `enable_eager_root_bubble` flag (✓ in `config.py`, default `False`); `_pass_bubbled` bookkeeping; raise on write.
- **T1.5a.17.2** — Root state machine (`SearchNode.forced: bool`; `KnowledgeBase.committed_hypotheses: set[FactId]`; `_candidates_for` filter; `state_index` visited-as-DAG-link marker).
- **T1.5a.17.3** — Promoted-dead → root fact (`_synthesise_promoted_dead_facts` after `_promote_verdicts`).
- **T1.5a.17.4** — Re-entry dedup audit (`state_index` across passes; `ConsumeStats.outer_passes`).
- **T1.5a.17.5** — Trace + dumper updates (`(pass N)` marker in trace IR; `pass_started.json` per outer pass; `bench_solve --verbose` outer-pass counter).
- **T1.5a.17.6** — Acceptance gate composition (default flip gated on S1.5a.16 green at depths 0–3 with flag on; doc in inference README).
- *Note:* `test_eager_root_bubble.py` passes; task-level shipped-vs-pending split needs an in-file audit pass (the file marks "active design" overall, doesn't tag individual tasks).

### S1.5a.18 — Path-condition no-good clause learning (partial)
- **T1.5a.18.1** — Storage + ContextVar + config flag (`KnowledgeBase._nogoods`; `_path_ctx` ContextVar; `enable_path_condition_nogoods` ✓ in `config.py`, default `False`).
- **T1.5a.18.2** — `nogoods.py` helpers (✓ module exists: `emit_nogood` subsumption-aware insert; `filter_by_nogoods`; single-element guard).
- **T1.5a.18.3** — Wire into the death sites (`_consume` sweep, `_descend` dead-leaf, `_explore_inner` contradiction-on-entry; three pre-fork filter sites).
- **T1.5a.18.4** — Tests (`test_path_condition_nogoods.py` ✓; subsumption / filter / flag-off parity / flag-on conditional-death emit / eager + nogoods composition).
- *Acceptance still open:* demo 10 tree-node count ≤ 20 (vs 32 flag-off baseline); zebra2 clause-set bounded < 200 after depth-3 run; default flip.

## Open design questions parked in stage files

These are explicitly parked in their stage files for "decide later" without blocking the active work:

- **Q-S1.5a.2a.B** — Broader entity back-pointer audit (S1.5a.2a).
- **Q-S1.5a.17.A** — Abort on *intermediate*-level bubbles that don't change root candidates? (option 1 vs option 2 — ship option 1, measure thrash).
- **Q-S1.5a.17.B** — Partial-subtree preservation on abort? (discard for now; alternative deferred until measurement says it matters).
- **Q-S1.5a.17.C** — Does eager mode change `_promote_verdicts`? (algorithm unchanged; order-of-operations explicit).
- **Q-S1.5a.17.D** — Cond-dead at depth ≥ 1 — still cached per-`_consume`? (yes).
- **Q-S1.5a.17.E** — Symmetry with hypothesis scoring (S1.5a.7)? (composes naturally; re-baseline scoring measurement under eager abort if both land).
- **Q-S1.5a.18.A** — Where does parent saturation state matter under future retraction-capable rules? (ship under monotonicity assumption; re-audit when retraction lands).
- **Q-S1.5a.18.B** — Index for `filter_by_nogoods`? (naive is fine at < 200 clauses; defer index until measurement says it matters).
- **Q-S1.5a.18.C** — Trace representation for learned clauses? (out of scope for engine; revisit in P1.6).
- **Q-S1.5a.18.D** — Cross-puzzle clause reuse? (out of scope for M1; perf-followup note).
- **Q-S1.5a.18.E** — Interaction with S1.5a.15 Phase 3? (composition benign — subsumption catches it).

## Cross-links

- Phase intro + design background: [`README.md`](README.md).
- Upstream: [P1.5 hypothesis loop](../p1.5_hypothesis_loop/) (closed through S1.5.8c).
- Downstream: [P1.6 rendering + trace](../p1.6_rendering_and_trace/) (needed for S1.5a.12).
- Followups catalog: [P1.9](../p1.9_hypothesis_loop_followups/).
- Acceptance gates: [M1 README — acceptance](../README.md#acceptance).
- Trace fidelity target: [`docs/ideas/08-human-style-deductive-trace.md`](../../../docs/ideas/08-human-style-deductive-trace.md).
- Engine doc: [`docs/kernel/inference/README.md`](../../../docs/kernel/inference/README.md) (carries the NAF / determinism / eager-bubble notes that several of these stages need to update on ship).
