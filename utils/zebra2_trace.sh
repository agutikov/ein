#!/usr/bin/env bash
#
# Solve zebra2 and render its markdown trace (S1.6.4) into build/zebra2/.
#
# The engine is `ein.rs/target/release/ein`; `$EIN_BIN` overrides it. It was
# `.venv-pypy/bin/python -m ein.cli` until M1a S1a.10.4, for the reason the
# PyPy venv existed at all — CPython took >90s over the full zebra2 lattice
# solve where PyPy took ~35s. ein.rs takes ~40ms, so the interpreter choice
# that shaped this script is not a choice any more.
#
# Output:
#   build/zebra2/zebra2.md         — the trace (inline fenced `dot` blocks)
#   build/zebra2/img/stepNNN.svg   — with --svg: each dot block rasterised
#   build/zebra2/zebra2.view.md    — with --svg: the trace with the dot
#                                    blocks replaced by ![](img/…svg) refs,
#                                    viewable in any markdown viewer
#
# Run `utils/zebra2_trace.sh --help` for usage. By default it writes the
# goal-pruned (~11-step) trace; `--full` gives the complete ~560-firing
# saturation log.
#
# Setup (one-time):  cargo build --release --manifest-path ein.rs/Cargo.toml

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "${SCRIPT_DIR}/.." && pwd)"
EIN_BIN="${EIN_BIN:-${REPO_ROOT}/ein.rs/target/release/ein}"
ZEBRA2="${REPO_ROOT}/examples/zebra2.ein"

usage() {
    cat <<'USAGE'
zebra2_trace.sh — solve zebra2, render its markdown trace.

Usage: utils/zebra2_trace.sh [OPTIONS] [OUT_DIR]

Default: the goal-pruned (~11-step) trace → build/zebra2/zebra2.md.

Options:
  --full                  the complete saturation log (~560 firings)
  --svg                   also rasterise each inline dot block to
                          OUT_DIR/img/ + a viewable OUT_DIR/zebra2.view.md
  --reorder               cluster steps by target entity
  --no-diagrams           omit the inline dot blocks
  --full-kb-snapshots     append a whole-KB snapshot of the final state
  -- <flags…>             forward arbitrary flags to `ein solve`
                          (e.g. -- --exhaustive --max-set-size 4)
  -h, --help              show this help

OUT_DIR defaults to build/zebra2. Needs the `ein` binary; build it with
`cargo build --release --manifest-path ein.rs/Cargo.toml`, or name another
one with $EIN_BIN.
USAGE
}

# ── Arg parsing. Default = the goal-pruned trace; --full opts out. ──
WANT_SVG=0
RELEVANT=1
OUT_DIR=""
SOLVE_ARGS=()
forward=0
for arg in "$@"; do
    if (( forward )); then SOLVE_ARGS+=( "${arg}" ); continue; fi
    case "${arg}" in
        -h|--help)  usage; exit 0 ;;
        --full)     RELEVANT=0 ;;
        --relevant) RELEVANT=1 ;;
        --svg)      WANT_SVG=1 ;;
        --reorder|--no-diagrams|--full-kb-snapshots)
                    SOLVE_ARGS+=( "${arg}" ) ;;
        --)         forward=1 ;;
        -*)         echo "unknown option: ${arg} (try --help)" >&2; exit 2 ;;
        *)          OUT_DIR="${arg}" ;;
    esac
done
(( RELEVANT )) && SOLVE_ARGS+=( --relevant )
OUT_DIR="${OUT_DIR:-${REPO_ROOT}/build/zebra2}"
MD="${OUT_DIR}/zebra2.md"

# The engine. `$EIN_BIN` is a *path*, so a build with a feature or an
# allocator arm is named the same way `e2e_baseline.py --bin` names one.
# shellcheck disable=SC2206
EIN=( ${EIN_BIN} )
if ! command -v "${EIN[0]}" >/dev/null 2>&1 && [[ ! -x "${EIN[0]}" ]]; then
    echo "error: ${EIN[0]} is not executable — build it with" >&2
    echo "       cargo build --release --manifest-path ein.rs/Cargo.toml" >&2
    exit 1
fi
# The --svg post-processing is plain Python: it shells out to `dot` and
# rewrites markdown, and imports nothing from this repo.
PYBIN="python3"

if [[ ! -f "${ZEBRA2}" ]]; then
    echo "error: ${ZEBRA2} not found" >&2
    exit 1
fi

mkdir -p "${OUT_DIR}"
echo "solver:  ${EIN[*]}"
echo "puzzle:  ${ZEBRA2}"
echo "trace:   ${MD}"
echo "solve flags: ${SOLVE_ARGS[*]:-<none>}"
echo

# ── Solve + render the markdown trace. ──
"${EIN[@]}" solve "${ZEBRA2}" --trace "${MD}" "${SOLVE_ARGS[@]+"${SOLVE_ARGS[@]}"}"

# ── Optional: rasterise the inline dot blocks for viewing. ──
if (( WANT_SVG )); then
    if ! command -v dot >/dev/null 2>&1; then
        echo "warn: graphviz 'dot' not in PATH; skipping --svg render" >&2
    else
        "${PYBIN}" - "${MD}" "${OUT_DIR}" <<'PY'
import pathlib, re, subprocess, sys
md_path, out_dir = pathlib.Path(sys.argv[1]), pathlib.Path(sys.argv[2])
img = out_dir / "img"; img.mkdir(exist_ok=True)
md = md_path.read_text(encoding="utf-8")
n = 0
def render(m):
    global n
    n += 1
    svg = img / f"step{n:03d}.svg"
    r = subprocess.run(["dot", "-Tsvg", "-o", str(svg)],
                       input=m.group(1), capture_output=True, text=True)
    if r.returncode:
        sys.stderr.write(f"warn: dot failed on block {n}\n")
        return m.group(0)
    return f"![diagram {n}](img/{svg.name})"
view = re.sub(r"```dot\n(.*?)\n```", render, md, flags=re.DOTALL)
(out_dir / "zebra2.view.md").write_text(view, encoding="utf-8")
print(f"rendered {n} dot blocks → {img}/ ; viewable: {out_dir/'zebra2.view.md'}")
PY
    fi
fi

echo
echo "done — ${MD}"
