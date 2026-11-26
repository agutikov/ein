#!/usr/bin/env bash
# P2.2's report material, written from the binaries into out/: every command the report quotes, its stdout,
# stderr, exit code and wall time; the programs' renderings, DOT drawn to SVG by Graphviz. With no model by
# default: everything the model did is replayed from the committed records of S2.2.4 and S2.2.5. Re-run it to
# see the phase's behaviour on the current checkout:
#
#   ./build.sh --engine --einaf
#   plans/m2_nl_to_ir/p2.2_formalizer/report/make_report.sh              # no model; 8 min here (8 min 34 s with --model), E-126 most of it
#   plans/m2_nl_to_ir/p2.2_formalizer/report/make_report.sh --quick      # skips the five slow runs (E-126); 35 s here
#
# and, with the model's runtime built (the `cuda` feature) and the model's file, one live run of `bob1`:
#
#   CMAKE_CUDA_ARCHITECTURES=89 cargo build --release --features cuda --manifest-path einaf/Cargo.toml
#   plans/m2_nl_to_ir/p2.2_formalizer/report/make_report.sh --model /var/lib/ollama/blobs/sha256-a9d0a3e1…
#   plans/m2_nl_to_ir/p2.2_formalizer/report/make_report.sh --live-only --model …     # the live run alone
#
# EIN_BIN and EINAF_BIN name the binaries (default: the release builds). Every command is run from the
# repository's root; out/INDEX.tsv lists each with its exit code and seconds, out/NAME.cmd holds it as run.
# out/.gitignore is written here too: a file over 200 KB (an SVG over 250 KB), every DOT, each command's .cmd,
# .exit and .time (INDEX.tsv has them), and every model record a replay writes again are not committed.
set -u
HERE=$(cd "$(dirname "$0")" && pwd)
ROOT=$(git -C "$HERE" rev-parse --show-toplevel)
OUT="$HERE/out"
EIN=${EIN_BIN:-$ROOT/ein.rs/target/release/ein}
EINAF=${EINAF_BIN:-$ROOT/einaf/target/release/einaf}
P=plans/m2_nl_to_ir/p2.2_formalizer          # the phase, from the root
FT=$P/from_text/runs                          # S2.2.5's committed records
TX=$P/steps/texts                             # the word problems' texts
cd "$ROOT"
OUT=${OUT#"$ROOT"/}                           # every path from the root: no output names the host's home

MODEL=""; QUICK=0; LIVE_ONLY=0
while [ $# -gt 0 ]; do
  case "$1" in
    --model) MODEL=${2:?--model needs the file of the model}; shift 2 ;;
    --quick) QUICK=1; shift ;;
    --live-only) LIVE_ONLY=1; shift ;;
    -h|--help) sed -n '2,20p' "$HERE/$(basename "$0")"; exit 0 ;;
    *) echo "make_report.sh: unknown option $1" >&2; exit 2 ;;
  esac
done
[ "$LIVE_ONLY" = 1 ] && [ -z "$MODEL" ] && { echo "make_report.sh: --live-only needs --model" >&2; exit 2; }
for b in "$EIN" "$EINAF"; do [ -x "$b" ] || { echo "make_report.sh: no binary $b (./build.sh --engine --einaf)" >&2; exit 2; }; done
command -v dot >/dev/null || { echo "make_report.sh: Graphviz's dot is not on PATH" >&2; exit 2; }

# A full run starts from an empty out/; --quick and --live-only rewrite only what they run.
if [ "$QUICK" = 0 ] && [ "$LIVE_ONLY" = 0 ]; then rm -rf "$OUT"; fi
mkdir -p "$OUT"
touch "$OUT/INDEX.tsv"

