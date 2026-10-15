"""One run, fully explicit — M1e2 S1e2.1.5.

`build_run` drives every artefact `ein` can write for one `(file, flags, env)`,
keeps each verbatim under `raw/`, and renders it as pages a person reads
without decoding anything: every graph an SVG, every JSON a table, every
snapshot pair a diff.

    DIR/run.md            what was run, the answer, and every page with its size
    DIR/raw/              everything the engine wrote, never edited
    DIR/img/              every DOT above and every ```dot block of the trace, as SVG
    DIR/pages/            program, root, trace/, states/, lattice/, region/, answer,
                          summary, events/, consistency

A solve has **one dumper**, so the run is several invocations: `main` carries
every flag that can share a solve (`--trace --full-kb-snapshots`,
`--dump-states`, `--dump-answer`, `--dump-region`, `--events`,
`--json-summary`, and the stdout diagnostics), `lattice` carries
`--dump-lattice`, and `grid` / `key` the two model forms. `--dump-region`'s
three files are kept under `raw/region/` and drawn by `ein render region`
(S1e2.1a.2) into `pages/region/`; the case page shows each run's drawing, and
its § The region in R's vocabulary is S1e2.1a.7's. Their answers have to be
the same answer, which is cross-check 1.
"""

from __future__ import annotations

import json
import re
from collections import Counter, OrderedDict, defaultdict
from dataclasses import dataclass, field
from pathlib import Path

from review.util import (REPO, Invocation, SvgJob, details, doc, header, image, jsonl, load_json,
                         read, rebase, rel, render_svgs, run, size_of, split_digraphs, table, write)
from review.rasterise import rasterise_text

STEPS_PER_PAGE = 50
ROWS_PER_PAGE = 500
REFUTED_PER_PAGE = 100
# Event rows rendered per kind. `branching/06 -e` narrates 2.2 million verbose
# events, nearly all of one kind; thousands of pages of `fire` rows are not a
# thing a reviewer reads, and building them took longer than the run. Past
# this, a kind's page says how many rows there are, which it shows (the first
# and the last), and that `raw/events.jsonl` holds every one.
EVENT_ROWS_PER_KIND = 10_000


@dataclass
class RunSpec:
    """What to run: `ein solve <file> <flags…>` under `env`."""

    file: str
    flags: list[str] = field(default_factory=list)
    env: dict[str, str] = field(default_factory=dict)
    title: str = ""
    events: bool = True
    trace: bool = True
    # A model set to compare the answer with (`--diff-models`): the case page
    # writes one from the register's `right.models` (S1e2.1.6).
    diff_models: Path | None = None


@dataclass
class RunResult:
    out: Path
    spec: RunSpec
    invocations: list[Invocation]
    summary: dict | None
    checks: list[tuple[str, bool, str]]
    pages: list[tuple[str, Path, str]]
    verdict: str = ""
    k: int | None = None
    exhausted: bool | None = None


def tree(spec: RunSpec) -> bool:
    return spec.env.get("EIN_TRAVERSAL") == "tree"


def max_set_size(flags: list[str]) -> str | None:
    for i, f in enumerate(flags):
        if f in ("-m", "--max-set-size") and i + 1 < len(flags):
            return flags[i + 1]
        if f.startswith("--max-set-size="):
            return f.split("=", 1)[1]
    return None


# ── driving ───────────────────────────────────────────────────────────────────

def drive(spec: RunSpec, out: Path, bin: str) -> list[Invocation]:
    raw = out / "raw"
    raw.mkdir(parents=True, exist_ok=True)
    solve = [bin, "solve", spec.file, *spec.flags]
    main = solve + ["--json-summary", str(raw / "summary.json"),
                    "--dump-states", str(raw / "states"),
                    "--dump-answer", str(raw / "answer"),
                    "--dump-region", str(raw / "region"),
                    "--stats", "--timing", "--hyp-stats", "--dump-config", "-p"]
    if spec.trace:
        main += ["--trace", str(raw / "trace.md"), "--full-kb-snapshots"]
    if spec.events:
        main += ["--events", str(raw / "events.jsonl"), "--events-level", "verbose"]
    invs = [run("main", main, spec.env)]
    if not tree(spec):
        invs.append(run("lattice", solve + ["--dump-lattice", str(raw / "lattice"),
                                            "--json-summary", str(raw / "summary.lattice.json")],
                        spec.env))
    invs.append(run("grid", solve + ["--models", "grid",
                                     "--json-summary", str(raw / "summary.grid.json")], spec.env))
    invs.append(run("key", solve + ["--models", "key",
                                    "--json-summary", str(raw / "summary.key.json")], spec.env))
    if spec.diff_models:
        invs.append(run("diff", solve + ["--diff-models", str(spec.diff_models),
                                         "--json-summary", str(raw / "summary.diff.json")], spec.env))
    m = max_set_size(spec.flags)
    lattice = [bin, "render", "lattice", spec.file] + (["--max-set-size", m] if m else [])
    for name, argv in [("render-program", [bin, "render", "program", spec.file]),
                       ("render-rules", [bin, "render", "rules", spec.file]),
                       ("render-constraints", [bin, "render", "constraints", spec.file]),
                       ("render-lattice", lattice)]:
        invs.append(run(name, argv, spec.env, timeout=120))
    # The region drawn from the directory the main run wrote (S1e2.1a.2):
    # `auto`, the lines (the view that reads at any n) when `auto` is not
    # already them, and every node as a table.
    if (raw / "region" / "region.json").exists():
        region = str(raw / "region")
        auto = run("render-region", [bin, "render", "region", region], timeout=120)
        invs.append(auto)
        said = auto.stdout.split("\n", 2)[1] if auto.stdout.count("\n") > 1 else ""
        if auto.code == 0 and "view lines" not in said and "view none" not in said:
            invs.append(run("render-region-lines", [bin, "render", "region", region, "--view", "lines"],
                            timeout=120))
        invs.append(run("render-region-table", [bin, "render", "region", region, "--view", "table"],
                        timeout=120))
    answer = raw / "answer"
    if (answer / "root.ein").exists():
        invs.append(run("render-root", [bin, "render", "kb", str(answer / "root.ein")]))
        index = load_json(answer / "index.json") or {}
        for stem in index.get("models", []) + index.get("open", []):
            invs.append(run(f"render-{stem.replace('/', '-')}",
                            [bin, "render", "kb", str(answer / f"{stem}.ein"),
                             "--since", str(answer / "root.ein")]))
    # Every layer's root after it, with what the layer added in bold — the
    # snapshots load as programs since M1e2 S1e2.1.7.
    for post in sorted((raw / "states" / "layers").glob("layer_*_post.ein")):
        pre = post.with_name(post.name.replace("_post", "_pre"))
        argv = [bin, "render", "kb", str(post)] + (["--since", str(pre)] if pre.exists() else [])
        invs.append(run(f"render-{post.stem}", argv, timeout=120))
    version = run("version", [bin, "--version"])
    commit = run("git", ["git", "rev-parse", "HEAD"])
    dirty = run("git-status", ["git", "status", "--porcelain"])
    for inv in invs:
        write(raw / f"{inv.name}.stdout.txt", inv.stdout)
        write(raw / f"{inv.name}.stderr.txt", inv.stderr)
        write(raw / f"{inv.name}.exit.txt", f"{inv.code}\n")
    lines = [f"# {spec.title or spec.file}", "",
             f"commit: {commit.stdout.strip()}{' (working tree dirty)' if dirty.stdout.strip() else ''}",
             "", "ein --version:", version.stdout.rstrip(), ""]
    for inv in invs:
        lines.append(f"[{inv.name}] exit {inv.code}: {inv.shell()}")
    write(raw / "command.txt", "\n".join(lines) + "\n")
    return invs


