# P1.5b — Per-candidate processing in layer N

Companion to [`diagrams/algorithm_layer_n.dot`](diagrams/algorithm_layer_n.dot).
Renders the per-candidate flow inside one BFS layer; the
layer-wrapping loop (enumerate → process → close layer →
recompute alive → next layer) sits one level above.

## Render

```sh
dot -Tsvg diagrams/algorithm_layer_n.dot -o diagrams/algorithm_layer_n.svg
# or
dot -Tpng diagrams/algorithm_layer_n.dot -o diagrams/algorithm_layer_n.png
```

The diagram uses six colour bands:

| colour            | meaning                                                |
|-------------------|--------------------------------------------------------|
| light blue        | setup / data flow                                       |
| light cyan        | accumulators (forced positives, proposed negatives)     |
| light yellow      | decision diamonds                                       |
| light gray        | optional / disabled-by-default                          |
| pale green        | ALIVE outcome (and bubbled forward)                     |
| medium sea green  | SOLUTION (terminal)                                     |
| salmon            | DEAD outcome                                            |
| plum              | back-propagation / nogood emission step                 |

## Algorithm

For layer ``N`` with frozen ``A_1, …, A_{N-1}`` and the current
``root._nogoods``:

### 1. Generate candidates (Apriori prefix-join)

Sort every set in ``A_{N-1}`` canonically (Q1.5b.3 — tuple of
sorted ``FactId``s); for each prefix-class (sets agreeing on
their first ``N-2`` elements), pair every two members
``(a_1, …, a_{N-2}, b)`` and ``(a_1, …, a_{N-2}, c)`` with
``b < c``; emit ``(a_1, …, a_{N-2}, b, c)`` once. Each
size-``N`` candidate produced exactly once.

### 2. Filter

Drop ``C`` if any of:

- some ``(N-1)``-subset of ``C`` is **not** in ``A_{N-1}``
  (covered by construction with prefix-join on closed
  ``A_{N-1}``);
- ``matches_any_nogood(C, root._nogoods)`` — some learned
  clause is a subset of ``C``, so committing all of ``C``
  triggers the known contradiction;
- some element of ``C`` is no longer in the current alive set
  (a `(not h)` landed at root since layer-``N-1`` closed —
  e.g., from a forced-positive bubble or another layer-``N``
  candidate's death within this same layer under Option A
  cadence).

### 3. For each surviving candidate ``C`` of size ``N``

#### 3a. Find parents

A size-``N`` set ``C`` has **``N`` parents** in the lattice
DAG — the ``N`` distinct ``(N-1)``-subsets of ``C``. Each is
in ``A_{N-1}`` by the Apriori invariant; lookup is a
dict-by-canonical-tuple in the lattice index.

#### 3b. (optional, disabled by default) Sanity check

For each parent ``P_i`` of ``C``: fork ``P_i.kb_snapshot``,
add ``h_i = C \ P_i``, saturate; verify every resulting
post-saturation kb has the same ``state_hash``. This validates
**saturation commutativity** end-to-end — if it fails on any
fixture, the monotone-rule-set premise is broken and the
lattice's "set determines kb" invariant is invalid for that
fixture.

Off by default because the check costs ``N × saturation`` per
candidate; ship it as `--lattice-sanity-check` on
`bench_dag.py` and run it as a regression on
`examples/branching/*` + `examples/zebra2.ein` once per
release.

#### 3c. Fork → add → pre-saturation contradiction check

Fork **any one** parent's kb (commutativity ⟹ choice is
immaterial); write the new hypothesis ``h = C \ P`` to the
fork. Before saturating, run
``ContradictionDetector(fork).detect()``. This is the
**apriori filter** — catches:

- ``(not h)`` already in the parent's KB (would mean ``h`` is
  not in alive — should be caught at step 2, defensive here);
- a derived ``(not X)`` against a fact the parent has, where
  adding ``h`` introduces ``X`` directly (rare);
- a ``(false)`` fact already in the parent's kb (parent itself
  was contradictory — should not happen for alive parents,
  defensive).

Note (user, 2026-05-25): under M1 all hypotheses are positive
facts; negative hypotheses are a follow-up. The pre-sat
contradiction filter still applies but its yield is small —
most deaths surface only after saturation.

If pre-sat contradiction fires → **DEAD (pre-sat)** (3d.i).
Else → continue to 3d.ii.

