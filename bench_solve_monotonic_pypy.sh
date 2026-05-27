#!/usr/bin/env bash
#
# Run ein.py/demo/bench_monotonic.py under the project's PyPy
# venv — the monotonic engine's PyPy entry point, peer of
# ./bench_solve_pypy.sh (which runs the tree engine).
#
# Usage:
#   ./bench_solve_monotonic_pypy.sh <puzzle.ein> [bench_monotonic flags...]
#
# Examples:
#   ./bench_solve_monotonic_pypy.sh examples/zebra2.ein --max-set-size 2
#   ./bench_solve_monotonic_pypy.sh examples/branching/03_five_hyps_one_alive.ein
#
# Setup (one-time):
#   ./venv_install.sh pypy3          # creates .venv-pypy/
#
# CPython equivalent:
#   .venv/bin/python ein.py/demo/bench_monotonic.py <puzzle.ein> [flags...]
#
# Why a PyPy bench runner: the tree-side comparison
# (S1.5a.13) measured a 6x speedup; the monotonic engine
# exercises the same Saturator + hypgen kernels, so PyPy is
# the natural fast-path. The pytest suite (which includes
# tests/inference/monotonic/) already runs under CPython
# during normal CI; this script is for ad-hoc puzzle solves
# where wall-clock matters.

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PYPY_VENV="${SCRIPT_DIR}/.venv-pypy"
BENCH="${SCRIPT_DIR}/ein.py/demo/bench_monotonic.py"

if [[ ! -d "${PYPY_VENV}" ]]; then
    echo "error: PyPy venv not found at ${PYPY_VENV}" >&2
    echo "       create it with: ./venv_install.sh pypy3" >&2
    exit 1
fi

if [[ ! -f "${BENCH}" ]]; then
    echo "error: bench_monotonic.py not found at ${BENCH}" >&2
    exit 1
fi

# shellcheck disable=SC1091
source "${PYPY_VENV}/bin/activate"
exec python "${BENCH}" "$@"