# ── pages ─────────────────────────────────────────────────────────────────────

class Pages:
    """The page set of one run, and the SVG queue they share."""

    def __init__(self, out: Path, spec: RunSpec, invs: list[Invocation]):
        self.out, self.spec = out, spec
        self.raw, self.img, self.dir = out / "raw", out / "img", out / "pages"
        self.inv = {i.name: i for i in invs}
        self.pages: list[tuple[str, Path, str]] = []
        self.checks: list[tuple[str, bool, str]] = []
        self.refused: list[str] = []

    def add(self, title: str, path: Path, lines: list[str], note: str = "") -> Path:
        write(path, "\n".join(lines).rstrip() + "\n")
        self.pages.append((title, path, note))
        return path

    def svgs(self, jobs: list[SvgJob]):
        res = render_svgs(jobs)
        self.refused += [f"{r.svg.with_suffix('.dot').name}: {r.error}" for r in res if not r.ok]
        return res

    def dot_stream(self, text: str, sub: str) -> list:
        """A multi-digraph stdout as one SVG per digraph."""
        blocks = split_digraphs(text)
        return list(zip([n for n, _ in blocks],
                        self.svgs([SvgJob(b, self.img / sub / f"{i + 1:02d}_{_safe(n)}.svg")
                                   for i, (n, b) in enumerate(blocks)])))


def paged(p: "Pages", stem: Path, title: str, head_lines, head: list[str], rows: list[list],
          per: int = ROWS_PER_PAGE) -> list[Path]:
    """A table too long for one page, as pages of `per` rows. `head_lines(page,
    note)` writes each page's header; the pages link each other."""
    chunks = [rows[i:i + per] for i in range(0, max(len(rows), 1), per)]
    paths = [stem.with_suffix(".md") if len(chunks) == 1 else stem.with_name(f"{stem.name}-{i + 1:02d}.md")
             for i in range(len(chunks))]
    for i, (chunk, path) in enumerate(zip(chunks, paths)):
        nav = " · ".join(f"[{j + 1}]({rel(q, path)})" if q != path else f"**{j + 1}**" for j, q in enumerate(paths))
        note = f"rows {i * per + 1}–{i * per + len(chunk)} of {len(rows)}" + (f" · pages {nav}" if len(paths) > 1 else "")
        p.add(f"{title}" + (f" {i + 1}" if len(paths) > 1 else ""), path,
              head_lines(path, note) + (table(head, chunk) if chunk else ["_none_"]))
    return paths


def _safe(s: str) -> str:
    """A file-name-safe form of `s`, unique: a name cut short carries a hash
    of the whole, because two commitments' slugs can share sixty characters
    (`pets.ein` has 32 that do)."""
    import hashlib
    t = re.sub(r"[^A-Za-z0-9_.-]+", "_", s) or "x"
    return t if len(t) <= 60 else t[:51] + "-" + hashlib.sha1(s.encode()).hexdigest()[:8]


def fact_key(line: str) -> str:
    """A fact line of a dump without its provenance keywords — what a
    snapshot diff compares, because one fact's `:rule` can differ between two
    snapshots of the same KB."""
    return re.sub(r"\s+:(rule|source|hypothesis|using)\s+(\"[^\"]*\"|\S+)", "", line.strip())


def snapshot_facts(path: Path) -> list[str]:
    return sorted({fact_key(l) for l in read(path).splitlines() if l.strip().startswith("(")})


def by_relation(facts) -> "OrderedDict[str, list[str]]":
    out: dict[str, list[str]] = defaultdict(list)
    for f in facts:
        m = re.match(r"\(\s*(\S+)", f)
        out[m.group(1).rstrip(")") if m else "?"].append(f)
    return OrderedDict(sorted(out.items()))


def commit_text(c) -> str:
    """A `{relation, args}` commitment, as `rel(a, b)` joined."""
    if not c:
        return "∅"
    def one(f):
        args = ", ".join(json.dumps(a) if isinstance(a, dict) else str(a) for a in f.get("args", []))
        return f"{f.get('relation')}({args})"
    return ", ".join(one(f) for f in c)


def verdict_block(stdout: str) -> str:
    """The solve table, verbatim: from `solve ·` to the blank line before
    whatever diagnostic follows it."""
    lines = stdout.splitlines()
    try:
        start = next(i for i, l in enumerate(lines) if l.startswith("solve ·"))
    except StopIteration:
        return stdout.strip()
    end = len(lines)
    for i in range(start + 1, len(lines)):
        if re.match(r"^(final-state facts|unsat-core facts|stats$|timing|── solution)", lines[i]):
            end = i
            break
    return "\n".join(lines[start:end]).rstrip()


def diff_block(inv: Invocation) -> str:
    """`--diff-models`' block, verbatim: from `models against` to the blank
    line that ends it — or the refusal, when the file did not parse."""
    lines = inv.stdout.splitlines()
    start = next((i for i, l in enumerate(lines) if l.startswith("models against")), None)
    if start is None:
        return inv.stderr.strip() or "(no diff printed)"
    end = next((i for i in range(start, len(lines)) if not lines[i].strip()), len(lines))
    return "\n".join(lines[start:end])


def program_page(p: Pages) -> None:
    page = p.dir / "program.md"
    src = REPO / p.spec.file
    lines = header("The program", "the source, and three views of it drawn before anything is loaded",
                   "`ein render program`, `ein render rules`, `ein render constraints`",
                   [("04_dot_rendering.md", "docs/kernel/ir/03-ein-lang/04_dot_rendering.md")],
                   [src, p.raw / "render-program.stdout.txt", p.raw / "render-rules.stdout.txt",
                    p.raw / "render-constraints.stdout.txt"], page)
    lines += ["## Source", "", f"`{p.spec.file}` — {size_of(src)}", "", "```lisp", read(src).rstrip(), "```", ""]
    for title, inv, sub in [("The program as written (`render program`)", "render-program", "program"),
                            ("Every rule (`render rules`)", "render-rules", "rules"),
                            ("Constraint scopes (`render constraints`)", "render-constraints", "constraints")]:
        lines += [f"## {title}", ""]
        i = p.inv.get(inv)
        if i is None or i.code != 0:
            lines += [f"_exit {i.code if i else '—'}: {(i.stderr.strip() if i else 'not run') or 'nothing printed'}_", ""]
            continue
        for name, svg in p.dot_stream(i.stdout, sub):
            lines += [f"### `{name}`", "", *image(svg, page, name)]
    p.add("Program", page, lines)


