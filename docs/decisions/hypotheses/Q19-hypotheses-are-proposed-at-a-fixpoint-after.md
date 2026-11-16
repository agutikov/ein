# Q19 — Hypotheses are proposed at a fixpoint, after saturation

Root is the program's facts saturated to quiescence by its rules, with nothing hypothesised, and H is proposed at
root's fixpoint, never while a saturation runs. A node's state is root's fixpoint with its commitment written in and
the saturation resumed to quiescence; what it owes is read off that fixpoint, and where a route chooses among the
payments of an owed instance, it does so at such a quiescent state.

**Why.** Propagation decides everything it can before anything is supposed, so every candidate is proposed against
what the rules have already derived, and a state is judged as the rules leave it, as a solver by hand deduces before
guessing.

**Held by.** `ein_infer::hypgen::generate_config_free`, which proposes H from root's saturated session;
`ein-infer/tests/explore.rs`: `every_mode_is_the_oracle`, which holds every route to the oracle, whose node is
root's fixpoint with the commitment written in and saturated;
[`solution_semantics.md`](../../kernel/inference/solution_semantics.md) § 1.3 to § 1.5.
