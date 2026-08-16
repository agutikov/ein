# R2 report — `unconditional_facts`: code+docs contradict the live NAF-safe solver

**Review point:** [REVIEW_M1-01 §2](../../REVIEW_M1-01.md) (P0).
**Stage:** [s1.21.2_unconditional_facts.md](../s1.21.2_unconditional_facts.md), task T1.21.2.1 (investigation, read-only).
**Investigated:** 2026-08-16, on `master` @ `db9e396`.

## Verdict

**Confirmed.** [`commitment.py`](../../../../ein.py/src/ein/inference/commitment.py)
still *computes* the unconditional-fact extraction on every alive commitment
(commitment.py:149-156) and still *documents* it as "the soundness-critical
novel piece … the engine merges these into root" (commitment.py:15-20), while
the live solver explicitly refuses the merge as "UNSOUND under NAF"
([solver.py:376-388](../../../../ein.py/src/ein/inference/monotonic/solver.py)) —
and the solver is right: `AbsentGuard` matches contribute **no premise** to a
firing ([match.py:164-172](../../../../ein.py/src/ein/inference/match.py)), so
`_is_unconditional`'s positive-edge DFS
([provenance.reaches](../../../../ein.py/src/ein/kb/provenance.py), provenance.py:282-320)
is structurally blind to dependence-through-absence. A runnable counterexample
(§3 below, executed against the real engine) shows an `absent`-derived fact
classified unconditional under one commitment while a sibling commitment's
world *refutes* it — simulating the documented merge flips that
genuinely-consistent world from `alive` to `dead-post`, i.e. would undercount
`k` and flip the verdict. The exhaustive consumer census (§1) finds **zero
merge consumers**: every reader of `unconditional_facts` is diagnostic
(dump counters / a `.jsonl` writer no CLI path reaches) or a test of the
extraction itself. The unsoundness is therefore *latent*, not live — but the
API + three doc surfaces actively teach the wrong model, exactly the
"conceptually dangerous dormant API" the review flags. Recommendation: **full
removal** (§Recommendation), with the NAF-free-precondition rescope documented
as the rejected alternative.

## Evidence

Claim → evidence → consequence, each verified by reading the cited lines on
`master` today (line numbers are current, not copied from the stage file):

1. **The extraction runs on every alive commitment.**
   [commitment.py:149-156](../../../../ein.py/src/ein/inference/commitment.py)
   — the `# Alive — extract unconditional facts.` block loops `fork.facts`,
   filters by `_is_new_relative_to` (171-178) and `_is_unconditional`
   (181-196), and returns the result in
   `CommitmentSetResult.unconditional_facts` (60-64). Consequence: every alive
   entering pays a provenance DFS per new fact for a value nothing sound
   consumes (§1, §2).

