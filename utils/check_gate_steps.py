#!/usr/bin/env python3
"""check_gate_steps.py — `./run_tests.sh` runs what CI runs, a check, not a convention.

    python3 utils/check_gate_steps.py

`run_tests.sh` holds three step lists, and two workflows hold the remote
copies of two of them: the gate against `.github/workflows/per-commit.yml`,
`--nightly` against `.github/workflows/nightly.yml`, and `--dev-host` against
the nightly plus the steps that left it. Hand-written copies drift, and a
local gate that is a subset of the remote one is a local gate that lies: this
script reads each list out of the file that runs it and fails on any
difference. Nothing here is a third copy of a list. It is a static step of
the gate, in both copies of the gate's list, and reads only files: the
script, the two workflows, the crates' test sources (for what reads
`EIN_CORPUS_SLOW`) and one documentation page.

What is compared is the **commands**, in order, after one normalisation:

| side | what a step is | normalisation |
|---|---|---|
| `run_tests.sh` | the statement after a `step "…"` (gate), `nightly_step "…"` or `dev_host_step "…"` banner | strip `if ! ` … `; then`, join `\\` continuations, drop `"${SCRIPT_DIR}/"`, `--manifest-path "${MANIFEST}"`, `-q`, `${ARGS[@]…}` |
| `per-commit.yml` | a `- run:` marked `# gate-step` | fold a following `env: { K: "V" }` into a `K="V" ` prefix |
| `nightly.yml` | every `- run:` but the named exceptions, job by job, a matrix job once per value with `${{ matrix.KEY }}` replaced | the same fold, of the step's own `env:` |

The checks, each a function below:

- the gate's list is per-commit.yml's, and every unmarked `run:` there is a
  provisioning step named in `NOT_GATE_STEPS` with why;
- what `--tests-only` skips is read out of the script's `CHECKS` guard and is
  the static checks and the bench smoke, and the script's header states that
  count in words;
- `--nightly`'s list is nightly.yml's; every exception names a step the
  workflow has; the slow cells and the round trip's slow half run under the
  gate profile with `EIN_CORPUS_SLOW`; every job's tests run under the checked
  `TMPDIR` and are followed by the scratch check; every sharded step runs every
  one of its shards;
- `--dev-host` runs every nightly step, in order, at the dev host's fuzz
  budget, and exactly the steps `DEV_HOST_ONLY` names with why besides; the
  fuzzer runs at each tier's budget; the dev host's tests run under the
  checked `TMPDIR`;
- a nightly step that names one test runs that test, and either it is the one
  place its target reads `EIN_CORPUS_SLOW` or the dev host runs the whole
  target with the flag; every test target that reads `EIN_CORPUS_SLOW` runs
  with it on some tier, or `NO_SLOW_TIER` excuses it with a premise checked
  here.

What it does not catch: a step that differs only in the environment a job
gives it beyond its `env:` blocks (the runner's image, `GATE_TMP`'s spelling,
the `working-directory`), and a flag dropped from both copies at once, which
is why the slow flag's two steps are pinned by name.
"""

from __future__ import annotations

import re
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
SCRIPT = "run_tests.sh"
PER_COMMIT = ".github/workflows/per-commit.yml"
NIGHTLY = ".github/workflows/nightly.yml"


def read(rel: str) -> str:
    return (ROOT / rel).read_text(encoding="utf-8")


# ── the shared parse ──────────────────────────────────────────────────

def normalise(cmd: str) -> str:
    """One space between words, nothing at the ends."""
    return " ".join(cmd.split())


