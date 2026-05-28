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

# `_kb_to_ein_text` lives on the tree side after S1.5b.1's split
# (corrected by commit 995315b — see plans/.../s1.5b.1...). The
# helper is a pure (kb → str) renderer; importing across engine
# folders is consistent with monotonic/solver.py already importing
# Verdict types from `ein_bot.inference.tree.solver`.
from ein_bot.inference.tree.state_dump import _kb_to_ein_text
from ein_bot.kb.store import KnowledgeBase


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


@dataclass
class LatticeDumper:
    """Filesystem-snapshotting hooks attached to a :func:`gaps_solve`
    or :func:`contradictions_solve` run.

    Sibling of :class:`MonotonicDumper`; shares the same
    lifecycle-hook pattern but adds entry-specific sections
    (``solutions/`` under :func:`gaps_solve`, ``dead/`` under
    :func:`contradictions_solve`, ``kb_index/`` when
    ``store_lattice=True``). The two dumpers may merge into a
    single class end-of-phase; for now keeping them separate
    keeps each entry's audit shape independent.

    ``out_dir=None`` skips every filesystem write — the hooks
    still fire but produce no on-disk artefacts.

    **Skeleton stage — S1.5b.20.** All hooks are no-op
    callables. S1.5b.29 fills the real per-set audit layout
    per ``s1.5b.29_lattice_proof.md``.
    """

    out_dir: Path | None = None
    started_at: float = field(default_factory=time.time)

    # Lifecycle hooks — names mirror MonotonicDumper for
    # consistency. S1.5b.29 will override these with the real
    # writers and add the lattice-specific
    # ``solution_recorded`` / ``dead_recorded`` /
    # ``proof_summary`` hooks (already stubbed below).

    def root_initial(self, kb: KnowledgeBase) -> None:
        """Called once after Phase 1's initial saturation."""

    def layer_start(
        self, layer: int, kb: KnowledgeBase, alive_size: int,
    ) -> None:
        """Called at the top of each Phase 2 layer iteration."""

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
        """Called after each ``try_commitment_set`` returns."""

    def layer_end(
        self,
        layer: int,
        kb: KnowledgeBase,
        alive_size: int,
        survived_count: int,
    ) -> None:
        """Called at the bottom of each Phase 2 layer iteration."""

    def solution_recorded(
        self, record: Any, layer: int,
    ) -> None:
        """Called when :func:`gaps_solve` appends a SolutionRecord."""

    def dead_recorded(self, dead: Any) -> None:
        """Called when :func:`contradictions_solve` appends a DeadCommitment."""

    def proof_summary(self, proof: Any) -> None:
        """Called from :meth:`summary` when ``proof`` is non-None."""

    def summary(self, verdict: Any, stats: Any) -> None:
        """Called once at the end of the solve."""

    def close(self) -> None:
        """Called on abort (budget exceeded) — flush partial state."""


__all__ = ["LatticeDumper", "MonotonicDumper"]
