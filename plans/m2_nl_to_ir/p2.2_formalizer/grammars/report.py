#!/usr/bin/env python3
"""S2.2.3's tables, from its records: `python3 report.py RECORD` (`first/record.jsonl.gz`, `fenced/record.jsonl.gz`),
`python3 report.py --speeds`.

- the recorded generations under a grammar (RECORD), a line a run, each beside the probe's run of the
  same request with no grammar (`../model_runtime/probe.jsonl.gz`): the tokens, the stop and, where a generation ends
  in a cycle, its period and where it began; the grammar's build time, where it began and the tokens whose first
  choice it refused; whether the program the answer holds (its fenced block where it has one, else the answer whole)
  parses and loads, the outcome under `--single` and the answer's lines; and, beside the probe's run, the tokens the
  two share from the first and the text each goes on with where they part;
- the speeds (`speed/record.jsonl.gz`), per request: the generated tokens, the stop, the tokens a second and the
  build time, each the median of the three calls, as `speed/README.md` reads them.

Reads the records alone: no model, no kernel. Standard library only.
"""

import gzip
import json
import statistics
import sys
from pathlib import Path

HERE = Path(__file__).resolve().parent


def lines(path):
    op = gzip.open if str(path).endswith(".gz") else open
    with op(path, "rt") as f:
        for l in f:
            if l.strip():
                yield json.loads(l)


