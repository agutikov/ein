# F9 — Hypothesis-loop follow-ups (the E-catalog)

**Status:** parking lot for the post-S1.5.4 hypothesis-loop ideas that
didn't make the M1 acceptance critical path. Created 2026-05-21 in
response to the S1.5.4 implementation-order call (*"All Open ideas — defer
to P1.9"*); **relocated** here from `m1_core_graph_reasoning/p1.9_…/` when
the M1 plan folder was deleted (P1.22 S1.22.99).

**4 of the original 24 entries remain open.** The 2026-06-15 review closed
the other twenty — shipped, superseded, or measured-and-rejected — and
those stubs were deleted rather than kept as tombstones.
[§Closed](#closed) is the one-line record of each, so nothing gets
rediscovered as a gap.

This README is the **authoritative spec** for a live entry until it
promotes to a stage.

**Legend.** 📅 parked (awaiting an activation signal) · 📌 recorded for
visibility, not a P1.9 entry.
*Effort:* S (≤ ½ day) · M (1-3 days) · L (> 3 days).

## Live entries

| ref | idea | mechanism | effort | value | stub |
|-----|------|-----------|--------|-------|------|
| 📅 E16 | Lazy alive-set materialisation | stream `_compute_alive` instead of materialising a `frozenset[FactId]`; saves memory on huge puzzles. **Premise re-grounded** — there is no `root_alive`, and `state_hash` doesn't use the alive set, so the win is smaller than first framed | S | L (memory) | [E16](s1.9.e16_lazy_root_alive.md) |
| 📅 E20 | Conflict-cache cross-call | persist `(not h)` learnings across `solve()` calls when the same puzzle re-runs | M | L | [E20](s1.9.e20_conflict_cache.md) |
| 📅 E23 | Speed up the complete (exhaustive) search | **Re-anchored** from the never-shipped `Mode.PROVE` to `solve()` — P1.7a's complete entry, the actual bottleneck. Open question: can exhaustive search go faster *without* giving up the uniqueness guarantee? Uniqueness is a global property, so some form of full coverage is required; the design question is which form | L | M (perf) | [E23](s1.9.e23_prove_speedup.md) |
| 📅 E24 | Lattice perf optimisations | three optimisations deferred from S1.5b.30 — none passed the measurement-based decision rules on the zebra2 reference; **baseline re-measured 2026-06-15** post-P1.8a and the deferral was reinforced. Queued for re-evaluation if a workload pushes past the trigger thresholds | S each | — | [E24](s1.9.e24_lattice_perf_optimisations.md) |

### Deductive-layer perf — 📌 not hypothesis-loop items

P1.8a's profiling established that **~95% of a solve is the matcher inside
saturation** (O1+O2 — the *deductive* layer), not search
([architecture_and_algorithms.md](../../../docs/kernel/inference/architecture_and_algorithms.md) §7).
Every E-entry is a *search-layer* optimisation, so the catalog's
highest-leverage perf lever has no home here. Recorded so it isn't lost:

| ref | idea | mechanism | effort | value | references |
|-----|------|-----------|--------|-------|------------|
| 📌 D1 | RETE **beta-memories** | persist partial joins across firings (the one thing P1.8a's semi-naive join still recomputes); the named next step up the Datalog ladder | M | **H** (perf) | Arch §6 O1 / §7; Forgy *Rete* (1982) |
| 📌 D2 | Worst-case-optimal join | Leapfrog-Triejoin / Generic-Join — only if cyclic join patterns appear (they don't yet) | L | L (until cyclic) | AGM bound; NPRR (2012) |

Promote these into a reopened perf phase when saturation again dominates
past the P1.8a gains — **not** into this catalog.

## Activation criteria

Entries activate on **measurable need**, not on schedule:

- a user-facing puzzle (Zebra, M2 NL output, M3 SMT slice) exceeds the
  engine's ergonomic time/space envelope **and** a catalog entry is the
  demonstrable bottleneck;
- a regression from a downstream change re-opens the efficiency hole an
  entry closes;
- an empirical study surfaces a specific entry as the highest-leverage
  next move.

Without one of those signals this stays cold.

## Closed

Deleted on 2026-08-17 (P1.22): the entry is settled, so the stub is
history. Kept as one line each because the *reason* is what stops a
future reader reopening it.

**Why the whole search/CSP cluster closed.** Re-judged against the
engine's actual search — a *complete BFS over commitment-set cardinality*
(Apriori), not a DPLL/DFS decision tree — reorderers are inert (within-layer
order cannot change Apriori pruning) and, measured on zebra2, even worse
than lexicographic: LCV ranks the first completer 35th and informativeness
19th, versus lex's 11th, because completers are pre-fork-indistinguishable
and dead-post singletons dominate every cascade signal. The space-shrinkers
went the same way. A complete cardinality-BFS over a connected corpus
leaves no purchase for any of them.

| ref | idea | outcome (2026-06-15 unless noted) |
|-----|------|---|
| E1 | `(functional R)` activator | ✅ resolved by the P1.8 stdlib — `functional`/`injective`/`bijective` ship across `std.algebra`/`std.bijection`/`std.elim`/`std.closure`; `single-parent` retired |
| E2 | `(at-most-one R slot)` activator | ✅ resolved differently — at-most-one *is* `functional`/`injective` + `std.closure`; no dedicated activator needed |
| E3 | `:no-hypothesis` query key | ✅ implemented — the exclusion dual of the `:hypothesis-relations` whitelist; blind-enumerator-scoped, saturation untouched |
| E4 | `(symmetry-class R T)` | ⛔ superseded by the symmetric D/A/B/C decomposition. Residual = *object*-value symmetry (lex-leader SBP/SBDD), L-effort and unexercised |
| E5 | Static rule-conflict pre-analysis | ⛔ reframed as rule induction ([F7 §C](../f7_rule_induction.md)) — a mutex is a *negative hrule* (zebra2 ships `functional-negative`), so this is companion-rule synthesis, not a hypgen table |
| E6 | Transitive premise walk | ✅ done — `walk_premises` shipped in `provenance.py`; `store.unsat_core` refactored onto it, parity-verified |
| E6a | Tree-solver cleanup | ✅ done — `back_prop.py` deleted, behaviour-identical |
| E7 | Learned clause from unsat-core | ✅ largely resolved — the deriving half ships; the pruning half is **measured vacuous** (all 49 zebra2 deaths are singletons, so nogoods are already Apriori-minimal) *and* unsound under NAF. The remainder was E19, also closed |
| E8 | Watched-fact rule applicability | ⛔ motivation superseded by P1.8a's delta-driven semi-naive saturation; literal watched-literals judged premature |
| E9 | Least-constraining-value | ❌ rejected, measured — worst ordering on zebra2 (see cluster note) |
| E10 | Iterative deepening | ⛔ inapplicable — cardinality layering already *is* breadth-first deepening; there is no DFS depth bound to raise |
| E11 | Goal-driven hypothesis filter | ❌ rejected (user) — you cannot filter a hypothesis without testing it; unsound. The sound variant is cold on the corpus and changes the `solve()` contract |
| E12 | Ordering by "informativeness" | ❌ rejected, measured — the discriminating signal is irreducibly post-fork |
| E13 | Per-hypothesis saturation budget | ❌ dropped (user) — saturation is correctness-critical; a per-fork budget aborts before quiescence, so the fork's verdict is unsound |
| E14 | Arc-consistency pre-pass | ❌ rejected — subsumed by rule saturation: the engine is append-only (no domains to prune) and the puzzle's elimination rules already derive the negatives AC-3 would |
| E15 | Path-consistency | ❌ rejected — the k-tuple generalisation of E14; eagerly computes what `_nogoods` already builds lazily and Apriori-minimal |
| E17 | Engine-level branch budget | ✅ done — `solve(on_budget="verdict")` returns an `Aborted` verdict with partial stats instead of raising (opt-in) |
| E18 | Rule-applicability pruning | ❌ rejected, measured — drops 0/30 rules on zebra2; generic rules' variable assert-heads can produce any relation, so nothing is provably unreachable |
| E19 | Unsat-core minimisation | ✅ done — shipped, then renamed `frontier.smallest_contradiction_frontier` (P1.21 R3: minimal only over recorded derivations, not a subset-minimal MUS). Re-saturation deletion-MUS is NAF-unsound and was not shipped. Use-site wired by S1.21.7 |
| E21 | `solve` vs `prove` mode split | ⛔ superseded by P1.7a — shipped differently: `solve()` is the exhaustive-with-uniqueness side, `solve(stop_after=1)` the fast side. `Mode` is the orthogonal task-class axis |
| E22 | Alive-hyps in the state hash | ✅ resolved in code — `canon.state_hash` keys on KB facts only; the "extend the hash" fix was deliberately *not* taken |
| R1 | Soft-constraint / probabilistic weighting | ❌ out of scope — M1 is hard-constraint only; soft constraints are an [M3 SMT](../../m3_smt_integration/) concern |
| R2 | Cross-puzzle learning | ❌ out of scope until M2's NL pipeline supplies a stream of puzzles (needs session persistence) |
| R3 | Parallel branch evaluation | ❌ out of scope — engineering; the engine is single-threaded |
| R4 | Domain-specific filters (e.g. spatial) | ❌ rejected — violates the canonical-zebra2 direction: constraints belong in user rules, not engine hardcode |

## Cross-links

- M1 S1.5.4 — the stage that spawned this catalog (plans removed at P1.22;
  see git history). S1.5.5 / S1.5.6 / S1.5.7 were the follow-ups that
  *did* promote inside M1; these are the ones that didn't.
- [F7 — rule induction](../f7_rule_induction.md) — the long-term framing
  several closed entries (E1–E5, E11) were interim workarounds for.
- [F10 — M1 refactor-debt tail](../f10_m1_refactor_tail/README.md) — the
  other body relocated out of the M1 plans; that one is structural debt,
  this one is feature backlog.
- [docs/lib/02 — solvers / CSP / SAT / SMT](../../../docs/lib/02-solvers-csp-sat-smt.md)
  and [docs/lib/09 — cognitive architectures](../../../docs/lib/09-cognitive-architectures-neurosymbolic.md)
  — external background for the closed CSP/CDCL entries.
