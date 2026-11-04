"""One run, fully explicit — M1e2 S1e2.1.5, re-pointed at the region modes.

`build_run` drives every artefact `ein` can write for one `(file, flags, env)`,
keeps each verbatim under `raw/`, and renders it as pages a person reads
without decoding anything: every graph an SVG, every JSON a table.

    DIR/run.md            what was run, the answer, and every page with its size
    DIR/raw/              everything the engine wrote, never edited
    DIR/img/              every DOT above and every ```dot block of the trace, as SVG
    DIR/pages/            program, root, trace/, lattice/, region/, answer, nogoods,
                          summary, events/, consistency

A run is several invocations. `main` carries every flag that can share a solve
(`--trace --full-kb-snapshots`, `--dump-answer`, `--dump-lattice`,
`--events`, `--json-summary` and the stdout diagnostics), `grid` / `key` the
two model forms, `diff` the comparison with the register's right answer, and
`oracle` R by brute force: `ein oracle --dump`, whose directory
`ein render region` draws into `pages/region/` with this run on it (its
`--dump-lattice` and `--dump-answer`, S1e2.1a.7), and whose answer the run is
held to (cross-checks 5 and 7). Their answers have to be the same answer,
which is cross-check 1.

**Since S1e2.1a.5's switch (2026-10-18) there is no walk.** A region run
classifies nodes and never writes root, so the walk's `--dump-states`
timeline, its per-entering tree, `root_writes.json`, `refuted/` and
`alive_at_end.json` have no producer: `--dump-lattice` is every node the run
visited (`ein-lattice/2`), N replaces `refuted/`, and the cube is
`ein oracle --dump`'s rather than `--dump-region`'s. The case page's § The
region (S1e2.1a.7) is the cube with this run drawn on it.
"""

from __future__ import annotations

import json
import re
from collections import OrderedDict, defaultdict
from dataclasses import dataclass, field
from pathlib import Path

from review.util import (REPO, Invocation, SvgJob, details, doc, header, image, jsonl, load_json,
                         read, rebase, rel, render_svgs, run, size_of, split_digraphs, table, warning_words,
                         write)
from review.rasterise import rasterise_text

STEPS_PER_PAGE = 50
ROWS_PER_PAGE = 500
NOGOODS_PER_PAGE = 100
# Event rows rendered per kind. A zebra-sized run narrates millions of verbose
# events, nearly all of one kind; thousands of pages of `fire` rows are not a
# thing a reviewer reads, and building them took longer than the run. Past
# this, a kind's page says how many rows there are, which it shows (the first
# and the last), and that `raw/events.jsonl` holds every one.
EVENT_ROWS_PER_KIND = 10_000
# Region nodes paged in full up to this many (n ≤ 16); past it, each layer's
# first page, and the raw table keeps every row — an n = 20 cube is 2 100
# pages otherwise.
REGION_ROWS_PAGED = 1 << 16
# The visited-node DAGs `ein render lattice` draws are laid out up to this many
# nodes. Past it Graphviz takes minutes (`branching/07`'s describe visited
# 19 192 nodes and `zebra2-hints`' 8 389 at M1e2 S1e2.1a.7, by route 1; the
# state route visits 133 and 78, the first since S1e2.2.10 typed the file,
# 2026-11-04, `ein solve FILE --dump-lattice D`) and the picture is not one a person
# reads: the page says how many nodes there are and links the DOT, and the
# step pages list every node.
DAG_NODES_DRAWN = 1_000
# State files given a section of their own — a grid, a drawing and their
# facts — on the answer page. `--dump-answer` writes every state the run
# reached, and a describe of `saturation/square-bwd/houses` reaches 10 077:
# one `ein render kb` and one Graphviz layout each is an hour and 1.8 GB. Past
# this the rest are rows of a paged table, each linking its loadable file.
# 100 is the line budget stdout's listings share.
STATES_DRAWN = 100
# The oracle is 2ⁿ saturations, n ≤ --region-limit (16): seconds on the
# register's witnesses, and a refusal past the limit.
ORACLE_TIMEOUT = 300


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
    checks: list[tuple[str, bool | None, str]]
    pages: list[tuple[str, Path, str]]
    # `--single`'s word, the only place the four verdict words appear; "" for
    # every other question.
    verdict: str = ""
    # The states the run lists, as `[case.now]` counts them.
    k: int | None = None
    exhausted: bool | None = None
    # The run in `[case.now]`'s fields: line, exhausted, k, empty, relations,
    # models — what `ein-cli/tests/wrong_answers.rs` holds a row to.
    now: dict | None = None
    # `lattice_pages`' steps: each a title, its page and its commitments —
    # what a `layer N` or `entering …` pointer resolves against.
    lattice: list[dict] = field(default_factory=list)
    # Each cube node's row in `pages/region/`: mask → the layer page holding
    # its `<a id="mMASK">` (S1e2.1a.7).
    node_pages: dict[int, Path] = field(default_factory=dict)


def flag_value(flags: list[str], *names: str) -> str | None:
    for i, f in enumerate(flags):
        if f in names and i + 1 < len(flags):
            return flags[i + 1]
        for n in names:
            if n.startswith("--") and f.startswith(n + "="):
                return f.split("=", 1)[1]
    return None


def solve_flags(run_name: str) -> list[str]:
    """A corpus run name's flags with every `{out}`-valued option dropped,
    the option's own name included: `solve --trace {out}/t.md -n 3` → `-n 3`."""
    toks = run_name.split()[1:]
    out = []
    for i, t in enumerate(toks):
        if "{out}" in t:
            if out and out[-1].startswith("-") and out[-1] == toks[i - 1]:
                out.pop()
            continue
        out.append(t)
    return out


# ── the answer, in the register's terms ───────────────────────────────────────

def relations(stdout: str) -> list[str]:
    """The relations `-H` lists, sorted — `wrong_answers.rs`'s `relations`."""
    out, on = [], False
    for line in stdout.splitlines():
        if line.startswith("H — the universe"):
            on = True
            continue
        if not on:
            continue
        if line.startswith("  ") and not line.startswith("   "):
            words = line.split()
            if len(words) >= 2 and words[1].isdigit():
                out.append(words[0])
            else:
                on = False
        elif not line.startswith("    "):
            on = False
    return sorted(set(out))


def listed_states(s: dict | None) -> list[dict]:
    return [x for x in ((s or {}).get("region") or {}).get("states") or [] if x.get("listed")]


def now_of(s: dict | None, stdout: str) -> dict | None:
    """The summary in `[case.now]`'s fields — `wrong_answers.rs`'s `now_of`."""
    if not s:
        return None
    region = s.get("region") or {}
    states = listed_states(s)
    empty = region.get("empty")
    if not states and empty == "owing":
        models = [sorted(x.get("facts", [])) for x in region.get("frontier") or []]
    else:
        models = [sorted(x.get("facts", [])) for x in states]
    return {"line": (s.get("asked") or {}).get("which", "?"), "exhausted": region.get("exhausted") is True,
            "k": len(states), "empty": empty, "relations": relations(stdout), "models": sorted(models)}


def asked_text(s: dict | None) -> str:
    a = (s or {}).get("asked") or {}
    which = {"all": "all of R", "minimal": "the minimal line", "maximal": "the maximal line"}.get(
        a.get("which"), str(a.get("which", "?")))
    ask = a.get("ask", "?")
    n = f", -n {a['n']}" if a.get("n") is not None else ""
    return f"{which} — {ask}{n}"


# ── driving ───────────────────────────────────────────────────────────────────

