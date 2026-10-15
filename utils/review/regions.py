"""The region gallery — M1e2 S1e2.1a.8.

    utils/review_bundle.py --regions [--out DIR]       default build/review/regions/

Every corpus program's region, surveyed, and a page for the ones worth
reading: the **synthetic** programs `utils/region_synth.py` wrote for a chosen
region, the **biggest** regions, and the **most interesting**, each one named
for why. The survey runs each program at its corpus entry's first declared
`solve` flags (or plain `solve`) with `--dump-region`, and reads the numbers
off `region.json` and `ein render region --view table`'s summary: the script
sorts and links, and computes no region and no overlay of its own.

    DIR/README.md                       the gallery: the three sections, then every region
    DIR/programs/<path>/README.md       one program, every artefact (utils/review_bundle.py's program page)
    DIR/programs/<path>/right/          synthetic only: each line node as a loadable program, drawn
"""

from __future__ import annotations

import json
import re
import shutil
import tempfile
import tomllib
from pathlib import Path

from review.util import REPO, SvgJob, details, doc, load_json, read, rel, render_svgs, run, table, write

# Raised `--region-limit`s: the corpus's largest cubes under a minute each
# (measured 2026-10-15 on 16 jobs). The n = 18 pair, `branching/10` and `/11`,
# is not here: it ran past 11 minutes on 8 jobs, its per-node cost growing with
# the commitment far past the 64-node projection.
BIG = {"examples/syntax/arg-kinds.ein": 19, "examples/saturation/square-bwd/houses.ein": 20}
SURVEY_TIMEOUT = 120
BIGGEST = 8


def corpus_entries() -> list[tuple[str, list[str]]]:
    """Every corpus entry with the flags of its declared `solve -e` run, else
    its first `solve`, else none — the cube does not depend on them, the walk
    does."""
    corpus = tomllib.loads((REPO / "corpus/corpus.toml").read_text(encoding="utf-8"))
    out = []
    for e in corpus["entry"]:
        if e.get("slow"):
            continue
        solves = [r for r in e.get("runs", []) if r.split()[0] == "solve" and "{out}" not in r]
        # The exhaustive run where one is declared: the walk against R is then
        # the whole walk, not the first model's.
        run_name = next((r for r in solves if "-e" in r.split()), solves[0] if solves else "solve")
        flags = [f for f in run_name.split()[1:] if "{out}" not in f and f != "--dump-region"]
        out.append((e["path"], flags))
    return out


def summary_of(table_md: str) -> dict:
    """The table view's head, as fields: its lines say what the overlay found."""
    s: dict = {}
    for line in table_md.splitlines():
        if line.startswith("| bit |"):
            break
        m = re.match(r"- \*\*(minimal|maximal) line\*\* (.*) — missing (.*); spurious (.*)$", line)
        if m:
            s[m.group(1)] = (m.group(2), m.group(3), m.group(4))
        m = re.match(r"- \*\*exclusions\*\* (\d+) checked, (\d+) unsound", line)
        if m:
            s["exclusions"] = (int(m.group(1)), int(m.group(2)))
        m = re.match(r"- \*\*the walk\*\* (.*)$", line)
        if m:
            s["walk"] = m.group(1)
    s["disagree"] = table_md.count("≠ the search") + table_md.count("**≠ ")
    return s


def survey(bin: str, tmp: Path) -> list[dict]:
    rows = []
    for path, flags in corpus_entries():
        d = tmp / path.replace("/", "_")
        limit = BIG.get(path)
        argv = [bin, "solve", path, *flags, "--dump-region", str(d), "-j", "8"]
        if limit:
            argv += ["--region-limit", str(limit)]
        inv = run("survey", argv, timeout=SURVEY_TIMEOUT)
        region = load_json(d / "region.json")
        walk = load_json(d / "walk.json") or {}
        row = {"path": path, "flags": flags, "limit": limit, "n": None, "refused": None, "exit": inv.code}
        if region is None:
            row["refused"] = "timed out" if inv.code == -2 else "no region.json"
            rows.append(row)
            continue
        row["n"] = region["n"]
        r = region.get("region")
        if r is None:
            row["refused"] = f"n = {region['n']}, over the limit"
            rows.append(row)
            continue
        nodes = r["nodes"]
        t = run("table", [bin, "render", "region", str(d), "--view", "table"], timeout=SURVEY_TIMEOUT)
        row.update(
            size=r["size"], minimal=len(r["minimal"]), maximal=len(r["maximal"]), holes=len(r["holes"]),
            components=len(r["components"]),
            subcubes=len(r["subcubes"]) if r["subcubes"] is not None else None,
            owing=sum(x["status"] == "owing" for x in nodes),
            dead=sum(x["status"].startswith("dead") for x in nodes),
            moved=sum(not x["status"].startswith("dead") and x.get("closure", x["mask"]) != x["mask"] for x in nodes),
            local=len(r["local_max"]) + len(r["local_min"]),
            monotone=region["monotonicity"]["monotone"],
            verdict=walk.get("verdict", {}).get("type", "?"), k=walk.get("verdict", {}).get("k"),
            exhausted=walk.get("verdict", {}).get("exhausted"),
            **{"overlay": summary_of(t.stdout)})
        shutil.rmtree(d, ignore_errors=True)
        rows.append(row)
    return rows


