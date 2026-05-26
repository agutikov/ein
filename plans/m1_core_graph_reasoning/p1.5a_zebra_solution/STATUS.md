# P1.5a — status report

**Generated:** 2026-05-26.
**Phase acceptance gate:** [S1.5a.13](s1.5a.13_acceptance_zebra2_solves.md) — zebra2 cold-solve under M1 perf budget (< 60s laptop). **Solve arm SHIPPED 2026-05-26.** Both CPython 3.14 and PyPy3 return `Solution` with the canonical bindings on the laptop reference; engine docs and verdict landed the same day. The trace-fidelity arm of M1 acceptance #3 relocated to [P1.6 S1.6.5](../p1.6_rendering_and_trace/s1.6.5_idea08_trace_acceptance.md) — P1.5a closes on the solve arm alone.

Smoke (depth 1, current canonical) — fresh re-run 2026-05-26 post-`455bfd6`:
- **PyPy3**: `./bench_solve_pypy.sh examples/zebra2.ein --max-depth 1` → `Solution` (h_water=House-1, h_zebra=House-5; tree_nodes=32, leaves dead=31 / solution=1 / open=0; **solve 8.4 s, total wall 9.4 s**; `apriori_dead_in_sweep=28`).
- **CPython 3.14**: `.venv/bin/python ein.py/demo/bench_solve.py examples/zebra2.ein --max-depth 1` → same verdict + bindings (**solve 50.9 s, total wall 51.4 s**; identical tree shape — search is content-deterministic per S1.5a.1a).
- Baseline pre-S1.5a.19: 568 nodes, Ambiguity, 49.9 s on PyPy3. Tree-node delta: **−94 %** (32 vs 568).
- Suite: **696 tests** collected, 696/696 green on CPython in 22.8 s (1 skipped: zebra2 d=1 solve gated on `EIN_RUN_SLOW=1`; passes on PyPy in 9 s). Up from 683 yesterday via T1.5a.19.6 (+13 d=0 negative-completion unit tests + 1 integration).

## Reorganisation 2026-05-26

- **S1.5a.12 → [S1.6.5](../p1.6_rendering_and_trace/s1.6.5_idea08_trace_acceptance.md)** — trace acceptance moved to P1.6 (runs after the trace renderer ships).
- **S1.5a.8 → [S1.7.4](../p1.7_bootstrapping_zebra/s1.7.4_naf_dependency_map.md)**, **S1.5a.10 → [S1.7.5](../p1.7_bootstrapping_zebra/s1.7.5_query_semantics_who_vs_where.md)** — NAF dep map + query semantics moved to P1.7 bootstrap.
- **S1.5a.17 + S1.5a.20 DROPPED** — superseded by [P1.5b](../p1.5b_lattice_search/) set-indexed engines (monotonic + lattice). Files preserved in place with DROPPED headers for historical design notes; P1.5b README's prior dependency on S1.5a.20 was also dropped.

## Stage status table

