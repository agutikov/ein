#!/bin/bash
cd "$1"
for c in c_peg c_pest c_logos c_lalrpop c_grm c_parol; do
  T=${PG_TARGET:-/tmp}/t_$c; rm -rf "$T"
  s=$(date +%s.%N); CARGO_TARGET_DIR=$T cargo build --release -j 4 -p $c >/dev/null 2>"$T.log"; rc=$?; e=$(date +%s.%N)
  printf '%-10s rc=%s clean build %6.1f s  target %s\n' $c $rc $(echo "$e-$s"|bc) "$(du -sh "$T"|cut -f1)"
  rm -rf "$T" "$T.log"
done
