#!/usr/bin/env python3
"""M1e2 inventory re-take (2026-11-06, snapshot c523f3ab): every item of the
inventory of 2026-11-05 (W-1…W-15, S-1…S-14, D-1…D-11) and every item added
since, each witness run once on the snapshot's release binary. One engine
process at a time. Each run's stdout and stderr are written whole under
./retake/<id>/, and retake_items.log gets the command, the exit code, the
stderr's byte count and its first lines, and the stdout lines the item's
`grep` names (or the first 40).

    EIN_STDLIB=src/stdlib nice -n 19 python3 retake_items.py [-k ID]

Commands run from ./src, the `git archive` of the tree being re-taken, so every
path is repo-relative. The probes of 2026-11-05 are read from the snapshot's
plans/m1e2_correctness/inventory_2026-11-05/, and the probes added here from
plans/m1e2_correctness/inventory_2026-11-06/probes/ (copied into the snapshot
before the run).
"""
from __future__ import annotations

import argparse
import os
import re
import shutil
import subprocess
import sys
from pathlib import Path

HERE = Path(__file__).resolve().parent
SRC = HERE / "src"
BIN = str(HERE / "target/release/ein")
OUT = HERE / "retake"
I05 = "plans/m1e2_correctness/inventory_2026-11-05"
I06 = "plans/m1e2_correctness/inventory_2026-11-06/probes"
WA = "examples/ein-bugs/wrong-answers"

