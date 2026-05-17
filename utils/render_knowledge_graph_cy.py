#!/usr/bin/env python3
"""
Render docs/index/knowledge-graph.dot into a Cytoscape.js view.

Produces, under docs/index/knowledge-graph.cy/ :

    elements.js   window.cyElements = [ … ]    — compound nodes + leaf nodes + edges
    style.js      window.cyStyle    = [ … ]    — Cytoscape style array
    index.html    template.html with elements.js + style.js inlined

The Cytoscape core and the fcose layout extension are *not* embedded — the
template loads them from unpkg CDN.

Usage:  utils/render_knowledge_graph_cy.py
"""
from __future__ import annotations

import itertools
import json
import re
import sys
from pathlib import Path

REPO_ROOT = Path(__file__).resolve().parent.parent
SRC_DOT   = REPO_ROOT / "docs" / "index" / "knowledge-graph.dot"
OUT_DIR   = REPO_ROOT / "docs" / "index" / "knowledge-graph.cy"
TEMPLATE  = OUT_DIR / "template.html"

# ---------------------------------------------------------------------------
# DOT → Cytoscape encoding tables
#
# knowledge-graph.dot uses (shape, fillcolor) as a coupled visual encoding
# (see the legend at the top of the .dot file). We collapse the pair into
# a single semantic `kind` tag and emit one style rule per kind.
# ---------------------------------------------------------------------------

SHAPE_TO_CY: dict[str, tuple[str, str]] = {
    "ellipse":        ("ellipse",         "concept"),
    "hexagon":        ("hexagon",         "algorithm"),
    "box3d":          ("rectangle",       "software"),
    "note":           ("tag",             "paper"),
    "cylinder":       ("barrel",          "language"),
    "folder":         ("cut-rectangle",   "data_structure"),
    "tab":            ("round-tag",       "standard"),
    "diamond":        ("diamond",         "problem_class"),
    "doubleoctagon":  ("octagon",         "cog_arch"),
    "parallelogram":  ("rhomboid",        "notation"),
    "component":      ("round-rectangle", "proof"),
    "septagon":       ("heptagon",        "domain"),
}

KIND_BG: dict[str, str] = {
    "concept":         "#cfe6f5",  # ~ DOT lightblue
    "algorithm":       "#fff3a8",  # ~ DOT khaki1
    "software":        "#b8edb8",  # ~ DOT palegreen
    "paper":           "#ffd9b8",  # ~ DOT peachpuff
    "language":        "#ffd1da",  # ~ DOT pink
    "data_structure":  "#ecc5ff",  # ~ DOT plum1
    "standard":        "#dddddd",  # ~ DOT lightgrey
    "problem_class":   "#ffe066",  # ~ DOT gold
    "cog_arch":        "#f7a4a4",  # ~ DOT lightcoral
    "notation":        "#d4f4f4",  # ~ DOT lightcyan
    "proof":           "#efd7a8",  # ~ DOT wheat
    "domain":          "#dbc6dc",  # ~ DOT thistle
}

EDGE_COLOR: dict[str, str] = {
    "purple":    "#7e3aa1",
    "red":       "#c0392b",
    "blue":      "#2a6fcf",
    "darkgreen": "#1f7a30",
    "grey40":    "#666666",
    "grey30":    "#4d4d4d",
}
DEFAULT_EDGE_COLOR = "#888888"


# ---------------------------------------------------------------------------
# Minimal DOT parser
#
# Covers the subset used by knowledge-graph.dot:
#   - digraph header
#   - nested  subgraph cluster_XXX { … }  blocks
#   - default attr blocks:  graph|node|edge [ … ] ;
#   - node decl:  IDENT [ … ] ;
#   - edge decl:  IDENT (-> IDENT)+ [ … ] ;
#   - cluster attrs:  key = value ;
#   - quoted strings with `\n` escapes
#   - // line and /* block */ comments
# ---------------------------------------------------------------------------

