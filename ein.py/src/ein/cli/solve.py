"""``ein solve`` — the one solver command.

Runs the sound set-search engine (:func:`ein.inference.monotonic.solve`) and
prints the result: the solution(s) when satisfiable, or the unsat core when
not. The verdict is *read from the result* (``k = 0 / 1 / >1`` →
contradiction / solution / gaps), never chosen by a flag — there is no
``--mode`` / ``--gaps`` / ``--contradictions``; those are three *answers* to
one problem, not three commands (the unsound split they replaced is gone —
see ``acceptance/test_mode_consistency.py``). Merges the former ``search`` and
``lattice`` engine-runner subcommands (P1.11) into this one entry — including
their full diagnostic surface (``--verbose`` progress, ``--timing`` phase
table, ``--shuffle``, ``--stats``, the engine toggles), all opt-in so the
default output stays clean.

Every option carries a short key (``-h`` lists them).

Stop policy (how many solutions to look for):

    (default)        stop at the first solution           stop_after=1
    -n / --solutions N    stop after N distinct solutions  stop_after=N
    -e / --exhaustive     exhaust the lattice              stop_after=None
                          → certifies unique / ambiguous / unsat

Output:

    stdout           the answer (the solution, "ambiguous — k models", or
                     "no solution — unsat core …"); -s/--stats adds engine
                     counters; -t/--timing adds a per-phase timing table;
                     --print-final-* dumps the model facts / core.
    -r / --trace FILE    the self-contained markdown derivation trace, to a
                         FILE (never stdout).
    -v / --verbose       per-layer + per-entering progress to stderr.
"""
from __future__ import annotations

import argparse
import random
import sys
import time
from pathlib import Path

from ._factdump import (
    fact_sexpr,
    hypothesis_target_relations,
    print_final_state,
)


def _timed_load(path: Path):
    """Parse + build the KB, timing each phase. Returns
    ``(kb, parse_ms, load_ms, n_forms)`` or ``(None, …)`` on error (printing
    it). Replaces the shared ``_load_kb_or_exit`` so ``--timing`` can split
    parse from kb-load without re-doing the work."""
    from ..ir import IRParseError, parse
    from ..kb import KBLoadError, KnowledgeBase

    try:
        text = path.read_text(encoding="utf-8")
        t = time.perf_counter()
        forms = parse(text, filename=str(path))
        parse_ms = (time.perf_counter() - t) * 1000.0
    except IRParseError as e:
        print(e, file=sys.stderr)
        return None, 0.0, 0.0, 0
    try:
        t = time.perf_counter()
        kb = KnowledgeBase.from_ir(forms, base_dir=path.parent)
        load_ms = (time.perf_counter() - t) * 1000.0
    except KBLoadError as e:
        print(f"kb load error: {e}", file=sys.stderr)
        return None, 0.0, 0.0, 0
    return kb, parse_ms, load_ms, len(forms)


class _TimingDumper:
    """Captures per-phase wall-clock off the solve loop's lifecycle hooks (no
    file I/O). ``t0`` = solve start (this object is built immediately before
    ``solve()``); ``root_initial`` fires when root saturation finishes;
    ``summary`` fires at the end — so root-saturation and hypothesis-search
    split cleanly. Duck-typed (not a MonotonicDumper subclass) to keep the
    engine import off the ``ein --help`` path."""

    def __init__(self) -> None:
        self.t0 = time.perf_counter()
        self.t_root: float | None = None
        self.t_end: float | None = None
        self.root_facts = 0

    def root_initial(self, kb) -> None:
        self.t_root = time.perf_counter()
        self.root_facts = len(kb.facts)

    def layer_start(self, layer, kb, alive_size) -> None:
        pass

    def entering(self, layer, commitment, result, *, outcome="alive",
                 facts_merged=0, nogood_emitted=False,
                 nogood_subsumed=False) -> None:
        pass

    def layer_end(self, layer, kb, alive_size, survived_count) -> None:
        pass

    def summary(self, verdict, stats) -> None:
        self.t_end = time.perf_counter()

    def close(self) -> None:
        if self.t_end is None:
            self.t_end = time.perf_counter()


def _resolved_config(kb, args):
    """Start from ``kb.config`` (or defaults) and apply the CLI overrides:
    the engine toggles, the lattice-order / sanity flags, and ``--shuffle``
    (a per-layer commitment-order permutation via ``lattice_order_seed``)."""
    from dataclasses import replace

    from ..inference.config import SolverConfig

    cfg = kb.config or SolverConfig()
    overrides: dict = {}
    if args.no_lookahead:
        overrides["enable_pre_branch_lookahead"] = False
    if args.no_kill_cache:
        overrides["enable_lookahead_kill_cache"] = False
    if args.lattice_sanity_check:
        overrides["lattice_sanity_check"] = True
    if args.lattice_order is not None:
        overrides["lattice_order"] = args.lattice_order
    if args.shuffle:
        overrides["lattice_order_seed"] = args.seed
    return replace(cfg, **overrides) if overrides else cfg


