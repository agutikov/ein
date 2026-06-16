"""Serialisation helpers + the timeline-jsonl mixin (split out of state_dump.py).

Leaf module: the engine-agnostic Fact / Firing / KnowledgeBase → ein-source /
JSON renderers, plus `_TimelineMixin` (the shared `00_timeline.jsonl` writer),
used by both `MonotonicDumper` (state_dump.py) and `LatticeDumper`
(_lattice_dump.py).
"""
from __future__ import annotations

import json
import time
from collections.abc import Callable
from dataclasses import fields
from typing import Any

from ein.ir.types import Atom, Int, Keyword, KwPair, SForm, String
from ein.kb.entities import Fact, Layer
from ein.kb.store import KnowledgeBase

# ── Serialisation helpers ────────────────────────────────────
#
# Migrated 2026-05-29 out of ``inference.tree.state_dump``. The
# renderers + JSON serialisers are engine-agnostic: they project a
# :class:`Fact` / :class:`Firing` / :class:`KnowledgeBase` into ein
# source text or machine-parseable JSON, used by both
# :class:`MonotonicDumper` and :class:`LatticeDumper` below.


def _arg_to_node(arg: Any) -> Any:
    """Lower a Fact arg to an IR node."""
    if isinstance(arg, Fact):
        return _fact_to_sform(arg, with_kwargs=False)
    if isinstance(arg, int):
        return Int(value=arg)
    return Atom(name=str(arg))


def _fact_to_sform(fact: Fact, *, with_kwargs: bool = True) -> SForm:
    """Render a Fact as ``(rel arg0 arg1 ... :source "..." :rule "..." :layer ...)``.

    Nested-Fact args are recursively lowered without their kwargs
    (so they read as bare ``(rel args...)`` inside the outer form).
    """
    args: list[Any] = [_arg_to_node(a) for a in fact.args]
    if with_kwargs:
        prov = fact.provenance
        if prov is not None:
            if prov.kind == "source" and prov.source:
                args.append(KwPair(
                    key=Keyword(name="source"),
                    value=String(value=prov.source),
                ))
            elif prov.kind == "rule" and prov.rule:
                args.append(KwPair(
                    key=Keyword(name="rule"),
                    value=String(value=prov.rule),
                ))
            elif prov.kind == "hypothesis":
                args.append(KwPair(
                    key=Keyword(name="hypothesis"),
                    value=Int(value=prov.branch or 0),
                ))
        # Layer kw-pair only when provenance-derivation wouldn't recover
        # it on reload (P1.7c S1.7c.1): a `:rule` fact derives REASONING
        # and a `:source` fact derives FACT for free, so `:layer` is
        # emitted only for the residue — a hypothesis/unannotated
        # REASONING fact, an unsourced FACT, a sourced ONTOLOGY fact.
        # Keeps the flat dump → reload layer round-trip exact (the
        # wrapped loader silently dropped this annotation).
        has_rule = prov is not None and prov.kind == "rule" and bool(prov.rule)
        has_source = (
            prov is not None and prov.kind == "source" and bool(prov.source)
        )
        derived = (
            Layer.REASONING if has_rule
            else Layer.FACT if has_source
            else Layer.ONTOLOGY
        )
        if fact.layer is not derived:
            args.append(KwPair(
                key=Keyword(name="layer"),
                value=Atom(name=fact.layer.value),
            ))
    return SForm(head=Atom(name=fact.relation_name), args=tuple(args))


def _kb_to_ein_text(kb: KnowledgeBase) -> str:
    """Render a KB as a **flat** sequence of ein forms (P1.7c S1.7c.3).

    The block wrappers are gone — each fact is a top-level form. Layer is
    carried per fact: ONTOLOGY facts (the ``(relation ...)``, ``(is-a ...)``,
    ``(bijective ...)`` schema) carry no annotation; FACT facts carry
    ``:source``; REASONING facts (everything the saturator derived,
    including ``(not ...)``) carry ``:rule`` / ``:using`` — or an explicit
    ``:layer`` when provenance alone wouldn't re-derive the layer (see
    :func:`_fact_to_sform`). ONTOLOGY facts are emitted first purely for
    readability; the round-trip is order-independent.
    """
    from ein.ir.dump import dump_canonical

    ont: list[SForm] = []
    rest: list[SForm] = []
    for f in kb.facts:
        (ont if f.layer is Layer.ONTOLOGY else rest).append(_fact_to_sform(f))
    return dump_canonical([*ont, *rest])