| ID | Title | Status | Shipped | Open work / blocker |
|----|-------|--------|---------|---------------------|
| [S1.5a.1](s1.5a.1_naf_semantic_rearch.md) | NAF semantic re-architecture | shipped | 2026-05-24 (`ccf2aa8`) | — |
| [S1.5a.1a](s1.5a.1a_branch_order_determinism.md) | Branch exploration order determinism | shipped | 2026-05-24 (`fb04f1a`) | — |
| [S1.5a.2](s1.5a.2_hypgen_pre_pruning_recovery.md) | Hypgen pre-pruning recovery (disjunctive-prune fwd/bwd) | shipped | 2026-05-24 (`00b5ed3`) | — |
| [S1.5a.2a](s1.5a.2a_statistics_determinism.md) | Statistics determinism | partial | 2026-05-24 V1 (`bbe3690`); 2026-05-26 T1.5a.2a.2 (`27aee13`) | T1.5a.2a.2 shipped (knob + demo-only test); fuller S1.5a.16 harness pending; T1.5a.2a.3 doc pending; Q-S1.5a.2a.B parked |
| [S1.5a.5](s1.5a.5_house_to_location_rename.md) | zebra2 rename family (`*-loc`, `co-located`, `adjacent-via`) | shipped | 2026-05-24 (`ef7c69e`) | — |
| [S1.5a.6](s1.5a.6_pypy_compat_perf.md) | PyPy compatibility + perf measurement | partial | 2026-05-25 runner only (`a2a802d`) | T1.5a.6.1–.4 (audit, bench, report, docs) |
| [S1.5a.7](s1.5a.7_hypgen_scoring_branch_info.md) | Hypothesis scoring + branch-info ordering | partial | 2026-05-25 Idea 1 default (`10550d4`) | T1.5a.7.3 (Idea 2 impl, currently `NotImplementedError`); T1.5a.7.4/.5 (empirical compare + recommendation) |
| [S1.5a.11](s1.5a.11_state_dump.md) | State dump harness | shipped | 2026-05-24 v1 + v2 (`6680956`, `691d349`) | non-blocking polish (pre-backprop snapshot, JSON mirror, streaming, round-trippable `.ein`) |
| [S1.5a.13](s1.5a.13_acceptance_zebra2_solves.md) | Acceptance — zebra2 solves uniquely (solve arm) | **shipped (solve arm)** | 2026-05-26 (verdict + measurement + docs) | — (trace arm at P1.6 S1.6.5) |
| [S1.5a.14](s1.5a.14_transitive_backprop.md) | Transitive back-prop | partial | 2026-05-25 Phase 1 (`17bd821`) | Phase 2 dropped with S1.5a.17; Phase 3 absorbed into S1.5a.15; nothing standalone remains |
| [S1.5a.15](s1.5a.15_dead_caching_unsat_core.md) | Dead-branch caching by unsat-core + per-level back-prop | in-progress (design) | — | T1.5a.15.1–.4 (all four phases); ready to start when its perf-headroom slot comes |
| [S1.5a.16](s1.5a.16_branch_order_shuffle_invariance.md) | Branch-order shuffle invariance (depth-bounded) | partial | 2026-05-26 knob + demo harness (`27aee13`) | T1.5a.16.2 zebra2 row deferred (~30 s CPython solve); T1.5a.16.3 triage open; T1.5a.16.4 doc |
| [S1.5a.18](s1.5a.18_path_condition_nogoods.md) | Path-condition no-good clause learning | partial (flag off) | 2026-05-25 mechanism (`2d8cac3`) | T1.5a.18.1–.4 surface in code & tests; default flip gated on demo-10 + zebra2 measurement |
| [S1.5a.19](s1.5a.19_d0_negative_completion_gap.md) | d=0 negative-completion gap + downstream stall + dumper enhancement | **shipped** | 2026-05-25 dumper (`6affc06`); 2026-05-26 rules + per-sibling re-sat (`455bfd6`); 2026-05-26 tests + Q2a/c closure | T1.5a.19.1 → S1.6.5 (NL trace diff lives there); Q2c hardening parked (no observed need at d=1) |

**Legend.** *shipped* = all tasks closed, in git, default-on if config-gated. *partial* = mechanism shipped behind an off-by-default flag, or only a subset of tasks closed. *in-progress (design)* = doc + design exists, code not yet started.

### Relocated or dropped 2026-05-26

| ID | Disposition |
|----|-------------|
| ~~S1.5a.8~~  | → [S1.7.4](../p1.7_bootstrapping_zebra/s1.7.4_naf_dependency_map.md) (NAF dep map relocated to P1.7) |
| ~~S1.5a.10~~ | → [S1.7.5](../p1.7_bootstrapping_zebra/s1.7.5_query_semantics_who_vs_where.md) (query semantics relocated to P1.7) |
| ~~S1.5a.12~~ | → [S1.6.5](../p1.6_rendering_and_trace/s1.6.5_idea08_trace_acceptance.md) (trace acceptance relocated to P1.6) |
| ~~S1.5a.17~~ | DROPPED — superseded by [P1.5b](../p1.5b_lattice_search/) set-indexed engines ([file](s1.5a.17_eager_root_bubble_outer_loop.md) preserved with DROPPED header) |
| ~~S1.5a.20~~ | DROPPED — superseded by [P1.5b](../p1.5b_lattice_search/) `try_set` / per-set `integrate` ([file](s1.5a.20_branch_isolation_rearch.md) preserved with DROPPED header) |

## Acceptance-gate dependency chain