# rel ARG…: the arguments as a shell line, paths from the root, a word with a character the shell reads quoted.
rel() {
  local a out=()
  for a in "$@"; do
    a=${a#"$ROOT"/}
    if [[ $a =~ ^[A-Za-z0-9_./=:@+-]+$ ]]; then out+=("$a"); else out+=("'$a'"); fi   # no argument here holds a '
  done
  echo "${out[*]}"
}

# run NAME SECONDS CMD...: stdout to out/NAME.out, stderr to .err, exit code to .exit, wall seconds to .time,
# the command as run to .cmd, and a line of out/INDEX.tsv.
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

# draw NAME FILE: `ein render program FILE` into out/render/NAME/, each digraph of it a .dot of its own named by
# the graph (kb, query, rule_…), and each drawn to an .svg beside it.
draw() {
  local name=$1 file=$2 d="$OUT/render/$1"
  rm -rf "$d"; mkdir -p "$d"
  run "render/$name/render-program" 300 "$EIN" render program "$file"
  awk -v pre="$d/" '/^(strict )?digraph /{ g=$0; sub(/^(strict )?digraph /, "", g); sub(/[ {].*$/, "", g); f = pre g ".dot" } f { print > f }' \
    "$d/render-program.out"
  local g
  for g in "$d"/*.dot; do
    [ -e "$g" ] || continue
    dot -Tsvg "$g" -o "${g%.dot}.svg" 2>"${g%.dot}.dot-err" || echo "dot failed on $g" >&2
    [ -s "${g%.dot}.dot-err" ] || rm -f "${g%.dot}.dot-err"
  done
}

slow() { [ "$QUICK" = 0 ]; }

if [ "$LIVE_ONLY" = 0 ]; then
{
  echo "written by make_report.sh on $(date '+%Y-%m-%d %H:%M %Z')"
  echo "checkout   $(git rev-parse HEAD) ($(git log -1 --format=%s | cut -c1-60)…)"
  echo "code dirty $(git status --short -- einaf ein.rs stdlib | wc -l) paths under einaf/, ein.rs/, stdlib/"
  echo "ein        $(rel "$EIN")"; "$EIN" --version | sed "s|$ROOT/||; s/^/           /"
  echo "einaf      $(rel "$EINAF"), $(du -h "$EINAF" | cut -f1) (about 630M with the model's runtime, the cuda feature; about 19M without)"
  echo "graphviz   $(dot -V 2>&1)"
  echo "power      AC online=$(cat /sys/class/power_supply/A*/online 2>/dev/null | head -1)"
  echo "mode       quick=$QUICK model=${MODEL:+given}"
} >"$OUT/RUN.txt"

# ── 1. The kernel through EinAf's one door: outcome and feedback ─────────────────────────────────────────────
run kernel/outcome-hand-zebra-step1-no-search 300 "$EINAF" outcome einaf/contract/zebra/step1.ein --no-search
run kernel/outcome-hand-zebra-step2-single    300 "$EINAF" outcome einaf/contract/zebra/step2.ein --single
run kernel/outcome-hand-zebra-step3-single    300 "$EINAF" outcome einaf/contract/zebra/step3.ein --single
run kernel/outcome-model-bystander-single     300 "$EINAF" outcome $FT/module-dir/bystander/program.ein --single
run kernel/feedback-model-bystander-single    300 "$EINAF" feedback $FT/module-dir/bystander/program.ein --single

# ── 2. The grammars ──────────────────────────────────────────────────────────────────────────────────────────
run grammars/write-check            300 "$EINAF" grammar write --check
run grammars/render-1-zebra-fence   300 "$EINAF" grammar render 1 --text nl2smt/puzzles/zebra.txt --fence
run grammars/render-2-zebra-fence   300 "$EINAF" grammar render 2 --program einaf/contract/zebra/step1.ein --fence
run grammars/render-whole-fence     300 "$EINAF" grammar render whole --fence
run grammars/accept-root            300 "$EINAF" grammar accept --root root $FT/library/zebra/program.ein examples/zebra3.1.ein einaf/contract/zebra/step3.ein
run grammars/accept-program         300 "$EINAF" grammar accept examples/zebra3.1.ein einaf/contract/zebra/step3.ein

