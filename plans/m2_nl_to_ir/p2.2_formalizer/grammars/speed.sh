#!/usr/bin/env bash
# speed.sh MODEL OUT — S2.2.3's speeds: every call of speed/order.txt, in its
# order, in one process (`einaf generate`, the model loaded once), through
# `utils/bench_env.sh --cores P --require-ac`, its record (a JSON line a call)
# into OUT and the machine's state into OUT.env. From the repository's root,
# with `einaf` built with `cuda`. speed/README.md says how it is read.
set -euo pipefail
here=$(cd "$(dirname "$0")" && pwd)
root=$(cd "$here/../../../.." && pwd)
dir=$(mktemp -d)
trap 'rm -rf "$dir"' EXIT
python3 - "$here/speed/requests.jsonl.gz" "$dir" <<'PY'
import gzip, json, sys
for line in gzip.open(sys.argv[1], "rt"):
    e = json.loads(line)
    open(f"{sys.argv[2]}/{e['name']}.json", "w").write(json.dumps(e["request"], ensure_ascii=False) + "\n")
PY
files=()
while read -r name; do files+=("$dir/$name.json"); done < "$here/speed/order.txt"
"$root/utils/bench_env.sh" --cores P --require-ac \
  "$root/einaf/target/release/einaf" generate --model "$1" "${files[@]}" > "$2" 2> "$2.env"
