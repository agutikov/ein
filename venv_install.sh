#!/usr/bin/env bash
#
# Create .venv/, install ein-bot in editable mode with dev extras, and
# leave the `ein-bot` console script on the venv's PATH.
#
# Usage:
#   ./venv_install.sh                # creates .venv (uses python3) and installs
#   ./venv_install.sh /usr/bin/python3.12   # pin a specific interpreter
#   ./venv_install.sh pypy3          # creates .venv-pypy (separate dir,
#                                    # doesn't touch the CPython .venv)
#
# The venv dir is auto-picked from the interpreter: pypy* → .venv-pypy,
# anything else → .venv. So the CPython and PyPy venvs coexist.
#
# Re-running is safe: existing venv dir is reused, pip upgrades in-place.
#
# After install:
#   source .venv/bin/activate         # CPython
#   source .venv-pypy/bin/activate    # PyPy
#   ein-bot ir parse examples/zebra.ein | head
#   pytest

set -euo pipefail

PYTHON="${1:-python3}"
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

# Auto-pick venv dir from interpreter name so pypy and cpython coexist.
python_basename="$(basename "${PYTHON}")"
if [[ "${python_basename}" == pypy* ]]; then
    VENV_DIR="${SCRIPT_DIR}/.venv-pypy"
else
    VENV_DIR="${SCRIPT_DIR}/.venv"
fi

if ! command -v "${PYTHON}" >/dev/null 2>&1; then
    echo "error: interpreter '${PYTHON}' not found in PATH" >&2
    exit 1
fi

py_version="$("${PYTHON}" -c 'import sys; print("%d.%d" % sys.version_info[:2])')"
required_major=3
required_minor=10
read -r have_major have_minor <<<"$(echo "${py_version}" | tr '.' ' ')"
if (( have_major < required_major )) \
   || (( have_major == required_major && have_minor < required_minor )); then
    echo "error: ein-bot requires Python >= ${required_major}.${required_minor};"
    echo "       '${PYTHON}' reports ${py_version}" >&2
    exit 1
fi

if [[ ! -d "${VENV_DIR}" ]]; then
    echo "creating venv at ${VENV_DIR} (python ${py_version})"
    "${PYTHON}" -m venv "${VENV_DIR}"
else
    echo "reusing existing venv at ${VENV_DIR}"
fi

# shellcheck disable=SC1091
source "${VENV_DIR}/bin/activate"

echo "upgrading pip / setuptools / wheel"
python -m pip install --quiet --upgrade pip setuptools wheel

echo "installing ein-bot (editable) with dev extras"
python -m pip install --quiet -e "${SCRIPT_DIR}/ein.py[dev]"

venv_rel="$(realpath --relative-to="${SCRIPT_DIR}" "${VENV_DIR}" 2>/dev/null || echo "${VENV_DIR}")"
echo
echo "done."
echo "  activate:   source ${venv_rel}/bin/activate"
echo "  cli:        ein-bot ir parse examples/zebra.ein | head"
echo "  tests:      pytest"
echo "  lint:       ruff check ."
