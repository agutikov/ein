# P1.8a — Performance

**Estimate:** TBD.
**Status:** **CLOSED 2026-06-15.** Split out of [P1.8](../p1.8_ein_lang_modules/)
on **2026-06-02** (TODO: "move performance part into P1.8a, leave ein-language
and lib things in P1.8"); collected every *runtime* optimisation the M1
implementation surfaced, plus the P1.8 stdlib followup that landed here.
**Blocks:** nothing within M1 acceptance — these were always post-M1 speedups.

## Closing summary (2026-06-15)

The phase did its job: **measure before optimising**, then ship only the levers.

- **Profiling settled the levers.** Copy-on-write hypothesis branching (Theme B,
  `B1`–`B4`) and negative-fact volume reduction (Theme C, `C1`–`C3`) were both
  **measured NOT a lever** — `fork()` is `0.000s`, negatives are ~17% of root
  facts with a ~0s detector — and are **parked** (re-open only on a real budget
  overrun). The cost is **re-saturation**.
- **Shipped wins:** the matcher **participation index** (`S1.8.B-indexes`,
  1.67× PyPy) and **incremental delta-driven saturation** (`S1.8.B2v`, D2 +
  D5 semi-naive). B2v has further headroom (deeper incrementalisation) but hit
  diminishing returns; deferred to a future perf phase if a puzzle's branching
  cost demands it.
- **Stdlib followup done:** `S1.8a.f20` (the deferred A8 reconciliation) — full
  bijection-rule migration to `std.algebra` + new `std.bijection`, `:symbols`
  auto-closure, self-contained modules + import dedup, the unexpanded-macro
  guard. zebra2 import block: 4 lines → 2; suite 1205 green.
- `S1.8a.B0` (a coherent `P1.8a.*` renumber) is **moot** — the phase is closed;
  the sticky `s1.8.b*`/`s1.8.c*` ids stay as the historical record.

**Depends on:** the post-M1 engine (`inference/monotonic/`, `kb/store.py`).
Each parked theme re-activates only when its signal arrives (a perf budget
overrun, a puzzle whose branching cost overruns the laptop).

> **Replan note (2026-06-02).** The themes below were lifted verbatim from
> P1.8's old "Theme B / Theme C" sections; their stage-id numbering is
> internally inconsistent (the **stage table** uses `S1.8.B1–B4` for the
> COW chain, while the **theme prose** reuses `B1`=indexes, `B2`=COW,
> `B3`=compression, `B4`=fingerprint, `B5`=participation). Ids are kept
> **sticky** through the move (the files keep their `s1.8.b*`/`s1.8.c*`
> names — same convention as S1.5.9's P1.5→P1.8 relocation). A coherent
> renumber (`P1.8a.*`) is itself a first task here — see B0 below.

## Stages

| ID                | Status | Title / outcome                                | File                                                          |
|-------------------|--------|------------------------------------------------|---------------------------------------------------------------|
| S1.8a.f20         | ✅ **DONE** | (P1.8 stdlib followup) bijection rules → std.algebra + new `std.bijection`; `:symbols` auto-closure; self-contained modules + import dedup; unexpanded-macro guard | [s1.8a.f20_stdlib_property_rules.md](s1.8a.f20_stdlib_property_rules.md) |
| S1.8.B-indexes    | ✅ **SHIPPED** | Participation index for the matcher (1.67× PyPy) | [s1.8.b_indexes.md](s1.8.b_indexes.md)                       |
| S1.8.B2v          | 🟡 **SHIPPED (partial)** | Incremental delta-driven saturation — D2 + D5 semi-naive; further headroom deferred | [s1.8.b2v_incremental_saturation.md](s1.8.b2v_incremental_saturation.md) |
| S1.8.B1–B4        | ⏸ **parked** | Copy-on-write hypothesis branching — measured NOT a lever (`fork()` is `0.000s`) | [b1](s1.8.b1_cow_overlay.md) · [b2](s1.8.b2_engine_lookups.md) · [b3](s1.8.b3_saturator_invariants.md) · [b4](s1.8.b4_perf_bench.md) |
| S1.8.C1–C3        | ⏸ **parked** | Negative-fact volume reduction — measured NOT a lever (~17% of root, ~0s detector) | [c1](s1.8.c1_measure_negative_volume.md) · [c2](s1.8.c2_pick_representation.md) · [c3](s1.8.c3_refactor_saturator.md) |
| S1.8a.B0          | ⊘ **moot** | Perf-theme id renumber — phase closed, sticky ids kept as history | *(this README)* |

The version-COW (B2.v), atom-vector compression (B3), unsat-core
fingerprint (B4), and participation-index (B5) sub-themes are
**prose-only** below (no stage files yet) and fold into the renumber.

## Theme B — performance

> saturation that produces 150 facts and runs 1.5 seconds IS INSANE!
> I estimate Python performance as 1M–10M+ ops/s, so it's already over
> 1M theoretical ops performed. Before saturation we have 64 facts and
> 8 rules. WHAT IS THE SATURATION DOING SO LONG??? — the original motivation.

(NB: since this rant, P1.7b's `_explore_layers` decomposition + the M1
sound search shipped; re-measure before optimising — the baseline moved.)

### Theme B1 — indexes
Speed up every step with indexes: an index of all names; per-category
name indexes (object / relation / rule / …); facts indexed by relation
name, by arity, by object. See [s1.8.b_indexes.md](s1.8.b_indexes.md).

### Theme B2 — Copy-on-write hypothesis branching
`KnowledgeBase.fork()` shallow-copies the `facts` list + the reverse
indexes (O(|facts|)); for the hypothesis loop's many sub-branches that
compounds. The M1 KB is **append-only** (user direction 2026-05-21), so
**true COW** is trivially correct: a fork inherits the parent's facts +
indexes by reference, its own appends go to per-fork overlays, lookups
consult the overlay then fall through to the parent. Stages
[B1](s1.8.b1_cow_overlay.md)–[B4](s1.8.b4_perf_bench.md) (overlay class →
parent-chain lookups → saturator/detector parity → O(1)-fork bench). Out
of scope: GC of collapsed branches (append-only ⇒ nothing freed).

### Theme B2.v — version-based COW + version-based saturation
*(prose-only, 2026-05-27.)* Each `fork()` stamps a monotonically
increasing version; facts carry their introduction version; indexes are
versioned persistent maps; a query at version N returns facts with
version ≤ N in the caller's lineage (O(1) roll-back). **Version-based
saturation:** a `Firing` carries a version; re-saturation after a
back-prop write processes only facts newer than the last
saturator-completion mark — incremental saturation against a version
delta. Useful for P1.5b's per-layer integrate (advance the stamp, let the
delta drive only the changed firings).

### Theme B3 — atom-vector compression
*(prose-only, 2026-05-27.)* For a puzzle with < 256 atoms, an 8-slot flat
fact (head + 7 args) packs into a single 64-bit int (`atom_idx << shift`);
equality / indexing / membership become integer ops (10×+ over
`hash((rel, args))`). Non-flat facts (`(not X)`, `(not (and X Y))`) go via
variable-length `bytes` or sequential `fact_id`s. The integer encoding IS
the hash input (composes with B4).

### Theme B4 — unsat-core fingerprint / consistent hashing
*(prose-only, 2026-05-27.)* The lattice per-set state-hash dedup catches
equivalent kbs but not equivalent *deaths*. A fingerprint over the
unsat-core's source-frontier (exact sorted-tuple hash, or a
Bloom-filter for approximate-superset detection) lets "this death's core
⊆ my prospective kb" return cheap True. Measure first: does the existing
state-hash dedup already catch most of it?