class DotParser:
    def __init__(self, text: str) -> None:
        self.text = self._strip_comments(text)
        self.pos  = 0
        self.clusters: dict[str, dict] = {}     # cid → {parent, attrs}
        self.nodes:    dict[str, dict] = {}     # nid → {parent, attrs}
        self.edges:    list[dict]      = []     # ordered list
        self.edge_defaults: dict[str, str] = {}

    # ---- comment stripping ------------------------------------------------
    @staticmethod
    def _strip_comments(text: str) -> str:
        text = re.sub(r"/\*.*?\*/", "", text, flags=re.DOTALL)
        out, in_str, i = [], False, 0
        while i < len(text):
            c = text[i]
            if c == '"' and (i == 0 or text[i - 1] != "\\"):
                in_str = not in_str
                out.append(c)
                i += 1
                continue
            if (not in_str and c == "/"
                    and i + 1 < len(text) and text[i + 1] == "/"):
                while i < len(text) and text[i] != "\n":
                    i += 1
                continue
            out.append(c)
            i += 1
        return "".join(out)

    # ---- low-level helpers ------------------------------------------------
    def _skip_ws(self) -> None:
        while self.pos < len(self.text) and self.text[self.pos].isspace():
            self.pos += 1

    def _consume(self, lit: str) -> bool:
        if self.text.startswith(lit, self.pos):
            self.pos += len(lit)
            return True
        return False

    _RE_NUM   = re.compile(r"-?(?:\.\d+|\d+\.?\d*)")
    _RE_IDENT = re.compile(r"[A-Za-z_-￿][A-Za-z0-9_-￿]*")

    def _read_id(self) -> str:
        """Read either a quoted string, a bare identifier, or a numeral.

        Per DOT spec, bare identifiers are `[A-Za-z_][A-Za-z0-9_]*` —
        importantly hyphens are NOT permitted inside them, so `Idris->X`
        tokenises as `Idris`, `->`, `X` (not `Idris-`, `>`, `X`).
        """
        self._skip_ws()
        if self.pos >= len(self.text):
            return ""
        if self.text[self.pos] == '"':
            self.pos += 1
            buf: list[str] = []
            while self.pos < len(self.text):
                c = self.text[self.pos]
                if c == "\\" and self.pos + 1 < len(self.text):
                    nxt = self.text[self.pos + 1]
                    # \n / \l / \r in DOT all introduce a line break.
                    buf.append({"n": "\n", "l": "\n", "r": "\n"}.get(nxt, nxt))
                    self.pos += 2
                    continue
                if c == '"':
                    self.pos += 1
                    return "".join(buf)
                buf.append(c)
                self.pos += 1
            raise ValueError("unterminated string literal")
        m = self._RE_NUM.match(self.text, self.pos)
        if m and m.group():
            self.pos = m.end()
            return m.group()
        m = self._RE_IDENT.match(self.text, self.pos)
        if m:
            self.pos = m.end()
            return m.group()
        return ""

    def _read_attr_list(self) -> dict[str, str]:
        attrs: dict[str, str] = {}
        self._skip_ws()
        if not self._consume("["):
            return attrs
        while True:
            self._skip_ws()
            if self._consume("]"):
                break
            key = self._read_id()
            if not key:               # malformed — bail
                self._skip_ws()
                self._consume("]")
                break
            self._skip_ws()
            if self._consume("="):
                attrs[key] = self._read_id()
            self._skip_ws()
            self._consume(",") or self._consume(";")
        return attrs

    # ---- top-level --------------------------------------------------------
    def parse(self) -> None:
        m = re.match(r"\s*(?:strict\s+)?(?:di)?graph(?:\s+\w+)?\s*\{",
                     self.text[self.pos:])
        if not m:
            raise ValueError("expected graph header")
        self.pos += m.end()
        self._parse_block(parent=None)

    def _parse_block(self, parent: str | None) -> None:
        while self.pos < len(self.text):
            self._skip_ws()
            if self.pos >= len(self.text):
                return
            if self._consume("}"):
                return
            if self._consume(";"):
                continue

            # `subgraph NAME { … }`
            m = re.match(r"subgraph\s+([A-Za-z_]\w*)\s*\{",
                         self.text[self.pos:])
            if m:
                cid = m.group(1)
                self.pos += m.end()
                self.clusters[cid] = {"parent": parent, "attrs": {}}
                self._parse_block(parent=cid)
                continue
            # anonymous `subgraph { … }`
            m = re.match(r"subgraph\s*\{", self.text[self.pos:])
            if m:
                self.pos += m.end()
                self._parse_block(parent=parent)
                continue

            # default attr blocks
            m = re.match(r"(graph|node|edge)\b", self.text[self.pos:])
            if m:
                kind = m.group(1)
                self.pos += len(kind)
                attrs = self._read_attr_list()
                if kind == "edge":
                    self.edge_defaults.update(attrs)
                self._skip_ws()
                self._consume(";")
                continue

            ident = self._read_id()
            if not ident:
                self.pos += 1
                continue
            self._skip_ws()

            # cluster/graph-level attribute  KEY = VALUE
            if self._consume("="):
                value = self._read_id()
                if parent is not None:
                    self.clusters[parent]["attrs"][ident] = value
                self._skip_ws()
                self._consume(";")
                continue

            # node declaration
            if self.text.startswith("[", self.pos):
                attrs = self._read_attr_list()
                self.nodes[ident] = {"parent": parent, "attrs": attrs}
                self._skip_ws()
                self._consume(";")
                continue

            # edge declaration (possibly chained)
            if (self.text.startswith("->", self.pos)
                    or self.text.startswith("--", self.pos)):
                chain = [ident]
                while self._consume("->") or self._consume("--"):
                    chain.append(self._read_id())
                    self._skip_ws()
                attrs = self._read_attr_list()
                merged = dict(self.edge_defaults)
                merged.update(attrs)
                for s, t in itertools.pairwise(chain):
                    self.edges.append(
                        {"source": s, "target": t, "attrs": dict(merged)})
                self._skip_ws()
                self._consume(";")
                continue

            # bare node reference
            self.nodes.setdefault(ident, {"parent": parent, "attrs": {}})
            self._skip_ws()
            self._consume(";")


