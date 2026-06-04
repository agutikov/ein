"""ein-lang pattern macros — load-time AST rewriting (P1.8 S1.5.9).

A `(macro NAME (?p1 ?p2 …) BODY)` declaration defines an alias: an
invocation `(NAME a1 a2 …)` appearing in a rule clause is rewritten —
*before the compiler sees it* — into a fresh copy of BODY with
`?p1 → a1`, `?p2 → a2`, … substituted in. The compiler stays ignorant
of the macro; new syntactic sugar is added by writing more `(macro …)`
declarations rather than extending ``inference/compile.py``.

This module is pure IR-level rewriting: it depends only on
:mod:`ein_bot.ir.types` (no ``kb`` / ``inference`` imports), keeping the
``kb → ir`` layering intact. The loader (:mod:`ein_bot.kb.from_ir`)
builds the ``{name: Macro}`` registry in a pre-pass and applies
:func:`expand_macros` to each rule clause during the rules pass.

Bodies may invoke other macros (handled by re-expansion); a macro that
expands to itself is caught by the recursion-depth cap
(:data:`_MAX_EXPANSION_DEPTH` — Q-S1.5.9.3). Hygiene (Q-S1.5.9.4) is not
an issue for M1's ``forall`` / ``open`` macros — they introduce no fresh
vars — so substitution is a plain template walk; revisit if a user macro
gensyms.
"""
from __future__ import annotations

from dataclasses import dataclass, field

from .types import Atom, IRNode, KwPair, Loc, SForm, Var

# A macro that expands to itself would recurse forever; cap and reject.
_MAX_EXPANSION_DEPTH = 50


class MacroError(ValueError):
    """A macro invocation is malformed — arity mismatch or runaway recursion."""


@dataclass(frozen=True)
class Macro:
    """A `(macro NAME (params…) BODY)` declaration — an AST-rewrite alias.

    ``params`` are the body's substitution variables in declaration order;
    ``body`` is the template :class:`~ein_bot.ir.types.IRNode` an invocation
    expands into.
    """
    name: str
    params: tuple[str, ...]
    body: IRNode
    loc: Loc | None = field(default=None, compare=False, hash=False, repr=False)


def _substitute(template: IRNode, subst: dict[str, IRNode]) -> IRNode:
    """Return ``template`` with each :class:`Var` named in ``subst`` replaced
    by its node (which may itself be a complex :class:`SForm`).

    A parameter may appear in *head* position (``(?R ?a ?b)`` — a
    relation-poly body); it is substituted there too when the replacement
    is an :class:`Atom` / :class:`Var` (a head must stay atom-shaped). M1's
    ``forall`` / ``open`` bodies never use a var head, so this arm is
    forward-compat for the ``imply`` / ``converse`` family (S1.8.A7).
    """
    if isinstance(template, Var):
        return subst.get(template.name, template)
    if isinstance(template, SForm):
        head = template.head
        if isinstance(head, Var) and head.name in subst:
            sub = subst[head.name]
            if isinstance(sub, (Atom, Var)):
                head = sub
        new_args = tuple(_substitute(a, subst) for a in template.args)
        return SForm(head=head, args=new_args, loc=template.loc)
    if isinstance(template, KwPair):
        return KwPair(
            key=template.key,
            value=_substitute(template.value, subst),
            loc=template.loc,
        )
    # Atom / Wildcard / Int / Range / String / Keyword are leaves.
    return template


def expand_macros(
    node: IRNode, macros: dict[str, Macro], _depth: int = 0,
) -> IRNode:
    """Recursively rewrite every macro invocation reachable from ``node``.

    An :class:`SForm` whose head atom names a macro is replaced by the
    macro's substituted body, then *re-expanded* (a body may invoke other
    macros — handled inside-out). Non-macro forms are walked into so a
    macro invocation nested anywhere in a clause is found. Returns ``node``
    unchanged (structurally) when no macro applies, so it is safe to run
    over every clause whether or not it uses macros.

    Raises :class:`MacroError` on an arity mismatch or when expansion
    exceeds :data:`_MAX_EXPANSION_DEPTH` (a self-expanding macro).
    """
    if _depth > _MAX_EXPANSION_DEPTH:
        raise MacroError(
            f"macro expansion exceeded depth {_MAX_EXPANSION_DEPTH} — "
            f"a macro likely expands to itself"
        )
    if isinstance(node, SForm):
        head = node.head
        if isinstance(head, Atom) and head.name in macros:
            m = macros[head.name]
            # Positional args bind the params; trailing kw-pairs are
            # metadata (dropped downstream by the compiler — Q32) and do
            # not count toward arity.
            pos_args = tuple(a for a in node.args if not isinstance(a, KwPair))
            if len(pos_args) != len(m.params):
                raise MacroError(
                    f"macro {m.name}/{len(m.params)} invoked with "
                    f"{len(pos_args)} args at {node.loc}"
                )
            subst = dict(zip(m.params, pos_args, strict=True))  # lengths checked above
            expanded = _substitute(m.body, subst)
            return expand_macros(expanded, macros, _depth + 1)
        new_args = tuple(expand_macros(a, macros, _depth) for a in node.args)
        return SForm(head=head, args=new_args, loc=node.loc)
    if isinstance(node, KwPair):
        return KwPair(
            key=node.key,
            value=expand_macros(node.value, macros, _depth),
            loc=node.loc,
        )
    return node


__all__ = ["Macro", "MacroError", "expand_macros"]
