# S1e.3.7 — Code ↔ doc consistency (Medium)

**Phase:** [P1e.3](README.md) (Medium)
**Estimate:** 4 days
**Depends on:** [S1e.2.2](../p1e.2_high/s1e.2.2_code_doc_consistency.md) —
the triage rule and the page-state convention; these eight are the pages the
High stage did not name.
[CO-H3](../p1e.2_high/s1e.2.1_correctness.md) for `CD-M2`'s `traversal` row.
**Findings:** [`CD-M1`](../review/code-doc-consistency/medium.md) …
[`CD-M8`](../review/code-doc-consistency/medium.md).
**Status:** ✅ **done 2026-08-31** — eight findings, eight commits, one per
page group. What the stage is worth beyond the eight is in
[§ Outcome](#outcome): three of the eight became a **check**, and two of the
three found a defect the review had not.

## Context

Eight pages that describe something the code does not do. Unlike
[CD-H1](../p1e.2_high/s1e.2.2_code_doc_consistency.md)'s six, none of these
misrepresents the engine's *architecture* — each is a specific wrong symbol,
a wrong payload, a wrong command, or a number that was corrected in one
paragraph and not in the one citing it.

Two of them are worth reading as more than doc bugs.

**`CD-M2` — `events.md`.** It sells itself as the schema for external
observers, and [M20](../../m20_gui/README.md) is the likely feed. It claims
`watched` on all of park/admit/retire — `admit` carries none; it documents
`compile.n_guards` as a guard count when it is the **disjunct** count, a
knowingly wrong number kept as a comparison surface whose truth lives only in
an `engine.rs` comment; and the `traversal` event the tree's decline path
emits has **no row at all**, on a page that claims to be *every step the
engine took*. A consumer coding to the table looks for a field `admit` never
carries.

**`CD-M4` — `docs/api/rust.md`.** The page's `rust` block is the marked region
of [`embedding.rs`](../../../ein.rs/crates/ein-cli/tests/embedding.rs), and a
test diffs the two, so the block cannot rot. The **prose around it** says the
other verdict arms are exercised by `the_other_two_verdicts_are_reachable` and
that *the match is not three arms* — the test was renamed
`the_other_three_verdicts_are_reachable` when `Open` arrived, and the match
now has five arms. This is the one class of drift the mechanism structurally
cannot catch, demonstrated **on the mechanism's own page**.

The rest: three pages hyperlink a nonexistent `Engine::step()` (`CD-M1`); a
corrections section disagrees with the prose it corrected inside one file
(`CD-M3`); `stdlib/README` offers a CLI subcommand that does not exist
(`CD-M5`, verified: *"error: unrecognized subcommand 'ir'"*);
`examples/README` points at a deleted Python engine and a deleted acceptance
test (`CD-M6`); `utils/README` gives the right count with the wrong cause for
86 % of a set (`CD-M7`); and `architecture_and_algorithms.md` mixes as-built
`ein.rs` facts with as-was `ein.py` vocabulary unmarked, while a sibling page
marks the same divergence (`CD-M8`).

## Acceptance

- Every symbol, command, env var and payload field these eight pages name
  resolves in the current tree, or is marked historical in the tree's own
  banner convention.
- `events.md`'s schema table matches the emitters field for field, including
  the `traversal` row, and the `n_guards` truth moves out of an `engine.rs`
  comment into the page.
- `docs/api/rust.md`'s prose matches its test — and the **marker is widened**
  (or a second marker added) so the sentence naming the test is inside the
  diffed region.
- `examples/README`'s two-encodings claim names a test that exists
  ([Q8](../p1e.1_open_questions/s1e.1.6_coverage_gaps.md) supplies it).
- Each fix is one commit per page, so a later reader can see which page moved
  and why.

## Tasks

### Task T1e.3.7.1 — `CD-M1`: `Engine::step()` does not exist

Three pages hyperlink `engine.rs` for a method that is not there —
[`absent_semantics.md:124-127`](../../../docs/kernel/inference/absent_semantics.md),
[`architecture_and_algorithms.md:157-159`](../../../docs/kernel/inference/architecture_and_algorithms.md),
[`implementation.md:66`](../../../docs/kernel/inference/implementation.md).
`engine.rs` is a **compile cache**; the two-phase loop is
[`Saturator::step`](../../../ein.rs/crates/ein-infer/src/saturator.rs) at
`:540`, and it is **queue-based**, not queue-less as the pages say.

The symbol survived from `ein.py`'s `Engine`, and the S1a.10.6 doc pass missed
it in all three places — which is the same *milestone-scoped doc pass cannot
catch a two-milestone-old symbol* observation
[T1e.2.2.5](../p1e.2_high/s1e.2.2_code_doc_consistency.md) turns into a
checklist. Fix the symbol, the link and the queue-less description in all
three; then grep for other `Engine::` references, since one survivor implies
a family.

### Task T1e.3.7.2 — `CD-M2`: make `events.md` match the emitters

Three corrections and one addition:

| item | truth | source |
|---|---|---|
| `watched` on park/admit/retire | `admit` carries **none** | [`saturator.rs:1127-1135`](../../../ein.rs/crates/ein-infer/src/saturator.rs) |
| `compile.n_guards` | it is the **disjunct** count, knowingly wrong, kept as a comparison surface | [`engine.rs:184-190`](../../../ein.rs/crates/ein-infer/src/engine.rs) |
| the `traversal` event | `{kind:'tree', verdict:'declined', reason:…}` — no row exists | [`solve.rs:908-913`](../../../ein.rs/crates/ein-infer/src/solve.rs) |
| the page's own claim | *every step the engine took* | — |

For `n_guards`, do not silently rename: the name is wrong **on purpose** (it
is what `ein.py` printed, kept comparable), so the page states the name, the
real content, and the reason. That is a three-line note and it is worth more
than a rename would be.

For `traversal`, coordinate with
[SE-M3](s1e.3.2_semantics.md) T3 — the same row, one commit — and mark it
experimental if `T1d.10.6.4` wants the freedom to change it.

Then the durable part: `events.md` is a schema with **no mechanical check
against its emitters**. Whether one is feasible — an enumeration of `emit`
call sites compared against the table — is worth an hour's investigation and,
if cheap, is worth more than the three fixes. File it if it is not cheap;
M20's feed will want it either way.

### Task T1e.3.7.3 — `CD-M3`: propagate `features.md`'s own corrections

[`features.md:170-171, 175-179, 199-203`](../../../docs/kernel/inference/features.md)'s
§ Two corrections establishes **3 557** (not 3 831) enterings / 54.5× and
**101** (not 134) / 1.1×, and claims *"the two conclusions that rested on them
are amended where they stand"*. They are not: the per-lever notes still read
*101 → 3 831 (38×) … 56.6×* and *134 commitments … exactly the 1.2× that
implies … 33 more dead ends*, and
[`architecture_and_algorithms.md:683-688`](../../../docs/kernel/inference/architecture_and_algorithms.md)
still quotes 3 831/56.6×.

So a correction section and the section it corrects disagree **inside one
file**, and a second file quotes the uncorrected number. Apply the amendment
where the numbers stand, in both files. Note the second-order lesson for
[S1e.3.8](s1e.3.8_documentation.md): *"amended where they stand"* was written
as though it had been done, which is how a correction becomes a third wrong
statement.

### Task T1e.3.7.4 — `CD-M4`: fix the prose and widen the marker

[`docs/api/rust.md:266-267`](../../../docs/api/rust.md) names
`the_other_two_verdicts_are_reachable`; the test is
`the_other_three_verdicts_are_reachable`
([`embedding.rs:149`](../../../ein.rs/crates/ein-cli/tests/embedding.rs)), and
the match has five arms — four verdicts plus `Aborted`.

Fix the sentence, then fix the **mechanism**, because the finding's real
content is that the page's own anti-rot device has a blind spot: prose outside
the marked region. Two options — widen the existing marker to include the
sentence that names the test, or add a second marker around it. Prefer a
second marker: the first exists to quote a *code block*, and stretching it
over prose makes the diff test's failure messages harder to read.

Then check the rest of the page for other claims about the test file that sit
outside markers. One instance found by reading implies others.

### Task T1e.3.7.5 — `CD-M5`: `ein ir parse --resolve` does not exist

[`stdlib/README.md:212-217`](../../../stdlib/README.md) offers it as the way
to inline imports; the CLI refuses it (`error: unrecognized subcommand 'ir'`).
The removal is recorded in `utils/README.md:35-36`, and the two docs disagree
about **when** it happened — P1.7c versus `render_examples.sh:26`'s P1.11.

Two fixes: replace the instruction with the route that exists (a `ein render`
adjacent command, or the library call — establish which actually inlines
imports before writing it down), and reconcile the two removal dates from git
history. The date matters only a little; the disagreement matters more,
because it is two docs stating one fact differently, which is
[AR-M1](s1e.3.4_architecture.md)'s pattern in prose.

### Task T1e.3.7.6 — `CD-M6`: `examples/README`'s dead pointers

[`examples/README.md`](../../../examples/README.md) says *"see docs/api/ to
drive them from Python"* (`:5`) — no Python module exists, which is
`docs/api/`'s whole point — and names
`acceptance/test_zebra_two_ontologies.py` (`:27`), a file in a directory that
does not exist anywhere.

The claim that dead pointer carried is the substantive part: **zebra.ein and
zebra2.ein agree cell by cell**.
[Q8](../p1e.1_open_questions/s1e.1.6_coverage_gaps.md) either found the test
that pins it or wrote one; this task names it in the README, replacing the
dead pointer with a live one.

Also here: the dangling *"C2"* reference (`:28-29`, and the same at
[`stdlib/README.md:139-140`](../../../stdlib/README.md)) — bare link text
whose target is gone, almost certainly a deleted-plans link reduced to its
words. It is the worst of the dangling class because the referent (the
two-ontology design comparison) is now unlocatable from either document.
Either restore the content from git history into `docs/history/` or delete
the sentences; do not leave a third state.
[DO-M2](s1e.3.8_documentation.md) owns the dangling class and this is its
sharpest instance — decide it here and let that stage cite the decision.

### Task T1e.3.7.7 — `CD-M7`: 4 + 25, not 29

[`utils/README.md:51`](../../../utils/README.md) says *"29 entries drop
`solve` because it does not terminate on them"*. The script and
[`corpus/README.md:151-162`](../../../corpus/README.md) say non-termination
covers **four**; the other **25** drop `solve` because a solve does not ask
their question. The count is right and the causal claim is false for 86 % of
the set.

Split the sentence. And note the distinction is load-bearing beyond this line:
*a run is dropped from a `runs` column only when it does not ask the fixture's
question, never for costing too much* is one of the corpus's stated rules, and
a README that attributes 25 drops to cost is contradicting it.

### Task T1e.3.7.8 — `CD-M8`: mark the as-was vocabulary

[`architecture_and_algorithms.md`](../../../docs/kernel/inference/architecture_and_algorithms.md)
mixes as-built `ein.rs` facts with as-was `ein.py` vocabulary and does not say
which is which:

- `:62-73` — the deductive-layer file list names `saturator.rs` and
  `firing.rs` **twice each**;
- `:193` — O4 says `EqClasses` is *"wired into the API so firing can call
  `kb.classes.union`"*; `firing.rs` contains **no such call** — equality is a
  stub and matching does not resolve classes, pinned by
  `naf_semantics::matching_does_not_resolve_equality_classes`;
- `:434-439` — section 3's type table uses `ein.py` names (`JoinPlan`,
  `World`, `Scan`/`Join` opcodes) the Rust engine renders differently.

