#!/usr/bin/env bash
# P2.2e's report material, written from the binaries into out/ (which git ignores): every command the report
# quotes, its stdout, stderr, exit code and wall time; the renderings, DOT drawn to SVG by Graphviz. No model and
# no GPU: what the model did is replayed from EinAf's committed records. Re-run it on any checkout:
#
#   ./build.sh --engine --einaf
#   plans/m2_nl_to_ir/p2.2e_kernel_fixes/report/make_report.sh            # everything; the model's Zebra program costs 72 s twice
#   plans/m2_nl_to_ir/p2.2e_kernel_fixes/report/make_report.sh --quick    # leaves out the eight slow runs (the cores' searches, h-space)
#
# EIN_BIN and EINAF_BIN name the binaries (default: the release builds). Every command runs from the repository's
# root, with `ein` and `einaf` on PATH as those binaries (out/bin/), so each command is written as you would type
# it; out/INDEX.tsv lists each with its exit code and seconds, out/NAME.cmd holds it as run, out/NAME.out its
# stdout and out/NAME.err its stderr. The runs a time is the point of (the model's Zebra program under every
# question) go through utils/bench_env.sh, pinned to one P-core thread; its fingerprint is each run's stderr.
# "Before" a library fix is today's binary on the library before it (`git archive`, into out/); "before" a binary
# fix is quoted in the README from the stage's record, and no old binary is built. At the end the checkout's
# absolute path is written out of every output, as a path from the root.
set -u
HERE=$(cd "$(dirname "$0")" && pwd)
ROOT=$(git -C "$HERE" rev-parse --show-toplevel)
OUT="$HERE/out"
EIN=${EIN_BIN:-$ROOT/ein.rs/target/release/ein}
EINAF=${EINAF_BIN:-$ROOT/einaf/target/release/einaf}
P=plans/m2_nl_to_ir/p2.2e_kernel_fixes               # the phase, from the root
PR=$P/probes                                          # its probes
Z=plans/m2_nl_to_ir/p2.2_formalizer/from_text/runs/library/zebra/program.ein   # the model's Zebra program
DL=ein.rs/crates/ein-cli/tests/bounded/dead_link.ein  # S2.2e.3's fixture
FT=einaf/tests/fixtures                               # EinAf's recorded runs
cd "$ROOT"
OUT=${OUT#"$ROOT"/}                                   # every path from the root: no output names the host's home

QUICK=0
while [ $# -gt 0 ]; do
  case "$1" in
    --quick) QUICK=1; shift ;;
    -h|--help) sed -n '2,17p' "$HERE/$(basename "$0")"; exit 0 ;;
    *) echo "make_report.sh: unknown option $1" >&2; exit 2 ;;
  esac
done
for b in "$EIN" "$EINAF"; do [ -x "$b" ] || { echo "make_report.sh: no binary $b (./build.sh --engine --einaf)" >&2; exit 2; }; done
command -v dot >/dev/null || { echo "make_report.sh: Graphviz's dot is not on PATH" >&2; exit 2; }
slow() { [ "$QUICK" = 0 ]; }

# A full run starts from an empty out/; --quick rewrites only what it runs.
[ "$QUICK" = 0 ] && rm -rf "$OUT"
mkdir -p "$OUT/bin" "$OUT/tmp"
touch "$OUT/INDEX.tsv"
ln -sf "$EIN" "$OUT/bin/ein"
ln -sf "$EINAF" "$OUT/bin/einaf"
export PATH="$ROOT/$OUT/bin:$PATH"
export TMPDIR="$ROOT/$OUT/tmp"                        # a probe's mktemp lands here, not in /tmp
export CUDA_VISIBLE_DEVICES=""                        # a guard: no replay below touches the GPU, and none can
unset EIN_STDLIB

