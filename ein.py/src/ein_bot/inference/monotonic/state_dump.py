"""MonotonicDumper — per-layer filesystem snapshots of the monotonic solve.

Mirrors :class:`ein_bot.inference.tree.state_dump.StateDumper`'s shape
but with no per-set storage (monotonic doesn't carry SetNodes, so
there's nothing to dump per set). The output layout is:

::

    dump/<puzzle>-<ts>/
       00_root_initial.ein           ← root before any enterings
       00_timeline.jsonl             ← chronological event log
       layers/
           layer_01_pre.ein          ← root.kb at layer 1 start
           layer_01_post.ein         ← root.kb at layer 1 end (after all merges)
           layer_02_pre.ein
           layer_02_post.ein
           ...
       summary.json                  ← final stats + verdict

Reading ``00_timeline.jsonl`` linearly tells the full search story;
the per-layer ``.ein`` snapshots show what facts the engine had
accumulated at root at each layer boundary. No
``set/<canonical-slug>/`` folders — that's a lattice-engine feature
(S1.5b.23).

The dumper's six lifecycle hooks fire from :func:`monotonic_solve`
when the caller passes ``dumper=MonotonicDumper(out_dir=…)``.
``dumper=None`` is a no-op for every hook site; the backbone
behaviour is identical to the pre-S1.5b.7 path.
"""
from __future__ import annotations

import json
import time
from dataclasses import dataclass, field, fields
from pathlib import Path
from typing import IO, Any

from ein_bot.ir.types import Atom, Int, Keyword, KwPair, SForm, String
from ein_bot.kb.entities import Fact, Layer
from ein_bot.kb.store import KnowledgeBase

# ── Serialisation helpers ────────────────────────────────────
#
# Migrated 2026-05-29 out of ``inference.tree.state_dump`` as part
# of the tree-solver removal. The renderers + JSON serialisers are
# engine-agnostic: they project a :class:`Fact` / :class:`Firing` /
# :class:`KnowledgeBase` into ein source text or machine-parseable
# JSON, used by both :class:`MonotonicDumper` and
# :class:`LatticeDumper` below.


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
        # Layer kw-pair on FACT/REASONING facts so the reader
        # can tell them apart inside a single (facts ...) block.
        # ONTOLOGY facts don't carry one — they live in the
        # ontology block.
        if fact.layer is Layer.REASONING:
            args.append(KwPair(
                key=Keyword(name="layer"),
                value=Atom(name="reasoning"),
            ))
    return SForm(head=Atom(name=fact.relation_name), args=tuple(args))