def drive(spec: RunSpec, out: Path, bin: str) -> list[Invocation]:
    raw = out / "raw"
    raw.mkdir(parents=True, exist_ok=True)
    solve = [bin, "solve", spec.file, *spec.flags]
    main = solve + ["--json-summary", str(raw / "summary.json"),
                    "--dump-answer", str(raw / "answer"),
                    "--dump-lattice", str(raw / "lattice"),
                    "--stats", "--timing", "--hyp-stats", "--dump-config", "-p"]
    if spec.trace:
        main += ["--trace", str(raw / "trace.md"), "--full-kb-snapshots"]
    if spec.events:
        main += ["--events", str(raw / "events.jsonl"), "--events-level", "verbose"]
    invs = [run("main", main, spec.env)]
    # Since S1e2.3.3 a form shares stdout's 100 lines with the read-out, and
    # `--dump-answer` writes it uncut under `forms/`: each form run writes its
    # own answer directory, and the pages read the uncut file (`form_text`).
    invs.append(run("grid", solve + ["--models", "grid",
                                     "--json-summary", str(raw / "summary.grid.json"),
                                     "--dump-answer", str(raw / FORM_DIRS["grid"])], spec.env))
    invs.append(run("key", solve + ["--models", "key",
                                    "--json-summary", str(raw / "summary.key.json"),
                                    "--dump-answer", str(raw / FORM_DIRS["key"])], spec.env))
    if spec.diff_models:
        invs.append(run("diff", solve + ["--diff-models", str(spec.diff_models),
                                         "--json-summary", str(raw / "summary.diff.json"),
                                         "--dump-answer", str(raw / FORM_DIRS["diff"])], spec.env))
    # R by brute force: the cube `ein render region` draws, and the answer
    # cross-check 5 holds the run to. Past --region-limit it refuses, exit 2.
    limit = flag_value(spec.flags, "--region-limit")
    jobs = flag_value(spec.flags, "-j", "--jobs")
    invs.append(run("oracle", [bin, "oracle", spec.file, "--dump", str(raw / "region"), "--stats",
                               "--json-summary", str(raw / "summary.oracle.json")]
                    + (["--region-limit", limit] if limit else []) + (["-j", jobs] if jobs else []),
                    spec.env, timeout=ORACLE_TIMEOUT))
    budget = flag_value(spec.flags, "--region-budget")
    lattice = [bin, "render", "lattice", spec.file] + (["--region-budget", budget] if budget else [])
    for name, argv in [("render-program", [bin, "render", "program", spec.file]),
                       ("render-rules", [bin, "render", "rules", spec.file]),
                       ("render-constraints", [bin, "render", "constraints", spec.file]),
                       ("render-lattice", lattice),
                       ("render-lattice-full", lattice + ["--view", "full"]),
                       ("render-root", [bin, "render", "kb", spec.file])]:
        invs.append(run(name, argv, spec.env, timeout=120))
    # The region drawn from the oracle's directory (S1e2.1a.2), with this run
    # on it from its own two dumps (S1e2.1a.7): `auto`, the lines (the view
    # that reads at any n) when `auto` is not already them, and every node as
    # a table. Over the oracle's limit the cube is refused and `auto` is the
    # run alone, one note.
    if (raw / "region" / "region.json").exists():
        region = [bin, "render", "region", str(raw / "region")] + overlay_args(raw)
        auto = run("render-region", region, timeout=120)
        invs.append(auto)
        said = auto.stdout.split("\n", 2)[1] if auto.stdout.count("\n") > 1 else ""
        if auto.code == 0 and "view lines" not in said and "view none" not in said:
            invs.append(run("render-region-lines", region + ["--view", "lines"], timeout=120))
        invs.append(run("render-region-table", region + ["--view", "table"], timeout=120))
    # Each state file, drawn against the program's root: what it added, bold.
    for f in state_files(raw / "answer")[:STATES_DRAWN]:
        invs.append(run(f"render-{f['stem'].replace('/', '-')}",
                        [bin, "render", "kb", str(raw / "answer" / f["path"]), "--since", spec.file],
                        spec.env, timeout=120))
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


def overlay_args(raw: Path) -> list[str]:
    """`--visited` and `--answer` for each of the run's two dumps it wrote."""
    out = []
    if (raw / "lattice" / "nodes.json").exists():
        out += ["--visited", str(raw / "lattice")]
    if (raw / "answer" / "index.json").exists():
        out += ["--answer", str(raw / "answer")]
    return out


# The run's two dumps, as `overlay_args` reads them: the directory and the file
# that says it was written.
DUMPS = {"lattice": "nodes.json", "answer": "index.json"}


def dump_link(raw: Path, dump: str, label: str, page: Path, main: Invocation | None) -> str:
    """A link from `page` to one of the run's two dumps, or, where the run
    wrote none, the words saying so and why (M1e2 S1e2.1.7). A run that exits
    non-zero, such as a goal that does not compile, writes neither dump, and a
    link to it would dangle."""
    target = raw / dump / DUMPS[dump]
    if target.exists():
        return f"[{label}]({rel(target, page)})"
    why = f"the run exited {main.code}" if main is not None and main.code != 0 else "the run wrote none"
    return f"{label} (not written: {why})"


def state_files(answer: Path) -> list[dict]:
    """`index.json`'s state files — `models/` then `owing/` — each with its
    stem (`models/01`) and its kind."""
    index = load_json(answer / "index.json") or {}
    out = []
    for kind in ("models", "owing"):
        for f in index.get(kind) or []:
            path = f.get("path") if isinstance(f, dict) else str(f)
            if path:
                out.append({**(f if isinstance(f, dict) else {}), "path": path, "kind": kind,
                            "stem": path.removesuffix(".ein")})
    return out


def state_href(run_out: Path, state: int, page: Path) -> str | None:
    """A link from `page` to state `state` of a run's answer: its section on
    the answer page when it has one (the first `STATES_DRAWN` files), else the
    page of the states table that holds its row. None when the run did not
    reach it."""
    files = state_files(run_out / "raw" / "answer")
    i = next((i for i, f in enumerate(files) if f["kind"] == "models" and f.get("state") == state), None)
    if i is None:
        return None
    if i < STATES_DRAWN:
        return f"{rel(run_out / 'pages' / 'answer.md', page)}#{files[i]['stem'].replace('/', '')}"
    rest = sorted((run_out / "pages").glob("states*.md"))
    k = (i - STATES_DRAWN) // ROWS_PER_PAGE
    return rel(rest[k], page) if k < len(rest) else None


# ── pages ─────────────────────────────────────────────────────────────────────

class Pages:
    """The page set of one run, and the SVG queue they share."""

    def __init__(self, out: Path, spec: RunSpec, invs: list[Invocation]):
        self.out, self.spec = out, spec
        self.raw, self.img, self.dir = out / "raw", out / "img", out / "pages"
        self.inv = {i.name: i for i in invs}
        self.pages: list[tuple[str, Path, str]] = []
        self.checks: list[tuple[str, bool | None, str]] = []
        self.refused: list[str] = []
        self.trace_blocks = 0
        self.lattice: list[dict] = []
        self.node_pages: dict[int, Path] = {}

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
    of the whole, because two commitments' slugs can share sixty characters."""
    import hashlib
    t = re.sub(r"[^A-Za-z0-9_.-]+", "_", s) or "x"
    return t if len(t) <= 60 else t[:51] + "-" + hashlib.sha1(s.encode()).hexdigest()[:8]