```
S1.5a.13 (zebra2 solves — solve arm)
  ├── S1.5a.1   ✓   (NAF re-eval)
  ├── S1.5a.2   ✓   (disjunctive-prune fwd/bwd)
  └── S1.5a.19  ✓   (d=0 negative completion + per-sibling re-sat) — SOLVE METRIC MET

Trace arm of M1 acceptance #3 now lives at:
  P1.6 S1.6.5 (../p1.6_rendering_and_trace/s1.6.5_idea08_trace_acceptance.md)
    blocked on P1.6 S1.6.4 markdown trace renderer.

Perf headroom (decoupled — land on their own schedules):
  S1.5a.7   partial   (popularity default on; branch-info NIE)
  S1.5a.14  partial   (Phase 1 shipped; Phase 2 dropped with .17; Phase 3 in .15)
  S1.5a.15  design    (not started; promoted-dead caching)
  S1.5a.16  partial   (knob + demo-shuffle test 2026-05-26; zebra2 row + triage open)
  S1.5a.18  partial   (flag off; awaits demo-10 measurement)
  S1.5a.6   partial   (PyPy runner only)
```

**Next.** S1.5a.13 solve-arm finalisation (T1.5a.13.1 measurement write-up, T1.5a.13.3 docs, T1.5a.13.4 verdict) + S1.5a.19 follow-ups (T1.5a.19.2 Q2a/c reproducer, T1.5a.19.6 unit tests + engine README).

## Open tasks by stage

### S1.5a.2a — Statistics determinism (partial)
- **T1.5a.2a.2** — ✓ shipped 2026-05-26 (`27aee13`). `SolverConfig.candidate_order_seed: int = -1` (sentinel for default sort; ≥0 = content-mixed seeded shuffle). `_maybe_shuffle_candidates` wraps the existing `_candidates_for` sort with a `random.Random(seed_str).shuffle()` where `seed_str` joins the config seed with every candidate's `(relation, args)` (PYTHONHASHSEED-independent). `tests/inference/test_shuffle_invariance.py` (13 cases on demos 04/05 × depths 0–2 × seeds 0–1) asserts a five-set invariant — verdict type, root `_negated_facts`, solution-leaf state hashes, leaves-by-verdict Counter, unsat-core union — holds across orderings. **Zebra2 row deferred** (CPython ~30 s/solve blows the in-suite budget); the wider S1.5a.16 harness picks up the fuller param matrix + per-violation triage.
- **T1.5a.2a.3** — Document the invariant in `docs/kernel/inference/README.md` (Statistics determinism subsection).
- **Q-S1.5a.2a.B** — Broader entity back-pointer audit (`Type._kb` / `Relation._kb` / `Instance._kb`); decision deferred until another consumer trips the bug.

### S1.5a.6 — PyPy compatibility + perf (partial)
- **T1.5a.6.1** — Compatibility audit (pytest under PyPy3; categorise + fix/skip failures).
- **T1.5a.6.2** — Perf measurement (`bench_solve` + `bench_saturate` CPython vs PyPy cold vs warm; branching demos).
- **T1.5a.6.3** — Report (compat status + perf table + viability recommendation; bears on P1.8 Theme B urgency).
- **T1.5a.6.4** — Document caveats in `docs/kernel/inference/`.

### S1.5a.7 — Hypothesis scoring + branch-info (partial)
- **T1.5a.7.3** — Implement branch-info ordering (Idea 2); currently raises `NotImplementedError`; needs post-saturation signal integrating with S1.5.7b stable-alive caching.
- **T1.5a.7.4** — Empirical comparison (all 4 scoring modes × zebra2 + 5 branching demos; tabulate tree-node count, wall time, alive-set size, unsat-core size).
- **T1.5a.7.5** — Recommendation report (default-mode choice from T1.5a.7.4).

### S1.5a.13 — Acceptance — zebra2 solves uniquely (solve arm — ✓ shipped 2026-05-26)
- **T1.5a.13.1** — ✓ shipped. PyPy3 8.4 s solve / 32 nodes; CPython 3.14 50.9 s solve / 32 nodes (identical tree shape). Tabulated comparison vs pre-S1.5a.19 (568 nodes, Ambiguity, 49.9 s) lives in the stage file's measurement section.
- **T1.5a.13.3** — ✓ shipped. `docs/kernel/inference/README.md` now carries six S1.5a-era sections: NAF semantics + hypgen pre-pruning + determinism + d=0 negative-completion + mid-sweep saturation/per-sibling re-check + (existing) unconditional-death back-prop.
- **T1.5a.13.4** — ✓ shipped (SHIP). Verdict written at the end of the stage file. Trace arm at P1.6 S1.6.5.

