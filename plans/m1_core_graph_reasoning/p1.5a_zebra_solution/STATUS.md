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
| [S1.5a.2a](s1.5a.2a_statistics_determinism.md) | Statistics determinism | **shipped** | 2026-05-24 V1 (`bbe3690`); 2026-05-26 T1.5a.2a.2 (`27aee13`); closed 2026-05-26 | T1.5a.2a.3 dropped (covered by S1.5a.1a doc); Q-S1.5a.2a.B parked, promote at P1.5b S1.5b.3 if needed |
| [S1.5a.5](s1.5a.5_house_to_location_rename.md) | zebra2 rename family (`*-loc`, `co-located`, `adjacent-via`) | shipped | 2026-05-24 (`ef7c69e`) | — |
| [S1.5a.6](s1.5a.6_pypy_compat_perf.md) | PyPy compatibility + perf measurement | **shipped** | 2026-05-25 runner (`a2a802d`); closed 2026-05-26 | T1.5a.6.2/.3/.4 closed without execution — S1.5a.13.1 has the headline 6.0× data point; broader bench lands naturally when P1.8 Theme B starts |
| [S1.5a.7](s1.5a.7_hypgen_scoring_branch_info.md) | Hypothesis scoring + branch-info ordering | **shipped** | 2026-05-25 Idea 1 default (`10550d4`); closed 2026-05-26 | Idea 2 (branch-info) closed without execution — moot under P1.5b set-indexed engines; popularity reused by S1.5b.26. P1.9 E12 owns long-term informativeness successor. |
| [S1.5a.11](s1.5a.11_state_dump.md) | State dump harness | shipped | 2026-05-24 v1 + v2 (`6680956`, `691d349`) | non-blocking polish (pre-backprop snapshot, JSON mirror, streaming, round-trippable `.ein`) |
| [S1.5a.13](s1.5a.13_acceptance_zebra2_solves.md) | Acceptance — zebra2 solves uniquely (solve arm) | **shipped (solve arm)** | 2026-05-26 (verdict + measurement + docs) | — (trace arm at P1.6 S1.6.5) |
| [S1.5a.14](s1.5a.14_transitive_backprop.md) | Transitive back-prop | **shipped** | 2026-05-25 Phase 1 (`17bd821`); closed 2026-05-26 | Phase 2 dropped with S1.5a.17 (P1.5b absorbs); Phase 3 dropped with S1.5a.15 |
| [S1.5a.16](s1.5a.16_branch_order_shuffle_invariance.md) | Branch-order shuffle invariance (depth-bounded) | **shipped** | 2026-05-26 knob + demo harness (`27aee13`); closed 2026-05-26 | T1.5a.16.2/.3/.4 closed without execution; tree-side regression net is the 13-case test; lattice-side analog at [P1.5b S1.5b.31](../p1.5b_lattice_search/s1.5b.31_lattice_shuffle_invariance.md) |
| [S1.5a.18](s1.5a.18_path_condition_nogoods.md) | Path-condition no-good clause learning | **shipped (flag off)** | 2026-05-25 mechanism (`2d8cac3`); closed 2026-05-26 | Default kept off — measured no win at M1 scope without outer-loop re-entry. Mechanism is forward-going; P1.5b owns the default-flip decision at the monotonic engine. |
| [S1.5a.19](s1.5a.19_d0_negative_completion_gap.md) | d=0 negative-completion gap + downstream stall + dumper enhancement | **shipped** | 2026-05-25 dumper (`6affc06`); 2026-05-26 rules + per-sibling re-sat (`455bfd6`); 2026-05-26 tests + Q2a/c closure | T1.5a.19.1 → S1.6.5 (NL trace diff lives there); Q2c hardening parked (no observed need at d=1) |

**Legend.** *shipped* = all tasks closed, in git, default-on if config-gated. *partial* = mechanism shipped behind an off-by-default flag, or only a subset of tasks closed. *in-progress (design)* = doc + design exists, code not yet started.

### Relocated or dropped 2026-05-26

