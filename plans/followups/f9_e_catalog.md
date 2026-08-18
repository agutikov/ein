# F9 — Hypothesis-loop follow-ups (the E-catalog) — **CLOSED**

**Status: closed 2026-08-17. No live entries; this file is the ledger.**
All 24 E-entries and the 4 R-rows are settled — shipped, superseded, or
measured-and-rejected. The reason each closed is the point of keeping the
file: it is what stops a future reader rediscovering a measured dead end as
a gap.

History: created 2026-05-21 out of the S1.5.4 implementation-order call
(*"All Open ideas — defer to P1.9"*); **relocated** here from
`m1_core_graph_reasoning/p1.9_…/` when the M1 plan folder was deleted
(P1.22 S1.22.99); 20 entries closed in the 2026-06-15 review, the stubs
deleted 2026-08-17; the last four processed the same day (below). Was a
directory while it carried stage stubs — flattened to one file once it held
only this ledger.

**The deductive-layer perf items** that used to be parked here as 📌
"recorded for visibility, not an entry" (RETE beta-memories,
worst-case-optimal joins) moved to their own theme:
[F11 — deductive-layer perf](f11_deductive_layer_perf.md). They were never
hypothesis-loop items, and with F9 closed they are the live perf work.

## The 2026-08-17 four

The last four entries, each carried to a verdict with a measurement rather
than another deferral. Two shipped as code, two closed.

| ref | idea | outcome |
|-----|------|---|
| E16 | Lazy alive-set materialisation | ✅ **shipped, re-aimed.** The memory framing is refuted: the alive set peaks at ~38 fact-id tuples on zebra2, so streaming `_compute_alive` has nothing to save (and its callers need a set). What was real is the *other* consumer — `solution.complete` built the whole open set only to truth-test it, re-running the per-candidate filter pipeline (one-step lookahead included) for candidates whose value was already irrelevant. It now short-circuits on the generator's first element: 54 ms of a 1.7 s `solve(stop_after=1)`, with 8 of 9 calls answered by candidate #1. The stub's doc-debt note is also discharged — `enable_alive_inherit` was gone from both code *and* the kernel README; only a stale `_compute_alive` link path survived (it lives in `monotonic/_helpers.py`) |
| E20 | Conflict-cache cross-call | ❌ **rejected — measured, and the number is not the point.** Ceiling measured in-process by harvesting a cold run's learnings and preloading them: **+57 %** on zebra2 `stop_after=1` (1694 ms → 731 ms), clearing the stub's own ≥50 % bar. Rejected anyway. (a) The win is *puzzle memoisation*, not reasoning: it is available only when re-solving a byte-identical source, and its maximal form — cache the verdict — is 30 lines and a bigger win, which the project has never wanted. (b) The only in-repo re-run loops are the acceptance gate and the benches, i.e. exactly the places a warm cache falsifies. (c) It would trade the deliverable for the metric: a warm run's dying branch has no refutation derivation to show, and the trace *is* the product ([idea 08](../ideas/08-human-style-deductive-trace.md)). (d) M2's NL pipeline yields fresh puzzles each run, so the "stream of puzzles" case is R2 — already out of scope. (e) The cache is correctness-critical (provenance-free `(not …)` injected into a *pre-saturation* root shifts the NAF fixpoint) for a perf-only feature |
| E23 | Speed up the complete (exhaustive) search | ✅ **shipped — ~2×, uniqueness untouched.** Not via any of its four candidates (all branch-count reducers, all closed: E7 vacuous, E11/E14 rejected, E4 superseded) but by making a *dying* branch cheap instead of rarer. `try_commitment_set` used to saturate a fork to quiescence and only then scan for a contradiction; the KB is append-only, so a fork inconsistent at firing *n* is inconsistent at the fixpoint, and every firing past the clash is provably waste. Measured: the clash lands after ~320 of ~2790 firings — **88 %** of a dying fork's saturation, **64 %** of all fork-saturation time across an exhaustive run. `contradiction.contradicts(kb, fact)` (O(1), one dict lookup per derived fact) + `enable_fail_fast_fork` (default on): exhaustive zebra2 **1.9×** in the [`features.md`](../../docs/kernel/inference/features.md) harness (7.57 s → 3.92 s) and 2.3–2.4× standalone at `max_set_size=5` (8.5 s → 3.7 s), fast path 1.3×, with identical verdict, entering / alive / dead / solution counts, refuted commitments and learned clauses. This also answers the entry's standing question — *"not doing full search, replace it with what???"* — with "keep full coverage; stop paying full price for the branches that fail" |
| E24 | Lattice perf optimisations (F1.a/b/c) | ❌ **closed — measured-refuted, not deferred again.** Two deferrals (2026-05-29, 2026-06-15) weighed the three against thresholds; the fresh profile refutes them outright rather than merely failing to trigger. **F1.a** subset-trie for Apriori-gen: the whole `apriori/elim` subsystem is 0.3 s of a 69 s exhaustive profile (0.4 %), so a perfect trie is a 0.4 % ceiling; `\|A_{k-1}\|` peaks ~19-38 against a trigger of 200. **F1.b** interned set ids: lattice memory is < 20 KB against a trigger of 500 MB. **F1.c** specialised `try_commitment_set`: `fork/copy` measures **0.002 s of 69 s** — its entire win, in the limit, is ~0; the cost it was aimed at was always the saturator downstream, which is what E23 actually cut. Should a puzzle ever push past the F1.a/F1.b thresholds (`\|A_{k-1}\|` > 200; > 500 MB peak), the implementations are textbook and the sketches are in git history (`plans/followups/f9_e_catalog/s1.9.e24_*.md`) |

