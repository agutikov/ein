"""The case page for a register row whose run is not a solve — M1f S1f.5.19.

Since the register's schema /16 (M1f S1f.5.5, T1f.5.5.13) a row's
`[case.now]` can be a command's run that is not a solve: `saturate --dump`,
a `render` view, `kb save {out}/again.einb`. It writes no summary. Its answer
is a refusal or an error line, or that it answered: exit 0 with nothing on
stderr, and with `container` (the witness saved as a `.einb` first) the same
stdout the run prints on the witness, every number a line times masked and
the lines that say what it read aside. `ein-cli/tests/wrong_answers.rs` holds
the row so, and this page shows what it holds, under the user's rule that a
review artefact carries every engine artefact, raw and rendered:

    <case>/<witness>/README.md     the case page, in the solve page's look
    <case>/<witness>/runs/now/     run.md: the command as run, its exit, stdout and stderr in full
        raw/                       every invocation's argv, exit, stdout and stderr, never edited;
                                   container/ (the `.einb` the run read), out/ (the run's `{out}`),
                                   witness-out/ (the same run's on the witness)
        img/                       every DOT a view printed, as SVG

The case page is `case.py`'s for every row: its title, fixed callout, § 1
table and checklist are the solve page's, and `case_page` hands a command's
row here. Nothing here interprets an answer of its own: the comparison with
`[case.now]` is `wrong_answers.rs`' (`differences`, `comparable` and the run
planner `ein_corpus::plan::argv`, ported line for line).
"""

from __future__ import annotations

import difflib
import json
import re
from dataclasses import dataclass, field
from pathlib import Path

from review.util import (REPO, Invocation, SvgJob, SvgResult, details, image, read, rel, render_svgs, run, run_closed,
                         size_of, split_digraphs, table, write)

# The register's `{out}` and the container, kept apart: a `kb save` run writes
# into `{out}`, and every file there is the run's own.
SCRATCH, CONTAINER, WITNESS_OUT = "out", "container", "witness-out"


def is_command(run_name: str, stdout: str | None = None) -> bool:
    """`ein_corpus::register::Now::is_command`: a run that is not a solve, or
    one whose stdout is closed (schema /21), whatever its subcommand."""
    return stdout == "closed" or not run_name.startswith("solve")


def panic_message(stderr: str) -> str | None:
    """`wrong_answers.rs`' `panic_message`: the line of stderr after `thread
    'main' (…) panicked at …:` (schema /21)."""
    ls = lines_of(stderr)
    at = next((i for i, l in enumerate(ls) if " panicked at " in l), None)
    return ls[at + 1] if at is not None and at + 1 < len(ls) else None


def plan_argv(run_name: str, file: str, out: Path | str) -> list[str]:
    """`ein_corpus::plan::argv`: `ein <run>` with the file in its place and
    `{out}` the run's scratch directory — `render <view> FILE …`,
    `kb save FILE …`, else `<cmd> FILE …`; a solve gains its summary."""
    toks = [t.replace("{out}", str(out)) for t in run_name.split()]
    if toks and toks[0] in ("render", "kb"):
        argv = toks[:2] + [file] + toks[2:]
    else:
        argv = toks[:1] + [file] + toks[1:]
    if toks and toks[0] == "solve":
        argv += ["--json-summary", f"{out}/summary.json"]
    return argv


def comparable(stdout: str) -> str:
    """`wrong_answers.rs`' `comparable`: a command's stdout as two runs of it
    are compared (schema /16) — the lines that say what the run read
    (`input:`, its size, `parse:`, `kb load:`, `open:`) aside, every number a
    line times (`… ms`) masked, and runs of spaces one."""
    out = []
    for line in lines_of(stdout):
        t = line.lstrip()
        size = t.endswith(" chars") or t.endswith(" bytes, a .einb container")
        if t.startswith(("input:", "parse:", "kb load:", "open:")) or (size and t[:1].isdigit()):
            continue
        words = line.split()
        masked = []
        for i, w in enumerate(words):
            timed = i + 1 < len(words) and words[i + 1] in ("ms", "s", "s)")
            masked.append("#" if timed and _number(w) else w)
        out.append(" ".join(masked))
    return "\n".join(out)


