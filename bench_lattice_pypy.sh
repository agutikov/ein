#!/usr/bin/env bash
#
# Run `ein lattice` (the promoted bench_lattice command, P1.11 S1.11.3)
# under the project's PyPy venv — the lattice engine's PyPy entry point,
# peer of ./bench_solve_monotonic_pypy.sh (set-search).
#
# Usage:
#   ./bench_lattice_pypy.sh <puzzle.ein> [ein lattice flags...]
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
#   .venv/bin/python -m ein.cli lattice <puzzle.ein> [flags...]
#
# Both lattice entries (gaps_solve, contradictions_solve)
# exercise the same Saturator + Apriori-gen kernels as the
# set-search engine, so PyPy is the natural fast-path. The
# pytest suite (which includes
# tests/inference/lattice/) already runs under CPython during
# normal CI; this script is for ad-hoc puzzle solves where
# wall-clock matters — notably the S1.5b.30 zebra2 perf round.

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PYPY_VENV="${SCRIPT_DIR}/.venv-pypy"

if [[ ! -d "${PYPY_VENV}" ]]; then
    echo "error: PyPy venv not found at ${PYPY_VENV}" >&2
    echo "       create it with: ./venv_install.sh pypy3" >&2
    exit 1
fi

# shellcheck disable=SC1091
source "${PYPY_VENV}/bin/activate"
exec python -m ein.cli lattice "$@"
