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

### Task T1e.1.6.1 — Q8: find the assertion or write it ✅

**Done 2026-08-29 — it exists, and it is stronger than the assertion this task
would have written.** Two tests in
[`ein-infer/tests/acceptance.rs`](../../../ein.rs/crates/ein-infer/tests/acceptance.rs),
sharing one constant:

| test | what it holds |
|---|---|
| `the_generic_link_encoding_is_the_unique_solution` | `zebra.ein`'s model places all 25 `GRID` cells through `co-located`, at `k = 1` and **`exhausted`** |
| `both_ontologies_reach_the_same_model` | `zebra2.ein`'s model is **exactly** those 25 cells, read through the five `*-loc` projections |

`GRID` is the Wikipedia answer as `(house, value)` pairs — vocabulary
independent — and both encodings are compared against **it**, not against each
other. That is the stronger claim, and the file's own module doc says why in a
sentence this task should not have needed to rediscover: *"a port can agree
with an oracle about a wrong model, and the corpus diff would be green."* Two
encodings agreeing is compatible with both being wrong; two encodings each
matching the published grid is not.

Verified independently before reading them, by projecting both `-e` model sets
onto the shared cell vocabulary — `(co-located V House-k)` against
`(<attr>-loc V House-k)`: **25 cells each, identical key set, identical
values.**

What was actually absent is the **pointer**.
[`examples/README.md`](../../../examples/README.md) still named
`acceptance/test_zebra_two_ontologies.py`, in a directory deleted with
`ein.py/` at M1a S1a.10.5 — six days before the review read it. It names the
two tests now, and says what `GRID` is, because a reader who follows a dead
pointer concludes the claim is unenforced, which is what the review concluded.

**`TE-L2`'s list, as a by-product, and the finding understates it.** Not four
crates: **six**, and 26 test files, plus `docs/api/rust.md`, seven `utils/`
scripts, `corpus.toml`, the generator and its four generated files, and four
goldens — two of which are `from_ein_py/` and may never be re-blessed. The
whole table is handed to
[T1e.4.5.2](../p1e.4_low/s1e.4.5_tests.md) rather than written twice.

The task as written:

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

### Task T1e.1.6.2 — Q9: scope the four, sweep the one ✅

**Done 2026-08-29. The sweep found a rule, not a list** — which is what makes
it a closure of `CO-H1`'s class rather than a second instance:

> [`00_ebnf.md` §2](../../../docs/kernel/ir/03-ein-lang/00_ebnf.md) has a block
> headed **Kernel meta-primitives (shape-pinned)** with four productions —
> `NotForm`, `NeqForm`, `AndForm`, `OrForm`. The engine has **seven** such
> primitives. Every cell that panics or silently misbehaves is one of the three
> the block does not name; every cell of the four it does is a positioned parse
> error.

21 cells, **seven wrong** — six rows below, since the first covers two —
and every one of them in `eq` or `absent`:

| written | a reader expects | today |
|---|---|---|
| `(eq)` · `(eq ?x)` | a diagnostic | **panic**, exit 101 — this is `CO-H1` |
| `(eq ?x A B)`, `A ≠ B` | a diagnostic, or silence | **fires** — `guard_holds` reads `args[0..2]` and drops the rest |
| `(eq ?x A B C)` | as above | **fires** |
| `(absent)` | a diagnostic — `(absent ?x)` gets a `CompileError` saying the guard can never pass | **silence**, and the rule is retired for the run |
| `(absent (q ?x) (p ?x))`, `p` non-empty | a diagnostic, or silence | **fires** — everything past the first argument is dropped |
| `(absent (q ?x) (p ?x) (p ?x))` | as above | **fires** |

against `neq` — the same registry, the *pinned* half — where arity 0, 1, 3 and
4 are each a positioned `file:line:col: unexpected input`.

**The five silent cells are worse than the two panics**, and that is the part
the review could not have guessed from the one instance it had. A panic is
loud and stops the run. A guard that quietly evaluates a weaker condition than
the one written is a **wrong answer with a success exit code**: a three-way
equality reads as two-way, a two-subject `absent` reads as one-subject.

