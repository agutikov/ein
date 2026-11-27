#!/usr/bin/env python3
"""The six texts with steps by hand, split into their sentences: `python3 texts.py [--check] [--json]`.

A text is `einaf/contract/<name>/text.txt`; the Zebra puzzle's is a sentence a line, the others' a paragraph split
after `.` or `?`. `--check` holds each split to the hand's step 1: every `:source` of `step1.ein` must be one of
the text's sentences. Standard library only, and runs under every Python the check's environments have (3.8 on).
"""

import json
import os
import re
import sys

ROOT = os.path.abspath(os.path.join(os.path.dirname(__file__), *[".."] * 4))
CONTRACT = os.path.join(ROOT, "einaf", "contract")
NAMES = ["apples", "more", "seating", "together", "youngest", "zebra"]


def text_of(name):
    with open(os.path.join(CONTRACT, name, "text.txt"), encoding="utf-8") as f:
        return f.read().strip()


def sentences(name):
    text = text_of(name)
    if "\n" in text:
        return [line.strip() for line in text.splitlines() if line.strip()]
    return [s.strip() for s in re.split(r"(?<=[.?!])\s+", text) if s.strip()]


def all_texts():
    return [{"name": n, "text": text_of(n), "sentences": sentences(n)} for n in NAMES]


def check():
    bad = 0
    for n in NAMES:
        with open(os.path.join(CONTRACT, n, "step1.ein"), encoding="utf-8") as f:
            sources = set(re.findall(r':source\s+"([^"]*)"', f.read()))
        sents = set(sentences(n))
        for s in sorted(sources - sents):
            print(f"{n}: :source not a sentence of the split: {s!r}")
            bad += 1
        print(f"{n}: {len(sentences(n))} sentences, {len(sources)} distinct :source, all among them: {not (sources - sents)}")
    return bad


if __name__ == "__main__":
    if "--check" in sys.argv:
        sys.exit(1 if check() else 0)
    if "--json" in sys.argv:
        json.dump(all_texts(), sys.stdout, indent=1)
        print()
    else:
        for t in all_texts():
            for i, s in enumerate(t["sentences"], 1):
                print(f"{t['name']}\t{i}\t{s}")
