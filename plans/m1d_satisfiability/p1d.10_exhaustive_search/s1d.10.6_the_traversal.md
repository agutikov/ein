# S1d.10.6 — The traversal: one obligation per node

**Phase:** P1d.10 (Exhaustive search over many models)
**Estimate:** 5 days
**Added:** 2026-08-26, by [the reconnaissance](README.md#what-the-reconnaissance-found--2026-08-26).
**Depends on:** [S1d.10.2](s1d.10.2_depth_required.md) — the depth accounting,
so the baseline this is measured against is one number and not three; and
[P1d.2 S1d.2.5](../p1d.2_obligations/s1d.2.5_hypotheses_from_obligations.md),
which built the interface and left the note.

## Context

This stage exists because a deferral in another phase turned out to be the
largest number in this one.

[S1d.2.5 §1](../p1d.2_obligations/hypotheses_from_obligations.md) shipped the
obligations rung and recorded, up front, the one place it is not what its own
plan drew:

> **The rung proposes the union of every accepted obligation's candidates,
> where the plan said "one chosen obligation's".** … Branch on obligation *O*'s
> candidates alone and layer 1 is *O*'s alternatives — correct, mutually
> exclusive, jointly exhaustive. Layer 2 is then **pairs of them**, every one of
> which is two witnesses for a slot that needs one … "Choose one obligation,
> branch, recurse *at that node*" is a **depth-first** move, and the traversal
> that could take it is P1d.10's subject.

[§4](../p1d.2_obligations/hypotheses_from_obligations.md) then built the choice
heuristic the depth-first move would need, measured it at **0 difference on
every counter** under the lattice, recorded it inert per
[F9](../../followups/f9_e_catalog.md)'s rule, and kept it *because* this stage
would need it on day one.

**The reconnaissance ran the move outside the engine and it is worth five orders
of magnitude.** On `examples/zebra2-minus-15-obligations.ein`, against the
lattice's 17 204 592 enterings and 24 min 56 s for the same exhaustion claim:
**171 nodes and 2.1 s**, with 171 fresh processes and 171 root saturations from
scratch, the same 32 models verified fact for fact, and a maximum depth of
**6** against the lattice's 22 layers. The determinate control does not regress:
16–26 nodes against 101 enterings on `zebra2`.

The number is an upper bound in two separate ways
([README §1](README.md#1-the-proof-costs-83-517-what-the-answer-does)) — a
fresh process and a fresh load per node, and a branch coarser than the rung's —
which is the right direction for a number a stage is going to be judged
against.

## What is actually being claimed

Not "depth-first is faster". The census
([§8](layer_census.md#8-where-the-time-goes-in-this-regime--the-lattice-is-12-))
already priced the lattice machinery at **1.2 %** of the run, which rules out
every proposal that makes the lattice cheaper. The claim is the milestone's own
sentence, and it is about what a *node* means:

> a subset-lattice only prunes through death, whereas a choice point prunes by
> construction: committing to one alternative excludes its four siblings
> without anybody having to refute them.

A `total-owed` instance says *`a` has an `R`-arrow to some member of `B`*. That
is a set of alternatives which is **jointly exhaustive** (the obligation says
so) and, where the relation is also `functional`, **mutually exclusive**. A
branch over it is complete at that node by the meaning of the declaration, and
needs no clause, no death and no depth cap to be complete. And it is why the
two searches have different *bounds*, not just different observed depths: the
lattice's depth is bounded by `|alive| = 96` and reaches 22, while the tree's is
bounded by **the 46 instances root owes** — each commitment discharges at least
one and creates none — and reaches 6.

## Acceptance

- **The completeness argument is written before the code**, and it names its
  own precondition. A tree that terminates by *discharge* proves a different
  thing from a lattice that terminates by *exhaustion*, and they coincide only
  where saturation determines every relation no obligation names — `uncovered`,
  which the rung already reports and which is **not 0** on the zebra family.
  Where it is non-zero the claim rests on saturation, and the only instrument
  that checks it is a model-set comparison.
- **The same models, on every entry that can run both.** Not "the same count" —
  the same fact sets, the way
  [S1d.2.5 §2](../p1d.2_obligations/hypotheses_from_obligations.md) compared the
  two generator paths. The corpus offers five entries and the reconnaissance
  has already done four of them by hand; the stage does it in `cargo test`.
- **The determinate puzzles do not regress**, in time or in verdict. `zebra -e`
  and `zebra2 -e` are P1a.6 baselines and a traversal that costs them anything
  is a traversal that ships off by default at best.
- **What a tree reports is decided, not improvised.** `layers_explored`,
  `enterings_total`, the `layer` census row, the shape digests and
  `utils/layer_census.py` are all statements about a cardinality-ordered
  lattice. Either the tree reports them with a stated re-reading, or it reports
  something else and the stage says what every consumer does with the
  difference.
- **The choice heuristic is measured again**, because this is the stage that
  makes it live: 171 … 316 nodes over four policies in the reconnaissance, on
  one puzzle. One puzzle is not a heuristic result, and the stage should say so
  rather than pick a winner from it.
- **The decision is explicit** — on by default, behind a flag, or recorded with
  its number and not shipped — and default-on needs
  [Q-M1a.18](../../../docs/history/m1a_rust/open_questions.md#q-m1a18--may-a-fork-stop-re-narrating-the-roots-fixpoint)'s
  shape of decision taken deliberately, because the counters move.

## Tasks

### Task T1d.10.6.1 — The completeness argument

Written, before anything is built. Three parts, and the third is the one that
can fail: the branch is jointly exhaustive *by the obligation's meaning*; the
recursion terminates *because the owed set strictly decreases and nothing adds
to it*; and the leaves are models *iff* saturation determines everything no
obligation owes. Name what makes the third true, and what a program looks like
for which it is false.

### Task T1d.10.6.2 — `uncovered`, read from the right state

`-H` reports `rung.uncovered 2` on the zebra family where
[S1d.2.5 §6](../p1d.2_obligations/hypotheses_from_obligations.md) recorded
**4**, and **both are right about different states**, which is a trap the stage
should walk into deliberately rather than by accident.
[`closed.rs`](../../../ein.rs/crates/ein-infer/src/closed.rs) says so in its own
header: *"Where it is called from matters, and it is not `solve`"* — the
`--hyp-stats` preview runs `emit_closed` on a **fork** and the search does not,
*"so the search itself sees every relation open"*. The four are `is-a`, `is-a*`,
`right-of`, `next-to`; the closure pass closes `is-a` and `right-of`, which is
the missing two exactly.

The consequence is a strengthening rather than a caveat, and T1d.10.6.1 should
use it: of the four relations no obligation owes, **two cannot be guessed at
all** — no rule positively concludes them, so a hypothesis about them could
never be confirmed — and the other two (`is-a*`, `next-to`) are derived by the
puzzle's own rules from what is. That is why the tree's model set matched the
lattice's on the reconnaissance's four entries, and it is the shape of the
argument the stage owes for the general case.

**And it is a warning about the instrument.** Any number read off `-H` is read
off a closed-emitted fork; `rung.owed`, `rung.branches` and `root hyps` on that
line are not the search's. The reconnaissance confirmed the size of the gap:
`-H` says **81** root candidates on `zebra2-minus-15-obligations` where layer 1
enters **96**, **43** against 56 on `zebra2-obligations`, and **0** against 35
on `features/01_not_and_absent` — while on the two hrule files, where the
closure pass changes nothing the hrule proposes, it agrees exactly (56 and 96).
Use the `rung` event or the `layer` census row when the search's own view is
what is wanted.

### Task T1d.10.6.3 — The branch

One obligation per node, its candidates from
[`oblgen`](../../../ein.rs/crates/ein-infer/src/oblgen.rs)'s existing
guard-scan — the same candidate set the rung already computes, taken per
instance instead of unioned. The interface S1d.2.5 kept (`EIN_OBLIGATION_CHOICE`,
the walk order, the decline rule, the `rung` event's `owed` / `branches` /
`declined` split) is what this consumes.

### Task T1d.10.6.4 — What a tree reports

The vocabulary question, and it is not cosmetic: `Σ entered = enterings_total`
is an invariant a test checks, and a tree has no layers to sum over. Decide
whether a node is an entering (it is a fork and a saturation, so probably yes),
what `layers_explored` means when depth is per-branch, and whether the `layer`
event gets a sibling or a re-reading. `--max-set-size` is the sharpest case:
under a tree it bounds nothing the search needs.

### Task T1d.10.6.5 — Measure both regimes

The under-determined entry and its twin against the lattice's 17 204 592, and
the three determinate zebras against 101 and 111. Report nodes, wall, peak RSS
and the model set — and report the **model set as a set**, not as a `k`.

### Task T1d.10.6.6 — The decision

Ship on, ship behind a flag, or record with the number. All three are
acceptable; only an unmeasured one is not. Where the switch lives is part of
the decision and has a precedent one phase back: `EIN_OBLIGATION_CHOICE` is an
environment variable rather than a `(config …)` field precisely because
`SolverConfig` is rendered into the KB-shape digest, so a knob whose settings
are being compared would re-bless every shape golden in the corpus.

## Notes

- **The reconnaissance's emulator is not the deliverable and should not be
  ported.** It is six lines of policy over `ein solve -m 0 --json-summary`, it
  re-loads the program at every node, and it enumerates the witness type's whole
  extent where the rung scans a guard. Its value was that it needed no engine
  change to price the idea; a stage that inherits its shape inherits its
  slowness.
- **`examples/zebra.ein` is the entry the emulator could not do**, and the
  reason is instructive rather than incidental. It is the B0 `co-located`
  encoding, so its obligations are `std.slots`' `slot-owed-room` /
  `slot-owed-fill` pair and the same relation is owed from **both** argument
  positions. Under the emulator's coarse branch — the witness type's whole
  extent — the tree had not closed after 900 s, where `zebra2`'s B1 `*-loc`
  encoding closes in 26 nodes. That is either the encoding or the coarse branch,
  and T1d.10.6.3 finds out which by using the rung's own candidate set. Treat it
  as the stage's first regression case, not as a footnote.
- **The failure mode to design against** is a tree that finds the models and
  cannot say it found all of them — which is the same failure the lattice has,
  arrived at from the other side, and is why T1d.10.6.1 comes first.
- The one thing this stage does **not** get from the corpus is a second
  under-determined puzzle to check the heuristic on
  ([README §3](README.md#3-the-vocabulary-reaches-five-of-fifty-one)). If that
  matters to the decision, say so in the decision rather than inventing one.
