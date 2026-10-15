"""The case page, and the bundle — M1e2 S1e2.1.6.

A **case** is several runs — the register's wrong one, its control, its twins
— and a right answer no run produced. Each run is a S1e2.1.5 run directory
under `runs/`; the case page compares them and never re-renders one. The line
is `utils/README.md` § Renderers': `ein` writes for one run, and this script,
which needs several runs, the register and Graphviz, assembles.

    build/review/README.md            the index: one row per register row
    build/review/<case>/<witness>/    one case: README.md, runs/, right/
"""

from __future__ import annotations

import difflib
import json
import re
import shutil
import tomllib
from dataclasses import dataclass
from datetime import date
from pathlib import Path

from review.run import (RunResult, RunSpec, _safe, build_run, commit_text, diff_block, grids_from,
                        lattice_page_for, tree, verdict_block)
from review.util import REPO, details, doc, esc, load_json, read, rel, run, table, write

REGISTER = REPO / "corpus/wrong_answers.toml"


def register() -> list[dict]:
    return tomllib.loads(REGISTER.read_text(encoding="utf-8"))["case"]


def flags_of(run_name: str) -> list[str]:
    """`"solve -e -L"` → `["-e", "-L"]`: the register's run names are the
    corpus's, `ein <run>` with the file elided."""
    toks = run_name.split()
    assert toks and toks[0] == "solve", f"a register run is a solve, got {run_name!r}"
    return toks[1:]


def env_of(answer: dict) -> dict[str, str]:
    return dict(e.split("=", 1) for e in answer.get("env", []))


def models_file(right: dict) -> str | None:
    """The register's `right.models` in `--diff-models`' spelling — the same
    `closes` and the same fact lists, so the CLI compares what the row says."""
    if "models" not in right or not right.get("closes"):
        return None
    lines = ["; written by utils/review_bundle.py from corpus/wrong_answers.toml",
             "closes " + " ".join(right["closes"])]
    for m in right["models"]:
        lines.append("model")
        lines += list(m)
    return "\n".join(lines) + "\n"


def owner_page(owner: str) -> Path | None:
    stage = owner.lower().replace("s1e2.", "s1e2.")
    for p in (REPO / "plans/m1e2_correctness").rglob(f"{stage}_*.md"):
        return p
    return None


def header_comment(path: Path) -> str:
    """The witness's leading `;` comment block, as prose."""
    out = []
    for line in read(path).splitlines():
        s = line.strip()
        if not s:
            if out:
                out.append("")
            continue
        if not s.startswith(";"):
            break
        out.append(re.sub(r"^;+ ?", "", s))
    return "\n".join(out).strip()


# ── a run, summarised for a column ────────────────────────────────────────────

@dataclass
class Col:
    label: str
    spec: RunSpec | None
    result: RunResult | None
    answer: dict | None          # the register's block for this run
    right: dict | None = None    # set for the right-answer column


def verdict_line(stdout: str) -> str:
    m = re.search(r"^\s*verdict\s+(.*)$", stdout, re.M)
    return m.group(1).strip() if m else "—"


def qualifier(stdout: str) -> str:
    m = re.search(r"^[ \t]*solutions \(k\)[ \t]+\d+[ \t]*(.*)$", stdout, re.M)
    return (m.group(1).strip() if m else "") or "—"


def states_of(r: RunResult) -> list[dict]:
    raw = r.out / "raw" / "answer"
    index = load_json(raw / "index.json") or {}
    return [load_json(raw / f"{stem}.json") | {"stem": stem} for stem in index.get("models", []) + index.get("open", [])
            if load_json(raw / f"{stem}.json") is not None]


def project(facts: list[str], closes: list[str] | None) -> list[str]:
    if not closes:
        return facts
    return sorted(f for f in facts if re.match(r"\(\s*(\S+)", f) and re.match(r"\(\s*(\S+)", f).group(1).rstrip(")") in closes)


# ── look_at ───────────────────────────────────────────────────────────────────

