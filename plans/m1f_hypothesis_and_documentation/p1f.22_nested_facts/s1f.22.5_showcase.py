#!/usr/bin/env python3
"""S1f.22.5, task 1: today's saved state and the proposed one, side by side,
for the programs of `s1f.22.5_the_format.md`.

    python3 s1f.22.5_showcase.py [--bin EIN] [NAME …]

Run from the repository's root. For each program it writes, into
`s1f.22.5_the_format/NAME/`:

- `today/`: what `ein solve PROGRAM --dump-answer today` writes, every file;
- `today/second-NN/`: what `ein solve today/models/NN.ein --no-search
  --dump-answer` writes of a state file loaded again (today's second save);
- `today/dump-*.txt`: the `SATURATED KB DUMP` section of `ein saturate --dump`
  on the program, on the state file, and on the second save;
- `proposed/models/NN.ein` (or `owing/`): the proposed saved file, assembled by
  `s1f.22.5_propose.py` from the program's bytes, its modules' bytes and the
  commitment the state file names: **not a file today's `ein` reads**;
- `commands.txt`: every command run, with its exit code and its output, then
  the commands that would check the two invariances on the proposed file.

The binary is the gate build, `ein.rs/target/gate/ein`, written `ein` in the
transcript.
"""
import importlib.util
import os
import shutil
import subprocess
import sys

HERE = os.path.dirname(os.path.abspath(__file__))
REL = os.path.relpath(HERE)
OUT = os.path.join(REL, "s1f.22.5_the_format")
_spec = importlib.util.spec_from_file_location("propose", os.path.join(HERE, "s1f.22.5_propose.py"))
propose = importlib.util.module_from_spec(_spec)
_spec.loader.exec_module(propose)

# name, program (relative to the repository's root; a bare name is the
# folder's own file), the state files to show.
PROGRAMS = [
    ("01_q392_demo", f"{REL}/q392_demo.ein", ["models/01.ein"]),
    ("02_q392_demo_hypothesis", "q392_demo_hypothesis.ein", ["models/01.ein", "models/02.ein"]),
    ("03_q392_demo_stdlib", "q392_demo_stdlib.ein", ["models/01.ein"]),
    ("04_b_branch", "examples/domain_elim/b_branch.ein", ["models/01.ein", "models/02.ein"]),
    ("05_zebra", "examples/zebra.ein", ["models/01.ein"]),
    ("06_satisfiable", "examples/binary_logic/satisfiable.ein", ["models/01.ein"]),
    ("07_owing", "tests/region/04_owing.ein", ["owing/01.ein"]),
    ("08_what_stays_silent", "tests/stdlib/math/decimal/05_what_stays_silent.ein", ["models/01.ein"]),
]


def section(stdout):
    k = stdout.find("=  SATURATED KB DUMP")
    return stdout[k:] if k >= 0 else "(no dump: the load failed)\n"


class Log:
    def __init__(self, ein):
        self.ein = ein
        self.lines = []

    def run(self, args, keep=True):
        p = subprocess.run([self.ein, *args], capture_output=True, text=True)
        shown = " ".join(["ein", *args])
        self.lines.append(f"$ {shown}\n")
        self.lines.append(f"# exit {p.returncode}\n")
        if keep:
            for name, text in (("stdout", p.stdout), ("stderr", p.stderr)):
                if text:
                    self.lines.append(f"# {name}:\n{text}" + ("" if text.endswith("\n") else "\n"))
        self.lines.append("\n")
        return p

    def note(self, text):
        self.lines.append(text + "\n\n")


def diff(a, b, out):
    p = subprocess.run(["diff", "-u", a, b], capture_output=True, text=True)
    open(out, "w").write(p.stdout)
    plus = sum(1 for l in p.stdout.splitlines() if l.startswith("+") and not l.startswith("+++"))
    minus = sum(1 for l in p.stdout.splitlines() if l.startswith("-") and not l.startswith("---"))
    return p.returncode == 0, plus, minus


