# S1e.3.2 — Semantics (Medium)

**Phase:** [P1e.3](README.md) (Medium)
**Estimate:** 2 days
**Depends on:** [S1e.3.4](s1e.3.4_architecture.md) for `SE-M1` if the seam
fix is taken; [CO-H3](../p1e.2_high/s1e.2.1_correctness.md) for `SE-M3`,
which documents whatever that decides.
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

### Task T1e.3.2.1 — `SE-M1`: two things called `k`

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

### Task T1e.3.2.2 — `SE-M2`: the `Aborted` shape

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

### Task T1e.3.2.3 — `SE-M3`: document the shipped subset

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
