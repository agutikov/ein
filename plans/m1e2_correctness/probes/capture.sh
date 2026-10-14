#!/usr/bin/env bash
# Re-takes every witness in the inventory and writes the verbatim output to
# build/m1e2_probes/<id>.txt (or $OUT).  Run from anywhere; reads the repo and these probes.
set -u
R="$(cd "$(dirname "${BASH_SOURCE[0]}")/../../.." && pwd)"
E="${EIN_BIN:-$R/ein.rs/target/release/ein}"
S="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
# The witnesses were banked at S1e2.1.1 — corpus/wrong_answers.toml is their register.
B="$R/examples/ein-bugs/wrong-answers"
O="${OUT:-$R/build/m1e2_probes}"   # build/ is gitignored
mkdir -p $O
cd $R

run() { # id, then the command line
  local id=$1; shift
  {
    echo "\$ $*"
    "$@" 2>&1
    echo "[exit=$?]"
    echo
  } >> $O/$id.txt
}
envrun() { # id, VAR=val, command...
  local id=$1; local kv=$2; shift 2
  {
    echo "\$ $kv $*"
    env "$kv" "$@" 2>&1
    echo "[exit=$?]"
    echo
  } >> $O/$id.txt
}
rm -f $O/*.txt

# W1 — maximal alive commitment not recorded (Q-M1e.8)
run W1 $E solve -e -s examples/branching/15_lookahead_two_step_on.ein
run W1 $E solve -e -s examples/branching/16_lookahead_two_step_off.ein
run W1 $E solve -e -s -L examples/lattice/02_genuine_3set_death.ein
run W1 $E solve -e -s examples/lattice/02_genuine_3set_death.ein
run W1 $E solve -e -s -p $B/forced-2-two.ein
run W1 $E solve -e -s -p $B/forced-1-two.ein
run W1 $E solve -e -s $B/owed-two-step.ein
run W1 $E solve -e -s $B/tree-dead.ein

# W2 — forced-positive cascade asserts a refutable singleton
run W2 $E solve -e -s -p $B/forced-2.ein
run W2 $E solve -e -s -p $B/forced-2-off.ein
run W2 $E solve -e -s -p -L $B/forced-1.ein
run W2 $E solve -e -s -p $B/forced-1.ein

# W3 — dead not upward-closed under absent (Q-M1e.9)
run W3 $E solve -e -s -P examples/ein-bugs/naf-upward-closure.ein
run W3 $E solve -e -s -P -K examples/ein-bugs/naf-upward-closure.ein
run W3 $E solve -e -s -P -L $B/naf-nowb.ein

# W4 — fresh name unreachable (Q-M1e.21)
run W4 $E solve -e -s -P $B/fresh-name-default.ein
run W4 $E solve -e -s -P examples/ein-bugs/alive-set-fresh-name.ein
run W4 $E solve -e -s -P examples/ein-bugs/alive-set-fresh-name-declared.ein
run W4 $E solve -e -s -P $B/fresh-name-nospawn.ein

# W5 — obligations rung + lattice: superset of a recorded model recorded, order-dependent
run W5 $E solve -e -s -P $B/owed-seats.ein
envrun W5 EIN_TRAVERSAL=tree $E solve -e -s -P $B/owed-seats.ein
run W5 $E solve -e -s -P $B/owed-seats-renamed.ein

# W6 — uncovered relation (M1d #4)
run W6 $E solve -e -s -P $B/uncovered.ein
run W6 $E solve -e -s -P $B/uncovered-knows.ein
run W6 $E solve -e -s -H $B/arity3.ein

# W7 — binding key (Q-M1e.16)
run W7 $E saturate --dump corpus/fuzz_findings/binding-key-layout.ein
run W7 $E saturate --dump corpus/fuzz_findings/binding-key-layout-swapped.ein
run W7 $E saturate --dump $S/binding-key-nested-only.ein

# W8 — a NAF-derived positive survives the commitment that falsifies its guard
run W8 $E solve -e -s -P examples/features/07_unstratifiable.ein
run W8 $E solve -e -s -P $B/f07-fpoff.ein
run W8 $E solve -e -s -p $B/stale-naf-fork-warn.ein

# W9 — -K
run W9 $E solve -e -s -P -K examples/ein-bugs/complete-records-stale.ein
run W9 $E solve -e -s -P examples/ein-bugs/complete-records-stale.ein
run W9 $E solve -e -s -K examples/ein-bugs/alive-empty-phase1.ein

# W10 — alternatives cap (Q-M1e.15)
run W10 $E solve examples/ein-bugs/alt-cap-core.ein
run W10 $E solve examples/ein-bugs/alt-cap-core-reordered.ein

# W12 — empty core
run W12 $E solve -e -s examples/ein-bugs/alive-empty-phase1.ein

# S1 — not-exhausted reason
run S1 $E test -v $S/tree-owed.ein
envrun S1 EIN_TRAVERSAL=tree $E test -v $S/tree-owed.ein
run S1 $E solve -e -m 1 -s examples/zebra2.ein

# S2/S3 — trace headline and JSON type on a truncated search
run S2 $E solve -e -s examples/saturation/type-exclusivity/pets.ein --trace $O/pets_trace.md
{ echo '$ head -4 pets_trace.md'; head -4 $O/pets_trace.md; echo; echo '$ head -4 ein.rs/crates/ein-render/tests/golden/trace_unsat.md'; head -4 $R/ein.rs/crates/ein-render/tests/golden/trace_unsat.md; echo; } >> $O/S2.txt
run S2 $E solve -e -m 12 -s examples/saturation/type-exclusivity/pets.ein
$E solve -e examples/saturation/type-exclusivity/pets.ein --json-summary $O/pets.json >/dev/null 2>&1
python3 -c "
import json;d=json.load(open('$O/pets.json'));v=d['verdict']
print('\$ jq .verdict pets.json  (summarised)')
print({k:(v[k] if not isinstance(v[k],list) else '%d items'%len(v[k])) for k in v})" >> $O/S3.txt

# S4 — -H against --json-summary root.hypgen
for f in examples/zebra2.ein examples/zebra2-minus-15-obligations.ein; do
  { echo "\$ ein solve -H -n 1 $f | (root hyps / raw / emitted)"; $E solve -H -n 1 $f 2>&1 | grep -E 'root hyps|^    raw|^    emitted'; } >> $O/S4.txt
  $E solve -n 1 $f --json-summary $O/h.json >/dev/null 2>&1
  python3 -c "
import json;d=json.load(open('$O/h.json'));h=d['root']['hypgen'];print('  json root.hypgen raw',h['raw'],'emitted',h['emitted'])" >> $O/S4.txt
  $E solve -n 1 $f --events $O/h.jsonl >/dev/null 2>&1
  python3 -c "
import json
for l in open('$O/h.jsonl'):
    e=json.loads(l)
    if e['e']=='layer' and e['layer']==1: print('  events layer 1 alive',e['alive']); break" >> $O/S4.txt
  echo >> $O/S4.txt
done

# S5 — clamped budgets (Q-M1e.17)
run S5 $E solve -E=-3 examples/zebra2.ein
run S5 $E solve -m=-3 -s examples/zebra2.ein

# S6 — "(no query goal to project)"
run S6 $E solve -e examples/lattice/02_genuine_3set_death.ein

# S8 — ein.py repr in a core
run S8 $E solve $S/nested-core.ein

# P1 — render panic
run P1 $E render constraints corpus/fuzz_findings/kwpair-below-the-filter.ein
run P1 $E saturate corpus/fuzz_findings/kwpair-below-the-filter.ein

# C1 — branching/06 types as objects
run C1 $E solve -e examples/branching/06_lookahead_on.ein
echo done
