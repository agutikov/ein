"""The case page, and the bundle — M1e2 S1e2.1.6, re-pointed at the region modes.

A **case** is a register row: the walk's wrong answer, its control and its
twins, the right answer no run produced, and since S1e2.1a.5's switch
(2026-10-18) the row's `[case.now]` — the region run that asks the row's
question. The walk has no CLI path since the switch, so its blocks are the
register's **record** of what it said, shown as data and not run; the one run
is `now`, a S1e2.1.5 run directory under `runs/now/`. The case page compares
the columns and never re-renders a run. The line is `utils/README.md`
§ Renderers': `ein` writes for one run, and this script, which needs several
answers, the register and Graphviz, assembles.

    build/review/README.md            the index: one row per register row
    build/review/<case>/<witness>/    one case: README.md, runs/now/, right/
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

from review.run import (RunResult, RunSpec, _safe, asked_text, build_run, call_to_fact, diff_block, grids_from,
                        set_text, state_files, state_href, verdict_block, yes)
from review.util import REPO, details, doc, read, rel, run, table, write

REGISTER = REPO / "corpus/wrong_answers.toml"

# The last commit whose `ein solve` ran the walk: a build of it re-runs a
# recorded `wrong` / `control` / `twin` block (status_2026-10-18.md § 9).
WALK_COMMIT = "7ec8a480"


def register() -> list[dict]:
    return tomllib.loads(REGISTER.read_text(encoding="utf-8"))["case"]


def flags_of(run_name: str) -> list[str]:
    """`"solve --maximal"` → `["--maximal"]`: the register's run names are the
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
    stage = owner.lower()
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


# ── a column ──────────────────────────────────────────────────────────────────

@dataclass
class Col:
    label: str
    spec: RunSpec | None
    result: RunResult | None
    answer: dict | None          # the register's block for this column
    right: dict | None = None    # set for the right-answer column
    recorded: bool = False       # a walk block the register keeps; not run


def listed(r: RunResult) -> list[dict]:
    return [x for x in ((r.summary or {}).get("region") or {}).get("states") or [] if x.get("listed")]


def project(facts: list[str], closes: list[str] | None) -> list[str]:
    if not closes:
        return facts
    out = []
    for f in facts:
        m = re.match(r"\(\s*([^\s)]+)", f)
        if m and m.group(1) in closes:
            out.append(f)
    return sorted(out)


def now_differences(want: dict, got: dict | None) -> list[str]:
    """Every field where the run disagrees with the row's `[case.now]` —
    `wrong_answers.rs`'s `differences`, which the gate holds."""
    if got is None:
        return ["no summary was written"]
    out = []
    for name, w, g in [("line", want.get("line"), got["line"]), ("exhausted", want.get("exhausted"), got["exhausted"]),
                       ("k", want.get("k"), got["k"]), ("empty", want.get("empty"), got["empty"]),
                       ("relations", sorted(want.get("relations", [])), got["relations"]),
                       ("models", sorted(sorted(m) for m in want.get("models", [])), got["models"])]:
        if w != g:
            out.append(f"{name}: recorded {json.dumps(w, ensure_ascii=False)}, got {json.dumps(g, ensure_ascii=False)}")
    return out


# ── look_at ───────────────────────────────────────────────────────────────────

def look(r: RunResult, pointer: str, page: Path) -> str:
    """One `look_at` pointer, resolved to a link into the run directory — or
    named as absent from this run, with why, which is information too. The
    pointers were written against the walk's run; a region run has no root
    writes, no `--dump-states` timeline and no `refuted/` (N replaces it)."""
    pages = r.out / "pages"
    word, _, arg = pointer.partition(" ")
    target, anchor, why = None, "", ""
    if word == "root":
        target = pages / "root.md"
    elif word == "root-writes":
        why = "a region run never writes root"
    elif word == "timeline":
        why = "the walk's `--dump-states` timeline has no producer since the switch"
    elif word == "layer":
        step = next((s for s in r.lattice if s.get("layer") == int(arg)), None)
        target = step["page"] if step else None
        why = f"the run visited no layer {arg} — a line mode's steps are batches, not layers"
    elif word == "entering":
        want = sorted(call_to_fact(arg))
        step = next((s for s in r.lattice if any(sorted(c) == want for c in s["commitments"])), None)
        target = step["page"] if step else None
        why = f"the run did not visit {set_text(want)}"
    elif word == "refuted":
        target = pages / "nogoods.md"   # N replaces the walk's `refuted/` (answer_dump.md)
    elif word == "trace":
        target = pages / "trace" / "index.md"
    elif word == "answer":
        target = pages / "answer.md"
    elif word == "model":
        href = state_href(r.out, int(arg), page)
        if href:
            return f"[`{pointer}`]({href})"
        why = f"the run reached fewer than {arg} state(s)"
    elif word == "summary":
        target = pages / "summary.md"
    elif word == "events":
        target = next(iter(sorted((pages / "events").glob(f"{_safe(arg)}*.md"))), None) if arg else pages / "events" / "index.md"
        why = f"the run emitted no `{arg}` event"
    if target is None or not target.exists():
        return f"`{pointer}` — not in this run" + (f": {why}" if why else "")
    return f"[`{pointer}`]({rel(target, page)}{anchor})"


