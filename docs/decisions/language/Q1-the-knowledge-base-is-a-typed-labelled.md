# Q1 — The knowledge base is a typed labelled hypergraph with provenance

The knowledge base Ein reasons over is a hypergraph whose nodes are of five kinds, object, type, relation, rule and
fact, and whose one kind of edge is labelled by a relation. A fact is one edge over any number of arguments, each a
name or a fact, so a relation of three arguments or more is one edge and is not built from binary ones. Every fact
carries its provenance: written by a file, with its `:source` if it has one, or derived by a rule from the premises
the firing names. Names are never merged: `eq` is name identity, and the equality classes the store keeps are
written by nothing and read by no matcher.

**Why.** A constraint over many objects is one fact, and an explanation is a walk over provenance: a core, a trace
and `refuted by` name the facts written and the rules that derived the rest.

**Held by.** The store, `ein_core::kb::Kb`, and its provenance, `ein_core::prov::ProvKind`;
`ein-infer/tests/naf_semantics.rs`: `matching_does_not_resolve_equality_classes`; `ein-core/src/kb.rs`'s unit test
`equality_classes_are_copied_and_stay_inert`. The graph's own description: `01_kb.md`.
