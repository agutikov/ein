#!/usr/bin/env python3
"""The six texts through Stanza: a Universal Dependencies parse, named entities and coreference across sentences.
`python run_stanza.py [--no-coref]`.

Each text is given whole (so that coreference sees every sentence). Writes under $NLP_RUNS/stanza/: `<text>.conllu`,
the UD parse (word, lemma, UPOS, features, head, relation) with each sentence's text; `<text>.coref.json`, each
chain's mentions (sentence, words, text) and representative; `<text>.ner.json`, the entities; `meta.json`, the
versions, the processors' models and torch's peak of GPU memory.
"""

import json
import os
import sys
import time

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import texts  # noqa: E402


def main():
    import stanza
    import torch
    from stanza.utils.conll import CoNLL

    coref = "--no-coref" not in sys.argv
    out = os.path.join(os.environ["NLP_RUNS"], "stanza")
    os.makedirs(out, exist_ok=True)
    processors = "tokenize,mwt,pos,lemma,depparse,ner" + (",coref" if coref else "")
    t0 = time.time()
    nlp = stanza.Pipeline("en", processors=processors, use_gpu=torch.cuda.is_available(),
                          download_method=None, model_dir=os.environ["STANZA_RESOURCES_DIR"])
    load_s = time.time() - t0
    t1 = time.time()
    for t in texts.all_texts():
        name = t["name"]
        # a sentence a line for the Zebra puzzle: keep its lines as Stanza's sentences
        doc = nlp(t["text"]) if "\n" not in t["text"] else nlp(t["text"].replace("\n", " "))
        CoNLL.write_doc2conll(doc, os.path.join(out, f"{name}.conllu"))
        chains = []
        for ch in (getattr(doc, "coref", None) or []):
            chains.append({
                "representative": ch.representative_text,
                "mentions": [{
                    "sentence": m.sentence + 1,
                    "words": [m.start_word + 1, m.end_word],
                    "text": " ".join(w.text for w in doc.sentences[m.sentence].words[m.start_word:m.end_word]),
                } for m in ch.mentions],
            })
        with open(os.path.join(out, f"{name}.coref.json"), "w") as f:
            json.dump(chains, f, indent=1)
        with open(os.path.join(out, f"{name}.ner.json"), "w") as f:
            json.dump([{"text": e.text, "type": e.type} for e in doc.ents], f, indent=1)
        n_sents = len(doc.sentences)
        if n_sents != len(t["sentences"]):
            print(f"{name}: Stanza split {n_sents} sentences, the text has {len(t['sentences'])}", file=sys.stderr)
    parse_s = time.time() - t1
    meta = {
        "tool": f"stanza {stanza.__version__}",
        "processors": processors,
        "config": {k: v for k, v in nlp.config.items() if k.endswith(("_model_path", "_package", "pretrain_path"))},
        "versions": {"stanza": stanza.__version__, "torch": torch.__version__, "cuda": torch.version.cuda},
        "torch_max_memory_reserved_mib": round(torch.cuda.max_memory_reserved() / 2**20) if torch.cuda.is_available() else None,
        "torch_max_memory_allocated_mib": round(torch.cuda.max_memory_allocated() / 2**20) if torch.cuda.is_available() else None,
        "load_s": round(load_s, 1),
        "parse_s": round(parse_s, 1),
    }
    with open(os.path.join(out, "meta.json"), "w") as f:
        json.dump(meta, f, indent=1, default=str)
    print(json.dumps({k: meta[k] for k in ("tool", "processors", "torch_max_memory_reserved_mib", "load_s", "parse_s")}))


if __name__ == "__main__":
    main()
