#!/usr/bin/env bash
#
# Render every examples/*.ein into DOT + SVG variants.
#
# For each input file, the script invokes `ein-bot ir dot` with every
# combination of --rule-mode (a, c) × --trace-view (a, b, c) — six
# variants per file — splits the multi-digraph output into one DOT
# file per top-level form / rule, renders each through Graphviz, AND
# emits a combined `_all.svg` per variant via `gvpack` showing every
# form + every rule on the same page.
#
# Layout engines per form (per docs/ir.md §6 hints + readability):
#   ontology → fdp   (force-directed; clusters and instances spread)
#   facts    → dot   (hierarchical default)
#   reasoning → dot
#   query    → dot
#   rule_*   → dot
#   trace    → dot
#   _all     → neato -n2 (uses gvpack-assigned positions verbatim)
#
# Usage:
#   utils/render_examples.sh                  # default output: build/dot/
#   utils/render_examples.sh /tmp/out         # custom output dir
#   FORMATS="svg pdf" utils/render_examples.sh
#
# Layout of output:
#   <out>/<example>/rule-<rm>_trace-<tv>/NN_<digraph-name>.dot
#   <out>/<example>/rule-<rm>_trace-<tv>/NN_<digraph-name>.svg
#   <out>/<example>/rule-<rm>_trace-<tv>/_all.dot         (gvpack-merged)
#   <out>/<example>/rule-<rm>_trace-<tv>/_all.svg         (combined render)
#
# Environment overrides:
#   EINBOT     — command to invoke (default: `ein-bot` if on PATH,
#                else `python3 -m ein_bot.cli` with PYTHONPATH=src).
#   FORMATS    — space-separated Graphviz output formats per DOT
#                (default: "svg"). Each format produces a sibling
#                file next to the .dot.
#
# Skips examples/broken/ — those are intentional parse-failure fixtures.

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "${SCRIPT_DIR}/.." && pwd)"
EXAMPLES_DIR="${REPO_ROOT}/examples"
OUT_DIR="${1:-${REPO_ROOT}/build/dot}"

FORMATS="${FORMATS:-svg}"
RULE_MODES=(a c)
TRACE_VIEWS=(a b c)

# Pick the ein-bot invocation: console script if installed, else the
# in-tree module entrypoint.
if [[ -n "${EINBOT:-}" ]]; then
    # shellcheck disable=SC2206
    EIN_CMD=( ${EINBOT} )
elif command -v ein-bot >/dev/null 2>&1; then
    EIN_CMD=( ein-bot )
else
    export PYTHONPATH="${REPO_ROOT}/src${PYTHONPATH:+:${PYTHONPATH}}"
    EIN_CMD=( python3 -m ein_bot.cli )
fi

for tool in dot fdp gvpack neato; do
    if ! command -v "${tool}" >/dev/null 2>&1; then
        echo "error: graphviz '${tool}' not in PATH" >&2
        exit 1
    fi
done