def _make_dumper(args):
    """Pick the lifecycle dumper. ``--timing`` takes precedence (it needs the
    hook timestamps); else ``--verbose`` streams progress; ``--dump-states``
    persists the dump tree."""
    if args.timing:
        return _TimingDumper()
    from ..inference.monotonic import ProgressDumper
    from ..inference.monotonic.state_dump import MonotonicDumper

    out_dir = Path(args.dump_states) if args.dump_states else None
    if args.verbose:
        return ProgressDumper(progress_every=args.progress_every, out_dir=out_dir)
    if out_dir is not None:
        return MonotonicDumper(out_dir=out_dir)
    return None


def _print_final(verdict: object, kb: object, args: argparse.Namespace) -> None:
    """``--print-final-*``: dump the model facts per solution (or, for an
    unsat verdict, the unsat-core facts — there is no model to dump)."""
    from ..inference.verdict import Ambiguity, Contradiction

    modes = [m for flag, m in (
        (args.print_final_state, "all"),
        (args.print_final_positive, "positive"),
        (args.print_final_hfacts, "hfacts"),
    ) if flag]
    if not modes:
        return

    if isinstance(verdict, Contradiction):
        core = verdict.unsat_core
        print()
        print(f"unsat-core facts ({len(core)} facts):")
        for f in sorted(core, key=lambda f: (
                f.relation_name, tuple(fact_sexpr(a) for a in f.args))):
            print(f"  {fact_sexpr(f)}")
        return

    targets = hypothesis_target_relations(kb)
    branches = verdict.branches if isinstance(verdict, Ambiguity) else (verdict,)
    for i, branch in enumerate(branches, 1):
        if len(branches) > 1:
            print()
            print(f"── solution {i}/{len(branches)} ──")
        for mode in modes:
            print_final_state(branch.kb, mode=mode, targets=targets)


def _print_stats(stats: object, elapsed_ms: float) -> None:
    print()
    print("stats")
    print(f"  solutions (k)    {stats.solution_nodes}")
    print(f"  exhausted        {str(stats.exhausted).lower()}")
    print(f"  enterings        {stats.enterings_total} "
          f"(alive={stats.enterings_alive} "
          f"dead_pre={stats.enterings_dead_pre} "
          f"dead_post={stats.enterings_dead_post})")
    print(f"  layers_explored  {stats.layers_explored}")
    print(f"  saturate_count   {stats.saturate_count}")
    print(f"  nogoods          emitted={stats.nogoods_emitted} "
          f"subsumed={stats.nogoods_subsumed}")
    print(f"  wall             {elapsed_ms:.1f} ms")


def _print_timing(td: _TimingDumper, *, parse_ms: float, load_ms: float,
                  n_forms: int, compile_ms: float, n_plans: int,
                  stats: object) -> None:
    """The per-phase wall-clock table (-t/--timing)."""
    root_ms = (td.t_root - td.t0) * 1000.0 if td.t_root else 0.0
    end = td.t_end if td.t_end is not None else time.perf_counter()
    search_ms = (end - td.t_root) * 1000.0 if td.t_root else 0.0
    solve_ms = root_ms + search_ms
    e2e_ms = parse_ms + load_ms + solve_ms
    enterings = stats.enterings_total
    per_hyp = search_ms / enterings if enterings else 0.0

    print()
    print("timing (ms)")
    print(f"  parse              {parse_ms:9.2f}    ({n_forms} forms)")
    print(f"  kb load            {load_ms:9.2f}")
    print(f"  root saturation    {root_ms:9.2f}    "
          f"({td.root_facts} facts after saturation)")
    print(f"  hypothesis search  {search_ms:9.2f}    "
          f"({enterings} enterings / {stats.layers_explored} layers / "
          f"{stats.saturate_count} saturations)")
    print(f"    per hypothesis   {per_hyp:9.2f}    (avg over enterings)")
    print("  " + "─" * 40)
    print(f"  solve              {solve_ms:9.2f}    (root saturation + search)")
    print(f"  end-to-end         {e2e_ms:9.2f}    (parse + load + solve)")
    print(f"  compile            {compile_ms:9.2f}    ({n_plans} plans; isolated "
          f"— the solve compiles these lazily inside saturation)")


