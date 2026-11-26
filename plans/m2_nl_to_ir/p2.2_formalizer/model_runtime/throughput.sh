#!/usr/bin/env bash
# Throughput and device memory of the binding, at the contexts the probe's prompts need.
#
#     plans/m2_nl_to_ir/p2.2_formalizer/model_runtime/throughput.sh MODEL OUT_DIR
#
# Through utils/bench_env.sh (on AC, every P-core), from the repository's root:
# 1. memory: `einaf model --ctx N` for each N, the device's free memory before the
#    model and once its context is made;
# 2. speed: one process, a context of 65 536 tokens, five text prompts of about
#    1 000, 6 000, 16 000, 32 000 and 60 000 tokens (the excerpt of Ein repeated),
#    each evaluated whole and followed by 256 greedy tokens; the envelope's times.
set -euo pipefail
MODEL=$1; OUT=$2; mkdir -p "$OUT"
EINAF=einaf/target/release/einaf
for n in 4096 16384 32768 65536 131072; do
  utils/bench_env.sh --cores P --require-ac "$EINAF" model --model "$MODEL" --ctx $n > "$OUT/model_ctx_$n.json" 2> "$OUT/model_ctx_$n.err"
done
python3 -I - "$OUT" <<'PY'
import json, sys
out = sys.argv[1]
ex = open("einaf/prompts/ein-excerpt.v1.md").read()
for n, reps in [("01k", None), ("06k", 2), ("16k", 5), ("32k", 11), ("60k", 20)]:
    text = ex[:4000] if reps is None else "\n\n".join([ex] * reps)
    json.dump({"prompt": {"kind": "text", "text": text}, "thinking": False, "grammar": None,
               "sampler": [{"sampler": "greedy"}], "seed": 1, "max_tokens": 256, "logprobs": None, "keep": None},
              open(f"{out}/prompt_{n}.json", "w"))
PY
utils/bench_env.sh --cores P --require-ac "$EINAF" generate --model "$MODEL" --ctx 65536 \
  "$OUT"/prompt_01k.json "$OUT"/prompt_06k.json "$OUT"/prompt_16k.json "$OUT"/prompt_32k.json "$OUT"/prompt_60k.json \
  > "$OUT/speed.jsonl" 2> "$OUT/speed.err"
python3 -I - "$OUT" <<'PY'
import json, sys, glob
out = sys.argv[1]
print("| n_ctx | the device's free memory before the model (MiB) | once the context is made (MiB) | the model and its context (MiB) |\n|---:|---:|---:|---:|")
for n in (4096, 16384, 32768, 65536, 131072):
    try:
        c = json.load(open(f"{out}/model_ctx_{n}.json"))["context"]
        print(f"| {n} | {c['device_free_mib_at_load']} | {c['device_free_mib_loaded']} | {c['device_free_mib_at_load'] - c['device_free_mib_loaded']} |")
    except Exception as e:
        print(f"| {n} | failed: {e} | | |")
print("\n| prompt tokens | prompt ms | prompt tokens/s | generated | generation ms | generated tokens/s |\n|---:|---:|---:|---:|---:|---:|")
for l in open(f"{out}/speed.jsonl"):
    r = json.loads(l); g = r["generation"]["envelope"]
    p, e = g["prompt"]["tokens"], g["generation"]
    n = len(e["tokens"])
    print(f"| {p} | {e['prompt_ms']:.0f} | {p / e['prompt_ms'] * 1e3:.0f} | {n} | {e['generation_ms']:.0f} | {n / e['generation_ms'] * 1e3:.1f} |")
PY
