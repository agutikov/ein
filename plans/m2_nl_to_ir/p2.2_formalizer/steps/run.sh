#!/bin/sh
# S2.2.4's runs with the model: each line of plan.tsv, one `einaf steps run`, its record under
# runs/CONDITION/ITEM/ (calls.jsonl, run.json, each step's output, program.ein, summary.txt,
# stderr.txt). A run already recorded is not made again. ONLY=CONDITION runs one condition.
#
#   EINAF_MODEL=/var/lib/ollama/blobs/sha256-a9d0a3e1… sh run.sh
set -eu
HERE=$(cd "$(dirname "$0")" && pwd)
ROOT=$(cd "$HERE/../../../.." && pwd)
EINAF=${EINAF:-$ROOT/einaf/target/release/einaf}
MODEL=${EINAF_MODEL:?the model's file}
grep -v '^#' "$HERE/plan.tsv" | while IFS='	' read -r cond id line flags; do
    [ -n "$cond" ] || continue
    if [ -n "${ONLY:-}" ] && [ "$cond" != "$ONLY" ]; then continue; fi
    out=$HERE/runs/$cond/$id
    [ -f "$out/run.json" ] && continue
    mkdir -p "$out"
    echo "$(date +%H:%M:%S) $cond $id" >&2
    # shellcheck disable=SC2086
    "$EINAF" steps run "$HERE/texts/$id.txt" --id "$id" --answer-line "$line" --out "$out" \
        --model "$MODEL" $flags > "$out/summary.txt" 2> "$out/stderr.txt" || echo "  failed: $?" >&2
done
