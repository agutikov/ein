#!/usr/bin/env python3
"""graph2ein: a throwaway prototype, a DOT drawing read as an Ein knowledge base.

Reading 1, any labelled digraph: an arrow a -[r]-> b is the fact (r a b), a node's label an atom's
name (its id where it has no label), an arrow's label the relation; a node no arrow touches is a
fact of its atom alone, (Dog); an undirected arc a -- b is the two facts (r a b) and (r b a).
Labels are required: an unlabelled arrow is a failure, unless `--unlabelled single` gives every
unlabelled arrow one default label (`edge`) or `--unlabelled distinct` gives each its own
(`e1`, `e2`, ...). `--repair` applies one plausible repair to every other failure and says which.

Reading 2, `ein render kb`'s own drawing as it is drawn today (the folded form): a box or an oval is
an atom, an octagon a fact whose relation is written in it, `(likes)`, and whose arrows `#1`, `#2`,
... point at its arguments; any other labelled arrow is a fact of two arguments, its label's first
word the relation; an octagon in a frame named `... ctx` is pointed at and not held.

Graphviz does the parsing (`dot -Tjson0`); `gvpr` says which node labels are HTML, which the JSON
does not. The Ein binary loads each program (`ein solve FILE --no-search`) and draws it back
(`ein render kb FILE`). Nothing from a downloaded file is ever run: a downloaded file is read as
DOT and nothing else.

    graph2ein.py read1 FILE.gv OUT.ein [--repair] [--unlabelled single|distinct] [--source-text]
    graph2ein.py read2 FILE.dot OUT.ein [--loop unary|binary]
    graph2ein.py survey [--only NAME ...]      # every example, results.json, the table
"""
from __future__ import annotations

import argparse
import html
import json
import os
import re
import subprocess
import sys
import tempfile
import unicodedata
from collections import Counter, defaultdict
from dataclasses import dataclass, field
from pathlib import Path

HERE = Path(__file__).resolve().parent
REPO = HERE.parents[3]
EXT = REPO / "build" / "p1f30-external"
EIN = os.environ.get("EIN_BIN", str(REPO / "build" / "review-bin" / "ein-head-b5fb8d70"))

# --------------------------------------------------------------------------------------------
# Ein's names, as the lexer (ein.l) and the loader take them
# --------------------------------------------------------------------------------------------

NAME_CHARS = set("ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789_.-$%^*+=<>!&@~/'`[]{}")
NAME_RE = re.compile(r"[A-Za-z0-9_.\-$%^*+=<>!&@~/'`\[\]{}]+\Z")
WRITEBACK = {"<forced-positive>", "<monotonic-unconditional>", "<lookahead-dies-immediately>"}

# What Ein does with a kernel word in each of the three places reading 1 writes a word, as the
# binary answered one-line probes `(w a b)`, `(r w b)` and `(w)` (the survey re-takes them).
NEVER_A_NAME = {"not", "and", "or", "neq", "rule", "hrule", "query", "solve", "config", "trace",
                "macro", "import", "open", "_"}
KERNEL_AS_RELATION = NEVER_A_NAME | {"relation", "eq", "absent", "false", "region", "merged",
                                     "owing", "program", "module", "commit"}
KERNEL_AS_ARGUMENT = set(NEVER_A_NAME)
KERNEL_AS_LONE = (NEVER_A_NAME - {"solve", "config", "trace"}) | {
    "relation", "eq", "absent", "region", "merged", "owing", "module", "commit"}
LONE_VANISHES = {"solve", "config", "trace", "program"}   # loads as a form, and no fact is written
LONE_KILLS = {"false"}                                     # (false): root is dead
KERNEL_MEANING = {
    "relation": "a declaration, which wants a :why",
    "eq": "a check, not stored: (eq a b) of two names makes root dead",
    "false": "(false a b) is refused; (false) makes root dead",
    "program": "the head of a saved file", "module": "the head of a saved file",
    "commit": "the head of a saved file",
}
KERNEL_MARKERS = {"__closed__", "__symmetric__"}           # load as facts; read where a KB grows


def stdlib_modules() -> set[str]:
    root = REPO / "stdlib"
    return {"std." + ".".join(p.relative_to(root).with_suffix("").parts) for p in root.rglob("*.ein")}


STDLIB = stdlib_modules()


def module_prefix(name: str, beside: Path | None) -> str | None:
    """A dotted name is the full name of a module's name when a prefix of it, up to a dot, is a
    module the program could import: a standard-library module, or a file beside the program."""
    parts = name.split(".")
    for i in range(len(parts) - 1, 0, -1):
        prefix = ".".join(parts[:i])
        if not prefix:
            continue
        if prefix in STDLIB:
            return prefix
        if beside is not None and (beside / (prefix.replace(".", "/") + ".ein")).is_file():
            return prefix
    return None


def why_not_a_name(text: str, place: str, beside: Path | None = None) -> str | None:
    """None if `text` is a name Ein takes in `place` ('relation', 'argument', 'lone'), else why not."""
    if text == "":
        return "empty"
    if "\n" in text:
        return "several lines"
    if any(ord(c) > 127 for c in text):
        return "not ASCII"
    if any(c.isspace() for c in text):
        return "space"
    if text in WRITEBACK:
        return "an engine name"
    if not NAME_RE.match(text):
        return "punctuation"
    bad = {"relation": KERNEL_AS_RELATION, "argument": KERNEL_AS_ARGUMENT,
           "lone": KERNEL_AS_LONE | LONE_VANISHES | LONE_KILLS}[place]
    if text in bad:
        return "kernel word"
    if "." in text and module_prefix(text, beside):
        return "full name of a module"
    return None


def slug(text: str) -> str:
    """One plausible repair of a label that is no name: fold to ASCII, every run of other
    characters a hyphen. Lossy: two labels can come to one slug."""
    folded = unicodedata.normalize("NFKD", text)
    out = []
    for c in folded:
        if unicodedata.combining(c):
            continue
        out.append(c if c in NAME_CHARS else "-")
    s = re.sub(r"-{2,}", "-", "".join(out)).strip("-")
    return s


def escape(text: str) -> str:
    """The reversible alternative: percent-encode every byte that is not a name character,
    and `%` itself, since `%` is a name character."""
    out = []
    for c in text:
        if c in NAME_CHARS and c != "%":
            out.append(c)
        else:
            out.extend("%%%02X" % b for b in c.encode("utf-8"))
    return "".join(out)