def _number(w: str) -> bool:
    """Rust's `w.trim_start_matches("Δ=").trim_start_matches('(').parse::<f64>().is_ok()`."""
    while w.startswith("Δ="):
        w = w[2:]
    w = w.lstrip("(")
    if not w or w != w.strip() or "_" in w:
        return False
    try:
        float(w)
        return True
    except ValueError:
        return False


def lines_of(text: str) -> list[str]:
    """Rust's `str::lines`: split at `\\n`, a trailing `\\r` dropped, no last
    empty line."""
    out = text.split("\n")
    if out and out[-1] == "":
        out.pop()
    return [l[:-1] if l.endswith("\r") else l for l in out]


def first_line(stderr: str) -> str:
    """The first non-empty line of stderr, as `wrong_answers.rs` reads it."""
    return next((l for l in lines_of(stderr) if l), "<empty>")


def shown(path: Path) -> str:
    """A path as the bundle hands it to `ein`: relative to the repo, where the
    bundle runs, when it is under it."""
    return str(path.relative_to(REPO)) if path.is_relative_to(REPO) else str(path)


def answer_of(inv: Invocation) -> str:
    """A command's answer in the register's words: `answered`, the signal that
    ended it (schema /21), or the exit and stderr's first line."""
    if inv.code < 0:
        return f"**ended by signal {-inv.code}**" + (", with stderr" if inv.stderr.strip() else ", nothing on stderr")
    if inv.code == 0:
        return "**answered**" + (", with stderr" if inv.stderr.strip() else "")
    return f"exit {inv.code}: {code_span(first_line(inv.stderr))}"


def code_span(text: str) -> str:
    return f"`` {text} ``" if "`" in text else f"`{text}`"


def fenced(text: str, lang: str = "text") -> list[str]:
    """A block quoted verbatim, its fence longer than any backtick run that
    starts one of its lines."""
    runs = [len(m.group(0)) for m in re.finditer(r"(?m)^`+", text)]
    fence = "`" * max(3, max(runs, default=0) + 1)
    return [fence + lang, text.rstrip("\n") if text.strip() else "(empty)", fence]


def save_container(witness: str, into: Path, bin: str) -> tuple[Invocation, Path]:
    """`ein kb save WITNESS INTO/<stem>.einb`, as `wrong_answers.rs` saves a
    `container` row's witness before the run (schema /16)."""
    into.mkdir(parents=True, exist_ok=True)
    einb = into / f"{Path(witness).stem}.einb"
    return run("save", [bin, "kb", "save", witness, shown(einb)]), einb


# ── one run ───────────────────────────────────────────────────────────────────

@dataclass
class CommandRun:
    """A command's run and everything it left behind, under `out`."""

    out: Path
    file: str                       # the path the run was handed
    scratch: Path                   # the run's `{out}`
    invocations: list[Invocation]
    main: Invocation
    save: Invocation | None = None
    container: Path | None = None
    witness: Invocation | None = None   # with `container`: the same run on the witness
    diffs: list[str] = field(default_factory=list)
    compared: tuple[str, str] | None = None   # (the witness's, the container's), compared
    svgs: dict[str, list[tuple[str, SvgResult]]] = field(default_factory=dict)
    written: dict[str, list[Path]] = field(default_factory=dict)   # `{out}`'s files, per run

    # What `cmd_bundle` and the index read off a run, as off a solve's.
    k = None
    exhausted = None
    now = None
    summary = None

    @property
    def raw(self) -> Path:
        return self.out / "raw"


