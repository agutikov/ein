#!/usr/bin/env python3
"""The builder of the standard library's usage pages, as the lane that wrote them left it (L-64 of ../findings.md).

Kept for what its directives mean: `directives.txt` beside it lists, per page and in order, the directive
behind each program and output block, and `ex/` holds the programs. **It does not run as it stands**: the
templates it read (`pages/*.md`, the pages' prose with a directive where a block is) are not kept, since the
pages were edited after they were built. A generator would read the blocks off the pages and take the commands
and filters from `directives.txt`.

pages/*.md (templates) → docs/ein/user/04-reference/04-stdlib/*.md.

Directives, each alone on a line:

  @@ein FILE [body]            the program as a lisp block. FILE is under c2/ex/, or `repo:PATH`.
                               `body` drops a leading run of comment lines and the blank lines after it.
  @@run ARGS… [:: FILTER]…     runs `$EIN ARGS…` with cwd c2/ex (or the repo for a `repo:` path) and
                               pastes stdout (+stderr) as an output block.
                               Filters: from=RE  to=RE  keep=RE  head=N  tail=N  cut=RE (drop matching lines)
                               exit=N asserts the exit code (default 0).
  @@rule MODULE NAME           a rule's subsection: heading and an empty region, which
                               utils/gen_stdlib_rule_pages.py fills.
  @@rules MODULE A B C …       several, in that order.
"""
import re, subprocess, sys, os
from pathlib import Path

HERE = Path(__file__).resolve().parent
REPO = HERE.parents[2]
OUT = REPO / "docs/ein/user/04-reference/04-stdlib"
EIN = os.environ.get("EIN_BIN", str(REPO / "ein.rs/target/release/ein"))
EX = HERE / "ex"

def resolve(tok):
    if tok.startswith("repo:"):
        return REPO / tok[5:]
    return EX / tok

def ein_block(args):
    path = resolve(args[0])
    text = path.read_text().rstrip("\n").splitlines()
    if "body" in args[1:]:
        i = 0
        while i < len(text) and (text[i].startswith(";") or not text[i].strip()):
            i += 1
        text = text[i:]
    return ["```lisp", *text, "```"]

def run_block(spec, where):
    parts = [p.strip() for p in spec.split(" :: ")]
    args = parts[0].split()
    filters = parts[1:]
    cwd = EX
    argv = []
    for a in args:
        if a.startswith("repo:"):
            cwd = REPO
            argv.append(a[5:])
        else:
            argv.append(a)
    want_exit = 0
    for f in filters:
        if f.startswith("exit="):
            want_exit = int(f[5:])
    r = subprocess.run([EIN, *argv], cwd=cwd, capture_output=True, text=True)
    if r.returncode != want_exit:
        sys.exit(f"{where}: `ein {' '.join(argv)}` exits {r.returncode}, wanted {want_exit}\n{r.stdout[-2000:]}\n{r.stderr[-2000:]}")
    out = (r.stdout + r.stderr).rstrip("\n").splitlines()
    for f in filters:
        k, _, v = f.partition("=")
        if k == "from":
            idx = next((i for i, l in enumerate(out) if re.search(v, l)), None)
            if idx is None: sys.exit(f"{where}: from={v} matches nothing in\n" + "\n".join(out))
            out = out[idx:]
        elif k == "to":
            idx = next((i for i, l in enumerate(out) if re.search(v, l)), None)
            if idx is None: sys.exit(f"{where}: to={v} matches nothing")
            out = out[:idx]
        elif k == "keep":
            kept, gap, any_ = [], False, False
            for l in out:
                if re.search(v, l):
                    if gap and any_: kept.append("  …")
                    kept.append(l); gap = False; any_ = True
                else:
                    gap = True
            if not kept: sys.exit(f"{where}: keep={v} matches nothing in\n" + "\n".join(out))
            out = kept
        elif k == "drop":
            r1, r2 = v.split("~")
            i1 = next((i for i, l in enumerate(out) if re.search(r1, l)), None)
            if i1 is None: sys.exit(f"{where}: drop start {r1} matches nothing")
            i2 = next((i for i, l in enumerate(out) if i > i1 and re.search(r2, l)), len(out))
            out = out[:i1] + out[i2:]
        elif k == "cut":
            out = [l for l in out if not re.search(v, l)]
        elif k == "head":
            out = out[: int(v)]
        elif k == "tail":
            out = out[-int(v):]
        elif k == "exit":
            pass
        else:
            sys.exit(f"{where}: unknown filter {f}")
    while out and not out[-1].strip(): out.pop()
    while out and not out[0].strip(): out.pop(0)
    return ["<!-- output -->", "```text", *out, "```"]

def top_form(text, kind, name):
    i, n, depth, start = 0, len(text), 0, None
    while i < n:
        c = text[i]
        if c == ";":
            while i < n and text[i] != "\n": i += 1
            continue
        if c == '"':
            i += 1
            while i < n and text[i] != '"': i += 2 if text[i] == "\\" else 1
        elif c == "(":
            if depth == 0: start = i
            depth += 1
        elif c == ")":
            depth -= 1
            if depth == 0:
                form = text[start:i + 1]
                if re.match(r"\(" + re.escape(kind) + r"\s+" + re.escape(name) + r"[\s)]", form): return form
        i += 1
    return None

def rule_block(mod, name):
    return [f"#### `{name}`", "", f"<!-- rule std.{mod} {name} -->", "<!-- /rule -->"]

def build(tpl: Path):
    out = []
    fence = None
    for n, line in enumerate(tpl.read_text().splitlines(), 1):
        where = f"{tpl.name}:{n}"
        if line.startswith("@@ein "):
            out += ein_block(line.split()[1:])
        elif line.startswith("@@run "):
            out += run_block(line[6:], where)
        elif line.startswith("@@form "):
            _, path, kind, name = line.split()
            text = resolve(path).read_text()
            form = top_form(text, kind, name)
            if form is None: sys.exit(f"{where}: no ({kind} {name} in {path}")
            out += ["```lisp", *form.splitlines(), "```"]
        elif line.startswith("@@dot "):
            blk = run_block(line[6:], where)
            out += ["```dot", *blk[2:]]
        elif line.startswith("@@rules "):
            toks = line.split()
            for i, name in enumerate(toks[2:]):
                if i: out.append("")
                out += rule_block(toks[1], name)
        elif line.startswith("@@rule "):
            head, _, sentence = line.partition(" :: ")
            toks = head.split()
            blk = rule_block(toks[1], toks[2])
            if sentence.strip():
                import textwrap
                blk = [blk[0], "", *textwrap.wrap(sentence.strip(), 116, break_long_words=False, break_on_hyphens=False), *blk[1:]]
            out += blk
        elif line.startswith("@@"):
            sys.exit(f"{where}: unknown directive {line}")
        else:
            out.append(line)
    (OUT / tpl.name).write_text("\n".join(out).rstrip("\n") + "\n")

if __name__ == "__main__":
    OUT.mkdir(parents=True, exist_ok=True)
    names = sys.argv[1:] or sorted(p.name for p in (HERE / "pages").glob("*.md"))
    for name in names:
        build(HERE / "pages" / name)
        print("built", name)
