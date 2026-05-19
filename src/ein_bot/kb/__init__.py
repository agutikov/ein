"""ein-bot knowledge base — typed entity views over the canonical graph.

Designed in `plans/m1_core_graph_reasoning/p1.2_typed_hypergraph/`.

The graph is the data model (objects, types, and relations are all
nodes; non-binary facts are Levi-bipartite hyperedge nodes); the
entity classes exposed here are *typed views* over that store. See
the framing note in
``plans/m1_core_graph_reasoning/p1.2_typed_hypergraph/s1.2.1_data_model.md``.

Public API:

- :class:`KnowledgeBase` — the registry of entities and cross-refs.
- :class:`Type`, :class:`Instance`, :class:`Relation`, :class:`Rule`,
  :class:`Fact` — entity dataclasses.
- :class:`Layer` — three knowledge populations (ontology / fact /
  reasoning).
- :class:`Pattern` — structural view of a `:match` / `:assert` clause.
- :class:`Query` — a `(query …)` block.
- :func:`load` — build a KB from parsed IR forms (also exposed as
  ``KnowledgeBase.from_ir(forms)``).
"""
from .entities import Fact, Instance, Layer, Relation, Rule, Type
from .from_ir import KBLoadError, load
from .pattern import Pattern
from .provenance import DerivationDAG, Provenance
from .store import EqClasses, KnowledgeBase, Query
from .views import (
    FactView,
    instance_name,
    logical_instances,
    logical_types,
    type_name,
)

__all__ = [
    "DerivationDAG",
    "EqClasses",
    "Fact",
    "FactView",
    "Instance",
    "KBLoadError",
    "KnowledgeBase",
    "Layer",
    "Pattern",
    "Provenance",
    "Query",
    "Relation",
    "Rule",
    "Type",
    "instance_name",
    "load",
    "logical_instances",
    "logical_types",
    "type_name",
]
