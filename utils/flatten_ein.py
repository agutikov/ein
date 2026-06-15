#!/usr/bin/env python3
"""flatten_ein.py — migrate wrapped `.ein` files to flat forms (P1.7c S1.7c.4).

The block wrappers `(ontology …)` / `(facts …)` / `(reasoning …)` /
`(rules …)` are gone post-P1.7c: a program is a flat sequence of forms,
classified by head (see `docs/kernel/ir/03-ein-lang/06_reserved_names.md`).
This tool rewrites a wrapped file to that flat form **KB-preservingly**:

- it hoists each wrapper's children to the top level (comment-preserving —
  a structural text transform, NOT parse∘dump, so the puzzle's comments
  and layout survive), dedenting them by one block level;
- for each *fact* child whose layer the flat loader would not re-derive
  from its own annotations (S1.7c.1: `:rule`/`:using`→REASONING,
  `:source`→FACT, else ONTOLOGY), it appends an explicit
  `:layer <ontology|fact|reasoning>` so the byte-identical KB is preserved
  (the three edge cases: a sourced ONTOLOGY fact, an unsourced FACT, an
  authored REASONING fact). Relation / rule / hrule decls never get a
  `:layer`.

Modes:
  --verify   load the original AND the rewrite, diff the resulting KB
             (facts + per-fact layer + provenance + relations + rules);
             exit non-zero on any divergence. The S1.7c.1/.3 oracle.
  --in-place rewrite the file(s) on disk (implies a --verify gate first).
  --check    rewrite to stdout (default); report whether a change is needed.

Usage:
  utils/flatten_ein.py FILE...                 # print flattened to stdout
  utils/flatten_ein.py --verify FILE...        # KB-equivalence check only
  utils/flatten_ein.py --in-place FILE...      # rewrite + verify each
  utils/flatten_ein.py -h | --help
"""
from __future__ import annotations

import argparse
import sys
from dataclasses import dataclass
from pathlib import Path

# ── ein source scanner (string / comment aware) ────────────────────
#
# A hand-rolled char scanner is used (not the Lark parser) because the
# transform must preserve comments + layout, which the AST discards.

_WRAPPERS = {"ontology": "ontology", "facts": "fact", "reasoning": "reasoning"}
_WRAPPERS["rules"] = None  # rules has no layer — its children are decls
_DECL_HEADS = {"relation", "rule", "hrule"}
_SYMBOL_CHARS = set(
    "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789_*-",
)


@dataclass
class Span:
    start: int       # index of the opening '('
    end: int         # index just past the matching ')'
    head: str        # first symbol after '('  (e.g. "ontology", "rule")
    body_lo: int     # index just past the head token
    body_hi: int     # index of the closing ')'


def _skip_ws_comments(text: str, i: int, n: int) -> int:
    """Advance past whitespace / line / block comments (not into forms)."""
    while i < n:
        c = text[i]
        if c in " \t\r\n":
            i += 1
        elif c == ";":
            while i < n and text[i] != "\n":
                i += 1
        elif c == "#" and i + 1 < n and text[i + 1] == "|":
            j = text.find("|#", i + 2)
            i = n if j < 0 else j + 2
        else:
            break
    return i


def _scan_form(text: str, start: int, n: int) -> Span:
    """Scan one parenthesised form beginning at ``text[start] == '('``."""
    assert text[start] == "("
    i = start + 1
    i = _skip_ws_comments(text, i, n)
    # Head token (a SYMBOL / EQ / etc.); empty for "()".
    head_lo = i
    while i < n and text[i] in _SYMBOL_CHARS:
        i += 1
    head = text[head_lo:i]
    body_lo = i
    # Walk to the matching ')'.
    depth = 1
    in_str = False
    while i < n:
        c = text[i]
        if in_str:
            if c == "\\":
                i += 2
                continue
            if c == '"':
                in_str = False
            i += 1
            continue
        if c == '"':
            in_str = True
            i += 1
            continue
        if c == ";":
            while i < n and text[i] != "\n":
                i += 1
            continue
        if c == "#" and i + 1 < n and text[i + 1] == "|":
            j = text.find("|#", i + 2)
            i = n if j < 0 else j + 2
            continue
        if c == "(":
            depth += 1
        elif c == ")":
            depth -= 1
            if depth == 0:
                return Span(start, i + 1, head, body_lo, i)
        i += 1
    raise ValueError(f"unbalanced parens from offset {start}")


