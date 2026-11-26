#!/usr/bin/env python3
r"""Make the formulas of a Markdown note render: `\\[ … \\]` becomes a `$$` block and
`\\( … \\)` becomes `$…$`.

A note exported with its formulas escaped for Markdown writes `\\[ a+b=42 \\]` and
`\\(\Phi\\)`, and inside a formula `\\{`, `\\;` and `\_` for TeX's `\{`, `\;` and `_`.
Markdown reads those as text, so no formula is drawn. This script writes each display
formula between two `$$` lines, each inline one between `$`, and inside a formula undoes
Markdown's escapes (a backslash before ASCII punctuation is dropped, as CommonMark reads
it), so that `\\{x\\}` becomes TeX's `\{x\}`. Outside a formula, and inside a fenced or
inline code span, nothing changes. Running it twice changes nothing the second time.

    python3 utils/md_math.py NOTE.md …            # rewrite in place, print what changed
    python3 utils/md_math.py --check NOTE.md …    # change nothing; exit 1 if a file would change
"""

import argparse
import re
import sys

# A Markdown escape: a backslash before an ASCII punctuation character.
ESCAPE = re.compile(r"\\([!-/:-@\[-`{-~])")
# The escaped delimiters as they are written in the source: two backslashes, then the bracket.
DISPLAY = re.compile(r"\\\\\[(.*?)\\\\\]", re.S)
INLINE = re.compile(r"\\\\\((.*?)\\\\\)", re.S)
# Fenced code blocks and inline code spans, which are left as they are.
CODE = re.compile(r"(^```.*?^```[^\n]*$|`[^`\n]*`)", re.S | re.M)


def tex(body: str) -> str:
    """A formula's body with Markdown's escapes undone and its ends trimmed."""
    return ESCAPE.sub(r"\1", body).strip()


def display(m: re.Match, text: str) -> str:
    body = tex(m.group(1))
    line_start = m.start() == 0 or text[m.start() - 1] == "\n"
    line_end = m.end() == len(text) or text[m.end()] == "\n"
    if line_start and line_end:
        return f"$$\n{body}\n$$"
    return f"$${body}$$"


def convert_prose(text: str) -> str:
    text = DISPLAY.sub(lambda m: display(m, text), text)
    return INLINE.sub(lambda m: f"${tex(m.group(1))}$", text)


def convert(text: str) -> str:
    out, last = [], 0
    for m in CODE.finditer(text):
        out.append(convert_prose(text[last:m.start()]))
        out.append(m.group(0))
        last = m.end()
    out.append(convert_prose(text[last:]))
    return "".join(out)


def main() -> int:
    ap = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    ap.add_argument("--check", action="store_true", help="change nothing; exit 1 if a file would change")
    ap.add_argument("files", nargs="+")
    args = ap.parse_args()
    changed = 0
    for path in args.files:
        with open(path, encoding="utf-8") as f:
            text = f.read()
        new = convert(text)
        if new == text:
            continue
        changed += 1
        n = len(DISPLAY.findall(text)) + len(INLINE.findall(text))
        if args.check:
            print(f"{path}: {n} formulas to convert")
        else:
            with open(path, "w", encoding="utf-8") as f:
                f.write(new)
            print(f"{path}: {n} formulas converted")
    return 1 if args.check and changed else 0


if __name__ == "__main__":
    sys.exit(main())
