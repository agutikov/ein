#!/usr/bin/env python3
"""Which programs the loader refuses for a missing `:why` — M1f S1f.5.14's census, its flag half.

    utils/nl_census.py                       # the census, to stdout
    utils/nl_census.py --json census.json    # + the machine copy
    utils/nl_census.py --check               # exit 1 while a program the loader reads lacks a :why

It runs the engine ($EIN_BIN or --bin, default `ein.rs/target/release/ein`).
**The flag** (`--no-require-why`): every `.ein` under `examples/`, `tests/`,
`corpus/` and `stdlib/`, per root: how many files the loader reads at all
(`ein kb save` with `:why` not required), and of those how many it refuses
when a `:why` is required, with the forms the refusals name (`relation`,
`rule`, `macro`). A file that does not load either way (a fixture
under `examples/broken/`, a witness refused at the parse) is counted apart and
has no form counted: the loader never reads its `:why`s. Which spelling turns
the requirement on and off is read from `ein kb save --help`.

Its answer half, which counted the facts of every state file that read as
English, is retired: a `:why` reads only the form it is written on, so a fact
has no English of its own, and a saved state holds no fact (S1f.23.5).
"""
from __future__ import annotations

import argparse
import collections
import json
import os
import re
import subprocess
import sys
import tempfile
from pathlib import Path

REPO = Path(__file__).resolve().parents[1]
EIN = Path(os.environ.get("EIN_BIN", REPO / "ein.rs" / "target" / "release" / "ein"))
ROOTS = ("examples", "tests", "corpus", "stdlib")
FORMS = ("relation", "rule", "macro")

# ── the flag ─────────────────────────────────────────────────

def flag_spellings(ein: Path) -> tuple[list[str], list[str]]:
    """(argv that requires a :why, argv that does not), as this binary spells them."""
    help_ = subprocess.run([str(ein), "kb", "save", "--help"], capture_output=True, text=True).stdout
    if "--no-require-why" in help_:
        return [], ["--no-require-why"]
    return ["--require-why"], []


REFUSAL = re.compile(r"(relation|rule|macro) '([^']*)' has no :why")


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


# ── the report ───────────────────────────────────────────────

def report(flag: dict) -> None:
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


def main() -> int:
    ap = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    ap.add_argument("--bin", type=Path, default=EIN, help="the ein binary ($EIN_BIN)")
    ap.add_argument("--json", type=Path, default=None, metavar="FILE")
    ap.add_argument("--check", action="store_true",
                    help="exit 1 while the loader refuses a program it reads for a missing :why")
    args = ap.parse_args()
    if not args.bin.exists():
        print(f"nl_census: no binary at {args.bin} (build it, or set $EIN_BIN / --bin)", file=sys.stderr)
        return 2
    with tempfile.TemporaryDirectory(prefix="nl_census-") as tmp:
        flag = flag_half(args.bin, Path(tmp))
    report(flag)
    if args.json:
        args.json.write_text(json.dumps({"flag": flag}, indent=1, sort_keys=True) + "\n")
    if args.check:
        refused = sum(r["refused"] for r in flag.values())
        if refused:
            print(f"\nnl_census --check: {refused} programs the loader reads lack a :why", file=sys.stderr)
            return 1
    return 0


if __name__ == "__main__":
    sys.exit(main())
