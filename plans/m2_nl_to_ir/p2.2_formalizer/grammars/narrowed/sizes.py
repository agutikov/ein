#!/usr/bin/env python3
"""sizes.py — the grammars' sizes before the narrowing and after it.

    python3 sizes.py [--einaf einaf/target/release/einaf] [--before REV] > sizes.md

Before: the file `einaf/grammars/ein.gbnf` held at REV (98e6e0a3), and the step grammars for the Zebra puzzle as
S2.2.3's speed requests carry them (`../speed/requests.jsonl.gz`, in the fenced frame). After: the file the binary
writes, the text a request for a whole program carries (`einaf grammar render whole`), and the step grammars for the
Zebra puzzle rendered as S2.2.3's `plan.py` rendered them, in the fenced frame. A rule is a line `NAME ::= …`; a
rule's bytes are its lines but the comments above it; reachable is from the root by the names its rules write.
Standard library only.
"""

from __future__ import annotations

import argparse
import gzip
import json
import re
import subprocess
from pathlib import Path

HERE = Path(__file__).resolve().parent
ROOT = HERE.parents[4]
CONTRACT = "einaf/contract/zebra"


def rules(text: str) -> dict[str, str]:
    out, cur = {}, None
    for line in text.split("\n"):
        if line.startswith("#") or not line.strip():
            continue
        m = re.match(r"^([A-Za-z0-9-]+) ::= ", line)
        if m:
            cur = m.group(1)
            out[cur] = line
        elif cur:
            out[cur] += "\n" + line
    return out


def refs(body: str) -> set[str]:
    out, i = set(), 0
    while i < len(body):
        c = body[i]
        if c in '"[':
            end = '"' if c == '"' else "]"
            i += 1
            while i < len(body) and body[i] != end:
                i += 2 if body[i] == "\\" else 1
            i += 1
        elif c.isalnum() or c == "-":
            j = i
            while j < len(body) and (body[j].isalnum() or body[j] == "-"):
                j += 1
            out.add(body[i:j])
            i = j
        else:
            i += 1
    return out


def reach(text: str, root: str) -> tuple[int, int, dict[str, str]]:
    rs = rules(text)
    seen, todo = set(), [root]
    while todo:
        n = todo.pop()
        if n in seen or n not in rs:
            continue
        seen.add(n)
        todo.extend(refs(rs[n].split("::=", 1)[1]))
    return len(seen), sum(len(rs[n].encode()) + 1 for n in seen), {n: rs[n] for n in seen}


def tree(rs: dict[str, str], base: str) -> int:
    return sum(1 for n in rs if n == base or n.startswith(base + "-"))


def render(einaf: str, *args: str) -> str:
    g = json.loads(subprocess.run([einaf, "grammar", "render", *args, "--fence", "--request"], cwd=ROOT,
                                  check=True, capture_output=True, text=True).stdout)
    return g["text"]


def main() -> None:
    ap = argparse.ArgumentParser(description=__doc__.split("\n\n")[0])
    ap.add_argument("--einaf", default=str(ROOT / "einaf/target/release/einaf"))
    ap.add_argument("--before", default="98e6e0a3")
    a = ap.parse_args()
    before = subprocess.run(["git", "show", f"{a.before}:einaf/grammars/ein.gbnf"], cwd=ROOT, check=True,
                            capture_output=True, text=True).stdout
    after = (ROOT / "einaf/grammars/ein.gbnf").read_text()
    request = subprocess.run([a.einaf, "grammar", "render", "whole"], cwd=ROOT, check=True, capture_output=True,
                             text=True).stdout
    print("# The grammars' sizes, before the narrowing and after it\n")
    print("| grammar | before: bytes, rules | after: bytes, rules |")
    print("|---|---|---|")
    print(f"| the file `einaf/grammars/ein.gbnf`, both roots | {len(before.encode()):,}, {len(rules(before))} | "
          f"{len(after.encode()):,}, {len(rules(after))} |")
    print(f"| what a request for a whole program carries (`render whole`, no frame) | {len(before.encode()):,}, "
          f"{len(rules(before))} (the file) | {len(request.encode()):,}, {len(rules(request))} (`root` and its rules) |")
    for root in ["root", "program"]:
        (bn, bb, brs), (an, ab, ars) = reach(before, root), reach(after, root)
        print(f"| the rules `{root}` reaches, their bytes but comments | {bb:,}, {bn} | {ab:,}, {an} |")
        if root == "root":
            b_tree, a_tree = tree(brs, "tok-name"), tree(ars, "m-tok-name")
            b_wb = before.count("# after `<")
            print(f"| `root`'s name tree (`tok-name…`, then `m-tok-name…`) | {b_tree} rules, {b_wb} of them after "
                  f"`<`, the writeback names' | {a_tree} rules |")
            print(f"| `root`'s variable (`tok-var…`, then `m-tok-var…`) | {tree(brs, 'tok-var')} rules | "
                  f"{tree(ars, 'm-tok-var')} rules |")
    old = {}
    with gzip.open(HERE.parent / "speed" / "requests.jsonl.gz", "rt") as f:
        for line in f:
            e = json.loads(line)
            if e["name"].startswith("zebra.") and e["name"].endswith(".chosen-first"):
                old[e["name"].split(".")[1]] = e["request"]["grammar"]["text"]
    new = {
        "whole": render(a.einaf, "whole"),
        "step1": render(a.einaf, "1", "--text", f"{CONTRACT}/text.txt"),
        "step2": render(a.einaf, "2"),
        "step2b": render(a.einaf, "2b", "--program", f"{CONTRACT}/step2.ein"),
        "step3": render(a.einaf, "3", "--program", f"{CONTRACT}/step2.ein"),
    }
    print("\n**The Zebra puzzle's grammars as a request carries them**, in the fenced frame (S2.2.3's speed requests "
          "before; rendered as its `plan.py` renders them after):\n")
    print("| grammar | before: bytes, rules | after: bytes, rules |")
    print("|---|---|---|")
    for k in ["whole", "step1", "step2", "step2b", "step3"]:
        print(f"| {k} | {len(old[k].encode()):,}, {len(rules(old[k]))} | {len(new[k].encode()):,}, "
              f"{len(rules(new[k]))} |")


if __name__ == "__main__":
    main()