def shell_steps(text: str, banner: str) -> list[str]:
    """The steps of `run_tests.sh` that a `banner "…"` line announces, in order.

    A line is a banner when it *starts* with `banner "`, so the one-line
    definition `nightly_step() { step "$@"; }` is not one. The command is the
    statement after the banner, one logical line that trailing backslashes
    may spread over several, with what a shell needs and a workflow does not
    taken off; `"${SCRIPT_DIR}/x"` is rewritten per token, so semantic quotes,
    `RUSTDOCFLAGS="-D warnings"`, survive and are compared."""
    opening = f'{banner} "'
    lines = text.split("\n")
    out = []
    for i, line in enumerate(lines):
        if not line.lstrip().startswith(opening):
            continue
        j = i + 1
        cmd = ""
        while j < len(lines):
            piece = lines[j].strip()
            cmd += piece[:-1] if piece.endswith("\\") else piece
            if not piece.endswith("\\"):
                break
            cmd += " "
            j += 1
        if cmd.startswith("if ! "):
            cmd = cmd[len("if ! "):]
        while cmd.endswith("; then"):
            cmd = cmd[: -len("; then")]
        cmd = (cmd.replace('--manifest-path "${MANIFEST}" ', "")
                  .replace(" -q ", " ")
                  .replace('${ARGS[@]+"${ARGS[@]}"}', ""))
        tokens = []
        for t in cmd.split():
            if t.startswith('"${SCRIPT_DIR}/') and t.endswith('"'):
                t = t[len('"${SCRIPT_DIR}/'):-1]
            tokens.append(t)
        out.append(normalise(" ".join(tokens)))
    return out


def env_prefix(line: str) -> str | None:
    """A workflow step's `env: { K: "V", … }` as the shell's `K="V" … ` prefix."""
    body = line.strip()
    if not (body.startswith("env: { ") and body.endswith(" }")):
        return None
    body = body[len("env: { "):-len(" }")]
    out = ""
    for kv in body.split(", "):
        if ": " in kv:
            k, v = kv.split(": ", 1)
            out += f"{k}={v} "
    return out


# ── the gate and per-commit.yml ───────────────────────────────────────

# The workflow's `- run:` lines that are not gate steps, with why.
NOT_GATE_STEPS = [
    ("sudo apt-get update && sudo apt-get install -y graphviz libgvplugin-neato-layout8",
     "provisioning: the gate checks that `dot` and `fdp` lay a graph out, and exits 127 "
     "instead (Ubuntu 26.04's `graphviz` installs no `fdp` layout)"),
    ('mkdir -p "${GATE_TMP}"',
     "provisioning: the tests' TMPDIR, which the gate makes itself just before its test step"),
]

# What `--tests-only` skips: the static checks, then the bench smoke, in the
# script's order. Read out of the guard, held to this list, so a step that
# gains or loses the guard fails here.
TESTS_ONLY_SKIPS = [
    "utils/stdlib_manifest.py",
    "utils/check_hashmap_iteration.py",
    "utils/doc_audit.py --links --check",
    "utils/check_release_voice.py",
    "utils/check_gate_steps.py",
    "utils/check_tool_mirrors.py",
    "utils/check_doc_blocks.py",
    "cargo fmt --all --check",
    "cargo clippy --workspace --all-targets -D warnings",
    "cargo doc --no-deps, -D warnings",
    "cargo bench --bench engine -- --test",
]

WORDS = ["zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
         "ten", "eleven", "twelve"]


def per_commit_steps(text: str) -> tuple[list[str], list[str]]:
    """The gate steps of per-commit.yml, in order, and every `run:` it has."""
    lines = text.split("\n")
    steps, every = [], []
    for i, line in enumerate(lines):
        t = line.strip()
        if not t.startswith("- run: "):
            continue
        cmd = t[len("- run: "):].strip()
        every.append(cmd)
        before = next((l.strip() for l in reversed(lines[:i]) if l.strip()), None)
        if before != "# gate-step":
            continue
        env = env_prefix(lines[i + 1]) if i + 1 < len(lines) else None
        steps.append(normalise((env or "") + cmd))
    return steps, every


