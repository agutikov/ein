"""StateDumper — per-phase filesystem snapshots of the hypothesis loop.

A diagnostic harness for "show me the complete picture of inference"
on a `solve()` run. The dumper is wired through optional `dumper=`
arguments on :func:`solver.solve` and :func:`solver.try_branch`; when
provided, the engine calls back at six lifecycle points:

==============================  ===================================
hook                            output path
==============================  ===================================
``root_initial(kb)``            ``00_root_initial.ein``
``root_saturated(kb, firings,   ``01_root_saturated.ein``
naf_dropped)``                  ``01_root_saturated/stats.json``
                                ``01_root_saturated/firings.jsonl``
``root_hyps(alive, stats)``     ``02_root_hyps.ein``
                                ``02_root_hyps/hyp_stats.json``
``branch_pre(bid, parent, h)``  ``branches/b<bid>/hypothesis.ein``
                                ``branches/b<bid>/pre_sat.ein``
``branch_post(bid, kb, firings, ``branches/b<bid>/post_sat.ein``
result)``                       ``branches/b<bid>/firings.jsonl``
                                ``branches/b<bid>/verdict.json``
``backprop(parent_before,       ``branches/b<bid>/backprop.ein``
parent_after, neg_facts)``      ``branches/b<bid>/backprop.json``
``summary(verdict, tree, cfg)`` ``summary.json``
==============================  ===================================

The ``.ein`` files are full-KB snapshots — every layer (ONTOLOGY,
FACT, REASONING) collapsed into one ``(ontology …)`` + ``(facts …)``
pair so each file is independently readable. Provenance lands in
each fact's ``:source`` / ``:rule`` kw-pair where present; layer is
recorded via a ``:layer`` kw-pair for the FACT and REASONING facts
(ONTOLOGY facts go in the ontology block).

Designed for human inspection + diff; not a serialisation format.
The output isn't round-trippable through `parse` unless the puzzle
happens to declare every relation referenced (parser will reject
unknown relations); inspect the files, don't re-feed them.
"""
from __future__ import annotations

import json
import time
from dataclasses import dataclass, field
from pathlib import Path
from typing import TYPE_CHECKING, Any

from ein_bot.ir.types import Atom, Int, Keyword, KwPair, SForm, String

if TYPE_CHECKING:
    from ein_bot.inference.firing import Firing
    from ein_bot.inference.hypgen import HypGenStats
    from ein_bot.inference.solver import BranchResult
    from ein_bot.kb.entities import Fact
    from ein_bot.kb.store import KnowledgeBase


# ── Serialisation helpers ─────────────────────────────────────────


def _arg_to_node(arg: Any) -> Any:
    """Lower a Fact arg to an IR node."""
    from ein_bot.kb.entities import Fact
    if isinstance(arg, Fact):
        return _fact_to_sform(arg, with_kwargs=False)
    if isinstance(arg, int):
        return Int(value=arg)
    return Atom(name=str(arg))


def _fact_to_sform(fact: Fact, *, with_kwargs: bool = True) -> SForm:
    """Render a Fact as `(rel arg₀ arg₁ … :source "…" :rule "…" :layer …)`.

    Nested-Fact args are recursively lowered without their kwargs (so
    they read as bare `(rel args…)` inside the outer form).
    """
    args: list[Any] = [_arg_to_node(a) for a in fact.args]
    if with_kwargs:
        prov = fact.provenance
        if prov is not None:
            if prov.kind == "source" and prov.source:
                args.append(
                    KwPair(key=Keyword(name="source"), value=String(value=prov.source)),
                )
            elif prov.kind == "rule" and prov.rule:
                args.append(
                    KwPair(key=Keyword(name="rule"), value=String(value=prov.rule)),
                )
            elif prov.kind == "hypothesis":
                args.append(
                    KwPair(
                        key=Keyword(name="hypothesis"),
                        value=Int(value=prov.branch or 0),
                    ),
                )
        # Layer kw-pair on FACT/REASONING facts so the reader can tell
        # them apart inside a single (facts …) block. ONTOLOGY facts
        # don't carry one — they live in the ontology block.
        from ein_bot.kb.entities import Layer
        if fact.layer is Layer.REASONING:
            args.append(
                KwPair(key=Keyword(name="layer"), value=Atom(name="reasoning")),
            )
    return SForm(head=Atom(name=fact.relation_name), args=tuple(args))


