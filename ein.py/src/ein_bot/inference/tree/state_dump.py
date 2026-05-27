"""StateDumper — per-phase filesystem snapshots of the hypothesis loop.

A diagnostic harness for "show me the complete picture of inference"
on a `solve()` run. The dumper is wired through an optional `dumper=`
argument on :func:`solver.solve`; when provided, the engine calls
back at lifecycle hooks. **The filesystem hierarchy mirrors the
search tree** — each SearchNode gets a folder whose `branches/`
subfolder holds its children, recursively.

==============================  ===================================
hook                            output path
==============================  ===================================
*every hook*                    appends one record to ``00_timeline.jsonl``
``root_initial(kb)``            ``00_root_initial.ein``
``root_saturated(kb, firings,   ``01_root_saturated.ein``
naf_dropped)``                  ``01_root_saturated/stats.json``
                                ``01_root_saturated/firings.jsonl``
``root_hyps(alive, stats)``     ``02_root_hyps.ein``
                                ``02_root_hyps/hyp_stats.json``
``node_alloc(nid, parent_id,    registers ``<parent>/branches/b<nid>[_<slug>]/``
hypothesis=None)``              (e.g. ``b3_drink-loc_milk_house-1``)
``node_dump(nid, parent_id, h,  ``<parent>/branches/b<nid>[_<slug>]/{hypothesis.ein,
kb, firings, verdict_kind,      post_sat.ein, firings.jsonl, verdict.json}``
unsat_core)``
``node_resaturated(nid, cycle,  ``<nid>/resats/<cycle>.ein,
kb, new_firings, dead_children, <cycle>.json,
neg_facts_added)``              <cycle>_firings.jsonl``
``summary(verdict, tree, cfg)`` ``summary.json``
==============================  ===================================

The ``.ein`` files are full-KB snapshots — every layer (ONTOLOGY,
FACT, REASONING) collapsed into one ``(ontology …)`` + ``(facts …)``
pair so each file is independently readable. Provenance lands in
each fact's ``:source`` / ``:rule`` / ``:hypothesis`` kw-pair where
present; layer is recorded via a ``:layer reasoning`` kw-pair on
derived facts (ONTOLOGY facts go in the ontology block).

`node_alloc` pre-registers each search-tree node's directory so
that any children allocated inside the same `_explore` / `_consume`
call land under their parent's already-existing dir. The `_dirs`
map is keyed by node id; the root maps to ``out_dir`` itself.

`node_resaturated` fires at the end of each `_consume` sweep that
ended in at least one unconditional death — after `back_propagate`
+ re-saturation on the parent. Each event is sequence-numbered
inside the parent's `resats/` subfolder so the consume loop's
multi-cycle structure is preserved on disk. The per-cycle JSON
splits the facts written during the cycle into two attribution
buckets (T1.5a.19.3.b):

- ``back_prop_writes`` — synthetic ``(not h)`` (and forced-positive
  bubble) writes from :func:`back_propagate` / :func:`_mirror_forced_positive`;
  each carries ``from_dead_child: <branch_id>`` linking the write
  back to the dead child whose unconditional refutation produced
  it (``None`` for symmetric mirrors or cross-puzzle bubbles that
  don't match a current dead child).
- ``resat_derivations`` — facts derived by the saturator pass that
  followed the back-prop writes; each carries the firing
  ``rule`` and a flat ``premises`` list (``[{relation, args}, …]``).

The flat ``negatives_added`` field is retained for backward compat
— it is the concatenation of the two buckets in firing order.

The chronological event log lives at ``00_timeline.jsonl``
(T1.5a.19.3.a). One JSON record per line, written in firing order,
each with:

- ``seq`` — monotonic event counter (0-indexed)
- ``ts_ms`` — wall-time milliseconds relative to dumper construction
- ``event`` — hook name (``root_initial``, ``root_saturated``,
  ``root_hyps``, ``node_alloc``, ``node_dump``,
  ``node_resaturated``, ``summary``)
- event-specific summary fields (counts, ids, verdict kinds — *not*
  full Fact bodies; those land in the per-event ``.ein`` / ``.json``
  files). Reading the timeline alone tells you the order events
  fired, including the interleave between branch exploration
  (``node_alloc`` / ``node_dump``) and re-saturation
  (``node_resaturated``) — the relationship the per-event files
  alone can't show.

Designed for human inspection + diff; not a serialisation format.
The output isn't round-trippable through `parse` unless the puzzle
happens to declare every relation referenced; inspect the files,
don't re-feed them.
"""
from __future__ import annotations