## Closed 2026-06-15 (stubs deleted 2026-08-17)

**Why the whole search/CSP cluster closed.** Re-judged against the
engine's actual search — a *complete BFS over commitment-set cardinality*
(Apriori), not a DPLL/DFS decision tree — reorderers are inert (within-layer
order cannot change Apriori pruning) and, measured on zebra2, even worse
than lexicographic: LCV ranks the first completer 35th and informativeness
19th, versus lex's 11th, because completers are pre-fork-indistinguishable
and dead-post singletons dominate every cascade signal. The space-shrinkers
went the same way. A complete cardinality-BFS over a connected corpus
leaves no purchase for any of them.

| ref | idea | outcome |
|-----|------|---|
| E1 | `(functional R)` activator | ✅ resolved by the P1.8 stdlib — `functional`/`injective`/`bijective` ship across `std.algebra`/`std.bijection`/`std.elim`/`std.closure`; `single-parent` retired |
| E2 | `(at-most-one R slot)` activator | ✅ resolved differently — at-most-one *is* `functional`/`injective` + `std.closure`; no dedicated activator needed |
| E3 | `:no-hypothesis` query key | ✅ implemented — the exclusion dual of the `:hypothesis-relations` whitelist; blind-enumerator-scoped, saturation untouched |
| E4 | `(symmetry-class R T)` | ⛔ superseded by the symmetric D/A/B/C decomposition. Residual = *object*-value symmetry (lex-leader SBP/SBDD), L-effort and unexercised |
| E5 | Static rule-conflict pre-analysis | ⛔ reframed as rule induction ([F7 §C](f7_rule_induction.md)) — a mutex is a *negative hrule* (zebra2 ships `functional-negative`), so this is companion-rule synthesis, not a hypgen table |
| E6 | Transitive premise walk | ✅ done — `walk_premises` shipped in `provenance.py`; `store.unsat_core` refactored onto it, parity-verified |
| E6a | Tree-solver cleanup | ✅ done — `back_prop.py` deleted, behaviour-identical |
| E7 | Learned clause from unsat-core | ✅ largely resolved — the deriving half ships; the pruning half is **measured vacuous** (all 49 zebra2 deaths are singletons, so nogoods are already Apriori-minimal) *and* unsound under NAF. The remainder was E19, also closed |
| E8 | Watched-fact rule applicability | ⛔ motivation superseded by P1.8a's delta-driven semi-naive saturation; literal watched-literals judged premature |
| E9 | Least-constraining-value | ❌ rejected, measured — worst ordering on zebra2 (see cluster note) |
| E10 | Iterative deepening | ⛔ inapplicable — cardinality layering already *is* breadth-first deepening; there is no DFS depth bound to raise |
| E11 | Goal-driven hypothesis filter | ❌ rejected (user) — you cannot filter a hypothesis without testing it; unsound. The sound variant is cold on the corpus and changes the `solve()` contract |
| E12 | Ordering by "informativeness" | ❌ rejected, measured — the discriminating signal is irreducibly post-fork |
| E13 | Per-hypothesis saturation budget | ❌ dropped (user) — saturation is correctness-critical; a per-fork budget aborts before quiescence, so the fork's verdict is unsound. (Not to be confused with E23's fail-fast, which aborts only *after* the fork is already provably dead — the verdict is settled, not truncated) |
| E14 | Arc-consistency pre-pass | ❌ rejected — subsumed by rule saturation: the engine is append-only (no domains to prune) and the puzzle's elimination rules already derive the negatives AC-3 would |
| E15 | Path-consistency | ❌ rejected — the k-tuple generalisation of E14; eagerly computes what `_nogoods` already builds lazily and Apriori-minimal |
| E17 | Engine-level branch budget | ✅ done — `solve(on_budget="verdict")` returns an `Aborted` verdict with partial stats instead of raising (opt-in) |
| E18 | Rule-applicability pruning | ❌ rejected, measured — drops 0/30 rules on zebra2; generic rules' variable assert-heads can produce any relation, so nothing is provably unreachable |
| E19 | Unsat-core minimisation | ✅ done — shipped, then renamed `frontier.smallest_contradiction_frontier` (P1.21 R3: minimal only over recorded derivations, not a subset-minimal MUS). Re-saturation deletion-MUS is NAF-unsound and was not shipped. Use-site wired by S1.21.7 |
| E21 | `solve` vs `prove` mode split | ⛔ superseded by P1.7a — shipped differently: `solve()` is the exhaustive-with-uniqueness side, `solve(stop_after=1)` the fast side. `Mode` is the orthogonal task-class axis |
| E22 | Alive-hyps in the state hash | ✅ resolved in code — `canon.state_hash` keys on KB facts only; the "extend the hash" fix was deliberately *not* taken |
| R1 | Soft-constraint / probabilistic weighting | ❌ out of scope — M1 is hard-constraint only. Was parked as "an M3 SMT concern"; M3 was dropped 2026-08-18, so soft constraints now have no home at all — a fresh proposal would have to justify them on the graph engine's own terms |
| R2 | Cross-puzzle learning | ❌ out of scope until M2's NL pipeline supplies a stream of puzzles (needs session persistence) |
| R3 | Parallel branch evaluation | ❌ out of scope — engineering; the engine is single-threaded |
| R4 | Domain-specific filters (e.g. spatial) | ❌ rejected — violates the canonical-zebra2 direction: constraints belong in user rules, not engine hardcode |

