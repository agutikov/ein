#!/usr/bin/env python3
"""S1f.23.7 task 1: the constraint view's facts against the user's slice, every corpus program.

    python3 census.py --bin BIN [-j 6] [--out DIR] [--only PATH …]

For every `path` of `corpus/corpus.toml`:

1. `ein render constraints FILE`: does the program load (the view reads it
   through the loader first)? Its DOT is read back into facts (`eintext.read_dot`).
2. **V, the view's selection**, computed from the file as written, the way
   `ein-render/src/constraints.rs` computes it: every top-level form headed by
   a name that is none of the eight declarators, with no `:source`, `:rule` or
   `:using`, whose head the file's own `(relation …)` forms do not declare.
   The DOT is the check: every fact the view draws that nothing drawn points at
   is in V, and every fact of V is drawn. A program where they differ is listed.
3. `ein saturate --dump FILE`: every fact root holds once saturated, by origin.
4. **What a relation is**, three readings:
   (a) an atom under the first arrow of a fact of root's KB, held or under an arrow;
   (b) a declared relation, from the dump's declared list;
   (c) a relation of the program, the loader's own list (Q-M1f.119): (a), (b)
       and the relation table, and every other atom an argument of a held fact
       holds that the loader takes for a relation of the program. The last part
       is asked of the binary: a copy of the program with one
       `(query :goal (ATOM ?v))` per such atom, loaded by
       `ein render constraints`; the loader refuses each goal whose relation is no
       relation of the program, one clause each, and the others are relations.
5. **U, the user's slice**: every fact root holds with a relation among its
   arguments, under each reading, directly (an atom under the fact's arrow
   2, 3, …) and at any depth (under an argument arrow of a fact under its arrows too).

Writes `census.md` (the tables) and `census.tsv` (one row per program) to
`--out`, the script's folder by default. Programs are run `-j` at a time,
each command with a timeout.
"""
from __future__ import annotations

import argparse
import collections
import concurrent.futures as cf
import re
import shutil
import subprocess
import sys
import tempfile
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
import eintext as et  # noqa: E402

REPO = Path(__file__).resolve().parents[4]
DECLARATORS = {"rule", "hrule", "macro", "query", "solve", "config", "import", "trace"}
# The words the grammar never reads as a name, and the loader's reserved ones:
# no probe is written with one as its relation.
NOT_PROBED = DECLARATORS | {
    "not", "and", "or", "neq", "eq", "open", "absent", "false", "relation", "__closed__",
}
NAME = re.compile(r"^[A-Za-z0-9_.\-$%^*+=<>!&@~/'`\[\]{}]+$")
REFUSED = re.compile(r"the program names no relation (\S+) —")


def corpus_paths() -> list[str]:
    text = (REPO / "corpus" / "corpus.toml").read_text()
    return re.findall(r'^path\s*=\s*"([^"]+)"', text, re.M)


def run(argv: list[str], timeout: int) -> tuple[int | None, str, str]:
    try:
        p = subprocess.run(argv, capture_output=True, text=True, timeout=timeout, cwd=REPO)
        return p.returncode, p.stdout, p.stderr
    except subprocess.TimeoutExpired:
        return None, "", "timeout"


# ── the view's selection, as constraints.rs makes it ─────────────────────────


def view_selection(forms: list) -> tuple[list[list], list[list]]:
    """(V, W): the view's facts, and every top-level fact the file writes."""
    written, unannotated = [], []
    for f in forms:
        if not (isinstance(f, list) and f and isinstance(f[0], str)):
            continue
        head = f[0]
        if isinstance(head, et.Str) or head.startswith(("?", ":")) or head in DECLARATORS:
            continue
        written.append(f)
        if not (et.keywords(f) & {":source", ":rule", ":using"}):
            unannotated.append(f)
    declared = {
        f[1]
        for f in unannotated
        if f[0] == "relation" and len(f) > 1 and isinstance(f[1], str) and not et.is_kw(f[1])
    }
    view = [f for f in unannotated if f[0] not in declared]
    return view, written


# ── one program ──────────────────────────────────────────────────────────────


