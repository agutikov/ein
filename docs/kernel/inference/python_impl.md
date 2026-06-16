# Inference engine — Python implementation map

The **file-by-file** developer reference for the engine. The *idiomatic*
(language-agnostic, algorithm-level) view — the nine core operations, their
CS analogs, complexity — is
[`architecture_and_algorithms.md`](architecture_and_algorithms.md); this page
is the concrete module map. Source root:
[`ein.py/src/ein/inference/`](../../../ein.py/src/ein/inference/).

> **Audience: engine contributors.** A puzzle author never reads this — the
> authoring surface is [`../ir/03-ein-lang/`](../ir/03-ein-lang/).

## Data flow

```text
KB ─▶ Engine.compile_all ─▶ JoinPlan ─▶ Saturator.saturate ─▶ reasoning facts
        (compile.py)                       (saturator.py)
                                              │ match.py · firing.py
   ┌──────────────────────────────────────────┘
   ▼ on quiescence, no goal:
 hypgen ─▶ apriori (layer N sets) ─▶ commitment.try_commitment_set ─▶ detect
                                       (fork + write + saturate)        │
                              monotonic/solver.py drives the BFS ◀──────┘
                                       └▶ verdict.py reads k → Solution / Ambiguity / Contradiction
```

## Saturation core — the deductive (monotone, append-only) layer

| module | role |
|--------|------|
| [`engine.py`](../../../ein.py/src/ein/inference/engine.py) | `Engine` driver: per-(rule, activator) compile cache; `compile_all` / `compile_for`; tracks `_fired`; `naf_dependency_map` |
| [`compile.py`](../../../ein.py/src/ein/inference/compile.py) | lowers each (rule, activator) to a `JoinPlan` of opcodes: `Scan` / `Join` / `Guard` / `AbsentGuard` (NAF) / `NestedPattern` |
| [`match.py`](../../../ein.py/src/ein/inference/match.py) | runtime matcher: `_run_steps` executes a `JoinPlan`; `_bind_arg` unification; `absents_still_pass` (fire-time NAF re-check) |
| [`firing.py`](../../../ein.py/src/ein/inference/firing.py) | `Firing` record; `fire()` substitutes `:assert`, builds the derived `Fact` with `Provenance.from_rule` |
| [`saturator.py`](../../../ein.py/src/ein/inference/saturator.py) | the fixpoint loop: priority-banded queue, delta-driven (semi-naive) re-enqueue, `_apply` (calls `absents_still_pass` before `fire`), `naf_dropped` |
| [`primitives.py`](../../../ein.py/src/ein/inference/primitives.py) | structural reserved atoms (`not` / `and` / `or` / `absent` / `false`) — `STRUCTURAL` |
| [`predicates.py`](../../../ein.py/src/ein/inference/predicates.py) | computed-predicate registry (`eq` / `neq`) — the `Guard` evaluators |
| [`resolve.py`](../../../ein.py/src/ein/inference/resolve.py) | leaf-node resolution in bindings |

## Hypothesis generation & commitment-lattice search — the non-monotone layer

| module | role |
|--------|------|
| [`hypgen.py`](../../../ein.py/src/ein/inference/hypgen.py) | candidate enumeration (type-blind, S1.7.23); the filter pipeline (`_negated_facts` / already-exists / lookahead / seen); `score_hypothesis`; `HypGenStats` |
| [`hrule.py`](../../../ein.py/src/ein/inference/hrule.py) | hypothesis-rule registry (`hrules` drive generation, never the saturator) |
| [`lookahead.py`](../../../ein.py/src/ein/inference/lookahead.py) | pre-branch one-step death simulator (`enable_pre_branch_lookahead`) |
| [`apriori.py`](../../../ein.py/src/ein/inference/apriori.py) | commitment-lattice layer generation by set-size (prefix-join + no-good prune); `order_candidates` / `_set_score` — the deterministic candidate ordering |
| [`commitment.py`](../../../ein.py/src/ein/inference/commitment.py) | `try_commitment_set`: fork + write hypotheses + saturate + detect; `_is_unconditional` (transitive death walk) |
| [`nogoods.py`](../../../ein.py/src/ein/inference/nogoods.py) | no-good learning: dead set → `root_kb._nogoods`; singletons → `_negated_facts` |
| [`monotonic/solver.py`](../../../ein.py/src/ein/inference/monotonic/solver.py) | **the main loop**: `solve()` — BFS over the commitment lattice; `_phase1_root`, `_phase2_layers`; dedup by `state_hash` |
| [`monotonic/lattice.py`](../../../ein.py/src/ein/inference/monotonic/lattice.py) | `LatticeProof`, `Solution`, `DeadCommitment`, `LatticeStats` |
| [`monotonic/_state.py`](../../../ein.py/src/ein/inference/monotonic/_state.py) · [`_helpers.py`](../../../ein.py/src/ein/inference/monotonic/_helpers.py) | loop state; `_compute_alive` / `_promote_forced_positives` / `_record_node` / `_handle_dead` |
| `monotonic/{state_dump,_lattice_dump,_serialise,snapshot,sanity,contract}.py` | lattice/state dumps, commutativity sanity check, the solver contract |

## Contradiction, verdict, provenance, config

| module | role |
|--------|------|
| [`contradiction.py`](../../../ein.py/src/ein/inference/contradiction.py) | detector: same-layer `(X, ¬X)` pairs + `(false)` |
| [`min_core.py`](../../../ein.py/src/ein/inference/min_core.py) | minimal unsat core (sound, provenance-based) |
| [`verdict.py`](../../../ein.py/src/ein/inference/verdict.py) | `Solution` / `Ambiguity` / `Contradiction`; verdict read from the model count `k`; `goal_bindings` |
| [`solution.py`](../../../ein.py/src/ein/inference/solution.py) | solution-node tracking; `open_hypotheses` |
| [`canon.py`](../../../ein.py/src/ein/inference/canon.py) | `state_hash` — order-insensitive KB dedup |
| [`closed.py`](../../../ein.py/src/ein/inference/closed.py) | `__closed__` handling (`CLOSED` constant; suppress guessing) |
| [`naf_deps.py`](../../../ein.py/src/ein/inference/naf_deps.py) | static NAF-dependency map; `DerivedNafWarning` |
| [`why.py`](../../../ein.py/src/ein/inference/why.py) | `:why` / `:goal-text` template rendering |
| [`config.py`](../../../ein.py/src/ein/inference/config.py) | `SolverConfig` — the live solver flags (`enable_pre_branch_lookahead`, `enable_lookahead_kill_cache`, `hypgen_scoring`, `candidate_order_seed`, `lattice_order`, …) |

## Cross-cutting invariants

- **Append-only KB** — the saturator only adds facts; the one retracting flow
  is `kb.fork()` for a hypothesis branch, which takes a fresh saturator.
- **NAF fire-time re-eval** (S1.5a.1) — `match.absents_still_pass` re-checks
  every `AbsentGuard` before `fire()`; see [`README.md` § NAF semantics](README.md).
- **Alive-set soundness** (the M1 invariant) — rules assert no new objects /
  relations / nested-Fact hypotheses, so `alive = f(closed KB)`; see
  [`README.md` § M1 invariant](README.md).

## See also

- [`architecture_and_algorithms.md`](architecture_and_algorithms.md) — the
  idiomatic (O1–O9, CS-analog) view this map is the code-level companion to.
- [`README.md`](README.md) — design principles, M1 invariant, NAF, determinism.
- [`reserved_engine_strings.md`](reserved_engine_strings.md) — the engine-internal
  reserved atoms these modules key on.
