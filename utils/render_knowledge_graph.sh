#!/usr/bin/env bash
#
# Render plans/index/knowledge-graph.dot into SVG (or another Graphviz
# format), choosing between several layout engines.
#
# Usage:
#   utils/render_knowledge_graph.sh                  # → knowledge-graph.svg (dot)
#   utils/render_knowledge_graph.sh png              # PNG via dot
#   utils/render_knowledge_graph.sh svg fdp          # use fdp (force-directed)
#   utils/render_knowledge_graph.sh svg osage        # tile clusters
#   utils/render_knowledge_graph.sh svg all          # render with all engines
#   utils/render_knowledge_graph.sh svg dot /tmp/out.svg   # custom output path
#
# Args (positional, all optional):
#   $1  format         (svg | png | pdf | ...)              default: svg
#   $2  engine         (dot | fdp | sfdp | osage | all)     default: dot
#   $3  output path    (ignored when engine=all)            default: plans/index/knowledge-graph.<fmt>
#
# Engine notes:
#   * dot    — hierarchical; current primary view.
#   * fdp    — force-directed, respects clusters. Auto-passes
#              `-Goverlap=prism -Goverlap_scaling=4` to remove node overlap.
#   * sfdp   — scalable variant of fdp for big graphs; same overlap flags.
#              Cluster rendering is less polished than fdp.
#   * osage  — packs clusters as non-overlapping tiles. Modern Graphviz
#              does not honour per-cluster `layout=`; osage just lays
#              everything out itself. Pass `-Gpack=true -Gpackmode=array_4`
#              to arrange the cluster tiles in a 4-column grid.
#   * all    — runs dot/fdp/sfdp/osage in turn, producing
#              knowledge-graph.<engine>.<fmt>  next to the .dot source.

set -euo pipefail

FMT="${1:-svg}"
ENGINE="${2:-dot}"

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "${SCRIPT_DIR}/.." && pwd)"
SRC="${REPO_ROOT}/plans/index/knowledge-graph.dot"

if [[ ! -f "${SRC}" ]]; then
    echo "error: source DOT file not found: ${SRC}" >&2
    exit 1
fi

render_one() {
    local engine="$1"
    local out="$2"
    local -a extra_flags=()

    case "${engine}" in
        fdp|sfdp)
            # Force-directed engines: explicitly remove node overlap.
            # `splines=polyline` (from the DOT file) crashes fdp on this
            # graph; override to straight-line edges.
            extra_flags+=( "-Goverlap=prism" "-Goverlap_scaling=4"
                           "-Gsplines=line" )
            ;;
        osage)
            # Tile-pack clusters as a 4-column grid.
            extra_flags+=( "-Gpack=true" "-Gpackmode=array_t4" )
            ;;
        dot|neato|twopi|circo|patchwork)
            ;;
        *)
            echo "error: unknown engine '${engine}'" >&2
            return 2
            ;;
    esac

    if ! command -v "${engine}" >/dev/null 2>&1; then
        echo "error: '${engine}' not found in PATH (install graphviz)" >&2
        return 1
    fi

    echo "rendering ${SRC}"
    echo "      via ${engine} -T${FMT} ${extra_flags[*]:-}"
    echo "      to  ${out}"

    "${engine}" "-T${FMT}" "${extra_flags[@]}" "${SRC}" -o "${out}"

    local size
    size=$(stat -c '%s' "${out}" 2>/dev/null || stat -f '%z' "${out}")
    echo "done — ${out} (${size} bytes)"
}

if [[ "${ENGINE}" == "all" ]]; then
    for e in dot fdp sfdp osage; do
        out="${REPO_ROOT}/plans/index/knowledge-graph.${e}.${FMT}"
        # Keep going if one engine fails — others may still work.
        render_one "${e}" "${out}" || echo "  ↳ ${e} failed, continuing." >&2
        echo
    done
else
    OUT="${3:-${REPO_ROOT}/plans/index/knowledge-graph.${FMT}}"
    render_one "${ENGINE}" "${OUT}"
fi