def root_page(p: Pages, summary: dict | None, steps: list[dict]) -> None:
    page = p.dir / "root.md"
    lines = header("Root, after saturation", "the KB before any hypothesis: what the program derives on its own",
                   "`--dump-answer` (`root.ein`), `ein render kb root.ein`, the trace's § Before any assumption",
                   [("answer_dump.md", "docs/kernel/inference/answer_dump.md")],
                   [p.raw / "answer" / "root.ein", p.raw / "answer" / "root_writes.json"], page)
    root = (summary or {}).get("root", {})
    if root:
        lines += ["## Facts by relation", "", f"From `summary.json`'s `root` block — {root.get('n_facts', '?')} facts.", ""]
        lines += table(["relation", "facts"], [[r, n] for r, n in (root.get("by_relation") or {}).items()])
        lines.append("")
    inv = p.inv.get("render-root")
    if inv and inv.code == 0:
        lines += ["## The KB (`ein render kb root.ein`)", ""]
        for name, svg in p.dot_stream(inv.stdout, "root"):
            lines += image(svg, page, "root")
    writes = load_json(p.raw / "answer" / "root_writes.json") or []
    lines += ["## What the search wrote into root afterwards", "",
              "`root_writes.json`: every fact the final root holds and the saturated one did not, with the rule its provenance names.", ""]
    lines += table(["fact", "written by"], [[w.get("fact"), w.get("by") or "—"] for w in writes]) if writes else ["_none_"]
    lines.append("")
    uncond = [s for s in steps if s["unconditional"]]
    if uncond:
        lines += [f"## Before any assumption — {len(uncond)} steps (from the trace)", ""]
        lines += table(["step", "rule", "what it did", "page"],
                       [[s["n"], f"`{s['rule']}`", s["gist"], f"[step {s['n']}]({rel(s['page'], page)}#{s['anchor']})"]
                        for s in uncond])
    p.add("Root", page, lines)


# ── the trace ─────────────────────────────────────────────────────────────────

def split_trace(text: str) -> tuple[str, list[tuple[str, str]]]:
    parts = re.split(r"(?m)^## ", text)
    pre, out = parts[0], []
    for part in parts[1:]:
        title, _, body = part.partition("\n")
        out.append((title.strip(), body))
    return pre, out


def anchor_of(title: str) -> str:
    a = re.sub(r"[^\w\- ]", "", title.lower()).strip().replace(" ", "-")
    return a


def trace_pages(p: Pages) -> list[dict]:
    raw = p.raw / "trace.md"
    tdir = p.dir / "trace"
    text = read(raw)
    if not text:
        p.add("Trace", tdir / "index.md", header("The trace", "not written", "`--trace`", [], [raw], tdir / "index.md"))
        return []
    pre, sections = split_trace(text)
    defined = [("solution_semantics.md", "docs/kernel/inference/solution_semantics.md")]
    uncond_n = 0
    steps: list[dict] = []
    others: list[tuple[str, str]] = []
    for title, body in sections:
        m = re.match(r"Before any assumption — (\d+) steps", title)
        if m:
            uncond_n = int(m.group(1))
            continue
        m = re.match(r"Step (\d+) — `([^`]+)`", title)
        if m:
            n = int(m.group(1))
            gist = next((l[2:].strip() for l in body.splitlines() if l.startswith("> ")), "")
            steps.append({"n": n, "rule": m.group(2), "title": title, "body": body, "gist": gist,
                          "unconditional": len(steps) < uncond_n, "anchor": anchor_of(f"Step {n} — {m.group(2)}")})
        else:
            others.append((title, body))
    block = 0
    # Step pages, 50 to a page, every dot block an SVG.
    for start in range(0, len(steps), STEPS_PER_PAGE):
        chunk = steps[start:start + STEPS_PER_PAGE]
        name = f"steps-{chunk[0]['n']:03d}-{chunk[-1]['n']:03d}.md"
        page = tdir / name
        md = "".join(f"## {s['title']}\n{s['body']}" for s in chunk)
        body, n, refused = rasterise_text(md, p.img / "trace", page, start=block)
        block += n
        p.refused += refused
        for s in chunk:
            s["page"] = page
        lines = header(f"Trace — steps {chunk[0]['n']}–{chunk[-1]['n']}",
                       "the engine's narration of each firing, with its derivation graph",
                       "`--trace FILE --full-kb-snapshots`", defined, [raw], page,
                       f"**Steps {chunk[0]['n']}–{chunk[-1]['n']}** of {len(steps)}; "
                       f"{uncond_n} are before any assumption. [Index](index.md)")
        p.add(f"Trace steps {chunk[0]['n']}–{chunk[-1]['n']}", page, lines + [body])
    # The named sections, each a page — Refuted paged by its <details> blocks.
    for title, body in others:
        slug = {"Refuted hypotheses": "refuted", "Commitment lattice": "lattice",
                "Solution": "solution", "Full KB (final state)": "full-kb"}.get(title, _safe(title).lower())
        chunks = [body]
        if slug == "refuted":
            entries = re.split(r"(?m)^(?=<details>)", body)
            head, entries = entries[0], entries[1:]
            chunks = [head + "".join(entries[i:i + REFUTED_PER_PAGE])
                      for i in range(0, max(len(entries), 1), REFUTED_PER_PAGE)]
        for ci, chunk in enumerate(chunks):
            name = f"{slug}.md" if len(chunks) == 1 else f"{slug}-{ci + 1:02d}.md"
            page = tdir / name
            body_r, n, refused = rasterise_text(chunk, p.img / "trace", page, start=block)
            block += n
            p.refused += refused
            lines = header(f"Trace — {title}" + (f" ({ci + 1}/{len(chunks)})" if len(chunks) > 1 else ""),
                           f"the trace's § {title}", "`--trace FILE --full-kb-snapshots`", defined, [raw], page,
                           "[Index](index.md)")
            p.add(f"Trace: {title}" + (f" {ci + 1}" if len(chunks) > 1 else ""), page, lines + [body_r])
    # The index: the preamble, then one row per step.
    page = tdir / "index.md"
    lines = header("The trace", "the engine's *explanation* of the run — what it chose to narrate, with `:why` text",
                   "`--trace FILE --full-kb-snapshots`", defined, [raw], page,
                   f"**{len(steps)} steps** ({uncond_n} before any assumption), **{block} graphs**, "
                   f"{len(others)} named sections")
    lines += [pre.replace("# Solution trace", "").strip(), ""]
    lines += ["## Sections", ""] + [f"- [{t}]({anchor})" for t, anchor in
                                    [(t, f"{({'Refuted hypotheses': 'refuted', 'Commitment lattice': 'lattice', 'Solution': 'solution', 'Full KB (final state)': 'full-kb'}.get(t, _safe(t).lower()))}"
                                      + (".md" if not (t == "Refuted hypotheses" and (tdir / "refuted-01.md").exists()) else "-01.md"))
                                     for t, _ in others]] + [""]
    lines += ["## Every step", ""]
    lines += table(["step", "", "rule", "what it did"],
                   [[f"[{s['n']}]({rel(s['page'], page)}#{s['anchor']})",
                     "root" if s["unconditional"] else "assumed", f"`{s['rule']}`", s["gist"]] for s in steps])
    p.add("Trace index", page, lines)
    p.trace_blocks = block
    return steps


# ── the state dump ────────────────────────────────────────────────────────────

def lattice_index(p: Pages) -> dict[str, Path]:
    """`--dump-lattice`'s per-entering folders, keyed by their commitment."""
    out = {}
    for f in (p.raw / "lattice" / "enterings").rglob("commitment.json"):
        c = load_json(f)
        layer = f.parent.parent.name
        out[layer + "|" + json.dumps(c, sort_keys=True)] = f.parent
    return out