# ── 3. The catalogue ─────────────────────────────────────────────────────────────────────────────────────────
run catalogue/catalogue             300 "$EINAF" catalogue
run catalogue/catalogue-grammar     300 "$EINAF" catalogue --grammar

# ── 4. The contract's check, where the kernel loads in silence ───────────────────────────────────────────────
run check/model-gap                 300 "$EINAF" steps check $FT/library/gap/program.ein --text $TX/gap.txt
run check/model-gap-ein-single      300 "$EIN" solve $FT/library/gap/program.ein --single
run check/model-zebra               300 "$EINAF" steps check $FT/library/zebra/program.ein --text nl2smt/puzzles/zebra.txt
run check/model-bystander           300 "$EINAF" steps check $FT/module-dir/bystander/program.ein --text $TX/bystander.txt
run check/hand-zebra-step3          300 "$EINAF" steps check einaf/contract/zebra/step3.ein --text einaf/contract/zebra/text.txt

# ── 5. The steps, replayed from S2.2.4's gate fixture ────────────────────────────────────────────────────────
rm -rf "$OUT/steps/bob1"
run steps/bob1-replay               300 "$EINAF" steps run $TX/bob1.txt --id bob1 --answer-line word \
    --replay einaf/tests/fixtures/steps/bob1/calls.jsonl --out "$OUT/steps/bob1"

# ── 6. The one shot, replayed from S2.2.5's committed records (no model) ─────────────────────────────────────
ft() {  # ft NAME CONDITION ITEM TEXT LINE GOLD [OPTIONS…]: einaf from-text --replay, its record under out/from-text/NAME/record
  local name=$1 cond=$2 item=$3 text=$4 line=$5 gold=$6; shift 6
  rm -rf "$OUT/from-text/$name/record"
  run "from-text/$name/replay" 900 "$EINAF" from-text "$text" --id "$item" --answer-line "$line" --gold-answer "$gold" \
      --replay "$FT/$cond/$item/generations.jsonl" --record "$OUT/from-text/$name/record" "$@"
  cmp -s "$OUT/from-text/$name/record/program.ein" "$FT/$cond/$item/program.ein" \
    && echo "the replayed program is the committed one, byte for byte" >"$OUT/from-text/$name/same-program.txt" \
    || echo "the replayed program DIFFERS from $FT/$cond/$item/program.ein" >"$OUT/from-text/$name/same-program.txt"
}
slow && ft zebra library zebra nl2smt/puzzles/zebra.txt zebra 'Norwegian water|Japanese zebra'
ft zebra-minus-15      library zebra-minus-15      nl2smt/puzzles/zebra-minus-15.txt      zebra @undetermined
ft zebra-contradiction library zebra-contradiction nl2smt/puzzles/zebra-contradiction.txt zebra @contradiction
ft bob1                library bob1                $TX/bob1.txt word 3
run from-text/bob1/replay-no-format 300 "$EINAF" from-text $TX/bob1.txt --id bob1 --answer-line word \
    --replay $FT/library/bob1/generations.jsonl --no-format
ft bystander           module-dir bystander $TX/bystander.txt word 9 --module-dir examples/word_problems \
    --explain "$OUT/from-text/bystander/explain.md"
run from-text/bystander/replay-no-format 300 "$EINAF" from-text $TX/bystander.txt --id bystander --answer-line word \
    --module-dir examples/word_problems --replay $FT/module-dir/bystander/generations.jsonl --no-format
ft gap                 library gap $TX/gap.txt word @undetermined

# ── 7. Ein on the programs: the answers, H, and E-126 ────────────────────────────────────────────────────────
Z=$FT/library/zebra/program.ein
if slow; then
  run ein/model-zebra-describe       900 "$EIN" solve $Z
  run ein/model-zebra-single         900 "$EIN" solve $Z --single
  run ein/model-zebra-single-T5      900 "$EIN" solve $Z --single -T 5
