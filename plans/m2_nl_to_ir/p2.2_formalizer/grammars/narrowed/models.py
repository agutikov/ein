#!/usr/bin/env python3
"""models.py — every program the model wrote in S2.2.2's and S2.2.3's records that loaded, read by the narrowed
root and by the root before it.

    python3 models.py [--einaf einaf/target/release/einaf] [--before REV] > models.txt

A record line's program is the reading the probe judged that loaded (its fenced block where it has one, else the
answer whole). Each is written to a scratch file and read by `einaf grammar accept --root root`, once with the whole
grammar the binary writes and once with the file `einaf/grammars/ein.gbnf` held at REV (98e6e0a3, the grammar before
the narrowing); a program the narrowed root refuses is listed with the first character it holds that the root leaves
out, and a form the contract does not admit, read off its tokens as `einaf/tests/grammar.rs` reads them. Standard
library only.
"""

from __future__ import annotations

import argparse
import gzip
import json
import subprocess
import tempfile
from pathlib import Path

HERE = Path(__file__).resolve().parent
ROOT = HERE.parents[4]
P22 = HERE.parents[1]
RECORDS = [
    ("S2.2.2, the probe", P22 / "model_runtime" / "probe.jsonl.gz"),
    ("S2.2.3, the first plan", P22 / "grammars" / "first" / "record.jsonl.gz"),
    ("S2.2.3, the second plan", P22 / "grammars" / "fenced" / "record.jsonl.gz"),
    ("S2.2.3, the third plan", P22 / "grammars" / "after" / "record.jsonl.gz"),
]
EXCLUDED = set("<>|`")
NAME = set("ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789_.*+=-")
NOT_THE_CONTRACTS = {"config", "trace", "macro", "program", "module", "commit"}


def lines(path: Path):
    with gzip.open(path, "rt") as f:
        for l in f:
            if l.strip():
                yield json.loads(l)


def outside_the_root(t: str) -> str | None:
    """The first thing a text holds that the narrowed root leaves out, as the EinAf test's scan reads it."""
    depth, i, n = 0, 0, len(t)
    heads = []
    found = None
    while i < n:
        c = t[i]
        if c == ";":
            j = t.find("\n", i)
            j = n if j < 0 else j
            bad = [x for x in t[i:j] if x in EXCLUDED]
            if bad and found is None:
                found = f"{bad[0]!r} in a comment"
            i = j
        elif t.startswith("#|", i):
            if found is None:
                found = "a block comment"
            j = t.find("|#", i + 2)
            i = n if j < 0 else j + 2
        elif c == '"':
            j = i + 1
            while j < n and t[j] != '"':
                j += 2 if t[j] == "\\" else 1
            bad = [x for x in t[i + 1:j] if x in EXCLUDED]
            if bad and found is None:
                found = f"{bad[0]!r} in a string"
            i = j + 1
        elif c in "()":
            if c == "(":
                if depth == 0:
                    k = i + 1
                    while k < n and not (t[k].isspace() or t[k] in '()";'):
                        k += 1
                    heads.append(t[i + 1:k])
                depth += 1
            else:
                depth -= 1
            i += 1
        elif c.isspace():
            i += 1
        else:
            j = i
            while j < n and not (t[j].isspace() or t[j] in '();"' or t.startswith("#|", j)):
                j += 1
            tok = t[i:j]
            body = tok[1:] if tok.startswith("?") else tok
            bad = [x for x in body if x not in NAME]
            if not tok.startswith(":") and bad and found is None:
                found = f"{bad[0]!r} in the name `{tok}`"
            i = j
    forms = [f"a `{h}` form" for h in heads if h in NOT_THE_CONTRACTS]
    return "; ".join(([found] if found else []) + forms[:1]) or None


def accept(einaf: str, files: list[Path], grammar: Path | None) -> dict[str, str]:
    args = [einaf, "grammar", "accept", "--root", "root"]
    if grammar:
        args += ["--grammar", str(grammar)]
    out = subprocess.run(args + [str(f) for f in files], capture_output=True, text=True).stdout
    verdicts = {}
    for line in out.splitlines():
        path, _, v = line.partition(": ")
        verdicts[path] = v
    return verdicts


def main() -> None:
    ap = argparse.ArgumentParser(description=__doc__.split("\n\n")[0])
    ap.add_argument("--einaf", default=str(ROOT / "einaf/target/release/einaf"))
    ap.add_argument("--before", default="98e6e0a3")
    a = ap.parse_args()
    with tempfile.TemporaryDirectory() as tmp:
        tmp = Path(tmp)
        old = tmp / "before.gbnf"
        old.write_text(subprocess.run(["git", "show", f"{a.before}:einaf/grammars/ein.gbnf"], cwd=ROOT,
                                      check=True, capture_output=True, text=True).stdout)
        programs = []
        for label, path in RECORDS:
            for l in lines(path):
                pid = l["planned"]["id"]
                loaded = [j for j in l.get("judged") or [] if j.get("loaded")]
                if not loaded:
                    continue
                j = next((j for j in loaded if j["reading"] == "fenced"), loaded[0])
                f = tmp / f"{len(programs):03d}.ein"
                f.write_text(j["text"])
                programs.append((label, pid, j["reading"], f, j["text"]))
        files = [p[3] for p in programs]
        new = accept(a.einaf, files, None)
        before = accept(a.einaf, files, old)
        rows = []
        counts = {}
        for label, pid, reading, f, text in programs:
            v_new, v_old = new[str(f)], before[str(f)]
            why = outside_the_root(text) if v_new != "accepted" else None
            rows.append((label, pid, reading, v_old, v_new, why))
            key = (label, v_new == "accepted", v_old == "accepted")
            counts[key] = counts.get(key, 0) + 1
    print(f"# The programs the model wrote that loaded, read by the root (before: `{a.before}`'s file)\n")
    print(f"{len(rows)} programs; the narrowed root accepts {sum(r[4] == 'accepted' for r in rows)}, "
          f"the root before it {sum(r[3] == 'accepted' for r in rows)}.\n")
    print("| record | programs | accepted by both | refused by both | accepted before, refused now |")
    print("|---|---:|---:|---:|---:|")
    for label, _ in RECORDS:
        n = sum(1 for r in rows if r[0] == label)
        both = counts.get((label, True, True), 0)
        neither = counts.get((label, False, False), 0)
        now = counts.get((label, False, True), 0)
        print(f"| {label} | {n} | {both} | {neither} | {now} |")
    print("\n| record | run | reading | the root before | the narrowed root | what it holds that the root leaves out |")
    print("|---|---|---|---|---|---|")
    for label, pid, reading, v_old, v_new, why in rows:
        if v_new != "accepted" or v_old != "accepted":
            print(f"| {label} | `{pid}` | {reading} | {v_old} | {v_new} | {why or '—'} |")


if __name__ == "__main__":
    main()
