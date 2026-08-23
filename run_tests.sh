#!/usr/bin/env bash
#
# run_tests.sh — the gate.
#
#     ./run_tests.sh                 # cargo test --workspace
#     ./run_tests.sh --slow          # + the 12 slow corpus cells, + 8 id seeds
#     ./run_tests.sh -p ein-ir       # anything else is forwarded to cargo test
#
# **This was a three-phase runner until M1a S1a.10.5**, and each phase went
# for a different reason. It is kept as a name rather than as a script: the
# name is in `AGENTS.md`, in a dozen plan documents' "Gate:" lines and in the
# user's habits, and a wrapper that still works is cheaper than re-teaching
# the habit.
#
#   Phase 1  the pytest unit/integration suite (`ein.py/tests`). Its 1 538
#            tests reduce to **275 behaviours** in fifteen Rust files —
#            plans/m1a_rust/p1a.10_single_implementation/suite_dispositions.md
#            has the file-by-file record, including the 96 subjects that died
#            with their code. Ported at S1a.10.2.
#   Phase 2  the P1.7a acceptance gate (`ein.py/acceptance`) — the three
#            zebra2 task-class fixtures, ~40 s, deliberately outside the
#            pytest testpaths because it was slow. All 21 tests are
#            `ein-infer/tests/acceptance.rs` (16) and
#            `ein-cli/tests/acceptance_cli.rs` (5) now, where the same work
#            takes 0.26 s. It was a separate phase because it was slow; it is
#            not slow any more, so the runner lost a phase rather than a
#            check.
#   Phase 3  `cargo test --workspace` — which is all of it.
#
# The interpreter selection (PyPy venv, then .venv, then python3, `EIN_PY` to
# override) went with the engine that needed it, along with `-j`, `--fast`,
# `--acceptance-only` and `--no-rust`. There is nothing left for them to
# select between.
#
# **The gate needs Graphviz on `PATH`**: `ein-render/tests/dot_wellformed.rs`
# is the only authority the DOT views have on being well-formed, and since
# S1a.10.3 it *fails* rather than skips without it — a skip went to a stderr
# line `cargo test` captures for a passing test, so CI had been reporting a
# pass over 5 209 renderings nothing checked.
#
# Budget: **616 tests in ~1 m 51 s** (re-measured 2026-08-23, S1a.9.3, warm
# build, twice within 0.3 s). It was 577 in ~1 m 16 s at S1a.9.0 the day
# before, and the 35 s between them is not the eleven tests S1a.9.3 added:
# P1a.7 closed in between and put `jobs_invariance` — 20 712 (file, op, jobs)
# cells — into the default gate. It was 312 in 9 m 13 s before S1a.10.2, of
# which nine minutes were 42 integration tests starting a `python3` per corpus
# file. No test is marked slow; four targets are 78 % of the wall clock —
# `dot_wellformed` 51 s, `jobs_invariance` 12.5 s, `search_invariant` 12.4 s,
# `id_order_invariance` 11.5 s.
#
#   EIN_BLESS=1 ./run_tests.sh          # re-bank every golden
#   EIN_CORPUS_SLOW=1 ./run_tests.sh    # the 2 slow corpus entries, 12 cells
#   EIN_ID_SEEDS=8    ./run_tests.sh    # more id-space permutations

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
MANIFEST="${SCRIPT_DIR}/ein.rs/Cargo.toml"

if [[ "${1:-}" == "-h" || "${1:-}" == "--help" ]]; then
    sed -n '3,47p' "${BASH_SOURCE[0]}" | sed 's/^# \{0,1\}//'
    exit 0
fi

if ! command -v cargo >/dev/null 2>&1; then
    # Loud, and not a pass: a skipped gate that reads as green is how a gate
    # stops being one.
    echo "error: no cargo on PATH — the gate did not run." >&2
    exit 127
fi
if ! command -v dot >/dev/null 2>&1; then
    echo "error: no Graphviz 'dot' on PATH — dot_wellformed.rs would fail." >&2
    echo "       Install graphviz; it is a hard dependency of the gate." >&2
    exit 127
fi

# `--slow` is the nightly tier in one flag: the 12 slow corpus cells and
# eight id-space permutations instead of one. It read 118 until S1a.9.0
# re-priced the tier and 19 until T1a.7.2.0 took `branching/07` out of it —
# `corpus/corpus.toml` is the authority and `cost_ms` is the measurement.
ARGS=()
for arg in "$@"; do
    case "${arg}" in
        --slow) export EIN_CORPUS_SLOW=1 EIN_ID_SEEDS="${EIN_ID_SEEDS:-8}" ;;
        *)      ARGS+=( "${arg}" ) ;;
    esac
done

echo "── cargo test --workspace ─────────────────────────────────────" >&2
exec cargo test --manifest-path "${MANIFEST}" --workspace \
     ${ARGS[@]+"${ARGS[@]}"}
