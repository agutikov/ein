#!/usr/bin/env python3
"""Every sentence of the six texts through IBM's transition-based AMR parser (Structured-BART, AMR 3.0):
`python run_ibm.py [MODEL_NAME]` in the parser's own venv (Python 3.8, torch 1.13.1, fairseq 0.10.2).
MODEL_NAME doc-sen-conll-amr-seed42 is the document parser: each text whole, one graph, coreference across its
sentences as `:same-as`, written under $NLP_RUNS/amr-ibm-doc/.

The parser aligns each node to the token it was made from, by construction; the graph is written with those
alignments (ISI style, `~N` after a concept, N the token's index from 0) and the tokens (`# ::tok`). Writes under
$NLP_RUNS/amr-ibm/: `<text>.amr`, each sentence's graph (`# ::id <text>.<n>`, `# ::snt`, `# ::tok`); `meta.json`,
the versions, the model's checkpoint and its hash, the count of nodes and of aligned nodes a sentence, and torch's
peak of GPU memory.

The parser loads BART through `torch.hub.load('pytorch/fairseq', 'bart.large')`, which would fetch fairseq's GitHub
main and build it; that one call is pointed at the BART fairseq 0.10.2 reads from $NLP/models/ibm/bart.large
(tools/fetch_ibm_models.sh). Nothing else of the parser is changed.
"""

import hashlib
import json
import os
import re
import sys
import time

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import texts  # noqa: E402

import torch  # noqa: E402

BART_DIR = os.path.join(os.environ["NLP"], "models", "ibm", "bart.large")
_hub_load = torch.hub.load


def _local_hub_load(repo, name, *args, **kwargs):
    if repo == "pytorch/fairseq" and name == "bart.large":
        from fairseq.models.bart import BARTModel
        return BARTModel.from_pretrained(BART_DIR, checkpoint_file="model.pt")
    return _hub_load(repo, name, *args, **kwargs)


torch.hub.load = _local_hub_load


def sha256(path):
    h = hashlib.sha256()
    with open(path, "rb") as f:
        for chunk in iter(lambda: f.read(1 << 20), b""):
            h.update(chunk)
    return h.hexdigest()


def main():
    name = sys.argv[1] if len(sys.argv) > 1 else "AMR3-structbart-L"
    import fairseq
    import penman
    from transition_amr_parser.parse import AMRParser

    doc = name.startswith("doc-")
    out = os.path.join(os.environ["NLP_RUNS"], "amr-ibm-doc" if doc else "amr-ibm")
    os.makedirs(out, exist_ok=True)
    t0 = time.time()
    parser = AMRParser.from_pretrained(name)
    load_s = time.time() - t0
    per_text = {}
    t1 = time.time()
    for t in texts.all_texts():
        if doc:
            # the document parser: the text's sentences in, one graph out, coreference as :same-as
            toks = [parser.tokenize(s)[0] for s in t["sentences"]]
            _annotations, machines = parser.parse_docs([toks])
            body = machines[0].get_amr().to_penman(jamr=False, isi=True)
            body_lines = [ln for ln in body.splitlines() if not ln.startswith("# ::")]
            head = f"# ::id {t['name']}.doc\n# ::snt {' '.join(t['sentences'])}\n"
            with open(os.path.join(out, f"{t['name']}.amr"), "w") as f:
                f.write(head + "\n".join(body_lines).strip() + "\n")
            per_text[t["name"]] = {"same_as": body.count(":same-as"), "sentences": len(t["sentences"]),
                                   "sentence_roles": len(re.findall(r":snt\d+", body))}
            continue
        blocks, stats = [], []
        for i, s in enumerate(t["sentences"], 1):
            tokens, _positions = parser.tokenize(s)
            _annotations, machine = parser.parse_sentence(tokens)
            amr = machine.get_amr()
            body = amr.to_penman(jamr=False, isi=True)
            g = penman.decode(re.sub(r"~\d+", "", body))
            nodes = len(g.instances())
            aligned = len(set(re.findall(r"\(\s*\S+\s*/\s*[^\s()~]+~(\d+)", body))) if body else 0
            aligned_nodes = len(re.findall(r"/\s*[^\s()~]+~\d+", body))
            head = f"# ::id {t['name']}.{i}\n# ::snt {s}\n# ::tok {' '.join(tokens)}\n"
            body_lines = [ln for ln in body.splitlines() if not ln.startswith("# ::")]
            blocks.append(head + "\n".join(body_lines).strip())
            stats.append({"n": i, "nodes": nodes, "aligned_nodes": aligned_nodes, "tokens_aligned_to": aligned})
        with open(os.path.join(out, f"{t['name']}.amr"), "w") as f:
            f.write("\n\n".join(blocks) + "\n")
        per_text[t["name"]] = stats
    parse_s = time.time() - t1
    ckpt_dir = os.path.join(os.environ["TORCH_HOME"], "DATA")
    ckpts = []
    for root, _dirs, files in os.walk(ckpt_dir):
        for fn in files:
            if fn.endswith(".pt"):
                p = os.path.join(root, fn)
                ckpts.append({"path": os.path.relpath(p, os.environ["NLP"]), "sha256": sha256(p)})
    meta = {
        "tool": "transition-neural-parser 0.5.4 (IBM)" + (", the document parser" if doc else ""),
        "model": name,
        "checkpoints": ckpts,
        "bart": {"dir": os.path.relpath(BART_DIR, os.environ["NLP"]),
                 "model.pt sha256": sha256(os.path.join(BART_DIR, "model.pt"))},
        "versions": {"torch": torch.__version__, "cuda": torch.version.cuda, "fairseq": fairseq.__version__,
                     "penman": penman.__version__, "python": sys.version.split()[0]},
        "device": torch.cuda.get_device_name(0) if torch.cuda.is_available() else "cpu",
        "torch_max_memory_reserved_mib": round(torch.cuda.max_memory_reserved() / 2**20) if torch.cuda.is_available() else None,
        "torch_max_memory_allocated_mib": round(torch.cuda.max_memory_allocated() / 2**20) if torch.cuda.is_available() else None,
        "load_s": round(load_s, 1),
        "parse_s": round(parse_s, 1),
        "decoding": "the parser's default, beam 1",
        "per_text": per_text,
    }
    with open(os.path.join(out, "meta.json"), "w") as f:
        json.dump(meta, f, indent=1)
    print(json.dumps({k: meta[k] for k in ("tool", "model", "versions", "torch_max_memory_reserved_mib", "load_s", "parse_s")}))


if __name__ == "__main__":
    main()
