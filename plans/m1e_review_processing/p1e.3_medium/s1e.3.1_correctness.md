# S1e.3.1 — Correctness (Medium)

**Phase:** [P1e.3](README.md) (Medium)
**Estimate:** 4 days
**Depends on:** [Q4](../p1e.1_open_questions/s1e.1.1_search_soundness_probes/README.md)
for T1; [S1e.3.4](s1e.3.4_architecture.md) for T2 if the seam fix is taken
there.
**Findings:** [`CO-M1`](../review/correctness/medium.md) …
[`CO-M6`](../review/correctness/medium.md).

## Context

Six code defects, none of which changes an answer on a corpus program today.
Three are **latent soundness or safety gaps** (`CO-M1`, `CO-M3`, `CO-M5`),
one is a **read-out that would contradict itself** the day a defined regime
is reached (`CO-M2`), and two are **API-contract hazards** — a duplicated
pipeline whose doc comment points at the wrong one (`CO-M4`) and a predicate
that mutates while claiming to ask (`CO-M6`).

What they have in common is worth naming, because it decides how they get
fixed: in every one of the six, the *code* is defensible and the *statement
about the code* is wrong. `is_stalled` really is safe for its current
callers; `UNBOUND` really is never produced by `pack()`; the second macro
pipeline really does work for its non-loader consumer. Each finding is the
gap between that and what a next caller would reasonably assume.

## Acceptance

- ✅ Each of the six has a disposition, and **all six are `fixed`** — four of
  them here, `CO-M2` at [S1e.3.4](s1e.3.4_architecture.md) with its witness
  fixture here, and none `accepted`, so no reason had to be written in place of
  a change.
- ✅ **`UNBOUND` cannot be mistaken for a real `FactId` through the public
  accessors.** `as_fact` rejects the sentinel, `tag` refuses to answer about it
  in a debug build, and `value.rs`'s `the_sentinel_names_no_fact` is the
  assertion that failed before.
- ✅ **One macro-ingestion path.** `macros::read_macros` is the reading and
  `from_ir::ingest_macros` is the interning; the four non-loader consumers
  refuse exactly what a load refuses, with the same sentences.
- ✅ **No behaviour change reaches a golden without being named here first** —
  the [outcome](#outcome) names all three: five verdict rows (the three
  `ein-bugs` fixtures), seven `ir[expand]` renderings (`CO-M4`, the dump path
  agreeing with the loader), and `saturate_count` on 191 more (`CO-M1`, a
  saturation the engine really performs).

## Tasks

### Task T1e.3.1.1 — `CO-M1`: the inter-layer alive-∅ path ✅

`phase2` calls `record_node(root)` when `compute_alive` comes back empty
([`solve.rs:1528-1551`](../../../ein.rs/crates/ein-infer/src/solve.rs)),
**without** the `has_contradiction` re-check that phase1 (`:1091`) and the
cascade (`:2131`) both perform.

[Q4](../p1e.1_open_questions/s1e.1.1_search_soundness_probes/README.md) has already
determined which of three this is — a real gap, a refuted one, or an
unreachable branch — and its outcome table says what this task does. Execute
that row; do not re-derive it. If the row is *fixed*, the change is one call
and the fixture from Q4 is the regression test; if it is *accepted*, the
argument goes beside `:1528` and states the two premises explicitly — the
writebacks are `(not h)` for `h ∉ root`, and root has not been re-saturated
on this path — with the second one's limit named, since that limit is the
finding.

### Task T1e.3.1.2 — `CO-M2`: the `Solution` arm's count ✅

`finalise` admits a state where one node is a discharged model and others are
open: `branches.len() == 1` with `open_states` non-empty →
`Verdict::Solution`
([`solve.rs:2428-2441`](../../../ein.rs/crates/ein-infer/src/solve.rs), whose
own comment says *no corpus entry is in that regime today … defined rather
than measured*). There `stats.solution_nodes > 1`, and `ein-cli` passes it
into `render_solution_table`
([`answer.rs:419-433`](../../../ein.rs/crates/ein-render/src/answer.rs),
[`solve.rs:648`](../../../ein.rs/crates/ein-cli/src/solve.rs)), whose
`Solution` arm prints it as `solutions (k) N`. So the table would print
`k = 2` beside `verdict Solution`. The `Ambiguity` arm computes its own
distinct count; `Open` prints 0; only `Solution` inherits the raw node count.

Fix: print `Verdict::k()` in the `Solution` arm, as the verdict *event*
already does. Then **build the witness** — a synthetic fixture that reaches
the mixed regime — because a regime that is *defined rather than measured* is
one nobody has ever seen a read-out from, and this is the milestone that
should change that. The fixture is a program with one discharged model and at
least one open state; the obligation machinery from S1d.2.4 makes that
expressible.

