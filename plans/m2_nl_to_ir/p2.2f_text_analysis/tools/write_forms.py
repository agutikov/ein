#!/usr/bin/env python3
"""The two forms of each recorded analysis, written, loaded, drawn and measured:
`python3 write_forms.py [--out DIR] [--model GGUF] [--repeat N]`.

For each of the six texts' recorded analyses (`einaf/tests/fixtures/analysis/<text>.json`, `write_analysis.py`'s)
and each parser it holds, through EinAf's own converter (`einaf analysis show`, `einaf analysis counts`):

- **the text forms**, `penman` and `lines`, the form alone and the block step 1's prompt holds (`--block`), with
  and without the named entities beside;
- **the facts form**, a program, loaded by `ein solve --no-search` (its exit, a refusal, root's facts by relation
  from `--json-summary`), drawn by `ein render kb` (its exit, its arrows) and, where `dot` is on the PATH, laid out
  as SVG; what a load costs, EinAf's own measure of the load (`einaf feedback FILE --no-search`, `work.load_us`,
  the median of `--repeat` calls) and the process's wall time; and the same program without its last line,
  `(solve :no-search)`, under `--no-search` once: the candidates the blind enumerator writes at root and the time;
- with `--model GGUF` (or `$EINAF_MODEL`), each form's and each block's length in the model's tokens, by
  `llama-tokenize` on the CPU (`CUDA_VISIBLE_DEVICES` empty: the card is not touched).

Writes under `--out` (default: `../forms/runs/`, gitignored) a folder a text, `summary.json`, and prints the tables
the stage's record holds. Binaries: `$EINAF_BIN` (else `einaf/target/release/einaf`, else `einaf/target/gate/einaf`),
`$EIN_BIN` (else `ein.rs/target/release/ein`, else `ein.rs/target/gate/ein`). Standard library only.
"""

import argparse
import json
import os
import shutil
import statistics
import subprocess
import sys
import time

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import texts  # noqa: E402

ROOT = texts.ROOT
FIXTURES = os.path.join(ROOT, "einaf", "tests", "fixtures", "analysis")
HERE = os.path.dirname(os.path.abspath(__file__))
PHASE = os.path.dirname(HERE)


def binary(env, *rels):
    if os.environ.get(env):
        return os.environ[env]
    for rel in rels:
        p = os.path.join(ROOT, rel)
        if os.path.exists(p):
            return p
    raise SystemExit(f"no binary: set ${env} or build one of {', '.join(rels)}")


def run(cmd, **kw):
    return subprocess.run(cmd, capture_output=True, text=True, **kw)


def timed(cmd):
    t = time.perf_counter()
    p = run(cmd)
    return p, (time.perf_counter() - t) * 1000


def tokens(model, path):
    if not model:
        return None
    env = dict(os.environ, CUDA_VISIBLE_DEVICES="")
    p = run(["llama-tokenize", "-m", model, "-f", path, "--show-count", "--ids", "--log-disable"], env=env)
    for line in p.stdout.splitlines():
        if line.startswith("Total number of tokens:"):
            return int(line.split(":")[1])
    raise SystemExit(f"llama-tokenize on {path}: {p.stderr[-400:]}")


