#!/usr/bin/env python3
"""Run the sound set-search engine (``solve``) on a .ein file.

Mirrors :mod:`bench_solve`'s CLI shape but reaches into
``inference/monotonic/`` instead of ``inference/tree/``. The
verdict is SOUND — :func:`ein.inference.monotonic.solve`
reads Solution / Ambiguity / Contradiction off the deduped
solution-node count ``k`` (not a first-goal-match). Default
``--stop_after=1`` stops at the first complete∧consistent
node (fast, ``exhausted=False``); ``--exhaustive`` runs the
lattice to the end so ``k`` is exact (``exhausted=True``).
Output is one-shot: verdict + goal bindings (if Solution) +
per-run stats + elapsed wall. With ``--dump-states DIR``, also persist
a minimal monotonic dump (root snapshot per layer +
``00_timeline.jsonl``). With ``--verbose``, emit per-layer +
per-entering progress lines to stderr — bench_solve parity
adapted to the monotonic vocabulary (layer = commitment-set
size, entering = ``try_commitment_set`` call).
"""
from __future__ import annotations

import argparse
import sys
import time
from pathlib import Path
from typing import Any

from ein.inference.config import SolverConfig
from ein.inference.lookahead import Lookahead
from ein.inference.monotonic.solver import (
    BudgetExceededError,
    solve,
)
from ein.inference.monotonic.state_dump import MonotonicDumper
from ein.ir import parse
from ein.kb.store import KnowledgeBase

# ── Verbose progress dumper ───────────────────────────────────────


class _VerboseDumper(MonotonicDumper):
    """:class:`MonotonicDumper` that also streams progress to stderr.

    Three signal types — matching the user's ask:

    - **Per-layer summary.** ``layer N start`` lines the size of
      ``alive`` going into the layer; ``layer N end`` lines the
      enterings classified (alive / dead-pre / dead-post),
      nogoods emitted, and rate.
    - **Per-entering progress.** Every ``progress_every``-th
      entering prints its commitment + outcome (``alive``,
      ``dead-pre``, ``dead-post``) + delta facts merged + nogood
      flag. With ``progress_every=1`` every entering surfaces.
    - **Solution announcement.** When ``early_terminate`` fires
      mid-loop on ``is_solved``, that's noted; the bench's
      post-solve goal-bindings print picks up from there.

    If ``out_dir`` is ``None`` the filesystem-write paths in
    the parent class are no-ops — verbose mode then runs without
    paying for the dump. If ``out_dir`` is set both effects
    happen (super() writes files; this class also prints).
    """

    progress_every: int = 100

    def __init__(
        self,
        out_dir: Path | None = None,
        *,
        progress_every: int = 100,
    ) -> None:
        super().__init__(out_dir=out_dir)
        self.progress_every = progress_every
        # Per-layer counters.
        self._enterings_in_layer = 0
        self._alive_in_layer = 0
        self._dead_pre_in_layer = 0
        self._dead_post_in_layer = 0
        self._nogoods_in_layer = 0
        self._layer_started_at: float = self.started_at

    # ── Hook overrides ───────────────────────────────────────────

    def root_initial(self, kb: KnowledgeBase) -> None:  # type: ignore[override]
        super().root_initial(kb)
        print(
            f"root              {len(kb.facts)} facts (post-saturation)",
            file=sys.stderr,
        )

    def layer_start(  # type: ignore[override]
        self, layer: int, kb: KnowledgeBase, alive_size: int,
    ) -> None:
        super().layer_start(layer, kb, alive_size)
        self._enterings_in_layer = 0
        self._alive_in_layer = 0
        self._dead_pre_in_layer = 0
        self._dead_post_in_layer = 0
        self._nogoods_in_layer = 0
        self._layer_started_at = time.time()
        print(
            f"layer {layer:>2d} start    "
            f"alive={alive_size} ({len(kb.facts)} root facts)",
            file=sys.stderr,
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
            layer, commitment, result,
            outcome=outcome,
            facts_merged=facts_merged,
            nogood_emitted=nogood_emitted,
            nogood_subsumed=nogood_subsumed,
        )
        self._enterings_in_layer += 1
        if result.kind == "alive":
            self._alive_in_layer += 1
        elif result.kind == "dead-pre":
            self._dead_pre_in_layer += 1
        else:  # dead-post
            self._dead_post_in_layer += 1
        if nogood_emitted:
            self._nogoods_in_layer += 1

        if self._enterings_in_layer % self.progress_every == 0:
            # ``outcome`` subsumes ``result.kind`` and adds
            # "solution"; show it so a solved fork is visible.
            extras: list[str] = [outcome]
            if facts_merged:
                extras.append(f"merged={facts_merged}")
            if result.firings:
                extras.append(f"firings={len(result.firings)}")
            if nogood_emitted:
                extras.append("nogood")
            elif nogood_subsumed:
                extras.append("nogood-subsumed")
            print(
                f"  e={self._enterings_in_layer:>6d}  "
                f"{_commitment_label(commitment)}  →  "
                f"{' '.join(extras)}",
                file=sys.stderr,
            )

    def layer_end(  # type: ignore[override]
        self, layer: int, kb: KnowledgeBase, alive_size: int,
        survived_count: int,
    ) -> None:
        super().layer_end(layer, kb, alive_size, survived_count)
        dt = max(time.time() - self._layer_started_at, 1e-9)
        rate = self._enterings_in_layer / dt
        print(
            f"layer {layer:>2d} end      "
            f"enterings={self._enterings_in_layer} "
            f"(alive={self._alive_in_layer} "
            f"dead-pre={self._dead_pre_in_layer} "
            f"dead-post={self._dead_post_in_layer}) "
            f"nogoods={self._nogoods_in_layer} "
            f"survived={survived_count} "
            f"alive_after={alive_size} "
            f"in {dt:.2f}s ({rate:.0f} e/s)",
            file=sys.stderr,
        )


