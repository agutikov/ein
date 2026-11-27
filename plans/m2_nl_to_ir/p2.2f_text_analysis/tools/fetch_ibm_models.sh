#!/usr/bin/env bash
# IBM's transition-based AMR parser's models and the BART it reads its sentences with, under $NLP/models/ibm/:
#   - AMR3-structbart-L (amr3.0-structured-bart-large-neur-al-seed42.zip), the sentence parser, and
#     doc-sen-conll-amr-seed42 (both_doc+sen_trainsliding_ws400x100-seed42.zip), the document parser, which writes
#     coreference across sentences as :same-as; Git LFS objects of github.com/transition-amr-parsing/parser-model-hub,
#     each checked against its LFS pointer's sha256 and unzipped where the parser's from_pretrained() looks for it
#     ($TORCH_HOME/DATA/AMR3.0/models/…);
#   - fairseq's bart.large (dl.fbaipublicfiles.com), which the parser would otherwise load through
#     torch.hub.load('pytorch/fairseq', …), that is fairseq's current GitHub main built from source; run_ibm.py
#     points that load at this folder and the installed fairseq 0.10.2 instead.
# Usage: tools/fetch_ibm_models.sh
set -euo pipefail
. "$(dirname "$0")/env.sh"

dir="$NLP/models/ibm"
mkdir -p "$dir/archives"

# zip file -> the LFS pointer's oid
declare -A ZIPS=(
  [amr3.0-structured-bart-large-neur-al-seed42.zip]="f503fd95b98710feeebd6c225078e9b15c0743076383717eb8fee0c74bb33e67"
  [both_doc+sen_trainsliding_ws400x100-seed42.zip]="24f816a21ee0aaf3cff60c577694478f547e3340ccb8426d840d5c6a027bb84f"
)
bart_sha="456f692b98c57c06f536bf714f464b62166376f5a9dddf622fd2d59027cc6b95"   # bart.large.tar.gz as fetched for S2.2f.1 (2026-11-27); no published hash

fetch() {  # url file
  [ -s "$2" ] && return 0
  curl -fL --retry 5 -C - -o "$2.part" "$1"
  mv "$2.part" "$2"
}

for zip in "${!ZIPS[@]}"; do
  fetch "https://media.githubusercontent.com/media/transition-amr-parsing/parser-model-hub/main/models/$zip" "$dir/archives/$zip"
  got="$(sha256sum "$dir/archives/$zip" | cut -d' ' -f1)"
  [ "$got" = "${ZIPS[$zip]}" ] || { echo "sha256 mismatch for $zip: $got" >&2; exit 1; }
  echo "$zip $got"
  model="${zip%-seed*}"
  if [ ! -d "$TORCH_HOME/DATA/AMR3.0/models/$model/seed42" ]; then
    python3 -I -c "import sys, zipfile; zipfile.ZipFile(sys.argv[1]).extractall(sys.argv[2])" \
      "$dir/archives/$zip" "$TORCH_HOME"
  fi
done

fetch "https://dl.fbaipublicfiles.com/fairseq/models/bart.large.tar.gz" "$dir/archives/bart.large.tar.gz"
got="$(sha256sum "$dir/archives/bart.large.tar.gz" | cut -d' ' -f1)"
[ "$got" = "$bart_sha" ] || { echo "sha256 mismatch for bart.large.tar.gz: $got" >&2; exit 1; }
echo "bart.large.tar.gz $got"
if [ ! -d "$dir/bart.large" ]; then
  tar -xzf "$dir/archives/bart.large.tar.gz" -C "$dir" --no-same-owner
fi
find "$TORCH_HOME/DATA" -maxdepth 5 -type d -name 'seed*'
