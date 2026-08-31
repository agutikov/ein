# S1e.3.2 — Semantics (Medium)

**Phase:** [P1e.3](README.md) (Medium)
**Estimate:** 2 days
**Depends on:** [S1e.3.4](s1e.3.4_architecture.md) for `SE-M1` if the seam
fix is taken; [CO-H3](../p1e.2_high/s1e.2.1_correctness.md) for `SE-M3`,
which documents whatever that decides; and
[Q-M1e.7](../open_questions.md#q-m1e7--the-read-out-prints-the-solution-kb-and-calls-it-a-model)'s
ruling, which this stage **applies** rather than owns — the two objects the
engine conflates (the solution *state* and the *model* projected out of it)
need two names in four surfaces, and that is `SE-M1`'s defect at a second
site. The ruling is
[S1e.1.1](../p1e.1_open_questions/s1e.1.1_search_soundness_probes/README.md#task-t1e114--the-record-site-conformance-check)
T1e.1.1.4's, taken two phases earlier because Q-M1e.8's fix waits on it.
**Findings:** [`SE-M1`](../review/semantics/medium.md),
[`SE-M2`](../review/semantics/medium.md),
[`SE-M3`](../review/semantics/medium.md).

## Context

Three findings about **what a word means to a consumer**, which is the axis
M1d spent a whole phase on and which is why these are worth more than their
size.

`SE-M1` — S1d.3.3 deliberately split two numbers that had always agreed:
`verdict.k` counts *models*, `stats.solution_nodes` counts what the *search*
recorded. They part on `Open`. But `ein test`'s verbose per-query header
prints `stats.solution_nodes` under the label `k = {}`
([`test.rs:832-838`](../../../ein.rs/crates/ein-cli/src/test.rs) against
`:784-795`), deliberately, because that is *what it has printed since
S1c.1.3* — so the human-facing `k =` and the machine-facing `ran.k` disagree
**by name** on every `Open` entry. That is the exact vocabulary confusion
S1d.2.6 eliminated everywhere else.

`SE-M2` — [`summary.rs`](../../../ein.rs/crates/ein-cli/src/summary.rs)
justifies unconditional field emission three times, with the same sentence:
*a field that appears only sometimes is a field a consumer has to guess
about*. `build_aborted` (`:600-632`) then omits `verdict.open_states` and the
whole `leftover` block, both of which the normal build emits unconditionally.
A consumer switching on schema `ein-summary/1` sees them only on non-aborted
runs.

`SE-M3` — under `EIN_TRAVERSAL=tree`, four things are observable and none is
stated where a user or consumer would look: `exhausted` is `false` by design
(termination by **discharge**, not exhaustion); dead branches contribute
nothing to nogood counters or the proof; `-n`/`-m` are ignored; and the
`traversal` event the decline path emits has **no row** in
[`events.md`](../../../docs/kernel/inference/events.md), on a page that calls
itself *every step the engine took*. `T1d.10.6.4` is honestly recorded as the
open question for what a tree reports where a lattice reports layers — but the
pieces that already **ship** have no statement anywhere.

## Acceptance

- No surface prints one number under another's name. Where a surface must
  keep printing the search count for continuity, the **label** changes, not
  the reader's job.
- `ein-summary/1`'s shape is the same on every arm, or the exception is
  recorded in the schema's own documentation as deliberate — the schema states
  its rule three times and does not get to break it silently.
- The shipped subset of tree-mode reporting is documented — one paragraph in
  `events.md` plus the `EIN_TRAVERSAL` block — **independent of**
  `T1d.10.6.4`.
- Each fix has a test: a `--json-report` vs verbose-header consistency check
  for `SE-M1`, an aborted-summary shape assertion for `SE-M2`, and the
  `traversal` event's row pinned the way `events.md`'s other rows are.

## Tasks

### Task T1e.3.2.1 — `SE-M1`: two things called `k` ✅

The finding is one label. The fix is a choice between two, and the second is
better:

- **Rename the label** — `recorded = {}` — which is honest and breaks no
  consumer, since the verbose header is human-facing by construction.
- **Print both**, as the `Ran` row already does: `k = {verdict}` beside
  `recorded = {search}`. This is strictly more informative and it is the
  form the rest of the tool converged on at S1d.3.3, where an `Ambiguity`
  learned to qualify its own count rather than print a bare number.

Take the second. Then add the consistency test that makes the class closed
rather than the instance: for every entry the runner solves, the verbose
header's numbers and `--json-report`'s row agree field for field, under the
names each uses. That test is what would have caught this.

The `Solution` table arm has the same inheritance and is
[CO-M2](s1e.3.1_correctness.md); the seam both live on is
[AR-M2](s1e.3.4_architecture.md).

### Task T1e.3.2.2 — `SE-M2`: the `Aborted` shape ✅

Emit `verdict.open_states` and `leftover` present-and-empty on aborts, the
way `owes` already is. That is the schema's own rule applied to the one arm
that breaks it, and the diff is small.

The alternative — record the exception as deliberate — is available and worse:
the rule is stated three times in one file precisely so that a consumer does
not have to read the producer, and an exception written into the schema doc
puts the burden back.

Then pin it: `summary_properties.rs` holds thirteen counter identities
already, and *every arm emits the same key set* is the same kind of property.
Note that an aborted run is easy to produce on demand (`-E 1`), so the
fixture is a flag rather than a program.

While in `build_aborted`, check the rest of its shape against the normal
build's: the review found two omissions by reading, and the property test
will find any others by construction.

### Task T1e.3.2.3 — `SE-M3`: document the shipped subset ✅

Not the design question — the shipped facts. Four of them, and each has a
home:

| fact | where it goes |
|---|---|
| the `traversal` event (`{kind:'tree', verdict:'declined', reason:…}`) | a row in [`events.md`](../../../docs/kernel/inference/events.md)'s schema table, marked experimental if `T1d.10.6.4` wants the freedom |
| `exhausted = false` by design — termination by discharge | the `EIN_TRAVERSAL` block, with the one sentence from T1d.10.5.1 that says what discharge licenses |
| dead branches contribute nothing to nogood counters or the proof | same block; and after [CO-H3](../p1e.2_high/s1e.2.1_correctness.md)(b) it also says what the `Contradiction` arm now prints instead |
| `-n` / `-m` behaviour | same block, stating whatever `CO-H3`(a) decided — honoured, or refused with a reason |

The `traversal` row is also [CD-M2](s1e.3.7_code_doc_consistency.md)'s third
item; do it once, in whichever stage reaches it first, and mark the other
finding as closed by that commit rather than writing the row twice.

The point of doing this *now*, before the design question is answered: a
capability observable only through surfaces whose meaning is unstated is a
capability nobody can use, and `EIN_TRAVERSAL=tree` is the headline M1d
result.

## Notes

`SE-M1` and `CO-M2` are the same defect at two surfaces and `AR-M2` is the
reason there are two. If [S1e.3.4](s1e.3.4_architecture.md) takes the seam
fix, both collapse into it and this stage keeps only the consistency test —
which is the more valuable half anyway, since it is what keeps the class
closed after the next verdict word arrives.

---

## Outcome

Taken 2026-08-31, after [S1e.3.4](s1e.3.4_architecture.md) had already fixed
`SE-M1`'s label and [S1e.3.1](s1e.3.1_correctness.md) had banked its witness.

| | |
|---|---|
| **`SE-M1`** | **closed** — the label was S1e.3.4's; this stage added the half its own notes called the more valuable one. `the_verbose_header_and_the_report_row_agree_field_for_field` rebuilds every `-v` header **from the report row** and compares, over `ein test examples tests stdlib` — 68 checked queries in 0.06 s, of which **13** have `k != solution_nodes`. Its control: printing `solution_nodes` under `k =` again fails it on `examples/features/13_mixed_solution_and_open.ein`, by name |
| **`SE-M2`** | **fixed at the seam, not the arm.** `build_aborted` is now `build` with an `Answer::Aborted`, so *every arm emits the same key set* is true by construction; the two omissions the review found (`verdict.open_states`, the whole `leftover` block) and one it did not (`verdict.reason`, which existed only on the aborted arm) are all consequences of that one line. Pinned by `the_summary_has_one_shape_on_every_arm` |
| **`SE-M3`** | **documented** — [`events.md` § `traversal`](../../../docs/kernel/inference/events.md) and [`configuration.md` § What `EIN_TRAVERSAL=tree` reports](../../../docs/kernel/configuration.md). Five facts, not the four the task listed: the fifth is below |
| `CD-M2` | its **third item** — *the `traversal` event has no row* — is closed by this commit, as the task's own instruction says. The other two ( `watched` on `admit`, `n_guards` naming a disjunct count) stay [S1e.3.7](s1e.3.7_code_doc_consistency.md)'s |
| gate | `./run_tests.sh` green — **783 tests**, exit 0, five static checks and the bench smoke unmoved. No golden moved, no corpus cell moved, no counter moved |

### The shipped subset was five facts, and the fifth is the one that bites

The task named four. Measuring them turned up a fifth that nothing in the repo
says and that a user meets on their first attempt:

**Under `EIN_TRAVERSAL=tree`, `ein test` can never mark a claim `held`.** An
expectation is a claim about the *exhausted* answer, and a tree reports
`exhausted = false` by design. So every `:expect` on a program the tree
**accepts** comes back `NOT CHECKED` and the runner exits **1** — measured on a
five-line obligations program with an `(or …)` claim the lattice holds: `1
held` under the lattice, `0 held, 1 not checked` under the tree, same two
models. Neither component is wrong; it is what running a non-certifying
traversal under a command that exhausts by definition comes to, and it is now
the third bullet under *`exhausted` is `false`, always* in
[`configuration.md` § What `EIN_TRAVERSAL=tree` reports](../../../docs/kernel/configuration.md).

### Three things the review's four did not include

**The four kinds a tree does not emit.** `SE-M3` says the `traversal` event has
no row, which was true; what nothing said is that `enter`, `layer`, `nogood`
and `writeback` are **not emitted at all** under this traversal. `tree_node`
calls `commitment::try_commitment_set` directly rather than through
`Run::finish_entering`, which is where `enter` is emitted, so **the enterings
are invisible in a stream that still counts them**; there are no layers to
census; and a dead branch is recorded without being learned from, which is
`CO-H3`(b)'s decision seen from the stream side. Measured on the smallest
program that reaches the dead arm — the lattice emits seven lines across those
four kinds where the tree emits one `traversal` and none of them. It also
falsifies one sentence `events.md` already had: *"`Σ entered` is
`enterings_total` on any run at all"* is now qualified to *any lattice run*.

**A declined tree run is the lattice's answer and not the lattice's stream.**
`the_tree_declines_on_a_rung_that_is_not_the_obligations_one` pins the entering
count to the digit, and every field of `--json-summary` matches too — measured,
0 of them differ on `examples/zebra2.ein`. The **stream** does not: root's probe
is a real generation call, so a declined run carries one extra pass of it — 125
further `hyp` lines, 125 further `compile` lines and the `traversal` line, 16
435 events against 16 184. Worth writing down because it is the shape a
consumer would get wrong in exactly one direction: a verdict diff across this
variable is a finding and a stream diff is not.

**A kind reachable only under an environment variable is a kind no sweep can
see.** `every_event_kind_the_schema_defines_is_reachable_from_the_corpus`
parses the kinds out of the page and requires a fixture for each, which is
exactly the check that should have caught a missing `traversal` row — and
could not, because it also requires the row to *exist*, and because every
cover file ran in the default environment. `EVENT_COVER` grew a third column
(the environment a row needs, empty for six of the seven) and its
`ein_traversal` twin was folded into one helper. The floor on the kind count
went from `>= 18` to `>= 21`, which is the number of rows.

### What this stage did **not** do

- **Fix the `NOT CHECKED` diagnostic.** It reads *"Either the run stopped at
  `-n`, or the frontier is still alive at `--max-set-size`"* — and under
  `EIN_TRAVERSAL=tree` **neither is true**: the run exhausted its tree and `-m`
  is refused outright. That is the same shape as `CO-H3`(b) — a surface stating
  evidence it does not have — one layer further out, and it is recorded on
  [Q-M1e.5](../open_questions.md#q-m1e5--is-experimental-a-licence-to-ship-a-lying-surface)
  rather than fixed here: `expect::check` is handed a `bool` and has no way to
  know *why* a search did not exhaust, so the honest fix is a signature change
  in `ein-infer` and not a string edit in a documentation task.
- **Rename anything on the `Aborted` arm.** `verdict.k` there is still
  `stats.solution_nodes`, which is what `Answer::k` returns for an answer that
  is not a verdict, and that is right: an abort has no models, so what it can
  report is what the search recorded.
- **Touch `T1d.10.6.4`.** Both pages say what ships and name the open question;
  neither answers it.
