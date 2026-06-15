#!/usr/bin/env python3
"""Wall-clock benchmark + state dump for the Saturator on a .ein file.

Informational (S1.3.3 acceptance) — not a CI gate. Run from repo
root:

    python scripts/bench_saturate.py [example.ein] [--dump]
                                     [--max-steps N] [--progress-every K]

Prints, in order:

1. Phase timings — parse, kb-load, compile, saturate.
2. A full state snapshot **before** saturation: every counter the
   KB and Engine expose.
3. While saturating, one progress line every ``--progress-every``
   steps (default 500): step number, time-since-last-mark, current
   fact count, last rule fired.
4. A full state snapshot **after** saturation, showing deltas
   (Δ columns) alongside the absolutes.
5. Saturation-specific stats: total / productive / redundant
   firings, per-rule firing counts, per-relation derived counts.
6. With ``--dump``: the saturated KB itself — schema + every fact
   grouped by layer (ONTOLOGY / FACT / REASONING), with each
   REASONING-layer fact annotated by the rule that produced it.

The snapshot is intentionally exhaustive — it's both a benchmark
and a diagnostic for "what does the engine actually do on this
input?".

For inputs that runaway (e.g. transitive-closure + sibling pairs
producing O(N²) firings), pass ``--max-steps 5000`` to cap the
budget. The Saturator raises ``SaturatorStepLimitError``; the script
catches it and prints a per-rule firing breakdown so you can see
which rule is blowing up.

Target on a laptop: < 200 ms wall-clock for zebra.ein initial
saturation. Treat numbers as informational; performance work
proper is a P1.7+ concern.
"""
from __future__ import annotations

import sys
import time
from collections import Counter
from collections.abc import Iterable
from pathlib import Path
from typing import Any

from ein.inference.engine import Engine
from ein.inference.firing import Firing
from ein.inference.saturator import Saturator
from ein.ir import parse
from ein.kb.entities import Fact, Layer
from ein.kb.store import KnowledgeBase

# ── Snapshot ────────────────────────────────────────────────────


def _has_nested_fact_args(f: Fact) -> bool:
    return any(isinstance(a, Fact) for a in f.args)


def snapshot(kb: KnowledgeBase, eng: Engine | None = None) -> dict[str, Any]:
    """Capture every countable property of a KB + (optional) Engine.

    Returns a flat dict for easy diffing; nested counters are stored
    as ``Counter`` objects so before/after diffs are subtractable.
    """
    s: dict[str, Any] = {}

    # ── Entity counts ────────────────────────────────────────────
    # S1.7.23 — no `kb.types` / `kb.instances` registries; `(type …)` /
    # `(instance …)` are ordinary facts, counted by relation below.
    s["type_facts"] = len(kb._facts_by_relation.get("type", ()))
    s["instance_facts"] = len(kb._facts_by_relation.get("instance", ()))
    s["relations"] = len(kb.relations)
    s["relations_declared"] = sum(1 for r in kb.relations.values() if r.declared)
    s["relations_open_world"] = sum(
        1 for r in kb.relations.values() if not r.declared
    )
    s["rules"] = len(kb.rules)

    # ── Global names index — encoding-agnostic node set ──────────
    names_by_cat: Counter[str] = Counter(
        ref.category for ref in kb.names.values()
    )
    s["names_total"]    = len(kb.names)
    s["names_objects"]  = names_by_cat.get("object", 0)
    s["names_relations"] = names_by_cat.get("relation", 0)
    s["names_rules"]    = names_by_cat.get("rule", 0)
    # Total head-participation and arg-participation across all names.
    s["names_as_head_total"] = sum(len(r.as_head) for r in kb.names.values())
    s["names_as_arg_total"]  = sum(len(r.as_arg)  for r in kb.names.values())
    # Names that appear ONLY as head (declared relations/rules with no
    # facts yet) or ONLY as arg (orphan objects mentioned without
    # declaration) — useful diagnostics for ontology authoring.
    s["names_head_only"] = sum(
        1 for r in kb.names.values()
        if r.as_head and not r.as_arg
    )
    s["names_arg_only"] = sum(
        1 for r in kb.names.values()
        if r.as_arg and not r.as_head
    )

    # ── Facts (totals + per layer) ───────────────────────────────
    s["facts_total"] = len(kb.facts)
    layers = Counter(f.layer for f in kb.facts)
    s["facts_ontology"] = layers.get(Layer.ONTOLOGY, 0)
    s["facts_fact"] = layers.get(Layer.FACT, 0)
    s["facts_reasoning"] = layers.get(Layer.REASONING, 0)

    # ── Fact-shape breakdowns ────────────────────────────────────
    arities = Counter(len(f.args) for f in kb.facts)
    s["fact_arity_hist"] = dict(arities)
    s["facts_with_nested_args"] = sum(
        1 for f in kb.facts if _has_nested_fact_args(f)
    )
    s["facts_negated"] = sum(1 for f in kb.facts if f.relation_name == "not")

    # ── Provenance kinds ─────────────────────────────────────────
    prov_kinds: Counter[str] = Counter()
    for f in kb.facts:
        if f.provenance is None:
            prov_kinds["<none>"] += 1
        else:
            prov_kinds[f.provenance.kind] += 1
    s["provenance_kinds"] = dict(prov_kinds)

    # ── Facts by relation ────────────────────────────────────────
    s["facts_by_relation"] = Counter(f.relation_name for f in kb.facts)

    # ── Reverse-index sizes (sanity for the kb's bookkeeping) ────
    s["index_facts_by_relation"] = sum(
        len(v) for v in kb._facts_by_relation.values()
    )
    s["index_facts_by_instance"] = sum(
        len(v) for v in kb._facts_by_instance.values()
    )
    s["index_rule_apps_by_rule"] = sum(
        len(v) for v in kb._rule_apps_by_rule.values()
    )
    s["index_rule_apps_on_relation"] = sum(
        len(v) for v in kb._rule_apps_on_relation.values()
    )

    # ── Engine cache (compiled (rule, activator) plans) ──────────
    if eng is not None:
        s["engine_cache_size"] = len(eng.cache)
        s["engine_fired"] = len(eng._fired)

    return s