# ── the lattice ───────────────────────────────────────────────────────────────

def lattice_block(cols: list, page: Path) -> list[str]:
    """The cube, drawn (S1e2.1a.2): `ein oracle --dump`'s directory as
    `pages/region/` holds it, linked to that page. No run is overlaid on it
    since the switch; § The region, which compares R with the register's
    lines, is S1e2.1a.7's."""
    out = ["**The lattice** — every node of the commitment cube by saturation alone, `ein oracle --dump` "
           f"([legend]({doc('docs/kernel/ir/03-ein-lang/04_dot_rendering.md', page)}#region-rendering--the-cube-and-the-walk-s1e21a2)):", ""]
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
            out += [f"*{c.label}* — [the region]({rel(index, page)}) (not drawn: the oracle did not answer)", ""]
            any_drawn = True
    return out if any_drawn else []


# ── the case page ─────────────────────────────────────────────────────────────

CHECKLIST = [
    "Is the program the one the case describes?",
    "Is the right answer right — under the definitions it cites, and under the reading it names?",
    "Is the wrong answer what the engine said? (the *wrong* column is the register's record of the walk; "
    f"a build of `{WALK_COMMIT}` re-runs it)",
    "Is it one defect — or two the witness confounds?",
    "Is the lever, if there is one, the only way in?",
]


def review_cell(row: dict) -> str:
    """The user's verdict on a row (S1e2.1.7), or that it has not been read."""
    rv = row.get("review")
    if not rv:
        return "— *not yet reviewed*"
    return f"**{rv['verdict']}**, {rv['date']}, on the bundle at `{rv['bundle']}`" + (f" — {rv['note']}" if rv.get("note") else "")


def walk_answer(a: dict) -> str:
    return (f"`ein {a['run']}`" + (f" `{' '.join(a['env'])}`" if a.get("env") else "")
            + f" → {a.get('verdict', '—')}, k = {a.get('k', '—')}, exhausted = {str(a.get('exhausted', '—')).lower()}")


def region_cell(reg: dict | None) -> str:
    if not reg:
        return "—"
    lines = lambda ls: " · ".join(set_text(x) for x in ls) if ls is not None else "—"  # noqa: E731
    return (f"{reg.get('route', '—')}, monotone = {str(reg.get('monotone', '—')).lower()}, k_s = {reg.get('k_s', '—')}, "
            f"k_c = {reg.get('k_c', '—')}" + (f", empty: {reg['empty']}" if reg.get("empty") else "")
            + f"; minimal {lines(reg.get('minimal'))}; maximal {lines(reg.get('maximal'))}")


