#!/bin/bash
# The corpus's core searches before the clock reached them and after, with no
# clock and with a far one, interleaved, three rounds, each run pinned to one
# P-core thread through bench_env.sh.
#
#     corpus.sh BEFORE_COMMIT      # outputs under out/, which git ignores
#
# BEFORE_COMMIT is the commit before the change; the checkout is the after.
# The probe is built at each in a worktree of its own under out/.
set -euo pipefail
HERE=$(cd "$(dirname "$0")" && pwd)
REPO=$(cd "$HERE/../../../../.." && pwd)
OUT=$HERE/out
mkdir -p "$OUT"
build() {  # build <tree> <name>
  cp "$HERE/core_clock_probe.rs" "$1/ein.rs/crates/ein-infer/examples/"
  (cd "$1/ein.rs" && cargo build --release -p ein-infer --example core_clock_probe)
  cp "$1/ein.rs/target/release/examples/core_clock_probe" "$OUT/probe-$2"
  rm "$1/ein.rs/crates/ein-infer/examples/core_clock_probe.rs"
}
[ -d "$OUT/before-tree" ] || git -C "$REPO" worktree add --detach "$OUT/before-tree" "$1"
build "$OUT/before-tree" before
build "$REPO" after
cd "$REPO"
for rep in 1 2 3; do
  for v in before-none after-none after-far; do
    utils/bench_env.sh --cores PT:1 "$OUT/probe-${v%-*}" "${v#*-}" \
      > "$OUT/$v-$rep.tsv" 2> "$OUT/$v-$rep.err"
    echo "$v rep $rep $(grep total "$OUT/$v-$rep.err")"
  done
done
