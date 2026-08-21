#!/usr/bin/env bash
#
# Render every .ein under examples/ into DOT + SVG.
# Includes nested demo files (examples/zebra/demos/<rule>/<name>.ein);
# skips examples/broken/.
#
# Three views per example, all of them `ein render`:
#
#   rules/NN_rule_<name>.dot   one digraph per rule, LHS|RHS side-by-side
#                              clusters (`--rule-mode overlay` for the other)
#   _constraints.dot           the constraint-scope view (S1.6.2)
#   _lattice.dot               the commitment-lattice / proof-DAG (S1.6.3).
#                              This one RUNS a solve, so it is best-effort
#                              under a timeout; `--no-lattice` skips it.
#
# Convention: `ein` emits DOT only. Turning that DOT into SVG is a
# shell-script job — that's what this script is for. Writes both `.dot` and
# rendered SVG by default; pass `--no-svg` (alias `--dot-only`) to skip
# rasterising.
#
# ── What this used to render, and does not ──────────────────────────────
#
# Until M1a S1a.10.4 the headline output was the **IR-graph** view — one
# `NN_<name>.dot` per top-level form — plus `_unified.dot`, the whole-KB
# "everything on one page" view. Neither is on the CLI: `ein ir dot` and
# `ein kb dot` were removed in P1.11 and never came back, so this script
# reached `ein.ir.to_dot` and `KnowledgeBase.to_dot` through `python3 -c`,
# which is why it wanted `PYTHONPATH` as well as an engine.
#
# The Python engine left the tree at
# [P1a.10](../plans/m1a_rust/p1a.10_single_implementation/README.md) and the
# workaround left with it. Both renderers are **ported and alive** —
# `ein_render::ir_dot` and `ein_render::kb_dot`, seventeen views between them,
# rendered over the whole corpus by `ein-render/tests/dot_wellformed.rs` — but
# nothing outside a test can ask for one. Making them browsable again means
# putting them back on the CLI (`ein render ir|kb`), which is a decision about
# the shipping surface and not one a `utils/` clean-up should take.
#
# Layout engines per view (per readability):
#   *ontology* / *constraints* → fdp   (force-directed; instances spread)
#   everything else            → dot   (hierarchical default)
#
# Usage:
#   utils/render_examples.sh                 # .dot + .svg → build/dot/
#   utils/render_examples.sh /tmp/out        # custom output dir
#   utils/render_examples.sh --no-svg        # .dot only
#   utils/render_examples.sh --no-lattice    # skip the solver-run lattice
#   FORMATS="svg pdf" utils/render_examples.sh   # explicit raster formats
#
# Output layout:
#   <out>/<example-rel-path>/rules/NN_rule_<name>.dot (+ .<fmt> sibling)
#   <out>/<example-rel-path>/_constraints.dot         (+ .<fmt> sibling)
#   <out>/<example-rel-path>/_lattice.dot             (+ .<fmt> sibling)
#
# `<example-rel-path>` is the relative path under examples/ minus the
# `.ein` extension — top-level "zebra", nested
# "zebra/demos/symmetric/couple", etc.
#
# Environment overrides:
#   EIN_BIN         — the engine (default: ein.rs/target/release/ein). A path,
#                     so a feature or allocator build is named the same way
#                     `e2e_baseline.py --bin` names one.
#   EIN_RULE_MODE   — `sidebyside` (default) or `overlay`, forwarded to
#                     `ein render rules --rule-mode`.
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
EIN_BIN="${EIN_BIN:-${REPO_ROOT}/ein.rs/target/release/ein}"
RULE_MODE="${EIN_RULE_MODE:-sidebyside}"