def differences(now: dict, r: CommandRun) -> list[str]:
    """Every way the run differs from `[case.now]`, as
    `every_row_answers_what_the_register_records` reads a command's row: a
    refusal or an error is its exit and stderr's first line, `{FILE}` the path
    handed and `{out}` the scratch directory; `answered` is exit 0, nothing on
    stderr, and with `container` the witness's stdout, compared; and a run
    whose stdout is closed (schema /21) the signal that ended it, nothing on
    stderr, or its panic, exit 101, and the panic's message."""
    inv = r.main
    first = first_line(inv.stderr)
    if now.get("stdout") == "closed":
        ended = f"signal {-inv.code}" if inv.code < 0 else f"exit {inv.code}"
        if now.get("signal") is not None:
            d = [] if inv.code == -now["signal"] else [f"recorded: ended by signal {now['signal']}, got {ended}"]
            return d + ([f"the run wrote stderr: {first}"] if inv.stderr else [])
        d = [] if inv.code == 101 else [f"recorded: a panic, exit 101, got {ended}"]
        said = panic_message(inv.stderr)
        if said != now.get("panicked"):
            d.append(f"panicked: recorded {json.dumps(now.get('panicked'))}, got {json.dumps(said)}")
        return d
    ended = (("refused", "a refusal") if now.get("refused") is not None else
             ("error", "an error") if now.get("error") is not None else None)
    if ended:
        name, what = ended
        want = now[name].replace("{FILE}", r.file).replace("{out}", shown(r.scratch))
        exit = now.get("exit", 1)
        d = []
        if inv.code != exit:
            d.append(f"exit: recorded {exit}, {what}, got {inv.code}")
        if first != want:
            d.append(f"{name}: recorded {json.dumps(want, ensure_ascii=False)}, "
                     f"got {json.dumps(first, ensure_ascii=False)}")
        return d
    if inv.code != 0:
        return [f"exit: recorded 0, the command answers, got {inv.code} — stderr: {first}"]
    d = []
    if inv.stderr.strip():
        d.append(f"the command answered and wrote stderr: {inv.stderr.strip()}")
    if now.get("container") and r.compared is not None:
        want, have = r.compared
        if want != have:
            at = next((f"line {i + 1}" for i, (a, b) in enumerate(zip(want.split("\n"), have.split("\n"))) if a != b),
                      "their lengths")
            d.append(f"the container's stdout is not the witness's ({at} differs)")
    return d


def views(r: CommandRun, name: str, inv: Invocation) -> None:
    """A view's DOT stdout, one SVG per digraph, under `img/<name>/`."""
    if inv.code != 0:
        return
    blocks = split_digraphs(inv.stdout)
    res = render_svgs([SvgJob(b, r.out / "img" / name / f"{i + 1:02d}_{re.sub(r'[^A-Za-z0-9_.-]+', '_', n)}.svg")
                       for i, (n, b) in enumerate(blocks)])
    r.svgs[name] = list(zip([n for n, _ in blocks], res))