fi
mkdir -p "$OUT/ein"
{ cat $Z; printf '\n;; appended by make_report.sh, E-126: one justification a fact\n(config :record-alternative-justifications false)\n'; } \
  >"$OUT/ein/model-zebra-lever.ein"
run ein/model-zebra-lever-single     300 "$EIN" solve "$OUT/ein/model-zebra-lever.ein" --single
run ein/model-zebra-lever-describe   300 "$EIN" solve "$OUT/ein/model-zebra-lever.ein"
# What kills the model's Zebra root: the lever's copy (the lever moves a dead node's core, never whether it is dead)
# less the memberships of the six houses step 1 made of a description (the coloured ones and the middle one), less
# step 2's four kinds that have no member, and less both.
HOUSES='^\(instance (Green|Red|Ivory|Yellow|Blue|Middle)House House'
KINDS='^\(type (Smoker|Drinker|Owner|Color) Attribute\)'
grep -v -E "$HOUSES" "$OUT/ein/model-zebra-lever.ein" >"$OUT/ein/model-zebra-less-houses.ein"
grep -v -E "$KINDS" "$OUT/ein/model-zebra-lever.ein" >"$OUT/ein/model-zebra-less-kinds.ein"
grep -v -E "$HOUSES|$KINDS" "$OUT/ein/model-zebra-lever.ein" >"$OUT/ein/model-zebra-less-both.ein"
run ein/model-zebra-less-houses      300 "$EIN" solve "$OUT/ein/model-zebra-less-houses.ein"
run ein/model-zebra-less-kinds       300 "$EIN" solve "$OUT/ein/model-zebra-less-kinds.ein"
run ein/model-zebra-less-both        300 "$EIN" solve "$OUT/ein/model-zebra-less-both.ein"
run ein/hand-zebra-step2-single      300 "$EIN" solve einaf/contract/zebra/step2.ein --single
run ein/hand-zebra-step3-single      300 "$EIN" solve einaf/contract/zebra/step3.ein --single
run ein/zebra3.1-single              300 "$EIN" solve examples/zebra3.1.ein --single
run ein/model-bob1-single            300 "$EIN" solve $FT/library/bob1/program.ein --single
run ein/model-bystander-single       300 "$EIN" solve $FT/module-dir/bystander/program.ein --single
# H, as Ein facts: the model's steps against the hand's (the model's step programs are the replay's record)
if slow; then
  run ein/show-hyps-model-zebra-step1 300 "$EIN" show hyps "$OUT/from-text/zebra/record/steps/step1.ein"
  run ein/show-hyps-model-zebra-step2 300 "$EIN" show hyps "$OUT/from-text/zebra/record/steps/step2.ein"
fi
run ein/show-hyps-model-zebra        300 "$EIN" show hyps $Z
run ein/show-hyps-hand-zebra-step1   300 "$EIN" show hyps einaf/contract/zebra/step1.ein
run ein/show-hyps-hand-zebra-step2   300 "$EIN" show hyps einaf/contract/zebra/step2.ein
run ein/show-hyps-model-bystander    300 "$EIN" show hyps $FT/module-dir/bystander/program.ein
for f in "$OUT"/ein/show-hyps-*.out; do  # the first lines and the count, beside an output too large to commit
  { echo "$(wc -l <"$f") lines; the first 12:"; head -12 "$f"; } >"${f%.out}.head.txt"
done

# ── 8. Renderings: `ein render program`, drawn by Graphviz ───────────────────────────────────────────────────
draw model-zebra        $Z
draw hand-zebra-step1   einaf/contract/zebra/step1.ein
draw hand-zebra-step2   einaf/contract/zebra/step2.ein
draw zebra3.1           examples/zebra3.1.ein
draw model-bystander    $FT/module-dir/bystander/program.ein
draw model-bob1         $FT/library/bob1/program.ein
fi

