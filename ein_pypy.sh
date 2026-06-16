#!/usr/bin/env bash
#
# Run `ein <subcommand> …` under the project's PyPy venv. The engine is
# CPU-bound on saturation and PyPy is ~3-6x faster than CPython (S1.5a.13),
# so ad-hoc solves / benches where wall-clock matters want PyPy. This is the
# single generic runner that replaced the per-command bench_*_pypy.sh
# wrappers (P1.11 follow-up).
#
# Usage:
#   ./ein_pypy.sh <subcommand> [args…]
#
# Examples:
#   ./ein_pypy.sh solve    examples/zebra2.ein --print-final-hfacts
#   ./ein_pypy.sh solve    examples/zebra2.ein --exhaustive --stats
#   ./ein_pypy.sh solve    examples/zebra2.ein --trace /tmp/zebra.md
#   ./ein_pypy.sh saturate examples/saturation/transitive/taxonomy.ein
#
# Setup (one-time):
#   ./venv_install.sh pypy3          # creates .venv-pypy/
#
# CPython equivalent:
#   .venv/bin/python -m ein.cli <subcommand> [args…]

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
exec python -m ein.cli "$@"
