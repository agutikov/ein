#!/usr/bin/env bash
#
# run_tests.sh — the gate.
#
#     ./run_tests.sh                 # six static checks, the tests, their scratch check, the review-bundle smoke, the bench smoke
#     ./run_tests.sh --slow          # the same, every test at its slow depth (EIN_CORPUS_SLOW=1) and 8 id seeds
#     ./run_tests.sh --nightly       # instead of the gate: every step of .github/workflows/nightly.yml, in order
#     ./run_tests.sh --dev-host      # instead of the gate: the nightly's steps at full budget, and the steps CI no longer runs
#     ./run_tests.sh --tests-only    # cargo test, its scratch check and the review-bundle smoke — no static checks, no bench smoke
#     ./run_tests.sh -p ein-ir       # anything else is forwarded to cargo test
#
# **The gate needs Graphviz on `PATH`**: `ein-render/tests/dot_wellformed.rs`
# is the only authority the DOT views have on being well-formed, and since
# S1a.10.3 it *fails* rather than skips without it — a skip went to a stderr
# line `cargo test` captures for a passing test, so CI had been reporting a
# pass over 5 209 renderings nothing checked. And `fdp` with its layout plugin
# (M1f S1f.5.29): the review-bundle smoke lays every KB view out with it.
#
# Budget: **619 tests in ~1 m 51 s** (re-measured 2026-08-23, S1a.9.3/.9.4, warm
# build, twice within 0.3 s). It was 577 in ~1 m 16 s at S1a.9.0 the day
# before, and the 35 s between them is not the eleven tests S1a.9.3 added:
# P1a.7 closed in between and put `jobs_invariance` — 20 712 (file, op, jobs)
# cells — into the default gate. No test is marked slow; four targets are 78 % of the wall clock —
# `dot_wellformed` 51 s, `jobs_invariance` 12.5 s, `search_invariant` 12.4 s,
# `id_order_invariance` 11.5 s.
#
#   EIN_BLESS=1 ./run_tests.sh          # re-bank every golden
#   EIN_CORPUS_SLOW=1 ./run_tests.sh    # the slow entries' cells, and every test's slow half
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
# **Since M1e S1e.3.4 a test says so** — `ein-cli/tests/gate_steps.rs`, which
# reads both files and diffs the commands. What it takes to be a step here is
# a `step "…"` banner: the command it announces is the next statement, so
# adding a step to this script is adding it to the list. On the workflow side
# it is a `# gate-step` marker, and an unmarked `run:` the test does not
# recognise fails it — the drift that matters is CI gaining a step this script
# does not have.
#
#   stdlib_manifest.py            the embedded stdlib against its digest
#   check_hashmap_iteration.py    no hash-map iteration at an observable site
#   doc_audit.py --links --check  **264** dangling links and nonexistent
#                                 anchors the first time it ran over the whole
#                                 tree rather than over `docs/kernel/` — 251 of
#                                 them in one milestone record that had moved
#                                 out of `plans/` (M1e S1e.3.8, `DO-M2`)
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
# failures are the cheapest ones to read, and `--tests-only` turns all six off
# — and the bench smoke below with them, which is what
# `gate_steps::what_tests_only_skips_is_what_the_script_guards` reads out of
# this script rather than out of this sentence. Nothing turns them off
# silently — a missing `rustfmt`, `clippy` or `python3` is exit 127 here, the
# same as a missing `cargo` or `dot`.
#
# **`--nightly` runs the nightly instead of the gate** (M1e2 S1e2.3.6): every
# step of `.github/workflows/nightly.yml`, its three jobs one after another in
# the file's order (`deep-corpus`, the fuzzer once per seed of its matrix, the
# bench smoke), from the repo root, so that the nightly can be run before a
# push. Nothing local ran those steps before, and the nightly failed on every
# run from 2026-10-17 to 2026-11-03 while this gate stayed green. It takes no
# other argument and runs none of the gate's steps. A step is the statement
# after a `nightly_step "…"` banner, and `ein-cli/tests/nightly_steps.rs`
# diffs that list against the workflow's, as `gate_steps.rs` does the gate's.
# Its test steps run under their own `build/gate-tmp-<pid>`, checked as the
# gate's is. **What the nightly holds is a rule** (M1f S1f.5.27, the user's
# ruling of 2026-11-10, Q-M1f.102): a step that guards an answer or a contract
# no per-commit step guards, each at a budget that fits in 5 min on the
# runner. The fuzz job is in it at 20 000 inputs a seed (Q-M1f.103), where it
# was a million: both seeds and the bench smoke took 15 s together in
# S1e2.3.6's sweep of 2026-11-03, which is why this paragraph called them
# cheap, and since S1f.5.5's parser swap a million took up to 4 410 s a seed
# on the runner (run 42802597417, 2026-11-10). The bench smoke is still cheap.
#
# **`--dev-host` runs the dev host's tier instead of the gate** (M1f
# S1f.5.27): every step of the nightly, at full budget (the fuzzer at P1a.1's
# million inputs a seed), and the steps that left the nightly or never ran
# in it, each over 5 min on the runner or a measurement: the id space
# permuted eight ways, `route_invariance`'s slow half, `stdlib_census.py
# --check`, and the whole review bundle with its link check. Nothing
# schedules it; the user runs it, here or in the runner's image
# (`utils/ci_container.sh --dev-host`). It takes no other argument, runs from
# the repo root, and its test steps run under a checked
# `build/gate-tmp-<pid>`, as `--nightly`'s do. A step is the statement after
# a `dev_host_step "…"` banner, and `nightly_steps.rs` holds the list to the
# nightly's: every nightly step is in it, in order, at the dev host's
# budget, and every other step is one its `DEV_HOST_ONLY` names with why, so
# a step that leaves the nightly and is not named there fails a test.
# One thing differs from a fresh checkout: the bundle step writes
# `build/review/` over what an earlier run left there, case by case, and the
# link check after it reads the whole directory, so a gallery that
# `review_bundle.py --regions` left in `build/review/regions/` is checked too.
#
# **Since M1e2 S1e2.3.5 the gate checks its own scratch.** `cargo test` runs
# with `TMPDIR` set to `build/gate-tmp-<pid>`, fresh and empty, and
# `utils/check_test_scratch.py` fails the gate on anything a test left there,
# naming the file that writes each entry. Nothing had looked: 1 098
# `/tmp/ein-*` directories had piled up by 2026-11-03, eleven more per run, in
# a 16 GB tmpfs that test scratch had filled once. `build/` is gitignored and
# on disk. A test makes its scratch with `ein_corpus::Scratch`, which removes
# itself on drop. The directory is removed on every exit, and after a red
# `cargo test` it is not checked: a failing test may leave its scratch, and the
# failure is what to read. `--tests-only` keeps the check, which is not a
# static one.

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
# `fdp` too, and laying a graph out, not only on `PATH` (M1f S1f.5.29): every
# KB view suggests it, and the review bundle's smoke lays those out with it.
# Ubuntu 26.04's `graphviz` links `fdp` to `dot` and installs no plugin behind
# it, so `fdp` is found and refuses every graph; the smoke was red on the
# runner from `00c4ed34` with a green gate here.
if ! echo 'graph { a }' | fdp -Tsvg >/dev/null 2>&1; then
    echo "error: Graphviz's 'fdp' lays out no graph — install its layout plugin (libgvplugin-neato-layout8 on Ubuntu 26.04)." >&2
    exit 127