### S1.5a.14 — Transitive back-prop (partial)
- Phase 1 shipped. Phase 2 (outer driver) was absorbed into S1.5a.17 — **dropped 2026-05-26** with that stage (P1.5b's set-indexed engines re-saturate from root natively, so no outer driver is needed). Phase 3 (per-level) absorbed into S1.5a.15. Nothing standalone remains.

### S1.5a.15 — Dead-branch caching by unsat-core (design)
- **T1.5a.15.1** — Direct-dead unsat-core cache (`builder.dead_index: list[_DeadByCore]`; superset short-circuit in `_explore`; bench on `zebra2-hints`).
- **T1.5a.15.2** — Promoted-dead cache via `_promote_verdicts` (`aggregated_unsat_core` field; alive-fp pairing; register in `dead_index`).
- **T1.5a.15.3** — Per-level back-prop (`builder.depth_of[nid]`; `back_propagate_per_level(shallowest_required)`; per-binding analysis on every dead candidate).
- **T1.5a.15.4** — Promoted-dead back-prop — root case was formerly absorbed into S1.5a.17 (now dropped); the **general per-level case (depths ≥ 1)** remains here.

### S1.5a.16 — Branch-order shuffle invariance (partial)
- **T1.5a.16.1** — *Partly shipped 2026-05-26 (`27aee13`).* `SolverConfig.candidate_order_seed: int` + `_maybe_shuffle_candidates` in `solver.py`. The "snapshot serialiser" lives as an inline `_invariants(verdict, root_kb)` projection in the test file.
- **T1.5a.16.2** — *Partly shipped.* `tests/inference/test_shuffle_invariance.py` covers demos 04/05 at depths 0/1/2 × seeds 0/1 in 3.85 s (13 cases). Zebra2 row + the param-matrix expansion deferred — would need a `slow` marker or live in a separate file run via `bench_solve_pypy.sh`. `demo/diff_shuffle.py` not yet written.
- **T1.5a.16.3** — Triage open. Demos 04/05 pass clean across all probed seeds × depths. Promotion required for zebra2 + deeper depths.
- **T1.5a.16.4** — Document invariant in `docs/kernel/inference/README.md` Determinism subsection.

### S1.5a.18 — Path-condition no-good clause learning (partial)
- **T1.5a.18.1** — Storage + ContextVar + config flag (`KnowledgeBase._nogoods`; `_path_ctx` ContextVar; `enable_path_condition_nogoods` ✓ in `config.py`, default `False`).
- **T1.5a.18.2** — `nogoods.py` helpers (✓ module exists: `emit_nogood`, `filter_by_nogoods`).
- **T1.5a.18.3** — Wire into the death sites (`_consume` sweep, `_descend` dead-leaf, `_explore_inner` contradiction-on-entry; three pre-fork filter sites).
- **T1.5a.18.4** — Tests (`test_path_condition_nogoods.py` ✓).
- *Acceptance still open:* demo 10 tree-node count ≤ 20 (vs 32 flag-off baseline); zebra2 clause-set bounded < 200 after depth-3 run; default flip.

### S1.5a.19 — d=0 negative-completion gap + downstream stall (partial — core shipped)
**Status 2026-05-26.** Approach A (ein-lang rules) chosen and shipped: six new rules in `examples/zebra2.ein` + mirrored in `zebra2-hints.ein` close the d=0 negative completion. Combined with a per-sibling apriori Tier-A re-check + mid-sweep `Saturator(kb).saturate(...)` after every `back_propagate(...)` in `_consume`'s sweep, the puzzle now solves at `--max-depth 1`. Numbers (PyPy laptop ref):
- pre-S1.5a.19 baseline: `bench_solve_pypy.sh examples/zebra2.ein --max-depth 4` → `Ambiguity` (568 nodes, 472 dead, 77 open, 0 solutions, 49.9 s).
- post (`455bfd6`): `bench_solve_pypy.sh examples/zebra2.ein --max-depth 1` → `Solution` (h_water=H_1, h_zebra=H_5; 32 nodes; 31 dead / 1 sol / 0 open; **22.3 s**; `apriori_dead_in_sweep=28`).

The six rules added (`functional R` + `(R a b)` ⟹ `(not (R a b_other))` and four parallel shapes): `functional-negative`, `injective-negative`, `co-located-negative`, `adjacent-via-endpoint-{fwd,bwd}`, `adjacent-via-{fwd,bwd}-negative`.

Hypothesis ordering remains **out of scope** — see [S1.5a.7](s1.5a.7_hypgen_scoring_branch_info.md).

- **T1.5a.19.1** — Branch-by-branch NL ↔ engine diff. **Moved to [P1.6 S1.6.5](../p1.6_rendering_and_trace/s1.6.5_idea08_trace_acceptance.md)** alongside the trace acceptance — both are gated on the trace renderer.
- **T1.5a.19.2** — ✓ measured 2026-05-26. Q2a subsumed (28/31 dead leaves caught by apriori re-check; per-sibling sweep effectively prunes the inherited alive-set). Q2b closed by the six-rule landing. Q2c not observed at d=1; the code-path concern (no `ContradictionDetector` after `_consume`'s mid-sweep saturator) is parked — promote to a hardening sub-stage only if a deeper puzzle exhibits sweep-produced contradictions.
- **T1.5a.19.3** — ✓ shipped 2026-05-25 (`6affc06` — dumper enhancement: `00_timeline.jsonl` + `back_prop_writes` / `resat_derivations` split). [`s1.5a.11`](s1.5a.11_state_dump.md) carries the schema.
- **T1.5a.19.4** — ✓ shipped 2026-05-26 (`455bfd6` — approach A chosen, six rules added).
- **T1.5a.19.5** — ✓ shipped 2026-05-26. Solve at d=1 in 32 nodes / 8.4 s on PyPy / 50.9 s on CPython.
- **T1.5a.19.6** — ✓ shipped 2026-05-26. `tests/inference/test_d0_negative_completion.py` ships 13 unit tests (one per rule × positive/negative case) + 1 integration test on zebra2 (skipped on CPython by default, gated on `EIN_RUN_SLOW=1`; passes on PyPy in 9 s). `docs/kernel/inference/README.md` carries the d=0 negative-completion section + mid-sweep saturation section. `s1.5a.11_state_dump.md` updated with the T1.5a.19.3 timeline + per-fact attribution schema.

## Open design questions parked in stage files

These are explicitly parked in their stage files for "decide later" without blocking the active work:

- **Q-S1.5a.2a.B** — Broader entity back-pointer audit (S1.5a.2a).
- **Q-S1.5a.18.A** — Where does parent saturation state matter under future retraction-capable rules? (ship under monotonicity assumption; re-audit when retraction lands).
- **Q-S1.5a.18.B** — Index for `filter_by_nogoods`? (naive is fine at < 200 clauses; defer index until measurement says it matters).
- **Q-S1.5a.18.C** — Trace representation for learned clauses? (out of scope for engine; revisit in P1.6).
- **Q-S1.5a.18.D** — Cross-puzzle clause reuse? (out of scope for M1; perf-followup note).
- **Q-S1.5a.18.E** — Interaction with S1.5a.15 Phase 3? (composition benign — subsumption catches it).

(Q-S1.5a.17.A–.E dropped with S1.5a.17. Q-S1.5a.12.A–.C followed S1.5a.12 to [P1.6 S1.6.5](../p1.6_rendering_and_trace/s1.6.5_idea08_trace_acceptance.md). Q-S1.5a.8.A–.B + Q-S1.5a.10.A–.B followed to P1.7 ([S1.7.4](../p1.7_bootstrapping_zebra/s1.7.4_naf_dependency_map.md) / [S1.7.5](../p1.7_bootstrapping_zebra/s1.7.5_query_semantics_who_vs_where.md)).)

## Cross-links

- Phase intro + design background: [`README.md`](README.md).
- Upstream: [P1.5 hypothesis loop](../p1.5_hypothesis_loop/) (closed through S1.5.8c).
- Downstream:
  - [P1.6 rendering + trace](../p1.6_rendering_and_trace/) — now owns trace-fidelity acceptance via [S1.6.5](../p1.6_rendering_and_trace/s1.6.5_idea08_trace_acceptance.md) (relocated from S1.5a.12 on 2026-05-26).
  - [P1.7 bootstrapping](../p1.7_bootstrapping_zebra/) — now owns [S1.7.4](../p1.7_bootstrapping_zebra/s1.7.4_naf_dependency_map.md) (NAF dep map, formerly S1.5a.8) and [S1.7.5](../p1.7_bootstrapping_zebra/s1.7.5_query_semantics_who_vs_where.md) (query semantics, formerly S1.5a.10).
  - [P1.5b set-indexed engines](../p1.5b_lattice_search/) — supersedes S1.5a.17 + S1.5a.20.
- Followups catalog: [P1.9](../p1.9_hypothesis_loop_followups/).
- Acceptance gates: [M1 README — acceptance](../README.md#acceptance).
- Trace fidelity target: [`docs/ideas/08-human-style-deductive-trace.md`](../../../docs/ideas/08-human-style-deductive-trace.md).
- Engine doc: [`docs/kernel/inference/README.md`](../../../docs/kernel/inference/README.md) (carries the NAF / determinism / eager-bubble notes that several of these stages need to update on ship).
