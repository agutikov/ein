#!/usr/bin/env bash
# S2.2f.1's runs: every tool on every sentence of the six texts, one tool at a time, each under gpumon.py (the card's
# memory read before and every 100 ms during), then the AMR graphs drawn. Writes under $NLP_RUNS (gitignored).
# Usage: tools/run_all.sh [TOOL …]   (default: every tool, in this order: xfm spring ibm ibm-doc stanza spacy wsd)
set -uo pipefail
. "$(dirname "$0")/env.sh"

tools=("$@")
[ ${#tools[@]} -eq 0 ] && tools=(xfm spring ibm ibm-doc stanza spacy wsd)
mkdir -p "$NLP_RUNS/gpu"

run() {  # tag command…
  local tag="$1"; shift
  echo "== $tag: $(date -Iseconds)"
  nvidia-smi --query-gpu=memory.used,memory.total --format=csv,noheader
  python3 -I "$NLP_TOOLS/gpumon.py" "$NLP_RUNS/gpu/$tag.json" -- "$@" 2>&1 | tee "$NLP_RUNS/gpu/$tag.log"
}

for t in "${tools[@]}"; do
  case "$t" in
    xfm)    run xfm    "$NLP_PY" "$NLP_TOOLS/run_amrlib.py" "$NLP/models/amrlib/parse_xfm_bart_large-v0_1_0" xfm ;;
    spring) run spring "$NLP_PY" "$NLP_TOOLS/run_amrlib.py" "$NLP/models/amrlib/parse_spring-v0_1_0" spring ;;
    ibm)    run ibm    "$NLP/venv-ibm/bin/python" "$NLP_TOOLS/run_ibm.py" AMR3-structbart-L ;;
    ibm-doc) run ibm-doc "$NLP/venv-ibm/bin/python" "$NLP_TOOLS/run_ibm.py" doc-sen-conll-amr-seed42 ;;
    stanza) run stanza "$NLP_PY" "$NLP_TOOLS/run_stanza.py" ;;
    spacy)  run spacy  "$NLP_PY" "$NLP_TOOLS/run_spacy.py" en_core_web_trf ;;
    wsd)    run wsd    "$NLP_PY" "$NLP_TOOLS/run_wsd.py" ;;
    *) echo "unknown tool $t" >&2; exit 2 ;;
  esac
done

for d in "$NLP_RUNS"/amr-*; do
  [ -d "$d" ] && "$NLP_PY" "$NLP_TOOLS/render_amr.py" "$d"
done
