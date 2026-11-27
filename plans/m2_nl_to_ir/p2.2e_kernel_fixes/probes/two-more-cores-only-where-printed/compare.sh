#!/bin/bash
# After questions.sh: each run's stdout past its header, and its summary but for
# its timings, compared between the two binaries, for every run both answered.
# A line a run; `same` or what differs.
set -uo pipefail
HERE=$(cd "$(dirname "$0")" && pwd)
OUT=$HERE/out
for before in "$OUT"/*-before.txt; do
  name=$(basename "$before" -before.txt)
  after=$OUT/$name-after.txt
  [ -f "$after" ] || continue
  if ! grep -q '^solve · ' "$before" || ! grep -q '^solve · ' "$after"; then
    echo "$name: not answered by both"; continue
  fi
  body=same
  cmp -s <(tail -n +2 "$before") <(tail -n +2 "$after") || body=differs
  summary=$(python3 - "$OUT/$name-before.json" "$OUT/$name-after.json" <<'PY'
import json, sys
def strip(x):
    if isinstance(x, dict):
        return {k: strip(v) for k, v in x.items() if k not in ("timings", "wall", "wall_ms", "ms")}
    if isinstance(x, list):
        return [strip(v) for v in x]
    return x
a, b = (strip(json.load(open(p))) for p in sys.argv[1:3])
print("same" if a == b else "differs")
PY
)
  echo "$name: stdout $body, summary $summary"
done
