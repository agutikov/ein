#!/bin/bash
# The model's Zebra program under every question of `ein solve`, with no clock
# and under `-T 5`, by two binaries, each run pinned to one P-core thread
# through utils/bench_env.sh: the wall, the exit, and the rows that say the
# answer and root's core. The read-outs and summaries go under out/, which git
# ignores; the table goes to stdout.
#
#     questions.sh BEFORE_BIN AFTER_BIN
set -uo pipefail
# The C locale, so `printf %.2f` reads `bc`'s decimal point on every host.
export LC_ALL=C
HERE=$(cd "$(dirname "$0")" && pwd)
REPO=$(cd "$HERE/../../../../.." && pwd)
OUT=$HERE/out
mkdir -p "$OUT"
P=$REPO/plans/m2_nl_to_ir/p2.2_formalizer/from_text/runs/library/zebra/program.ein
cd "$REPO"
QUESTIONS=("" "--single" "-n 2" "--minimal" "--maximal" "--count" "--no-search")
printf '| binary | question | clock | exit | wall (s) | answer | unsat core rows | region.core |\n'
printf '|---|---|---|---|---|---|---|---|\n'
for clock in none 5; do
  for q in "${QUESTIONS[@]}"; do
    for v in before after; do
      bin=$1; [ "$v" = after ] && bin=$2
      name=$(echo "${q:-describe}-$clock-$v" | tr -d ' ' | tr -s '-')
      extra=(); [ "$clock" = 5 ] && extra=(-T 5)
      # shellcheck disable=SC2086
      s=$(date +%s.%N)
      utils/bench_env.sh --cores PT:1 "$bin" solve "$P" $q "${extra[@]}" \
        --json-summary "$OUT/$name.json" > "$OUT/$name.txt" 2> "$OUT/$name.err"
      rc=$?
      wall=$(echo "$(date +%s.%N) - $s" | bc)
      answer=$(grep -m1 -E '^  (verdict|states of R|minimal line|maximal line|empty|R) ' "$OUT/$name.txt" \
        | sed -E 's/^  [a-zA-Z ()]+ {2,}//' | cut -c1-60)
      [ $rc = 2 ] && answer=$(grep -m1 'aborted' "$OUT/$name.err" | cut -c1-70)
      rows=$(grep -c '^  unsat core ' "$OUT/$name.txt")
      core=$(python3 -c 'import json,sys
try: print(json.dumps(json.load(open(sys.argv[1]))["region"]["core"]))
except Exception: print("no summary")' "$OUT/$name.json" 2>/dev/null)
      printf '| %s | `%s` | %s | %s | %.2f | %s | %s | %s |\n' \
        "$v" "${q:-describe}" "$clock" "$rc" "$wall" "$answer" "$rows" "$core"
    done
  done
done