def check_gate() -> list[str]:
    bad = []
    shell = read(SCRIPT)
    gate = shell_steps(shell, "step")
    ci, every = per_commit_steps(read(PER_COMMIT))
    if gate != ci:
        bad.append("run_tests.sh and per-commit.yml no longer run the same steps in the same "
                   "order:\n  gate: " + "\n        ".join(gate) + "\n  ci:   " + "\n        ".join(ci))
    # Not vacuous: a parse that matched nothing against nothing would pass.
    if len(gate) < 7:
        bad.append(f"only {len(gate)} steps parsed out of run_tests.sh: the parse, not the gate, "
                   f"is what changed: {gate}")
    # Every `run:` is a gate step or a named exception.
    excused = [normalise(c) for c, _ in NOT_GATE_STEPS]
    for cmd in every:
        n = normalise(cmd)
        if any(s == n or s.endswith(n) for s in ci):
            continue
        if n not in excused:
            bad.append(f"per-commit.yml runs `{cmd}`, which is neither marked `# gate-step` nor "
                       "listed in NOT_GATE_STEPS with a reason: either ./run_tests.sh does not "
                       "run it, or nothing says why it should not")
    if len(every) != len(ci) + len(NOT_GATE_STEPS):
        bad.append(f"the workflow's `run:` lines are {len(every)} and the two lists account "
                   f"for {len(ci) + len(NOT_GATE_STEPS)}")
    return bad


def check_tests_only() -> list[str]:
    """What `--tests-only` skips is what the script guards, and its header says so."""
    bad = []
    text = read(SCRIPT)
    guarded, depth, checks_at = [], 0, None
    for line in text.split("\n"):
        t = line.strip()
        if t.startswith("if "):
            depth += 1
            if checks_at is None and "${CHECKS}" in t and "== 1" in t:
                checks_at = depth
        elif t == "fi":
            if checks_at == depth:
                checks_at = None
            depth = max(depth - 1, 0)
        elif checks_at is not None:
            m = re.fullmatch(r'step "(.*)"', t)
            if m:
                guarded.append(m.group(1))
    if depth != 0:
        bad.append(f"the shell parse is unbalanced: {depth} unclosed `if`")
    if guarded != TESTS_ONLY_SKIPS:
        bad.append("the set `--tests-only` skips moved: the guard holds\n  "
                   + "\n  ".join(guarded) + "\nand this check holds\n  "
                   + "\n  ".join(TESTS_ONLY_SKIPS))
    # The header is the other copy of the set, and the one that drifts.
    statics = len(guarded) - 1  # every guarded step but the bench smoke
    header = []
    for l in text.split("\n"):
        if l and not l.startswith("#"):
            break
        header.append(l)
    usage = next((l for l in header if "--tests-only" in l), None)
    if usage is None:
        bad.append("run_tests.sh's header lists no `--tests-only`")
    elif "bench" not in usage:
        bad.append(f"the header's `--tests-only` line reads `{usage}`: the flag skips the bench "
                   "smoke too")
    word = WORDS[statics] if 0 <= statics < len(WORDS) else str(statics)
    if f"{word} static check" not in "\n".join(header):
        bad.append(f"the header does not say `{word} static check…` anywhere, and the guard "
                   f"holds {statics} of them plus the bench smoke")
    return bad


# ── --nightly, --dev-host and nightly.yml ─────────────────────────────

NOT_NIGHTLY_STEPS = NOT_GATE_STEPS

# The dev host's steps that the nightly does not run, with why: each is over
# 5 min on the runner, measures rather than checks, or needs a tool the
# runner's image does not have.
DEV_HOST_ONLY = [
    ('EIN_ID_SEEDS="8" EIN_CORPUS_SLOW="1" TMPDIR="${GATE_TMP}" cargo test -p ein-render '
     '--profile gate --test id_order_invariance',
     "over 5 min on the runner; the nightly runs one permutation of every corpus program, in "
     "shards, and the per-commit tier one of every program outside tests/ that searches"),
    ("python3 utils/stdlib_census.py --check",
     "a measurement, over 5 min on the runner; `stdlib_coverage` is its per-commit gate"),
    ("python3 utils/review_bundle.py",
     "over 5 min on the runner; the per-commit `--smoke` reads every register row and builds "
     "four cases"),
    ("python3 utils/doc_audit.py --links --check build/review",
     "left the nightly with the bundle it reads"),
    ("python3 utils/check_doc_blocks.py --plantuml",
     "needs a PlantUML that knows `@startebnf`, which the runner's package predates; the "
     "per-commit step holds the `dot` blocks and the numbering"),
]

