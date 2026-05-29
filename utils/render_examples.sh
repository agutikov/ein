#!/usr/bin/env bash
#
# Render every .ein under examples/ into DOT + SVG.
# Includes nested demo files (examples/zebra/demos/<rule>/<name>.ein);
# skips examples/broken/.
#
# One variant per file (S1.6.0): `ein-bot ir dot` under the project
# defaults — compact entity-style facts/ontology/reasoning, rules as
# side-by-side LHS|RHS clusters (rankdir=LR), per-step trace. The other
# views are reachable per-file via flags/env for the rare cases that
# need them:
#   EIN_RENDER_LEVI=1   — Levi-bipartite hyperedge view
#   EINBOT_RULE_MODE=overlay / EINBOT_TRACE_VIEW=b|c (see below)
# The multi-variant cross-product (rule-mode × trace-view) this script
# used to emit was dropped — the trace dimension is a no-op for the
# example puzzles anyway (they carry no `(trace …)` blocks).
#
# Convention: the Python ein-bot tools emit DOT only (the CLI, the
# S1.6.1-2 `render …` commands, the S1.6.4 trace). Turning that DOT
# into SVG is a shell-script job — that's what this script is for.
# Writes both `.dot` and rendered SVG by default; pass `--no-svg`
# (alias `--dot-only`) to skip rasterising.
#
# The multi-digraph `ir dot` stream is split into one `NN_<name>.dot`
# file per top-level form / rule. **Rule diagrams land in a `rules/`
# subfolder** (numbered on their own); every other form stays flat in
# the example dir. Two whole-example views are added per example:
#   * `_unified.dot` — the PoC-style unified "everything-on-one-page"
#     view (`ein-bot kb dot`, S1.2.4), rendered through `fdp`.
#   * `_lattice.dot` — the commitment-lattice / proof-DAG
#     (`ein-bot render lattice`, S1.6.3), rendered through `dot`. This
#     one RUNS the solver, so it uses the project's **PyPy venv**
#     (`.venv-pypy/`) when present — CPython is too slow for the big
#     puzzles — and is best-effort under a timeout. `--no-lattice` skips
#     it; `EINBOT_LATTICE` overrides the interpreter.
#
# Layout engines per form (per readability):
#   *ontology* / _unified → fdp   (force-directed; instances spread)
#   everything else       → dot   (hierarchical default)
#
# Usage:
#   utils/render_examples.sh                 # .dot + .svg → build/dot/
#   utils/render_examples.sh /tmp/out        # custom output dir
#   utils/render_examples.sh --no-svg        # .dot only
#   utils/render_examples.sh --no-lattice    # skip the solver-run lattice
#   FORMATS="svg pdf" utils/render_examples.sh   # explicit raster formats
#
# Output layout:
#   <out>/<example-rel-path>/NN_<digraph-name>.dot   (+ .<fmt> sibling)
#   <out>/<example-rel-path>/rules/NN_rule_<name>.dot (+ .<fmt> sibling)
#   <out>/<example-rel-path>/_unified.dot            (+ .<fmt> sibling)
#   <out>/<example-rel-path>/_lattice.dot            (+ .<fmt> sibling)
#
# `<example-rel-path>` is the relative path under examples/ minus the
# `.ein` extension — top-level "zebra", nested
# "zebra/demos/symmetric/couple", etc.
#
# Environment overrides:
#   EINBOT          — command for the static renders (default: `ein-bot`
#                     if on PATH, else `python3 -m ein_bot.cli`,
#                     PYTHONPATH=ein.py/src).
#   EINBOT_LATTICE  — command for the solver-run lattice render (default:
#                     `.venv-pypy/bin/python -m ein_bot.cli` when the
#                     PyPy venv exists, else EINBOT).
#   FORMATS         — space-separated Graphviz formats to rasterise
#                     (e.g. "svg pdf"). Defaults to "svg"; `--no-svg`
#                     forces dot-only.
#   LATTICE_TIMEOUT — per-example seconds cap on the lattice solve
#                     (default 60; needs the `timeout` tool).
#
# Skips examples/broken/ — those are intentional parse-failure fixtures.

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "${SCRIPT_DIR}/.." && pwd)"
EXAMPLES_DIR="${REPO_ROOT}/examples"