# ── Pretty printing ─────────────────────────────────────────────


def _fmt_int(n: int) -> str:
    return f"{n:>6d}"


def _fmt_delta(d: int) -> str:
    if d == 0:
        return "      "  # blank-ish
    sign = "+" if d > 0 else ""
    return f"{sign}{d:>5d}"


SCALAR_KEYS = [
    ("types",                          "types (kernel)"),
    ("instances",                      "instances (kernel)"),
    ("relations",                      "relations (total)"),
    ("relations_declared",             "  declared"),
    ("relations_open_world",           "  open-world (auto-vivified)"),
    ("rules",                          "rules"),
    (None,                             None),  # separator
    ("names_total",                    "names (global, encoding-agnostic)"),
    ("names_objects",                  "  category = object"),
    ("names_relations",                "  category = relation"),
    ("names_rules",                    "  category = rule"),
    ("names_as_head_total",            "  total head-participations"),
    ("names_as_arg_total",             "  total arg-participations"),
    ("names_head_only",                "  appearing only as head"),
    ("names_arg_only",                 "  appearing only as arg"),
    (None,                             None),
    ("facts_total",                    "facts (total)"),
    ("facts_ontology",                 "  layer = ONTOLOGY"),
    ("facts_fact",                     "  layer = FACT"),
    ("facts_reasoning",                "  layer = REASONING"),
    (None,                             None),
    ("facts_negated",                  "facts whose head is `not`"),
    ("facts_with_nested_args",         "facts with nested-Fact args (Q40)"),
    (None,                             None),
    ("index_facts_by_relation",        "index entries: facts_by_relation"),
    ("index_facts_by_instance",        "index entries: facts_by_instance"),
    ("index_rule_apps_by_rule",        "index entries: rule_apps_by_rule"),
    ("index_rule_apps_on_relation",    "index entries: rule_apps_on_relation"),
    (None,                             None),
    ("engine_cache_size",              "engine cache: (rule, activator) plans"),
    ("engine_fired",                   "engine cache: bindings fired"),
]