def ein_text(s: str) -> str:
    """A TEXT literal: natural-language text, any Unicode, the five escapes."""
    s = s.replace("\\", "\\\\").replace('"', '\\"').replace("\n", "\\n").replace("\t", "\\t").replace("\r", "\\r")
    return '"' + "".join(c if (ord(c) >= 32 or c in "\\") else "\\u{%x}" % ord(c) for c in s) + '"'


# --------------------------------------------------------------------------------------------
# A drawing, as Graphviz parses it
# --------------------------------------------------------------------------------------------

@dataclass
class Node:
    gvid: int
    id: str
    attrs: dict
    html: bool = False

    @property
    def record(self) -> bool:
        return self.attrs.get("shape") in ("record", "Mrecord")


@dataclass
class Cluster:
    gvid: int
    id: str
    attrs: dict
    nodes: set
    subs: set


@dataclass
class Edge:
    idx: int
    tail: int
    head: int
    attrs: dict


@dataclass
class Graph:
    name: str
    directed: bool
    strict: bool
    nodes: dict
    clusters: dict
    edges: list
    nsub: int


def run(cmd, timeout=600, cwd=None):
    return subprocess.run(cmd, capture_output=True, text=True, timeout=timeout, cwd=cwd)


IMAGES_STRIPPED: set = set()


def load_dot(path: Path) -> Graph:
    p = run(["dot", "-Tjson0", str(path)])
    if (p.returncode != 0 or not p.stdout.strip()) and "image" in p.stderr:
        # -Tjson0 lays the drawing out, and a layout opens the images an HTML label names; those
        # were not downloaded, so a copy without its <IMG> cells is laid out instead
        text = re.sub(r"<IMG[^>]*/>", "", path.read_text(encoding="utf-8", errors="replace"), flags=re.I)
        with tempfile.NamedTemporaryFile("w", suffix=".gv", delete=False, encoding="utf-8") as t:
            t.write(text)
        p = run(["dot", "-Tjson0", t.name])
        os.unlink(t.name)
        IMAGES_STRIPPED.add(path.name)
    if p.returncode != 0 or not p.stdout.strip():
        raise RuntimeError(f"dot -Tjson0 {path}: {p.stderr.strip()[:300]}")
    d = json.loads(p.stdout)
    nsub = d.get("_subgraph_cnt", 0)
    objs = d.get("objects", [])
    nodes, clusters = {}, {}
    for i, o in enumerate(objs):
        assert o.get("_gvid", i) == i
        if i < nsub:
            if o["name"].startswith("cluster"):
                clusters[i] = Cluster(i, o["name"], o, set(o.get("nodes", [])), set(o.get("subgraphs", [])))
        else:
            nodes[i] = Node(i, o["name"], o)
    g = run(["gvpr", 'N { if (ishtml($.label)) print($.name); }', str(path)])
    htmls = set(g.stdout.splitlines()) if g.returncode == 0 else set()
    for n in nodes.values():
        n.html = n.id in htmls
    edges = [Edge(k, e["tail"], e["head"], e) for k, e in enumerate(d.get("edges", []))]
    return Graph(d.get("name", ""), bool(d.get("directed", True)), bool(d.get("strict", False)),
                 nodes, clusters, edges, nsub)


def decode(raw: str | None, node_id: str = "", graph: str = "") -> str:
    """A DOT label as the text it draws: \\N the node's id, \\G the graph's, \\n \\l \\r a line."""
    if raw is None:
        raw = "\\N"
    out, i = [], 0
    while i < len(raw):
        c = raw[i]
        if c == "\\" and i + 1 < len(raw):
            n = raw[i + 1]
            rep = {"N": node_id, "G": graph, "n": "\n", "l": "\n", "r": "\n", "\\": "\\", '"': '"'}.get(n)
            if rep is not None:
                out.append(rep)
                i += 2
                continue
        out.append(c)
        i += 1
    s = "".join(out)
    return s[:-1] if s.endswith("\n") else s


def html_lines(raw: str) -> list[str]:
    s = re.sub(r"<\s*br\s*[^>]*>|</\s*tr\s*>|</\s*td\s*>", "\n", raw, flags=re.I)
    s = html.unescape(re.sub(r"<[^>]*>", "", s))
    return [ln.strip() for ln in s.split("\n") if ln.strip()]


def looks_html(raw: str) -> bool:
    return bool(re.search(r"</?\s*(b|i|u|br|table|tr|td|font|img|sub|sup)\b", raw or "", re.I))


COMPASS = {"n", "ne", "e", "se", "s", "sw", "w", "nw", "c", "_"}
SEMANTIC_EDGE_ATTRS = ["dir", "color", "style", "weight", "penwidth", "arrowhead", "arrowtail",
                       "headlabel", "taillabel", "xlabel", "len", "minlen", "constraint", "URL",
                       "href", "tooltip", "labeltooltip"]


# --------------------------------------------------------------------------------------------
# Reading 1
# --------------------------------------------------------------------------------------------

@dataclass
class Policy:
    repair: bool = False                 # the label repairs: slug, first line, id, @id, kernel word_
    unlabelled: str | None = None        # None: a failure; 'single'; 'distinct'
    clusters: bool = False               # a frame's members as facts of in-cluster
    ports: bool = False                  # a record field or an HTML cell as an atom node/port
    read_dir: bool = False               # dir=back reversed, dir=both and dir=none two facts
    rename_coincidence: bool = False     # a node whose label is an arrow's: label@id
    source_text: bool = False            # keep an edge label's own text as the fact's :source
    colour_as_label: bool = False        # an unlabelled arrow's colour as its relation
    number_parallel: bool = False        # parallel arrows of one label: a third argument, 1, 2, ...

    def tag(self) -> str:
        if not self.repair and self.unlabelled is None:
            return "strict"
        t = {None: "repaired", "single": "a", "distinct": "b"}[self.unlabelled]
        for flag, word in (("clusters", "clusters"), ("ports", "ports"), ("read_dir", "dir"),
                           ("rename_coincidence", "rename"), ("colour_as_label", "colour"),
                           ("source_text", "source"), ("number_parallel", "numbered")):
            if getattr(self, flag):
                t += "+" + word
        return t


@dataclass
class Result:
    facts: list = field(default_factory=list)       # (relation, args, source) in written order
    failures: list = field(default_factory=list)    # dicts
    notes: list = field(default_factory=list)
    refused: bool = False
    input_nodes: set = field(default_factory=set)   # the drawing's labels, as drawn
    input_edges: set = field(default_factory=set)   # (tail label, arrow label, head label)


def fail(res: Result, kind: str, sub: str, item: str, refusal: bool, **kw):
    res.failures.append(dict(kind=kind, sub=sub, item=item, refusal=refusal, **kw))
    if refusal:
        res.refused = True


