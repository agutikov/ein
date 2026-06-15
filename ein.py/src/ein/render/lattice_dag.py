"""Commitment-lattice / proof-DAG DOT renderer — S1.6.3.

The engine no longer produces an ordered search tree; it produces a
**set-indexed commitment lattice**. Each visited commitment is a
`frozenset` of hypothesis facts (a `CanonicalSetId`); commitments are
related by subset/cover (Apriori prefix-join) and collapse by
post-saturation `state_hash`. :func:`render_lattice` draws that lattice
as a DAG — the analog of the old tree view, but a partial order.

Inputs (either is accepted; a `Verdict` is unwrapped to its proof):

- :class:`~ein.inference.monotonic.lattice.LatticeProof` — richest:
  ``kb_index`` `SetNode`s give the full lattice (under
  ``store_lattice=True``), and ``dead_commitments`` enrich dead nodes
  with their ``unsat_core`` (tooltip) + ``learned_clause`` (the lifted
  no-good, drawn as a dashed back-edge).
- :class:`~ein.inference.monotonic.snapshot.LatticeSnapshotV1` —
  the permutation-invariant, `state_hash`-collapsed projection
  (S1.5b.31). **Order-stable** across ``lattice_order_seed`` → a
  reproducible diagram; the preferred input when determinism matters.

Two views (``view=``):

- ``"full"`` — every visited commitment / state (needs the
  `kb_index` / `nodes_by_state_hash`; falls back to the solution view
  with a note when those are empty).
- ``"solution"`` — the surviving solution commitment(s) + the dead
  siblings pruned at each layer; the small sub-DAG the trace embeds.

Verdict colours: alive grey, dead red, solution green.
"""
from __future__ import annotations

from dataclasses import dataclass
from typing import TYPE_CHECKING

from .dot_util import digraph_open, fact_label, hashed_id, multiline, quote

if TYPE_CHECKING:
    from ..inference.apriori import CanonicalSetId
    from ..inference.monotonic.lattice import DeadCommitment
    from ..kb.provenance import FactId

_VERDICT_STYLE = {
    # verdict → (border, fill)
    "alive":    ("#7f7f7f", "#eeeeee"),
    "dead":     ("#d62728", "#fdeaea"),
    "solution": ("#2ca02c", "#e8f6e8"),
}
_VERDICT_RANK = {"alive": 0, "dead": 1, "solution": 2}   # precedence on merge


# ── normalised lattice cell ────────────────────────────────────────

@dataclass(frozen=True)
class _Cell:
    rep:            CanonicalSetId          # representative commitment
    commitments:    tuple[CanonicalSetId, ...]
    verdict:        str
    layer:          int
    state_hash:     int | None
    unsat_core:     frozenset | None
    learned_clause: frozenset[FactId] | None


def _combine(verdicts: frozenset[str] | set[str]) -> str:
    return max(verdicts, key=lambda v: _VERDICT_RANK.get(v, -1)) if verdicts else "alive"


def _make_cell(commitments, verdict, state_hash, dc: DeadCommitment | None) -> _Cell:
    commits = tuple(commitments)
    rep = min(commits, key=lambda c: (len(c), c)) if commits else ()
    return _Cell(
        rep=rep, commitments=commits, verdict=verdict, layer=len(rep),
        state_hash=state_hash,
        unsat_core=(dc.unsat_core if dc else None),
        learned_clause=(dc.learned_clause if dc else None),
    )


def _cells(source, view: str) -> tuple[list[_Cell], bool]:
    """Normalise a proof / snapshot into cells. Returns (cells, full_ok).

    ``full_ok`` is False when ``view='full'`` was requested but the
    source had no stored lattice (so the solution view was used).
    """
    # Unwrap a Verdict to its proof.
    if not hasattr(source, "kb_index") and not hasattr(source, "nodes_by_state_hash"):
        proof = getattr(source, "proof", None)
        if proof is None:
            raise ValueError("render_lattice needs a LatticeProof or LatticeSnapshotV1")
        source = proof

    if hasattr(source, "nodes_by_state_hash"):        # LatticeSnapshotV1
        return _snapshot_cells(source, view)
    return _proof_cells(source, view)                 # LatticeProof


def _proof_cells(proof, view: str) -> tuple[list[_Cell], bool]:
    dead_by = {d.commitment: d for d in proof.dead_commitments}
    if view == "full" and proof.kb_index:
        cells = []
        for node in proof.kb_index.values():
            commits = tuple(sorted(node.labels))
            dc = next((dead_by[c] for c in commits if c in dead_by), None)
            cells.append(_make_cell(commits, node.verdict, node.state_hash, dc))
        return _dedup(cells), True
    # solution view, or full-fallback when no stored lattice.
    cells = [_make_cell((s.commitment,), "solution", None, None)
             for s in proof.solutions]
    cells += [_make_cell((d.commitment,), "dead", None, d)
              for d in proof.dead_commitments]
    cells += [_make_cell((a,), "alive", None, None) for a in proof.alive_at_end]
    return _dedup(cells), view != "full"


def _snapshot_cells(snap, view: str) -> tuple[list[_Cell], bool]:
    if view == "full" and snap.nodes_by_state_hash:
        cells = [
            _make_cell(tuple(sorted(labels)), _combine(verdicts), sh, None)
            for sh, labels, verdicts in snap.nodes_by_state_hash
        ]
        return _dedup(cells), True
    cells = [_make_cell((c,), "solution", None, None) for c in sorted(snap.solutions)]
    cells += [_make_cell((c,), "dead", None, None) for c in sorted(snap.deads)]
    cells += [_make_cell((c,), "alive", None, None) for c in sorted(snap.alive_at_end)]
    return _dedup(cells), view != "full"