### Theme B5 — fact-participation indexes for re-saturation
*(prose-only, 2026-05-27.)* When a fact lands at version V, only a subset
of the KB can interact with it. Build `atom → [(rule, slot)]`,
`relation → [rule]` (extend `_rule_apps_by_rule`), `atom → [fact_id]`;
re-saturation then walks only the index-narrowed rule × fact subset.
Open: relation-variable params (e.g. `sibling-exclusive (?via ?under)`)
need indexing by their *instantiated* relations.

## Theme C — negative-fact volume reduction

P1.3 saturation produces **lots** of negative facts (zebra.ein's
`(type-exclusivity co-located)` alone derives ~120 `(not …)` vs ~25
positives). The volume is correct but mostly *load-bearing only via the
contradiction detector*. Possible optimisations (each a research
direction): **lazy materialisation** (derive `(not X)` on demand),
**layer-aware filtering** (move mechanically-true negatives to a virtual
layer), **goal-driven pruning** (suppress firings whose conclusions are
demonstrably unconsumed — cross-cuts F7 §C rule-set sufficiency),
**compressed representation** (a single `(not (co-located ? ?_T))` with a
distinct-under-T variable, needs compound-node-kind support, Q26).
Stages [C1](s1.8.c1_measure_negative_volume.md) (measure consumption) →
[C2](s1.8.c2_pick_representation.md) (pick a representation) →
[C3](s1.8.c3_refactor_saturator.md) (refactor saturator + detector +
trace renderer).

## Acceptance (TBD per theme)

- **B:** `fork()` is O(1); zebra2 saturation through a fork matches the
  non-fork baseline byte-for-byte; perf benchmark green (re-measured
  against the post-P1.7b baseline).
- **C:** Zebra REASONING-layer fact count drops ≥ 50% with no loss of
  contradiction-detection power; saturation time within 20% of
  pre-optimisation.

## Cross-links

- Sibling phase: [P1.8 — ein-language + stdlib](../p1.8_ein_lang_modules/)
  (the lang/lib half this split off from).
- Theme B context: P1.2's `KnowledgeBase.fork()` shallow-copy; P1.5
  hypothesis branching's per-fork cost; P1.7b's `_explore_layers`
  decomposition (the post-M1 baseline to re-measure against).
- Theme C upstream design partner:
  [F7 — rule taxonomy + induction](../../followups/f7_rule_induction.md) §C
  (rule-set sufficiency).
