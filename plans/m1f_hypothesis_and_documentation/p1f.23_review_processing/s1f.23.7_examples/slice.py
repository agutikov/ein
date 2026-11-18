#!/usr/bin/env python3
"""S1f.23.7 task 1: a goal's slice of a knowledge base, drawn — a MOCK built from today's drawing.

    python3 slice.py --bin BIN --program FILE --goal '(…)' --name NAME [--out DIR]

Nothing here is built into `ein`. What is the binary's and what is the mock:

1. the binary: a copy of FILE with `(query :goal GOAL)` appended is solved,
   `ein solve COPY --dump-answer DIR`, and the query's one class is read off its
   `as :expect`: the facts the goal selects, in the one state of R;
2. the binary: that state, `DIR/models/01.ein`, a saved file, is drawn as today,
   `ein render kb DIR/models/01.ein`;
3. **the mock**: of that drawing, the octagons of the selected facts are kept,
   with every arrow they draw and every fact and atom those arrows reach, at any
   depth; a fact that stands in a frame keeps one, labelled with the relations of
   the kept facts that point at it; nothing else is kept, and the graph is
   labelled MOCK;
4. the binary: `ein render constraints FILE` beside it, as today.

Writes NAME.slice.dot, NAME.constraints.dot and an .svg of each (Graphviz `dot`).
"""
from __future__ import annotations

import argparse
import re
import subprocess
import sys
import tempfile
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
import eintext as et  # noqa: E402

REPO = Path(__file__).resolve().parents[4]


def sh(argv: list[str]) -> str:
    p = subprocess.run(argv, capture_output=True, text=True, cwd=REPO)
    if p.returncode != 0:
        sys.exit(f"{' '.join(argv)}: exit {p.returncode}\n{p.stderr}")
    return p.stdout


def last_class(out: str) -> list[str]:
    """The facts of the last query's one class, from its `as :expect`."""
    blocks = re.split(r"\n  query \d+ of \d+", out)
    block = blocks[-1]
    at = block.find("as :expect")
    if at < 0:
        sys.exit("the query printed no `as :expect`: no class to draw\n" + block)
    text = block[at + len("as :expect"):]
    claim = et.parse(text)[0]
    if claim[0] != "model":
        sys.exit(f"the query has more than one class, or none: {et.canon(claim)}")
    return [et.canon(f) for f in claim[1:]]


def cut(dot: str, keep_texts: set[str], goal: str, source: str) -> tuple[str, int, int]:
    d = et.read_dot(dot)
    keep = {n for n, t in d.text.items() if t in keep_texts}
    missing = keep_texts - {d.text[n] for n in keep}
    if missing:
        sys.exit(f"facts of the class not in the drawing: {sorted(missing)}")
    todo = list(keep)
    while todo:
        n = todo.pop()
        for _, t in d.arrows.get(n, []):
            if t.startswith("f_") and t not in keep:
                keep.add(t)
                todo.append(t)
    head, body, clusters, framed, cluster = [], [], [], [], None
    in_body = False
    node_of = re.compile(r"^\s*(f_[0-9a-f]+)\b")
    for line in d.lines:
        s = line.strip()
        if not in_body:
            if s.startswith("// facts"):
                in_body = True
            elif s != "}" and not s.startswith("digraph"):
                head.append(line)
            continue
        if s.startswith("subgraph cluster_"):
            cluster = [line]
            continue
        if cluster is not None:
            cluster.append(line)
            if s == "}":
                framed.extend(c for c in cluster if node_of.match(c) and node_of.match(c).group(1) in keep)
                cluster = None
            continue
        m = node_of.match(line)
        if m and m.group(1) in keep:
            body.append(line)
    # A framed fact's frame names the relations of the facts that point at it
    # in the slice, not in the whole drawing: frames regrouped by that label.
    relation_of = {n: t[1:-1] for n in d.arrows for i, t in d.arrows[n] if i == 1}
    by_label: dict[str, list[str]] = {}
    for line in framed:
        n = node_of.match(line).group(1)
        rels = sorted({relation_of[p] for p in keep if any(t == n for _, t in d.arrows.get(p, []))})
        by_label.setdefault(", ".join(rels) + " ctx", []).append(line)
    for i, (label, members) in enumerate(by_label.items()):
        clusters.append(
            [f"  subgraph cluster_ctx_{i} {{",
             f'    label="{label}"; style="dashed,rounded"; color="#888888"; fontcolor="#888888";']
            + members + ["  }"]
        )
    atoms = {t for n in keep for _, t in d.arrows.get(n, []) if not t.startswith("f_")}
    out = ["digraph slice {",
           f'  label="MOCK, not built: the slice of {goal.replace(chr(34), chr(39))}\\ncut from {source}";',
           "  labelloc=t; fontname=\"Inter\";"]
    out += head
    out.append("")
    out.append("  // facts the goal selects, and what they point at")
    if clusters:
        out.append("  newrank=true;")
    out += body
    for c in clusters:
        out += c
    out.append("}")
    return "\n".join(out) + "\n", len(keep), len(atoms)


def main() -> int:
    ap = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    ap.add_argument("--bin", required=True)
    ap.add_argument("--program", required=True)
    ap.add_argument("--goal", required=True)
    ap.add_argument("--name", required=True)
    ap.add_argument("--out", default=str(Path(__file__).resolve().parent))
    args = ap.parse_args()
    bin_ = str(Path(args.bin).resolve())
    out = Path(args.out)
    with tempfile.TemporaryDirectory() as tmp:
        copy = Path(tmp) / Path(args.program).name
        copy.write_text((REPO / args.program).read_text() + f"\n(query :goal {args.goal})\n")
        solved = sh([bin_, "solve", str(copy), "--dump-answer", f"{tmp}/answer"])
        facts = last_class(solved)
        state = f"{tmp}/answer/models/01.ein"
        kb = sh([bin_, "render", "kb", state])
        whole = et.read_dot(kb)
        dot, kept, atoms = cut(kb, set(facts), args.goal, f"ein render kb, the state of R ({len(whole.arrows)} facts drawn)")
    (out / f"{args.name}.slice.dot").write_text(dot)
    cons = sh([bin_, "render", "constraints", args.program])
    (out / f"{args.name}.constraints.dot").write_text(cons)
    for kind in ("slice", "constraints"):
        sh(["dot", "-Tsvg", str(out / f"{args.name}.{kind}.dot"), "-o", str(out / f"{args.name}.{kind}.svg")])
    print(f"{args.name}: the goal selects {len(facts)} facts; the slice draws {kept} facts and {atoms} atoms "
          f"of the state's {len(whole.arrows)} octagons; the constraint view draws {len(et.read_dot(cons).arrows)} facts")
    for f in facts:
        print(f"  {f}")
    return 0


if __name__ == "__main__":
    sys.exit(main())
