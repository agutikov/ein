"""Closed-relation inference — auto `(closed R)` before saturation.

A relation R is *closed* — the hypothesis generator must never
speculate facts of it — when **no rule can positively conclude an
R-fact**. Such a relation's extension is fixed by the puzzle's
given facts: no inference path reaches a new R-fact, so a
hypothesis about R could never be confirmed by saturation; it
would only bloat the search.

`is-a` (the inheritance forest) and `right-of` (the house row) of
the Zebra puzzle are closed by this test; `co-located` and
`next-to` — propagated by `symmetric` / `transitive` / `implies` /
`square-*` — are not.

This replaces hand-written `(closed R)` declarations. :func:`emit_closed`
runs once, before the engine's initial saturation, and writes a
`(closed R)` fact for every declared relation no rule positively
asserts. The facts land in the KB like any other — hypgen's
`_is_closed`, a KB dump, etc. all see them with no further wiring.

Assumption: every relation that genuinely needs *hypothesised*
facts is also rule-propagated — true for the Zebra family (the
solved relation `co-located` carries `symmetric` / `transitive`
closure). A puzzle with a free, rule-inert attribute that still
needs guessing would be mis-closed; revisit if one appears.
"""
from __future__ import annotations

from ein_bot.kb.entities import Fact, Layer
from ein_bot.kb.store import KnowledgeBase

from .compile import NestedPattern
from .engine import Engine


def producible_relations(kb: KnowledgeBase) -> frozenset[str]:
    """Relation names that some compiled rule positively asserts.

    Walks the engine's compiled (rule, activator) plans; a plan
    whose ``:assert`` template is ``(R …)`` — head not ``not`` —
    proves ``R`` is rule-derivable. T2 rules contribute once per
    activator, so a relation reachable only through an
    *un-activated* rule is correctly absent.
    """
    engine = Engine(kb)
    engine.compile_all()
    out: set[str] = set()
    for plan in engine.cache.values():
        template = plan.assert_template
        if isinstance(template, NestedPattern) and template.relation != "not":
            out.add(template.relation)
    return frozenset(out)


def emit_closed(kb: KnowledgeBase) -> list[str]:
    """Write a ``(closed R)`` fact for every declared relation no
    rule can positively conclude. Returns the newly-closed names.

    Idempotent: a relation already carrying ``(closed R)`` is left
    alone, so an authored declaration (should one survive) and a
    re-run both no-op. Run before the initial saturation so
    ``hypgen`` sees the facts.
    """
    producible = producible_relations(kb)
    already = {
        f.args[0]
        for f in kb._facts_by_relation.get("closed", ())
        if f.args and isinstance(f.args[0], str)
    }
    newly: list[str] = []
    for name, rel in kb.relations.items():
        # Only declared *domain* relations carry a signature; the
        # property / rule-name relations (`symmetric`, …) do not
        # and are never hypothesis targets anyway.
        if not rel.signature:
            continue
        if name in producible or name in already:
            continue
        fact = Fact(relation_name="closed", args=(name,), layer=Layer.ONTOLOGY)
        kb.add_fact(fact)
        kb._index_fact(fact)
        newly.append(name)
    return newly


__all__ = ["emit_closed", "producible_relations"]
