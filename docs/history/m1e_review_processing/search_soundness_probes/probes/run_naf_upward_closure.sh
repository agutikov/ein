#!/bin/sh
# D4 / Q-M1e.9 — re-take the upward-closure matrix.
#
#   sh probes/run_naf_upward_closure.sh
#
# Prints one row per configuration: k, enterings, and the p/q/not facts of each
# recorded state. The expected answer, by hand, is one solution {(p A), (q A)}.
# Five of the six rows do not give it.
set -e
BIN="${EIN_BIN:-ein.rs/target/release/ein}"
SRC="$(dirname "$0")/naf_upward_closure.ein"
TMP="$(mktemp -d)"
trap 'rm -rf "$TMP"' EXIT

row() {          # row <label> <config-line-or-empty> <cli-args...>
  label="$1"; cfg="$2"; shift 2
  if [ -n "$cfg" ]; then
    printf '(config %s)\n' "$cfg" > "$TMP/p.ein"; cat "$SRC" >> "$TMP/p.ein"
  else
    cp "$SRC" "$TMP/p.ein"
  fi
  "$BIN" solve "$TMP/p.ein" -e "$@" --json-summary "$TMP/s.json" >/dev/null 2>&1 || true
  printf '%-46s ' "$label"
  python3 - "$TMP/s.json" <<'PY'
import json,sys
d=json.load(open(sys.argv[1])); v=d["verdict"]; s=d["stats"]
keep=lambda f: f.startswith("(p ") or f.startswith("(q ") or f.startswith("(not")
print("k=%s ent=%-3s %s" % (v["k"], s.get("enterings_total"),
      [[f for f in m["facts"] if keep(f)] for m in v["solutions"]]))
PY
}

row "default"                                   ""
row "-K"                                        ""                                      -K
row "-L"                                        ""                                      -L
row "-L -K"                                     ""                                      -L -K
row "kill-cache off (config)"                   ":enable-lookahead-kill-cache false"
row "singleton-writeback off (config)"          ":enable-singleton-writeback false"
row "singleton-writeback off (config) + -L"     ":enable-singleton-writeback false"      -L