def read1(g: Graph, pol: Policy, beside: Path | None) -> Result:
    res = Result()
    strict = not pol.repair
    touched = set()
    for e in g.edges:
        touched.add(e.tail)
        touched.add(e.head)
    lone = {gv for gv in g.nodes if gv not in touched}

    # --- the drawing's own text, for the round trip
    drawn = {}
    for gv, n in g.nodes.items():
        raw = n.attrs.get("label")
        drawn[gv] = ("\n".join(html_lines(raw)) if n.html else decode(raw, n.id, g.name))
    for gv, c in g.clusters.items():
        drawn[gv] = decode(c.attrs.get("label") or c.id, c.id, g.name)
    res.input_nodes = {drawn[gv] for gv in g.nodes}
    for e in g.edges:
        lab = e.attrs.get("label")
        res.input_edges.add((drawn.get(e.tail, "?"), decode(lab) if lab else "", drawn.get(e.head, "?")))

    # --- nodes: a name each
    name = {}
    for gv, n in g.nodes.items():
        raw = n.attrs.get("label")
        place = "lone" if gv in lone else "argument"
        if n.html:
            text = "\n".join(html_lines(raw))
            sub = "HTML label"
        elif n.record and raw not in (None, "\\N") and re.search(r"[|{}<>]", raw):
            text, sub = decode(raw, n.id, g.name), "record label"
        else:
            text = decode(raw, n.id, g.name)
            sub = None
        why = why_not_a_name(text, place, beside)
        if why:   # an HTML or a record label whose text is a name is read as that name
            kind = "kernel-word" if why == "kernel word" else "label-not-a-name"
            fail(res, kind, sub if (sub and why != "kernel word") else why, f"node {n.id}",
                 refusal=strict, text=text, slug=slug(text), escape=escape(text))
        if strict or not why:
            name[gv] = text
            continue
        # repair: HTML -> its first line of text; record -> the node's id; several lines -> the
        # first; then the slug; then the id's slug where that leaves nothing
        if n.html:
            lines = html_lines(raw)
            if n.record:   # an HTML label in a record shape: its fields are split at |, in braces
                lines = [x.strip(" {}") for ln in lines for x in ln.split("|") if x.strip(" {}")]
            cand = lines[0] if lines else n.id
        elif sub == "record label" or text == "":
            cand = n.id
        else:
            cand = text.split("\n")[0]
        if why_not_a_name(cand, place, beside) not in (None, "kernel word"):
            cand = slug(cand) or slug(n.id) or f"n{gv}"
        if why_not_a_name(cand, place, beside) == "full name of a module":
            cand = cand.replace(".", "-")
        if why_not_a_name(cand, place, beside) == "kernel word":
            cand = cand + "_"
        name[gv] = cand

    # --- two nodes of one label
    groups = defaultdict(list)
    for gv, nm in name.items():
        groups[nm].append(gv)
    for nm, gvs in groups.items():
        if len(gvs) > 1:
            ids = [g.nodes[x].id for x in gvs]
            first_text = drawn[gvs[0]]
            same_drawn = all(drawn[x] == first_text for x in gvs)
            fail(res, "duplicate-node-label", "one label" if same_drawn else "one name after the repair",
                 f"nodes {', '.join(ids)}", refusal=False, text=nm)
            if not strict:
                for x in gvs:
                    name[x] = f"{nm}@{slug(g.nodes[x].id) or x}"

    # --- clusters: a name each (used by the repair, and by an arrow to a cluster)
    cname = {}
    for gv, c in g.clusters.items():
        raw = c.attrs.get("label")
        text = (html_lines(raw)[0] if raw and looks_html(raw) and html_lines(raw) else decode(raw or c.id, c.id, g.name).split("\n")[0])
        cand = text if why_not_a_name(text, "argument", beside) is None else slug(text)
        if not cand or why_not_a_name(cand, "argument", beside):
            cand = slug(c.id)
        if cand in name.values():
            cand = f"{cand}@{slug(c.id)}"
        cname[gv] = cand

    # --- arrows
    raw_facts = []          # (rel, args, source, edge idx)
    nunl = 0
    taken_labels = {decode(e.attrs.get("label")) for e in g.edges if e.attrs.get("label")}
    default_single = "edge"
    while default_single in set(name.values()) | taken_labels:
        default_single = "_" + default_single
    distinct_prefix = "e"
    while any(re.fullmatch(re.escape(distinct_prefix) + r"\d+", x) for x in set(name.values()) | taken_labels):
        distinct_prefix = "arc"
    attr_counts = Counter()
    for e in g.edges:     # ports and arrows to a frame, counted over every arrow
        tl, hl = drawn.get(e.tail, "?"), drawn.get(e.head, "?")
        for gv, port_key in ((e.tail, "tailport"), (e.head, "headport")):
            if gv in g.clusters or gv not in g.nodes:
                fail(res, "edge-to-cluster", "an arrow to a frame", f"{tl} -> {hl}", refusal=strict)
            elif e.attrs.get(port_key):
                port = e.attrs[port_key]
                fail(res, "port", "compass point (layout)" if port.split(":")[0] in COMPASS
                     else "record field or HTML cell", f"{tl} -> {hl}", refusal=False, port=port)
    for e in g.edges:
        a = e.attrs
        for k in SEMANTIC_EDGE_ATTRS:
            if k in a and a[k] not in ("", None):
                if k == "dir" and a[k] == ("forward" if g.directed else "none"):
                    continue
                attr_counts[k if k != "dir" else f"dir={a[k]}"] += 1
        raw = a.get("label")
        text = decode(raw) if raw not in (None, "") else None
        if raw and looks_html(raw):
            text = " ".join(html_lines(raw))
        how = None
        if text is None and pol.repair and a.get("xlabel"):
            text, how = decode(a["xlabel"]), "xlabel"
        tl, hl = drawn.get(e.tail, "?"), drawn.get(e.head, "?")
        if text is None:
            if pol.colour_as_label and a.get("color"):
                rel = slug(a["color"]) or default_single
            elif pol.unlabelled == "single":
                rel = default_single
            elif pol.unlabelled == "distinct":
                nunl += 1
                rel = f"{distinct_prefix}{nunl}"
            else:
                fail(res, "unlabelled-edge", "no label" + (" (an xlabel)" if a.get("xlabel") else ""),
                     f"{tl} -> {hl}", refusal=True)
                continue
            if pol.unlabelled or pol.colour_as_label:
                fail(res, "unlabelled-edge", "given a default label", f"{tl} -> {hl}", refusal=False, label=rel)
        else:
            words = text.split()
            if len(words) >= 3 or "\n" in text or re.search(r"\[[^\]]+\]", text):
                fail(res, "sentence-label", "a sentence" if len(words) >= 3 else "several lines or a [tag]",
                     f"{tl} -> {hl}", refusal=False, text=text)
            if re.fullmatch(r"\s*-?\d+(\.\d+)?%?\s*", text):
                fail(res, "data-label", "a number", f"{tl} -> {hl}", refusal=False, text=text)
            why = why_not_a_name(text, "relation", beside)
            if why:
                kind = "kernel-word" if why == "kernel word" else "label-not-a-name"
                fail(res, kind, why, f"arrow {tl} -> {hl}", refusal=strict, text=text,
                     slug=slug(text), escape=escape(text))
            if strict:
                rel = text
            else:
                rel = text if not why else slug(text.replace("\n", " "))
                if not rel:   # nothing of it folds to ASCII: the reversible escape
                    rel = escape(text)
                if why_not_a_name(rel, "relation", beside) == "kernel word":
                    rel += "_"
                if why_not_a_name(rel, "relation", beside) == "full name of a module":
                    rel = rel.replace(".", "-")
        src = text if (pol.source_text and text and (len(text.split()) >= 2 or "\n" in text)) else None
        ends = []
        for gv, port_key in ((e.tail, "tailport"), (e.head, "headport")):
            if gv in g.clusters:
                ends.append(cname[gv])
                continue
            if gv not in name:
                ends.append(f"g{gv}")
                continue
            nm = name[gv]
            port = a.get(port_key)
            if port:
                pname = port.split(":")[0]
                if pname not in COMPASS:
                    if pol.ports:
                        pn = f"{nm}/{slug(pname) or pname}"
                        raw_facts.append(("port", (nm, pn), None, e.idx))
                        nm = pn
            ends.append(nm)
        t, h = ends
        pairs = [(t, h)]
        if not g.directed:
            pairs = [(t, h), (h, t)] if t != h else [(t, h)]
        elif pol.read_dir:
            d = a.get("dir")
            if d == "back":
                pairs = [(h, t)]
            elif d in ("both", "none"):
                pairs = [(t, h), (h, t)] if t != h else [(t, h)]
        for x, y in pairs:
            raw_facts.append((rel, (x, y), src, e.idx))
    for k, v in sorted(attr_counts.items()):
        fail(res, "edge-attributes", k, f"{v} arrows", refusal=False, count=v)
    if not g.directed:
        res.notes.append(f"undirected: {len(g.edges)} arcs, each two facts (the ruling)")

    # --- a node's label that is also an arrow's label
    rels = {f[0] for f in raw_facts}
    for gv in g.nodes:
        if name[gv] in rels and gv not in lone:
            fail(res, "node-label-is-arrow-label", "one atom", f"node {g.nodes[gv].id}", refusal=False,
                 text=name[gv])
            if pol.rename_coincidence:
                old, new = name[gv], f"{name[gv]}@{slug(g.nodes[gv].id) or gv}"
                raw_facts = [(r, tuple(new if (x == old) else x for x in args), s, i) for r, args, s, i in raw_facts]
                name[gv] = new

    # --- clusters
    if g.clusters:
        direct = {}
        for gv, c in g.clusters.items():
            inner = set()
            for s in c.subs:
                if s in g.clusters:
                    inner |= g.clusters[s].nodes
            direct[gv] = c.nodes - inner
        n_members = sum(len(v) for v in direct.values())
        fail(res, "cluster", "kept as facts of in-cluster" if pol.clusters else "lost",
             f"{len(g.clusters)} frames, {n_members} members", refusal=False)
        if pol.clusters:
            for gv, c in g.clusters.items():
                for m in sorted(direct[gv]):
                    if m in name:
                        raw_facts.append(("in-cluster", (name[m], cname[gv]), None, -1))
                for s in sorted(c.subs):
                    if s in g.clusters:
                        raw_facts.append(("in-cluster", (cname[s], cname[gv]), None, -1))

    # --- parallel arrows: one fact
    mult = Counter((r, args) for r, args, _, _ in raw_facts)
    par = {k: v for k, v in mult.items() if v > 1}
    for (r, args), v in par.items():
        fail(res, "parallel-arrows", "one fact", f"({r} {' '.join(args)})", refusal=False, count=v)
    seen, k_of = set(), Counter()
    for r, args, s, i in raw_facts:
        if pol.number_parallel and (r, args) in par:
            k_of[(r, args)] += 1
            res.facts.append((r, args + (str(k_of[(r, args)]),), s))
            continue
        if (r, args) in seen:
            continue
        seen.add((r, args))
        res.facts.append((r, args, s))

    # --- nodes no arrow touches
    for gv in sorted(lone):
        res.facts.append((name[gv], (), None))
    if lone:
        res.notes.append(f"{len(lone)} nodes no arrow touches, each a fact of its atom alone")
    return res


