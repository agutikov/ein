"""LatticeDumper — the complete per-hypothesis lattice dump (split out of
state_dump.py).

Every commitment tested at every layer, with the firings each one emitted,
survivors and casualties alike — the exhaustive audit trail (run a `solve`
exhaustively with `dumper=LatticeDumper(out_dir=…)`). See `lattice_dump.md`.
"""
from __future__ import annotations

import json
import time
from dataclasses import dataclass, field, fields
from pathlib import Path
from typing import IO, Any

from ein.inference.monotonic._serialise import (
    _fact_summary,
    _firing_to_dict,
    _kb_to_ein_text,
    _TimelineMixin,
)
from ein.kb.store import KnowledgeBase

# ── LatticeDumper helpers ────────────────────────────────────


def _commitment_slug(commitment: tuple) -> str:
    """Render a CanonicalSetId (tuple of ``(relation_name, args)``
    FactIds) as a filesystem-safe slug.

    Empty commitment renders as ``root``; size-1 renders as the
    bare FactId slug; multi-element renders as ``<slug1>+<slug2>``.
    Within each FactId, args are joined with ``_``; ``_`` in
    identifiers becomes ``-`` so the field separator stays
    unambiguous.
    """
    if not commitment:
        return "root"

    def _field(s: str) -> str:
        return str(s).lower().replace("_", "-")

    def _factid_slug(fid: tuple) -> str:
        rn, args = fid
        return "_".join([_field(rn), *(_field(a) for a in args)])

    return "+".join(_factid_slug(fid) for fid in commitment)


def _factid_json(fid: tuple) -> dict[str, Any]:
    """Render a FactId ``(relation_name, args)`` as JSON-friendly."""
    rn, args = fid
    return {
        "relation": rn,
        "args": [str(a) for a in args],
    }


def _commitment_json(commitment: tuple) -> list[dict[str, Any]]:
    """Render a CanonicalSetId as a list of FactId JSON dicts."""
    return [_factid_json(fid) for fid in commitment]


