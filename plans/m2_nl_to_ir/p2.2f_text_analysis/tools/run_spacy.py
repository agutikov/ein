#!/usr/bin/env python3
"""The six texts through spaCy's transformer pipeline: `python run_spacy.py [MODEL]` (default en_core_web_trf).

spaCy's part-of-speech tags are Universal Dependencies' (UPOS), its dependency relations are its own scheme
(ClearNLP's, not UD's), and it finds named entities and noun chunks. Writes under $NLP_RUNS/spacy-<MODEL>/:
`<text>.tsv`, a word a line (sentence, index, word, lemma, UPOS, morphology, head, relation, entity);
`<text>.ents.json` and `<text>.chunks.json`; `meta.json`, the versions, the device and torch's peak of GPU memory.
"""

import json
import os
import sys
import time

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import texts  # noqa: E402


def main():
    import spacy
    import torch

    model = sys.argv[1] if len(sys.argv) > 1 else "en_core_web_trf"
    on_gpu = False
    try:
        on_gpu = spacy.prefer_gpu()
    except Exception as e:  # noqa: BLE001
        print(f"spacy.prefer_gpu: {type(e).__name__}: {e}", file=sys.stderr)
    out = os.path.join(os.environ["NLP_RUNS"], f"spacy-{model}")
    os.makedirs(out, exist_ok=True)
    t0 = time.time()
    nlp = spacy.load(model)
    load_s = time.time() - t0
    t1 = time.time()
    for t in texts.all_texts():
        name = t["name"]
        doc = nlp(t["text"].replace("\n", " "))
        rows = ["# sent\tidx\tword\tlemma\tupos\tmorph\thead\tdep\tent"]
        for si, sent in enumerate(doc.sents, 1):
            rows.append(f"# text = {sent.text}")
            for w in sent:
                head = 0 if w.head.i == w.i else w.head.i - sent.start + 1
                ent = f"{w.ent_iob_}-{w.ent_type_}" if w.ent_type_ else w.ent_iob_
                rows.append(f"{si}\t{w.i - sent.start + 1}\t{w.text}\t{w.lemma_}\t{w.pos_}\t{w.morph}\t{head}\t{w.dep_}\t{ent}")
        with open(os.path.join(out, f"{name}.tsv"), "w") as f:
            f.write("\n".join(rows) + "\n")
        with open(os.path.join(out, f"{name}.ents.json"), "w") as f:
            json.dump([{"text": e.text, "label": e.label_} for e in doc.ents], f, indent=1)
        with open(os.path.join(out, f"{name}.chunks.json"), "w") as f:
            json.dump([c.text for c in doc.noun_chunks], f, indent=1)
    parse_s = time.time() - t1
    meta = {
        "tool": f"spacy {spacy.__version__}",
        "model": f"{model} {nlp.meta.get('version')}",
        "pipeline": nlp.pipe_names,
        "versions": {"spacy": spacy.__version__, "torch": torch.__version__},
        "device": "gpu" if on_gpu else "cpu",
        "torch_max_memory_reserved_mib": round(torch.cuda.max_memory_reserved() / 2**20) if torch.cuda.is_available() else None,
        "load_s": round(load_s, 1),
        "parse_s": round(parse_s, 1),
    }
    with open(os.path.join(out, "meta.json"), "w") as f:
        json.dump(meta, f, indent=1)
    print(json.dumps(meta))


if __name__ == "__main__":
    main()