If [S1e.3.4](s1e.3.4_architecture.md) takes the seam fix, this task is
subsumed by it and becomes the fixture alone.

### Task T1e.3.1.3 — `CO-M3`: `UNBOUND` through the accessors ✅

`UNBOUND.tag()` returns `Tag::Fact` via the `>> 30 == 3` fallthrough, and
`UNBOUND.as_fact()` returns `Some(FactId(0x3FFF_FFFF))` — a `FactId` the
store can legitimately assign, since the capacity check allows ids up to
`CAPACITY − 1`
([`value.rs:65-71, 94-120`](../../../ein.rs/crates/ein-core/src/value.rs),
[`facts.rs:122-124`](../../../ein.rs/crates/ein-core/src/facts.rs)). Any
consumer calling `as_fact()` on a register value without first testing
`is_unbound()` silently gets a phantom fact.

The type's own test proves only that `pack()` cannot *produce* `UNBOUND` —
so the sentinel's safety rests on call-site discipline in `ein-infer`, not on
the type. Two steps:

1. **Audit the call sites.** Every `as_fact()` / `tag()` on a value that can
   come from a register. The review found no current misuse; confirm it, and
   record the count, because *"we checked all N sites"* ages better than
   *"no misuse was found"*.
2. **Harden or expose.** Make `as_fact()` reject the sentinel (returning
   `None`), or — at minimum, and this is the review's own floor — add the
   test `assert_eq!(Value::UNBOUND.as_fact(), None)`, which **currently
   fails**, so that the gap is visible in the gate rather than in a review.
   Prefer the first: the cost is a comparison on a path that is already
   branching, and `is_unbound()` remains for callers that want the
   distinction.

If rejecting changes a hot path's codegen measurably, that is a real
trade-off — measure it with the bench set before deciding, and record the
number either way.

### Task T1e.3.1.4 — `CO-M4`: two macro pipelines ✅

[`macros.rs:50-76, 197-240`](../../../ein.rs/crates/ein-ir/src/macros.rs)'s
`collect_macros` (first-declaration-wins, silently skips malformed forms) +
`expand_rule_clauses` carries a doc comment claiming it is *"what the loader
does, and therefore the shape the parity gate compares"*. The loader is
actually [`from_ir::ingest_macros`](../../../ein.rs/crates/ein-ir/src/from_ir.rs)
(duplicate = error, reserved = error) + `expand_pair` per rule.

So a reader trusting the comment models the wrong duplicate and reserved
semantics, and the non-loader consumer (the dump / golden path) gets
different macro registration than a load does. The parity gate that comment
refers to no longer exists, which makes the claim doubly stale.

Fix, in order of preference:

1. **Route the secondary consumer through the loader's ingestion** and delete
   the parallel pipeline. Check first what the dump path actually needs — if
   it wants a *lenient* read (dump something even when the program would not
   load), that is a legitimate difference and option 2 applies.
2. **Keep both, state the difference at both sites**: lenient-for-dump vs
   strict-for-load, with the two error semantics named. And fix the comment
   either way — it is wrong under both options.

A fixture is worth having regardless: a program with a duplicate macro
declaration, dumped and loaded, showing the two behaviours or the one.

### Task T1e.3.1.5 — `CO-M5`: module identity from the display string ✅

`Resolver::locate` canonicalizes the **display** string
(`std::fs::canonicalize(&display)`,
[`imports.rs:271-311`](../../../ein.rs/crates/ein-ir/src/imports.rs)), which
fails silently for the embedded root — identity falls back to
`<embedded>/name` — and for any transiently-unreadable path; `base_dir` for a
module's own nested imports is then `None` for embedded modules.

It works today because stdlib modules only import `std.*`. A stdlib module
using a file-relative import would resolve under the checkout and override
roots and fail **only under the embedded root** — that is, only in an
installed binary, and never in the test harness, which always sets
`$EIN_STDLIB` ([`stdlib.rs:183`](../../../ein.rs/crates/ein-ir/src/stdlib.rs)).

Two fixes, and taking both is cheap:

- **Forbid file-relative imports in stdlib modules**, as a load-time check
  with a readable message. This is the one that prevents the shape.
- **Test through the embedded source.** The harness has never loaded that
  way, which is why the difference is invisible; one test that resolves with
  `$EIN_STDLIB` unset and the checkout walk defeated would cover the third
  resolution tier the release binary actually uses.

The second is worth more than this finding: it is the only tier of the
three-tier resolution with no coverage at all, and
[EH-M2](s1e.3.5_error_handling.md) is another finding in the same tier.