fi

# `--slow` is the gate with the two depths the nightly's and the dev host's
# test steps set, for every test at once: `EIN_CORPUS_SLOW=1` (the slow
# entries' cells, and the slow half of every test that has one) and eight
# id-space permutations instead of one. It is neither of those tiers, which
# run some test targets at those depths and scripts besides: `--nightly` and
# `--dev-host` are. `corpus/corpus.toml` is the
# authority on what is slow, with `cost_ms` the measurement, and since M1f
# S1f.5.13 on which entries' `--dump-answer` the round trip leaves to this
# tier, `heavy_dump`.
ARGS=()
CHECKS=1
NIGHTLY=0
DEV_HOST=0
for arg in "$@"; do
    case "${arg}" in
        --slow)       export EIN_CORPUS_SLOW=1 EIN_ID_SEEDS="${EIN_ID_SEEDS:-8}" ;;
        --tests-only) CHECKS=0 ;;
        --nightly)    NIGHTLY=1 ;;
        --dev-host)   DEV_HOST=1 ;;
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

# `--nightly` (the header's paragraph): nightly.yml's steps and nothing else,
# then exit. `nightly_step` is defined on one line on purpose:
# `gate_steps.rs` reads every line that *starts* `step "` as one of the gate's
# banners, and a body line `step "$@"` would be one.
nightly_step() { step "$@"; }