echo "ein-bot:   ${EIN_CMD[*]}"
echo "examples:  ${EXAMPLES_DIR}"
echo "output:    ${OUT_DIR}"
echo "formats:   ${FORMATS}"
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
# `NN_<name>.dot` file per `digraph NAME { ... }` block to the given
# directory. NN is a two-digit sequence; <name> comes from the
# digraph header.
split_dot_stream() {
    local outdir="$1"
    awk -v outdir="${outdir}" '
        BEGIN { i = 0; file = "" }
        /^digraph[[:space:]]/ {
            i++
            name = $2
            sub(/\{$/, "", name)
            gsub(/[^A-Za-z0-9_.-]/, "_", name)
            file = sprintf("%s/%02d_%s.dot", outdir, i, name)
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
# For each NN_<name>.dot in dir, render every $FORMATS sibling using
# the engine chosen by `engine_for()`.
render_each() {
    local dir="$1"
    shopt -s nullglob
    local d fmt out engine
    for d in "${dir}"/[0-9][0-9]_*.dot; do
        engine=$(engine_for "${d}")
        for fmt in ${FORMATS}; do
            out="${d%.dot}.${fmt}"
            if ! "${engine}" "-T${fmt}" "${d}" -o "${out}" 2>/dev/null; then
                echo "    warn: ${engine} -T${fmt} failed on ${d}" >&2
            fi
        done
    done
    shopt -u nullglob
}

# render_combined <dir>
#
# Pre-layout every NN_<name>.dot in <dir> with its preferred engine
# (writing intermediate .gv files), pack them with gvpack into a
# single graph, then re-render through neato -n2 (use existing
# positions) for every requested format. Outputs `_all.<fmt>` and
# leaves `_all.dot` as the merged source.
render_combined() {
    local dir="$1"
    shopt -s nullglob
    local layouts=()
    local d engine gv
    for d in "${dir}"/[0-9][0-9]_*.dot; do
        engine=$(engine_for "${d}")
        gv="${d%.dot}.gv"
        if ! "${engine}" "${d}" -o "${gv}" 2>/dev/null; then
            echo "    warn: ${engine} layout failed on ${d}" >&2
            continue
        fi
        layouts+=( "${gv}" )
    done

    if (( ${#layouts[@]} == 0 )); then
        shopt -u nullglob
        return
    fi

    local all_dot="${dir}/_all.dot"
    if ! gvpack -array_3 "${layouts[@]}" > "${all_dot}" 2>/dev/null; then
        echo "    warn: gvpack failed in ${dir}" >&2
        rm -f "${layouts[@]}"
        shopt -u nullglob
        return
    fi

    local fmt out
    for fmt in ${FORMATS}; do
        out="${dir}/_all.${fmt}"
        if ! neato -n2 "-T${fmt}" "${all_dot}" -o "${out}" 2>/dev/null; then
            echo "    warn: neato -n2 -T${fmt} failed on ${all_dot}" >&2
        fi
    done

    # Clean intermediate per-form .gv files; keep _all.dot for inspection.
    rm -f "${layouts[@]}"
    shopt -u nullglob
}

shopt -s nullglob
ein_files=( "${EXAMPLES_DIR}"/*.ein )
shopt -u nullglob

if (( ${#ein_files[@]} == 0 )); then
    echo "no examples/*.ein found"
    exit 0
fi

total_dots=0
total_imgs=0
total_combined=0
for ein in "${ein_files[@]}"; do
    base="$(basename "${ein}" .ein)"
    echo "==> ${ein}"
    for rmode in "${RULE_MODES[@]}"; do
        for tview in "${TRACE_VIEWS[@]}"; do
            variant="rule-${rmode}_trace-${tview}"
            out="${OUT_DIR}/${base}/${variant}"
            mkdir -p "${out}"
            # Wipe stale outputs from a previous run.
            find "${out}" -maxdepth 1 -type f \
                \( -name '*.dot' -o -name '*.svg' -o -name '*.pdf' \
                   -o -name '*.png' -o -name '*.gv' \) -delete

            "${EIN_CMD[@]}" ir dot "${ein}" \
                --rule-mode="${rmode}" --trace-view="${tview}" \
                | split_dot_stream "${out}"

            render_each "${out}"
            render_combined "${out}"

            shopt -s nullglob
            n_dot=$( ls "${out}"/[0-9][0-9]_*.dot 2>/dev/null | wc -l )
            n_img=$( ls "${out}"/[0-9][0-9]_*.${FORMATS%% *} 2>/dev/null | wc -l )
            n_all=$( ls "${out}"/_all.${FORMATS%% *} 2>/dev/null | wc -l )
            shopt -u nullglob
            total_dots=$((total_dots + n_dot))
            total_imgs=$((total_imgs + n_img))
            total_combined=$((total_combined + n_all))
            printf "    %-22s %2d dot, %2d %s + %d combined\n" \
                "${variant}" "${n_dot}" "${n_img}" "${FORMATS%% *}" "${n_all}"
        done
    done
done

echo
echo "done — ${total_dots} per-form DOT files, ${total_imgs} per-form renders,"
echo "       ${total_combined} combined _all renders under ${OUT_DIR}"