def loop(toks, maxp=4000):
    """The period of the cycle a generation ends in, and the token it began at; None where its last three
    periods are not one cycle. As the probe's report reads it."""
    n = len(toks)
    for p in range(1, min(maxp, n // 3)):
        if toks[n - p:] == toks[n - 2 * p:n - p] == toks[n - 3 * p:n - 2 * p]:
            i = n - p
            while i > 0 and toks[i - 1] == toks[i - 1 + p]:
                i -= 1
            return p, i
    return None


def outcome(j, i):
    fb = j.get("feedback") or []
    if len(fb) <= i or not fb[i].get("outcome"):
        return None
    return fb[i]["outcome"][0]["outcome"]


def answer_lines(j):
    fb = j.get("feedback") or []
    if len(fb) < 2:
        return []
    out = []
    for q in fb[1].get("queries") or []:
        for c in q.get("classes") or []:
            out.extend(c.get("lines") or [])
    return out


def reading(line, name):
    for j in line.get("judged") or []:
        if j["reading"] == name:
            return j
    return None


def read(line):
    """The program an answer holds: its first fenced block where it has one, else the answer whole."""
    f = reading(line, "fenced")
    if f is not None and f.get("text") is not None:
        return f
    return reading(line, "whole")


def verdict(j):
    if j is None or j.get("text") is None:
        return "none"
    if not (j.get("parsed") or {}).get("forms"):
        kinds = ",".join(j.get("kinds") or [])
        return f"parse ({kinds})" if kinds else "parse"
    if not j.get("loaded"):
        return "load (" + ",".join(j.get("kinds") or []) + ")"
    return f"loads; --single {outcome(j, 1)}"


def generation(line):
    g = line["recorded"].get("generation")
    if not g:
        return None, None
    return g, g["envelope"]


def recorded(path):
    probe = {l["planned"]["id"]: l for l in lines(HERE.parent / "model_runtime" / "probe.jsonl.gz")}
    twin = {"step1-grammar": "step1", "whole-grammar": "library", "step1-fenced": "step1",
            "whole-fenced": "library"}
    print("| run | tokens (thinking) | stop | cycle: period, from | chosen again | build ms | began at | "
          "its program | the answer's lines | no grammar, the probe's: tokens, stop | its program |")
    print("|---|---|---|---|---:|---:|---:|---|---|---|---|")
    for l in lines(path):
        pid = l["planned"]["id"]
        g, e = generation(l)
        if g is None:
            print(f"| `{pid}` | failed: {l['recorded'].get('failure')} |")
            continue
        gen, gr = e["generation"], e["grammar"]
        cyc = loop(gen["tokens"]) if gen["stop"] != "end-of-turn" else None
        w = read(l)
        lines_ = "; ".join(answer_lines(w) if w else []) or "—"
        kind, _, item = pid.partition(".")
        th, _, item = item.partition(".")
        if kind in ("whole-grammar", "whole-fenced") and item == "first-experiment":
            twin_id = "first-experiment.off.zebra"
        elif kind in twin:
            twin_id = f"{twin[kind]}.{th}.{item}"
        else:
            twin_id = None
        tw = probe.get(twin_id) if twin_id else None
        if tw:
            tg, te = generation(tw)
            tcell = f"{len(te['generation']['tokens'])}, {te['generation']['stop']}"
            tw_read = verdict(read(tw))
        else:
            tcell, tw_read = "—", "—"
        print(f"| `{pid}` | {len(gen['tokens'])} ({gen['thinking']}) | {gen['stop']} | "
              f"{'—' if not cyc else f'{cyc[0]}, {cyc[1]}'} | {gr.get('chosen_again')} | "
              f"{gr.get('build_ms', 0):.1f} | {gr.get('began_at')} | {verdict(w)} | {lines_} | {tcell} | "
              f"{tw_read} |")


def divergence(path):
    """Each run beside the probe's run of the same request with no grammar: the generated tokens they share from
    the first, and where the answers part, the text each goes on with. Under greedy decoding with the chain's choice
    first, the grammar refused the probe's token where they part."""
    probe = {l["planned"]["id"]: l for l in lines(HERE.parent / "model_runtime" / "probe.jsonl.gz")}
    twin = {"step1-grammar": "step1", "whole-grammar": "library", "step1-fenced": "step1",
            "whole-fenced": "library"}
    print("| run | tokens shared with no grammar's, from the first | of | no grammar went on with | the grammar's went on with |")
    print("|---|---:|---:|---|---|")
    for l in lines(path):
        pid = l["planned"]["id"]
        kind, _, rest = pid.partition(".")
        th, _, item = rest.partition(".")
        tid = ("first-experiment.off.zebra" if item == "first-experiment"
               else f"{twin.get(kind, '?')}.{th}.{item}")
        tw = probe.get(tid)
        g, e = generation(l)
        if not tw or not g:
            continue
        tg, te = generation(tw)
        a, b = te["generation"]["tokens"], e["generation"]["tokens"]
        n = 0
        while n < min(len(a), len(b)) and a[n] == b[n]:
            n += 1
        ta, tb = tg["thinking"] + tg["answer"], g["thinking"] + g["answer"]
        c = 0
        while c < min(len(ta), len(tb)) and ta[c] == tb[c]:
            c += 1
        def show(t):
            return "`" + t[c:c + 40].replace("\n", "⏎").replace("`", "ˋ").replace("|", "¦") + "`" if c < len(t) else "(the end)"
        same = "**all**" if a == b else str(n)
        print(f"| `{pid}` | {same} | {len(b)} | {show(ta)} | {show(tb)} |")


def speeds():
    path = HERE / "speed" / "record.jsonl.gz"
    if not path.exists():
        return
    names = (HERE / "speed" / "order.txt").read_text().split()
    by = {}
    for name, r in zip(names, lines(path)):
        by.setdefault(name, []).append(r)
    print()
    print("| request | tokens | stop | tokens a second (median of 3) | the three | grammar built in, ms (median) | "
          "chosen again | the same tokens as every-token |")
    print("|---|---:|---|---:|---|---:|---:|---|")
    for name, rs in by.items():
        gens = [r["generation"] for r in rs if r.get("generation")]
        if len(gens) != len(rs):
            print(f"| `{name}` | a call failed |")
            continue
        tps = [len(g["envelope"]["generation"]["tokens"]) / (g["envelope"]["generation"]["generation_ms"] / 1e3)
               for g in gens]
        e = gens[0]["envelope"]
        gr = e.get("grammar") or {}
        builds = [g["envelope"]["grammar"].get("build_ms", 0) for g in gens if g["envelope"].get("grammar")]
        same = ""
        if name.endswith(".chosen-first"):
            other = by.get(name.replace(".chosen-first", ".every-token"))
            if other and other[0].get("generation"):
                same = "yes" if other[0]["generation"]["envelope"]["generation"]["tokens"] == \
                    e["generation"]["tokens"] else "**no**"
        print(f"| `{name}` | {len(e['generation']['tokens'])} | {e['generation']['stop']} | "
              f"{statistics.median(tps):.1f} | {', '.join(f'{t:.1f}' for t in tps)} | "
              f"{statistics.median(builds):.2f} | {gr.get('chosen_again', '—')} | {same} |"
              if builds else
              f"| `{name}` | {len(e['generation']['tokens'])} | {e['generation']['stop']} | "
              f"{statistics.median(tps):.1f} | {', '.join(f'{t:.1f}' for t in tps)} | — | — | |")


if __name__ == "__main__":
    if sys.argv[1] == "--speeds":
        speeds()
    else:
        recorded(sys.argv[1])
        print()
        divergence(sys.argv[1])
