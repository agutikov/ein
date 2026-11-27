#!/usr/bin/env python3
"""The recorded analyses EinAf reads: what the parsers that passed S2.2f.1's check wrote of each text, one file a text.

    python3 write_analysis.py [--runs DIR] [--out DIR] [--check]

Reads S2.2f.1's runs (`--runs`, else `$NLP_RUNS`, else this checkout's `check/runs/`; a worktree names the main
checkout's): each AMR parser's graphs
of the text's sentences (`amr-spring/`, `amr-xfm/`, `amr-ibm/`: `<text>.amr` and `meta.json`) and spaCy's named
entities (`spacy-en_core_web_trf/`: `<text>.tsv`, `<text>.ents.json` and `meta.json`). Writes
`einaf/tests/fixtures/analysis/<text>.json` (`--out` moves it), schema `einaf-analysis/1`:

- `sentences`: the text's sentences, as `texts.py` splits them and EinAf's `contract::sentences` does;
- `amr`: a parser each, SPRING first (the check's proposal), then XFM and IBM: the tool, its version, its model and
  the model's hash, its decoding; **the aligner** whose markers the graphs carry and whether it is `learned` (IBM's
  parser aligns each node it makes) or `rule-based` (amrlib's RBW, on SPRING's and XFM's graphs); and a graph a
  sentence, its PENMAN as the run wrote it, the aligner's markers in it, and the tokens they index;
- `entities`: spaCy's named entities a sentence, each its words and its label, with the tool.

It gathers and converts nothing: the two forms, as text and as Ein facts, are EinAf's (`einaf analysis`). Standard
library only, so it runs under any Python 3.8 on, outside the environment. `--check` writes nothing and fails where
a committed file is not what it would write.
"""

import argparse
import hashlib
import json
import os
import sys

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import texts  # noqa: E402

SCHEMA = "einaf-analysis/1"
PARSERS = ["spring", "xfm", "ibm"]
SPACY = "spacy-en_core_web_trf"
RUNS = os.path.join(texts.ROOT, "plans", "m2_nl_to_ir", "p2.2f_text_analysis", "check", "runs")


def sha256_of(path):
    h = hashlib.sha256()
    with open(path, "rb") as f:
        h.update(f.read())
    return "sha256:" + h.hexdigest()


def blocks(path):
    """The blocks of a `.amr` file: its metadata lines and its graph, a block a sentence."""
    with open(path, encoding="utf-8") as f:
        raw = f.read()
    out = []
    for chunk in raw.strip().split("\n\n"):
        meta, body = {}, []
        for line in chunk.splitlines():
            if line.startswith("# ::"):
                key, _, value = line[4:].partition(" ")
                meta[key] = value
            elif line.startswith("#"):
                meta.setdefault("notes", []).append(line[1:].strip())
            elif line.strip():
                body.append(line.rstrip())
        out.append((meta, "\n".join(body)))
    return out


def tool_of(parser, meta):
    """What made a parser's graphs, from its run's `meta.json`."""
    if parser == "ibm":
        return {
            "name": "transition-neural-parser",
            "version": meta["tool"].split()[1],
            "model": meta["model"],
            "model_sha256": "sha256:" + meta["checkpoints"][0]["sha256"],
            "beside": {"fairseq bart.large model.pt": "sha256:" + meta["bart"]["model.pt sha256"]},
            "decoding": meta["decoding"],
            "versions": meta["versions"],
        }
    return {
        "name": "amrlib",
        "version": meta["versions"]["amrlib"],
        "model": os.path.basename(meta["model_dir"]),
        "model_sha256": "sha256:" + meta["weights_sha256"],
        "decoding": meta["decoding"],
        "versions": meta["versions"],
        "compat": meta.get("compat") or {},
    }


def aligner_of(parser):
    if parser == "ibm":
        return {"name": "the parser's own alignment", "kind": "learned", "markers": "~N, N a token's index from 0"}
    return {"name": "amrlib's RBW aligner", "kind": "rule-based", "markers": "~e.N, N a token's index from 0"}


