#!/usr/bin/env python3
"""llama_table.py — test 3 on the parser's banked table of refused shapes:
each row's program, as `ein-ir/tests/golden/grammar_table.txt` prints it,
read by llama.cpp's grammar sampler with the model's vocabulary and by
EinAf's acceptor, from the whole grammar's root `program`
(`einaf grammar llama`); a JSON line a row into OUT.

    python3 llama_table.py MODEL OUT [--einaf einaf/target/release/einaf]
"""

from __future__ import annotations

import argparse
import json
import subprocess
import tempfile
from pathlib import Path

HERE = Path(__file__).resolve().parent
ROOT = HERE.parents[3]
TABLE = ROOT / "ein.rs/crates/ein-ir/tests/golden/grammar_table.txt"


def rows() -> list[tuple[str, str]]:
    out, head, prog, inside = [], None, [], False
    for line in TABLE.read_text().split("\n"):
        if line.startswith("## "):
            head, prog, inside = line[3:], [], True
        elif inside:
            if line.startswith("    "):
                prog.append(line[4:])
            else:
                inside = False
                out.append((head, "\n".join(prog) + "\n"))
    return out


def main() -> None:
    ap = argparse.ArgumentParser(description=__doc__.split("\n\n")[0])
    ap.add_argument("model")
    ap.add_argument("out")
    ap.add_argument("--einaf", default=str(ROOT / "einaf/target/release/einaf"))
    a = ap.parse_args()
    rs = rows()
    with tempfile.TemporaryDirectory() as tmp:
        names = []
        for i, (_, prog) in enumerate(rs):
            p = Path(tmp) / f"row-{i:03}.ein"
            p.write_text(prog)
            names.append(str(p))
        lst = Path(tmp) / "list"
        lst.write_text("\n".join(names) + "\n")
        r = subprocess.run([a.einaf, "grammar", "llama", "--model", a.model, "--list", str(lst)],
                           capture_output=True, text=True)
    lines = []
    for (head, _), l in zip(rs, r.stdout.splitlines()):
        row = json.loads(l)
        row["file"] = head
        lines.append(json.dumps(row) + "\n")
    Path(a.out).write_text("".join(lines))
    print(r.stderr.strip().splitlines()[-1])


if __name__ == "__main__":
    main()
