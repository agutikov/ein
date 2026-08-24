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

The two differ in one way worth knowing before reading either: **M1a's
instruments are gone and M1c's are not.** Every number under `m1a_rust/` is a
record — `ein.py`, the conformance tiers and eleven `utils/` scripts left the
tree with the engine they measured. Every number under
`m1c_external_validation/` can be re-taken today, and one of them is re-taken
by `cargo test` on every commit.

**M1** (core graph reasoning, shipped 2026-06-17) predates this directory: what
survived its plan tree went to [`docs/kernel/inference/`](../kernel/inference/README.md)
and [`plans/followups/`](../../plans/followups/README.md) at P1.22, and the
rest is in git history.

Distinct from [`docs/kernel/`](../kernel/README.md), which is the *current*
specification and is checked by the gate; from [`docs/api/`](../api/README.md),
which is how to drive the engine today; and from `plans/`, which is what has
not been built yet.
