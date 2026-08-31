# S1e.1.2 — Determinism under `--jobs`: Q1

**Phase:** [P1e.1](README.md) (The ten questions)
**Estimate:** 2 days
**Depends on:** [S1e.1.1](s1e.1.1_search_soundness_probes/README.md) T1 — the
standard of proof, because this question's honest answer may be *the claim is
narrower than it sounds*.
**Answers:** [`review/open-questions.md`](../review/open-questions.md) Q1.

---

> **Done 2026-08-29 — acceptance (1), the argument, and the mechanism is not
> the one the question guessed.**
>
> The review's counter-argument was that *commits replay in candidate order on
> the committing thread*, which would make a mid-flight difference invisible in
> the result. That is true of the result and would not have been enough: a
> candidate `apriori::filter_candidate` drops is **never entered**, so it has no
> result to replay. The mechanism is one step earlier and simpler —
>
> - **no worker ever reads the store**, because a clause is consulted only when
>   a *layer's candidates are generated*, never while an entering runs; and
> - **no worker ever writes one**, because in the search `emit_nogood` has
>   exactly two callers — `Run::handle_dead` and `Run::integrate`, both on the
>   committing thread — and `Run::fan_out` is a barrier that thread sits inside
>   for exactly as long as a worker exists.
>
> So a worker cannot observe a clause arriving: while it lives there is no
> writer. Written at [`Nogoods`](../../../ein.rs/crates/ein-core/src/kb.rs) and
> as [design/02 §6a](../../../docs/history/m1a_rust/design/02_determinism_and_order.md),
> with [design/08 §6](../../../docs/history/m1a_rust/design/08_parallelism.md)'s
> `_nogoods` row — *"no sharing needed"*, third column empty — corrected to what
> shipped.
>
> **And the premise is now enforced**, which is the half that makes it an
> argument rather than a reading: it lived in one doc comment on
> `Run::fan_out_this_layer` and nothing checked it, so by
> [Q-M1e.1/2](../open_questions.md#q-m1e1--what-is-the-standard-of-proof-for-refuted)
> Rule 2 it was not yet enough. The store is **frozen** for the fan-out window
> (`Kb::freeze_nogoods`, a `Drop` guard) and a write from any thread panics —
> `assert!`, not `debug_assert!`. Three tests hold the mechanism itself,
> including that the guard is still *taken*, since one that stopped being taken
> would break nothing and fail nothing.
>
> **No behaviour changed**, and no golden moved. What did change is one
> coverage number and one filed question — see T1e.1.2.3.

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

### Task T1e.1.2.1 — Reconstruct the read/write discipline of `Nogoods` ✅

**Done 2026-08-29.** The table is
[design/02 §6a](../../../docs/history/m1a_rust/design/02_determinism_and_order.md)
§ *Every access to the store* — six rows plus the four instruments that are not
the engine — and the short form is beside `Nogoods` in `kb.rs`. The four
answers:

1. **A write is conditional on a read** — `emit_nogood` is a read-modify-write:
   it scans for a subsuming clause, removes the supersets it subsumes, then
   inserts. That is exactly where an ordering hazard would be, and it is why
   the answer could not be "the writes are independent". What saves it is that
   the whole read-modify-write happens under one write guard **on the
   committing thread**: `handle_dead` (`solve.rs:2341`, from `commit_entering`,
   *"called in candidate order, always"*) and `integrate` (`solve.rs:2398`, the
   batch and layer barriers).
2. **A clause is read per *layer*, not per candidate** — `Run::generate_layer`
   takes one read guard and holds it across the whole of layer *L+1*'s
   generation (`solve.rs:1206`). Nothing consults the store during an entering,
   which is the load-bearing fact: a clause landing between candidate *i* and
   candidate *i+1* of the same batch could not change candidate *i+1* at any
   job count.
3. **A fork inherits the shared `Arc`** (`Kb::branch`), not a snapshot — so it
   *can* see later writes. `Kb::snapshot` is the one that copies, for archival
   isolation. The choice is deliberate and, it turns out, **unexercised**: no
   branch reads the store at all, so the sharing is what `branch` does with an
   `Arc` field rather than a requirement. The field comment claiming *live
   branches read each other's learned clauses* was false and is corrected.
4. **The commit replay recomputes nothing** — it replays the worker's
   *narration* (`Events::replay` fills the ordinal at the commit) and consumes
   the worker's `Entered` as computed. So the review's masking mechanism was
   never available, and it is not needed.

The original four questions, for the record:

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

### Task T1e.1.2.2 — Decide: argument, test, or narrowing ✅

**Decided 2026-08-29: the argument** — but not by the route this task
anticipated, and the difference is worth keeping. Writes *do* depend on reads
(1) and the replay recomputes *nothing* (4), so on this task's own test the
argument should not have closed. It closes on a premise the task did not
consider: **the store is not read during an entering at all**, so what a worker
sees of it is not a question that arises.

The test was therefore not built, and an `EIN_NOGOOD_INJECT` lever would have
been actively wrong: injecting a clause mid-layer is now a **panic**, because
the store is frozen for that window. The freeze is the same probe made
permanent and corpus-wide — it asserts on every fanned-out layer of every entry
`cargo test` runs — rather than one synthetic injection into one file.

The three copies of the claim moved together (`AR-M1`): design/08 §6's row,
Q-M1a.7's *what would re-open this*, and `jobs_invariance`'s own module note is
left as it stands, because nothing it says became false — the evidence it
reports is still the evidence, and the mechanism is now written where a reader
of the code finds it.

The task as written:

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

### Task T1e.1.2.3 — Cover the counters, not only the answer ✅

**Done 2026-08-29, and it found one.** What the two sweeps compare:

| counter | `jobs_does_not_move_the_answer_or_a_counter` | `jobs_invariance` (corpus) |
|---|---|---|
| `BaseStats`' ten — the enterings split, `facts_merged`, `forced_positives`, `saturate_count`, `layers_explored`, `nogoods_emitted`, `nogoods_subsumed` | ✅ via `MonotonicStats` | ✅ `solve_shape`'s `STATS` line, **and** `proof_summary.json` inside `Op::Dump("lattice")`'s tree |
| `solution_nodes` / `exhausted` | ✅ same | ✅ `VERDICT k= exhausted=` |
| `LatticeStats`' own three — `solutions_found`, `state_key_merges`, `elapsed_seconds` | ➖ not in `MonotonicStats` | ✅ the first two byte-for-byte in `proof_summary.json`; `elapsed_seconds` **normalised to `<ts>`**, which is the one field legitimately allowed to differ |
| the learned clause **set** | ➖ | ✅ `solve_shape`'s `CLAUSE` lines and its filtered `nogood` events |
| **`LayerCensus::dropped_nogood`** | ❌ **not compared** | ⚠️ compared, and **empty** |

So the acceptance's *"the counters `LatticeStats` reports"* are covered in full,
and the only field of it that moves is wall clock — verified by probing
`Op::Dump("lattice")` on `examples/lattice/01_subset_pruned.ein`, whose
`proof_summary.json` renders `"solutions_found": 2, "state_key_merges": 0,
"elapsed_seconds": <ts>`. The gap is one counter that is **not** in
`LatticeStats` at all.

`LayerCensus` is per *layer*, so it is deliberately not in `MonotonicStats`,
and `dropped_nogood` — what the learned clauses took off the next layer's join
— is the **read** side of the store where the two counters above are the write
side. It was outside the unit comparison entirely; it is in it now, through a
`Dumper` that collects every layer's row and answers `reads_forks` **false**
(a dumper that reads forks makes a fanned-out layer keep them, which is a
different run to compare), with a non-vacuity assertion that some candidate
somewhere is actually dropped by a clause.

The corpus route is the finding. It exists — `Op::Dump("progress")`'s
`layer N gen:` line carries the column — and it runs under `dump_shape`'s
`max_enterings = 60`. Measured over all **202** corpus entries at that budget:
**0** have a nonzero `dropped_nogood`; **16** have a nonzero per-layer
`nogoods_emitted`. So the corpus sweep compares the write side for real and the
read side as a column of zeroes agreeing with itself. Raising the budget
re-blesses every `dump[progress]` cell of `corpus_shapes.md5`, which P1e.1's
acceptance says must be named in a stage file *before* it moves and this stage
did not predict — so it is filed as
[Q-M1e.14](../open_questions.md#q-m1e14--the-corpus---jobs-sweeps-per-layer-census-coverage-is-vacuous)
with the three options priced, and not taken here.

The task as written:

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

**It was not taken, and the note turned out to understate the case.** The sweep
that would have been raised is the one whose `dropped_nogood` column is empty
on all 202 entries (T1e.1.2.3): more cells of it would have been more agreement
about a counter nobody was comparing. The mechanism is what found that, which
is the argument for asking for a mechanism.

### What landed

| | |
|---|---|
| the argument | `ein-core/src/kb.rs` (`Nogoods`, and the `nogoods` field comment it corrects); [design/02 §6a](../../../docs/history/m1a_rust/design/02_determinism_and_order.md); [design/08 §6](../../../docs/history/m1a_rust/design/08_parallelism.md)'s row; [Q-M1a.7](../../../docs/history/m1a_rust/open_questions.md#q-m1a7--may---jobs--1-move-counters)'s *what would re-open this* |
| the enforcement | `Kb::freeze_nogoods` + `NogoodFreeze` (`kb.rs`), taken at `Run::fan_out` (`solve.rs:1827`); `Nogoods::insert` / `remove` assert |
| the tests | `ein-core` `a_frozen_store_refuses_a_write`, `the_freeze_lifts_when_the_guard_drops`; `ein-infer` `a_fanned_out_layer_freezes_the_clause_store`, plus `Ran::census` in the four `--jobs` tests |
| filed | [Q-M1e.14](../open_questions.md#q-m1e14--the-corpus---jobs-sweeps-per-layer-census-coverage-is-vacuous) — the corpus census coverage, owner unassigned |
| not changed | any behaviour, any golden, any counter |
