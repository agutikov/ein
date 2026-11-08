#!/usr/bin/env bash
#
# ci_container.sh — the gate, or the nightly, in a container built like
# GitHub's runner image, so that an image move can be checked before a push.
# M1f S1f.5.17.
#
#     utils/ci_container.sh                          # ./run_tests.sh on ubuntu:26.04, amd64v3 packages
#     utils/ci_container.sh --nightly                # ./run_tests.sh --nightly there
#     utils/ci_container.sh --image ubuntu:24.04 --variant none
#     utils/ci_container.sh --tests-only -p ein-render --test dot_wellformed
#     utils/ci_container.sh --shell                  # a shell in the checkout instead
#
# Every argument but --image, --variant and --shell goes to ./run_tests.sh.
#
# **What it checks is HEAD as committed**, not the working tree: the checkout
# is a clone of this repository under `build/ci-container/<image>-<variant>/`,
# moved to HEAD's commit on every run, and its `ein.rs/target` stays there
# between runs (the gate's own `TMPDIR` is under its `build/`). Commit first.
#
# **Why `--variant amd64v3` is the default.** The `ubuntu-26.04` runner
# installs Ubuntu 26.04's x86-64-v3 architecture variant of every package
# (its apt log unpacks `*_amd64v3.deb`), and a plain `ubuntu:26.04` container
# installs the baseline build. The two are not the same Graphviz: the amd64v3
# `libpathplan` contracts its orientation test into a fused multiply-add, and
# 14.1.2's spline router then failed on 20 corpus views the baseline build
# draws without a word (S1f.5.17 § Step 1, 2026-11-08). The image needs an
# x86-64-v3 CPU to run that variant. `--variant none` is the baseline.
#
# The image installs what the workflows install, `graphviz` and `python3`
# (3.14 on 26.04, which is what `setup-python` gives the runner; 24.04's is
# 3.12), plus `cmake` and `g++` for `snmalloc`, and rustup's toolchain from
# `ein.rs/rust-toolchain.toml`. It runs as your user, so nothing it writes
# under `build/` is root's. It needs `docker` (or `$CONTAINER`, a
# docker-compatible CLI) and network access the first time an image is built.

set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
CONTAINER="${CONTAINER:-docker}"
IMAGE="ubuntu:26.04"
VARIANT="amd64v3"
SHELL_ONLY=0
ARGS=()

if [[ "${1:-}" == "-h" || "${1:-}" == "--help" ]]; then
    awk 'NR>2 && !/^#/ {exit} NR>2 {sub(/^# ?/, ""); print}' "${BASH_SOURCE[0]}"
    exit 0
fi
while (( $# )); do
    case "$1" in
        --image)   IMAGE="$2"; shift 2 ;;
        --variant) VARIANT="$2"; shift 2 ;;
        --shell)   SHELL_ONLY=1; shift ;;
        *)         ARGS+=( "$1" ); shift ;;
    esac
done
[[ "${VARIANT}" == none ]] && VARIANT=""

if ! command -v "${CONTAINER}" >/dev/null 2>&1; then
    echo "error: no ${CONTAINER} on PATH — set \$CONTAINER to a docker-compatible CLI." >&2
    exit 127
fi

TOOLCHAIN="$(sed -n 's/^channel *= *"\(.*\)"/\1/p' "${ROOT}/ein.rs/rust-toolchain.toml")"
SLUG="$(echo "${IMAGE}" | tr ':/' '--')${VARIANT:+-${VARIANT}}"
TAG="ein-ci:${SLUG}-rust${TOOLCHAIN}"
WORK="${ROOT}/build/ci-container/${SLUG}"
mkdir -p "${WORK}/image"   # the build context, empty on purpose

# The image: rebuilt only when it is missing.
if ! "${CONTAINER}" image inspect "${TAG}" >/dev/null 2>&1; then
    echo "── building ${TAG}" >&2
    "${CONTAINER}" build -t "${TAG}" \
        --build-arg BASE="${IMAGE}" --build-arg VARIANT="${VARIANT}" \
        --build-arg TOOLCHAIN="${TOOLCHAIN}" -f - "${WORK}/image" <<'DOCKERFILE'
ARG BASE
FROM ${BASE}
ARG VARIANT
ARG TOOLCHAIN
ENV DEBIAN_FRONTEND=noninteractive
RUN if [ -n "${VARIANT}" ]; then \
        echo "APT::Architecture-Variants \"${VARIANT}\";" > /etc/apt/apt.conf.d/99variant; fi \
    && apt-get update && apt-get install -y \
        graphviz python3 cmake g++ make curl ca-certificates git pkg-config \
    && rm -rf /var/lib/apt/lists/*
ENV RUSTUP_HOME=/opt/rustup CARGO_HOME=/opt/cargo PATH=/opt/cargo/bin:$PATH
RUN curl -sSf https://sh.rustup.rs | sh -s -- -y --profile minimal \
        --default-toolchain "${TOOLCHAIN}" -c rustfmt -c clippy \
    && chmod -R a+rwX /opt/rustup /opt/cargo
DOCKERFILE
fi

# The checkout: a clone of this repository, at HEAD's commit.
HEAD="$(git -C "${ROOT}" rev-parse HEAD)"
if [[ ! -d "${WORK}/src/.git" ]]; then
    git clone -q --no-hardlinks "${ROOT}" "${WORK}/src"
fi
git -C "${WORK}/src" fetch -q "${ROOT}" "+refs/heads/*:refs/remotes/host/*"
git -C "${WORK}/src" checkout -q --detach "${HEAD}"
git -C "${WORK}/src" clean -q -fdx -e ein.rs/target -e build

# Mounted at its own path, not a short one: the tests bake
# `CARGO_MANIFEST_DIR` into their binaries, and `corpus_shapes.rs` makes a
# rendering portable by replacing the repo root's text with `<repo>`, so a
# root of `/w` rewrote every `/w…` in 1 446 renderings (2026-11-08).
SRC="${WORK}/src"
RUN=( "${CONTAINER}" run --rm --user "$(id -u):$(id -g)" -e HOME=/tmp
      -v "${SRC}:${SRC}" -w "${SRC}" )
"${RUN[@]}" "${TAG}" bash -c '
    . /etc/os-release; echo "── ${PRETTY_NAME}"
    dot -V 2>&1 | sed "s/^/── /"
    printf "── libpathplan4 %s %s\n" "$(dpkg-query -W -f "\${Version}" libpathplan4)" \
        "$(dpkg -s libpathplan4 | sed -n "s/^Architecture-Variant: //p")"
    echo "── $(python3 --version), $(rustc --version)"
    echo "── checkout $(git log -1 --format="%h %s" | cut -c1-70)"' >&2
if (( SHELL_ONLY )); then
    exec "${CONTAINER}" run --rm -it --user "$(id -u):$(id -g)" -e HOME=/tmp \
        -v "${SRC}:${SRC}" -w "${SRC}" "${TAG}" bash
fi
exec "${RUN[@]}" "${TAG}" ./run_tests.sh "${ARGS[@]}"
