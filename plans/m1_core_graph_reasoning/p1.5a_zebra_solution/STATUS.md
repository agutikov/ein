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
| [S1.5a.19](s1.5a.19_d0_negative_completion_gap.md) | d=0 negative-completion gap + downstream stall + dumper enhancement | active design | — | T1.5a.19.1–.6 (branch diff, downstream-stall diagnosis, dumper timeline+attribution, catalog/design, prototype, tests/docs); **named perf lever that closes the depth gap** for [[S1.5a.13]] |
| [S1.5a.20](s1.5a.20_branch_isolation_rearch.md) | Branch-isolation re-architecture (two channels: `try_branch` ↓ + `integrate` ↑) + logical/search branch vocabulary + per-child dump | active design | — | T1.5a.20.1–.7 (fork deep-isolation, `back_propagate` collapse, forced-positive return path, nogoods re-route, `_TreeBuilder` ownership audit, per-child branch_result+integrate dump, distributed snapshot bundle); **prerequisite for S1.5a.19 implementation** + closes Q2a/c for free |

**Legend.** *shipped* = all tasks closed, in git, default-on if config-gated. *partial* = mechanism shipped behind an off-by-default flag, or only a subset of tasks closed. *in-progress (design)* = doc + design exists, code not yet started. *parked* = doc exists, not currently scheduled — promote when blocker lifts. *planned* = terminal acceptance stage, runs after its dependencies.

## Acceptance-gate dependency chain

```
S1.5a.13 (zebra2 solves)
  ├── S1.5a.1   ✓   (NAF re-eval)
  ├── S1.5a.2   ✓   (disjunctive-prune fwd/bwd)
  ├── S1.5a.12  ✗   parked — needs P1.6 trace renderer
  └── perf levers for cold-solve in budget:
        S1.5a.20  design    **branch-isolation re-arch** — prerequisite for .19 impl; closes Q2a/c
        S1.5a.19  design    **closes the depth gap** (d=0 negative completion); lands on .20's isolated boundary
        S1.5a.7   partial   (popularity default on; branch-info NIE)
        S1.5a.14  partial   (Phase 1 shipped; Phase 2/3 elsewhere; .20 re-architects this mechanism)
        S1.5a.15  design    (not started; Phase 2 promoted-dead becomes natural under .20)
        S1.5a.16  design    (gate for .17 default-on)
        S1.5a.17  partial   (flag off; awaits .16 harness; .20 reroutes BubbleAbort)
        S1.5a.18  partial   (flag off; awaits demo-10 measurement; .20 routes emit_nogood per-frame)
        S1.5a.6   partial   (PyPy runner only)
```

**S1.5a.20** is the structural enabler — every "go up" channel (back-prop, forced-positive, nogood-emit, verdict-cache clear) collapses into one `integrate()` step on the immediate parent, and `kb.fork()` deep-isolates every shared mutable container. **S1.5a.19** is qualitatively different from the perf levers: every other lever makes the *depth-bounded search* cheaper; S1.5a.19 closes the inference gap at d=0, shrinking the depth needed to *find* the solution (NL trace lives at depth 1; engine currently needs > 4). Landing them in order — .20 then .19 — means S1.5a.19's chosen design (T1.5a.19.4 approach A/B/C) lands on the isolated boundary instead of inflating the leak surface.

The shortest path to closing the gate is now: **S1.5a.20** (rewrite the two channels) → **S1.5a.19** (BUG diagnosis + design + prototype on the isolated boundary; closes Q2a/c via integrate steps 6+7) → **S1.5a.16** (harness) → flip **S1.5a.17** + **S1.5a.18** defaults → land **S1.5a.15** → run **S1.5a.13** measurement → if budget met, queue **S1.5a.12** (after P1.6 S1.6.4) → final verdict.

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

### S1.5a.19 — d=0 negative-completion gap + downstream stall + dumper enhancement (active design)
**BUG.** `bench_solve_pypy ./examples/zebra2.ein --max-depth 4` → `Ambiguity` (568 nodes, 472 dead, 77 open, 0 solutions, 49.4 s) while `examples/README.md`'s NL trace solves the puzzle with four single-hypothesis branches refuted at depth 1.
**Diagnosis (2026-05-25, two passes).** Pass 1 claimed "engine never derives Yellow@H_1" — wrong. `dump/zebra2/resats/{001,002}.ein` show the engine **does** derive Yellow@H_1 (`range-elimination`), Kools@H_1 (`co-located`), Horse@H_2 (`adjacent-via-fwd`), Water@H_1 (`domain-elimination`) — but via the [[S1.5a.14]] back-prop + re-saturation loop after 24 dead d=1 branches. Pass 2 (after [[s1.5a.11]] dumper enhancement) surfaced **four confirmed sub-bugs** via `00_timeline.jsonl` + the resat attribution split:
- **(Q1)** d=0 inference incompleteness — engine pays 24 dead d=1 branches to learn negatives NL produces in one d=0 saturation pass.
- **(Q2a)** stale alive-set under `enable_alive_inherit=True` — root cycle 1 writes Yellow@H_1 at seq=25; b15 (Yellow@H_3) and b20 (Yellow@H_4) still get alloc'd + saturated at seq=32/42. **New sub-stage S1.5a.19a candidate.**
- **(Q2b)** 0-derivation resats — b25 cycle 1: 10 negatives written, 0 derivations. Back-prop's single-negative-per-dead-candidate doesn't satisfy domain/range-elim's `forall` premise. Same root cause as Q1; closed by Q1's fix.
- **(Q2c)** missed contradiction post-resat — b432 cycle 2 derives `(false)` via functional drink-loc; b432 nonetheless marked verdict=open and engine alloc's d=3 children for ~20 s. **New sub-stage S1.5a.19c candidate.**