## What the catalog taught

Worth reading before opening a successor backlog:

- **Every branch-count optimisation failed; the one cost optimisation
  worked.** Nine entries tried to explore fewer branches (reorder, filter,
  pre-propagate, deepen). All inert — a complete cardinality-BFS has to
  cover its layer, and within-layer order cannot change what Apriori
  prunes. The win came from the orthogonal axis nobody had catalogued:
  the *price* of a branch, specifically a failing one (E23).
- **"Where does the time go" and "where is the waste" are different
  questions.** P1.8a answered the first (the matcher, ~95 %) and that
  answer sent four entries to the reject pile for being search-layer. It
  took asking the second to find that **over half** the exhaustive
  wall-clock was that same matcher, running inside forks *already known to
  be dead*.
- **A measured win can still be the wrong feature** (E20): +57 % that
  memoises the puzzle instead of improving the reasoner, and costs the
  explanation.
- **Re-grounding beats re-deferring.** E16's premise, E23's anchor and
  E24's baseline each drifted twice under the engine (`root_alive` gone,
  `Mode.PROVE` never shipped, P1.8a moved the numbers). Two of the four
  final verdicts came from *reading the current code*, not from the stub.

## Cross-links

- [F11 — deductive-layer perf](f11_deductive_layer_perf.md) — the successor;
  D1 (RETE beta-memories) is now the largest remaining lever.
- [F7 — rule induction](f7_rule_induction.md) — the long-term framing
  several closed entries (E1–E5, E11) were interim workarounds for.
- [F10 — M1 refactor-debt tail](f10_m1_refactor_tail/README.md) — the other
  body relocated out of the M1 plans; that one is structural debt, still open.
- [`architecture_and_algorithms.md`](../../docs/kernel/inference/architecture_and_algorithms.md)
  §O5 / §7 — where the shipped E16/E23 mechanisms and the measurements live
  in the kernel docs.
- [docs/lib/02 — solvers / CSP / SAT / SMT](../../docs/lib/02-solvers-csp-sat-smt.md),
  [docs/lib/09 — cognitive architectures](../../docs/lib/09-cognitive-architectures-neurosymbolic.md)
  — external background for the closed CSP/CDCL entries.
- M1 S1.5.4 — the stage that spawned this catalog (plans removed at P1.22;
  see git history). S1.5.5 / S1.5.6 / S1.5.7 were the follow-ups that *did*
  promote inside M1; these are the ones that didn't.
