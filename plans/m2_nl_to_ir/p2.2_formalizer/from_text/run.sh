#!/bin/sh
# S2.2.5's runs with the model: each line of plan.tsv, one `einaf from-text`, its record under
# runs/CONDITION/ITEM/. Committed of a run, by the user's ruling on what a run commits: generations.jsonl (each
# call slim: its request's digest and what came back, no request body), program.ein, program.sidecar.json and
# stderr.txt (the live run's, with its times); the rest of the record (config.json, kernel.jsonl, answer.json,
# run.json, steps/, stdout.txt) replay.sh writes again and git ignores. A run already recorded is not made
# again. ONLY=CONDITION runs one condition.
#
#   EINAF_MODEL=/var/lib/ollama/blobs/sha256-a9d0a3e1… sh run.sh
set -eu
HERE=$(cd "$(dirname "$0")" && pwd)
ROOT=$(cd "$HERE/../../../.." && pwd)
EINAF=${EINAF:-$ROOT/einaf/target/release/einaf}
MODEL=${EINAF_MODEL:?the file of the model}
cd "$ROOT"
grep -v '^#' "$HERE/plan.tsv" | while IFS='	' read -r cond id text line gold flags; do
    [ -n "$cond" ] || continue
    if [ -n "${ONLY:-}" ] && [ "$cond" != "$ONLY" ]; then continue; fi
    out=$HERE/runs/$cond/$id
    [ -f "$out/generations.jsonl" ] && continue
    mkdir -p "$out"
    echo "$(date +%H:%M:%S) $cond $id" >&2
    # shellcheck disable=SC2086
    "$EINAF" from-text "$text" --id "$id" --answer-line "$line" --record "$out" --gold-answer "$gold" \
        --model "$MODEL" $flags > "$out/stdout.txt" 2> "$out/stderr.txt" || echo "  failed: $?" >&2
done