def _top_forms(text: str) -> list[Span]:
    """All depth-0 forms, in source order."""
    spans: list[Span] = []
    i, n = 0, len(text)
    while i < n:
        i = _skip_ws_comments(text, i, n)
        if i >= n:
            break
        if text[i] == "(":
            span = _scan_form(text, i, n)
            spans.append(span)
            i = span.end
        else:  # a bare top-level token (shouldn't occur in valid input)
            i += 1
    return spans


def _child_forms(text: str, lo: int, hi: int) -> list[Span]:
    """Depth-1 forms inside a wrapper body ``text[lo:hi]``."""
    spans: list[Span] = []
    i = lo
    while i < hi:
        i = _skip_ws_comments(text, i, hi)
        if i >= hi:
            break
        if text[i] == "(":
            span = _scan_form(text, i, hi)
            spans.append(span)
            i = span.end
        else:
            i += 1
    return spans


def _top_kw_names(text: str, span: Span) -> set[str]:
    """The keyword names at the child form's own (depth-1) level."""
    names: set[str] = set()
    i, hi = span.body_lo, span.body_hi
    depth = 0
    in_str = False
    while i < hi:
        c = text[i]
        if in_str:
            if c == "\\":
                i += 2
                continue
            if c == '"':
                in_str = False
            i += 1
            continue
        if c == '"':
            in_str = True
        elif c == ";":
            while i < hi and text[i] != "\n":
                i += 1
            continue
        elif c == "#" and i + 1 < hi and text[i + 1] == "|":
            j = text.find("|#", i + 2)
            i = hi if j < 0 else j + 2
            continue
        elif c == "(":
            depth += 1
        elif c == ")":
            depth -= 1
        elif c == ":" and depth == 0:
            k = i + 1
            while k < hi and text[k] in _SYMBOL_CHARS:
                k += 1
            names.add(text[i + 1:k])
            i = k
            continue
        i += 1
    return names


def _derived_layer(kw: set[str]) -> str:
    if "rule" in kw or "using" in kw:
        return "reasoning"
    if "source" in kw:
        return "fact"
    return "ontology"


def _col_of(text: str, idx: int) -> int:
    """0-based column of ``idx`` (distance from the previous newline)."""
    nl = text.rfind("\n", 0, idx)
    return idx - (nl + 1)


def _dedent_block(block: str, amount: int) -> str:
    """Remove up to ``amount`` leading spaces from each line but the first."""
    if amount <= 0:
        return block
    lines = block.split("\n")
    out = [lines[0]]
    for ln in lines[1:]:
        j = 0
        while j < amount and j < len(ln) and ln[j] == " ":
            j += 1
        out.append(ln[j:])
    return "\n".join(out)


def _unwrap(text: str, span: Span) -> str:
    """Flatten one wrapper form to its hoisted, layer-annotated children."""
    block_layer = _WRAPPERS[span.head]
    children = _child_forms(text, span.body_lo, span.body_hi)
    head_col = _col_of(text, span.start)

    # Rebuild the body region (between head and closing ')') child by
    # child, injecting :layer where needed; keep the inter-child text
    # (comments / blank lines) verbatim.
    pieces: list[str] = []
    cursor = span.body_lo
    for ch in children:
        pieces.append(text[cursor:ch.start])           # leading comments/ws
        child_text = text[ch.start:ch.end]
        if block_layer is not None and ch.head not in _DECL_HEADS:
            if _derived_layer(_top_kw_names(text, ch)) != block_layer:
                # inject ` :layer X` before the child's final ')'
                child_text = child_text[:-1] + f" :layer {block_layer})"
        pieces.append(child_text)
        cursor = ch.end
    pieces.append(text[cursor:span.body_hi])            # trailing comments/ws

    body = "".join(pieces)
    # The wrapper indented its children one level (first child col − head
    # col); dedent the whole body by that so children land at the head's
    # column as proper top-level forms.
    dedent = (_col_of(text, children[0].start) - head_col) if children else 0
    body = _dedent_block(body, dedent)
    # The text emitted before the wrapper already indents up to the head's
    # column, so the first child needs no leading indent of its own; later
    # lines keep their dedented (absolute) indent. Trim the blank lead that
    # followed `(head` and the trailing ws before the dropped ')'.
    return body.strip()


