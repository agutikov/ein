#!/bin/sh
# D1 / Q4 — re-take the alive-∅ matrix, and re-saturate what each run recorded.
#
#   sh probes/run_alive_empty.sh
#
# Two fixtures, one defect: `record_node` records root on a path that has not
# re-saturated it since the writes of the layer (or of `compute_alive` itself).
# For every row that answers `Solution`, the second line feeds the run's OWN
# recorded negatives back into the same program — and the same engine then
# answers `Contradiction`, which is what makes the recorded state not a
# fixpoint rather than merely surprising.
set -e
BIN="${EIN_BIN:-ein.rs/target/release/ein}"
HERE="$(dirname "$0")"
TMP="$(mktemp -d)"
trap 'rm -rf "$TMP"' EXIT

row() {          # row <label> <fixture> <config-line-or-empty> <cli-args...>
  label="$1"; src="$HERE/$2"; cfg="$3"; shift 3
  if [ -n "$cfg" ]; then
    printf '(config %s)\n' "$cfg" > "$TMP/p.ein"; cat "$src" >> "$TMP/p.ein"
  else
    cp "$src" "$TMP/p.ein"
  fi
  "$BIN" solve "$TMP/p.ein" -e "$@" --json-summary "$TMP/s.json" >/dev/null 2>&1 || true
  printf '  %-34s ' "$label"
  python3 - "$TMP/s.json" "$TMP/negs.ein" <<'PY'
import json,sys
d=json.load(open(sys.argv[1])); v=d["verdict"]; s=d["stats"]
negs=[f for m in v.get("solutions",[]) for f in m["facts"] if f.startswith("(not")]
open(sys.argv[2],"w").write("\n".join(negs)+("\n" if negs else ""))
print("%-14s k=%s ent=%-3s exhausted=%-5s recorded %s"
      % (v["type"], v["k"], s.get("enterings_total"), s.get("exhausted"),
         [[f for m in v.get("solutions",[]) for f in m["facts"]
           if f.startswith("(p ") or f.startswith("(not")]]))
PY
  if [ -s "$TMP/negs.ein" ]; then
    cat "$TMP/p.ein" "$TMP/negs.ein" > "$TMP/back.ein"
    "$BIN" solve "$TMP/back.ein" -e --json-summary "$TMP/b.json" >/dev/null 2>&1 || true
    printf '  %-34s ' "  ↳ that state, re-saturated:"
    python3 -c "import json,sys;v=json.load(open('$TMP/b.json'))['verdict'];print(v['type'],'k=%s'%v['k'])"
  fi
}

echo "alive_empty_phase1.ein   — the site at solve.rs:1114-1118 (phase 1)"
row "default"                    alive_empty_phase1.ein ""
row "-K (no kill cache)"         alive_empty_phase1.ein ""                              -K
row "-L (no lookahead)"          alive_empty_phase1.ein ""                              -L
row "forced-positive off"        alive_empty_phase1.ein ":enable-forced-positive false"

echo
echo "alive_empty_interlayer.ein — the site at solve.rs:1544-1550 (Route B)"
row "default"                    alive_empty_interlayer.ein ""
row "-K (no kill cache)"         alive_empty_interlayer.ein ""                              -K
row "-L (no lookahead)"          alive_empty_interlayer.ein ""                              -L
row "forced-positive off"        alive_empty_interlayer.ein ":enable-forced-positive false"
row "singleton-writeback off"    alive_empty_interlayer.ein ":enable-singleton-writeback false"