# The fuzzer's budget on the dev host; the nightly runs it at a budget that fits.
DEV_HOST_FUZZ_ITERS = 1_000_000
# The per-commit tier's budget, `EIN_FUZZ_ITERS`' default in
# `ein-ir/tests/fuzz_properties.rs`.
PER_COMMIT_FUZZ_ITERS = 2_000

# Test targets that read `EIN_CORPUS_SLOW` and whose slow half runs on no
# tier, with why; each premise is checked in `check_slow_tiers`.
NO_SLOW_TIER = [
    ("read_out_examples",
     "its slow branch adds only the read-out reference's blocks at `--region-budget 33554432`, "
     "and the page has none; a block that adds one needs a step"),
]

SCRATCH_CHECK = 'python3 utils/check_test_scratch.py "${GATE_TMP}"'


class Job:
    def __init__(self, name: str):
        self.name = name
        self.matrix: tuple[str, list[str]] | None = None
        self.runs: list[str] = []


def jobs(text: str) -> tuple[list[Job], list[str]]:
    """The jobs of a workflow, in order, and the parse's complaints. Enough
    YAML for this file's shapes and no more: a job is a two-space key under
    `jobs:`, and a step's `env:` and a job's `matrix:` are one-line flow maps."""
    lines = text.split("\n")
    bad = []
    start = lines.index("jobs:")
    out: list[Job] = []
    for i in range(start + 1, len(lines)):
        line = lines[i]
        indent = len(line) - len(line.lstrip())
        t = line.strip()
        if indent == 2 and t.endswith(":") and not t.startswith("#"):
            out.append(Job(t[:-1]))
            continue
        if not out:
            continue
        job = out[-1]
        if t.startswith("matrix: { "):
            m = t[len("matrix: { "):]
            assert m.endswith(" }") and ": " in m, f"a matrix is a one-line flow map: {t}"
            key, values = m[:-2].split(": ", 1)
            job.matrix = (key, [v.strip().strip('"') for v in values.strip("[]").split(",")])
            continue
        if not t.startswith("- run: "):
            continue
        cmd = t[len("- run: "):]
        if cmd.startswith("|") or cmd.startswith(">"):
            bad.append(f"nightly.yml's job `{job.name}` has a multi-line `run:` at line {i + 1}: "
                       "a step here is one command, so that `run_tests.sh --nightly` can run it "
                       "as one statement")
            continue
        env = ""
        for l in lines[i + 1:]:
            lt = l.strip()
            if not lt or lt.startswith("#") or lt.startswith("- ") \
                    or len(l) - len(l.lstrip()) <= indent:
                break
            e = env_prefix(l)
            if e is not None:
                env = e
                break
        job.runs.append(normalise(env + cmd))
    return out, bad


def nightly_steps(text: str) -> tuple[list[str], list[str], list[Job], list[str]]:
    """Every `run:` the nightly executes, in order, a matrix job expanded once
    per value; every `run:` as written; the jobs; the parse's complaints."""
    all_jobs, bad = jobs(text)
    excused = [normalise(c) for c, _ in NOT_NIGHTLY_STEPS]
    steps, every = [], []
    for job in all_jobs:
        every.extend(job.runs)
        runs = [r for r in job.runs if r not in excused]
        if job.matrix is None:
            steps.extend(runs)
        else:
            key, values = job.matrix
            for v in values:
                for r in runs:
                    steps.append(r.replace("${{ matrix." + key + " }}", v))
    return steps, every, all_jobs, bad


def at_budget(step: str, iters: int) -> str:
    """`step` with its `EIN_FUZZ_ITERS="…"`, if it sets one, read as `iters`."""
    return re.sub(r'EIN_FUZZ_ITERS="[^"]*"', f'EIN_FUZZ_ITERS="{iters}"', step, count=1)


