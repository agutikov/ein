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

- **Each of `AR-M1`'s four pairs ends in one of three states**, and the state
  is visible at the code: unified into one artifact; mechanically compared by
  a test; or deliberately different with the reason written **at both sites**.
  Silence is not one of the three.
- The gate/CI step lists are diffed by a test. Both files are simple enough to
  parse, and the failure they prevent has already happened once.
- **`AR-M2`**: either the seam is closed — `Verdict` (or a read-out struct
  beside it) carries the printable counts and qualifiers, and downstream
  surfaces *render* rather than recompute — or the fallback is taken in full:
  the two instances fixed **and** the hazard written where `finalise` is, so
  the next verdict word's author is warned by the code rather than by this
  plan.
- No new parallel copy is introduced by any fix in this milestone. (Worth
  stating: the obvious fix to a divergent pair is sometimes a third
  representation.)

## Tasks

### Task T1e.3.4.1 — The three remaining `AR-M1` pairs

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

### Task T1e.3.4.2 — `AR-M2`: decide seam-fix or fallback

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

### Task T1e.3.4.3 — Write the pattern down once

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
