#!/usr/bin/env bash
# P2.2f's environment from nothing, under $NLP (build/nlp/, gitignored; every cache there, nothing under ~):
#   venv/      Python 3.12 (fetched by uv into $NLP/python), torch on CUDA 13.0, amrlib, Stanza, spaCy, wn,
#              transformers: amrlib's two parsers, the UD parses, coreference, word senses;
#   venv-ibm/  Python 3.8, torch 1.13.1 on CUDA 11.7, fairseq 0.10.2: IBM's transition-based AMR parser, whose
#              pins (torch==1.13.1, numpy<=1.23.5, fairseq==0.10.2) no newer Python meets;
#   models/    amrlib's parse models, IBM's model and BART, Stanza's English models, WordNet; the Hugging Face cache
#              holds the WSD model.
# The versions are the ones S2.2f.1 ended on (2026-11-27): requirements-*.txt pin the packages installed by name,
# constraints-*.txt every package of each venv; each model's archive is checked against its sha256 by its fetch script.
# Usage: tools/setup.sh [main|ibm|models|all]   (default all). A worktree sets EIN_NLP to the main checkout's.
set -euo pipefail
here="$(cd "$(dirname "$0")" && pwd)"
. "$here/env.sh"
export UV_CONCURRENT_DOWNLOADS="${UV_CONCURRENT_DOWNLOADS:-2}" UV_HTTP_TIMEOUT="${UV_HTTP_TIMEOUT:-300}" UV_HTTP_RETRIES="${UV_HTTP_RETRIES:-8}"

what="${1:-all}"

main() {
  [ -x "$NLP_PY" ] || uv venv --seed --python 3.12 "$NLP_VENV"
  uv pip install --python "$NLP_PY" "torch==2.14.1" --index-url https://download.pytorch.org/whl/cu130
  uv pip install --python "$NLP_PY" -r "$here/requirements-main.txt" -c "$here/constraints-main.txt"
}

ibm() {
  local py="$NLP/venv-ibm/bin/python"
  [ -x "$py" ] || uv venv --seed --python 3.8 "$NLP/venv-ibm"
  uv pip install --python "$py" "torch==1.13.1+cu117" --index-url https://download.pytorch.org/whl/cu117
  uv pip install --python "$py" -r "$here/requirements-ibm.txt" -c "$here/constraints-ibm.txt"
  uv pip install --python "$py" "torch-scatter==2.1.1+pt113cu117" --find-links https://data.pyg.org/whl/torch-1.13.1+cu117.html --no-build
}

models() {
  bash "$here/fetch_amrlib_models.sh"
  bash "$here/fetch_ibm_models.sh"
  "$NLP_PY" "$here/fetch_models.py"
}

case "$what" in
  main) main ;;
  ibm) ibm ;;
  models) models ;;
  all) main; ibm; models ;;
  *) echo "usage: $0 [main|ibm|models|all]" >&2; exit 2 ;;
esac