#### 3d. Outcomes

##### 3d.i. DEAD (pre-sat)

- ``unsat_core`` = the witness facts the detector returned
  (typically the new ``h`` + one prior fact in the kb).
- Emit clause: ``frozenset(C)`` into ``root._nogoods`` via
  ``emit_nogood`` (subsumption-aware insert — Q1.5b.5.b).
- Record a `SetNode` in ``D_N`` with `verdict = "dead"`,
  carrying the unsat_core and the empty `forced_positives`
  + `proposed_negatives` (we never saturated).
- **Bubble up via integrate**: for **each** of ``C``'s ``N``
  parents ``P_i``, call ``integrate(P_i.kb, C's BranchResult)``.
  The integrate step on a pre-sat-dead child:
    - merges the clause ``frozenset(C)`` into ``P_i._nogoods``
      (idempotent under subsumption — writing the same clause
      ``N`` times is a no-op after the first);
    - does **not** propagate forced positives / proposed
      negatives (none exist).
- The recursive bubble continues upward through ``P_i``'s own
  parents until the clause reaches root's ``_nogoods``.

##### 3d.ii. Saturate

``Saturator(fork).saturate()``.

##### 3d.iii. Post-saturation state-hash dedup checkpoint

**Before** scanning firings or running any further detection,
compute ``state_hash(fork)`` and look it up in the lattice's
state-hash index:

- **Hit** — some previously-recorded `SetNode` ``E`` has the
  same post-saturation state hash. Two distinct commitment
  sets have converged on the same kb (Q1.5b.4.a merge case).
  Action:
  - Append ``C`` to ``E.labels`` (multilabel grow).
  - Append ``C``'s parents to ``E.parents`` (multi-parent
    union).
  - Record an audit-only row in ``A_N`` (if ``E.verdict ==
    "alive"``) or ``D_N`` (if dead) marking
    ``C → merged_into = E.canonical_set`` so the lattice's
    exploration count is observable.
  - **Skip the entire downstream pipeline** — no
    ``record_uncond``, no ``post_contra``, no goal check,
    no alive/dead branching, no integrate. Every effect has
    already been recorded on ``E`` and bubbled upward when
    ``E`` was first explored. Re-bubbling would write the
    same facts again — idempotent but wasteful.
  - Advance to the next candidate.
- **Miss** — first arrival at this saturated state. Register
  ``state_hash(fork) → C`` in the lattice's state-hash index
  and proceed to 3d.iv.

This is the **secondary** dedup (Q1.5b.4.a). The **primary**
set-equivalence dedup runs *before* the fork — a re-arrival
of an already-canonicalised set never reaches this point.
State-hash dedup catches the cross-set convergence: two
distinct ``C_1 ≠ C_2`` whose saturations happen to land on
the same kb.

Cost: one state_hash per arrival; one dict-by-int lookup.
Free relative to the saturation we just paid.

##### 3d.iv. Scan firings (forced positives, proposed negatives)

Scan the saturator firings:

- ``forced_positives`` — every productive firing ``f`` where
  ``f.derives_positive()`` and ``not reaches_hypothesis(fork,
  f.derived)`` (the derived fact's premise chain doesn't reach
  any hypothesis fact in ``C``). These are provably true
  regardless of ``C`` and bubble upward.
- ``proposed_negatives`` — every ``(not X)`` derived during
  saturation whose chain is also hypothesis-free.

This is the **uncond hoist** step from S1.5a.20 (mirrors what
``_mirror_forced_positive`` does in the tree side, now folded
into the per-set integrate).

##### 3d.v. Post-saturation contradiction check

``ContradictionDetector(fork).detect()`` again on the
post-saturation fork. If hits → **DEAD (post-sat)** (3d.vi).
Else → goal check (3d.vii).

##### 3d.vi. DEAD (post-sat)

Same shape as 3d.i, plus:
- The unsat_core is now the post-saturation conflict witnesses
  resolved against the fork via ``fork.unsat_core(witnesses)``.
- The accumulated ``forced_positives`` / ``proposed_negatives``
  collected pre-contradiction may still be valid (they were
  hypothesis-chain-free at derivation time). The integrate
  step adopts them at each parent if the per-level
  ``is_unconditional_at`` predicate confirms. Conservative: a
  fact that's unconditional inside ``C`` may become
  conditional at a parent ``P_i`` if ``P_i ⊊ C`` lacks some
  premise that ``C`` provided — the integrator drops it then.