# (id, argv after the binary, grep regex over stdout or None)
RUNS: list[tuple[str, list[str], str | None]] = [
    # ── W ─────────────────────────────────────────────────────────────────
    ("W-1", ["solve", "examples/features/07_unstratifiable.ein", "-p"], r"states|stale|state |model|\(p one\)|\(q one\)"),
    ("W-2", ["solve", f"{WA}/f07-rules-swapped.ein"], r"states|stale|model"),
    ("W-3", ["solve", f"{WA}/alt-cap-shared.ein"], r"core|refuted|states"),
    ("W-4", ["solve", "examples/zebra2.ein", "--dump-answer", "@D"], r"states|N "),
    ("W-5", ["solve", "examples/lattice/10_fifo_gap.ein"], r"states|stale|paths|model"),
    ("W-6", ["solve", "examples/saturation/square-unique/corner-house.ein"], r"states|state |stale|exhausted|minimal"),
    ("W-7", ["solve", f"{I05}/findings/F2_core_cut.ein"], r"core|refuted|cut"),
    ("W-7b", ["solve", f"{WA}/core-search-cut.ein"], r"core|refuted|cut"),
    ("W-8", ["solve", f"{WA}/guard-only-second.ein", "--dump-answer", "@D"], r"states|N "),
    ("W-8b", ["solve", f"{WA}/guard-only-first.ein", "--dump-answer", "@D"], r"states|N "),
    ("W-9", ["solve", "examples/ein-bugs/fork-delta-order.ein", "--dump-answer", "@D"], r"states|N |core"),
    ("W-10", ["solve", f"{WA}/closure-shared-given.ein"], r"core|refuted"),
    ("W-10b", ["solve", f"{WA}/closure-shared-given-control.ein"], r"core|refuted"),
    ("W-11", ["solve", f"{I05}/findings/F1_keyword_prefix_split.ein"], r"states|universe|query|class|\?y|model"),
    ("W-11b", ["saturate", f"{I05}/findings/F1b_or_prefix.ein", "--dump"], r"^\s+\((or|and|neq|orz|andz|z|w)\b"),
    ("W-12", ["solve", f"{I05}/probes/wild_in_guard.ein"], None),
    ("W-13", ["solve", f"{I05}/probes/bare_premise.ein"], None),
    ("W-14", ["solve", f"{I05}/probes/nested_unbound_head.ein"], None),
    ("W-15", ["solve", f"{WA}/oblig-derived-activator.ein"], r"states|empty|owes|∅|model|universe|outside"),
    ("W-15b", ["solve", f"{WA}/oblig-derived-activator-declared.ein"], r"states|empty|owes|∅|model|universe"),
    # new since 2026-11-05
    ("W-16a", ["solve", f"{WA}/past-limit-lines.ein", "--minimal"], r"minimal|maximal|states|exhausted|met"),
    ("W-16b", ["solve", f"{WA}/past-limit-lines.ein", "--maximal"], r"minimal|maximal|states|exhausted|met"),
    ("W-16c", ["oracle", "examples/lattice/10_fifo_gap.ein"], r"minimal|maximal|states"),
    ("W-16d", ["solve", "examples/lattice/10_fifo_gap.ein", "--region-limit", "1", "--minimal"], r"minimal|maximal|states|exhausted"),
    ("W-16e", ["solve", "examples/lattice/10_fifo_gap.ein", "--region-limit", "1", "--maximal"], r"minimal|maximal|states|exhausted"),
    ("W-17a", ["test", f"{WA}/claim-int-name.ein", "-v"], None),
    ("W-17b", ["solve", f"{WA}/claim-int-name-solve.ein"], r"claim|FAILED|held|expect"),
    ("W-17c", ["solve", f"{WA}/classes-int-name.ein"], r"states|class|over"),
    ("W-18", ["solve", "examples/zebra2.ein", "--dump-answer", "@D"], r"states|N "),
    ("W-19a", ["solve", f"{WA}/forall-core-false.ein", "--dump-answer", "@D"], r"states|N |core"),
    ("W-19b", ["solve", f"{WA}/forall-core-elim.ein", "--dump-answer", "@D"], r"states|N |core"),
    ("W-21a", ["solve", f"{WA}/form-in-guard.ein"], None),
    ("W-21b", ["solve", f"{WA}/kwpair-in-guard.ein"], None),
    ("W-21c", ["solve", f"{WA}/guard-before-binder.ein"], None),
    ("W-21d", ["solve", f"{WA}/nested-fact-activator.ein"], None),
    ("W-22a", ["saturate", f"{I06}/fact_wild_arg.ein", "--dump"], r"^\s+\(r |relation r"),
    ("W-22b", ["solve", f"{I06}/fact_wild_arg.ein"], r"states|universe|model"),
    ("W-23", ["solve", f"{WA}/kw-neq-in-absent.ein"], r"states|stale|state |model"),
    # ── S ─────────────────────────────────────────────────────────────────
    ("S-1a", ["saturate", "examples/syntax/arg-kinds.ein", "--dump"], r"^\s+\((r|given|f|g|p|q) "),
    ("S-1b", ["saturate", "examples/ein-bugs/activator-text-int-and-string.ein", "--dump"], r"^\s+\((holds|noted) "),
    ("S-1c", ["solve", f"{I05}/probes/n5_string_space.ein", "--dump-answer", "@D"], r"states"),
    ("S-1d", ["saturate", f"{I05}/probes/n5_int_vs_string.ein", "--dump"], r"^\s+\("),
    ("S-1e", ["saturate", f"{I06}/empty_arg.ein", "--dump"], r"^\s+\(f "),
    ("S-2", ["solve", "examples/lattice/06_friends_tables.ein", "--dump-answer", "@D"], r"states|N |exhausted"),
    ("S-3a", ["solve", "examples/ein-bugs/alt-cap-core.ein"], r"core|refuted"),
    ("S-3b", ["solve", "examples/ein-bugs/alt-cap-core-reordered.ein"], r"core|refuted"),
    ("S-4", ["test", "tests/region/07_cube.ein", "--json-report", "@F", "-v"], r"outside|types"),
    ("S-8", ["solve", f"{WA}/forbid-chain-node-writes.ein", "--monotone"], None),
    ("S-11", ["solve", "examples/features/01_not_and_absent.ein", "--monotone"], r"site|status|monotone"),
    ("S-12", ["solve", "examples/lattice/04_obligation_trigger_hole.ein"], r"maximal line|minimal line|hole|status"),
    ("S-15", ["solve", f"{I06}/diff_models_text.ein", "--diff-models", f"{I06}/diff_models_text.models"], r"matched|missing|spurious|="),
    ("S-15b", ["solve", f"{I06}/diff_models_text.ein", "--diff-models", f"{I06}/diff_models_text_control.models"], r"matched|missing|spurious|="),
    ("S-16a", ["solve", "examples/features/01_not_and_absent.ein", "--minimal", "--single"], r"minimal|single|decided|Solution|Ambiguity|states|work|exhausted"),
    ("S-16b", ["solve", "examples/features/01_not_and_absent.ein", "--minimal", "-n", "1"], r"minimal|may|states|work|exhausted"),
    ("S-16c", ["solve", "examples/features/01_not_and_absent.ein"], r"minimal line|maximal line|states|work"),
    ("S-16d", ["solve", "examples/features/01_not_and_absent.ein", "--minimal", "-n", "2"], r"minimal|states|work|exhausted"),
    ("S-17a", ["solve", f"{WA}/past-limit-lines.ein", "--region-limit", "0"], r"minimal line|maximal line|states|exhausted"),
    ("S-17b", ["solve", f"{WA}/past-limit-lines.ein", "--region-limit", "1"], r"minimal line|maximal line|states|exhausted"),
    ("S-17c", ["solve", f"{WA}/past-limit-lines.ein", "--region-limit", "2"], r"minimal line|maximal line|states|exhausted"),
    ("S-18", ["solve", f"{I06}/goal_guard.ein"], r"query|goal|class|expect|holds"),
    ("S-18b", ["solve", f"{I06}/goal_guard_claim.ein"], None),
    ("S-19", ["solve", f"{WA}/past-limit-lines.ein", "--region-limit", "17", "--json-summary", "@F"], r"hole|states|exhausted"),
    # ── D ─────────────────────────────────────────────────────────────────
    ("D-1a", ["solve", "nope.ein"], None),
    ("D-1b", ["saturate", "nope.ein"], None),
    ("D-1c", ["saturate", "examples/ein-bugs/unbound-assert-var.ein"], None),
    ("D-1d", ["solve", "examples/ein-bugs/unbound-assert-var.ein"], None),
    ("D-1e", ["oracle", "examples/ein-bugs/unbound-assert-var.ein"], None),
    ("D-1f", ["solve", "examples/ein-bugs/unbound-relation-head.ein"], None),
    ("D-1g", ["oracle", "examples/ein-bugs/unbound-relation-head.ein"], None),
    ("D-1h", ["saturate", "examples/ein-bugs/unbound-relation-head.ein"], None),
    ("D-1i", ["solve", f"{WA}/kw-or-in-obligation.ein"], r"states"),
    ("D-3a", ["test", f"{I05}/probes/refuted_region_claim.ein"], None),
    ("D-3b", ["solve", f"{I05}/probes/refuted_region_claim.ein"], r"claim|FAILED"),
    ("D-3c", ["test", f"{WA}/claim-int-name-solve.ein"], None),
    ("D-3d", ["test", f"{WA}/claim-int-name.ein"], None),
    ("D-4a", ["solve", "examples/ein-bugs/activator-all-int.ein"], None),
    ("D-4b", ["solve", "examples/ein-bugs/activator-all-symbol.ein"], None),
    ("D-6", ["saturate", "examples/zebra.ein", "--max-steps", "3"], r"max_steps|max-steps|includes|steps|last"),
    ("D-8a", ["solve", f"{I05}/probes/hrule_no_fact.ein"], r"states|universe|candidates"),
    ("D-8b", ["solve", f"{I06}/assert_and_kw.ein"], r"states|universe|model"),
    ("D-9", ["solve", f"{I06}/unused_param.ein"], r"states|model"),
    ("D-10", ["render", "rules", "examples/ein-bugs/kwpair-below-the-filter.ein"], None),
    ("D-15", ["solve", f"{I06}/where_t3_example.ein"], None),
    ("D-11a", ["solve", "examples/zebra.ein", "-T", "0"], None),
    ("D-11b", ["solve", f"{I05}/probes/empty_parens.ein"], None),
    # ── G ─────────────────────────────────────────────────────────────────
    ("G-1", ["solve", "examples/ein-bugs/naf-upward-closure.ein"], None),
    ("G-1b", ["oracle", "examples/ein-bugs/naf-upward-closure.ein"], None),
    ("G-1c", ["saturate", "examples/ein-bugs/naf-upward-closure.ein"], r"saturat|facts"),
    ("G-6", ["solve", f"{I06}/var_in_fact.ein"], r"states|universe|model"),
    ("G-9", ["oracle", "examples/zebra.ein"], None),
    ("G-14", ["solve", "examples/ein-bugs/unbound-assert-var.ein", "--count"], None),
    ("G-14b", ["solve", f"{I06}/assert_never_fires.ein"], r"states"),
    ("G-15", ["solve", f"{I05}/probes/kwpair_in_pattern.ein"], r"states|model"),
    ("G-20a", ["solve", "plans/m1e2_correctness/p1e2.3_surfaces_that_misreport_the_answer/s1e2.3.10_probes/two-rules.ein"], r"core|refuted"),
    ("G-20b", ["solve", "plans/m1e2_correctness/p1e2.3_surfaces_that_misreport_the_answer/s1e2.3.10_probes/two-rules-reordered.ein"], r"core|refuted"),
    ("G-21", ["saturate", f"{I06}/assert_neq.ein", "--dump"], r"^\s+\((neq|p) "),
    ("G-22", ["saturate", f"{I06}/assert_and_kw.ein", "--dump"], r"^\s+\((p|hit) "),
    # Moved to examples/ by M1f S1f.5.6 (2026-11-09), with a `:why` on each relation.
    ("F1", ["solve", "examples/ein-bugs/minimal-line-stated-obligation.ein"], r"minimal line|∅"),
    ("F1c", ["solve", "examples/ein-bugs/minimal-line-stated-obligation-control.ein"], r"minimal line|∅"),
    ("F1d", ["solve", "tests/stdlib/algebra/24_total_owed_satisfied.ein"], r"minimal line|∅"),
]