def _kb_to_ein_text(kb: KnowledgeBase) -> str:
    """Render a KB as `(ontology …) (facts …)` ein text.

    Splits by layer:
    - ONTOLOGY-layer facts (the `(relation …)`, `(is-a …)`,
      `(bijective …)`, etc.) land in the ontology block.
    - FACT-layer facts (the puzzle's authored conditions) and
      REASONING-layer facts (everything the saturator derived,
      including `(not …)`) land in the facts block, with
      `:layer reasoning` annotated on the derived ones.
    """
    from ein_bot.ir.dump import dump_canonical
    from ein_bot.kb.entities import Layer

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
        forms.append(SForm(head=Atom(name="ontology"), args=tuple(ont_args)))
    if fact_args:
        forms.append(SForm(head=Atom(name="facts"), args=tuple(fact_args)))
    return dump_canonical(forms)


def _firing_to_dict(firing: Firing) -> dict[str, Any]:
    """Serialise a Firing for JSONL output.

    Stripped-down: rule name, activator, derived fact id,
    redundancy flag, and the premise fact-ids. Bindings flattened
    to {str: str} (Fact bindings dropped — chase them via premises).
    """
    bindings_clean: dict[str, str] = {}
    for k, v in firing.bindings.items():
        from ein_bot.kb.entities import Fact
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
                {"relation": a.relation_name, "args": list(map(str, a.args))}
                if isinstance(a, type(firing.derived)) else str(a)
                for a in firing.derived.args
            ],
        },
        "premises": [
            {"relation": p.relation_name, "args": list(map(str, p.args))}
            for p in firing.premises
        ],
    }


def _fact_summary(fact: Fact) -> dict[str, Any]:
    return {
        "relation": fact.relation_name,
        "args": [str(a) for a in fact.args],
    }


# ── The dumper ────────────────────────────────────────────────────


