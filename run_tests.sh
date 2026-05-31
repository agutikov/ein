#!/usr/bin/env bash
#
# run_tests.sh — two-phase test runner.
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
#   --fast              Quick run: skip the acceptance gate AND the unit
#                       suite's EIN_RUN_SLOW-gated tests.
#   --acceptance-only   Phase 2 only — just the acceptance gate (with progress).
#   -h | --help         This help.
#   <other args>        Forwarded to Phase 1's pytest (e.g. -k, -x, a path).
#
# By default a full run is performed: EIN_RUN_SLOW=1 is set so the unit
# suite's slow zebra tests run in Phase 1, and Phase 2 (acceptance) runs
# after. --fast turns both off for a quick inner-loop run.
#
# Usage:
#   ./run_tests.sh                  # 4-way parallel suite, then acceptance
#   ./run_tests.sh -j auto          # one worker per CPU
#   ./run_tests.sh --fast -j8       # quick, 8-way, no acceptance
#   ./run_tests.sh --acceptance-only
set -uo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

FAST=0
ACCEPTANCE_ONLY=0
JOBS=4
ARGS=()
while [[ $# -gt 0 ]]; do
    case "$1" in
        -h|--help)
            sed -n '2,41p' "${BASH_SOURCE[0]}" | sed 's/^# \{0,1\}//'
            exit 0
            ;;
        --fast)            FAST=1 ;;
        --acceptance-only) ACCEPTANCE_ONLY=1 ;;
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

exit "${RC}"
