#!/usr/bin/env python3
"""plan.py — the narrowed root's speed, written before its run.

    python3 plan.py [--einaf einaf/target/release/einaf]

Writes `speed/requests.jsonl.gz` and `speed/order.txt` beside this script: five requests for the Zebra puzzle, each
S2.2.3's speed request (`../speed/requests.jsonl.gz`, the probe's `step1.off.zebra` request, greedy, seed 1,
thinking off, 256 tokens at most, the grammar from the first token in the fenced frame), three times each in a row:
`zebra.none` (no grammar) and `zebra.whole.every-token` and `zebra.whole.chosen-first` (the root before the
narrowing) as S2.2.3 wrote them, byte for byte; and `zebra.root.every-token` and `zebra.root.chosen-first`, the same
request with the narrowed root, `einaf grammar render whole --fence --request`. So the same session takes the root
before and after, and no grammar beside them. gzip with no time stamp.
"""

from __future__ import annotations

import argparse
import gzip
import json
import subprocess
from pathlib import Path

HERE = Path(__file__).resolve().parent
ROOT = HERE.parents[4]
REPEATS = 3


def write_gz(path: Path, text: str) -> None:
    with open(path, "wb") as raw, gzip.GzipFile(fileobj=raw, mode="wb", mtime=0) as f:
        f.write(text.encode())


def main() -> None:
    ap = argparse.ArgumentParser(description=__doc__.split("\n\n")[0])
    ap.add_argument("--einaf", default=str(ROOT / "einaf/target/release/einaf"))
    a = ap.parse_args()
    s223 = {}
    with gzip.open(HERE.parent / "speed" / "requests.jsonl.gz", "rt") as f:
        for line in f:
            e = json.loads(line)
            s223[e["name"]] = e["request"]
    lines, order = [], []
    for name in ["zebra.none", "zebra.whole.every-token", "zebra.whole.chosen-first"]:
        lines.append({"name": name, "request": s223[name]})
    g = json.loads(subprocess.run([a.einaf, "grammar", "render", "whole", "--fence", "--request"], cwd=ROOT,
                                  check=True, capture_output=True, text=True).stdout)
    for check in ["every-token", "chosen-first"]:
        req = json.loads(json.dumps(s223[f"zebra.whole.{check}"]))
        grammar = json.loads(json.dumps(g))
        if check == "every-token":
            grammar.pop("check", None)
        else:
            grammar["check"] = check
        req["grammar"] = grammar
        lines.append({"name": f"zebra.root.{check}", "request": req})
    for e in lines:
        order += [e["name"]] * REPEATS
    write_gz(HERE / "speed" / "requests.jsonl.gz",
             "".join(json.dumps(e, ensure_ascii=False) + "\n" for e in lines))
    (HERE / "speed" / "order.txt").write_text("\n".join(order) + "\n")
    print(f"plan.py: {len(lines)} requests, {len(order)} calls")


if __name__ == "__main__":
    main()
