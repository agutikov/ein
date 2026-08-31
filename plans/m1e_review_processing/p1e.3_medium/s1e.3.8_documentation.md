# S1e.3.8 — Documentation (Medium)

**Phase:** [P1e.3](README.md) (Medium)
**Estimate:** 3 days
**Depends on:**
[Q-M1e.4](../open_questions.md#q-m1e4--does-the-repo-want-an-exact-count-in-prose-at-all)
— decided before the pass, or the pass is a counting pass that rots again.
**Runs last** among the phase's doc stages: every other stage here changes a
count.
**Findings:** [`DO-M1`](../review/documentation/medium.md),
[`DO-M2`](../review/documentation/medium.md).
**Status:** ✅ **done 2026-08-31** — the decision first, then a pass that is
mostly a **de-counting** one; `DO-M2` turned out to be **264** links rather
than six, and the sweep that found them is now a gate step. See
[§ Outcome](#outcome).

## Context

**`DO-M1` is not a list of typos. It is one mechanism, observed eight times**,
and the repo already states the mechanism: *a page nothing runs goes stale*.

The verified instances, with the current truth where the plan re-checked it at
`9aa598a`:

| claim | stated | actual |
|---|---|---|
| stdlib rules | 73 | **77** |
| `tests/stdlib/` programs | 45 / 47 / 56 — *inconsistent inside `corpus/README.md` itself* | **56** |
| corpus entries | 180 / 189 | **197** |
| cells | 225 | 280 |
| sweep cells | 622 / 641 | ~990 |
| renderables | 84 | 89 |
| gate tests ([`README.md:73`](../../../README.md)) | 703 | **738** |
| `utils/` scripts ([`README.md:341-344`](../../../README.md)) | eighteen | **23** |
| `broken/load` fixtures ([`defined_behaviour.md:135`](../../../docs/kernel/defined_behaviour.md)) | *23 of the 30* | **37** |

By contrast **every number a test pins is exactly right** — 77 of 77, 56
expectations, 49/516 lines, the embedding output. The repo demonstrates its
own thesis on its own docs. The stdlib row of `corpus/README.md` was patched
for S1d.2.2 (+2) but not S1d.2.4 (+9), which dates most of the rot to one
milestone and shows the failure mode precisely: a milestone updates the
numbers it *thinks about* and not the ones it moves.

The second-order problem is worse than any single number: `CLAUDE.md` claims
the doc tree is checked by `cargo test`, so a reader has **no way to know
which numbers are in the checked class**.

**`DO-M2`** is the same rot in links: the M1a/M1c/M1d plan-tree deletions left
link text whose targets are gone, plus several section anchors that never
existed — `§3d.iii`, `§3d.iv`, `§3d.vii`, `§3e` in
[`inference/README.md`](../../../docs/kernel/inference/README.md) and
[`lattice_diagrams.md`](../../../docs/kernel/inference/lattice_diagrams.md).
The worst instance is *"see C2"* (`examples/README.md:28-29`,
`stdlib/README.md:139-140`), where the referent is unlocatable from either
document.

## Acceptance

- **No prose count survives that nothing generates, dates or owns.** Whichever
  shape [Q-M1e.4](../open_questions.md#q-m1e4--does-the-repo-want-an-exact-count-in-prose-at-all)
  picks, it is applied to **all** the sites above — including the in-code doc
  comments the review lists (`stdlib_coverage.rs`, `corpus_cli.rs`) and
  `utils/stdlib_census.py`'s own docstring, which are the same rot in a
  different file type.
- A reader can tell which numbers are checked. That is a sentence in
  `CLAUDE.md`/`README.md` and a convention in the pages, not a wish.
- **The doc tree has no dangling internal link or nonexistent anchor**, and
  the check that establishes this is repeatable — a markdown link checker over
  `docs/`, or an explicit statement that it was a one-time pass and why that
  is acceptable.
- The *"C2"* referent is resolved: restored into `docs/history/` from git, or
  the sentences deleted. Not left as bare text.

## Tasks

### Task T1e.3.8.1 — Decide the shape before counting

[Q-M1e.4](../open_questions.md#q-m1e4--does-the-repo-want-an-exact-count-in-prose-at-all)
lists three shapes the repo already uses somewhere, plus one it does not:

| shape | example in the tree | cost |
|---|---|---|
| **generated** — the number lives in a marked region a test diffs | `docs/api/rust.md`'s `rust` block | a test per site; only worth it where the surrounding text is also generated |
| **census-owned** — say *the census prints it* and link | `corpus_cost.md`, `stdlib_census.md` | zero, and it is the shape most of these sites want |
| **dated** — *as of the M1a close, 616* | the frozen CPython/PyPy columns | zero, and honest, but it makes a page read as history |
| **checked prose** — a script asserting a stated number matches a command's output | does not exist | new machinery; probably not worth it |

The recommendation, per site class: **census-owned** for anything a `utils/`
script already computes (stdlib rules, corpus entries, `tests/stdlib`
programs, cells, sweep cells); **dated** for measurements
([DO-L2](../p1e.4_low/s1e.4.7_documentation.md)'s frozen timings are the same
question); and **generated** for nothing new, because the one generated site
that exists already showed its blind spot
([CD-M4](s1e.3.7_code_doc_consistency.md)).

Take the decision, write it into
[`open_questions.md`](../open_questions.md) as decided, then pass.

### Task T1e.3.8.2 — The pass

Every site in the table above, plus the in-code doc comments
([`stdlib_coverage.rs:8, 20, 32, 210`](../../../ein.rs/crates/ein-infer/tests/stdlib_coverage.rs),
[`corpus_cli.rs:9, 44-45, 97-103, 107, 216`](../../../ein.rs/crates/ein-cli/tests/corpus_cli.rs)),
`utils/stdlib_census.py`'s docstring, and `README.md`'s Layout rows — which
also miss `m1d` from `docs/history` and describe `--version` as four things
where the report has five lines.

Two rules for the pass:

1. **Quote from a run.** The gate test count comes from `./run_tests.sh`, the
   corpus count from the manifest, the script count from `ls utils/`. A number
   copied from another document is how `corpus/README.md` came to state three
   different values for one thing.
2. **Do it after the phase's other stages.** [S1e.3.6](s1e.3.6_tests.md) adds
   tests and fixtures; [S1e.2.1](../p1e.2_high/s1e.2.1_correctness.md) adds
   `broken/` fixtures; both move numbers this pass would otherwise state
   wrongly a second time.

### Task T1e.3.8.3 — The dangling-reference sweep

The named instances:

- *"see C2"* — [`examples/README.md:28-29`](../../../examples/README.md),
  [`stdlib/README.md:139-140`](../../../stdlib/README.md). Decided in
  [CD-M6](s1e.3.7_code_doc_consistency.md); carried out here if not there.
- bare `M1 P1.3`-style paths —
  [`02_rules.md:586-587`](../../../docs/kernel/ir/01-ein-graph/02_rules.md).
- `s1.6.5_idea08_checklist.md` —
  [`zebra_walkthrough.md:52`](../../../docs/kernel/inference/zebra_walkthrough.md).
- `r6_seam.md` —
  [`architecture.md:132`](../../../docs/kernel/architecture.md).
- `[project-set-search-unified memory]` —
  [`algorithm_layer_n.md:42, 522`](../../../docs/kernel/inference/algorithm_layer_n.md)
  (a page [S1e.2.2](../p1e.2_high/s1e.2.2_code_doc_consistency.md) may have
  bannered or moved — check its disposition first).
- nonexistent anchors `§3d.iii` / `§3d.iv` / `§3d.vii` / `§3e` —
  `inference/README.md:916, 1060, 1064`, `lattice_diagrams.md:216, 251-252`.

For each: restore the target into `docs/history/` from git, retarget the link,
or delete the sentence. The middle option is usually right for a deleted plan
document — the *content* often survives in a `docs/history` README — and the
last is right where the sentence only existed to carry the link.

### Task T1e.3.8.4 — Make it repeatable, or say it is not

The `rustdoc` step already catches this class **for crate docs**, and it found
twelve unresolved intra-doc links the first time it ran — which is the
argument that a markdown-link checker over `docs/` would pay for itself. It is
a small script: resolve every relative link and anchor in every `.md` under
`docs/`, `plans/`, and the root READMEs; report the misses.

Two things it must handle to be worth having: **anchors**, since four of the
found defects are anchors that never existed, and **intentional non-links**
(code spans, historical references). If those make it fiddly, the honest
alternative is to state that the sweep was a one-time pass with a date, and to
put the check on the [S1e.2.2](../p1e.2_high/s1e.2.2_code_doc_consistency.md)
T5 doc-pass checklist instead.

Do not add it to `run_tests.sh` in the same commit that writes it. Run it, fix
what it finds, then decide whether it joins the five static checks — a new
gate step that fails on day one is a gate step people learn to skip.

## Notes

The temptation in this stage is to fix the numbers and move on. That is a
three-hour job and it is why the numbers are wrong: it has been done before.
The deliverable that lasts is T1e.3.8.1's decision — and the honest version of
that decision may well be *stop stating exact counts in prose*, which makes
most of the pass a deletion rather than an update.

One number is worth keeping exact wherever it appears, because a reader uses
it to check their own build: the gate's test count. If it stays, it is dated.

---

## Outcome

**Done 2026-08-31**, in four commits: the decision + the pass, the sweep, the
gate step, and this record.

### T1e.3.8.1 — the decision, and why it is a rule rather than a shape

[Q-M1e.4](../open_questions.md#q-m1e4--does-the-repo-want-an-exact-count-in-prose-at-all)
is answered **yes, and the warrant is the price**: a count stands in prose when
the sentence carries the **command** that re-takes it, the **date** it was
taken, or the **test** that pins it; a count that can carry none is deleted and
what prints it is named instead. That is this stage's own acceptance turned
into something an author applies without deciding anything.

The task offered four *shapes* and asked which one. The answer is that the
shape follows from **what the number is**, and the evidence is in the review's
own table: **all nine drifted counts are sizes** — how many files, rules,
entries, cells, scripts are in a place. A size has no interesting history, so a
date says nothing about it; it moves on a milestone that was not thinking about
it, which is the mechanism `DO-M1` measured. A *measurement* takes the date,
because a finding without one is not a finding. A *test-pinned* count keeps its
digit and names the test — the only class `AGENTS.md`'s **checked by
`cargo test`** was ever true of, which is the second-order problem the finding
names, and naming the test is what lets a reader tell.

**The fourth shape is refused**, and the reason generalises past this stage:
the counts that are right are the ones behind an instrument that is *run* —
`cargo test`, `stdlib_census`, `corpus_cost` — not the ones behind an
instrument that exists. A script written to keep a README honest and run by
nobody is the same failure one level up, and it would be the second thing in
the tree claiming the docs are checked.

**A date does not stop drift. It makes drift legible, which is the whole ask.**
`README.md:73` is the proof: it was the one site of the nine that already
carried a warrant, *"703 tests over 77 targets, 2026-08-25"*, and it was
**wrong** and *not misleading*. The other eight were indistinguishable from
current.

### T1e.3.8.2 — the pass

Every number quoted from a run, as the task's first rule requires:

| | stated | 2026-08-31 |
|---|---|---|
| stdlib rules | 73 | **77** |
| `tests/stdlib/` programs | 45 / 47 / 56 | **57** |
| corpus entries | 180 / 189 / 197 | **217** |
| `tests/stdlib`'s cells | 225 | **282** |
| sweep cells | 622 / 641 / 889 / 901 | **1 047**, 1 035 default |
| renderables | 84 | **100** |
| gate tests | 703 over 77 | **802 over 90** |
| `utils/` scripts | eighteen | **25** |
| `broken/load` ending `at None` | *23 of the 30* | **30 of 41** |

The cell count's warrant is the best one available and it was already in the
tree: `wc -l ein.rs/crates/ein-cli/tests/golden/corpus_exits.txt` is one line
per cell, written by the sweep itself, so it cannot drift from what it counts.

Four claims the review did not list, found by re-taking rather than by reading:
`corpus_cli.rs` documented **three** slow entries where
`the_slow_set_is_exactly_these_two` now pins two; `README.md`'s layout table
was missing M1d from `docs/history/`, still listed M1d as roadmap, and stopped
`plans/followups/` at F17; and `ein --version` is described as four things
where it prints five lines.

### T1e.3.8.3 — the sweep, and what it actually was

`DO-M2` names six dangling references. The instrument found **264**, and they
are one mechanism rather than a scatter:

| | |
|---|---:|
| `docs/history/m1d_satisfiability/` | **251** |
| the rest of `docs/` | 4 |
| `plans/` + the root READMEs | 9 |
| `docs/kernel/` | **0** — [S1e.2.2](../p1e.2_high/s1e.2.2_code_doc_consistency.md) had swept it |

M1d's fifteen documents were lifted out of `plans/m1d_satisfiability/p1d.N_*/`
when that tree was deleted on 2026-08-27, and every relative link went with
them unchanged. Retargeted against anchors that **exist**, by *number* rather
than by the filename's slug — `s1d.2.2_domains.md` is `### S1d.2.2 — The domain
contract`, `p1d.4_model_set_closure` is `## P1d.4 — Closing the model set` —
because the mapping that reads plausible is the one that silently writes a
dead anchor.

Thirteen needed a decision, and two are worth reading. Four references are to
**S1d.10.3, a stage that never shipped**; they now point at § What P1d.10 was
closed without, which names its ledger as dropped, and that is more use than
the file would have been. One link was **deleted**: its sentence answered its
own question in the clause after the colon, and its target had not survived the
move.

**The sweep found three defects in the checker itself**, all invisible while it
only ran on its own default: it crashed on a relative argv (so it could not be
pointed anywhere else at all — which is why nobody had); it read
`F_r[R,S,T](x)` and the stage skeleton's `[<idea>](../../ideas/<file>.md)` as
links; and its prose-`§` scope ran 100 characters past a link and could charge
one link with the *next* link's section reference. The last is the check
S1e.2.2 wrote this script for, mis-firing on the shape it was written to catch.

### T1e.3.8.4 — it joins the gate

0 findings over 280 pages, 0.29 s, no dependency the gate does not have. Three
lists move together — `run_tests.sh`, `per-commit.yml`, and `gate_steps.rs`'s
`--tests-only` set — and `gate_steps` **failed** on the third until it was
updated, which is `AR-M1`'s fourth pair doing its job.

Whole tree rather than `docs/kernel/`, and that is the finding rather than a
preference: `docs/kernel/` was clean and 251 of 264 were somewhere else. The
script's other two questions — do identifiers resolve, is the page in a
declared state — *are* specific to the specification tree and stay reports
there, check 1 permanently so, for the reason its own row gives.

### What this stage did not do

- **`plans/` was not de-counted.** Its counts drift too (`plans/README.md`
  still says *Q-M1e.1–5* for a file with twenty-two), but a plan is a document
  about work that has not happened, and the rule above is about documents a
  reader trusts for the state of the tree. The links there **are** in the gate.
- **`docs/history/` numbers were not re-taken.** They are the readings of
  their days. Only their *links* moved, which preserves a reference rather
  than rewriting a record.
- **No `doc_counts.py`.** That is the decision, not an omission.