def print_snapshot(
    before: dict[str, Any] | None,
    after: dict[str, Any],
    *, title: str,
) -> None:
    print()
    print(f"── {title} ──")
    if before is None:
        # Single-column view.
        for key, label in SCALAR_KEYS:
            if key is None:
                print()
                continue
            if key not in after:
                continue
            print(f"  {label:<42s}  {_fmt_int(after[key])}")
    else:
        # Side-by-side with deltas.
        print(f"  {'':<42s}  {'before':>6}  {'after':>6}  {'Δ':>6}")
        for key, label in SCALAR_KEYS:
            if key is None:
                print()
                continue
            if key not in after:
                continue
            b = before.get(key, 0)
            a = after[key]
            print(
                f"  {label:<42s}  {_fmt_int(b)}  "
                f"{_fmt_int(a)}  {_fmt_delta(a - b)}"
            )

    # Dict-valued breakdowns.
    print()
    _print_dict_breakdown(
        "provenance kinds",
        before.get("provenance_kinds", {}) if before else None,
        after.get("provenance_kinds", {}),
    )
    _print_dict_breakdown(
        "fact arities (arity → count)",
        before.get("fact_arity_hist", {}) if before else None,
        after.get("fact_arity_hist", {}),
    )
    _print_dict_breakdown(
        "facts by relation",
        before.get("facts_by_relation", {}) if before else None,
        after.get("facts_by_relation", {}),
        limit=None,
    )


def _print_dict_breakdown(
    label: str,
    before: dict | None,
    after: dict,
    *, limit: int | None = None,
) -> None:
    print(f"  {label}:")
    keys = sorted(set(after) | (set(before) if before else set()))
    if limit is not None:
        keys = sorted(
            keys,
            key=lambda k: -(after.get(k, 0)),
        )[:limit]
    for k in keys:
        a = after.get(k, 0)
        if before is not None:
            b = before.get(k, 0)
            d = a - b
            print(
                f"    {k!s:<38s}  {_fmt_int(b)}  "
                f"{_fmt_int(a)}  {_fmt_delta(d)}"
            )
        else:
            print(f"    {k!s:<38s}  {_fmt_int(a)}")


# ── Firing analysis ─────────────────────────────────────────────


def print_firings(firings: Iterable[Firing]) -> None:
    firings = list(firings)
    total = len(firings)
    productive = sum(1 for f in firings if not f.redundant)
    redundant = total - productive

    per_rule_total: Counter[str] = Counter()
    per_rule_productive: Counter[str] = Counter()
    per_relation: Counter[str] = Counter()
    for f in firings:
        per_rule_total[f.rule] += 1
        if not f.redundant:
            per_rule_productive[f.rule] += 1
            per_relation[f.derived.relation_name] += 1

    print()
    print("── saturation: firing breakdown ──")
    print(f"  total firings              {_fmt_int(total)}")
    print(f"    productive (new fact)    {_fmt_int(productive)}")
    print(f"    redundant (already in KB){_fmt_int(redundant)}")

    print()
    print("  per-rule firings (rule → productive / total):")
    for rule in sorted(per_rule_total, key=lambda r: -per_rule_total[r]):
        prod = per_rule_productive.get(rule, 0)
        tot = per_rule_total[rule]
        print(f"    {rule:<38s}  {_fmt_int(prod)} / {_fmt_int(tot)}")

    print()
    print("  derived facts by relation:")
    for rel in sorted(per_relation, key=lambda r: -per_relation[r]):
        print(f"    {rel:<38s}  {_fmt_int(per_relation[rel])}")


# ── Saturated KB dump (--dump) ──────────────────────────────────


def _ir_text(node: object) -> str:
    """Compact one-line render of any IRNode. Falls back to str()."""
    from ein.ir.dump import dump_compact

    if node is None:
        return "<none>"
    try:
        return dump_compact(node)  # type: ignore[arg-type]
    except (TypeError, AttributeError):
        return str(node)


def _arg_text(a: object) -> str:
    """Render one fact argument as text. Nested Facts recurse."""
    if isinstance(a, Fact):
        return _fact_text(a)
    return str(a)


def _fact_text(f: Fact) -> str:
    """One-line compact form of a fact: `(rel arg1 arg2 …)`."""
    if not f.args:
        return f"({f.relation_name})"
    return f"({f.relation_name} {' '.join(_arg_text(a) for a in f.args)})"