def _print_resolved_config(cfg) -> None:
    """``--dump-config``: print each resolved :class:`SolverConfig` field."""
    from dataclasses import fields as _dc_fields
    print("config (resolved)")
    for f in _dc_fields(cfg):
        v = getattr(cfg, f.name)
        shown = str(v).lower() if isinstance(v, bool) else v
        print(f"  {f.name.replace('_', '-'):<32s} {shown}")


def _print_root_hyp_preview(kb) -> None:
    """``--hyp-stats``: saturate a fork of root and report what the hypothesis
    generator would enumerate (total + per-relation breakdown + filter report)."""
    from collections import Counter

    from ..inference.closed import emit_closed
    from ..inference.hypgen import generate_hypotheses_with_stats
    from ..inference.saturator import Saturator

    preview = kb.fork()
    emit_closed(preview)
    list(Saturator(preview).saturate())
    root_facts, root_stats = generate_hypotheses_with_stats(preview)
    by_rel: Counter = Counter()
    for h in root_facts:
        by_rel[h.relation_name] += 1
    if not by_rel:
        print("root hyps        0 candidates")
        return
    total = sum(by_rel.values())
    print(f"root hyps        {total} candidates across {len(by_rel)} relations")
    for rel, n in by_rel.most_common():
        print(f"  {rel:<24s} {n:>6d}  ({100.0 * n / total:>5.1f}%)")
    print("root hyp-gen filter breakdown:")
    for line in root_stats.as_report_lines():
        print(f"  {line}")


def _write_trace(verdict: object, args: argparse.Namespace) -> None:
    from ..trace import linearize, render_markdown

    trace = linearize(
        verdict, diagrams=not args.no_diagrams,
        full_kb_snapshots=args.full_kb_snapshots, relevant=args.relevant,
    )
    md = render_markdown(
        trace, mode="reorder" if args.reorder else "engine",
        diagrams=not args.no_diagrams,
    )
    Path(args.trace).write_text(md, encoding="utf-8")
    print(f"wrote {args.trace} ({len(trace.steps)} steps, "
          f"{len(trace.reductios)} refuted)", file=sys.stderr)


def _cmd_solve(args: argparse.Namespace) -> int:
    from ..inference.monotonic import solve
    from ..inference.monotonic.solver import BudgetExceededError
    from ..trace import render_solution_table

    kb, parse_ms, load_ms, n_forms = _timed_load(Path(args.file))
    if kb is None:
        return 1

    # --shuffle randomises the within-layer commitment order (seeded by
    # lattice_order_seed). Traversal-only — the verdict is shuffle-invariant
    # (S1.5b.31); a fresh seed each run unless --seed pins it, echoed below.
    if args.shuffle and args.seed is None:
        args.seed = random.randrange(1, 2**31)
    config = _resolved_config(kb, args)
    if args.shuffle:
        print(f"shuffle seed: {args.seed}", file=sys.stderr)

    if args.dump_config:
        _print_resolved_config(config)
    if args.hyp_stats:
        _print_root_hyp_preview(kb)

    # --timing: isolate the (rule, activator) plan-compilation cost (the real
    # solve does this lazily inside saturation, so measure it standalone).
    compile_ms = 0.0
    n_plans = 0
    if args.timing:
        from ..inference.engine import Engine
        t = time.perf_counter()
        eng = Engine(kb)
        eng.compile_all()
        compile_ms = (time.perf_counter() - t) * 1000.0
        n_plans = len(eng.cache)

    dumper = _make_dumper(args)
    stop_after = None if args.exhaustive else args.solutions
    t0 = time.perf_counter()
    try:
        verdict, stats = solve(
            kb, stop_after=stop_after, max_set_size=args.max_set_size,
            config=config, dumper=dumper,
            max_time=args.max_time, max_enterings=args.max_enterings,
            store_lattice=bool(args.trace),
        )
    except BudgetExceededError as e:
        print(f"** aborted: {e.reason} **", file=sys.stderr)
        return 2
    elapsed_ms = (time.perf_counter() - t0) * 1000.0

    # Result-driven: the table reports k / verdict / query bindings / rendered
    # query facts / NL result — all text from puzzle templates (per-relation
    # :why + per-query :goal-text), nothing hardcoded here.
    print(render_solution_table(
        verdict, stats, exhausted=stats.exhausted, source=args.file))
    _print_final(verdict, kb, args)
    if args.stats:
        _print_stats(stats, elapsed_ms)
    if args.timing:
        _print_timing(dumper, parse_ms=parse_ms, load_ms=load_ms,
                      n_forms=n_forms, compile_ms=compile_ms, n_plans=n_plans,
                      stats=stats)
    if args.trace:
        _write_trace(verdict, args)
    return 0


