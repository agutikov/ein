#!/usr/bin/env python3
"""Word senses for the six texts, and the kind each sense is of: `python run_wsd.py` (after run_stanza.py).

The words are Stanza's (its UD parse, $NLP_RUNS/stanza/<text>.conllu): every noun, proper noun, verb and adjective,
by its lemma, and every two words joined by `compound` or `flat` also as one (*Old Gold*, *orange juice*). The
inventory is Open English WordNet (the `wn` package and its data, a lexicon; not NLTK). Where a word has one sense
it is taken; where it has several, a trained disambiguator chooses among their glosses: T5-large trained on SemCor
(jpwahle/t5-large-word-sense-disambiguation), given the sentence, the word and the glosses, its reply matched to the
nearest gloss. For the chosen sense, its hypernyms up to the top: what kind of thing the word names.

Writes under $NLP_RUNS/wsd/: `<text>.json`, a word a record (sentence, word, lemma, UPOS, the candidates' count, the
chosen synset, its gloss, its hypernym path, how it was chosen); `meta.json`.
"""

import difflib
import json
import os
import sys
import time

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import texts  # noqa: E402

MODEL = "jpwahle/t5-large-word-sense-disambiguation"
MODEL_REV = "fa17be594d80398422dd64eca6ba01c91b08cc8c"
LEXICON = "oewn:2024"
POS = {"NOUN": "n", "PROPN": "n", "VERB": "v", "ADJ": "a"}


def read_conllu(path):
    sents, cur, text = [], [], None
    for line in open(path, encoding="utf-8"):
        line = line.rstrip("\n")
        if line.startswith("# text = "):
            text = line[len("# text = "):]
        elif not line:
            if cur:
                sents.append({"text": text, "words": cur})
            cur, text = [], None
        elif not line.startswith("#"):
            c = line.split("\t")
            if "-" in c[0] or "." in c[0]:
                continue
            cur.append({"id": int(c[0]), "form": c[1], "lemma": c[2], "upos": c[3], "head": int(c[6]), "deprel": c[7]})
    if cur:
        sents.append({"text": text, "words": cur})
    return sents


def hypernym_path(ss, limit=8):
    path, cur = [], ss
    while cur is not None and len(path) < limit:
        hs = cur.get_related("hypernym") or cur.get_related("instance_hypernym")
        if not hs:
            break
        cur = hs[0]
        path.append(f"{cur.lemmas()[0]} ({cur.id})")
    return path


def main():
    import torch
    import transformers
    import wn
    from transformers import AutoModelForSeq2SeqLM, AutoTokenizer

    if not wn.lexicons(lexicon=LEXICON.split(":")[0]):
        wn.download(LEXICON)
    en = wn.Wordnet(LEXICON)
    device = "cuda:0" if torch.cuda.is_available() else "cpu"
    t0 = time.time()
    tok = AutoTokenizer.from_pretrained(MODEL, revision=MODEL_REV)
    model = AutoModelForSeq2SeqLM.from_pretrained(MODEL, revision=MODEL_REV).to(device).eval()
    load_s = time.time() - t0
    out = os.path.join(os.environ["NLP_RUNS"], "wsd")
    os.makedirs(out, exist_ok=True)

    def choose(sentence, word, cands):
        glosses = [c.definition() or "" for c in cands]
        quoted = sentence.replace(word, f'" {word} "', 1) if word in sentence else sentence
        descs = " , ".join(f'" {g} "' for g in glosses[:-1]) + f' , or " {glosses[-1]} "'
        prompt = (f'question: which description describes the word " {word} " best in the following context? '
                  f"descriptions: [ {descs} ] context: {quoted}")
        enc = tok(prompt, return_tensors="pt", truncation=True, max_length=1024).to(device)
        with torch.no_grad():
            ids = model.generate(**enc, max_length=135, num_beams=1, do_sample=False)
        reply = tok.decode(ids[0], skip_special_tokens=True).strip()
        scores = [difflib.SequenceMatcher(None, reply.lower(), g.lower()).ratio() for g in glosses]
        best = max(range(len(cands)), key=lambda i: scores[i])
        return best, reply, round(scores[best], 3)

    t1 = time.time()
    counts = {}
    for t in texts.all_texts():
        recs = []
        for si, s in enumerate(read_conllu(os.path.join(os.environ["NLP_RUNS"], "stanza", f"{t['name']}.conllu")), 1):
            items = []
            for w in s["words"]:
                if w["upos"] in POS:
                    items.append((w["form"], w["lemma"], w["upos"]))
            by_id = {w["id"]: w for w in s["words"]}
            for w in s["words"]:
                if w["deprel"] in ("compound", "flat") and w["head"] in by_id:
                    h = by_id[w["head"]]
                    a, b = (w, h) if w["id"] < h["id"] else (h, w)
                    items.append((f"{a['form']} {b['form']}", f"{a['lemma']} {b['lemma']}", "MWE"))
            for form, lemma, upos in items:
                pos = POS.get(upos, "n")
                poses = ("a", "s") if pos == "a" else (pos,)  # most adjectives are WordNet's satellites, `s`
                cands = [c for p in poses for c in en.synsets(lemma, pos=p)] or \
                    [c for p in poses for c in en.synsets(form, pos=p)] or \
                    (en.synsets(lemma) if upos == "MWE" else [])
                rec = {"sentence": si, "word": form, "lemma": lemma, "upos": upos, "candidates": len(cands)}
                if not cands:
                    rec["how"] = "not in WordNet"
                elif len(cands) == 1:
                    ss = cands[0]
                    rec.update({"synset": ss.id, "gloss": ss.definition(), "how": "one sense"})
                else:
                    i, reply, score = choose(s["text"], form, cands)
                    ss = cands[i]
                    rec.update({"synset": ss.id, "gloss": ss.definition(), "how": "chosen by T5",
                                "reply": reply, "match": score})
                if cands:
                    rec["hypernyms"] = hypernym_path(ss)
                    rec["lemmas"] = ss.lemmas()[:4]
                recs.append(rec)
        with open(os.path.join(out, f"{t['name']}.json"), "w") as f:
            json.dump(recs, f, indent=1)
        counts[t["name"]] = len(recs)
    parse_s = time.time() - t1
    meta = {
        "tool": f"wn {wn.__version__} + {MODEL}@{MODEL_REV[:12]}",
        "lexicon": LEXICON,
        "versions": {"wn": wn.__version__, "torch": torch.__version__, "transformers": transformers.__version__},
        "device": device,
        "torch_max_memory_reserved_mib": round(torch.cuda.max_memory_reserved() / 2**20) if device != "cpu" else None,
        "load_s": round(load_s, 1),
        "parse_s": round(parse_s, 1),
        "words": counts,
    }
    with open(os.path.join(out, "meta.json"), "w") as f:
        json.dump(meta, f, indent=1)
    print(json.dumps(meta))


if __name__ == "__main__":
    main()
