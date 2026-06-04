"""Derivation-slice + KB-snapshot DOT renderers — S1.6.2.

The trace (S1.6.4) does not want the whole 25-cell KB at each step. It
wants, per hypothesis, **just what that hypothesis touched** — a
*provenance cone*: the hypothesis fact(s), the KB facts the firings
consumed, the rules that fired, and the facts derived. That cone is
:func:`render_slice`, embedded in each trace section.

Three renderers:

- :func:`render_slice` — the per-hypothesis cone (primary; T1.6.2.1).
  Fed a `DeadCommitment`'s firing chain + ``contradiction=`` it becomes
  the refuted-branch slice terminating in ``⊥`` (T1.6.2.1b).
- :func:`render_state` — the whole-KB snapshot (T1.6.2.2), flag-gated
  in the trace behind ``--full-kb-snapshots``; delegates to the unified
  KB renderer.
- :func:`render_solution` — the solved-state view for the closing
  section (T1.6.2.4).

Both state renderers accept ``since=<kb_before>`` to thicken the facts a
step added (T1.6.2.3). All output is inline ``dot`` — no SVG (the
Python layer emits DOT; rasterising is a shell concern).

Colour key: hypothesis/seed facts red, derived facts bold, negative
(eliminated-alternative) facts grey, firing rule-nodes coloured by rule
(shared palette), the ``⊥`` refutation node red.
"""
from __future__ import annotations

from typing import TYPE_CHECKING

from ..inference.why import render_why
from .dot_util import digraph_open, fact_label, hashed_id, multiline, quote
from .palette import hash_color

if TYPE_CHECKING:
    from collections.abc import Iterable

    from ..inference.apriori import CanonicalSetId
    from ..inference.firing import Firing
    from ..kb.entities import Fact
    from ..kb.provenance import FactId
    from ..kb.store import KnowledgeBase

_SEED_COLOUR = "#d62728"      # red — hypothesis / seed facts, ⊥
_NEG_COLOUR = "#7f7f7f"       # grey — negative (eliminated-alternative) facts


# ── fact identity / labels ─────────────────────────────────────────

def _is_fact(x: object) -> bool:
    """Duck-type a Fact-shaped object (has relation_name + args)."""
    return hasattr(x, "relation_name") and hasattr(x, "args")


def _arg_str(a: object) -> str:
    if _is_fact(a):
        return _key(a.relation_name, a.args)            # type: ignore[attr-defined]
    return str(a)


def _key(relation_name: str, args: tuple) -> str:
    """Content key for a fact / fact-id — stable across runs."""
    return relation_name + "|" + ",".join(_arg_str(a) for a in args)


def _node_id(key: str) -> str:
    # S1.7c.25 — shared hash tail; the recursive `key` (built by `_key` /
    # `_arg_str`, which descends into nested Facts) stays LOCAL — that
    # recursion is load-bearing for these node ids, so it is NOT merged
    # into the flat `dot_util.fact_key`.
    return hashed_id("f_", key, quoted=True)


def _fact_keys(kb: KnowledgeBase) -> set[str]:
    return {_key(f.relation_name, f.args) for f in kb.facts}


# ── the provenance-cone renderer (primary) ─────────────────────────