def add_parser(sub) -> None:
    p = sub.add_parser(
        "solve",
        help="solve a puzzle: print the solution(s) or the unsat core",
    )
    p.add_argument("file")

    # ── stop policy (single / N / exhaustive) ──
    stop = p.add_mutually_exclusive_group()
    stop.add_argument("-n", "--solutions", type=int, default=1, metavar="N",
                      help="stop after N distinct solutions (default: 1)")
    stop.add_argument("-e", "--exhaustive", action="store_true",
                      help="exhaust the lattice — certify unique / ambiguous / "
                           "unsat (slower)")

    # ── engine knobs ──
    p.add_argument("-m", "--max-set-size", type=int, default=5,
                   help="commitment-set depth cap (default: 5)")
    p.add_argument("-T", "--max-time", type=float, default=None,
                   help="abort after N wall-clock seconds")
    p.add_argument("-E", "--max-enterings", type=int, default=None,
                   help="abort after N commitment tries")
    p.add_argument("-L", "--no-lookahead", action="store_true",
                   help="disable hypgen's one-step lookahead (forces deaths "
                        "through the monotonic CDCL path)")
    p.add_argument("-K", "--no-kill-cache", action="store_true",
                   help="disable the lookahead-kill (not h) cache (pairs with "
                        "--no-lookahead to exercise the raw loop)")
    p.add_argument("-o", "--lattice-order", choices=("lex", "score-sum"),
                   default=None,
                   help="within-layer candidate ordering (S1.5b.26): 'lex' "
                        "(canonical-tuple sort, the deterministic default) or "
                        "'score-sum' (S1.5a.7 hypgen scoring summed per set)")
    p.add_argument("-y", "--lattice-sanity-check", action="store_true",
                   help="S1.5b.27 regression: for every alive size-k≥2 "
                        "commitment, verify every (k-1)-subset parent path "
                        "saturates to the same kb (costs k+1 saturations each)")

    # ── search order (shuffle invariance — S1.5b.31) ──
    p.add_argument("-z", "--shuffle", action="store_true",
                   help="shuffle the within-layer commitment order (a per-layer "
                        "random permutation). Fresh seed each run unless --seed "
                        "pins it; the seed used is printed to stderr. Changes the "
                        "TRAVERSAL order, not the verdict — the answer is "
                        "shuffle-invariant.")
    p.add_argument("-d", "--seed", type=int, default=None,
                   help="pin the --shuffle RNG seed for a reproducible permutation")

    # ── progress / diagnostics ──
    p.add_argument("-v", "--verbose", action="store_true",
                   help="stream per-layer + per-entering progress to stderr")
    p.add_argument("-g", "--progress-every", type=int, default=100,
                   help="under --verbose, log every N-th entering (default 100; "
                        "set 1 to log every entering)")
    p.add_argument("-D", "--dump-states", type=Path, default=None, metavar="DIR",
                   help="persist the engine dump tree (root snapshot per layer + "
                        "timeline) to DIR")
    p.add_argument("-c", "--dump-config", action="store_true",
                   help="print the resolved SolverConfig before solving")
    p.add_argument("-H", "--hyp-stats", action="store_true",
                   help="print the root-hypothesis preview (total + per-relation "
                        "breakdown + hypgen filter report)")
    p.add_argument("-t", "--timing", action="store_true",
                   help="print a per-phase wall-clock table: parse, kb load, "
                        "compile, root saturation, hypothesis search (enterings, "
                        "layers, saturations, per-hypothesis avg), and totals")

    # ── extra stdout ──
    p.add_argument("-s", "--stats", action="store_true",
                   help="print engine counters (k, enterings, layers, wall)")
    p.add_argument("-p", "--print-final-state", action="store_true",
                   help="dump each solution's REASONING-layer facts (for an "
                        "unsat verdict, the unsat-core facts instead)")
    p.add_argument("-P", "--print-final-positive", action="store_true",
                   help="like --print-final-state but drops the (not …) facts")
    p.add_argument("-f", "--print-final-hfacts", action="store_true",
                   help="dump only the hypothesis-target (query :hrules) facts "
                        "per solution")

    # ── markdown trace (file only) ──
    p.add_argument("-r", "--trace", default=None, metavar="FILE.md",
                   help="write the self-contained markdown derivation trace to "
                        "FILE (a file, not stdout)")
    p.add_argument("-G", "--no-diagrams", action="store_true",
                   help="(--trace) suppress all inline dot blocks")
    p.add_argument("-F", "--full-kb-snapshots", action="store_true",
                   help="(--trace) append a whole-KB snapshot of the final state")
    p.add_argument("-R", "--reorder", action="store_true",
                   help="(--trace) cluster steps by target entity instead of "
                        "engine order")
    p.add_argument("-l", "--relevant", action="store_true",
                   help="(--trace) prune to the goal-relevant slice")
    p.set_defaults(func=_cmd_solve)