def case_page(row: dict, out: Path, bin: str) -> tuple[Path, list[Col]]:
    """Run one register row's `[case.now]`, and build its page."""
    if out.exists():
        shutil.rmtree(out)
    page = out / "README.md"
    right = row["right"]
    now = row["now"]
    mfile = models_file(right)
    diff_path = None
    if mfile:
        diff_path = write(out / "right" / "right.models", mfile)
    cols: list[Col] = []
    recorded = [("wrong", row["wrong"])]
    if "control" in row:
        recorded.append(("control", row["control"]))
    recorded += [(f"twin {i + 1}", t) for i, t in enumerate(row.get("twin", []))]
    for label, a in recorded:
        cols.append(Col(label, None, None, a, recorded=True))
    spec = RunSpec(row["witness"], flags_of(now["run"]), {},
                   title=f"{row['id']} — now: `ein {now['run']}`", diff_models=diff_path)
    r = build_run(spec, out / "runs" / "now", bin)
    nowcol = Col("now", spec, r, now)
    cols.append(nowcol)
    cols.append(Col("right", None, None, None, right))
    lines = [f"# {row['id']} — {row['title']}", ""]

    # 1 — the case.
    owner = owner_page(row["owner"])
    w = row["wrong"]
    rows = [
        ["case", f"**{row['case']}** — {row['title']}"],
        ["class", row["class"]],
        ["settings", row["settings"]],
        ["status", f"**{row['status']}**"],
        ["owner", f"[{row['owner']}]({rel(owner, page)})" if owner else row["owner"]],
        ["carried as", ", ".join(row.get("question", [])) or "—"],
        ["wrong", walk_answer(w) + " — *the walk, as the register records it*"],
        ["now", f"`ein {now['run']}` → {now.get('line')} line, k = {now.get('k')}, exhausted = "
                f"{str(now.get('exhausted')).lower()}" + (f", R empty: {now['empty']}" if now.get("empty") else "")
                + " — *the region run the register holds the row to*"],
        ["region", region_cell(row.get("region"))],
        ["right", (f"{right.get('verdict', '—')}, k = {right.get('k', '—')}"
                   + (f", exhausted = {str(right['exhausted']).lower()}" if "exhausted" in right else "")
                   + (f" — *{right['reading']}*" if right.get("reading") else ""))],
    ]
    if row.get("open_because"):
        rows.append(["open because", row["open_because"]])
    rows += [["note", row.get("note", "—")], ["review", review_cell(row)]]
    lines += ["## 1 · The case", ""] + table(["", ""], rows) + [""]

    # 2 — the problem.
    wpath = REPO / row["witness"]
    lines += ["## 2 · The problem", "", f"[`{row['witness']}`]({rel(wpath, page)})", ""]
    hc = header_comment(wpath)
    if hc:
        lines += details("the witness's own header", ["```text", hc, "```"])
    lines += details(f"the source — {read(wpath).count(chr(10))} lines", ["```lisp", read(wpath).rstrip(), "```"])
    for sub, title in (("program", "the program as written (`render program`)"), ("rules", "every rule (`render rules`)"),
                       ("root", "root after saturation (`render kb FILE`)")):
        svgs = sorted((r.out / "img" / sub).glob("*.svg"))
        if svgs:
            lines += [f"**{title}**", ""]
            lines += [f"![{s.stem}]({rel(s, page)})" for s in svgs] + [""]
    root = (r.summary or {}).get("root", {})
    if root.get("facts_by_relation"):
        lines += details(f"root's {root.get('facts', '?')} facts by relation",
                         table(["relation", "facts"], [[k, v] for k, v in root["facts_by_relation"].items()]))
    def code(p: Path) -> list[str]:
        return [l for l in read(p).splitlines() if l.strip() and not l.strip().startswith(";")]
    for c in cols:
        if not c.recorded or c.label == "wrong":
            continue
        twin_file = c.answer.get("witness", row["witness"])
        twin = REPO / twin_file
        if twin != wpath:
            d = list(difflib.unified_diff(code(wpath), code(twin), row["witness"], twin_file, lineterm="", n=1))
            full = list(difflib.unified_diff(read(wpath).splitlines(), read(twin).splitlines(),
                                             row["witness"], twin_file, lineterm="", n=1))
            lines += [f"**{c.label}: the program against the witness's, comments left out** — "
                      f"{c.answer.get('why', '')}", "", "```diff", *(d or ["(the code is identical)"]), "```", ""]
            lines += details("the whole-file diff, comments included", ["```diff", *full, "```"])
        elif c.answer.get("run") != w.get("run") or c.answer.get("env") != w.get("env"):
            lines += [f"**{c.label}: the same file, run by the walk as** `ein {c.answer['run']}`"
                      + (f" with `{' '.join(c.answer.get('env', []))}`" if c.answer.get("env") else "")
                      + f" — {c.answer.get('why', '')}", ""]

    # 3 — the answers, side by side.
    lines += ["## 3 · The answers, side by side", "",
              f"The walk's columns are the register's record (`wrong`, `control`, `twin`): the walk has no CLI path "
              f"since S1e2.1a.5's switch, and a build of `{WALK_COMMIT}` re-runs them. **now** is run here.", ""]
    head = ["", *[c.label + (" (walk, recorded)" if c.recorded else "") for c in cols]]
    def per(fn):
        return [yes(fn(c)) for c in cols]
    region = ((r.summary or {}).get("region") or {})
    def run_cell(c):
        if c.right is not None:
            return "*(the definitions)*"
        if c.recorded:
            return f"`ein {c.answer['run']}`" + (f" `{' '.join(c.answer.get('env', []))}`" if c.answer.get("env") else "")
        return f"[`ein {c.answer['run']}`]({rel(c.result.out / 'run.md', page)})"
    def field(name, live):
        def f(c):
            if c.right is not None:
                return c.right.get(name, "—")
            if c.recorded:
                return c.answer.get(name, "—")
            return live(c)
        return f
    diffs = now_differences(now, r.now)
    table_rows = [
        ["run", *per(run_cell)],
        ["asked", *per(lambda c: asked_text(c.result.summary) if c.result else "—")],
        ["verdict", *per(field("verdict", lambda c: c.result.verdict or "— *(`--single`'s alone)*"))],
        ["k", *per(field("k", lambda c: c.result.k))],
        ["exhausted", *per(field("exhausted", lambda c: c.result.exhausted))],
        ["qualifier", *per(field("qualifier", lambda c: "—"))],
        ["exit", *per(lambda c: "—" if c.right is not None else c.answer.get("exit", 0) if c.recorded
                      else c.result.invocations[0].code)],
        ["states", *per(lambda c: "—" if c.result is None else
                        "<br>".join(f"{i + 1}: from {set_text(s.get('from'))}" for i, s in enumerate(listed(c.result)))
                        or "—")],
        ["core", *per(lambda c: (" ".join(c.right.get("core", [])) or ("—" if "core" not in c.right else "∅"))
                      if c.right is not None else
                      (" ".join(c.answer.get("core", [])) or ("—" if "core" not in c.answer else "∅"))
                      if c.recorded else (" ".join(region.get("core") or []) or "—"))],
        ["the register's `now`", *per(lambda c: "—" if c.result is None else
                                      ("**matches**" if not diffs else "**differs**: " + "; ".join(diffs)))],
    ]
    lines += table(head, table_rows) + [""]
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
              if closes else "Every fact of each state's `(model …)`.", ""]
    for c in cols:
        if c.right is not None or c.recorded:
            ms = (c.right or c.answer).get("models")
            what = "right" if c.right is not None else f"{c.label} — the walk, recorded"
            lines += [f"### {what} — {len(ms) if ms is not None else 'not stated'} model(s)", ""]
            lines += [f"- {i + 1}: " + (" ".join(f"`{f}`" for f in m) or "*(no facts)*") for i, m in enumerate(ms or [])] or [
                "_The register states no model set for this column: its answer is a verdict, a count or a qualifier._"]
            lines.append("")
            continue
        st = listed(c.result)
        lines += [f"### {c.label} — {len(st)} listed state(s), {asked_text(c.result.summary)}", ""]
        grid = next((i for i in c.result.invocations if i.name == "grid"), None)
        grids = grids_from(grid.stdout) if grid else []
        files = {f.get("state"): f for f in state_files(c.result.out / "raw" / "answer") if f["kind"] == "models"}
        for i, s in enumerate(st):
            facts = project(s.get("facts", []), closes)
            f, href = files.get(i + 1), state_href(c.result.out, i + 1, page)
            label = f"[{f['stem']}]({href})" if f and href else f"state {i + 1}"
            lines += [f"- {label}: " + (" ".join(f"`{x}`" for x in facts) or "*(no facts)*")]
            if i < len(grids) and not grids[i].startswith("grid — none"):
                lines += ["", "  " + grids[i].replace("\n", "\n  "), ""]
        if not st:
            frontier = region.get("frontier") or []
            lines += ["_no state listed_" + (f" — R is empty ({region['empty']})" if region.get("empty") else "")]
            for i, x in enumerate(frontier):
                lines += [f"- owing {i + 1}: " + (" ".join(f"`{y}`" for y in project(x.get("facts", []), closes))
                                                  or "*(no facts)*")]
        lines.append("")
        diff_inv = next((i for i in c.result.invocations if i.name == "diff"), None)
        if diff_inv:
            lines += [f"**{c.label} against the right answer** (`--diff-models`, verbatim):", "", "```text",
                      diff_block(diff_inv), "```", ""]

    # 5 — where it went wrong.
    lines += ["## 5 · Where it went wrong", "",
              "The register's `look_at` pointers, written against the walk's run and resolved in **now**'s. "
              "The script links them; it does not find them.", ""]
    look_rows = [[f"`{ptr}`", look(r, ptr, page)] for ptr in row.get("look_at", [])]
    lines += table(["pointer", "now"], look_rows) if look_rows else ["_no pointers_"]
    lines.append("")

    # 6 — every artefact.
    lines += ["## 6 · Every artefact", ""]
    run_md = r.out / "run.md"
    fails = [x for x in r.checks if x[1] is False]
    lines += [f"- **now** — [run.md]({rel(run_md, page)}) · {len(r.pages)} pages · "
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
        lines += ["## The register", "",
                  "*wrong* is the walk's answer as the register records it; *now* is the row's `[case.now]`, run "
                  "here, and marked where it differs from what the register holds it to.", ""]
        rows = []
        for row, p, cols in built:
            w, r, now = row["wrong"], row["right"], row["now"]
            got = next(c.result for c in cols if c.result is not None)
            diffs = now_differences(now, got.now)
            # What the right answer states differently from the walk's — read
            # off the two register blocks, the index's one comparison of its own.
            differs = [f for f in ("verdict", "k", "exhausted", "core") if f in r and r[f] != w.get(f)]
            if "models" in r and sorted(map(sorted, r["models"])) != sorted(map(sorted, w.get("models", []) or [])):
                differs.append("models")
            if "qualifier" in r:
                differs.append("qualifier")
            rows.append([f"[{row['id']}]({rel(p, page)})", row["owner"], row["settings"],
                         f"{w['verdict']} k={w['k']}",
                         f"`{now['run']}` {got.now['line'] if got.now else '?'} k={got.k}"
                         + ("" if not diffs else " **(≠ the register: " + "; ".join(d.split(':')[0] for d in diffs) + ")**"),
                         f"{r.get('verdict', '—')} k={r.get('k', '—')}",
                         ", ".join(differs) or ("*a reading*" if r.get("reading") else "—"), row["status"],
                         row["review"]["verdict"] if "review" in row else "—"])
        lines += table(["case", "owner", "settings", "wrong (walk, recorded)", "now (run)", "right", "differs in",
                        "status", "review"], rows) + [""]
    if extra:
        lines += ["## Programs", ""]
        lines += table(["program", "asked", "states listed", "exhausted"],
                       [[f"[{name}]({rel(p, page)})", asked_text(r.summary), r.k, yes(r.exhausted)]
                        for name, p, r in extra]) + [""]
    if failed:
        lines += ["## Failed to build", ""] + [f"- `{n}`: {e}" for n, e in failed] + [""]
    if (out / "regions" / "README.md").exists():
        lines += ["## The region gallery", "",
                  f"[Every corpus program's region]({rel(out / 'regions' / 'README.md', page)}) — the synthetic "
                  "programs, the biggest and the most interesting (`utils/review_bundle.py --regions`).", ""]
    write(page, "\n".join(lines) + "\n")
    return page


def program_case(file: str, flags: list[str], out: Path, bin: str, events: bool = True,
                 timeout_note: str = "") -> tuple[Path, RunResult]:
    """A page for a program outside the register: the problem, one run, its
    states and every artefact — sections 2, 3, 4 and 6."""
    if out.exists():
        shutil.rmtree(out)
    r = build_run(RunSpec(file, flags, title=file, events=events), out / "runs" / "run", bin)
    page = out / "README.md"
    path = REPO / file
    lines = [f"# {file}", "", f"> `ein solve {file} {' '.join(flags)}`".rstrip() + f" — not a register case. {timeout_note}",
             "", "## The problem", ""]
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
                  f"every node by saturation alone, `ein oracle --dump`.", "", f"![region]({rel(svg, page)})", ""]
    lines += ["## The models", ""]
    files = {f.get("state"): f for f in state_files(r.out / "raw" / "answer") if f["kind"] == "models"}
    for i, s in enumerate(listed(r)):
        f, href = files.get(i + 1), state_href(r.out, i + 1, page)
        label = f"[{f['stem']}]({href})" if f and href else f"state {i + 1}"
        lines += [f"- {label}: " + (" ".join(f"`{x}`" for x in s.get("facts", [])) or "*(no facts)*")
                  + f"  *(from {set_text(s.get('from'))})*"]
    lines += ["", "## Every artefact", "", f"[run.md]({rel(r.out / 'run.md', page)})", ""]
    write(page, "\n".join(lines) + "\n")
    return page, r
