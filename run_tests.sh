#!/usr/bin/env bash
#
# run_tests.sh — three-phase test runner.
#
#   Phase 1  the pytest unit/integration suite  (ein.py/tests, the testpaths).
#            Run in PARALLEL by default (pytest-xdist, -j workers).
#   Phase 2  the P1.7a acceptance gate           (ein.py/acceptance) — the
#            three zebra2 task-class fixtures solved end-to-end. Slow
#            (~1-2 min each under PyPy) and deliberately OUTSIDE the pytest
#            testpaths, so it is NOT part of the unit suite; it runs AFTER it,
#            as its own phase, SERIALLY with live progress (pytest -s +
#            ProgressDumper) — kept serial so the progress lines don't
#            interleave across workers.
#
#            **M1a S1a.10.2: this phase has been ported and has no successor.**
#            All 21 of its tests are now ein.rs tests inside Phase 3 —
#            ein-infer/tests/acceptance.rs (the 16 engine claims) and
#            ein-cli/tests/acceptance_cli.rs (the 5 CLI ones) — where the same
#            work takes 0.26 s instead of ~40 s. It was a separate phase
#            because it was slow; it is not slow any more, so when ein.py goes
#            (S1a.10.5) the runner loses a phase rather than a check.
#   Phase 3  the ein.rs suite                    (`cargo test --workspace`).
#            Added at M1a S1a.6.11, and not as a courtesy: since S1a.6.10 the
#            parity harness no longer diffs ein.rs's *narration* against
#            ein.py's, so the trace, the `slice` cone and the event stream are
#            covered only by ein.rs's own checked-in goldens. A gate that runs
#            one implementation is no longer the gate. Skipped, loudly, when
#            there is no cargo on PATH, and skipped by --fast.
#            Regenerate a golden with:  EIN_BLESS=1 cargo test --workspace
#
#            **Budget, restated at M1a S1a.10.2: 566 tests in ~1 m 07 s.**
#            It was 312 tests in 9 m 13 s. Nine of those ten minutes were 42
#            of the 91 integration tests starting a `python3` per corpus file
#            — the stage un-differentialled all 42, so the gate stopped
#            paying for a second engine rather than getting faster. No test
#            is marked `slow`; the two that dominate are `dot_wellformed`
#            (~40 s, graphviz over 5 209 renderings) and
#            `id_order_invariance` (~11 s, the corpus twice per seed).
#            `EIN_ID_SEEDS=8` and `EIN_FUZZ_ITERS` raise the two that scale.
#
# The pytest config lives in ein.py/pyproject.toml ([tool.pytest.ini_options]
# — testpaths=tests, pythonpath=src), so both phases invoke pytest from ein.py/.
# (The old root pytest.ini was removed in P1.7a.)
#
# Interpreter: prefers the project PyPy venv (.venv-pypy) — the engine is
# CPU-bound on saturation and PyPy is ~3-6x faster (S1.5a.13). Falls back to
# .venv, then system python3. Override with EIN_PY=/path/to/python.
#
# Flags:
#   -j N | --jobs N     Phase 1 parallel workers (default 4; "auto" = #CPUs;
#                       1 = serial). Needs pytest-xdist (in the dev extra:
#                       pip install -e '.[dev]'); falls back to serial if absent.
#   --fast              Quick run: skip the acceptance gate, the ein.rs suite,
#                       AND the unit suite's EIN_RUN_SLOW-gated tests.
#   --acceptance-only   Phase 2 only — just the acceptance gate (with progress).
#   --no-rust           Skip Phase 3 (the ein.rs suite).
#   -h | --help         This help.
#   <other args>        Forwarded to Phase 1's pytest (e.g. -k, -x, a path).
#
# By default a full run is performed: EIN_RUN_SLOW=1 is set so the unit
# suite's slow zebra tests run in Phase 1, then Phase 2 (acceptance), then
# Phase 3 (ein.rs). --fast turns all three off for a quick inner-loop run.
#
# Usage:
#   ./run_tests.sh                  # 4-way parallel suite, then acceptance
#   ./run_tests.sh -j auto          # one worker per CPU
#   ./run_tests.sh --fast -j8       # quick, 8-way, no acceptance
#   ./run_tests.sh --acceptance-only
#   ./run_tests.sh --no-rust        # the Python half only
set -uo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

