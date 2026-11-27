#!/bin/bash
# The model's Zebra program under `ein solve --single`, with no clock, and with
# a clock it does not reach, by two binaries, interleaved, two rounds, pinned
# to one P-core thread; and under `-T 5` by the second.
#
#     single.sh BEFORE_BIN AFTER_BIN      # outputs under out/, which git ignores
set -uo pipefail
HERE=$(cd "$(dirname "$0")" && pwd)
REPO=$(cd "$HERE/../../../../.." && pwd)
OUT=$HERE/out
mkdir -p "$OUT"
P=$REPO/plans/m2_nl_to_ir/p2.2_formalizer/from_text/runs/library/zebra/program.ein
cd "$REPO"
for rep in 1 2; do
  for v in before after after-far; do
    bin=$2; extra=()
    [ "$v" = before ] && bin=$1
    [ "$v" = after-far ] && extra=(-T 1000)
    s=$(date +%s.%N)
    utils/bench_env.sh --cores PT:1 "$bin" solve "$P" --single "${extra[@]}" \
      > "$OUT/single-$v-$rep.txt" 2> "$OUT/single-$v-$rep.err"
    rc=$?
    echo "$v rep $rep exit $rc wall $(echo "$(date +%s.%N) - $s" | bc)"
  done
done
for q in --single "" --no-search --count; do
  s=$(date +%s.%N)
  utils/bench_env.sh --cores PT:1 "$2" solve "$P" $q -T 5 > /dev/null 2> "$OUT/clock.err"
  rc=$?
  echo "-T 5 $q exit $rc wall $(echo "$(date +%s.%N) - $s" | bc): $(tail -1 "$OUT/clock.err")"
done
