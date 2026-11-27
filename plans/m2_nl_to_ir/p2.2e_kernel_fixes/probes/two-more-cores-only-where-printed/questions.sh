#!/bin/bash
# The two fixtures whose cores S2.2e.7 is about, under the questions that print
# the core and those that do not, with no clock and under `-T 5`, by two
# binaries, each run pinned to one P-core thread through utils/bench_env.sh:
# the wall, the exit, and the row that says the answer. The read-outs,
# summaries and stderr go under out/, which git ignores; the table goes to
# stdout. A run's stdout past its header is compared between the binaries by
# compare.sh, beside this file.
#
#     questions.sh BEFORE_BIN AFTER_BIN
set -uo pipefail
export LC_ALL=C
HERE=$(cd "$(dirname "$0")" && pwd)
REPO=$(cd "$HERE/../../../../.." && pwd)
OUT=$HERE/out
mkdir -p "$OUT"
B=ein.rs/crates/ein-cli/tests/bounded
cd "$REPO"
# file|question, the question's words split on spaces.
RUNS=(
  "hole|"
  "hole|--count"
  "hole|--maximal"
  "hole|--minimal"
  "hole|--maximal -n 2"
  "hole|--maximal --single"
  "whole|--single"
  "whole|--single -v"
  "whole|--single --layer-progress"
  "whole|-n 2"
  "whole|-n 2 -v"
  "whole|--single --events EVENTS"
)
printf '| fixture | question | clock | binary | exit | wall (s) | answer |\n'
printf '|---|---|---|---|---|---|---|\n'
for run in "${RUNS[@]}"; do
  f=${run%%|*}; q=${run#*|}
  for clock in none 5; do
    for v in before after; do
      bin=$1; [ "$v" = after ] && bin=$2
      name=$(echo "$f-${q:-describe}-$clock-$v" | tr -d ' ' | tr -s '-')
      words=${q//EVENTS/$OUT/$name.jsonl}
      extra=(); [ "$clock" = 5 ] && extra=(-T 5)
      s=$(date +%s.%N)
      # shellcheck disable=SC2086
      utils/bench_env.sh --cores PT:1 "$bin" solve "$B/dead_link_$f.ein" $words "${extra[@]}" \
        --json-summary "$OUT/$name.json" > "$OUT/$name.txt" 2> "$OUT/$name.err"
      rc=$?
      wall=$(echo "$(date +%s.%N) - $s" | bc)
      answer=$(grep -m1 -E '^  (verdict|states of R|minimal line|maximal line|states \(k_s\)) ' "$OUT/$name.txt" \
        | sed -E 's/^  [a-zA-Z_ ()]+ {2,}//' | cut -c1-60)
      [ $rc = 2 ] && answer=$(grep -m1 'aborted' "$OUT/$name.err" | cut -c1-70)
      printf '| %s | `%s` | %s | %s | %s | %.2f | %s |\n' \
        "dead_link_$f.ein" "${q:-describe}" "$clock" "$v" "$rc" "$wall" "$answer"
    done
  done
done