def _firing_to_dict(firing: Any) -> dict[str, Any]:
    """Serialise a Firing for JSONL output.

    Stripped-down: rule name, activator, derived fact id,
    redundancy flag, and the premise fact-ids. Bindings flattened
    to ``{str: str}`` (Fact bindings dropped — chase them via
    premises).
    """
    bindings_clean: dict[str, str] = {}
    for k, v in firing.bindings.items():
        if isinstance(v, Fact):
            bindings_clean[k] = f"<fact:{v.relation_name}>"
        else:
            bindings_clean[k] = str(v)
    return {
        "rule": firing.rule,
        "activator": list(firing.activator),
        "bindings": bindings_clean,
        "redundant": firing.redundant,
        # S1.8.A13: `firing.derived` is now a tuple; this debug dump shows the
        # primary conclusion (the full fan-out is in the slice diagram + the KB
        # dump). `a` is a nested Fact (octagon) iff it has `.relation_name`.
        "derived": {
            "relation": firing.derived[0].relation_name,
            "args": [
                {"relation": a.relation_name,
                 "args": list(map(str, a.args))}
                if hasattr(a, "relation_name") else str(a)
                for a in firing.derived[0].args
            ],
        },
        "premises": [
            {"relation": p.relation_name,
             "args": list(map(str, p.args))}
            for p in firing.premises
        ],
    }


def _fact_summary(fact: Fact) -> dict[str, Any]:
    """Recursive Fact → JSON-friendly dict.

    Nested-Fact args (e.g. inside ``(not (color-loc Green House-3))``)
    render as nested ``{"relation": ..., "args": [...]}`` dicts so the
    output is machine-parseable. Atoms / ints / strings stringify.
    """
    return {
        "relation": fact.relation_name,
        "args": [
            _fact_summary(a) if isinstance(a, Fact) else str(a)
            for a in fact.args
        ],
    }


class _TimelineMixin:
    """Shared ``00_timeline.jsonl`` + ``summary.json`` plumbing.

    :class:`MonotonicDumper` and :class:`LatticeDumper` both stream a
    JSONL timeline and write a final ``summary.json``; they carried
    verbatim copies of ``_emit_timeline`` / ``summary`` / ``close``
    (F-ENG-11). The only behavioural knob is the timeline record's
    ``json.dumps`` ``default=`` serialiser — Monotonic emits with the
    strict default (``None``), Lattice passes ``str`` because its
    records can carry non-JSON-native payloads (FactId tuples). That
    knob is :attr:`_timeline_json_default`.

    Hosts must provide the ``_timeline_fp`` / ``_timeline_seq`` /
    ``started_at`` / ``out_dir`` attributes (both dumpers do, as
    dataclass fields).
    """

    # default= for timeline records; None ⇒ json.dumps' strict default.
    _timeline_json_default: Callable[[Any], Any] | None = None

    def close(self) -> None:
        """Close the timeline file without emitting ``summary.json``.

        Called by abort paths (``BudgetExceededError``) so the timeline
        is flushed + closed when no final summary will be written;
        normal exits close it via :meth:`summary`. Idempotent.
        """
        if self._timeline_fp is not None:
            self._timeline_fp.close()
            self._timeline_fp = None

    def summary(self, verdict: Any, stats: Any) -> None:
        verdict_kind = type(verdict).__name__ if verdict is not None else None
        elapsed = time.time() - self.started_at
        try:
            stats_dict = {
                f.name: getattr(stats, f.name)
                for f in fields(stats)
            }
        except TypeError:
            stats_dict = {"repr": repr(stats)}

        if self.out_dir is not None:
            (self.out_dir / "summary.json").write_text(
                json.dumps({
                    "verdict": verdict_kind,
                    "elapsed_seconds": round(elapsed, 3),
                    "stats": stats_dict,
                }, indent=2, sort_keys=True, default=str),
            )
        self._emit_timeline(
            "summary",
            verdict=verdict_kind,
            elapsed_seconds=round(elapsed, 3),
        )
        if self._timeline_fp is not None:
            self._timeline_fp.close()
            self._timeline_fp = None

    def _emit_timeline(self, event: str, **fields_: Any) -> None:
        if self._timeline_fp is None:
            return
        rec = {
            "seq": self._timeline_seq,
            "ts_ms": round((time.time() - self.started_at) * 1000, 3),
            "event": event,
            **fields_,
        }
        self._timeline_fp.write(
            json.dumps(rec, default=self._timeline_json_default) + "\n",
        )
        self._timeline_fp.flush()
        self._timeline_seq += 1