def by_relation(facts) -> "OrderedDict[str, list[str]]":
    out: dict[str, list[str]] = defaultdict(list)
    for f in facts:
        m = re.match(r"\(\s*(\S+)", f)
        out[m.group(1).rstrip(")") if m else "?"].append(f)
    return OrderedDict(sorted(out.items()))


def set_text(facts) -> str:
    """A commitment or a closure, as the read-out prints one: `{(p A), (p B)}`, `∅`."""
    if not facts:
        return "∅"
    return "{" + ", ".join(facts) + "}"


def call_to_fact(text: str) -> list[str]:
    """A walk-era commitment, `p(A), q(B, C)`, as region facts: `(p A)`, `(q B C)`."""
    return [f"({r} {' '.join(a.strip() for a in args.split(',') if a.strip())})".replace(" )", ")")
            for r, args in re.findall(r"([\w*.-]+)\(([^)]*)\)", text)]


def verdict_block(stdout: str) -> str:
    """The read-out, verbatim: from `solve ·` (or `oracle ·`) to the first
    diagnostic block after it — `H`, a state's facts, the timing, the key."""
    lines = stdout.splitlines()
    try:
        start = next(i for i, l in enumerate(lines) if l.startswith(("solve ·", "oracle ·")))
    except StopIteration:
        return stdout.strip()
    end = len(lines)
    for i in range(start + 1, len(lines)):
        if re.match(r"^(H — the universe|── state|timing$|stats$|final-state facts|the states listed against|"
                    r"determining key|grid —|work$)", lines[i]):
            end = i
            break
    return "\n".join(lines[start:end]).rstrip()


def diff_block(inv: Invocation, run_out: Path | None = None) -> str:
    """`--diff-models`' block, verbatim: the uncut file the run's
    `--dump-answer` wrote (S1e2.3.3) when there is one; else from `the states
    listed against` to the blank line that ends it — or the refusal, when the
    file did not parse."""
    if run_out is not None:
        text = form_text(run_out, "diff", "diff-models")
        if text.strip():
            return text.strip("\n")
    lines = inv.stdout.splitlines()
    start = next((i for i, l in enumerate(lines) if l.startswith(("the states listed against", "models against"))), None)
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


def root_page(p: Pages, summary: dict | None, trace: list[dict]) -> None:
    page = p.dir / "root.md"
    lines = header("Root, after saturation", "the KB before any hypothesis: what the program derives on its own",
                   "`--json-summary` (`root`), `ein render kb FILE`, the trace's § Root",
                   [("solution_semantics.md § 1.3", "docs/kernel/inference/solution_semantics.md")],
                   [p.raw / "summary.json", p.raw / "render-root.stdout.txt"], page,
                   "**Root is never written by a region run**: it is `sat(program)`, and every node is "
                   "`sat(root ∪ C)` — there are no root writes to show.")
    root = (summary or {}).get("root", {})
    if root:
        # The block is root's fixpoint and H as every route starts from them
        # (M1e2 S1e2.3.1, the user's ruling (a) of 2026-11-03), so its count
        # is the count of the KB the sentence above says is never written.
        link = doc("docs/kernel/inference/answer_dump.md", page) + "#the-run-summary"
        lines += ["## Facts by relation", "",
                  f"From `summary.json`'s `root` block — root's fixpoint, {root.get('facts', '?')} facts "
                  f"(`root.facts`, [`answer_dump.md` § The run summary]({link})).", ""]
        lines += table(["relation", "facts"], [[r, n] for r, n in (root.get("facts_by_relation") or {}).items()])
        lines.append("")
        rest = root_rows(root, page, skip=("facts", "facts_by_relation"))
        if rest:
            lines += ["## The rest of the block", "",
                      f"`root.hypgen` is the pass that built H, the universe every route reads, so "
                      f"`root.hypgen.emitted` is H's size. Meanings are [`answer_dump.md` § The run "
                      f"summary]({link})'s, quoted.", ""]
            lines += table(["field", "value", "meaning"], rest)
            lines.append("")
    inv = p.inv.get("render-root")
    if inv and inv.code == 0:
        lines += ["## The KB (`ein render kb FILE`)", ""]
        for name, svg in p.dot_stream(inv.stdout, "root"):
            lines += image(svg, page, "root")
    elif inv:
        lines += [f"`ein render kb` exited {inv.code}: `{(inv.stderr.strip().splitlines() or [''])[0]}`", ""]
    root_sec = next((s for s in trace if s["slug"] == "root"), None)
    if root_sec and root_sec["steps"]:
        lines += [f"## Root's derivation — {len(root_sec['steps'])} steps (from the trace)", ""]
        lines += table(["step", "rule", "what it did"],
                       [[f"[{s['n']}]({rel(s['page'], page)}#{s['anchor']})", f"`{s['rule']}`", s["gist"]]
                        for s in root_sec["steps"]])
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


def trace_slug(title: str) -> str:
    if title.startswith("Root — "):
        return "root"
    m = re.match(r"State (\d+) of \d+", title)
    if m:
        return f"state-{int(m.group(1)):02d}"
    if title.startswith("Owing — "):
        return "owing"
    if title.startswith("N — "):
        return "nogoods"
    return _safe(title).lower()


def trace_pages(p: Pages) -> list[dict]:
    """`--trace`'s region trace (`read_out.md` § 8.4): the head, § Root, one §
    per traced state, § Owing and § N — each section a page, paged by its
    steps (or N's sets), and every ```dot block an SVG."""
    raw = p.raw / "trace.md"
    tdir = p.dir / "trace"
    text = read(raw)
    if not text:
        p.add("Trace", tdir / "index.md", header("The trace", "not written", "`--trace`", [], [raw], tdir / "index.md"))
        return []
    pre, sections = split_trace(text)
    defined = [("solution_semantics.md", "docs/kernel/inference/solution_semantics.md"),
               ("read_out.md § 8.4", "plans/m1e2_correctness/p1e2.1a_the_solution_region/read_out.md")]
    out: list[dict] = []
    block = 0
    for title, body in sections:
        slug = trace_slug(title)
        per = NOGOODS_PER_PAGE if slug == "nogoods" else STEPS_PER_PAGE
        parts = re.split(r"(?m)^(?=### Step \d+ — |<details>)", body)
        head, items = parts[0], parts[1:]
        chunks = [head * (i == 0) + "".join(items[i:i + per]) for i in range(0, max(len(items), 1), per)]
        if not chunks[0].strip():
            chunks[0] = head
        sec = {"title": title, "slug": slug, "steps": [], "pages": []}
        for ci, chunk in enumerate(chunks):
            name = f"{slug}.md" if len(chunks) == 1 else f"{slug}-{ci + 1:02d}.md"
            page = tdir / name
            body_r, n, refused = rasterise_text(chunk, p.img / "trace", page, start=block)
            block += n
            p.refused += refused
            for m in re.finditer(r"(?m)^### Step (\d+) — `([^`]+)`\n(.*?)(?=^### Step |\Z)", chunk, re.S):
                gist = next((l[2:].strip() for l in m.group(3).splitlines() if l.startswith("> ")), "")
                derives = re.search(r"^Derives `([^`]*)`", m.group(3), re.M)
                sec["steps"].append({"n": int(m.group(1)), "rule": m.group(2), "page": page,
                                     "gist": gist or (f"derives `{derives.group(1)}`" if derives else ""),
                                     "anchor": anchor_of(f"Step {m.group(1)} — {m.group(2)}")})
            lines = header(f"Trace — {title}" + (f" ({ci + 1}/{len(chunks)})" if len(chunks) > 1 else ""),
                           f"the region trace's § {title.split(' — ')[0]}", "`--trace FILE --full-kb-snapshots`",
                           defined, [raw], page, "[Index](index.md)")
            p.add(f"Trace: {title.split(' — ')[0]}" + (f" {ci + 1}" if len(chunks) > 1 else ""), page,
                  lines + ["## " + title, "", body_r])
            sec["pages"].append(page)
        out.append(sec)
    # The index: the head block, then one row per section.
    page = tdir / "index.md"
    steps = sum(len(s["steps"]) for s in out)
    lines = header("The trace", "the engine's *explanation* of the run — root's derivation, each listed state's "
                   "from its first minimal commitment, the owing frontier and N, with `:why` text",
                   "`--trace FILE --full-kb-snapshots`", defined, [raw], page,
                   f"**{len(out)} sections**, **{steps} steps**, **{block} graphs**")
    head_md = pre.replace("# Region trace", "").strip()
    if head_md:
        lines += [head_md, ""]
    lines += ["## Sections", ""]
    lines += table(["section", "steps", "pages"],
                   [[f"[{s['title']}]({rel(s['pages'][0], page)})", len(s["steps"]), len(s["pages"])] for s in out])
    p.add("Trace index", page, lines)
    p.trace_blocks = block
    return out


