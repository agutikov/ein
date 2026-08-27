# S1e.1.6 — What nothing pins: Q8, Q9, Q10

**Phase:** [P1e.1](README.md) (The ten questions)
**Estimate:** 2 days
**Depends on:** nothing.
**Blocks:** nothing directly. It governs what the **milestone may claim** at
its close, which is why it is in the first phase and not the last.
**Answers:** [`review/open-questions.md`](../review/open-questions.md) Q8, Q9,
Q10.

## Context

Three questions about **absence**, which by
[Q-M1e.1](../open_questions.md#q-m1e1--what-is-the-standard-of-proof-for-refuted)'s
second rule are the cheap ones to answer: name the thing, or add it.

**Q8 — the two encodings.** `examples/zebra.ein` and `examples/zebra2.ein`
encode one puzzle two ways and are supposed to agree cell by cell. The claim's
named owner was `acceptance/test_zebra_two_ontologies.py` — a Python file in a
directory that no longer exists
([`examples/README.md:27`](../../../examples/README.md)). The acceptance CLI
tests solve both files; whether anything compares the two **models to each
other** was not established. If nothing does, a load-bearing claim about the
data model — *the same world, stated two ways, is the same world* — is
unenforced, and it is one assertion to fix.

**Q9 — the review's own hole.** The review's second stage — thirteen
per-dimension deep finders plus adversarial verification of every finding —
was **aborted before returning results**
([`review/summary.md`](../review/summary.md) § Method). Four surfaces got no
dedicated pass at all:

| surface | what was not done |
|---|---|
| algorithmic complexity / pathology | no analysis of where the search degrades, no adversarial input class |
| [`ein-einb/src/cast.rs`](../../../ein.rs/crates/ein-einb/src/cast.rs) | the one module in the tree permitted `unsafe`, audited only against its own stated invariants |
| parser / CLI edges | no hands-on fuzz-style probing — and the one such probe that *was* run found a process panic ([CO-H1](../README.md#the-findings)) |
| micro-CSP ground truth | no verdict checked against an independently computed answer |

*Absence of findings there is absence of evidence.* The milestone's closing
statement has to say so, and this stage is where that sentence gets written —
plus one of the four actually swept, because a stage that only files owners
has not moved anything.

**Q10 — the release matrix.** `release.yml`'s macOS, Windows and aarch64
legs, the jobs-cross-diff and the `--no-default-features` leg have never
executed; the workflow is honest about it in its own header. No repository
evidence can resolve this — only a tag can.

## Acceptance

- **Q8**: either the existing test that pins the two encodings to one model
  is named — in `examples/README.md`, replacing the dead Python pointer — or
  a cross-encoding assertion is added and named there. The fix to the
  surrounding prose is [CD-M6](../p1e.3_medium/s1e.3.7_code_doc_consistency.md)'s;
  the *assertion* is this stage's.
- **Q9**: the four unswept surfaces are scoped — one paragraph each saying
  what a pass would look like and who owns it — and **the CLI/parser edge
  sweep is done here**, because it is the one with a demonstrated hit rate.
- **Q10**: the release matrix's status is stated where a reader would
  otherwise believe the badge, and the decision — run it on a pre-release tag
  now, or accept it until the first real tag — is recorded with a date.
- The milestone README's closing claim is drafted in this stage, not
  improvised at the end: *what this milestone checked, and what it did not*.

## Tasks

### Task T1e.1.6.1 — Q8: find the assertion or write it

Grep the four crates' test files for a comparison between the two puzzles'
model sets — not for two solves, which certainly exist, but for a diff. The
likely places are `ein-cli/tests/cli_semantics.rs` (which already carries
zebra anchors) and `ein-cli/tests/embedding.rs`.

If it is absent, the assertion is small and belongs where the anchors already
are: solve both with `-e`, project each model onto the shared vocabulary —
the two encodings differ in *how* they say it, so the comparison is over the
puzzle's answer cells and not over raw fact lists — and assert equality. Note
in the test what the projection is; that projection *is* the claim, and an
undocumented one would be the next reviewer's question.

While there: [TE-L2](../README.md#the-findings) says any legitimate edit to
either puzzle fans into at least four crates' tests plus `docs/api/rust.md`,
with no list anywhere. The list is a natural by-product of this grep — hand
it to [S1e.4.5](../p1e.4_low/s1e.4.5_tests.md) rather than writing it twice.

### Task T1e.1.6.2 — Q9: scope the four, sweep the one

**Scope**, one paragraph each, into the milestone README or a short section
here: what a real pass over that surface would consist of, what it would
cost, and who owns it. Owners that exist: `cast.rs` is
[design/12](../../../docs/history/m1a_rust/design/README.md)'s subject and an
audit belongs with whoever next touches `ein-einb`; micro-CSP ground truth is
[M10](../../m10_external_benchmarks/README.md)'s thesis exactly — *the same
problem stated for six other systems* — and should be named there rather than
duplicated here; algorithmic pathology has no owner and becomes a
`Q-M1e.<n>`.

**Sweep** the parser/CLI edges here, because the evidence says it pays: the
one probe of this class that ran found a process panic from well-formed
input. The sweep is not a fuzzer — it is a systematic pass over the built-in
predicate and structural-form arities, in the shape
[CO-H1](../p1e.2_high/s1e.2.1_correctness.md) exposes:

- every built-in predicate (`eq`, `neq`) at arity 0, 1, 3;
- every structural form (`and`, `or`, `not`, `absent`, `false`) empty, at
  arity 1, and nested illegally;
- every declaration head with a missing or extra field;
- every CLI flag with an empty, zero, negative and absurd value.

Each cell must produce a **positioned diagnostic and exit 1** — never a
panic, never a silent accept. The product is a table of what each does today
and a `broken/` fixture for every cell that is wrong. This is where
[CO-H1](../p1e.2_high/s1e.2.1_correctness.md)'s *class* gets closed rather
than its one instance.

### Task T1e.1.6.3 — Q10: state the release matrix's status, or run it

Two options and the stage picks one:

- **Run it** on a pre-release tag (`v0.0.0-rc1` or similar), which converts
  four untested legs into evidence for the cost of one tag and one CI
  minute-budget. This is the review's own recommendation.
- **Accept** until the first real tag, and add one sentence where a reader
  would otherwise be misled — the workflow header already says it, so what is
  missing is the same sentence in `docs/install.md`, which is the page that
  offers *a release binary* as a channel.

Whichever, [TE-L5](../README.md#the-findings) is dispositioned by this task
and [S1e.4.5](../p1e.4_low/s1e.4.5_tests.md) does not re-decide it.

### Task T1e.1.6.4 — Draft what the milestone may claim

One paragraph, filed in the milestone README, written now and edited at the
close rather than composed then. It states: the review's verification stage
did not run; the four surfaces above had no dedicated pass; this milestone
swept one of them; and the tree's cleanliness outside those surfaces is
supported by a reading pass, a green gate and the fixtures this milestone
added — and by nothing else.

Writing it first is not ceremony. A closing claim drafted at the end is
written by someone who wants to be finished.

## Notes

Q9's honest reading is that this milestone is the *processing* of a review
that was not completed, and that the review that was not completed should
eventually be. That is a real piece of future work and it is not filed as a
task here, because re-running a thirteen-finder pass is a milestone's worth of
compute, not a stage's. If the sweep in T1e.1.6.2 finds a second panic-class
defect, that changes — two hits from two probes is a rate, and a rate is an
argument for the full pass.
