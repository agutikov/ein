#!/usr/bin/env bash
# Repeatability: the same request several times, the number fixed before the run.
#
#     plans/m2_nl_to_ir/p2.2_formalizer/model_runtime/repeatability.sh MODEL OUT_DIR
#
# Three requests: the host probe's 01 (the youngest, thinking off, with
# log-probabilities), 02 (the same, thinking on, 270 tokens), and the probe's
# `library.off.youngest` (a whole program, thinking off). Each is sent N = 5 times
# in one process, and the five again in a second process: ten calls a request.
# The token ids of the ten are equal, or the first index where one parts is printed.
set -euo pipefail
MODEL=$1; OUT=$2; mkdir -p "$OUT"
D=plans/m2_nl_to_ir/p2.2_formalizer/model_runtime
EINAF=einaf/target/release/einaf
N=5
for req in host_probe/01_youngest_off host_probe/02_youngest_on repeatability/library_off_youngest; do
  name=$(basename $req)
  for proc in 1 2; do
    args=(); for i in $(seq $N); do args+=("$D/$req.json"); done
    "$EINAF" generate --model "$MODEL" --ctx 65536 "${args[@]}" > "$OUT/${name}_p$proc.jsonl" 2> "$OUT/${name}_p$proc.err"
  done
done
python3 -I - "$OUT" <<'PY'
import json, sys, glob, os
out = sys.argv[1]
print("| request | calls | tokens | equal | parts at |\n|---|---:|---:|---|---|")
names = sorted({os.path.basename(p).rsplit("_p", 1)[0] for p in glob.glob(f"{out}/*_p1.jsonl")})
for name in names:
    seqs = []
    for proc in (1, 2):
        for l in open(f"{out}/{name}_p{proc}.jsonl"):
            seqs.append(json.loads(l)["generation"]["envelope"]["generation"]["tokens"])
    first = seqs[0]
    parts = [next((i for i, (a, b) in enumerate(zip(first, s)) if a != b), None if len(s) == len(first) else min(len(s), len(first))) for s in seqs[1:]]
    print(f"| {name} | {len(seqs)} | {len(first)} | {all(s == first for s in seqs)} | {[p for p in parts if p is not None]} |")
PY