def write_program(path: Path, res: Result, title: str, src_name: str, pol: Policy):
    lines = [f";; {title}: reading 1 of {src_name}, {pol.tag()}.",
             ";; An arrow is a fact, a label a name; nothing here is declared, so no relation is guessed.", ""]
    for r, args, s in res.facts:
        body = " ".join((r,) + tuple(args))
        lines.append(f"({body}" + (f" :source {ein_text(s)})" if s else ")"))
    path.write_text("\n".join(lines) + "\n", encoding="utf-8")


# --------------------------------------------------------------------------------------------
# The binary: load, draw
# --------------------------------------------------------------------------------------------

def load_check(path: Path, extra=()) -> dict:
    p = run([EIN, "solve", "--no-search", *extra, path.name], cwd=path.parent)
    out = p.stdout + p.stderr
    m = re.search(r"^\s*∅\s+(.*)$", out, re.M)
    if p.returncode == 0 and m:
        state = "alive" if m.group(1).startswith("in R") else ("dead" if "dead" in m.group(1) else m.group(1))
        return dict(loads=True, root=state, first="")
    first = next((ln for ln in out.splitlines() if ln.strip()), "")
    return dict(loads=False, root=None, first=first[:300], exit=p.returncode)


def render_kb(path: Path, out: Path, extra=()) -> bool:
    p = run([EIN, "render", "kb", *extra, path.name], cwd=path.parent)
    if p.returncode != 0:
        return False
    out.write_text(p.stdout, encoding="utf-8")
    return True