def one(ein, name, program, files):
    d = os.path.join(OUT, name)
    if "/" not in program:
        program = os.path.join(d, program)
    for sub in ("today", "proposed"):
        shutil.rmtree(os.path.join(d, sub), ignore_errors=True)
    today = os.path.join(d, "today")
    log = Log(ein)
    log.note(
        f"# {name}: {program}\n"
        "# Every command below was run on the gate build (ein.rs/target/gate/ein, written `ein`),\n"
        "# from the repository's root, by s1f.22.5_showcase.py; the output is as it printed."
    )
    log.note("## Today: solve, and save every state (`--dump-answer`)")
    log.run(["solve", program, "--dump-answer", today])
    p = log.run(["saturate", "--dump", program], keep=False)
    open(os.path.join(today, "dump-program.txt"), "w").write(section(p.stdout))
    log.note(f"# (its KB section: {today}/dump-program.txt)")
    for f in files:
        nn = f.replace("/", "-").removesuffix(".ein")
        sf = os.path.join(today, f)
        log.note(f"## Today: load {f} again (`ein solve` on the state file)")
        log.run(["solve", sf])
        p = log.run(["saturate", "--dump", sf], keep=False)
        d1 = os.path.join(today, f"dump-{nn}.txt")
        open(d1, "w").write(section(p.stdout))
        second = os.path.join(today, f"second-{nn}")
        log.note(f"## Today: save it again (`--no-search --dump-answer`), save -> load -> save")
        log.run(["solve", sf, "--no-search", "--dump-answer", second])
        f2 = os.path.join(second, "root.ein")
        same, plus, minus = diff(sf, f2, os.path.join(today, f"save-load-save-{nn}.diff"))
        log.note(
            f"$ cmp {sf} {f2}\n# {'byte-identical' if same else 'differ'}: "
            f"diff -u is +{plus} -{minus} lines ({today}/save-load-save-{nn}.diff)"
        )
        p = log.run(["saturate", "--dump", f2], keep=False)
        d2 = os.path.join(today, f"dump-{nn}-second.txt")
        open(d2, "w").write(section(p.stdout))
        same, plus, minus = diff(d1, d2, os.path.join(today, f"load-save-load-{nn}.diff"))
        log.note(
            f"## Today: load -> save -> load, the two loads' `ein saturate --dump` KB sections\n"
            f"$ diff -u {d1} {d2}\n# {'identical' if same else 'differ'}: +{plus} -{minus} lines"
        )
        if not propose.commitment(sf):
            # Root's state: the run's own is what `ein saturate --dump PROGRAM` prints.
            dp = os.path.join(today, "dump-program.txt")
            same, plus, minus = diff(dp, d1, os.path.join(today, f"run-vs-load-{nn}.diff"))
            log.note(
                f"## Today: run -> save -> load, root's state: the program's own dump against the"
                f" state file's load\n"
                f"$ diff -u {dp} {d1}\n# {'identical' if same else 'differ'}: +{plus} -{minus} lines"
                f" ({today}/run-vs-load-{nn}.diff)"
            )
        pf = os.path.join(d, "proposed", f)
        os.makedirs(os.path.dirname(pf), exist_ok=True)
        open(pf, "w").write(propose.saved(program, sf))
        log.note(
            f"## Proposed: {pf}, assembled by s1f.22.5_propose.py from {program}, its modules\n"
            f"## and the commitment {sf} names. NOT a file today's `ein` reads.\n"
            f"## The commands that would check the two invariances on it:\n"
            f"#   ein solve {program} --dump-answer A                      # save: A/{f} is {pf}\n"
            f"#   ein solve A/{f} --no-search --dump-answer B                # load it, save its root again\n"
            f"#   cmp A/{f} B/root.ein                                      # save -> load -> save: byte for byte\n"
            f"#   ein saturate --dump A/{f} > a.txt; ein saturate --dump B/root.ein > b.txt\n"
            f"#   diff <(sed -n '/SATURATED KB DUMP/,$p' a.txt) <(sed -n '/SATURATED KB DUMP/,$p' b.txt)"
            f"   # load -> save -> load"
        )
    open(os.path.join(d, "commands.txt"), "w").write("".join(log.lines))


