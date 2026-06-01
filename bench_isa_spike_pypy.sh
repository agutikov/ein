#!/usr/bin/env bash
#
# S1.7.7 T1.7.7.1 — run the is-a type-pruning de-risk spike under the
# project's PyPy venv (full zebra2 is too slow on CPython). Peer of
# ./bench_solve_monotonic_pypy.sh.
#
# Usage:
#   ./bench_isa_spike_pypy.sh [puzzle.ein] [spike_isa_pruning flags...]
#
# Examples:
#   ./bench_isa_spike_pypy.sh                       # default: examples/zebra2.ein
#   ./bench_isa_spike_pypy.sh --warmup              # prime the JIT first
#   ./bench_isa_spike_pypy.sh --exhaustive          # exact k (slower)
#   ./bench_isa_spike_pypy.sh --help
#
# Setup (one-time):
#   ./venv_install.sh pypy3          # creates .venv-pypy/

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PYPY_VENV="${SCRIPT_DIR}/.venv-pypy"
SPIKE="${SCRIPT_DIR}/ein.py/demo/spike_isa_pruning.py"

if [[ ! -d "${PYPY_VENV}" ]]; then
    echo "error: PyPy venv not found at ${PYPY_VENV}" >&2
    echo "       create it with: ./venv_install.sh pypy3" >&2
    exit 1
fi

if [[ ! -f "${SPIKE}" ]]; then
    echo "error: spike_isa_pruning.py not found at ${SPIKE}" >&2
    exit 1
fi

# shellcheck disable=SC1091
source "${PYPY_VENV}/bin/activate"
exec python "${SPIKE}" "$@"