### Task T1e.3.1.6 — `CO-M6`: `is_stalled` is not read-only ✅

[`saturator.rs:627-641`](../../../ein.rs/crates/ein-infer/src/saturator.rs)
runs a full `enqueue_pass(s, None)` without taking `self.delta` — so stale
delta facts can be re-seeded on a later `closure_step` (harmless via
seen-dedup, but wasted work) — and it **advances the tiebreaker**, documented
as intentional. So merely *asking* whether the saturator is stalled perturbs
subsequent enqueue ordering.

It is inert for current callers and a trap for the next one: an embedder
probing quiescence mid-drive is exactly the shape
[`docs/api/rust.md`](../../../docs/api/rust.md) invites, and that page says
nothing about it. Three options, in decreasing preference:

1. **Take the delta**, so the method is at least idempotent with respect to
   the queue, and keep the tiebreaker advance if it is genuinely required —
   with the requirement stated.
2. **Rename** to something that carries the effect (`probe_stall`,
   `stall_check_advancing`) so the hazard is in the name.
3. **Document it at the API level** — meaning the embedding page, not only
   the code comment, since the code comment already exists and is exactly
   what the finding is about.

Whichever is chosen, the embedding page gains the sentence. A hazard whose
only statement is a comment inside the crate is a hazard for everyone outside
it.

## Notes

Five of these six are one-to-two-hour fixes with a test each; `CO-M2`'s
fixture and `CO-M3`'s audit are the two that can run long. If the phase is
under pressure, `CO-M6` option 3 and `CO-M4` option 2 are the honest cheap
paths — both leave the code alone and fix the statement, which is what the
findings are actually about.

---

## Outcome

Taken 2026-08-30, after [S1e.3.4](s1e.3.4_architecture.md), which the phase's
ordering rule put first once the seam fix was chosen.

| | |
|---|---|
| **`CO-M1`** | **fixed** — `record_node` re-saturates a KB **written since its last saturation** and refuses to record one its own rules then refute. `Kb::written_since_saturation` is the invariant; the four record sites' comments say which conjuncts each establishes and which is now this function's. [D1](../p1e.1_open_questions/s1e.1.1_search_soundness_probes/d1_q4_which_route_reaches_the_site.md)'s **B** in effect and **A** in shape: the dirty bit is the *invariant*, not the optimisation it was selected as |
| **`CO-M2`** | **fixed** at S1e.3.4; the **witness** is this stage's — [`examples/features/13_mixed_solution_and_open.ein`](../../../examples/features/13_mixed_solution_and_open.ein), one discharged model beside one open state, `verdict.k = 1` where `stats.solution_nodes = 2`. `finalise` had defined that arm and said no corpus entry reached it |
| **`CO-M3`** | **fixed**, the review's preferred option: `as_fact` rejects the sentinel and `tag` refuses to answer about it in a debug build. The audit is **29** `as_fact` sites and **30** `tag` sites, and the assertion **did not fire once** across the suite — the discipline was real, and it was a convention |
| **`CO-M4`** | **fixed**, option 1: `macros::read_macros` is the one reading and `from_ir::ingest_macros` is the interning around it. The four non-loader consumers refuse what a load refuses, with the same sentences |
| **`CO-M5`** | **fixed**, both halves: a `std.*` module may import only `std.*` modules, refused at load in every tier; and the **embedded** tier has its first test, which is the tier a release binary uses and the one the harness could never reach |
| **`CO-M6`** | **fixed**, options 1 and 3: `is_stalled` takes the pending delta, so it is idempotent with respect to the queue, and the tiebreaker advance is stated as a **cost of asking** at the function and in [`docs/api/rust.md`](../../../docs/api/rust.md) § 3 — where an embedder meets it, which is the half a comment inside the crate cannot do |
| measured | `record_node` is called **2 153** times across the corpus's declared `solve` runs and **1 846 — 86 %** of those KBs are written since their last saturation; the dedup discards all but **233**, of which **86** reach the guard. **6** of the 86 refute, every one in `examples/ein-bugs/` |
| verdicts | **5 rows move**, all three fixtures: both `alive-empty-*` go `Solution k=1` → `Contradiction k=0` (the `-L` column, and the hand derivation), and `complete-records-stale -e` goes `Ambiguity k=2` → `Solution k=1` |
| cost | **0.98–1.03 ×** — `branching/06 -e` 0.98, `zebra -e` 1.03, `zebra2 -e` 0.98, i.e. inside the noise. It was 1.26–1.67 × before the two corrections below. No corpus entry crosses the `slow` threshold; `utils/corpus_cost.py` re-taken |
| goldens | `corpus_exits.txt` — the new fixture's cells, and `complete-records-stale :: solve` 0 → **1**. `corpus_shapes.md5` — the three rewritten fixtures, the new one, **7** `ir[expand]` refusals (`CO-M4`), and `saturate_count` wherever the guard actually saturates |
| gate | `./run_tests.sh` green — **780 tests**, exit 0, bench smoke unmoved |