def survey(ein):
    """`00_survey/commands.txt`: every writer of a state, run once."""
    d = os.path.join(OUT, "00_survey")
    shutil.rmtree(d, ignore_errors=True)
    os.makedirs(d)
    P = "examples/domain_elim/b_branch.ein"
    log = Log(ein)
    log.note(
        f"# 00_survey: every writer of a state, run on {P}\n"
        "# The gate build (ein.rs/target/gate/ein, written `ein`), from the repository's root,\n"
        "# by s1f.22.5_showcase.py; every file the commands write is under this folder."
    )
    log.note("## --dump-answer: models/NN.ein (and owing/NN.ein where R is empty and its nodes owe)")
    log.run(["solve", P, "--dump-answer", f"{d}/answer"], keep=False)
    log.note(f"# wrote: " + ", ".join(sorted(os.listdir(f"{d}/answer"))) + "; models/: "
             + ", ".join(sorted(os.listdir(f"{d}/answer/models"))))
    log.note("## --no-search --dump-answer: root.ein")
    log.run(["solve", P, "--no-search", "--dump-answer", f"{d}/root"], keep=False)
    log.note(f"# wrote: " + ", ".join(sorted(os.listdir(f"{d}/root"))))
    log.note("## ein oracle --dump --states: region.json, each node's commitment and its state beyond root")
    log.run(["oracle", P, "--dump", f"{d}/oracle", "--states"], keep=False)
    import json
    nodes = json.load(open(f"{d}/oracle/region.json"))["region"]["nodes"]
    log.note("# region.json's nodes, three keys each:\n" + "\n".join(
        json.dumps({k: n[k] for k in ("facts", "status", "state")}, ensure_ascii=False) for n in nodes))
    log.note("## --dump-lattice: nodes.json, each visited node's commitment and closure")
    log.run(["solve", P, "--dump-lattice", f"{d}/lattice"], keep=False)
    nodes = json.load(open(f"{d}/lattice/nodes.json"))["nodes"]
    log.note("# nodes.json's nodes, three keys each:\n" + "\n".join(
        json.dumps({k: n[k] for k in ("commitment", "kind", "closure")}, ensure_ascii=False) for n in nodes))
    log.note("## -p: each listed state's facts beyond the program, no origin")
    p = log.run(["solve", P, "-p"], keep=False)
    k = p.stdout.find("── state 1")
    log.note("# the form, as stdout prints it after the read-out:\n" + p.stdout[k:].rstrip())
    log.note("## ein saturate --dump: root's KB by origin; does its text load as a program?")
    p = log.run(["saturate", "--dump", P], keep=False)
    sec = section(p.stdout)
    open(f"{d}/saturate-dump.txt", "w").write(sec)
    prog = "".join(l.strip() + "\n" for l in sec.splitlines() if l.lstrip().startswith("("))
    open(f"{d}/saturate-dump-as-program.ein", "w").write(prog)
    log.note(f"# its KB section: {d}/saturate-dump.txt; its fact and declaration lines as a file:"
             f" {d}/saturate-dump-as-program.ein")
    log.run(["solve", f"{d}/saturate-dump-as-program.ein"])
    log.note("## ein kb save: a .einb (a cache, written under build/ and not kept), and the state file it writes")
    os.makedirs("build/s5", exist_ok=True)
    log.run(["kb", "save", "examples/binary_logic/satisfiable.ein", "build/s5/sat.einb"])
    log.run(["solve", "build/s5/sat.einb", "--dump-answer", f"{d}/einb"], keep=False)
    log.run(["solve", "examples/binary_logic/satisfiable.ein", "--dump-answer", f"{d}/ein"], keep=False)
    a_ = open(f"{d}/einb/models/01.ein").read().split(";;;\n", 1)[1]
    b_ = open(f"{d}/ein/models/01.ein").read().split(";;;\n", 1)[1]
    log.note(f"# einb/models/01.ein and ein/models/01.ein below their header: "
             f"{'byte-identical' if a_ == b_ else 'differ'} (ein/ is removed: it is 06_satisfiable/today)")
    shutil.rmtree(f"{d}/ein")
    log.note("## ein show hyps: H, one candidate a line")
    log.run(["show", "hyps", P])
    open(os.path.join(d, "commands.txt"), "w").write("".join(log.lines))


def main():
    a = sys.argv[1:]
    ein = "ein.rs/target/gate/ein"
    if a[:1] == ["--bin"]:
        ein, a = a[1], a[2:]
    if not a or "00_survey" in a:
        survey(os.path.abspath(ein))
        print("00_survey")
    for name, program, files in PROGRAMS:
        if not a or name in a:
            one(os.path.abspath(ein), name, program, files)
            print(name)


if __name__ == "__main__":
    main()