def look(r: RunResult, pointer: str, page: Path) -> str:
    """One `look_at` pointer, resolved to a link into `r`'s run directory — or
    named as absent from this run, which is information too."""
    pages = r.out / "pages"
    word, _, arg = pointer.partition(" ")
    target, anchor = None, ""
    if word == "root":
        target = pages / "root.md"
    elif word == "root-writes":
        target, anchor = pages / "root.md", "#what-the-search-wrote-into-root-afterwards"
    elif word == "timeline":
        target = next(iter(sorted((pages / "states").glob("timeline*.md"))), None)
    elif word == "layer":
        target = pages / "states" / f"layer-{int(arg):02d}.md"
    elif word == "entering":
        want = arg.replace(" ", "")
        for f in (r.out / "raw" / "lattice" / "enterings").rglob("commitment.json"):
            if commit_text(load_json(f)).replace(" ", "") == want:
                target = lattice_page_for(_PagesShim(r), f.parent)
                break
    elif word == "refuted":
        target = pages / "refuted.md"
    elif word == "trace":
        if arg.startswith("step "):
            n = int(arg.split()[1])
            for p in (pages / "trace").glob("steps-*.md"):
                a, b = (int(x) for x in re.findall(r"\d+", p.stem))
                if a <= n <= b:
                    target = p
                    anchor = "#" + re.sub(r"[^\w\- ]", "", (re.search(rf"^## (Step {n} — [^\n]*)", read(p), re.M) or [None, ""])[1].lower()).replace(" ", "-")
        else:
            target = pages / "trace" / "index.md"
    elif word == "answer":
        target = pages / "answer.md"
    elif word == "model":
        target, anchor = pages / "answer.md", f"#models{int(arg):02d}"
    elif word == "summary":
        target = pages / "summary.md"
    elif word == "events":
        target = next(iter(sorted((pages / "events").glob(f"{_safe(arg)}*.md"))), None) if arg else pages / "events" / "index.md"
    if target is None or not target.exists():
        return f"`{pointer}` — not in this run"
    return f"[`{pointer}`]({rel(target, page)}{anchor})"


class _PagesShim:
    """`lattice_page_for` wants a `Pages`; a finished run has its `dir`."""

    def __init__(self, r: RunResult):
        self.dir = r.out / "pages"


# ── the lattice ───────────────────────────────────────────────────────────────

def lattice_block(cols: list, page: Path) -> list[str]:
    """Each run's region, drawn (S1e2.1a.2): the SVG `pages/region/` holds,
    linked to that page. The right column has none — it is the definitions."""
    out = ["**The lattice** — every node of the commitment cube by saturation alone, with each run's walk "
           f"on it ([legend]({doc('docs/kernel/ir/03-ein-lang/04_dot_rendering.md', page)}#region-rendering--the-cube-and-the-walk-s1e21a2)):", ""]
    any_drawn = False
    for c in cols:
        if c.result is None:
            continue
        svg = c.result.out / "img" / "region" / "region.svg"
        index = c.result.out / "pages" / "region" / "index.md"
        if svg.exists():
            any_drawn = True
            out += [f"*{c.label}* — [the region]({rel(index, page)})", "", f"![{c.label}]({rel(svg, page)})", ""]
        elif index.exists():
            out += [f"*{c.label}* — [the region]({rel(index, page)}) (no drawing)", ""]
    return out if any_drawn else []


# ── the case page ─────────────────────────────────────────────────────────────

CHECKLIST = [
    "Is the program the one the case describes?",
    "Is the right answer right — under the definitions it cites, and under the reading it names?",
    "Is the wrong answer what the engine says? (the *wrong* column, and `raw/main.stdout.txt`)",
    "Is it one defect — or two the witness confounds?",
    "Is the lever, if there is one, the only way in?",
]


def review_cell(row: dict) -> str:
    """The user's verdict on a row (S1e2.1.7), or that it has not been read."""
    rv = row.get("review")
    if not rv:
        return "— *not yet reviewed*"
    return f"**{rv['verdict']}**, {rv['date']}, on the bundle at `{rv['bundle']}`" + (f" — {rv['note']}" if rv.get("note") else "")