`implementation.md:69` marks the `World` divergence; this page does not — so
the two designated as-built references disagree about how faithfully to
describe the port. Dedupe the list, fix O4 to *stub* (with the pinning test
named, since that is the strongest statement available), and mark the `ein.py`
vocabulary as historical where it is kept for continuity.

## Notes

Eight pages, one commit each, and the ordering that saves work: `CD-M2` and
`CD-M6` both have a partner task in another stage
([SE-M3](s1e.3.2_semantics.md), [Q8](../p1e.1_open_questions/s1e.1.6_coverage_gaps.md)),
so take them after those land. The other six are independent.

Every one of these eight is the same failure with a different surface: a claim
that no execution touches. `CD-M4` is the interesting one precisely because
the page **has** a mechanism and the claim slipped past its boundary — which
is the argument for [DO-M2](s1e.3.8_documentation.md)'s link checker and
against believing that a partial mechanism covers a page.

---

## Outcome

**Done 2026-08-31.** Eight findings, eight commits. The table is what each one
turned out to be, and the column that matters is the last: a doc fix that
leaves nothing running is the same fix again in six months.

| | what it was | what it left behind |
|---|---|---|
| `CD-M1` | **five** pages, not three — the grep found `inference/README.md` and `architecture.md` | nothing mechanical; the symbol is gone from the tree |
| `CD-M2` | **four** items, not three: `warn` had no row at all | [`events_reference.rs`](../../../ein.rs/crates/ein-cli/tests/events_reference.rs) — every `.emit("…")` against every documented kind, **both directions**, 22 kinds, 0.01 s |
| `CD-M3` | as reported, in four places | the second-order rule, written into § Two corrections itself |
| `CD-M4` | as reported, plus § 4 naming three verdicts | a second marker **and** `the_page_and_the_file_name_the_same_tests`, because the marker alone would not have caught it |
| `CD-M5` | as reported; both removal dates reconciled to one commit | `the_inlining_route_the_stdlib_readme_documents_round_trips` — the README's snippet is a test |
| `CD-M6` | as reported; the `C2` disposition decided | the decision, for [`DO-M2`](s1e.3.8_documentation.md) to cite |
| `CD-M7` | as reported, and fixed at the origin too | the split, in the guard's own comment |
| `CD-M8` | O4 was worse than reported: `EqClasses` has **no** engine caller | a nine-row as-was → as-built map, and the two lists point at `implementation.md` |