# ── Arg parsing: raster toggle + optional positional OUT_DIR ──
# SVG is rendered by default (alongside the .dot); --no-svg / --dot-only
# emits .dot only. `--svg` is accepted as an explicit (default) opt-in.
# The commitment-lattice DAG (`render lattice`) RUNS the solver, so it
# is best-effort under a timeout; --no-lattice skips it.
WANT_RASTER=1
WANT_LATTICE=1
OUT_DIR=""
for arg in "$@"; do
    case "${arg}" in
        --svg)               WANT_RASTER=1 ;;
        --no-svg|--dot-only) WANT_RASTER=0 ;;
        --no-lattice)        WANT_LATTICE=0 ;;
        -*)    echo "unknown option: ${arg}" >&2; exit 2 ;;
        *)     OUT_DIR="${arg}" ;;
    esac
done
OUT_DIR="${OUT_DIR:-${REPO_ROOT}/build/dot}"

# Lattice DAG runs a solve per example — bound it so the big puzzles
# (zebra/zebra2) finish on PyPy but not CPython; a timeout still guards
# against a genuine hang. Default 60s: PyPy solves zebra2's lattice in
# ~35s where CPython exceeds 90s.
LATTICE_TIMEOUT="${LATTICE_TIMEOUT:-60}"
if command -v timeout >/dev/null 2>&1; then
    LATTICE_PREFIX=( timeout "${LATTICE_TIMEOUT}" )
else
    LATTICE_PREFIX=()
fi

# FORMATS (explicit raster formats) wins; else SVG unless suppressed.
if [[ -n "${FORMATS:-}" ]]; then
    RASTER_FORMATS="${FORMATS}"
elif (( WANT_RASTER )); then
    RASTER_FORMATS="svg"
else
    RASTER_FORMATS=""
fi

# Pick the ein-bot invocation: console script if installed, else the
# in-tree module entrypoint. The static renders (`ir dot` / `kb dot`)
# are startup-dominated, so they stay on this interpreter.
if [[ -n "${EINBOT:-}" ]]; then
    # shellcheck disable=SC2206
    EIN_CMD=( ${EINBOT} )
elif command -v ein-bot >/dev/null 2>&1; then
    EIN_CMD=( ein-bot )
else
    # In-tree fallback: the package lives under ein.py/src after the
    # P1.11 restructure.
    export PYTHONPATH="${REPO_ROOT}/ein.py/src${PYTHONPATH:+:${PYTHONPATH}}"
    EIN_CMD=( python3 -m ein_bot.cli )
fi

# `render lattice` RUNS the solver — use the project's PyPy venv when
# present (CPython is too slow for the big puzzles; see the
# feedback-use-pypy-bench convention). Falls back to EIN_CMD. Override
# with EINBOT_LATTICE.
PYPY_PYTHON="${REPO_ROOT}/.venv-pypy/bin/python"
if [[ -n "${EINBOT_LATTICE:-}" ]]; then
    # shellcheck disable=SC2206
    LATTICE_CMD=( ${EINBOT_LATTICE} )
elif [[ -x "${PYPY_PYTHON}" ]]; then
    LATTICE_CMD=( "${PYPY_PYTHON}" -m ein_bot.cli )
else
    LATTICE_CMD=( "${EIN_CMD[@]}" )
fi

# Graphviz is only needed when rasterising.
if [[ -n "${RASTER_FORMATS}" ]]; then
    for tool in dot fdp; do
        if ! command -v "${tool}" >/dev/null 2>&1; then
            echo "error: graphviz '${tool}' not in PATH (needed for raster)" >&2
            exit 1
        fi
    done
fi

# Per-run view overrides (rare cases). EIN_RENDER_LEVI is read by the
# CLI directly from the inherited environment; rule-mode / trace-view
# are CLI flags, forwarded here from env for convenience.
IR_DOT_ARGS=()
[[ -n "${EINBOT_RULE_MODE:-}"  ]] && IR_DOT_ARGS+=( --rule-mode  "${EINBOT_RULE_MODE}" )
[[ -n "${EINBOT_TRACE_VIEW:-}" ]] && IR_DOT_ARGS+=( --trace-view "${EINBOT_TRACE_VIEW}" )

