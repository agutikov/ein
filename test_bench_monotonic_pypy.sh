#!/usr/bin/env bash
#
# Run ein.py/tests/test_bench_monotonic.py under the project's
# PyPy venv. The test invokes `bench_monotonic.py --help` via
# subprocess using `sys.executable`; running pytest from PyPy
# makes that subprocess PyPy too, so this verifies the bench
# script's imports + arg-parsing are PyPy-compatible.
#
# Usage:
#   ./test_bench_monotonic_pypy.sh [pytest flags...]
#
# Example:
#   ./test_bench_monotonic_pypy.sh -v
#
# Setup (one-time):
#   ./venv_install.sh pypy3          # creates .venv-pypy/
#
# CPython equivalent:
#   .venv/bin/python -m pytest ein.py/tests/test_bench_monotonic.py

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PYPY_VENV="${SCRIPT_DIR}/.venv-pypy"
TEST_FILE="${SCRIPT_DIR}/ein.py/tests/test_bench_monotonic.py"

if [[ ! -d "${PYPY_VENV}" ]]; then
    echo "error: PyPy venv not found at ${PYPY_VENV}" >&2
    echo "       create it with: ./venv_install.sh pypy3" >&2
    exit 1
fi

if [[ ! -f "${TEST_FILE}" ]]; then
    echo "error: test file not found at ${TEST_FILE}" >&2
    exit 1
fi

# shellcheck disable=SC1091
source "${PYPY_VENV}/bin/activate"
exec python -m pytest "${TEST_FILE}" "$@"
