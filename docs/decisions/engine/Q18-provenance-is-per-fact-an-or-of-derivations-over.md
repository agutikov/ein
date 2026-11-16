# Q18 — Provenance is per fact, an OR of derivations over premises

Every fact a KB holds records how it came to be there. A derivation record says what kind it is, a given (with its
`:source` where the program wrote one), a rule's firing or a commitment, and for a firing its rule, its activator,
the positive premises it read in plan order, its bindings in bind order and the `(absent …)` queries that had to
fail for it. A fact is an OR over its recorded derivations, each an AND over its premises: the first derivation is
its primary, the derivations that arrive after it are kept as alternatives up to a cap, sorted by premise count, and
the arrival with the smallest closure is kept beside them. The whole derivation graph of a fact is recovered by
walking its premises, and a dead node's core is read off that graph.

**Why.** An answer is only as explainable as what was recorded when it was derived; recording at the grain of a fact
and its premises is what lets a trace, a `:why` text and a core all be read off one structure.

**Held by.** `ein_core::prov::Prov` and `ProvKind`; `ein_core::Kb::record_justification`, `primary`,
`alternatives`, and `ein_core::kb::MAX_ALT_JUSTIFICATIONS`; `ein-infer/tests/explain_semantics.rs`:
`a_rebuild_preserves_the_alternative_justifications`,
`a_full_list_keeps_the_smallest_closure_and_the_pair_report_one_core`. What may be recorded is a
`(config …)` flag, `:record-alternative-justifications`, documented in
[configuration.md](../../kernel/configuration.md).
