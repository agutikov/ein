#!/bin/bash
# One corpus file through today's ein and the prototype (EIN_PARSER=grm):
# `solve` (describe) and `saturate --dump`, outputs compared with the timing
# lines removed. Prints: FILE  solve:SAME|DIFF|...  saturate:SAME|DIFF
cd "${S1F555:-$(dirname "$0")/../../../../../build/s1f.5.5}"
export EIN_STDLIB=$PWD/snapshot/stdlib
f="$1"; key=$(echo "$f" | tr '/' '_')
o=sweep/$key
norm() { grep -v -E '[0-9.]+ ms|chars$|^input:|seconds|[0-9.]+ s\)' ; }
res=""
for cmd in solve saturate; do
  args=($cmd snapshot/$f); [ $cmd = saturate ] && args+=(--dump)
  timeout 120 ./ein "${args[@]}" > $o.$cmd.today 2>&1; a=$?
  EIN_PARSER=grm timeout 120 ./ein-grm "${args[@]}" > $o.$cmd.grm 2>&1; b=$?
  if [ $a -ne $b ]; then r="EXIT($a/$b)"; elif diff -q <(norm < $o.$cmd.today | sed 's/ein-grm/ein/g') <(norm < $o.$cmd.grm | sed 's/ein-grm/ein/g') >/dev/null; then r=SAME; else r=DIFF; fi
  res="$res $cmd:$r($a)"
done
echo "$res $f"