def render_slice(
    commitment: CanonicalSetId,
    firings: Iterable[Firing],
    kb: KnowledgeBase | None,
    *,
    name: str = "slice",
    contradiction: tuple[frozenset[Fact], frozenset[FactId]] | None = None,
    since: KnowledgeBase | None = None,
) -> str:
    """Render one hypothesis's provenance cone as an inline ``dot`` block.

    ``commitment`` is the hypothesis fact-id set (drawn red). ``firings``
    are the surviving-path firings: each becomes a rule-node between its
    premises and its derived fact (labelled with the rendered ``:why``
    when ``kb`` carries the rule). Only facts in the cone appear — never
    the whole KB.

    Pass ``contradiction=(unsat_core, learned_clause)`` (a
    `DeadCommitment`'s data) to render the refuted-branch slice: the
    core facts point at a ``⊥`` node tagged with the lifted no-good.
    ``since`` thickens facts not present in that prior KB.
    """
    firings = list(firings)
    rules = getattr(kb, "rules", {}) or {}
    since_keys = _fact_keys(since) if since is not None else None

    seed_keys = {_key(fid[0], fid[1]) for fid in (commitment or ())}
    derived_keys = {_key(d.relation_name, d.args)
                    for f in firings for d in f.derived}

    node_decls: dict[str, str] = {}     # key → full decl line
    edges: list[str] = []
    firing_nodes: list[str] = []

    def _touch(relation_name: str, args: tuple) -> str:
        key = _key(relation_name, args)
        nid = _node_id(key)
        if key not in node_decls:
            negative = relation_name == "not"
            seed = key in seed_keys
            derived = key in derived_keys
            attrs = [f"label={quote(fact_label(relation_name, args))}", "shape=box"]
            if seed:
                attrs += [f'color="{_SEED_COLOUR}"', f'fontcolor="{_SEED_COLOUR}"',
                          'style="rounded,filled"', 'fillcolor="#fdeaea"']
            elif negative:
                attrs += [f'color="{_NEG_COLOUR}"', f'fontcolor="{_NEG_COLOUR}"',
                          "style=rounded"]
            else:
                attrs.append("style=rounded")
            if derived:
                attrs.append("penwidth=2")           # bold — newly derived
            if since_keys is not None and key not in since_keys:
                attrs.append("penwidth=3")           # transition highlight
            node_decls[key] = f"  {nid} [{', '.join(attrs)}];"
        return nid

    # Seeds first (so they declare red even if also a premise).
    for fid in (commitment or ()):
        _touch(fid[0], fid[1])

    # Each firing → a rule-node between premises and the derived fact.
    for idx, firing in enumerate(firings):
        fnode = quote(f"fire{idx}_{firing.rule}")
        colour = hash_color(firing.rule)
        rule = rules.get(firing.rule)
        why = render_why(rule.why, firing.bindings) if (rule and rule.why) else ""
        style = "rounded,dashed" if firing.redundant else "rounded,bold"
        firing_nodes.append(
            f'  {fnode} [shape=box, style="{style}", color="{colour}", '
            f"fontcolor=\"{colour}\", label={multiline(firing.rule, why)}];"
        )
        for p in firing.premises:
            pid = _touch(p.relation_name, p.args)
            edges.append(f'  {pid} -> {fnode} [color="{colour}"];')
        # One application fans out to each derived fact (S1.8.A13).
        for d in firing.derived:
            did = _touch(d.relation_name, d.args)
            edges.append(f'  {fnode} -> {did} [color="{colour}", style=bold];')

    # Refuted branch → ⊥ tied to the unsat-core, tagged with the no-good.
    if contradiction is not None:
        unsat_core, learned_clause = contradiction
        bottom = quote("⊥")
        firing_nodes.append(
            f'  {bottom} [shape=doublecircle, color="{_SEED_COLOUR}", '
            f'fontcolor="{_SEED_COLOUR}", label="⊥"];'
        )
        for f in unsat_core:
            cid = _touch(f.relation_name, f.args)
            edges.append(f'  {cid} -> {bottom} [color="{_SEED_COLOUR}"];')
        if learned_clause:
            ng = quote("learned-nogood")
            clause = " ∧ ".join(sorted(fact_label(fid[0], fid[1]) for fid in learned_clause))
            firing_nodes.append(
                f"  {ng} [shape=note, label={multiline('learned no-good', clause)}];"
            )
            edges.append(f'  {bottom} -> {ng} [style=dashed, label="lifts to"];')

    lines = digraph_open(name, rankdir="LR", node_defaults='fontname="Inter"')
    lines += list(node_decls.values())
    lines += firing_nodes
    lines += edges
    lines.append("}")
    return "\n".join(lines)


# ── whole-KB snapshot + solution (delegate to the unified renderer) ─

def render_state(
    kb: KnowledgeBase,
    *,
    layer_filter: Iterable | None = None,
    since: KnowledgeBase | None = None,
    name: str = "state",
) -> str:
    """The complete KB graph at a moment (flag-gated `--full-kb-snapshots`).

    Delegates to the unified KB renderer. ``layer_filter`` selects a
    layer subset (e.g. fact-layer only); ``since`` thickens the facts
    absent from that prior KB ("this step added E").
    """
    from ..kb.entities import Layer
    from ..kb.render import to_dot
    layers = (
        tuple(layer_filter) if layer_filter is not None
        else (Layer.ONTOLOGY, Layer.FACT, Layer.REASONING)
    )
    return to_dot(kb, layers=layers, since=since, name=name)


def render_solution(kb: KnowledgeBase, *, name: str = "solution") -> str:
    """The solved-state graph — the trace's closing answer view."""
    from ..kb.render import to_dot
    return to_dot(kb, name=name)


__all__ = ["render_slice", "render_solution", "render_state"]