Hypothesis ordering is **out of scope** — order affects time-to-first-solution but not the depth-`d` closure, which is fixed by (root kb + rule set + the *set* of length-≤`d` path conditions). Tracked separately under [S1.5a.7](s1.5a.7_hypgen_scoring_branch_info.md).

*Prerequisite added 2026-05-25:* [[S1.5a.20]] (branch-isolation re-architecture) lands first. The current engine has 12 violations of the "exactly two channels" rule; S1.5a.19's chosen approach (T1.5a.19.4 A/B/C) lands on the post-S1.5a.20 isolated boundary. Q2a (alive refresh) + Q2c (post-resat contradiction) close as side-effects of S1.5a.20's `integrate` steps 6+7.

- **T1.5a.19.1** — Branch-by-branch NL ↔ engine diff; cross-reference `resats/` dead_children to separate Step-1-closure sources from post-cycle-2 exploration.
- **T1.5a.19.2** — Downstream-stall fix work (Q2a/c land as new sub-stages 19a/19c; Q2b unified with Q1's fix; Q2d tracked in S1.5a.7).
- **T1.5a.19.3** — ✓ shipped 2026-05-25 (dumper enhancement: `00_timeline.jsonl` + back_prop/resat split + nested-Fact summary; 5 new tests, 670/670 suite green).
- **T1.5a.19.4** — Catalog missing d=0 inferences + design space (A: ein rules / B: engine semantics / C: lookahead → saturation feedback).
- **T1.5a.19.5** — Prototype + measure: `bench_solve examples/zebra2.ein --max-depth 1` → `Solution`, ≤ 30 nodes, < 5 s.
- **T1.5a.19.6** — Tests + docs.
- *Blocks:* S1.5a.13 — single perf lever that *closes the depth gap*.

### S1.5a.20 — Branch-isolation re-architecture (active design)
**Motivation.** User direction 2026-05-25: *"each branch processing MUST be isolated from both ancestors, descendants and siblings; some day maybe branches will be processed in distributed env, so they must be isolated already now"*. Inventory of the current implementation found ONE clean "args go down" channel (`try_branch`) but **four** "go up" channels — `back_propagate`, `_mirror_forced_positive`, `emit_nogood`, `_clear_ancestor_verdict_caches` — three of which reach across the full ancestor chain in one call via `_kb_chain_ctx` / `_consume_caches_ctx` ContextVars, plus shared mutable `kb.fork()` fields (`consume_stats`, `committed_hypotheses`, `_nogoods`, entity `_kb` back-pointers).
**Target.** Two channels: (1) `try_branch(parent_snapshot, hypothesis) → BranchResult`; (2) `integrate(parent_kb, child_result) → BranchResult | None`. Bubbling depth-N → root happens by N nested integrations, each one acting only on its immediate parent.
- **T1.5a.20.1** — `kb.fork()` deep-isolation: `consume_stats_delta`, `learned_nogoods`, `forced_positives` on `BranchResult`; entity `_kb` audit (rebuild table on fork vs explicit `kb=` arg).
- **T1.5a.20.2** — `back_propagate` collapse: writes only to given kb; remove `_kb_chain_ctx` + `_consume_caches_ctx`; stratify the chain-walk into per-level integrates.
- **T1.5a.20.3** — `_mirror_forced_positive` removal: forced positives travel via `BranchResult`; eager-mode `BubbleAbort` raised only from root frame.
- **T1.5a.20.4** — `emit_nogood` re-routing: per-frame `_nogoods`; explicit `path_condition` argument replaces `_path_ctx` reads.
- **T1.5a.20.5** — `_TreeBuilder` ownership audit: parent-only writes; child returns `SearchSubtree`.
- **T1.5a.20.6** — Per-child-branch dump: `branches/b{i}/branch_result.json` (full BranchResult — proposed_negatives + forced_positives + learned_nogoods + consume_stats_delta + search/logical branch ids) + `branches/b{i}/integrate.json` (parent-side adopt/reject decisions per item, with reasons). Per-cycle aggregate becomes a summary pointing at the per-child files.
- **T1.5a.20.7** — Distributed-ready `BranchRequest`/`BranchResponse` schema + pickle-roundtrip smoke fixture.
- *Blocks:* S1.5a.19 (T1.5a.19.4 design space lands on the isolated boundary); P1.8 Theme B parallel/distributed execution.

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