def lattice_page_for(p: Pages, folder: Path) -> Path:
    return p.dir / "lattice" / folder.parent.name / (_safe(folder.name) + ".md")


def states_pages(p: Pages, lat: dict[str, Path]) -> list[dict]:
    sdir = p.dir / "states"
    raw = p.raw / "states"
    events = jsonl(raw / "00_timeline.jsonl")
    defined = [("lattice_dump.md", "docs/kernel/inference/lattice_dump.md"),
               ("inference/README.md § Diagnostics", "docs/kernel/inference/README.md")]
    keys: list[str] = []
    for e in events:
        for k in e:
            if k not in keys:
                keys.append(k)
    def cell(e, k):
        if k not in e:
            return ""
        v = e[k]
        if k == "commitment":
            text = commit_text(v)
            key = f"layer_{int(e.get('layer', 0)):02d}|" + json.dumps(v, sort_keys=True)
            if key in lat:
                return f"[{text}]({rel(lattice_page_for(p, lat[key]), page)})"
            return text
        return json.dumps(v) if isinstance(v, (dict, list)) else v
    for start in range(0, max(len(events), 1), ROWS_PER_PAGE):
        chunk = events[start:start + ROWS_PER_PAGE]
        name = "timeline.md" if len(events) <= ROWS_PER_PAGE else f"timeline-{start // ROWS_PER_PAGE + 1:02d}.md"
        page = sdir / name
        lines = header("The state dump's timeline", "`00_timeline.jsonl`: every layer boundary and every entering, in order",
                       "`--dump-states DIR`", defined, [raw / "00_timeline.jsonl"], page,
                       f"rows {start + 1}–{start + len(chunk)} of {len(events)}; an entering links to its "
                       "`--dump-lattice` page")
        lines += table(keys, [[cell(e, k) for k in keys] for e in chunk])
        p.add("States: timeline" + (f" {start // ROWS_PER_PAGE + 1}" if len(events) > ROWS_PER_PAGE else ""), page, lines)
    # Per layer: what changed in root, and every entering.
    layers = sorted({int(m.group(1)) for f in (raw / "layers").glob("layer_*_*.ein")
                     if (m := re.match(r"layer_(\d+)_", f.name))})
    for n in layers:
        page = sdir / f"layer-{n:02d}.md"
        pre, post = raw / "layers" / f"layer_{n:02d}_pre.ein", raw / "layers" / f"layer_{n:02d}_post.ein"
        a, b = snapshot_facts(pre), snapshot_facts(post)
        added, removed = sorted(set(b) - set(a)), sorted(set(a) - set(b))
        start = next((e for e in events if e.get("event") == "layer_start" and e.get("layer") == n), {})
        entered = [e for e in events if e.get("event") == "entering" and e.get("layer") == n]
        lines = header(f"Layer {n}", "root before and after the layer, and every commitment the layer entered",
                       "`--dump-states DIR` (`layers/layer_NN_{pre,post}.ein`)", defined, [pre, post], page,
                       f"alive at the start: **{start.get('alive_size', '?')}**; entered: **{len(entered)}**")
        lines += ["## Entered", ""]
        entered_rows = [[cell(e, "commitment"), e.get("outcome", ""), e.get("firings", ""),
                         e.get("facts_merged", ""), e.get("unsat_core_size", "")] for e in entered]
        head = ["commitment", "outcome", "firings", "facts merged", "core size"]
        if len(entered_rows) <= ROWS_PER_PAGE:
            lines += table(head, entered_rows) if entered_rows else ["_none_"]
        else:
            lists = paged(p, sdir / f"layer-{n:02d}-entered", f"States: layer {n} entered",
                          lambda path, note: header(f"Layer {n} — entered", "every commitment the layer entered",
                                                    "`--dump-states DIR`", defined,
                                                    [raw / "00_timeline.jsonl"], path, note),
                          head, entered_rows)
            lines += [f"{len(entered_rows)} — " + ", ".join(f"[page {i + 1}]({rel(x, page)})" for i, x in enumerate(lists))]
        lines += ["", "## Root, pre → post", "",
                  "A **set difference of the two snapshots' fact lines**, provenance keywords ignored — "
                  "computed here, from files the engine wrote.", ""]
        if not (added or removed):
            lines += ["_No fact changed._", ""]
        for title, facts in (("Added", added), ("Removed", removed)):
            if facts:
                lines += [f"### {title} ({len(facts)})", ""]
                for r, fs in by_relation(facts).items():
                    lines += [f"- `{r}` ({len(fs)}): " + ", ".join(f"`{f}`" for f in fs)]
                lines.append("")
        lines += ["## The snapshots", ""]
        inv = p.inv.get(f"render-layer_{n:02d}_post")
        if inv and inv.code == 0:
            for name, svg in p.dot_stream(inv.stdout, f"layer_{n:02d}"):
                lines += ["**Root after the layer, what the layer added in bold**, "
                          "`ein render kb layer_NN_post.ein --since layer_NN_pre.ein`:", "",
                          *image(svg, page, f"layer {n}")]
        elif inv:
            lines += [f"`ein render kb` exited {inv.code} on the post snapshot: "
                      f"`{(inv.stderr.strip().splitlines() or [''])[0]}`", ""]
        for title, path in (("pre", pre), ("post", post)):
            facts = snapshot_facts(path)
            lines += details(f"{title}: {len(facts)} facts",
                             [f"- `{r}` ({len(fs)}): " + ", ".join(f"`{f}`" for f in fs)
                              for r, fs in by_relation(facts).items()])
        p.add(f"States: layer {n}", page, lines)
    return events


def lattice_pages(p: Pages, lat: dict[str, Path]) -> None:
    ldir = p.dir / "lattice"
    raw = p.raw / "lattice"
    defined = [("lattice_dump.md", "docs/kernel/inference/lattice_dump.md")]
    index_page = ldir / "index.md"
    rows = []
    for key, folder in sorted(lat.items(), key=lambda kv: (kv[1].parent.name, kv[1].name)):
        page = lattice_page_for(p, folder)
        commitment = load_json(folder / "commitment.json") or []
        outcome = read(folder / "outcome.txt").strip()
        firings = jsonl(folder / "firings.jsonl")
        core = jsonl(folder / "unsat_core.jsonl")
        lines = header(f"Entering {commit_text(commitment)}", "one commitment the search tested: its outcome and every firing",
                       "`--dump-lattice DIR` (`enterings/layer_NN/<slug>/`)", defined,
                       [folder / n for n in ("commitment.json", "outcome.txt", "firings.jsonl", "unsat_core.jsonl", "kb.ein")
                        if (folder / n).exists()], page,
                       f"{folder.parent.name.replace('_', ' ')} · **{outcome}** · {len(firings)} firings · [all enterings]({rel(index_page, page)})")
        if core:
            lines += ["## Unsat core", ""] + [f"- `{json.dumps(c)}`" for c in core] + [""]
        lines += ["## Firings", ""]
        lines += table(["#", "rule", "derived", "premises", "bindings"],
                       [[i + 1, f.get("rule", ""), json.dumps(f.get("derived", "")), json.dumps(f.get("premises", "")),
                         json.dumps(f.get("bindings", ""))] for i, f in enumerate(firings)]) if firings else ["_none_"]
        p.add(f"Lattice: {commit_text(commitment)}", page, lines)
        rows.append([folder.parent.name, f"[{commit_text(commitment)}]({rel(page, index_page)})", outcome, len(firings)])
    proof = load_json(raw / "proof_summary.json")
    lines = header("Every commitment tested", "`--dump-lattice`'s per-entering tree", "`--dump-lattice DIR`", defined,
                   [raw / "proof_summary.json", raw / "00_timeline.jsonl"], index_page,
                   f"**{len(rows)} enterings**")
    if proof:
        lines += ["## `proof_summary.json`", ""]
        lines += table(["field", "value"], [[k, json.dumps(v) if not isinstance(v, list) else f"{len(v)} items"]
                                            for k, v in proof.items()])
        lines.append("")
    # Rows link from the index's own directory; the list pages sit beside it.
    lists = paged(p, ldir / "enterings", "Lattice: enterings",
                  lambda path, note: header("Enterings", "every commitment `--dump-lattice` recorded, by layer",
                                            "`--dump-lattice DIR`", defined, [raw / "enterings"], path, note),
                  ["layer", "commitment", "outcome", "firings"], rows)
    lines += ["## Enterings", "", f"{len(rows)} — " + ", ".join(f"[page {i + 1}]({rel(x, index_page)})"
                                                                 for i, x in enumerate(lists))]
    p.add("Lattice", index_page, lines)


