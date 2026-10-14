# M1e — Review processing

**2026-08-27 → 2026-09-01, closed 2026-10-14.** Five phases and 28 stages,
~21 weeks of stage estimates when it was planned and ~11 after two phases
left for [M1f](../../../plans/m1f_hypothesis_and_documentation/README.md). The work took
six days, and the milestone stood open, with its ledger full, for six weeks
before it was closed. **Closed**, and `plans/m1e_review_processing/` was deleted the day it
closed. Its 31 phase and stage files, 8 008 lines, are in
`git log --diff-filter=D -- plans/m1e_review_processing`, and the other 37
files moved here.

This README is the milestone, its five phases and their stages, in one file.
Beside it are the documents that are still *read*:

- [`review/`](review/summary.md) — the 2026-08-27 review's reports, verbatim
  and unedited. They are the input every disposition below answers, and a
  later review is compared against them.
- [`open_questions.md`](open_questions.md) — **Q-M1e.1 – Q-M1e.22**. Fourteen
  are still open, and § [What it handed on](#what-it-handed-on) says who owns
  each.
- [`search_soundness_probes/`](search_soundness_probes/README.md)
  — [S1e.1.1](#s1e11--two-soundness-probes-q4-and-q5)'s nine decision records
  (D1 – D9), the golden audit and the two probe scripts. They are the evidence
  behind four of the wrong answers
  [M1e2](../../../plans/m1e2_correctness/README.md) now owns, which is why they were kept
  whole rather than distilled.

## What it was for

A seven-reader reconstruction pass over every crate, page, test, stdlib
module, corpus entry and script, taken against `master` @ `9aa598a` with a full
gate run beside it (**exit 0, 738 tests, 0 failures**). It produced **63
findings** in nine topics and **10 open questions**. Its own § Method says the
**verification stage was aborted** by an external session limit before it
returned. Three findings had been reproduced against the release binary, and
the other sixty carried a reading-pass confidence label and nothing more.

So the milestone's spine was the repo's own method, applied to the review:

> **A finding is a claim until something holds it.** Every task ends in a
> fixture, a test, a golden, a measured number or a written decision, never
> in a diff alone. A finding that cannot be reproduced is **refuted**, and
> saying so is a result.

Four dispositions, one per finding: **fixed** (reproduced, changed, and a named
test holds it), **refuted** (the probe came back the other way, and the probe is
banked), **accepted** (real, deliberately left, with the reason at the site),
**deferred** (real, out of scope, with a named owner). The rules for which one a
finding may take were ratified on 2026-08-28 and live in
[`docs/kernel/standard_of_proof.md`](../../kernel/standard_of_proof.md),
not here. A **behaviour** is refuted only by a banked probe, an **absence** by
naming the thing that checks it, and a **risk** not by argument at all. An
argument holds only while its premise is enforced, and since
[S1e.4.3](#s1e43--state-model-low) a second question is attached: *does the
named test enforce the premise, or something adjacent to it?*

## What shipped

| | |
|---|---|
| **The ledger is full** | all 63 findings dispositioned and all ten questions answered. § [The findings](#the-findings) is the ledger as it closed |
| **No known panic on a well-formed program** | `CO-H1`'s class swept (21 cells, seven wrong) and refused at compile time; `CO-L1`'s three-line exit-101 program a load error. [`defined_behaviour.md` § 4.5](../../kernel/defined_behaviour.md) |
| **One list per semantics** | `AR-M1`'s four parallel-copy pairs unified or diffed by a test, and the rule written as [`architecture.md` § One artifact, one owner](../../kernel/architecture.md), with the legitimate case (`SE-L2`'s rename) beside it |
| **`docs/kernel` triaged page by page** | 40 pages: 37 current, 3 with a banner, 0 moved. The whole tree at **0** broken links, anchors and prose `§x.y`, and the link sweep is the gate's seventh static step |
| **The verdict read-out has one owner** | `Verdict::read_out`. The `k`-vs-`solution_nodes` seam was nine copies over three surfaces |
| **Every recorded state is a fixpoint of its own rules** | `record_node` re-saturates a dirty KB and refuses one its rules then refute (`CO-M1`). 2 067 of 2 149 corpus records were dirty, 6 refuted, at ≤ 1.08× |
| **The gate is not softer than CI, and its floors are derived** | `gate_steps.rs` diffs `run_tests.sh` against the workflow; the corpus floors read off the manifest; no silent skip (`require_python3`) |
| **Two censuses that ran when remembered now run nightly** | `stdlib_census.py --check` and the new [`stdlib_mutants.py`](../../../utils/stdlib_mutants.py) (157 of 217 mutants killed; 48 of 60 survivors in `slots.ein`) |
| **Pages that did not exist** | [`configuration.md`](../../kernel/configuration.md) (17 flags, 52 options, the `EIN_*` census) and [`solution_semantics.md`](../../kernel/inference/solution_semantics.md) (what a solution is, and what a model is) |
| **A rule about numbers** | [Q-M1e.4](open_questions.md#q-m1e4--does-the-repo-want-an-exact-count-in-prose-at-all): an exact count in prose carries the command that re-takes it, the date it was taken, or the test that pins it, or it is not written |

The gate went from **738 tests** at the review's base to **821 over 93
targets** at P1e.4's close (`./run_tests.sh`, 2026-09-01; re-run 2026-10-14
with the same count).

**What it did not ship is as important.** The milestone's probes found
**wrong answers** that the review never listed. They are well-formed programs
on which the engine says *No solution* where a model exists, or reports one
model where there are two. M1e's rule was to file them rather than fix them
inside a review-processing stage, so they left as questions with no owner.
They are [M1e2](../../../plans/m1e2_correctness/README.md)'s, one stage each.

## The phases

| ID | title | stages | ran | ended with |
|---|---|---:|---|---|
| [P1e.1](#p1e1--the-ten-questions) | The ten questions | 6 | 2026-08-28 → 08-29 | every question answered by a probe, a measurement or a ruling. The four that gated a fix were answered first |
| [P1e.2](#p1e2--high-six-findings) | High: 6 findings | 3 | 2026-08-29 → 08-30 | no known panic; one reserved-name list; the tree traversal honours `-n` and refuses `-m`; `docs/kernel` triaged; Q-M1e.9's diagnostic |
| [P1e.3](#p1e3--medium-36-findings) | Medium: 36 findings | 9 | 2026-08-30 → 08-31 | the seams closed at their owners, the floors derived, every prose count either warranted or gone |
| [P1e.4](#p1e4--low-21-findings) | Low: 21 findings | 8 | 2026-09-01 | twelve of the seventeen live findings larger, smaller or different than reported |
| [P1e.5](#p1e5--documentation-and-other) | Documentation, and other | 2 | 2026-08-28 | the configuration reference and the solution/model page. Four further stages left for M1f |

**Two phases left on 2026-08-29.** `P1e.1b` (the structure of the hypothesis
set) and four stages of `P1e.5` (pages ein does not have, `:priority`, the doc
tree) were added on the user's instruction on 2026-08-28. They processed no
finding, so they became [M1f](../../../plans/m1f_hypothesis_and_documentation/README.md)'s
P1f.10 and P1f.5. The question ids stayed here, because an id is sticky.
`S1e.5.1` and `S1e.5.2` had already shipped and kept their numbers, because
five places in the tree cite them.

## P1e.1 — The ten questions

First, and not by severity, because four of the ten decided what a later fix
**was**: Q3 decided whether `CD-H3` was a doc fix, an engine bug or neither; Q5
whether a performance lever decides what a model is; Q4 whether `CO-M1` was a
soundness bug; Q6 what `CO-H3`(c) needed. All ten answered by 2026-08-29.

### S1e.1.1 — Two soundness probes: Q4 and Q5

The stage that found most of what M1e2 now owns, and the one whose
records are carried whole in [`search_soundness_probes/`](search_soundness_probes/README.md).

- **The standard of proof**, ratified first as T1, with Q-M1e.1 and Q-M1e.2
  together ([D5](search_soundness_probes/d5_does_t1_ratify_q_m1e2.md)).
- **Q4: the alive-∅ path records a state its own rules refute**
  ([D1](search_soundness_probes/d1_q4_which_route_reaches_the_site.md)).
  Three witnesses banked under `examples/ein-bugs/`
  (`alive-empty-phase1`, `alive-empty-interlayer`, `complete-records-stale`).
  The prescription moved from *re-check* to **re-saturate**, which `CO-M1`
  shipped in [S1e.3.1](#s1e31--correctness-medium). The phase-1 site is not
  rare: **51 of 125** non-slow `solve` entries record root there.
- **Q5: both sides of the lookahead lever under-report.** Derived by hand
  against [Q-M1e.6](open_questions.md#q-m1e6--what-is-a-solution-and-what-is-a-model)'s
  ruling of 2026-08-28. `lattice/02` has three solutions, which the default
  finds and `-L` answers as *contradictory* with `exhausted = true`. The new pair
  `branching/15`+`16`
  ([D6](search_soundness_probes/d6_the_new_q5_fixture.md))
  has **two**, where ON answers `Solution k = 1` and OFF `Contradiction k = 0`. The
  fix is **maximality** (record a surviving commitment whose every superset
  died). It was filed as [Q-M1e.8](open_questions.md#q-m1e8--exhausted-certifies-the-lattice-not-the-model-set)
  and not taken. The [golden audit](search_soundness_probes/golden_audit.md)
  priced it: neither big golden moves.
- **Q6** left for [S1f.10.6](../../../plans/m1f_hypothesis_and_documentation/p1f.10_hypothesis_structure/s1f.10.6_obligations_under_hypothesis.md)
  ([D2](search_soundness_probes/d2_q6_which_decline_to_construct.md)).
  The ruling stayed: the tree re-reads the rung mode at every node.
- **Raised:** Q-M1e.7 (ruled **A**: the recorded object is the *state*, and the
  model a projection of it; [D9](search_soundness_probes/d9_kernel_page_overclaims.md)
  withdrew a soundness claim from a kernel page), Q-M1e.8, Q-M1e.9 (reproduced
  in [D4](search_soundness_probes/d4_q_m1e9_upward_closure.md):
  `dead` is **not** upward-closed under `absent`, ruled *B now, C filed*),
  Q-M1e.12 ([D8](search_soundness_probes/d8_branching06_untyped_models.md):
  the blind rung binds a type as an object), Q-M1e.13.

### S1e.1.2 — Determinism under `--jobs`: Q1

Acceptance (1), the argument. *Shared no-goods across concurrent workers do not
change the computation* is written at `Nogoods` in
[`kb.rs`](../../../ein.rs/crates/ein-core/src/kb.rs) and in
[design/02 §6a](../m1a_rust/design/02_determinism_and_order.md),
and its premise is **enforced** by a freeze the fan-out takes. The corpus
half of the per-layer census check is vacuous, and that went to
[Q-M1e.14](open_questions.md#q-m1e14--the-corpus---jobs-sweeps-per-layer-census-coverage-is-vacuous).

### S1e.1.3 — What the core promises: Q2

**Yes:** `MAX_ALT_JUSTIFICATIONS = 32` does change which unsat core is reported.
The fixture pair [`alt-cap-core{,-reordered}.ein`](../../../examples/ein-bugs/alt-cap-core.ein)
reports a 3-fact and a 2-fact core one `:priority` apart. No shipped puzzle
is changed by it. Filed as [Q-M1e.15](open_questions.md#q-m1e15--the-alternatives-cap-decides-which-unsat-core-is-reported),
and it satisfied M1f S1f.5.6's hard dependency.

### S1e.1.4 — Q-M1a.8's real trigger: Q3

`CD-H3` taken as all three outcomes at once. The doc correction:
[`defined_behaviour.md`](../../kernel/defined_behaviour.md) §3.2 rewritten to
the shape that reproduces, and the same claim corrected in nine other places.
`Q-M1a.8` closed as stated: the `int` reaches the key, and a nested `Fact`
collides harmlessly. And a **different** engine bug: an `int` *beside* a
nested `Fact` in one position loses a derivation in a release build and trips
a `debug_assert` in a test one, filed as
[Q-M1e.16](open_questions.md#q-m1e16--the-binding-key-compares-two-register-layouts-as-one).
This cleared [M2](../../../plans/m2_nl_to_ir/README.md) P2.1's one dependency on M1e.

### S1e.1.5 — `-n 0`: Q7

It meant `-n 1` on every arm. It is **refused** now, exit 2, a deliberate
divergence from `ein.py` that no golden pinned (`EH-L1`, taken here). Two of
the three `py_int` options still clamp a negative, and `-E`'s abort line prints
the clamped number: [Q-M1e.17](open_questions.md#q-m1e17--three-py_int-options-silently-reinterpret-a-negative).

### S1e.1.6 — What nothing pins: Q8, Q9, Q10

**Q8: yes, and by something stronger.** `acceptance.rs` pins both Zebra
encodings to `GRID`, the published 25 cells, rather than to each other. **Q9:
no.** One of the four unswept surfaces was swept (parser and CLI edges,
`primitive_arity.rs`, 21 cells) and **seven** were wrong: two panics and five
silent. It found `CO-H1`'s class,
[Q-M1e.18](open_questions.md#q-m1e18--three-kernel-primitives-are-not-shape-pinned-and-drop-their-extra-arguments)
and a reproduction of `EH-M1`. The other three surfaces are in
[§ What this milestone may claim](#what-this-milestone-may-claim-at-its-close).
**Q10** and `TE-L5` were **accepted**: the release matrix's cross-platform legs
have never run, because a tag publishes a public release and is the
maintainer's decision ([`docs/install.md`](../../install.md)). The stage also
drafted the closing claim, before the work it describes.

## P1e.2 — High: six findings

Three code defects whose failure mode was a process taken down or a guard
bypassed, and three documentation findings about the tree the project declares
canonical.

### S1e.2.1 — Correctness: the panic, the guard, the traversal

`CO-H1` fixed **as the class**: seven wrong cells, refused at compile time
with a positioned `CompileError` (Q-M1e.18 answered, candidate 2). `CO-H2` was
wider than reported: four declarators and four import routes, `:as` among them.
There is one `ein_core::RESERVED` list now, and a 32-cell matrix pins it.
`CO-H3`, all three parts: the tree honours `-n`, **refuses** `-m` (6 of
`zebra2-minus-15-obligations`' 32 models sit at commitment size 6, past `-m`'s
default), records a dead branch's refutation, and re-reads the rung mode per
node. The headline was re-measured: **86 enterings, 32 models, fact for fact**.

### S1e.2.2 — Code ↔ doc: the canonical tree

`CD-H1` and `CD-H2`. All 40 `docs/kernel` pages triaged: 37 current, 3
bannered (all M1 P1.5b's design), 0 moved. Q-M1e.3 was answered **(c)**: *a page
is moved into an existing milestone record, never made into one*. Four
more pages were in the state the finding described and not in it, every one
found by resolving identifiers with [`doc_audit.py`](../../../utils/doc_audit.py)
rather than by reading. The `Open` verdict now reads the same on five
pages. Raised [Q-M1e.20](open_questions.md#q-m1e20--two-renderers-are-produced-tested-and-unreachable).

### S1e.2.3 — Q-M1e.9's containment: a diagnostic for a refutation resting on an `absent`

Option B of Q-M1e.9: a `RefutationUnderAbsentWarning` from the same walk
`warn-derived-naf` makes, with the probe banked as
[`naf-upward-closure.ein`](../../../examples/ein-bugs/naf-upward-closure.ein). The
corpus census corrected D4's prediction. **9 rules over 7 entries** are
exposed, and `connex` is not one of them, because both its activations write
`:no-hypothesis`. The real fix is [F18](../../../plans/followups/f18_world_aware_negatives.md).
The language half went to [S1f.10.8](../../../plans/m1f_hypothesis_and_documentation/p1f.10_hypothesis_structure/s1f.10.8_refutation_under_absent.md).

## P1e.3 — Medium: 36 findings

Read as a list, a grab bag. Read as mechanisms, three, each costing more than
the findings under it. **One semantics, two hand-maintained copies** (`AR-M1`,
four pairs, one of which had already bitten). **A claim nothing runs** (every
prose count, every floor). And **the seam where a count of models meets a count
of search events** (`k` against `solution_nodes`). **Five findings were larger
than reported, all in the same direction**, because the review sampled and the sample
understated: `CD-M1` three pages → five, `CD-M2` three items → four, `MA-M3`
one site → three, `MA-M4` two snapshots → three, `DO-M2` six links → **264**.
Closed 2026-08-31 at **804 tests over 90 targets**.

### S1e.3.1 — Correctness (Medium)

`CO-M1`'s re-saturate-and-refuse in `record_node`, one guard for all four
record sites, with the dedup *before* the saturation, which is what kept the
cost at ≤ 1.08×. `CO-M3` (`as_fact` rejects the sentinel), `CO-M4` (one macro
reading), `CO-M5` (a `std.*` module imports only `std.*`, and the embedded tier's
first test), `CO-M6` (`is_stalled` idempotent on the queue). `CO-M2` was subsumed by
S1e.3.4.

### S1e.3.2 — Semantics (Medium)

`SE-M2`: one summary shape on every arm, and it was three asymmetries.
`SE-M3`: what `EIN_TRAVERSAL=tree` reports, written in
[`configuration.md`](../../kernel/configuration.md) and
[`events.md`](../../kernel/inference/events.md). Its fifth fact is the
one to remember: `ein test` can never mark a claim `held` under the tree,
because the tree never exhausts. `SE-M1`'s cross-surface test was closed here.
It re-ran [Q-M1e.5](open_questions.md#q-m1e5--is-experimental-a-licence-to-ship-a-lying-surface),
whose fourth instance is a `NOT CHECKED` line naming two causes, neither of
which applies under the tree.

### S1e.3.3 — State model (Medium)

`ST-M1` **checked, and the invariant is false**.
[`invariant.rs`](../../../ein.rs/crates/ein-infer/src/invariant.rs) reads every rule's
`:assert` constants at load (7 µs on `zebra2`) and narrates a `warn` per
breach. Two corpus programs break it harmlessly. On
[`alive-set-fresh-name.ein`](../../../examples/ein-bugs/alive-set-fresh-name.ein) a
breach costs the **answer**: `k = 0, exhausted = true` where a model exists.
That is [Q-M1e.21](open_questions.md#q-m1e21--a-rule-may-name-an-object-the-search-can-never-hypothesise-about).

### S1e.3.4 — Architecture (Medium)

`AR-M1` (three of four pairs here: the entering event and the read-out
unified, the gate lists diffed by `gate_steps.rs`) and `AR-M2`: the read-out
seam had **nine** copies, and `render_solution_table` dropped its
`solution_nodes` parameter so an arm has nothing to choose with. Closed
`CO-M2`, `SE-M1`, `SE-L1` and `TE-M8` on the way.

### S1e.3.5 — Error handling (Medium)

`EH-M1` ruled: an artefact write's failure keeps the run's exit code, as a
written contract ([`defined_behaviour.md` § 4.4](../../kernel/defined_behaviour.md),
`artefact_contract.rs`). There is one message shape, and an empty path is a usage
error. It used to write four artefacts into the cwd. `EH-M2`: `$EIN_STDLIB`
must carry the manifest. The residue is
[Q-M1e.22](open_questions.md#q-m1e22--should-a-failed-artefact-write-have-an-exit-code-of-its-own).

### S1e.3.6 — Tests (Medium)

No silent skip (`TE-M1`), floors read off the manifest (`TE-M2`), the slow set
named per commit (`TE-M3`), `no_cell_crashes` reading the run rather than the
code (`TE-M4`), the or-matcher's two failures told apart (`TE-M5`), the NAF
boundary's exactness under test (`TE-M7`). `TE-M6` became
[`stdlib_mutants.py`](../../../utils/stdlib_mutants.py), the fifth re-takable census.

### S1e.3.7 — Code ↔ doc consistency (Medium)

Eight findings in eight commits. The durable half is the checks:
`events_reference.rs` (emitters against the schema, both directions, which found
`warn` had no row), `the_page_and_the_file_name_the_same_tests` (a marker
makes two texts agree, not true), and the inlining route the stdlib README
documents, as a test. `CD-M6`'s `C2` decision went to **git history**, by
Q-M1e.3's rule.

### S1e.3.8 — Documentation (Medium)

Q-M1e.4 answered first, then `DO-M1`'s pass, which was mostly a *de*-counting
one. All nine drifted counts were sizes, and most sites lost their digit and
gained the command. `DO-M2`: **264** dangling links fixed, 251 of them in
`docs/history/m1d_satisfiability/`, and the whole-tree sweep joined the gate.

### S1e.3.9 — Maintainability (Medium)

`phase_2_done` deleted (dead in `ein.py` too, since P1.7a removed the merge it
belonged to). Two rustdoc comments corrected against their correct copies
elsewhere. `state_key_merges` counts, at three sites (482 merges over 15 files
at `-m 3`). In-code numbers re-warranted. The sibling sweep over 133 entries ×
243 fields found **none**, which is the point.

## P1e.4 — Low: 21 findings

Planned as ~1.5 weeks of one-line fixes, droppable into a single followup.
**Twelve of the seventeen live findings were larger, smaller or different than
reported**, in both directions. `CO-L1`'s unreachable wrap sat next to a
**reachable process panic**. `ST-L1`'s cited test did not enforce its premise.
`CD-L2`'s transcript was **fabricated**, not drifted. `TE-L3`'s and `MA-L4`'s
recommendations were **refuted by measurement**. `MA-L2` shrank. **The phase's
mechanism, found three times:** a claim that names a test is harder to check
than one that names nothing, because the citation is what stops anybody
looking. Seventeen new checks, which is the whole of the gate's 804 → 821.
Had the fallback been taken, every one of those defects would still be there.

### S1e.4.1 — Correctness (Low)

`CO-L1`: both arenas guarded at their growth site, the byte bound shown to bind
first (at 19–53 % of the id ceiling), and the sibling grep's reachable arity
`expect` made a load error ([`defined_behaviour.md` § 4.5](../../kernel/defined_behaviour.md)).

### S1e.4.2 — Semantics (Low)

`SE-L2`: the lexer's set is `LEXER_KEYWORDS`, and `ein_core::RESERVED` keeps the
language's word. It is `AR-M1`'s mirror image, recorded as the legitimate case.

### S1e.4.3 — State model (Low)

`ST-L1`: `EqClasses::find` is a lookup now. The comment route's licence did not
exist, and that added Rule 2's second question to
[`standard_of_proof.md`](../../kernel/standard_of_proof.md).

### S1e.4.4 — Error handling (Low)

`EH-L2`: one `looks_like_einb`, one `EINB_MAGIC`. The light build's missing arm
answered a real `.einb` with `UnicodeDecodeError`.

### S1e.4.5 — Tests (Low)

`TE-L1` (one wall-clock band asserted only under `EIN_CORPUS_SLOW`, and `-T`
refused in a `runs` column), `TE-L2` (the world-anchor list generated, 28
files), `TE-L3` (the header amended, the recommendation refuted), `TE-L4` (two
nightly steps, and a `--check` that could not fail now can).

### S1e.4.6 — Code ↔ doc consistency (Low)

`CD-L1` (six banner sites, held by `api_banner.rs`), `CD-L2` (the guide's
transcript generated by `guide_transcripts.rs`), `CD-L3` (one
`FULL_VIEW_FALLBACK`, a 337-digest re-bless).

### S1e.4.7 — Documentation (Low)

`DO-L1` (three of four. The table's sentence claimed a tenth case nothing
tested, and it has a test now) and `DO-L2` (one site stale, and one misattributed
to a deleted engine).

### S1e.4.8 — Maintainability (Low)

`MA-L1` (the 900 band exists, and the default is pinned by a sandwich), `MA-L2`
(31 digests), `MA-L3` (**accepted**: both comments were wrong, differently,
and settleable from git), `MA-L4` (the memo shared, zero events removed, and
`-y`'s real cost is up to `2k+1`, not `k+1`).

## P1e.5 — Documentation, and other

Added 2026-08-28 on the user's instruction: *"we need a comprehensive config
options reference document in `docs/kernel/`."* Two stages shipped the same
day. The other four (`S1e.5.3` proposed, `.5.5`, `.5.6`, `.5.20`) left for M1f
on 2026-08-29.

### S1e.5.1 — The configuration reference

[`docs/kernel/configuration.md`](../../kernel/configuration.md), pinned by
`config_reference.rs`, whose defaults block **is** `ein solve --dump-config`'s
output. Five of the stage's own reconnaissance numbers were wrong. There are
**52** CLI options across eleven parsers, not 50. **Six** options shadow five
flags. There are **29** greppable `EIN_*` names, of which 9 are not variables at all.
**`enable-symmetric-mirror` changes the answer** (`features.md`'s `1.0×` is a
claim about `zebra2`). And **two flags are inert**, not probes, which is
[Q-M1e.10](open_questions.md#q-m1e10--two-config--flags-are-inert).

### S1e.5.2 — What a solution is, and what a model is

[`solution_semantics.md`](../../kernel/inference/solution_semantics.md),
taken first and out of order because Q5 needed a definition to derive
against. It states the vocabulary, the operational form (*a maximal alive
commitment*), the worked `lattice/02`, and § 6 on where `complete()` differs from
the definition. Ten new glossary entries.

## The findings

63 rows, carried as they stood at the close. `disp.` was filled in as the
milestone ran. It is the ledger, and the milestone closed when every row had
one.

### High — 6

| id | finding | stage | disp. |
|---|---|---|---|
| `CO-H1` | `(eq ?x)` panics the process at match time (binary-verified) | [S1e.2.1](#s1e21--correctness-the-panic-the-guard-the-traversal) | **fixed 2026-08-29 — as the class, not the case.** Its class was swept first ([S1e.1.6](#s1e16--what-nothing-pins-q8-q9-q10) T2: the panic is one of **seven** wrong cells, all three of them in the primitives the grammar does not shape-pin), so T1 refused all seven at compile time with a positioned `CompileError` and answered [Q-M1e.18](open_questions.md#q-m1e18--three-kernel-primitives-are-not-shape-pinned-and-drop-their-extra-arguments) with its candidate (2). Four fixtures under `examples/broken/compile/`, the matcher's `assert!` now a `debug_assert_eq!` against `Pred::arity` |
| `CO-H2` | reserved-name guard bypassed by import qualification; `(macro open …)` silently renamed (binary-verified) | [S1e.2.1](#s1e21--correctness-the-panic-the-guard-the-traversal) | **fixed 2026-08-29**, and it was wider than reported: **four** declarators, not `macro` alone, and **four** routes, not three — `:as` is a fourth and was equally broken. One list now (`ein_core::RESERVED`), the 32-cell matrix in `ein-ir`'s `reserved_names_are_reserved_through_every_import_route`, four fixtures under `examples/broken/load/`. `MA-L5`'s comment went with the duplication it explained |
| `CO-H3` | tree traversal: stop policy ignored · dead branches learn nothing · root-only rung probe | [S1e.2.1](#s1e21--correctness-the-panic-the-guard-the-traversal) | **fixed 2026-08-29, all three.** `-n` honoured; `-m` **refused** at exit 2 with a reason (6 of the 32 models sit at commitment size 6, past its default of 5, so the obvious depth-cap reading would have deleted a fifth of the headline); a dead branch **records** its refutation — a third option the stage's table did not have, and the only one that is both true and free; the rung re-read per node, per the 2026-08-28 ruling, its regression test owed to [S1f.10.6](../../../plans/m1f_hypothesis_and_documentation/p1f.10_hypothesis_structure/s1f.10.6_obligations_under_hypothesis.md). Headline re-measured: **86 enterings, 32 models, fact for fact** |
| `CD-H1` | `docs/kernel` presents removed or never-built machinery as current, ≥ 6 pages | [S1e.2.2](#s1e22--code--doc-the-canonical-tree) | **fixed 2026-08-30**, and the list was neither complete nor exactly right. All **40** pages triaged (37 at the review's base; three landed inside M1e) — **37 current · 3 banner · 0 move**, the zero being the rule's output since all three superseded pages are M1 P1.5b's and M1 has no `docs/history/` entry. **Four** more pages were in the same state and are not in the finding — `reserved_engine_strings.md` (a `Mode` enum with three members no crate has), `02_rules.md`, `01_entities.md`, `features.md` — every one found by resolving identifiers rather than by reading. **One item refuted**: `glossary.md` has never named a predicate registry (`git log -S` is empty). Six citations named four sections `algorithm_layer_n.md` has never had; the tree is now at **0** broken links, anchors and prose `§x.y` |
| `CD-H2` | M1d landed unevenly: live pages contradict each other on the `Open` verdict | [S1e.2.2](#s1e22--code--doc-the-canonical-tree) | **fixed 2026-08-30**, five pages in one commit, converging on the wording of the three that were already right. Plus `:expect`'s third form (`none` → `(false)`), `DO-L1`'s keyword arithmetic, and **three source comments** carrying the same claims — `program.rs` still said nothing reads `obligations`, `verdict.rs` said 119 corpus entries where the census says 92 |
| `CD-H3` | `defined_behaviour.md` §3.2's "preserved bug" does not reproduce | [S1e.1.4](#s1e14--q-m1a8s-real-trigger-q3) | **fixed** 2026-08-29 — three probes banked, §3.2 rewritten to the shape that reproduces, and the same claim corrected in **nine** other places (the README twice, five source comments, two history pages) |

### Medium — 36

| id | finding | stage | disp. |
|---|---|---|---|
| `CO-M1` | inter-layer alive-∅ records root as a model with no `has_contradiction` re-check | [S1e.3.1](#s1e31--correctness-medium) | **fixed 2026-08-30**, and the prescription was already corrected by [Q4](search_soundness_probes/d1_q4_which_route_reaches_the_site.md) from *re-check* to **re-saturate**: on this path the `(false)` was never derived, so the missing call would have caught neither witness. `record_node` re-saturates a KB written since its last saturation and refuses one its own rules then refute — one guard for all four record sites, not the one the finding names. **Measured: 2 067 of the corpus's 2 149 recorded states are dirty (96 %)**, which refutes the dirty bit's selection as an *optimisation*, and **6** refute, all three of Q4's fixtures. Five verdict rows move and no other. The cost is **≤ 1.08 ×** because the fix that mattered was putting the dedup *before* the saturation — `branching/06 -e` calls `record_node` 1 221 times to keep 22 |
| `CO-M2` | the `Solution` arm prints `stats.solution_nodes`, which diverges from `Verdict::k` | [S1e.3.1](#s1e31--correctness-medium) | **fixed 2026-08-30 by [S1e.3.4](#s1e34--architecture-medium)**, subsumed by the seam fix as the stage predicted: the arm prints no count at all now, because the row is rendered once above the match from `Verdict::read_out`. Latent as reported — **0** of 285 corpus `solve` runs are a `Solution` with `solution_nodes ≠ 1` — so the **fixture** for the mixed regime is still S1e.3.1 T2's, and the visible regression is an `Open` entry in `ein-cli/tests/read_out.rs` |
| `CO-M3` | `Value::UNBOUND` leaks through `tag()` / `as_fact()` | [S1e.3.1](#s1e31--correctness-medium) | **fixed 2026-08-30**, the review's preferred option rather than its floor: `as_fact` rejects the sentinel and `tag` refuses to answer about it in a debug build, pinned by `value.rs::the_sentinel_names_no_fact` — the assertion the finding said *currently fails*. The audit is a count rather than an absence: **29** `as_fact` sites and **30** `tag` sites, and the new assertion **did not fire once** across the suite, which is the discipline confirmed rather than assumed |
| `CO-M4` | `macros.rs` carries a second, laxer macro pipeline that is not what the loader runs | [S1e.3.1](#s1e31--correctness-medium) | **fixed 2026-08-30**, option 1 — `AR-M1`'s fourth pair, unified. `macros::read_macros` is the one reading and `from_ir::ingest_macros` is the interning around it. The lenient reading's one argument does not survive being stated: a dump of a program with two `(macro m …)` renders the expansion of whichever came first, which is a rendering of a program that cannot be run. **7** `ir[expand]` renderings now say `<refused>`, on the `examples/broken/load/` files a load already refuses |
| `CO-M5` | `Resolver::locate` derives module identity from the display string | [S1e.3.1](#s1e31--correctness-medium) | **fixed 2026-08-30**, both halves the stage named: a `std.*` module may import only `std.*` modules — refused at load, in every tier, so the shape that resolves in a checkout and fails only in an installed binary cannot be written; and the **embedded** tier has its first test (`the_embedded_stdlib_is_a_resolution_tier_that_works`), which is the tier a release binary uses and the one the harness can never reach because it always sets `$EIN_STDLIB` |
| `CO-M6` | `Saturator::is_stalled` is not read-only | [S1e.3.1](#s1e31--correctness-medium) | **fixed 2026-08-30**, options 1 and 3: the pending delta is consumed, so asking is idempotent with respect to the queue; and the tiebreaker advance — which is not removable, being ein.py's — is stated as *the cost of asking* both at the function and in [`docs/api/rust.md`](../../api/rust.md) § 3, where an embedder meets it. Nothing in the engine calls it, which is what makes it a hazard for the next caller rather than a bug for this one |
| `SE-M1` | `k` and `stats.solution_nodes` split by name; two surfaces print the wrong one | [S1e.3.2](#s1e32--semantics-medium) | **fixed 2026-08-30 by [S1e.3.4](#s1e34--architecture-medium)**, and it was **three** surfaces: `--stats` printed the label `solutions (k)` above `stats.solution_nodes`, so one `ein solve … --stats` on any of the twelve `Open` entries printed `solutions (k) 0` and `solutions (k) 1` a screen apart. `ein test -v` prints `k = {verdict}, recorded = {search}`; the `--stats` row is `solution_nodes`. **Closed 2026-08-31** by the cross-surface test S1e.3.2 owed: every `-v` header is rebuilt **from its `--json-report` row** and compared, over the whole corpus in 0.06 s — 68 checked queries, **13** of them cells where the two numbers differ |
| `SE-M2` | the `Aborted` summary breaks the no-sometimes-fields principle | [S1e.3.2](#s1e32--semantics-medium) | **fixed 2026-08-31 at the seam**: `build_aborted` is now `build` with an `Answer::Aborted`, so *every arm emits the same key set* holds by construction and not by inspection. It was **three** asymmetries, not the review's two — `verdict.reason` existed only on the aborted arm, which is the same rule broken in the other direction, and it is `null` everywhere else now. `the_summary_has_one_shape_on_every_arm` compares the key frame across the four verdict words and the abort; its control (`reason` pushed conditionally again) fails it |
| `SE-M3` | tree-traversal reporting semantics under-specified relative to the shipped surface | [S1e.3.2](#s1e32--semantics-medium) | **documented 2026-08-31** — [`events.md` § `traversal`](../../kernel/inference/events.md) and [`configuration.md` § What `EIN_TRAVERSAL=tree` reports](../../kernel/configuration.md), independent of `T1d.10.6.4` as the stage required. **Five** shipped facts, not four: the fifth is that `ein test` can never mark a claim `held` on a program the tree accepts, because an expectation is a claim about the *exhausted* answer and a tree reports `exhausted = false` by design — `0 held, 1 not checked`, exit 1, where the lattice holds the same claim. Also new: `enter` / `layer` / `nogood` / `writeback` are **not emitted at all** under this traversal, so the enterings are invisible in a stream that still counts them |
| `ST-M1` | the M1 alive-set invariant — the warrant for state-key dedup — is enforced nowhere | [S1e.3.3](#s1e33--state-model-medium) | **checked 2026-08-31, and the invariant is false.** [`ein-infer/src/invariant.rs`](../../../ein.rs/crates/ein-infer/src/invariant.rs) reads the rules' `:assert` constants at load — **static**, not the post-fixpoint scan the finding asked for, because that form is free (**7 µs** on `zebra2`), total (it answers for every run the program could have) and finds every breach the scan finds. **Two corpus programs break it**, `mixed-type-hypothesis.ein` and `tests/stdlib/algebra/07_schroder.ein`, and neither pays for it — one drives the hrule rung, the other fires before `alive₀` is taken. What a breach costs is an **answer**: `examples/ein-bugs/alive-set-fresh-name.ein` says `k = 0, exhausted = true`, *No solution*, where a model exists, and its `-declared` twin is the same file plus one fact naming the invented object and answers `Solution k = 1` over that model. Reported as a `warn` event, the disposition `warn_derived_naf` already carries; the defect is [Q-M1e.21](open_questions.md#q-m1e21--a-rule-may-name-an-object-the-search-can-never-hypothesise-about) |
| `AR-M1` | hand-maintained parallel copies as the recurring drift mechanism — four pairs, one already bit | [S1e.3.4](#s1e34--architecture-medium) | **fixed 2026-08-30**, three of the four pairs here (reserved names were S1e.2.1's, the macro pipeline is `CO-M4`'s): the entering event **unified**, the gate lists **compared by a test**, the read-out **unified**. The rule is now a page section rather than a plan paragraph — [`architecture.md` § One artifact, one owner](../../kernel/architecture.md), five rows with what each cost, and the **legitimate** case (`SE-L2`'s two `RESERVED` sets, where the fix is a rename) with the test for telling the two apart |
| `AR-M2` | verdict read-out ownership split across three crates | [S1e.3.4](#s1e34--architecture-medium) | **fixed 2026-08-30 — the seam, and it had nine copies rather than two.** `Verdict::read_out` carries the count and the qualifier; `render_solution_table` **drops** its `solution_nodes` parameter, so an arm has nothing to choose with. Beside the count, the same `match` was hand-written for *which branches are distinct models* (three, one of them documenting itself as a copy), *which states does a verdict carry* (three) and *`Aborted` falls back to the counter* (three) — nine sites, three functions. The refactor's own cost was **measured first and was zero**: the printed count equals `verdict.k` on all 230 corpus runs that render a table, before and after |
| `EH-M1` | artefact-write failures never affect the exit code | [S1e.3.5](#s1e35--error-handling-medium) | **reproduced** 2026-08-29 by [S1e.1.6](#s1e16--what-nothing-pins-q8-q9-q10)'s sweep, **ruled 2026-08-31**: the exit code stays, and that is now a written contract with a test rather than an accident with five spellings — [`defined_behaviour.md` § 4.4](../../kernel/defined_behaviour.md), `ein-cli/tests/artefact_contract.rs`. What had no defence was the *message*: four flags, four diagnostics, three of them a bare OS error on a run that may carry three artefact flags. One shape now — `error: --<flag> <path>: <os error>`. And the sweep's *empty path* half was worse than reported: `--dump-states ""` **succeeded**, dropping four artefacts into the caller's working directory, because `create_dir_all("")` is `Ok`. Empty is a usage error now, exit 2, on all five. The exit-code residue is [Q-M1e.22](open_questions.md#q-m1e22--should-a-failed-artefact-write-have-an-exit-code-of-its-own), filed with `TE-M4` |
| `EH-M2` | `$EIN_STDLIB` accepted with no validation while the checkout walk requires the marker | [S1e.3.5](#s1e35--error-handling-medium) | **fixed 2026-08-31**: the override must carry `MANIFEST.sha256`, the marker the walk already required, and the refusal names the variable, the path and what is missing — where a typo used to surface as *module not found at &lt;path&gt;/algebra.ein*, a true sentence naming the module rather than the cause. Asked at the first `std.*` import, so a program importing nothing from the stdlib is unaffected; `ein --version` still prints `unreadable` rather than refusing. The `current_exe()` walk is **kept with a written reason** ([`docs/install.md`](../../install.md)): the stage's preferred guard — warn when the resolved manifest differs from the embedded one — fires on the *normal* state of stdlib development, which is how a warning gets turned off. All three tiers now have a test that cannot skip: `resolve` is `resolve_with(from, override)` plus one line that reads the environment |
| `TE-M1` | the zebra2-variant byte check silently skips when python3 is absent | [S1e.3.6](#s1e36--tests-medium) | **fixed 2026-08-31**: `require_python3()`, written the way `dot_wellformed::require_graphviz` is and carrying its sentence — *a missing gate, not a missing convenience*. Control: a `python3` on PATH that exits 127 fails the test by name. The workspace sweep for siblings found **none** — the `skipped` counters elsewhere are sweep skips under a non-vacuity floor, which is `TE-M2`'s class. The justification being deleted had `DO-M1`'s own defect in it: it argued from a test count of **566**, against today's 796 |
| `TE-M2` | non-vacuity floors have drifted far below the corpus they guard | [S1e.3.6](#s1e36--tests-medium) | **derived 2026-08-31**, and two of the three caps are read off the manifest. `assert_census`'s `checked >= 55` over 216 entries is now `total − (parse/load negatives + compile-negatives + 2 + 20)` = 142 against a measured 145; `every_positive_entry_answers`'s `>= 60` is now `manifest.select(positive, stdlib, include_slow).len()` — **exact**, and the same `select` the sweep itself uses, so the two cannot disagree about what is in play |
| `TE-M3` | the "no longer slow" direction of the slow-flag check runs only nightly | [S1e.3.6](#s1e36--tests-medium) | **fixed 2026-08-31** for the half that needs no stopwatch: `the_slow_set_is_exactly_these_two` names both entries in `ein-corpus`, per commit. The direction nothing watched is an entry *gaining* the flag — which removes it from the default sweep in silence, so every assertion about it keeps passing on a cell that no longer runs. The timing direction stays nightly, as the review agrees |
| `TE-M4` | `no_cell_crashes` hard-codes *exit 2 = the CLI refused the argv* | [S1e.3.6](#s1e36--tests-medium) | **fixed 2026-08-31**: the check reads the run rather than the bare code — a `2` is allowed exactly where the run names `-E` / `-T` — and `examples/zebra2.ein` declares **`solve -E 1`**, because a relaxation nothing exercises is a hole rather than a decision. With the guard removed that one cell fails, which is the control. Written into [`corpus/README.md`](../../../corpus/README.md) § The run vocabulary. Its exit-code half is [Q-M1e.22](open_questions.md#q-m1e22--should-a-failed-artefact-write-have-an-exit-code-of-its-own)'s evidence |
| `TE-M5` | `expect_semantics`' or-matcher tests assert almost nothing | [S1e.3.6](#s1e36--tests-medium) | **fixed 2026-08-31**, from the current output rather than from the doc comment: the distinguishing phrase is *"matches a model that another expectation also claims — the 2 expectations are not distinct"*, and each test now also asserts the **other** failure's sentence is absent. Telling a *fit* problem from a *distinctness* one is the whole of what the augmenting-path search buys over greedy, and neither test could see the difference before |
| `TE-M6` | the stdlib mutation survivor has no re-take instrument | [S1e.3.6](#s1e36--tests-medium) | **banked 2026-08-31** as [`utils/stdlib_mutants.py`](../../../utils/stdlib_mutants.py), the fifth re-takable census: four mechanical families over every `(rule …)` in `stdlib/*.ein`, **157 of 217 killed in 7 s**. Not comparable to the hand-taken 50 of 51 — that was one defect per *family* — and the useful number is the distribution: **48 of the 60 survivors are in `slots.ein`**. The recorded survivor is **dead**, killed by `tests/stdlib/slots/13_adjacent_bwd_neg_direction.ein`, which killed two siblings on the way. Why nothing had: the exchanged rule is **sound and strictly weaker**, so no contradiction fixture could catch it and only a program that *needs* the derivation can |
| `TE-M7` | the NAF boundary's exactness machinery has no direct unit test | [S1e.3.6](#s1e36--tests-medium) | **two tests 2026-08-31**, both controlled by disabling the epoch fast path: a parked candidate is judged **once** over six rounds that grew neither relation it watches, and a resumed fork re-judges an inherited candidate **exactly** when its watched extent grew. Read from the `park` / `admit` event lines rather than from `guard_evals`, because the claim is about a decision *not taken* and a skipped judgement is invisible in a sum |
| `TE-M8` | gate = CI is enforced only by convention, and the convention already failed once | [S1e.3.6](#s1e36--tests-medium) | **fixed 2026-08-30 by [S1e.3.4](#s1e34--architecture-medium)** as `AR-M1`'s fourth pair — `ein-cli/tests/gate_steps.rs`, seven steps each side, and a second test that fails on a CI `run:` which is neither a marked gate step nor a named exception. Mutation-checked against the three drift shapes. **S1e.3.6 added the flag's half**, which the stage asked for: `what_tests_only_skips_is_what_the_script_guards` reads `--tests-only`'s list out of the script — six steps, not the five the header claims. `TE-L3` is still P1e.4's, and is now a one-line wording fix with a check already behind it |
| `CD-M1` | three kernel pages attribute the two-phase loop to a nonexistent `Engine::step()` | [S1e.3.7](#s1e37--code--doc-consistency-medium) | **fixed 2026-08-31, and it is five pages rather than three** — the grep the task asked for found `docs/kernel/inference/README.md` and `docs/kernel/architecture.md` carrying the same sentence. What the symbol was: ein.py's `Engine` held a **second, queue-less loop** the `Saturator` wrapped (`ein.py/src/ein/inference/engine.py:157`, `git show 4c1a5b3^:…`), and the port kept the compile cache and dropped the loop. So the pages were not misnaming `Saturator::step` — they were describing a mirror that no longer exists, which is why *queue-less* travelled with it onto a queue-based loop. The rest of the `Engine::` family resolves: `compile_all`, `compile_for`, `check_layout` |
| `CD-M2` | `events.md` misdocuments payloads and omits the emitted `traversal` event | [S1e.3.7](#s1e37--code--doc-consistency-medium) | its **third item was closed 2026-08-31** by [S1e.3.2](#s1e32--semantics-medium), which the two stages had agreed to write once. The row exists, it is covered by a fixture (`EVENT_COVER` grew an *environment* column for it — a kind reachable only under an `EIN_*` name is a kind no default-environment sweep can see), and the kind-count floor is now the row count. **The other two closed the same day**, and the durable half found a *fourth* item the review had not: `admit` now has its own row saying it carries no `watched` and why; `compile`'s note says **three** of its six numbers are not what they are named (`n_guards` = *d*, `n_disjuncts` = *d* − 1, `n_steps` = the first disjunct's), moved out of an `engine.rs` comment; the `emitted at` column's nine `ein.py` spellings resolve; and **`warn` had no row at all** — emitted since S1e.2.3, named only in § Comparison's parity spine. What is durable is [`events_reference.rs`](../../../ein.rs/crates/ein-cli/tests/events_reference.rs), the check the task priced at an hour: `.emit("…")` grepped over the two narrating crates against the three schema tables, **both** directions, 22 kinds, 0.01 s. Mutation-checked by deleting the `warn` row and by renaming `quiesce` in the page. **The gate then found the fourth thing**: `cli_semantics::every_event_kind_the_schema_defines_is_reachable_from_the_corpus` parses the same tables and requires a *corpus fixture* per kind, so the two checks are complementary — one asks *does an emitter exist*, the other *does a program reach it*. `EVENT_COVER` grew two rows, one per corpus-reachable `warn` category, and its floor went 21 → 22 |
| `CD-M3` | `features.md`'s own corrections were not propagated to the prose that cites them | [S1e.3.7](#s1e37--code--doc-consistency-medium) | **amended 2026-08-31, and re-measured rather than transcribed** — `zebra2 -e` with `enable-singleton-writeback false` is **3 557** enterings today and `lattice-order "score-sum"` is **101 with 64 dead** against the baseline's 67, reproducing the 2026-08-23 re-take to the digit. Four sites: the two § Per-lever notes, and `architecture_and_algorithms.md` twice — the levers paragraph (3 831/56.6× → 3 557/54.5×) and the dated 2026-08-19 profile blockquote, which keeps its reading and gains today's beside it. The second-order finding is banked **in § Two corrections itself**: *the two conclusions … are amended where they stand* was written in the past tense before the edit was made, and a correction that claims completion is harder to catch than an uncorrected number, because it answers the question a reader would otherwise go and check. That is [`DO-M1`](#s1e38--documentation-medium)'s rule, found by its instance |
| `CD-M4` | `docs/api/rust.md` has rotted outside its marker-guarded region | [S1e.3.7](#s1e37--code--doc-consistency-medium) | **fixed 2026-08-31, and the second marker is only half the fix.** The prose now names `the_other_three_verdicts_are_reachable` and says the `match` is five arms of which four have run (`Aborted` needs a budget no example sets), and it lives between `// ─── prose ───` in the test file and `<!-- prose -->` on the page, diffed by `the_page_quotes_this_files_prose_too`. But a marker makes two texts *agree*, not *true*: renaming a test and leaving the comment alone keeps page and file in perfect agreement about a name neither still has, which is exactly how this rotted. So the closing test is `the_page_and_the_file_name_the_same_tests` — every `the_*` the page prints is a `fn` there, and every `#[test] fn` there is named by the page, **both directions**, which is what would have failed on the day of the rename. The page now names all three mechanism tests, so the closure needs no exception list. Also fixed on the way: § 4 listed **three** verdicts, and `Open` is the arm an embedder mis-files, its `k` being 0 exactly as a `Contradiction`'s is |
| `CD-M5` | `stdlib/README` documents `ein ir parse --resolve`, which does not exist | [S1e.3.7](#s1e37--code--doc-consistency-medium) | **replaced 2026-08-31 by the route that exists, and the route is a test.** Established rather than guessed: `ein ir` (`parse`/`lint`/`dot`) and `ein kb dot` went in **one** commit — `8378ad7`, 2026-06-16, **P1.11** (`1b42cce` closes the phase two commits later) — so *both* dates were reconcilable to one, and `utils/README`'s **P1.7c** is the wrong one, a date before `01e5d65` *added* `--resolve` at P1.8. What survives is a library call with no CLI in front: `parse` → `resolve_and_minimize` → `dump_canonical`, and `the_inlining_route_the_stdlib_readme_documents_round_trips` in `ein-ir/tests/imports_semantics.rs` inlines `zebra2.ein`, re-parses it with **no** base directory, and gets the same relations, rules and facts. So the README shows a snippet the gate runs, which is what `CD-M5` was about |
| `CD-M6` | `examples/README` points at the deleted Python engine; the two-encodings claim has no owner | [S1e.3.7](#s1e37--code--doc-consistency-medium) | **closed 2026-08-31**, in three pieces. The two-encodings owner landed with [S1e.1.6](#s1e16--what-nothing-pins-q8-q9-q10) (Q8) — `both_ontologies_reach_the_same_model`, named in the README's own table. *"drive them from Python"* now points at [`docs/api/rust.md`](../../api/rust.md) and cites the standing *There is no Python module* statement. The **`C2` decision**, which the stage asked to be made here for `DO-M2` to cite: **git history, not `docs/history/`** — filing it there means creating `docs/history/m1_core/`, and [Q-M1e.3](open_questions.md#q-m1e3--who-owns-a-page-that-should-be-neither-fixed-nor-deleted)'s rule is that a page is moved *into* a milestone record and never made into one; M1's own disposition, in `docs/history/README.md`, is *the rest is in git history*. So the two halves still read moved to where they are read — §5(ii)'s anchoring argument (150 positive edges against 25, 900 negative pairs against 125, and the cost is the **product**) into `stdlib/README.md`, and §0's measurements were already superseded by `examples/README`'s own table — and the bare `C2` became a `git show ff1d6c5^:…` that runs. A dangling reference with no link and no location was the third state the stage forbade |
| `CD-M7` | `utils/README` attaches the wrong reason to the 29 no-`solve` corpus entries | [S1e.3.7](#s1e37--code--doc-consistency-medium) | **split 2026-08-31**, and the sub-counts read off `corpus.toml` rather than off the prose: **4** cost (`features/04_open` + the three `square-unique`, unbounded hypothesis space, OOM rather than a verdict) and **25** question (**17** `examples/syntax/*` node probes, 6 `square-{fwd,bwd}` per-rule demos, `features/02` and `features/05`). The 25 is the half that mattered: `corpus/README.md`'s stated rule is *a run is dropped only when it does not ask the fixture's question, never for costing too much*, and `features/05` solves in 3.0 s and is still not declared, so a README attributing 25 drops to cost was contradicting the rule with the rule's own counter-example. Fixed at the **origin** too — the guard in `openness_census.py` now carries the split, because its comment named the right four while the guard covered all 29, which is where the README's reading came from |
| `CD-M8` | `architecture_and_algorithms.md` mixes as-built and as-was vocabulary unmarked | [S1e.3.7](#s1e37--code--doc-consistency-medium) | **marked 2026-08-31 with a mapping table rather than a banner**, nine rows, generalising the precedent the review named (`implementation.md`'s boundary row marks `World`; this page marks nothing). The page keeps its `ein.py` register — it is the design discussion, and renaming one to match a port makes it neither — and now says so and says which name is which. **O4 was worse than reported**: `EqClasses` is called by *no engine code at all*, not merely not by `firing`, and its two callers are tests; the stub's second half is pinned by `naf_semantics::matching_does_not_resolve_equality_classes`, which is load-bearing rather than pedantic, since the boundary's extent-size stamp implies match-set equality only while the matcher ignores classes. The file lists named `saturator.rs`, `firing.rs` and `hypgen.rs` **twice each** — the review found two of the three — and `plan.rs` not at all; both are now `implementation.md`'s layer split and say they are, which is `AR-M1`'s remedy for a hand-maintained second copy |
| `DO-M1` | systemic count rot: every number no test pins has drifted at least one milestone | [S1e.3.8](#s1e38--documentation-medium) | **passed 2026-08-31 under a rule, not a re-count** — [Q-M1e.4](open_questions.md#q-m1e4--does-the-repo-want-an-exact-count-in-prose-at-all) answered first, as the stage insisted: a count stands in prose only if the sentence carries the **command**, the **date**, or the **test**, and which one follows from what the number *is*. **All nine drifted counts are sizes**, so most sites lost their digit and gained a one-liner (`grep -c '^\[\[entry\]\]' corpus/corpus.toml`, `wc -l …/golden/corpus_exits.txt` — the sweep's own output, one line per cell, which cannot drift from what it counts). The gate line keeps its digits and gains today's date: **802 over 90** (804 after S1e.3.9's two tests), where it read 703 over 77. Re-taken: 73 → **77** rules, 45/47/56 → **57** programs, 180/189/197 → **217** entries, 622/641/889/901 → **1 047** cells, 84 → **100** renderables, eighteen → **25** scripts, *23 of 30* → **30 of 41** `at None` fixtures. **A date does not stop drift; it makes drift legible** — `README.md:73` was the one site of the nine with a warrant and it was wrong and not misleading. The fourth shape the question offered, a checker asserting prose against a command, is **refused**: the counts that are right are the ones behind an instrument that is *run* |
| `DO-M2` | dangling references across the doc tree, incl. anchors that never existed | [S1e.3.8](#s1e38--documentation-medium) | **264 fixed 2026-08-31, against the review's six**, and the sweep is in the gate. The instrument existed — `doc_audit.py` checked links and anchors — and had only ever been run on `docs/kernel/`, which was already clean; pointed at the rest of the repo it found **251 in `docs/history/m1d_satisfiability/` alone**: fifteen documents lifted out of `plans/m1d_satisfiability/p1d.N_*/` on 2026-08-27 with every relative link left aimed at the tree they came from. Retargeted against anchors that exist, by **number** rather than by slug (`s1d.2.2_domains.md` is `### S1d.2.2 — The domain contract`); thirteen needed a decision, of which four point at **S1d.10.3, a stage that never shipped**, and now go to § What P1d.10 was closed without. The sweep found three defects in the checker too — it crashed on a relative argv, it read `F_r[R,S,T](x)` and the stage skeleton's `[<idea>](…<file>.md)` as links, and its prose-`§` scope could charge one link with the next link's section. `C2`'s disposition was taken in [`CD-M6`](#s1e37--code--doc-consistency-medium) |
| `MA-M1` | `phase_2_done` is dead scaffolding with an explicit warning-suppressor | [S1e.3.9](#s1e39--maintainability-medium) | **deleted 2026-08-31, and it was dead in `ein.py` too** — `False` at `solver.py:285`, tested at `:287` and `:413`, assigned nowhere, so the port reproduced dead scaffolding faithfully. What set it was `_merge_and_recheck` returning `stop`, removed at P1.7a (`8439fc9`, *"PURE PER-BRANCH search; keep root STABLE"*), which is `absent_semantics.md`'s **C1 — no root-merge**: the exit went with the merge it belonged to. The flag, both unreachable `break`s and `let _ = &mut phase_2_done;` are gone; one comment stays, because a reader was previously told there were three termination paths and there is one |
| `MA-M2` | stale rustdoc contradicting the code it documents — two sites | [S1e.3.9](#s1e39--maintainability-medium) | **fixed 2026-08-31**, and both had a **correct copy elsewhere**, which is `AR-M1`'s pattern in prose: `commitment.rs` said `resume` is `None` on every shipping path while `solve::resume_forks`'s own doc comment has said the opposite since S1a.6.9; `solve.rs` said `--dump-states` sets `store_lattice` where only `--trace` and `ein render lattice` do. The behaviour question the task attached to the second is answered *intended*: `--dump-states` builds a `MonotonicDumper`, which writes the state dump as the search goes and takes `Dumper`'s empty default for `proof_summary` — that file is `LatticeDumper`'s. Verified by running both flags |
| `MA-M3` | `LatticeStats.state_key_merges` is a named counter that never counts | [S1e.3.9](#s1e39--maintainability-medium) | **counts since 2026-08-31**, at **three** sites rather than the one the review named: in `record_node` the incoming node can lose to the stored one, lose again after re-saturation moved its key, or **replace** it — one event, three sides. Measured at the harness's `-m 3`: **15 files, 482 merges, 256 on one file**. Held by a corpus **non-vacuity floor** rather than an identity, because the number it should equal is `record_node` calls minus nodes kept and nothing counts the calls. `lattice_semantics.rs` documented the zero as deliberate port scope and now says the DAG went while the dedup it stood for did not. **The sibling sweep § Notes reserved slack for was run** — 133 entries × 243 fields — and found **none**: `naf_dropped` is the only other never-incremented counter and is documented *structurally 0*, which is exactly the distinction. 11 of 9 015 shape renderings moved, all `dump[…]`, all at unchanged byte counts |
| `MA-M4` | numeric drift across load-bearing in-code comments | [S1e.3.9](#s1e39--maintainability-medium) | **fixed 2026-08-31 under `DO-M1`'s rule**: `solve.rs`'s *119 of 146* cites `openness_census.md` for the 92 of 121 `verdict.rs` already carried, `summary.rs`'s *eleven* becomes the twelve `expect.rs` and `answer.rs` both said. The `zebra2-bad` pair was worse than reported — **three** snapshots across three files (`explain.rs` 123/38/1–5, `solve.rs` and `shape.rs` 126/39) and **no owner** — and measuring it showed all three wrong: **126 witnesses, union 36, frontiers 1–4**. It is a *test* rather than a date, because the pair **is** the argument for `source_frontier_core` and stops holding if the two converge |

### Low — 21

| id | finding | stage | disp. |
|---|---|---|---|
| `CO-L1` | Interner / FactStore `u32` arena offsets bounded by id count, not arena bytes | [S1e.4.1](#s1e41--correctness-low) | **fixed 2026-09-01 — and the sibling grep found a reachable panic.** Both arenas guarded at their one growth site; `CAPACITY`'s comment had the binding direction backwards (the *byte* bound arrives at 19–53 % of the id ceiling, measured). T1e.4.1.2's sweep found `facts.rs`'s arity `expect` — a **three-line program** takes the process down with exit 101, the shape [CO-H1](#the-findings) closed one phase earlier — now a load error. Two unit tests, [`defined_behaviour.md` § 4.5](../../kernel/defined_behaviour.md), and a before/after bench showing no cost |
| `SE-L1` | the two entering-timeline emitters write the same event with different key orders | [S1e.4.2](#s1e42--semantics-low) | **fixed 2026-08-30 by [S1e.3.4](#s1e34--architecture-medium)** as `AR-M1`'s third pair. One `Timeline::entering`, and the order kept is the one ein.py used when it did not have to append — the divergence is a fossil of `LatticeDumper`'s `rec.update({…}) if result is not None`, a condition `EnteringInfo` cannot express. Two goldens moved, **110 of 8835 renderings**, every one by the key permutation and nothing else — proved by a control that restored only the order and made the sweep green |
| `SE-L2` | two different sets are both named `RESERVED` | [S1e.4.2](#s1e42--semantics-low) | **fixed 2026-09-01** — the **lexer's** is `LEXER_KEYWORDS` now; `ein_core::RESERVED` keeps the word the language uses for it (the loader's message, six fixture names, a page title). Five doc sites, one of which was a false *causal* claim — `03_implementation.md` said the three tables are in `ein-core` *because the lexer needs them*, and `lex.rs` reads nothing from that crate but its counters. Pinned by `lex::the_lexer_keywords_are_eleven_and_are_not_ein_cores_nine`, and `architecture.md` § The legitimate case records the rename as `AR-M1`'s mirror image |
| `ST-L1` | `EqClasses` auto-vivifies on read: a read-shaped query mutates state `fork()` copies | [S1e.4.3](#s1e43--state-model-low) | **fixed 2026-09-01 — and the licence to leave it alone did not exist.** `find` is a lookup now; `record` is the vivifying half and only `union` calls it. The comment route rested on [Q-M1e.2](open_questions.md#q-m1e2--may-a-review-finding-be-closed-by-a-comment)'s own worked example, and a probe refuted it: the named test unions *by hand* and asserts the **matcher** is blind, so an engine unioning on every stored fact leaves it green. `standard_of_proof.md` Rule 2 gains the second question — *does the named test enforce the premise, or something adjacent?* — and its table is now three absences of four, all three probed and all three false |
| `EH-L1` | `-n 0` is accepted while `--jobs 0` is refused with a reasoned message | [S1e.1.5](#s1e15---n-0-q7) | **fixed** 2026-08-29 — refused with a message in the `jobs_spec` form, pinned by `cli_semantics::solutions_takes_a_count_of_one_or_more_and_nothing_else`; taken in the ruling stage, not S1e.4.4 |
| `EH-L2` | non-`einb` builds sniff 5 magic bytes where `is_einb` requires 8 | [S1e.4.4](#s1e44--error-handling-low) | **fixed 2026-09-01 — three sniffs, not two.** One `looks_like_einb` and one `EINB_MAGIC`; the third sniff, in `solve.rs`, was `#[cfg(feature = "einb")]` with **no `not` counterpart**, so a light build's `ein solve` answered a real `.einb` with `UnicodeDecodeError` instead of the refusal `Cargo.toml`'s own feature comment promises. Two more five-byte literals were in the test file, whose helper was named `ein_is_text` and returned *is einb*. Pinned by `einb_cli::the_two_magic_constants_agree` (default build) and `cli_semantics::a_file_that_starts_like_a_container_but_is_not_one_is_text_in_either_build` (both) |
| `TE-L1` | wall-clock-sensitive assertions inside the deterministic gate | [S1e.4.5](#s1e45--tests-low) | **fixed 2026-09-01 — and the finding leads with the safe one.** Measured: the 20 s ceiling has a ~1 300× margin and cannot flake; the 4× `cost_ms` band trips at **2.9×**. That one is asserted under `EIN_CORPUS_SLOW` (nightly, `--release`, the profile it was measured on) and *reported* per commit; the flag-threshold directions stay. A **third** site the finding calls non-existent is `einb/roundtrip.rs`. And the worst hazard is not an assertion: `-T` in a `runs` column would put a stopwatch in `corpus_exits.txt`, now refused by `manifest::no_declared_run_budgets_by_wall_clock` |
| `TE-L2` | hard-coded world anchors couple four crates' tests to two puzzle files | [S1e.4.5](#s1e45--tests-low) | the list is measured — **six** crates and 26 test files, not four ([S1e.1.6](#s1e16--what-nothing-pins-q8-q9-q10) T1, handed over). **fixed 2026-09-01, and generated rather than written**: it was **28** two days later, both additions from this milestone's own commits, and whatever goes in `zebra2.ein` is copied into four generated files. The registry is [`examples/README.md` § What an edit to these two files fans out into](../../../examples/README.md), its list a marked block `world_anchors::the_anchor_list_is_the_greps_own_answer` diffs, and both puzzles point at it. Ring 4 is the sentence it exists for: **three** `from_ein_py/` goldens, and an edit that changes the parse spends them |
| `TE-L3` | `run_tests.sh --tests-only` also skips the bench smoke, contradicting its header | [S1e.4.5](#s1e45--tests-low) | **fixed 2026-09-01 by amending the header — the finding's own recommendation is refuted.** Running the bench under the flag cannot buy the property it is argued for (still a subset by six steps), restores half a pair (clippy type-checks the bench and the flag skips it), and costs **5.3 s** of release codegen against the **1.6 s** the flag saves. The header meanwhile contradicted *itself* — *five* at `:5`, *six* at `:91` — and so did `gate_steps.rs`'s own message. `what_tests_only_skips_is_what_the_script_guards` holds the header to the set it parses now |
| `TE-L4` | `stdlib_census.py --check` is wired to no gate or workflow | [S1e.4.5](#s1e45--tests-low) | **fixed 2026-09-01 — two steps in the nightly that already existed**, and `stdlib_mutants.py --check` with it: the stage's premise *"only this one has a `--check` already written"* was falsified by S1e.3.6, one phase earlier in this milestone. `--check` also **could not fail** for the case it is most exposed to — a declared run exiting 0 and narrating nothing was printed and never returned. Re-taken: 77 of 77, 217 entries, 38 s, where three shipped sites said 180 and 37 s — and the warrant two of them cite for stating a count exactly named a **test that does not exist** |
| `TE-L5` | the release workflow's cross-platform legs have never run | [S1e.1.6](#s1e16--what-nothing-pins-q8-q9-q10) | **accepted** 2026-08-29 — the status is stated in three places and one of the three cross-references was broken, which is fixed. Not run: `publish` creates a **public GitHub release**, so a tag is the maintainer's decision, not a stage's ([Q10](#the-questions--10)) |
| `CD-L1` | the five history-page banners omit `ein test` from the CLI enumeration | [S1e.4.6](#s1e46--code--doc-consistency-low) | **fixed 2026-09-01 — six sites, not five.** `README.md`'s crate table carried the same enumeration, written the same day and stale for the same reason. Every other sentence of the banner checks out. Marked with `<!-- api-history-banner -->` and held by `api_banner.rs`: the five regions are one string, and every `COMMAND ein <name>` in `golden/help_shape.txt` is named in it — the banner cites the surface's owner instead of becoming a sixth list |
| `CD-L2` | guide chapter 4's transcript does not match actual output | [S1e.4.6](#s1e46--code--doc-consistency-low) | **fixed 2026-09-01 — and it was fabricated, not drifted.** Two hunks (the bindings *and* a missing rule line), and `git show b4d5158` adds that text on 2026-06-17 while that day's engine already printed one binding per line under a rule. No engine in this repo's history printed it — so a pin was not enough and the block is **generated**, by `guide_transcripts.rs` from the command in its marker. Chapter 2's three blocks stay hand-maintained (they are *excerpts*, and a diff cannot express an elision), which `docs/guide/README.md` now says — replacing a sentence false of half the guide |
| `CD-L3` | `render_lattice`'s fallback comment states a wrong reason at the one site that triggers it | [S1e.4.6](#s1e46--code--doc-consistency-low) | **fixed 2026-09-01 — the sentence is false twice, and a *third* surface disagreed.** Every caller that reaches it sets `store_lattice = true`, and `LatticeProof` has no `kb_index` for any value of the flag to fill; `04_dot_rendering.md` said `full` *needs* the flag. The phase's **second** named re-bless (it expected one): **337 of 9 015 digests plus one text line, and not one line count changed**. One exported `FULL_VIEW_FALLBACK`, and two tests — one per crate — make *agrees with the help text* checkable rather than inspected |
| `DO-L1` | small internal defects in otherwise-normative pages | [S1e.4.7](#s1e47--documentation-low) | **fixed 2026-09-01 — three of four, and one was already done.** (b) fell to T1e.2.2.3 on 2026-08-30. (a) is larger: *nine* over a ten-row table, **and** the sentence under it claimed all ten were pinned by a test that covered **nine** — the tenth refusal had no case anywhere; it does now. (c) is four DOT blocks on three pages drawing a relation as `Rule`'s hexagon, and the renderer settles it **negatively** — no shipped view nodes a relation at all, and the normative legend calls it a dashed edge |
| `DO-L2` | frozen measurements presented as current | [S1e.4.7](#s1e47--documentation-low) | **fixed 2026-09-01, and the review's *mechanism* is refuted** — S1d.2.4's activator facts moved neither site. `examples/README.md` is 3–9 % stale (44.1 / 28.2 ms re-taken) so its digits keep their date and gain the re-take; its *12 rules* is a size and was 14, so it becomes the command. The guide's `≥23×, 3336+, does not finish` is **misattributed**, not stale: `ein.py`/PyPy numbers cut by a 90 s budget, on an engine that left the tree — today 54× and 3 557, and it finishes in 1.5 s. And `features.md`'s own footnote said 3 557 was the search *run to the end*; it is where the default depth cap stops it |
| `MA-L1` | `DEFAULT_PRIORITY`'s doc comment is arithmetically self-contradicting | [S1e.4.8](#s1e48--maintainability-low) | **fixed 2026-09-01**, and the review's sub-claim **refuted**: the 900 band *does* exist (three `hypothesis-contradiction` rules; only the stdlib stops at 500), the comment is a verbatim port of ein.py's already-wrong one, and `obligations::priority_of` is a second consumer nobody had written down. The value is not merely unpinned but **unobservable** — no program mixes a banded rule with an unbanded one — so the pin **sandwiches** it between 999 and 1001. M1f S1f.5.6's claim to this finding is corrected rather than left standing |
| `MA-L2` | a literal ~22-space run inside the non-exhausted `Contradiction` headline | [S1e.4.8](#s1e48--maintainability-low) | **fixed 2026-09-01, and it is *smaller* than reported** — `ein-cli` never calls `render_answer`, so it reaches no CLI output at all; it reaches 31 `trace[answer]` digests and the `ein-render` public API. The pinning gap is the finding's real content and is exactly as reported: **nothing asserted either `Contradiction` headline**. The re-bless was predicted and then measured — 31 of 9 015, one op, no line count changed. And the sibling sweep is **not** empty: a 14-space run in `saturator.rs`'s assertion message |
| `MA-L3` | `summary.rs`'s `write()` doc comment contradicts the JSON writer's tested behavior | [S1e.4.8](#s1e48--maintainability-low) | **accepted 2026-09-01 — and the question *is* settleable, from git rather than from the goldens.** `4c1a5b3^:_summary.py:208` passed `ensure_ascii=False`, so the review's *one of the two is wrong* is false: **both** are, differently, and only one was reported — `summary.rs` is right about ein.py and wrong about its own function; `json.rs`'s *no caller overrides it* is wrong about ein.py, where **two** did. The port implements one of them, so the same run emits `\u2014` in the summary and an em dash in `--events`. The escaping stays, both comments are corrected, and the premise is pinned |
| `MA-L4` | `sanity -y` re-saturates parents with a fresh memo, polluting the live event stream | [S1e.4.8](#s1e48--maintainability-low) | **split 2026-09-01: memo fixed, cause refuted, and a third thing found.** Sharing the memo is right and free (66.9 → 63.1 ms on `zebra2 -e -y`) but removes **zero** events — `compile` fires on an *engine* miss, never a memo miss, which `engine.rs` says at the site. What `-y` really adds is **four** kinds (`compile` `enqueue` `fire` `quiesce`), inherent to narrating the extra saturations, so it is documented where `-y` is. And the cost every document states — `k+1` — is ein.py's docstring's error: it is `1 + k + alive parents`, up to **2k+1** |
| `MA-L5` | `imports.rs` predicts a refactor that never happened, above the list that then drifted | [S1e.4.8](#s1e48--maintainability-low) | **fixed 2026-08-29 by [S1e.2.1](#s1e21--correctness-the-panic-the-guard-the-traversal)** T2, as its own stage predicted (*"the comment goes when the duplication it explains goes"*). The list and the comment were deleted together; `qualify()`'s doc now says what `CO-H2` was instead of what P1a.3 was going to do |

### The questions — 10

| id | question | stage |
|---|---|---|
| `Q1` | Shared no-goods across concurrent workers: is the determinism argument airtight? | [S1e.1.2](#s1e12--determinism-under---jobs-q1) — **answered 2026-08-29**, acceptance (1): the argument, written at [`Nogoods`](../../../ein.rs/crates/ein-core/src/kb.rs) and [design/02 §6a](../m1a_rust/design/02_determinism_and_order.md), with its premise **enforced** by a freeze the fan-out takes |
| `Q2` | Does `MAX_ALT_JUSTIFICATIONS = 32` ever change which unsat core is reported? | [S1e.1.3](#s1e13--what-the-core-promises-q2) — **answered 2026-08-29: yes**, acceptance (1). The fixture pair [`alt-cap-core{,-reordered}.ein`](../../../examples/ein-bugs/alt-cap-core.ein) reports a 3-fact and a 2-fact core one `:priority` apart; the fix is [Q-M1e.15](open_questions.md#q-m1e15--the-alternatives-cap-decides-which-unsat-core-is-reported), and **no shipped puzzle is changed by it** |
| `Q3` | What is the true trigger shape of Q-M1a.8, if any? | [S1e.1.4](#s1e14--q-m1a8s-real-trigger-q3) — **answered 2026-08-29**: not the `int`, which reaches the key. A nested `Fact` collides harmlessly; an `int` **beside** a nested `Fact` in one position loses a derivation. `Q-M1a.8` closed as stated, the live half is [Q-M1e.16](open_questions.md#q-m1e16--the-binding-key-compares-two-register-layouts-as-one) |
| `Q4` | Can the inter-layer alive-∅ path record a false model? | [S1e.1.1](search_soundness_probes/README.md) |
| `Q5` | Which side of the lookahead verdict flip is correct, and is the wrong one golden-pinned? | [S1e.1.1](search_soundness_probes/README.md) |
| `Q6` | Is the tree traversal's inner-node rung flip actually constructible? | [S1e.1.1](search_soundness_probes/README.md) |
| `Q7` | What does `-n 0` mean? | [S1e.1.5](#s1e15---n-0-q7) — **answered 2026-08-29**: it meant `-n 1`, on every arm, and so did every negative. **Refused** now, exit 2; ein.py did the same and no golden pinned it, so it is a deliberate divergence |
| `Q8` | Does anything still pin `zebra.ein` and `zebra2.ein` to the same model? | [S1e.1.6](#s1e16--what-nothing-pins-q8-q9-q10) — **answered 2026-08-29: yes, and by something stronger.** `ein-infer/tests/acceptance.rs`'s two tests pin each encoding to `GRID`, the *published* 25 cells, rather than to each other. `examples/README.md`'s pointer at the deleted Python file is replaced by their names |
| `Q9` | Was the unverified remainder of the review surface clean? | [S1e.1.6](#s1e16--what-nothing-pins-q8-q9-q10) — **partly answered, and the answer is no.** One of the four surfaces swept: 21 cells, **seven** wrong (two panics, five silent), a rule that predicts all of them, [Q-M1e.18](open_questions.md#q-m1e18--three-kernel-primitives-are-not-shape-pinned-and-drop-their-extra-arguments) filed and `EH-M1` reproduced. The other three are scoped with owners; the milestone's closing claim is drafted [above](#what-this-milestone-may-claim-at-its-close) |
| `Q10` | The release matrix — the cross-platform legs have never executed | [S1e.1.6](#s1e16--what-nothing-pins-q8-q9-q10) — **accepted 2026-08-29**, with the reason and the one command that changes it recorded in [`docs/install.md`](../../install.md). The status was already stated in three places; `release.yml` pointed at a section that has never existed, and now points at the one that says it |

Questions this milestone *raises* get `Q-M1e.<n>` ids in
[`open_questions.md`](open_questions.md); the review's `Q1`–`Q10` keep the
review's numbering and are answered, not re-filed.

## What it handed on

Fourteen of the twenty-two questions this milestone raised are open at its
close, and **twelve of them had no owner** on the day the ledger filled
(2026-09-01). Closing them by assignment rather than by leaving the column
empty is the close's one piece of work beyond the record. Q-M1e.9 is counted
with them: it was answered, and its fix was not. The wrong answers went to a
milestone of their own,
[M1e2](../../../plans/m1e2_correctness/README.md), created 2026-10-14 for exactly them.

| question | owner at the close | why there |
|---|---|---|
| [Q-M1e.8](open_questions.md#q-m1e8--exhausted-certifies-the-lattice-not-the-model-set): `exhausted` certifies the lattice, not the model set | [M1e2 S1e2.2.1](../../../plans/m1e2_correctness/p1e2.2_wrong_answers/s1e2.2.1_maximal_not_recorded.md) (W1) | the maximality fix, and the layer-0 form the survey found |
| [Q-M1e.9](open_questions.md#q-m1e9--is-dead-really-upward-closed-under-absent): `dead` and `absent` (*answered no*, B shipped) | [M1e2 S1e2.2.5](../../../plans/m1e2_correctness/p1e2.2_wrong_answers/s1e2.2.5_naf_upward_closure.md) (W3), with M1f S1f.10.8's ruling | B contained it; the wrong answer is still there with the warning off |
| [Q-M1e.12](open_questions.md#q-m1e12--the-blind-rung-is-untyped-and-a-model-binds-a-type-as-an-object): the untyped blind rung | [M1e2 S1e2.2.10](../../../plans/m1e2_correctness/p1e2.2_wrong_answers/s1e2.2.10_types_as_objects.md) (C1) | the reading is a ruling, and it changes answers |
| [Q-M1e.15](open_questions.md#q-m1e15--the-alternatives-cap-decides-which-unsat-core-is-reported): the alternatives cap | [M1e2 S1e2.2.11](../../../plans/m1e2_correctness/p1e2.2_wrong_answers/s1e2.2.11_alt_cap_core.md) (W10) | a wrong core |
| [Q-M1e.16](open_questions.md#q-m1e16--the-binding-key-compares-two-register-layouts-as-one): the binding key | [M1e2 S1e2.2.7](../../../plans/m1e2_correctness/p1e2.2_wrong_answers/s1e2.2.7_binding_key_layout.md) (W7) | a lost derivation |
| [Q-M1e.21](open_questions.md#q-m1e21--a-rule-may-name-an-object-the-search-can-never-hypothesise-about): fresh constants | [M1e2 S1e2.2.4](../../../plans/m1e2_correctness/p1e2.2_wrong_answers/s1e2.2.4_fresh_name_unreachable.md) (W4) | a wrong model at default settings |
| [Q-M1e.20](open_questions.md#q-m1e20--two-renderers-are-produced-tested-and-unreachable): two renderers unreachable | [M1e2 S1e2.1.3](../../../plans/m1e2_correctness/p1e2.1_seeing_the_answer/s1e2.1.3_the_doors.md), option (a) | the question ended on *who wants them*, and the review of wrong answers does |
| [Q-M1e.11](open_questions.md#q-m1e11--what-happens-to-an-obligation-derived-under-a-hypothesis): an obligation derived under a hypothesis | [M1f S1f.10.6](../../../plans/m1f_hypothesis_and_documentation/p1f.10_hypothesis_structure/s1f.10.6_obligations_under_hypothesis.md), unchanged | owned since 2026-08-28; no witness exists, and the shipped guard makes any loss a lower bound |
| [Q-M1e.5](open_questions.md#q-m1e5--is-experimental-a-licence-to-ship-a-lying-surface): *experimental* as a licence | [M1f S1f.10.7](../../../plans/m1f_hypothesis_and_documentation/p1f.10_hypothesis_structure/s1f.10.7_tree_calibration_and_flag.md) and `T1d.10.6.4`; its fourth instance is S1 of [M1e2's proposed P1e2.3](../../../plans/m1e2_correctness/README.md#p1e23--surfaces-that-misreport-the-answer-proposed) | the honest fix is a reason type, which is what a tree reports |
| [Q-M1e.17](open_questions.md#q-m1e17--three-py_int-options-silently-reinterpret-a-negative): `py_int` clamps | S5 of [M1e2's proposed P1e2.3](../../../plans/m1e2_correctness/README.md#p1e23--surfaces-that-misreport-the-answer-proposed) | a surface that misreports a run |
| [Q-M1e.22](open_questions.md#q-m1e22--should-a-failed-artefact-write-have-an-exit-code-of-its-own): a failed artefact write's exit code | [M2 S2.1.2](../../../plans/m2_nl_to_ir/p2.1_kernel_as_instrumentation/s2.1.2_feedback_object.md) | the feedback object already names the other half: a budget abort exits 2, the usage-error code, so *a loop reading exit codes cannot tell a typo from a timeout*. One CLI-vocabulary decision, taken where its consumer is |
| [Q-M1e.13](open_questions.md#q-m1e13--expect-cannot-state-an-answer-with-an-empty-goal-extent): `:expect` and an empty goal extent | **open, worked around.** [M1e2's case register](../../../plans/m1e2_correctness/p1e2.1_seeing_the_answer/s1e2.1.1_the_case_register.md) states right answers as data, which is what `:expect` could not | the language gap is real, and no stage needs it closed |
| [Q-M1e.10](open_questions.md#q-m1e10--two-config--flags-are-inert): two inert flags | **open, unowned on purpose** | it changes no answer, and [`configuration.md`](../../kernel/configuration.md) says so. The next change to `ein-core/src/config.rs` decides delete or wire, the `cast.rs` rule below |
| [Q-M1e.14](open_questions.md#q-m1e14--the-corpus---jobs-sweeps-per-layer-census-coverage-is-vacuous): vacuous `--jobs` census coverage | **open, unowned on purpose** | a coverage gap that costs a golden. The next change to the parallel fan-out owns it |
| [Q-M1e.19](open_questions.md#q-m1e19--algorithmic-pathology-has-no-owner): algorithmic pathology | **open, no owner** | the one unswept surface with no home, and [the closing claim](#what-this-milestone-may-claim-at-its-close) says so in those words |

The ids stay `Q-M1e.<n>`, and the entries stay in
[`open_questions.md`](open_questions.md), here. An owner cites them across the
boundary, which is the rule M1f already lives by.

## Acceptance for the milestone

Checked at the close, line by line.

| line | |
|---|---|
| every one of the 63 findings has a disposition, and each disposition names its test, probe, site or owner | **met.** § [The findings](#the-findings) has no empty cell. The word in each cell is what was done (*fixed*, *derived*, *banked*, *documented*, *split*, *checked, and the invariant is false*), which is finer than the four dispositions and maps onto them. The two that are `accepted` as a whole, `TE-L5` and `MA-L3`, carry the reason at the site, and the sub-claims marked *refuted* name the probe |
| every one of Q1 – Q10 answered, the four that gate a fix answered first | **met** 2026-08-29. Q3, Q4 and Q5 before their fixes. Q6's ruling before `CO-H3`(c), its probe handed to S1f.10.6 |
| no well-formed `.ein` program can panic the process | **met for every shape known**, and that is the honest limit. `CO-H1`'s class was swept and refused, and `CO-L1`'s was found by a sibling grep and made a load error. The fuzz surface is one of the three unswept below |
| one list per semantics | **met.** `AR-M1`'s four pairs, plus `SE-L2` as the legitimate case |
| `docs/kernel` triaged page by page | **met.** 37 current, 3 bannered, 0 moved, 0 in the fourth state |
| the gate not weaker, and stronger where the review found it soft | **met.** No silent skip, derived floors, the step lists diffed, the censuses re-takable and nightly |
| `./run_tests.sh` green at every phase boundary, the counts quoted from a run | **met.** 738 → 744 (S1e.5.1) → 804 over 90 (P1e.3) → **821 over 93** (P1e.4, and again on 2026-10-14) |

### What this milestone may claim at its close

**Drafted 2026-08-29 by S1e.1.6, before the work it describes**, because a
closing claim written at the end is written by someone who wants to be
finished. Edited at the close, not composed then. The edit adds the last
paragraph.

> This milestone processed a review whose **verification stage never ran**
> ([`review/summary.md`](review/summary.md) § Method). Three of its 63 findings
> were reproduced against the binary before it aborted. The other sixty
> carried a reading-pass confidence label and nothing more, and this
> milestone's dispositions are what happened when each was probed.
>
> **Four surfaces had no dedicated pass in that review**, and this milestone
> swept **one** of them:
>
> | surface | status |
> |---|---|
> | parser / CLI edges | **swept**, S1e.1.6 T2: `ein-cli/tests/primitive_arity.rs`, 21 cells. It found `CO-H1`'s class and [Q-M1e.18](open_questions.md#q-m1e18--three-kernel-primitives-are-not-shape-pinned-and-drop-their-extra-arguments) |
> | `ein-einb/src/cast.rs`'s `unsafe` | **not swept.** Owned by whoever next changes `ein-einb`, against [design/12](../m1a_rust/design/12_toolchain_and_layout.md) §2 |
> | micro-CSP ground truth | **not swept.** It is [M10](../../../plans/m10_external_benchmarks/README.md)'s thesis and is named there, not duplicated here |
> | algorithmic complexity / pathology | **not swept, and has no owner**: [Q-M1e.19](open_questions.md#q-m1e19--algorithmic-pathology-has-no-owner) |
>
> So the tree's cleanliness **outside** those three surfaces rests on a
> reading pass, a green gate, and the fixtures and tests this milestone added,
> and on nothing else. Absence of findings in the three is absence of
> evidence. **This milestone may not be read as saying the tree is clean.**
> The review it processed should eventually be re-run. A thirteen-finder pass
> is a milestone's worth of compute, not a stage's, and S1e.1.6's own sweep
> is the argument that it would pay: one probe of one surface returned five
> silent wrong-answer cells, on top of the one panic the review had already
> found there.
>
> **And it may not be read as saying the engine's answers are right.** The
> review listed no wrong answer. This milestone's probes found several:
> programs on which `ein solve` says *No solution* with `exhausted = true`
> where a model exists, or one model where there are two, some of them under
> the default settings. It filed them rather than fixing them, because they
> are not review findings and their fixes move answers. At the close they were
> the largest open item in the tree, and they are
> [M1e2](../../../plans/m1e2_correctness/README.md)'s.

## The documents beside this one

| document | what it is | why it is kept |
|---|---|---|
| [`review/`](review/summary.md) | the seven readers' reports, 18 files, verbatim | the input; every disposition above answers a sentence in it, and a re-run of the review is compared against it |
| [`open_questions.md`](open_questions.md) | Q-M1e.1 – Q-M1e.22 | fourteen are open, and the owners above cite them by anchor |
| [`search_soundness_probes/`](search_soundness_probes/README.md) | S1e.1.1's stage file, D1 – D9, the golden audit, two probe scripts and a probe program | the evidence behind the wrong answers M1e2 owns: the hand derivations, the witnesses' construction, and what a fix moves |

Everything else, the milestone README as planned, five phase READMEs and 27
stage files, is in `git log --diff-filter=D -- plans/m1e_review_processing`,
and what of it still matters is in the five phase sections above.
