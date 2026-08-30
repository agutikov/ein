# S1e.3.4 — Architecture (Medium)

**Phase:** [P1e.3](README.md) (Medium)
**Estimate:** 3 days
**Depends on:** [CO-H2](../p1e.2_high/s1e.2.1_correctness.md) — the reserved-name
pair is `AR-M1`'s first instance and is unified there; this stage takes the
other three and the pattern.
**Blocks:** [S1e.3.1](s1e.3.1_correctness.md) T2 and
[S1e.3.2](s1e.3.2_semantics.md) T1 if the `AR-M2` seam fix is taken.
**Findings:** [`AR-M1`](../review/architecture/medium.md),
[`AR-M2`](../review/architecture/medium.md).

> The review's architecture pass **did not complete** — these two come from
> the system-reconstruction reading and the report says so. Both are
> nonetheless evidenced by observed divergences rather than by judgment, which
> is why they survive that caveat.

## Context

Two findings, and between them they explain roughly a third of this
milestone's other findings.

**`AR-M1` — one semantic artifact, two hand-synchronized copies.** Four
instances:

| pair | consequence so far |
|---|---|
| [`imports.rs:49-51`](../../../ein.rs/crates/ein-ir/src/imports.rs) vs [`terms.rs:184-193`](../../../ein.rs/crates/ein-core/src/terms.rs) — reserved names | **a real behavioural bug** ([CO-H2](../p1e.2_high/s1e.2.1_correctness.md)): `(macro open …)` loads silently through one import route |
| [`macros.rs`](../../../ein.rs/crates/ein-ir/src/macros.rs) vs [`from_ir.rs`](../../../ein.rs/crates/ein-ir/src/from_ir.rs) — two macro pipelines | divergent duplicate/reserved error semantics, plus a doc comment naming the wrong one ([CO-M4](s1e.3.1_correctness.md)) |
| [`dump/state.rs:129-143`](../../../ein.rs/crates/ein-render/src/dump/state.rs) vs [`dump/lattice.rs:140-152`](../../../ein.rs/crates/ein-render/src/dump/lattice.rs) — two timeline emitters | divergent key order in `00_timeline.jsonl`, on a writer whose insertion-order preservation is *its whole reason to exist* ([SE-L1](../p1e.4_low/s1e.4.2_semantics.md)) |
| `run_tests.sh:56-62` vs `.github/workflows/per-commit.yml:5-9` — gate vs CI step lists | **three red CI commits**, recorded in both files' own headers ([TE-M8](s1e.3.6_tests.md)) |

Four pairs, three already-realised divergences. And in two of the four, a
comment *predicting the unification* exists and the unification never
happened — `imports.rs:46-48` names the phase (P1a.3) that would end the
duplication, and P1a.3 shipped without doing it.

The project's own method argues the fix: *checks over convention*, and *a
local gate that is a subset of the remote one is a local gate that lies*.
Both sentences are already in the tree; neither was applied to the four
places where one artifact is maintained twice.

**`AR-M2` — verdict read-out ownership split across three crates.** `Verdict`
is computed **once**, in `finalise`
([`solve.rs:2386-2445`](../../../ein.rs/crates/ein-infer/src/solve.rs)) — the
repo is careful about that and it is the right design. But what a *user*
reads is assembled downstream: the `Ambiguity` arm computes its own distinct
count and its own qualifier, the `Solution` arm inherits the raw node count
([`answer.rs:419-487`](../../../ein.rs/crates/ein-render/src/answer.rs)),
`ein test`'s header prints the search count under the verdict's letter, and
the summary emits both numbers
([`ein-cli/src/solve.rs:648`](../../../ein.rs/crates/ein-cli/src/solve.rs),
[`test.rs:784-838`](../../../ein.rs/crates/ein-cli/src/test.rs)).

**Every** observed k-vs-`solution_nodes` inconsistency in this review lives at
one of those seams. None is inside `finalise`. S1d.2.6 and S1d.3.3 each
touched the vocabulary and each missed a site — which is the prediction the
architecture makes.

## Acceptance

- ✅ **Each of `AR-M1`'s four pairs ends in one of three states**, and the state
  is visible at the code: unified into one artifact; mechanically compared by
  a test; or deliberately different with the reason written **at both sites**.
  Silence is not one of the three. Three end here — reserved names were
  [S1e.2.1](../p1e.2_high/s1e.2.1_correctness.md)'s, the entering event and the
  read-out are **unified**, the gate lists are **compared by a test** — and the
  macro pipeline is [`CO-M4`](s1e.3.1_correctness.md)'s, taken in the stage
  that owns it. The table of all five, with what each cost, is
  [`architecture.md` § One artifact, one owner](../../../docs/kernel/architecture.md).