Banked as
[`ein-cli/tests/primitive_arity.rs`](../../../ein.rs/crates/ein-cli/tests/primitive_arity.rs)
— two tests, one per half of the rule, pinning today's behaviour **including
the defects**, so the fix has to come through them. It is a subprocess sweep
because two of the five outcomes are *process* outcomes (exit 101, a panic
line); `saturate` rather than `solve` because `q` is a declared relation and
the blind enumerator would guess `(q A)`, which would make every silent cell
read as a firing one. The fix is filed as
[Q-M1e.18](../open_questions.md#q-m1e18--three-kernel-primitives-are-not-shape-pinned-and-drop-their-extra-arguments)
with three candidates; `CO-H1`'s repair stays
[S1e.2.1](../p1e.2_high/s1e.2.1_correctness.md)'s and now has a rule to fix
rather than a case.

**The CLI half of the sweep reproduced an existing finding against the
binary.** Every value-taking option × {empty, zero, negative, absurd}, four
subcommands: the numeric clamps are
[Q-M1e.17](../open_questions.md#q-m1e17--three-py_int-options-silently-reinterpret-a-negative)'s
already, and the artefact paths are
[`EH-M1`](../README.md#the-findings) — `--events`, `--json-summary` and
`--trace` print *No such file or directory* and **exit 0**, on an empty path
and on an unwritable one alike; `--dump-states` exits 1 on the second and is
silent on the first. That finding was one reader's reading until today.

**The four surfaces, scoped:**

| surface | what a pass would be | owner |
|---|---|---|
| **parser / CLI edges** | this | **done here** |
| `ein-einb/src/cast.rs` | read the module against [design/12](../../../docs/history/m1a_rust/design/12_toolchain_and_layout.md) §2's permission rather than against its own doc comments: every `unsafe` block's precondition traced to a caller that establishes it, and a `#[cfg(miri)]` run of `roundtrip.rs` over the corpus containers. Half a day, and it needs someone with the container format in their head | **whoever next changes `ein-einb`** — it is the only crate not `#![forbid(unsafe_code)]` and the audit belongs with the change, not with a calendar |
| micro-CSP ground truth | a verdict checked against an answer computed outside the engine — which is not a probe, it is [M10](../../m10_external_benchmarks/README.md)'s entire thesis (*the same problem stated for six other systems*) and its `smt/` encodings are already in the tree | **M10**, named there rather than duplicated here |
| algorithmic complexity / pathology | a family parameterised by *n*, run until it stops finishing, with the exponent read off — a measurement phase, not a stage | **nobody** — [Q-M1e.19](../open_questions.md#q-m1e19--algorithmic-pathology-has-no-owner) |

One incident from building the sweep is worth keeping, because it is the same
mistake the sweep is about: both tests first shared a scratch directory keyed
on the pid, so the second test's setup deleted the first's files mid-run and a
missing input read as **a refusal** — in a sweep whose entire subject is what
gets refused. `cli_semantics.rs`'s `Scratch::new(tag)` already had the answer.

The task as written:

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

### Task T1e.1.6.3 — Q10: state the release matrix's status, or run it ✅

**Decided 2026-08-29: accept, and the reason is not cost.** The workflow's
`publish` job **creates a public GitHub release**. Pushing `v0.0.0-rc1` is
therefore a statement about what this project has shipped, and that is the
maintainer's to make, not a stage's — a throw-away tag is only throw-away if
nobody sees it. The decision is recorded where a reader of the *binary* channel
meets it, in [`docs/install.md`](../../../docs/install.md)'s existing caveat
block, with the one command that reverses it.

**The status was already stated in three places** — `release.yml`'s header,
`docs/install.md`'s block, and the M1a README's S1a.9.3 (*"What no machine here
has run is the build matrix"*). So Q10 is answered by naming them, which is
[`standard_of_proof.md`](../../../docs/kernel/standard_of_proof.md) Rule 1's
second row. **One of the three was a broken pointer**: `release.yml` cited
*docs/history/m1a_rust/README.md § What CI has not yet proved*, and no such
section has ever existed. It now cites the one that says it. That is the whole
of the defect Q10 could actually find in the repository — the rest of the
question is only answerable by a tag.

`TE-L5` is dispositioned **accepted** by this task, and
[S1e.4.5](../p1e.4_low/s1e.4.5_tests.md) does not re-decide it.

The task as written:

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

### Task T1e.1.6.4 — Draft what the milestone may claim ✅

**Drafted 2026-08-29**, into the milestone README as
[§ What this milestone may claim at its close](../README.md#what-this-milestone-may-claim-at-its-close),
inside the acceptance section so that it is read where the closing is decided.
It states the verification stage did not run, tables the four surfaces with
*swept* / *not swept* and an owner apiece, and ends where the Notes below
predicted it would have to: **this milestone may not be read as saying the tree
is clean**, and the review it processed should eventually be re-run.

The Notes' trigger fired, and the draft says so. One probe of one surface
returned a **five-cell silent defect class** on top of the one instance the
review had already found there — not a second panic, but five cells that are
worse than a panic, because they answer wrongly and exit 0. Two hits from two probes is a
rate.

The task as written:

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

**It found one.** Not a panic — **five** cells that answer *wrongly* and exit
0, which is the same argument one severity up. `Q9`'s honest reading stands, and
the closing claim drafted in T1e.1.6.4 says so in the milestone's own words.

## What landed

| | |
|---|---|
| **Q8** | **answered — named, not written.** `ein-infer/tests/acceptance.rs`'s two tests pin each encoding to `GRID`, the published 25 cells; comparing the two models *to each other*, which the task proposed, would have been the weaker claim. `examples/README.md`'s pointer at the deleted Python file now names them |
| **Q9** | **partly answered, and the answer is no.** One surface swept — 21 cells, **seven wrong** — and the rule that predicts every one of them. Three surfaces scoped with owners, one of which had none and is now [Q-M1e.19](../open_questions.md#q-m1e19--algorithmic-pathology-has-no-owner) |
| **Q10** | **accepted, dated, with the reason** — `publish` creates a public release, so a tag is the maintainer's call. The status was already stated three times; one of the three pointers was broken and is fixed |
| the test | [`ein-cli/tests/primitive_arity.rs`](../../../ein.rs/crates/ein-cli/tests/primitive_arity.rs) — two tests, 21 cells, pinning today's behaviour *including the defects* |
| filed | [Q-M1e.18](../open_questions.md#q-m1e18--three-kernel-primitives-are-not-shape-pinned-and-drop-their-extra-arguments) (the unpinned three, three candidate fixes) · [Q-M1e.19](../open_questions.md#q-m1e19--algorithmic-pathology-has-no-owner) (no owner) |
| reproduced | [`EH-M1`](../README.md#the-findings) against the binary — reading-pass → verified, on both an empty and an unwritable path. The ruling stays [S1e.3.5](../p1e.3_medium/s1e.3.5_error_handling.md)'s |
| dispositioned | `TE-L5` **accepted**; `CO-H1` gains its class; `TE-L2`'s list measured — **six** crates and 26 test files, not four — and handed to [T1e.4.5.2](../p1e.4_low/s1e.4.5_tests.md) |
| the closing claim | [§ What this milestone may claim at its close](../README.md#what-this-milestone-may-claim-at-its-close), drafted before the work it describes |
| not changed | the engine, and no golden |

**No golden moves, and nothing this stage added is a corpus entry.** The sweep
generates its cells into a scratch directory at test time, for the reason
[S1e.1.4](s1e.1.4_defined_behaviour_q_m1a8.md) could not add its reproducer
either: two of the twenty-one cells **panic**, and a `.ein` file under
`examples/` needs a `corpus.toml` entry, which would panic the gate.

**What the sweep is not.** It is not a fuzzer and it is not exhaustive: it is
the seven kernel primitives at four arities, the declaration heads at their
edges, and every value-taking CLI option at four values. A fuzzer over the
parser is a different instrument with a different cost, and this stage's
argument is only that a *systematic* pass over a small, enumerable surface paid
— seven cells from twenty-one, on a surface the review had reached once by
accident.