def probe_relations(bin_: str, path: str, atoms: list[str], scratch: Path) -> tuple[set[str] | None, str]:
    """Which of `atoms` the loader takes for relations of the program."""
    if not atoms:
        return set(), ""
    src = REPO / path
    copy_dir = scratch / "mirror" / Path(path).parent
    copy_dir.mkdir(parents=True, exist_ok=True)
    probe = copy_dir / (src.stem + ".probe.ein")
    lines = [src.read_text(), ""]
    for i, a in enumerate(atoms):
        lines.append(f"(query :goal ({a} ?probe{i}))")
    probe.write_text("\n".join(lines) + "\n")
    rc, _, err = run([bin_, "render", "constraints", str(probe)], 120)
    if rc == 0:
        return set(atoms), ""
    refused = set(REFUSED.findall(err))
    # every clause of the load error must be one of the probes' refusals
    clauses = [c for c in err.strip().removeprefix("kb load error: ").split("; ") if c]
    if rc is None or len(clauses) != len(refused) or not refused <= set(atoms):
        return None, err.strip().splitlines()[0][:300] if err.strip() else f"rc={rc}"
    return set(atoms) - refused, ""


def one(bin_: str, path: str, scratch: Path) -> dict:
    row: dict = {"path": path}
    rc, out, err = run([bin_, "render", "constraints", path], 120)
    if rc != 0:
        row["status"] = "refused" if rc is not None else "timeout"
        row["why"] = (err.strip().splitlines() or [""])[0][:200]
        return row
    dot = et.read_dot(out)
    text = (REPO / path).read_text()
    forms = et.parse(text)
    # a saved file (Q-M1f.392) is one (program …): the view reads the program in it
    if forms and isinstance(forms[0], list) and forms[0][:1] == ["program"]:
        forms = forms[0][1:]
    row["calc"] = "(import std.math" in text
    view, written = view_selection(forms)
    vt = {et.canon(f) for f in view}
    drawn = set(dot.text.values())
    top = {dot.text[n] for n in dot.arrows if n not in dot.pointed_at()}
    row["view_check"] = "ok" if (top <= vt <= drawn) else f"top-not-V={sorted(top - vt)[:3]} V-not-drawn={sorted(vt - drawn)[:3]}"

    rc, out, err = run([bin_, "saturate", "--dump", path], 300)
    if rc is None:
        row["status"] = "saturate-timeout"
        return row
    d = et.read_dump(out)
    if not d.complete:
        row["status"] = "no-dump"
        row["why"] = (err.strip().splitlines() or [""])[0][:200]
        return row
    row["status"] = "ok"
    held = {t: form for t, (_, form) in d.facts.items()}
    origin = {t: o for t, (o, _) in d.facts.items()}

    rel_a: set[str] = set()
    for form in held.values():
        for node in et.fact_nodes(form):
            h = et.positional(node)[0]
            if isinstance(h, str):
                rel_a.add(h)
    rel_b = set(d.declared)
    known = rel_a | rel_b | set(d.table) | {"eq", "neq"}
    arg_atoms = sorted({a for form in held.values() for a in et.deep_args(form)} - known)
    candidates = [a for a in arg_atoms if a not in NOT_PROBED and NAME.match(a)]
    accepted, probe_err = probe_relations(bin_, path, candidates, scratch)
    row["probe"] = "ok" if accepted is not None else f"failed: {probe_err}"
    rel_c = known | (accepted or set())

    def select(rel: set[str], deep: bool) -> set[str]:
        args = et.deep_args if deep else et.direct_args
        return {t for t, form in held.items() if any(a in rel for a in args(form))}

    u = {
        "a": select(rel_a, False),
        "b": select(rel_b, False),
        "c": select(rel_c, False),
        "c_deep": select(rel_c, True),
    }
    # facts under an arrow, not held, with a relation of the program under their own arrows
    nested_only = set()
    for form in held.values():
        for node in list(et.fact_nodes(form))[1:]:
            t = et.canon(node)
            if t not in held and any(a in rel_c for a in et.direct_args(node)):
                nested_only.add(t)

    wt = {}
    for f in written:
        wt.setdefault(et.canon(f), f)
    main_declared = {f[1] for f in written if f[0] == "relation" and len(f) > 1}

    kinds: list[tuple[str, str]] = []
    uc = u["c"]
    for t in sorted(vt - uc):
        f = wt.get(t)
        if t not in held:
            kinds.append(("V only: written, not a fact root holds", t))
        elif t.startswith("(not "):
            kinds.append(("V only: a stored negation the file writes", t))
        elif t in u["c_deep"]:
            kinds.append(("V only: a relation only inside a fact under an arrow", t))
        elif f is not None and f[0] in rel_b:
            kinds.append(("V only: a fact of a relation an imported module declares, about no relation", t))
        else:
            kinds.append(("V only: a fact about no relation, of an undeclared relation", t))
    for t in sorted(uc - vt):
        f = wt.get(t)
        form = held[t]
        if origin[t] == "DERIVED":
            kinds.append(("U only: derived at root", t))
        elif f is None and form[0] == "relation" and len(et.positional(form)) == 2 and form[1] in rel_b:
            kinds.append(("U only: the membership fact (relation R) the loader adds", t))
        elif f is None:
            kinds.append(("U only: stated outside the file, by an import or a macro", t))
        elif ":source" in et.keywords(f):
            kinds.append(("U only: written with a :source", t))
        elif et.keywords(f) & {":rule", ":using"}:
            kinds.append(("U only: written with a :rule", t))
        elif f[0] in main_declared:
            kinds.append(("U only: a fact of a declared relation, about a relation", t))
        else:
            kinds.append(("U only: other", t))
    selectors = collections.Counter()
    for t in uc:
        for a in set(et.direct_args(held[t])) & rel_c:
            selectors[a] += 1
    row.update(
        selectors=selectors,
        V=len(vt),
        Ua=len(u["a"]),
        Ub=len(u["b"]),
        Uc=len(uc),
        Uc_deep=len(u["c_deep"]),
        nested_only=len(nested_only),
        held=len(held),
        agree=vt == uc,
        agree_written=vt == (uc & set(wt)),
        empty=not vt and not uc,
        kinds=kinds,
        a_vs_c=sorted(uc - u["a"])[:2],
        b_vs_c=sorted(uc - u["b"])[:2],
        uc_minus_ua=len(uc - u["a"]),
        uc_minus_ub=len(uc - u["b"]),
    )
    return row


