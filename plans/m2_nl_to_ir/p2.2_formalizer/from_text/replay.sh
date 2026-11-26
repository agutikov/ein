#!/bin/sh
# Every run of plan.tsv again from its committed record, with no model: `einaf from-text --replay` on the run's
# generations.jsonl, writing the whole record again under replayed/CONDITION/ITEM/ (git ignores it) with the einaf
# given (EINAF, by default the gate's build): each step's program, every feedback object, the answer, the run.
# analyse.py reads what it writes, and holds each replayed program to the committed one. ONLY=CONDITION replays
# one condition.
set -eu
HERE=$(cd "$(dirname "$0")" && pwd)
ROOT=$(cd "$HERE/../../../.." && pwd)
EINAF=${EINAF:-$ROOT/einaf/target/gate/einaf}
cd "$ROOT"
grep -v '^#' "$HERE/plan.tsv" | while IFS='	' read -r cond id text line gold flags; do
    [ -n "$cond" ] || continue
    if [ -n "${ONLY:-}" ] && [ "$cond" != "$ONLY" ]; then continue; fi
    rec=$HERE/runs/$cond/$id
    [ -f "$rec/generations.jsonl" ] || continue
    out=$HERE/replayed/$cond/$id
    mkdir -p "$out"
    # shellcheck disable=SC2086
    "$EINAF" from-text "$text" --id "$id" --answer-line "$line" --record "$out" --gold-answer "$gold" \
        --replay "$rec/generations.jsonl" $flags > "$out/stdout.txt" 2> "$out/stderr.txt" \
        || echo "$cond $id: failed: $?" >&2
done
