#!/usr/bin/env python3
"""Top-level forms of an Ein file, as text, each with the comment above it.

A reader of the six Zebra encodings for this folder's probes: it splits a file into
its top-level forms (strings and `;` comments respected), classifies each by what a
formalization's step writes, and writes a file back with the same forms in another
order. Nothing here parses Ein beyond the form's head and its keywords.

    python3 forms.py FILE                 # the forms, one line each: kind, head, first words
    python3 forms.py FILE --order synthesis|conventional > OUT.ein
"""
from __future__ import annotations

import argparse
import re
import sys
from dataclasses import dataclass, field

MEMBERSHIP = {"instance", "type", "is-a"}
DIRECTIVES = {"query", "solve", "config", "trace"}


@dataclass
class Form:
    text: str                      # the form itself
    comment: list[str] = field(default_factory=list)  # comment lines just above it
    head: str = ""
    kind: str = ""


def split(src: str) -> tuple[list[str], list[Form]]:
    """The file's header comment and its top-level forms."""
    forms: list[Form] = []
    pending: list[str] = []
    header: list[str] = []
    i, n = 0, len(src)
    seen_form = False
    while i < n:
        c = src[i]
        if c == ";":
            j = src.find("\n", i)
            j = n if j < 0 else j
            line = src[i:j]
            if not seen_form and (line.startswith(";;;") or not forms and not pending and line.startswith(";;;")):
                header.append(line)
            else:
                pending.append(line)
            i = j + 1
            continue
        if c == "\n":
            if pending and src[i - 1:i] == "\n" and not seen_form:
                pass
            i += 1
            continue
        if c.isspace():
            i += 1
            continue
        if c != "(":
            raise SystemExit(f"unexpected {c!r} at {i}")
        depth, j, in_str = 0, i, False
        while j < n:
            ch = src[j]
            if in_str:
                if ch == "\\":
                    j += 2
                    continue
                if ch == '"':
                    in_str = False
            elif ch == '"':
                in_str = True
            elif ch == ";":
                k = src.find("\n", j)
                j = n if k < 0 else k
                continue
            elif ch == "(":
                depth += 1
            elif ch == ")":
                depth -= 1
                if depth == 0:
                    break
            j += 1
        text = src[i:j + 1]
        f = Form(text=text, comment=[p for p in pending if "────" not in p])
        pending = []
        seen_form = True
        f.head = re.match(r"\(\s*([^\s()]+)", text).group(1)
        forms.append(f)
        i = j + 1
    return header, forms


def classify(forms: list[Form]) -> None:
    declared = set()
    for f in forms:
        if f.head == "relation":
            declared.add(re.match(r"\(\s*relation\s+([^\s()]+)", f.text).group(1))
    for f in forms:
        h = f.head
        if h == "relation":
            f.kind = "declaration"
        elif h == "rule":
            f.kind = "rule"
        elif h == "macro":
            f.kind = "macro"
        elif h == "import":
            f.kind = "import"
        elif h in DIRECTIVES:
            f.kind = "query" if h == "query" else "directive"
        elif ":source" in f.text:
            f.kind = "statement"
        elif h in MEMBERSHIP:
            f.kind = "membership"
        elif h in declared:
            f.kind = "structure"
        else:
            f.kind = "selector"


# What a formalization writes in each step, in that order (the user's steps), and the
# order the corpus files were written in before (theory first, as `zebra.ein` is).
SYNTHESIS = ["statement", "query", "declaration", "membership", "structure",
             "directive", "import", "selector", "macro", "rule"]
CONVENTIONAL = ["directive", "import", "macro", "rule", "declaration", "selector",
                "membership", "structure", "statement", "query"]


def write(header: list[str], forms: list[Form], order: list[str]) -> str:
    out = list(header) + [""]
    for kind in order:
        group = [f for f in forms if f.kind == kind]
        if not group:
            continue
        out.append(f";; ──── {kind} ────")
        for f in group:
            out.extend(f.comment)
            out.append(f.text)
        out.append("")
    return "\n".join(out) + "\n"


def main() -> None:
    ap = argparse.ArgumentParser()
    ap.add_argument("file")
    ap.add_argument("--order", choices=["synthesis", "conventional"])
    a = ap.parse_args()
    header, forms = split(open(a.file).read())
    classify(forms)
    if a.order:
        order = SYNTHESIS if a.order == "synthesis" else CONVENTIONAL
        sys.stdout.write(write(header, forms, order))
        return
    for f in forms:
        print(f"{f.kind:12} {f.head:32} {' '.join(f.text.split())[:90]}")


if __name__ == "__main__":
    main()