### Five things the tasks did not predict

**1. The chosen fix's premise was false, and what saved the cost was a
different reordering.** [D1](../p1e.1_open_questions/s1e.1.1_search_soundness_probes/d1_q4_which_route_reaches_the_site.md)
picks **B** — a dirty bit — over **A** because it *"prices the common case at a
branch"*. Measured, the common case **is** dirty: **1 846 of the 2 153** KBs
that arrive at `record_node`, because the lookahead kill cache writes into
every fork `complete()` is asked about. So B and A cost the same, and the
naive fix was 1.26–1.67 ×. What made it free was putting the **dedup** before
the saturation: a node the dedup throws away has the same facts as the
representative it loses to, so it has the same closure, and that one was
already put through the guard. `branching/06 -e` calls `record_node` **1 221
times to keep 22** — a number already written at that function, for a
different reason. Corpus-wide the ordering is **1 846 re-saturations against
86**, and 1.27 × against 0.98 ×.

**2. The guard has three observable side channels and two had to be closed.**
A check that re-runs the rules is not free of narration:

- its **firings** would join the recorded trace, and they are re-derivations
  the dedup absorbs — `branching/03`'s `trace[answer]` went from 1 236 lines to
  2 360 before the trace was made to grow only when the *state* does;
- its **events** would advance the stream's `n` for every later event, which is
  [`MA-L4`](../p1e.4_low/s1e.4.8_maintainability.md)'s complaint about
  `sanity -y` arriving by a second route. The guard runs silent;
- its **`saturate_count`** stays, and moves goldens, because the engine really
  does saturate more and a counter that hid it would be S1e.3.4's finding
  again.

Which of the three to keep is not a style question, and the split was found by
running the goldens rather than by reasoning about them.

**2b. And the clean mark was in the wrong function.** `Saturator::saturate`
looked like where a fixpoint is reached; the fail-fast fork loop drives
`Saturator::step` directly, so a mark in `saturate` was never set for the forks
that loop saturates and **every** one of them looked dirty forever — a check
that always fires, which is a check that checks nothing. What found it is
`lattice_semantics::the_fail_fast_fork_is_verdict_and_proof_neutral`, which
compares the whole `MonotonicStats` between the flag's two settings and saw
`saturate_count` 2 against 1. The mark is in `step`'s fixpoint arm now, which
took the dirty share from an artefactual 96 % to a real 86 % before the dedup
and 37 % after it — and the cost from 1.08 × to 1.00 ×.

**3. A property was one regime too narrow, and the new fixture is what showed
it.** `summary_properties`' counter identity read *`verdict.k ==
stats.solution_nodes` (except `Open`)*. The real exception is *the program
declared an obligation*: `finalise` answers `Open` only when **every** recorded
node owes, and where one discharges and another does not the verdict is
`Solution` with the two counts still apart. Written against the regimes that
existed, and the mixed-regime fixture — built for `CO-M2` — failed it within a
minute of being added.

**4. The lenient macro reading had one argument and it does not survive.** The
case for `collect_macros` staying lenient is that a *dump* should render
something where a *load* would refuse. What it renders for a program with two
`(macro m …)` is the expansion of whichever came first — a rendering of a
program that cannot be run, offered without a word saying so. Seven
`ir[expand]` renderings now say `<refused>` instead, on the six
`examples/broken/load/` files that declare a duplicate or a reserved macro
name, and the dump path and the loader give the same sentence.

### What this stage did **not** do

- **The cheap re-saturation.** The guard runs a *fresh* saturator, which
  re-offers every match and lets the dedup absorb it; the incremental form is
  `Saturator::resume` seeded from the facts written since the mark, which needs
  the saturator snapshot that produced the fixpoint. Root has one
  (`Run::root_snapshot`); a fork does not, and giving every alive fork one is a
  `Snapshot` clone per entering. Not taken, because the dedup ordering already
  put the cost under 1.1 × — the note is at `record_node` so the next reader
  does not re-derive it.
- **An unsat core for `alive-empty-phase1`.** It reports `Contradiction` with
  **0** facts, and honestly: the refutation's premises are the kill cache's
  `(not h)`, whose provenance is `<lookahead-dies-immediately>` and cites no
  premises, so the source frontier really is empty. What a reader wants there —
  *the lookahead killed every candidate and totality then refuted* — is not a
  fact set, and saying it would be a read-out change rather than a core.

