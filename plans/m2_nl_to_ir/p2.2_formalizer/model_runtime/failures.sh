#!/usr/bin/env bash
# Each typed failure of the runtime, from a cause the binding meets, recorded.
#
#     plans/m2_nl_to_ir/p2.2_formalizer/model_runtime/failures.sh MODEL OUT.jsonl
#
# A file that is no GGUF (model-load); a chat with no message (template); the
# excerpt of Ein, about 3 000 tokens, in a context of 512 (prompt-too-long); a
# grammar that does not parse (grammar-parse); and a grammar that asks for a NUL,
# `[\x00]`, which llama.cpp's grammar never lets a token's text begin with
# (grammar-dead-end). A line a call, as `einaf generate` writes it.
set -euo pipefail
MODEL=$1; OUT=$2
D=plans/m2_nl_to_ir/p2.2_formalizer/model_runtime/failures
EINAF=einaf/target/release/einaf
NOT_A_MODEL=$(mktemp); printf 'This is not a GGUF file.\n' > "$NOT_A_MODEL"
: > "$OUT"
"$EINAF" generate --model "$NOT_A_MODEL" "$D/model_load.json" >> "$OUT"
rm -f "$NOT_A_MODEL"
"$EINAF" generate --model "$MODEL" --ctx 4096 "$D/template_no_messages.json" "$D/grammar_parse.json" "$D/grammar_dead_end.json" >> "$OUT"
"$EINAF" generate --model "$MODEL" --ctx 512 "$D/prompt_too_long.json" >> "$OUT"