# ── the region ────────────────────────────────────────────────────────────────

def region_view(inv) -> str:
    """The view a `render region` DOT says it is, from its second line."""
    lines = inv.stdout.split("\n", 2)
    return lines[1].split("view ", 1)[-1].strip() if len(lines) > 1 else "?"


def region_pages(p: Pages) -> None:
    """`--dump-region`'s directory as pages: the drawings, `region.md`, and
    every node by layer — S1e2.1a.2. The script draws and splits; the join of
    the walk to the cube is `ein render region`'s."""
    raw = p.raw / "region"
    if not (raw / "region.json").exists():
        return
    rdir = p.dir / "region"
    index = rdir / "index.md"
    defined = [("region_dump.md", "docs/kernel/inference/region_dump.md"),
               ("04_dot_rendering.md § Region rendering",
                "docs/kernel/ir/03-ein-lang/04_dot_rendering.md")]
    region = load_json(raw / "region.json") or {}
    walk = load_json(raw / "walk.json") or {}
    r = region.get("region") or {}
    n = region.get("n", "?")
    head = (f"n = {n} · |R| = {r.get('size', '—')}"
            + ("" if region.get("monotonicity", {}).get("monotone", True) else " · **not monotone**")
            + f" · walk: **{walk.get('verdict', {}).get('type', '?')}**, k = {walk.get('verdict', {}).get('k', '?')}")
    lines = header("The region", "the lattice of commitments, every node classified by saturation alone, "
                   "with this run's walk on it", "`--dump-region DIR`, `ein render region DIR`", defined,
                   [raw / "region.json", raw / "walk.json", raw / "region.md"], index, head)
    lines += ["Fill is what the cube says, shape is what the search did, border marks the lines and the defects. "
              "Every node's reason is in its tooltip: open the SVG on its own to hover.", ""]
    for name, title in [("render-region", "The drawing"), ("render-region-lines", "The lines")]:
        inv = p.inv.get(name)
        if inv is None:
            continue
        lines += [f"## {title} — `--view {region_view(inv)}`", ""]
        if inv.code != 0:
            lines += [f"_exit {inv.code}: {inv.stderr.strip() or 'nothing printed'}_", ""]
            continue
        (res,) = p.svgs([SvgJob(inv.stdout, p.img / "region" / f"{name.removeprefix('render-')}.svg")])
        lines += image(res, index, f"region — {region_view(inv)}")
    # Every node, by layer, from `--view table`.
    tinv = p.inv.get("render-region-table")
    if tinv and tinv.code == 0:
        pre, sections = split_trace(tinv.stdout)
        lines += ["## Every node", "", *pre.splitlines()[1:], ""]
        rows = []
        for title, body in sections:
            stem = rdir / _safe(title.split(" —")[0].lower())
            tl = [l for l in body.splitlines() if l.startswith("|")]
            head_row, body_rows = tl[:2], tl[2:]
            chunks = [body_rows[i:i + ROWS_PER_PAGE] for i in range(0, max(len(body_rows), 1), ROWS_PER_PAGE)]
            paths = [stem.with_suffix(".md") if len(chunks) == 1 else stem.with_name(f"{stem.name}-{i + 1:02d}.md")
                     for i in range(len(chunks))]
            for i, (chunk, path) in enumerate(zip(chunks, paths)):
                nav = " · ".join(f"[{j + 1}]({rel(q, path)})" if q != path else f"**{j + 1}**" for j, q in enumerate(paths))
                ph = header(f"The region — {title}", "every node of this layer: status, why, where saturation takes it, "
                            "what the walk did", "`ein render region DIR --view table`", defined,
                            [raw / "region.json", raw / "walk.json"], path,
                            f"[the region]({rel(index, path)})" + (f" · pages {nav}" if len(paths) > 1 else ""))
                p.add(f"Region: {title}" + (f" {i + 1}" if len(paths) > 1 else ""), path, ph + head_row + chunk)
            rows.append([f"[{title}]({rel(paths[0], index)})", len(body_rows), len(paths)])
        lines += table(["layer", "nodes", "pages"], rows) + [""]
    md = read(raw / "region.md")
    if md:
        lines += details("`region.md`, as the dump wrote it", [re.sub(r"(?m)^#", "###", md)])
    p.add("Region", index, lines)


# ── the answer ────────────────────────────────────────────────────────────────

def grids_from(stdout: str) -> list[str]:
    """`--models grid`'s per-state grid blocks, in order, verbatim."""
    out, cur = [], None
    for line in stdout.splitlines():
        if line.strip().startswith("grid —"):
            # A blank line between the licence and the table, so a markdown
            # viewer starts a table rather than continuing the paragraph.
            cur = [line.strip(), ""]
            out.append(cur)
        elif cur is not None and line.strip().startswith(("|", "(no ")):
            cur.append(line.strip())
        else:
            cur = None
    return ["\n".join(g) for g in out]


