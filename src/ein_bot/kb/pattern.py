"""Pattern — structural view of `:match` / `:assert` clauses — S1.2.1 T1.2.1.5.

A `Pattern` lifts a rule's `:match` or `:assert` IR clause into a
typed object that knows three things:

- the variables bound by the pattern (``variables``),
- the relation names mentioned by literal head (``relation_names``),
- the type names mentioned via ``(instance ?_ T)`` shape (``type_names``).

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
from dataclasses import dataclass, field

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
    type_names: tuple[str, ...]
    has_instance_pattern: bool = field(default=False)

    @classmethod
    def from_ir(cls, expr: IRNode) -> Pattern:
        """Build a Pattern from a raw `:match` / `:assert` IR node."""
        vars_: list[str] = []
        rels: list[str] = []
        types: list[str] = []
        has_instance = False

        def walk(node: IRNode, position: str = "top") -> None:
            nonlocal has_instance
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
                if head_name == "instance":
                    has_instance = True
                    # (instance EntExpr TypeExpr) — recurse into both,
                    # and pluck the type name if it's a literal atom.
                    if len(node.args) >= 2 and isinstance(node.args[1], Atom):
                        t = node.args[1].name
                        if t not in types:
                            types.append(t)
                    for a in node.args:
                        walk(a, "instance-arg")
                    return
                if head_name in {"and", "or", "not", "neq"}:
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
            has_instance_pattern=has_instance,
        )

    def __iter__(self) -> Iterator[str]:
        """Iterate variable names (handy for `for v in pattern`)."""
        return iter(self.variables)


__all__ = ["Pattern"]