def case_page(row: dict, out: Path, bin: str) -> tuple[Path, list[Col]]:
    """Build every run of one register row, and its page."""
    if out.exists():
        shutil.rmtree(out)
    page = out / "README.md"
    right = row["right"]
    mfile = models_file(right)
    diff_path = None
    if mfile:
        diff_path = write(out / "right" / "right.models", mfile)
    cols: list[Col] = []
    runs = [("wrong", row["wrong"])]
    if "control" in row:
        runs.append(("control", row["control"]))
    runs += [(f"twin {i + 1}", t) for i, t in enumerate(row.get("twin", []))]
    for label, a in runs:
        spec = RunSpec(a.get("witness", row["witness"]), flags_of(a["run"]), env_of(a),
                       title=f"{row['id']} — {label}: {a.get('why', 'the wrong run')}",
                       diff_models=diff_path)
        r = build_run(spec, out / "runs" / label.replace(" ", "-"), bin)
        cols.append(Col(label, spec, r, a))
    cols.append(Col("right", None, None, None, right))
    wrong = cols[0].result
    lines = [f"# {row['id']} — {row['title']}", ""]

    # 1 — the case.
    owner = owner_page(row["owner"])
    w, r_ = row["wrong"], right
    lines += ["## 1 · The case", ""]
    lines += table(["", ""], [
        ["case", f"**{row['case']}** — {row['title']}"],
        ["class", row["class"]],
        ["settings", row["settings"]],
        ["status", f"**{row['status']}**"],
        ["owner", f"[{row['owner']}]({rel(owner, page)})" if owner else row["owner"]],
        ["carried as", ", ".join(row.get("question", [])) or "—"],
        ["wrong", f"`ein {w['run']}` → {w['verdict']}, k = {w['k']}, exhausted = {str(w['exhausted']).lower()}"],
        ["right", (f"{r_.get('verdict', '—')}, k = {r_.get('k', '—')}"
                   + (f", exhausted = {str(r_['exhausted']).lower()}" if "exhausted" in r_ else "")
                   + (f" — *{r_['reading']}*" if r_.get("reading") else ""))],
        ["note", row.get("note", "—")],
        ["review", review_cell(row)],
    ])
    lines.append("")

    # 2 — the problem.
    wpath = REPO / row["witness"]
    lines += ["## 2 · The problem", "", f"[`{row['witness']}`]({rel(wpath, page)})", ""]
    hc = header_comment(wpath)
    if hc:
        lines += details("the witness's own header", ["```text", hc, "```"])
    lines += details(f"the source — {read(wpath).count(chr(10))} lines", ["```lisp", read(wpath).rstrip(), "```"])
    for sub, title in (("program", "the program as written (`render program`)"), ("rules", "every rule (`render rules`)"),
                       ("root", "root after saturation (`render kb root.ein`)")):
        svgs = sorted((wrong.out / "img" / sub).glob("*.svg"))
        if svgs:
            lines += [f"**{title}**", ""]
            lines += [f"![{s.stem}]({rel(s, page)})" for s in svgs] + [""]
    root = (wrong.summary or {}).get("root", {})
    if root.get("by_relation"):
        lines += details(f"root's {root.get('n_facts', '?')} facts by relation",
                         table(["relation", "facts"], [[k, v] for k, v in root["by_relation"].items()]))
    def code(p: Path) -> list[str]:
        return [l for l in read(p).splitlines() if l.strip() and not l.strip().startswith(";")]
    for c in cols[1:-1]:
        twin = REPO / c.spec.file
        if twin != wpath:
            d = list(difflib.unified_diff(code(wpath), code(twin), row["witness"], c.spec.file, lineterm="", n=1))
            full = list(difflib.unified_diff(read(wpath).splitlines(), read(twin).splitlines(),
                                             row["witness"], c.spec.file, lineterm="", n=1))
            lines += [f"**{c.label}: the program against the witness's, comments left out** — "
                      f"{c.answer.get('why', '')}", "", "```diff", *(d or ["(the code is identical)"]), "```", ""]
            lines += details("the whole-file diff, comments included", ["```diff", *full, "```"])
        elif c.spec.flags != cols[0].spec.flags or c.spec.env != cols[0].spec.env:
            lines += [f"**{c.label}: the same file, run as** `ein {c.answer['run']}`"
                      + (f" with `{' '.join(c.answer.get('env', []))}`" if c.answer.get("env") else "")
                      + f" — {c.answer.get('why', '')}", ""]

    # 3 — the answers, side by side.
    lines += ["## 3 · The answers, side by side", ""]
    head = ["", *[c.label for c in cols]]
    def per(fn):
        return [fn(c) for c in cols]
    def run_cell(c):
        if c.right is not None:
            return "*(the definitions)*"
        return f"[`ein {c.answer['run']}`]({rel(c.result.out / 'run.md', page)})" + (
            f" `{' '.join(c.answer.get('env', []))}`" if c.answer.get("env") else "")
    rows = [
        ["run", *per(run_cell)],
        ["verdict", *per(lambda c: c.right.get("verdict", "—") if c.right is not None
                         else verdict_line(c.result.invocations[0].stdout))],
        ["k", *per(lambda c: c.right.get("k", "—") if c.right is not None else c.result.k)],
        ["exhausted", *per(lambda c: c.right.get("exhausted", "—") if c.right is not None else c.result.exhausted)],
        ["qualifier", *per(lambda c: c.right.get("qualifier", "—") if c.right is not None
                           else qualifier(c.result.invocations[0].stdout))],
        ["exit", *per(lambda c: "—" if c.right is not None else c.result.invocations[0].code)],
        ["commitments", *per(lambda c: "—" if c.right is not None else
                             "<br>".join(f"{s['stem']}: {commit_text(s.get('commitment'))}" for s in states_of(c.result)) or "—")],
        ["core", *per(lambda c: " ".join(c.right.get("core", [])) or ("—" if "core" not in c.right else "∅")
                      if c.right is not None else
                      (" ".join((c.result.summary or {}).get("verdict", {}).get("unsat_core", [])) or
                       ("∅" if c.result.verdict == "Contradiction" else "—")))],
    ]
    lines += table(head, rows) + [""]
    lines += lattice_block(cols, page)
    if right.get("derived"):
        lines += [f"*Right answer derived from:* {right['derived']}", ""]
    for alt in right.get("alternative", []):
        lines += [f"*Under {alt.get('reading')}:* " + ", ".join(
            f"{k} = {json.dumps(alt[k]) if k != 'models' else len(alt[k])}"
            for k in ("verdict", "k", "exhausted", "models") if k in alt) + f" — {alt.get('derived', '')}", ""]

    # 4 — the models, side by side.
    closes = right.get("closes")
    lines += ["## 4 · The models, side by side", "",
              f"Facts over **{', '.join(closes)}** — the register's `closes`, `:expect`'s *naming a relation closes it*."
              if closes else "Every fact of each recorded state.", ""]
    for c in cols:
        if c.right is not None:
            ms = c.right.get("models")
            lines += [f"### right — {len(ms) if ms is not None else 'not stated'} model(s)", ""]
            lines += [f"- {i + 1}: " + (" ".join(f"`{f}`" for f in m) or "*(no facts)*") for i, m in enumerate(ms or [])] or [
                "_The register states no model set for this case: its right answer is a verdict, a count or a qualifier._"]
            lines.append("")
            continue
        st = states_of(c.result)
        lines += [f"### {c.label} — {len(st)} recorded state(s)", ""]
        grids = grids_from(c.result.invocations[[i.name for i in c.result.invocations].index("grid")].stdout) \
            if any(i.name == "grid" for i in c.result.invocations) else []
        for i, s in enumerate(st):
            facts = project(s.get("facts", []), closes)
            lines += [f"- [{s['stem']}]({rel(c.result.out / 'pages' / 'answer.md', page)}#{s['stem'].replace('/', '')})"
                      f"{'' if s.get('reported', True) else ' (not reported)'}: "
                      + (" ".join(f"`{f}`" for f in facts) or "*(no facts)*")]
            if i < len(grids) and not grids[i].startswith("grid — none"):
                lines += ["", "  " + grids[i].replace("\n", "\n  "), ""]
        if not st:
            lines += ["_no state recorded_"]
        lines.append("")
        diff_inv = next((i for i in c.result.invocations if i.name == "diff"), None)
        if diff_inv:
            other = c.spec.file != row["witness"]
            lines += [f"**{c.label} against the right answer** (`--diff-models`, verbatim)"
                      + (" — *a different program, held to the witness's right answer: read it as a comparison, "
                         "not as this program's verdict*" if other else "") + ":", "", "```text",
                      diff_block(diff_inv), "```", ""]

    # 5 — where it went wrong.
    lines += ["## 5 · Where it went wrong", "",
              "The register's `look_at` pointers, resolved in each run. The script links them; it does not find them.", ""]
    look_rows = []
    for ptr in row.get("look_at", []):
        look_rows.append([f"`{ptr}`", *[look(c.result, ptr, page) for c in cols if c.result is not None]])
    lines += table(["pointer", *[c.label for c in cols if c.result is not None]], look_rows) if look_rows else ["_no pointers_"]
    lines.append("")

    # 6 — every artefact.
    lines += ["## 6 · Every artefact", ""]
    for c in cols:
        if c.result is None:
            continue
        run_md = c.result.out / "run.md"
        fails = [x for x in c.result.checks if x[1] is False]
        lines += [f"- **{c.label}** — [run.md]({rel(run_md, page)}) · {len(c.result.pages)} pages · "
                  + (f"**{len(fails)} cross-check mismatch(es)**: " + "; ".join(f"{n} — {d}" for n, _, d in fails)
                     if fails else "every cross-check passes")]
        sec = read(run_md).split("## Every page", 1)[-1].split("## Every invocation", 1)[0]
        for m in re.finditer(r"^\| \[([^\]]+)\]\(([^)]+)\) \| (\d+) \| (\d+) \|$", sec, re.M):
            lines.append(f"  - [{m.group(1)}]({rel(run_md.parent / m.group(2), page)}) — {m.group(3)} pages, {m.group(4)} lines")
    lines.append("")

    # 7 — what to check.
    lines += ["## 7 · What to check", "", "The review (S1e2.1.7) answers these for this row:", ""]
    lines += [f"- [ ] {q}" for q in CHECKLIST]
    lines += ["", "Verdict: *confirmed* · *right answer corrected* · *not wrong* · *split* · *merge*", ""]
    if right:
        write(out / "right" / "README.md", "\n".join(
            [f"# {row['id']} — the right answer", "", f"[The case](../README.md) · from `corpus/wrong_answers.toml`", "",
             "```json", json.dumps(right, indent=2, ensure_ascii=False), "```", ""]
            + ([f"`--diff-models` reads [right.models](right.models), written from it."] if mfile else [])))
    write(page, "\n".join(lines) + "\n")
    return page, cols


