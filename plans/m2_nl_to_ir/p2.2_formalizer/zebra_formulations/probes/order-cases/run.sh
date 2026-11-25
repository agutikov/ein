#!/usr/bin/env bash
# Each case in both orders: the read-out of `ein solve`, the path line taken off, and whether the two are byte for byte the same.
set -u
cd "$(dirname "$0")"
EIN=${EIN_BIN:-../../../../../../ein.rs/target/release/ein}
for late in *.late.ein; do
  usual=${late%.late.ein}.usual.ein
  a=$("$EIN" solve "$late" 2>&1); ea=$?; a=$(printf '%s\n' "$a" | tail -n +2)
  b=$("$EIN" solve "$usual" 2>&1); eb=$?; b=$(printf '%s\n' "$b" | tail -n +2)
  if [ "$a" = "$b" ]; then same=same; else same=DIFFERENT; fi
  first=$(printf '%s\n' "$a" | grep -m1 -E 'error|refused|verdict|states \(k_s\)|∅' | sed 's/^ *//')
  printf '%-36s exit %s/%s  read-out %s  %s\n' "${late%.late.ein}" "$ea" "$eb" "$same" "$first"
done