def drawing_sets(g: Graph):
    """The labels a drawing shows: its nodes' and its arrows', an octagon by its label."""
    lab = {}
    for gv, n in g.nodes.items():
        lab[gv] = decode(n.attrs.get("label"), n.id, g.name)
    nodes = set(lab.values())
    edges = {(lab.get(e.tail, "?"), decode(e.attrs.get("label")) if e.attrs.get("label") else "",
              lab.get(e.head, "?")) for e in g.edges}
    octagons = {gv for gv, n in g.nodes.items() if n.attrs.get("shape") == "octagon"}
    return nodes, edges, octagons, lab


def round_trip(prog: Path, res: Result, kb_dot: Path, extra=()) -> dict:
    if not render_kb(prog, kb_dot, extra):
        return dict(drawn=False)
    g = load_dot(kb_dot)
    nodes_out, edges_out, octs, lab = drawing_sets(g)
    two = {(args[0], r, args[1]) for r, args, _ in res.facts if len(args) == 2}
    lone = {r for r, args, _ in res.facts if len(args) == 0}
    other = [(r, args) for r, args, _ in res.facts if len(args) not in (0, 2)]
    atoms = {a for r, args, _ in res.facts for a in args}
    # an arrow whose label carries a :source sentence after the relation reads back as no name
    two_back = {t for t in two if t in edges_out}
    lone_back_as_octagon = {r for r in lone if f"({r})" in nodes_out}
    back1 = read1(g, Policy(), None)
    facts_back = {(r, args) for r, args, _ in back1.facts}
    facts_in = {(r, args) for r, args, _ in res.facts}
    return dict(
        drawn=True,
        facts=len(res.facts), two=len(two), two_back=len(two_back),
        two_missing=sorted(map(list, two - two_back))[:6],
        lone=len(lone), lone_as_octagon=len(lone_back_as_octagon),
        other=len(other), atoms=len(atoms), atoms_back=len(atoms & nodes_out),
        drawing_nodes=len(g.nodes), drawing_edges=len(g.edges), octagons=len(octs),
        read1_back_equal=(facts_back == facts_in), read1_back_refusals=sum(f["refusal"] for f in back1.failures),
        read1_back_missing=len(facts_in - facts_back), read1_back_extra=len(facts_back - facts_in),
        input_nodes_verbatim=len(res.input_nodes & nodes_out), input_nodes=len(res.input_nodes),
        input_edges_verbatim=len(res.input_edges & edges_out), input_edges=len(res.input_edges),
    )


# --------------------------------------------------------------------------------------------
# Reading 2: `ein render kb`'s drawing as drawn today
# --------------------------------------------------------------------------------------------

def split_label(raw: str) -> tuple[str, str]:
    """An octagon's `(rel)\\nannotation`, an arrow's `rel annotation`: the word and the rest."""
    s = decode(raw or "")
    s = re.sub(r"\\n", "\n", s)
    if s.startswith("("):
        head, _, rest = s.partition("\n")
        return head.strip()[1:-1], rest.strip()
    word, _, rest = s.partition(" ")
    return word, rest.strip()


def term_text(t) -> str:
    return t if isinstance(t, str) else "(" + " ".join(term_text(x) for x in t) + ")"


def read2(g: Graph, loop: str = "unary") -> dict:
    framed = set()
    for c in g.clusters.values():
        if "ctx" in decode(c.attrs.get("label") or ""):
            framed |= c.nodes
    octs = {gv for gv, n in g.nodes.items() if n.attrs.get("shape") == "octagon"}
    args = defaultdict(dict)
    folded, notes, ambiguous = [], [], 0
    for e in g.edges:
        lab = decode(e.attrs.get("label") or "")
        m = re.fullmatch(r"#(\d+)", lab)
        if m and e.tail in octs:
            args[e.tail][int(m.group(1))] = e.head
        else:
            folded.append(e)
    memo = {}

    def term(gv):
        if gv not in octs:
            return g.nodes[gv].id
        if gv not in memo:
            rel, _ = split_label(g.nodes[gv].attrs.get("label"))
            memo[gv] = (rel,) + tuple(term(args[gv][k]) for k in sorted(args[gv]))
        return memo[gv]

    facts = []    # (fact tuple, provenance)
    for gv in sorted(octs):
        if gv in framed:
            continue
        _, ann = split_label(g.nodes[gv].attrs.get("label"))
        derived = "dashed" in (g.nodes[gv].attrs.get("style") or "")
        facts.append((term(gv), ann, derived))
    for e in folded:
        rel, ann = split_label(e.attrs.get("label"))
        is_isa = e.attrs.get("arrowhead") == "empty"
        derived = ("dashed" in (e.attrs.get("style") or "")) and not is_isa
        if e.tail == e.head:
            ambiguous += 1
            f = (rel, term(e.tail)) if loop == "unary" else (rel, term(e.tail), term(e.head))
        else:
            f = (rel, term(e.tail), term(e.head))
        facts.append((f, ann, derived))
    return dict(facts=facts, loops=ambiguous, framed=len(framed & octs), octagons=len(octs))


def write_read2(path: Path, r2: dict, src_name: str, loop: str):
    decls = defaultdict(list)
    rest = []
    for f, ann, derived in r2["facts"]:
        if f[0] == "relation" and all(isinstance(x, str) for x in f[1:]) and len(f) >= 2:
            decls[f[1]].append(f[2:])
        else:
            rest.append((f, ann, derived))
    lines = [f";; reading 2 of {src_name}, a loop read as {'one argument' if loop == 'unary' else 'two'}.",
             ";; A declaration's :why is not in the drawing: load with --no-require-why.", ""]
    for r, sigs in decls.items():
        sig = max(sigs, key=len)
        lines.append("(relation " + " ".join((r,) + tuple(sig)) + ")")
    for f, ann, derived in rest:
        txt = term_text(f)
        if derived and ann.startswith("by "):
            lines.append(f"{txt[:-1]})  ; derived {ann}")
        elif ann:
            lines.append(f"{txt[:-1]} :source {ein_text(ann)})")
        else:
            lines.append(txt)
    path.write_text("\n".join(lines) + "\n", encoding="utf-8")
    return lines


def parse_terms(text: str):
    toks = re.findall(r'"(?:[^"\\]|\\.)*"|[()]|[^\s()"]+', text)
    pos = 0

    def one():
        nonlocal pos
        t = toks[pos]
        pos += 1
        if t != "(":
            return t
        items = []
        while toks[pos] != ")":
            if toks[pos].startswith(":"):
                pos += 1
                one()
                continue
            items.append(one())
        pos += 1
        return tuple(items)

    out = []
    while pos < len(toks):
        out.append(one())
    return out