# ── the visited nodes ─────────────────────────────────────────────────────────

def lattice_pages(p: Pages) -> list[dict]:
    """`--dump-lattice`'s `nodes.json` (`ein-lattice/2`): every node the run
    visited, in visit order, one page per step — a route-1 layer or a mode's
    batch — and `ein render lattice`'s two DAGs."""
    ldir = p.dir / "lattice"
    raw = p.raw / "lattice"
    index = ldir / "index.md"
    defined = [("lattice_dump.md § Under a region mode", "docs/kernel/inference/lattice_dump.md")]
    d = load_json(raw / "nodes.json") or {}
    nodes, steps = d.get("nodes") or [], d.get("steps") or []
    lines = header("Every node visited", "the nodes this run's route visited, in visit order, each with its "
                   "commitment, kind, closure, what it owes and why it is dead",
                   "`--dump-lattice DIR`, `ein render lattice FILE [--view full]`", defined,
                   [raw / "nodes.json", raw / "nodes.md", p.raw / "render-lattice.stdout.txt"], index,
                   f"route **{d.get('route', '?')}** · **{d.get('visited', len(nodes))} nodes** over "
                   f"{len(d.get('candidates') or [])} candidates · {len(steps)} steps")
    if not d:
        p.add("Lattice", index, lines + ["_`nodes.json` was not written._"])
        return []
    drawn: list[int] = []
    for name, title, what in [("render-lattice", "R and one fact above it", "`ein render lattice FILE` — the nodes of "
                               "R and the nodes one fact above them that are not, from a describing run"),
                              ("render-lattice-full", "every node a describing run visits",
                               "`ein render lattice FILE --view full`")]:
        inv = p.inv.get(name)
        if inv is None:
            continue
        lines += [f"## The DAG — {title}", "", what + ". R green, owing amber, dead red.", ""]
        if inv.code != 0:
            lines += [f"_exit {inv.code}: {inv.stderr.strip() or 'nothing printed'}_", ""]
            continue
        sub = p.img / "lattice" / name.removeprefix("render-")
        for i, (n, block) in enumerate(split_digraphs(inv.stdout)):
            count = len(re.findall(r"(?m)^\s*n\d+ \[", block))
            if name == "render-lattice-full" and drawn and count == drawn[0]:
                lines += [f"The same {count} nodes as the view above: every node the describing run visited is "
                          "in R or one fact above it.", ""]
                continue
            drawn.append(count)
            if count > DAG_NODES_DRAWN:
                dot = write(sub / f"{i + 1:02d}_{_safe(n)}.dot", block)
                lines += [f"**{count} nodes — not laid out**: past {DAG_NODES_DRAWN} Graphviz takes minutes and the "
                          f"picture is not one a person reads. The [DOT]({rel(dot, index)}) is kept, and the step "
                          "pages below list every node.", ""]
                continue
            (res,) = p.svgs([SvgJob(block, sub / f"{i + 1:02d}_{_safe(n)}.svg")])
            lines += image(res, index, title)
    lines += ["## The candidates", ""] + [f"- `{c}`" for c in d.get("candidates") or []] + [""]
    out = []
    rows = []
    for i, st in enumerate(steps):
        lo = st.get("from", 0)
        hi = steps[i + 1]["from"] if i + 1 < len(steps) else len(nodes)
        title = f"layer {st['layer']}" if st.get("layer") is not None else f"step {i + 1}"
        stem = ldir / f"step-{i + 1:02d}"
        node_rows = [[x.get("n", j) + 1, set_text(x.get("commitment")), x.get("kind", ""), set_text(x.get("closure")),
                      x.get("facts", ""), x.get("owes") or "", "; ".join(x.get("owed") or []),
                      " ".join(x.get("core") or []), ", ".join(x.get("refuted_by") or [])]
                     for j, x in enumerate(nodes[lo:hi], lo)]
        paths = paged(p, stem, f"Lattice: {title}",
                      lambda path, note, title=title, st=st: header(
                          f"Visited — {title}", "every node of this step, in visit order", "`--dump-lattice DIR`",
                          defined, [raw / "nodes.json"], path,
                          f"{st.get('visited')} visited: {st.get('dead')} dead, {st.get('owing')} owing, "
                          f"{st.get('in_r')} in R; {st.get('facts')} facts beyond root · {note} · "
                          f"[all steps]({rel(index, path)})"),
                      ["#", "commitment", "kind", "closure", "facts", "owes", "owed", "core", "refuted by"], node_rows)
        out.append({"layer": st.get("layer"), "title": title, "page": paths[0],
                    "commitments": [x.get("commitment") or [] for x in nodes[lo:hi]]})
        rows.append([f"[{title}]({rel(paths[0], index)})", st.get("visited"), st.get("dead"), st.get("owing"),
                     st.get("in_r"), st.get("facts")])
    lines += ["## Steps", ""]
    lines += table(["step", "visited", "dead", "owing", "in R", "facts"], rows)
    p.add("Lattice", index, lines)
    return out


# ── the region ────────────────────────────────────────────────────────────────

def region_view(inv) -> str:
    """The view a `render region` DOT says it is, from its second line."""
    lines = inv.stdout.split("\n", 2)
    return lines[1].split("view ", 1)[-1].strip() if len(lines) > 1 else "?"


