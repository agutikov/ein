#!/usr/bin/env python3
"""Every sentence of the six texts through one of amrlib's parse models: `python run_amrlib.py MODEL_DIR TAG`.

Writes under $NLP_RUNS/amr-<TAG>/: `<text>.amr`, each sentence's graph as PENMAN with its metadata (`# ::snt`,
`# ::id <text>.<n>`), and its alignments by amrlib's rule-based aligner (RBW) as `# ::alignments`; `<text>.whole.amr`,
the whole text given as one input (a multi-sentence graph, where the parser writes one); `meta.json`, the versions,
the model's folder and the hash of its weights, the device and torch's peak of reserved GPU memory. Beam search of
4, amrlib's default, which is deterministic: a run is repeatable.
"""

import hashlib
import json
import os
import sys
import time

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import texts  # noqa: E402


def sha256_of_weights(model_dir):
    h = hashlib.sha256()
    for name in sorted(os.listdir(model_dir)):
        if name.endswith((".bin", ".safetensors", ".pt")):
            with open(os.path.join(model_dir, name), "rb") as f:
                for chunk in iter(lambda: f.read(1 << 20), b""):
                    h.update(chunk)
    return h.hexdigest()


def main():
    model_dir, tag = sys.argv[1], sys.argv[2]
    import torch
    import amrlib
    import penman
    import transformers
    from amrlib.alignments.rbw_aligner import RBWAligner
    from amrlib.graph_processing.annotator import add_lemmas

    out = os.path.join(os.environ["NLP_RUNS"], f"amr-{tag}")
    os.makedirs(out, exist_ok=True)
    device = "cuda:0" if torch.cuda.is_available() else "cpu"
    t0 = time.time()
    stog = amrlib.load_stog_model(model_dir=model_dir, device=device, batch_size=1, num_beams=4)
    # SPRING's model config asks for attentions, which transformers 4.5x carries into the generation config with
    # return_dict_in_generate, so generate() returns an object amrlib's parse_sents does not expect: switched off.
    gen = getattr(getattr(stog, "model", None), "generation_config", None)
    compat = {}
    if gen is not None and (getattr(gen, "return_dict_in_generate", False) or getattr(gen, "output_attentions", False)):
        gen.return_dict_in_generate = False
        gen.output_attentions = False
        compat = {"generation_config": "return_dict_in_generate and output_attentions set False"}
    load_s = time.time() - t0

    def aligned(graph_str):
        """The graph with RBW alignments, and the count of its concept nodes aligned to a word."""
        try:
            g = add_lemmas(graph_str, snt_key="snt")
            al = RBWAligner.from_penman_w_json(g)
            pg = al.get_penman_graph()
            n_nodes = len(pg.instances())
            aligned_nodes = {a for a in (al.alignments or []) if a is not None}
            text = penman.encode(pg, indent=6)
            return text, n_nodes, len(aligned_nodes)
        except Exception as e:  # noqa: BLE001
            return graph_str + f"\n# alignment failed: {type(e).__name__}: {e}", None, None

    per_text = {}
    t1 = time.time()
    for t in texts.all_texts():
        name, sents = t["name"], t["sentences"]
        graphs = stog.parse_sents(sents, add_metadata=True)
        blocks, stats = [], []
        for i, (s, g) in enumerate(zip(sents, graphs), 1):
            if g is None:
                blocks.append(f"# ::id {name}.{i}\n# ::snt {s}\n# no graph")
                stats.append({"n": i, "graph": False})
                continue
            g = g.replace("# ::snt", f"# ::id {name}.{i}\n# ::snt", 1)
            text, n_nodes, n_al = aligned(g)
            blocks.append(text)
            stats.append({"n": i, "graph": True, "nodes": n_nodes, "aligned_nodes": n_al})
        with open(os.path.join(out, f"{name}.amr"), "w") as f:
            f.write("\n\n".join(blocks) + "\n")
        whole = stog.parse_sents([t["text"].replace("\n", " ")], add_metadata=True)[0]
        with open(os.path.join(out, f"{name}.whole.amr"), "w") as f:
            f.write((whole or f"# ::snt {t['text']}\n# no graph") + "\n")
        per_text[name] = stats
    parse_s = time.time() - t1

    meta = {
        "tool": f"amrlib {amrlib.__version__}",
        "model_dir": model_dir,
        "model_meta": json.load(open(os.path.join(model_dir, "amrlib_meta.json")))
        if os.path.exists(os.path.join(model_dir, "amrlib_meta.json")) else None,
        "weights_sha256": sha256_of_weights(model_dir),
        "versions": {"torch": torch.__version__, "cuda": torch.version.cuda, "transformers": transformers.__version__,
                     "penman": penman.__version__, "amrlib": amrlib.__version__},
        "device": device,
        "gpu": torch.cuda.get_device_name(0) if device != "cpu" else None,
        "torch_max_memory_reserved_mib": round(torch.cuda.max_memory_reserved() / 2**20) if device != "cpu" else None,
        "torch_max_memory_allocated_mib": round(torch.cuda.max_memory_allocated() / 2**20) if device != "cpu" else None,
        "load_s": round(load_s, 1),
        "parse_s": round(parse_s, 1),
        "decoding": "beam search, num_beams=4, batch_size=1",
        "compat": compat,
        "per_text": per_text,
    }
    with open(os.path.join(out, "meta.json"), "w") as f:
        json.dump(meta, f, indent=1)
    print(json.dumps({k: meta[k] for k in ("tool", "versions", "torch_max_memory_reserved_mib", "load_s", "parse_s")}))


if __name__ == "__main__":
    main()
