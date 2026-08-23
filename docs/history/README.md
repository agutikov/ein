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

**M1** (core graph reasoning, shipped 2026-06-17) predates this directory: what
survived its plan tree went to [`docs/kernel/inference/`](../kernel/inference/README.md)
and [`plans/followups/`](../../plans/followups/README.md) at P1.22, and the
rest is in git history.

Distinct from [`docs/kernel/`](../kernel/README.md), which is the *current*
specification and is checked by the gate; from [`docs/api/`](../api/README.md),
which is how to drive the engine today; and from `plans/`, which is what has
not been built yet.