- Clause emit + multi-parent integrate as in 3d.i.

##### 3d.vii. Goal check

``is_solved(fork, mode)`` — does the fork's saturated kb
satisfy the goal? If yes → **SOLUTION** (3d.viii). Else →
**ALIVE** (3d.ix).

##### 3d.viii. SOLUTION

Produce the ``LatticeProof``:
- ``final_kb = fork``
- ``lattice = current DAG``
- ``alive_at_end = C``
- ``dead_commitments = D_1 ∪ … ∪ D_N`` (so far)

Under ``SOLVE`` mode terminate immediately; under ``GAPS``
mode record the solution and continue to the next candidate
(awaiting Q1.5b.10).

##### 3d.ix. ALIVE

- Record a `SetNode` in ``A_N``:
  - ``canonical_set = C``
  - ``labels = (C,)`` — grows on future state-hash merges
  - ``parents = (P_1, …, P_N)`` — the ``N`` ``(N-1)``-subsets
  - ``kb_snapshot, firings, verdict = "alive"``
  - ``forced_positives, proposed_negatives, learned_nogoods``
- **Integrate to all N parents**: for each ``P_i``, call
  ``integrate(P_i.kb, C's BranchResult)``:
  - adopt forced positives where ``is_unconditional_at`` at
    ``P_i`` confirms;
  - adopt proposed negatives similarly;
  - merge learned nogoods (idempotent);
  - re-saturate ``P_i.kb`` after integration;
  - re-prune ``P_i.alive`` against the updated
    ``_negated_facts``.
- The recursive bubble through ``P_i``'s own parents
  continues until root is reached. **Option A cadence
  (Q1.5b.2.a)**: this happens immediately, not at end-of-layer.
- Effects on the rest of layer ``N``:
  - root has new facts → ``current_alive`` may shrink;
  - root has new nogoods → next candidates' filter (step 2)
    catches more;
  - the remaining queue of layer-``N`` candidates is
    re-filtered before the next iteration.

#### 3e. Dedup summary

Two dedup checkpoints in the per-candidate pipeline:

- **Primary — set-equivalence dedup.** Before the fork (step
  3a / lattice index lookup). A re-arrival of an
  already-canonicalised set returns the same `SetNode` and
  skips the whole 3b–3d pipeline. No saturation, no
  contradiction check.
- **Secondary — state-hash dedup.** After saturation (step
  3d.iii). Two distinct ``C_1 ≠ C_2`` whose saturated kbs
  hash identically merge into one `SetNode`. The pipeline
  stops at 3d.iii; the merged node carries multilabel +
  multi-parent.

The two dedups cost is asymmetric: primary is one
dict-by-canonical-tuple lookup before any work; secondary
is one state_hash computation after the saturation we
already paid. Primary saves the whole pipeline; secondary
saves the downstream (3d.iv onwards).

### 4. Layer wrap-up

When the queue is exhausted:

- ``A_N`` and ``D_N`` are populated.
- ``root._nogoods`` carries every nogood emitted this layer.
- ``current_alive`` reflects every back-propagated negative
  (single-element nogoods from layer-1 deaths + any
  forced negatives propagated up).
- The orchestrator advances to layer ``N+1`` (if budget
  remains and no SOLVE-mode solution was found).

If ``A_N`` is empty AND ``D_N`` is empty (every candidate was
filtered out at step 2) → **search complete with no
remaining frontier**; final verdict (Q1.5b.10) is derived
from the cumulative state.

## Cross-references

- Filter helpers: [`nogoods.py`](../../../ein.py/src/ein_bot/inference/nogoods.py)
  (`matches_any_nogood`, `emit_nogood`).
- Per-set data model: [Q1.5b.4.b in `open_questions.md`](open_questions.md#q15b4--set-equivalence-dedup--state-hash-dedup)
  (`SetNode` schema with `canonical_set / labels / parents`).
- Integrate contract:
  [S1.5a.20 § integrate](../p1.5a_zebra_solution/s1.5a.20_branch_isolation_rearch.md#integrate-the-up-channel)
  — the 9-step parent-side ingestion that this algorithm
  invokes per parent on every outcome.
- Forced-positive mining: same S1.5a.20 doc, T1.5a.20.3.
- Saturation commutativity premise:
  [README § Motivation](README.md#motivation).