def answer_pages(p: Pages) -> None:
    raw = p.raw / "answer"
    page = p.dir / "answer.md"
    index = load_json(raw / "index.json") or {}
    defined = [("answer_dump.md", "docs/kernel/inference/answer_dump.md"),
               ("solution_semantics.md", "docs/kernel/inference/solution_semantics.md")]
    lines = header("The answer", "every recorded state as the files `--dump-answer` wrote: grid, graph, commitment, what it added",
                   "`--dump-answer DIR`, `--models grid`, `--models key`, `ein render kb … --since root.ein`",
                   defined, [raw / "index.json", raw / "answer.json", p.raw / "grid.stdout.txt", p.raw / "key.stdout.txt"], page,
                   f"**{index.get('verdict', '?')}**, k = {index.get('k', '?')}, exhausted = {index.get('exhausted', '?')}")
    key = p.inv.get("key")
    if key and "determining key" in key.stdout:
        lines += ["## The determining key (`--models key`)", "", "```text"]
        lines += [l for l in key.stdout.splitlines() if l.startswith("  ") and not l.startswith("  solutions")
                  and "verdict" not in l][:200]
        lines += ["```", ""]
    grids = grids_from(p.inv["grid"].stdout) if "grid" in p.inv else []
    gi = 0
    for kind in ("models", "open"):
        for stem in index.get(kind, []):
            rec = load_json(raw / f"{stem}.json") or {}
            lines += [f"## {stem}" + ("" if rec.get("reported", True) else " — not reported (a model outranks it)"), ""]
            lines += [f"- **commitment:** {commit_text(rec.get('commitment'))} (layer {rec.get('layer')})",
                      f"- **goal rows:** {json.dumps(rec.get('goal_bindings', []))}",
                      f"- **owes:** {rec.get('owes', {}).get('total', 0)}"
                      + "".join(f" — {i.get('why')}" for i in rec.get("owes", {}).get("instances", [])),
                      f"- **file:** [`{stem}.ein`]({rel(raw / (stem + '.ein'), page)}) "
                      f"([record]({rel(raw / (stem + '.json'), page)}))", ""]
            if gi < len(grids) and (kind == "models" or index.get("verdict") == "Open"):
                lines += ["**Grid** (`--models grid`, verbatim):", "", grids[gi], ""]
                gi += 1
            inv = p.inv.get(f"render-{stem.replace('/', '-')}")
            if inv and inv.code == 0:
                for name, svg in p.dot_stream(inv.stdout, stem):
                    lines += ["**Root + what it added** (bold), `ein render kb … --since root.ein`:", "",
                              *image(svg, page, stem)]
            added = rec.get("added", [])
            lines += details(f"added — {len(added)} facts not in root", [f"- `{f}`" for f in added])
            lines += details(f"every fact — {len(rec.get('facts', []))}",
                             [f"- `{r}` ({len(fs)}): " + ", ".join(f"`{f}`" for f in fs)
                              for r, fs in by_relation(rec.get("facts", [])).items()])
    refuted = sorted((raw / "refuted").glob("*.json"))
    if refuted:
        rpage = p.dir / "refuted.md"
        rows = []
        for f in refuted:
            r = load_json(f) or {}
            rows.append([f.stem, commit_text(r.get("commitment")), r.get("layer"), r.get("kind"),
                         " ".join(r.get("unsat_core", [])), " ".join(r.get("learned_clause", []))])
        p.add("Refuted commitments", rpage,
              header("Every refuted commitment", "`refuted/NN.json`: commitment, layer, kind, core and learned clause",
                     "`--dump-answer DIR`", defined, [raw / "refuted"], rpage, f"**{len(rows)}**")
              + table(["#", "commitment", "layer", "kind", "unsat core", "learned clause"], rows))
        lines += [f"## Refuted — {len(rows)}", "", f"[Every refuted commitment]({rel(rpage, page)})", ""]
    alive = load_json(raw / "alive_at_end.json") or []
    lines += [f"## Alive at the depth cap — {len(alive)}", ""]
    if not alive:
        lines += ["_none: the search exhausted, or no cap was reached_"]
    else:
        lists = paged(p, p.dir / "alive", "Alive at the depth cap",
                      lambda path, note: header("Alive at the depth cap",
                                                "`alive_at_end.json`: the frontier `--max-set-size` left",
                                                "`--dump-answer DIR`", defined, [raw / "alive_at_end.json"], path, note),
                      ["#", "commitment"], [[i + 1, commit_text(c)] for i, c in enumerate(alive)])
        lines += [", ".join(f"[page {i + 1}]({rel(x, page)})" for i, x in enumerate(lists))]
    p.add("Answer", page, lines)


# ── summary, events, consistency ──────────────────────────────────────────────

def counter_meanings() -> dict[str, str]:
    """`answer_dump.md`'s counter table: `| `name` | meaning |`."""
    out = {}
    text = read(REPO / "docs/kernel/inference/answer_dump.md")
    sec = text.split("## The run summary", 1)[-1] if "## The run summary" in text else ""
    for line in sec.splitlines():
        m = re.match(r"^\|\s*`([^`]+)`\s*\|\s*(.+?)\s*\|\s*$", line)
        if m:
            out[m.group(1)] = m.group(2)
    return out


def summary_page(p: Pages, s: dict | None) -> None:
    page = p.dir / "summary.md"
    raw = p.raw / "summary.json"
    defined = [("answer_dump.md § The run summary", "docs/kernel/inference/answer_dump.md"),
               ("configuration.md", "docs/kernel/configuration.md")]
    lines = header("The run summary", "`--json-summary`, as tables", "`--json-summary FILE`", defined, [raw], page)
    if not s:
        p.add("Summary", page, lines + ["_No summary was written._"])
        return
    v = s.get("verdict", {})
    lines += ["## Verdict", ""]
    lines += table(["field", "value"], [[k, json.dumps(v.get(k))] for k in ("type", "k", "exhausted", "reason")]
                   + [["unsat_core", " ".join(v.get("unsat_core", [])) or "—"],
                      ["solutions", f"{len(v.get('solutions', []))} (facts on the answer page)"],
                      ["open_states", f"{len(v.get('open_states', []))}"]])
    meanings = counter_meanings()
    link = doc("docs/kernel/inference/answer_dump.md", page) + "#the-run-summary"
    lines += ["", "## Counters (`stats`)", "", f"Meanings are [`answer_dump.md` § The run summary]({link})'s, quoted.", ""]
    lines += table(["counter", "value", "meaning"],
                   [[k, json.dumps(val), rebase(meanings.get(k, "**not defined on any page**"),
                                                "docs/kernel/inference/answer_dump.md", page)]
                    for k, val in s.get("stats", {}).items()])
    owes = s.get("owes", {})
    lines += ["", "## Owes", "", f"declared obligation rules: {owes.get('declared', 0)}; owed at root: "
              f"{owes.get('root', {}).get('total', 0)}", ""]
    lines += table(["recorded state", "owes", "instances"],
                   [[i + 1, m.get("total"), "; ".join(x.get("why", "") for x in m.get("instances", []))]
                    for i, m in enumerate(owes.get("models", []))]) or []
    root = s.get("root", {})
    lines += ["", "## Root", ""] + table(["field", "value"], [[k, json.dumps(val)] for k, val in root.items()])
    lines += ["", "## Leftover", "", "```json", json.dumps(s.get("leftover"), indent=2), "```", ""]
    lines += ["## Resolved configuration", "",
              f"Every flag is documented in [`configuration.md`]({doc('docs/kernel/configuration.md', page)}).", ""]
    lines += table(["flag", "value"], [[k, json.dumps(val)] for k, val in s.get("config", {}).items()])
    p.add("Summary", page, lines)


