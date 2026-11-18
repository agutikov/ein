#!/usr/bin/env python3
"""S1f.23.5 task 1: every relation's `:why`, today and renumbered, and the trace's statements.

    python3 census.py decls                      # the declarations: counts per root and per stdlib module
    python3 census.py decls --list OUT.md        # + every declaration, today and renumbered, as a table
    python3 census.py decls --list OUT.md --root stdlib   # + the standard library's alone
    python3 census.py touch                      # what else writes a relation's `:why` (Rust strings, pages, generators)
    python3 census.py trace --bin BIN [-j 4]     # `ein solve FILE --no-search --trace`, every corpus entry: what it prints

Run from anywhere; paths are the repository's. Nothing here is built: the
*renumbered* text and the *after* reading are this script applying the
ruling (`{?k}` of a fact's k-th argument becomes `{?k+2}`, the declaration's
k-th label; `{?1}` is the atom `relation`, `{?2}` the relation's name), not a
run of the engine. The `trace` half runs the binary it is given and reads what
it prints.
"""
from __future__ import annotations

import argparse
import collections
import concurrent.futures as cf
import re
import subprocess
import sys
import tempfile
from pathlib import Path

REPO = Path(__file__).resolve().parents[4]
ROOTS = ("stdlib", "examples", "tests", "corpus")
POS = re.compile(r"\{\?([1-9][0-9]*)\}")
REF = re.compile(r"\{\?([^}\s]+)\}")


# ── reading a .ein ───────────────────────────────────────────────────────────

def tokens(text: str):
    """(kind, value, line): '(' ')' 'str' 'atom'; comments skipped."""
    i, n, line = 0, len(text), 1
    while i < n:
        c = text[i]
        if c == "\n":
            line += 1
            i += 1
        elif c.isspace():
            i += 1
        elif c == ";":
            while i < n and text[i] != "\n":
                i += 1
        elif text.startswith("#|", i):
            j = text.find("|#", i + 2)
            j = n if j < 0 else j + 2
            line += text.count("\n", i, j)
            i = j
        elif c in "()":
            yield c, c, line
            i += 1
        elif c == '"':
            j = i + 1
            while j < n and text[j] != '"':
                j += 2 if text[j] == "\\" else 1
            yield "str", text[i + 1:j], line
            line += text.count("\n", i, j)
            i = j + 1
        else:
            j = i
            while j < n and not text[j].isspace() and text[j] not in '();"':
                j += 1
            yield "atom", text[i:j], line
            i = j


def forms(text: str):
    """Top-level forms as nested lists of (kind, value, line) leaves; a list is
    (line, [items])."""
    stack: list[tuple[int, list]] = []
    out = []
    for kind, value, line in tokens(text):
        if kind == "(":
            stack.append((line, []))
        elif kind == ")":
            if not stack:
                continue
            done = stack.pop()
            (stack[-1][1] if stack else out).append(done)
        else:
            (stack[-1][1] if stack else out).append((kind, value, line))
    return out


def is_list(x) -> bool:
    return isinstance(x, tuple) and len(x) == 2 and isinstance(x[1], list)


def declarations(path: Path):
    """Every `(relation NAME LABEL… :why "…")` form, at the top or inside a
    top-level wrapper (a saved file's `(program …)`), with its depth."""
    text = path.read_text(errors="replace")

    def walk(items, depth):
        for f in items:
            if not is_list(f):
                continue
            line, xs = f
            head = xs[0] if xs else None
            if head and not is_list(head) and head[0] == "atom" and head[1] == "relation" \
                    and len(xs) > 1 and not is_list(xs[1]) and xs[1][0] == "atom" \
                    and not xs[1][1].startswith("?"):
                labels, why, k = [], None, 2
                while k < len(xs) and not is_list(xs[k]) and xs[k][0] == "atom" \
                        and not xs[k][1].startswith(":"):
                    labels.append(xs[k][1])
                    k += 1
                rest = xs[k:]
                for a, b in zip(rest, rest[1:]):
                    if not is_list(a) and a[1] == ":why" and not is_list(b) and b[0] == "str":
                        why = b[1]
                if depth == 0 or why is not None:
                    yield {"line": line, "name": xs[1][1], "labels": labels, "why": why,
                           "depth": depth}
            elif head and not is_list(head) and head[1] in ("program", "module"):
                yield from walk(xs[1:], depth + 1)

    yield from walk(forms(text), 0)


