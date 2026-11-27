#!/bin/sh
# Two programs a pair, each pair the same forms in two orders, every question asked of each,
# the read-outs diffed with the path line taken off. Run from anywhere; EIN_BIN names the binary.
here=$(cd "$(dirname "$0")" && pwd)
ein=${EIN_BIN:-$here/../../../../../ein.rs/target/gate/ein}
out=$(mktemp -d)
for pair in "gap-reader-first gap-producer-first" "one-food-each-ann-first one-food-each-carl-first"; do
  set -- $pair
  for q in "" "--single" "-n 1" "--minimal" "--maximal" "--count" "--no-search"; do
    "$ein" solve "$here/$1.ein" $q | sed 1d > "$out/a"
    "$ein" solve "$here/$2.ein" $q | sed 1d > "$out/b"
    if cmp -s "$out/a" "$out/b"; then echo "$1 / $2 [$q]: same"; else echo "$1 / $2 [$q]: differ"; diff "$out/a" "$out/b" | sed 's/^/    /'; fi
  done
done
rm -r "$out"