@dataclass
class LatticeDumper(_TimelineMixin):
    """Filesystem-snapshotting hooks attached to a :func:`solve` run
    with ``store_lattice=True``.

    Sibling of :class:`MonotonicDumper`; shares the same
    lifecycle-hook pattern but adds the lattice-proof sections
    (per-commitment ``enterings/`` folders + the ``store_lattice``
    ``kb_index/`` + ``proof_summary.json``).

    Dump layout::

        out_dir/
        ├── 00_root_initial.ein   ← root before any hypothesis
        ├── 00_timeline.jsonl     ← lifecycle event log
        ├── layers/
        │   └── layer_NN/
        │       ├── pre.ein       ← root.kb at start of layer NN
        │       └── post.ein      ← root.kb at end of layer NN
        ├── enterings/            ← per-hypothesis emission tracking
        │   └── layer_NN/
        │       └── <C-slug>/
        │           ├── commitment.json
        │           ├── outcome.txt          ← alive | dead-pre | dead-post | solution
        │           ├── unconditional_facts.jsonl   (non-dead-pre)
        │           ├── firings.jsonl               (non-dead-pre)
        │           ├── kb.ein                      (solution only)
        │           ├── unsat_core.jsonl            (dead-pre / dead-post)
        │           └── learned_clause.json         (dead-pre / dead-post)
        ├── kb_index/             ← under store_lattice=True
        │   └── layer_NN/
        │       └── kb_<i>/
        │           ├── state_hash.txt    ← hex of state_hash field
        │           ├── canonical_set.json
        │           ├── labels.json
        │           └── verdict.txt
        ├── proof_summary.json    ← top-level proof index
        └── summary.json          ← cumulative stats

    ``out_dir=None`` skips every filesystem write — hooks still
    fire (so the solver loop's call sites stay uniform) but
    produce no on-disk artefacts. Subfolders are created lazily
    on first per-section write so the layout reflects what
    actually happened (e.g. no empty ``kb_index/`` under
    ``store_lattice=False``).

    User-facing documentation:
    ``docs/kernel/inference/lattice_dump.md``.
    """

    out_dir: Path | None = None
    started_at: float = field(default_factory=time.time)
    _timeline_fp: IO[str] | None = field(
        default=None, init=False, repr=False,
    )
    _timeline_seq: int = field(default=0, init=False)

    # Lattice timeline records can carry non-JSON-native payloads
    # (FactId tuples), so serialise them with ``default=str`` — the one
    # knob that distinguishes this dumper's timeline from Monotonic's.
    _timeline_json_default = str

    def __post_init__(self) -> None:
        if self.out_dir is None:
            return
        self.out_dir.mkdir(parents=True, exist_ok=True)
        (self.out_dir / "layers").mkdir(exist_ok=True)
        self._timeline_fp = (
            self.out_dir / "00_timeline.jsonl"
        ).open("w")

    # ── Path helpers ─────────────────────────────────────────

    def _layer_dir(self, layer: int) -> Path:
        """``layers/layer_NN/`` — created lazily on first call."""
        assert self.out_dir is not None
        p = self.out_dir / "layers" / f"layer_{layer:02d}"
        p.mkdir(parents=True, exist_ok=True)
        return p

    def _entering_dir(self, layer: int, commitment: tuple) -> Path:
        """``enterings/layer_NN/<slug>/`` — created lazily."""
        assert self.out_dir is not None
        slug = _commitment_slug(commitment)
        p = (
            self.out_dir / "enterings" / f"layer_{layer:02d}" / slug
        )
        p.mkdir(parents=True, exist_ok=True)
        return p

    # ── Lifecycle hooks ──────────────────────────────────────

    def root_initial(self, kb: KnowledgeBase) -> None:
        """Called once after Phase 1's initial saturation."""
        if self.out_dir is not None and kb is not None:
            (self.out_dir / "00_root_initial.ein").write_text(
                _kb_to_ein_text(kb),
            )
        self._emit_timeline(
            "root_initial",
            facts=len(kb.facts) if kb is not None else 0,
        )

    def layer_start(
        self, layer: int, kb: KnowledgeBase, alive_size: int,
    ) -> None:
        if self.out_dir is not None and kb is not None:
            (self._layer_dir(layer) / "pre.ein").write_text(
                _kb_to_ein_text(kb),
            )
        self._emit_timeline(
            "layer_start", layer=layer, alive_size=alive_size,
        )

    def entering(
        self,
        layer: int,
        commitment: tuple,
        result: Any,
        *,
        outcome: str = "alive",
        facts_merged: int = 0,
        nogood_emitted: bool = False,
        nogood_subsumed: bool = False,
    ) -> None:
        """One ``try_commitment_set`` outcome.

        ``outcome`` is one of ``"alive"`` / ``"dead-pre"`` /
        ``"dead-post"`` / ``"solution"`` — the engine's per-
        commitment classification. Writes a per-commitment
        folder under ``enterings/layer_NN/<C-slug>/`` with the
        artefacts relevant to the outcome:

        - ``commitment.json`` + ``outcome.txt`` — always.
        - ``unconditional_facts.jsonl`` + ``firings.jsonl`` —
          for non-``dead-pre`` outcomes (the fork saturated;
          ``result.unconditional_facts`` / ``result.firings``
          reflect that saturation).
        - ``kb.ein`` — for ``"solution"`` only (the saturated
          fork's full kb).
        - ``unsat_core.jsonl`` + ``learned_clause.json`` —
          for ``"dead-pre"`` / ``"dead-post"`` (the
          contradiction witnesses + the learned negative
          clause).

        The timeline carries one record per call with counts.
        """
        rec: dict[str, Any] = {
            "layer": layer,
            "outcome": outcome,
            "commitment": (
                _commitment_json(commitment)
                if commitment is not None else []
            ),
            "facts_merged": facts_merged,
            "nogood_emitted": nogood_emitted,
            "nogood_subsumed": nogood_subsumed,
        }
        if result is not None:
            rec.update({
                "kind": result.kind,
                "firings": len(result.firings),
                "unconditional_count": len(result.unconditional_facts),
                "unsat_core_size": len(result.unsat_core),
            })
        self._emit_timeline("entering", **rec)

        if self.out_dir is None or result is None or commitment is None:
            return

        folder = self._entering_dir(layer, commitment)
        (folder / "commitment.json").write_text(
            json.dumps(_commitment_json(commitment), indent=2),
        )
        (folder / "outcome.txt").write_text(outcome)

        if result.kind != "dead-pre":
            # Both alive and dead-post have saturated forks
            # whose unconditional_facts + firings are
            # meaningful per-hypothesis emissions.
            with (folder / "unconditional_facts.jsonl").open("w") as fp:
                for f in result.unconditional_facts:
                    fp.write(json.dumps(_fact_summary(f)) + "\n")
            with (folder / "firings.jsonl").open("w") as fp:
                for firing in result.firings:
                    fp.write(json.dumps(_firing_to_dict(firing)) + "\n")

        if outcome == "solution":
            # Full saturated kb of the satisfying fork.
            (folder / "kb.ein").write_text(_kb_to_ein_text(result.kb))

        if outcome in ("dead-pre", "dead-post"):
            with (folder / "unsat_core.jsonl").open("w") as fp:
                for f in result.unsat_core:
                    fp.write(json.dumps(_fact_summary(f)) + "\n")
            (folder / "learned_clause.json").write_text(
                json.dumps(
                    [
                        _factid_json(fid)
                        for fid in sorted(
                            commitment,
                            key=lambda f: (
                                f[0], tuple(map(str, f[1])),
                            ),
                        )
                    ],
                    indent=2,
                    default=str,
                ),
            )

    def layer_end(
        self,
        layer: int,
        kb: KnowledgeBase,
        alive_size: int,
        survived_count: int,
    ) -> None:
        if self.out_dir is not None and kb is not None:
            (self._layer_dir(layer) / "post.ein").write_text(
                _kb_to_ein_text(kb),
            )
        self._emit_timeline(
            "layer_end",
            layer=layer,
            facts=len(kb.facts) if kb is not None else 0,
            alive_size=alive_size,
            survived_count=survived_count,
        )

    def proof_summary(self, proof: Any) -> None:
        """Top-level proof index — written from ``_finish`` when
        ``verdict.proof`` is non-None. Materialises the
        ``kb_index/layer_NN/kb_<i>/`` folder hierarchy with
        per-layer ordered ids (i = 0..n within each layer,
        deterministic via state_hash sort) when
        ``proof.kb_index`` is populated."""
        if self.out_dir is None or proof is None:
            return

        # Per-layer ordered ids: group by node.layer, sort
        # within layer by state_hash for determinism, assign
        # kb_0 ... kb_n.
        kb_id_by_state_hash: dict[int, tuple[int, int]] = {}
        if proof.kb_index:
            (self.out_dir / "kb_index").mkdir(exist_ok=True)
            by_layer: dict[int, list[Any]] = {}
            for node in proof.kb_index.values():
                by_layer.setdefault(node.layer, []).append(node)
            for layer_n, nodes in by_layer.items():
                nodes_sorted = sorted(nodes, key=lambda n: n.state_hash)
                layer_dir = (
                    self.out_dir / "kb_index"
                    / f"layer_{layer_n:02d}"
                )
                layer_dir.mkdir(parents=True, exist_ok=True)
                for idx, node in enumerate(nodes_sorted):
                    kb_id_by_state_hash[node.state_hash] = (
                        layer_n, idx,
                    )
                    folder = layer_dir / f"kb_{idx}"
                    folder.mkdir(exist_ok=True)
                    (folder / "state_hash.txt").write_text(
                        f"{node.state_hash & 0xFFFFFFFFFFFFFFFF:016x}",
                    )
                    (folder / "canonical_set.json").write_text(
                        json.dumps(
                            _commitment_json(node.canonical_set),
                            indent=2,
                        ),
                    )
                    (folder / "labels.json").write_text(
                        json.dumps(
                            [_commitment_json(c) for c in node.labels],
                            indent=2,
                        ),
                    )
                    (folder / "verdict.txt").write_text(node.verdict)

        def _kb_id_label(node: Any) -> str:
            layer_n, idx = kb_id_by_state_hash[node.state_hash]
            return f"layer_{layer_n:02d}/kb_{idx}"

        # Top-level index.
        summary = {
            "solutions": [
                {
                    "slug": _commitment_slug(s.commitment),
                    "layer": s.layer,
                    "commitment": _commitment_json(s.commitment),
                    "path": (
                        f"enterings/layer_{s.layer:02d}/"
                        f"{_commitment_slug(s.commitment)}"
                    ),
                }
                for s in proof.solutions
            ],
            "dead_commitments": [
                {
                    "slug": _commitment_slug(d.commitment),
                    "layer": d.layer,
                    "kind": d.kind,
                    "commitment": _commitment_json(d.commitment),
                    "path": (
                        f"enterings/layer_{d.layer:02d}/"
                        f"{_commitment_slug(d.commitment)}"
                    ),
                }
                for d in proof.dead_commitments
            ],
            "kb_index": [
                {
                    "kb_id": _kb_id_label(node),
                    "state_hash_hex": (
                        f"{node.state_hash & 0xFFFFFFFFFFFFFFFF:016x}"
                    ),
                    "canonical_set": _commitment_json(
                        node.canonical_set,
                    ),
                    "labels": [
                        _commitment_json(c) for c in node.labels
                    ],
                    "verdict": node.verdict,
                    "layer": node.layer,
                }
                for node in proof.kb_index.values()
            ],
            "alive_at_end": [
                _commitment_json(c) for c in proof.alive_at_end
            ],
            "learned_nogoods_count": len(proof.learned_nogoods),
            "stats": {
                f.name: getattr(proof.stats, f.name)
                for f in fields(proof.stats)
            },
        }
        (self.out_dir / "proof_summary.json").write_text(
            json.dumps(summary, indent=2, sort_keys=True, default=str),
        )
        self._emit_timeline(
            "proof_summary",
            solutions=len(proof.solutions),
            dead_commitments=len(proof.dead_commitments),
            kb_index_size=len(proof.kb_index),
        )


__all__ = ["LatticeDumper"]