echo "ein-bot:   ${EIN_CMD[*]}"
echo "examples:  ${EXAMPLES_DIR}"
echo "output:    ${OUT_DIR}"
echo "raster:    ${RASTER_FORMATS:-<none — .dot only>}"
echo "ir-dot:    ${IR_DOT_ARGS[*]:-<defaults: compact, rule=sidebyside, trace=a>}"
if (( WANT_LATTICE )); then
    echo "lattice:   ${LATTICE_CMD[*]} (timeout ${LATTICE_TIMEOUT}s)"
else
    echo "lattice:   <disabled (--no-lattice)>"
fi
echo

# Pick layout engine by filename. Ontology gets force-directed (fdp);
# everything else uses the default hierarchical `dot`.
engine_for() {
    local name
    name=$(basename "$1" .dot)
    case "${name}" in
        *ontology*) echo fdp ;;
        *)          echo dot ;;
    esac
}

# split_dot_stream <variant-dir>
#
# Reads a multi-digraph DOT stream from stdin and writes one
# `NN_<name>.dot` file per `digraph NAME { ... }` block. Rule diagrams
# (`digraph rule_*`) go into a `rules/` subfolder, numbered on their
# own; every other form stays flat in <variant-dir>. NN is a two-digit
# sequence; <name> comes from the digraph header.
split_dot_stream() {
    local outdir="$1"
    awk -v outdir="${outdir}" '
        BEGIN { ri = 0; oi = 0; file = "" }
        /^digraph[[:space:]]/ {
            name = $2
            sub(/\{$/, "", name)
            gsub(/[^A-Za-z0-9_.-]/, "_", name)
            if (name ~ /^rule_/) {
                ri++
                system("mkdir -p \"" outdir "/rules\"")
                file = sprintf("%s/rules/%02d_%s.dot", outdir, ri, name)
            } else {
                oi++
                file = sprintf("%s/%02d_%s.dot", outdir, oi, name)
            }
            # Truncate any pre-existing file for idempotent re-runs.
            printf "" > file
        }
        file != "" { print >> file }
        /^\}[[:space:]]*$/ {
            if (file != "") close(file)
            file = ""
        }
        END {
            if (file != "") close(file)
        }
    '
}

# render_each <dir>
#
# For each NN_<name>.dot in dir, render every $RASTER_FORMATS sibling
# using the engine chosen by `engine_for()`. No-op when not rasterising.
render_each() {
    local dir="$1"
    [[ -z "${RASTER_FORMATS}" ]] && return 0
    shopt -s nullglob
    local d fmt out engine
    for d in "${dir}"/[0-9][0-9]_*.dot; do
        engine=$(engine_for "${d}")
        for fmt in ${RASTER_FORMATS}; do
            out="${d%.dot}.${fmt}"
            if ! "${engine}" "-T${fmt}" "${d}" -o "${out}" 2>/dev/null; then
                echo "    warn: ${engine} -T${fmt} failed on ${d}" >&2
            fi
        done
    done
    shopt -u nullglob
}

# Recursive discovery — picks up the top-level examples (zebra.ein,
# zebra2.ein, …) AND any nested demo directories under
# examples/zebra/demos/<rule>/<scenario>.ein. Skips examples/broken/
# (intentional parse-failure fixtures).
mapfile -d '' -t ein_files < <(
    find "${EXAMPLES_DIR}" \
        -path "${EXAMPLES_DIR}/broken" -prune -o \
        -name '*.ein' -type f -print0
)