# ── the index, and the modes ──────────────────────────────────────────────────

def git_commit() -> str:
    c = run("git", ["git", "rev-parse", "--short", "HEAD"]).stdout.strip()
    dirty = run("git", ["git", "status", "--porcelain"]).stdout.strip()
    return c + (" (working tree dirty)" if dirty else "")


def index_page(out: Path, built: list[tuple[dict, Path, list[Col]]], extra: list[tuple[str, Path, RunResult]],
               failed: list[tuple[str, str]], bin: str) -> Path:
    page = out / "README.md"
    version = run("version", [bin, "--version"]).stdout.splitlines()
    lines = ["# The review bundle", "",
             f"> Built {date.today().isoformat()} at commit `{git_commit()}` · `{version[0] if version else bin}` · "
             f"from [`corpus/wrong_answers.toml`]({rel(REGISTER, page)}) — M1e2 P1e2.1, for "
             f"[the review]({doc('plans/m1e2_correctness/p1e2.1_seeing_the_answer/s1e2.1.7_the_review.md', page)}). "
             "`build/` is gitignored: rebuild with `python3 utils/review_bundle.py`.", ""]
    if built:
        lines += ["## The register", ""]
        rows = []
        for row, p, cols in built:
            w, r = row["wrong"], row["right"]
            got = cols[0].result
            agrees = (got.verdict == w["verdict"] and got.k == w["k"])
            # What the right answer states differently — read off the two
            # register blocks, the index's one comparison of its own.
            differs = [f for f in ("verdict", "k", "exhausted", "core") if f in r and r[f] != w.get(f)]
            if "models" in r and sorted(map(sorted, r["models"])) != sorted(map(sorted, w.get("models", []) or [])):
                differs.append("models")
            if "qualifier" in r:
                differs.append("qualifier")
            rows.append([f"[{row['id']}]({rel(p, page)})", row["owner"], row["settings"],
                         f"{w['verdict']} k={w['k']}" + ("" if agrees else f" **(now {got.verdict} k={got.k})**"),
                         f"{r.get('verdict', '—')} k={r.get('k', '—')}",
                         ", ".join(differs) or ("*a reading*" if r.get("reading") else "—"), row["status"],
                         row["review"]["verdict"] if "review" in row else "—"])
        lines += table(["case", "owner", "settings", "wrong (register)", "right", "differs in", "status", "review"], rows) + [""]
    if extra:
        lines += ["## Programs", ""]
        lines += table(["program", "verdict", "k", "exhausted"],
                       [[f"[{name}]({rel(p, page)})", r.verdict, r.k, r.exhausted] for name, p, r in extra]) + [""]
    if failed:
        lines += ["## Failed to build", ""] + [f"- `{n}`: {e}" for n, e in failed] + [""]
    write(page, "\n".join(lines) + "\n")
    return page


