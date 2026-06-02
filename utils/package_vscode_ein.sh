#!/usr/bin/env bash
#
# Package the ein VSCode syntax-highlighting extension (utils/vscode-ein/,
# S1.7c.8) into an installable `.vsix` using the official packaging tool,
# @vscode/vsce.
#
# Packaging is a shell-script job (the grammar itself is just data —
# ein.tmLanguage.json); this wraps `vsce package` with the project's
# defaults so you don't have to remember the flags:
#
#   * Output goes to build/vsix/ein-lang-<version>.vsix by default (the
#     repo keeps generated artifacts under build/, like render_examples.sh).
#   * --baseContentUrl is auto-derived from `git remote get-url origin` +
#     the current branch, so the README's relative links resolve on the
#     web (vsce needs this because the extension lives in a repo subfolder).
#     Falls back silently to the package.json `repository` field if no
#     remote is configured.
#
# vsce is fetched on demand via `npx --yes @vscode/vsce` — no global
# install needed (override with VSCE=/path/to/vsce). Requires Node/npm.
#
# Usage:
#   utils/package_vscode_ein.sh                 # → build/vsix/ein-lang-<ver>.vsix
#   utils/package_vscode_ein.sh -o /tmp         # write the .vsix into /tmp/
#   utils/package_vscode_ein.sh --install       # package, then install it
#   utils/package_vscode_ein.sh -h
#
# Flags:
#   -o, --out DIR     Output directory for the .vsix (default: build/vsix/).
#   --install         After packaging, install the .vsix into the first of
#                     code / codium / cursor found on PATH (override: CODE=...).
#   -h, --help        This help.
#
# Environment overrides:
#   VSCE    command for the packager      (default: npx --yes @vscode/vsce)
#   CODE    editor CLI used by --install   (default: first of code/codium/cursor)
#
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"
EXT_DIR="$SCRIPT_DIR/vscode-ein"

usage() { sed -n '2,/^set -euo/{/^set -euo/d;s/^# \{0,1\}//;p;}' "${BASH_SOURCE[0]}"; }

OUT_DIR="$REPO_ROOT/build/vsix"
DO_INSTALL=0

while [[ $# -gt 0 ]]; do
  case "$1" in
    -o|--out)   OUT_DIR="$2"; shift 2 ;;
    --install)  DO_INSTALL=1; shift ;;
    -h|--help)  usage; exit 0 ;;
    *) echo "unknown argument: $1" >&2; echo "try --help" >&2; exit 2 ;;
  esac
done

[[ -f "$EXT_DIR/package.json" ]] || { echo "error: $EXT_DIR/package.json not found" >&2; exit 1; }
command -v node >/dev/null 2>&1 || { echo "error: node not found (needed for vsce)" >&2; exit 1; }

VSCE="${VSCE:-npx --yes @vscode/vsce}"

# Resolve the output dir to an ABSOLUTE path *before* we cd into the
# extension dir to run vsce — otherwise a relative -o (e.g. ./build) gets
# re-interpreted relative to utils/vscode-ein/ and vsce ENOENTs.
mkdir -p "$OUT_DIR"
OUT_DIR="$(cd "$OUT_DIR" && pwd)"

# name + version straight from the manifest (deterministic output name).
NAME="$(node -p "require('$EXT_DIR/package.json').name")"
VERSION="$(node -p "require('$EXT_DIR/package.json').version")"
VSIX_PATH="$OUT_DIR/${NAME}-${VERSION}.vsix"

# Derive --baseContentUrl so README relative links resolve (extension is a
# repo subfolder; vsce assumes the README sits at repo root otherwise).
base_url=""
remote="$(git -C "$REPO_ROOT" remote get-url origin 2>/dev/null || true)"
if [[ -n "$remote" ]]; then
  branch="$(git -C "$REPO_ROOT" rev-parse --abbrev-ref HEAD 2>/dev/null || echo master)"
  [[ "$branch" == "HEAD" ]] && branch="master"   # detached → assume master
  url="${remote%.git}"
  case "$url" in
    git@*:*)      host="${url#git@}"; host="${host%%:*}"; base_url="https://$host/${url#*:}" ;;
    ssh://git@*)  rest="${url#ssh://git@}"; base_url="https://${rest%%/*}/${rest#*/}" ;;
    https://*|http://*) base_url="$url" ;;
  esac
  [[ -n "$base_url" ]] && base_url="$base_url/blob/$branch/utils/vscode-ein/"
fi

echo ">> packaging $NAME v$VERSION → $VSIX_PATH"
pkg_args=(package --out "$VSIX_PATH")
if [[ -n "$base_url" ]]; then
  echo ">> baseContentUrl: $base_url"
  pkg_args+=(--baseContentUrl "$base_url")
else
  echo ">> no git remote — relying on package.json repository field"
fi

( cd "$EXT_DIR" && $VSCE "${pkg_args[@]}" )

echo ">> done: $VSIX_PATH"

if [[ "$DO_INSTALL" -eq 1 ]]; then
  editor="${CODE:-}"
  if [[ -z "$editor" ]]; then
    for c in code codium cursor; do
      if command -v "$c" >/dev/null 2>&1; then editor="$c"; break; fi
    done
  fi
  [[ -n "$editor" ]] || { echo "error: no editor CLI found (code/codium/cursor); set CODE=..." >&2; exit 1; }
  echo ">> installing into '$editor'"
  "$editor" --install-extension "$VSIX_PATH"
  echo ">> installed — reload the editor window to activate."
fi