def amr_of(runs, parser, name, sentences):
    folder = os.path.join(runs, f"amr-{parser}")
    with open(os.path.join(folder, "meta.json"), encoding="utf-8") as f:
        meta = json.load(f)
    graphs = []
    for n, (m, body) in enumerate(blocks(os.path.join(folder, f"{name}.amr")), 1):
        if m.get("id") != f"{name}.{n}":
            raise SystemExit(f"{parser}/{name}.amr: block {n} is {m.get('id')!r}")
        if m.get("snt") != sentences[n - 1]:
            raise SystemExit(f"{parser}/{name}.amr: sentence {n} is not the text's: {m.get('snt')!r}")
        if "tokens" in m:
            tokens = json.loads(m["tokens"])
        else:
            tokens = m.get("tok", "").split()
        failed = [x for x in m.get("notes", []) if x.startswith(("no graph", "alignment failed"))]
        graphs.append({"n": n, "tokens": tokens, "penman": body or None, **({"notes": failed} if failed else {})})
    if len(graphs) != len(sentences):
        raise SystemExit(f"{parser}/{name}.amr: {len(graphs)} graphs for {len(sentences)} sentences")
    return {"parser": parser, "tool": tool_of(parser, meta), "aligner": aligner_of(parser), "graphs": graphs}


def entities_of(runs, name, sentences):
    folder = os.path.join(runs, SPACY)
    with open(os.path.join(folder, "meta.json"), encoding="utf-8") as f:
        meta = json.load(f)
    with open(os.path.join(folder, f"{name}.ents.json"), encoding="utf-8") as f:
        ents = json.load(f)
    # The tsv's B- tags, in order, are the document's entities, in order: each one's sentence.
    split, starts = [], []
    with open(os.path.join(folder, f"{name}.tsv"), encoding="utf-8") as f:
        for line in f:
            line = line.rstrip("\n")
            if line.startswith("# text = "):
                split.append(line[len("# text = "):].strip())
            elif line and not line.startswith("#"):
                cols = line.split("\t")
                if cols[8].startswith("B-"):
                    starts.append((int(cols[0]), cols[8][2:]))
    if split != sentences:
        raise SystemExit(f"spacy/{name}: its sentences are not the text's")
    if len(starts) != len(ents) or any(lab != e["label"] for (_, lab), e in zip(starts, ents)):
        raise SystemExit(f"spacy/{name}: the tsv's entities are not ents.json's")
    per = [[] for _ in sentences]
    for (s, _), e in zip(starts, ents):
        per[s - 1].append({"text": e["text"], "label": e["label"]})
    tool = {
        "name": "spacy",
        "version": meta["versions"]["spacy"],
        "model": meta["model"],
        "pipeline": meta["pipeline"],
        "device": meta["device"],
        "versions": meta["versions"],
    }
    return {"tool": tool, "sentences": per}


def analysis_of(runs, t):
    name, sentences = t["name"], t["sentences"]
    return {
        "schema": SCHEMA,
        "text": name,
        "text_sha256": sha256_of(os.path.join(texts.CONTRACT, name, "text.txt")),
        "sentences": sentences,
        "amr": [amr_of(runs, p, name, sentences) for p in PARSERS],
        "entities": entities_of(runs, name, sentences),
    }


def main():
    ap = argparse.ArgumentParser(description=__doc__.split("\n\n")[0])
    ap.add_argument("--runs", default=os.environ.get("NLP_RUNS") or RUNS)
    ap.add_argument("--out", default=os.path.join(texts.ROOT, "einaf", "tests", "fixtures", "analysis"))
    ap.add_argument("--check", action="store_true")
    a = ap.parse_args()
    runs = a.runs
    if not os.path.isdir(os.path.join(runs, "amr-spring")):
        raise SystemExit(f"{runs}: no runs of S2.2f.1's here; --runs names them")
    bad = 0
    os.makedirs(a.out, exist_ok=True)
    for t in texts.all_texts():
        text = json.dumps(analysis_of(runs, t), indent=1, ensure_ascii=False) + "\n"
        path = os.path.join(a.out, f"{t['name']}.json")
        if a.check:
            old = open(path, encoding="utf-8").read() if os.path.exists(path) else None
            if old != text:
                print(f"{path}: not what write_analysis.py writes")
                bad += 1
            continue
        with open(path, "w", encoding="utf-8") as f:
            f.write(text)
        print(f"{path}: {len(t['sentences'])} sentences, {sha256_of(path)}")
    return 1 if bad else 0


if __name__ == "__main__":
    sys.exit(main())