def region_pages(p: Pages) -> None:
    """`ein oracle --dump`'s directory as pages: the drawings, `region.md`,
    and every node by layer — S1e2.1a.2. The script draws and splits; the cube
    is the oracle's, and no run is overlaid on it since the switch."""
    raw = p.raw / "region"
    rdir = p.dir / "region"
    index = rdir / "index.md"
    defined = [("region_dump.md", "docs/kernel/inference/region_dump.md"),
               ("04_dot_rendering.md § Region rendering",
                "docs/kernel/ir/03-ein-lang/04_dot_rendering.md")]
    oracle = p.inv.get("oracle")
    region = load_json(raw / "region.json") or {}
    r = region.get("region")
    if r is None:
        why = (oracle.stderr.strip() if oracle else "") or region.get("refused") or "no `region.json`"
        lines = header("The region", "R by brute force — not computed for this program",
                       "`ein oracle FILE --dump DIR`", defined,
                       [p.raw / "oracle.stdout.txt", p.raw / "oracle.stderr.txt"], index,
                       f"`ein oracle` exited {oracle.code if oracle else '—'}") + ["```text", str(why), "```", ""]
        # The run alone: `render region` draws it as one note, and its table
        # lists what it reported with no cube to hold it to.
        inv = p.inv.get("render-region")
        if inv is not None and inv.code == 0:
            (res,) = p.svgs([SvgJob(inv.stdout, p.img / "region" / "region.svg")])
            lines += ["## The run alone — `ein render region` with no cube", ""] + image(res, index, "the run alone")
        tinv = p.inv.get("render-region-table")
        if tinv is not None and tinv.code == 0:
            lines += ["## The run's states", "", *tinv.stdout.splitlines()[2:], ""]
        p.add("Region", index, lines)
        return
    n = region.get("n", "?")
    head = (f"n = {n} · |R| = {r.get('size', '—')}"
            + ("" if region.get("monotonicity", {}).get("monotone", True)
               else " · **not monotone**" + warning_words(region.get("monotonicity"))))
    lines = header("The region", "the lattice of commitments, every node classified by saturation alone — R by "
                   "brute force, the reference every mode is held to", "`ein oracle FILE --dump DIR`, "
                   "`ein render region DIR`", defined, [raw / "region.json", raw / "region.md",
                                                       p.raw / "oracle.stdout.txt"], index, head)
    lines += ["Fill is what the cube says, shape what this run did, border marks the lines and the defects. Every "
              "node's reason is in its tooltip: open the SVG on its own to hover. The run is drawn from its own "
              f"{dump_link(p.raw, 'lattice', '`--dump-lattice`', index, p.inv.get('main'))} and "
              f"{dump_link(p.raw, 'answer', '`--dump-answer`', index, p.inv.get('main'))} (`ein render region DIR "
              "--visited … --answer …`).", ""]
    if oracle and oracle.code == 0:
        lines += details("the oracle's read-out (`ein oracle FILE`)", ["```text", verdict_block(oracle.stdout), "```"])
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
    # Every node, by layer, from `--view table`. Each row carries
    # `<a id="mMASK">`; the head's lists link to `#mMASK`, routed here to the
    # layer page that holds the row.
    tinv = p.inv.get("render-region-table")
    if tinv and tinv.code == 0:
        pre, sections = split_trace(tinv.stdout)
        total = sum(sum(1 for l in body.splitlines() if l.startswith("| <a id=")) for _, body in sections)
        capped = total > REGION_ROWS_PAGED
        rows, pages = [], []
        for title, body in sections:
            stem = rdir / _safe(title.split(" —")[0].lower())
            tl = [l for l in body.splitlines() if l.startswith("|")]
            head_row, body_rows = tl[:2], tl[2:]
            if capped:
                body_rows = body_rows[:ROWS_PER_PAGE]
            chunks = [body_rows[i:i + ROWS_PER_PAGE] for i in range(0, max(len(body_rows), 1), ROWS_PER_PAGE)]
            paths = [stem.with_suffix(".md") if len(chunks) == 1 else stem.with_name(f"{stem.name}-{i + 1:02d}.md")
                     for i in range(len(chunks))]
            for chunk, path in zip(chunks, paths):
                for row in chunk:
                    m = re.match(r'\| <a id="m(\d+)">', row)
                    if m:
                        p.node_pages[int(m.group(1))] = path
            pages.append((title, head_row, chunks, paths))
            rows.append([f"[{title}]({rel(paths[0], index)})", len(body_rows), len(paths)])
        for title, head_row, chunks, paths in pages:
            for i, (chunk, path) in enumerate(zip(chunks, paths)):
                nav = " · ".join(f"[{j + 1}]({rel(q, path)})" if q != path else f"**{j + 1}**" for j, q in enumerate(paths))
                ph = header(f"The region — {title}", "every node of this layer: status, why, where saturation takes it, "
                            "what the run did", "`ein render region DIR --visited … --answer … --view table`", defined,
                            [raw / "region.json"], path,
                            f"[the region]({rel(index, path)})" + (f" · pages {nav}" if len(paths) > 1 else ""))
                p.add(f"Region: {title}" + (f" {i + 1}" if len(paths) > 1 else ""), path,
                      ph + head_row + [node_links(row, p.node_pages, p.raw, path) for row in chunk])
        lines += ["## Every node", "", *[node_links(l, p.node_pages, p.raw, index) for l in pre.splitlines()[1:]], ""]
        if capped:
            lines += [f"**{total} nodes**: past {REGION_ROWS_PAGED} the layer pages show each layer's first "
                      f"{ROWS_PER_PAGE} rows, and [`render-region-table.stdout.txt`]"
                      f"({rel(p.raw / 'render-region-table.stdout.txt', index)}) holds every one.", ""]
        lines += table(["layer", "nodes", "pages"], rows) + [""]
    md = read(raw / "region.md")
    if md:
        lines += details("`region.md`, as the dump wrote it", [re.sub(r"(?m)^#", "###", md)])
    p.add("Region", index, lines)


def node_links(text: str, node_pages: dict[int, Path], raw: Path, page: Path) -> str:
    """`ein render region --view table`'s `(#mMASK)` links, routed from `page`
    to the layer page that holds the row — or to the raw table, when the
    pages cap left the row out."""
    def to(m):
        mask = int(m.group(1))
        target = node_pages.get(mask)
        if target is None:
            return f"({rel(raw / 'render-region-table.stdout.txt', page)})"
        return f"({'' if target == page else rel(target, page)}#m{mask})"
    return re.sub(r"\(#m(\d+)\)", to, text)


def overlay_head(table_md: str) -> list[str]:
    """The table view's comparison of the run with the oracle, verbatim: the
    `- **missing**`, `- **spurious**`, `- **not reached**`, `- **≠ the
    cube**` and `- **visited**` lines `ein render region` computed."""
    out = []
    for line in table_md.split("\n| bit |", 1)[0].splitlines():
        if line.startswith(("**Compared with", "- **missing**", "- **spurious**", "- **not reached**",
                            "- **≠ the cube**", "- **visited**", "- **reported outside")):
            out.append(line)
    return out


# ── the answer ────────────────────────────────────────────────────────────────

# The answer directory each invocation writes. `main`'s holds `-p` and `-H`
# under `forms/`, and each form run's holds its form (S1e2.3.3).
FORM_DIRS = {"main": "answer", "grid": "answer.grid", "key": "answer.key", "diff": "answer.diff"}


def form_text(run_out: Path, inv: str, form: str, stdout: str = "") -> str:
    """A form uncut: `forms/<form>.txt` in the answer directory invocation
    `inv` wrote — since S1e2.3.3 stdout holds only what fits of it in the 100
    lines it shares with the read-out. A run that wrote no file (an older
    binary, a refused run) falls back to its stdout."""
    f = run_out / "raw" / FORM_DIRS[inv] / "forms" / f"{form}.txt"
    return read(f) if f.exists() else stdout


def form_files(run_out: Path) -> list[Path]:
    """Every form file the run's invocations wrote, for the pages' raw lists."""
    return sorted(f for d in FORM_DIRS.values() for f in (run_out / "raw" / d / "forms").glob("*.txt"))


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