# rel ARG…: the arguments as a shell line, paths from the root, a word with a character the shell reads quoted: in
# single quotes, or, where it holds one and nothing double quotes expand, in double quotes.
rel() {
  local a out=()
  for a in "$@"; do
    a=${a#"$ROOT"/}; a=${a//"=$ROOT/"/=}
    if [[ $a =~ ^[A-Za-z0-9_./=:@+-]+$ ]]; then out+=("$a")
    elif [[ $a != *"'"* ]]; then out+=("'$a'")
    elif [[ $a != *[\"\$\`]* ]]; then out+=("\"$a\"")
    else out+=("'${a//\'/\'\\\'\'}'"); fi
  done
  echo "${out[*]}"
}

# run NAME SECONDS CMD...: stdout to out/NAME.out, stderr to .err, exit code to .exit, wall seconds to .time, the
# command as run to .cmd, and a line of out/INDEX.tsv.
run() {
  local name=$1 limit=$2; shift 2
  local base="$OUT/$name"; mkdir -p "$(dirname "$base")"
  rel "$@" >"$base.cmd"
  local t0 t1 code
  t0=$(date +%s.%N)
  timeout "$limit" "$@" >"$base.out" 2>"$base.err"; code=$?
  t1=$(date +%s.%N)
  echo "$code" >"$base.exit"
  awk -v a="$t0" -v b="$t1" 'BEGIN { printf "%.2f\n", b - a }' >"$base.time"
  grep -v "^$name	" "$OUT/INDEX.tsv" >"$OUT/INDEX.tsv.new"; mv "$OUT/INDEX.tsv.new" "$OUT/INDEX.tsv"
  printf '%s\t%s\t%s\t%s\n' "$name" "$code" "$(cat "$base.time")" "$(cat "$base.cmd")" >>"$OUT/INDEX.tsv"
  echo "$(date +%H:%M:%S) $name: exit $code, $(cat "$base.time") s" >&2
}

# bench NAME SECONDS CMD...: run, pinned to one P-core thread by utils/bench_env.sh (its fingerprint on stderr).
bench() { local name=$1 limit=$2; shift 2; run "$name" "$limit" utils/bench_env.sh --cores PT:1 "$@"; }

# svg NAME: out/render/NAME.out, one DOT digraph, drawn to out/render/NAME.svg.
svg() {
  local f="$OUT/render/$1"
  dot -Tsvg "$f.out" -o "$f.svg" 2>"$f.dot-err" || echo "dot failed on $f.out" >&2
  [ -s "$f.dot-err" ] || rm -f "$f.dot-err"
}

# view NAME CMD...: an `ein render` command whose stdout is one digraph, run and drawn.
view() { local name=$1; shift; run "render/$name" 300 "$@"; svg "$name"; }

# program NAME FILE: `ein render program FILE`, each of its digraphs (kb, query, rule_…) a .dot of its own under
# out/render/NAME/, each drawn to an .svg beside it.
program() {
  local name=$1 file=$2 d="$OUT/render/$1"
  rm -rf "$d"; mkdir -p "$d"
  run "render/$name/render-program" 300 ein render program "$file"
  awk -v pre="$d/" '/^(strict )?digraph /{ g=$0; sub(/^(strict )?digraph /, "", g); sub(/[ {].*$/, "", g); f = pre g ".dot" } f { print > f }' \
    "$d/render-program.out"
  local g
  for g in "$d"/*.dot; do
    [ -e "$g" ] || continue
    dot -Tsvg "$g" -o "${g%.dot}.svg" 2>"${g%.dot}.dot-err" || echo "dot failed on $g" >&2
    [ -s "${g%.dot}.dot-err" ] || rm -f "${g%.dot}.dot-err"
  done
}

{
  echo "written by make_report.sh on $(date '+%Y-%m-%d %H:%M %Z')$([ "$QUICK" = 1 ] && echo ', --quick')"
  echo "checkout   $(git rev-parse HEAD) ($(git log -1 --format=%s | cut -c1-60)…)"
  echo "code       last commit under ein.rs/ or stdlib/: $(git log -1 --format='%h' -- ein.rs stdlib); under einaf/src, einaf/Cargo.*: $(git log -1 --format='%h' -- einaf/src einaf/Cargo.toml einaf/Cargo.lock)"
  echo "code dirty $(git status --short -- einaf ein.rs stdlib | wc -l) paths under einaf/, ein.rs/, stdlib/"
  echo "ein        $(rel "$EIN"), built $(date -r "$EIN" '+%Y-%m-%d %H:%M')"; "$EIN" --version | sed "s|$ROOT/||; s/^/           /"
  echo "einaf      $(rel "$EINAF"), built $(date -r "$EINAF" '+%Y-%m-%d %H:%M'), $(du -h "$EINAF" | cut -f1) (the cuda feature's build; CUDA_VISIBLE_DEVICES is empty for every command)"
  echo "graphviz   $(dot -V 2>&1)"
  echo "power      AC online=$(cat /sys/class/power_supply/A*/online 2>/dev/null | head -1)"
  echo "gpu        $(nvidia-smi --query-compute-apps=pid,process_name --format=csv,noheader 2>/dev/null | tr '\n' ';' || echo 'no nvidia-smi')"
  echo "pinned runs, utils/bench_env.sh --cores PT:1 --report:"
  utils/bench_env.sh --cores PT:1 --report 2>&1 | sed 's/^/           /'
} >"$OUT/RUN.txt"

# ── S2.2e.1 — a relation declared again, under an import ─────────────────────────────────────────────────────
F25=examples/features/25_declared_twice_with_an_import.ein
E104=plans/m2_nl_to_ir/p2.2_formalizer/zebra_formulations/probes/language
run s1/feature-25-solve             60 ein solve $F25
run s1/feature-25-test              60 ein test $F25
run s1/e104-two-counts              60 ein solve $E104/declared-at-two-counts.ein
run s1/e104-two-counts-import       60 ein solve $E104/declared-at-two-counts-with-an-import.ein
run s1/zebra3.2-the                 60 grep -n '^(relation the ' examples/zebra3.2.ein
run s1/zebra3.2-single              60 ein solve examples/zebra3.2.ein --single
for c in library module-dir; do  # the model's `unused`, both conditions: `has` declared twice beside an import
  U=plans/m2_nl_to_ir/p2.2_formalizer/from_text/runs/$c/unused
  run s1/unused-$c-recorded         60 grep -h -- '--no-search: invalid' $U/stderr.txt
  run s1/unused-$c-outcome          60 einaf outcome $U/program.ein --no-search
  run s1/unused-$c-single           60 einaf outcome $U/program.ein --single
done
# E-129: a rule or a macro written twice, with and without an import
for f in rule-twice-the-same rule-twice-the-same-with-an-import rule-twice-two-bodies rule-twice-two-bodies-with-an-import \
         macro-twice-the-same macro-twice-the-same-with-an-import macro-twice-two-bodies macro-twice-two-bodies-with-an-import; do
  run s1/e129-$f                    60 ein solve $PR/$f.ein
done
# Q-M2.43 (P2.2g's since its move): what Ein does today with a module's relation declared in the program
for f in listed aliased aliased-undeclared full-name; do
  run s1/q-m2.43-$f                 60 ein solve $PR/a-modules-relation/$f.ein
done
run s1/q-m2.43-aliased-saturate     60 ein saturate $PR/a-modules-relation/aliased.ein --dump
run s1/q-m2.43-aliased-undeclared-saturate 60 ein saturate $PR/a-modules-relation/aliased-undeclared.ein --dump

# ── S2.2e.2 — std.slots: two neighbours do not share a place ─────────────────────────────────────────────────
# The library before the fix: b2d8276f's parent, 0ab06c69, under today's binary.
OLD=$OUT/s2/old/stdlib
mkdir -p "$OUT/s2/old"
git archive 0ab06c69 stdlib | tar -x -C "$OUT/s2/old"
run s2/slots-fix-diff               60 git diff 0ab06c69 b2d8276f -- stdlib/slots.ein
run s2/version-old-library          60 env EIN_STDLIB=$OLD ein --version
# The six Zebra encodings with sentence (15) dropped (six.py writes each variant beside its --maximal dump), and
# the whole puzzle under --single, on the library after and before.
run s2/six-after                    900 env EIN_BIN="$EIN" python3 $PR/slots-neighbours/six.py $OUT/s2/six-after
run s2/six-before                   900 env EIN_BIN="$EIN" python3 $PR/slots-neighbours/six.py $OUT/s2/six-before --stdlib $OLD
run s2/zebra3.1-drop15-maximal      300 ein solve $OUT/s2/six-after/zebra3.1.drop-15.ein --maximal
run s2/zebra3.1-drop15-maximal-old  300 env EIN_STDLIB=$OLD ein solve $OUT/s2/six-after/zebra3.1.drop-15.ein --maximal
run s2/zebra2-minus-15-maximal      300 ein solve examples/zebra2-minus-15.ein --maximal
run s2/zebra3.1-single              300 ein solve examples/zebra3.1.ein --single
# An enumeration that is not Ein's: nl2smt, NLTK to Racket to cvc5, on the text with sentence (15) dropped.
run s2/nl2smt-minus-15              600 python3 nl2smt/nl2smt.py nl2smt/puzzles/zebra-minus-15.txt --solutions 100
python3 -I - "$OUT/s2/nl2smt-minus-15.out" "$OUT/s2/six-after/six.json" "$OUT/s2/six-before/six.json" >"$OUT/s2/classes.txt" <<'PY'
import collections, json, re, sys
text = open(sys.argv[1]).read()
pairs = re.findall(r"Who drinks water\?\s+(\w+)\s+Who owns the zebra\?\s+(\w+)", text)
smt = collections.Counter(f"{w}/{z}" for w, z in pairs)
after = json.load(open(sys.argv[2])); before = json.load(open(sys.argv[3]))
print(f"nl2smt: {len(pairs)} solutions, {len(smt)} classes (who drinks water / who owns the zebra)")
encs = list(after)
print("class".ljust(22) + "nl2smt".rjust(7) + "".join(e.rjust(10) for e in encs) + "   | the library before the fix: " + " ".join(encs))
keys = sorted(set(smt) | {k for e in encs for k in after[e]["tally"]} | {k for e in encs for k in before[e]["tally"]})
for k in keys:
    print(k.ljust(22) + str(smt.get(k, 0)).rjust(7) + "".join(str(after[e]["tally"].get(k, 0)).rjust(10) for e in encs)
          + "   | " + " ".join(str(before[e]["tally"].get(k, 0)) for e in encs))
print("states".ljust(22) + str(len(pairs)).rjust(7) + "".join(str(after[e]["maximal"]).rjust(10) for e in encs)
      + "   | " + " ".join(str(before[e]["maximal"]) for e in encs))
for e in encs:
    print(f"{e}: classes as nl2smt's, each over the same count: after {after[e]['tally'] == dict(smt)}, before {before[e]['tally'] == dict(smt)}")
PY
# The three-house instance written for the check, counted by Ein and by brute force.
T3=$PR/slots-neighbours/three-houses.ein
run s2/three-houses-brute           60 python3 $PR/slots-neighbours/three_houses.py
run s2/three-houses-count           300 ein solve $T3 --count
run s2/three-houses-count-old       300 env EIN_STDLIB=$OLD ein solve $T3 --count
run s2/three-houses-describe        300 ein solve $T3 --dump-answer $OUT/s2/three-houses-answer
run s2/three-houses-describe-old    300 env EIN_STDLIB=$OLD ein solve $T3 --dump-answer $OUT/s2/three-houses-answer-old
# The new programs under tests/stdlib/slots/, on the library after and before.
run s2/tests-slots                  300 ein test tests/stdlib/slots
for t in 14_prune_placed_position 15_prune_reflexive 16_spatial_endpoints; do
  run s2/test-$t                    60 ein test tests/stdlib/slots/$t.ein
  run s2/test-$t-old                60 env EIN_STDLIB=$OLD ein test tests/stdlib/slots/$t.ein
done
run s2/solve-14                     60 ein solve tests/stdlib/slots/14_prune_placed_position.ein
run s2/solve-14-old                 60 env EIN_STDLIB=$OLD ein solve tests/stdlib/slots/14_prune_placed_position.ein
# H's structure on zebra.ein, the three counts that moved to zebra2.ein's.
if slow; then
  run s2/h-space-zebra              300 ein render h-space examples/zebra.ein
  run s2/h-space-zebra-old          300 env EIN_STDLIB=$OLD ein render h-space examples/zebra.ein
  run s2/h-space-zebra2             300 ein render h-space examples/zebra2.ein
fi

# ── S2.2e.3 and S2.2e.6 — the clock, and a dead root's core only where a question prints it ──────────────────
# The model's Zebra program under every question, with no clock and under -T 5, pinned; each with its summary.
mkdir -p "$OUT/s3/q"
for clock in none 5; do
  for q in describe --single "-n 2" --minimal --maximal --count --no-search; do
    name=$(echo "$q-$clock" | tr -d ' ' | sed 's/^-*//')
    args=(); [ "$q" != describe ] && args=($q)
    [ "$clock" = 5 ] && args+=(-T 5)
    if [ "$clock" = none ] && { [ "$q" = describe ] || [ "$q" = --no-search ]; } && ! slow; then continue; fi
    bench s3/q/$name 300 ein solve $Z "${args[@]}" --json-summary $OUT/s3/q/$name.json
  done
done
python3 -I - "$OUT/s3/q" >"$OUT/s3/questions.tsv" <<'PY'
import glob, json, os, re, sys
d = sys.argv[1]
rows = []
for q in ["describe", "single", "n2", "minimal", "maximal", "count", "no-search"]:
    for clock in ["none", "5"]:
        b = os.path.join(d, f"{q}-{clock}")
        if not os.path.exists(b + ".out"):
            continue
        out = open(b + ".out").read(); err = open(b + ".err").read()
        code = open(b + ".exit").read().strip(); wall = open(b + ".time").read().strip()
        m = re.search(r"^  (verdict|states of R|minimal line|maximal line|empty|R) +(.*)$", out, re.M)
        answer = f"{m.group(1)} {m.group(2)}" if m else ""
        if code == "2":
            m = re.search(r"\*\* aborted[^\n]*", err)
            answer = m.group(0) if m else err.strip().splitlines()[-1]
        rows_core = len(re.findall(r"^  unsat core ", out, re.M))
        try:
            core = json.dumps(json.load(open(b + ".json"))["region"]["core"])
        except Exception:
            core = "no answer" if code == "2" else ("no region block" if os.path.exists(b + ".json") else "no summary")
        rows.append((q, clock, code, wall, answer, str(rows_core), core))
print("question\tclock\texit\twall s (bench_env's fingerprint included)\tthe answer's row\tunsat core rows\tregion.core")
for r in rows:
    print("\t".join(r))
PY
run s3/model-zebra-single-timing    300 ein solve $Z --single -t
# The lever Q-M2.34 asks about, on a copy: one justification a fact.
{ cat $Z; printf '\n;; appended by make_report.sh: one justification a fact (Q-M2.34)\n(config :record-alternative-justifications false)\n'; } \
  >"$OUT/s3/model-zebra-lever.ein"
run s3/model-zebra-lever-describe   300 ein solve $OUT/s3/model-zebra-lever.ein
# The fixture: a root that dies on a link several rules derive.
slow && run s3/dead-link-describe   300 ein solve $DL
for q in describe --single --count "-n 2" --minimal --maximal --no-search; do
  name=$(echo "$q" | tr -d ' ' | sed 's/^-*//')
  args=(); [ "$q" != describe ] && args=($q)
  run s3/dead-link-$name-T0.3       60 ein solve $DL "${args[@]}" -T 0.3
done
run s3/dead-link-test-T0.3          60 ein test $DL -T 0.3
run s3/dead-link-test-summary-T0.3  60 ein test $DL -T 0.3 --json-summary $OUT/s3/dead-link-test-summary.json
run s3/dead-link-oracle-T0.3        60 ein oracle $DL -T 0.3
run s3/dead-link-single-events-T0.3 60 ein solve $DL --single --events $OUT/s3/dead-link-single-events.jsonl -T 0.3
# EinAf's bounded call: the outcome a question, under a clock.
for q in describe --single --count --no-search; do
  name=$(echo "$q" | sed 's/^-*//')
  args=(); [ "$q" != describe ] && args=($q)
  run s3/einaf-dead-link-$name      60 einaf outcome $DL "${args[@]}" -T 0.2
done
run s3/einaf-dead-link-feedback-single 60 einaf feedback $DL --single -T 0.2
run s3/einaf-dead-link-feedback-describe 60 einaf feedback $DL -T 0.2
python3 -I - "$OUT/s3" >"$OUT/s3/einaf-dead-link-feedback-fields.txt" <<'PY'
import json, sys
for q in ["single", "describe"]:
    d = json.load(open(f"{sys.argv[1]}/einaf-dead-link-feedback-{q}.out"))
    print(f"einaf feedback {'--single ' if q == 'single' else ''}-T 0.2:")
    for k in ["asked", "outcome", "core", "diagnostics", "work"]:
        print(f"  {k:12} {json.dumps(d[k], ensure_ascii=False)}")
PY
run s3/einaf-model-zebra-single     60 einaf outcome $Z --single
run s3/einaf-model-zebra-describe-T5 60 einaf outcome $Z -T 5
run s3/einaf-model-zebra-no-search-T5 60 einaf outcome $Z --no-search -T 5

# ── S2.2e.4 — the order of a program's forms, and two rows of the table ──────────────────────────────────────
run s4/order-run                    300 env EIN_BIN="$EIN" sh $PR/order-of-forms/run.sh
O=$PR/order-of-forms
run s4/gap-reader-first             60 ein solve $O/gap-reader-first.ein
run s4/gap-producer-first           60 ein solve $O/gap-producer-first.ein
run s4/gap-diff                     60 diff $O/gap-reader-first.ein $O/gap-producer-first.ein
run s4/one-food-ann-first-n1        60 ein solve $O/one-food-each-ann-first.ein -n 1
run s4/one-food-carl-first-n1       60 ein solve $O/one-food-each-carl-first.ein -n 1
run s4/one-food-ann-first           60 ein solve $O/one-food-each-ann-first.ein
run s4/one-food-carl-first          60 ein solve $O/one-food-each-carl-first.ein
run s4/decision-held-by             60 grep -A1 '^\*\*Held by' docs/decisions/language/Q-M2.24-an-answer-does-not-depend-on-the-order.md
# The table's two new rows: as banked, through the binary, and through EinAf's grammar (each file the row's
# program as the table writes it, with no newline after it).
run s4/table-rows                   60 sed -n '/^## 352\./,$p' ein.rs/crates/ein-ir/tests/golden/grammar_table.txt
printf '%s' '(relation t T :why "t"#|c|#)' >"$OUT/s4/row-352.ein"
printf '%s' '(relation p T) (p a ;c)' >"$OUT/s4/row-353.ein"
run s4/row-352-solve                60 ein solve $OUT/s4/row-352.ein
run s4/row-353-solve                60 ein solve $OUT/s4/row-353.ein
run s4/rows-einaf-accept            60 einaf grammar accept $OUT/s4/row-352.ein $OUT/s4/row-353.ein

# ── S2.2e.5 — EinAf's recordings follow the library ──────────────────────────────────────────────────────────
run s5/catalogue-line3              60 bash -c 'einaf catalogue | sed -n 3p'
run s5/catalogue-line3-recorded     60 bash -c "git show 969ae30d^:$FT/steps/bob1/calls.jsonl | grep -o 'The library is sha256:[0-9a-f]*\\.' | sort | uniq -c"
run s5/catalogue-generated-head     60 bash -c 'einaf catalogue --generated | sed -n 1,3p'
# The two runs recorded again, replayed with no model.
ZC=$FT/from_text/zebra-contradiction
rm -rf "$OUT/s5/zc" "$OUT/s5/bob1" "$OUT/s5/zc-moved"
run s5/zc-replay                    300 einaf from-text $ZC/text.txt --id zebra-contradiction --answer-line zebra \
    --replay $ZC/generations.jsonl --gold-answer @contradiction --record $OUT/s5/zc
run s5/zc-same-program              60 cmp $OUT/s5/zc/program.ein $ZC/program.ein
run s5/zc-run-stdlib                60 python3 -c 'import json, sys; r = json.load(open(sys.argv[1]))["run"]; print(r["stdlib"], sorted({s["stdlib"] for s in r["steps"]}))' $OUT/s5/zc/run.json
run s5/zc-core                      300 ein solve $ZC/program.ein --no-search
run s5/bob1-steps                   300 einaf steps run $FT/steps/bob1/text.txt --id bob1 --answer-line word --out $OUT/s5/bob1 \
    --replay $FT/steps/bob1/calls.jsonl
run s5/bob1-from-text               300 einaf from-text $FT/steps/bob1/text.txt --answer-line word --replay $FT/steps/bob1/calls.jsonl --gold-answer 3
# What moved in the re-recording beyond the catalogue's line: the programs before (969ae30d's parent) and after.
git show "969ae30d^:$FT/steps/bob1/run.json" | python3 -c 'import json, sys; print(json.load(sys.stdin)["program"], end="")' >"$OUT/s5/bob1-before.ein"
python3 -c 'import json, sys; print(json.load(open(sys.argv[1]))["program"], end="")' $FT/steps/bob1/run.json >"$OUT/s5/bob1-after.ein"
run s5/bob1-program-diff            60 diff $OUT/s5/bob1-before.ein $OUT/s5/bob1-after.ein
run s5/bob1-before-single           300 ein solve $OUT/s5/bob1-before.ein --single
run s5/bob1-after-single            300 ein solve $OUT/s5/bob1-after.ein --single
git show "969ae30d^:$ZC/program.ein" >"$OUT/s5/zc-before.ein"
run s5/zc-program-diff              60 git diff --stat 969ae30d^ 969ae30d -- $ZC $FT/steps/bob1
run s5/zc-program-diff-text         60 git diff 969ae30d^ 969ae30d -- $ZC/program.ein
run s5/zc-before-core               300 ein solve $OUT/s5/zc-before.ein --no-search
# The probe line judged again from its own record, with no model.
python3 -c 'import json, sys; [print(json.dumps(json.loads(l)["planned"], ensure_ascii=False)) for l in open(sys.argv[1]) if l.strip()]' \
  $FT/model/probe_lines.jsonl >"$OUT/s5/probe-plan.jsonl"
rm -f "$OUT/s5/probe-again.jsonl"
run s5/probe-replay                 300 einaf probe run --plan $OUT/s5/probe-plan.jsonl --out $OUT/s5/probe-again.jsonl --replay $FT/model/probe_lines.jsonl
python3 -I - "$OUT/s5/probe-again.jsonl" "$FT/model/probe_lines.jsonl" >"$OUT/s5/probe-compare.txt" <<'PY'
import json, sys
TIMES = {"run_us", "load_us", "ask_us"}
def strip(v, path=""):
    if isinstance(v, dict):
        return {k: strip(x, f"{path}.{k}") for k, x in v.items() if not (k in TIMES and path.endswith(".work"))}
    if isinstance(v, list):
        return [strip(x, path) for x in v]
    return v
def diffs(a, b, path=""):
    if isinstance(a, dict) and isinstance(b, dict):
        for k in sorted(set(a) | set(b)):
            yield from diffs(a.get(k), b.get(k), f"{path}.{k}")
    elif isinstance(a, list) and isinstance(b, list) and len(a) == len(b):
        for i, (x, y) in enumerate(zip(a, b)):
            yield from diffs(x, y, f"{path}[{i}]")
    elif a != b:
        yield path
again = [json.loads(l) for l in open(sys.argv[1]) if l.strip()]
fixture = [json.loads(l) for l in open(sys.argv[2]) if l.strip()]
print(f"{len(again)} lines judged again, {len(fixture)} in the fixture")
for i, (a, f) in enumerate(zip(again, fixture)):
    d = list(diffs(strip(a), strip(f)))
    print(f"line {i + 1}: " + ("the same as the fixture, field for field, but the judge's own times (work.run_us, load_us, ask_us)"
                              if not d else "differs at " + ", ".join(d)))
PY
# A library whose digest moved and whose catalogue did not: a comment added to std.math.gcd, its manifest line
# written again (as einaf/tests/from_text.rs does), and the one-shot fixture replayed under it.
MOVED=$OUT/s5/moved/stdlib
rm -rf "$OUT/s5/moved"; mkdir -p "$OUT/s5/moved"; cp -r stdlib "$MOVED"
printf ';; A comment the report adds: the module'"'"'s forms are as they were.\n' >>"$MOVED/math/gcd.ein"
sed -i "s|^[0-9a-f]\{64\}  math/gcd.ein\$|$(sha256sum "$MOVED/math/gcd.ein" | cut -c1-64)  math/gcd.ein|" "$MOVED/MANIFEST.sha256"
run s5/moved-version                60 env EIN_STDLIB=$MOVED ein --version
run s5/moved-catalogue-same         60 bash -c "cmp <(EIN_STDLIB=$MOVED einaf catalogue) <(einaf catalogue) && echo 'einaf catalogue: the same text under the moved library as under the checkout'"
run s5/moved-zc-replay              300 env EIN_STDLIB=$MOVED einaf from-text $ZC/text.txt --id zebra-contradiction --answer-line zebra \
    --replay $ZC/generations.jsonl --gold-answer @contradiction --record $OUT/s5/zc-moved
run s5/moved-zc-same-program        60 cmp $OUT/s5/zc-moved/program.ein $ZC/program.ein
run s5/moved-zc-run-stdlib          60 python3 -c 'import json, sys; r = json.load(open(sys.argv[1]))["run"]; print(r["stdlib"], sorted({s["stdlib"] for s in r["steps"]}))' $OUT/s5/zc-moved/run.json

# ── S2.2e.7 — two more cores taken only where a question prints them ─────────────────────────────────────────
# Its two fixtures, pinned: a flagged program whose minimal hole dies on the link, and a monotone one whose whole
# node does; and E-132's probe, the whole node's core under --events, whose search reads no clock.
B=ein.rs/crates/ein-cli/tests/bounded
HOLE=$B/dead_link_hole.ein
WHOLE=$B/dead_link_whole.ein
WCLOCK=$PR/two-more-cores-only-where-printed/whole-node-clock.ein
mkdir -p "$OUT/s7/q"
bench s7/q/hole-maximal             300 ein solve $HOLE --maximal
bench s7/q/hole-maximal-T5          300 ein solve $HOLE --maximal -T 5
bench s7/q/hole-minimal             300 ein solve $HOLE --minimal
bench s7/q/hole-minimal-T5          300 ein solve $HOLE --minimal -T 5
bench s7/q/hole-maximal-single-T5   300 ein solve $HOLE --maximal --single -T 5
bench s7/q/hole-count-T5            300 ein solve $HOLE --count -T 5
slow && bench s7/q/hole-describe    300 ein solve $HOLE
bench s7/q/whole-single             300 ein solve $WHOLE --single
bench s7/q/whole-single-v-T5        300 ein solve $WHOLE --single -v -T 5
bench s7/q/whole-single-layer-progress-T5 300 ein solve $WHOLE --single --layer-progress -T 5
bench s7/q/whole-n2-v-T5            300 ein solve $WHOLE -n 2 -v -T 5
slow && bench s7/q/wclock-events-T5 300 ein solve $WCLOCK --single --events $OUT/s7/wclock-events.jsonl -T 5

# ── Renderings: `ein render`, drawn by Graphviz ──────────────────────────────────────────────────────────────
mkdir -p "$OUT/render"
# S2.2e.1: the three declarations of `likes`, each a fact of `relation`, beside an import
program feature-25 $F25
view feature-25-kb ein render kb --fold $F25
# S2.2e.2: the rule before and after, from the module run as a program; test 14's exclusions on each library
view slot-prune-fwd-after ein render rule --name slot-prune-fwd stdlib/slots.ein
view slot-prune-fwd-before env EIN_STDLIB=$OLD ein render rule --name slot-prune-fwd $OLD/slots.ein
view slots-14-excluded-after ein render kb --fold --query 1 tests/stdlib/slots/14_prune_placed_position.ein
view slots-14-excluded-before env EIN_STDLIB=$OLD ein render kb --fold --query 1 tests/stdlib/slots/14_prune_placed_position.ein
view slots-14-kb ein render kb --fold tests/stdlib/slots/14_prune_placed_position.ein
# S2.2e.3: the fixture's rules, and its links on three towns (the fixture has thirty), each drawn with the rule
# whose derivation was recorded first
program dead-link $DL
awk '/^\(road1 t3 t4\)/ { skip = 1 } /^\(solve / { skip = 0 } !skip' $DL | sed 's/(apart t1 t30)/(apart t1 t3)/' \
  >"$OUT/render/dead-link-3-towns.ein"
run render/dead-link-3-towns-solve  60 ein solve $OUT/render/dead-link-3-towns.ein --no-search
view dead-link-3-towns-links ein render kb --fold --colour-by origin --goal '(link ?a ?b)' $OUT/render/dead-link-3-towns.ein
# S2.2e.4: root's fixpoint of the pair that differs by order
program gap-reader-first $O/gap-reader-first.ein
view gap-reader-first-kb ein render kb --fold $O/gap-reader-first.ein
view gap-producer-first-kb ein render kb --fold $O/gap-producer-first.ein
# S2.2e.5: bob1's program before and after the re-recording, its rules and its answer query's slice
program bob1-before $OUT/s5/bob1-before.ein
program bob1-after $OUT/s5/bob1-after.ein
view bob1-before-rows ein render kb --fold --query 1 $OUT/s5/bob1-before.ein
view bob1-after-rows ein render kb --fold --query 1 $OUT/s5/bob1-after.ein
# S2.2e.7: the hole fixture's cube, an oracle dump drawn; from a copy with the lever, which moves a dead node's core
# and nothing else (Q-M1e2.30's one exemption), so that the oracle, which takes every dead node's core, is instant
{ cat $HOLE; printf '\n;; appended by make_report.sh: one justification a fact, so the oracle'"'"'s cores cost nothing\n(config :record-alternative-justifications false)\n'; } \
  >"$OUT/render/dead-link-hole-lever.ein"
rm -rf "$OUT/render/dead-link-hole-cube"
run render/dead-link-hole-oracle    60 ein oracle $OUT/render/dead-link-hole-lever.ein --dump $OUT/render/dead-link-hole-cube
view dead-link-hole-region ein render region $OUT/render/dead-link-hole-cube
# The size of each picture
{
  printf 'picture\tedges\tsvg bytes\n'
  for s in $(cd "$OUT/render" && find . -name '*.svg' | sort); do
    s=${s#./}; d="$OUT/render/${s%.svg}"
    if [ -e "$d.out" ]; then src="$d.out"; else src="$d.dot"; fi
    printf '%s\t%s\t%s\n' "$s" "$(grep -c -- '->' "$src")" "$(stat -c %s "$OUT/render/$s")"
  done
} >"$OUT/render/SIZES.tsv"

rm -rf "$OUT/tmp"
# The checkout's absolute path, where a binary printed one (a library named by $EIN_STDLIB, bench_env's taskset
# line), written from the root as every other path is, so that no output names the host's home.
grep -rlF --include='*.out' --include='*.err' --include='*.txt' --include='*.tsv' "$ROOT/" "$OUT" | xargs -r sed -i "s|$ROOT/||g"
find "$OUT" -type f -empty -delete
echo "make_report.sh: done, $(wc -l <"$OUT/INDEX.tsv") commands; $OUT/INDEX.tsv" >&2
