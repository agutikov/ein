#!/usr/bin/env python3
"""The models the main venv's tools read, fetched into $NLP (setup.sh runs it): Stanza's English models (its
resources for the installed version, into $STANZA_RESOURCES_DIR), Open English WordNet (the `wn` package's data,
into $WN_DATA_DIR), the WSD model and the BART amrlib's parsers build on (into the Hugging Face cache, $HF_HOME, each
at a pinned revision). spaCy's model is a package, in requirements-main.txt. Prints what it fetched and where.
"""

import os
import sys

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))

STANZA_PROCESSORS = "tokenize,mwt,pos,lemma,depparse,ner,coref"
WSD_MODEL = "jpwahle/t5-large-word-sense-disambiguation"
WSD_REV = "fa17be594d80398422dd64eca6ba01c91b08cc8c"
LEXICON = "oewn:2024"
BART = "facebook/bart-large"
BART_REV = "cb48c1365bd826bd521f650dc2e0940aee54720c"
# Stanza's coreference reads FacebookAI/xlm-roberta-large from the hub's main (c23d21b0620b… when S2.2f.1 ran).


def main():
    import stanza
    import wn
    from huggingface_hub import snapshot_download

    stanza.download("en", processors=STANZA_PROCESSORS, model_dir=os.environ["STANZA_RESOURCES_DIR"])
    # The coreference model reads a transformer from the Hugging Face hub (XLM-RoBERTa); building the pipeline once,
    # allowed to fetch, puts it in $HF_HOME, so that run_stanza.py runs with no download.
    stanza.Pipeline("en", processors=STANZA_PROCESSORS, use_gpu=False, model_dir=os.environ["STANZA_RESOURCES_DIR"],
                    download_method=stanza.DownloadMethod.REUSE_RESOURCES)
    print("stanza", stanza.__version__, "->", os.environ["STANZA_RESOURCES_DIR"])
    if not wn.lexicons(lexicon=LEXICON.split(":")[0]):
        wn.download(LEXICON)
    print("wn", LEXICON, "->", os.environ["WN_DATA_DIR"])
    p = snapshot_download(WSD_MODEL, revision=WSD_REV,
                          allow_patterns=["*.json", "*.model", "model.safetensors", "README.md"])
    print("wsd", WSD_MODEL, WSD_REV, "->", p)
    # amrlib's two parsers build on BART and read its tokenizer (and SPRING its weights) from the hub at load time;
    # transformers takes the revision of the hub's main, which S2.2f.1 found to be this one.
    p = snapshot_download(BART, revision=BART_REV,
                          allow_patterns=["config.json", "merges.txt", "vocab.json", "tokenizer*.json", "pytorch_model.bin"])
    print("bart", BART, BART_REV, "->", p)


if __name__ == "__main__":
    main()