def _commitment_label(commitment: tuple) -> str:
    """Render a CanonicalSetId as ``(R a b), (R c d)`` for the log."""
    parts: list[str] = []
    for (rn, args) in commitment:
        args_str = " ".join(str(a) for a in args)
        parts.append(f"({rn} {args_str})")
    return ", ".join(parts)


# ── Argparse + main ───────────────────────────────────────────────


def _build_argparser() -> argparse.ArgumentParser:
    ap = argparse.ArgumentParser(
        description=__doc__.split("\n", 1)[0],
    )
    ap.add_argument("puzzle", type=Path,
                    help="path to .ein puzzle file")
    ap.add_argument("--max-set-size", type=int, default=5,
                    help="largest commitment size to enumerate "
                         "(default: 5)")
    ap.add_argument("--dump-states", type=Path, default=None,
                    help="if set, write a minimal monotonic dump "
                         "to this directory")
    ap.add_argument("--max-time", type=float, default=None,
                    help="abort after N wall-clock seconds")
    ap.add_argument("--max-enterings", type=int, default=None,
                    help="abort after N try_commitment_set calls "
                         "(monotonic equivalent of bench_solve "
                         "--max-nodes)")
    ap.add_argument("--dump-config", action="store_true",
                    help="print the resolved SolverConfig before "
                         "solving (origin tag + each field)")
    ap.add_argument("--hyp-stats", action="store_true",
                    help="print the root-hyp preview: total + "
                         "per-relation breakdown + hypgen filter "
                         "report (mirrors bench_solve --hyp-stats)")
    ap.add_argument("--no-lookahead", action="store_true",
                    help="disable hypgen's one-step lookahead "
                         "(SolverConfig.enable_pre_branch_lookahead=False) "
                         "— forces deaths through the monotonic CDCL "
                         "path rather than being pre-empted")
    ap.add_argument("--no-kill-cache", action="store_true",
                    help="disable the lookahead-kill (not h) cache "
                         "(SolverConfig.enable_lookahead_kill_cache=False) "
                         "— pairs with --no-lookahead when exercising "
                         "the raw monotonic loop")
    ap.add_argument("--exhaustive", action="store_true",
                    help="exhaust the lattice (stop_after=None) instead "
                         "of stopping at the first complete∧consistent "
                         "solution node (stop_after=1). Exhausting lets "
                         "the verdict prove uniqueness (Solution) vs "
                         "ambiguity (Ambiguity, k>1) — stats.exhausted "
                         "is True only then")
    ap.add_argument("--print-final-state", action="store_true",
                    help="on Solution, dump root.kb's REASONING-layer "
                         "facts (bookkeeping heads omitted) sorted "
                         "by (relation, args). Monotonic equivalent "
                         "of bench_solve --solution-facts.")
    ap.add_argument("--print-final-positive", action="store_true",
                    help="like --print-final-state but also drops the "
                         "(not …) facts — the positive residue of the solve.")
    ap.add_argument("--print-final-hfacts", action="store_true",
                    help="dump only the positive facts whose relation is "
                         "targeted by the query's :hrules — the hypothesis "
                         "commitments (e.g. zebra2's 25 *-loc), across all "
                         "layers (so given conditions count too).")
    ap.add_argument("--verbose", "-v", action="store_true",
                    help="emit per-layer + per-entering progress to "
                         "stderr (matches bench_solve --verbose)")
    ap.add_argument("--progress-every", type=int, default=100,
                    help="under --verbose, log every N-th entering "
                         "(default 100; set 1 to log every entering)")
    return ap


