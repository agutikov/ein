#!/usr/bin/env python3
"""empty.py — the corpus with its comments emptied, as `einaf/tests/grammar.rs` empties them: each line comment its
`;` alone, each block comment a space, nothing else of a file changed.

    python3 empty.py LIST OUTDIR > OUT.list

Reads the paths of LIST (from the repository's root, `../llama/corpus.list`), writes each file emptied under OUTDIR at
the same path, and prints the paths written. Standard library only.
"""

import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[5]


def emptied(t: str) -> str:
    out, i, frm, n = [], 0, 0, len(t)
    while i < n:
        c = t[i]
        if c == ";":
            out.append(t[frm:i + 1])
            while i < n and t[i] != "\n":
                i += 1
            frm = i
        elif t.startswith("#|", i):
            out.append(t[frm:i])
            out.append(" ")
            i += 2
            while i + 1 < n and not (t[i] == "|" and t[i + 1] == "#"):
                i += 1
            i = min(i + 2, n)
            frm = i
        elif c == '"':
            i += 1
            while i < n and t[i] != '"':
                i += 2 if t[i] == "\\" else 1
            i += 1
        else:
            i += 1
    out.append(t[min(frm, n):])
    return "".join(out)


def main() -> None:
    out = Path(sys.argv[2])
    for rel in Path(sys.argv[1]).read_text().split():
        dst = out / rel
        dst.parent.mkdir(parents=True, exist_ok=True)
        dst.write_text(emptied((ROOT / rel).read_text()))
        print(dst)


if __name__ == "__main__":
    main()
