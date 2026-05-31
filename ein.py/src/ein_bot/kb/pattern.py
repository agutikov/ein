"""Pattern — structural view of `:match` / `:assert` clauses — S1.2.1 T1.2.1.5.

A `Pattern` lifts a rule's `:match` or `:assert` IR clause into a
typed object that knows three things:

- the variables bound by the pattern (``variables``),
- the relation names mentioned by literal head (``relation_names`` —
  this now includes ``instance``, an ordinary relation since S1.7.6),
- ``type_names`` — vestigial; it was plucked from the old
  ``(instance ?_ T)`` special-case and is now always empty.

This is **structural** — no matching semantics, no binding, no
backtracking. The pattern matcher lives in P1.3 and consumes these
fields when planning rule firings.

The `expr` field keeps the raw IRNode so P1.3 can walk it; everything
else is a pre-computed view.

This module deliberately stays decoupled from the rest of `kb` — it
operates on IR nodes only, so it can be unit-tested without a full
KnowledgeBase.
"""
from __future__ import annotations

from collections.abc import Iterator
from dataclasses import dataclass

from ein_bot.ir.types import Atom, IRNode, KwPair, SForm, Var


@dataclass(frozen=True)
class Pattern:
    """The structural view of a `:match` / `:assert` clause.

    Identity is by `expr` only — two patterns over structurally
    identical IR are equal (loc is excluded from IR-node equality).
    """
    expr: IRNode
    variables: tuple[str, ...]
    relation_names: tuple[str, ...]
    # `type_names` is vestigial since S1.7.6: it used to be plucked from
    # `(instance ?_ T)` patterns, but `instance` is now an ordinary
    # relation (no special-case), so this stays empty until/unless a
    # relation-signature-based type extractor repopulates it. Kept for
    # the `Rule.types` / `_rules_by_type` / `Type.rules` API surface.
    type_names: tuple[str, ...]

    @classmethod
    def from_ir(cls, expr: IRNode) -> Pattern:
        """Build a Pattern from a raw `:match` / `:assert` IR node."""
        vars_: list[str] = []
        rels: list[str] = []
        types: list[str] = []

        def walk(node: IRNode, position: str = "top") -> None:
            if isinstance(node, Var):
                if node.name not in vars_:
                    vars_.append(node.name)
                return
            if isinstance(node, SForm):
                # Walk the head first — when it's a Var like
                # `(?rel ?a ?b)`, we must record `?rel` as bound.
                if isinstance(node.head, Var):
                    walk(node.head, "head-var")
                head_name = node.head.name if isinstance(node.head, Atom) else None
                # `instance` is no longer special (S1.7.6): it falls
                # through to the generic-relation handler below, which
                # registers it in `relation_names` and walks its args.
                if head_name in {"and", "or", "not", "neq", "eq"}:
                    # Kernel structural primitives (`and`, `or`, `not`)
                    # and the built-in predicates from
                    # `inference/predicates.py` (`eq`, `neq` — Q33). Walk
                    # the args to collect inner variables and relation
                    # names, but do NOT register the wrapper head as a
                    # relation name itself — it's not a fact relation.
                    for a in node.args:
                        walk(a, head_name)
                    return
                if head_name == "=":
                    for a in node.args:
                        walk(a, "=")
                    return
                # generic SForm with head Atom = relation name (or kw-pair carrier)
                if head_name is not None and head_name not in {"@empty", "@params"}:
                    if head_name not in rels:
                        rels.append(head_name)
                for a in node.args:
                    walk(a, head_name or "")
                return
            if isinstance(node, KwPair):
                # :where (...) carries a side condition; walk its value
                walk(node.value, "kw")
                return
            # Atom / Wildcard / literals: nothing to extract
            return

        walk(expr)
        return cls(
            expr=expr,
            variables=tuple(vars_),
            relation_names=tuple(rels),
            type_names=tuple(types),
        )

    def __iter__(self) -> Iterator[str]:
        """Iterate variable names (handy for `for v in pattern`)."""
        return iter(self.variables)


__all__ = ["Pattern"]