if (( ${#ein_files[@]} == 0 )); then
    echo "no *.ein files found under ${EXAMPLES_DIR}"
    exit 0
fi

total_dots=0
total_imgs=0
total_unified_dot=0
total_unified=0
total_lattice_dot=0
total_lattice=0
raster_first="${RASTER_FORMATS%% *}"
for ein in "${ein_files[@]}"; do
    # Relative path from EXAMPLES_DIR, minus the .ein extension.
    # Top-level files keep their bare stem (e.g. "zebra"); nested
    # demo files preserve their path so outputs don't collide
    # (e.g. "zebra/demos/symmetric/couple").
    rel="${ein#${EXAMPLES_DIR}/}"
    base="${rel%.ein}"
    echo "==> ${ein}"
    out="${OUT_DIR}/${base}"
    mkdir -p "${out}"
    # Wipe stale outputs from a previous run (parent + rules/ subdir).
    find "${out}" -maxdepth 1 -type f \
        \( -name '*.dot' -o -name '*.svg' -o -name '*.pdf' \
           -o -name '*.png' -o -name '*.gv' \) -delete
    rm -rf "${out}/rules"

    "${EIN_CMD[@]}" ir dot "${ein}" "${IR_DOT_ARGS[@]+"${IR_DOT_ARGS[@]}"}" \
        | split_dot_stream "${out}"
    render_each "${out}"
    render_each "${out}/rules"

    shopt -s nullglob
    n_dot=$( ls "${out}"/[0-9][0-9]_*.dot "${out}"/rules/[0-9][0-9]_*.dot \
                2>/dev/null | wc -l )
    n_img=0
    if [[ -n "${raster_first}" ]]; then
        n_img=$( ls "${out}"/[0-9][0-9]_*."${raster_first}" \
                    "${out}"/rules/[0-9][0-9]_*."${raster_first}" \
                    2>/dev/null | wc -l )
    fi
    shopt -u nullglob
    total_dots=$((total_dots + n_dot))
    total_imgs=$((total_imgs + n_img))
    printf "    %2d dot, %2d %s\n" "${n_dot}" "${n_img}" "${raster_first:-(no raster)}"

    # ── Unified KB view (S1.2.4) — one DOT per example, all layers,
    #    rendered with `fdp` to match the PoC's aesthetic. ──
    unified_dot="${out}/_unified.dot"
    if "${EIN_CMD[@]}" kb dot "${ein}" > "${unified_dot}" 2>/dev/null; then
        total_unified_dot=$((total_unified_dot + 1))
        for fmt in ${RASTER_FORMATS}; do
            unified_out="${out}/_unified.${fmt}"
            if fdp "-T${fmt}" "${unified_dot}" -o "${unified_out}" 2>/dev/null; then
                total_unified=$((total_unified + 1))
                printf "    _unified 1 dot, 1 %s (unified KB view, fdp)\n" "${fmt}"
            else
                echo "    warn: fdp -T${fmt} failed on ${unified_dot}" >&2
            fi
        done
    else
        echo "    warn: 'kb dot' failed on ${ein}; skipping unified view" >&2
    fi

    # ── Commitment-lattice / proof-DAG (S1.6.3) — RUNS a solve, so it
    #    is best-effort under LATTICE_TIMEOUT; the big puzzles skip. ──
    if (( WANT_LATTICE )); then
        lattice_dot="${out}/_lattice.dot"
        if "${LATTICE_PREFIX[@]+"${LATTICE_PREFIX[@]}"}" \
                "${LATTICE_CMD[@]}" render lattice "${ein}" \
                > "${lattice_dot}" 2>/dev/null && [[ -s "${lattice_dot}" ]]; then
            total_lattice_dot=$((total_lattice_dot + 1))
            for fmt in ${RASTER_FORMATS}; do
                # Lattice is rankdir=TB → hierarchical `dot`, not fdp.
                if dot "-T${fmt}" "${lattice_dot}" -o "${out}/_lattice.${fmt}" \
                        2>/dev/null; then
                    total_lattice=$((total_lattice + 1))
                    printf "    _lattice 1 dot, 1 %s (commitment lattice, dot)\n" "${fmt}"
                else
                    echo "    warn: dot -T${fmt} failed on ${lattice_dot}" >&2
                fi
            done
        else
            echo "    note: lattice skipped (slow >${LATTICE_TIMEOUT}s / no proof)" \
                 "for ${rel}" >&2
            rm -f "${lattice_dot}"
        fi
    fi
done

echo
echo "done — ${total_dots} per-form DOT files (+${total_unified_dot} unified,"
echo "       +${total_lattice_dot} lattice), ${total_imgs} per-form renders,"
echo "       ${total_unified} unified + ${total_lattice} lattice renders"
echo "       under ${OUT_DIR}"
