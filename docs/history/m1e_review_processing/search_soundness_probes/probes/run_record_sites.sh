#!/bin/sh
# D1 / Q4 and D3 / Q-M1e.8 — the record-site conformance matrix.
#
#   sh probes/run_record_sites.sh
#
# The three fixtures now live in `examples/ein-bugs/` (M1e D1, banked
# 2026-08-28); this script drives them there. One defect: `record_node` records a KB that has not been
# re-saturated since the last write into it — by the layer, by `compute_alive`,
# or by `complete()`'s own kill cache. This is
# `docs/kernel/inference/solution_semantics.md` § 2's **first** conjunct,
# checked rather than argued.
#
# For every recorded model the second line feeds THAT model's own negatives —
# one model at a time, never merged — back into the same program, and reports
# what the same engine then answers. A `Contradiction` there is a state the
# engine called a solution and its own rules refute.
set -e
BIN="${EIN_BIN:-ein.rs/target/release/ein}"
# The fixtures are corpus entries now; like $EIN_BIN, this path is relative
# to the repo root, which is where this script is run from.
HERE="examples/ein-bugs"
TMP="$(mktemp -d)"
trap 'rm -rf "$TMP"' EXIT

row() {          # row <label> <fixture> <config-line-or-empty> <cli-args...>
  label="$1"; src="$HERE/$2"; cfg="$3"; shift 3
  if [ -n "$cfg" ]; then
    printf '(config %s)\n' "$cfg" > "$TMP/p.ein"; cat "$src" >> "$TMP/p.ein"
  else
    cp "$src" "$TMP/p.ein"
  fi
  rm -f "$TMP"/neg_*.ein
  "$BIN" solve "$TMP/p.ein" -e "$@" --json-summary "$TMP/s.json" >/dev/null 2>&1 || true
  printf '  %-30s ' "$label"
  python3 - "$TMP/s.json" "$TMP" <<'PY'
import json,os,sys
d=json.load(open(sys.argv[1])); v=d["verdict"]; s=d["stats"]; tmp=sys.argv[2]
keep=lambda f: f.startswith("(not") or not (
    f.startswith("(relation") or f.startswith("(is-a") or f=="(marker)")
print("%-14s k=%s ent=%-3s exhausted=%s" % (v["type"], v["k"],
      s.get("enterings_total"), s.get("exhausted")))
for i,m in enumerate(v.get("solutions",[]),1):
    shown=[f for f in m["facts"] if keep(f)]
    negs=[f for f in m["facts"] if f.startswith("(not")]
    print("       model %d  %s" % (i, " ".join(shown) or "(nothing but the given KB)"))
    if negs:
        open(os.path.join(tmp,"neg_%d.ein"%i),"w").write("\n".join(negs)+"\n")
PY
  for n in "$TMP"/neg_*.ein; do
    [ -e "$n" ] || continue
    i=$(basename "$n" .ein | sed 's/neg_//')
    cat "$TMP/p.ein" "$n" > "$TMP/back.ein"
    "$BIN" solve "$TMP/back.ein" -e --json-summary "$TMP/b.json" >/dev/null 2>&1 || true
    printf '         ↳ model %s re-saturated: ' "$i"
    python3 -c "import json;v=json.load(open('$TMP/b.json'))['verdict'];print(v['type'],'k=%s'%v['k'])"
  done
}

echo "alive-empty-phase1.ein     — record_node at solve.rs:1118 (phase 1)"
row "default"                    alive-empty-phase1.ein ""
row "-K (no kill cache)"         alive-empty-phase1.ein ""                              -K
row "-L (no lookahead)"          alive-empty-phase1.ein ""                              -L
row "forced-positive off"        alive-empty-phase1.ein ":enable-forced-positive false"

echo
echo "alive-empty-interlayer.ein — record_node at solve.rs:1550 (between layers)"
row "default"                    alive-empty-interlayer.ein ""
row "-K (no kill cache)"         alive-empty-interlayer.ein ""                              -K
row "-L (no lookahead)"          alive-empty-interlayer.ein ""                              -L
row "forced-positive off"        alive-empty-interlayer.ein ":enable-forced-positive false"
row "singleton-writeback off"    alive-empty-interlayer.ein ":enable-singleton-writeback false"

echo
echo "complete-records-stale.ein   — record_node at solve.rs:1977 (every corpus solve)"
row "default"                    complete-records-stale.ein ""
row "-K (no kill cache)"         complete-records-stale.ein ""                              -K
row "-L (no lookahead)"          complete-records-stale.ein ""                              -L