@dataclass
class StateDumper:
    """Filesystem-snapshotting hooks attached to a `solve()` invocation.

    Usage::

        dumper = StateDumper(Path("dumps/zebra2-2026-05-24/"))
        solve(kb, dumper=dumper)
        # → reads `dumps/zebra2-…/{00_root_initial.ein, …}`

    Each hook is independently optional — if the solver omits a call
    (e.g. the back-prop hook), the corresponding files just don't
    appear.
    """

    out_dir: Path
    started_at: float = field(default_factory=time.time)
    _branches_seen: set[int] = field(default_factory=set)

    def __post_init__(self) -> None:
        self.out_dir.mkdir(parents=True, exist_ok=True)
        (self.out_dir / "branches").mkdir(exist_ok=True)

    # ── Root pipeline ────────────────────────────────────────────

    def root_initial(self, kb: KnowledgeBase) -> None:
        """Snapshot of the parsed input, before any inference runs."""
        (self.out_dir / "00_root_initial.ein").write_text(_kb_to_ein_text(kb))

    def root_saturated(
        self, kb: KnowledgeBase, firings: tuple, naf_dropped: int,
    ) -> None:
        """Snapshot after root saturation completes."""
        (self.out_dir / "01_root_saturated.ein").write_text(_kb_to_ein_text(kb))
        sub = self.out_dir / "01_root_saturated"
        sub.mkdir(exist_ok=True)
        productive = [f for f in firings if not f.redundant]
        per_rule: dict[str, int] = {}
        for f in productive:
            per_rule[f.rule] = per_rule.get(f.rule, 0) + 1
        (sub / "stats.json").write_text(json.dumps({
            "total_firings": len(firings),
            "productive": len(productive),
            "redundant": len(firings) - len(productive),
            "naf_dropped": naf_dropped,
            "per_rule_productive": per_rule,
            "fact_count_by_layer": _fact_count_by_layer(kb),
        }, indent=2, sort_keys=True))
        with (sub / "firings.jsonl").open("w") as fp:
            for f in firings:
                fp.write(json.dumps(_firing_to_dict(f)) + "\n")

    def root_hyps(self, alive: list[Fact], stats: HypGenStats) -> None:
        """Snapshot of the root alive hypothesis set + filter stats."""
        forms = [_fact_to_sform(h, with_kwargs=False) for h in alive]
        wrapper = SForm(head=Atom(name="alive"), args=tuple(forms))
        from ein_bot.ir.dump import dump_canonical
        (self.out_dir / "02_root_hyps.ein").write_text(dump_canonical([wrapper]))
        sub = self.out_dir / "02_root_hyps"
        sub.mkdir(exist_ok=True)
        (sub / "hyp_stats.json").write_text(json.dumps({
            "raw": stats.raw,
            "emitted": stats.emitted,
            "filtered": dict(stats.filtered),
            "pre_candidate": dict(stats.pre_candidate),
        }, indent=2, sort_keys=True))

    # ── Per-branch ───────────────────────────────────────────────

    def branch_pre(
        self, bid: int, parent_kb: KnowledgeBase, hypothesis: Fact,
    ) -> None:
        """Snapshot the parent KB + the hypothesis being tried.

        Called just before the branch's saturation. The hypothesis is
        *not yet* in `parent_kb`; the dumped `pre_sat.ein` shows the
        parent at the moment of fork.
        """
        bdir = self._branch_dir(bid)
        from ein_bot.ir.dump import dump_canonical
        (bdir / "hypothesis.ein").write_text(
            dump_canonical([_fact_to_sform(hypothesis, with_kwargs=False)]),
        )
        (bdir / "pre_sat.ein").write_text(_kb_to_ein_text(parent_kb))

    def branch_post(
        self,
        bid: int,
        branch_kb: KnowledgeBase,
        firings: tuple,
        result: BranchResult,
    ) -> None:
        """Snapshot after the branch's saturation closes."""
        bdir = self._branch_dir(bid)
        (bdir / "post_sat.ein").write_text(_kb_to_ein_text(branch_kb))
        with (bdir / "firings.jsonl").open("w") as fp:
            for f in firings:
                fp.write(json.dumps(_firing_to_dict(f)) + "\n")
        verdict_json = {
            "branch_id": bid,
            "kind": result.kind,
            "hypothesis": _fact_summary(result.hypothesis),
            "firings": len(firings),
            "unsat_core": [_fact_summary(f) for f in result.unsat_core],
        }
        (bdir / "verdict.json").write_text(json.dumps(verdict_json, indent=2))

    def backprop(
        self,
        bid: int,
        parent_before: KnowledgeBase,
        parent_after: KnowledgeBase,
        neg_facts: list[Fact],
    ) -> None:
        """Snapshot of the parent KB before and after a back-prop write.

        Fires when a branch dies unconditionally and writes a
        ``(not h)`` (plus its unsat-core's derived companions) into
        the parent. The parent state changes; the dumper snapshots
        the delta.
        """
        bdir = self._branch_dir(bid)
        (bdir / "backprop_after.ein").write_text(_kb_to_ein_text(parent_after))
        (bdir / "backprop.json").write_text(json.dumps({
            "added_negatives": [_fact_summary(f) for f in neg_facts],
            "parent_fact_count_before": len(parent_before.facts),
            "parent_fact_count_after": len(parent_after.facts),
        }, indent=2))

    # ── Summary ──────────────────────────────────────────────────

    def summary(self, verdict: Any, tree: Any, config: Any) -> None:
        """Top-level summary written at the end of the run."""
        verdict_kind = type(verdict).__name__
        leaves = {
            "solution": len(tree.solutions()) if tree is not None else 0,
            "dead": len(tree.dead_branches()) if tree is not None else 0,
            "open": len(tree.open_branches()) if tree is not None else 0,
        }
        cfg_dict: dict[str, Any] = {}
        if config is not None:
            from dataclasses import fields as _fields
            try:
                cfg_dict = {f.name: getattr(config, f.name) for f in _fields(config)}
            except TypeError:
                cfg_dict = {"repr": repr(config)}
        elapsed = time.time() - self.started_at
        (self.out_dir / "summary.json").write_text(json.dumps({
            "verdict": verdict_kind,
            "leaves": leaves,
            "tree_nodes": len(tree.nodes) if tree is not None else 0,
            "branches_dumped": sorted(self._branches_seen),
            "elapsed_seconds": round(elapsed, 3),
            "config": cfg_dict,
        }, indent=2, sort_keys=True, default=str))

    # ── Internals ────────────────────────────────────────────────

    def _branch_dir(self, bid: int) -> Path:
        self._branches_seen.add(bid)
        d = self.out_dir / "branches" / f"b{bid}"
        d.mkdir(parents=True, exist_ok=True)
        return d


def _fact_count_by_layer(kb: KnowledgeBase) -> dict[str, int]:
    out: dict[str, int] = {}
    for f in kb.facts:
        key = f.layer.value if hasattr(f.layer, "value") else str(f.layer)
        out[key] = out.get(key, 0) + 1
    return out


__all__ = ["StateDumper"]
