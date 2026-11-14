#!/usr/bin/env python3
"""How much of what Ein answers reads as English — M1f S1f.5.14's census.

    utils/nl_census.py                       # both halves, to stdout
    utils/nl_census.py --json census.json    # + the machine copy
    utils/nl_census.py -k zebra2             # the answer half on matching entries only
    utils/nl_census.py --flag-only           # the loader half alone
    utils/nl_census.py --check               # exit 1 while a program the loader reads lacks a :why

Two halves, and both run the engine ($EIN_BIN or --bin, default
`ein.rs/target/release/ein`):

1. **The flag** ([T1f.5.14.6](../plans/m1f_hypothesis_and_documentation/p1f.5_the_no_owner_items/s1f.5.14_nl_required.md)):
   every `.ein` under `examples/`, `tests/`, `corpus/` and `stdlib/`, per root:
   how many files the loader reads at all (`ein kb save` with `:why` not
   required), and of those how many it refuses when a `:why` is required, with
   the forms the refusals name (`relation`, `rule`, `hrule`, `macro`). A file
   that does not load either way (a fixture under `examples/broken/`, a
   witness refused at the parse) is counted apart and has no form counted: the
   loader never reads its `:why`s. Which spelling turns the requirement on and
   off is read from `ein kb save --help`, so the census runs on either side of
   T1f.5.14.7's flip (`--require-why` before it, the default and
   `--no-require-why` after).
2. **The answer**, reading (3) of the stage file's § What "convertible to NL"
   has to mean: every corpus entry with a `solve` run (the `slow` entries
   with `--slow`, the `heavy_dump` ones with `--heavy`), solved as `ein solve FILE
   --dump-answer DIR`, and every fact of every state file the run writes
   (`DIR/models/NN.ein`: the whole state, root's facts included — the fact
   set [Q-M1e.7](../docs/history/m1e_review_processing/open_questions.md#q-m1e7--the-read-out-prints-the-solution-kb-and-calls-it-a-model)
   is about, and not the read-out's listed projection). Each fact is read with
   the program the state file re-writes above it (imports inlined) and the
   provenance comment after it, and is counted three ways:

   - **statement** — its relation carries a `:why`: the fact's own sentence,
     positional (`{?1}` …);
   - **reason** — its first recorded justification has a text: a `:source`
     sentence, or the deriving rule's `:why`, filled with the firing's
     bindings;
   - **rendered** — either, which is the fallback ladder
     [`09_natural_language.md`](../docs/kernel/ir/03-ein-lang/09_natural_language.md)
     states: a fact with neither reads as its s-expression, and is a
     **fallback**.

   Each fact's relation is in one of four **registers**: `puzzle` (declared,
   or a rule the program's own sources define — an activator), `stdlib` (a
   `std.*` rule's name, or a relation a `std.*` rule asserts, where the
   program does not define it), `kernel` (a name the engine reserves,
   `ein_core::RESERVED`: `not`, `relation`, `__closed__`, …) and
   `undeclared` (written or derived, never declared, and no rule's). The
   register split of the fallbacks is the deliverable, not the total.

The headline is one number per entry and one for the corpus: the facts that
render, of the facts in its answer, with how many entries render every fact.
The census reads what the engine wrote and the program as written: its
`rendered` is what the ladder reaches, the texts a state file's comments carry
since T1f.5.14.2 ([`ein_render::sentence`](../ein.rs/crates/ein-render/src/sentence.rs)),
which `ein-cli/tests/natural_language_text.rs` holds on every kind of fact.
"""
from __future__ import annotations

import argparse
import collections
import json
import os
import re
import shutil
import subprocess
import sys
import tempfile
import tomllib
from pathlib import Path

REPO = Path(__file__).resolve().parents[1]
MANIFEST = REPO / "corpus" / "corpus.toml"
STDLIB = REPO / "stdlib"
EIN = Path(os.environ.get("EIN_BIN", REPO / "ein.rs" / "target" / "release" / "ein"))
ROOTS = ("examples", "tests", "corpus", "stdlib")
FORMS = ("relation", "rule", "hrule", "macro")

#: `ein_core::RESERVED` (ein-core/src/terms.rs) — the names the engine owns.
KERNEL = {"__closed__", "__symmetric__", "absent", "and", "eq", "false", "neq",
          "not", "open", "or", "relation"}

TOK = re.compile(r'#\|.*?\|#|;[^\n]*|"(?:\\.|[^"\\])*"|[()]|[^\s()";]+', re.S)
STATE_MARK = ";; ── the state"


# ── reading Ein, enough for declaration heads ────────────────

