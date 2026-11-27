#!/usr/bin/env python3
"""Draw every AMR graph a run wrote: `python render_amr.py RUN_DIR …`.

For each `<text>.amr` (and `<text>.whole.amr`) in a run's folder, each graph is written as `svg/<id>.dot` and drawn
by Graphviz's `dot` into `svg/<id>.svg`: a concept a node (`v / concept`), a constant a box, a role an arrow, the
sentence as the picture's label. `index.md` beside them lists each sentence with its PENMAN and its picture.
"""

import os
import subprocess
import sys

import penman
from penman.models.noop import NoOpModel


def esc(s):
    return s.replace("\\", "\\\\").replace('"', '\\"')


def to_dot(g, title):
    lines = ["digraph amr {", '  rankdir=TB; node [fontname="Helvetica", fontsize=11]; edge [fontname="Helvetica", fontsize=10];',
             f'  labelloc=t; label="{esc(title)}";']
    for v, _, concept in g.instances():
        lines.append(f'  "{v}" [label="{esc(v)} / {esc(str(concept))}", shape=ellipse];')
    variables = set(g.variables())
    k = 0
    for src, role, tgt in g.edges():
        lines.append(f'  "{src}" -> "{tgt}" [label="{esc(role)}"];')
    for src, role, tgt in g.attributes():
        k += 1
        lines.append(f'  "c{k}" [label="{esc(str(tgt))}", shape=box];')
        lines.append(f'  "{src}" -> "c{k}" [label="{esc(role)}"];')
    lines.append("}")
    return "\n".join(lines) + "\n"


def main():
    for run in sys.argv[1:]:
        svg = os.path.join(run, "svg")
        os.makedirs(svg, exist_ok=True)
        index = [f"# {os.path.basename(run.rstrip('/'))}\n"]
        for name in sorted(os.listdir(run)):
            if not name.endswith(".amr"):
                continue
            stem = name[:-4]
            with open(os.path.join(run, name)) as f:
                blocks = [b for b in f.read().split("\n\n") if b.strip()]
            for i, block in enumerate(blocks, 1):
                try:
                    g = penman.decode(block, model=NoOpModel())
                except Exception as e:  # noqa: BLE001
                    index.append(f"## {stem}.{i}\n\nnot decoded: {e}\n")
                    continue
                gid = g.metadata.get("id", f"{stem}.{i}") if not stem.endswith(".whole") else stem
                snt = g.metadata.get("snt", "")
                dot = os.path.join(svg, f"{gid}.dot")
                with open(dot, "w") as f:
                    f.write(to_dot(g, snt))
                subprocess.run(["dot", "-Tsvg", dot, "-o", os.path.join(svg, f"{gid}.svg")], check=False)
                body = penman.encode(g, model=NoOpModel(), indent=6)
                index.append(f"## {gid}\n\n*{snt}*\n\n```\n{body}\n```\n\n![{gid}](svg/{gid}.svg)\n")
        with open(os.path.join(run, "index.md"), "w") as f:
            f.write("\n".join(index))
        print(f"{run}: {len(os.listdir(svg)) // 2} graphs drawn")


if __name__ == "__main__":
    main()