def program_case(file: str, flags: list[str], out: Path, bin: str, events: bool = True,
                 timeout_note: str = "") -> tuple[Path, RunResult]:
    """A page for a program outside the register: the problem, one run, its
    models and every artefact — sections 2, 3, 4 and 6."""
    if out.exists():
        shutil.rmtree(out)
    r = build_run(RunSpec(file, flags, title=file, events=events), out / "runs" / "run", bin)
    page = out / "README.md"
    path = REPO / file
    lines = [f"# {file}", "", f"> `ein solve {file} {' '.join(flags)}` — not a register case. {timeout_note}", "",
             "## The problem", ""]
    hc = header_comment(path)
    if hc:
        lines += details("the file's own header", ["```text", hc, "```"])
    lines += details(f"the source — {read(path).count(chr(10))} lines", ["```lisp", read(path).rstrip(), "```"])
    for sub in ("program", "root"):
        lines += [f"![{s.stem}]({rel(s, page)})" for s in sorted((r.out / "img" / sub).glob("*.svg"))]
    lines += ["", "## The answer", "", "```text", verdict_block(r.invocations[0].stdout), "```", ""]
    svg = r.out / "img" / "region" / "region.svg"
    if svg.exists():
        lines += ["## The lattice", "", f"[The region]({rel(r.out / 'pages' / 'region' / 'index.md', page)}) — "
                  f"every node by saturation alone, this run's walk on it.", "", f"![region]({rel(svg, page)})", ""]
    lines += ["## The models", ""]
    for s in states_of(r):
        lines += [f"- [{s['stem']}]({rel(r.out / 'pages' / 'answer.md', page)}): "
                  + " ".join(f"`{f}`" for f in s.get("added", [])) + "  *(added to root)*"]
    lines += ["", "## Every artefact", "", f"[run.md]({rel(r.out / 'run.md', page)})", ""]
    write(page, "\n".join(lines) + "\n")
    return page, r