# ---------------------------------------------------------------------------
# DOT model → Cytoscape elements / style
# ---------------------------------------------------------------------------

def _top_cluster(cid: str | None, clusters: dict[str, dict]) -> str | None:
    cur = cid
    while cur is not None and clusters.get(cur, {}).get("parent") is not None:
        cur = clusters[cur]["parent"]
    return cur


def build_elements(p: DotParser) -> list[dict]:
    elements: list[dict] = []

    # 1) Compound nodes for clusters.
    for cid, info in p.clusters.items():
        attrs  = info["attrs"]
        depth, cur = 0, info["parent"]
        while cur is not None:
            depth += 1
            cur = p.clusters.get(cur, {}).get("parent")
        data = {
            "id":     cid,
            "label":  attrs.get("label", cid),
            "fill":   attrs.get("fillcolor", "#f6f6f6"),
            "border": attrs.get("color",     "#bbbbbb"),
            "depth":  depth,
        }
        if info["parent"]:
            data["parent"] = info["parent"]
        elements.append(
            {"data": data, "classes": f"cluster cluster-d{depth}"})

    # 2) Leaf nodes.
    for nid, info in p.nodes.items():
        a       = info["attrs"]
        shape   = a.get("shape", "ellipse")
        _, kind = SHAPE_TO_CY.get(shape, ("ellipse", "concept"))
        data = {
            "id":        nid,
            "label":     a.get("label", nid),
            "kind":      kind,
            "dotShape":  shape,
            "dotFill":   a.get("fillcolor", ""),
        }
        if info["parent"]:
            data["parent"] = info["parent"]
        elements.append({"data": data})

    # 3) Edges.
    for i, e in enumerate(p.edges):
        a   = e["attrs"]
        src, tgt = e["source"], e["target"]
        if a.get("dir") == "back":
            src, tgt = tgt, src

        s_top = _top_cluster(p.nodes.get(src, {}).get("parent"), p.clusters)
        t_top = _top_cluster(p.nodes.get(tgt, {}).get("parent"), p.clusters)
        cross = bool(s_top and t_top and s_top != t_top)

        data = {
            "id":     f"e{i:04d}",
            "source": src,
            "target": tgt,
            "label":  a.get("label", ""),
            "eStyle": a.get("style", "solid"),
            "eColor": a.get("color", "default"),
        }
        if a.get("penwidth"):
            data["penwidth"] = a["penwidth"]
        if cross:
            data["cross"] = 1
        if a.get("style") == "invis":
            data["invis"] = 1
        elements.append({"data": data})

    return elements


