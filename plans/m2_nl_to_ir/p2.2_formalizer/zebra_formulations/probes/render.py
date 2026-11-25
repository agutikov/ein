#!/usr/bin/env python3
"""The renderings of the analysis page, written to ../renderings/.

For each encoding: `ein render program`, split into its digraphs, the query's and the
one rule that reads a sentence into the theory drawn to SVG with `dot`; the knowledge
base's digraph, every other rule's, `ein render kb` and `ein render h-space --view
exclusion` counted (sizes.json) and drawn only where small enough to read; and
`ein render h-space`'s read-out as text.

    python3 render.py          # $EIN_BIN, default ein.rs/target/release/ein
"""
from __future__ import annotations

import json
import os
import re
import subprocess
from pathlib import Path

HERE = Path(__file__).resolve().parent
REPO = HERE.parents[4]
OUT = HERE.parent / "renderings"
EIN = os.environ.get("EIN_BIN", str(REPO / "ein.rs/target/release/ein"))
ENCODINGS = ["zebra", "zebra2", "zebra3.1", "zebra3.2", "zebra3.3", "zebra3.4"]
# The rule drawn beside the query: the one that reads a sentence into the theory.
DRAWN = {"zebra": "rule_guess_lhs_rhs", "zebra2": "rule_co_located_lhs_rhs",
         "zebra3.2": "rule_the_thing_described_lhs_rhs",
         "zebra3.3": "rule_a_house_of_a_colour_lhs_rhs", "zebra3.4": "rule_same_house_lhs_rhs"}
# Small enough to read.
MAX_NODES, MAX_EDGES = 120, 300


def ein(*args: str) -> str:
    return subprocess.run([EIN, *args], cwd=REPO, capture_output=True, text=True, timeout=600).stdout


def graphs(dot: str) -> list[tuple[str, str]]:
    out = []
    for p in re.split(r"(?m)^(?=(?:di)?graph\s)", dot):
        m = re.match(r"(?:di)?graph\s+([^\s{]+)", p)
        if m:
            out.append((m.group(1), p))
    return out


def size(dot: str) -> dict:
    nodes = set(re.findall(r'(?m)^\s*("[^"]+"|n\d+)\s*\[', dot))
    return {"bytes": len(dot.encode()), "nodes": len(nodes),
            "edges": len(re.findall(r" -[->] ", dot)), "lines": dot.count("\n")}


def draw(dot: str, svg: Path) -> None:
    subprocess.run(["dot", "-Tsvg", "-o", str(svg)], input=dot, text=True, check=True, timeout=300)


def main() -> None:
    OUT.mkdir(exist_ok=True)
    sizes: dict = {}
    for name in ENCODINGS:
        path = f"examples/{name}.ein"
        s: dict = {"program": {}}
        for gname, g in graphs(ein("render", "program", path)):
            s["program"][gname] = size(g)
            if gname == "query" or gname == DRAWN.get(name):
                draw(g, OUT / f"{name}.program.{gname}.svg")
        s["kb"] = size(ein("render", "kb", path))
        (OUT / f"{name}.h-space.txt").write_text(ein("render", "h-space", path))
        ex = ein("render", "h-space", path, "--view", "exclusion")
        s["h-space-exclusion"] = size(ex)
        if s["h-space-exclusion"]["nodes"] <= MAX_NODES and s["h-space-exclusion"]["edges"] <= MAX_EDGES:
            draw(ex, OUT / f"{name}.h-space.exclusion.svg")
        sizes[name] = s
        print(name, "kb", s["kb"], "program kb", s["program"].get("kb"), "exclusion", s["h-space-exclusion"])
    (OUT / "sizes.json").write_text(json.dumps(sizes, indent=1) + "\n")


if __name__ == "__main__":
    main()