def event_kinds_doc() -> dict[str, tuple[str, str]]:
    """`events.md`'s three schema tables: kind → (section anchor, emitted at).

    A schema table is found by its header row, `| `e` | emitted at | payload |`,
    the way `events_reference.rs` finds it: the page has other tables keyed by
    kind (what the tree traversal omits), and their second column is not
    *emitted at*."""
    out = {}
    text = read(REPO / "docs/kernel/inference/events.md")
    section, in_table = "", False
    for line in text.splitlines():
        if line.startswith("### "):
            section = anchor_of(line[4:])
        if line.startswith("| `e` | emitted at | payload |"):
            in_table = True
            continue
        if in_table and not line.startswith("|"):
            in_table = False
        m = re.match(r"^\|\s*`([^`]+)`(?:\s*/\s*`([^`]+)`)?\s*\|\s*(.+?)\s*\|", line)
        if in_table and m:
            for k in (m.group(1), m.group(2)):
                if k:
                    out[k] = (section, m.group(3))
    return out


def events_pages(p: Pages) -> list[dict]:
    raw = p.raw / "events.jsonl"
    edir = p.dir / "events"
    events = jsonl(raw)
    docs = event_kinds_doc()
    by_kind: dict[str, list[dict]] = defaultdict(list)
    for e in events:
        by_kind[e.get("e", "?")].append(e)
    index = edir / "index.md"
    rows = []
    for kind, all_es in sorted(by_kind.items(), key=lambda kv: -len(kv[1])):
        capped = len(all_es) > EVENT_ROWS_PER_KIND
        half = EVENT_ROWS_PER_KIND // 2
        es = all_es[:half] + all_es[-half:] if capped else all_es
        keys: list[str] = []
        for e in es:
            keys += [k for k in e if k not in keys and k != "e"]
        pages_n = (len(es) + ROWS_PER_PAGE - 1) // ROWS_PER_PAGE
        first = None
        for pi in range(pages_n):
            chunk = es[pi * ROWS_PER_PAGE:(pi + 1) * ROWS_PER_PAGE]
            page = edir / (f"{_safe(kind)}.md" if pages_n == 1 else f"{_safe(kind)}-{pi + 1:03d}.md")
            first = first or page
            sec, at = docs.get(kind, ("", ""))
            schema = doc("docs/kernel/inference/events.md", page) + (f"#{sec}" if sec else "")
            at = rebase(at, "docs/kernel/inference/events.md", page)
            lines = header(f"Events — `{kind}`" + (f" ({pi + 1}/{pages_n})" if pages_n > 1 else ""),
                           f"every `{kind}` line of the event stream" + (f" — emitted at: {at}" if at else ""),
                           "`--events FILE --events-level verbose`",
                           [("events.md" + (f" § {sec}" if sec else " (no schema row)"), "docs/kernel/inference/events.md")],
                           [raw], page, f"rows {pi * ROWS_PER_PAGE + 1}–{pi * ROWS_PER_PAGE + len(chunk)} of {len(es)} · "
                           f"[schema row]({schema}) · [all kinds]({rel(index, page)})"
                           + (f"  \n> **{len(all_es)} `{kind}` lines; these pages show the first {half} and the last "
                              f"{half}** — every one is in the raw file, and `n` is each line's place in it" if capped else ""))
            lines += table(keys, [[json.dumps(e.get(k), ensure_ascii=False) if isinstance(e.get(k), (list, dict))
                                   else e.get(k, "") for k in keys] for e in chunk])
            p.add(f"Events: {kind}" + (f" {pi + 1}" if pages_n > 1 else ""), page, lines)
        rows.append([f"[`{kind}`]({rel(first, index)})", len(all_es) if not capped else f"{len(all_es)} (first and last {half} rendered)", pages_n,
                     rebase(docs.get(kind, ("", "—"))[1], "docs/kernel/inference/events.md", index)])
    lines = header("The event stream", "the engine's *record* of the run — every step, by kind",
                   "`--events FILE --events-level verbose`", [("events.md", "docs/kernel/inference/events.md")],
                   [raw], index, f"**{len(events)} events**, {len(by_kind)} kinds")
    if not events:
        lines += ["_Not written._" if not raw.exists() else "_Empty._"]
    lines += table(["kind", "lines", "pages", "emitted at (events.md)"], rows)
    p.add("Events", index, lines)
    return events


def consistency(p: Pages, s: dict | None, steps: list[dict], states: list[dict], events: list[dict]) -> None:
    """The five cross-checks of S1e2.1.5, each a pass or a named mismatch."""
    def strip(x):
        if isinstance(x, dict):
            return {k: strip(v) for k, v in x.items() if k not in ("source",)}
        if isinstance(x, list):
            return [strip(v) for v in x]
        return x
    # 1 — the invocations agree.
    base = strip(s) if s else None
    for other in ("summary.lattice.json", "summary.grid.json", "summary.key.json", "summary.diff.json"):
        o = load_json(p.raw / other)
        if o is None:
            if (other == "summary.lattice.json" and tree(p.spec)) or (other == "summary.diff.json" and not p.spec.diff_models):
                continue
            p.checks.append((f"1 · {other} agrees with summary.json", False, "not written"))
            continue
        same = strip(o) == base
        diff = "" if same else ", ".join(k for k in (base or {}) if strip(o).get(k) != (base or {}).get(k))
        p.checks.append((f"1 · {other} agrees with summary.json", same, diff and f"differs in: {diff}"))
    # 2 — answer.json is --json-summary.
    a, b = read(p.raw / "answer" / "answer.json"), read(p.raw / "summary.json")
    p.checks.append(("2 · answer/answer.json is summary.json, byte for byte", bool(a) and a == b,
                     "" if a == b else ("answer.json missing" if not a else "the bytes differ")))
    # 3 — the trace's headline and § Solution against the dumped answer.
    trace = read(p.raw / "trace.md")
    if trace and s:
        head = next((l for l in trace.splitlines() if l.startswith("> ")), "")
        k = s.get("verdict", {}).get("k")
        exhausted = s.get("verdict", {}).get("exhausted")
        n_ref = len(list((p.raw / "answer" / "refuted").glob("*.json")))
        m = re.search(r"(\d+) solution\(s\), (\d+) refuted", head)
        m0 = re.search(r"No solution — (\d+) commitments? refuted", head)
        mo = re.search(r"Open — owes \d+;.*?(\d+) refuted", head)
        if m:
            said = (int(m.group(1)), int(m.group(2)))
        elif m0:
            said = (0, int(m0.group(1)))
        elif mo:
            said = (0, int(mo.group(1)))
        else:
            said = None
        if said is None:
            p.checks.append(("3a · the trace headline's counts are the answer's", False,
                             f"the headline states no count: {head!r} (k = {k})"))
        else:
            ok = said == (k, n_ref)
            p.checks.append(("3a · the trace headline's counts are the answer's", ok,
                             "" if ok else f"the headline says {said[0]} solution(s), {said[1]} refuted; "
                                           f"the answer has k = {k}, {n_ref} refuted"))
            # The inventory's S2: "No solution" printed on a run that did not
            # exhaust — the CLI's word was fixed and the headline's was not.
            if m0 and exhausted is False:
                p.checks.append(("3a′ · the headline does not claim *no solution* for a run that did not exhaust",
                                 False, f"{head!r} on a run with exhausted = false (M1e2 S2)"))
        _, secs = split_trace(trace)
        sol = next((b for t, b in secs if t == "Solution"), None)
        if sol is not None:
            dot = sol.split("```dot", 1)[-1].split("```", 1)[0]
            statements = [l for l in dot.strip().splitlines()[1:]
                          if l.strip() and not l.strip().startswith(("//", "}", "rankdir", "node ", "edge ", "graph "))]
            ok = bool(statements) or not k
            p.checks.append(("3b · the trace's § Solution draws something when there is a model", ok,
                             "" if ok else f"§ Solution is an empty graph, and the answer has {k} model(s)"))
            # 3c — every labelled edge it draws between two named entities is a
            # fact of a dumped model: `"a" -> "b" [label="R"]` is `(R a b)`, a
            # self-loop `(R a)` (the compact view's unary idiom).
            facts = set()
            for stem in (load_json(p.raw / "answer" / "index.json") or {}).get("models", []):
                facts |= set((load_json(p.raw / "answer" / f"{stem}.json") or {}).get("facts", []))
            drawn = re.findall(r'^\s*"([^"]+)"\s*->\s*"([^"]+)"\s*\[label="([^"#][^"]*)"', dot, re.M)
            missing = []
            for a, b, label in drawn:
                r = label.split()[0]   # `right-of (1)` — the count is the renderer's
                cands = [f"({r} {a} {b})", f"({r} {b} {a})"] + ([f"({r} {a})"] if a == b else [])
                if not any(c in facts for c in cands):
                    missing.append(cands[0])
            if drawn and facts:
                p.checks.append(("3c · every edge § Solution draws is a fact of a dumped model", not missing,
                                 "" if not missing else f"{len(missing)} of {len(drawn)} drawn edges are in no model, "
                                 f"e.g. {', '.join(missing[:3])}"))
    # 4 — Σ entered (events) = stats.enterings_total, on a lattice run.
    if s and events and not tree(p.spec):
        entered = sum(e.get("entered", 0) for e in events if e.get("e") == "layer")
        total = s.get("stats", {}).get("enterings_total")
        p.checks.append(("4 · Σ entered over the `layer` events is stats.enterings_total", entered == total,
                         "" if entered == total else f"Σ entered = {entered}, enterings_total = {total}"))
    # 5 — the timeline's last layer and layers_explored.
    if s and states and tree(p.spec):
        p.checks.append(("5 · the timeline's last layer is stats.layers_explored", None,
                         "not applicable: under EIN_TRAVERSAL=tree there are no layers and layers_explored "
                         "carries the deepest node (documented — M1e2 S9)"))
    elif s and states:
        last = max([e.get("layer", 0) for e in states if e.get("event") in ("layer_start", "layer_end", "entering")] or [0])
        le = s.get("stats", {}).get("layers_explored")
        p.checks.append(("5 · the timeline's last layer is stats.layers_explored", last == le,
                         "" if last == le else f"timeline reaches layer {last}, layers_explored = {le}"))
    page = p.dir / "consistency.md"
    lines = header("Cross-checks", "five claims the artefacts make about each other, checked against each other",
                   "this bundle — reading the files above", [("S1e2.1.5", "plans/m1e2_correctness/p1e2.1_seeing_the_answer/s1e2.1.5_every_artefact_rendered.md")],
                   [], page, "**A mismatch is a finding**, filed when seen — not smoothed over by the renderer.")
    lines += table(["check", "result", "detail"],
                   [[c, "n/a" if ok is None else ("pass" if ok else "**MISMATCH**"), d] for c, ok, d in p.checks])
    lines += ["", f"## Graphs Graphviz refused — {len(p.refused)}", ""] + ([f"- {r}" for r in p.refused] or ["_none_"])
    p.add("Consistency", page, lines)


