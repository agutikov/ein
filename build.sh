#!/usr/bin/env bash
#
# build.sh — the plain-C Zebra baseline.
#
#     ./build.sh              # -> build/zebra
#     CC=clang ./build.sh
#
# One translation unit, no dependencies, nothing generated. The engine is
# built with cargo instead: `cargo build --release -p ein-cli` inside ein.rs/
# (see AGENTS.md § Running the gate), and this script deliberately does not
# wrap that — a `build.sh` that built two unrelated things would be a
# question about which one failed.
#
# Output goes to build/, which is gitignored, so the binary never lands in a
# commit.
set -euo pipefail

cd "$(dirname "$0")"
CC="${CC:-cc}"
mkdir -p build

$CC -O2 -std=c11 -Wall -Wextra -pedantic -o build/zebra zebra.c

echo "built build/zebra — run it with:  build/zebra"