def facts_of_text(program: Path) -> set:
    """The knowledge base a program with no rule loads to, read off its text by the loader's rule:
    each written fact, and for each declaration (relation r L...) the facts (relation r L...) and
    (relation r). A generated program is loaded, and never saturated."""
    text = "\n".join(ln.split(";", 1)[0] if '"' not in ln else ln for ln in program.read_text().splitlines())
    facts = set()
    for t in parse_terms(text):
        if isinstance(t, tuple):
            facts.add(t)
            if t[0] == "relation" and len(t) > 2:
                facts.add(t[:2])
    return facts


def kb_of(program: Path, extra=()) -> set:
    """The facts of a program's root, as `ein saturate --dump` prints them (an original program
    only; a generated one is loaded and never saturated)."""
    p = run([EIN, "saturate", "--dump", *extra, program.name], cwd=program.parent)
    dump = p.stdout.split("SATURATED KB DUMP", 1)[-1]
    facts = set()
    for ln in dump.splitlines():
        if ln.startswith("  ("):
            facts |= set(parse_terms(ln.strip()))
    return facts


# --------------------------------------------------------------------------------------------
# The survey
# --------------------------------------------------------------------------------------------

DRAWN = HERE / "drawn"
OWN_OUT = HERE / "out"

EXAMPLES = [
    # name, kind, path (a downloaded file's URL is in SOURCE_URL beside it)
    ("turnstile-fsm", "finite state machine", DRAWN / "turnstile-fsm.gv"),
    ("gallery-fsm", "finite state machine (an LR(0) automaton)", EXT / "gallery-fsm" / "fsm.gv"),
    ("pressure-valve", "finite state machine", EXT / "web-c4-irf" / "pressure-valve.gv"),
    ("euclid-flowchart", "algorithm flowchart", DRAWN / "euclid-flowchart.gv"),
    ("gallery-cluster", "processes in clusters", EXT / "gallery-cluster" / "cluster.gv"),
    ("gallery-UML_Class_diagram", "UML class diagram (HTML labels)", EXT / "gallery-UML_Class_diagram" / "UML_Class_diagram.gv"),
    ("uml-classes", "UML class diagram (records)", DRAWN / "uml-classes.gv"),
    ("gallery-datastruct", "data structure (records, ports)", EXT / "gallery-datastruct" / "datastruct.gv"),
    ("c4-context", "C4 system context", DRAWN / "c4-context.gv"),
    ("c4-container", "C4 container", EXT / "web-c4-irf" / "c4-container.gv"),
    ("ein-crates", "dependency graph", DRAWN / "ein-crates.gv"),
    ("gallery-go-package", "dependency graph", EXT / "gallery-go-package" / "go-package.gv"),
    ("gallery-unix", "version history", EXT / "gallery-unix" / "unix.gv"),
    ("gallery-pprof", "call graph (profile)", EXT / "gallery-pprof" / "pprof.gv"),
    ("family-tree", "family tree", DRAWN / "family-tree.gv"),
    ("gallery-lion_share", "pedigree (marriage nodes)", EXT / "gallery-lion_share" / "lion_share.gv"),
    ("gallery-kennedyanc", "family tree (undirected, colour)", EXT / "gallery-kennedyanc" / "kennedyanc.gv"),
    ("gallery-ER", "ER diagram (undirected)", EXT / "gallery-ER" / "ER.gv"),
    ("gallery-traffic_lights", "Petri net", EXT / "gallery-traffic_lights" / "traffic_lights.gv"),
    ("gallery-networkmap_twopi", "network map (undirected, HTML)", EXT / "gallery-networkmap_twopi" / "networkmap_twopi.gv"),
    ("gallery-happiness", "mind map (undirected)", EXT / "gallery-happiness" / "happiness.gv"),
    ("gallery-fdpclust", "undirected, nested clusters", EXT / "gallery-fdpclust" / "fdpclust.gv"),
    ("gallery-git", "concepts and commands (HTML, ports)", EXT / "gallery-git" / "git.gv"),
    ("kernel-words", "Ein's kernel words as labels", DRAWN / "kernel-words.gv"),
    ("edge-cases", "every case reading 1 must decide", DRAWN / "edge-cases.gv"),
]
def _v(**kw):
    return Policy(repair=True, unlabelled="single", **kw)


VARIANTS = {   # extra runs: one plausible repair of a loss, shown on the examples that have it
    "gallery-cluster": [_v(clusters=True)],
    "c4-context": [_v(clusters=True), _v(source_text=True)],
    "ein-crates": [_v(clusters=True)],
    "gallery-fdpclust": [_v(clusters=True)],
    "gallery-datastruct": [_v(ports=True)],
    "gallery-UML_Class_diagram": [_v(ports=True, read_dir=True)],
    "uml-classes": [_v(read_dir=True)],
    "family-tree": [_v(read_dir=True)],
    "gallery-lion_share": [_v(read_dir=True)],
    "gallery-kennedyanc": [Policy(repair=True, colour_as_label=True)],
    "kernel-words": [_v(rename_coincidence=True)],
    "edge-cases": [_v(read_dir=True), _v(rename_coincidence=True), _v(number_parallel=True)],
}
EIN_OWN = [
    ("three-houses", "Ein's own drawing (declarations, is-a, rules)", HERE / "ein" / "three-houses.ein"),
    ("common-belief", "Ein's own drawing (nested facts, negation)", REPO / "examples" / "beliefs" / "01_common_belief.ein"),
    ("turnstile-kb", "Ein's own drawing of reading 1's turnstile (loops of two arguments)",
     OWN_OUT / "turnstile-fsm" / "turnstile-fsm.a.ein"),
]


def source_of(path: Path) -> str:
    u = path.parent / "SOURCE_URL"
    if u.is_file():
        url = u.read_text().strip()
        if path.parent.name == "web-c4-irf":
            url += f" (its {'first' if 'c4' in path.name else 'third'} DOT block)"
        return url
    if path.is_relative_to(HERE):
        return "drawn here"
    return str(path.relative_to(REPO))


def out_dir_of(name: str, path: Path) -> Path:
    d = (OWN_OUT / name) if path.is_relative_to(HERE) else (path.parent / "out")
    d.mkdir(parents=True, exist_ok=True)
    return d