def flatten_text(text: str) -> str:
    """Return ``text`` with every top-level wrapper flattened."""
    out: list[str] = []
    cursor = 0
    for span in _top_forms(text):
        out.append(text[cursor:span.start])
        if span.head in _WRAPPERS:
            out.append(_unwrap(text, span))
        else:
            out.append(text[span.start:span.end])
        cursor = span.end
    out.append(text[cursor:])
    return "".join(out)


# ── KB-equivalence oracle ───────────────────────────────────────────


def _kb_fingerprint(kb) -> dict:
    """Order-independent signature of the loaded KB."""
    facts = sorted(
        (
            f.relation_name,
            tuple(str(a) for a in f.args),
            f.layer.name,
            f.provenance.kind if f.provenance else None,
            f.provenance.source if f.provenance else None,
            f.provenance.rule if f.provenance else None,
        )
        for f in kb.facts
    )
    relations = sorted(
        (r.name, tuple(r.signature), r.declared) for r in kb.relations.values()
    )
    rules = sorted(kb.rules.keys())
    hrules = sorted(kb.hrules.keys())
    return {
        "facts": facts, "relations": relations,
        "rules": rules, "hrules": hrules,
    }


def _load(text: str, filename: str):
    # Imported lazily so the scanner is usable without the package.
    import warnings
    repo = Path(__file__).resolve().parents[1]
    sys.path.insert(0, str(repo / "ein.py" / "src"))
    from ein.ir import parse
    from ein.kb.from_ir import load
    with warnings.catch_warnings():
        warnings.simplefilter("ignore", DeprecationWarning)
        return load(parse(text, filename=filename))


def _verify(original: str, rewritten: str, name: str) -> list[str]:
    """Return a list of human-readable KB divergences (empty == identical)."""
    a = _kb_fingerprint(_load(original, name))
    b = _kb_fingerprint(_load(rewritten, name + " (flat)"))
    diffs: list[str] = []
    for key in ("relations", "rules", "hrules", "facts"):
        if a[key] != b[key]:
            set_a, set_b = {repr(x) for x in a[key]}, {repr(x) for x in b[key]}
            only_a = sorted(set_a - set_b)[:6]
            only_b = sorted(set_b - set_a)[:6]
            diffs.append(
                f"{key}: -{len(set_a - set_b)} +{len(set_b - set_a)}"
                + ("".join(f"\n      - {x}" for x in only_a))
                + ("".join(f"\n      + {x}" for x in only_b)),
            )
    return diffs


# ── Python-test-file migration (inline ein string literals) ─────────
#
# Many tests embed ein fixtures as Python string literals. This migrates
# *only* literals that (a) parse as ein, (b) still parse after flattening,
# and (c) load to a byte-identical KB — so a docstring that merely mentions
# `(ontology …)` in prose, or any non-fixture string, is left untouched.


def _line_offsets(src: str) -> list[int]:
    offs, pos = [0], 0
    for line in src.splitlines(keepends=True):
        pos += len(line)
        offs.append(pos)
    return offs


def _reencode(raw: str, flat: str) -> str | None:
    """Re-encode flattened content as a Python string literal, preserving
    triple-quoting for multi-line fixtures. Returns None to skip (raw/f/b)."""
    pfx = raw[:len(raw) - len(raw.lstrip("rRbBfFuU"))]
    if any(c in pfx.lower() for c in "bf"):
        return None  # don't rewrite bytes / f-strings
    rest = raw[len(pfx):]
    if "\n" in flat or rest.startswith(('"""', "'''")):
        quote = '"""' if '"""' not in flat else "'''"
        if quote in flat:
            return None  # can't safely triple-quote; hand-migrate
        keep = "" if any(c in pfx for c in "rR") else pfx  # drop raw-ness if unused
        return f'{keep}{quote}{flat}{quote}'
    return repr(flat)


