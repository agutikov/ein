"""The region gallery — M1e2 S1e2.1a.8.

    utils/review_bundle.py --regions [--out DIR]       default build/review/regions/

Every corpus program's region, surveyed, and a page for the ones worth
reading: the **synthetic** programs `utils/region_synth.py` wrote for a chosen
region, the **biggest** regions, and the **most interesting**, each one named
for why. The survey runs `ein oracle FILE --dump` on each program and reads the
numbers off `region.json`: the script sorts and links, and computes no region
of its own. A program's page runs its corpus entry's first declared `solve`
(or plain `solve`, which describes R), and its § The region draws that run on
the cube (S1e2.1a.7). Every mode is held to the oracle by
`ein-infer/tests/explore.rs`; the page's drawing and its cross-check 7 say
what this run did.

    DIR/README.md                       the gallery: the three sections, then every region
    DIR/programs/<path>/README.md       one program, every artefact (utils/review_bundle.py's program page)
    DIR/programs/<path>/right/          synthetic only: each line node as a loadable program, drawn
"""

from __future__ import annotations

import json
import shutil
import tempfile
import tomllib
from pathlib import Path

from review.run import solve_flags
from review.util import REPO, SvgJob, doc, load_json, read, rel, render_svgs, run, table, write

# Raised `--region-limit`s: the corpus's largest cubes under a minute each
# (measured 2026-10-15 on 16 jobs). The n = 18 pair, `branching/10` and `/11`,
# is not here: it ran past 11 minutes on 8 jobs, its per-node cost growing with
# the commitment far past the 64-node projection.
BIG = {"examples/syntax/arg-kinds.ein": 19, "examples/saturation/square-bwd/houses.ein": 20}
SURVEY_TIMEOUT = 120
BIGGEST = 8


def corpus_entries() -> list[tuple[str, list[str]]]:
    """Every corpus entry with the flags of its first declared `solve` run,
    else none — the cube does not depend on them, the program page's run does."""
    corpus = tomllib.loads((REPO / "corpus/corpus.toml").read_text(encoding="utf-8"))
    out = []
    for e in corpus["entry"]:
        if e.get("slow"):
            continue
        solves = [r for r in e.get("runs", []) if r.split()[0] == "solve" and "{out}" not in r]
        out.append((e["path"], solve_flags(solves[0]) if solves else []))
    return out


def survey(bin: str, tmp: Path) -> list[dict]:
    rows = []
    for path, flags in corpus_entries():
        d = tmp / path.replace("/", "_")
        limit = BIG.get(path)
        argv = [bin, "oracle", path, "--dump", str(d), "-j", "8"]
        if limit:
            argv += ["--region-limit", str(limit)]
        inv = run("survey", argv, timeout=SURVEY_TIMEOUT)
        region = load_json(d / "region.json")
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
        row.update(
            size=r["size"], minimal=len(r["minimal"]), maximal=len(r["maximal"]), holes=len(r["holes"]),
            components=len(r["components"]),
            subcubes=len(r["subcubes"]) if r["subcubes"] is not None else None,
            owing=sum(x["status"] == "owing" for x in nodes),
            dead=sum(x["status"].startswith("dead") for x in nodes),
            moved=sum(not x["status"].startswith("dead") and x.get("closure", x["mask"]) != x["mask"] for x in nodes),
            local=len(r["local_max"]) + len(r["local_min"]),
            monotone=region["monotonicity"]["monotone"])
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
        print(f"{r['path']:60s} |R|={r.get('size')}  {', '.join(tags(r))}")

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
    head = ["program", "n", "|R|", "min", "max", "holes", "comp.", "owing", "dead", "moved", "monotone", "why"]
    def row_of(r):
        if r.get("size") is None:
            return [link(r), r["n"] if r["n"] is not None else "—", "—", "", "", "", "", "", "", "", "", r["refused"]]
        return [link(r), r["n"], r["size"], r["minimal"], r["maximal"], r["holes"], r["components"], r["owing"],
                r["dead"], r["moved"], "yes" if r["monotone"] else "**no**", ", ".join(tags(r))]

    lines = ["# The region gallery", "",
             f"> Every corpus program's region — {len(have)} computed, {len(rows) - len(have)} not — and a page for the ones "
             f"worth reading. Built by `python3 utils/review_bundle.py --regions` (M1e2 S1e2.1a.8). The legend is "
             f"[§ Region rendering]({doc('docs/kernel/ir/03-ein-lang/04_dot_rendering.md', index)}#region-rendering--the-cube-and-a-region-run); "
             f"the raw survey is [`survey.json`](survey.json). Each region is `ein oracle --dump`'s; each program "
             f"page runs the corpus entry's first declared `solve`.", ""]
    for title, what, group in [
            ("Synthetic — written for their region",
             "Programs `utils/region_synth.py` wrote for a chosen region (`examples/lattice/synth/`). The oracle is held "
             "to each target node for node; each page also carries the right answer, by construction, as files.", synth),
            ("The biggest", f"The {BIGGEST} largest regions, twins merged (same n, |R|, lines and counts).", biggest),
            ("The most interesting", "Every other region with two or more of: holes, several components, not monotone, "
             "saturation moves, local ≠ global, several nodes on both lines.", interesting)]:
        lines += [f"## {title}", "", what, ""]
        lines += table(head, [row_of(r) for r in group]) + [""]
        for r in group:
            lines += [f"### {r['path']}", "", ", ".join(tags(r)) or "—", ""] + img(r)
    lines += ["## Every region", "", "Sorted by |R|. *moved* counts alive nodes saturation takes to a larger one.", ""]
    lines += table(head, [row_of(r) for r in sorted(rows, key=lambda r: (-(r.get("size") or -1), r["path"]))])
    write(index, "\n".join(lines) + "\n")
    print(f"{index} — {len(pages)} program pages")
    return 0
