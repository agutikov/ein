# S1e.1.2 — Determinism under `--jobs`: Q1

**Phase:** [P1e.1](README.md) (The ten questions)
**Estimate:** 2 days
**Depends on:** [S1e.1.1](s1e.1.1_search_soundness_probes.md) T1 — the
standard of proof, because this question's honest answer may be *the claim is
narrower than it sounds*.
**Answers:** [`review/open-questions.md`](../review/open-questions.md) Q1.

## Context

*`--jobs N` is the same computation* is a headline claim — it is in the
milestone record, it is what `jobs_invariance` sweeps, and it is one of the
four invariants M1a said it would hold. The evidence is strong and unusually
concrete: 20 712 cells, byte-identical verbose streams, and
`jobs_does_not_move_the_answer_or_a_counter`.

The review's question is not whether the evidence is real. It is that nobody
has written the **mechanism**. The `Nogoods` store is shared across forks by
`Arc<RwLock<…>>`
([`kb.rs:369-408`](../../../ein.rs/crates/ein-core/src/kb.rs)); a layer is
fanned out at [`solve.rs:1789-1801`](../../../ein.rs/crates/ein-infer/src/solve.rs);
so a clause learned by worker A while worker B is mid-candidate can, in
principle, prune a candidate B would otherwise have entered — and whether B
sees it depends on scheduling. The known counter-argument is that **commits
replay in candidate order on the committing thread**, which would make any
mid-flight difference invisible in the result. That argument was not
reconstructed by the review, and it is not written anywhere in the tree.

This matters more than a missing paragraph. Invariance sweeps prove *these
runs agreed*; they cannot prove *all runs agree*, and the distance between
those two is exactly where a scheduling-dependent bug lives. The repo already
knows this shape — it is why `check_hashmap_iteration.py` is a lint and not a
test over observed orders.

## Acceptance

- One of three, and the stage says which:
  1. **The argument.** A written structural argument that mid-flight reads
     cannot change an observable, placed where a reader of the determinism
     rules will find it — beside `Nogoods` in
     [`kb.rs`](../../../ein.rs/crates/ein-core/src/kb.rs) **and** in
     [`design/02`](../../../docs/history/m1a_rust/design/02_determinism_and_order.md),
     which is the page that owns this class of claim.
  2. **The test.** A test that injects a clause from another thread mid-layer
     and shows the commit-order replay masks it — the mechanism demonstrated
     rather than asserted.
  3. **The narrowing.** If neither holds, the claim is restated to what the
     evidence supports (*every measured configuration agrees; the argument
     for all configurations is open*), and the gap becomes a `Q-M1e.<n>` with
     an owner.
- Whichever lands, the counters `LatticeStats` reports under `--jobs N` are
  covered by it explicitly, not only the verdict — the review's phrasing is
  *the answer or a counter*, and a counter that drifts with thread count
  would be a real defect even where the answer holds.
- No behaviour change. If the stage finds a genuine race it stops and files
  it: that is engine work with its own stage, not a task here.

## Tasks

### Task T1e.1.2.1 — Reconstruct the read/write discipline of `Nogoods`

Read the store and every caller, and write down the four things the argument
needs, each with a `file:line`:

1. **When a clause is written.** Which paths call `emit_nogood`, under which
   lock, and whether a write is ever conditional on a *read* of the same
   store (a read-modify-write is where an ordering hazard would be).
2. **When a clause is read.** Where the candidate join consults the store —
   the `dropped_nogood` path the `layer` event counts — and whether that read
   is per-candidate, per-layer, or snapshotted.
3. **What a fork inherits.** `fork()` copies the shared `Arc`, so a fork sees
   later writes by other workers; a *snapshot* would not. Which is it, and is
   the choice deliberate?
4. **What the commit replay actually replays.** The claimed masking mechanism.
   Establish precisely what is re-executed on the committing thread and what
   is carried over from the worker — because the argument is only as good as
   the set of results the replay recomputes.

The product is a short table, and it is the argument's skeleton whichever way
the stage ends.

### Task T1e.1.2.2 — Decide: argument, test, or narrowing

If (1) from T1e.1.2.1 shows writes never depend on reads *and* (4) shows the
replay recomputes every observable from the committing thread's own view,
then the argument is available and the task is to write it — 15 lines beside
`Nogoods`, cross-linked from `design/02`, naming the sweep as the evidence
and the replay as the mechanism.

If the replay carries anything over from the worker's view, the argument does
not close, and the task becomes the test: a harness that holds one worker at
a known point, writes a clause from another, releases, and asserts the
recorded result equals the single-threaded run's. This is the shape
`EIN_ID_SEEDS` and `EIN_JOBS_SWEEP` already use — a lever that makes an
otherwise-unobservable order observable — so an `EIN_NOGOOD_INJECT`-style
test hook is in keeping, if it can be built without shipping a lever nobody
needs.

If neither, narrow the claim, and be specific about *what* narrows: the
milestone record's sentence, `design/02`'s statement, and the test's own doc
comment are three copies of one claim
([AR-M1](../README.md#the-findings)'s pattern again) and all three move
together or the tree contradicts itself.

### Task T1e.1.2.3 — Cover the counters, not only the answer

`jobs_does_not_move_the_answer_or_a_counter` is named for both. Confirm which
counters it actually compares, and whether the set includes the ones a shared
store would perturb first — `dropped_nogood`, `nogood_emitted`,
`nogood_subsumed`, the enterings split. If any of those is outside the
comparison, either add it or record why it is legitimately allowed to differ,
which would itself be a significant fact about the claim.

## Notes

There is a cheaper answer available and it is worth naming so it is
deliberately **not** taken: raising `EIN_JOBS_SWEEP` and running more cells.
More agreement is more of the same evidence. The question is about a
mechanism, and one more sweep does not supply one.