- ✅ The gate/CI step lists are diffed by a test —
  `ein-cli/tests/gate_steps.rs`, **seven steps** each side, mutation-checked
  against the three drift shapes (CI gains a marked step, CI gains an unmarked
  one, a flag drifts) and it catches all three.
- ✅ **`AR-M2`**: the seam is **closed**. `Verdict::read_out` carries the count
  and the qualifier; `render_solution_table` no longer *takes* a count, so an
  arm has nothing to choose with; `ein test`'s header and `--stats` print what
  they are, under their own names. The fallback's comment is written at
  `finalise` anyway, because the hazard it warns about is now *where to add a
  word* rather than *which sites to visit*.
- ✅ **No new parallel copy is introduced by any fix in this milestone.** The
  opposite: nine hand-written matches over `Verdict` became three functions —
  see [the outcome](#outcome).

## Tasks

### Task T1e.3.4.1 — The three remaining `AR-M1` pairs ✅ (two of three; the third is `CO-M4`'s)

The reserved-name pair is [CO-H2](../p1e.2_high/s1e.2.1_correctness.md)'s.
The other three:

**Macro pipelines** — [CO-M4](s1e.3.1_correctness.md) owns the fix; this task
owns making sure the fix does not leave a *third* shape. If the dump path
genuinely needs lenient ingestion, the lenient path is a documented mode of
the one pipeline, not a second pipeline with a comment.

**Timeline emitters** — [SE-L1](../p1e.4_low/s1e.4.2_semantics.md) is the
finding. Extract a shared emitter, or pick one key order and use it in both.
The one thing to establish first: whether either order is pinned by a golden,
because `00_timeline.jsonl` is a dumped artifact and the JSON writer's
insertion-order preservation is deliberate. If a golden pins the current
divergence, the unification is a named re-bless.

**Gate vs CI** — a test that parses `run_tests.sh`'s step list and
`per-commit.yml`'s and diffs them. Both are line-oriented and neither is
generated, so the parse is a grep with an anchor comment on each side
(`# gate-steps-begin`). Put the test where the other repo-shape invariants
live rather than in a crate that has nothing to do with CI.
[TE-M8](s1e.3.6_tests.md) and [TE-L3](../p1e.4_low/s1e.4.5_tests.md) are the
same mechanism — `--tests-only` also skips the bench smoke, which is CI's last
step — so the diff should compare the flag's list too, not only the default
one.

### Task T1e.3.4.2 — `AR-M2`: decide seam-fix or fallback ✅ **seam fix**

Take the decision explicitly and early in the phase, because two other stages
branch on it.

**The seam fix.** `Verdict` gains what a read-out needs: the count that
should be printed, and the qualifier that should accompany it (`(a lower
bound — the search did not exhaust)`, `(not certified — pass --exhaustive)`,
or none). Then `answer.rs`'s arms print fields instead of choosing numbers,
`ein test`'s header prints the same fields, and the summary serialises them.
The property that makes it worth doing: **adding the next verdict word
becomes a change in one crate**, and S1d.2.6 demonstrated that it currently
is not.

Cost: a struct in `ein-infer`, three call sites, and every golden that
contains a rendered verdict line — which is most of the corpus's rendered
output. That is the honest reason it might not fit. If the rendered strings
are unchanged, no golden moves; establish that first, on the two entries that
exercise the qualifiers, before touching the rest.

**The fallback.** Fix [CO-M2](s1e.3.1_correctness.md) and
[SE-M1](s1e.3.2_semantics.md) at their arms, add the cross-surface
consistency test from
[S1e.3.2](s1e.3.2_semantics.md) T1, and write the hazard beside `finalise`:
*this is the only constructor of a verdict; three crates render it, and each
chooses its own count — the list is here, and a new count or qualifier has to
visit all of them.* Name the three sites in that comment. A hazard with a
site list is a materially different thing from a hazard.

Recommended: **attempt the seam fix, timeboxed to the stage**, with the
fallback as the stated exit. The fallback alone leaves the mechanism that
produced two findings in this review intact, and the review's phrasing is
exact — the seams are *where every observed inconsistency lives*.

### Task T1e.3.4.3 — Write the pattern down once ✅

`AR-M1` is a pattern, not four bugs, and the milestone will not be the last
time it matters. One short section — in
[`docs/kernel/architecture.md`](../../../docs/kernel/architecture.md) or
beside the determinism rules, which are the closest existing statement of
*this project's checks over this project's conventions* — naming the four
pairs, what each cost, and the rule: **a semantic artifact that exists twice
is unified, compared by a test, or annotated at both sites with the reason it
must differ.**

Include the case where the difference is legitimate, because there is one:
the lexer's `RESERVED` and the loader's `RESERVED` are genuinely different
sets ([SE-L2](../p1e.4_low/s1e.4.2_semantics.md)), and the fix there is a
rename, not a unification. A rule that cannot express the legitimate case
will be ignored the first time someone meets it.

## Notes

Three days assumes the seam fix is attempted and possibly abandoned. If it is
abandoned, the stage still delivers: three pairs resolved, the pattern
written, the fallback comment in place. If it succeeds, two findings in other
stages collapse into it and the phase gets shorter — which is the usual sign
the seam was the real finding.

---

## Outcome

Taken 2026-08-30, first stage of the phase, because
[T1e.3.4.2](#task-t1e342--ar-m2-decide-seam-fix-or-fallback--seam-fix) is the
decision two other stages branch on.

| | |
|---|---|
| **`AR-M1`** | **three of four pairs closed here**: the entering event **unified** (one `Timeline::entering`, plus `root_initial` / `layer_start` / `layer_end`, which were already identical and were written twice anyway), the gate lists **compared by a test**, the read-out **unified** under `AR-M2`. The fourth is `CO-M4`'s. The pattern is [`docs/kernel/architecture.md` § One artifact, one owner](../../../docs/kernel/architecture.md), five rows, with the legitimate case (`SE-L2`'s two `RESERVED` sets) and the test for telling them apart |
| **`AR-M2`** | **fixed — the seam, not the arms.** `Verdict::read_out(exhausted) -> ReadOut { k, qualifier }` in `ein-infer`; `render_solution_table` **drops** its `solution_nodes` parameter, so its four arms have no count to choose between; `Verdict::models` / `Verdict::states` / `Answer::k` replace nine hand-written matches in four crates |
| **`CO-M2`** | **fixed here**, subsumed as [S1e.3.1](s1e.3.1_correctness.md) T2 predicted. The `Solution` arm printed `stats.solution_nodes`; it prints nothing now, because the row is printed once above the match. The **fixture** for the mixed regime is still S1e.3.1's |
| **`SE-M1`** | **fixed here**, subsumed as [S1e.3.2](s1e.3.2_semantics.md)'s Notes predicted, taking that stage's option 2: `ein test -v` prints `k = {verdict}, recorded = {search}`. The cross-surface consistency **test** stays S1e.3.2's |
| **`SE-L1`** | **fixed here** ([P1e.4](../p1e.4_low/s1e.4.2_semantics.md)'s finding, closed by this commit). One golden moved: `dump_enterings_subset-pruned.txt`, **12 lines, key order only** — verified by parsing both sides and comparing field *sets* and values, not by reading the diff |
| **`TE-M8`** | **fixed here** ([S1e.3.6](s1e.3.6_tests.md)'s finding, which is the same mechanism as this pair). `TE-L3` — `--tests-only` also skips the bench smoke — is **not** closed: it is a claim about the script's header, not about the two lists, and stays with [S1e.4.5](../p1e.4_low/s1e.4.5_tests.md) |
| new | **a third `k` instance the review did not have**, and it is the visible one — `--stats` printed the label `solutions (k)` above `stats.solution_nodes`, so on the twelve `Open` entries one invocation printed `solutions (k) 0` and `solutions (k) 1` a screen apart. The row is `solution_nodes` now, which is what it has always been |
| measured | **285** declared `solve` runs in the corpus, **230** render a table; the printed count equals `verdict.k` on all 230 **before and after** the seam fix, so no rendered count moved. **12** rows have `verdict.k ≠ stats.solution_nodes`, every one an `Open` at 0 against 1; **0** rows are a `Solution` with `solution_nodes ≠ 1`, which is `CO-M2`'s regime being unreached |
| gate | `./run_tests.sh` green — **776 tests**, exit 0, bench smoke unmoved. **Two** goldens moved and both are the timeline's: `dump_enterings_subset-pruned.txt` (12 lines) and **110 of 8 835** lines of `corpus_shapes.md5`, every one a `dump[lattice]` or `dump[abort]` rendering at an unchanged line count. `corpus_exits.txt` unmoved |

### Five things the tasks did not predict

**1. The seam had nine copies, not two.** The review counted the sites that
choose a *count*. The same `match` over `Verdict` was also hand-written for
*which branches are distinct models* — three implementations, one of whose doc
comments said **“keyed the way `answer.rs` counts `k`”**, a parallel copy
naming the copy it is parallel to — for *which states does this verdict carry*
(three: `expect::check`, `print_final`, `events_verdict`, each deciding
independently what `Open` contributes), and for *`Aborted` falls back to the
counter* (three: `--json-summary`, the `verdict` event, `ein test`'s row).
Nine sites, three functions: `distinct` / `models`, `states`, `Answer::k`. The
`states` / `models` split is worth having a name for — it is M1d S1d.2.6's
distinction, and three surfaces want the states precisely because all three
are about a **fact set**.

**2. The cost was measured before the refactor, and it was zero.** The task's
own risk is *“every golden that contains a rendered verdict line — which is
most of the corpus's rendered output”*, and it says to establish otherwise
“on the two entries that exercise the qualifiers”. Establishing it on **230**
was no harder: sweep every declared `solve` run, capture the `solutions (k)`
row, diff before against after. Nothing moved. What made that cheap is that
the qualifier strings were *moved*, not rewritten — the seam fix is a change
of owner, and a change of owner should be invisible.

**3. The third instance was in one screen, and the test that should have
caught it had never read the row it names.**
`cli_semantics::the_stats_block_reports_the_same_counters_as_the_json_summary`
looks a row up by label prefix and takes the **first** match — and the table
is printed before the `--stats` block, so its very first assertion has been
comparing the *table's* `verdict.k` against `stats.solution_nodes` since it
was written. It passed because its fixture is an `Ambiguity` where the two
agree; on any of the twelve `Open` entries it would have compared 0 against 1.
A test that cannot tell two surfaces apart is the same defect as a reader who
cannot, which is `AR-M2` stated once more.

**4. A digest golden can still be verified exactly — by control.**
`corpus_shapes.md5` moved on 110 renderings, and its own header is candid that
*"it does not say **what** moved, and that is a real loss against a byte
golden"*. Restoring **only** the lattice key order — the shared emitter left in
place, everything else in the stage untouched — made the whole 8 835-rendering
sweep green again. So the 110 differ by the key permutation and by nothing
else: the seam fix, the read-out change and the nine-match collapse moved not
one byte of any rendering. That is a cheap experiment and it is what turns a
blessed digest from *accepted* into *verified*.

**5. The timeline divergence is a fossil of an `if` that no longer exists.**
`git show 4c1a5b3^` has both Python dumpers. `LatticeDumper.entering` built the
six always-present fields as a dict and then `rec.update({kind, firings,
unsat_core_size})` **only if `result is not None`**, so the three
result-derived keys landed last *because they were conditional*;
`MonotonicDumper.entering` passed all nine as keyword arguments in one call and
got the natural order. In ein.rs `EnteringInfo` is not optional — the condition
cannot arise — and the order it produced was ported anyway and then maintained
by hand on both sides. The order kept is `MonotonicDumper`'s, which is the one
ein.py used when it did not have to append; the finding's *“pick one key
order”* had a right answer rather than a coin flip.

### What this stage did **not** do

- **The macro pipeline pair.** `CO-M4` owns it and
  [S1e.3.1](s1e.3.1_correctness.md) is next. The constraint this task owes it
  is recorded there and in `architecture.md`: the fix must not leave a *third*
  shape, and the two ingestion functions are not merely two copies — they
  differ in **strictness** (`collect_macros` is first-declaration-wins and
  silently skips a malformed form; `from_ir::ingest_macros` errors on a
  duplicate and on a reserved name), so unifying them is a behaviour change for
  the four non-loader consumers and has to be measured, not assumed.
- **A fixture for `CO-M2`'s mixed regime** — one discharged model beside an
  open state. `finalise` defines it and nothing reaches it; S1e.3.1 T2 owns
  building one, and until it exists the *visible* regression for the class is
  the `Open` entry in `ein-cli/tests/read_out.rs`.
- **`T1d.10.6.4`**, which the tree traversal's read-out still waits on. Nothing
  here touches what a tree reports.

### Tests and files

| | |
|---|---|
| new | `ein-cli/tests/read_out.rs` — the seven reachable `(word, exhausted)` cells, one count per label per invocation, and the `Open` entry where the two numbers differ |
| new | `ein-cli/tests/gate_steps.rs` — the two step lists, and *every CI command is a gate step or says why not* |
| new | `ein-infer/src/verdict.rs` unit tests — the **eight**-cell qualifier table, including the truncated `Open` no program reaches; `k` is `models().len()`; `distinct` keys by facts; `Aborted` reports the counter |
| new | `ein-render/tests/dump_shape.rs::both_file_dumpers_write_one_entering_shape` |
| moved | `ein-render/tests/golden/dump_enterings_subset-pruned.txt` (12 lines) and `corpus_shapes.md5` (110 of 8 835) — key order, verified by control |
| doc | `docs/kernel/architecture.md` § One artifact, one owner + one cookbook row; `run_tests.sh` and `per-commit.yml` headers; the hazard at `finalise` |