def _query_goal_bindings(kb: KnowledgeBase) -> list[dict[str, str]]:
    """Run the query's ``:goal`` pattern against ``kb``; return
    binding rows. Mirrors bench_solve.query_goal_bindings."""
    from ein.inference.compile import JoinPlan, compile_pattern
    from ein.inference.match import run as match_run

    if kb is None or kb.query is None:
        return []
    for kp in kb.query.kw_pairs:
        if kp.key.name == "goal":
            steps = compile_pattern(kp.value, {})
            plan = JoinPlan(
                rule_name="<query>",
                activator_args=(),
                bindings_seed={},
                steps=tuple(steps),
                assert_templates=(),
                why="",
            )
            return [dict(b) for b, _premises in match_run(plan, kb)]
    return []


def _make_dumper(args: argparse.Namespace) -> MonotonicDumper | None:
    if args.verbose:
        return _VerboseDumper(
            out_dir=args.dump_states,
            progress_every=args.progress_every,
        )
    if args.dump_states is not None:
        return MonotonicDumper(out_dir=args.dump_states)
    return None


def _print_resolved_config(kb: KnowledgeBase) -> None:
    """Print the resolved SolverConfig — kb.config when a
    ``(config …)`` head was parsed, else defaults. Mirrors
    bench_solve.py's --dump-config output shape.
    """
    from dataclasses import fields as _dc_fields

    _cfg = kb.config
    origin = (
        "from (config …) block" if _cfg is not None
        else "defaults (no (config …) head)"
    )
    cfg = _cfg or SolverConfig()
    print(f"  config           {origin}")
    for f in _dc_fields(cfg):
        v = getattr(cfg, f.name)
        shown = str(v).lower() if isinstance(v, bool) else v
        print(f"    {f.name.replace('_', '-'):<32s} {shown}")