# ── 9. With the model: one live run of bob1, as S2.2.5's run.sh made it ──────────────────────────────────────
if [ -n "$MODEL" ]; then
  if command -v nvidia-smi >/dev/null; then
    nvidia-smi --query-gpu=name,memory.used,memory.total --format=csv >"$OUT/live-nvidia-smi.txt" 2>&1
  fi
  run live/model 300 "$EINAF" model --model "$MODEL"
  rm -rf "$OUT/live/bob1"
  run live/bob1 900 "$EINAF" from-text $TX/bob1.txt --id bob1 --answer-line word --record "$OUT/live/bob1" \
      --gold-answer 3 --model "$MODEL"
  {
    cmp -s "$OUT/live/bob1/program.ein" $FT/library/bob1/program.ein \
      && echo "program.ein: the committed one ($FT/library/bob1/program.ein), byte for byte" \
      || echo "program.ein: DIFFERS from $FT/library/bob1/program.ein"
    # each call's generated text against the committed record's, by the tokens' digest
    python3 -I - "$OUT/live/bob1/generations.jsonl" $FT/library/bob1/generations.jsonl <<'PY'
import json, sys
def digests(path):
    out = []
    for line in open(path):
        call = json.loads(line)
        gen = call.get("generation") or {}
        env = gen.get("envelope") or {}
        g = env.get("generation") or {}
        out.append(g.get("tokens_sha256") or json.dumps(g, sort_keys=True)[:80])
    return out
live, rec = digests(sys.argv[1]), digests(sys.argv[2])
same = sum(a == b for a, b in zip(live, rec))
req = lambda path: [json.loads(l)["digest"] for l in open(path)]
same_req = sum(a == b for a, b in zip(req(sys.argv[1]), req(sys.argv[2])))
print(f"calls: {len(live)} live, {len(rec)} recorded; the request alike on {same_req}, the generated tokens alike on {same}")
PY
  } >"$OUT/live/bob1-against-the-record.txt" 2>&1
  if command -v nvidia-smi >/dev/null; then  # after: no model process left, the device's memory given back
    { nvidia-smi --query-gpu=name,memory.used,memory.total --format=csv
      nvidia-smi --query-compute-apps=pid,process_name,used_memory --format=csv; } >"$OUT/live-nvidia-smi-after.txt" 2>&1
  fi
fi

# ── What is not committed: out/.gitignore ────────────────────────────────────────────────────────────────────
{
  echo "# Written by make_report.sh: what it writes again and git does not keep. Every DOT (its SVG is kept); each"
  echo "# command's .cmd, .exit and .time (INDEX.tsv holds all three); a model record or a run's JSON a replay"
  echo "# writes again; and any file over 200 KB (an SVG over 250 KB)."
  echo "*.dot"
  echo "*.cmd"
  echo "*.exit"
  echo "*.time"
  echo "*/*/record/*"
  echo "!*/*/record/steps/"
  echo "!*/*/record/answer.json"
  echo "live/bob1/*"
  echo "!live/bob1/program.ein"
  echo "!live/bob1/steps/"
  echo "!live/bob1/answer.json"
  echo "render/*/render-program.out"
  echo "steps/bob1/calls.jsonl"
  echo "steps/bob1/run.json"
  (cd "$OUT" && find . -type f ! -name '*.svg' ! -name '*.dot' -size +200k -printf '%P\n' | sort)
  (cd "$OUT" && find . -type f -name '*.svg' -size +250k -printf '%P\n' | sort)
} >"$OUT/.gitignore"
find "$OUT" -type f -empty -delete
echo "make_report.sh: done, $(wc -l <"$OUT/INDEX.tsv") commands; out/INDEX.tsv" >&2