| ID | Disposition |
|----|-------------|
| ~~S1.5a.8~~  | → [S1.7.4](../p1.7_bootstrapping_zebra/s1.7.4_naf_dependency_map.md) (NAF dep map relocated to P1.7) |
| ~~S1.5a.10~~ | → [S1.7.5](../p1.7_bootstrapping_zebra/s1.7.5_query_semantics_who_vs_where.md) (query semantics relocated to P1.7) |
| ~~S1.5a.12~~ | → [S1.6.5](../p1.6_rendering_and_trace/s1.6.5_idea08_trace_acceptance.md) (trace acceptance relocated to P1.6) |
| ~~S1.5a.15~~ | DROPPED — original motivation closed by S1.5a.19 (32 nodes / 8.4 s, well inside budget); Phases 2/3/4 obsoleted by [P1.5b](../p1.5b_lattice_search/) set-indexed engines; Phase 1 idea may compose with [S1.5b.22](../p1.5b_lattice_search/s1.5b.22_lattice_dedup.md) if lattice measurement exposes dead-redundancy. [File](s1.5a.15_dead_caching_unsat_core.md) preserved with DROPPED header. |
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

Perf headroom (all reviewed + closed 2026-05-26):
  S1.5a.6   ✓ closed   (PyPy runner shipped; 6.0× headline at S1.5a.13.1)
  S1.5a.7   ✓ closed   (popularity default on; branch-info dropped — moot under P1.5b)
  S1.5a.14  ✓ closed   (Phase 1 shipped; Phases 2/3 dropped with .17/.15)
  S1.5a.16  ✓ closed   (knob + 13-case test ship; broader sweep moot, lattice analog at P1.5b S1.5b.31)
  S1.5a.18  ✓ closed   (mechanism ships, default off — no win at M1 scope; flip moves to P1.5b)