def migrate_py_text(src: str, name: str) -> tuple[str, int]:
    """Flatten every ein fixture string literal in Python source ``src``.

    Returns (new_src, n_migrated). Each candidate must parse as ein both
    before and after, and load to an identical KB fingerprint."""
    import ast

    from ein.ir import IRParseError, parse  # noqa: F401 (import guard)

    tree = ast.parse(src)
    offs = _line_offsets(src)
    repls: list[tuple[int, int, str]] = []
    for node in ast.walk(tree):
        if not (isinstance(node, ast.Constant) and isinstance(node.value, str)):
            continue
        value = node.value
        if "(" not in value:
            continue
        # (a) must currently parse as ein, and contain a wrapper to migrate.
        try:
            _load(value, name)
        except Exception:
            continue
        flat = flatten_text(value)
        if flat == value:
            continue
        # (b) must still parse + (c) load to an identical KB.
        try:
            if _verify(value, flat, name):
                continue
        except Exception:
            continue
        raw = ast.get_source_segment(src, node)
        if raw is None:
            continue
        new_lit = _reencode(raw, flat)
        if new_lit is None:
            continue
        start = offs[node.lineno - 1] + node.col_offset
        end = offs[node.end_lineno - 1] + node.end_col_offset
        if src[start:end] != raw:    # offset/segment mismatch — be safe
            continue
        repls.append((start, end, new_lit))

    for start, end, new_lit in sorted(repls, reverse=True):
        src = src[:start] + new_lit + src[end:]
    return src, len(repls)


# ── CLI ─────────────────────────────────────────────────────────────


def main(argv: list[str] | None = None) -> int:
    ap = argparse.ArgumentParser(
        description="Migrate wrapped .ein files to flat forms (P1.7c).",
        formatter_class=argparse.RawDescriptionHelpFormatter,
        epilog=__doc__,
    )
    ap.add_argument("files", nargs="+", type=Path, help=".ein or .py file(s)")
    mode = ap.add_mutually_exclusive_group()
    mode.add_argument("--in-place", action="store_true",
                      help="rewrite on disk (verifies KB-equivalence first)")
    mode.add_argument("--verify", action="store_true",
                      help="only check wrapped vs flat load to the same KB")
    ap.add_argument("--py", action="store_true",
                    help="treat inputs as Python test files; flatten the ein "
                         "string literals inside (per-literal KB-verified)")
    args = ap.parse_args(argv)

    # Python-test-file mode: flatten inline ein fixtures, KB-verified each.
    if args.py:
        rc = 0
        for path in args.files:
            src = path.read_text(encoding="utf-8")
            new_src, n = migrate_py_text(src, str(path))
            if n and args.in_place:
                path.write_text(new_src, encoding="utf-8")
                print(f"✓ {path}: migrated {n} fixture string(s)", file=sys.stderr)
            elif n:
                print(f"  {path}: would migrate {n} fixture string(s)",
                      file=sys.stderr)
            else:
                print(f"· {path}: no wrapped fixtures", file=sys.stderr)
        return rc

    rc = 0
    for path in args.files:
        original = path.read_text(encoding="utf-8")
        rewritten = flatten_text(original)
        changed = rewritten != original

        if args.verify or args.in_place:
            diffs = _verify(original, rewritten, str(path))
            if diffs:
                rc = 1
                print(f"✗ {path}: KB DIVERGES after flatten:", file=sys.stderr)
                for d in diffs:
                    print(f"    {d}", file=sys.stderr)
                continue
            tag = "identical KB" if changed else "already flat"
            print(f"✓ {path}: {tag}", file=sys.stderr)

        if args.in_place:
            if changed:
                path.write_text(rewritten, encoding="utf-8")
                print(f"  rewrote {path}", file=sys.stderr)
        elif not args.verify:
            sys.stdout.write(rewritten)
    return rc


if __name__ == "__main__":
    raise SystemExit(main())