def _dedup(cells: list[_Cell]) -> list[_Cell]:
    """Collapse cells sharing a representative; keep highest-precedence
    verdict and any enriching unsat-core / learned-clause."""
    by_rep: dict[CanonicalSetId, _Cell] = {}
    for c in cells:
        prev = by_rep.get(c.rep)
        if prev is None:
            by_rep[c.rep] = c
            continue
        hi = max((prev, c), key=lambda x: _VERDICT_RANK.get(x.verdict, -1))
        verdict = hi.verdict
        by_rep[c.rep] = _Cell(
            rep=c.rep,
            commitments=tuple(dict.fromkeys((*prev.commitments, *c.commitments))),
            verdict=verdict, layer=c.layer,
            state_hash=prev.state_hash if prev.state_hash is not None else c.state_hash,
            unsat_core=prev.unsat_core or c.unsat_core,
            learned_clause=prev.learned_clause or c.learned_clause,
        )
    # deterministic order: by (layer, representative).
    return sorted(by_rep.values(), key=lambda c: (c.layer, c.rep))


# ── labels / ids ───────────────────────────────────────────────────

def _commit_label(commitment: CanonicalSetId) -> str:
    if not commitment:
        return "∅"
    return "{" + ", ".join(fact_label(fid[0], fid[1]) for fid in commitment) + "}"


def _cell_id(cell: _Cell) -> str:
    if not cell.rep:
        return quote("root")
    seed = "|".join(fact_label(fid[0], fid[1]) for fid in cell.rep)
    return hashed_id("n_", seed, quoted=True)  # S1.7c.25 — shared hash tail


# ── the renderer ───────────────────────────────────────────────────

def render_lattice(source, *, view: str = "full", name: str = "lattice") -> str:
    """Render the commitment lattice as an inline ``dot`` block.

    ``source`` is a `LatticeProof`, a `LatticeSnapshotV1`, or a
    `Verdict` carrying one. ``view`` is ``"full"`` (every commitment)
    or ``"solution"`` (survivors + pruned siblings).
    """
    if view not in ("full", "solution"):
        raise ValueError(f"unknown lattice view: {view!r} (expected 'full' or 'solution')")
    cells, full_ok = _cells(source, view)

    lines = digraph_open(name, rankdir="LR", node_defaults='fontname="Inter", shape=box')
    if view == "full" and not full_ok:
        lines.append("  // no stored lattice (store_lattice=False) — "
                     "showing the solution frontier instead")

    root_cell = next((c for c in cells if not c.rep), None)
    non_root = [c for c in cells if c.rep]

    # Root node (layer 0). Reuse a root-cell's verdict colour if present.
    if root_cell is not None:
        border, fill = _VERDICT_STYLE.get(root_cell.verdict, _VERDICT_STYLE["alive"])
        lines.append(f'  {quote("root")} [label={multiline("∅ root", "(saturation)")}, '
                     f'style=filled, color="{border}", fillcolor="{fill}"];')
    else:
        lines.append(f'  {quote("root")} '
                     f'[label={multiline("root", "(saturation)")}, '
                     f'style=filled, color="#7f7f7f", fillcolor="#eeeeee"];')

    # Cell nodes.
    id_by_rep: dict[CanonicalSetId, str] = {(): quote("root")}
    for cell in non_root:
        nid = _cell_id(cell)
        id_by_rep[cell.rep] = nid
        border, fill = _VERDICT_STYLE.get(cell.verdict, _VERDICT_STYLE["alive"])
        extra = (len(cell.commitments) - 1)
        label_parts = [_commit_label(cell.rep)]
        if extra > 0:
            label_parts.append(f"(+{extra} ≡ same state)")
        attrs = [f"label={multiline(*label_parts)}", "style=filled",
                 f'color="{border}"', f'fillcolor="{fill}"']
        if cell.unsat_core:
            core = ", ".join(sorted(fact_label(f.relation_name, f.args)
                                    for f in cell.unsat_core))
            note = f"unsat-core: {core}"
            if cell.learned_clause:
                note += " | no-good: " + ", ".join(
                    sorted(fact_label(fid[0], fid[1]) for fid in cell.learned_clause))
            attrs.append(f"tooltip={quote(note)}")
        lines.append(f"  {nid} [{', '.join(attrs)}];")

    # Cover edges (subset, differ by one) + root fallbacks.
    reps_by_layer: dict[int, list[CanonicalSetId]] = {}
    for cell in non_root:
        reps_by_layer.setdefault(cell.layer, []).append(cell.rep)
    for cell in non_root:
        parents = [
            p for p in reps_by_layer.get(cell.layer - 1, [])
            if set(p) < set(cell.rep)
        ]
        if parents:
            for p in parents:
                lines.append(f"  {id_by_rep[p]} -> {_cell_id(cell)};")
        else:
            # No shown immediate parent — hang it off the root.
            style = "" if cell.layer == 1 else " [style=dotted]"
            lines.append(f"  {quote('root')} -> {_cell_id(cell)}{style};")

    # Dead nodes: dashed back-edge labelled with the lifted no-good.
    for cell in non_root:
        if cell.verdict == "dead" and cell.learned_clause:
            parents = [p for p in reps_by_layer.get(cell.layer - 1, [])
                       if set(p) < set(cell.rep)]
            target = id_by_rep[parents[0]] if parents else quote("root")
            lines.append(f'  {_cell_id(cell)} -> {target} '
                         f'[style=dashed, color="#d62728", '
                         f'constraint=false, label="no-good"];')

    # Rank alignment per layer.
    for layer in sorted(reps_by_layer):
        ids = " ".join(id_by_rep[r] for r in reps_by_layer[layer])
        lines.append(f"  {{rank=same; {ids}}}")

    lines.append("}")
    return "\n".join(lines)


__all__ = ["render_lattice"]