def _print_root_hyp_preview(kb: KnowledgeBase, verbose: bool) -> None:
    """Saturate a fork of ``kb`` and report what hypgen would
    enumerate at root.

    Mirrors :func:`bench_solve.main`'s preview block: the
    short summary (total + relation count) lands always; the
    per-relation breakdown + the hypgen filter report fire
    behind the verbose flag (the same flag bench_solve uses).
    """
    from collections import Counter

    from ein.inference.closed import emit_closed
    from ein.inference.hypgen import (
        generate_hypotheses_with_stats,
    )
    from ein.inference.saturator import Saturator

    preview = kb.fork()
    emit_closed(preview)
    list(Saturator(preview).saturate())
    root_facts, root_stats = generate_hypotheses_with_stats(preview)
    by_rel: Counter[str] = Counter()
    for h in root_facts:
        by_rel[h.relation_name] += 1
    if not by_rel:
        print("  root hyps        0 candidates")
        return
    total = sum(by_rel.values())
    print(
        f"  root hyps        {total} candidates "
        f"across {len(by_rel)} relations",
    )
    if verbose:
        for rel, n in by_rel.most_common():
            pct = 100.0 * n / total
            print(f"    {rel:<24s} {n:>6d}  ({pct:>5.1f}%)")
        print("  root hyp-gen filter breakdown:")
        for line in root_stats.as_report_lines():
            print(f"  {line}")


def _fact_sexpr(arg: Any) -> str:
    """Render a Fact arg as an s-expression, recursing into nested
    Fact args (e.g. for ``(not (co-located Blue Green))``)."""
    from ein.kb.entities import Fact

    if isinstance(arg, Fact):
        inner = " ".join(_fact_sexpr(a) for a in arg.args)
        return f"({arg.relation_name} {inner})" if inner else f"({arg.relation_name})"
    return str(arg)


def _hypothesis_target_relations(kb: KnowledgeBase) -> set[str]:
    """Relations a query ``:hrules`` clause targets — the hypothesis
    commitments (e.g. zebra2's five ``*-loc``). Generic: the atoms named in
    the ``:hrules`` activators that are *declared relations*, so type/object
    atoms (``House``, ``Color``) drop out position-independently. Empty when
    the puzzle has no query/hrules."""
    if kb is None or kb.query is None:
        return set()
    from ein.ir import Atom, SForm

    def _atoms(node: Any, out: set[str]) -> set[str]:
        if isinstance(node, Atom):
            out.add(node.name)
        elif isinstance(node, SForm):
            if isinstance(node.head, Atom):
                out.add(node.head.name)
            for a in node.args:
                _atoms(a, out)
        return out

    names: set[str] = set()
    for kp in kb.query.kw_pairs:
        if kp.key.name == "hrules":
            _atoms(kp.value, names)
    return names & set(kb.relations)


def _print_final_state(
    kb: KnowledgeBase, *, mode: str = "all",
    targets: set[str] | None = None,
) -> None:
    """Dump a slice of the solution kb's facts in canonical order.

    Mirrors :func:`bench_solve._print_solution_facts` shape but for the single
    monotonic root.kb. Three modes (the three ``--print-final-*`` flags):

    - ``all`` — the full REASONING layer: the propositional residue of the
      solve.
    - ``positive`` — ``all`` with the ``(not …)`` facts dropped too: the
      positive residue.
    - ``hfacts`` — only the positive facts whose relation is a query
      ``:hrules`` target (``targets``), across *every* layer so the given
      conditions count too — the hypothesis commitments (e.g. zebra2's 25
      ``*-loc``). Negatives have relation_name ``not`` ∉ targets, so they drop
      out for free.
    """
    from ein.kb.entities import Layer

    if mode == "hfacts":
        targets = targets or set()
        facts = [f for f in kb.facts if f.relation_name in targets]
        label = f"positive hypothesis-relation facts; :hrules {sorted(targets)}"
    else:
        facts = [
            f for f in kb.facts
            if f.layer == Layer.REASONING
            and not (mode == "positive" and f.relation_name == "not")
        ]
        label = (
            "REASONING layer, (not …) omitted" if mode == "positive"
            else "REASONING layer"
        )
    facts.sort(key=lambda f: (
        f.relation_name,
        tuple(_fact_sexpr(a) for a in f.args),
    ))
    print(f"final-state facts ({label}; {len(facts)} facts):")
    for f in facts:
        args_str = " ".join(_fact_sexpr(a) for a in f.args)
        print(f"  ({f.relation_name} {args_str})")