```

**P1.5a complete.** All stages reviewed; nothing active left. The active in-tree stages (S1.5a.1/.1a/.2/.2a/.5/.6/.7/.11/.13/.14/.16/.18/.19) are shipped or closed; S1.5a.8/.10/.12 relocated to P1.6/P1.7; S1.5a.15/.17/.20 dropped (P1.5b supersedes). Trace arm of M1 acceptance #3 lives at P1.6 S1.6.5.

## Open tasks by stage

### S1.5a.2a — Statistics determinism (✓ shipped 2026-05-26)
- **T1.5a.2a.1** — ✓ V1 shipped 2026-05-24 (`bbe3690`). `engine._activators_for` reads `self.kb._rule_apps_by_rule` directly; `Rule._kb` back-pointer no longer drives plan compilation. Regression test in `test_saturator_fork_parity.py`.
- **T1.5a.2a.2** — ✓ shipped 2026-05-26 (`27aee13`). `SolverConfig.candidate_order_seed` + `_maybe_shuffle_candidates`; `test_shuffle_invariance.py` covers demos 04/05 × depths 0–2 × seeds 0–1 (13 cases). Zebra2 row deferred to S1.5a.16's harness.
- **T1.5a.2a.3** — dropped 2026-05-26. The invariant is documented implicitly via S1.5a.1a's "Determinism — content-based candidate ordering" section in `docs/kernel/inference/README.md` + the test file's docstring; a separate "Statistics determinism" subsection would add little signal.
- **Q-S1.5a.2a.B** — parked. Broader `Type._kb` / `Relation._kb` / `Instance._kb` audit deferred until P1.5b's set-batch primitive (S1.5b.3) trips the pattern or another consumer surfaces a divergence.

### S1.5a.6 — PyPy compatibility + perf (✓ closed 2026-05-26)
- **T1.5a.6.1** — implicit closure. Full pytest + the EIN_RUN_SLOW=1 integration test all pass under PyPy3; compat is fine.
- **T1.5a.6.2** — closed without execution. The S1.5a.13.1 PyPy/CPython matrix (PyPy 8.4 s vs CPython 50.9 s = 6.0× speedup on zebra2 `--max-depth 1`) is the headline data point. `bench_saturate` + branching-demo expansion is confirmatory only.
- **T1.5a.6.3** — closed. The ≥5× threshold for "changes P1.8 Theme B indexes urgency" is met by the existing data point; a formal report would add weight without signal. P1.8 Theme B can reference the S1.5a.13.1 section directly when it starts.
- **T1.5a.6.4** — closed. `docs/kernel/inference/` carries the PyPy reference path via the `feedback_use_pypy_bench` memory + `bench_solve_pypy.sh` cross-link in the stage file.

### S1.5a.7 — Hypothesis scoring + branch-info (✓ closed 2026-05-26)
- **T1.5a.7.1/.2** — ✓ shipped 2026-05-25 (`10550d4`). Popularity scoring is the M1 default; `most-constrained` kept as escape hatch.
- **T1.5a.7.3** — closed without execution. Branch-info ordering becomes moot under P1.5b's set-indexed engines (selection is per commitment set; the tree-side per-branch `_consume` signal it needed doesn't exist in monotonic/lattice). The `NotImplementedError` config slot stays as a tombstone — cleaned up naturally when the tree solver is deprecated end of P1.5b. P1.9 E12 carries the long-term informativeness successor.
- **T1.5a.7.4/.5** — closed without execution. The default flip to popularity didn't regress the suite; S1.5a.13 closed inside budget so a formal comparison adds no signal. P1.5b S1.5b.26 picks up the within-layer scoring axis natively (reuses S1.5a.7's per-element scorer).

### S1.5a.13 — Acceptance — zebra2 solves uniquely (solve arm — ✓ shipped 2026-05-26)
- **T1.5a.13.1** — ✓ shipped. PyPy3 8.4 s solve / 32 nodes; CPython 3.14 50.9 s solve / 32 nodes (identical tree shape). Tabulated comparison vs pre-S1.5a.19 (568 nodes, Ambiguity, 49.9 s) lives in the stage file's measurement section.
- **T1.5a.13.3** — ✓ shipped. `docs/kernel/inference/README.md` now carries six S1.5a-era sections: NAF semantics + hypgen pre-pruning + determinism + d=0 negative-completion + mid-sweep saturation/per-sibling re-check + (existing) unconditional-death back-prop.
- **T1.5a.13.4** — ✓ shipped (SHIP). Verdict written at the end of the stage file. Trace arm at P1.6 S1.6.5.

### S1.5a.14 — Transitive back-prop (✓ closed 2026-05-26)
- Phase 1 shipped 2026-05-25 (`17bd821`) — `_kb_chain_ctx`, `_consume_caches_ctx`, `_mirror_forced_positive`, ancestor-write + verdict-cache invalidation. Code live in current solver.
- Phase 2 dropped with S1.5a.17 (P1.5b set-indexed engines absorb the outer-driver pattern natively).
- Phase 3 dropped with S1.5a.15 (P1.5b's per-set verdict has no ancestor-chain concept).
- Phase 1's ContextVars get cleaned up naturally when the tree solver is deprecated end of P1.5b.

### S1.5a.16 — Branch-order shuffle invariance (✓ closed 2026-05-26)
- **T1.5a.16.1** — ✓ shipped 2026-05-26 (`27aee13`). `SolverConfig.candidate_order_seed` knob + `_maybe_shuffle_candidates` + inline `_invariants(verdict, root_kb)` projection in the test file. The 13-case `test_shuffle_invariance.py` is the tree-side regression net.
- **T1.5a.16.2/.3** — closed without execution. The original triage taxonomy (state-hash cache class / back-prop-depth class) fed S1.5a.15 phases that are now DROPPED; the broader sweep tests the tree solver that P1.5b deprecates at end of phase.
- **T1.5a.16.4** — closed. S1.5a.1a's existing "Determinism — content-based candidate ordering" section in `docs/kernel/inference/README.md` already names the determinism testing surface; a depth-bounded sibling note would add little signal at the current scope.
- Lattice-side analog (shuffle-invariance over commitment-set visit order) tracked at [P1.5b S1.5b.31](../p1.5b_lattice_search/s1.5b.31_lattice_shuffle_invariance.md).

### S1.5a.18 — Path-condition no-good clause learning (✓ shipped 2026-05-26, flag off)
- **T1.5a.18.1/.2/.3/.4** — ✓ shipped 2026-05-25 (`2d8cac3`). Storage + ContextVar + `nogoods.py` helpers + death-site wiring + tests.
- **Default flip** — closed 2026-05-26 with flag kept off. Measured no win at M1 scope: demo 10 with flag on learns 6 clauses but prunes 0 nodes (same 32-node tree); zebra2 d=1 learns 0 clauses (deaths are depth-1, in `back_propagate`'s 1-element domain). Without outer-loop re-entry (S1.5a.17 dropped), clauses are learned-but-unused within a single solve.
- The mechanism is sound and forward-going — P1.5b's monotonic engine has the layer-by-layer outer loop that consults `_nogoods` between layers (P1.5b README: "unified CDCL prune mechanism for both engines"). Default-flip decision moves to the new engine.

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