def forms(text: str) -> list:
    """Top-level forms; comments dropped, strings kept quoted."""
    stack = [[]]
    for t in TOK.findall(text):
        if t.startswith(";") or t.startswith("#|"):
            continue
        if t == "(":
            stack.append([])
        elif t == ")":
            if len(stack) > 1:
                x = stack.pop()
                stack[-1].append(x)
        else:
            stack[-1].append(t)
    while len(stack) > 1:
        x = stack.pop()
        stack[-1].append(x)
    return stack[0]


def kw(form: list, key: str):
    for i, x in enumerate(form):
        if x == key and i + 1 < len(form):
            return form[i + 1]
    return None


def has_why(form: list) -> bool:
    w = kw(form, ":why")
    return isinstance(w, str) and w.startswith('"') and len(w) > 2


def heads(node, out: set) -> None:
    """Constant relation heads a conclusion writes."""
    if isinstance(node, list) and node:
        h = node[0]
        if h == "and":
            for x in node[1:]:
                heads(x, out)
        elif isinstance(h, str) and not h.startswith("?"):
            out.add(h)
            if h == "not" and len(node) > 1:
                heads(node[1], out)


def program_names(text: str) -> tuple[dict, dict, set]:
    """(relation → has :why, rule → has :why, constant heads its rules assert)."""
    rels, rules, asserted = {}, {}, set()
    for f in forms(text):
        if not isinstance(f, list) or not f or not isinstance(f[0], str):
            continue
        if f[0] == "relation" and len(f) > 1 and isinstance(f[1], str):
            rels[f[1]] = has_why(f)
        elif f[0] in ("rule", "hrule") and len(f) > 1 and isinstance(f[1], str):
            rules[f[1]] = has_why(f)
            heads(kw(f, ":assert"), asserted)
    return rels, rules, asserted


def stdlib_names() -> tuple[set, set]:
    rules, asserted = set(), set()
    for p in sorted(STDLIB.rglob("*.ein")):  # every depth: `math/` since M1f S1f.6.31
        _, r, a = program_names(p.read_text())
        rules |= set(r)
        asserted |= a
    return rules, asserted


def own_sources(path: Path, seen: set | None = None) -> str:
    """The text of a file and of every corpus file it imports, not `std.*`."""
    seen = set() if seen is None else seen
    if path in seen or not path.exists():
        return ""
    seen.add(path)
    text = path.read_text(errors="replace")
    out = [text]
    for f in forms(text):
        if isinstance(f, list) and f and f[0] == "import" and len(f) > 1:
            mod = f[1]
            if isinstance(mod, str) and not mod.startswith("std.") and not mod.startswith('"'):
                out.append(own_sources(path.parent / (mod.replace(".", "/") + ".ein"), seen))
            elif isinstance(mod, str) and mod.startswith('"'):
                out.append(own_sources((path.parent / mod.strip('"')).resolve(), seen))
    return "\n".join(out)


# ── half 1: the flag ─────────────────────────────────────────

def flag_spellings(ein: Path) -> tuple[list[str], list[str]]:
    """(argv that requires a :why, argv that does not), as this binary spells them."""
    help_ = subprocess.run([str(ein), "kb", "save", "--help"], capture_output=True, text=True).stdout
    if "--no-require-why" in help_:
        return [], ["--no-require-why"]
    return ["--require-why"], []


REFUSAL = re.compile(r"(relation|rule|hrule|macro) '([^']*)' has no :why")


def flag_half(ein: Path, scratch: Path) -> dict:
    on, off = flag_spellings(ein)
    out = {}
    for root in ROOTS:
        # A file: W74's module is a directory named `*.ein` (M1f S1f.5.22).
        files = sorted(p for p in (REPO / root).rglob("*.ein") if p.is_file())
        row = {"files": len(files), "load": 0, "refused": 0, "not_loaded": 0,
               "forms": collections.Counter(), "refused_files": []}
        for p in files:
            rel = p.relative_to(REPO).as_posix()
            base = subprocess.run([str(ein), "kb", "save", *off, rel, str(scratch / "x.einb")],
                                  cwd=REPO, capture_output=True, text=True)
            if base.returncode != 0:
                row["not_loaded"] += 1
                continue
            row["load"] += 1
            req = subprocess.run([str(ein), "kb", "save", *on, rel, str(scratch / "x.einb")],
                                 cwd=REPO, capture_output=True, text=True)
            if req.returncode != 0:
                row["refused"] += 1
                row["refused_files"].append(rel)
                for m in REFUSAL.finditer(req.stderr):
                    row["forms"][m.group(1)] += 1
        row["forms"] = dict(row["forms"])
        out[root] = row
    return out


# ── half 2: the answer ───────────────────────────────────────

COMMENT = re.compile(r'^(\(.*?\))\s*(?:;\s?(.*))?$')