def budget_of(step: str) -> int | None:
    m = re.search(r'EIN_FUZZ_ITERS="(\d+)"', step)
    return int(m.group(1)) if m else None


def unsharded(step: str) -> tuple[str, tuple[int, int] | None]:
    """`step` with its `EIN_TEST_SHARD="k/n" ` taken out, and the shard."""
    m = re.search(r'EIN_TEST_SHARD="([^"]*)"', step)
    if not m:
        return step, None
    k, _, n = m.group(1).partition("/")
    return normalise(step[:m.start()] + step[m.end():]), (int(k), int(n))


def check_nightly() -> list[str]:
    shell = read(SCRIPT)
    nightly, every, all_jobs, bad = nightly_steps(read(NIGHTLY))
    local = shell_steps(shell, "nightly_step")
    dev = shell_steps(shell, "dev_host_step")

    # The two lists are the same list.
    if local != nightly:
        bad.append("`run_tests.sh --nightly` and nightly.yml no longer run the same steps in the "
                   "same order:\n  --nightly:   " + "\n               ".join(local)
                   + "\n  nightly.yml: " + "\n               ".join(nightly))
    if len(local) < 12:
        bad.append(f"only {len(local)} steps parsed out of run_tests.sh's --nightly: the parse, "
                   "not the nightly, is what changed")
    if any("${{" in s for s in nightly):
        bad.append("a `${{ … }}` expression is left in a step after the matrix was expanded, so "
                   "the shell cannot spell it")

    # Every exception names a step the workflow has.
    for cmd, why in NOT_NIGHTLY_STEPS:
        if normalise(cmd) not in every:
            bad.append(f"NOT_NIGHTLY_STEPS excuses `{cmd}` ({why}), and nightly.yml runs no such step")

    # The slow cells and the round trip's slow half, under the gate profile
    # with the flag: a flag dropped from both copies would pass the diff.
    for target, why in [("corpus_cli", "the slow cells' timing band"),
                        ("answer_round_trip", "the heavy_dump and slow entries' dumps")]:
        tail = f"cargo test -p ein-cli --profile gate --test {target}"
        if not any('EIN_CORPUS_SLOW="1"' in s and s.endswith(tail) for s in nightly):
            bad.append(f'no nightly step runs {target} under --profile gate with '
                       f'EIN_CORPUS_SLOW="1": {why} would run on no CI tier')

    # Every job's tests run under the checked TMPDIR, the check after them.
    checked = 0
    for job in all_jobs:
        tests = [i for i, r in enumerate(job.runs) if "cargo test" in r]
        if SCRATCH_CHECK not in job.runs:
            if any('TMPDIR="${GATE_TMP}"' in job.runs[i] for i in tests):
                bad.append(f"`{job.name}` runs a cargo test under the tests' TMPDIR and never "
                           "checks it")
            continue
        at = job.runs.index(SCRATCH_CHECK)
        checked += 1
        if not tests:
            bad.append(f"`{job.name}` runs no cargo test")
        for i in tests:
            if 'TMPDIR="${GATE_TMP}" cargo test' not in job.runs[i]:
                bad.append(f"`{job.name}` runs `{job.runs[i]}` outside the checked TMPDIR")
        if not all(i < at for i in tests):
            bad.append(f"`{job.name}` checks its TMPDIR before its last cargo test")
    if checked == 0:
        bad.append("no nightly job runs utils/check_test_scratch.py")

    # Every sharded step runs every one of its shards, once each.
    shards: dict[str, list[tuple[int, int]]] = {}
    for s in nightly:
        cmd, shard = unsharded(s)
        if shard:
            shards.setdefault(cmd, []).append(shard)
    if not shards:
        bad.append("no nightly step is sharded: the steps split into shards are back in one "
                   "job each")
    for cmd, got in shards.items():
        got.sort()
        n = got[0][1]
        want = [(k, n) for k in range(1, n + 1)]
        if got != want:
            bad.append(f"`{cmd}` is run on the shards {got}, where a sharding into {n} runs each "
                       f"of 1/{n} to {n}/{n} once")

    # The dev host runs the nightly, at its budget, and what left it.
    want = [at_budget(s, DEV_HOST_FUZZ_ITERS) for s in nightly]
    k, extras = 0, []
    for s in dev:
        if k < len(want) and want[k] == s:
            k += 1
        else:
            extras.append(s)
    if k != len(want):
        bad.append(f"`run_tests.sh --dev-host` does not run the nightly's step `{want[k]}`, in the "
                   "nightly's order and at the dev host's budget: the dev host runs every nightly "
                   "step")
    named = [normalise(c) for c, _ in DEV_HOST_ONLY]
    if extras != named:
        bad.append("the dev host's steps that the nightly does not run are not the ones "
                   "DEV_HOST_ONLY names with why, in its order:\n  extra: "
                   + "\n         ".join(extras) + "\n  named: " + "\n         ".join(named))
    if len(dev) < 17:
        bad.append(f"only {len(dev)} steps parsed out of run_tests.sh's --dev-host")

    # Each tier sets the fuzzer's budget, which the diff above reads past.
    for tier, steps in [("nightly.yml", nightly), ("--dev-host", dev)]:
        fuzz = [s for s in steps if "--test fuzz_properties" in s]
        if len(fuzz) != 2:
            bad.append(f"{tier} runs the fuzzer {len(fuzz)} times, where it runs once per seed")
        for s in fuzz:
            n = budget_of(s)
            if n is None:
                bad.append(f"{tier}'s `{s}` sets no EIN_FUZZ_ITERS, so it runs at the per-commit "
                           f"{PER_COMMIT_FUZZ_ITERS}")
            elif tier == "--dev-host" and n != DEV_HOST_FUZZ_ITERS:
                bad.append(f"{tier}'s `{s}` runs at {n}, not the dev host's {DEV_HOST_FUZZ_ITERS}")
            elif tier != "--dev-host" and not PER_COMMIT_FUZZ_ITERS < n < DEV_HOST_FUZZ_ITERS:
                bad.append(f"{tier}'s `{s}`: the nightly's budget is above the per-commit tier's "
                           "and below the dev host's")

    # The dev host's tests' scratch is checked, as the nightly's is.
    last = max((i for i, r in enumerate(dev) if r == SCRATCH_CHECK), default=None)
    if last is None:
        bad.append("--dev-host runs no utils/check_test_scratch.py on the tests' TMPDIR")
    else:
        for i, s in enumerate(dev[:last]):
            if "cargo test" in s:
                if 'TMPDIR="${GATE_TMP}" cargo test' not in s:
                    bad.append(f"--dev-host runs `{s}` outside the checked TMPDIR")
                if SCRATCH_CHECK not in dev[i:]:
                    bad.append(f"--dev-host runs `{s}` with no scratch check after it")
        for s in dev[last:]:
            if 'TMPDIR="${GATE_TMP}"' in s:
                bad.append(f"--dev-host runs `{s}` under the tests' TMPDIR after it was last checked")
        for cmd, _ in DEV_HOST_ONLY:
            if "cargo test" in cmd and normalise(cmd) not in dev[:last]:
                bad.append(f"`{cmd}` runs on the dev host after its last scratch check, or not at all")

    bad.extend(check_slow_tiers(nightly, dev))
    return bad