def _fact_text_with_provenance(f: Fact) -> str:
    """Compact form with `:source` / `:rule` annotation appended."""
    base = _fact_text(f)[1:-1]  # strip outer parens
    if f.provenance is None:
        return f"({base})"
    p = f.provenance
    if p.kind == "source" and p.source:
        return f'({base} :source "{p.source}")'
    if p.kind == "rule" and p.rule:
        return f"({base} :rule {p.rule})"
    if p.kind == "hypothesis":
        return f"({base} :hypothesis {p.branch})"
    if p.kind == "rejected":
        return f"({base} :rejected {p.branch})"
    return f"({base})"


def dump_kb(kb: KnowledgeBase) -> None:
    """Print the saturated KB as text — schema + facts by layer.

    Output is **readable** but not strictly round-trippable through
    the parser (the `:source` / `:rule` annotation is grammar-clean;
    the `:using` premises chain is omitted because the M1 grammar
    doesn't accept its compact-form value — see S1.2.3 T1.2.3.4).
    """
    print()
    print("=" * 70)
    print("=  SATURATED KB DUMP")
    print("=" * 70)

    # ── Schema (type/instance facts / relations / rules) ─────────
    # S1.7.23 — `(type …)` / `(instance …)` are ordinary facts now;
    # dump them straight from the fact index, not a type/instance view.
    type_facts = kb._facts_by_relation.get("type", ())
    if type_facts:
        print()
        print(f";; Type facts ({len(type_facts)})")
        for f in type_facts:
            print(f"(type {' '.join(str(a) for a in f.args)})")

    instance_facts = kb._facts_by_relation.get("instance", ())
    if instance_facts:
        print()
        print(f";; Instance facts ({len(instance_facts)})")
        for f in instance_facts:
            print(f"(instance {' '.join(str(a) for a in f.args)})")

    if kb.relations:
        declared = [r for r in kb.relations.values() if r.declared]
        open_w = [r for r in kb.relations.values() if not r.declared]
        if declared:
            print()
            print(f";; Relations — declared ({len(declared)})")
            for r in declared:
                sig = " ".join(r.signature) if r.signature else ""
                print(f"(relation {r.name}{(' ' + sig) if sig else ''})")
        if open_w:
            print()
            print(f";; Relations — auto-vivified ({len(open_w)})")
            for r in open_w:
                print(f";;   {r.name}")

    if kb.rules:
        print()
        print(f";; Rules ({len(kb.rules)})")
        for rule in kb.rules.values():
            band = _band_label(rule.priority)
            params = " ".join(f"?{p}" for p in rule.params)
            print(
                f";;   {rule.name}  :priority {rule.priority} ({band})"
                f"  :params ({params})"
            )

    # ── Facts grouped by layer ───────────────────────────────────
    layer_buckets: dict[Layer, list[Fact]] = {
        Layer.ONTOLOGY: [],
        Layer.FACT: [],
        Layer.REASONING: [],
    }
    for f in kb.facts:
        layer_buckets[f.layer].append(f)

    for layer, label in [
        (Layer.ONTOLOGY,  "ONTOLOGY"),
        (Layer.FACT,      "FACT"),
        (Layer.REASONING, "REASONING"),
    ]:
        facts = layer_buckets[layer]
        if not facts:
            continue
        print()
        print(f";; ── {label} ({len(facts)} facts) ──")
        # Group within a layer by relation name so the eye groups by
        # what's happening; preserve insertion order within each group.
        groups: dict[str, list[Fact]] = {}
        for f in facts:
            groups.setdefault(f.relation_name, []).append(f)
        for rel in sorted(groups):
            facts_for_rel = groups[rel]
            if len(facts_for_rel) > 1:
                print(f";;   {rel} ({len(facts_for_rel)})")
            for f in facts_for_rel:
                print("  " + _fact_text_with_provenance(f))

    # ── Query ────────────────────────────────────────────────────
    if kb.query is not None:
        print()
        print(";; Query")
        for kp in kb.query.kw_pairs:
            if hasattr(kp, "key"):
                print(f";;   :{kp.key.name}  {_ir_text(kp.value)}")

    print()
    print("=" * 70)


def _band_label(priority: int | None) -> str:
    """Human-readable Q41 priority-band name."""
    if priority is None:
        return "unbanded"
    if priority < 200:
        return "propagate"
    if priority < 300:
        return "derive"
    if priority < 900:
        return "eliminate"
    return "hypothesis"