def drive(row: dict, out: Path, bin: str) -> CommandRun:
    """Run a command's `[case.now]`, and every invocation its page shows."""
    now, witness = row["now"], row["witness"]
    raw = out / "raw"
    scratch = raw / SCRATCH
    scratch.mkdir(parents=True, exist_ok=True)
    invs: list[Invocation] = []
    save = einb = None
    file = witness
    if now.get("container"):
        save, einb = save_container(witness, raw / CONTAINER, bin)
        invs.append(save)
        if save.code != 0:
            raise RuntimeError(f"`ein kb save {witness}` exited {save.code}: {first_line(save.stderr)}")
        file = shown(einb)
    argv = [bin, *plan_argv(now["run"], file, shown(scratch))]
    main = run_closed("main", argv) if now.get("stdout") == "closed" else run("main", argv)
    invs.append(main)
    r = CommandRun(out, file, scratch, invs, main, save, einb)
    if now.get("container"):
        there = raw / WITNESS_OUT
        there.mkdir(parents=True, exist_ok=True)
        r.witness = run("witness", [bin, *plan_argv(now["run"], witness, shown(there))])
        invs.append(r.witness)
        r.compared = (comparable(r.witness.stdout), comparable(main.stdout))
    # The problem, drawn from the witness, as the solve page's § 2 draws it.
    for name, view in (("context-program", "program"), ("context-rules", "rules"), ("context-root", "kb")):
        invs.append(run(name, [bin, "render", view, witness], timeout=120))
    # What the run printed or wrote, drawn: a view's DOT; a saturate's KB,
    # through `render kb` on the file it read; each container under `{out}`,
    # listed (`saturate --dump`) and its program drawn.
    cmd = now["run"].split()[0]
    if cmd == "saturate":
        invs.append(run("saturated", [bin, "render", "kb", file], timeout=120))
    for label, d in (("now", scratch), ("witness", raw / WITNESS_OUT)):
        files = sorted(p for p in d.rglob("*") if p.is_file()) if d.exists() else []
        r.written[label] = files
        for p in files:
            if p.suffix == ".einb":
                stem = f"{label}-{p.stem}"
                invs.append(run(f"list-{stem}", [bin, "saturate", shown(p), "--dump"], timeout=120))
                invs.append(run(f"draw-{stem}", [bin, "render", "program", shown(p)], timeout=120))
    version = run("version", [bin, "--version"])
    commit = run("git", ["git", "rev-parse", "HEAD"])
    dirty = run("git-status", ["git", "status", "--porcelain"])
    for inv in invs:
        write(raw / f"{inv.name}.stdout.txt", inv.stdout)
        write(raw / f"{inv.name}.stderr.txt", inv.stderr)
        write(raw / f"{inv.name}.exit.txt", f"{inv.code}\n")
    lines = [f"# {row['id']} — now: `ein {now['run']}`" + (" on the container" if einb else ""), "",
             f"commit: {commit.stdout.strip()}{' (working tree dirty)' if dirty.stdout.strip() else ''}",
             "", "ein --version:", version.stdout.rstrip(), ""]
    lines += [f"[{inv.name}] exit {inv.code}: {inv.shell()}" for inv in invs]
    write(raw / "command.txt", "\n".join(lines) + "\n")
    by = {i.name: i for i in invs}
    if cmd == "render":
        views(r, "now", main)
        if r.witness is not None:
            views(r, "witness", r.witness)
    for name in ("context-program", "context-rules", "context-root", "saturated"):
        if name in by:
            views(r, name, by[name])
    for i in invs:
        if i.name.startswith("draw-"):
            views(r, i.name, i)
    r.diffs = differences(now, r)
    return r


# ── the pages ─────────────────────────────────────────────────────────────────

def raw_link(r: CommandRun, name: str, stream: str, page: Path) -> str:
    p = r.raw / f"{name}.{stream}.txt"
    return f"[`{p.name}`]({rel(p, page)}) ({size_of(p)})"


def drawn(r: CommandRun, name: str, page: Path) -> list[str]:
    out = []
    for n, svg in r.svgs.get(name, []):
        out += [f"`{n}`", "", *image(svg, page, n)]
    return out


def streams(r: CommandRun, inv: Invocation, page: Path) -> list[str]:
    """One invocation's exit, stdout and stderr: the files, then each in full."""
    ended = f"signal **{-inv.code}**" if inv.code < 0 else f"exit **{inv.code}**"
    return [f"`{inv.shell()}` — {ended} ({raw_link(r, inv.name, 'exit', page)}); stdout "
            f"{raw_link(r, inv.name, 'stdout', page)}; stderr {raw_link(r, inv.name, 'stderr', page)}", "",
            "**stdout**", "", *fenced(inv.stdout), "", "**stderr**", "", *fenced(inv.stderr), ""]