def test_source(package: str, target: str) -> str:
    return read(f"ein.rs/crates/{package}/tests/{target}.rs")


def slow_readers() -> list[tuple[str, str]]:
    """Every `tests/*.rs` target of the workspace whose code, comments aside,
    reads `EIN_CORPUS_SLOW`, as `(package, target)`."""
    out = []
    for c in sorted((ROOT / "ein.rs/crates").iterdir()):
        tests = c / "tests"
        if not tests.is_dir():
            continue
        for f in sorted(tests.glob("*.rs")):
            text = f.read_text(encoding="utf-8", errors="replace")
            if any('"EIN_CORPUS_SLOW"' in l for l in text.split("\n")
                   if not l.lstrip().startswith("//")):
                out.append((c.name, f.stem))
    return out


def check_slow_tiers(nightly: list[str], dev: list[str]) -> list[str]:
    bad = []
    excused = {normalise(c) for c, _ in DEV_HOST_ONLY}
    # A slow half run by name leaves nothing out.
    named = 0
    for s in (s for s in nightly if "cargo test" in s):
        if " -- " not in s:
            continue
        head, name = s.split(" -- ", 1)
        m = re.search(r" -p (\S+) --profile gate --test (\S+)$", head)
        if not m:
            bad.append(f"`{s}` names a test but no target")
            continue
        package, target = m.groups()
        text = test_source(package, target)
        if f"fn {name}(" not in text:
            bad.append(f"`{s}`: {target} has no test `{name}`")
            continue
        whole = (f'EIN_CORPUS_SLOW="1" TMPDIR="${{GATE_TMP}}" cargo test -p {package} '
                 f"--profile gate --test {target}")
        if whole in dev and whole in excused:
            named += 1
            continue
        code = [(n, l) for n, l in enumerate(text.split("\n")) if not l.lstrip().startswith("//")]
        at = next(i for i, (_, l) in enumerate(code) if l.lstrip().startswith(f"fn {name}("))
        nxt = next((at + 1 + k for k, (_, l) in enumerate(code[at + 1:])
                    if l.startswith("fn ") or l.startswith("#[test]")), len(code))
        reads = [i for i, (_, l) in enumerate(code) if '"EIN_CORPUS_SLOW"' in l]
        if not reads or not all(at < i < nxt for i in reads):
            bad.append(f"`{s}` runs `{name}` alone, and {target} reads EIN_CORPUS_SLOW outside it "
                       f"(lines {[code[i][0] + 1 for i in reads]}): its slow half is more than the "
                       "one test, and the dev host does not run the target whole with the flag")
        named += 1
    if named == 0:
        bad.append("no nightly step names its test: region's slow half is by name")

    # Every slow half runs on a tier.
    readers = slow_readers()
    if len(readers) < 8:
        bad.append(f"only {len(readers)} test targets read EIN_CORPUS_SLOW: {readers}")
    for package, target in readers:
        excuse = next((why for t, why in NO_SLOW_TIER if t == target), None)
        if excuse is not None:
            if target == "read_out_examples" and \
                    "33554432" in read("docs/ein/user/04-reference/09-read-out-examples.md"):
                bad.append(f"NO_SLOW_TIER excuses `{target}` ({excuse}), and the page has a block "
                           "at --region-budget 33554432 now: give the target a step")
            continue
        tail = f"cargo test -p {package} --profile gate --test {target}"

        def runs(steps: list[str]) -> bool:
            return any('EIN_CORPUS_SLOW="1"' in s and (s.endswith(tail) or f"{tail} -- " in s)
                       for s in steps)

        if not (runs(nightly) or runs(dev)):
            bad.append(f"{package}'s `{target}` reads EIN_CORPUS_SLOW, and neither the nightly nor "
                       "--dev-host runs it under --profile gate with the flag, so its slow half "
                       "runs on no tier")
    for target, why in NO_SLOW_TIER:
        if not any(t == target for _, t in readers):
            bad.append(f"NO_SLOW_TIER excuses `{target}` ({why}), which reads no EIN_CORPUS_SLOW")
    return bad


def main() -> int:
    bad = check_gate() + check_tests_only() + check_nightly()
    for b in bad:
        print(f"check_gate_steps: {b}", file=sys.stderr)
    if bad:
        print(f"check_gate_steps: {len(bad)} finding(s)", file=sys.stderr)
        return 1
    shell = read(SCRIPT)
    print(f"check_gate_steps: the gate's {len(shell_steps(shell, 'step'))} steps are "
          f"per-commit.yml's, --nightly's {len(shell_steps(shell, 'nightly_step'))} are "
          f"nightly.yml's, and --dev-host's {len(shell_steps(shell, 'dev_host_step'))} are the "
          "nightly's and what left it")
    return 0


if __name__ == "__main__":
    sys.exit(main())