def survey_one(name, kind, path) -> dict:
    g = load_dot(path)
    out = out_dir_of(name, path)
    unl = [e for e in g.edges if not e.attrs.get("label")]
    pairs = Counter((e.tail, e.head) if g.directed else tuple(sorted((e.tail, e.head))) for e in unl)
    rec = dict(name=name, kind=kind, source=source_of(path), path=str(path.relative_to(REPO)),
               directed=g.directed, nodes=len(g.nodes), edges=len(g.edges), clusters=len(g.clusters),
               unlabelled=len(unl), unlabelled_parallel=sum(v for v in pairs.values() if v > 1),
               runs={})
    pols = [Policy(), Policy(repair=True, unlabelled="single"), Policy(repair=True, unlabelled="distinct")]
    pols += VARIANTS.get(name, [])
    for pol in pols:
        res = read1(g, pol, out)
        tag = pol.tag()
        run_rec = dict(tag=tag, refused=res.refused, failures=res.failures, notes=res.notes,
                       facts=len(res.facts) if not res.refused else None,
                       relations=len({f[0] for f in res.facts}) if not res.refused else None)
        if not res.refused:
            prog = out / f"{name}.{tag}.ein"
            write_program(prog, res, name, path.name, pol)
            run_rec["program"] = str(prog.relative_to(REPO))
            run_rec["load"] = load_check(prog)
            if run_rec["load"]["loads"]:
                run_rec["round_trip"] = round_trip(prog, res, out / f"{name}.{tag}.kb.dot")
        if tag in ("a", "b") and rec["unlabelled"]:
            # the same drawing with its arrows written in the reverse order: is it the same KB?
            rev = Graph(g.name, g.directed, g.strict, g.nodes, g.clusters, list(reversed(g.edges)), g.nsub)
            again = read1(rev, pol, out)
            run_rec["same_kb_reversed"] = ({(r, a_) for r, a_, _ in again.facts} == {(r, a_) for r, a_, _ in res.facts})
        rec["runs"][tag] = run_rec
        if tag in ("a", "b"):
            for r, args, _ in res.facts:
                NAMES_SEEN.update(((r, "lone" if not args else "relation"),) + tuple((x, "argument") for x in args))
    return rec


NAMES_SEEN: set = set()


def survey_own(name, kind, prog: Path) -> dict:
    out = out_dir_of(name, HERE / "ein" / "x")
    dot_path = HERE / "ein" / f"{name}.kb.dot"
    render_kb(prog, dot_path)
    g = load_dot(dot_path)
    rec = dict(name=name, kind=kind, source=str(prog.relative_to(REPO)), path=str(dot_path.relative_to(REPO)),
               directed=True, nodes=len(g.nodes), edges=len(g.edges), clusters=len(g.clusters), runs={})
    # reading 1, strictly, on Ein's own drawing
    res = read1(g, Policy(), out)
    rec["runs"]["strict"] = dict(tag="strict", refused=res.refused, failures=res.failures, notes=res.notes,
                                 facts=len(res.facts), relations=len({f[0] for f in res.facts}))
    original = kb_of(prog) if prog.is_relative_to(REPO / "examples") or prog.parent.name == "ein" else facts_of_text(prog)
    for loop in ("unary", "binary"):
        r2 = read2(g, loop)
        p2 = out / f"{name}.r2-{loop}.ein"
        write_read2(p2, r2, dot_path.name, loop)
        load = load_check(p2, ("--no-require-why",))
        r = dict(tag=f"r2-{loop}", facts=len(r2["facts"]), loops=r2["loops"], framed=r2["framed"],
                 octagons=r2["octagons"], load=load, program=str(p2.relative_to(REPO)))
        if load["loads"]:
            back_dot = out / f"{name}.r2-{loop}.kb.dot"
            render_kb(p2, back_dot, ("--no-require-why",))
            gb = load_dot(back_dot)
            n1, e1 = by_id(g)
            n2, e2 = by_id(gb)
            r["drawing_equal"] = (n1 == n2 and e1 == e2)
            r["drawing_nodes"] = (len(n1), len(n2), len(n1 & n2))
            r["drawing_edges"] = (len(e1), len(e2), len(e1 & e2))
            r["drawing_differs_in"] = sorted({"style" for _ in [0] if styles(g) != styles(gb)}
                                             | {"annotation" for _ in [0] if annotations(g) != annotations(gb)})
            back = facts_of_text(p2)
            r["kb_original"] = len(original)
            r["kb_back"] = len(back)
            r["kb_missing"] = sorted(term_text(f) for f in original - back)
            r["kb_extra"] = sorted(term_text(f) for f in back - original)
        rec["runs"][r["tag"]] = r
    return rec


def by_id(g: Graph):
    """A drawing as node ids with their shape and first word, and arrows as (tail id, word, head id):
    a fact's node id is a hash of the fact, so two drawings of one knowledge base share it."""
    word = lambda raw: split_label(raw)[0]
    nodes = {(n.id, n.attrs.get("shape", ""), word(n.attrs.get("label")) if n.attrs.get("shape") == "octagon" else "")
             for n in g.nodes.values()}
    edges = {(g.nodes[e.tail].id, word(e.attrs.get("label")), g.nodes[e.head].id) for e in g.edges}
    return nodes, edges


def styles(g: Graph):
    return {(n.id, n.attrs.get("style")) for n in g.nodes.values()} | \
           {(e.tail, e.head, e.attrs.get("style")) for e in g.edges}


def annotations(g: Graph):
    return {split_label(n.attrs.get("label"))[1] for n in g.nodes.values()} | \
           {split_label(e.attrs.get("label"))[1] for e in g.edges}


def label_oracle(records) -> dict:
    """Every single-token label the survey met, asked of the binary: is it a name, as an argument
    and as a relation? The prototype's own check is held to the answers."""
    words = {w for w, _ in NAMES_SEEN if w.isascii()}
    for rec in records:
        for run in rec["runs"].values():
            for f in run.get("failures", []):
                t = f.get("text")
                if t and not re.search(r'[\s()";]', t) and t.isascii():
                    words.add(t)
    for kw in KERNEL_AS_RELATION | KERNEL_MARKERS | {"step", "true", "model", "is-a"}:
        words.add(kw)
    agree, disagree = 0, []
    with tempfile.TemporaryDirectory() as td:
        for w in sorted(words):
            for place, prog in (("argument", f"(r {w} x)"), ("relation", f"({w} a b)")):
                f = Path(td) / "p.ein"
                f.write_text(prog + "\n")
                ein_ok = load_check(f)
                ein_says = ein_ok["loads"] and ein_ok["root"] == "alive"
                ours = why_not_a_name(w, place, Path(td)) is None
                if ein_says == ours:
                    agree += 1
                else:
                    disagree.append(dict(word=w, place=place, ein=ein_says, prototype=ours,
                                         first=ein_ok.get("first", "") or ein_ok.get("root")))
    return dict(words=len(words), agree=agree, disagree=disagree)


