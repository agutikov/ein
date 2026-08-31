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
| `ST-M1` | the M1 alive-set invariant — the warrant for state-key dedup — is enforced nowhere | [S1e.3.3](p1e.3_medium/s1e.3.3_state_model.md) | |
| `AR-M1` | hand-maintained parallel copies as the recurring drift mechanism — four pairs, one already bit | [S1e.3.4](p1e.3_medium/s1e.3.4_architecture.md) | **fixed 2026-08-30**, three of the four pairs here (reserved names were S1e.2.1's, the macro pipeline is `CO-M4`'s): the entering event **unified**, the gate lists **compared by a test**, the read-out **unified**. The rule is now a page section rather than a plan paragraph — [`architecture.md` § One artifact, one owner](../../docs/kernel/architecture.md), five rows with what each cost, and the **legitimate** case (`SE-L2`'s two `RESERVED` sets, where the fix is a rename) with the test for telling the two apart |
| `AR-M2` | verdict read-out ownership split across three crates | [S1e.3.4](p1e.3_medium/s1e.3.4_architecture.md) | **fixed 2026-08-30 — the seam, and it had nine copies rather than two.** `Verdict::read_out` carries the count and the qualifier; `render_solution_table` **drops** its `solution_nodes` parameter, so an arm has nothing to choose with. Beside the count, the same `match` was hand-written for *which branches are distinct models* (three, one of them documenting itself as a copy), *which states does a verdict carry* (three) and *`Aborted` falls back to the counter* (three) — nine sites, three functions. The refactor's own cost was **measured first and was zero**: the printed count equals `verdict.k` on all 230 corpus runs that render a table, before and after |
| `EH-M1` | artefact-write failures never affect the exit code | [S1e.3.5](p1e.3_medium/s1e.3.5_error_handling.md) | **reproduced** 2026-08-29 by [S1e.1.6](p1e.1_open_questions/s1e.1.6_coverage_gaps.md)'s sweep — `--events` / `--json-summary` / `--trace` print *No such file or directory* and exit **0**, on an empty path and on an unwritable one; `--dump-states` exits 1 on the second and is silent on the first. Reading-pass → binary-verified; the ruling is still S1e.3.5's |
| `EH-M2` | `$EIN_STDLIB` accepted with no validation while the checkout walk requires the marker | [S1e.3.5](p1e.3_medium/s1e.3.5_error_handling.md) | |
| `TE-M1` | the zebra2-variant byte check silently skips when python3 is absent | [S1e.3.6](p1e.3_medium/s1e.3.6_tests.md) | |
| `TE-M2` | non-vacuity floors have drifted far below the corpus they guard | [S1e.3.6](p1e.3_medium/s1e.3.6_tests.md) | |
| `TE-M3` | the "no longer slow" direction of the slow-flag check runs only nightly | [S1e.3.6](p1e.3_medium/s1e.3.6_tests.md) | |
| `TE-M4` | `no_cell_crashes` hard-codes *exit 2 = the CLI refused the argv* | [S1e.3.6](p1e.3_medium/s1e.3.6_tests.md) | |
| `TE-M5` | `expect_semantics`' or-matcher tests assert almost nothing | [S1e.3.6](p1e.3_medium/s1e.3.6_tests.md) | |
| `TE-M6` | the stdlib mutation survivor has no re-take instrument | [S1e.3.6](p1e.3_medium/s1e.3.6_tests.md) | |
| `TE-M7` | the NAF boundary's exactness machinery has no direct unit test | [S1e.3.6](p1e.3_medium/s1e.3.6_tests.md) | |
| `TE-M8` | gate = CI is enforced only by convention, and the convention already failed once | [S1e.3.6](p1e.3_medium/s1e.3.6_tests.md) | **fixed 2026-08-30 by [S1e.3.4](p1e.3_medium/s1e.3.4_architecture.md)** as `AR-M1`'s fourth pair — `ein-cli/tests/gate_steps.rs`, seven steps each side, and a second test that fails on a CI `run:` which is neither a marked gate step nor a named exception. Mutation-checked against the three drift shapes. `TE-L3` is **not** closed by it: that is a claim about the script's header, not about the two lists |
| `CD-M1` | three kernel pages attribute the two-phase loop to a nonexistent `Engine::step()` | [S1e.3.7](p1e.3_medium/s1e.3.7_code_doc_consistency.md) | |
| `CD-M2` | `events.md` misdocuments payloads and omits the emitted `traversal` event | [S1e.3.7](p1e.3_medium/s1e.3.7_code_doc_consistency.md) | its **third item is closed 2026-08-31** by [S1e.3.2](p1e.3_medium/s1e.3.2_semantics.md), which the two stages had agreed to write once. The row exists, it is covered by a fixture (`EVENT_COVER` grew an *environment* column for it — a kind reachable only under an `EIN_*` name is a kind no default-environment sweep can see), and the kind-count floor is now the row count. The other two — `watched` on `admit`, `n_guards` naming a disjunct count — stay S1e.3.7's |
| `CD-M3` | `features.md`'s own corrections were not propagated to the prose that cites them | [S1e.3.7](p1e.3_medium/s1e.3.7_code_doc_consistency.md) | |
| `CD-M4` | `docs/api/rust.md` has rotted outside its marker-guarded region | [S1e.3.7](p1e.3_medium/s1e.3.7_code_doc_consistency.md) | |
| `CD-M5` | `stdlib/README` documents `ein ir parse --resolve`, which does not exist | [S1e.3.7](p1e.3_medium/s1e.3.7_code_doc_consistency.md) | |
| `CD-M6` | `examples/README` points at the deleted Python engine; the two-encodings claim has no owner | [S1e.3.7](p1e.3_medium/s1e.3.7_code_doc_consistency.md) | |
| `CD-M7` | `utils/README` attaches the wrong reason to the 29 no-`solve` corpus entries | [S1e.3.7](p1e.3_medium/s1e.3.7_code_doc_consistency.md) | |
| `CD-M8` | `architecture_and_algorithms.md` mixes as-built and as-was vocabulary unmarked | [S1e.3.7](p1e.3_medium/s1e.3.7_code_doc_consistency.md) | |
| `DO-M1` | systemic count rot: every number no test pins has drifted at least one milestone | [S1e.3.8](p1e.3_medium/s1e.3.8_documentation.md) | |
| `DO-M2` | dangling references across the doc tree, incl. anchors that never existed | [S1e.3.8](p1e.3_medium/s1e.3.8_documentation.md) | |
| `MA-M1` | `phase_2_done` is dead scaffolding with an explicit warning-suppressor | [S1e.3.9](p1e.3_medium/s1e.3.9_maintainability.md) | |
| `MA-M2` | stale rustdoc contradicting the code it documents — two sites | [S1e.3.9](p1e.3_medium/s1e.3.9_maintainability.md) | |
| `MA-M3` | `LatticeStats.state_key_merges` is a named counter that never counts | [S1e.3.9](p1e.3_medium/s1e.3.9_maintainability.md) | |
| `MA-M4` | numeric drift across load-bearing in-code comments | [S1e.3.9](p1e.3_medium/s1e.3.9_maintainability.md) | |

### Low — 21

| id | finding | stage | disp. |
|---|---|---|---|
| `CO-L1` | Interner / FactStore `u32` arena offsets bounded by id count, not arena bytes | [S1e.4.1](p1e.4_low/s1e.4.1_correctness.md) | |
| `SE-L1` | the two entering-timeline emitters write the same event with different key orders | [S1e.4.2](p1e.4_low/s1e.4.2_semantics.md) | **fixed 2026-08-30 by [S1e.3.4](p1e.3_medium/s1e.3.4_architecture.md)** as `AR-M1`'s third pair. One `Timeline::entering`, and the order kept is the one ein.py used when it did not have to append — the divergence is a fossil of `LatticeDumper`'s `rec.update({…}) if result is not None`, a condition `EnteringInfo` cannot express. Two goldens moved, **110 of 8835 renderings**, every one by the key permutation and nothing else — proved by a control that restored only the order and made the sweep green |
| `SE-L2` | two different sets are both named `RESERVED` | [S1e.4.2](p1e.4_low/s1e.4.2_semantics.md) | |
| `ST-L1` | `EqClasses` auto-vivifies on read: a read-shaped query mutates state `fork()` copies | [S1e.4.3](p1e.4_low/s1e.4.3_state_model.md) | |
| `EH-L1` | `-n 0` is accepted while `--jobs 0` is refused with a reasoned message | [S1e.1.5](p1e.1_open_questions/s1e.1.5_cli_semantics.md) | **fixed** 2026-08-29 — refused with a message in the `jobs_spec` form, pinned by `cli_semantics::solutions_takes_a_count_of_one_or_more_and_nothing_else`; taken in the ruling stage, not S1e.4.4 |
| `EH-L2` | non-`einb` builds sniff 5 magic bytes where `is_einb` requires 8 | [S1e.4.4](p1e.4_low/s1e.4.4_error_handling.md) | |
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