def renumber(why: str) -> str:
    return POS.sub(lambda m: "{?%d}" % (int(m.group(1)) + 2), why)


def after_reading(name: str, labels: list[str], why: str) -> str:
    arrows = ["relation", name, *labels]

    def fill(m):
        k = int(m.group(1))
        return arrows[k - 1] if k <= len(arrows) else m.group(0)
    return POS.sub(fill, renumber(why))


def classify(d) -> str:
    why = d["why"]
    if why is None:
        return "no :why"
    refs = REF.findall(why)
    pos = [r for r in refs if POS.fullmatch("{?%s}" % r)]
    if any(int(r) > len(d["labels"]) for r in pos):
        return "a {?k} past its labels today"
    if len(pos) != len(refs):
        return "a non-positional reference"
    return "renumbered" if pos else "no reference: text unchanged"


def ein_files():
    """Every tracked `.ein` file under the four corpus roots."""
    out = subprocess.run(["git", "-C", str(REPO), "ls-files", "-z", "--", *ROOTS],
                         capture_output=True, text=True, check=True).stdout
    return sorted(REPO / rel for rel in out.split("\0")
                  if rel.endswith(".ein") and (REPO / rel).is_file())


def root_of(p: Path) -> str:
    return p.relative_to(REPO).parts[0]


def module_of(p: Path) -> str:
    rel = p.relative_to(REPO / "stdlib").with_suffix("")
    return "std." + ".".join(rel.parts)


def cmd_decls(args) -> int:
    per_root = collections.defaultdict(collections.Counter)
    per_module = collections.defaultdict(collections.Counter)
    files_with = collections.Counter()
    rows = []
    for p in ein_files():
        ds = list(declarations(p))
        root = root_of(p)
        per_root[root]["files"] += 1
        if ds:
            files_with[root] += 1
        for d in ds:
            c = classify(d)
            per_root[root]["declarations"] += 1
            per_root[root][c] += 1
            if not d["labels"]:
                per_root[root]["no label"] += 1
            if root == "stdlib":
                m = module_of(p)
                per_module[m]["declarations"] += 1
                per_module[m][c] += 1
            rows.append((p.relative_to(REPO).as_posix(), d, c))
    cols = ["declarations", "renumbered", "no reference: text unchanged",
            "a non-positional reference", "a {?k} past its labels today", "no :why"]
    print("| root | files | files that declare | " + " | ".join(cols) + " |")
    print("|---|" + "---:|" * (len(cols) + 2))
    tot = collections.Counter()
    for root in ROOTS:
        c = per_root[root]
        tot.update(c)
        tot["files that declare"] += files_with[root]
        print(f"| `{root}/` | {c['files']} | {files_with[root]} | "
              + " | ".join(str(c[k]) for k in cols) + " |")
    print(f"| all | {tot['files']} | {tot['files that declare']} | "
          + " | ".join(str(tot[k]) for k in cols) + " |")
    print()
    print("| module | " + " | ".join(cols[:3]) + " |")
    print("|---|" + "---:|" * 3)
    for m in sorted(per_module):
        c = per_module[m]
        print(f"| `{m}` | " + " | ".join(str(c[k]) for k in cols[:3]) + " |")
    if args.list:
        with open(args.list, "w") as fh:
            fh.write("# Every relation declaration's `:why`, today and renumbered\n\n")
            fh.write("Written by `census.py decls --list`; *renumbered* and *reads after* are the "
                     "script applying the ruling, **by hand, not built**. *reads after* is what "
                     "the declaration `(relation R L1 … Ln)` reads as: `{?1}` the atom `relation`, "
                     "`{?2}` `R`, `{?k+2}` the label `Lk`.\n\n")
            fh.write("| file:line | relation | labels | `:why` today | renumbered | reads after | |\n")
            fh.write("|---|---|---|---|---|---|---|\n")
            for path, d, c in rows:
                why = d["why"]
                if why is None or (args.root and not path.startswith(args.root + "/")):
                    continue
                cell = lambda s: s.replace("|", "\\|")
                code = lambda s: f"`` {cell(s)} ``" if "`" in s else f"`{cell(s)}`"
                fh.write(f"| {path}:{d['line']} | `{d['name']}` | {' '.join(d['labels']) or '—'} | "
                         f"{code(why)} | {code(renumber(why)) if c == 'renumbered' else '='} | "
                         f"{cell(after_reading(d['name'], d['labels'], why))} | "
                         f"{'' if c in ('renumbered', 'no reference: text unchanged') else c} |\n")
    return 0


