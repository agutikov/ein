#!/usr/bin/env bash
#
# build.sh — everything this repo builds, in one command.
#
#     ./build.sh                   # the engine (release) + the C baseline
#     ./build.sh --debug           # cargo's dev profile instead of release
#     ./build.sh --no-snmalloc     # link the engine against the system allocator
#     ./build.sh --all-targets     # + tests, benches and the measurement examples
#     ./build.sh --engine|--c      # one target only
#
# Two targets, and the script says which one it is on before it starts it,
# because a script that builds unrelated things owes the reader "which one
# failed" before anything else:
#
#   ein.rs/    the Rust workspace — eight crates; `ein` is the binary
#   c/         three plain-C Zebra baselines -> build/zebra-{levels,oracles,blackbox}
#
# **Prerequisites.** A Rust toolchain (`ein.rs/rust-toolchain.toml` pins it),
# a C compiler, and — unless `--no-snmalloc` — `cmake` and a C++ compiler,
# because `ein` links snmalloc by default (M1a S1a.6.2, worth 8–16 % of a
# solve). Those two are checked for up front rather than left to a cargo
# error 200 lines deep. **Graphviz** is a hard dependency of the *gate*
# (`./run_tests.sh`) and not of this script: `dot_wellformed.rs` fails rather
# than skips without it.
#
# What this does not do is run anything. `./run_tests.sh` is the gate.
set -euo pipefail

cd "$(dirname "$0")"

PROFILE=release
SNMALLOC=1
ALL_TARGETS=0
DO_ENGINE=1
DO_C=1

usage() { sed -n '2,26p' "$0" | sed 's/^# \{0,1\}//'; exit "${1:-0}"; }

while [ $# -gt 0 ]; do
    case "$1" in
        --debug)        PROFILE=dev ;;
        --release)      PROFILE=release ;;
        --no-snmalloc)  SNMALLOC=0 ;;
        --all-targets)  ALL_TARGETS=1 ;;
        --engine)       DO_C=0 ;;
        --c)            DO_ENGINE=0 ;;
        -h|--help)      usage 0 ;;
        *) echo "build.sh: unknown argument '$1'" >&2; usage 2 ;;
    esac
    shift
done

step() { printf '\n\033[1m── %s\033[0m\n' "$1"; }
need() {
    command -v "$1" >/dev/null 2>&1 || {
        echo "build.sh: '$1' not found — $2" >&2
        exit 127
    }
}

# ── the engine ─────────────────────────────────────────────────────

if [ "$DO_ENGINE" = 1 ]; then
    step "ein.rs — the Rust workspace ($PROFILE)"
    need cargo "install a Rust toolchain (rustup); ein.rs/rust-toolchain.toml pins the version"

    CARGO_ARGS=(build --manifest-path ein.rs/Cargo.toml --workspace)
    [ "$PROFILE" = release ] && CARGO_ARGS+=(--release)
    [ "$ALL_TARGETS" = 1 ] && CARGO_ARGS+=(--all-targets)

    if [ "$SNMALLOC" = 1 ]; then
        need cmake "snmalloc's build needs it. Pass --no-snmalloc to link the system allocator instead (costs 8-16% of a solve)"
        need c++ "snmalloc is C++. Pass --no-snmalloc to link the system allocator instead"
    else
        CARGO_ARGS+=(-p ein-cli --no-default-features)
        echo "   (system allocator: -p ein-cli --no-default-features)"
    fi

    cargo "${CARGO_ARGS[@]}"
fi

# ── the C baseline ─────────────────────────────────────────────────

if [ "$DO_C" = 1 ]; then
    step "c/ — the plain-C baselines"
    CC="${CC:-cc}"
    need "$CC" "set CC to a C compiler"
    mkdir -p build
    CFLAGS="-std=c11 -Wall -Wextra -pedantic"
    [ "$PROFILE" = dev ] && CFLAGS="-O0 -g $CFLAGS" || CFLAGS="-O2 $CFLAGS"

    # One line per binary: output name, then its translation units. The third
    # is two, and that is the point of it — `blackbox.c` cannot see inside
    # `zebra_module.c` (c/README.md).
    build_c() {
        local out="$1"
        shift
        # shellcheck disable=SC2086
        $CC $CFLAGS -o "build/$out" "$@"
        echo "   build/$out  <-  $*"
    }
    build_c zebra-levels   c/zebra_levels.c
    build_c zebra-oracles  c/zebra_oracles.c
    build_c zebra-blackbox c/blackbox.c c/zebra_module.c
fi

# ── what came out ──────────────────────────────────────────────────

step "built"
TARGET_DIR="ein.rs/target/$([ "$PROFILE" = release ] && echo release || echo debug)"
ARTEFACTS=()
[ "$DO_ENGINE" = 1 ] && ARTEFACTS+=("$TARGET_DIR/ein")
[ "$DO_C" = 1 ] && ARTEFACTS+=(build/zebra-levels build/zebra-oracles build/zebra-blackbox)
for artefact in "${ARTEFACTS[@]}"; do
    printf '   %-28s %s\n' "$artefact" "$(du -h "$artefact" | cut -f1)"
done
echo
[ "$DO_ENGINE" = 1 ] &&
    echo "   the engine:    $TARGET_DIR/ein solve examples/zebra.ein" &&
    echo "   what it is:    $TARGET_DIR/ein --version   (features, and the stdlib it resolves)"
[ "$DO_C" = 1 ] &&
    echo "   the baselines: build/zebra-levels   (milliseconds)" &&
    echo "                  build/zebra-oracles  (minutes — c/README.md says why)" &&
    echo "                  build/zebra-blackbox (minutes)"
echo "   the gate:      ./run_tests.sh"