### The three things this stage found that the review did not

**1 — `events.md` was missing a whole event kind.** `warn` has been in the
stream since [S1e.2.3](../p1e.2_high/s1e.2.3_naf_refutation_diagnostic.md) and
gained a third category at
[S1e.3.3](s1e.3.3_state_model.md); the page named it only inside § Comparison's
parity spine, which is a list of what is *diffed*. The review found two wrong
payload fields on a page whose bigger problem was a missing row — which is the
argument for the check rather than for the three fixes, and is why
`T1e.3.7.2`'s "worth an hour's investigation" was the right instruction. It
also found that **three** of `compile`'s six numbers are misnamed, not one:
`n_disjuncts` is *d* − 1 and `n_steps` is the first disjunct's alone.

**2 — a second marker does not close `CD-M4`.** The acceptance asked for the
sentence to be inside a diffed region, and it is. But a marker makes the page
and the file **one text**; it does not make that text **true**. Rename a test
and leave the comment alone and the two agree perfectly about a name neither of
them has. The check that would have failed on the day of the rename resolves
the names in both directions, and it is 40 lines. *A partial mechanism is worse
than none where it is mistaken for coverage* — the stage's own § Notes says
this, and the fix had to be built to the sentence rather than to the task list.

**3 — `EqClasses` has no caller at all.** The review said `firing.rs` contains
no `kb.classes.union` call. Nothing does: the union-find's only two callers in
the workspace are tests. The difference matters for `O4`'s status — *a stub
with an unused API* is a different claim from *a stub with a wrong caller
named* — and it is the kind of thing only a grep settles.