def answer_pages(p: Pages, s: dict | None) -> None:
    raw = p.raw / "answer"
    page = p.dir / "answer.md"
    index = load_json(raw / "index.json") or {}
    region = (s or {}).get("region") or index.get("region") or {}
    states = region.get("states") or []
    defined = [("answer_dump.md", "docs/kernel/inference/answer_dump.md"),
               ("solution_semantics.md", "docs/kernel/inference/solution_semantics.md")]
    listed = [x for x in states if x.get("listed")]
    lines = header("The answer", "every state the run reached, as the files `--dump-answer` wrote: grid, graph, "
                   "commitment, its `(model …)`", "`--dump-answer DIR`, `--models grid`, `--models key`, "
                   "`ein render kb … --since FILE`", defined,
                   [raw / "index.json", raw / "answer.json", raw / "answer.txt", p.raw / "grid.stdout.txt",
                    p.raw / "key.stdout.txt", *form_files(p.out)], page,
                   f"asked **{asked_text(s)}** · {len(listed)} state(s) listed of {len(states)} reached · "
                   f"exhausted = {yes(region.get('exhausted'))}"
                   + (f" · R empty: **{region['empty']}**" if region.get("empty") else ""))
    lines += details("`answer.txt` — the read-out, uncut", ["```text", read(raw / "answer.txt").rstrip(), "```"])
    key = p.inv.get("key")
    key_text = form_text(p.out, "key", "key", key.stdout) if key else ""
    if "determining key" in key_text:
        lines += ["## The determining key (`--models key`)", "", "```text"]
        lines += key_text.split("determining key", 1)[1].splitlines()[:200]
        lines += ["```", ""]
    grids = grids_from(form_text(p.out, "grid", "grid", p.inv["grid"].stdout)) if "grid" in p.inv else []
    gi = 0
    files = state_files(raw)
    for f in files[:STATES_DRAWN]:
        stem = f["stem"]
        st = states[f["state"] - 1] if f["kind"] == "models" and isinstance(f.get("state"), int) \
            and 0 < f["state"] <= len(states) else {}
        # The heading is the stem alone, so `#models01` is the anchor a case
        # page's `model 1` pointer and § 4 link to.
        lines += [f"## {stem}", ""]
        lines += [f"- **state:** {f.get('state', '—')} · **lines:** {', '.join(f.get('lines') or []) or '—'} · "
                  + ("listed on stdout" if f.get("listed", True) else "**not listed on stdout**"),
                  f"- **from:** {set_text(st.get('from'))}" if st else "- **from:** —",
                  "- **minimal generators:** " + (" · ".join(set_text(g) for g in st.get("minimal_generators") or [])
                                                   or "—"),
                  f"- **commitments reaching it:** {st.get('commitments') if st.get('commitments') is not None else '— (not counted by this question)'}",
                  f"- **file:** [`{f['path']}`]({rel(raw / f['path'], page)})", ""]
        if f["kind"] == "models" and f.get("listed", True) and gi < len(grids):
            lines += ["**Grid** (`--models grid`, verbatim):", "", grids[gi], ""]
            gi += 1
        inv = p.inv.get(f"render-{stem.replace('/', '-')}")
        if inv and inv.code == 0:
            for name, svg in p.dot_stream(inv.stdout, stem):
                lines += ["**The state, what it adds to root in bold**, `ein render kb … --since FILE`:", "",
                          *image(svg, page, stem)]
        elif inv:
            lines += [f"`ein render kb` exited {inv.code}: `{(inv.stderr.strip().splitlines() or [''])[0]}`", ""]
        if st:
            lines += details(f"its `(model …)` — {len(st.get('facts', []))} facts",
                             [f"- `{x}`" for x in st.get("facts", [])])
            lines += details(f"its closure — {len(st.get('closure', []))} candidates",
                             [f"- `{x}`" for x in st.get("closure", [])])
    if not files:
        lines += ["_No state file: the run reached no state._", ""]
    rest = files[STATES_DRAWN:]
    if rest:
        def row(f):
            st = states[f["state"] - 1] if f["kind"] == "models" and isinstance(f.get("state"), int) \
                and 0 < f["state"] <= len(states) else {}
            return [f.get("state", "—"), f["kind"], ", ".join(f.get("lines") or []) or "—",
                    "yes" if f.get("listed", True) else "no", set_text(st.get("from")) if st else "—",
                    len(st.get("facts", [])) if st else "—", f"[`{f['path']}`]({rel(raw / f['path'], p.dir / 'x.md')})"]
        pages = paged(p, p.dir / "states", "Answer: the other states",
                      lambda path, note: header("The other states", f"every state file past the first {STATES_DRAWN}, "
                                                "one row each — the file loads as a program", "`--dump-answer DIR`",
                                                defined, [raw / "index.json"], path,
                                                f"{note} · [the answer]({rel(page, path)})"),
                      ["state", "kind", "lines", "listed", "from", "facts", "file"], [row(f) for f in rest])
        lines += [f"## The other {len(rest)} states", "",
                  f"Past the first {STATES_DRAWN} a state is a row, not a section: no grid and no drawing, and its "
                  "file is one link away — " + ", ".join(f"[page {i + 1}]({rel(x, page)})" for i, x in enumerate(pages)),
                  ""]
    frontier = region.get("frontier") or []
    if frontier:
        lines += [f"## The owing frontier — {len(frontier)}", ""]
        lines += table(["#", "commitment", "facts", "owes"],
                       [[i + 1, set_text(x.get("from") or x.get("commitment")), " ".join(x.get("facts", [])),
                         json.dumps(x.get("owed", x.get("owes")), ensure_ascii=False)] for i, x in enumerate(frontier)])
        lines.append("")
    if region.get("core") or region.get("refuted_by"):
        lines += ["## Root is dead", "", f"- **core:** {' '.join(region.get('core') or []) or '∅'}",
                  f"- **refuted by:** {', '.join(region.get('refuted_by') or []) or '—'}", ""]
    nogoods = load_json(raw / "nogoods.json") or []
    npage = p.dir / "nogoods.md"
    p.add("N", npage,
          header("N — every minimal dead set", "`nogoods.json`: each minimal dead set, its core and the rules that "
                 "refuted it — what the walk's `refuted/` was", "`--dump-answer DIR`", defined,
                 [raw / "nogoods.json"], npage,
                 f"**{len(nogoods)}**" + ("" if nogoods else " — N is empty, or this question does not compute it "
                                                             "(a line mode does not)"))
          + (table(["#", "set", "core", "refuted by"],
                   [[i + 1, set_text(x.get("set")), " ".join(x.get("core") or []) or "—",
                     ", ".join(x.get("refuted_by") or []) or "—"] for i, x in enumerate(nogoods)])
             if nogoods else ["_none_"]))
    lines += [f"## N — {len(nogoods)} minimal dead set(s)", "", f"[Every minimal dead set]({rel(npage, page)})", ""]
    p.add("Answer", page, lines)


# ── summary, events, consistency ──────────────────────────────────────────────

def counter_meanings() -> dict[str, str]:
    """`answer_dump.md`'s counter table: `| `name` | meaning |`."""
    out = {}
    text = read(REPO / "docs/kernel/inference/answer_dump.md")
    sec = text.split("## The run summary", 1)[-1] if "## The run summary" in text else ""
    sec = sec.split("\n## ", 1)[0]
    for line in sec.splitlines():
        m = re.match(r"^\|\s*`([^`]+)`\s*\|\s*(.+?)\s*\|\s*$", line)
        if m:
            out[m.group(1)] = m.group(2)
    return out


def root_rows(root: dict, page: Path, skip: tuple = ()) -> list[list[str]]:
    """The `root` block as `[field, value, meaning]` rows, one per field
    `answer_dump.md` defines: dotted down to the sparse blocks, whose keys are
    data, and each meaning quoted from the page."""
    meanings = counter_meanings()
    sparse = ("facts_by_relation", "filtered", "pre_candidate")
    out: list[list[str]] = []

    def walk(v: dict, at: str) -> None:
        for k, child in v.items():
            name = f"{at}.{k}"
            if isinstance(child, dict) and k not in sparse:
                walk(child, name)
            else:
                out.append([f"`{name}`", cell(child),
                            rebase(meanings.get(name, "**not defined on any page**"),
                                   "docs/kernel/inference/answer_dump.md", page)])

    walk({k: v for k, v in root.items() if k not in skip}, "root")
    return out


