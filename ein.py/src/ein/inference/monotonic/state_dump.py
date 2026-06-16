"""MonotonicDumper — per-layer filesystem snapshots of the monotonic solve.

Engine-agnostic renderers + JSON serialisers, with no per-set storage
(monotonic doesn't carry SetNodes, so there's nothing to dump per
set). The output layout is:

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

The dumper's six lifecycle hooks fire from :func:`solve` when the
caller passes ``dumper=MonotonicDumper(out_dir=…)``.
``dumper=None`` is a no-op for every hook site; the backbone
behaviour is identical to the pre-S1.5b.7 path.
"""
from __future__ import annotations

import sys
import time
from dataclasses import dataclass, field
from pathlib import Path
from typing import IO, Any

from ein.inference.canon import state_hash
from ein.inference.monotonic._lattice_dump import LatticeDumper
from ein.inference.monotonic._serialise import _kb_to_ein_text, _TimelineMixin
from ein.kb.store import KnowledgeBase

__all__ = ["LatticeDumper", "MonotonicDumper", "ProgressDumper"]

@dataclass
class MonotonicDumper(_TimelineMixin):
    """Filesystem-snapshotting hooks attached to a :func:`solve` run.

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

    def root_saturating(self, n_firings: int) -> None:
        """Streamed periodically *during* Phase-1 root saturation, before
        :meth:`root_initial`. Base dumper ignores it; :class:`ProgressDumper`
        streams a live line so a slow root saturation isn't a silent gap."""

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
        *,
        outcome: str = "alive",
        facts_merged: int = 0,
        nogood_emitted: bool = False,
        nogood_subsumed: bool = False,
    ) -> None:
        """One ``try_commitment_set`` outcome — alive or dead.

        ``outcome`` is one of ``"alive"`` / ``"dead-pre"`` /
        ``"dead-post"`` / ``"solution"``. MonotonicDumper logs
        it in the timeline but doesn't write per-commitment
        folders (that's :class:`LatticeDumper`'s job).
        """
        self._emit_timeline(
            "entering",
            layer=layer,
            outcome=outcome,
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


def _fmt_commitment(commitment: tuple) -> str:
    """Render a commitment (CanonicalSetId — a tuple of ``(rel, args)``
    FactIds) as its hypothesis fact(s): ``(color-loc Green House-4)``. Multiple
    facts (layer ≥ 2) are space-joined; the empty set shows as ``∅``."""
    out: list[str] = []
    for fid in commitment:
        if (isinstance(fid, tuple) and len(fid) == 2
                and isinstance(fid[1], tuple)):
            rel, args = fid
            inner = " ".join(str(a) for a in args)
            out.append(f"({rel} {inner})" if inner else f"({rel})")
        else:
            out.append(str(fid))
    return " ".join(out) if out else "∅"


class ProgressDumper(MonotonicDumper):
    """Live progress emitter for slow ``solve`` runs.

    Prints concise progress to ``stream`` (default ``sys.stderr``) so a
    multi-minute exhaustive search isn't a silent hang — used by the P1.7a
    acceptance runner, and available to the CLI/bench. As a
    :class:`MonotonicDumper` subclass, passing ``out_dir`` ALSO writes the
    full filesystem log (``00_timeline.jsonl`` + per-layer ``.ein``
    snapshots) alongside the live lines.

    Every ``progress_every``-th entering prints a line; layer boundaries,
    each solution node, and the final summary always print.
    """

    def __init__(
        self,
        *,
        stream: IO[str] | None = None,
        progress_every: int = 10,
        label: str = "",
        out_dir: Path | None = None,
    ) -> None:
        super().__init__(out_dir=out_dir)
        self.stream = stream if stream is not None else sys.stderr
        self.progress_every = progress_every
        self.label = label
        self._enterings = 0
        # Distinct solution-node states (deduped by state_hash) — matches
        # the verdict's k, not the raw count of solution-outcome events.
        self._node_hashes: set[int] = set()
        # Phase wall-clock (perf_counter), so this dumper doubles as the CLI
        # `--timing` source — lets `-v` and `-t` compose (the CLI's
        # `_TimingDumper` duck-type: t0 / t_root / t_end / root_facts).
        self.t0 = time.perf_counter()
        self.t_root: float | None = None
        self.t_end: float | None = None
        self.root_facts = 0
        # Throttle for root-saturation progress: surfaced only once saturation
        # runs > 1s, then ≤ 1/s — so a fast root saturation adds no noise ahead
        # of the branch-testing search (the progress that matters here).
        self._last_sat_say = self.t0

    def _say(self, msg: str) -> None:
        print(msg, file=self.stream, flush=True)

    def _el(self) -> str:
        return f"{time.time() - self.started_at:5.0f}s"

    def root_saturating(self, n_firings: int) -> None:  # type: ignore[override]
        # Quiet while root saturation is fast; only speak when it's slow enough
        # to look like a hang (≥ 1s in, then at most once a second).
        now = time.perf_counter()
        if now - self._last_sat_say < 1.0:
            return
        self._last_sat_say = now
        head = f"[{self.label}] " if self.label else ""
        self._say(f"{head}  saturating root: {n_firings} firings  ({self._el()})")

    def root_initial(self, kb: KnowledgeBase) -> None:  # type: ignore[override]
        super().root_initial(kb)
        self.t_root = time.perf_counter()
        self.root_facts = len(kb.facts)
        head = f"[{self.label}] " if self.label else ""
        self._say(f"{head}root saturated: {len(kb.facts)} facts  ({self._el()})")

    def layer_start(  # type: ignore[override]
        self, layer: int, kb: KnowledgeBase, alive_size: int,
    ) -> None:
        super().layer_start(layer, kb, alive_size)
        self._say(
            f"  layer {layer}: alive={alive_size} root_facts={len(kb.facts)}"
            f"  ({self._el()})",
        )

    def entering(  # type: ignore[override]
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
        super().entering(
            layer, commitment, result, outcome=outcome,
            facts_merged=facts_merged, nogood_emitted=nogood_emitted,
            nogood_subsumed=nogood_subsumed,
        )
        self._enterings += 1
        if outcome == "solution":
            self._node_hashes.add(state_hash(result.kb))
        if outcome == "solution" or self._enterings % self.progress_every == 0:
            self._say(
                f"    e={self._enterings:>5d} layer={layer}"
                f"  {_fmt_commitment(commitment)}  -> {outcome:<9s}"
                f" solution-nodes={len(self._node_hashes)}  ({self._el()})",
            )

    def layer_end(  # type: ignore[override]
        self,
        layer: int,
        kb: KnowledgeBase,
        alive_size: int,
        survived_count: int,
    ) -> None:
        super().layer_end(layer, kb, alive_size, survived_count)
        self._say(
            f"  layer {layer} done: survivors={survived_count}"
            f" enterings={self._enterings} solution-nodes={len(self._node_hashes)}"
            f"  ({self._el()})",
        )

    def summary(self, verdict: Any, stats: Any) -> None:  # type: ignore[override]
        super().summary(verdict, stats)
        self.t_end = time.perf_counter()
        k = getattr(stats, "solution_nodes", "?")
        ex = getattr(stats, "exhausted", "?")
        self._say(
            f"  => {type(verdict).__name__}  k={k}  exhausted={ex}"
            f"  enterings={stats.enterings_total}  ({self._el()})",
        )