2. **The stated guarantee is "provably true at root".**
   commitment.py:15-20 ("A fact whose entire derivation grounds out at root
   facts … is provably true at root level given ``root + rules``; the engine
   merges these into root"), repeated at the field comment (61-64: "Provably
   true at root level … engine merges these into root") and the function
   docstring (78-79, 96). Consequence: the module contract asserts both a
   false theorem (§3) and a merge that no longer exists (§1).

3. **The provenance walk cannot see negative dependencies.**
   [match.py:164-172](../../../../ein.py/src/ein/inference/match.py): on an
   `AbsentGuard` the matcher continues with `premises` **unchanged** (167-171)
   — the absence that licensed the firing is never recorded in
   `premises_raw`. [provenance.py:282-320](../../../../ein.py/src/ein/kb/provenance.py)
   (`reaches`) walks only `prov.premises_raw` of `kind == "rule"` facts
   (313-318). Consequence: `Deps(Y) = PositiveDeps(Y)` only; the review's
   required `∪ NegativeDeps(Y)` has no carrier in the data model — the test is
   not fixable by a smarter walk without changing what firings record.

4. **The live solver contradicts the module it calls.**
   [solver.py:376-388](../../../../ein.py/src/ein/inference/monotonic/solver.py)
   ("P1.7a — PURE PER-BRANCH search; keep root STABLE. Do NOT merge
   unconditional facts … extraction is UNSOUND under NAF (`absent`)"),
   restated at solver.py:403-409 (the sound inter-layer prune is the
   forced-positive cascade, "NOT the NAF-unsound unconditional-fact merge")
   and solver.py:465-466. The only root writes during search are the
   singleton-death `(not h)` writeback
   ([_helpers.py:80-123](../../../../ein.py/src/ein/inference/monotonic/_helpers.py))
   and the forced-positive promotion (_helpers.py:126-178; the sole
   incrementer of `stats.facts_merged`, _helpers.py:168). Consequence: the
   engine is NAF-safe *today*; only the API and docs are wrong.

5. **The same file still teaches the merge.** solver.py:21-25 — the module
   docstring's termination-condition list says "Solution at root — after
   merging unconditional facts from an alive commitment … 30 unconditional
   facts that complete the puzzle at root" — a self-contradiction 350 lines
   above the do-NOT-merge block. Consequence: even the solver's own docstring
   is on the doc-drift list (§4).

6. **Baseline is green.** `pytest tests/inference/test_commitment.py
   tests/inference/lattice/test_lattice_dumper.py
   tests/inference/monotonic/test_monotonic_dumper.py -q` → 21 passed
   (run 2026-08-16). Consequence: the deletion inventory below starts from a
   passing suite; no pre-existing failure masks the change.

## 1. Consumer census

Grep-verified over `ein.py/src`, `ein.py/tests`, `ein.py/acceptance`,
`utils/`, `docs/` (patterns `unconditional`, `unconditional_facts`,
`_is_unconditional`, `unconditional_facts.jsonl`, `reaches`).

### Producer

| site | role |
|---|---|
| [commitment.py:60-64](../../../../ein.py/src/ein/inference/commitment.py) | `CommitmentSetResult.unconditional_facts` field |
| commitment.py:149-156, 163 | extraction block in `try_commitment_set` (every alive entering) |
| commitment.py:171-178 | `_is_new_relative_to` — used **only** by the extraction |
| commitment.py:181-196 | `_is_unconditional` — the positive-edge DFS |
| [provenance.py:282-320](../../../../ein.py/src/ein/kb/provenance.py) | `reaches` — **its only importer is commitment.py:41**; exported in `__all__` (provenance.py:376) but unused elsewhere in src/tests |

### Consumers of `result.unconditional_facts` (all diagnostic; none merge)

| site | what it does | reachability |
|---|---|---|
| [state_dump.py:137](../../../../ein.py/src/ein/inference/monotonic/state_dump.py) | `unconditional_count=len(...)` into the `MonotonicDumper` timeline record | CLI `ein solve --dump-states` ([cli/solve.py:146-154](../../../../ein.py/src/ein/cli/solve.py)); no test asserts the field |
| [_lattice_dump.py:226](../../../../ein.py/src/ein/inference/monotonic/_lattice_dump.py) | `unconditional_count` in the `LatticeDumper` timeline | library/tests only — the CLI never constructs `LatticeDumper` (cli/solve.py:139-155 builds only `ProgressDumper` / `_TimingDumper` / `MonotonicDumper`) |
| _lattice_dump.py:240-246 | writes `unconditional_facts.jsonl` per non-dead-pre entering (layout comment at :88, docstring :196-201) | same; [test_lattice_dumper.py](../../../../ein.py/tests/inference/lattice/test_lattice_dumper.py) asserts `firings.jsonl` / `unsat_core.jsonl` existence but **never** `unconditional_facts.jsonl` |

**No solver consumer exists.** solver.py references the field only inside the
do-NOT-merge comments (376-388, 407, 465); `facts_merged` is fed exclusively
by forced positives (_helpers.py:168); dead-path dumper calls pass
`facts_merged=0` (_helpers.py:447-455, solver.py:364, 392).

### Tests of the extraction

| site | assertion |
|---|---|
| [test_commitment.py:64](../../../../ein.py/tests/inference/test_commitment.py) | `result.unconditional_facts == ()` (conditional derivation) |
| test_commitment.py:72-98 | `("r",("b","a")) in unconditional_facts` — pins the extraction positively |
| test_commitment.py:101-129 | conditional excluded / unconditional included |
| test_commitment.py:275 | empty-commitment sentinel: `unconditional_facts == ()` |
| test_commitment.py:5 | module docstring: "the soundness-critical novel piece" |

### Docstring/comment references (no data flow)

[_helpers.py:141-147](../../../../ein.py/src/ein/inference/monotonic/_helpers.py)
(`_promote_forced_positives` explains its `<forced-positive>` provenance in
terms of `_is_unconditional`'s walk);
[snapshot.py:75-78](../../../../ein.py/src/ein/inference/monotonic/snapshot.py)
(`root_state_hash` "carries the accumulated unconditional-facts merges" —
false: only forced positives write root); provenance.py:303-304 (`reaches`
docstring cross-ref); [test_shuffle_invariance.py:9-12](../../../../ein.py/tests/inference/lattice/test_shuffle_invariance.py)
(lists "the order in which a layer's alive commitments merge their
unconditional facts into root_kb" as a live leak source — the merge doesn't
exist); [test_lattice_fixtures.py:17-21](../../../../ein.py/tests/inference/lattice/test_lattice_fixtures.py)
(correctly *negative*: "no mid-search unconditional-fact merge" — accurate,
keep); [config.py:44-48](../../../../ein.py/src/ein/inference/config.py)
(historical rename note `enable_back_prop_unconditional` →
`enable_lookahead_kill_cache` — accurate history, keep).

### Sound homonyms (same word, different concept — NOT consumers)

- Trace's empty-commitment label `"∅ (unconditional)"`:
  [linearize.py:8, 62, 141-142, 158](../../../../ein.py/src/ein/trace/linearize.py),
  [render.py:72](../../../../ein.py/src/ein/trace/render.py),
  [golden/trace_3step.md:3](../../../../ein.py/tests/golden/trace_3step.md),
  [test_render.py:83-85](../../../../ein.py/tests/trace/test_render.py),
  [04_dot_rendering.md:205](../../../../docs/kernel/ir/03-ein-lang/04_dot_rendering.md).
  Sound: commitment `()` genuinely assumes nothing.
- "d=0 unconditional spine/saturation":
  [relevance.py:22](../../../../ein.py/src/ein/trace/relevance.py),
  [zebra_walkthrough.md:22](../../../../docs/kernel/inference/zebra_walkthrough.md). Sound.
- "called `_index_fact` unconditionally":
  [store.py:298](../../../../ein.py/src/ein/kb/store.py),
  [test_store_indexing.py:6](../../../../ein.py/tests/kb/test_store_indexing.py). Unrelated English.

### Empty scopes

`ein.py/acceptance/` — zero hits. `utils/` — zero hits
([measure_redundant_firings.py](../../../../utils/measure_redundant_firings.py)
calls `try_commitment_set` for firing counts only, :5).
`docs/api/` — zero hits.

## 2. Soundness verdict per consumer

- **Merge consumers: none.** The unsound operation the docstrings describe is
  not performed anywhere. The engine is NAF-safe *by omission*.
- **`LatticeDumper`'s `unconditional_facts.jsonl`:** dead-diagnostic — not
  CLI-reachable, untested content, and *mislabeled*:
  [lattice_dump.md:101](../../../../docs/kernel/inference/lattice_dump.md)
  annotates it "facts merged back to root", :141-142 "the subset that merged
  back into the shared root (facts true regardless of the hypothesis)". Both
  clauses are false (no merge; "true regardless" is refuted in §3). A
  diagnostic that lies about semantics is worse than no diagnostic.
- **`MonotonicDumper`'s `unconditional_count`:** reachable via
  `ein solve --dump-states`, asserted by no test, meaningless to the model —
  safe to drop from the timeline schema.
- **`test_commitment.py` extraction tests:** they pin the *mechanism*
  (positive-chain-reaches-commitment) correctly, but the property they
  certify ("unconditional ⇒ true at root") is false; they are tests of dead
  code once the field goes.
- **Is a NAF-free precondition checkable?** Yes, in principle:
  [compile.py:471-503](../../../../ein.py/src/ein/inference/compile.py)
  (`naf_watched_relations`) enumerates every `AbsentGuard` per compiled plan,
  surfaced as [`Engine.naf_dependency_map`](../../../../ein.py/src/ein/inference/engine.py)
  (engine.py:92). A ruleset where every plan's watch-list is empty is pure
  monotone Horn; there `premises_raw` is a *complete* dependency record and
  the extraction theorem actually holds (a chain grounding out at root facts
  replays at root by monotonicity). **But nothing enforces this anywhere
  today** — `try_commitment_set` runs the extraction on zebra2 and every
  `absent`-bearing fixture unconditionally (sic) — and no live consumer would
  use the rescued value. A checkable precondition without a consumer does not
  rescue the API; it preserves the trap.

**Net: every use is dead or diagnostic; none is sound-as-labeled.**

## 3. The NAF counterexample, concretised

Executed against `master` (probe under scratch, not committed; transcript
below is the actual output). The fixture is expressible in current ein-lang —
`(absent P)` premises are a shipped feature
([examples/features/01_not_and_absent.ein:33-39](../../../../examples/features/01_not_and_absent.ein)).

```lisp
;; r2-naf-counterexample.ein (sketch — viable as examples/broken/ fixture)
(rule y-when-no-x ()
  :match (and (seed ?s) (absent (x a)))
  :assert (y ?s)
  :why "no x(a) -> y {?s}" :priority 100)
(rule x-y-clash ()
  :match (and (x ?o) (y ?s))
  :assert (false)
  :why "x and y together are inconsistent" :priority 250)
(relation is-a T T)
(relation x T) (relation y T) (relation seed T) (relation g T)
(is-a a T) (is-a s T) (is-a b T)
(seed s)
```

Probe (via `try_commitment_set` directly):

```text
commit {g(b)}: kind = alive | unconditional = [('y', ('s',))]
commit {x(a)} clean root:  kind = alive | fork has y(s): False
commit {x(a)} merged root: kind = dead-post
```

Reading:

1. Under the unrelated commitment `{g(b)}`, the fork derives `(y s)` via
   `absent (x a)`; its only recorded premise is the root fact `(seed s)`
   (match.py:167-171 records nothing for the guard), so
   `_is_unconditional` classifies it **unconditional** — "provably true at
   root given root + rules" per commitment.py:61-64.
2. The commitment `{x(a)}` is a genuinely consistent world (`alive`) in which
   `(y s)` does **not** hold — directly refuting "true at root": a fact true
   at root must hold in every consistent extension of root.
3. Performing the merge the docs describe (root′ = root + `(y s)`) flips
   `{x(a)}` to `dead-post`: a real model is refuted, `k` is undercounted, and
   since [verdict_of](../../../../ein.py/src/ein/inference/monotonic/solver.py)
   reads the verdict off `k`, `Ambiguity` degrades to `Solution` or
   `Solution` to `Contradiction`. This is the review's soundness bug,
   demonstrated end-to-end on the real engine.

**Bonus observation (R4's territory, recorded for cross-ref):** replacing the
direct `{x(a)}` commitment with `h(a)` + rule `h→x` (priority 200 vs the
absent-rule's 100) yields a fork containing **both** `(x a)` and `(y s)` —
the absent-rule fired before `h→x` populated `x`, and the monotone store
never retracts. Fork-internal NAF ordering is a distinct hole from the merge
and belongs to [R4](../s1.21.4_absent_semantics.md)'s `absent`-semantics
formalisation; it does not weaken this counterexample (which commits `x(a)`
directly, making the guard fail at match time) but should be cited by the R4
report.

## 4. Doc-drift list

Every surface still teaching the merge / the false theorem, with the
correction each needs:

| surface | drift | correction |
|---|---|---|
| [commitment.py:1-30](../../../../ein.py/src/ein/inference/commitment.py) module docstring | "extracts unconditional facts" (:4-6); "soundness-critical novel piece … engine merges these into root" (:15-20); cross-ref to `reaches` walk (:26-29) | rewrite: fork-isolation story stays; extraction paragraph replaced by one line pointing to the historical note in kernel README |
| commitment.py:60-64, 78-79, 93-97, 149 | field comment + docstring restate "provably true at root … engine merges" | deleted with the field/extraction |
| [solver.py:21-25](../../../../ein.py/src/ein/inference/monotonic/solver.py) | termination list: "Solution at root — after merging unconditional facts … 30 unconditional facts that complete the puzzle at root" | rewrite: Solution at root comes from the **forced-positive cascade** (`_promote_forced_positives`), no merge |
| solver.py:376-388, 403-409, 465-466 | correct today, but reference an extraction about to disappear | reword to past tense ("the retired extraction was unsound under NAF — see kernel README historical note") |
| [snapshot.py:75-78](../../../../ein.py/src/ein/inference/monotonic/snapshot.py) | `root_state_hash` "carries the accumulated unconditional-facts merges + the forced-positive promotions" | drop the merges clause |
| [_helpers.py:141-147](../../../../ein.py/src/ein/inference/monotonic/_helpers.py) | `<forced-positive>` provenance explained via `_is_unconditional`'s walk | reword: empty-premise rule provenance marks a root-level ground fact for provenance walks generally (`unsat_core` frontier) |
| [docs/kernel/inference/README.md:520-553](../../../../docs/kernel/inference/README.md) | whole §"Unconditional facts — `_is_unconditional` soundness": extraction + merge as live architecture, "asymmetry is load-bearing" soundness argument | rewrite as **historical note**: what was believed (S1.5.7), the NAF counterexample (§3), why P1.7a's keep-root-stable is the model now |
| README.md:430-435 | superseded-tree-solver note ends "the transitive unconditional walk now lives in `commitment._is_unconditional`" | point to the historical note instead |
| README.md:566-567 | monotonic-engine intro: "merges only the unconditional consequences back into a single root KB" | delete the clause; root is stable mid-search |
| README.md:594-600 | termination condition 2: "After merging unconditional facts … cascades into 30 unconditional facts" | rewrite over the forced-positive cascade (matching solver.py:403-409) |
| [lattice_dump.md:94](../../../../docs/kernel/inference/lattice_dump.md) | `post.ein ← root.kb at the end of layer NN (after merges)` | "(after the inter-layer `(not h)` writebacks + forced-positive promotions)" |
| lattice_dump.md:101, 141-142, 169 | `unconditional_facts.jsonl ← facts merged back to root`; "subset that merged back into the shared root (facts true regardless of the hypothesis)"; negatives note | delete the file row + both paragraphs once the writer goes |
| [python_impl.md:48](../../../../docs/kernel/inference/python_impl.md) | module table: "`_is_unconditional` (transitive death walk)" — doubly wrong (not a death walk; about to not exist) | row becomes "`try_commitment_set`: fork + write hypotheses + saturate + detect" |
| [architecture_and_algorithms.md:126-128](../../../../docs/kernel/inference/architecture_and_algorithms.md) | "An alive branch also merges its *unconditional* consequences (`commitment._is_unconditional`) into the root." | delete sentence |
| architecture_and_algorithms.md:356-358 | calls the forced-positive cascade "merging an alive commitment's unconditional consequences (`commitment._is_unconditional`) into the root" — conflates two mechanisms | describe the cascade as singleton-alive promotion (`_promote_forced_positives`) |
| [03_ein_model.md:293-294](../../../../docs/kernel/ir/01-ein-graph/03_ein_model.md) | `(false)` dedup justified by "the unconditional-death analysis (`commitment._is_unconditional`, S1.5.7)" | justify via the unsat-core frontier walk (`KnowledgeBase.unsat_core` / `walk_premises`) |
| [test_shuffle_invariance.py:9-12](../../../../ein.py/tests/inference/lattice/test_shuffle_invariance.py) | lists the unconditional-fact merge order as a live leak source | reword to the two real suspects (nogood order, multilabel rep) |
| [test_commitment.py:5](../../../../ein.py/tests/inference/test_commitment.py) | "the soundness-critical novel piece" | docstring rewritten with the extraction tests' removal |

Plan-archive files (`plans/m1_core_graph_reasoning/p1.5*/…`,
[s1.5.7_back_prop_unconditional.md](../../../m1_core_graph_reasoning/p1.5_hypothesis_loop/s1.5.7_back_prop_unconditional.md),
[f5_rules_as_data.md:138](../../../followups/f5_rules_as_data.md), etc.) are
historical records of stages as they shipped — **out of scope**, per the
repo's plans-are-history convention. No phase doc outside `docs/kernel/`
presents the merge as *current* shipped behaviour;
[plans/m1_core_graph_reasoning/README.md:73](../../README.md) already lists
the retirement as P1.21's task.

## Recommendation

**Full removal (chosen).** Delete `CommitmentSetResult.unconditional_facts`,
`_is_unconditional`, `_is_new_relative_to`, the alive-branch extraction block,
both dumpers' `unconditional_count` fields, the `unconditional_facts.jsonl`
writer, and — cascade — `provenance.reaches` (its **only** importer is
commitment.py:41; no test exercises it directly;
`walk_premises` (provenance.py:323+) remains the shared premise-closure
substrate for `unsat_core`/min-core). Rewrite the doc surfaces per §4, with
[README.md](../../../../docs/kernel/inference/README.md) §520 becoming the
single historical note (belief → counterexample → why keep-root-stable is the
model) that every other correction links to.

Why this path:

- **Zero consumers to preserve** (§1): removal breaks no engine behaviour;
  `solve()` output is byte-identical (the extraction influenced nothing).
- **Removes cost from the hot path**: one provenance DFS per new fact per
  alive entering, currently paid on every solve for a value that is
  discarded.
- **Kills the trap**: a dataclass field named "provably true at root" is an
  invitation to reintroduce the merge; the review is explicit — don't keep a
  conceptually dangerous dormant API.

**Alternative (rejected): rescope under a checked NAF-free precondition** —
gate the extraction on `Engine.naf_dependency_map` being empty
(engine.py:92 / compile.py:471-503), where the theorem genuinely holds (§2).
Rejected because no consumer exists to justify the maintenance surface; if a
future stage wants root-merge acceleration for pure-Horn puzzles, this report
plus the compile-level check is the documented resurrection path, and the
right dependency carrier then is ATMS-style environments
(`Deps = PositiveDeps ∪ NegativeDeps`, review §2/§4 — cross-ref
[R4](../s1.21.4_absent_semantics.md) and [R6](../s1.21.6_architecture_seam.md)).

**Closeout grep (scoped).** The stage's "zero `unconditional` refs outside
history docs" must target the API names, not the bare word — the census found
six *sound* homonym families (§1: trace's `∅ (unconditional)` label, d=0
spine, "unconditionally called", config.py's rename note). Gate on:
`grep -rn "unconditional_facts\|_is_unconditional" ein.py/src ein.py/tests
ein.py/acceptance utils docs` → zero hits outside the kernel README
historical note and `plans/`.

## Improvement inventory

Files T1.21.2.2 will touch (exhaustive — scheduling basis):

**Code (deletions + docstring rewrites)**

1. `ein.py/src/ein/inference/commitment.py` — drop field (:60-64), extraction
   block (:149-156, 163), `_is_new_relative_to` (:171-178),
   `_is_unconditional` (:181-196); prune the `FactId`/`reaches` import (:41);
   rewrite module + function docstrings (fork-isolation story stays).
2. `ein.py/src/ein/kb/provenance.py` — delete `reaches` (:282-320) + its
   `__all__` entry (:376); no other src/test importer exists.
3. `ein.py/src/ein/inference/monotonic/solver.py` — module docstring :21-25;
   reword comments :376-388, :403-409, :465-466 to historical references.
4. `ein.py/src/ein/inference/monotonic/_helpers.py` — reword :141-147.
5. `ein.py/src/ein/inference/monotonic/snapshot.py` — fix :75-78.
6. `ein.py/src/ein/inference/monotonic/state_dump.py` — drop
   `unconditional_count` from the timeline record (:137).
7. `ein.py/src/ein/inference/monotonic/_lattice_dump.py` — drop layout line
   :88, docstring :196-201, timeline field :226, writer :240-246 (keep the
   `firings.jsonl` half of the block).

**Tests**

8. `ein.py/tests/inference/test_commitment.py` — delete
   `test_alive_unconditional_derivation_from_root_facts` (:72-98) and
   `test_alive_conditional_excluded_from_unconditional_facts` (:101-129);
   strip `unconditional_facts` assertions from :64, :275; rewrite module
   docstring (:1-8).
9. `ein.py/tests/inference/lattice/test_shuffle_invariance.py` — docstring
   :9-12.
10. `ein.py/tests/inference/lattice/test_lattice_fixtures.py` — comment
    :17-21 (optional touch; the negative statement stays accurate).
11. `ein.py/tests/kb/test_provenance.py` — reword the `walk_premises`
    docstring's "dual of `reaches`" (:412) after `reaches` is deleted.
12. `ein.py/tests/inference/monotonic/test_root_stability_naf.py` — **new**:
    pin §3 as a regression — solve the r2 NAF fixture, assert the root KB's
    fact set is unchanged across the search (no `absent`-derived fork fact
    leaks to root) and the `{x(a)}`-style world stays satisfiable.

**Docs**

13. `docs/kernel/inference/README.md` — §520-553 → historical note; :430-435,
    :566-567, :594-600.
14. `docs/kernel/inference/lattice_dump.md` — :94, :101, :141-142, :164-172.
15. `docs/kernel/inference/python_impl.md` — :48.
16. `docs/kernel/inference/architecture_and_algorithms.md` — :126-128,
    :356-358.
17. `docs/kernel/ir/01-ein-graph/03_ein_model.md` — :293-294.

**Phase bookkeeping**

18. `plans/m1_core_graph_reasoning/p1.21_review_response/s1.21.2_unconditional_facts.md`
    — status/closeout.
19. `plans/m1_core_graph_reasoning/p1.21_review_response/reports/r2_unconditional_facts.md`
    — closeout grep transcript appended (this file).

**Tests to add:** item 12 (root stability under NAF); optionally promote the
§3 fixture to `examples/broken/` or `examples/features/` if R4 wants a shared
fixture — coordinate with T1.21.4.2 to avoid a duplicate.

**Risks**

- *Dump-schema change:* `unconditional_count` disappears from both dumpers'
  timeline records and `unconditional_facts.jsonl` from the lattice layout.
  No test, `utils/` script, or acceptance run reads either (census §1);
  external consumers of ad-hoc `--dump-states` output would see a removed
  field — mitigated by the lattice_dump.md update (item 14).
- *Public-surface shrink:* `reaches` leaves `provenance.__all__`. No known
  importer; `vulture` (whitelist checked — no entry) and `ruff` stay green
  either way.
- *Conflict with parallel P1.21 tasks:* R4's report may cite
  commitment.py/README.md lines that move; R5 rewrites
  architecture_and_algorithms.md's CDCL section (:350-361) adjacent to item
  16's :356-358 — schedule T1.21.2.2 and T1.21.5.2 in different waves, or
  hand R5 the post-R2 line numbers.
- *Golden files:* `tests/golden/trace_3step.md` keeps `∅ (unconditional)` —
  intentionally untouched (sound homonym); do not let a broad
  search-and-replace rename the trace label.

## Closeout (2026-08-16, T1.21.2.2)

Full removal executed per §Recommendation. Closeout grep (scoped to the API
names, per §Closeout grep):

```text
$ grep -rn "unconditional_facts\|_is_unconditional" \
    ein.py/src ein.py/tests ein.py/acceptance utils docs
docs/kernel/inference/README.md:527:> ([report](../../../plans/m1_core_graph_reasoning/p1.21_review_response/reports/r2_unconditional_facts.md)):
docs/kernel/inference/README.md:529:> `CommitmentSetResult.unconditional_facts`, `commitment._is_unconditional`,
docs/kernel/inference/README.md:531:> `unconditional_facts.jsonl` lattice-dump writer. This note records what
```

All three hits are inside the kernel README **historical note** (the removal
inventory it records) — zero hits elsewhere, as gated. The sound homonyms
(trace `∅ (unconditional)` label, d=0 spine, store.py's adverb, config.py's
rename note, `tests/golden/trace_3step.md`) are untouched. Regression added:
[`tests/inference/monotonic/test_root_stability_naf.py`](../../../../ein.py/tests/inference/monotonic/test_root_stability_naf.py)
(§3 pinned at the primitive level + a solve-level root-byte-stability run).
`./run_tests.sh` + `ruff check .` green.