def yes(v) -> str:
    """A value as a page prints it: JSON's `true` / `false` / `null`."""
    return json.dumps(v) if isinstance(v, bool) or v is None else str(v)


def cell(v) -> str:
    if isinstance(v, bool):
        return yes(v)
    if isinstance(v, (dict, list)):
        return json.dumps(v, ensure_ascii=False)
    return "null" if v is None else str(v)


def summary_page(p: Pages, s: dict | None) -> None:
    page = p.dir / "summary.md"
    raw = p.raw / "summary.json"
    defined = [("answer_dump.md § The run summary", "docs/kernel/inference/answer_dump.md"),
               ("read_out.md § 8.1", "plans/m1e2_correctness/p1e2.1a_the_solution_region/read_out.md"),
               ("configuration.md", "docs/kernel/configuration.md")]
    lines = header("The run summary", "`--json-summary` (`ein-summary/2`), as tables", "`--json-summary FILE`",
                   defined, [raw], page)
    if not s:
        p.add("Summary", page, lines + ["_No summary was written._"])
        return
    lines += ["## Asked", ""] + table(["field", "value"], [[k, cell(v)] for k, v in (s.get("asked") or {}).items()])
    region = s.get("region") or {}
    lines += ["", "## Region", "", "Every field but `states` (the answer page has them).", ""]
    lines += table(["field", "value"], [[k, cell(v)] for k, v in region.items() if k != "states"])
    lines += ["", "## Verdict", "", f"`{cell(s.get('verdict'))}` — `--single`'s alone; `null` for every other question.", ""]
    lines += ["## Queries", ""]
    qs = s.get("queries") or []
    lines += table(["goal", "over", "exact", "classes", "expect"],
                   [[f"`{q.get('goal')}`", q.get("over"), q.get("exact"), len(q.get("classes") or []),
                     cell(q.get("expect"))] for q in qs]) if qs else ["_none_"]
    lines += ["", "## The claim", "", f"`{cell(s.get('claim'))}`", ""]
    meanings = counter_meanings()
    link = doc("docs/kernel/inference/answer_dump.md", page) + "#the-run-summary"
    lines += ["## Work", "", f"Meanings are [`answer_dump.md` § The run summary]({link})'s, quoted.", ""]
    lines += table(["counter", "value", "meaning"],
                   [[k, cell(val), rebase(meanings.get(k, "**not defined on any page**"),
                                          "docs/kernel/inference/answer_dump.md", page)]
                    for k, val in (s.get("work") or {}).items()])
    lines += ["", "## Monotonicity", ""]
    lines += table(["field", "value"], [[k, cell(v)] for k, v in (s.get("monotonicity") or {}).items()])
    lines += ["", "## Root", "", "Root's fixpoint and how H was built, as every route starts from them; "
              f"meanings are [`answer_dump.md` § The run summary]({link})'s, quoted.", ""]
    lines += table(["field", "value", "meaning"], root_rows(s.get("root") or {}, page))
    lines += ["", "## Leftover", "", "```json", json.dumps(s.get("leftover"), indent=2, ensure_ascii=False), "```", ""]
    lines += ["## Resolved configuration", "",
              f"Every flag is documented in [`configuration.md`]({doc('docs/kernel/configuration.md', page)}).", ""]
    lines += table(["flag", "value"], [[k, cell(val)] for k, val in (s.get("config") or {}).items()])
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
                   "`--events FILE --events-level verbose`", [("events.md § Region runs", "docs/kernel/inference/events.md")],
                   [raw], index, f"**{len(events)} events**, {len(by_kind)} kinds")
    if not events:
        lines += ["_Not written._" if not raw.exists() else "_Empty._"]
    lines += table(["kind", "lines", "pages", "emitted at (events.md)"], rows)
    p.add("Events", index, lines)
    return events


def facts_by_line(s: dict | None, line: str) -> list[list[str]]:
    """The states of `s` on `line` (`all`, `minimal`, `maximal`), each as its
    sorted `(model …)` facts, sorted."""
    states = ((s or {}).get("region") or {}).get("states") or []
    return sorted(sorted(x.get("facts", [])) for x in states if line == "all" or line in (x.get("lines") or []))