def prov_of(comment: str | None) -> tuple[str, str | None]:
    """The provenance a state file's comment opens with: (kind, rule)."""
    if not comment:
        return "given", None
    if comment.startswith("source "):
        return "source", None
    if comment.startswith("hypothesis "):
        return "hypothesis", None
    m = re.match(r"rule ([^\s:]+)", comment)
    if m:
        return "rule", m.group(1)
    return "given", None


def fact_relation(text: str) -> str:
    f = forms(text)
    return f[0][0] if f and isinstance(f[0], list) and f[0] else "?"


def state_facts(text: str):
    """(program text, [(fact text, comment)]) of one state file."""
    i = text.find(STATE_MARK)
    if i < 0:
        return text, []
    program, rest = text[:i], text[i:]
    facts = []
    for line in rest.splitlines()[1:]:
        if not line.startswith("("):
            continue
        # the fact ends where its parentheses balance
        depth, j, inq = 0, 0, False
        while j < len(line):
            c = line[j]
            if inq:
                if c == "\\":
                    j += 1
                elif c == '"':
                    inq = False
            elif c == '"':
                inq = True
            elif c == "(":
                depth += 1
            elif c == ")":
                depth -= 1
                if depth == 0:
                    break
            j += 1
        fact, tail = line[:j + 1], line[j + 1:].strip()
        facts.append((fact, tail[1:].strip() if tail.startswith(";") else None))
    return program, facts


def register(rel: str, own_rels: set, own_rules: set, own_asserted: set,
             std_rules: set, std_asserted: set) -> str:
    if rel in KERNEL:
        return "kernel"
    if rel in own_rels or rel in own_rules:
        return "puzzle"
    if rel in std_rules or rel in std_asserted:
        return "stdlib"
    return "undeclared"


def answer_entries(args) -> list[dict]:
    entries = tomllib.loads(MANIFEST.read_text())["entry"]
    out = []
    for e in entries:
        if not any(r.split()[0] == "solve" for r in e.get("runs", [])):
            continue
        if (e.get("slow") and not args.slow) or (e.get("heavy_dump") and not args.heavy):
            continue
        if args.only and args.only not in e["path"]:
            continue
        out.append(e)
    return out


def answer_half(ein: Path, scratch: Path, args) -> dict:
    std_rules, std_asserted = stdlib_names()
    rows = {}
    for e in answer_entries(args):
        path = e["path"]
        d = scratch / "answer"
        shutil.rmtree(d, ignore_errors=True)
        try:
            r = subprocess.run([str(ein), "solve", path, "--dump-answer", str(d)], cwd=REPO,
                               capture_output=True, text=True, timeout=args.timeout)
        except subprocess.TimeoutExpired:
            rows[path] = {"skipped": "timeout"}
            continue
        models = sorted((d / "models").glob("*.ein")) if (d / "models").is_dir() else []
        if r.returncode != 0 or not models:
            rows[path] = {"skipped": f"exit {r.returncode}" if r.returncode else "no state file"}
            continue
        own = own_sources(REPO / path)
        own_rels, own_rules, own_asserted = program_names(own)
        row = {"states": len(models), "facts": 0, "statement": 0, "reason": 0, "rendered": 0,
               "fallback": collections.Counter(), "by_kind": collections.Counter(),
               "relations": collections.Counter(), "fallback_relations": collections.Counter()}
        for m in models:
            program, facts = state_facts(m.read_text())
            rel_why, rule_why, _ = program_names(program)
            for fact, comment in facts:
                rel = fact_relation(fact)
                kind, rule = prov_of(comment)
                statement = rel_why.get(rel, False)
                reason = kind == "source" or (kind == "rule" and rule_why.get(rule, False))
                reg = register(rel, set(own_rels), set(own_rules), own_asserted,
                               std_rules, std_asserted)
                row["facts"] += 1
                row["statement"] += statement
                row["reason"] += bool(reason)
                row["by_kind"][kind] += 1
                row["relations"][reg] += 1
                if statement or reason:
                    row["rendered"] += 1
                else:
                    row["fallback"][reg] += 1
                    row["fallback_relations"][f"{reg}:{rel}"] += 1
        for k in ("fallback", "by_kind", "relations", "fallback_relations"):
            row[k] = dict(row[k])
        rows[path] = row
    return rows


# ── the report ───────────────────────────────────────────────

def pct(a: int, b: int) -> str:
    return f"{100 * a / b:5.1f} %" if b else "    —  "