def run_page(row: dict, r: CommandRun) -> Path:
    """`runs/now/run.md`: the command as run, its exit, stdout and stderr in
    full, every invocation and every raw file."""
    page = r.out / "run.md"
    now = row["now"]
    lines = [f"# {row['id']} — now: `ein {now['run']}`" + (" on the container" if r.container else "")
             + (", its stdout closed" if r.main.stdout_closed else ""), "",
             f"> `{r.main.shell()}` · " + (f"signal **{-r.main.code}**" if r.main.code < 0 else f"exit **{r.main.code}**")
             + f" · {answer_of(r.main)} · "
             f"[raw/command.txt]({rel(r.raw / 'command.txt', page)})", ""]
    if r.save is not None:
        lines += ["## The container", "", "The witness saved as a `.einb` first, as `wrong_answers.rs` saves it:", ""]
        lines += streams(r, r.save, page)
    lines += ["## The run", ""] + streams(r, r.main, page)
    if r.witness is not None:
        lines += ["## The same run on the witness", ""] + streams(r, r.witness, page)
    lines += ["## Every invocation", ""]
    lines += table(["", "exit", "command"], [[i.name, i.code, f"`{i.shell()}`"] for i in r.invocations])
    lines += ["", "## Every raw file", ""]
    lines += [f"- [`{f.relative_to(r.out)}`]({rel(f, page)}) — {size_of(f)}"
              for f in sorted(p for p in r.raw.rglob("*") if p.is_file())]
    write(page, "\n".join(lines) + "\n")
    return page


def look(r: CommandRun, pointer: str, page: Path) -> str:
    """A `look_at` pointer, resolved against what this page has: `root` is the
    KB drawn in § 2; a pointer into a solve's run is absent, and says so."""
    if pointer == "root":
        svgs = [s for _, s in r.svgs.get("context-root", []) if s.ok]
        if svgs:
            return f"[`root`]({rel(svgs[0].svg, page)}) — root after saturation, drawn in § 2 (`ein render kb`)"
    return f"`{pointer}` — not in this run: a command's run writes no summary, trace or dump of a solve"