# ── what else writes a relation's :why ───────────────────────────────────────

WHY_STR = re.compile(r':why\s+(\\"|")')


def why_strings(text: str):
    """Every `:why` string in a file, a Rust `\\"…\\"` or a plain `"…"`."""
    for m in WHY_STR.finditer(text):
        q = m.group(1)
        start = m.end()
        if q == '"':
            j = start
            while j < len(text) and text[j] != '"':
                j += 2 if text[j] == "\\" else 1
            yield text[start:j]
        else:
            j = text.find('\\"', start)
            yield text[start:j if j >= 0 else len(text)]


def cmd_touch(args) -> int:
    def count(globs, label):
        files, n = 0, 0
        for g in globs:
            for p in sorted(REPO.glob(g)):
                if "vendor" in p.parts or "target" in p.parts:
                    continue
                k = sum(1 for s in why_strings(p.read_text(errors="replace")) if POS.search(s))
                if k:
                    files += 1
                    n += k
        print(f"| {label} | {files} | {n} |")
    print("| where | files | `:why` strings with a `{?k}` |")
    print("|---|---:|---:|")
    count(["ein.rs/crates/*/src/**/*.rs"], "`ein.rs/crates/*/src/`")
    count(["ein.rs/crates/*/tests/**/*.rs"], "`ein.rs/crates/*/tests/` (`.rs`)")
    count(["ein.rs/crates/*/tests/**/*.ein"], "`ein.rs/crates/*/tests/` (`.ein` fixtures)")
    count(["ein.rs/crates/*/tests/golden/*.txt", "ein.rs/crates/*/tests/golden/*.golden"],
          "`ein.rs/crates/*/tests/golden/` (`.txt`, `.golden`)")
    count(["docs/ein/**/*.md", "README.md"], "`docs/ein/` pages, `README.md`")
    count(["docs/decisions/**/*.md"], "`docs/decisions/`")
    count(["docs/ein/**/*.dot", "docs/ein/**/*.puml"], "`docs/ein/` diagram sources")
    gens = []
    for p in sorted(REPO.glob("utils/*.py")):
        t = p.read_text(errors="replace")
        if re.search(r"\{\{?\?[1-9]", t) and ":why" in t:
            gens.append(p.relative_to(REPO).as_posix())
    print()
    print("utils/ scripts that write a positional `{?k}` beside a `:why`: " + ", ".join(f"`{g}`" for g in gens))
    return 0


# ── root's trace ─────────────────────────────────────────────────────────────

def conclusions(line: str):
    """The (fact, statement or None) pairs of one `Derives …` line."""
    s = line[len("Derives "):]
    out = []
    i = 0
    while i < len(s) and s[i] == "`":
        j = s.find("`", i + 1)
        if j < 0:
            break
        fact = s[i + 1:j]
        i = j + 1
        stmt = None
        if s.startswith(" — ", i):
            k = s.find("; `", i)
            k = len(s) if k < 0 else k
            stmt = s[i + 3:k]
            i = k
        if s.startswith("; ", i):
            i += 2
        out.append((fact, stmt))
        if s[i:i + 1] == ".":
            break
    return out


def corpus_paths():
    text = (REPO / "corpus" / "corpus.toml").read_text()
    return re.findall(r'^path\s*=\s*"([^"]+)"', text, re.M)