def consistency(p: Pages, s: dict | None, trace: list[dict], events: list[dict]) -> None:
    """Seven claims the artefacts make about each other, each a pass, a named
    mismatch, or not applicable with the reason."""
    def strip(x):
        if isinstance(x, dict):
            return {k: strip(v) for k, v in x.items() if k not in ("source",)}
        if isinstance(x, list):
            return [strip(v) for v in x]
        return x
    # 1 — the invocations agree.
    base = strip(s) if s else None
    for other in ("summary.grid.json", "summary.key.json", "summary.diff.json"):
        o = load_json(p.raw / other)
        if o is None:
            if other == "summary.diff.json" and not p.spec.diff_models:
                continue
            inv = p.inv.get(other.split(".")[1])
            p.checks.append((f"1 · {other} agrees with summary.json", False, "not written" + (
                f": the `{inv.name}` run exited {inv.code}" + (" (killed — the OOM killer, or a signal)" if inv.code == -9
                                                               else "") + f", `raw/{inv.name}.stderr.txt`"
                if inv and inv.code != 0 else "")))
            continue
        same = strip(o) == base
        diff = "" if same else ", ".join(k for k in (base or {}) if strip(o).get(k) != (base or {}).get(k))
        p.checks.append((f"1 · {other} agrees with summary.json", same, diff and f"differs in: {diff}"))
    # 2 — answer.json is --json-summary.
    a, b = read(p.raw / "answer" / "answer.json"), read(p.raw / "summary.json")
    p.checks.append(("2 · answer/answer.json is summary.json, byte for byte", bool(a) and a == b,
                     "" if a == b else ("answer.json missing" if not a else "the bytes differ")))
    region = (s or {}).get("region") or {}
    states = region.get("states") or []
    listed = [x for x in states if x.get("listed")]
    # 3 — the trace narrates every listed state.
    if s and spec_traced(p):
        traced = sum(1 for t in trace if t["slug"].startswith("state-"))
        m = re.search(r"_(\d+) more states? (?:is|are) not traced", read(p.raw / "trace.md"))
        untraced = int(m.group(1)) if m else 0
        ok = traced + untraced == len(listed)
        p.checks.append(("3 · the trace has one § State per listed state", ok,
                         "" if ok else f"{traced} traced + {untraced} untraced, and the summary lists {len(listed)}"))
    # 4 — the lattice dump, the events and the summary count the same nodes.
    nodes = load_json(p.raw / "lattice" / "nodes.json")
    if nodes is not None:
        node_list = nodes.get("nodes") or []
        n_nodes = len(node_list)
        steps_sum = sum(st.get("visited", 0) for st in nodes.get("steps") or [])
        # Route 1 stopped by the budget saturates the top — every candidate —
        # to bound what it did not reach (`downset.rs`, after the layer loop).
        # That node is narrated and dumped like any other and is in no step.
        top_probe = ((region.get("stopped") or {}).get("why") == "budget" and n_nodes == steps_sum + 1
                     and len(node_list[-1].get("commitment") or []) == len(nodes.get("candidates") or []))
        ok = n_nodes == nodes.get("visited") and (steps_sum == n_nodes or top_probe)
        detail = (f"{n_nodes} nodes, visited = {nodes.get('visited')}, Σ steps.visited = {steps_sum}" if not ok else
                  f"Σ steps.visited = {steps_sum}: the last node is route 1's post-budget probe of the top, every "
                  "candidate, which no step covers" if top_probe else "")
        if events:
            enter = sum(1 for e in events if e.get("e") == "enter")
            ok = ok and enter == n_nodes
            detail = detail or ("" if enter == n_nodes else f"{enter} `enter` events, {n_nodes} nodes")
        p.checks.append(("4 · nodes.json, its steps and the `enter` events count the same nodes", ok, detail))
    # 5 — the run against the oracle: every answer is the oracle's.
    oracle = load_json(p.raw / "summary.oracle.json")
    asked = (s or {}).get("asked") or {}
    name = "5 · the run's states are the oracle's, on the line it asked"
    if s is None:
        pass
    elif oracle is None:
        p.checks.append((name, None, "not applicable: the oracle did not answer — "
                         + (((p.inv.get("oracle").stderr.strip().splitlines() or ["no summary"])[0])
                            if p.inv.get("oracle") else "not run")))
    elif not region.get("exhausted") or asked.get("n") is not None or asked.get("ask") == "single":
        p.checks.append((name, None, "not applicable: the run lists part of what it asked (`-n`, `--single`, or a "
                         "stopped run), so it is held to nothing it did not claim"))
    else:
        line = asked.get("which", "all")
        mine, theirs = sorted(sorted(x.get("facts", [])) for x in listed), facts_by_line(oracle, line)
        ok = mine == theirs
        p.checks.append((name, ok, "" if ok else f"the run lists {len(mine)} state(s) on `{line}`, the oracle "
                         f"{len(theirs)}; {sum(1 for m in mine if m not in theirs)} not the oracle's, "
                         f"{sum(1 for t in theirs if t not in mine)} missing"))
        if line == "all":
            o_r = oracle.get("region") or {}
            ok = (region.get("k_s"), region.get("k_c")) == (o_r.get("k_s"), o_r.get("k_c"))
            p.checks.append(("5′ · k_s and k_c are the oracle's", ok, "" if ok else
                             f"run k_s = {region.get('k_s')}, k_c = {region.get('k_c')}; oracle "
                             f"k_s = {o_r.get('k_s')}, k_c = {o_r.get('k_c')}"))
    # 6 — one loadable file per state reached.
    files = state_files(p.raw / "answer")
    if s is not None:
        models = [f for f in files if f["kind"] == "models"]
        missing = [f["path"] for f in files if not (p.raw / "answer" / f["path"]).exists()]
        ok = len(models) == len(states) and not missing
        p.checks.append(("6 · answer/ has one state file per state the summary reached", ok,
                         "" if ok else f"{len(models)} model files, {len(states)} states"
                         + (f"; missing {', '.join(missing)}" if missing else "")))
    # 7 — the drawing's own comparison: what `ein render region` found when it
    # put the run on the cube (S1e2.1a.7).
    tinv = p.inv.get("render-region-table")
    head = overlay_head(tinv.stdout) if tinv is not None and tinv.code == 0 else []
    name = "7 · the drawing names nothing missing, spurious or read otherwise than the cube"
    if s is not None and head:
        bad = [l for l in head if l.startswith(("- **missing**", "- **spurious**", "- **≠ the cube**"))
               and not l.endswith(" none")]
        bad += [l for l in head if l.startswith("- **visited**") and "**unsound**" in l]
        p.checks.append((name, not bad, "; ".join(re.sub(r"\]\([^)]*\)", "]", l.lstrip("- ")) for l in bad)))
    elif s is not None:
        p.checks.append((name, None, "not applicable: no cube to draw the run on — "
                         + (((p.inv.get("oracle").stderr.strip().splitlines() or ["no summary"])[0])
                            if p.inv.get("oracle") else "the oracle was not run")))
    page = p.dir / "consistency.md"
    lines = header("Cross-checks", "seven claims the artefacts make about each other, checked against each other",
                   "this bundle — reading the files above",
                   [("S1e2.1.5", "plans/m1e2_correctness/p1e2.1_seeing_the_answer/s1e2.1.5_every_artefact_rendered.md")],
                   [], page, "**A mismatch is a finding**, filed when seen — not smoothed over by the renderer.")
    lines += table(["check", "result", "detail"],
                   [[c, "n/a" if ok is None else ("pass" if ok else "**MISMATCH**"), d] for c, ok, d in p.checks])
    lines += ["", f"## Graphs Graphviz refused — {len(p.refused)}", ""] + ([f"- {r}" for r in p.refused] or ["_none_"])
    p.add("Consistency", page, lines)


def spec_traced(p: Pages) -> bool:
    return p.spec.trace and (p.raw / "trace.md").exists()


# ── the run page ──────────────────────────────────────────────────────────────

def run_page(p: Pages, s: dict | None) -> Path:
    page = p.out / "run.md"
    main = p.inv["main"]
    region = (s or {}).get("region") or {}
    verdict = ((s or {}).get("verdict") or {}).get("type")
    lines = [f"# {p.spec.title or p.spec.file}", "",
             f"> `ein solve {p.spec.file} {' '.join(p.spec.flags)}`"
             + (f" with `{' '.join(f'{k}={x}' for k, x in p.spec.env.items())}`" if p.spec.env else "")
             + f"  \n> asked **{asked_text(s)}** · {len(listed_states(s))} state(s) listed · "
             f"exhausted = {yes(region.get('exhausted'))}" + (f" · **{verdict}**" if verdict else "")
             + f" · exit {main.code} · [raw/command.txt]({rel(p.raw / 'command.txt', page)})", ""]
    lines += ["## The answer — stdout, verbatim", "", "```text", verdict_block(main.stdout), "```", ""]
    if main.code != 0 or main.stderr.strip():
        lines += ["## stderr", "", "```text", main.stderr.strip() or "(empty)", "```", ""]
    if "diff" in p.inv:
        lines += ["## Against the right answer — `--diff-models`, verbatim", "", "```text",
                  diff_block(p.inv["diff"], p.out), "```", ""]
    fails = [c for c in p.checks if c[1] is False]
    applicable = [c for c in p.checks if c[1] is not None]
    lines += [f"## Cross-checks — {len(applicable) - len(fails)} of {len(applicable)} pass", ""]
    lines += [f"- **{c}** — {d}" for c, _, d in fails] or ["_all pass_"]
    lines += ["", f"[Consistency]({rel(p.dir / 'consistency.md', page)})", ""]
    # Grouped by section, each linking its own index, so this page stays
    # short whatever the run.
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
    summary = load_json(out / "raw" / "summary.json")
    program_page(p)
    trace = trace_pages(p) if spec.trace else []
    root_page(p, summary, trace)
    p.lattice = lattice_pages(p)
    region_pages(p)
    answer_pages(p, summary)
    summary_page(p, summary)
    events = events_pages(p) if spec.events else []
    consistency(p, summary, trace, events)
    run_page(p, summary)
    region = (summary or {}).get("region") or {}
    now = now_of(summary, form_text(p.out, "main", "hyp-stats", p.inv["main"].stdout))
    return RunResult(out, spec, invs, summary, p.checks, p.pages,
                     ((summary or {}).get("verdict") or {}).get("type", ""),
                     now["k"] if now else None, region.get("exhausted") if summary else None, now, p.lattice,
                     p.node_pages)