# ── the tables ───────────────────────────────────────────────────────────────


def report(rows: list[dict], bin_: str) -> str:
    ok = [r for r in rows if r["status"] == "ok"]
    out = [
        "# S1f.23.7 task 1 — the census",
        "",
        f"Written by `census.py --bin {bin_}` over the {len(rows)} paths of `corpus/corpus.toml`. "
        "V is the constraint view's selection, U the user's slice of root's saturated KB; "
        "(a), (b), (c) the three readings of *a relation*, *direct* an atom under the fact's own arrows "
        "and *deep* one under a fact under them too.",
        "",
        "## Which programs",
        "",
        "| status | programs |",
        "|---|---:|",
    ]
    by = collections.Counter(r["status"] for r in rows)
    for k, v in sorted(by.items(), key=lambda kv: -kv[1]):
        out.append(f"| {k} | {v} |")
    bad_view = [r for r in ok if r["view_check"] != "ok"]
    bad_probe = [r for r in ok if r["probe"] != "ok"]
    out += [
        "",
        f"The view's selection read from the file agrees with its DOT on {len(ok) - len(bad_view)} of {len(ok)} "
        f"programs; the probe of (c) ran clean on {len(ok) - len(bad_probe)} of {len(ok)}.",
        "",
    ]
    for r in bad_view:
        out.append(f"- view check: `{r['path']}`: {r['view_check']}")
    for r in bad_probe:
        out.append(f"- probe: `{r['path']}`: {r['probe']}")
    s = lambda k: sum(r[k] for r in ok)  # noqa: E731
    out += [
        "",
        "## How many facts each selects",
        "",
        f"Over the {len(ok)} programs that load and saturate, root's KBs hold {s('held')} facts.",
        "",
        "| selection | facts | programs where it is not empty |",
        "|---|---:|---:|",
        f"| V, the view | {s('V')} | {sum(1 for r in ok if r['V'])} |",
        f"| U (a), direct: a relation is a head in root's KB | {s('Ua')} | {sum(1 for r in ok if r['Ua'])} |",
        f"| U (b), direct: a relation is a declared one | {s('Ub')} | {sum(1 for r in ok if r['Ub'])} |",
        f"| **U (c), direct: a relation of the program** | **{s('Uc')}** | {sum(1 for r in ok if r['Uc'])} |",
        f"| U (c), deep | {s('Uc_deep')} | {sum(1 for r in ok if r['Uc_deep'])} |",
        f"| facts under an arrow only, not held, with a relation of the program under their own arrows | {s('nested_only')} | {sum(1 for r in ok if r['nested_only'])} |",
        "",
        f"U (c) holds {s('uc_minus_ua')} facts U (a) does not, on {sum(1 for r in ok if r['uc_minus_ua'])} programs, "
        f"and {s('uc_minus_ub')} facts U (b) does not, on {sum(1 for r in ok if r['uc_minus_ub'])} programs.",
        "",
    ]
    ex_a = next((r for r in ok if r["a_vs_c"]), None)
    ex_b = next((r for r in ok if r["b_vs_c"]), None)
    if ex_a:
        out.append(f"- in (c), not (a): `{ex_a['path']}`: `{ex_a['a_vs_c'][0]}`")
    if ex_b:
        out.append(f"- in (c), not (b): `{ex_b['path']}`: `{ex_b['b_vs_c'][0]}`")
    agree = [r for r in ok if r["agree"]]
    calc = [r for r in ok if r["calc"]]
    rest = [r for r in ok if not r["calc"]]
    sm = lambda rows, k: sum(r[k] for r in rows)  # noqa: E731
    out += [
        "",
        "The calculator's programs (those that import a `std.math` module) against the rest:",
        "",
        "| programs | how many | root's facts | V | U (c), direct | V = U (c) | V = U (c) over the written facts |",
        "|---|---:|---:|---:|---:|---:|---:|",
    ]
    for name, rows_ in (("import `std.math`", calc), ("the rest", rest)):
        out.append(
            f"| {name} | {len(rows_)} | {sm(rows_, 'held')} | {sm(rows_, 'V')} | {sm(rows_, 'Uc')} | "
            f"{sum(1 for r in rows_ if r['agree'])} | {sum(1 for r in rows_ if r['agree_written'])} |"
        )
    out += [
        "",
        "## Where V and U (c) agree",
        "",
        f"- **exactly**, as sets of facts: {len(agree)} of {len(ok)} programs, "
        f"{sum(1 for r in agree if r['empty'])} of them with both empty;",
        f"- **over the facts the file writes** (U cut to the top-level facts of the file, as the view reads it): "
        f"{sum(1 for r in ok if r['agree_written'])} of {len(ok)}.",
        "",
        "## The kinds of difference",
        "",
        "| kind | facts | programs | one example |",
        "|---|---:|---:|---|",
    ]
    facts = collections.Counter()
    progs: dict[str, set] = collections.defaultdict(set)
    example: dict[str, str] = {}
    for r in ok:
        for k, t in r["kinds"]:
            facts[k] += 1
            progs[k].add(r["path"])
            example.setdefault(k, f"`{r['path']}`: `{t if len(t) < 120 else t[:117] + '…'}`")
    for k in sorted(facts, key=lambda k: (not k.startswith("V"), -facts[k])):
        out.append(f"| {k} | {facts[k]} | {len(progs[k])} | {example[k]} |")
    out += ["", "Every fact of the kind *written, not a fact root holds*:", ""]
    for r in ok:
        for k, t in r["kinds"]:
            if k.startswith("V only: written, not"):
                out.append(f"- `{r['path']}`: `{t}`")
    sel = collections.Counter()
    sel_progs: dict[str, set] = collections.defaultdict(set)
    for r in ok:
        for a, n in r["selectors"].items():
            sel[a] += n
            sel_progs[a].add(r["path"])
    out += [
        "",
        "## The atoms that select",
        "",
        "Each fact of U (c) is selected by the relations under its own arrows; the twenty-five atoms that select "
        "the most facts, over all programs (a fact with two counts for each).",
        "",
        "| atom | facts it selects | programs |",
        "|---|---:|---:|",
    ]
    for a, n in sel.most_common(25):
        out.append(f"| `{a}` | {n} | {len(sel_progs[a])} |")
    out.append("")
    return "\n".join(out)


