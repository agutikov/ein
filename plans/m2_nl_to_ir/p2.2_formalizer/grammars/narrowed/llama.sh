#!/usr/bin/env bash
# llama.sh MODEL — test 3 on the narrowed grammars, from the repository's root, with `einaf` built with `cuda`:
# llama.cpp's grammar sampler with the model's vocabulary beside EinAf's acceptor (`einaf grammar llama`), a JSON
# line a text, into llama/ beside this script:
#   corpus.jsonl.gz          the corpus's 937 files as written, from `root`, the grammar a request carries
#   corpus-emptied.jsonl.gz  the same files with their comments emptied (empty.py)
#   steps.jsonl.gz           the steps by hand under the narrowed step grammars (../llama_steps.py)
#   special.jsonl            special/*.txt with the special tokens parsed (`--special`), under the root before
#                            the narrowing (`98e6e0a3`'s file) and under the narrowed one
set -euo pipefail
here=$(cd "$(dirname "$0")" && pwd)
root=$(cd "$here/../../../../.." && pwd)
E="$root/einaf/target/release/einaf"
M="$1"
tmp=$(mktemp -d)
trap 'rm -rf "$tmp"' EXIT
cd "$root"
"$E" grammar render whole > "$tmp/root.gbnf"
git show 98e6e0a3:einaf/grammars/ein.gbnf > "$tmp/before.gbnf"
run() { local out=$1; shift; "$E" grammar llama --model "$M" "$@" > "$tmp/out.jsonl" 2> "$tmp/err" || true;
        grep -v ggml_cuda_init "$tmp/err" | grep -v 'Device 0' >&2; gzip -n -c "$tmp/out.jsonl" > "$out"; }
run "$here/llama/corpus.jsonl.gz" --grammar "$tmp/root.gbnf" --root root --list "$here/../llama/corpus.list"
python3 "$here/empty.py" "$here/../llama/corpus.list" "$tmp/emptied" > "$tmp/emptied.list"
run "$here/llama/corpus-emptied.jsonl.gz" --grammar "$tmp/root.gbnf" --root root --list "$tmp/emptied.list"
python3 "$here/../llama_steps.py" "$M" "$tmp/steps.jsonl" --einaf "$E"
gzip -n -c "$tmp/steps.jsonl" > "$here/llama/steps.jsonl.gz"
: > "$here/llama/special.jsonl"
for g in before root; do
  (cd "$here" && "$E" grammar llama --model "$M" --grammar "$tmp/$g.gbnf" --root root --special special/*.txt \
     2> /dev/null | sed "s/^{/{\"grammar\":\"$g\",/") >> "$here/llama/special.jsonl" || true
done