def report(flag: dict | None, answer: dict | None) -> None:
    if flag is not None:
        print("## The flag — files the loader reads, and how many it refuses when a :why is required\n")
        print(f"{'root':10} {'files':>6} {'load':>6} {'refused':>8} {'not loaded':>11}  forms the refusals name")
        tot = collections.Counter()
        forms_tot = collections.Counter()
        for root, r in flag.items():
            print(f"{root:10} {r['files']:6} {r['load']:6} {r['refused']:8} {r['not_loaded']:11}  "
                  + (", ".join(f"{n} {k}" for k, n in sorted(r["forms"].items())) or "—"))
            for k in ("files", "load", "refused", "not_loaded"):
                tot[k] += r[k]
            forms_tot.update(r["forms"])
        print(f"{'all':10} {tot['files']:6} {tot['load']:6} {tot['refused']:8} {tot['not_loaded']:11}  "
              + (", ".join(f"{n} {k}" for k, n in sorted(forms_tot.items())) or "—"))
        print()
    if answer is not None:
        print("## The answer — every fact of every state `ein solve FILE --dump-answer DIR` writes\n")
        print(f"{'entry':60} {'states':>6} {'facts':>7} {'statement':>10} {'reason':>8} {'rendered':>9}")
        tot = collections.Counter()
        fb = collections.Counter()
        regs = collections.Counter()
        fbrel = collections.Counter()
        skipped = []
        for path, r in answer.items():
            if "skipped" in r:
                skipped.append((path, r["skipped"]))
                continue
            print(f"{path:60} {r['states']:6} {r['facts']:7} {pct(r['statement'], r['facts']):>10} "
                  f"{pct(r['reason'], r['facts']):>8} {pct(r['rendered'], r['facts']):>9}")
            for k in ("states", "facts", "statement", "reason", "rendered"):
                tot[k] += r[k]
            fb.update(r["fallback"])
            regs.update(r["relations"])
            fbrel.update(r["fallback_relations"])
        print(f"{'corpus':60} {tot['states']:6} {tot['facts']:7} {pct(tot['statement'], tot['facts']):>10} "
              f"{pct(tot['reason'], tot['facts']):>8} {pct(tot['rendered'], tot['facts']):>9}")
        # Fact-weighted, the corpus row leans on the entries with the most
        # states; the per-entry view does not.
        answered = [r for r in answer.values() if "skipped" not in r and r["facts"]]
        if answered:
            full = sum(r["rendered"] == r["facts"] for r in answered)
            mean = sum(r["rendered"] / r["facts"] for r in answered) / len(answered)
            print(f"\nper entry: {full} of {len(answered)} entries render every fact; "
                  f"the mean entry renders {100 * mean:.1f} % of its facts")
        print(f"\n{len(answer) - len(skipped)} entries answered, {len(skipped)} with no state file "
              f"({', '.join(sorted(set(s for _, s in skipped)))})" if skipped else "")
        print("\n### Facts by register, and the fallbacks in each\n")
        print(f"{'register':12} {'facts':>8} {'fallback':>9}")
        for reg in ("puzzle", "stdlib", "kernel", "undeclared"):
            print(f"{reg:12} {regs[reg]:8} {fb[reg]:9}")
        print("\n### The relations that fall back most often\n")
        for k, n in fbrel.most_common(15):
            print(f"  {n:7}  {k}")


def main() -> int:
    ap = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    ap.add_argument("--bin", type=Path, default=EIN, help="the ein binary ($EIN_BIN)")
    ap.add_argument("-k", "--only", default=None, metavar="SUBSTR",
                    help="the answer half on entries whose path contains SUBSTR")
    ap.add_argument("--slow", action="store_true", help="include the slow entries")
    ap.add_argument("--heavy", action="store_true",
                    help="include the heavy_dump entries, whose dumps write 40 000 state files or more")
    ap.add_argument("--flag-only", action="store_true", help="the loader half alone")
    ap.add_argument("--answer-only", action="store_true", help="the answer half alone")
    ap.add_argument("--timeout", type=float, default=300.0, metavar="SEC")
    ap.add_argument("--json", type=Path, default=None, metavar="FILE")
    ap.add_argument("--check", action="store_true",
                    help="exit 1 while the loader refuses a program it reads for a missing :why")
    args = ap.parse_args()
    if not args.bin.exists():
        print(f"nl_census: no binary at {args.bin} (build it, or set $EIN_BIN / --bin)", file=sys.stderr)
        return 2
    with tempfile.TemporaryDirectory(prefix="nl_census-") as tmp:
        scratch = Path(tmp)
        flag = None if args.answer_only else flag_half(args.bin, scratch)
        answer = None if (args.flag_only or args.check) else answer_half(args.bin, scratch, args)
    report(flag, answer)
    if args.json:
        args.json.write_text(json.dumps({"flag": flag, "answer": answer}, indent=1, sort_keys=True) + "\n")
    if args.check and flag is not None:
        refused = sum(r["refused"] for r in flag.values())
        if refused:
            print(f"\nnl_census --check: {refused} programs the loader reads lack a :why", file=sys.stderr)
            return 1
    return 0


if __name__ == "__main__":
    sys.exit(main())