def main() -> int:
    ap = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    ap.add_argument("--bin", required=True)
    ap.add_argument("-j", type=int, default=6)
    ap.add_argument("--out", default=str(Path(__file__).resolve().parent))
    ap.add_argument("--only", nargs="*")
    args = ap.parse_args()
    bin_ = str(Path(args.bin).resolve())
    paths = args.only or corpus_paths()
    with tempfile.TemporaryDirectory() as tmp:
        scratch = Path(tmp)
        # local imports are paths from the program's directory: the probe copies
        # sit beside a copy of every corpus root
        for root in ("examples", "tests", "stdlib", "corpus"):
            if (REPO / root).is_dir():
                shutil.copytree(REPO / root, scratch / "mirror" / root, dirs_exist_ok=True)
        with cf.ThreadPoolExecutor(args.j) as pool:
            rows = list(pool.map(lambda p: one(bin_, p, scratch), paths))
    out = Path(args.out)
    (out / "census.md").write_text(report(rows, args.bin))
    with (out / "census.tsv").open("w") as f:
        cols = ["path", "status", "calc", "V", "Ua", "Ub", "Uc", "Uc_deep", "nested_only", "held", "agree", "agree_written"]
        f.write("\t".join(cols) + "\n")
        for r in rows:
            f.write("\t".join(str(r.get(c, "")) for c in cols) + "\n")
    print((out / "census.md").read_text())
    return 0


if __name__ == "__main__":
    sys.exit(main())
