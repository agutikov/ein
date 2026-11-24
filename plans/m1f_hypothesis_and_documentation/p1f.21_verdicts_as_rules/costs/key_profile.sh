#!/usr/bin/env bash
# The profile of a state's key, before and after it is read at its own size: three
# `perf record --call-graph lbr -e instructions:u` of `taskset -c 0-15 ein solve FILE
# --jobs 1` a program and an engine, each reported with
# `perf report --stdio --children --sort symbol -F overhead_children,overhead,symbol -g none`.
# Scratch: an A/B for the coordinator, not a tool of the repository.
#   key_profile.sh OUTDIR BIN-BEFORE BIN-AFTER
set -eu
OUT=$1; BEFORE=$(readlink -f "$2"); AFTER=$(readlink -f "$3")
HERE=$(cd "$(dirname "$0")/../../../.." && pwd)
mkdir -p "$OUT"
cd "$HERE"
for prog in examples/beyond_arith/send_more_money.ein examples/beyond_arith/equation_two_unknowns.ein; do
  name=$(basename "$prog" .ein)
  for point in before after; do
    if [ "$point" = before ]; then bin=$BEFORE; else bin=$AFTER; fi
    for run in 1 2 3; do
      data="$OUT/$name.$point.$run.perf.data"
      perf record -q --call-graph lbr -e instructions:u -o "$data" -- \
        taskset -c 0-15 "$bin" solve "$prog" --jobs 1 > /dev/null 2> /dev/null
      perf report -i "$data" --stdio --children --sort symbol \
        -F overhead_children,overhead,symbol -g none 2> /dev/null > "$OUT/$name.$point.$run.perf-report.txt"
      echo "$name $point $run"
    done
  done
done