# ── Main ────────────────────────────────────────────────────────


def bench(
    path: Path,
    *,
    dump: bool = False,
    max_steps: int | None = None,
    progress_every: int = 500,
) -> None:
    src = path.read_text()
    print(f"input:   {path}")
    print(f"         {len(src)} chars")

    # ── Phase: parse ─────────────────────────────────────────────
    t0 = time.perf_counter()
    forms = parse(src)
    t1 = time.perf_counter()
    print(f"parse:    {(t1 - t0) * 1000:8.2f} ms  ({len(forms)} top-level forms)")

    # ── Phase: kb load ───────────────────────────────────────────
    t0 = time.perf_counter()
    kb = KnowledgeBase.from_ir(forms)
    t1 = time.perf_counter()
    print(f"kb load:  {(t1 - t0) * 1000:8.2f} ms")

    # ── Phase: engine compile ────────────────────────────────────
    eng = Engine(kb)
    t0 = time.perf_counter()
    eng.compile_all()
    t1 = time.perf_counter()
    print(f"compile:  {(t1 - t0) * 1000:8.2f} ms")

    before = snapshot(kb, eng)
    print_snapshot(None, before, title="state BEFORE saturation")

    # ── Phase: saturate ──────────────────────────────────────────
    sat = Saturator(kb, engine=eng)
    from ein.inference.saturator import SaturatorStepLimitError

    print()
    if max_steps is not None:
        print(f"saturate: running with max_steps={max_steps}, "
              f"progress every {progress_every} steps")
    else:
        print("saturate: running unbounded")

    firings: list[Firing] = []
    per_rule: Counter[tuple[str, bool]] = Counter()
    t_start = time.perf_counter()
    t_mark = t_start
    hit_limit = False
    try:
        for i, f in enumerate(sat.saturate(max_steps=max_steps)):
            firings.append(f)
            per_rule[(f.rule, f.redundant)] += 1
            if progress_every > 0 and (i + 1) % progress_every == 0:
                t_now = time.perf_counter()
                print(
                    f"  step {i + 1:6d}  "
                    f"Δ={ (t_now - t_mark) * 1000:8.2f} ms  "
                    f"facts={len(kb.facts):6d}  "
                    f"last={f.rule!r}"
                    f"{' [redundant]' if f.redundant else ''}"
                )
                t_mark = t_now
    except SaturatorStepLimitError as e:
        hit_limit = True
        print()
        print(f"!! saturator step limit hit: {e}")
    t_end = time.perf_counter()
    print()
    print(f"saturate: {(t_end - t_start) * 1000:8.2f} ms  ({len(firings)} firings)")

    after = snapshot(kb, eng)
    print_snapshot(before, after, title="state AFTER saturation")
    print_firings(firings)

    if hit_limit:
        print()
        print("── per-rule firing breakdown at limit ──")
        for (rule, redundant), n in per_rule.most_common():
            tag = "redundant" if redundant else "productive"
            print(f"  {rule:30s} [{tag:10s}] {n:6d}")

    if dump:
        dump_kb(kb)


def main(argv: list[str] | None = None) -> int:
    import argparse

    p = argparse.ArgumentParser(
        prog="ein saturate",
        description="Benchmark + state dump for the Saturator.",
    )
    p.add_argument(
        "file",
        help="path to a .ein file",
    )
    p.add_argument(
        "--dump",
        action="store_true",
        help="after the benchmark, print the saturated KB grouped by layer",
    )
    p.add_argument(
        "--max-steps",
        type=int,
        default=None,
        help="hard cap on saturator firings (raises SaturatorStepLimitError "
             "when exceeded); useful for runaway-debugging on a fresh "
             "input. Default: no cap.",
    )
    p.add_argument(
        "--progress-every",
        type=int,
        default=500,
        help="log a one-line progress sample every N steps "
             "(0 disables; default: 500).",
    )
    args = p.parse_args(argv)

    target = Path(args.file)
    if not target.exists():
        print(f"error: {target} not found", file=sys.stderr)
        return 1
    bench(
        target,
        dump=args.dump,
        max_steps=args.max_steps,
        progress_every=args.progress_every,
    )
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