def build_style() -> list[dict]:
    style: list[dict] = []

    style.append({
        "selector": "node",
        "style": {
            "label":            "data(label)",
            "text-wrap":        "wrap",
            "text-max-width":   140,
            "text-valign":      "center",
            "text-halign":      "center",
            "font-family":      "Helvetica, Arial, sans-serif",
            "font-size":        10,
            "color":            "#222",
            "background-color": "#f8f8f8",
            "border-width":     1,
            "border-color":     "#666",
            "width":            "label",
            "height":           "label",
            "padding":          "8px",
            "shape":            "round-rectangle",
        }
    })

    style.append({
        "selector": ":parent",
        "style": {
            "shape":              "round-rectangle",
            "background-color":   "data(fill)",
            "background-opacity": 0.45,
            "border-width":       1,
            "border-color":       "data(border)",
            "padding":            "18px",
            "text-valign":        "top",
            "text-halign":        "center",
            "font-size":          12,
            "font-weight":        "bold",
            "color":              "#333",
        }
    })
    style.append({
        "selector": "node.cluster-d0",
        "style": {
            "font-size":          13,
            "background-opacity": 0.30,
            "border-width":       2,
        }
    })
    style.append({
        "selector": "node.cluster-d1",
        "style": {
            "font-size":          11,
            "background-opacity": 0.55,
            "border-style":       "dashed",
        }
    })

    for _shape, (cy_shape, kind) in SHAPE_TO_CY.items():
        style.append({
            "selector": f'node[kind = "{kind}"]',
            "style": {
                "shape":            cy_shape,
                "background-color": KIND_BG[kind],
                "border-color":     "#555",
            }
        })

    style.append({
        "selector": "edge",
        "style": {
            "label":                   "data(label)",
            "font-size":               8,
            "font-family":             "Helvetica, Arial, sans-serif",
            "color":                   "#444",
            "line-color":              DEFAULT_EDGE_COLOR,
            "target-arrow-color":      DEFAULT_EDGE_COLOR,
            "target-arrow-shape":      "triangle",
            "curve-style":             "bezier",
            "width":                   1.2,
            "text-background-color":   "#ffffff",
            "text-background-opacity": 0.85,
            "text-background-padding": "2px",
            "text-rotation":           "autorotate",
        }
    })
    style.append({"selector": 'edge[eStyle = "dashed"]',
                  "style":    {"line-style": "dashed"}})
    style.append({"selector": 'edge[eStyle = "dotted"]',
                  "style":    {"line-style": "dotted"}})
    style.append({"selector": 'edge[eStyle = "bold"]',
                  "style":    {"width": 2.6}})

    for dot_color, css in EDGE_COLOR.items():
        style.append({
            "selector": f'edge[eColor = "{dot_color}"]',
            "style": {
                "line-color":         css,
                "target-arrow-color": css,
            }
        })

    style.append({"selector": "edge[?invis]",
                  "style":    {"display": "none"}})
    style.append({"selector": "edge[?cross]",
                  "style":    {"z-index": 10, "opacity": 0.95}})

    return style


# ---------------------------------------------------------------------------
# Output: elements.js / style.js / single-file index.html
# ---------------------------------------------------------------------------

def _js_dump(value, var_name: str) -> str:
    body = json.dumps(value, indent=2, ensure_ascii=False)
    return f"window.{var_name} = {body};\n"


INJECT_RE = re.compile(
    r"<!--\s*INJECT:(?P<name>[\w.\-]+)\s*-->.*?<!--\s*/INJECT:(?P=name)\s*-->",
    re.DOTALL,
)


def _inline(template: str, scripts: dict[str, str]) -> str:
    def repl(m: re.Match) -> str:
        name = m.group("name")
        body = scripts.get(name)
        if body is None:
            return m.group(0)
        return f"<!-- inlined: {name} -->\n<script>\n{body}\n</script>"
    return INJECT_RE.sub(repl, template)


def _warn_dangling(p: DotParser) -> None:
    declared = set(p.nodes)
    missing  = {ep for e in p.edges for ep in (e["source"], e["target"])
                if ep and ep not in declared}
    if missing:
        print(f"warning: {len(missing)} edge endpoint(s) refer to "
              f"undeclared nodes: {sorted(missing)[:8]}…", file=sys.stderr)


def main() -> int:
    if not SRC_DOT.exists():
        print(f"error: source not found: {SRC_DOT}", file=sys.stderr)
        return 1
    if not TEMPLATE.exists():
        print(f"error: template not found: {TEMPLATE}", file=sys.stderr)
        return 1

    parser = DotParser(SRC_DOT.read_text(encoding="utf-8"))
    parser.parse()
    _warn_dangling(parser)

    elements = build_elements(parser)
    style    = build_style()

    OUT_DIR.mkdir(parents=True, exist_ok=True)
    elements_js = _js_dump(elements, "cyElements")
    style_js    = _js_dump(style,    "cyStyle")
    (OUT_DIR / "elements.js").write_text(elements_js, encoding="utf-8")
    (OUT_DIR / "style.js"   ).write_text(style_js,    encoding="utf-8")

    bundled = _inline(
        TEMPLATE.read_text(encoding="utf-8"),
        {"elements.js": elements_js, "style.js": style_js},
    )
    (OUT_DIR / "index.html").write_text(bundled, encoding="utf-8")

    print(f"parsed: {len(parser.clusters)} clusters, "
          f"{len(parser.nodes)} nodes, {len(parser.edges)} edges")
    print(f"wrote   {OUT_DIR/'elements.js'}  ({len(elements_js):>8,} bytes)")
    print(f"wrote   {OUT_DIR/'style.js'}     ({len(style_js):>8,} bytes)")
    print(f"wrote   {OUT_DIR/'index.html'}   ({len(bundled):>8,} bytes)")
    return 0


if __name__ == "__main__":
    sys.exit(main())
