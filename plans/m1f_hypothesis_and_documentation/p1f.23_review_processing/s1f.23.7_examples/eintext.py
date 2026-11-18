"""Readers shared by `census.py` and `slice.py` (S1f.23.7, task 1).

Three things are read here, each from the binary's own output or from a
program as written:

- a `.ein` text, as nested lists (`parse`), with `canon` writing a fact as
  its relation and its arguments, every `:keyword value` pair left out, so a
  fact written with a `:source` and the same fact under an arrow are one text;
- `ein saturate --dump FILE`'s KB section (`read_dump`): every fact root holds,
  by origin, the declared relations and the relation table;
- the DOT the KB and constraint views write (`read_dot`): every octagon as the
  fact it draws, rebuilt from its numbered arrows.

Nothing here is the engine: it reads what the engine printed.
"""
from __future__ import annotations

import re
from dataclasses import dataclass, field

# ── a .ein text ──────────────────────────────────────────────────────────────

_TOKEN = re.compile(
    r'\s+|;[^\n]*|#\|.*?\|#|\(|\)|"(?:[^"\\]|\\.)*"|[^\s()";]+', re.S
)


class Str(str):
    """A string literal: natural-language text, never an object."""


def parse(text: str) -> list:
    """Every top-level form of `text`, a form a list, an atom a `str`."""
    stack: list[list] = [[]]
    for m in _TOKEN.finditer(text):
        tok = m.group(0)
        if tok[0].isspace() or tok[0] == ";" or tok.startswith("#|"):
            continue
        if tok == "(":
            stack.append([])
        elif tok == ")":
            done = stack.pop()
            stack[-1].append(done)
        elif tok[0] == '"':
            stack[-1].append(Str(tok[1:-1]))
        else:
            stack[-1].append(tok)
    while len(stack) > 1:  # an unclosed form: keep what was read
        done = stack.pop()
        stack[-1].append(done)
    return stack[0]


def is_kw(x) -> bool:
    return isinstance(x, str) and not isinstance(x, Str) and x.startswith(":")


def keywords(form: list) -> set[str]:
    return {x for x in form if is_kw(x)}


def positional(form: list) -> list:
    """A form's head and arguments, every `:keyword value` pair left out."""
    out, skip = [], False
    for x in form:
        if skip:
            skip = False
            continue
        if is_kw(x):
            skip = True
            continue
        out.append(x)
    return out


def canon(x) -> str:
    """A fact's text as an object: `(rel a (g b))`, clauses left out."""
    if isinstance(x, list):
        return "(" + " ".join(canon(e) for e in positional(x)) + ")"
    return str(x)


def fact_nodes(fact: list):
    """The fact and every fact under its arrows, at any depth."""
    yield fact
    for a in positional(fact)[1:]:
        if isinstance(a, list):
            yield from fact_nodes(a)


def direct_args(fact: list) -> list[str]:
    """The atoms under the fact's arrows 2, 3, …"""
    return [a for a in positional(fact)[1:] if not isinstance(a, list)]


def deep_args(fact: list) -> list[str]:
    """The atoms under an argument arrow of the fact or of a fact under its
    arrows, at any depth; a nested fact's own relation is not among them."""
    out = []
    for a in positional(fact)[1:]:
        if isinstance(a, list):
            out.extend(deep_args(a))
        else:
            out.append(a)
    return out


# ── `ein saturate --dump` ────────────────────────────────────────────────────


@dataclass
class Dump:
    facts: dict[str, tuple[str, list]] = field(default_factory=dict)  # text -> (origin, form)
    declared: list[str] = field(default_factory=list)
    table: list[str] = field(default_factory=list)  # declared + auto-vivified
    complete: bool = False


_SECTION = re.compile(r"^;; ── (BACKGROUND|GIVEN|DERIVED) \(\d+ facts\) ──")


def read_dump(out: str) -> Dump:
    d = Dump()
    at = out.find("=  SATURATED KB DUMP")
    if at < 0:
        return d
    d.complete = True
    mode = None
    for line in out[at:].splitlines():
        if line.startswith(";; Relations — declared"):
            mode = "declared"
            continue
        if line.startswith(";; Relations — auto-vivified"):
            mode = "auto"
            continue
        m = _SECTION.match(line)
        if m:
            mode = m.group(1)
            continue
        if line.startswith(";; Query") or line.startswith(";; Rules") or line.startswith("====="):
            mode = None
            continue
        if not line.strip():
            if mode in ("declared", "auto"):
                mode = None
            continue
        if mode == "declared" and line.startswith("(relation "):
            name = parse(line)[0][1]
            d.declared.append(name)
            d.table.append(name)
        elif mode == "auto" and line.startswith(";;   "):
            d.table.append(line[5:].strip())
        elif mode in ("BACKGROUND", "GIVEN", "DERIVED") and line.startswith("  ("):
            form = parse(line)[0]
            d.facts.setdefault(canon(form), (mode, form))
    return d


# ── the DOT of `ein render kb` and `ein render constraints` ─────────────────

_NODE = re.compile(r"^\s*(f_[0-9a-f]+) \[shape=octagon")
_EDGE = re.compile(r'^\s*(f_[0-9a-f]+) -> ("(?:[^"\\]|\\.)*"|f_[0-9a-f]+) \[xlabel="(\d+)"')


@dataclass
class Dot:
    lines: list[str]
    arrows: dict[str, list[tuple[int, str]]]  # fact node -> [(number, target)]
    text: dict[str, str]  # fact node -> canonical text

    def pointed_at(self) -> set[str]:
        return {t for arr in self.arrows.values() for _, t in arr if t.startswith("f_")}


def read_dot(dot: str) -> Dot:
    lines = dot.splitlines()
    arrows: dict[str, list[tuple[int, str]]] = {}
    for line in lines:
        m = _NODE.match(line)
        if m:
            arrows.setdefault(m.group(1), [])
            continue
        m = _EDGE.match(line)
        if m:
            arrows.setdefault(m.group(1), []).append((int(m.group(3)), m.group(2)))
    text: dict[str, str] = {}

    def txt(node: str) -> str:
        if node in text:
            return text[node]
        parts = []
        for _, t in sorted(arrows.get(node, [])):
            parts.append(txt(t) if t.startswith("f_") else t[1:-1])
        text[node] = "(" + " ".join(parts) + ")"
        return text[node]

    for node in arrows:
        txt(node)
    return Dot(lines, arrows, text)