### What the eight had in common, and the one that did not

Seven of the eight are a claim no execution touches. `CD-M4` is the exception
and the interesting one: the page **had** a mechanism, and the claim slipped
past its boundary. Both of this stage's other new tests are the same shape as
its fix — take the thing a page asserts about the tree, and make the tree
answer. `events_reference.rs` asks the emitters; `the_inlining_route_…` runs the
snippet; `the_page_and_the_file_name_the_same_tests` resolves the names. None
of the three is more than fifty lines.

### The gate found the fourth thing

Adding the `warn` row broke
`cli_semantics::every_event_kind_the_schema_defines_is_reachable_from_the_corpus`,
which parses the same three tables and requires **a corpus fixture** behind
every kind — so the page and the corpus are held together from the other end,
and the two checks are complementary rather than redundant:
`events_reference.rs` asks *does an emitter exist*, that one asks *does a
program reach it*. Two rows added to `EVENT_COVER`, one per category a corpus
entry can reach — `naf-upward-closure.ein` for
`RefutationUnderAbsentWarning` (the only entry carrying `(config
:warn-derived-naf true)`, so the only one from which the gated half is
reachable at all) and `alive-set-fresh-name.ein` for `alive-set-invariant`,
which is a different call site in `solve.rs`. One kind, two rows, because a
cover reaching `warn` through the gated category alone would stay green if the
static check stopped narrating. Its `>= 21` floor is `>= 22`.

It also broke the parse in a way worth recording: the category table's first
column is `` `category` ``, and `schema_kinds()` reads every `| \`x\` |` line in
a payload section as a kind cell. The fix is the page's own idiom — a `####`
subsection, as `rung`, `layer` and `traversal` already have — so a kind that
needs elaboration gets it *outside* the tables that are parsed.

### Left for later, deliberately

- **The `emitted at` and payload columns are still hand-written.** The check
  covers the *kind set*, because the conditional payload fields live inside the
  emit closure — `traversal` carries `depth` only when a node declines — and a
  payload checker would have to model `EventLine`'s builder. The kind set is
  the axis the page was actually wrong on: a missing row is invisible to every
  reader, where a wrong field at least appears beside a right one.
- **`docs/history/` was not touched.** `baseline.md` still records 3 831 /
  56.6× and `design/07` still records 3 831 enterings; those are the readings of
  their days and rewriting them would falsify the record. The one *current*
  page that quotes a dated profile keeps the reading and gains today's beside
  it.
- **A superseded-number register.** `CD-M3` would have been caught by a check
  that no retracted figure appears outside the section retracting it. It is
  [`DO-M1`](s1e.3.8_documentation.md)'s shape, not this stage's, and the rule
  it needs is now written where the next reader of a correction will be.