def trace_one(binary: str, rel: str, scratch: str, limit: int):
    out = Path(scratch) / (rel.replace("/", "__") + ".md")
    try:
        r = subprocess.run([binary, "solve", str(REPO / rel), "--no-search", "--trace", str(out), "-G",
                            "-T", str(limit)], capture_output=True, text=True, timeout=limit + 30,
                           cwd=REPO)
        code = r.returncode
    except subprocess.TimeoutExpired:
        code = "timeout"
    row = {"path": rel, "code": code, "written": out.exists(), "steps": 0, "conclusions": 0,
           "statement": 0, "relation_statement": 0, "premises": 0, "relation_premises": 0,
           "rels": collections.Counter()}
    if out.exists():
        for line in out.read_text(errors="replace").splitlines():
            if line.startswith("### Step "):
                row["steps"] += 1
            elif line.startswith("Premises: "):
                facts = line.split("`")[1::2]
                row["premises"] += len(facts)
                row["relation_premises"] += sum(1 for f in facts if f.startswith("relation("))
            elif line.startswith("Derives `"):
                for fact, stmt in conclusions(line):
                    row["conclusions"] += 1
                    if stmt is not None:
                        row["statement"] += 1
                        rel_name = fact.split("(", 1)[0]
                        row["rels"][rel_name] += 1
                        if rel_name == "relation":
                            row["relation_statement"] += 1
        out.unlink()
    return row


def cmd_trace(args) -> int:
    paths = corpus_paths()
    rows = []
    with tempfile.TemporaryDirectory(prefix="s1f235-") as scratch:
        with cf.ThreadPoolExecutor(args.jobs) as ex:
            futs = [ex.submit(trace_one, args.bin, p, scratch, args.limit) for p in paths]
            for f in cf.as_completed(futs):
                rows.append(f.result())
    rows.sort(key=lambda r: r["path"])
    per = collections.defaultdict(collections.Counter)
    rels = collections.Counter()
    for r in rows:
        root = r["path"].split("/", 1)[0]
        c = per[root]
        c["programs"] += 1
        if r["written"]:
            c["traced"] += 1
            c["steps"] += r["steps"]
            c["conclusions"] += r["conclusions"]
            c["statement"] += r["statement"]
            c["relation_statement"] += r["relation_statement"]
            c["premises"] += r["premises"]
            c["relation_premises"] += r["relation_premises"]
            if r["statement"]:
                c["programs with a statement"] += 1
            rels.update({f"{root}: {k}": v for k, v in r["rels"].items()})
        else:
            c[f"not traced (exit {r['code']})"] += 1
    keys = ["programs", "traced", "programs with a statement", "steps", "conclusions", "statement",
            "relation_statement", "premises", "relation_premises"]
    print("| root | " + " | ".join(keys) + " | not traced |")
    print("|---|" + "---:|" * (len(keys) + 1))
    tot = collections.Counter()
    for root in sorted(per):
        c = per[root]
        tot.update(c)
        nt = ", ".join(f"{k[12:-1]}: {v}" for k, v in sorted(c.items()) if k.startswith("not traced"))
        print(f"| `{root}/` | " + " | ".join(str(c[k]) for k in keys) + f" | {nt or '—'} |")
    nt = ", ".join(f"{k[12:-1]}: {v}" for k, v in sorted(tot.items()) if k.startswith("not traced"))
    print("| all | " + " | ".join(str(tot[k]) for k in keys) + f" | {nt or '—'} |")
    print()
    print("The relations whose conclusions carry a statement most often:")
    for k, v in rels.most_common(args.top):
        print(f"  {v:8}  {k}")
    if args.rows:
        with open(args.rows, "w") as fh:
            for r in rows:
                fh.write(f"{r['path']}\t{r['code']}\t{r['written']}\t{r['steps']}\t{r['conclusions']}"
                         f"\t{r['statement']}\n")
    return 0


def main() -> int:
    ap = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    sub = ap.add_subparsers(dest="cmd", required=True)
    d = sub.add_parser("decls")
    d.add_argument("--list", default=None, metavar="OUT.md")
    d.add_argument("--root", default=None, choices=ROOTS, help="--list one root's declarations")
    sub.add_parser("touch")
    t = sub.add_parser("trace")
    t.add_argument("--bin", required=True)
    t.add_argument("-j", "--jobs", type=int, default=4)
    t.add_argument("--limit", type=int, default=60, metavar="SEC", help="-T for each run")
    t.add_argument("--top", type=int, default=15)
    t.add_argument("--rows", default=None, metavar="OUT.tsv")
    args = ap.parse_args()
    return {"decls": cmd_decls, "touch": cmd_touch, "trace": cmd_trace}[args.cmd](args)


if __name__ == "__main__":
    sys.exit(main())
