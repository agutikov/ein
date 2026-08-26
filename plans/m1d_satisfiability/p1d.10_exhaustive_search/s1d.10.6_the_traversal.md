# S1d.10.6 — The traversal: one obligation per node

**Phase:** P1d.10 (Exhaustive search over many models)
**Estimate:** 5 days
**Added:** 2026-08-26, by [the reconnaissance](README.md#what-the-reconnaissance-found--2026-08-26).
**Depends on:** [S1d.10.2](s1d.10.2_depth_required.md) — the depth accounting,
so the baseline this is measured against is one number and not three; and
[P1d.2 S1d.2.5](../p1d.2_obligations/s1d.2.5_hypotheses_from_obligations.md),
which built the interface and left the note.
**Two of six tasks done 2026-08-26** — `T1d.10.6.1`, the completeness argument
([`completeness.md`](completeness.md)), which the acceptance requires before any
code, and `T1d.10.6.2`, the instrument reconciliation. **What they change about
the rest**: part (3) of the completeness argument is false, it is false of the
*shipped* engine and not of the tree, and the tree therefore cannot lose a model
the lattice keeps — so `T1d.10.6.3` is unblocked and `T1d.10.6.5`'s comparison
is looking for a traversal bug rather than for a completeness one.

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

### Task T1d.10.6.1 — The completeness argument — **done 2026-08-26**

**[`completeness.md`](completeness.md)** is the deliverable, written before any
code as the acceptance requires. Three parts were asked for; two hold and the
third is false, and the useful part of the finding is *why*.

- **(1) jointly exhaustive** — holds, by the obligation's own meaning, with C4
  (already enforced by the rung) as its precondition. Measured: mean branch
  width **5.0** on the phase entry, which is `ext(House)` exactly, and
  `stuck = 0`. Mutual exclusivity is a separate and weaker property that the
  argument does not need — without it a model is reached twice, which
  `state_key` already dedupes.
- **(2) terminates** — holds, and by measurement rather than by argument. The
  emulator records the owed count at every node: **65 of 65, 70 of 70 and 72 of
  72 edges** strictly decreased under the three walk policies, and the four
  policies reproduce the reconnaissance's table node for node. The bound is the
  root's own debt (46) and the observed depth is 6.
- **(3) the leaves** — **false in general, false today, and not the tree's
  doing.** Since S1d.2.5 a program that declares an obligation has its
  candidates generated by the rung *whichever way the search walks them*, so the
  tree and the lattice search the same candidate space and differ only in
  traversal. `uncovered ≠ 0` does not mean the tree loses models; it means the
  **rung** does not propose those relations, and the lattice does not either.

The stage's phrasing — *"they coincide only where saturation determines every
relation no obligation names"* — is therefore stronger than what is true and
weaker than what matters, and `completeness.md` § 3d restates it. What checks it
is [S1d.3.1](../p1d.3_model_sets/model_set_census.md)'s `EIN_LEFTOVER=1`, split
by relation: `zebra2-obligations`' unique model leaves **3 678** proposable
facts and they are **exactly the four `uncovered` relations, with no `*-loc`
among them** — two closed, two rule-derived — which is the argument holding.

**And the program for which it is false now exists**, 25 lines, in
`completeness.md` § 3c: an obligation on `seats` and an uncovered, un-closed
`knows`. `ein solve -e` says **`k = 3`, `exhausted = true`**; adding
`(knows Ann Bob)` to the program yields **three more** consistent exhausted
models. The phase README's risk list predicted this shape and said *"the trap is
that on this corpus it would not fire"* — it fires, and it fires for the
**lattice**, which is the part nobody had predicted.

### Task T1d.10.6.2 — `uncovered`, read from the right state — **done 2026-08-26**

**Measured, and the gap is bigger and simpler than the task expected: it is one
whole relation.** The `rung` event is the search's own view; `-H` is a
closed-emitted fork's. Side by side on `examples/zebra2-minus-15-obligations.ein`:

| | the search (`rung` event) | `-H` (closed-emitted fork) |
|---|---:|---:|
| `owed` | 46 | 46 |
| `branches` | **46** | 38 |
| `declined` | **0** | 8 |
| `candidates` | **230** | 190 |
| emitted / layer-1 `alive` | **96** | 81 |
| `uncovered` | **4** | 2 |

Every field but `owed` differs. `96 − 81 = 15` is **all of `nation-loc`**: the
search enters five relations at layer 1 (`pet-loc` 24, `smoke-loc` 23,
`color-loc` 20, `nation-loc` 15, `drink-loc` 14) and `-H` reports four. Proven
by construction rather than by reading — write `(__closed__ nation-loc)` into
the program and **the search reproduces the preview to the fact**: `alive`
96 → 81, `branches` 46 → 38, `declined` 0 → 8, `candidates` 230 → 190.

**Why `nation-loc` and not the other four is the part worth knowing.**
`emit_closed` runs **before** saturation, so the only `co-located` activators it
sees are the four the puzzle writes — and `nation-loc` sits in slot 1 of every
one of them. Saturation then derives the swapped activators
(`(co-located color-loc Red nation-loc Englishman :rule co-located-fanout)`),
after which `nation-loc` *is* rule-derivable. So the preview's closure is
**order-sensitive**, and it is not true by the criterion
[`closed.rs`](../../../ein.rs/crates/ein-infer/src/closed.rs) states — *"no rule
can positively conclude an R-fact"*. It is harmless here for the opposite
reason: those rules derive the `nation-loc` facts from the other four anyway, so
guessing them is redundant rather than unconfirmable.

**And the preview is describing a strictly cheaper search that `solve` declines
to run.** With that one declaration, `-e -m 3` on the phase's entry costs
**29 144 enterings and 15.3 s** against **48 745 and 26.0 s**, and the model set
is **identical fact for fact** — 32 both ways, `*-loc` extents equal, full fact
sets equal modulo the marker. A 40 % cut from one line. Whether the closure is
sound *in general* is **not** established: it is right here for a reason the
module does not claim, so this is a lead for
[S1d.10.3](s1d.10.3_stopping_criterion.md)'s ledger rather than a change to make.

**One more read-out disagrees, in the same binary.** `--json-summary`'s
`root.hypgen` reports the **search's** numbers — `raw 230, emitted 96,
pre_candidate {}` — where `-H` reports `190 / 81 / closed_relation 8`. Two
answers to "what would the generator propose at root", and the caller picks
which by choosing a flag.

---

*The task as written, kept because the reasoning is what led to the measurement:*

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

### Task T1d.10.6.3 — The branch — **done 2026-08-26**

**Built, behind `EIN_TRAVERSAL=tree`, and it is 86 enterings.**

| `examples/zebra2-minus-15-obligations.ein` | enterings | wall | k | certified |
|---|---:|---:|---:|---|
| lattice `-m 3` (finds all 32) | 48 745 | 26.0 s | 32 | no |
| lattice `-m 5` (`solve -e`) | 618 076 | 432 s | 32 | no |
| lattice `-m 38` (exhausts) | 17 204 592 | 1 496 s | 32 | yes |
| **tree**, no cap | **86** | **0.083 s** | **32** | not claimed |

**200 053× the enterings and 18 024× the wall** against the run that certifies,
and the model set is **identical fact for fact** — verified in `cargo test`, on
fact sets and never on `k`. The determinate control improves too:
`zebra2-obligations` is **9** nodes against 101 enterings.

It beats the reconnaissance's own emulator (86 against 171–206) for the reason
[the note](#notes) predicted: the emulator branched over the witness type's
whole extent where this takes the rung's guard-scan, which is a subset.

**The seam is one parameter, and where it sits is the design.**
`oblgen::generate` has always built a per-instance `Vec<Branch>` and flattened
it at the last step; `one_branch` stops after the instance [`Choice`] picks,
and `hypgen::generate_one_branch` is the entry point.

The selection is **before** the filter pipeline and not after it, which is why
this is a parameter rather than a grouping a caller could reconstruct:
`apply_filters`' `seen_in_call` drops a candidate already offered earlier *in
the same call*, so under the union a later instance whose alternatives an
earlier one already proposed comes back short or empty. Harmless when the caller
wants the union; a silently truncated branch when it wants that instance's, and
a truncated branch is not jointly exhaustive.

**And building it produced the stage's sharpest finding, which is a guard.**

A tree on a rung that is **not** the obligations one is the solver `ein.rs`
deleted. An hrule's candidates and the blind enumerator's are not one owed
instance's alternatives — they are not jointly exhaustive — so branching on them
walks hypothesis *paths* and reaches a size-`d` commitment by `d!` routes.
Measured before the guard existed:

| | lattice | tree, unguarded |
|---|---:|---:|
| `examples/zebra2.ein` | 101 | **7 877** |
| `examples/zebra.ein` | 111 | **11 083** |
| `examples/zebra2-minus-15.ein` | 48 745 | did not finish in 60 s |

Same models, 78× and 100× the cost — which is
[§ 1](../../../docs/kernel/inference/README.md)'s *"the tree engine's
depth-first ordering over hypothesis branches prices in d! orderings of the same
commitment set"*, arriving 88 days after `8d77b02` deleted it. So `tree()` probes
the rung at root, **declines** if it is not `Obligations`, narrates a
`traversal` event saying so, and hands the run to the lattice — which then
reproduces the lattice's numbers to the digit, and that is what the second test
pins. `examples/zebra.ein`, the entry the emulator could not do at all, is
answered by the decline: it is an hrule program.

**What it does not claim.** `truncated` is set, so the tree reports
`exhausted = false` and *models found*. A tree terminates by discharge and a
lattice by exhaustion; the sentence that says what discharge licenses is
[T1d.10.5.1](s1d.10.5_contract.md)'s and is not written, and the phase's own
rule is *never a quiet `exhausted = true`*. `layers_explored` carries the
deepest node, which is a different quantity wearing the same name — that is
[T1d.10.6.4](#task-t1d1064--what-a-tree-reports)'s to settle and is why this is
an environment variable rather than a flag.

**Byte-identical with the variable unset**: `./run_tests.sh` green with no
golden re-blessed, which is the statement that nothing here reached the lattice.

*The task as written:*

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