def tags(row: dict) -> list[str]:
    """Why a region is worth a look, each tag read off the survey."""
    if row.get("size") is None:
        return []
    out = []
    if row["holes"]:
        out.append(f"{row['holes']} hole(s)")
    if row["components"] > 1:
        out.append(f"{row['components']} components")
    if not row["monotone"]:
        out.append("not monotone")
    if row["moved"]:
        out.append(f"saturation moves {row['moved']}")
    if row["local"]:
        out.append("local ≠ global")
    if row["minimal"] > 1 and row["maximal"] > 1:
        out.append(f"{row['minimal']} minimal × {row['maximal']} maximal")
    ov = row.get("overlay", {})
    uns = ov.get("exclusions", (0, 0))[1]
    if uns:
        out.append(f"{uns} unsound exclusion(s)")
    mn, mx = ov.get("minimal"), ov.get("maximal")
    if mn and mx and mn[1] != "none" and mx[1] != "none":
        out.append("the walk misses both lines")
    if mn and mx and mn[2] != "none" and mx[2] != "none":
        out.append("a recorded model on neither line")
    if row["size"] == 0 and row["owing"]:
        out.append("R empty, owing")
    return out


def twin_key(row: dict) -> tuple:
    return (row["n"], row["size"], row["minimal"], row["maximal"], row["holes"], row["owing"], row["dead"])


def right_answer(page_dir: Path, file: str, bin: str) -> list[str]:
    """A synthetic program's right answer, by construction: each node of R's
    two lines as the program plus its facts, and each drawn as a KB."""
    right = page_dir / "right"
    inv = run("solution", ["python3", "-I", str(REPO / "utils/region_synth.py"), "--solution", file,
                           "--out", str(right)])
    files = [Path(l) for l in inv.stdout.split() if l.endswith(".ein")]
    jobs, names = [], []
    for f in files:
        k = run("kb", [bin, "render", "kb", str(f)], timeout=60)
        if k.code == 0:
            jobs.append(SvgJob(k.stdout, f.with_suffix(".svg")))
            names.append(f)
    res = render_svgs(jobs)
    lines = ["## The right answer, by construction", "",
             "Each node of R's two lines as the program plus that node's facts — a file that loads and saturates to "
             "the node's state (`utils/region_synth.py --solution`), drawn by `ein render kb`. Written from the target, "
             "not by the engine: the engine's own answer is the run below.", ""]
    for f, r in zip(names, res):
        lines += [f"### {f.parent.name} line: `{f.stem}`", "", f"[`{f.name}`]({rel(f, page_dir / 'README.md')})", ""]
        if r.ok:
            lines += [f"![{f.stem}]({rel(r.svg, page_dir / 'README.md')})", ""]
    return lines


