"""Rule-driven hypothesis generation — S1.5.6b T1.5.6b.2.

A *hypothesis rule* (`hrule`) is declared by a ``(hrule …)`` form
in the ``(rules …)`` block — the same shape as ``(rule …)``, but
the loader routes it to ``kb.hrules`` rather than ``kb.rules``.
Its firing yields a *candidate hypothesis* instead of a derived
fact: the puzzle declares which hypotheses are worth trying,
rather than leaving it all to ``hypgen``'s blind combinatorial
enumerator.

Because hrules live outside ``kb.rules``, the engine's
``compile_all`` never compiles them — the **saturator and the
one-step lookahead never see them** (no skip logic needed).
``hypgen`` is the sole consumer: each ``:match`` over the KB
yields a binding, and ``build_fact`` over the ``:assert`` template
becomes a candidate ``Fact`` routed through the ordinary
``_apply_filters`` pipeline — so a rule-generated candidate is
still negated-fact / lookahead filtered and counted.

This is the generation-side counterpart of S1.5.8's
``domain-elimination``: that rule *derives* a forced value, an
`hrule` *proposes* a speculative one.
"""
from __future__ import annotations

from collections.abc import Iterator

from ein_bot.kb.entities import Fact
from ein_bot.kb.store import KnowledgeBase

from . import match
from .compile import NestedPattern, compile_rule
from .firing import build_fact


class Hrules:
    """Runs the `(hrule …)` rules to produce candidate hypotheses.

    Compile the plans once (``Hrules(kb)``); call :meth:`candidates`
    per generation pass. When ``kb.hrules`` is empty, no plan is
    compiled and :meth:`candidates` yields nothing.
    """

    def __init__(self, kb: KnowledgeBase) -> None:
        # One plan per hrule. M1 hrules are parameter-less — they
        # scan KB structure; `compile_rule` with `activator=None`
        # binds no params (a parameterised hrule would simply not
        # match, degrading cleanly).
        self._plans = tuple(
            compile_rule(h, None) for h in kb.hrules.values()
        )

    def candidates(self, kb: KnowledgeBase) -> Iterator[Fact]:
        """Yield one candidate ``Fact`` per `hrule` match against `kb`.

        The fact is *not* written to the KB — it is a hypothesis,
        handed back to ``hypgen`` for filtering and ``try_branch``.
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
