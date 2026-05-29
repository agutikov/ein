#!/usr/bin/env bash
#
# Run ein.py/demo/bench_lattice.py under the project's PyPy
# venv — the lattice engine's PyPy entry point, peer of
# ./bench_solve_pypy.sh (tree) and ./bench_solve_monotonic_pypy.sh
# (monotonic).
#
# Usage:
#   ./bench_lattice_pypy.sh <puzzle.ein> [bench_lattice flags...]
#
# Examples:
#   ./bench_lattice_pypy.sh examples/zebra2.ein --gaps --max-set-size 4
#   ./bench_lattice_pypy.sh examples/zebra2.ein --contradictions \
#       --store-lattice --max-set-size 3
#
# Setup (one-time):
#   ./venv_install.sh pypy3          # creates .venv-pypy/
#
# CPython equivalent:
#   .venv/bin/python ein.py/demo/bench_lattice.py <puzzle.ein> [flags...]
#
# Both lattice entries (gaps_solve, contradictions_solve)
# exercise the same Saturator + Apriori-gen kernels as the
# monotonic engine, so PyPy is the natural fast-path. The
# pytest suite (which includes
# tests/inference/lattice/) already runs under CPython during
# normal CI; this script is for ad-hoc puzzle solves where
# wall-clock matters — notably the S1.5b.30 zebra2 perf round.

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PYPY_VENV="${SCRIPT_DIR}/.venv-pypy"
BENCH="${SCRIPT_DIR}/ein.py/demo/bench_lattice.py"

if [[ ! -d "${PYPY_VENV}" ]]; then
    echo "error: PyPy venv not found at ${PYPY_VENV}" >&2
    echo "       create it with: ./venv_install.sh pypy3" >&2
    exit 1
fi

if [[ ! -f "${BENCH}" ]]; then
    echo "error: bench_lattice.py not found at ${BENCH}" >&2
    exit 1
fi

# shellcheck disable=SC1091
source "${PYPY_VENV}/bin/activate"
exec python "${BENCH}" "$@"