def _kb_to_ein_text(kb: KnowledgeBase) -> str:
    """Render a KB as ``(ontology ...) (facts ...)`` ein text.

    Splits by layer:
    - ONTOLOGY-layer facts (the ``(relation ...)``, ``(is-a ...)``,
      ``(bijective ...)``, etc.) land in the ontology block.
    - FACT-layer facts (the puzzle's authored conditions) and
      REASONING-layer facts (everything the saturator derived,
      including ``(not ...)``) land in the facts block, with
      ``:layer reasoning`` annotated on the derived ones.
    """
    from ein_bot.ir.dump import dump_canonical

    ont_args: list[SForm] = []
    fact_args: list[SForm] = []
    for f in kb.facts:
        sform = _fact_to_sform(f)
        if f.layer is Layer.ONTOLOGY:
            ont_args.append(sform)
        else:
            fact_args.append(sform)

    forms: list[SForm] = []
    if ont_args:
        forms.append(SForm(
            head=Atom(name="ontology"), args=tuple(ont_args),
        ))
    if fact_args:
        forms.append(SForm(
            head=Atom(name="facts"), args=tuple(fact_args),
        ))
    return dump_canonical(forms)


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
        "derived": {
            "relation": firing.derived.relation_name,
            "args": [
                {"relation": a.relation_name,
                 "args": list(map(str, a.args))}
                if isinstance(a, type(firing.derived)) else str(a)
                for a in firing.derived.args
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


@dataclass
class MonotonicDumper:
    """Filesystem-snapshotting hooks attached to a :func:`monotonic_solve` run.

    ``out_dir=None`` skips every filesystem write — the hooks still
    fire but produce no on-disk artefacts. Useful for subclasses
    that consume the lifecycle stream (e.g. ``bench_monotonic.py``'s
    verbose-mode progress emitter) without paying for the dump.
    """

    out_dir: Path | None = None
    started_at: float = field(default_factory=time.time)
    _timeline_fp: IO[str] | None = field(
        default=None, init=False, repr=False,
    )
    _timeline_seq: int = field(default=0, init=False)

    def __post_init__(self) -> None:
        if self.out_dir is None:
            return
        self.out_dir.mkdir(parents=True, exist_ok=True)
        (self.out_dir / "layers").mkdir(exist_ok=True)
        self._timeline_fp = (
            self.out_dir / "00_timeline.jsonl"
        ).open("w")

    # ── Lifecycle hooks ──────────────────────────────────────────

    def root_initial(self, kb: KnowledgeBase) -> None:
        if self.out_dir is not None:
            (self.out_dir / "00_root_initial.ein").write_text(
                _kb_to_ein_text(kb),
            )
        self._emit_timeline("root_initial", facts=len(kb.facts))

    def layer_start(
        self, layer: int, kb: KnowledgeBase, alive_size: int,
    ) -> None:
        """Beginning of a layer's candidate loop.

        Writes ``layer_NN_pre.ein`` (the root kb as the layer
        sees it) and records the timeline event. Deviation from
        the spec's surface: the spec had ``layer_end`` writing
        the next layer's pre file, but that produces a stray
        ``layer_(N+1)_pre.ein`` when layer N is the last (no
        layer N+1 ever runs). Owning the pre write here removes
        the off-by-one and keeps ``layer_NN_pre`` and
        ``layer_NN_post`` paired.
        """
        if self.out_dir is not None:
            (self.out_dir / "layers" / f"layer_{layer:02d}_pre.ein").write_text(
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
        facts_merged: int,
        nogood_emitted: bool,
        nogood_subsumed: bool,
    ) -> None:
        """One ``try_commitment_set`` outcome — alive or dead."""
        self._emit_timeline(
            "entering",
            layer=layer,
            commitment=[
                {"relation": rn, "args": [str(a) for a in args]}
                for (rn, args) in commitment
            ],
            kind=result.kind,
            firings=len(result.firings),
            facts_merged=facts_merged,
            unconditional_count=len(result.unconditional_facts),
            unsat_core_size=len(result.unsat_core),
            nogood_emitted=nogood_emitted,
            nogood_subsumed=nogood_subsumed,
        )

    def layer_end(
        self,
        layer: int,
        kb: KnowledgeBase,
        alive_size: int,
        survived_count: int,
    ) -> None:
        if self.out_dir is not None:
            (self.out_dir / "layers" / f"layer_{layer:02d}_post.ein").write_text(
                _kb_to_ein_text(kb),
            )
        self._emit_timeline(
            "layer_end",
            layer=layer,
            facts=len(kb.facts),
            alive_size=alive_size,
            survived_count=survived_count,
        )

    def early_terminate(self, layer: int, reason: str) -> None:
        """The loop returned mid-layer (e.g. is_solved fired)."""
        self._emit_timeline(
            "early_terminate", layer=layer, reason=reason,
        )

    def close(self) -> None:
        """Close the timeline file without emitting ``summary.json``.

        Called by the abort path (``BudgetExceededError``) so the
        timeline file is flushed + closed when no final summary
        will be written. Normal exits close it via :meth:`summary`.
        Idempotent.
        """
        if self._timeline_fp is not None:
            self._timeline_fp.close()
            self._timeline_fp = None

    def summary(self, verdict: Any, stats: Any) -> None:
        verdict_kind = type(verdict).__name__
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

    # ── Internals ────────────────────────────────────────────────

    def _emit_timeline(self, event: str, **fields_: Any) -> None:
        if self._timeline_fp is None:
            return
        rec = {
            "seq": self._timeline_seq,
            "ts_ms": round((time.time() - self.started_at) * 1000, 3),
            "event": event,
            **fields_,
        }
        self._timeline_fp.write(json.dumps(rec) + "\n")
        self._timeline_fp.flush()
        self._timeline_seq += 1


# ── LatticeDumper helpers ────────────────────────────────────


def _commitment_slug(commitment: tuple) -> str:
    """Render a CanonicalSetId (tuple of ``(relation_name, args)``
    FactIds) as a filesystem-safe slug.

    Empty commitment renders as ``root``; size-1 renders as the
    bare FactId slug; multi-element renders as ``<slug1>+<slug2>``.
    Within each FactId, args are joined with ``_``; ``_`` in
    identifiers becomes ``-`` so the field separator stays
    unambiguous. Matches :func:`tree.state_dump._fact_slug`'s
    style on a per-FactId basis.
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
class LatticeDumper:
    """Filesystem-snapshotting hooks attached to a :func:`gaps_solve`
    or :func:`contradictions_solve` run.

    Sibling of :class:`MonotonicDumper`; shares the same
    lifecycle-hook pattern but adds entry-specific sections.
    The dump folder layout::

        out_dir/
        ├── 00_root_initial.ein   ← initial root snapshot
        ├── 00_timeline.jsonl     ← lifecycle event log
        ├── layers/
        │   ├── layer_NN_pre.ein  ← root.kb at layer NN start
        │   └── layer_NN_post.ein ← root.kb at layer NN end
        ├── solutions/            ← gaps_solve: one folder per SolutionRecord
        │   └── <C-slug>/
        │       ├── commitment.json
        │       ├── kb.ein
        │       └── firings.jsonl
        ├── dead/                 ← contradictions_solve: one folder per DeadCommitment
        │   └── <C-slug>/
        │       ├── commitment.json
        │       ├── unsat_core.jsonl
        │       └── learned_clause.json
        ├── kb_index/             ← store_lattice=True: one folder per SetNode
        │   └── <state_hash_hex>/
        │       ├── canonical_set.json
        │       ├── labels.json
        │       └── verdict.txt
        ├── proof_summary.json    ← top-level proof index (gaps/contra only)
        └── summary.json          ← cumulative stats

    ``out_dir=None`` skips every filesystem write — hooks still
    fire (so the solver loop's call sites stay uniform) but
    produce no on-disk artefacts. Subfolders are created lazily
    on first per-section write so the layout reflects what
    actually happened (no empty ``dead/`` under
    :func:`gaps_solve`, no empty ``solutions/`` under
    :func:`contradictions_solve`).
    """

    out_dir: Path | None = None
    started_at: float = field(default_factory=time.time)
    _timeline_fp: IO[str] | None = field(
        default=None, init=False, repr=False,
    )
    _timeline_seq: int = field(default=0, init=False)
    _solutions_dir_made: bool = field(default=False, init=False)
    _dead_dir_made: bool = field(default=False, init=False)
    _kb_index_dir_made: bool = field(default=False, init=False)

    def __post_init__(self) -> None:
        if self.out_dir is None:
            return
        self.out_dir.mkdir(parents=True, exist_ok=True)
        (self.out_dir / "layers").mkdir(exist_ok=True)
        self._timeline_fp = (
            self.out_dir / "00_timeline.jsonl"
        ).open("w")

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
            (
                self.out_dir / "layers"
                / f"layer_{layer:02d}_pre.ein"
            ).write_text(_kb_to_ein_text(kb))
        self._emit_timeline(
            "layer_start", layer=layer, alive_size=alive_size,
        )

    def entering(
        self,
        layer: int,
        commitment: tuple,
        result: Any,
        *,
        facts_merged: int,
        nogood_emitted: bool,
        nogood_subsumed: bool,
    ) -> None:
        """One ``try_commitment_set`` outcome — alive or dead."""
        rec: dict[str, Any] = {
            "layer": layer,
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

    def layer_end(
        self,
        layer: int,
        kb: KnowledgeBase,
        alive_size: int,
        survived_count: int,
    ) -> None:
        if self.out_dir is not None and kb is not None:
            (
                self.out_dir / "layers"
                / f"layer_{layer:02d}_post.ein"
            ).write_text(_kb_to_ein_text(kb))
        self._emit_timeline(
            "layer_end",
            layer=layer,
            facts=len(kb.facts) if kb is not None else 0,
            alive_size=alive_size,
            survived_count=survived_count,
        )

    def solution_recorded(
        self, record: Any, layer: int,
    ) -> None:
        """Called when :func:`gaps_solve` appends a SolutionRecord.

        Writes a per-solution subfolder under ``solutions/``
        with the commitment, the SolutionRecord's kb snapshot
        (full ein render), and the firings that produced this
        solution.
        """
        self._emit_timeline(
            "solution_recorded",
            layer=layer,
            commitment=(
                _commitment_json(record.commitment)
                if record is not None and record.commitment
                else []
            ),
        )
        if self.out_dir is None or record is None:
            return
        if not self._solutions_dir_made:
            (self.out_dir / "solutions").mkdir(exist_ok=True)
            self._solutions_dir_made = True
        slug = _commitment_slug(record.commitment)
        folder = self.out_dir / "solutions" / slug
        folder.mkdir(parents=True, exist_ok=True)
        (folder / "commitment.json").write_text(
            json.dumps(_commitment_json(record.commitment), indent=2),
        )
        (folder / "kb.ein").write_text(_kb_to_ein_text(record.kb))
        with (folder / "firings.jsonl").open("w") as fp:
            for firing in record.firings:
                fp.write(json.dumps(_firing_to_dict(firing)) + "\n")

    def dead_recorded(self, dead: Any) -> None:
        """Called when :func:`contradictions_solve` appends a
        DeadCommitment.

        Writes a per-dead subfolder under ``dead/`` with the
        commitment, the unsat_core (one fact per line), and the
        learned_clause (sorted FactId list).
        """
        self._emit_timeline(
            "dead_recorded",
            layer=getattr(dead, "layer", None),
            kind=getattr(dead, "kind", None),
            unsat_core_size=(
                len(dead.unsat_core) if dead is not None else 0
            ),
            commitment=(
                _commitment_json(dead.commitment)
                if dead is not None and dead.commitment
                else []
            ),
        )
        if self.out_dir is None or dead is None:
            return
        if not self._dead_dir_made:
            (self.out_dir / "dead").mkdir(exist_ok=True)
            self._dead_dir_made = True
        slug = _commitment_slug(dead.commitment)
        folder = self.out_dir / "dead" / slug
        folder.mkdir(parents=True, exist_ok=True)
        (folder / "commitment.json").write_text(
            json.dumps({
                "commitment": _commitment_json(dead.commitment),
                "layer": dead.layer,
                "kind": dead.kind,
            }, indent=2),
        )
        with (folder / "unsat_core.jsonl").open("w") as fp:
            for f in dead.unsat_core:
                fp.write(json.dumps(_fact_summary(f)) + "\n")
        (folder / "learned_clause.json").write_text(
            json.dumps(
                [
                    _factid_json(fid)
                    for fid in sorted(
                        dead.learned_clause,
                        key=lambda f: (f[0], tuple(map(str, f[1]))),
                    )
                ],
                indent=2,
                default=str,
            ),
        )

    def proof_summary(self, proof: Any) -> None:
        """Top-level proof index — written from ``_finish`` when
        ``verdict.proof`` is non-None. Also materialises the
        ``kb_index/`` folder (one subfolder per :class:`SetNode`)
        when ``proof.kb_index`` is populated."""
        if self.out_dir is None or proof is None:
            return

        # Materialise kb_index/ subfolders.
        if proof.kb_index:
            if not self._kb_index_dir_made:
                (self.out_dir / "kb_index").mkdir(exist_ok=True)
                self._kb_index_dir_made = True
            for key, node in proof.kb_index.items():
                slug = f"{key & 0xFFFFFFFFFFFFFFFF:016x}"
                folder = self.out_dir / "kb_index" / slug
                folder.mkdir(parents=True, exist_ok=True)
                (folder / "canonical_set.json").write_text(
                    json.dumps(
                        _commitment_json(node.canonical_set),
                        indent=2,
                    ),
                )
                (folder / "labels.json").write_text(
                    json.dumps([
                        _commitment_json(c) for c in node.labels
                    ], indent=2),
                )
                (folder / "verdict.txt").write_text(node.verdict)

        # Top-level index.
        summary = {
            "solutions": [
                {
                    "slug": _commitment_slug(s.commitment),
                    "layer": s.layer,
                    "commitment": _commitment_json(s.commitment),
                }
                for s in proof.solutions
            ],
            "dead_commitments": [
                {
                    "slug": _commitment_slug(d.commitment),
                    "layer": d.layer,
                    "kind": d.kind,
                    "commitment": _commitment_json(d.commitment),
                }
                for d in proof.dead_commitments
            ],
            "kb_index": [
                {
                    "state_hash_hex": (
                        f"{key & 0xFFFFFFFFFFFFFFFF:016x}"
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
                for key, node in proof.kb_index.items()
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

    def close(self) -> None:
        """Flush the timeline file without emitting ``summary.json``.

        Called by abort paths (``BudgetExceededError``).
        Idempotent.
        """
        if self._timeline_fp is not None:
            self._timeline_fp.close()
            self._timeline_fp = None

    # ── Internals ────────────────────────────────────────────

    def _emit_timeline(self, event: str, **fields_: Any) -> None:
        if self._timeline_fp is None:
            return
        rec = {
            "seq": self._timeline_seq,
            "ts_ms": round((time.time() - self.started_at) * 1000, 3),
            "event": event,
            **fields_,
        }
        self._timeline_fp.write(json.dumps(rec, default=str) + "\n")
        self._timeline_fp.flush()
        self._timeline_seq += 1


__all__ = ["LatticeDumper", "MonotonicDumper"]
