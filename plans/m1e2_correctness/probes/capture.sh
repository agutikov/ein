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

# ── P1e2.3's re-take (S1e2.3.4, 2026-11-03): the S and P lines in today's
# spellings, writing the phase README's table's evidence. The W lines above are
# the milestone's: its acceptance re-takes them at its close, and since the
# switch most of them are refused as retired options (`-e`, `-m`, `-L`, `-K`).
P="$R/plans/m1e2_correctness/p1e2.3_surfaces_that_misreport_the_answer"
PETS=examples/saturation/type-exclusivity/pets.ein

# S1 — not-exhausted reason: gone. A stopped run says what it was asked, and a
# partial NOT CHECKED names the question. (`probes/tree-owed.ein` was dropped
# at S1e2.3.4: `:no-hypothesis` belongs on `(solve …)` since S1e2.1a.4.)
run S1 $E solve -n 1 examples/zebra2.ein
run S1 $E test -v --region-budget 2 examples/branching/16_lookahead_two_step_off.ein

# S2/S3 — the trace's headline and the summary's verdict on a cut run: gone.
for b in "" "--region-budget 100"; do
  $E solve $PETS $b --trace $O/pets_trace.md --json-summary $O/pets.json >/dev/null 2>&1
  { echo "\$ ein solve $PETS $b --trace T.md --json-summary J  # head -8 T.md"; head -8 $O/pets_trace.md; echo; } >> $O/S2.txt
  python3 -c "
import json;d=json.load(open('$O/pets.json'));r=d['region']
print('\$ ein solve $PETS $b --json-summary J  # verdict, and region (summarised)')
print('verdict', d.get('verdict'))
print({k:r[k] for k in ('exhausted','k_s','k_c','empty','stopped') if k in r})
print()" >> $O/S3.txt
done

# S4 — -H against the summary's root.hypgen and region.universe: fixed at
# S1e2.3.1. Every one of the three, on the files where they disagreed.
for f in examples/branching/02_one_dead_one_alive.ein $B/owed-seats.ein $B/arity3.ein; do
  { echo "\$ ein solve -n 1 -H $f  # the H line"; $E solve -n 1 -H $f 2>&1 | grep '^H — the universe'; } >> $O/S4.txt
  $E solve -n 1 $f --json-summary $O/h.json >/dev/null 2>&1
  python3 -c "
import json;d=json.load(open('$O/h.json'))
print('  root.hypgen.emitted', d['root']['hypgen']['emitted'], ' region.universe.candidates', d['region']['universe']['candidates'], ' root.facts', d['root']['facts'])
print()" >> $O/S4.txt
done

# S5 — -g: 0 is no node lines, a negative refused (S1e2.3.4, Q-M1e.17 ruled (b)).
for g in "-g 1" "-g 0" "-g=-1" "-g -1"; do
  $E solve examples/branching/05_mini_zebra.ein -v $g >/dev/null 2>$O/g.err
  rc=$?
  { echo "\$ ein solve examples/branching/05_mini_zebra.ein -v $g"
    echo "  exit $rc · $(grep -c '^  node ' $O/g.err) node lines · $(grep -c ' visited — ' $O/g.err) step lines"
    grep '^error' $O/g.err; echo; } >> $O/S5.txt
done

# S6 — "(no query goal to project)": gone.
run S6 $E solve examples/lattice/02_genuine_3set_death.ein

# S7 — the goal row under a permuted interner: gone, and pinned by the corpus
# entry examples/ein-bugs/d3-goal-row-order.ein, which id_order_invariance.rs
# permutes per commit. Here, the class that holds both rows.
run S7 $E solve examples/ein-bugs/d3-goal-row-order.ein

# S8 — a struct repr in a core: gone, pinned by ein-cli/tests/read_out.rs.
run S8 $E solve examples/ein-bugs/nested-core.ein

# S9 — EIN_TRAVERSAL=tree against the same without: identical, timing rows aside.
$E solve -s $B/owed-seats.ein --json-summary $O/t0.json > $O/t0.txt 2>&1
EIN_TRAVERSAL=tree $E solve -s $B/owed-seats.ein --json-summary $O/t1.json > $O/t1.txt 2>&1
{ echo "\$ diff <(ein solve -s owed-seats.ein) <(EIN_TRAVERSAL=tree ein solve -s owed-seats.ein), wall rows aside"
  diff <(grep -v 'wall' $O/t0.txt) <(grep -v 'wall' $O/t1.txt) && echo "  stdout identical"
  cmp -s $O/t0.json $O/t1.json && echo "  summaries identical"; echo; } >> $O/S9.txt

# P1 — a keyword pair below a form: both doors draw it since S1e2.3.2.
run P1 $E render constraints examples/ein-bugs/kwpair-below-the-filter.ein
run P1 $E render program examples/ein-bugs/kwpair-below-the-filter.ein

# S10 — the forms past 100 lines: they share them since S1e2.3.3.
for a in "" "--models grid" "--models key" "-p" "-P" "-f" "-H"; do
  echo "\$ ein solve $PETS $a | wc -lc: $($E solve $PETS $a 2>/dev/null | wc -lc)" >> $O/S10.txt
done

# C1 — branching/06 types as objects
run C1 $E solve -e examples/branching/06_lookahead_on.ein
echo done
