# S1e.3.1 — Correctness (Medium)

**Phase:** [P1e.3](README.md) (Medium)
**Estimate:** 4 days
**Depends on:** [Q4](../p1e.1_open_questions/s1e.1.1_search_soundness_probes.md)
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

- Each of the six has a disposition, and each `accepted` has its reason at a
  `file:line` — not in this plan
  ([Q-M1e.1](../open_questions.md#q-m1e1--what-is-the-standard-of-proof-for-refuted)).
- `UNBOUND` cannot be mistaken for a real `FactId` through the public
  accessors, or a test asserts today's behaviour so the gap is visible.
- One macro-ingestion path, or two with the difference stated at both.
- No behaviour change reaches a golden without being named here first.

## Tasks

### Task T1e.3.1.1 — `CO-M1`: the inter-layer alive-∅ path

`phase2` calls `record_node(root)` when `compute_alive` comes back empty
([`solve.rs:1528-1551`](../../../ein.rs/crates/ein-infer/src/solve.rs)),
**without** the `has_contradiction` re-check that phase1 (`:1091`) and the
cascade (`:2131`) both perform.

[Q4](../p1e.1_open_questions/s1e.1.1_search_soundness_probes.md) has already
determined which of three this is — a real gap, a refuted one, or an
unreachable branch — and its outcome table says what this task does. Execute
that row; do not re-derive it. If the row is *fixed*, the change is one call
and the fixture from Q4 is the regression test; if it is *accepted*, the
argument goes beside `:1528` and states the two premises explicitly — the
writebacks are `(not h)` for `h ∉ root`, and root has not been re-saturated
on this path — with the second one's limit named, since that limit is the
finding.

### Task T1e.3.1.2 — `CO-M2`: the `Solution` arm's count

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

### Task T1e.3.1.3 — `CO-M3`: `UNBOUND` through the accessors

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

### Task T1e.3.1.4 — `CO-M4`: two macro pipelines

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

### Task T1e.3.1.5 — `CO-M5`: module identity from the display string

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

### Task T1e.3.1.6 — `CO-M6`: `is_stalled` is not read-only

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
