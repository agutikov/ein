#!/usr/bin/env bash
#
# run_tests.sh — the gate.
#
#     ./run_tests.sh                 # the five static checks, then the tests
#     ./run_tests.sh --slow          # + the 12 slow corpus cells, + 8 id seeds
#     ./run_tests.sh --tests-only    # skip the static checks
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
#            docs/history/m1a_rust/suite_dispositions.md
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
# Budget: **619 tests in ~1 m 51 s** (re-measured 2026-08-23, S1a.9.3/.9.4, warm
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
#
# **This runs what CI runs** — every step of the per-commit tier
# (`.github/workflows/per-commit.yml`), in its order, so that a green
# `./run_tests.sh` means a green CI. That was **not** true until M1c S1c.1.5,
# and it cost three red commits: CI ran `cargo clippy -D warnings` and two
# Python greps that nothing local ran, so this script reported a pass over two
# findings it could not see. A local gate that is a subset of the remote one
# is a local gate that lies.
#
#   stdlib_manifest.py            the embedded stdlib against its digest
#   check_hashmap_iteration.py    no hash-map iteration at an observable site
#   cargo fmt --all --check       three files were unformatted the first time
#                                 it ran, all three from M1c's `:expect` work
#   cargo clippy -D warnings      a `for i in 0..n` indexing a slice, and four
#                                 `&file` where `file` was already a `&str` —
#                                 latent, because clippy stopped at the first
#                                 crate that failed and never reached `ein-cli`
#   cargo doc -D warnings         twelve **unresolved** intra-doc links and
#                                 seven public items whose docs linked to a
#                                 private one. Local-only until S1c.1.5 put it
#                                 in the workflow too
#
# They cost about a second each warm, they run before the tests because their
# failures are the cheapest ones to read, and `--tests-only` turns all five
# off for a targeted iteration. Nothing turns them off silently — a missing
# `rustfmt`, `clippy` or `python3` is exit 127 here, the same as a missing
# `cargo` or `dot`.

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
MANIFEST="${SCRIPT_DIR}/ein.rs/Cargo.toml"

if [[ "${1:-}" == "-h" || "${1:-}" == "--help" ]]; then
    # The header, to its own end — a line number here goes stale the first
    # time somebody adds a paragraph, and one had: `3,47p` stopped mid-sentence.
    awk 'NR>2 && !/^#/ {exit} NR>2 {sub(/^# ?/, ""); print}' "${BASH_SOURCE[0]}"
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
CHECKS=1
for arg in "$@"; do
    case "${arg}" in
        --slow)       export EIN_CORPUS_SLOW=1 EIN_ID_SEEDS="${EIN_ID_SEEDS:-8}" ;;
        --tests-only) CHECKS=0 ;;
        *)            ARGS+=( "${arg}" ) ;;
    esac
done

# One banner per step, ruled to a fixed width so the column of them reads as
# a list rather than as ragged output.
step() {
    local label="$*" rule=""
    local width=$(( 62 - ${#label} ))
    while (( ${#rule} < width )); do rule="${rule}─"; done
    echo "── ${label} ${rule}" >&2
}

if [[ "${CHECKS}" == 1 ]]; then
    for tool in python3 rustfmt; do
        if ! command -v "${tool}" >/dev/null 2>&1; then
            echo "error: no ${tool} on PATH — the static checks did not run." >&2
            echo "       Install it, or pass --tests-only." >&2
            exit 127
        fi
    done
    if ! cargo clippy --version >/dev/null 2>&1; then
        echo "error: no clippy — 'rustup component add clippy', or --tests-only." >&2
        exit 127
    fi

    step "utils/stdlib_manifest.py"
    python3 "${SCRIPT_DIR}/utils/stdlib_manifest.py"

    step "utils/check_hashmap_iteration.py"
    python3 "${SCRIPT_DIR}/utils/check_hashmap_iteration.py"

    step "cargo fmt --all --check"
    if ! cargo fmt --manifest-path "${MANIFEST}" --all --check; then
        echo >&2
        echo "error: formatting drift above — the diff is what rustfmt would do." >&2
        echo "       Fix with: cargo fmt --manifest-path ein.rs/Cargo.toml --all" >&2
        exit 1
    fi

    # `--all-targets`, so the lints see tests and benches too. Four of the six
    # findings S1c.1.5 fixed were in `ein-cli`, which clippy had never reached
    # because it stops at the first crate that fails to compile.
    step "cargo clippy --workspace --all-targets -D warnings"
    cargo clippy --manifest-path "${MANIFEST}" --workspace --all-targets -- -D warnings

    # `--no-deps` on purpose: what is being checked is *this* workspace's docs,
    # and a dependency's rustdoc warnings are not ours to fix. `-D warnings`
    # rather than `-D rustdoc::broken_intra_doc_links` alone, because the other
    # lints in that family are the same kind of rot — a private item linked
    # from a public one, an `<path>` read as an HTML tag, a link whose explicit
    # target repeats its label.
    step "cargo doc --no-deps, -D warnings"
    if ! RUSTDOCFLAGS="-D warnings" cargo doc -q --manifest-path "${MANIFEST}" \
            --workspace --no-deps; then
        echo >&2
        echo "error: rustdoc above — every link in a public doc comment must resolve." >&2
        echo "       A reference that cannot be a link is still fine as \`code\`." >&2
        exit 1
    fi
fi

step "cargo test --workspace"
cargo test --manifest-path "${MANIFEST}" --workspace ${ARGS[@]+"${ARGS[@]}"}

# CI's last step, and it is not a measurement: `--test` runs each bench once
# to see that it runs. A bench that stopped compiling is invisible to
# `cargo test`, and `ein-corpus`'s bench set is the only consumer of several
# `pub` items in the engine.
if [[ "${CHECKS}" == 1 ]]; then
    step "cargo bench --bench engine -- --test"
    cargo bench -q --manifest-path "${MANIFEST}" --bench engine -- --test
fi