def main():
    ap = argparse.ArgumentParser(description=__doc__.split("\n\n")[0])
    ap.add_argument("--out", default=os.path.join(PHASE, "forms", "runs"))
    ap.add_argument("--model", default=os.environ.get("EINAF_MODEL"))
    ap.add_argument("--repeat", type=int, default=5)
    a = ap.parse_args()
    einaf = binary("EINAF_BIN", "einaf/target/release/einaf", "einaf/target/gate/einaf")
    ein = binary("EIN_BIN", "ein.rs/target/release/ein", "ein.rs/target/gate/ein")
    dot = shutil.which("dot")
    summary = {"einaf": einaf, "ein": ein, "model": a.model, "texts": {}}
    for t in texts.all_texts():
        name = t["name"]
        src = os.path.join(FIXTURES, f"{name}.json")
        out = os.path.join(a.out, name)
        os.makedirs(out, exist_ok=True)
        counts = {}
        for line in run([einaf, "analysis", "counts", src]).stdout.splitlines()[1:]:
            c = line.split("\t")
            counts[c[1]] = {"aligner": c[2], "graphs": c[3], "nodes": int(c[4]), "aligned": int(c[5]),
                            "no_word": int(c[6]), "facts": int(c[7])}
        per = {"counts": counts, "forms": {}, "facts": {}}
        for parser in counts:
            for form in ["penman", "lines", "facts"]:
                for ents in [False, True]:
                    stem = f"{parser}.{form}" + (".entities" if ents else "")
                    ext = "ein" if form == "facts" else "txt"
                    extra = ["--entities"] if ents else []
                    base = [einaf, "analysis", "show", src, "--parser", parser, "--form", form, *extra]
                    form_path = os.path.join(out, f"{stem}.{ext}")
                    block_path = os.path.join(out, f"{stem}.block.md")
                    with open(form_path, "w") as f:
                        f.write(run(base).stdout)
                    with open(block_path, "w") as f:
                        f.write(run(base + ["--block"]).stdout)
                    per["forms"][stem] = {
                        "bytes": os.path.getsize(form_path),
                        "block_bytes": os.path.getsize(block_path),
                        "tokens": tokens(a.model, form_path),
                        "block_tokens": tokens(a.model, block_path),
                    }
            # The facts form: loaded, drawn, timed; and without its (solve :no-search).
            facts = os.path.join(out, f"{parser}.facts.ein")
            summ = os.path.join(out, f"{parser}.facts.summary.json")
            p, wall = timed([ein, "solve", "--no-search", facts, "--json-summary", summ])
            root = json.load(open(summ))["root"] if p.returncode == 0 else {}
            loads = []
            for _ in range(a.repeat):
                fb = json.loads(run([einaf, "feedback", facts, "--no-search"]).stdout)
                loads.append(fb["work"]["load_us"])
            outcome = fb["outcome"][0]["outcome"] if fb.get("outcome") else None
            kb = os.path.join(out, f"{parser}.kb.dot")
            k = run([ein, "render", "kb", facts])
            with open(kb, "w") as f:
                f.write(k.stdout)
            svg = None
            if dot and k.returncode == 0:
                s = run([dot, "-Tsvg", kb, "-o", kb[:-4] + ".svg"])
                svg = s.returncode == 0
            bare = os.path.join(out, f"{parser}.facts.no-solve.ein")
            with open(facts) as f:
                text = f.read()
            with open(bare, "w") as f:
                f.write(text.replace("\n(solve :no-search)\n", "\n"))
            bsumm = os.path.join(out, f"{parser}.facts.no-solve.summary.json")
            bp, bwall = timed([ein, "solve", "--no-search", bare, "--json-summary", bsumm])
            broot = json.load(open(bsumm))["root"] if bp.returncode == 0 else {}
            per["facts"][parser] = {
                "solve_exit": p.returncode,
                "diagnostics": json.load(open(summ)).get("diagnostics") if p.returncode == 0 else p.stderr,
                "root_facts": root.get("facts"),
                "by_relation": root.get("facts_by_relation"),
                "outcome": outcome,
                "load_us_median": int(statistics.median(loads)),
                "wall_ms": round(wall),
                "kb_exit": k.returncode,
                "kb_arrows": k.stdout.count("->"),
                "kb_bytes": len(k.stdout),
                "svg": svg,
                "without_no_search": {
                    "wall_ms": round(bwall),
                    "candidates": (broot.get("facts_by_relation") or {}).get("open", 0),
                    "root_facts": broot.get("facts"),
                },
            }
        summary["texts"][name] = per
    with open(os.path.join(a.out, "summary.json"), "w") as f:
        json.dump(summary, f, indent=1)
    print_tables(summary)


def print_tables(s):
    print("| text | parser | aligner | graphs | nodes | aligned | no word |")
    print("|---|---|---|---|---:|---:|---:|")
    for name, per in s["texts"].items():
        for parser, c in per["counts"].items():
            print(f"| {name} | {parser} | {c['aligner']} | {c['graphs']} | {c['nodes']} | {c['aligned']} | {c['no_word']} |")
    print()
    print("| text | parser | facts | `ein solve --no-search` | root's facts | load (EinAf, median) | wall | `ein render kb` arrows | without `(solve :no-search)`: candidates, wall |")
    print("|---|---|---:|---|---:|---:|---:|---:|---|")
    for name, per in s["texts"].items():
        for parser, f in per["facts"].items():
            w = f["without_no_search"]
            print(f"| {name} | {parser} | {per['counts'][parser]['facts']} | exit {f['solve_exit']}, {f['outcome']} | "
                  f"{f['root_facts']} | {f['load_us_median'] / 1000:.1f} ms | {f['wall_ms']} ms | {f['kb_arrows']} | "
                  f"{w['candidates']}, {w['wall_ms']} ms |")
    print()
    print("| text | form | bytes | block bytes | tokens | block tokens |")
    print("|---|---|---:|---:|---:|---:|")
    for name, per in s["texts"].items():
        for stem, f in per["forms"].items():
            print(f"| {name} | {stem} | {f['bytes']} | {f['block_bytes']} | {f['tokens']} | {f['block_tokens']} |")


if __name__ == "__main__":
    main()
