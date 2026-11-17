#!/usr/bin/env python3
"""S1f.22.5, task 1: the proposed saved file, assembled from real bytes.

    python3 s1f.22.5_propose.py PROGRAM [STATE_FILE] > saved.ein
    python3 s1f.22.5_propose.py --modules PROGRAM          # the modules it carries, and their sizes

Run from the repository's root. The output is the format the page proposes
(Q-M1f.400 to .402 at their defaults), **not a file today's `ein` reads**:

    ;;; <one fixed line: what the file is>
    (program
    <the program file's bytes, as read>
    )
    (module NAME
    <that module's bytes, as read>
    )                                   ; one per module the load reads, sorted by name
    (commit F …)                        ; the choices, absent where none was made

The modules are the ones the program's imports reach, transitively, each once:
`std.x.y` read from `stdlib/x/y.ein` (the checkout, which is what the gate
build reads here), any other name from the program file's directory, a dot a
directory, for a module's own imports too (AGENTS.md). The choices are the
commitment the state file's header names (`from:` or `commitment:`), in the
order it lists them; none for a root's file or with no STATE_FILE.
"""
import os
import re
import sys

HEADER = (
    ";;; A saved state of ein (ein-saved/1): the program as written, every module it\n"
    ";;; imports as written, and the choices made. It reads no other file.\n"
)


def code_only(text):
    """The text with comments and strings blanked, so a form can be found."""
    out = []
    i, n = 0, len(text)
    while i < n:
        c = text[i]
        if c == ";":
            while i < n and text[i] != "\n":
                out.append(" ")
                i += 1
        elif text.startswith("#|", i):
            j = text.find("|#", i + 2)
            j = n if j < 0 else j + 2
            out.append(re.sub(r"[^\n]", " ", text[i:j]))
            i = j
        elif c == '"':
            j = i + 1
            while j < n and text[j] != '"':
                j += 2 if text[j] == "\\" else 1
            out.append(" " * (j + 1 - i))
            i = j + 1
        else:
            out.append(c)
            i += 1
    return "".join(out)


def imports(text):
    return re.findall(r"\(\s*import\s+([^\s()]+)", code_only(text))


def module_path(name, base):
    parts = name.split(".")
    if parts[0] == "std":
        return os.path.join("stdlib", *parts[1:]) + ".ein"
    return os.path.join(base, *parts) + ".ein"


def modules_of(program):
    base = os.path.dirname(program)
    seen = {}
    todo = imports(open(program, encoding="utf-8").read())
    while todo:
        m = todo.pop(0)
        if m in seen:
            continue
        p = module_path(m, base)
        seen[m] = p
        todo += imports(open(p, encoding="utf-8").read())
    return dict(sorted(seen.items()))


def commitment(state_file):
    if state_file is None:
        return []
    for line in open(state_file, encoding="utf-8"):
        m = re.match(r";;; (from|commitment):\s+(.*)$", line)
        if m:
            text = m.group(2).strip()
            if text == "∅":
                return []
            body, out, i = text[1:-1], [], 0
            while i < len(body):
                if body[i] == "(":
                    depth, j = 0, i
                    while True:
                        depth += {"(": 1, ")": -1}.get(body[j], 0)
                        j += 1
                        if depth == 0:
                            break
                    out.append(body[i:j])
                    i = j
                else:
                    i += 1
            return out
    return []


def saved(program, state_file=None):
    raw = open(program, "rb").read().decode("utf-8")
    parts = [HEADER, "(program\n", raw, "\n)\n"]
    for name, path in modules_of(program).items():
        parts += [f"(module {name}\n", open(path, "rb").read().decode("utf-8"), "\n)\n"]
    c = commitment(state_file)
    if c:
        parts.append("(commit\n" + "".join(f"  {f}\n" for f in c[:-1]) + f"  {c[-1]})\n")
    return "".join(parts)


def main():
    a = sys.argv[1:]
    if a and a[0] == "--modules":
        total = 0
        for name, path in modules_of(a[1]).items():
            n = os.path.getsize(path)
            total += n
            print(f"{n:>8}  {name}  {path}")
        print(f"{total:>8}  (modules)  {os.path.getsize(a[1]):>8}  (program)")
        return
    sys.stdout.write(saved(a[0], a[1] if len(a) > 1 else None))


if __name__ == "__main__":
    main()
