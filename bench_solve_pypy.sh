#!/usr/bin/env bash
#
# Run ein.py/demo/bench_solve.py under the project's PyPy venv.
#
# Usage:
#   ./bench_solve_pypy.sh <puzzle.ein> [bench_solve flags...]
#
# Example:
#   ./bench_solve_pypy.sh examples/zebra2-hints.ein --max-depth 3 -v
#
# Setup (one-time):
#   ./venv_install.sh pypy3          # creates .venv-pypy/ (doesn't
#                                    # touch the CPython .venv)
#
# Pipe-target for S1.5a.6 (PyPy compatibility + perf measurement).
# CPython baseline is `python ein.py/demo/bench_solve.py ...`
# inside the regular `.venv`.

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PYPY_VENV="${SCRIPT_DIR}/.venv-pypy"
BENCH="${SCRIPT_DIR}/ein.py/demo/bench_solve.py"

if [[ ! -d "${PYPY_VENV}" ]]; then
    echo "error: PyPy venv not found at ${PYPY_VENV}" >&2
    echo "       create it with: ./venv_install.sh pypy3" >&2
    exit 1
fi

if [[ ! -f "${BENCH}" ]]; then
    echo "error: bench_solve.py not found at ${BENCH}" >&2
    exit 1
fi

# shellcheck disable=SC1091
source "${PYPY_VENV}/bin/activate"
exec python "${BENCH}" "$@"