def command_case(row: dict, out: Path, bin: str, page: Path, head: list[str], case_rows: list[list[str]],
                 checklist: list[str]) -> CommandRun:
    """Run a command's `[case.now]` and write its case page at `page`, after
    `head` (the title and the fixed callout) and with `case.py`'s § 1 rows and
    checklist, so every case page reads the same way."""
    now, w, right = row["now"], row["wrong"], row["right"]
    r = drive(row, out / "runs" / "now", bin)
    run_md = run_page(row, r)
    on = " on the container" if r.container else ""
    lines = list(head) + ["## 1 · The case", ""] + table(["", ""], case_rows) + [""]

    # 2 — the problem.
    wpath = REPO / row["witness"]
    lines += ["## 2 · The problem", "", f"[`{row['witness']}`]({rel(wpath, page)})", ""]
    from review.case import header_comment
    hc = header_comment(wpath)
    if hc:
        lines += details("the witness's own header", ["```text", hc, "```"])
    lines += details(f"the source — {read(wpath).count(chr(10))} lines", ["```lisp", read(wpath).rstrip(), "```"])
    for name, title in (("context-program", "the program as written (`render program`)"),
                        ("context-rules", "every rule (`render rules`)"),
                        ("context-root", "root after saturation (`render kb FILE`)")):
        if r.svgs.get(name):
            lines += [f"**{title}**", ""] + drawn(r, name, page)
    if r.save is not None:
        lines += ["**The container.** `[case.now]` reads the witness saved as a `.einb` (`container = true`, schema "
                  f"/16), so the run's file is [`{r.container.name}`]({rel(r.container, page)}) "
                  f"({r.container.stat().st_size} bytes), written by `{r.save.shell()}`, exit {r.save.code}; its "
                  f"stdout and stderr are [run.md]({rel(run_md, page)})'s § The container.", ""]

    # 3 — the answers, side by side.
    recorded_line = w.get("refused") or w.get("error") or w.get("panicked")
    lines += ["## 3 · The answers, side by side", "",
              "The *wrong* column is the register's record of " + recorded_what(w)
              + f"; **now** is run here, through the corpus's run planner (`ein_corpus::plan::argv`), `{{out}}` "
              f"its scratch directory, `{shown(r.scratch)}`; *right* is the definitions'. A command's run writes no "
              "summary, so what is compared is its exit and stderr, and with a container its stdout against the "
              "same run's on the witness, as `wrong_answers.rs` compares them.", ""]
    rows = [
        ["run", f"`ein {w['run']}`" + (" on the container" if w.get("container") else ""),
         f"[`ein {now['run']}`]({rel(run_md, page)}){on}", "*(the definitions)*"],
        ["file", "the witness saved as a `.einb`" if w.get("container") else "the witness",
         f"`{r.file}`", "—"],
        ["exit", w.get("exit", 0), f"signal {-r.main.code}" if r.main.code < 0 else r.main.code,
         f"signal {right['signal']}" if right.get("signal") is not None else "—"],
        ["stderr, first line" + (" (a panic's message)" if w.get("panicked") else ""),
         code_span(recorded_line) if recorded_line else "—",
         code_span((panic_message(r.main.stderr) if w.get("panicked") else None) or first_line(r.main.stderr))
         if r.main.stderr.strip() else "*(empty)*", "—"],
        ["answer", recorded_answer(w), answer_of(r.main), recorded_answer(right) if right else "—"],
    ]
    if r.witness is not None:
        rows.append(["the same run on the witness", "—",
                     f"exit {r.witness.code}; stdout " + ("**the same**" if r.compared[0] == r.compared[1]
                                                          else "**not the same**")
                     + " as the container's, compared as `wrong_answers.rs` compares them (§ 4)", "—"])
    rows.append(["the register's `now`", "—",
                 "**matches**" if not r.diffs else "**differs**: " + "; ".join(r.diffs), "—"])
    lines += table(["", f"wrong ({recorded_kind(w)}, recorded)", "now", "right"], rows) + [""]
    if "{FILE}" in (recorded_line or ""):
        lines += ["`{FILE}` in a recorded line is the path the run was handed (schema /16).", ""]
    if right.get("derived"):
        lines += [f"*Right answer derived from:* {right['derived']}", ""]
    if w.get("why"):
        lines += [f"*How the wrong answer was taken:* {w['why']}", ""]

    # 4 — stdout and stderr, in full.
    lines += ["## 4 · stdout and stderr", "",
              f"Every byte the run printed, inline and as the files the bundle kept under "
              f"[`raw/`]({rel(r.raw / 'command.txt', page)}); [run.md]({rel(run_md, page)}) has the same, and "
              "every invocation. The register records the wrong answer's first line of stderr and nothing else.", ""]
    lines += [f"### now — `ein {now['run']}`{on}", ""] + streams(r, r.main, page)
    if r.witness is not None:
        lines += [f"### the same run on the witness — `ein {now['run']}`", ""] + streams(r, r.witness, page)
        want, have = r.compared
        lines += ["### the two, compared", "",
                  "As `wrong_answers.rs`' `comparable` reads them: the lines that say what the run read (`input:`, "
                  "its size, `parse:`, `kb load:`, `open:`) aside, every number a line times masked, and runs of "
                  "spaces one. " + ("**The same.**" if want == have else "**They differ**:"), ""]
        if want != have:
            lines += fenced("\n".join(difflib.unified_diff(want.split("\n"), have.split("\n"), "the witness",
                                                           "the container", lineterm="")), "diff") + [""]

    # 5 — what the run wrote.
    lines += ["## 5 · What the run wrote", ""]
    wrote = False
    if r.svgs.get("now"):
        wrote = True
        lines += [f"**The view `ein {now['run']}` printed, drawn**{on} "
                  f"({raw_link(r, 'main', 'stdout', page)} is its DOT):", ""] + drawn(r, "now", page)
        if r.svgs.get("witness"):
            lines += ["**The same view of the witness**:", ""] + drawn(r, "witness", page)
    if r.svgs.get("saturated"):
        wrote = True
        lines += [f"**The KB `ein {now['run']}` saturated, drawn** — `ein render kb {r.file}` "
                  f"({raw_link(r, 'saturated', 'stdout', page)}); the dump itself is § 4's stdout:", ""]
        lines += drawn(r, "saturated", page)
    for label, what in (("now", "the run"), ("witness", "the same run on the witness")):
        files = r.written.get(label) or []
        if not files:
            continue
        wrote = True
        lines += [f"**Every file {what} wrote under `{{out}}`**:", ""]
        for p in files:
            lines += [f"- [`{p.name}`]({rel(p, page)}) — {p.stat().st_size} bytes"
                      + (f", a `.einb` container (its first bytes `{p.read_bytes()[:4]!r}`)" if p.suffix == ".einb"
                         else "")]
        lines.append("")
        for p in files:
            stem = f"{label}-{p.stem}"
            lst = next((i for i in r.invocations if i.name == f"list-{stem}"), None)
            if lst is not None:
                lines += details(f"`{p.name}` listed — `{lst.shell()}`, exit {lst.code}",
                                 fenced(lst.stdout + (("\n" + lst.stderr) if lst.stderr.strip() else "")))
            if r.svgs.get(f"draw-{stem}"):
                lines += [f"**The program `{p.name}` stores, drawn** (`ein render program`):", ""]
                lines += drawn(r, f"draw-{stem}", page)
    if not wrote:
        lines += [f"_`ein {now['run']}` wrote no file and printed no view: its exit, stdout and stderr (§ 3, § 4) "
                  "are the whole of what it answered._", ""]

    # 6 — where it went wrong.
    lines += ["## 6 · Where it went wrong", "",
              "The register's `look_at` pointers, resolved against what this page has.", ""]
    look_rows = [[f"`{ptr}`", look(r, ptr, page)] for ptr in row.get("look_at", [])]
    lines += (table(["pointer", "now"], look_rows) if look_rows else ["_no pointers_"]) + [""]

    # 7 — every artefact.
    raws = sorted(p for p in r.raw.rglob("*") if p.is_file())
    nsvg = sum(len(v) for v in r.svgs.values())
    lines += ["## 7 · Every artefact", "",
              f"- **now** — [run.md]({rel(run_md, page)}) · {len(r.invocations)} invocations · {len(raws)} raw files "
              f"· {nsvg} SVGs · " + ("**matches** the register's `now`" if not r.diffs
                                     else f"**{len(r.diffs)} difference(s)** from the register's `now`")]
    lines += [f"  - [`{p.relative_to(r.out)}`]({rel(p, page)}) — {size_of(p)}" for p in raws]
    lines.append("")

    # 8 — what to check.
    lines += ["## 8 · What to check", "", "The review (S1e2.1.7) answers these for this row:", ""]
    lines += [f"- [ ] {q}" for q in checklist]
    lines += ["", "Verdict: *confirmed* · *right answer corrected* · *not wrong* · *split* · *merge* · "
              "*deferred* (the user deferred the reading to a followup, which owns the open row)", ""]
    write(page, "\n".join(lines) + "\n")
    return r


# ── the register's words, shared with case.py ─────────────────────────────────

def recorded_kind(a: dict) -> str:
    """Who gave a recorded answer: the walk (the default), a region run
    (S1e2.1a.7) or a command's run (schema /16)."""
    return {"region": "region run", "command": "command"}.get(a.get("by"), "walk")


def recorded_what(a: dict) -> str:
    return {"walk": "the walk", "region run": "a region run",
            "command": "a command's run"}[recorded_kind(a)] + (f" ({a['why']})" if a.get("why") else "")


def recorded_answer(a: dict) -> str:
    """A recorded block's answer in a word: a refusal, an error, `answered`,
    a signal or a panic (schema /21), or its verdict and k."""
    if a.get("signal") is not None:
        return f"**ended by signal {a['signal']}**, nothing on stderr"
    if a.get("panicked"):
        return "**a panic**, exit 101"
    refused = a.get("refused")
    if refused is True:
        return "**a refusal**"
    if refused:
        return "**refused**" + (f", exit {a['exit']}" if a.get("exit", 1) != 1 else "")
    if a.get("error"):
        return "**an error**"
    if a.get("answered"):
        return "**answered**"
    return f"{a.get('verdict', '—')}, k = {a.get('k', '—')}"
