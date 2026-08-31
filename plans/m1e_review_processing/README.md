# M1e — Review processing

**Estimate:** ~11 weeks — 5 phases, 28 stages, **54 days** of stage estimates
remaining, plus two stages that have shipped. It was ~21 weeks and 113 days
until 2026-08-29, when the two phases that were **not review processing** left
for [M1f](../m1f_hypothesis_and_documentation/README.md) and took **55.5 days** with them — not one of which
closed one of the 63 findings, which is why the split cost this milestone
nothing. See the phase table's footnote.
**Status:** created 2026-08-27, out of the full-tree review taken the same day
against `master` @ `9aa598a`. The reports are carried in
[`review/`](review/summary.md), verbatim and unedited — the
[`m1d/ideas.md`](../../docs/history/m1d_satisfiability/ideas.md) precedent: a
source document a milestone processes lives inside the milestone, so a later
review cannot silently overwrite the evidence a plan cites.
**Depends on:** nothing. Every finding is stated against the shipped tree, and
the first phase reads before it changes.
**Blocks:** [M2](../m2_nl_to_ir/README.md) P2.1 in one place only — S2.1.1's
census re-reads `defined_behaviour.md`, and [CD-H3](#the-findings) said one of
its sections is false. **Cleared 2026-08-29** by
[S1e.1.4](p1e.1_open_questions/s1e.1.4_defined_behaviour_q_m1a8.md): §3.2 was
false and is rewritten. Nothing else on the critical path; the engine is green
today.

---

## What this is

A seven-reader reconstruction pass over all crates, docs, tests, stdlib,
corpus and tooling — with a full gate run beside it (**exit 0, 738 tests, 0
failures**) — produced **63 findings** in nine topics and **10 open
questions**. This milestone processes them.

It is not a bug list to burn down. The review's own § Method is explicit that
its **verification stage was aborted** by an external session limit before it
returned: three findings were reproduced against the release binary, and the
other sixty carry a reading-pass confidence label and nothing more. So the
milestone's spine is the repo's own method, applied to the review itself:

> **A finding is a claim until something holds it.** Every task ends in a
> fixture, a test, a golden, a measured number or a written decision — never
> in a diff alone. A finding that cannot be reproduced is **refuted**, and
> saying so is a result.

Four dispositions, one per finding, recorded in [the index](#the-findings):

| disposition | what it means |
|---|---|
| **fixed** | reproduced, changed, and a named test or fixture now holds the fix |
| **refuted** | the probe came back the other way; the finding is wrong, and the probe is banked so it stays wrong |
| **accepted** | real, and deliberately left as it is — with the reason written where a reader of the code will find it, not only here |
| **deferred** | real, out of scope, and handed to a named owner (a milestone, a followup, a `Q-M1e.<n>`) |

`accepted` is a first-class outcome. Several findings are of the form *this
is stated but not enforced*, and for some of them the honest answer is a
comment at the site rather than a check — but the repo's rule is that the
argument gets **written down beside the code**, which is work, not a shrug.

**Two rules govern which disposition a finding may take**, ratified 2026-08-28
and written into
[`docs/kernel/standard_of_proof.md`](../../docs/kernel/standard_of_proof.md)
rather than left in this plan:

- a **behaviour** is refuted only by an executed probe banked as a test; an
  **absence** by naming the thing that checks it; and a **risk** not by
  argument at all — only `fixed`, `accepted` with the reason at the site, or
  `deferred` to a named owner;
- an **argument suffices when its premise is itself enforced**, which is what
  separates `design/02`'s determinism argument (enforced by the ordering tests)
  from `design/08`'s *`dead` is monotone* (enforced by nothing, and broken by a
  twenty-line program four days into this milestone).

## Why the open questions come first

Not by severity. The first phase is the ten questions because **four of them
decide what a later phase's fix is**, and taking the fix first would be
guessing:

| question | what it decides |
|---|---|
| [Q3](p1e.1_open_questions/s1e.1.4_defined_behaviour_q_m1a8.md) — Q-M1a.8's true trigger | whether [CD-H3](#the-findings) is a doc correction, an engine bug, or a `not-a-bug` closure. All three are different work. **Answered 2026-08-29: all three at once** — the doc correction is taken, `Q-M1a.8` closes as stated, and the engine bug is a *different* shape, filed as [Q-M1e.16](open_questions.md#q-m1e16--the-binding-key-compares-two-register-layouts-as-one) |
| [Q5](p1e.1_open_questions/s1e.1.1_search_soundness_probes/README.md) — which side of the lookahead verdict flip is right | whether a **performance lever decides what a complete model is** — and whether the wrong verdict is golden-pinned, which turns a semantics fix into a deliberate re-bless |
| [Q4](p1e.1_open_questions/s1e.1.1_search_soundness_probes/README.md) — the inter-layer alive-∅ path | whether [CO-M1](#the-findings) is a soundness bug or an invariant nobody wrote down |
| [Q6](../m1f_hypothesis_and_documentation/p1f.10_hypothesis_structure/s1f.10.6_obligations_under_hypothesis.md) — the tree's inner-node rung flip | **the guard is ruled** (2026-08-28: the rung mode is re-read at every node); what is left is whether the flip is constructible, and what the search does with an obligation a fork derived — [Q-M1e.11](open_questions.md#q-m1e11--what-happens-to-an-obligation-derived-under-a-hypothesis) |

The rest of P1e.1 is cheaper and independent, and two of its questions
([Q1](p1e.1_open_questions/s1e.1.2_determinism_under_jobs.md),
[Q2](p1e.1_open_questions/s1e.1.3_unsat_core_completeness.md)) are about
**headline claims** — *`--jobs N` is the same computation*, *the core is the
smallest frontier over recorded derivations* — that the review could not
close either way. A headline claim with no argument is the shape this repo
treats as a defect wherever else it finds it.

[Q9](p1e.1_open_questions/s1e.1.6_coverage_gaps.md) is the one that governs
the milestone's own reading: the aborted stage left **no dedicated pass** over
algorithmic pathology, the `ein-einb/src/cast.rs` unsafe audit, parser/CLI
fuzz edges, or micro-CSP ground-truth verdicts. *Absence of findings there is
absence of evidence.* The milestone may not close claiming the tree is clean.

## Phases

| ID | title | stages | est. | ends with |
|---|---|---:|---:|---|
| [P1e.1](p1e.1_open_questions/README.md) | The ten questions | 6 | 9 d | each of Q1–Q10 answered by a probe, a measurement or a ruling — and banked as a test where the answer is a property; the four that gate a fix answered **first** |
| [P1e.2](p1e.2_high/README.md) | High — 6 findings | 3 | 12 d | no well-formed program panics the process; one reserved-name list; the tree traversal either honours its contract or refuses to ship the surfaces that lie; `docs/kernel` triaged page by page into current / bannered / history; and — not one of the 63 — a diagnostic where the search's soundness premise fails ([Q-M1e.9](open_questions.md#q-m1e9--is-dead-really-upward-closed-under-absent), ruled *B now, C filed*) |
| [P1e.3](p1e.3_medium/README.md) | Medium — 36 findings | 9 | 26 d | the k-vs-`solution_nodes` seams closed at the one place that owns them; the four parallel-copy pairs unified or diffed by a test; the gate's floors derived rather than constant; every prose count either generated or gone |
| [P1e.4](p1e.4_low/README.md) | Low — 21 findings | 8 | 7 d | the one-line class, batched by topic — each stage one commit, each finding fixed, refuted or accepted with a reason at the site |
| [P1e.5](p1e.5_documentation_and_other/README.md) † | Documentation, and other | 2 | **shipped** | the configuration reference — 17 flags, the live `EIN_*` set, the 52 CLI options, with a *does it change the answer* column and a test that fails when the flag list drifts from it; and what a **solution** is against what a **model** is, as a page rather than as a ruling in a plan file |

† **Not review processing, and since 2026-08-29 not here either.** Two phases
were added on 2026-08-28 on the user's instruction — `P1e.1b`, engine work on
hypothesis-set structure, and `P1e.5`, which writes pages that do not exist and,
after three further notes the same day, also removes a language keyword and
rebuilds the doc tree. Neither was required by any of the 63 findings. This
footnote used to say they were *"additive and may be cut whole"*; on 2026-08-29
that was taken up, and they became [M1f](../m1f_hypothesis_and_documentation/README.md)'s
[P1f.10](../m1f_hypothesis_and_documentation/p1f.10_hypothesis_structure/README.md) and
[P1f.5](../m1f_hypothesis_and_documentation/p1f.5_documentation_and_other/README.md).

**What stayed, and why.** `P1e.5`'s first two stages had already **shipped**
under M1e's numbering, and five places in the tree cite `S1e.5.1` as the stage
that delivered [`configuration.md`](../../docs/kernel/configuration.md).
Renumbering a shipped stage to match a directory it moved into would have made
all five false, so this milestone keeps the two it delivered and M1f starts at
the first stage that had not run. `P1e.1b` had run nothing, so all eight of its
stages went and were renumbered `S1f.10.<n>` from `S1e.1b.<n>`.

**M1e's acceptance is unchanged by the move.** The 63 dispositions never
depended on either phase. The one question that came to — the review's `Q6` —
was **ruled inside P1e.1** on 2026-08-28 (the rung mode is re-read at every
node), and what went to M1f is the *probe* that would let a later milestone
remove the guard, not the ruling that installs it. Applying the ruling is
[S1e.2.1](p1e.2_high/s1e.2.1_correctness.md) T3's, here.

**The question ids did not move.** `Q-M1e.5`, `.6`, `.9`, `.10` and `.11` are
this milestone's, keep their ids, and are cited from M1f across the boundary —
the repo's own rule that an id is sticky and is never reused. M1f's
[`open_questions.md`](../m1f_hypothesis_and_documentation/open_questions.md) is for what M1f raises.

**One dependency now runs the other way.** M1f's `S1f.5.20` — 23 days, the
largest single stage in either milestone — cannot start until this milestone's
`S1e.2.2`, `S1e.3.7` and `S1e.3.8` have run.

Severity is the review's, and the phase split is the user's instruction. The
one place it cuts across a finding is the **tree traversal**, whose defects
are High ([CO-H3](#the-findings)), Medium
([SE-M3](#the-findings), [CD-M2](#the-findings)) and a question
([Q6](p1e.1_open_questions/s1e.1.1_search_soundness_probes/README.md)); the phases
cross-reference rather than merge, and
[S1e.2.1](p1e.2_high/s1e.2.1_correctness.md) owns the decision the others
render.

## Reading the input

The reports are [`review/`](review/summary.md): a `summary.md`, an
`open-questions.md`, and one file per (topic × severity). Finding ids in this
milestone are **this plan's**, not the review's — the reports number nothing —
and they are `<TOPIC>-<SEVERITY><n>`, where `<n>` is the finding's ordinal
within its report file, top to bottom:

| code | topic | report |
|---|---|---|
| `CO` | correctness | [`review/correctness/`](review/correctness/high.md) |
| `SE` | semantics | [`review/semantics/`](review/semantics/medium.md) |
| `ST` | state model | [`review/state-model/`](review/state-model/medium.md) |
| `AR` | architecture | [`review/architecture/medium.md`](review/architecture/medium.md) |
| `EH` | error handling | [`review/error-handling/`](review/error-handling/medium.md) |
| `TE` | tests | [`review/tests/`](review/tests/medium.md) |
| `CD` | code ↔ doc consistency | [`review/code-doc-consistency/`](review/code-doc-consistency/high.md) |
| `DO` | documentation | [`review/documentation/`](review/documentation/medium.md) |
| `MA` | maintainability | [`review/maintainability/`](review/maintainability/medium.md) |

### Erratum in the input — three cross-references are off by one

Found while indexing, and recorded here so nobody chases the wrong question.
Three findings cite the question **after** the one they mean, which dates them
to before `open-questions.md` gained a question:

| finding | cites | means |
|---|---|---|
| [CO-M1](review/correctness/medium.md) (inter-layer alive-∅) | Q5 | **Q4** — *Can the inter-layer alive-∅ path record a false model?* |
| [CD-H3](review/code-doc-consistency/high.md) (`defined_behaviour` §3.2) | Q4 | **Q3** — *What is the true trigger shape of Q-M1a.8?* |
| [CD-M6](review/code-doc-consistency/medium.md) (examples/README) | Q10 | **Q8** — *Does anything still pin zebra.ein and zebra2.ein to the same model?* |

The reports are not edited to fix this — they are the record. The corrected
mapping is what this plan's stage files use.

### What was checked before planning

The plan does not take the reports on trust either. Spot-checked at
`9aa598a` while indexing, all confirmed:

- `RESERVED_NAMES: [&str; 8]` ([`imports.rs:49`](../../ein.rs/crates/ein-ir/src/imports.rs))
  against `RESERVED: [&str; 9]` with `open`
  ([`terms.rs:191`](../../ein.rs/crates/ein-core/src/terms.rs)) — drifted, as
  stated, and the comment predicting the unification (`imports.rs:46-48`) is
  still there.
- The `assert!(args.len() >= 2, "…ein.py raises IndexError here")` at
  [`match_.rs:777`](../../ein.rs/crates/ein-infer/src/match_.rs).
- The tree's dead arm — counters, `dumper.entering`, `continue` — with **no**
  `emit_nogood`, no writeback, no `lstate.dead` push, and no `stop_after`
  check after `record_node`
  ([`solve.rs:991-1013`](../../ein.rs/crates/ein-infer/src/solve.rs)).
- `phase_2_done` at exactly four sites, never assigned `true`, with
  `let _ = &mut phase_2_done;` at `solve.rs:1566`.
- `state_key_merges` declared, zeroed, copied into the proof, emitted to JSON
  — and incremented nowhere.
- The counts, which are **worse than the reports say** in one place: 197
  corpus entries, 23 `utils/` scripts, 56 `tests/stdlib/` programs, and
  **37** fixtures under `examples/broken/load/` where
  [`defined_behaviour.md:135`](../../docs/kernel/defined_behaviour.md) says
  *23 of the 30*.

## The findings

63 rows. `disp.` is filled in as the milestone runs — it is the ledger, and
the milestone closes when every row has one.

### High — 6

| id | finding | stage | disp. |
|---|---|---|---|
| `CO-H1` | `(eq ?x)` panics the process at match time (binary-verified) | [S1e.2.1](p1e.2_high/s1e.2.1_correctness.md) | **fixed 2026-08-29 — as the class, not the case.** Its class was swept first ([S1e.1.6](p1e.1_open_questions/s1e.1.6_coverage_gaps.md) T2: the panic is one of **seven** wrong cells, all three of them in the primitives the grammar does not shape-pin), so T1 refused all seven at compile time with a positioned `CompileError` and answered [Q-M1e.18](open_questions.md#q-m1e18--three-kernel-primitives-are-not-shape-pinned-and-drop-their-extra-arguments) with its candidate (2). Four fixtures under `examples/broken/compile/`, the matcher's `assert!` now a `debug_assert_eq!` against `Pred::arity` |
| `CO-H2` | reserved-name guard bypassed by import qualification; `(macro open …)` silently renamed (binary-verified) | [S1e.2.1](p1e.2_high/s1e.2.1_correctness.md) | **fixed 2026-08-29**, and it was wider than reported: **four** declarators, not `macro` alone, and **four** routes, not three — `:as` is a fourth and was equally broken. One list now (`ein_core::RESERVED`), the 32-cell matrix in `ein-ir`'s `reserved_names_are_reserved_through_every_import_route`, four fixtures under `examples/broken/load/`. `MA-L5`'s comment went with the duplication it explained |
| `CO-H3` | tree traversal: stop policy ignored · dead branches learn nothing · root-only rung probe | [S1e.2.1](p1e.2_high/s1e.2.1_correctness.md) | **fixed 2026-08-29, all three.** `-n` honoured; `-m` **refused** at exit 2 with a reason (6 of the 32 models sit at commitment size 6, past its default of 5, so the obvious depth-cap reading would have deleted a fifth of the headline); a dead branch **records** its refutation — a third option the stage's table did not have, and the only one that is both true and free; the rung re-read per node, per the 2026-08-28 ruling, its regression test owed to [S1f.10.6](../m1f_hypothesis_and_documentation/p1f.10_hypothesis_structure/s1f.10.6_obligations_under_hypothesis.md). Headline re-measured: **86 enterings, 32 models, fact for fact** |
| `CD-H1` | `docs/kernel` presents removed or never-built machinery as current, ≥ 6 pages | [S1e.2.2](p1e.2_high/s1e.2.2_code_doc_consistency.md) | **fixed 2026-08-30**, and the list was neither complete nor exactly right. All **40** pages triaged (37 at the review's base; three landed inside M1e) — **37 current · 3 banner · 0 move**, the zero being the rule's output since all three superseded pages are M1 P1.5b's and M1 has no `docs/history/` entry. **Four** more pages were in the same state and are not in the finding — `reserved_engine_strings.md` (a `Mode` enum with three members no crate has), `02_rules.md`, `01_entities.md`, `features.md` — every one found by resolving identifiers rather than by reading. **One item refuted**: `glossary.md` has never named a predicate registry (`git log -S` is empty). Six citations named four sections `algorithm_layer_n.md` has never had; the tree is now at **0** broken links, anchors and prose `§x.y` |
| `CD-H2` | M1d landed unevenly: live pages contradict each other on the `Open` verdict | [S1e.2.2](p1e.2_high/s1e.2.2_code_doc_consistency.md) | **fixed 2026-08-30**, five pages in one commit, converging on the wording of the three that were already right. Plus `:expect`'s third form (`none` → `(false)`), `DO-L1`'s keyword arithmetic, and **three source comments** carrying the same claims — `program.rs` still said nothing reads `obligations`, `verdict.rs` said 119 corpus entries where the census says 92 |
| `CD-H3` | `defined_behaviour.md` §3.2's "preserved bug" does not reproduce | [S1e.1.4](p1e.1_open_questions/s1e.1.4_defined_behaviour_q_m1a8.md) | **fixed** 2026-08-29 — three probes banked, §3.2 rewritten to the shape that reproduces, and the same claim corrected in **nine** other places (the README twice, five source comments, two history pages) |

### Medium — 36

| id | finding | stage | disp. |
|---|---|---|---|
| `CO-M1` | inter-layer alive-∅ records root as a model with no `has_contradiction` re-check | [S1e.3.1](p1e.3_medium/s1e.3.1_correctness.md) | **fixed 2026-08-30**, and the prescription was already corrected by [Q4](p1e.1_open_questions/s1e.1.1_search_soundness_probes/d1_q4_which_route_reaches_the_site.md) from *re-check* to **re-saturate**: on this path the `(false)` was never derived, so the missing call would have caught neither witness. `record_node` re-saturates a KB written since its last saturation and refuses one its own rules then refute — one guard for all four record sites, not the one the finding names. **Measured: 2 067 of the corpus's 2 149 recorded states are dirty (96 %)**, which refutes the dirty bit's selection as an *optimisation*, and **6** refute, all three of Q4's fixtures. Five verdict rows move and no other. The cost is **≤ 1.08 ×** because the fix that mattered was putting the dedup *before* the saturation — `branching/06 -e` calls `record_node` 1 221 times to keep 22 |
| `CO-M2` | the `Solution` arm prints `stats.solution_nodes`, which diverges from `Verdict::k` | [S1e.3.1](p1e.3_medium/s1e.3.1_correctness.md) | **fixed 2026-08-30 by [S1e.3.4](p1e.3_medium/s1e.3.4_architecture.md)**, subsumed by the seam fix as the stage predicted: the arm prints no count at all now, because the row is rendered once above the match from `Verdict::read_out`. Latent as reported — **0** of 285 corpus `solve` runs are a `Solution` with `solution_nodes ≠ 1` — so the **fixture** for the mixed regime is still S1e.3.1 T2's, and the visible regression is an `Open` entry in `ein-cli/tests/read_out.rs` |
| `CO-M3` | `Value::UNBOUND` leaks through `tag()` / `as_fact()` | [S1e.3.1](p1e.3_medium/s1e.3.1_correctness.md) | **fixed 2026-08-30**, the review's preferred option rather than its floor: `as_fact` rejects the sentinel and `tag` refuses to answer about it in a debug build, pinned by `value.rs::the_sentinel_names_no_fact` — the assertion the finding said *currently fails*. The audit is a count rather than an absence: **29** `as_fact` sites and **30** `tag` sites, and the new assertion **did not fire once** across the suite, which is the discipline confirmed rather than assumed |
| `CO-M4` | `macros.rs` carries a second, laxer macro pipeline that is not what the loader runs | [S1e.3.1](p1e.3_medium/s1e.3.1_correctness.md) | **fixed 2026-08-30**, option 1 — `AR-M1`'s fourth pair, unified. `macros::read_macros` is the one reading and `from_ir::ingest_macros` is the interning around it. The lenient reading's one argument does not survive being stated: a dump of a program with two `(macro m …)` renders the expansion of whichever came first, which is a rendering of a program that cannot be run. **7** `ir[expand]` renderings now say `<refused>`, on the `examples/broken/load/` files a load already refuses |
| `CO-M5` | `Resolver::locate` derives module identity from the display string | [S1e.3.1](p1e.3_medium/s1e.3.1_correctness.md) | **fixed 2026-08-30**, both halves the stage named: a `std.*` module may import only `std.*` modules — refused at load, in every tier, so the shape that resolves in a checkout and fails only in an installed binary cannot be written; and the **embedded** tier has its first test (`the_embedded_stdlib_is_a_resolution_tier_that_works`), which is the tier a release binary uses and the one the harness can never reach because it always sets `$EIN_STDLIB` |
| `CO-M6` | `Saturator::is_stalled` is not read-only | [S1e.3.1](p1e.3_medium/s1e.3.1_correctness.md) | **fixed 2026-08-30**, options 1 and 3: the pending delta is consumed, so asking is idempotent with respect to the queue; and the tiebreaker advance — which is not removable, being ein.py's — is stated as *the cost of asking* both at the function and in [`docs/api/rust.md`](../../docs/api/rust.md) § 3, where an embedder meets it. Nothing in the engine calls it, which is what makes it a hazard for the next caller rather than a bug for this one |
| `SE-M1` | `k` and `stats.solution_nodes` split by name; two surfaces print the wrong one | [S1e.3.2](p1e.3_medium/s1e.3.2_semantics.md) | **fixed 2026-08-30 by [S1e.3.4](p1e.3_medium/s1e.3.4_architecture.md)**, and it was **three** surfaces: `--stats` printed the label `solutions (k)` above `stats.solution_nodes`, so one `ein solve … --stats` on any of the twelve `Open` entries printed `solutions (k) 0` and `solutions (k) 1` a screen apart. `ein test -v` prints `k = {verdict}, recorded = {search}`; the `--stats` row is `solution_nodes`. **Closed 2026-08-31** by the cross-surface test S1e.3.2 owed: every `-v` header is rebuilt **from its `--json-report` row** and compared, over the whole corpus in 0.06 s — 68 checked queries, **13** of them cells where the two numbers differ |
| `SE-M2` | the `Aborted` summary breaks the no-sometimes-fields principle | [S1e.3.2](p1e.3_medium/s1e.3.2_semantics.md) | **fixed 2026-08-31 at the seam**: `build_aborted` is now `build` with an `Answer::Aborted`, so *every arm emits the same key set* holds by construction and not by inspection. It was **three** asymmetries, not the review's two — `verdict.reason` existed only on the aborted arm, which is the same rule broken in the other direction, and it is `null` everywhere else now. `the_summary_has_one_shape_on_every_arm` compares the key frame across the four verdict words and the abort; its control (`reason` pushed conditionally again) fails it |
| `SE-M3` | tree-traversal reporting semantics under-specified relative to the shipped surface | [S1e.3.2](p1e.3_medium/s1e.3.2_semantics.md) | **documented 2026-08-31** — [`events.md` § `traversal`](../../docs/kernel/inference/events.md) and [`configuration.md` § What `EIN_TRAVERSAL=tree` reports](../../docs/kernel/configuration.md), independent of `T1d.10.6.4` as the stage required. **Five** shipped facts, not four: the fifth is that `ein test` can never mark a claim `held` on a program the tree accepts, because an expectation is a claim about the *exhausted* answer and a tree reports `exhausted = false` by design — `0 held, 1 not checked`, exit 1, where the lattice holds the same claim. Also new: `enter` / `layer` / `nogood` / `writeback` are **not emitted at all** under this traversal, so the enterings are invisible in a stream that still counts them |
| `ST-M1` | the M1 alive-set invariant — the warrant for state-key dedup — is enforced nowhere | [S1e.3.3](p1e.3_medium/s1e.3.3_state_model.md) | **checked 2026-08-31, and the invariant is false.** [`ein-infer/src/invariant.rs`](../../ein.rs/crates/ein-infer/src/invariant.rs) reads the rules' `:assert` constants at load — **static**, not the post-fixpoint scan the finding asked for, because that form is free (**7 µs** on `zebra2`), total (it answers for every run the program could have) and finds every breach the scan finds. **Two corpus programs break it**, `mixed-type-hypothesis.ein` and `tests/stdlib/algebra/07_schroder.ein`, and neither pays for it — one drives the hrule rung, the other fires before `alive₀` is taken. What a breach costs is an **answer**: `examples/ein-bugs/alive-set-fresh-name.ein` says `k = 0, exhausted = true`, *No solution*, where a model exists, and its `-declared` twin is the same file plus one fact naming the invented object and answers `Solution k = 1` over that model. Reported as a `warn` event, the disposition `warn_derived_naf` already carries; the defect is [Q-M1e.21](open_questions.md#q-m1e21--a-rule-may-name-an-object-the-search-can-never-hypothesise-about) |
| `AR-M1` | hand-maintained parallel copies as the recurring drift mechanism — four pairs, one already bit | [S1e.3.4](p1e.3_medium/s1e.3.4_architecture.md) | **fixed 2026-08-30**, three of the four pairs here (reserved names were S1e.2.1's, the macro pipeline is `CO-M4`'s): the entering event **unified**, the gate lists **compared by a test**, the read-out **unified**. The rule is now a page section rather than a plan paragraph — [`architecture.md` § One artifact, one owner](../../docs/kernel/architecture.md), five rows with what each cost, and the **legitimate** case (`SE-L2`'s two `RESERVED` sets, where the fix is a rename) with the test for telling the two apart |
| `AR-M2` | verdict read-out ownership split across three crates | [S1e.3.4](p1e.3_medium/s1e.3.4_architecture.md) | **fixed 2026-08-30 — the seam, and it had nine copies rather than two.** `Verdict::read_out` carries the count and the qualifier; `render_solution_table` **drops** its `solution_nodes` parameter, so an arm has nothing to choose with. Beside the count, the same `match` was hand-written for *which branches are distinct models* (three, one of them documenting itself as a copy), *which states does a verdict carry* (three) and *`Aborted` falls back to the counter* (three) — nine sites, three functions. The refactor's own cost was **measured first and was zero**: the printed count equals `verdict.k` on all 230 corpus runs that render a table, before and after |
| `EH-M1` | artefact-write failures never affect the exit code | [S1e.3.5](p1e.3_medium/s1e.3.5_error_handling.md) | **reproduced** 2026-08-29 by [S1e.1.6](p1e.1_open_questions/s1e.1.6_coverage_gaps.md)'s sweep, **ruled 2026-08-31**: the exit code stays, and that is now a written contract with a test rather than an accident with five spellings — [`defined_behaviour.md` § 4.4](../../docs/kernel/defined_behaviour.md), `ein-cli/tests/artefact_contract.rs`. What had no defence was the *message*: four flags, four diagnostics, three of them a bare OS error on a run that may carry three artefact flags. One shape now — `error: --<flag> <path>: <os error>`. And the sweep's *empty path* half was worse than reported: `--dump-states ""` **succeeded**, dropping four artefacts into the caller's working directory, because `create_dir_all("")` is `Ok`. Empty is a usage error now, exit 2, on all five. The exit-code residue is [Q-M1e.22](open_questions.md#q-m1e22--should-a-failed-artefact-write-have-an-exit-code-of-its-own), filed with `TE-M4` |
| `EH-M2` | `$EIN_STDLIB` accepted with no validation while the checkout walk requires the marker | [S1e.3.5](p1e.3_medium/s1e.3.5_error_handling.md) | **fixed 2026-08-31**: the override must carry `MANIFEST.sha256`, the marker the walk already required, and the refusal names the variable, the path and what is missing — where a typo used to surface as *module not found at &lt;path&gt;/algebra.ein*, a true sentence naming the module rather than the cause. Asked at the first `std.*` import, so a program importing nothing from the stdlib is unaffected; `ein --version` still prints `unreadable` rather than refusing. The `current_exe()` walk is **kept with a written reason** ([`docs/install.md`](../../docs/install.md)): the stage's preferred guard — warn when the resolved manifest differs from the embedded one — fires on the *normal* state of stdlib development, which is how a warning gets turned off. All three tiers now have a test that cannot skip: `resolve` is `resolve_with(from, override)` plus one line that reads the environment |
| `TE-M1` | the zebra2-variant byte check silently skips when python3 is absent | [S1e.3.6](p1e.3_medium/s1e.3.6_tests.md) | **fixed 2026-08-31**: `require_python3()`, written the way `dot_wellformed::require_graphviz` is and carrying its sentence — *a missing gate, not a missing convenience*. Control: a `python3` on PATH that exits 127 fails the test by name. The workspace sweep for siblings found **none** — the `skipped` counters elsewhere are sweep skips under a non-vacuity floor, which is `TE-M2`'s class. The justification being deleted had `DO-M1`'s own defect in it: it argued from a test count of **566**, against today's 796 |
| `TE-M2` | non-vacuity floors have drifted far below the corpus they guard | [S1e.3.6](p1e.3_medium/s1e.3.6_tests.md) | **derived 2026-08-31**, and two of the three caps are read off the manifest. `assert_census`'s `checked >= 55` over 216 entries is now `total − (parse/load negatives + compile-negatives + 2 + 20)` = 142 against a measured 145; `every_positive_entry_answers`'s `>= 60` is now `manifest.select(positive, stdlib, include_slow).len()` — **exact**, and the same `select` the sweep itself uses, so the two cannot disagree about what is in play |
| `TE-M3` | the "no longer slow" direction of the slow-flag check runs only nightly | [S1e.3.6](p1e.3_medium/s1e.3.6_tests.md) | **fixed 2026-08-31** for the half that needs no stopwatch: `the_slow_set_is_exactly_these_two` names both entries in `ein-corpus`, per commit. The direction nothing watched is an entry *gaining* the flag — which removes it from the default sweep in silence, so every assertion about it keeps passing on a cell that no longer runs. The timing direction stays nightly, as the review agrees |
| `TE-M4` | `no_cell_crashes` hard-codes *exit 2 = the CLI refused the argv* | [S1e.3.6](p1e.3_medium/s1e.3.6_tests.md) | **fixed 2026-08-31**: the check reads the run rather than the bare code — a `2` is allowed exactly where the run names `-E` / `-T` — and `examples/zebra2.ein` declares **`solve -E 1`**, because a relaxation nothing exercises is a hole rather than a decision. With the guard removed that one cell fails, which is the control. Written into [`corpus/README.md`](../../corpus/README.md) § The run vocabulary. Its exit-code half is [Q-M1e.22](open_questions.md#q-m1e22--should-a-failed-artefact-write-have-an-exit-code-of-its-own)'s evidence |
| `TE-M5` | `expect_semantics`' or-matcher tests assert almost nothing | [S1e.3.6](p1e.3_medium/s1e.3.6_tests.md) | **fixed 2026-08-31**, from the current output rather than from the doc comment: the distinguishing phrase is *"matches a model that another expectation also claims — the 2 expectations are not distinct"*, and each test now also asserts the **other** failure's sentence is absent. Telling a *fit* problem from a *distinctness* one is the whole of what the augmenting-path search buys over greedy, and neither test could see the difference before |
| `TE-M6` | the stdlib mutation survivor has no re-take instrument | [S1e.3.6](p1e.3_medium/s1e.3.6_tests.md) | **banked 2026-08-31** as [`utils/stdlib_mutants.py`](../../utils/stdlib_mutants.py), the fifth re-takable census: four mechanical families over every `(rule …)` in `stdlib/*.ein`, **157 of 217 killed in 7 s**. Not comparable to the hand-taken 50 of 51 — that was one defect per *family* — and the useful number is the distribution: **48 of the 60 survivors are in `slots.ein`**. The recorded survivor is **dead**, killed by `tests/stdlib/slots/13_adjacent_bwd_neg_direction.ein`, which killed two siblings on the way. Why nothing had: the exchanged rule is **sound and strictly weaker**, so no contradiction fixture could catch it and only a program that *needs* the derivation can |
| `TE-M7` | the NAF boundary's exactness machinery has no direct unit test | [S1e.3.6](p1e.3_medium/s1e.3.6_tests.md) | **two tests 2026-08-31**, both controlled by disabling the epoch fast path: a parked candidate is judged **once** over six rounds that grew neither relation it watches, and a resumed fork re-judges an inherited candidate **exactly** when its watched extent grew. Read from the `park` / `admit` event lines rather than from `guard_evals`, because the claim is about a decision *not taken* and a skipped judgement is invisible in a sum |
| `TE-M8` | gate = CI is enforced only by convention, and the convention already failed once | [S1e.3.6](p1e.3_medium/s1e.3.6_tests.md) | **fixed 2026-08-30 by [S1e.3.4](p1e.3_medium/s1e.3.4_architecture.md)** as `AR-M1`'s fourth pair — `ein-cli/tests/gate_steps.rs`, seven steps each side, and a second test that fails on a CI `run:` which is neither a marked gate step nor a named exception. Mutation-checked against the three drift shapes. **S1e.3.6 added the flag's half**, which the stage asked for: `what_tests_only_skips_is_what_the_script_guards` reads `--tests-only`'s list out of the script — six steps, not the five the header claims. `TE-L3` is still P1e.4's, and is now a one-line wording fix with a check already behind it |
| `CD-M1` | three kernel pages attribute the two-phase loop to a nonexistent `Engine::step()` | [S1e.3.7](p1e.3_medium/s1e.3.7_code_doc_consistency.md) | **fixed 2026-08-31, and it is five pages rather than three** — the grep the task asked for found `docs/kernel/inference/README.md` and `docs/kernel/architecture.md` carrying the same sentence. What the symbol was: ein.py's `Engine` held a **second, queue-less loop** the `Saturator` wrapped (`ein.py/src/ein/inference/engine.py:157`, `git show 4c1a5b3^:…`), and the port kept the compile cache and dropped the loop. So the pages were not misnaming `Saturator::step` — they were describing a mirror that no longer exists, which is why *queue-less* travelled with it onto a queue-based loop. The rest of the `Engine::` family resolves: `compile_all`, `compile_for`, `check_layout` |
| `CD-M2` | `events.md` misdocuments payloads and omits the emitted `traversal` event | [S1e.3.7](p1e.3_medium/s1e.3.7_code_doc_consistency.md) | its **third item was closed 2026-08-31** by [S1e.3.2](p1e.3_medium/s1e.3.2_semantics.md), which the two stages had agreed to write once. The row exists, it is covered by a fixture (`EVENT_COVER` grew an *environment* column for it — a kind reachable only under an `EIN_*` name is a kind no default-environment sweep can see), and the kind-count floor is now the row count. **The other two closed the same day**, and the durable half found a *fourth* item the review had not: `admit` now has its own row saying it carries no `watched` and why; `compile`'s note says **three** of its six numbers are not what they are named (`n_guards` = *d*, `n_disjuncts` = *d* − 1, `n_steps` = the first disjunct's), moved out of an `engine.rs` comment; the `emitted at` column's nine `ein.py` spellings resolve; and **`warn` had no row at all** — emitted since S1e.2.3, named only in § Comparison's parity spine. What is durable is [`events_reference.rs`](../../ein.rs/crates/ein-cli/tests/events_reference.rs), the check the task priced at an hour: `.emit("…")` grepped over the two narrating crates against the three schema tables, **both** directions, 22 kinds, 0.01 s. Mutation-checked by deleting the `warn` row and by renaming `quiesce` in the page. **The gate then found the fourth thing**: `cli_semantics::every_event_kind_the_schema_defines_is_reachable_from_the_corpus` parses the same tables and requires a *corpus fixture* per kind, so the two checks are complementary — one asks *does an emitter exist*, the other *does a program reach it*. `EVENT_COVER` grew two rows, one per corpus-reachable `warn` category, and its floor went 21 → 22 |
| `CD-M3` | `features.md`'s own corrections were not propagated to the prose that cites them | [S1e.3.7](p1e.3_medium/s1e.3.7_code_doc_consistency.md) | **amended 2026-08-31, and re-measured rather than transcribed** — `zebra2 -e` with `enable-singleton-writeback false` is **3 557** enterings today and `lattice-order "score-sum"` is **101 with 64 dead** against the baseline's 67, reproducing the 2026-08-23 re-take to the digit. Four sites: the two § Per-lever notes, and `architecture_and_algorithms.md` twice — the levers paragraph (3 831/56.6× → 3 557/54.5×) and the dated 2026-08-19 profile blockquote, which keeps its reading and gains today's beside it. The second-order finding is banked **in § Two corrections itself**: *the two conclusions … are amended where they stand* was written in the past tense before the edit was made, and a correction that claims completion is harder to catch than an uncorrected number, because it answers the question a reader would otherwise go and check. That is [`DO-M1`](p1e.3_medium/s1e.3.8_documentation.md)'s rule, found by its instance |
| `CD-M4` | `docs/api/rust.md` has rotted outside its marker-guarded region | [S1e.3.7](p1e.3_medium/s1e.3.7_code_doc_consistency.md) | **fixed 2026-08-31, and the second marker is only half the fix.** The prose now names `the_other_three_verdicts_are_reachable` and says the `match` is five arms of which four have run (`Aborted` needs a budget no example sets), and it lives between `// ─── prose ───` in the test file and `<!-- prose -->` on the page, diffed by `the_page_quotes_this_files_prose_too`. But a marker makes two texts *agree*, not *true*: renaming a test and leaving the comment alone keeps page and file in perfect agreement about a name neither still has, which is exactly how this rotted. So the closing test is `the_page_and_the_file_name_the_same_tests` — every `the_*` the page prints is a `fn` there, and every `#[test] fn` there is named by the page, **both directions**, which is what would have failed on the day of the rename. The page now names all three mechanism tests, so the closure needs no exception list. Also fixed on the way: § 4 listed **three** verdicts, and `Open` is the arm an embedder mis-files, its `k` being 0 exactly as a `Contradiction`'s is |
| `CD-M5` | `stdlib/README` documents `ein ir parse --resolve`, which does not exist | [S1e.3.7](p1e.3_medium/s1e.3.7_code_doc_consistency.md) | **replaced 2026-08-31 by the route that exists, and the route is a test.** Established rather than guessed: `ein ir` (`parse`/`lint`/`dot`) and `ein kb dot` went in **one** commit — `8378ad7`, 2026-06-16, **P1.11** (`1b42cce` closes the phase two commits later) — so *both* dates were reconcilable to one, and `utils/README`'s **P1.7c** is the wrong one, a date before `01e5d65` *added* `--resolve` at P1.8. What survives is a library call with no CLI in front: `parse` → `resolve_and_minimize` → `dump_canonical`, and `the_inlining_route_the_stdlib_readme_documents_round_trips` in `ein-ir/tests/imports_semantics.rs` inlines `zebra2.ein`, re-parses it with **no** base directory, and gets the same relations, rules and facts. So the README shows a snippet the gate runs, which is what `CD-M5` was about |
| `CD-M6` | `examples/README` points at the deleted Python engine; the two-encodings claim has no owner | [S1e.3.7](p1e.3_medium/s1e.3.7_code_doc_consistency.md) | **closed 2026-08-31**, in three pieces. The two-encodings owner landed with [S1e.1.6](p1e.1_open_questions/s1e.1.6_coverage_gaps.md) (Q8) — `both_ontologies_reach_the_same_model`, named in the README's own table. *"drive them from Python"* now points at [`docs/api/rust.md`](../../docs/api/rust.md) and cites the standing *There is no Python module* statement. The **`C2` decision**, which the stage asked to be made here for `DO-M2` to cite: **git history, not `docs/history/`** — filing it there means creating `docs/history/m1_core/`, and [Q-M1e.3](open_questions.md#q-m1e3--who-owns-a-page-that-should-be-neither-fixed-nor-deleted)'s rule is that a page is moved *into* a milestone record and never made into one; M1's own disposition, in `docs/history/README.md`, is *the rest is in git history*. So the two halves still read moved to where they are read — §5(ii)'s anchoring argument (150 positive edges against 25, 900 negative pairs against 125, and the cost is the **product**) into `stdlib/README.md`, and §0's measurements were already superseded by `examples/README`'s own table — and the bare `C2` became a `git show ff1d6c5^:…` that runs. A dangling reference with no link and no location was the third state the stage forbade |
| `CD-M7` | `utils/README` attaches the wrong reason to the 29 no-`solve` corpus entries | [S1e.3.7](p1e.3_medium/s1e.3.7_code_doc_consistency.md) | **split 2026-08-31**, and the sub-counts read off `corpus.toml` rather than off the prose: **4** cost (`features/04_open` + the three `square-unique`, unbounded hypothesis space, OOM rather than a verdict) and **25** question (**17** `examples/syntax/*` node probes, 6 `square-{fwd,bwd}` per-rule demos, `features/02` and `features/05`). The 25 is the half that mattered: `corpus/README.md`'s stated rule is *a run is dropped only when it does not ask the fixture's question, never for costing too much*, and `features/05` solves in 3.0 s and is still not declared, so a README attributing 25 drops to cost was contradicting the rule with the rule's own counter-example. Fixed at the **origin** too — the guard in `openness_census.py` now carries the split, because its comment named the right four while the guard covered all 29, which is where the README's reading came from |
| `CD-M8` | `architecture_and_algorithms.md` mixes as-built and as-was vocabulary unmarked | [S1e.3.7](p1e.3_medium/s1e.3.7_code_doc_consistency.md) | **marked 2026-08-31 with a mapping table rather than a banner**, nine rows, generalising the precedent the review named (`implementation.md`'s boundary row marks `World`; this page marks nothing). The page keeps its `ein.py` register — it is the design discussion, and renaming one to match a port makes it neither — and now says so and says which name is which. **O4 was worse than reported**: `EqClasses` is called by *no engine code at all*, not merely not by `firing`, and its two callers are tests; the stub's second half is pinned by `naf_semantics::matching_does_not_resolve_equality_classes`, which is load-bearing rather than pedantic, since the boundary's extent-size stamp implies match-set equality only while the matcher ignores classes. The file lists named `saturator.rs`, `firing.rs` and `hypgen.rs` **twice each** — the review found two of the three — and `plan.rs` not at all; both are now `implementation.md`'s layer split and say they are, which is `AR-M1`'s remedy for a hand-maintained second copy |
| `DO-M1` | systemic count rot: every number no test pins has drifted at least one milestone | [S1e.3.8](p1e.3_medium/s1e.3.8_documentation.md) | **passed 2026-08-31 under a rule, not a re-count** — [Q-M1e.4](open_questions.md#q-m1e4--does-the-repo-want-an-exact-count-in-prose-at-all) answered first, as the stage insisted: a count stands in prose only if the sentence carries the **command**, the **date**, or the **test**, and which one follows from what the number *is*. **All nine drifted counts are sizes**, so most sites lost their digit and gained a one-liner (`grep -c '^\[\[entry\]\]' corpus/corpus.toml`, `wc -l …/golden/corpus_exits.txt` — the sweep's own output, one line per cell, which cannot drift from what it counts). The gate line keeps its digits and gains today's date: **802 over 90** (804 after S1e.3.9's two tests), where it read 703 over 77. Re-taken: 73 → **77** rules, 45/47/56 → **57** programs, 180/189/197 → **217** entries, 622/641/889/901 → **1 047** cells, 84 → **100** renderables, eighteen → **25** scripts, *23 of 30* → **30 of 41** `at None` fixtures. **A date does not stop drift; it makes drift legible** — `README.md:73` was the one site of the nine with a warrant and it was wrong and not misleading. The fourth shape the question offered, a checker asserting prose against a command, is **refused**: the counts that are right are the ones behind an instrument that is *run* |
| `DO-M2` | dangling references across the doc tree, incl. anchors that never existed | [S1e.3.8](p1e.3_medium/s1e.3.8_documentation.md) | **264 fixed 2026-08-31, against the review's six**, and the sweep is in the gate. The instrument existed — `doc_audit.py` checked links and anchors — and had only ever been run on `docs/kernel/`, which was already clean; pointed at the rest of the repo it found **251 in `docs/history/m1d_satisfiability/` alone**: fifteen documents lifted out of `plans/m1d_satisfiability/p1d.N_*/` on 2026-08-27 with every relative link left aimed at the tree they came from. Retargeted against anchors that exist, by **number** rather than by slug (`s1d.2.2_domains.md` is `### S1d.2.2 — The domain contract`); thirteen needed a decision, of which four point at **S1d.10.3, a stage that never shipped**, and now go to § What P1d.10 was closed without. The sweep found three defects in the checker too — it crashed on a relative argv, it read `F_r[R,S,T](x)` and the stage skeleton's `[<idea>](…<file>.md)` as links, and its prose-`§` scope could charge one link with the next link's section. `C2`'s disposition was taken in [`CD-M6`](p1e.3_medium/s1e.3.7_code_doc_consistency.md) |
| `MA-M1` | `phase_2_done` is dead scaffolding with an explicit warning-suppressor | [S1e.3.9](p1e.3_medium/s1e.3.9_maintainability.md) | **deleted 2026-08-31, and it was dead in `ein.py` too** — `False` at `solver.py:285`, tested at `:287` and `:413`, assigned nowhere, so the port reproduced dead scaffolding faithfully. What set it was `_merge_and_recheck` returning `stop`, removed at P1.7a (`8439fc9`, *"PURE PER-BRANCH search; keep root STABLE"*), which is `absent_semantics.md`'s **C1 — no root-merge**: the exit went with the merge it belonged to. The flag, both unreachable `break`s and `let _ = &mut phase_2_done;` are gone; one comment stays, because a reader was previously told there were three termination paths and there is one |
| `MA-M2` | stale rustdoc contradicting the code it documents — two sites | [S1e.3.9](p1e.3_medium/s1e.3.9_maintainability.md) | **fixed 2026-08-31**, and both had a **correct copy elsewhere**, which is `AR-M1`'s pattern in prose: `commitment.rs` said `resume` is `None` on every shipping path while `solve::resume_forks`'s own doc comment has said the opposite since S1a.6.9; `solve.rs` said `--dump-states` sets `store_lattice` where only `--trace` and `ein render lattice` do. The behaviour question the task attached to the second is answered *intended*: `--dump-states` builds a `MonotonicDumper`, which writes the state dump as the search goes and takes `Dumper`'s empty default for `proof_summary` — that file is `LatticeDumper`'s. Verified by running both flags |
| `MA-M3` | `LatticeStats.state_key_merges` is a named counter that never counts | [S1e.3.9](p1e.3_medium/s1e.3.9_maintainability.md) | **counts since 2026-08-31**, at **three** sites rather than the one the review named: in `record_node` the incoming node can lose to the stored one, lose again after re-saturation moved its key, or **replace** it — one event, three sides. Measured at the harness's `-m 3`: **15 files, 482 merges, 256 on one file**. Held by a corpus **non-vacuity floor** rather than an identity, because the number it should equal is `record_node` calls minus nodes kept and nothing counts the calls. `lattice_semantics.rs` documented the zero as deliberate port scope and now says the DAG went while the dedup it stood for did not. **The sibling sweep § Notes reserved slack for was run** — 133 entries × 243 fields — and found **none**: `naf_dropped` is the only other never-incremented counter and is documented *structurally 0*, which is exactly the distinction. 11 of 9 015 shape renderings moved, all `dump[…]`, all at unchanged byte counts |
| `MA-M4` | numeric drift across load-bearing in-code comments | [S1e.3.9](p1e.3_medium/s1e.3.9_maintainability.md) | **fixed 2026-08-31 under `DO-M1`'s rule**: `solve.rs`'s *119 of 146* cites `openness_census.md` for the 92 of 121 `verdict.rs` already carried, `summary.rs`'s *eleven* becomes the twelve `expect.rs` and `answer.rs` both said. The `zebra2-bad` pair was worse than reported — **three** snapshots across three files (`explain.rs` 123/38/1–5, `solve.rs` and `shape.rs` 126/39) and **no owner** — and measuring it showed all three wrong: **126 witnesses, union 36, frontiers 1–4**. It is a *test* rather than a date, because the pair **is** the argument for `source_frontier_core` and stops holding if the two converge |

### Low — 21

| id | finding | stage | disp. |
|---|---|---|---|
| `CO-L1` | Interner / FactStore `u32` arena offsets bounded by id count, not arena bytes | [S1e.4.1](p1e.4_low/s1e.4.1_correctness.md) | **fixed 2026-09-01 — and the sibling grep found a reachable panic.** Both arenas guarded at their one growth site; `CAPACITY`'s comment had the binding direction backwards (the *byte* bound arrives at 19–53 % of the id ceiling, measured). T1e.4.1.2's sweep found `facts.rs`'s arity `expect` — a **three-line program** takes the process down with exit 101, the shape [CO-H1](#the-findings) closed one phase earlier — now a load error. Two unit tests, [`defined_behaviour.md` § 4.5](../../docs/kernel/defined_behaviour.md), and a before/after bench showing no cost |
| `SE-L1` | the two entering-timeline emitters write the same event with different key orders | [S1e.4.2](p1e.4_low/s1e.4.2_semantics.md) | **fixed 2026-08-30 by [S1e.3.4](p1e.3_medium/s1e.3.4_architecture.md)** as `AR-M1`'s third pair. One `Timeline::entering`, and the order kept is the one ein.py used when it did not have to append — the divergence is a fossil of `LatticeDumper`'s `rec.update({…}) if result is not None`, a condition `EnteringInfo` cannot express. Two goldens moved, **110 of 8835 renderings**, every one by the key permutation and nothing else — proved by a control that restored only the order and made the sweep green |
| `SE-L2` | two different sets are both named `RESERVED` | [S1e.4.2](p1e.4_low/s1e.4.2_semantics.md) | **fixed 2026-09-01** — the **lexer's** is `LEXER_KEYWORDS` now; `ein_core::RESERVED` keeps the word the language uses for it (the loader's message, six fixture names, a page title). Five doc sites, one of which was a false *causal* claim — `03_implementation.md` said the three tables are in `ein-core` *because the lexer needs them*, and `lex.rs` reads nothing from that crate but its counters. Pinned by `lex::the_lexer_keywords_are_eleven_and_are_not_ein_cores_nine`, and `architecture.md` § The legitimate case records the rename as `AR-M1`'s mirror image |
| `ST-L1` | `EqClasses` auto-vivifies on read: a read-shaped query mutates state `fork()` copies | [S1e.4.3](p1e.4_low/s1e.4.3_state_model.md) | **fixed 2026-09-01 — and the licence to leave it alone did not exist.** `find` is a lookup now; `record` is the vivifying half and only `union` calls it. The comment route rested on [Q-M1e.2](open_questions.md#q-m1e2--may-a-review-finding-be-closed-by-a-comment)'s own worked example, and a probe refuted it: the named test unions *by hand* and asserts the **matcher** is blind, so an engine unioning on every stored fact leaves it green. `standard_of_proof.md` Rule 2 gains the second question — *does the named test enforce the premise, or something adjacent?* — and its table is now three absences of four, all three probed and all three false |
| `EH-L1` | `-n 0` is accepted while `--jobs 0` is refused with a reasoned message | [S1e.1.5](p1e.1_open_questions/s1e.1.5_cli_semantics.md) | **fixed** 2026-08-29 — refused with a message in the `jobs_spec` form, pinned by `cli_semantics::solutions_takes_a_count_of_one_or_more_and_nothing_else`; taken in the ruling stage, not S1e.4.4 |
| `EH-L2` | non-`einb` builds sniff 5 magic bytes where `is_einb` requires 8 | [S1e.4.4](p1e.4_low/s1e.4.4_error_handling.md) | **fixed 2026-09-01 — three sniffs, not two.** One `looks_like_einb` and one `EINB_MAGIC`; the third sniff, in `solve.rs`, was `#[cfg(feature = "einb")]` with **no `not` counterpart**, so a light build's `ein solve` answered a real `.einb` with `UnicodeDecodeError` instead of the refusal `Cargo.toml`'s own feature comment promises. Two more five-byte literals were in the test file, whose helper was named `ein_is_text` and returned *is einb*. Pinned by `einb_cli::the_two_magic_constants_agree` (default build) and `cli_semantics::a_file_that_starts_like_a_container_but_is_not_one_is_text_in_either_build` (both) |
| `TE-L1` | wall-clock-sensitive assertions inside the deterministic gate | [S1e.4.5](p1e.4_low/s1e.4.5_tests.md) | |
| `TE-L2` | hard-coded world anchors couple four crates' tests to two puzzle files | [S1e.4.5](p1e.4_low/s1e.4.5_tests.md) | the list is measured — **six** crates and 26 test files, not four ([S1e.1.6](p1e.1_open_questions/s1e.1.6_coverage_gaps.md) T1, handed over) |
| `TE-L3` | `run_tests.sh --tests-only` also skips the bench smoke, contradicting its header | [S1e.4.5](p1e.4_low/s1e.4.5_tests.md) | |
| `TE-L4` | `stdlib_census.py --check` is wired to no gate or workflow | [S1e.4.5](p1e.4_low/s1e.4.5_tests.md) | |
| `TE-L5` | the release workflow's cross-platform legs have never run | [S1e.1.6](p1e.1_open_questions/s1e.1.6_coverage_gaps.md) | **accepted** 2026-08-29 — the status is stated in three places and one of the three cross-references was broken, which is fixed. Not run: `publish` creates a **public GitHub release**, so a tag is the maintainer's decision, not a stage's ([Q10](#the-questions--10)) |
| `CD-L1` | the five history-page banners omit `ein test` from the CLI enumeration | [S1e.4.6](p1e.4_low/s1e.4.6_code_doc_consistency.md) | |
| `CD-L2` | guide chapter 4's transcript does not match actual output | [S1e.4.6](p1e.4_low/s1e.4.6_code_doc_consistency.md) | |
| `CD-L3` | `render_lattice`'s fallback comment states a wrong reason at the one site that triggers it | [S1e.4.6](p1e.4_low/s1e.4.6_code_doc_consistency.md) | |
| `DO-L1` | small internal defects in otherwise-normative pages | [S1e.4.7](p1e.4_low/s1e.4.7_documentation.md) | |
| `DO-L2` | frozen measurements presented as current | [S1e.4.7](p1e.4_low/s1e.4.7_documentation.md) | |
| `MA-L1` | `DEFAULT_PRIORITY`'s doc comment is arithmetically self-contradicting | [S1e.4.8](p1e.4_low/s1e.4.8_maintainability.md) | |
| `MA-L2` | a literal ~22-space run inside the non-exhausted `Contradiction` headline | [S1e.4.8](p1e.4_low/s1e.4.8_maintainability.md) | |
| `MA-L3` | `summary.rs`'s `write()` doc comment contradicts the JSON writer's tested behavior | [S1e.4.8](p1e.4_low/s1e.4.8_maintainability.md) | |
| `MA-L4` | `sanity -y` re-saturates parents with a fresh memo, polluting the live event stream | [S1e.4.8](p1e.4_low/s1e.4.8_maintainability.md) | |
| `MA-L5` | `imports.rs` predicts a refactor that never happened, above the list that then drifted | [S1e.4.8](p1e.4_low/s1e.4.8_maintainability.md) | **fixed 2026-08-29 by [S1e.2.1](p1e.2_high/s1e.2.1_correctness.md)** T2, as its own stage predicted (*"the comment goes when the duplication it explains goes"*). The list and the comment were deleted together; `qualify()`'s doc now says what `CO-H2` was instead of what P1a.3 was going to do |

### The questions — 10

| id | question | stage |
|---|---|---|
| `Q1` | Shared no-goods across concurrent workers: is the determinism argument airtight? | [S1e.1.2](p1e.1_open_questions/s1e.1.2_determinism_under_jobs.md) — **answered 2026-08-29**, acceptance (1): the argument, written at [`Nogoods`](../../ein.rs/crates/ein-core/src/kb.rs) and [design/02 §6a](../../docs/history/m1a_rust/design/02_determinism_and_order.md), with its premise **enforced** by a freeze the fan-out takes |
| `Q2` | Does `MAX_ALT_JUSTIFICATIONS = 32` ever change which unsat core is reported? | [S1e.1.3](p1e.1_open_questions/s1e.1.3_unsat_core_completeness.md) — **answered 2026-08-29: yes**, acceptance (1). The fixture pair [`alt-cap-core{,-reordered}.ein`](../../examples/ein-bugs/alt-cap-core.ein) reports a 3-fact and a 2-fact core one `:priority` apart; the fix is [Q-M1e.15](open_questions.md#q-m1e15--the-alternatives-cap-decides-which-unsat-core-is-reported), and **no shipped puzzle is changed by it** |
| `Q3` | What is the true trigger shape of Q-M1a.8, if any? | [S1e.1.4](p1e.1_open_questions/s1e.1.4_defined_behaviour_q_m1a8.md) — **answered 2026-08-29**: not the `int`, which reaches the key. A nested `Fact` collides harmlessly; an `int` **beside** a nested `Fact` in one position loses a derivation. `Q-M1a.8` closed as stated, the live half is [Q-M1e.16](open_questions.md#q-m1e16--the-binding-key-compares-two-register-layouts-as-one) |
| `Q4` | Can the inter-layer alive-∅ path record a false model? | [S1e.1.1](p1e.1_open_questions/s1e.1.1_search_soundness_probes/README.md) |
| `Q5` | Which side of the lookahead verdict flip is correct, and is the wrong one golden-pinned? | [S1e.1.1](p1e.1_open_questions/s1e.1.1_search_soundness_probes/README.md) |
| `Q6` | Is the tree traversal's inner-node rung flip actually constructible? | [S1e.1.1](p1e.1_open_questions/s1e.1.1_search_soundness_probes/README.md) |
| `Q7` | What does `-n 0` mean? | [S1e.1.5](p1e.1_open_questions/s1e.1.5_cli_semantics.md) — **answered 2026-08-29**: it meant `-n 1`, on every arm, and so did every negative. **Refused** now, exit 2; ein.py did the same and no golden pinned it, so it is a deliberate divergence |
| `Q8` | Does anything still pin `zebra.ein` and `zebra2.ein` to the same model? | [S1e.1.6](p1e.1_open_questions/s1e.1.6_coverage_gaps.md) — **answered 2026-08-29: yes, and by something stronger.** `ein-infer/tests/acceptance.rs`'s two tests pin each encoding to `GRID`, the *published* 25 cells, rather than to each other. `examples/README.md`'s pointer at the deleted Python file is replaced by their names |
| `Q9` | Was the unverified remainder of the review surface clean? | [S1e.1.6](p1e.1_open_questions/s1e.1.6_coverage_gaps.md) — **partly answered, and the answer is no.** One of the four surfaces swept: 21 cells, **seven** wrong (two panics, five silent), a rule that predicts all of them, [Q-M1e.18](open_questions.md#q-m1e18--three-kernel-primitives-are-not-shape-pinned-and-drop-their-extra-arguments) filed and `EH-M1` reproduced. The other three are scoped with owners; the milestone's closing claim is drafted [above](#what-this-milestone-may-claim-at-its-close) |
| `Q10` | The release matrix — the cross-platform legs have never executed | [S1e.1.6](p1e.1_open_questions/s1e.1.6_coverage_gaps.md) — **accepted 2026-08-29**, with the reason and the one command that changes it recorded in [`docs/install.md`](../../docs/install.md). The status was already stated in three places; `release.yml` pointed at a section that has never existed, and now points at the one that says it |

Questions this milestone *raises* get `Q-M1e.<n>` ids in
[`open_questions.md`](open_questions.md); the review's `Q1`–`Q10` keep the
review's numbering and are answered, not re-filed.

## Acceptance for the milestone

- **Every one of the 63 findings has a disposition** in the table above, and
  every `fixed` names the test that holds it, every `refuted` names the probe
  that refutes it, every `accepted` names the `file:line` where the reason is
  now written, and every `deferred` names an owner that exists.
- **Every one of Q1–Q10 is answered** — by a fixture, a measurement or a
  written ruling — and the four that gate a fix are answered before that fix
  lands.
- **No well-formed `.ein` program can panic the process.** The one known
  shape has a `broken/` fixture with a positioned diagnostic; the class is
  swept once ([S1e.1.6](p1e.1_open_questions/s1e.1.6_coverage_gaps.md) T4).
- **One list per semantics.** The four parallel-copy pairs of
  [AR-M1](#the-findings) are unified, or a test diffs them, or the reason
  they must differ is written at both sites.
- **`docs/kernel` is triaged page by page** into *current* / *superseded with
  a banner* / *moved to `docs/history/`*, with no page left in the fourth
  state the review found — describing an engine that does not exist, unmarked.
- **The gate is not weaker than it was**, and is stronger in the four places
  the review found it soft: no silent skip, floors derived from the manifest,
  the step lists diffed, and the census-shaped claims re-takable by a script.
- **`./run_tests.sh` is green** at every phase boundary, and the two counts
  the milestone changes — the test total and the corpus size — are quoted
  from a run, not from memory.

### What this milestone may claim at its close

**Drafted 2026-08-29 by
[S1e.1.6](p1e.1_open_questions/s1e.1.6_coverage_gaps.md) T1e.1.6.4, before the
work it describes was done** — because a closing claim written at the end is
written by someone who wants to be finished. Edit it at the close; do not
compose it then.

> This milestone processed a review whose **verification stage never ran**
> ([`review/summary.md`](review/summary.md) § Method). Three of its 63 findings
> were reproduced against the binary before it aborted; the other sixty carried
> a reading-pass confidence label and nothing more, and this milestone's
> dispositions are what happened when each was probed.
>
> **Four surfaces had no dedicated pass in that review**, and this milestone
> swept **one** of them:
>
> | surface | status |
> |---|---|
> | parser / CLI edges | **swept**, S1e.1.6 T2 — `ein-cli/tests/primitive_arity.rs`, 21 cells; it found `CO-H1`'s class and [Q-M1e.18](open_questions.md#q-m1e18--three-kernel-primitives-are-not-shape-pinned-and-drop-their-extra-arguments) |
> | `ein-einb/src/cast.rs`'s `unsafe` | **not swept** — owned by whoever next changes `ein-einb`, against [design/12](../../docs/history/m1a_rust/design/12_toolchain_and_layout.md) §2 |
> | micro-CSP ground truth | **not swept** — it is [M10](../m10_external_benchmarks/README.md)'s thesis and is named there, not duplicated here |
> | algorithmic complexity / pathology | **not swept, and has no owner** — [Q-M1e.19](open_questions.md#q-m1e19--algorithmic-pathology-has-no-owner) |
>
> So: the tree's cleanliness **outside** those three surfaces rests on a
> reading pass, a green gate, and the fixtures and tests this milestone added
> — and on nothing else. Absence of findings in the three is absence of
> evidence. **This milestone may not be read as saying the tree is clean**, and
> the review it processed should eventually be re-run: a thirteen-finder pass
> is a milestone's worth of compute, not a stage's, and S1e.1.6's own sweep is
> the argument that it would pay — one probe of one surface returned five
> silent wrong-answer cells, on top of the one panic the review had already
> found there.

## Risks

- **Treating the reports as ground truth.** The verification stage never ran.
  Sixty of sixty-three findings are one reader's reading, and the review says
  so. The mitigation is the milestone's spine — reproduce first — and the
  expected outcome is that some findings are **refuted**. A milestone that
  refutes none of them did not check.
- **A doc pass that re-states rather than re-checks.** [CD-H1](#the-findings)
  is the largest single item here and the temptation is to rewrite prose to
  match today's code. Half of those pages should be *bannered or moved*, not
  rewritten: a page rewritten to describe the current engine is neither
  history nor a specification — the rule
  [`docs/api/`](../../docs/api/rust.md) already lives by.
- **Fixing the tree traversal into a shape T1d.10.6.4 has not chosen.**
  [CO-H3](#the-findings) is three defects; only two are safe to fix now. What
  a tree *reports* where a lattice reports layers is an open M1d design
  question, and [S1e.2.1](p1e.2_high/s1e.2.1_correctness.md) fixes the
  contract violations without answering it — or, where it cannot, makes the
  surface refuse rather than lie. *(Outcome, 2026-08-29: all three were fixed.
  The third stopped being a risk on 2026-08-28, when the user ruled that the
  mode is re-read at every node; of the other two, one refuses — `-m` — and the
  other turned out not to need to, because the evidence the read-out was
  missing was already in hand.)*
- **Scope.** 63 findings is a milestone-sized backlog and P1e.4's 21 are
  mostly one-line. The phase is deliberately last and deliberately batched;
  if the milestone has to end early, it ends after
  [P1e.3](p1e.3_medium/README.md) with P1e.4 carried as a single followup
  issue — the one place here where dropping scope costs nothing but tidiness.
- **The engine is green and the milestone touches it.** Every stage that
  changes `ein-infer` names the goldens it may move and whether the move is
  deliberate; a re-bless that was not predicted in the stage file is a stop,
  not a step.

## Connections

- [`review/summary.md`](review/summary.md) — the system model, the method,
  and the eight most consequential findings.
- [`docs/kernel/`](../../docs/kernel/README.md) — the tree
  [CD-H1](#the-findings) and [CD-H2](#the-findings) are about, and the one
  the project declares canonical.
- [`docs/history/m1d_satisfiability/open_questions.md`](../../docs/history/m1d_satisfiability/open_questions.md)
  — `Q-M1d.1` / `Q-M1d.6` / `T1d.10.6.4`, the three M1d questions several
  findings terminate in; this milestone answers none of them and says so.
- [`docs/history/m1a_rust/oracle_ledger.md`](../../docs/history/m1a_rust/oracle_ledger.md)
  — *41 tests passing on a SKIP line nobody read*, the precedent
  [TE-M1](#the-findings) reproduces.
- [M2 P2.1](../m2_nl_to_ir/p2.1_kernel_as_instrumentation/README.md) — the
  next reader of `defined_behaviour.md`, and the reason
  [CD-H3](#the-findings) was worth closing before it starts. Closed
  2026-08-29.
