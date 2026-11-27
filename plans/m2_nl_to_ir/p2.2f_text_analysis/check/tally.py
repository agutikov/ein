#!/usr/bin/env python3
"""The table's counts: `python3 tally.py [cells.tsv]`, a Markdown table of what each column reads right, wrong and
not at all, by class and in all; the models twice (as the classes define a fault, and on the reading alone).
`python3 tally.py --cells` prints the cells themselves, a Markdown table a text (check/README.md's § 3).

Each tool is also set against the model on the cells its layer speaks to (the stage's default rule: a tool goes on if
it fits 8 GB and is right on more cells than the model's step 1): AMR and the two UD parses on every cell, coreference
on the coreference cells, word senses on the kind cells. Standard library only.
"""

import os
import sys

MODELS = ["M1", "M2"]
TOOLS = ["xfm", "spring", "ibm", "stanza", "spacy", "coref", "wsd"]
SCOPE = {"coref": {"coreference"}, "wsd": {"kind"}}
CLASSES = ["name", "voice", "negation", "kind", "number", "unstated", "coreference", "order", "question"]


def read(path):
    rows, header = [], None
    for line in open(path, encoding="utf-8"):
        line = line.rstrip("\n")
        if not line or line.startswith("#"):
            continue
        cols = line.split("\t")
        if header is None:
            header = cols
            continue
        cols += [""] * (len(header) - len(cols))
        rows.append(dict(zip(header, cols)))
    return rows


def verdict(v, reading=False):
    v = v.strip()
    if "|" in v:
        strict, lenient = v.split("|", 1)
        return lenient if reading else strict
    return {"-": "N", "": "?"}.get(v, v)


def count(rows, col, reading=False, classes=None):
    c = {"R": 0, "W": 0, "N": 0, "n/a": 0, "?": 0}
    for r in rows:
        if classes and r["class"] not in classes:
            continue
        c[verdict(r[col], reading)] = c.get(verdict(r[col], reading), 0) + 1
    return c


def fmt(c):
    s = f"{c['R']} / {c['W']} / {c['N']}"
    if c.get("n/a"):
        s += f" ({c['n/a']} n/a)"
    if c.get("?"):
        s += f" ({c['?']} unread)"
    return s


def cells(rows):
    """The cells as Markdown, a table a text, the sentence quoted above its cells."""
    here = os.path.dirname(os.path.abspath(__file__))
    sys.path.insert(0, os.path.join(here, "..", "tools"))
    sys.dont_write_bytecode = True
    import texts  # noqa: E402
    sents = {t["name"]: t["sentences"] for t in texts.all_texts()}
    cols = ["M1", "M2"] + TOOLS

    def md(v):
        return v.replace("|", "\\|")

    for name in texts.NAMES:
        sub = [r for r in rows if r["text"] == name]
        print(f"#### `{name}`\n")
        print("| id | s | class | the hand's step 1 | " + " | ".join(cols) + " | the sentence; note |")
        print("|---|---:|---|---|" + "---|" * len(cols) + "---|")
        for r in sub:
            s = sents[name][int(r["s"]) - 1]
            print(f"| {r['id']} | {r['s']} | {r['class']} | {md(r['ref'])} | " + " | ".join(md(r[c]) for c in cols)
                  + f" | *{md(s)}* {md(r['note'])} |")
        print()


def main():
    args = [a for a in sys.argv[1:] if not a.startswith("--")]
    path = args[0] if args else os.path.join(os.path.dirname(os.path.abspath(__file__)), "cells.tsv")
    rows = read(path)
    if "--cells" in sys.argv:
        cells(rows)
        return
    cols = [("M1", False), ("M1", True), ("M2", False), ("M2", True)] + [(t, False) for t in TOOLS]
    names = ["M1", "M1 reading", "M2", "M2 reading"] + TOOLS
    print(f"{len(rows)} cells. Each entry: right / wrong / not carried.\n")
    print("| class | cells | " + " | ".join(names) + " |")
    print("|---|---:|" + "---|" * len(names))
    for cl in CLASSES + [None]:
        sub = [r for r in rows if cl is None or r["class"] == cl]
        if not sub:
            print(f"| {cl} | 0 | " + " | ".join("no case" for _ in names) + " |")
            continue
        label = "**all**" if cl is None else cl
        print(f"| {label} | {len(sub)} | " + " | ".join(fmt(count(sub, c, rd)) for c, rd in cols) + " |")
    print("\nEach tool against the model on the cells its layer speaks to (right cells; the model's on the same cells):\n")
    print("| tool | cells | tool right | tool wrong | M1 right (reading) | M2 right (reading) |")
    print("|---|---:|---:|---:|---:|---:|")
    for t in TOOLS:
        sub = [r for r in rows if t not in SCOPE or r["class"] in SCOPE[t]]
        ct = count(sub, t)
        m1, m1r = count(sub, "M1"), count(sub, "M1", True)
        m2, m2r = count(sub, "M2"), count(sub, "M2", True)
        print(f"| {t} | {len(sub)} | {ct['R']} | {ct['W']} | {m1['R']} ({m1r['R']}) | {m2['R']} ({m2r['R']}) |")
    print("\nCell by cell against M1 on the reading alone (the cells where one is right and the other is not):\n")
    print("| tool | tool right, M1 not | M1 right, tool not | of which the tool wrong |")
    print("|---|---|---|---|")
    for t in TOOLS:
        sub = [r for r in rows if t not in SCOPE or r["class"] in SCOPE[t]]
        gain = [r["id"] for r in sub if verdict(r[t]) == "R" and verdict(r["M1"], True) != "R"]
        loss = [r["id"] for r in sub if verdict(r["M1"], True) == "R" and verdict(r[t]) != "R"]
        lossw = [i for i in loss if verdict(next(r for r in sub if r["id"] == i)[t]) == "W"]
        print(f"| {t} | {len(gain)}: {' '.join(gain)} | {len(loss)}: {' '.join(loss)} | {len(lossw)}: {' '.join(lossw)} |")


if __name__ == "__main__":
    main()
