#!/usr/bin/env bash
#
# Render every .ein under examples/ into DOT + SVG variants.
# Includes nested demo files (examples/zebra/demos/<rule>/<name>.ein);
# skips examples/broken/.
#
# For each input file, the script invokes `ein-bot ir dot` with every
# combination of --rule-mode (a, c) × --trace-view (a, b, c) — six
# variants per file — splits the multi-digraph output into one DOT
# file per top-level form / rule, and renders each through Graphviz.
#
# Layout engines per form (per docs/ir.md §6 hints + readability):
#   ontology → fdp   (force-directed; clusters and instances spread)
#   facts    → dot   (hierarchical default)
#   reasoning → dot
#   query    → dot
#   rule_*   → dot
#   trace    → dot
#
# The **PoC-style unified "everything-on-one-page" view** is generated
# by `ein-bot kb dot` (S1.2.4) — one unified DOT per example,
# rendered through fdp (force-directed) for the PoC aesthetic. Output
# files: `<out>/<example>/_unified.dot` + `.svg`. gvpack-based packing
# was tried and rejected — it stacks independent graphs side-by-side
# rather than merging them.
#
# Usage:
#   utils/render_examples.sh                  # default output: build/dot/
#   utils/render_examples.sh /tmp/out         # custom output dir
#   FORMATS="svg pdf" utils/render_examples.sh
#
# Layout of output:
#   <out>/<example-rel-path>/rule-<rm>_trace-<tv>/NN_<digraph-name>.dot
#   <out>/<example-rel-path>/rule-<rm>_trace-<tv>/NN_<digraph-name>.svg
#
# `<example-rel-path>` is the relative path under examples/ minus
# the `.ein` extension — top-level "zebra", nested
# "zebra/demos/symmetric/couple", etc.
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

for tool in dot fdp; do
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
total_unified=0
for ein in "${ein_files[@]}"; do
    # Relative path from EXAMPLES_DIR, minus the .ein extension.
    # Top-level files keep their bare stem (e.g. "zebra"); nested
    # demo files preserve their path so outputs don't collide
    # (e.g. "zebra/demos/symmetric/couple").
    rel="${ein#${EXAMPLES_DIR}/}"
    base="${rel%.ein}"
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

            shopt -s nullglob
            n_dot=$( ls "${out}"/[0-9][0-9]_*.dot 2>/dev/null | wc -l )
            n_img=$( ls "${out}"/[0-9][0-9]_*.${FORMATS%% *} 2>/dev/null | wc -l )
            shopt -u nullglob
            total_dots=$((total_dots + n_dot))
            total_imgs=$((total_imgs + n_img))
            printf "    %-22s %2d dot, %2d %s\n" \
                "${variant}" "${n_dot}" "${n_img}" "${FORMATS%% *}"
        done
    done

    # ── Unified KB view (S1.2.4) — one DOT per example, all layers,
    #    rendered with `fdp` to match the PoC's aesthetic. ──
    unified_dir="${OUT_DIR}/${base}"
    mkdir -p "${unified_dir}"
    unified_dot="${unified_dir}/_unified.dot"
    if "${EIN_CMD[@]}" kb dot "${ein}" > "${unified_dot}" 2>/dev/null; then
        for fmt in ${FORMATS}; do
            unified_out="${unified_dir}/_unified.${fmt}"
            if fdp "-T${fmt}" "${unified_dot}" -o "${unified_out}" 2>/dev/null; then
                total_unified=$((total_unified + 1))
                printf "    %-22s 1 dot, 1 %s (unified KB view, fdp)\n" \
                    "_unified" "${fmt}"
            else
                echo "    warn: fdp -T${fmt} failed on ${unified_dot}" >&2
            fi
        done
    else
        echo "    warn: 'kb dot' failed on ${ein}; skipping unified view" >&2
    fi
done

echo
echo "done — ${total_dots} per-form DOT files, ${total_imgs} per-form renders,"
echo "       ${total_unified} unified-KB renders under ${OUT_DIR}"
