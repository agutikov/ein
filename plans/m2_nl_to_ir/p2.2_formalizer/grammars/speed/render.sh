#!/usr/bin/env bash
# render.sh — how long EinAf takes to write each grammar a request carries
# (`einaf grammar render … --fence --request`: the catalogue read off the
# library's texts, the program's vocabulary, the GBNF written), ten runs of
# each for the Zebra puzzle and `apples`, through `utils/bench_env.sh --cores P
# --require-ac`; a line `grammar text milliseconds` a run on stdout. From the
# repository's root.
set -euo pipefail
root=$(cd "$(dirname "$0")/../../../../.." && pwd)
e="$root/einaf/target/release/einaf"
run() {
  for t in zebra apples; do
    c="$root/einaf/contract/$t"
    for g in whole 1 2 2b 3; do
      case $g in
        1) a=(--text "$c/text.txt");; 2) a=(--program "$c/step1.ein");;
        2b|3) a=(--program "$c/step2.ein");; *) a=();;
      esac
      for i in 1 2 3 4 5 6 7 8 9 10; do
        s=$(date +%s%N); "$e" grammar render "$g" "${a[@]}" --fence --request > /dev/null; f=$(date +%s%N)
        echo "$g $t $(( (f - s) / 1000000 ))"
      done
    done
  done
}
export -f run; export root e
"$root/utils/bench_env.sh" --cores P --require-ac bash -c run
