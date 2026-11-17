# Q1 — The knowledge base is a hypergraph of atoms and facts, with provenance

The knowledge base Ein reasons over is a hypergraph, held as a graph with two sorts of node, the atom and the fact
([Q-M1f.379](Q-M1f.379-an-object-is-a-node-of-the-knowledge-base-graph.md)). A fact is one directed hyperedge over
its relation and any number of arguments, each an atom or a fact, so a fact of three arguments or more is one fact
and is not built from facts of two. *Relation* and *rule* are parts an atom plays and no sort of node: the atom
under a fact's first arrow ([Q-M1f.381](Q-M1f.381-a-relation-is-the-atom-under-a-facts-first-arrow.md)), and the
atom that names a rule. No node is a type
([Q-M1f.372](Q-M1f.372-ein-has-no-types-a-declarations-slot-names-are.md)). Every fact carries its provenance:
written by a file, with its `:source` if it has one, or derived by a rule from the premises the firing names. Atoms
are never merged: `eq` is identity of names, and the equality classes the store keeps are written by nothing and
read by no matcher.

**Why.** A constraint over many atoms is one fact, and an explanation is a walk over provenance: a core, a trace
and `refuted by` name the facts written and the rules that derived the rest.

**Held by.** The store, `ein_core::kb::Kb`, and its provenance, `ein_core::prov::ProvKind`;
`ein-infer/tests/naf_semantics.rs`: `matching_does_not_resolve_equality_classes`; `ein-core/src/kb.rs`'s unit test
`equality_classes_are_copied_and_stay_inert`. The graph's own description:
[the knowledge base](../../ein/reasoning/01-static-ir/01-knowledge-base.md), and for a reader who writes programs
[objects](../../ein/user/03-syntax-and-semantics/01-objects.md),
[relations](../../ein/user/03-syntax-and-semantics/02-relations.md) and
[facts](../../ein/user/03-syntax-and-semantics/03-facts.md).
