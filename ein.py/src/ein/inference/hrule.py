"""Rule-driven hypothesis generation — S1.5.6b T1.5.6b.2.

A *hypothesis rule* (`hrule`) is declared by a ``(hrule …)`` form
in the ``(rules …)`` block — the same shape as ``(rule …)``, but
the loader routes it to ``kb.hrules`` rather than ``kb.rules``.
Its firing yields a *candidate hypothesis* instead of a derived
fact: the puzzle declares which hypotheses are worth trying,
rather than leaving it all to ``hypgen``'s blind combinatorial
enumerator.

A hrule may be **generic** — parameterised, e.g.
``(hrule guess (?rel ?type) …)`` — exactly like a generic
``(rule …)``. Its activators live in the ``(query …)`` block under
``:hrules`` keywords (*not* in ``(ontology …)``: an hrule
activator steers the search, it is not puzzle state). A
``:hrules (NAME (a b) (c d) …)`` keyword binds ``NAME``'s
parameters once per ``(a b)`` argument tuple. A parameter-less
hrule needs no activator.

Because hrules live outside ``kb.rules``, the engine's
``compile_all`` never compiles them — the **saturator and the
one-step lookahead never see them**. ``hypgen`` is the sole
consumer: each ``:match`` over the KB yields a binding, and
``build_fact`` over the ``:assert`` template becomes a candidate
``Fact`` routed through the ordinary ``_apply_filters`` pipeline.
"""
from __future__ import annotations

from collections.abc import Iterator

from ein.ir.types import Atom, SForm
from ein.kb.entities import Fact
from ein.kb.store import KnowledgeBase

from . import match
from .compile import NestedPattern, compile_rule
from .firing import build_fact


def _hrule_activators(kb: KnowledgeBase) -> dict[str, list[tuple[str, ...]]]:
    """Activator argument-tuples per hrule name, read from the
    ``(query … :hrules (NAME (a b) …))`` keywords.

    A `:hrules` value is ``(NAME (args…) (args…) …)`` — its head is
    the hrule name; each remaining item is one argument tuple,
    binding that hrule's parameters once.
    """
    out: dict[str, list[tuple[str, ...]]] = {}
    q = kb.query
    if q is None:
        return out
    for kp in q.kw_pairs:
        key = getattr(kp, "key", None)
        if key is None or getattr(key, "name", None) != "hrules":
            continue
        val = kp.value
        if not isinstance(val, SForm) or not isinstance(val.head, Atom):
            continue
        for item in val.args:
            if not isinstance(item, SForm):
                continue
            argtuple = tuple(
                p.name for p in (item.head, *item.args)
                if isinstance(p, Atom)
            )
            out.setdefault(val.head.name, []).append(argtuple)
    return out


class Hrules:
    """Runs the `(hrule …)` rules to produce candidate hypotheses.

    Compile the plans once (``Hrules(kb)``); call :meth:`candidates`
    per generation pass. A parameter-less hrule yields one plan; a
    generic hrule yields one plan per ``:hrules`` activator.
    """

    def __init__(self, kb: KnowledgeBase) -> None:
        activators = _hrule_activators(kb)
        plans = []
        for h in kb.hrules.values():
            if not h.params:
                # Parameter-less hrule — scans KB structure directly.
                plans.append(compile_rule(h, None))
                continue
            # Generic hrule — one plan per `:hrules` activator; the
            # activator's args bind the hrule's parameters. With no
            # matching activator a generic hrule contributes nothing.
            for argtuple in activators.get(h.name, ()):
                act = Fact(relation_name=h.name, args=argtuple)
                plans.append(compile_rule(h, act))
        self._plans = tuple(plans)

    def candidates(self, kb: KnowledgeBase) -> Iterator[Fact]:
        """Yield one candidate ``Fact`` per `hrule` match against `kb`.

        The fact is *not* written to the KB — it is a hypothesis,
        handed back to ``hypgen`` for filtering and commitment-set
        entering.
        """
        for plan in self._plans:
            template = plan.assert_template
            if not isinstance(template, NestedPattern):
                continue
            for bindings, _premises in match.run(plan, kb):
                try:
                    yield build_fact(template, bindings)
                except (KeyError, TypeError):
                    # Defensive: a malformed assert template yields
                    # no candidate rather than crashing generation.
                    continue


__all__ = ["Hrules"]