# ── the run page ──────────────────────────────────────────────────────────────

def run_page(p: Pages, s: dict | None) -> Path:
    page = p.out / "run.md"
    main = p.inv["main"]
    v = (s or {}).get("verdict", {})
    lines = [f"# {p.spec.title or p.spec.file}", "",
             f"> `ein solve {p.spec.file} {' '.join(p.spec.flags)}`"
             + (f" with `{' '.join(f'{k}={x}' for k, x in p.spec.env.items())}`" if p.spec.env else "")
             + f"  \n> **{v.get('type', '—')}**, k = {v.get('k', '—')}, exhausted = {v.get('exhausted', '—')}"
             + f" · [raw/command.txt]({rel(p.raw / 'command.txt', page)})", ""]
    lines += ["## The answer — stdout, verbatim", "", "```text", verdict_block(main.stdout), "```", ""]
    if tree(p.spec):
        lines += ["**Under `EIN_TRAVERSAL=tree` there is no lattice tree**: the traversal has no layers, so "
                  "`--dump-lattice` is not run and there are no per-commitment pages "
                  f"([configuration.md]({doc('docs/kernel/configuration.md', page)})).", ""]
    if "diff" in p.inv:
        lines += ["## Against the right answer — `--diff-models`, verbatim", "", "```text",
                  diff_block(p.inv["diff"]), "```", ""]
    fails = [c for c in p.checks if c[1] is False]
    applicable = [c for c in p.checks if c[1] is not None]
    lines += [f"## Cross-checks — {len(applicable) - len(fails)} of {len(applicable)} pass", ""]
    lines += [f"- **{c}** — {d}" for c, _, d in fails] or ["_all pass_"]
    lines += ["", f"[Consistency]({rel(p.dir / 'consistency.md', page)})", ""]
    # Grouped by section, each linking its own index, so this page stays
    # short whatever the run: `pets.ein -e` is 4 200 pages, nearly all of them
    # one per commitment tested.
    lines += ["## Every page", ""]
    groups: "OrderedDict[str, list[Path]]" = OrderedDict()
    for t, path, _ in p.pages:
        r = path.relative_to(p.dir)
        groups.setdefault(r.parts[0] if len(r.parts) > 1 else r.stem, []).append(path)
    rows = []
    for g, paths in groups.items():
        index = next((x for x in paths if x.name == "index.md"), paths[0])
        n_lines = sum(read(x).count(chr(10)) for x in paths)
        rows.append([f"[{g}]({rel(index, page)})", len(paths), n_lines])
    lines += table(["section", "pages", "lines"], rows)
    lines += ["", "## Every invocation", ""]
    lines += table(["", "exit", "command"], [[i.name, i.code, f"`{i.shell()}`"] for i in p.inv.values()])
    lines += ["", "## Every raw file", ""]
    lines += [f"- [`{f.relative_to(p.out)}`]({rel(f, page)}) — {size_of(f)}"
              for f in sorted(p.raw.iterdir())]
    write(page, "\n".join(lines) + "\n")
    return page


def build_run(spec: RunSpec, out: Path, bin: str) -> RunResult:
    """Drive, keep, render. `out` is replaced."""
    import shutil
    if out.exists():
        shutil.rmtree(out)
    invs = drive(spec, out, bin)
    p = Pages(out, spec, invs)
    p.trace_blocks = 0
    summary = load_json(out / "raw" / "summary.json")
    program_page(p)
    steps = trace_pages(p) if spec.trace else []
    root_page(p, summary, steps)
    lat = lattice_index(p)
    states = states_pages(p, lat)
    if not tree(spec):
        lattice_pages(p, lat)
    region_pages(p)
    answer_pages(p)
    summary_page(p, summary)
    events = events_pages(p) if spec.events else []
    consistency(p, summary, steps, states, events)
    run_page(p, summary)
    v = (summary or {}).get("verdict", {})
    return RunResult(out, spec, invs, summary, p.checks, p.pages, v.get("type", ""), v.get("k"), v.get("exhausted"))
