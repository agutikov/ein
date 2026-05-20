#!/usr/bin/env python3
"""Wall-clock benchmark + state dump for the Saturator on a .ein file.

Informational (S1.3.3 acceptance) — not a CI gate. Run from repo
root:

    python scripts/bench_saturate.py [example.ein]

Prints, in order:

1. Phase timings — parse, kb-load, compile, saturate.
2. A full state snapshot **before** saturation: every counter the
   KB and Engine expose.
3. A full state snapshot **after** saturation, showing deltas
   (Δ columns) alongside the absolutes.
4. Saturation-specific stats: total / productive / redundant
   firings, per-rule firing counts, per-relation derived counts.

The snapshot is intentionally exhaustive — it's both a benchmark
and a diagnostic for "what does the engine actually do on this
input?".

Target on a laptop: < 200 ms wall-clock for the Zebra initial
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

REPO = Path(__file__).resolve().parent.parent
sys.path.insert(0, str(REPO / "src"))

from ein_bot.inference.engine import Engine  # noqa: E402
from ein_bot.inference.firing import Firing  # noqa: E402
from ein_bot.inference.saturator import Saturator  # noqa: E402
from ein_bot.ir import parse  # noqa: E402
from ein_bot.kb.entities import Fact, Layer  # noqa: E402
from ein_bot.kb.store import KnowledgeBase  # noqa: E402

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
    s["types"] = len(kb.types)
    s["instances"] = len(kb.instances)
    s["relations"] = len(kb.relations)
    s["relations_declared"] = sum(1 for r in kb.relations.values() if r.declared)
    s["relations_open_world"] = sum(
        1 for r in kb.relations.values() if not r.declared
    )
    s["rules"] = len(kb.rules)

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
    ("types",                          "types"),
    ("instances",                      "instances"),
    ("relations",                      "relations (total)"),
    ("relations_declared",             "  declared"),
    ("relations_open_world",           "  open-world (auto-vivified)"),
    ("rules",                          "rules"),
    (None,                             None),  # separator
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


# ── Main ────────────────────────────────────────────────────────


def bench(path: Path) -> None:
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
    t0 = time.perf_counter()
    firings = list(sat.saturate())
    t1 = time.perf_counter()
    print()
    print(f"saturate: {(t1 - t0) * 1000:8.2f} ms")

    after = snapshot(kb, eng)
    print_snapshot(before, after, title="state AFTER saturation")
    print_firings(firings)


if __name__ == "__main__":
    target = (
        Path(sys.argv[1]) if len(sys.argv) > 1
        else REPO / "examples" / "zebra.ein"
    )
    if not target.exists():
        print(f"error: {target} not found", file=sys.stderr)
        sys.exit(1)
    bench(target)