usage() {
    cat <<'USAGE'
render_examples.sh — render every examples/*.ein to DOT + SVG.

Usage: utils/render_examples.sh [OPTIONS] [OUT_DIR]

Default: one .dot + SVG per rule (in a rules/ subfolder), the
constraint-scope view, and the commitment-lattice DAG (runs a solve).

Options:
  --no-svg, --dot-only    emit .dot only (skip rasterising)
  --no-lattice            skip the solver-run lattice DAG
  -h, --help              show this help

OUT_DIR defaults to build/dot. Env: EIN_BIN, EIN_RULE_MODE, FORMATS,
LATTICE_TIMEOUT (see the file header).
USAGE
}

# ── Arg parsing: raster + lattice toggles + optional positional OUT_DIR.
# SVG and the lattice DAG are on by default; --no-svg / --no-lattice opt out.
WANT_RASTER=1
WANT_LATTICE=1
OUT_DIR=""
for arg in "$@"; do
    case "${arg}" in
        -h|--help)           usage; exit 0 ;;
        --svg)               WANT_RASTER=1 ;;
        --no-svg|--dot-only) WANT_RASTER=0 ;;
        --no-lattice)        WANT_LATTICE=0 ;;
        -*)    echo "unknown option: ${arg} (try --help)" >&2; exit 2 ;;
        *)     OUT_DIR="${arg}" ;;
    esac
done
OUT_DIR="${OUT_DIR:-${REPO_ROOT}/build/dot}"

# The lattice DAG runs a solve per example. Bound it: `render lattice` on a
# puzzle whose domain the demo never narrows is a blind enumeration, and the
# corpus has one that takes 83 s (square-unique/cul-de-sac.ein). 60s default.
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

# shellcheck disable=SC2206
EIN_CMD=( ${EIN_BIN} )
if ! command -v "${EIN_CMD[0]}" >/dev/null 2>&1 && [[ ! -x "${EIN_CMD[0]}" ]]; then
    echo "error: ${EIN_CMD[0]} is not executable — build it with" >&2
    echo "       cargo build --release --manifest-path ein.rs/Cargo.toml" >&2
    exit 1
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

echo "ein:       ${EIN_CMD[*]}"
echo "examples:  ${EXAMPLES_DIR}"
echo "output:    ${OUT_DIR}"
echo "raster:    ${RASTER_FORMATS:-<none — .dot only>}"
echo "rule mode: ${RULE_MODE}"
if (( WANT_LATTICE )); then
    echo "lattice:   on (timeout ${LATTICE_TIMEOUT}s)"
else
    echo "lattice:   <disabled (--no-lattice)>"
fi
echo

# Pick layout engine by filename. Constraint / ontology views get
# force-directed (fdp); everything else uses the default hierarchical `dot`.
engine_for() {
    local name
    name=$(basename "$1" .dot)
    case "${name}" in
        *ontology*|*constraints*) echo fdp ;;
        *)                        echo dot ;;
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

# count_files <example-dir> <glob>
#
# How many files under <example-dir> and its rules/ subdir match <glob>.
count_files() {
    find "$1" -maxdepth 2 -type f -name "$2" 2>/dev/null | wc -l
}

# render_one <dot-file>
#
# Rasterise a single whole-example view (_constraints, _lattice) with the
# engine `engine_for` picks. Prints one line per format.
render_one() {
    local d="$1" engine fmt
    engine=$(engine_for "${d}")
    for fmt in ${RASTER_FORMATS}; do
        if "${engine}" "-T${fmt}" "${d}" -o "${d%.dot}.${fmt}" 2>/dev/null; then
            printf "    %s 1 dot, 1 %s (%s)\n" "$(basename "${d}" .dot)" \
                   "${fmt}" "${engine}"
        else
            echo "    warn: ${engine} -T${fmt} failed on ${d}" >&2
        fi
    done
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
total_constraints=0
total_lattice=0
raster_first="${RASTER_FORMATS%% *}"
for ein in "${ein_files[@]}"; do
    # Relative path from EXAMPLES_DIR, minus the .ein extension.
    # Top-level files keep their bare stem (e.g. "zebra"); nested
    # demo files preserve their path so outputs don't collide
    # (e.g. "zebra/demos/symmetric/couple").
    rel="${ein#"${EXAMPLES_DIR}"/}"
    base="${rel%.ein}"
    echo "==> ${ein}"
    out="${OUT_DIR}/${base}"
    mkdir -p "${out}"
    # Wipe stale outputs from a previous run (parent + rules/ subdir).
    find "${out}" -maxdepth 1 -type f \
        \( -name '*.dot' -o -name '*.svg' -o -name '*.pdf' \
           -o -name '*.png' -o -name '*.gv' \) -delete
    rm -rf "${out}/rules"

    # ── Rules: a multi-digraph stream, one per rule. A file with no rule
    #    forms exits 1 with a message on stderr, which is not an error here. ──
    "${EIN_CMD[@]}" render rules "${ein}" --rule-mode "${RULE_MODE}" \
        2>/dev/null | split_dot_stream "${out}" || true

    render_each "${out}"
    render_each "${out}/rules"

    # `find`, not `ls`: under `nullglob` a glob that matches nothing expands to
    # nothing at all, and `ls` with no arguments lists the *working directory*.
    # A file with no rule forms therefore counted the repo root — 17 "dot
    # files" for a fixture that produced none.
    n_dot=$( count_files "${out}" '[0-9][0-9]_*.dot' )
    n_img=0
    if [[ -n "${raster_first}" ]]; then
        n_img=$( count_files "${out}" "[0-9][0-9]_*.${raster_first}" )
    fi
    total_dots=$((total_dots + n_dot))
    total_imgs=$((total_imgs + n_img))
    printf "    %2d dot, %2d %s\n" "${n_dot}" "${n_img}" "${raster_first:-(no raster)}"

    # ── Constraint scope (S1.6.2) — one DOT per example. ──
    constraints_dot="${out}/_constraints.dot"
    if "${EIN_CMD[@]}" render constraints "${ein}" > "${constraints_dot}" \
            2>/dev/null && [[ -s "${constraints_dot}" ]]; then
        total_constraints=$((total_constraints + 1))
        render_one "${constraints_dot}"
    else
        echo "    warn: 'render constraints' failed on ${ein}" >&2
        rm -f "${constraints_dot}"
    fi

    # ── Commitment-lattice / proof-DAG (S1.6.3) — RUNS a solve, so it
    #    is best-effort under LATTICE_TIMEOUT; the big puzzles skip. ──
    if (( WANT_LATTICE )); then
        lattice_dot="${out}/_lattice.dot"
        if "${LATTICE_PREFIX[@]+"${LATTICE_PREFIX[@]}"}" \
                "${EIN_CMD[@]}" render lattice "${ein}" \
                > "${lattice_dot}" 2>/dev/null && [[ -s "${lattice_dot}" ]]; then
            total_lattice=$((total_lattice + 1))
            render_one "${lattice_dot}"
        else
            echo "    note: lattice skipped (slow >${LATTICE_TIMEOUT}s / no proof)" \
                 "for ${rel}" >&2
            rm -f "${lattice_dot}"
        fi
    fi
done

echo
echo "done — ${total_dots} per-rule DOT files (+${total_constraints} constraints,"
echo "       +${total_lattice} lattice), ${total_imgs} per-rule renders"
echo "       under ${OUT_DIR}"
