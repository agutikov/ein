#!/usr/bin/env bash
# P2.1's report material, written from the binaries: every program of the
# report under `ein` (read-out, summary, answer dump, renderings) and under
# `einaf` (outcome per question, feedback object). Re-run it to see the
# phase's behaviour on the current checkout:
#
#   ./build.sh --engine --einaf
#   plans/m2_nl_to_ir/p2.1_kernel_as_instrumentation/report/make_report.sh
#
# EIN_BIN and EINAF_BIN name the binaries (default: the release builds).
set -u
ROOT=$(git -C "$(dirname "$0")" rev-parse --show-toplevel)
OUT="$ROOT/plans/m2_nl_to_ir/p2.1_kernel_as_instrumentation/report/programs"
EIN=${EIN_BIN:-$ROOT/ein.rs/target/release/ein}
EINAF=${EINAF_BIN:-$ROOT/einaf/target/release/einaf}
cd "$ROOT"

# name | path | outcome it shows | draw the oracle's region (small cubes only)
PROGRAMS=(
  "one|examples/zebra2.ein|one|no"
  "several|examples/lattice/03_state_hash_collision.ein|several (one over the maximal line)|yes"
  "incomplete|tests/region/04_owing.ein|incomplete|yes"
  "inconsistent|examples/ein-bugs/zebra2-bad.ein|inconsistent|no"
  "flagged|examples/lattice/04_obligation_trigger_hole.ein|flagged|no"
  "invalid-parse|examples/broken/unclosed_paren.ein|invalid (parse)|no"
  "invalid-declaration|examples/broken/load/rule_duplicate_name.ein|invalid (declaration)|no"
  "invalid-macro-call|examples/broken/load/macro_arity_mismatch.ein|invalid (macro-call)|no"
  "invalid-derivation-cycle|examples/broken/load/derivation_cycle.ein|invalid (derivation-cycle)|no"
  "invalid-mechanisms|examples/broken/load/no_hypothesis_beside_open.ein|invalid (mechanisms)|no"
  "not-decided-grow|ein.rs/crates/ein-cli/tests/bounded/grow.ein|not decided (no fixpoint)|no"
)

# Run a command, its stdout to $1.out, stderr to $1.err, exit code to $1.exit.
run() {
  local base=$1; shift
  timeout 300 "$@" >"$base.out" 2>"$base.err"
  echo $? >"$base.exit"
}

# DOT on stdin of $1 to SVG beside it (one SVG a graph: x.dot.svg, x.dot.2.svg, …).
svg() { [ -s "$1" ] && dot -Tsvg -O "$1" 2>"$1.dot-err" || true; [ -s "$1.dot-err" ] || rm -f "$1.dot-err"; }

rm -rf "$OUT"; mkdir -p "$OUT"
for entry in "${PROGRAMS[@]}"; do
  IFS='|' read -r name path shows region <<<"$entry"
  d="$OUT/$name"; mkdir -p "$d"
  echo "$path — $shows" >"$d/PROGRAM"

  if [ "$name" = not-decided-grow ]; then
    # No fixpoint: every question needs a bound. The clock, the cap, and the
    # saturator's own cap under `ein saturate`.
    run "$d/solve-T1" "$EIN" solve "$path" -T 1 --max-steps 1000000000 --json-summary "$d/solve-T1.summary.json"
    run "$d/solve-default" "$EIN" solve "$path" --json-summary "$d/solve-default.summary.json"
    run "$d/solve-T0" "$EIN" solve "$path" -T 0 --json-summary "$d/solve-T0.summary.json"
    run "$d/solve-max-steps" "$EIN" solve "$path" --max-steps 10000 --json-summary "$d/solve-max-steps.summary.json"
    run "$d/single-max-steps" "$EIN" solve "$path" --single --max-steps 10000 --json-summary "$d/single-max-steps.summary.json"
    run "$d/saturate-max-steps" "$EIN" saturate "$path" --max-steps 200
  else
    run "$d/solve" "$EIN" solve "$path" --json-summary "$d/solve.summary.json" --dump-answer "$d/answer"
    run "$d/single" "$EIN" solve "$path" --single --json-summary "$d/single.summary.json"
    run "$d/no-search" "$EIN" solve "$path" --no-search --json-summary "$d/no-search.summary.json"
    run "$d/count" "$EIN" solve "$path" --count
    if grep -q ':expect' "$path" || [[ $name == invalid-* ]]; then
      run "$d/test" "$EIN" test "$path" --json-report "$d/test.report.json"
    fi
  fi

  case "$name" in invalid-*|not-decided-*) ;; *)
    run "$d/render-program" "$EIN" render program "$path"; cp "$d/render-program.out" "$d/program.dot"; svg "$d/program.dot"
    run "$d/render-kb" "$EIN" render kb "$path" --fold; cp "$d/render-kb.out" "$d/kb.dot"; svg "$d/kb.dot"
    run "$d/render-lattice" "$EIN" render lattice "$path"; cp "$d/render-lattice.out" "$d/lattice.dot"; svg "$d/lattice.dot"
    run "$d/show-hyps" "$EIN" show hyps "$path"
    run "$d/h-space" "$EIN" render h-space "$path" --dump "$d/h-space"
    for g in "$d"/h-space/*.dot; do [ -e "$g" ] && svg "$g"; done
    if [ "$region" = yes ]; then
      run "$d/oracle" "$EIN" oracle "$path" --dump "$d/oracle"
      run "$d/render-region" "$EIN" render region "$d/oracle" --answer "$d/answer"
      cp "$d/render-region.out" "$d/region.dot"; svg "$d/region.dot"
    fi
  ;; esac

  # EinAf: the outcome under every question, and the feedback object.
  if [ -x "$EINAF" ]; then
    for q in "" "--single" "--maximal --single" "--minimal --single" "--maximal" "--minimal" "--count" "--no-search"; do
      tag=${q//--/}; tag=${tag// /-}; tag=${tag:-describe}
      extra=(); [ "$name" = not-decided-grow ] && extra=(--max-steps 10000)
      run "$d/einaf-outcome-$tag" "$EINAF" outcome "$path" $q "${extra[@]}"
    done
    if "$EINAF" 2>&1 | grep -q 'einaf feedback'; then
      extra=(); [ "$name" = not-decided-grow ] && extra=(--max-steps 10000)
      run "$d/einaf-feedback-single" "$EINAF" feedback "$path" --single "${extra[@]}"
      run "$d/einaf-feedback-describe" "$EINAF" feedback "$path" "${extra[@]}"
    fi
  fi
  find "$d" -type f -empty -delete
  echo "$name: done"
done
