#!/usr/bin/env python3
"""
Find candidate dead definitions in the ein_bot package — a zero-dependency
companion to vulture (which needs installing; see [tool.vulture] in
ein.py/pyproject.toml).

Walks every .py under the *def-roots*, collecting top-level `def`/`class`
names (and methods with --all-kinds), then walks every .py under the
*ref-roots*, collecting every load-time Name / attribute reference plus
`__all__` exports. A definition whose name is referenced nowhere outside its
own def site is reported.

Deliberately conservative-but-noisy: it keys on *names*, so it cannot see
dynamic dispatch (string registries, getattr, the Lark Transformer methods
keyed on grammar terminals, pytest fixtures) or tell two same-named methods
apart. Treat the output as a TRIAGE list, not a delete list — confirm each
hit with `rg -w <name>` before removing (that is how `committed_hypotheses`
and `BOOKKEEPING_HEADS` were confirmed dead).

Usage:
    utils/find_dead_defs.py                 # report dead top-level defs in ein.py/src
    utils/find_dead_defs.py --all-kinds     # also report unused methods (noisier)
    utils/find_dead_defs.py --def-roots ein.py/src/ein_bot/inference
    utils/find_dead_defs.py -h
"""
from __future__ import annotations

import argparse
import ast
import fnmatch
import sys
from pathlib import Path

REPO_ROOT = Path(__file__).resolve().parent.parent
DEFAULT_DEF_ROOTS = [REPO_ROOT / "ein.py" / "src"]
DEFAULT_REF_ROOTS = [REPO_ROOT / "ein.py" / d for d in ("src", "tests", "demo")]

# Implicitly-used names — never reported (pytest entry points, CLI main,
# dunder protocol methods).
IGNORE_GLOBS = ("test_*", "Test*", "main", "__*__")


def _py_files(roots: list[Path]) -> list[Path]:
    out: list[Path] = []
    for root in roots:
        if root.is_file() and root.suffix == ".py":
            out.append(root)
        elif root.is_dir():
            out.extend(sorted(root.rglob("*.py")))
    return out


def _parse(path: Path) -> ast.Module | None:
    try:
        return ast.parse(path.read_text(encoding="utf-8"), filename=str(path))
    except (SyntaxError, UnicodeDecodeError) as exc:
        print(f"  (skipped {path}: {exc})", file=sys.stderr)
        return None


def _ignored(name: str) -> bool:
    return any(fnmatch.fnmatchcase(name, glob) for glob in IGNORE_GLOBS)


def _rel(path: Path) -> Path:
    try:
        return path.relative_to(REPO_ROOT)
    except ValueError:
        return path


def collect_defs(
    files: list[Path], *, include_methods: bool,
) -> dict[str, tuple[Path, int, str]]:
    """Map name -> (path, lineno, kind); first definition site wins for display."""
    defs: dict[str, tuple[Path, int, str]] = {}

    class Visitor(ast.NodeVisitor):
        def __init__(self) -> None:
            self.depth = 0  # 0 = module level, 1 = inside a class body, ...

        def _record(self, node: ast.AST, name: str, kind: str) -> None:
            if _ignored(name):
                return
            if self.depth == 0 or (include_methods and kind == "method"):
                defs.setdefault(name, (path, node.lineno, kind))

        def visit_ClassDef(self, node: ast.ClassDef) -> None:
            self._record(node, node.name, "class" if self.depth == 0 else "nested")
            self.depth += 1
            self.generic_visit(node)
            self.depth -= 1

        def visit_FunctionDef(self, node: ast.FunctionDef) -> None:
            kind = "func" if self.depth == 0 else ("method" if self.depth == 1 else "nested")
            self._record(node, node.name, kind)
            self.depth += 1
            self.generic_visit(node)
            self.depth -= 1

        visit_AsyncFunctionDef = visit_FunctionDef

    for path in files:
        tree = _parse(path)
        if tree is not None:
            Visitor().visit(tree)
    return defs


def collect_refs(files: list[Path]) -> set[str]:
    """Every name that appears as a load-time reference or an `__all__` export."""
    refs: set[str] = set()
    for path in files:
        tree = _parse(path)
        if tree is None:
            continue
        for node in ast.walk(tree):
            if isinstance(node, ast.Name) and isinstance(node.ctx, ast.Load):
                refs.add(node.id)
            elif isinstance(node, ast.Attribute):
                refs.add(node.attr)
            elif isinstance(node, ast.Assign) and any(
                isinstance(t, ast.Name) and t.id == "__all__" for t in node.targets
            ):
                for elt in ast.walk(node.value):
                    if isinstance(elt, ast.Constant) and isinstance(elt.value, str):
                        refs.add(elt.value)
    return refs


def main(argv: list[str] | None = None) -> int:
    parser = argparse.ArgumentParser(
        description="Find candidate dead definitions (zero-dep vulture companion).",
        formatter_class=argparse.RawDescriptionHelpFormatter,
        epilog="Output is a TRIAGE list — confirm with `rg -w <name>` before deleting.",
    )
    parser.add_argument(
        "--def-roots", nargs="+", type=Path, default=DEFAULT_DEF_ROOTS,
        help="dirs/files whose definitions are checked (default: ein.py/src)",
    )
    parser.add_argument(
        "--ref-roots", nargs="+", type=Path, default=DEFAULT_REF_ROOTS,
        help="dirs/files scanned for references (default: ein.py/{src,tests,demo})",
    )
    parser.add_argument(
        "--all-kinds", action="store_true",
        help="also report unused class methods (noisier — name-based)",
    )
    args = parser.parse_args(argv)

    def_files = _py_files(args.def_roots)
    ref_files = _py_files(args.ref_roots)
    if not def_files:
        parser.error(f"no .py files under def-roots: {args.def_roots}")

    defs = collect_defs(def_files, include_methods=args.all_kinds)
    refs = collect_refs(ref_files)

    dead = sorted(
        (
            (path, lineno, kind, name)
            for name, (path, lineno, kind) in defs.items()
            if name not in refs
        ),
        key=lambda row: (str(row[0]), row[1]),
    )

    print(
        f"{len(dead)} candidate dead definition(s) "
        f"[{len(defs)} defs across {len(def_files)} files, "
        f"cross-referenced against {len(ref_files)} files]:\n"
    )
    for path, lineno, kind, name in dead:
        print(f"  {_rel(path)}:{lineno}  {kind:6}  {name}")
    if dead:
        print(
            "\nName-based heuristic — dynamic dispatch / registries / fixtures / "
            "recursion cause\nfalse positives. Confirm each with `rg -w <name>` "
            "before deleting."
        )
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
