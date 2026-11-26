#!/usr/bin/env python3
"""The probe's tables, from its record: `python3 report.py probe.jsonl[.gz]`.

Per condition and thinking, and per reading (whole, fenced): the runs, how many
parsed, how many loaded, how many the bound stopped, the outcomes under
`--single`; the kinds of error with what would have prevented each; and a line a
run. Reads the record alone: no model, no kernel. Standard library only.
"""

import collections
import gzip
import json
import re
import sys


def lines(path):
    op = gzip.open if path.endswith(".gz") else open
    with op(path, "rt") as f:
        for l in f:
            if l.strip():
                yield json.loads(l)


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


def parse_what(message):
    """What a parse error is, by its line: a fence, a keyword a form does not take, a string where none
    goes, a parenthesis not closed or one too many, a list or a variable where the form takes something
    else, `_`, or another token."""
    first = message.split("\n")[0]
    what = first.split(": ", 1)[1] if ": " in first else first
    if "```" in what:
        return "a fence line"
    if what.startswith("unexpected `:"):
        return "a keyword the form does not take"
    if what.startswith('unexpected `"') or "a string" in message:
        return "a string where none goes"
    if "end of input" in what or what.startswith("unexpected `)`"):
        return "a parenthesis not closed, or one too many"
    if what.startswith("unexpected `(`"):
        return "a list where the form takes a name or a keyword"
    if ": a variable" in what:
        return "a variable where the form takes a name or a list"
    if "`_`" in what:
        return "`_`"
    return "another token: " + what[:60]


def loop(toks, maxp=4000):
    """The period of the cycle a generation ends in, and the generated token it began at; None where its
    last three periods are not one cycle."""
    n = len(toks)
    for p in range(1, min(maxp, n // 3)):
        if toks[n - p:] == toks[n - 2 * p:n - p] == toks[n - 3 * p:n - 2 * p]:
            i = n - p
            while i > 0 and toks[i - 1] == toks[i - 1 + p]:
                i -= 1
            return p, i
    return None


def sentences(text):
    return [s.strip() for s in re.split(r"(?<=[.?!])\s+", re.sub(r"\s+", " ", text.strip())) if s.strip()]


def unfound(j, text):
    """The `:source`s of a reading that are no sentence of the item's text."""
    srcs = re.findall(r':source\s+"((?:[^"\\]|\\.)*)"', j.get("text") or "")
    sents = sentences(text)
    return [s for s in srcs if re.sub(r"\s+", " ", s).strip() not in sents]


def stopped(j):
    for f in j.get("feedback") or []:
        r = f.get("readings") or {}
        s = r.get("stopped")
        if s and s.get("why") in ("clock", "steps", "budget", "limit"):
            return True
        for d in f.get("diagnostics") or []:
            if d["kind"] in ("deadline", "partial"):
                return True
    return False


def main():
    rows = list(lines(sys.argv[1]))
    groups = collections.OrderedDict()
    for r in rows:
        p = r["planned"]
        groups.setdefault((p["condition"], "on" if p["thinking"] else "off"), []).append(r)

    print("| condition | thinking | runs | stopped by the budget | reading | parses | loads | `--single` one / several / other | not decided |")
    print("|---|---|---:|---:|---|---:|---:|---|---:|")
    kinds = collections.Counter()
    parse_whats = collections.Counter()
    prevent = {}
    for (cond, th), rs in groups.items():
        budget = sum(1 for r in rs if (r["recorded"].get("generation") or {}).get("envelope", {})
                     .get("generation", {}).get("stop") == "max-tokens")
        for reading in ("whole", "fenced"):
            js = [next((j for j in r["judged"] if j["reading"] == reading), None) for r in rs]
            present = [j for j in js if j and j.get("text") is not None]
            parses = [j for j in present if (j.get("parsed") or {}).get("forms") is not None]
            loads = [j for j in parses if j.get("loaded")]
            singles = collections.Counter(outcome(j, 1) for j in loads)
            nd = sum(1 for j in loads if stopped(j))
            other = sum(v for k, v in singles.items() if k not in ("one", "several"))
            print(f"| {cond} | {th} | {len(rs)} | {budget} | {reading} | {len(parses)} | {len(loads)} | "
                  f"{singles.get('one', 0)} / {singles.get('several', 0)} / {other} | {nd} |")
            for j in present:
                for k, pv in zip(j.get("kinds") or [], j.get("prevention") or []):
                    kinds[(cond, th, reading, k)] += 1
                    prevent[k] = pv
                if (j.get("parsed") or {}).get("forms") is None:
                    for w in sorted({parse_what(f["message"]) for f in j.get("findings") or []}):
                        parse_whats[(cond, th, reading, w)] += 1
    print()
    print("| condition | thinking | reading | kind | runs | prevented by |")
    print("|---|---|---|---|---:|---|")
    for (cond, th, reading, k), n in kinds.items():
        print(f"| {cond} | {th} | {reading} | `{k}` | {n} | {prevent[k]} |")
    print()
    print("| condition | thinking | reading | what the parse refused | runs |")
    print("|---|---|---|---|---:|")
    for (cond, th, reading, w), n in parse_whats.items():
        print(f"| {cond} | {th} | {reading} | {w} | {n} |")
    print()
    print("| run | tokens | thinking | stop | loop: period, from | s | whole | fenced | `--single` (fenced) | `:source` no sentence | the answer's lines |")
    print("|---|---:|---:|---|---|---:|---|---|---|---:|---|")
    for r in rows:
        p = r["planned"]
        g = r["recorded"].get("generation")
        if not g:
            print(f"| {p['id']} | | | failed: {r['recorded'].get('failure')} | | | | | |")
            continue
        e = g["envelope"]["generation"]
        cells = []
        for reading in ("whole", "fenced"):
            j = next((j for j in r["judged"] if j["reading"] == reading), None)
            if not j or j.get("text") is None:
                cells.append("none")
            elif (j.get("parsed") or {}).get("forms") is None:
                cells.append("parse: " + ", ".join(j["kinds"]))
            elif not j.get("loaded"):
                cells.append("load: " + ", ".join(j["kinds"]))
            else:
                cells.append("loads")
        fj = next((j for j in r["judged"] if j["reading"] == "fenced"), {})
        single = outcome(fj, 1) if fj.get("loaded") else ""
        if single and stopped(fj):
            single += " (stopped)"
        lns = "; ".join(answer_lines(fj))[:120] if fj.get("loaded") else ""
        lp = loop(e["tokens"]) if e["stop"] != "end-of-turn" else None
        lp = f"{lp[0]}, {lp[1]}" if lp else ""
        uf = len(unfound(fj, p["text"])) if fj.get("loaded") else ""
        print(f"| {p['id']} | {len(e['tokens'])} | {e['thinking']} | {e['stop']} | {lp} | "
              f"{e['generation_ms'] / 1e3:.0f} | {cells[0]} | {cells[1]} | {single} | {uf} | {lns} |")


if __name__ == "__main__":
    main()
