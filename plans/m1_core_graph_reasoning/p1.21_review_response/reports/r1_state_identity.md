# R1 report — `state_hash` as model identity is collision-unsafe (and layer-polluted)

**Review point:** [REVIEW_M1-01 §1](../../REVIEW_M1-01.md) (P0 — soundness).
**Stage:** [s1.21.1_state_identity.md](../s1.21.1_state_identity.md), task T1.21.1.1 (investigation, read-only).
**Investigated:** 2026-08-16, on `master` @ `db9e396`.

## Verdict

**Confirmed.** [`canon.state_hash`](../../../../ein.py/src/ein/inference/canon.py)
returns a bare Python `int` (canon.py:29-32), and that int **is** model
identity at the verdict-deciding site: `_record_node` keys
`lstate.solution_nodes[h]` on it and *replaces* the stored record on key
equality ([_helpers.py:331-339](../../../../ein.py/src/ein/inference/monotonic/_helpers.py)),
so a hash collision between two distinct complete models silently drops one
of them; `verdict_of` then reads `k = len(nodes)`
([_state.py:144-153](../../../../ein.py/src/ein/inference/monotonic/_state.py))
and an exhausted run **certifies** the collapsed `k=1` as the unique
Solution ([solver.py:159-167](../../../../ein.py/src/ein/inference/monotonic/solver.py)) —
exactly the review's `Ambiguity → Solution` flip, a soundness bug by the
engine's own definition of the result. The identity chain is
`state → hash:int → identity` with **no equality verification anywhere**
(census §1: five more identity-class uses, incl. an equality-*proxy* use in
`sanity.py` where a collision masks a commutativity violation). The
secondary claim is also confirmed: the canonical form includes
`f.layer.value` (canon.py:30) although fact identity everywhere else —
`Fact.__eq__/__hash__` ([entities.py:243](../../../../ein.py/src/ein/kb/entities.py)),
KB dedup ([store.py:280-283, 300-302](../../../../ein.py/src/ein/kb/store.py)),
the fork-diff contract ([commitment.py:171-178](../../../../ein.py/src/ein/inference/commitment.py)) —
is `(relation_name, args)` with layer excluded; the inclusion is provably a
no-op today (§2) and should go. The fix is mechanical and cheap: a measured
zebra2 exhaustive solve has **1 solution node + 67 dead states over 101
enterings** (§3), so keying every identity site on a real `StateKey`
(sorted tuple of canonical facts, ~70 KiB deep, ~0.3 ms to build) costs
≈ 0.03 % latency and ≈ 5 MiB worst-case memory — no digest→bucket
two-step needed at Zebra scale.

## Evidence

Claim → evidence → consequence, each verified by reading the cited lines on
`master` today (line numbers re-checked, not copied from the stage file).

1. **The canonical form is a hash, not a representation.**
   [canon.py:29-32](../../../../ein.py/src/ein/inference/canon.py) —
   `return hash(tuple(sorted((f.layer.value, f.relation_name,
   _hashable_args(f.args)) for f in kb.facts)))`. The sorted canonical
   tuple is *built and immediately discarded*; only the 64-bit SipHash
   survives. Consequence: every downstream comparison of two states is a
   comparison of two ints — `M1 ≠ M2 ∧ hash(M1) == hash(M2)` is
   indistinguishable from `M1 == M2`.

2. **The verdict-deciding dedup replaces on key equality.**
   [_helpers.py:331-339](../../../../ein.py/src/ein/inference/monotonic/_helpers.py) —
   `h = state_hash(node_kb); cur = ctx.lstate.solution_nodes.get(h); if cur
   is None or <lex-smaller commitment>: ctx.lstate.solution_nodes[h] = …`.
   The dict is `dict[int, SolutionRecord]`
   ([_state.py:93](../../../../ein.py/src/ein/inference/monotonic/_state.py)).
   On collision the two distinct models fight for one slot; the loser's
   record is gone. `_finalise_solve` sets `stats.solution_nodes =
   len(lstate.solution_nodes)` (_helpers.py:385) and `verdict_of` maps
   `k=1 → Solution`, `k>1 → Ambiguity` (_state.py:144-153). Consequence:
   the review's `k=2 → reported k=1` scenario is real, and because
   `stats.exhausted=True` on a full sweep, the wrong `k=1` is presented as
   *certified unique* (solver.py:159-167, acceptance
   [test_mode_consistency.py:87-89](../../../../ein.py/acceptance/test_mode_consistency.py)).