def main() -> int:
    ap = argparse.ArgumentParser()
    ap.add_argument("-k", "--key")
    a = ap.parse_args()
    env = dict(os.environ)
    env.setdefault("EIN_STDLIB", str(SRC / "stdlib"))
    env["TMPDIR"] = str(HERE / "scratch" / "tmp")
    (HERE / "scratch" / "tmp").mkdir(parents=True, exist_ok=True)
    OUT.mkdir(exist_ok=True)
    log = []
    for rid, argv, grep in RUNS:
        if a.key and a.key != rid and not rid.startswith(a.key):
            continue
        d = OUT / rid
        if d.exists():
            shutil.rmtree(d)
        d.mkdir(parents=True)
        argv = [str(d / "dump") if x == "@D" else str(d / "file.json") if x == "@F" else x for x in argv]
        p = subprocess.run([BIN, *argv], cwd=SRC, capture_output=True, text=True, timeout=900, env=env)
        (d / "stdout.txt").write_text(p.stdout)
        (d / "stderr.txt").write_text(p.stderr)
        lines = p.stdout.splitlines()
        shown = [l for l in lines if re.search(grep, l)] if grep else lines
        cmd = "ein " + " ".join(x.replace(str(d), f"retake/{rid}") for x in argv)
        log.append(f"== {rid}  $ {cmd}\n   exit {p.returncode} · stderr {len(p.stderr.encode())} bytes")
        for l in p.stderr.strip().splitlines()[:4]:
            log.append(f"   ! {l[:300]}")
        for l in shown[:40]:
            log.append(f"   | {l[:300]}")
        print(rid, p.returncode, file=sys.stderr, flush=True)
    (HERE / ("retake_items.log" if not a.key else f"retake_{a.key}.log")).write_text("\n".join(log) + "\n")
    return 0


if __name__ == "__main__":
    sys.exit(main())
