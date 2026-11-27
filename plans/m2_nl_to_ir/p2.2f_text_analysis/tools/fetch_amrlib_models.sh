#!/usr/bin/env bash
# amrlib's parse models, from the release assets of github.com/bjascob/amrlib-models, each into a folder of its own
# under $NLP/models/amrlib/, its archive's sha256 checked against the pinned one (or recorded, with --record).
# Usage: tools/fetch_amrlib_models.sh [--record] [MODEL …]   (default: the two the check runs)
set -euo pipefail
. "$(dirname "$0")/env.sh"

declare -A URL=(
  [parse_xfm_bart_large-v0_1_0]="https://github.com/bjascob/amrlib-models/releases/download/parse_xfm_bart_large-v0_1_0/model_parse_xfm_bart_large-v0_1_0.tar.gz"
  [parse_spring-v0_1_0]="https://github.com/bjascob/amrlib-models/releases/download/model_parse_spring-v0_1_0/model_parse_spring-v0_1_0.tar.gz"
)
# The archives' sha256, as fetched for S2.2f.1 (2026-11-27).
declare -A SHA=(
  [parse_xfm_bart_large-v0_1_0]="2fcfe2113527accb752b6c574994022b3375b66d59da5042356c0f853e879849"
  [parse_spring-v0_1_0]="ffc3e377dab73a80a2b9636ff82c57965c344c910f51b7f4cbbbd799f77cd5b7"
)

record=0
if [ "${1:-}" = "--record" ]; then record=1; shift; fi
models=("$@")
[ ${#models[@]} -eq 0 ] && models=(parse_xfm_bart_large-v0_1_0 parse_spring-v0_1_0)

for m in "${models[@]}"; do
  dir="$NLP/models/amrlib/$m"
  arc="$NLP/models/amrlib/archives/$m.tar.gz"
  mkdir -p "$(dirname "$arc")"
  if [ ! -s "$arc" ]; then
    curl -fL --retry 3 -o "$arc.part" "${URL[$m]}"
    mv "$arc.part" "$arc"
  fi
  got="$(sha256sum "$arc" | cut -d' ' -f1)"
  if [ "$record" = 1 ]; then
    echo "$m $got"
  elif [ "$got" != "${SHA[$m]}" ]; then
    echo "sha256 mismatch for $m: $got, pinned ${SHA[$m]}" >&2; exit 1
  fi
  if [ ! -d "$dir" ]; then
    tmp="$NLP/models/amrlib/.extract-$m"
    rm -rf "$tmp"; mkdir -p "$tmp"
    tar -xzf "$arc" -C "$tmp" --no-same-owner
    # the archive holds one folder, the model's
    inner="$(find "$tmp" -mindepth 1 -maxdepth 1 -type d | head -1)"
    mv "$inner" "$dir"
    rm -rf "$tmp"
  fi
  echo "$m: $dir"
done
