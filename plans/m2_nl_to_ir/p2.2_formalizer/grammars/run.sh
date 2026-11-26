#!/usr/bin/env bash
# run.sh MODEL OUT [--only PREFIX]... — S2.2.3's recorded generations: every
# run of plan.jsonl.gz made by the binding and judged through the kernel by
# `einaf probe run`, a JSON line a run appended to OUT (a run OUT holds is not
# made again). From the repository's root, with `einaf` built with `cuda`.
set -euo pipefail
here=$(cd "$(dirname "$0")" && pwd)
root=$(cd "$here/../../../.." && pwd)
plan=$(mktemp --suffix=.jsonl)
trap 'rm -f "$plan"' EXIT
zcat "$here/plan.jsonl.gz" > "$plan"
"$root/einaf/target/release/einaf" probe run --plan "$plan" --out "$2" --model "$1" "${@:3}"