3. **Probability is small, but the property is load-bearing.** The hash is
   process-seeded 64-bit SipHash over interned strings; a per-run birthday
   collision among ~10² states is astronomically unlikely — but the number
   `k` is the *definition* of the verdict
   ([monotonic/__init__.py:8-16](../../../../ein.py/src/ein/inference/monotonic/__init__.py)),
   so correctness must be independent of hash quality. The review is right
   to grade this P0 *architecture*, not P0 *incident*.

4. **Layer is in the canonical form but nowhere else in fact identity.**
   canon.py:30 hashes `f.layer.value`; meanwhile
   [entities.py:243](../../../../ein.py/src/ein/kb/entities.py) declares
   `layer: Layer = field(…, compare=False, hash=False)` (docstring
   entities.py:216-221: *"Identity is `(relation_name, args)`; layer …
   not part of identity"*), [store.py:271-277](../../../../ein.py/src/ein/kb/store.py)
   (add_fact: *"Layer is excluded from identity"*, dedup at :280-283),
   store.py:300-302 (`add_and_index_fact` dedups via
   `_fact_by_id(relation_name, args)`), and
   [commitment.py:171-178](../../../../ein.py/src/ein/inference/commitment.py)
   (`_is_new_relative_to`: *"layer + provenance are ignored, matching
   add_fact's dedup contract"*). Note the canonical form is even
   *internally* inconsistent: nested-Fact args recurse through
   `_hashable_args` (canon.py:35-41) **without** their layer — only
   top-level facts carry it.

5. **The layer term is a no-op for dedup today (verified, §2)** — every
   fork-side write is `Layer.REASONING`
   (hypothesis writes [commitment.py:107-111](../../../../ein.py/src/ein/inference/commitment.py);
   rule firings [firing.py:93-101,159](../../../../ein.py/src/ein/inference/firing.py),
   [saturator.py:373](../../../../ein.py/src/ein/inference/saturator.py);
   root-side promotions/writebacks
   [_helpers.py:108-123,159-167](../../../../ein.py/src/ein/inference/monotonic/_helpers.py)),
   and root facts (parse-time ONTOLOGY/FACT,
   [from_ir.py:92-95](../../../../ein.py/src/ein/kb/from_ir.py)) are shared
   by every fork with `add_and_index_fact` keeping the first arrival's
   layer. So within one solve, two branches can never reach the same
   `{(R, args)}` set with different layers — dropping the layer term is
   behaviour-preserving *and* removes a silent-split trap for any future
   engine that writes layers differently.

6. **Persisted dumps carry the hash but are already non-comparable across
   runs.** [_lattice_dump.py:326-328, 377-379](../../../../ein.py/src/ein/inference/monotonic/_lattice_dump.py)
   write `state_hash.txt` / `state_hash_hex`. Measured: two `python3` runs
   of `state_hash` on the same saturated fixture give different values
   (`-2630985118891284384` vs `-2401623520949248430`; identical only under
   `PYTHONHASHSEED=0`) — string-hash randomisation makes the persisted hex
   process-local. Consequence: **there is no dump-compatibility obligation**
   for the migration, and the documented cross-run diff workflow
   ([lattice_dump.md:230](../../../../docs/kernel/inference/lattice_dump.md))
   is already broken as written.

7. **Two doc surfaces overstate the canonicalisation.**
   [architecture_and_algorithms.md:370-374](../../../../docs/kernel/inference/architecture_and_algorithms.md)
   claims the hash excludes "bookkeeping heads", and
   [reserved_engine_strings.md:21-30](../../../../docs/kernel/inference/reserved_engine_strings.md)
   cites a single source `canon.BOOKKEEPING_HEADS` — **no such symbol
   exists** (`grep -rn BOOKKEEPING ein.py/src` → empty); `(hypothesis …)`
   wrappers were retired for a provenance *kind*
   ([provenance.py:99-100](../../../../ein.py/src/ein/kb/provenance.py)), so
   canon.py excludes nothing. Stale docs to fix in the same pass.

8. **Baseline is green.** `pytest tests/inference/lattice
   tests/render/test_lattice_dag.py tests/inference/test_symmetric_hypothesis.py`
   → 380 passed, 1 skipped (29 s). The exhaustive zebra2 acceptance solve
   reproduces `Solution k=1 exhausted=True` (§3 probe).

## 1. Use census — every `state_hash` consumer, identity vs display

**Identity** (dict/set key, equality proxy, or a count that must equal `k`):

| # | site | use | collision effect |
|---|------|-----|------------------|
| I1 | [_helpers.py:331-339](../../../../ein.py/src/ein/inference/monotonic/_helpers.py) `_record_node` → [_state.py:93](../../../../ein.py/src/ein/inference/monotonic/_state.py) `solution_nodes: dict[int, SolutionRecord]` | **THE** solution-node dedup; `k = len(...)` decides the verdict (_state.py:144-153, _helpers.py:385) | **soundness**: model dropped, `Ambiguity`→`Solution`, falsely certified unique |
| I2 | [_helpers.py:264-281](../../../../ein.py/src/ein/inference/monotonic/_helpers.py) `_record_setnode`, "contradictions" keying of `kb_index[h_state]` (merge + `state_hash_merges` tick) | SetNode DAG merge primitive (dormant under `solve` — kb_index left `{}` at _helpers.py:363-367; exercised by unit tests + any DAG builder) | wrong-merge of distinct dead states in the proof DAG |
| I3 | [_helpers.py:283-289](../../../../ein.py/src/ein/inference/monotonic/_helpers.py) "gaps" keying `kb_index[hash(commitment)]` | **also** collision-unsafe — `hash()` of a `CanonicalSetId` used as key where the tuple itself is hashable; codified in [contract.py:65-67](../../../../ein.py/src/ein/inference/monotonic/contract.py) | distinct commitments collapse in the DAG |
| I4 | [snapshot.py:149-157](../../../../ein.py/src/ein/inference/monotonic/snapshot.py) grouping by `node.state_hash`; :186 `solutions=frozenset(state_hash(s.kb)…)`; :187 `deads=frozenset(d.state_hash…)`; fields :110-118 | result-level identity for the shuffle-invariance `==` contract ([test_shuffle_invariance.py:101-108](../../../../ein.py/tests/inference/lattice/test_shuffle_invariance.py), in-process) | snapshot equality becomes less discriminating (test-level, not verdict) |
| I5 | [sanity.py:125,147-148](../../../../ein.py/src/ein/inference/monotonic/sanity.py) `parent_hash != direct_hash` | **equality proxy** for the saturation-commutativity check | collision *masks* a real commutativity violation (false pass) |
| I6 | [answer.py:121,218](../../../../ein.py/src/ein/trace/answer.py) `len({state_hash(b.kb) for b in branches})` | recomputed `k` for the user-facing headline / table | displayed `k` under-counts (mirrors I1) |
| I7 | [state_dump.py:271](../../../../ein.py/src/ein/inference/monotonic/state_dump.py) `ProgressDumper._node_hashes: set[int]` (:206-208) | live "solution-nodes=N" progress counter, documented to match `k` | progress `k` under-counts |
| I8 | [_lattice_dump.py:307,321,344](../../../../ein.py/src/ein/inference/monotonic/_lattice_dump.py) `kb_id_by_state_hash: dict[int, (layer, idx)]` | assigns `kb_index/layer_NN/kb_<i>` folder ids | two nodes share a folder id (dump corruption, not verdict) |
| I9 | acceptance [test_mode_consistency.py:72](../../../../ein.py/acceptance/test_mode_consistency.py) `_distinct_models`, [test_zebra_three_classes.py:115-116](../../../../ein.py/acceptance/test_zebra_three_classes.py) | test-side distinct-model counting | test asserts pass on a collapsed count |

**Display / log only** (no behaviour depends on the value):

| # | site | use |
|---|------|-----|
| D1 | [_lattice_dump.py:314](../../../../ein.py/src/ein/inference/monotonic/_lattice_dump.py) within-layer sort key; :326-328 `state_hash.txt` hex; :377-379 `state_hash_hex` in `proof_summary.json` | deterministic-within-process ordering + hex display; **nothing in the repo reads these back** (grep: writers + existence-asserting [test_lattice_dumper.py](../../../../ein.py/tests/inference/lattice/test_lattice_dumper.py) only), and Evidence 6 shows they are already unstable across processes |
| D2 | [sanity.py:78-83](../../../../ein.py/src/ein/inference/monotonic/sanity.py) `SanityError.__str__` hex formatting (`:#x`) | diagnostic message |
| D3 | [lattice_dag.py:61,70-75,106,121,145](../../../../ein.py/src/ein/render/lattice_dag.py) `_Cell.state_hash` | carried through cells but **never rendered** into the DOT output (the label uses commitment slugs, `_cell_id` hashes label text) |
| D4 | record fields [lattice.py:157](../../../../ein.py/src/ein/inference/monotonic/lattice.py) `DeadCommitment.state_hash`, :179 `SetNode.state_hash` | data carriers — identity-class only via their consumers I4/I8 |
| D5 | counter [lattice.py:110](../../../../ein.py/src/ein/inference/monotonic/lattice.py) `LatticeStats.state_hash_merges` (+ [_state.py:89](../../../../ein.py/src/ein/inference/monotonic/_state.py)) | merge statistic; asserted by [test_lattice_proof.py:129,157](../../../../ein.py/tests/inference/lattice/test_lattice_proof.py), [test_contradictions_backbone.py:204,219](../../../../ein.py/tests/inference/lattice/test_contradictions_backbone.py), [test_lattice_fixtures.py:152](../../../../ein.py/tests/inference/lattice/test_lattice_fixtures.py) |
| D6 | docstrings: [solution.py:43](../../../../ein.py/src/ein/inference/solution.py), [solver.py:9,42,158,356,462](../../../../ein.py/src/ein/inference/monotonic/solver.py), [monotonic/__init__.py:8](../../../../ein.py/src/ein/inference/monotonic/__init__.py), [lattice.py:27,153-157,173-179,208](../../../../ein.py/src/ein/inference/monotonic/lattice.py), [test_symmetric_hypothesis.py:14](../../../../ein.py/tests/inference/test_symmetric_hypothesis.py) | terminology only |
| D7 | [utils/profile_solve.py:62](../../../../utils/profile_solve.py) `("canon/hash", ("canon.py", "state_hash"))` | profiler bucket label (goes silently empty if the symbol renames) |

Verdict of the census: **one soundness-critical identity site (I1)**, one
equality-proxy that can mask a bug (I5), a cluster of `k`-mirroring counts
(I4, I6, I7, I9) that must stay consistent with I1, and a DAG/dump layer
(I2, I3, I8) that should migrate for coherence. Everything else is display.

## 2. Model identity decision — `M = {(R, args)}`, layer dropped

Recommendation: **adopt the review's extensional identity** `M = {(R, args)}`
(with nested-Fact args recursing on `(R, args)` as `_hashable_args` already
does). Grounds:

- **No live behaviour distinguishes branches by layer.** Evidence 5's walk
  of every fact-construction site shows layer is a deterministic function of
  (shared root, extensional fact set) within one solve: all fork-side and
  mid-run root-side writes are `REASONING`; parse-time layers ride the shared
  root into every fork; `add_and_index_fact` never relabels
  (store.py:300-306). Therefore including `f.layer.value` can neither merge
  nor split any pair of states the `(R, args)` projection wouldn't — it is
  dead weight *today*, and a latent state-splitter (spurious `k` inflation
  ⇒ spurious `Ambiguity`) the day any code writes the same proposition at a
  different layer.
- **Every other identity in the system already says so** (Evidence 4):
  `Fact.__eq__/__hash__`, both KB dedup paths, `_is_new_relative_to`. The
  KB *cannot even hold* two same-`(R,args)` facts at different layers, so
  "layer in state identity" has no in-KB semantics to encode.
- **`(not …)` and REASONING-layer hypothesis writes need nothing special**:
  a negation is an ordinary fact `("not", (inner_fact,))` whose *nested*
  layer is already excluded (canon.py:35-41), and hypotheses enter as plain
  positive `(R, args)` facts (commitment.py:107-114) — the extensional set
  distinguishes worlds exactly as the search requires.
- Verification hook for the improvement: acceptance verdicts + bindings
  byte-identical (layer-drop cannot change any current `k` per the argument
  above; the gate proves it empirically).

## 3. `StateKey` design + measurements

```python
# canon.py (target shape)
CanonicalFact = tuple[str, tuple]              # (relation_name, _hashable_args(args))
StateKey      = tuple[CanonicalFact, ...]      # sorted, layer-free

def state_key(kb: KnowledgeBase) -> StateKey:
    return tuple(sorted(
        ((f.relation_name, _hashable_args(f.args)) for f in kb.facts),
        key=repr,                              # total order even for mixed str/int args
    ))
```

- **Identity mechanics**: `StateKey` is used *directly* as the dict/set key
  (I1, I2, I4, I5, I6, I7, I8, I9). Python's dict compares tuples on hash
  collision, so correctness no longer depends on hash quality — verified in
  a probe: two distinct zebra2/minus-15 keys wrapped in a
  constant-`__hash__` subclass still occupy two dict slots.
- **`sorted(key=repr)`, not bare `sorted`**: bare tuple comparison raises
  `TypeError` if two facts share a relation and differ int-vs-str at one
  arg position — latent in `state_hash` today (no mixed-type slots exist in
  the zebra2 closure, probe-verified) but free to fix; `repr` gives a
  deterministic total order and doubles as the dump/render sort key
  (frozensets and heterogeneous tuples must never be `sorted()` raw —
  see [lattice_dag.py:124-126](../../../../ein.py/src/ein/render/lattice_dag.py),
  [snapshot.py:159-169](../../../../ein.py/src/ein/inference/monotonic/snapshot.py),
  [_lattice_dump.py:314](../../../../ein.py/src/ein/inference/monotonic/_lattice_dump.py)).
- **Digest demoted**: `state_digest(kb_or_key) -> int = hash(state_key)`
  survives *only* for hex display in dumps/`SanityError`; **digest alone is
  never identity** (the two-step `digest → bucket → canonical equality` is
  exactly what Python's dict already implements over `StateKey`, so no
  bespoke bucketing layer is warranted).

**Measured on the zebra2 acceptance lattice** (CPython 3.14, this machine;
exhaustive `solve`, `max_set_size=5`, `store_lattice=True`):

| metric | value |
|--------|-------|
| zebra2 exhaustive | `Solution k=1, exhausted=True`, **101 enterings** (34 alive / 67 dead-post), 1 solution node @ 425 facts, **67 DeadCommitments**, 2 layers, 134.5 s |
| zebra2-minus-15 (`stop_after=2`) | `Ambiguity k=2`, 215 enterings, 30 deads, models @ 416 facts, 178 s |
| root-saturated zebra2 KB | 369 facts |
| build cost per key (369 facts) | `state_hash` 0.253 ms → sorted-`repr` `StateKey` 0.338 ms (frozenset-of-canonical 0.150 ms; `frozenset(kb.facts)` 0.034 ms) |
| deep size per key | ~70 KiB incl. strings (strings shared with the KB; fresh tuple structure dominates) |
| whole-lattice cost | 68 stored keys (1 solution + 67 deads) ≈ **≤ 5 MiB**, +34 ms over a 134.5 s solve ≈ **0.03 %** — noise next to the per-`SolutionRecord` full KB snapshots already retained ([lattice.py:114-133](../../../../ein.py/src/ein/inference/monotonic/lattice.py)) |

Conclusion: the plain `StateKey` is affordable at every current site,
including `DeadCommitment` (67 × ~70 KiB worst case); the two-step digest
bucketing stays a *documented fallback* for a future puzzle with orders of
magnitude more dead states, never a licence to compare digests as identity.

**Rejected alternatives** (recorded for the improvement):

- `frozenset(kb.facts)` as the key — semantically perfect (Fact equality is
  already layer-free `(R,args)`) and 7× faster, but each `Fact` pins its
  whole fork KB via `Fact._kb`
  ([entities.py:247](../../../../ein.py/src/ein/kb/entities.py)) — storing it
  in 67 `DeadCommitment`s would retain 67 dead forks. Do not use for stored
  records.
- `frozenset[CanonicalFact]` — fine (0.150 ms, no sort), but every
  display/dump consumer then needs its own `sorted(key=repr)` pass and the
  snapshot's `nodes` tuple needs key-sorting anyway; the sorted tuple keeps
  one canonical serialisation everywhere. Either satisfies soundness; the
  tuple is the stage-file shape and the one recommended.

## 4. Migration inventory (int → StateKey)

Signatures / fields that change type or name:

| where | today | target |
|-------|-------|--------|
| [canon.py](../../../../ein.py/src/ein/inference/canon.py) | `state_hash(kb) -> int` | `state_key(kb) -> StateKey` + `state_digest(...) -> int` (display); `state_hash` **deleted** (no public re-export exists — `inference/__init__.py` and `monotonic/__init__.py` don't export it) |
| [_state.py:87,89,93](../../../../ein.py/src/ein/inference/monotonic/_state.py) | `kb_index: dict[int, SetNode]`, `state_hash_merges`, `solution_nodes: dict[int, SolutionRecord]` | `dict[StateKey \| CanonicalSetId, SetNode]`, `state_key_merges`, `dict[StateKey, SolutionRecord]` |
| [_helpers.py:264,283,331,348,444](../../../../ein.py/src/ein/inference/monotonic/_helpers.py) | `state_hash(...)` calls; gaps key `hash(commitment)` | `state_key(...)`; gaps key = **the commitment tuple itself** (fixes I3) |
| [lattice.py:110,157,179](../../../../ein.py/src/ein/inference/monotonic/lattice.py) | `LatticeStats.state_hash_merges`, `DeadCommitment.state_hash: int = 0`, `SetNode.state_hash: int` | `state_key_merges`, `state_key: StateKey = ()` (× both records) |
| [snapshot.py:110-118,149-157,159-169,173,186-187](../../../../ein.py/src/ein/inference/monotonic/snapshot.py) | `nodes_by_state_hash`, `root_state_hash: int`, `solutions/deads: frozenset[int]` | `nodes_by_state_key` (sorted `key=repr`), `root_state_key: StateKey`, `frozenset[StateKey]` |
| [sanity.py:72-73,83,125,147-148,154-155](../../../../ein.py/src/ein/inference/monotonic/sanity.py) | int compare + `:#x` format | `StateKey` equality (exact); hex via `state_digest` in the message only |
| [state_dump.py:39,206-208,271](../../../../ein.py/src/ein/inference/monotonic/state_dump.py) | `_node_hashes: set[int]` | `set[StateKey]` (progress `k` now exact) |
| [_lattice_dump.py:299,305-307,314,321,326-328,344,377-379](../../../../ein.py/src/ein/inference/monotonic/_lattice_dump.py) | dict keyed by int; hex of the int | dict keyed by `StateKey`; sort `key=repr`; hex from `state_digest` (file names/format unchanged — Evidence 6: no compat obligation; optionally note a stable `sha256(repr(key))` as future work) |
| [contract.py:63-67](../../../../ein.py/src/ein/inference/monotonic/contract.py) | invariant text "state_hash matching its dict key" | restate over `state_key` / commitment keying |
| [answer.py:26,121,218](../../../../ein.py/src/ein/trace/answer.py) | `{state_hash(b.kb)…}` | `{state_key(b.kb)…}` |
| [lattice_dag.py:7,18,25,61,70-75,88,94,106,118-121,124-126,145](../../../../ein.py/src/ein/render/lattice_dag.py) | `_Cell.state_hash: int \| None`; `sorted(snap.solutions)` | `_Cell.state_key`; `sorted(..., key=repr)` |
| [solver.py:9,42,158,356,462](../../../../ein.py/src/ein/inference/monotonic/solver.py), [solution.py:43](../../../../ein.py/src/ein/inference/solution.py), [monotonic/__init__.py:8](../../../../ein.py/src/ein/inference/monotonic/__init__.py), lattice.py docstrings | "deduped by state_hash" | "deduped by canonical `state_key`" |
| acceptance [test_mode_consistency.py:40,72](../../../../ein.py/acceptance/test_mode_consistency.py), [test_zebra_three_classes.py:31,115](../../../../ein.py/acceptance/test_zebra_three_classes.py) | import + set-of-int | `state_key` |
| [whitelist_vulture.py:16,23](../../../../ein.py/whitelist_vulture.py) | `root_state_hash` entry | `root_state_key` |
| [utils/profile_solve.py:62](../../../../utils/profile_solve.py) | `("canon.py", "state_hash")` bucket | `("canon.py", "state_key")` |

**Persisted dumps**: write-only diagnostics (Census D1) — no reader in the
repo, no cross-process stability today ⇒ no migration shim; keep the
`state_hash.txt` / `state_hash_hex` *names* to avoid gratuitous layout
churn, sourced from `state_digest`.

**Docs** (same pass, keep guarantees precise): the canonical-hash claims at
[kernel/inference/README.md:139-146,569](../../../../docs/kernel/inference/README.md),
[architecture_and_algorithms.md:33,98,145,174,370-374](../../../../docs/kernel/inference/architecture_and_algorithms.md)
(drop the false "excluding bookkeeping heads"; :126's unconditional-merge
sentence belongs to R2 — coordinate),
[python_impl.md:50,63](../../../../docs/kernel/inference/python_impl.md),
[reserved_engine_strings.md:21-30,35,144](../../../../docs/kernel/inference/reserved_engine_strings.md)
(delete the nonexistent `canon.BOOKKEEPING_HEADS` table intro or restate as
"none currently"), [lattice_dump.md:107-108,183-185,230](../../../../docs/kernel/inference/lattice_dump.md)
(rewrite the cross-run-diff advice), [ir/01-ein-graph/03_ein_model.md:262](../../../../docs/kernel/ir/01-ein-graph/03_ein_model.md),
[ir/03-ein-lang/04_dot_rendering.md:181](../../../../docs/kernel/ir/03-ein-lang/04_dot_rendering.md).
`plans/` mentions stay untouched (historical archives).
`examples/lattice/03_state_hash_collision.ein` keeps its name (it names the
*same-state merge* scenario, not a hash collision; renaming would churn six
test files for zero semantic gain).

**Tests to add** (new file `ein.py/tests/inference/test_state_key.py`):

1. *Layer-free*: two KBs holding the same `(R, args)` facts installed at
   different `Layer`s ⇒ equal `state_key` (pins §2's decision).
2. *Distinctness*: zebra2 vs zebra2-minus-15 closures ⇒ unequal keys;
   nested-Fact args (Q40) participate recursively.
3. *Forced-collision e2e* (the stage's (b), no real SipHash collision
   needed): monkeypatch `_helpers.state_key` to wrap the real key in a
   `class _Collide(tuple): __hash__ = lambda s: 0`; run `solve` on
   `examples/branching/04_two_levels.ein` (2 models,
   [test_lattice_dag.py:115-126](../../../../ein.py/tests/render/test_lattice_dag.py));
   assert `k == 2` and `Ambiguity` — proves identity survives total digest
   collapse. (Probe-verified the mechanism: two distinct constant-hash keys
   occupy two dict slots.)
4. *Determinism*: two independent parse+saturate runs of one fixture ⇒
   identical keys (probe-verified green today).

**Tests to update**: [test_lattice_sanity.py:105-138](../../../../ein.py/tests/inference/lattice/test_lattice_sanity.py)
(monkeypatch target `sanity.state_hash` → `sanity.state_key`; returned
sentinel values stay ints — any hashable works),
[test_lattice_proof.py:113-133,142-157](../../../../ein.py/tests/inference/lattice/test_lattice_proof.py),
[test_contradictions_backbone.py:182-219](../../../../ein.py/tests/inference/lattice/test_contradictions_backbone.py),
[test_lattice_fixtures.py:128-160](../../../../ein.py/tests/inference/lattice/test_lattice_fixtures.py)
(`state_hash_merges` → `state_key_merges`, `n.state_hash` → `n.state_key`),
[test_shuffle_invariance.py:104-105,141-142](../../../../ein.py/tests/inference/lattice/test_shuffle_invariance.py)
(`nodes_by_state_hash` field rename),
[test_lattice_scoring.py:151-153,170-172](../../../../ein.py/tests/inference/lattice/test_lattice_scoring.py)
(import + set-of-key), [test_render lattice_dag](../../../../ein.py/tests/render/test_lattice_dag.py)
(docstrings + any field access),
[test_symmetric_hypothesis.py:14](../../../../ein.py/tests/inference/test_symmetric_hypothesis.py)
(docstring).

## Recommendation

**One path: replace the int with the canonical representation itself.** Add
`CanonicalFact` / `StateKey` / `state_key(kb)` (layer-free, sorted
`key=repr`) to `canon.py`, delete `state_hash`, key every Census-§1
identity site on `StateKey`, keep a `state_digest` **only** where a hex
string is displayed (dumps, `SanityError`), and rename the merge counter +
snapshot fields so the word "hash" no longer names an identity. This is the
review's own proposal, the stage file's expected shape, and §3 shows it
costs 0.03 % latency / ≤ 5 MiB on the heaviest acceptance lattice — there
is no performance case for anything subtler.

Alternatives noted and rejected: (a) *digest → bucket → equality
verification* — semantically identical to what Python's dict already does
over `StateKey`, so hand-rolling it is pure complexity until a workload
with ≫10³ stored states appears (document as the escape hatch); (b)
*`frozenset(kb.facts)` keys* — fastest and semantically exact, but pins
dead forks via `Fact._kb` (§3); (c) *keep layer in the key* — contradicts
the KB's own identity contract with zero live behaviour depending on it
(§2). A follow-up worth queuing separately (not this change): a *stable*
content digest (e.g. `sha256` of the key's canonical serialisation) for
cross-run dump diffing, which the current seeded hash never provided.

## Improvement inventory

Files T1.21.1.2 will touch (exhaustive; repo-relative):

- `ein.py/src/ein/inference/canon.py` — `StateKey`/`state_key`/`state_digest`; delete `state_hash`; docstring states the guarantee ("identity = canonical representation; any hash is an accelerator").
- `ein.py/src/ein/inference/solution.py` — docstring (:43).
- `ein.py/src/ein/inference/monotonic/_state.py` — dict types, `state_key_merges`, docstrings.
- `ein.py/src/ein/inference/monotonic/_helpers.py` — `_record_node`, `_record_setnode` (incl. gaps keying by commitment), `_root_dead`, `_handle_dead`, `_build_lattice_stats`, docstrings.
- `ein.py/src/ein/inference/monotonic/lattice.py` — `SolutionRecord`-adjacent record fields (`DeadCommitment.state_key`, `SetNode.state_key`), `LatticeStats.state_key_merges`, module docstring.
- `ein.py/src/ein/inference/monotonic/snapshot.py` — `LatticeSnapshotV1` field renames + `StateKey` types, `lattice_snapshot` grouping/sorting.
- `ein.py/src/ein/inference/monotonic/solver.py` — docstrings only.
- `ein.py/src/ein/inference/monotonic/sanity.py` — key equality, `SanityError` fields/format.
- `ein.py/src/ein/inference/monotonic/state_dump.py` — `_node_hashes` → keys; import.
- `ein.py/src/ein/inference/monotonic/_lattice_dump.py` — keying, sort, hex via digest.
- `ein.py/src/ein/inference/monotonic/contract.py` — invariant 5 restated.
- `ein.py/src/ein/inference/monotonic/__init__.py` — docstring.
- `ein.py/src/ein/render/lattice_dag.py` — `_Cell` field, snapshot-field rename follow-through, repr-sorts.
- `ein.py/src/ein/trace/answer.py` — `state_key` for the displayed `k`.
- `ein.py/tests/inference/test_state_key.py` — **new** (tests 1-4 above).
- `ein.py/tests/inference/lattice/test_lattice_sanity.py`
- `ein.py/tests/inference/lattice/test_lattice_proof.py`
- `ein.py/tests/inference/lattice/test_contradictions_backbone.py`
- `ein.py/tests/inference/lattice/test_lattice_fixtures.py`
- `ein.py/tests/inference/lattice/test_shuffle_invariance.py`
- `ein.py/tests/inference/lattice/test_lattice_scoring.py`
- `ein.py/tests/render/test_lattice_dag.py`
- `ein.py/tests/inference/test_symmetric_hypothesis.py` — docstring.
- `ein.py/acceptance/test_mode_consistency.py`
- `ein.py/acceptance/test_zebra_three_classes.py`
- `ein.py/whitelist_vulture.py`
- `utils/profile_solve.py`
- `docs/kernel/inference/README.md`
- `docs/kernel/inference/architecture_and_algorithms.md`
- `docs/kernel/inference/python_impl.md`
- `docs/kernel/inference/reserved_engine_strings.md`
- `docs/kernel/inference/lattice_dump.md`
- `docs/kernel/ir/01-ein-graph/03_ein_model.md`
- `docs/kernel/ir/03-ein-lang/04_dot_rendering.md`

**Risks**

- *Cross-point doc conflicts*: `architecture_and_algorithms.md` (:126 unconditional-merge sentence → R2; :370-374 SOTA para → R5's positioning) and `kernel/inference/README.md` are shared surfaces with R2/R5 improvements — schedule in separate waves or coordinate hunks.
- *Sort discipline*: any `sorted()` over keys or key-sets must pass `key=repr` (frozensets don't total-order; heterogeneous tuples can raise `TypeError`) — three call sites listed in §3.
- *Ambiguity branch order*: `verdict_of` iterates `solution_nodes.values()` in insertion order; keeping the lex-smallest-commitment tie-break in `_record_node` preserves today's deterministic representative — required for the byte-identical acceptance gate.
- *Counter rename* (`state_hash_merges` → `state_key_merges`) changes a `summary.json` stats key; no in-repo reader asserts it ([lattice.py:102-107](../../../../ein.py/src/ein/inference/monotonic/lattice.py) documents the same for field order), but out-of-repo dump consumers (none known) would notice.
- *Memory regression on future workloads*: 67 stored dead keys ≈ 5 MiB on zebra2; a future ≫10³-dead lattice should switch `DeadCommitment` to the documented digest-display + on-demand-key form — leave a NOTE in `lattice.py`.
- *Acceptance runtime*: the gate re-runs the ~2-3 min-per-fixture exhaustive solves (`./run_tests.sh`, Phase 2); the migration adds ≈ 0.03 % — no budget concern.

**Gate** (per stage file): `./run_tests.sh` + `ruff check .` green;
acceptance verdicts/bindings byte-identical; new forced-collision test red
against a simulated digest-identity regression, green on `StateKey`.
