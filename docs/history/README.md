# docs/history — shipped milestones, kept as record

What a milestone leaves behind once it has shipped: the design contracts other
documents and source comments still cite, the measurements nothing can re-take,
the decisions that were made against a number, and the questions that outlived
the work. **Not plans.** A plan describes work that has not happened; these
describe work that has, and the plan trees they came from are in git history.

The rule that decides what lives here rather than in
[`plans/`](../../plans/README.md): a document belongs here when it is still
*read* — as a specification, as evidence, or as the reason something is the way
it is — and belongs in git history when the only thing it can still do is say
what somebody intended to build.

| milestone | ran | record |
|---|---|---|
| **M1a — the Rust port (ein.rs)** | 2026-08-17 → 2026-08-23 | [`m1a_rust/`](m1a_rust/README.md) — eleven phases and 53 stages as one record, plus the eleven design contracts, six measurement documents, the divergence ledger, twenty-three questions and the oracle ledger |
| **M1c — External validation** | 2026-08-23 → 2026-08-24 | [`m1c_external_validation/`](m1c_external_validation/README.md) — one phase and five stages as one record, plus the stdlib census (the evidence) and seven questions, two still open. **`:expect`, `ein test`, 45 programs, and 38 of 73 unfired stdlib rules → 0** |
| **M1d — From saturation to satisfiability** | 2026-08-21 → 2026-08-27 | [`m1d_satisfiability/`](m1d_satisfiability/README.md) — four phases and eighteen stages as one record, plus fifteen documents: four re-takable censuses, seven arguments and specifications, the intent note and seven questions. **A program can state a requirement, a state can say what it *owes*, and a traversal that branches on requirements reaches the same 32 models in 86 enterings where the lattice needs 17 204 592** |

They differ in one way worth knowing before reading any of them: **M1a's
instruments are gone and M1c's and M1d's are not.** Every number under
`m1a_rust/` is a record — `ein.py`, the conformance tiers and eleven `utils/`
scripts left the tree with the engine they measured. Every number under
`m1c_external_validation/` and `m1d_satisfiability/` can be re-taken today; one
of M1c's is re-taken by `cargo test` on every commit, and M1d ships four census
scripts under `utils/` for the same reason.

**M1d is also the one that was closed with work left in it**, deliberately and
at the user's direction, so its record carries a section the other two do not
need: § Eight measurements with no owner. Read it before re-opening anything
there — two of the eight are about *shipped* engine behaviour rather than about
the milestone's plan.

**M1** (core graph reasoning, shipped 2026-06-17) predates this directory: what
survived its plan tree went to [`docs/kernel/inference/`](../kernel/inference/README.md)
and [`plans/followups/`](../../plans/followups/README.md) at P1.22, and the
rest is in git history.

Distinct from [`docs/kernel/`](../kernel/README.md), which is the *current*
specification and is checked by the gate; from [`docs/api/`](../api/README.md),
which is how to drive the engine today; and from `plans/`, which is what has
not been built yet.