def survey(only=None):
    records = []
    for name, kind, path in EXAMPLES:
        if only and name not in only:
            continue
        print(f"-- {name}", file=sys.stderr)
        records.append(survey_one(name, kind, path))
    for name, kind, prog in EIN_OWN:
        if only and name not in only:
            continue
        print(f"-- {name}", file=sys.stderr)
        records.append(survey_own(name, kind, prog))
    oracle = label_oracle(records) if not only else None
    for rec in records:
        for run_rec in rec["runs"].values():
            run_rec["failures"] = summarise(run_rec.get("failures", []))
    data = dict(binary=Path(EIN).name, images_stripped=sorted(IMAGES_STRIPPED), examples=records, oracle=oracle)
    EXT.mkdir(parents=True, exist_ok=True)
    (EXT / "results.json").write_text(json.dumps(data, indent=1, ensure_ascii=False, default=list) + "\n",
                                      encoding="utf-8")
    # beside the prototype: a downloaded drawing's counts, and none of its text
    mine = json.loads(json.dumps(data, default=list))
    for rec in mine["examples"]:
        if rec["path"].startswith("build/"):
            for run_rec in rec["runs"].values():
                for f in run_rec.get("failures", []):
                    f.pop("samples", None)
                for k in ("round_trip",):
                    if k in run_rec:
                        run_rec[k].pop("two_missing", None)
    if mine["oracle"]:
        mine["oracle"]["disagree"] = len(mine["oracle"]["disagree"])
    (HERE / "results.json").write_text(json.dumps(mine, indent=1, ensure_ascii=False) + "\n", encoding="utf-8")
    print_table(records)
    print_kinds(records)
    if oracle:
        print(f"\nlabel oracle: {oracle['words']} words, {oracle['agree']} agreements, "
              f"{len(oracle['disagree'])} disagreements", file=sys.stderr)
        for d in oracle["disagree"]:
            print("  ", d, file=sys.stderr)


def summarise(failures):
    """Per kind and sub-kind: how many, whether a refusal, and the first three cases."""
    groups = defaultdict(list)
    for f in failures:
        groups[(f["kind"], f["sub"], f["refusal"])].append(f)
    out = []
    for (k, sub, ref), fs in groups.items():
        n = sum(f.get("count", 1) for f in fs)
        out.append(dict(kind=k, sub=sub, refusal=ref, count=n,
                        samples=[{x: y for x, y in f.items() if x not in ("kind", "sub", "refusal")} for f in fs[:3]]))
    return out


def print_kinds(records):
    """Across the examples read by reading 1: per failure kind, in how many examples, how many
    cases, under the strict reading."""
    per = defaultdict(lambda: [0, 0, set()])
    for rec in records:
        if rec["name"] in [n for n, _, _ in EIN_OWN]:
            continue
        for f in rec["runs"]["strict"]["failures"]:
            k = per[(f["kind"], f["sub"], f["refusal"])]
            k[1] += f["count"]
            k[2].add(rec["name"])
    print("\n## failure kinds, strict reading 1, drawings read by reading 1")
    for (kind, sub, ref), (_, n, ex) in sorted(per.items()):
        print(f"  {kind:28} {sub:34} {'refused' if ref else 'lost   '} {n:5} cases in {len(ex):2} examples")


def print_table(records):
    for rec in records:
        print(f"\n## {rec['name']} — {rec['kind']} ({rec['nodes']} nodes, {rec['edges']} edges, "
              f"{rec['clusters']} clusters, {'directed' if rec['directed'] else 'undirected'})")
        for tag, r in rec["runs"].items():
            kinds = Counter({(f["kind"], f["sub"]): f["count"] for f in r.get("failures", [])})
            ref = sum(f["count"] for f in r.get("failures", []) if f["refusal"])
            line = f"  {tag:16} facts {r.get('facts')} rel {r.get('relations', '-')} refusals {ref}"
            if "same_kb_reversed" in r:
                line += f" | same KB reversed {r['same_kb_reversed']}"
            if "load" in r:
                ld = r["load"]
                line += f" | load {'ok ' + str(ld['root']) if ld['loads'] else 'REFUSED ' + ld['first'][:90]}"
            if "round_trip" in r:
                t = r["round_trip"]
                line += (f" | rt two {t['two_back']}/{t['two']} lone-oct {t['lone_as_octagon']}/{t['lone']}"
                         f" read1-back-equal {t['read1_back_equal']} raw-edges {t['input_edges_verbatim']}/{t['input_edges']}"
                         f" raw-nodes {t['input_nodes_verbatim']}/{t['input_nodes']}")
            if "drawing_equal" in r:
                line += (f" | drawing-equal {r['drawing_equal']} nodes {r['drawing_nodes']} edges {r['drawing_edges']}"
                         f" kb {r['kb_back']}/{r['kb_original']} missing {r['kb_missing']} extra {r['kb_extra']}")
            print(line)
            for (k, s), v in sorted(kinds.items()):
                print(f"      {k}: {s} ×{v}")


def main():
    ap = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    sub = ap.add_subparsers(dest="cmd", required=True)
    a = sub.add_parser("read1")
    a.add_argument("dot")
    a.add_argument("out")
    a.add_argument("--repair", action="store_true")
    a.add_argument("--unlabelled", choices=["single", "distinct"])
    a.add_argument("--source-text", action="store_true")
    b = sub.add_parser("read2")
    b.add_argument("dot")
    b.add_argument("out")
    b.add_argument("--loop", choices=["unary", "binary"], default="unary")
    s = sub.add_parser("survey")
    s.add_argument("--only", nargs="*")
    args = ap.parse_args()
    if args.cmd == "read1":
        out = Path(args.out)
        pol = Policy(repair=args.repair or bool(args.unlabelled), unlabelled=args.unlabelled,
                     source_text=args.source_text)
        res = read1(load_dot(Path(args.dot)), pol, out.parent)
        for f in res.failures:
            print(("refused " if f["refusal"] else "lost    ") + f"{f['kind']}: {f['sub']}: {f['item']}", file=sys.stderr)
        if res.refused:
            sys.exit(1)
        write_program(out, res, out.stem, Path(args.dot).name, pol)
    elif args.cmd == "read2":
        r2 = read2(load_dot(Path(args.dot)), args.loop)
        write_read2(Path(args.out), r2, Path(args.dot).name, args.loop)
    else:
        survey(args.only)


if __name__ == "__main__":
    main()
