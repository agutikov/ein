#!/bin/sh
# Every run of plan.tsv again from its record, with no model: `einaf steps run --replay` on the run's
# calls (unpacked from calls.jsonl.gz where they are packed), writing the run, its programs, its
# judgements and the check's findings again with the einaf given (EINAF, by default the gate's build).
# The tables (analyse.py) are read from what it writes. ONLY=CONDITION replays one condition.
set -eu
HERE=$(cd "$(dirname "$0")" && pwd)
ROOT=$(cd "$HERE/../../../.." && pwd)
EINAF=${EINAF:-$ROOT/einaf/target/gate/einaf}
grep -v '^#' "$HERE/plan.tsv" | while IFS='	' read -r cond id line flags; do
    [ -n "$cond" ] || continue
    if [ -n "${ONLY:-}" ] && [ "$cond" != "$ONLY" ]; then continue; fi
    out=$HERE/runs/$cond/$id
    [ -f "$out/run.json" ] || [ -f "$out/run.json.gz" ] || continue
    calls=$out/calls.jsonl
    if [ ! -f "$calls" ] && [ -f "$calls.gz" ]; then gunzip -k "$calls.gz"; fi
    # shellcheck disable=SC2086
    "$EINAF" steps run "$HERE/texts/$id.txt" --id "$id" --answer-line "$line" --out "$out" \
        --replay "$calls" $flags > "$out/summary.txt" 2> "$out/replay.stderr.txt" || echo "$cond $id: failed: $?" >&2
done