if [[ "${NIGHTLY}" == 1 ]]; then
    if [[ $# -ne 1 ]]; then
        echo "error: --nightly takes no other argument: it runs nightly.yml's steps as written." >&2
        exit 2
    fi
    if ! command -v python3 >/dev/null 2>&1; then
        echo "error: no python3 on PATH — the nightly's scripts did not run." >&2
        exit 127
    fi
    # The workflow runs its scripts from the checkout's root, and
    # `doc_audit.py` reads `build/review` relative to where it runs.
    cd "${SCRIPT_DIR}"
    GATE_TMP="${SCRIPT_DIR}/build/gate-tmp-$$"
    rm -rf "${GATE_TMP}"
    mkdir -p "${GATE_TMP}"
    trap 'rm -rf "${GATE_TMP}"' EXIT

    # ── deep-corpus ──
    nightly_step "deep-corpus: corpus_cli, the slow entries"
    EIN_CORPUS_SLOW="1" TMPDIR="${GATE_TMP}" cargo test --manifest-path "${MANIFEST}" -p ein-cli --release --test corpus_cli

    nightly_step "deep-corpus: answer_dump"
    EIN_CORPUS_SLOW="1" TMPDIR="${GATE_TMP}" cargo test --manifest-path "${MANIFEST}" -p ein-cli --release --test answer_dump

    nightly_step "deep-corpus: answer_round_trip, the slow and heavy_dump entries"
    EIN_CORPUS_SLOW="1" TMPDIR="${GATE_TMP}" cargo test --manifest-path "${MANIFEST}" -p ein-cli --release --test answer_round_trip

    nightly_step "deep-corpus: lever_invariance, the slow cells"
    EIN_CORPUS_SLOW="1" TMPDIR="${GATE_TMP}" cargo test --manifest-path "${MANIFEST}" -p ein-cli --release --test lever_invariance

    nightly_step "deep-corpus: corpus_cores, the slow entries"
    EIN_CORPUS_SLOW="1" TMPDIR="${GATE_TMP}" cargo test --manifest-path "${MANIFEST}" -p ein-infer --release --test corpus_cores

    nightly_step "deep-corpus: region, the slow half"
    EIN_CORPUS_SLOW="1" TMPDIR="${GATE_TMP}" cargo test --manifest-path "${MANIFEST}" -p ein-infer --release --test region

    nightly_step "deep-corpus: obligation_rung, the slow half"
    EIN_CORPUS_SLOW="1" TMPDIR="${GATE_TMP}" cargo test --manifest-path "${MANIFEST}" -p ein-infer --release --test obligation_rung

    nightly_step "deep-corpus: utils/check_test_scratch.py"
    python3 "${SCRIPT_DIR}/utils/check_test_scratch.py" "${GATE_TMP}"

    nightly_step "deep-corpus: utils/stdlib_mutants.py --check"
    python3 "${SCRIPT_DIR}/utils/stdlib_mutants.py" --check

    # ── frontend-fuzz, once per seed of its matrix, at the nightly's budget ──
    nightly_step "frontend-fuzz: seed 1"
    EIN_FUZZ_ITERS="20000" EIN_FUZZ_SEED="1" cargo test --manifest-path "${MANIFEST}" -p ein-ir --release --test fuzz_properties

    nightly_step "frontend-fuzz: seed 987654321"
    EIN_FUZZ_ITERS="20000" EIN_FUZZ_SEED="987654321" cargo test --manifest-path "${MANIFEST}" -p ein-ir --release --test fuzz_properties

    # ── benches ──
    nightly_step "benches: cargo bench --bench engine -- --test"
    cargo bench --manifest-path "${MANIFEST}" --bench engine -- --test

    exit 0
fi

# `--dev-host` (the header's paragraph): the nightly's steps at full budget,
# in the nightly's order, with the steps that left it where they ran, and the
# ones that never ran in it beside its other slow halves; then exit.
# `dev_host_step` is one line for `nightly_step`'s reason.
dev_host_step() { step "$@"; }

if [[ "${DEV_HOST}" == 1 ]]; then
    if [[ $# -ne 1 ]]; then
        echo "error: --dev-host takes no other argument: it runs the dev host's steps as written." >&2
        exit 2
    fi
    if ! command -v python3 >/dev/null 2>&1; then
        echo "error: no python3 on PATH — the dev host's scripts did not run." >&2
        exit 127
    fi
    cd "${SCRIPT_DIR}"
    GATE_TMP="${SCRIPT_DIR}/build/gate-tmp-$$"
    rm -rf "${GATE_TMP}"
    mkdir -p "${GATE_TMP}"
    trap 'rm -rf "${GATE_TMP}"' EXIT

    # ── deep-corpus, and the slow steps that left it ──
    dev_host_step "deep-corpus: corpus_cli, the slow entries"
    EIN_CORPUS_SLOW="1" TMPDIR="${GATE_TMP}" cargo test --manifest-path "${MANIFEST}" -p ein-cli --release --test corpus_cli

    dev_host_step "dev host: id_order_invariance, eight seeds"
    EIN_ID_SEEDS="8" TMPDIR="${GATE_TMP}" cargo test --manifest-path "${MANIFEST}" -p ein-render --release --test id_order_invariance

    dev_host_step "deep-corpus: answer_dump"
    EIN_CORPUS_SLOW="1" TMPDIR="${GATE_TMP}" cargo test --manifest-path "${MANIFEST}" -p ein-cli --release --test answer_dump

    dev_host_step "deep-corpus: answer_round_trip, the slow and heavy_dump entries"
    EIN_CORPUS_SLOW="1" TMPDIR="${GATE_TMP}" cargo test --manifest-path "${MANIFEST}" -p ein-cli --release --test answer_round_trip

    dev_host_step "deep-corpus: lever_invariance, the slow cells"
    EIN_CORPUS_SLOW="1" TMPDIR="${GATE_TMP}" cargo test --manifest-path "${MANIFEST}" -p ein-cli --release --test lever_invariance

    dev_host_step "deep-corpus: corpus_cores, the slow entries"
    EIN_CORPUS_SLOW="1" TMPDIR="${GATE_TMP}" cargo test --manifest-path "${MANIFEST}" -p ein-infer --release --test corpus_cores

    dev_host_step "deep-corpus: region, the slow half"
    EIN_CORPUS_SLOW="1" TMPDIR="${GATE_TMP}" cargo test --manifest-path "${MANIFEST}" -p ein-infer --release --test region

    dev_host_step "deep-corpus: obligation_rung, the slow half"
    EIN_CORPUS_SLOW="1" TMPDIR="${GATE_TMP}" cargo test --manifest-path "${MANIFEST}" -p ein-infer --release --test obligation_rung

    dev_host_step "dev host: route_invariance, the slow half"
    EIN_CORPUS_SLOW="1" TMPDIR="${GATE_TMP}" cargo test --manifest-path "${MANIFEST}" -p ein-cli --release --test route_invariance

    dev_host_step "deep-corpus: utils/check_test_scratch.py"
    python3 "${SCRIPT_DIR}/utils/check_test_scratch.py" "${GATE_TMP}"

    dev_host_step "dev host: utils/stdlib_census.py --check"
    python3 "${SCRIPT_DIR}/utils/stdlib_census.py" --check

    # The whole review bundle, every register case, then every link and image
    # in it: two steps, where M1e2 S1e2.1.6 had one `a && b`, because `set -e`
    # does not stop on the left of an `&&` (S1e2.3.6).
    dev_host_step "dev host: utils/review_bundle.py"
    python3 "${SCRIPT_DIR}/utils/review_bundle.py"

    dev_host_step "dev host: utils/doc_audit.py --links --check build/review"
    python3 "${SCRIPT_DIR}/utils/doc_audit.py" --links --check build/review

    dev_host_step "deep-corpus: utils/stdlib_mutants.py --check"
    python3 "${SCRIPT_DIR}/utils/stdlib_mutants.py" --check

    # ── frontend-fuzz, once per seed of the nightly's matrix, at P1a.1's budget ──
    dev_host_step "frontend-fuzz: seed 1, a million inputs"
    EIN_FUZZ_ITERS="1000000" EIN_FUZZ_SEED="1" cargo test --manifest-path "${MANIFEST}" -p ein-ir --release --test fuzz_properties

    dev_host_step "frontend-fuzz: seed 987654321, a million inputs"
    EIN_FUZZ_ITERS="1000000" EIN_FUZZ_SEED="987654321" cargo test --manifest-path "${MANIFEST}" -p ein-ir --release --test fuzz_properties

    # ── benches ──
    dev_host_step "benches: cargo bench --bench engine -- --test"
    cargo bench --manifest-path "${MANIFEST}" --bench engine -- --test

    exit 0
fi

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

    # M1e S1e.3.8, `DO-M2`. Not scoped to `docs/kernel/`, which `doc_audit.py`
    # defaults to and S1e.2.2 had already swept: a relative link resolves or it
    # does not wherever it is written, and 251 of the 264 broken ones this
    # first found were in `docs/history/m1d_satisfiability/` — a milestone
    # record moved out of `plans/` with its links still pointing at the tree it
    # left. 280 pages, 0.29 s.
    step "utils/doc_audit.py --links --check"
    python3 "${SCRIPT_DIR}/utils/doc_audit.py" --links --check \
        docs plans README.md AGENTS.md corpus/README.md tests/README.md \
        examples/README.md stdlib/README.md utils/README.md c/README.md

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

# The tests' `TMPDIR` (M1e2 S1e2.3.5, the header's last paragraph): this
# process's, so two gates never share one, and gone however the script ends.
GATE_TMP="${SCRIPT_DIR}/build/gate-tmp-$$"
rm -rf "${GATE_TMP}"
mkdir -p "${GATE_TMP}"
trap 'rm -rf "${GATE_TMP}"' EXIT

step "cargo test --workspace"
TMPDIR="${GATE_TMP}" cargo test --manifest-path "${MANIFEST}" --workspace ${ARGS[@]+"${ARGS[@]}"}

# The calculator's batch under the work counters, `cargo test -p ein-infer
# --features counters --test arith_batch`, was a step here from M1f S1f.6.3 to
# S1f.6.21, when the user ruled it out of the gate. No tier runs it: a stage
# that changes a rule of the calculator runs it by hand.

# Everything a test left in its `TMPDIR`, named with the file that wrote it;
# exit 1 if there is anything. Reached only after a green `cargo test`.
step "utils/check_test_scratch.py"
python3 "${SCRIPT_DIR}/utils/check_test_scratch.py" "${GATE_TMP}"

# M1e2 S1e2.1.6 — the review bundle for one small register case, built into a
# temporary directory with the debug `ein` the step above just tested, every
# SVG non-empty and every link and image reference resolving. The bundle is
# generated and gitignored, so nothing else would notice it break until the
# review that needed it. Not behind the `--tests-only` guard: it is a test of
# what the engine writes, not a static check. The whole bundle is nightly's.
step "utils/review_bundle.py --smoke"
python3 "${SCRIPT_DIR}/utils/review_bundle.py" --smoke

# CI's last step, and it is not a measurement: `--test` runs each bench once
# to see that it runs. A bench that stopped compiling is invisible to
# `cargo test` — `cargo test -p ein-corpus --no-run -v` builds no `engine`
# target — and `ein-corpus`'s bench set is the only consumer of several `pub`
# items in the engine.
#
# **It is inside the `CHECKS` guard on purpose**, which is M1e S1e.4.5 asking
# `TE-L3` the opposite of what it recommended. Not because it is slow — 0.21 s
# warm — but because it is the other half of a pair the flag drops whole: the
# bench is *type-checked* by `cargo clippy --workspace --all-targets` above,
# which emits its `.rmeta`, and codegen'd and run here. Running only this half
# would restore the expensive one while leaving the cheap, stricter one
# skipped. Nor can it buy the property the finding argues for: `--tests-only`
# would still skip six CI steps, so it is a subset either way, and the
# header's job is to say so rather than to stop being true. What it would cost
# is the number that decided it — the bench builds the **release** profile,
# which `cargo test`'s dev profile shares no artifacts with, so the first
# `--tests-only` after an engine edit pays a release codegen: **5.3 s**
# against **1.6 s** for all six static checks warm (measured 2026-09-01,
# `touch ein-infer/src/lib.rs` then `cargo bench --bench engine --no-run`),
# three times the flag's whole saving, on the iteration loop the flag exists
# for.
if [[ "${CHECKS}" == 1 ]]; then
    step "cargo bench --bench engine -- --test"
    cargo bench -q --manifest-path "${MANIFEST}" --bench engine -- --test
fi