import json
import time
from dataclasses import dataclass, field
from pathlib import Path
from typing import IO, TYPE_CHECKING, Any

from ein_bot.ir.types import Atom, Int, Keyword, KwPair, SForm, String
from ein_bot.kb.entities import Fact

# Rule-name prefixes that mark a Fact as a back-prop / lookahead write
# (vs a normal saturator derivation). Used by `_classify_resat_write` to
# split `node_resaturated`'s `neg_facts_added` into the two attribution
# buckets. Kept in one place so future back-prop variants (S1.5a.17
# forced-positive bubbles are already covered by the `<` prefix) stay
# classified consistently.
_BACK_PROP_RULE_PREFIXES: tuple[str, ...] = (
    "<back-prop",
    "<lookahead",
    "<forced-positive",
)

if TYPE_CHECKING:
    from ein_bot.inference.firing import Firing
    from ein_bot.inference.hypgen import HypGenStats
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
    """Recursive Fact → JSON-friendly dict.

    Nested-Fact args (e.g. inside ``(not (color-loc Green House-3))``)
    render as nested ``{"relation": …, "args": […]}`` dicts so the
    output is machine-parseable. Atoms / ints / strings stringify.
    """
    return {
        "relation": fact.relation_name,
        "args": [_fact_summary(a) if isinstance(a, Fact) else str(a)
                 for a in fact.args],
    }


def _classify_resat_write(
    fact: Fact, dead_lookup: dict[tuple[str, tuple], int],
) -> tuple[str, dict[str, Any]]:
    """Classify a fact written during a `node_resaturated` event.

    Returns ``(category, record)`` where ``category`` is one of
    ``"back_prop"`` (synthetic write — ``(not h)`` from
    :func:`back_propagate` or a forced-positive bubble) or
    ``"resat"`` (saturator-derived fact carrying named-rule
    provenance like ``"range-elimination"``).

    Back-prop writes link to the dead child whose death produced
    them via ``from_dead_child: branch_id``; the inner Fact's
    identity ``(relation_name, args)`` is matched against
    ``dead_lookup``. Cross-puzzle cases (mirror writes for symmetric
    relations, forced-positive bubbles) may not match a dead child
    and report ``None``.

    Resat derivations carry the firing rule + a flat list of
    ``(relation, args)`` premise ids from the Fact's provenance.
    """
    prov = fact.provenance
    rule = prov.rule if (prov is not None and prov.kind == "rule") else None
    is_back_prop = (
        rule is not None
        and any(rule.startswith(p) for p in _BACK_PROP_RULE_PREFIXES)
    )
    if is_back_prop:
        from_dead_child: int | None = None
        if (fact.relation_name == "not"
                and fact.args and isinstance(fact.args[0], Fact)):
            inner = fact.args[0]
            from_dead_child = dead_lookup.get((inner.relation_name, inner.args))
        return ("back_prop", {
            "from_dead_child": from_dead_child,
            "fact": _fact_summary(fact),
            "rule": rule,
        })
    premises: list[dict[str, Any]] = []
    if prov is not None and prov.kind == "rule":
        premises = [
            {"relation": rel, "args": [str(a) for a in args]}
            for rel, args in prov.premises_raw
        ]
    return ("resat", {
        "fact": _fact_summary(fact),
        "rule": rule,
        "premises": premises,
    })


