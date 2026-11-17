#!/usr/bin/env python3
"""wrong_answers.py — the open wrong answers, held: a step of the gate.

    python3 utils/wrong_answers.py [--no-build]

`plans/wrong_answers/register.toml` holds every known wrong answer still open,
in the project's format (`ein_corpus::regressions`, whose field reference is
`corpus/regressions.toml`'s header) with the register's own keys beside each
row. This checks those keys (the rules are `plans/wrong_answers/README.md`'s),
then runs the project's runner on the file:

    cargo run -p ein-cli --example held_answers -- \\
        --ein ein.rs/target/gate/ein --open plans/wrong_answers/register.toml

which holds each row's `[case.now]` to what the binary answers, each
`[case.region]` to what the library computes, and no row answered right by the
method. So a run that changes answer, a fix nobody intended or a new wrong
answer, fails the gate in the commit that makes it. The binary and the example
are built under the gate profile first, as the test step builds them;
`--no-build` takes them as they are. Exit 1 on any finding.
"""

from __future__ import annotations

import re
import subprocess
import sys
import tomllib
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
REGISTER = ROOT / "plans/wrong_answers/register.toml"
FOLLOWUPS = ROOT / "plans/followups"
MANIFEST = ROOT / "ein.rs/Cargo.toml"
TARGET = ROOT / "ein.rs/target/gate"

#: The keys of a row the project's format reads.
PROJECT_KEYS = {"id", "title", "settings", "decisions", "witness", "not_in_corpus", "id_order",
                "right", "region", "now"}
#: The register's own keys, which the runner keeps whole and does not read.
REGISTER_KEYS = {"case", "class", "owner", "question", "status", "open_because", "look_at", "note",
                 "wrong", "control", "twin", "review"}
REVIEW_VERDICTS = {"confirmed", "right-answer-corrected", "split", "merge", "deferred"}
LOOK_AT = {"root", "root-writes", "timeline", "layer", "entering", "refuted", "trace", "answer",
           "model", "summary", "events"}
ANSWERED_BY = {"walk", "region", "command"}
DATE = re.compile(r"^\d{4}-\d{2}-\d{2}$")
COMMIT = re.compile(r"^[0-9a-f]{7,}$")
CASE = re.compile(r"^[WC]\d+$")
STAGE = re.compile(r"^S\d+[a-z0-9]*\.\d+\.\d+$")


def followup(owner: str) -> Path | None:
    """The followup file an owner `F<n>` names, or None."""
    m = re.fullmatch(r"F(\d+)", owner)
    if not m:
        return None
    hits = sorted(FOLLOWUPS.glob(f"f{m.group(1)}_*"))
    return hits[0] if hits else None


def defects(rows: list[dict]) -> list[str]:
    out: list[str] = []
    first: dict[str, tuple[str, str]] = {}
    for r in rows:
        rid = r.get("id", "?")
        at = lambda m: out.append(f"{rid}: {m}")  # noqa: E731
        unknown = set(r) - PROJECT_KEYS - REGISTER_KEYS
        if unknown:
            at(f"keys neither the format's nor the register's: {sorted(unknown)}")
        case = r.get("case", "")
        if not CASE.match(case) or not rid.startswith(f"{case}/"):
            at(f"id {rid!r} is not `<case>/<witness>` of its case {case!r}")
        if r.get("status") != "open":
            at(f"status {r.get('status')!r}: a row here is open; a fixed one is a regression")
        if not r.get("open_because"):
            at("`open_because` says why the row's `now` is still not `right`")
        owner = r.get("owner", "")
        if not (followup(owner) or STAGE.match(owner)):
            at(f"owner {owner!r} is neither a followup under plans/followups/ nor a stage")
        for p in r.get("look_at", []):
            if p.split()[0] not in LOOK_AT:
                at(f"look_at {p!r} is not one of {sorted(LOOK_AT)}")
        rv = r.get("review")
        if rv is not None:
            if rv.get("verdict") not in REVIEW_VERDICTS:
                at(f"review verdict {rv.get('verdict')!r} is not one of {sorted(REVIEW_VERDICTS)}")
            if not DATE.match(str(rv.get("date", ""))):
                at(f"review date {rv.get('date')!r} is not YYYY-MM-DD")
            if not COMMIT.match(str(rv.get("bundle", ""))):
                at(f"review bundle {rv.get('bundle')!r} is not a commit")
            if rv.get("verdict") == "deferred" and not followup(owner) and not STAGE.match(owner):
                at("a deferred row waits on its owner, a followup or another plan's stage")
        witnesses = [r.get("witness", "")]
        answers = [("wrong", r.get("wrong"))] if "wrong" in r else []
        if "control" in r:
            answers.append(("control", r["control"]))
        answers += [(f"twin {i + 1}", t) for i, t in enumerate(r.get("twin", []))]
        if not answers:
            at("a row records the answer it was filed with, `[case.wrong]`")
        for label, a in answers:
            if not a.get("run"):
                at(f"[{label}] states its run")
            by = a.get("by", "walk")
            if by not in ANSWERED_BY:
                at(f"[{label}] by {by!r} is not one of {sorted(ANSWERED_BY)}")
            if (label != "wrong" or by != "walk") and not a.get("why"):
                at(f"[{label}] says what differs or how it was taken (`why`)")
            if a.get("witness"):
                witnesses.append(a["witness"])
        for w in witnesses:
            if not (ROOT / w).is_file():
                at(f"witness {w} does not exist")
        title, own = first.setdefault(case, (r.get("title", ""), owner))
        if (title, own) != (r.get("title", ""), owner):
            at("title or owner differs from its case's first row")
    return out


def main(argv: list[str]) -> int:
    build = "--no-build" not in argv
    reg = tomllib.loads(REGISTER.read_text(encoding="utf-8"))
    rows = reg.get("case", [])
    bad = defects(rows)
    for b in bad:
        print(f"register: {b}", file=sys.stderr)
    print(f"wrong_answers: {len(rows)} open rows, {len(bad)} defects in the register's keys")
    if build:
        for args in (["-p", "ein-cli", "--bin", "ein"], ["-p", "ein-cli", "--example", "held_answers"]):
            done = subprocess.run(["cargo", "build", "-q", "--manifest-path", str(MANIFEST),
                                   "--profile", "gate", *args], cwd=ROOT)
            if done.returncode:
                return done.returncode
    run = subprocess.run([str(TARGET / "examples/held_answers"), "--ein", str(TARGET / "ein"),
                          "--open", str(REGISTER)], cwd=ROOT)
    return 1 if bad or run.returncode else 0


if __name__ == "__main__":
    sys.exit(main(sys.argv[1:]))