def cmd_regions(out: Path, bin: str) -> int:
    from review.case import program_case
    out.mkdir(parents=True, exist_ok=True)
    with tempfile.TemporaryDirectory(prefix="ein-regions-") as tmp:
        rows = survey(bin, Path(tmp))
    write(out / "survey.json", json.dumps(rows, indent=1, ensure_ascii=False) + "\n")
    have = [r for r in rows if r.get("size") is not None]
    synth = [r for r in have if r["path"].startswith("examples/lattice/synth/")]
    biggest, seen = [], set()
    for r in sorted(have, key=lambda r: (-r["size"], -r["n"], r["path"])):
        if r in synth or twin_key(r) in seen:
            continue
        seen.add(twin_key(r))
        biggest.append(r)
        if len(biggest) == BIGGEST:
            break
    interesting, seen = [], set()
    for r in sorted(have, key=lambda r: (-len(tags(r)), r["path"])):
        if r in synth or r in biggest or len(tags(r)) < 2 or twin_key(r) in seen:
            continue
        seen.add(twin_key(r))
        interesting.append(r)

    pages: dict[str, Path] = {}
    for r in synth + biggest + interesting:
        flags = list(r["flags"]) + (["--region-limit", str(r["limit"]), "-j", "16"] if r["limit"] else [])
        page_dir = out / "programs" / r["path"].removesuffix(".ein")
        page, _ = program_case(r["path"], flags, page_dir, bin, events=False,
                               timeout_note="The region gallery (S1e2.1a.8).")
        if r in synth:
            extra = right_answer(page_dir, r["path"], bin)
            text = read(page)
            write(page, text.replace("## The models", "\n".join(extra) + "\n## The engine's models", 1))
        pages[r["path"]] = page
        print(f"{r['path']:60s} {r.get('verdict')} k={r.get('k')}  {', '.join(tags(r))}")

    index = out / "README.md"
    def link(r):
        p = pages.get(r["path"])
        return f"[`{r['path']}`]({rel(p, index)})" if p else f"`{r['path']}`"
    def img(r):
        p = pages.get(r["path"])
        if not p:
            return []
        svg = p.parent / "runs" / "run" / "img" / "region" / "region.svg"
        return [f"![{r['path']}]({rel(svg, index)})", ""] if svg.exists() else []
    def lines_cell(r):
        ov = r.get("overlay", {})
        cell = []
        for k in ("minimal", "maximal"):
            if k in ov:
                cell.append(f"{k[:3]}: missing {ov[k][1]}")
        return "<br>".join(cell)
    head = ["program", "n", "|R|", "min", "max", "holes", "comp.", "owing", "dead", "moved", "monotone",
            "engine", "the walk against R", "why"]
    def row_of(r):
        if r.get("size") is None:
            return [link(r), r["n"] if r["n"] is not None else "—", "—", "", "", "", "", "", "", "", "", "", r["refused"], ""]
        return [link(r), r["n"], r["size"], r["minimal"], r["maximal"], r["holes"], r["components"], r["owing"],
                r["dead"], r["moved"], "yes" if r["monotone"] else "**no**", f"{r['verdict']} k={r['k']}",
                lines_cell(r), ", ".join(tags(r))]

    lines = ["# The region gallery", "",
             f"> Every corpus program's region — {len(have)} computed, {len(rows) - len(have)} not — and a page for the ones "
             f"worth reading. Built by `python3 utils/review_bundle.py --regions` (M1e2 S1e2.1a.8). The legend is "
             f"[§ Region rendering]({doc('docs/kernel/ir/03-ein-lang/04_dot_rendering.md', index)}#region-rendering--the-cube-and-the-walk-s1e21a2); "
             f"the raw survey is [`survey.json`](survey.json). Each program ran at its corpus entry's declared "
             f"`solve -e` flags where it has them, else its first `solve`, so *the walk* is that run's.", ""]
    for title, what, group in [
            ("Synthetic — written for their region",
             "Programs `utils/region_synth.py` wrote for a chosen region (`examples/lattice/synth/`). The oracle is held "
             "to each target node for node; each page also carries the right answer, by construction, as files.", synth),
            ("The biggest", f"The {BIGGEST} largest regions, twins merged (same n, |R|, lines and counts).", biggest),
            ("The most interesting", "Every other region with two or more of: holes, several components, not monotone, "
             "saturation moves, local ≠ global, several nodes on both lines, an unsound exclusion, a walk that misses "
             "both lines, a recorded model on neither line.", interesting)]:
        lines += [f"## {title}", "", what, ""]
        lines += table(head, [row_of(r) for r in group]) + [""]
        for r in group:
            lines += [f"### {r['path']}", "", ", ".join(tags(r)) or "—", ""] + img(r)
    lines += ["## Every region", "", "Sorted by |R|. *moved* counts alive nodes saturation takes to a larger one.", ""]
    lines += table(head, [row_of(r) for r in sorted(rows, key=lambda r: (-(r.get("size") or -1), r["path"]))])
    write(index, "\n".join(lines) + "\n")
    print(f"{index} — {len(pages)} program pages")
    return 0