def _fact_slug(fact: Fact) -> str:
    """Filesystem-safe lowercase slug for a fact.

    ``(drink-loc Milk House_1)`` → ``drink-loc_milk_house-1``. Fields
    are joined with ``_``; within each field, ``_`` in source names
    becomes ``-`` so the field separator stays unambiguous. Hyphens
    in source names (kebab-case relations like ``drink-loc``) are
    preserved verbatim. Nested-Fact args render via ``_fact_slug``
    recursively, wrapped in ``[]`` so they don't merge with the
    flat field stream.
    """
    def field_to_slug(s: str) -> str:
        return s.lower().replace("_", "-")

    parts = [field_to_slug(fact.relation_name)]
    for a in fact.args:
        if isinstance(a, Fact):
            parts.append(f"[{_fact_slug(a)}]")
        else:
            parts.append(field_to_slug(str(a)))
    return "_".join(parts)


# ── The dumper ────────────────────────────────────────────────────


@dataclass
class StateDumper:
    """Filesystem-snapshotting hooks attached to a `solve()` invocation.

    Directory layout — **the folder hierarchy mirrors the search
    tree.** Each node has its own folder containing its
    hypothesis (if non-root), pre/post-saturation kb snapshots,
    firings + verdict, a `resats/` subfolder for re-saturation
    events triggered by child unconditional deaths (S1.5.7b), and
    a `branches/` subfolder holding its children — recursively the
    same shape::

        dumps/<puzzle>-<ts>/
          00_root_initial.ein
          01_root_saturated.ein
          01_root_saturated/{stats.json, firings.jsonl}
          02_root_hyps.ein
          02_root_hyps/hyp_stats.json
          resats/                 ← root re-sats (if any)
            001.ein
            001.json
          branches/               ← root's children
            b1/
              hypothesis.ein
              pre_sat.ein         ← parent kb at fork time
              post_sat.ein        ← b1 kb after b1's saturation
              firings.jsonl
              verdict.json
              resats/             ← re-sats on b1 (after a child died)
                001.ein           ← b1's kb after back-prop + re-sat
                001.json          ← child bids that died + neg facts added
              branches/           ← b1's children (recursive)
                b3/
                  ...
          summary.json

    Each hook is independently optional — if the solver omits a
    call (e.g. the re-sat hook), the corresponding files just
    don't appear.

    Branch nesting requires the solver to call `branch_pre` and
    `branch_post` with a `parent_id` so the dumper can resolve the
    target dir from `_node_dirs[parent_id]`. The root node has
    `parent_id=None`; its dir is `out_dir` itself, registered under
    a sentinel key `_NODE_ROOT`.
    """

    out_dir: Path
    started_at: float = field(default_factory=time.time)
    _node_dirs: dict[int, Path] = field(default_factory=dict)
    _timeline_seq: int = field(default=0, init=False)
    _timeline_fp: IO[str] | None = field(default=None, init=False, repr=False)

    def __post_init__(self) -> None:
        self.out_dir.mkdir(parents=True, exist_ok=True)
        # Root node has search-tree id 0; its dir is out_dir itself.
        self._node_dirs[0] = self.out_dir
        # `00_timeline.jsonl` — chronological event log; one record per
        # hook call in firing order. Opened in __post_init__, flushed
        # after each emit so a tailing reader sees events live. The
        # handle leaks on GC if `summary()` is never called; harmless
        # since the OS reclaims it.
        self._timeline_fp = (self.out_dir / "00_timeline.jsonl").open("w")

    # ── Timeline ─────────────────────────────────────────────────

    def _emit_timeline(self, event: str, **fields: Any) -> None:
        """Append one JSON record to ``00_timeline.jsonl``.

        Records carry a monotonic ``seq``, wall-time ``ts_ms``
        relative to ``started_at``, the ``event`` name (matching the
        hook method), and event-specific fields. Each line is one
        complete JSON object — JSONL, not a JSON array.

        Flushes per write so a reader can ``tail -f`` the file
        during a long run.
        """
        if self._timeline_fp is None:
            return  # defensive — closed/never-opened path
        rec: dict[str, Any] = {
            "seq": self._timeline_seq,
            "ts_ms": round((time.time() - self.started_at) * 1000, 3),
            "event": event,
            **fields,
        }
        self._timeline_fp.write(json.dumps(rec) + "\n")
        self._timeline_fp.flush()
        self._timeline_seq += 1

    # ── Root pipeline ────────────────────────────────────────────

    def root_initial(self, kb: KnowledgeBase) -> None:
        """Snapshot of the parsed input, before any inference runs."""
        (self.out_dir / "00_root_initial.ein").write_text(_kb_to_ein_text(kb))
        self._emit_timeline("root_initial",
                            facts=len(kb.facts))

    def root_saturated(
        self, kb: KnowledgeBase, firings: tuple, naf_dropped: int,
    ) -> None:
        """Snapshot after root saturation completes."""
        (self.out_dir / "01_root_saturated.ein").write_text(_kb_to_ein_text(kb))
        sub = self.out_dir / "01_root_saturated"
        sub.mkdir(exist_ok=True)
        _write_firings(sub, firings, naf_dropped, kb)
        self._emit_timeline("root_saturated",
                            firings=len(firings), naf_dropped=naf_dropped,
                            facts=len(kb.facts))

    def root_hyps(self, alive: list[Fact], stats: HypGenStats) -> None:
        """Snapshot of the root alive hypothesis set + filter stats."""
        _write_alive(self.out_dir, "02_root_hyps", alive, stats)
        self._emit_timeline("root_hyps",
                            alive=len(alive), raw=stats.raw,
                            emitted=stats.emitted,
                            filtered=dict(stats.filtered))

    # ── Per-node ─────────────────────────────────────────────────

    def node_alloc(
        self, nid: int, parent_id: int | None,
        hypothesis: Fact | None = None,
    ) -> None:
        """Pre-register a node's directory under its parent.

        Called by the solver right after ``builder.alloc()`` so that
        any children dumped inside the node's exploration find the
        parent already at ``_node_dirs[parent_id]``. Idempotent.

        When ``hypothesis`` is provided, the directory name carries
        a lowercase slug of the fact — e.g.
        ``b3_drink-loc_milk_house-1`` for
        ``(drink-loc Milk House_1)``. Useful for human inspection;
        the leading ``b<nid>`` keeps lexical sort = allocation order.
        """
        if parent_id is None:
            # Root node — its dir is out_dir; recorded in __post_init__.
            self._node_dirs.setdefault(nid, self.out_dir)
            return
        if nid in self._node_dirs:
            return  # already registered
        self._make_branch_dir(nid, parent_id, hypothesis)
        self._emit_timeline("node_alloc",
                            node_id=nid, parent_id=parent_id,
                            hypothesis=(_fact_summary(hypothesis)
                                        if hypothesis is not None else None))

    def node_dump(
        self,
        nid: int,
        parent_id: int | None,
        hypothesis: Fact | None,
        kb: KnowledgeBase,
        firings: tuple,
        verdict_kind: str,
        unsat_core: frozenset = frozenset(),
    ) -> None:
        """Snapshot one search-tree node at the moment it's finalised.

        Called once per ``builder.add(SearchNode(...))`` in the
        solver. Non-root nodes get a dedicated folder under their
        parent's ``branches/``; the root (parent_id=None) writes its
        verdict.json in-place at ``out_dir`` but skips hypothesis /
        post_sat (those are already captured by `root_initial` +
        `root_saturated`).
        """
        if parent_id is None:
            # Root node — only the verdict tag is interesting at
            # node-finalisation time; the rest is in 00_/01_/02_.
            (self.out_dir / "node.json").write_text(json.dumps({
                "branch_id": nid,
                "kind": verdict_kind,
                "hypothesis": None,
                "firings": len(firings),
                "unsat_core": [_fact_summary(f) for f in unsat_core],
            }, indent=2))
            self._emit_timeline("node_dump",
                                node_id=nid, parent_id=None,
                                verdict_kind=verdict_kind,
                                firings=len(firings),
                                unsat_core_size=len(unsat_core))
            return
        # The dir is normally pre-registered by node_alloc; fall back
        # to creating it now if the caller skipped alloc registration.
        ndir = self._node_dirs.get(nid) or self._make_branch_dir(
            nid, parent_id, hypothesis,
        )
        from ein_bot.ir.dump import dump_canonical
        if hypothesis is not None:
            (ndir / "hypothesis.ein").write_text(
                dump_canonical([_fact_to_sform(hypothesis, with_kwargs=False)]),
            )
        (ndir / "post_sat.ein").write_text(_kb_to_ein_text(kb))
        with (ndir / "firings.jsonl").open("w") as fp:
            for f in firings:
                fp.write(json.dumps(_firing_to_dict(f)) + "\n")
        (ndir / "verdict.json").write_text(json.dumps({
            "branch_id": nid,
            "parent_id": parent_id,
            "kind": verdict_kind,
            "hypothesis": _fact_summary(hypothesis) if hypothesis else None,
            "firings": len(firings),
            "unsat_core": [_fact_summary(f) for f in unsat_core],
        }, indent=2))
        self._emit_timeline("node_dump",
                            node_id=nid, parent_id=parent_id,
                            verdict_kind=verdict_kind,
                            firings=len(firings),
                            unsat_core_size=len(unsat_core))

    def node_resaturated(
        self,
        nid: int,
        cycle: int,
        kb: KnowledgeBase,
        new_firings: tuple,
        dead_children: list[tuple[int, Fact]],
        neg_facts_added: list[Fact],
    ) -> None:
        """A consume-loop re-saturation closed on `nid`.

        Called after `_consume` back-props one or more unconditional
        deaths into this node's KB and runs a re-saturation pass to
        propagate the new ``(not h)`` premises (S1.5.7b T1.5.7.5).
        `cycle` is 1-indexed and increments per re-sat on this node.
        """
        ndir = self._node_dirs.get(nid)
        if ndir is None:
            return  # node not registered (shouldn't happen)
        resats = ndir / "resats"
        resats.mkdir(exist_ok=True)
        (resats / f"{cycle:03d}.ein").write_text(_kb_to_ein_text(kb))
        # T1.5a.19.3.b — split the flat `neg_facts_added` into two
        # attribution buckets by classifying each fact's provenance.
        # Bridges the "where did this bubble from?" gap in the
        # previous flat list.
        dead_lookup: dict[tuple[str, tuple], int] = {
            (h.relation_name, h.args): bid for bid, h in dead_children
        }
        back_prop_writes: list[dict[str, Any]] = []
        resat_derivations: list[dict[str, Any]] = []
        for f in neg_facts_added:
            kind, rec = _classify_resat_write(f, dead_lookup)
            if kind == "back_prop":
                back_prop_writes.append(rec)
            else:
                resat_derivations.append(rec)
        (resats / f"{cycle:03d}.json").write_text(json.dumps({
            "cycle": cycle,
            "node_id": nid,
            "dead_children": [
                {"branch_id": bid, "hypothesis": _fact_summary(h)}
                for bid, h in dead_children
            ],
            "back_prop_writes": back_prop_writes,
            "resat_derivations": resat_derivations,
            # Backward-compat — pre-T1.5a.19.3 flat field. Readers
            # should prefer the two split fields above; this one is
            # the concatenation in the same order.
            "negatives_added": [_fact_summary(f) for f in neg_facts_added],
            "new_firings_count": len(new_firings),
        }, indent=2))
        with (resats / f"{cycle:03d}_firings.jsonl").open("w") as fp:
            for f in new_firings:
                fp.write(json.dumps(_firing_to_dict(f)) + "\n")
        self._emit_timeline("node_resaturated",
                            node_id=nid, cycle=cycle,
                            dead_children_count=len(dead_children),
                            back_prop_writes_count=len(back_prop_writes),
                            resat_derivations_count=len(resat_derivations),
                            new_firings_count=len(new_firings))

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
        branches_dumped = sorted(
            bid for bid in self._node_dirs if isinstance(bid, int)
        )
        (self.out_dir / "summary.json").write_text(json.dumps({
            "verdict": verdict_kind,
            "leaves": leaves,
            "tree_nodes": len(tree.nodes) if tree is not None else 0,
            "branches_dumped": branches_dumped,
            "elapsed_seconds": round(elapsed, 3),
            "config": cfg_dict,
        }, indent=2, sort_keys=True, default=str))
        self._emit_timeline("summary",
                            verdict=verdict_kind, leaves=leaves,
                            tree_nodes=(len(tree.nodes)
                                        if tree is not None else 0),
                            elapsed_seconds=round(elapsed, 3))
        # Close the timeline handle on summary — the dumper's lifecycle
        # ends here (one summary per solve() call). Safe to do here
        # rather than __del__ because the solver always calls summary
        # in its `finally:` block.
        if self._timeline_fp is not None:
            self._timeline_fp.close()
            self._timeline_fp = None

    # ── Internals ────────────────────────────────────────────────

    def _make_branch_dir(
        self, bid: int, parent_id: int, hypothesis: Fact | None = None,
    ) -> Path:
        """Allocate the directory for branch `bid` under its parent.

        Falls back to out_dir if the parent isn't registered yet
        (defensive — every parent should have been registered via
        a prior `branch_pre` call, except the root, which is keyed 0
        in __post_init__).

        Folder name is ``b<bid>`` for no-hypothesis nodes (interior
        / forced) and ``b<bid>_<slug>`` when a hypothesis is known
        at alloc time, where ``<slug>`` is the lowercase fact (e.g.
        ``drink-loc_milk_house-1`` for ``(drink-loc Milk House_1)``).
        """
        parent_dir = self._node_dirs.get(parent_id, self.out_dir)
        name = f"b{bid}"
        if hypothesis is not None:
            slug = _fact_slug(hypothesis)
            if slug:
                name = f"{name}_{slug}"
        d = parent_dir / "branches" / name
        d.mkdir(parents=True, exist_ok=True)
        self._node_dirs[bid] = d
        return d


def _write_firings(
    sub: Path, firings: tuple, naf_dropped: int, kb: KnowledgeBase,
) -> None:
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


def _write_alive(
    out_dir: Path, stem: str, alive: list[Fact], stats: HypGenStats,
) -> None:
    forms = [_fact_to_sform(h, with_kwargs=False) for h in alive]
    wrapper = SForm(head=Atom(name="alive"), args=tuple(forms))
    from ein_bot.ir.dump import dump_canonical
    (out_dir / f"{stem}.ein").write_text(dump_canonical([wrapper]))
    sub = out_dir / stem
    sub.mkdir(exist_ok=True)
    (sub / "hyp_stats.json").write_text(json.dumps({
        "raw": stats.raw,
        "emitted": stats.emitted,
        "filtered": dict(stats.filtered),
        "pre_candidate": dict(stats.pre_candidate),
    }, indent=2, sort_keys=True))


def _fact_count_by_layer(kb: KnowledgeBase) -> dict[str, int]:
    out: dict[str, int] = {}
    for f in kb.facts:
        key = f.layer.value if hasattr(f.layer, "value") else str(f.layer)
        out[key] = out.get(key, 0) + 1
    return out


__all__ = ["StateDumper"]
