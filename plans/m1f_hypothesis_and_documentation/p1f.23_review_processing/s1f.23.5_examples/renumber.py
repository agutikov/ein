#!/usr/bin/env python3
"""One-off: every relation declaration's `:why` renumbered, `{?k}` -> `{?k+2}`.

A `:why` reads the form it is written on: `{?1}` is the atom `relation`, `{?2}`
the relation's name, `{?k+2}` its k-th label. A declaration
`(relation NAME LABEL... :why "...")` is found wherever it is written: a `.ein`
file, a Rust string (plain, `\\"…\\"`, or raw), a Python string, a page. Only
the `:why` of a declaration moves; a rule's, an hrule's and a macro's do not.

    renumber.py [--write] [--leftovers] PATH...
"""
import re, sys, collections
from pathlib import Path

# whitespace inside a program, as a .ein, a Rust string (`\n`, `\t`, a line
# continuation `\` + newline) or a Python string writes it
WS = r'(?:\s|\\n|\\t|\\\n)+'
TOKEN = r'[^\s()"\\]+'
DECL = re.compile(
    r'\(relation' + WS + r'(?P<name>(?![?:])' + TOKEN + r')'
    r'(?P<labels>(?:' + WS + r'(?!:)' + TOKEN + r')*)'
    + WS + r':why' + WS +
    r'(?P<q>\\"|")')
POS = re.compile(r'(\{\{?\?)([0-9]+)(\}\}?)')

def body_end(text, start, q):
    """Index where the `:why` string that starts at `start` ends (its closing quote)."""
    j = start
    n = len(text)
    if q == '"':
        while j < n and text[j] != '"':
            j += 2 if text[j] == '\\' else 1
        return j
    # a `\"`-quoted string inside a host string: up to the next `\"` that is not `\\\"`
    while j < n:
        if text.startswith('\\\\', j):
            j += 2
            continue
        if text.startswith('\\"', j):
            return j
        if text[j] == '"':
            return j  # the host string ended
        j += 1
    return j

def renumber(text):
    out, last, count, refs = [], 0, 0, 0
    spans = []
    for m in DECL.finditer(text):
        s = m.end()
        e = body_end(text, s, m.group('q'))
        if s < last:
            continue
        body = text[s:e]
        new, k = POS.subn(lambda p: f"{p.group(1)}{int(p.group(2)) + 2}{p.group(3)}", body)
        count += 1
        refs += k
        spans.append((s, e))
        out.append(text[last:s])
        out.append(new)
        last = e
    out.append(text[last:])
    return ''.join(out), count, refs, spans

def leftovers(text, spans):
    """`{?digit}` outside every declaration's :why: for a reader to check."""
    res = []
    for m in POS.finditer(text):
        if not any(s <= m.start() < e for s, e in spans):
            line = text.count('\n', 0, m.start()) + 1
            res.append(line)
    return res

def main():
    args = sys.argv[1:]
    write = '--write' in args
    show = '--leftovers' in args
    paths = [Path(a) for a in args if not a.startswith('--')]
    tot = collections.Counter()
    for p in paths:
        try:
            text = p.read_text()
        except (UnicodeDecodeError, IsADirectoryError, FileNotFoundError):
            continue
        new, count, refs, spans = renumber(text)
        if count:
            tot['files'] += 1
            tot['decls'] += count
            tot['refs'] += refs
            if refs:
                tot['files changed'] += 1
                tot['decls with refs'] += sum(1 for s, e in spans if POS.search(text[s:e]))
            print(f"{p}\t{count}\t{refs}")
        if show:
            for line in leftovers(text, spans):
                print(f"LEFTOVER {p}:{line}")
        if write and new != text:
            p.write_text(new)
    print(f"TOTAL {dict(tot)}", file=sys.stderr)

main()