FAST=0
ACCEPTANCE_ONLY=0
NO_RUST=0
JOBS=4
ARGS=()
while [[ $# -gt 0 ]]; do
    case "$1" in
        -h|--help)
            sed -n '2,62p' "${BASH_SOURCE[0]}" | sed 's/^# \{0,1\}//'
            exit 0
            ;;
        --fast)            FAST=1 ;;
        --acceptance-only) ACCEPTANCE_ONLY=1 ;;
        --no-rust)         NO_RUST=1 ;;
        -j|--jobs)         shift; JOBS="${1:-4}" ;;
        -j*)               JOBS="${1#-j}" ;;
        --jobs=*)          JOBS="${1#*=}" ;;
        *)                 ARGS+=("$1") ;;
    esac
    shift
done

# Pick interpreter: EIN_PY override > PyPy venv > CPython venv > system.
if [[ -n "${EIN_PY:-}" ]]; then
    PY="${EIN_PY}"
elif [[ -x "${SCRIPT_DIR}/.venv-pypy/bin/python" ]]; then
    PY="${SCRIPT_DIR}/.venv-pypy/bin/python"
elif [[ -x "${SCRIPT_DIR}/.venv/bin/python" ]]; then
    PY="${SCRIPT_DIR}/.venv/bin/python"
else
    PY="python3"
fi

cd "${SCRIPT_DIR}/ein.py"

# Full run by default: enable the unit suite's EIN_RUN_SLOW gates. --fast
# leaves EIN_RUN_SLOW unset (a caller-provided value is still respected).
if [[ "${FAST}" == "0" ]]; then
    export EIN_RUN_SLOW=1
fi

# Parallelism for Phase 1: -n <JOBS> when pytest-xdist is installed and
# JOBS != 1. "auto" and any integer > 1 parallelise; 0/1 stay serial.
PAR=()
if [[ "${JOBS}" =~ ^[0-9]+$ ]] && [[ "${JOBS}" -le 1 ]]; then
    :  # serial
elif "${PY}" -c "import xdist" >/dev/null 2>&1; then
    PAR=(-n "${JOBS}")
else
    echo "note: pytest-xdist not installed — Phase 1 runs serially." >&2
    echo "      install it with:  ${PY} -m pip install -e '.[dev]'" >&2
fi

echo "run_tests.sh: $("${PY}" --version 2>&1 | head -1) @ ${PY}" \
     "(EIN_RUN_SLOW=${EIN_RUN_SLOW:-unset}, jobs=${PAR[*]:-1})" >&2

RC=0

if [[ "${ACCEPTANCE_ONLY}" == "0" ]]; then
    echo "" >&2
    echo "── Phase 1: unit / integration suite (tests/, parallel) ───────" >&2
    "${PY}" -m pytest "${PAR[@]}" "${ARGS[@]}" || RC=$?
fi

if [[ "${FAST}" == "0" ]]; then
    echo "" >&2
    echo "── Phase 2: P1.7a acceptance gate (acceptance/, after the suite) ─" >&2
    echo "   (slow, end-to-end, serial; live progress below)" >&2
    # -s: don't capture, so ProgressDumper's live progress shows.
    # -v: name each task-class test as it runs. Serial (no -n) so the
    #     progress lines stay readable.
    "${PY}" -m pytest -s -v acceptance/ || RC=$?
fi

# Phase 3: ein.rs. `cargo test --workspace` covers the port's unit tests, its
# differential tests against utils/ir_oracle.py, and — since S1a.6.11 — the
# goldens that are the *only* coverage of what the parity contract stopped
# diffing between the two engines.
if [[ "${FAST}" == "0" && "${ACCEPTANCE_ONLY}" == "0" && "${NO_RUST}" == "0" ]]; then
    echo "" >&2
    echo "── Phase 3: the ein.rs suite (cargo test --workspace) ──────────" >&2
    if ! command -v cargo >/dev/null 2>&1; then
        # Loud, and not a pass: a skipped phase that reads as green is how a
        # gate stops being one.
        echo "   SKIPPED: no cargo on PATH — the ein.rs half of the gate did" >&2
        echo "   not run. Install a Rust toolchain, or pass --no-rust to mean" >&2
        echo "   it on purpose." >&2
    else
        # The differential tests shell out to `python3 utils/ir_oracle.py`,
        # which puts `ein.py/src` on its own `sys.path` — so this phase needs
        # a python3 on PATH but not an installed `ein`. When it cannot start
        # one, those tests print SKIP and say so.
        cargo test --manifest-path "${SCRIPT_DIR}/ein.rs/Cargo.toml" \
            --workspace || RC=$?
    fi
fi

exit "${RC}"