def _resolved_config(
    kb: KnowledgeBase, args: argparse.Namespace,
) -> SolverConfig:
    """Start from kb.config (or defaults) and apply CLI overrides
    (--no-lookahead, --no-kill-cache)."""
    from dataclasses import replace as _dc_replace

    cfg = kb.config or SolverConfig()
    overrides: dict = {}
    if args.no_lookahead:
        overrides["enable_pre_branch_lookahead"] = False
    if args.no_kill_cache:
        overrides["enable_lookahead_kill_cache"] = False
    if overrides:
        cfg = _dc_replace(cfg, **overrides)
    return cfg


def main(argv: list[str] | None = None) -> int:
    args = _build_argparser().parse_args(argv)
    text = args.puzzle.read_text()
    kb = KnowledgeBase.from_ir(parse(text))

    if args.dump_config:
        _print_resolved_config(kb)
    if args.hyp_stats:
        _print_root_hyp_preview(kb, verbose=True)

    config = _resolved_config(kb, args)
    dumper = _make_dumper(args)

    aborted_reason: str | None = None
    verdict = None
    Lookahead.reset_call_count()  # count one-step sims across this solve
    t0 = time.perf_counter()
    try:
        verdict, stats = solve(
            kb,
            stop_after=None if args.exhaustive else 1,
            max_set_size=args.max_set_size,
            config=config,
            dumper=dumper,
            max_time=args.max_time,
            max_enterings=args.max_enterings,
        )
    except BudgetExceededError as e:
        aborted_reason = e.reason
        stats = e.stats
    elapsed = time.perf_counter() - t0
    lookahead_sims = Lookahead.call_count()

    print(f"file              {args.puzzle}")
    if aborted_reason is not None:
        print(f"** ABORTED: {aborted_reason} **")
    else:
        print(f"verdict           {type(verdict).__name__}")

    sol_kb = getattr(verdict, "kb", None)
    if sol_kb is None and getattr(verdict, "branches", None):
        sol_kb = verdict.branches[0].kb
    if sol_kb is not None:
        rows = _query_goal_bindings(sol_kb)
        if rows:
            print("goal bindings (from query :goal):")
            for row in rows:
                pairs = ", ".join(
                    f"{k}={v}" for k, v in sorted(row.items())
                )
                print(f"  {pairs}")
        if args.print_final_state:
            print()
            _print_final_state(sol_kb, mode="all")
        if args.print_final_positive:
            print()
            _print_final_state(sol_kb, mode="positive")
        if args.print_final_hfacts:
            print()
            _print_final_state(
                sol_kb, mode="hfacts",
                targets=_hypothesis_target_relations(kb),
            )

    print()
    print("stats")
    print(f"  solution_nodes (k) {stats.solution_nodes}")
    print(f"  exhausted          {str(stats.exhausted).lower()}")
    print(f"  enterings_total    {stats.enterings_total}")
    print(f"  enterings_alive    {stats.enterings_alive}")
    print(f"  enterings_dead_pre  {stats.enterings_dead_pre}")
    print(f"  enterings_dead_post {stats.enterings_dead_post}")
    print(f"  facts_merged       {stats.facts_merged}")
    print(f"  forced_positives   {stats.forced_positives}")
    print(f"  saturate_count     {stats.saturate_count}")
    print(f"  lookahead_sims     {lookahead_sims}")
    print(f"  layers_explored    {stats.layers_explored}")
    print(f"  nogoods_emitted    {stats.nogoods_emitted}")
    print(f"  nogoods_subsumed   {stats.nogoods_subsumed}")
    print(f"  wall               {elapsed * 1000:.1f} ms")
    if dumper is not None and args.dump_states is not None:
        print(f"dump              {args.dump_states}")
    return 2 if aborted_reason is not None else 0


if __name__ == "__main__":
    sys.exit(main())
