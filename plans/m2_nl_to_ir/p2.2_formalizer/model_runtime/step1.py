#!/usr/bin/env python3
"""Step 1 with the model against the hand-written step 1: `python3 step1.py probe.jsonl[.gz]`.

For each step-1 run, the model's program (its fenced block, else the whole answer) is
read form by form and set against `einaf/contract/<text>/step1.ein`: per sentence of
the text, the facts that quote it in each (`:source` dropped, white space folded);
the names the statements use in each; the memberships each writes (a fact of one or
two names whose relation is a kind's: `instance`, `person`, `is-a`, …); the
declarations' relations and counts; and the query's goal. Reads the record and the
repository's files; no kernel. Standard library only.
"""

import gzip
import json
import os
import re
import sys

ROOT = os.path.abspath(os.path.join(os.path.dirname(__file__), *[".."] * 4))
DECLARATORS = {"relation", "rule", "query", "solve", "config", "macro", "import", "trace"}


def tokens(text):
    i, n = 0, len(text)
    while i < n:
        c = text[i]
        if c in " \t\r\n":
            i += 1
        elif c == ";":
            while i < n and text[i] != "\n":
                i += 1
        elif text.startswith("#|", i):
            j = text.find("|#", i + 2)
            i = n if j < 0 else j + 2
        elif c in "()":
            yield c
            i += 1
        elif c == '"':
            j = i + 1
            while j < n and text[j] != '"':
                j += 2 if text[j] == "\\" else 1
            yield ("str", text[i + 1:j])
            i = j + 1
        else:
            j = i
            while j < n and text[j] not in ' \t\r\n();"':
                j += 1
            yield text[i:j]
            i = j


def forms(text):
    stack, out = [[]], None
    try:
        for t in tokens(text):
            if t == "(":
                stack.append([])
            elif t == ")":
                if len(stack) == 1:
                    return None
                f = stack.pop()
                stack[-1].append(f)
            else:
                stack[-1].append(t)
        if len(stack) != 1:
            return None
        out = [f for f in stack[0] if isinstance(f, list)]
    except Exception:  # noqa: BLE001 - a text that is no s-expression reads as none
        return None
    return out


def show(f):
    if isinstance(f, tuple):
        return '"' + f[1] + '"'
    if isinstance(f, list):
        return "(" + " ".join(show(x) for x in f) + ")"
    return f


def split_kw(f):
    body, kw, i = [], {}, 0
    while i < len(f):
        x = f[i]
        if isinstance(x, str) and x.startswith(":") and i + 1 < len(f):
            kw[x] = f[i + 1]
            i += 2
        else:
            body.append(x)
            i += 1
    return body, kw


def read_program(text):
    fs = forms(text)
    if fs is None:
        return None
    p = {"facts": [], "relations": {}, "queries": [], "other": []}
    for f in fs:
        head = f[0] if f and isinstance(f[0], str) else None
        body, kw = split_kw(f)
        src = kw.get(":source")
        src = re.sub(r"\s+", " ", src[1]).strip() if isinstance(src, tuple) else None
        if head == "relation":
            labels = [x for x in body[2:] if isinstance(x, str)]
            p["relations"].setdefault(body[1] if len(body) > 1 else "?", []).append(len(labels))
        elif head == "query":
            p["queries"].append(show(kw.get(":goal")))
        elif head in DECLARATORS:
            p["other"].append(head)
        else:
            p["facts"].append((show(body), src, body))
    return p


def names(body):
    out = []
    for x in body:
        if isinstance(x, list):
            out.extend(names(x[1:]))
        elif isinstance(x, str) and not x.startswith("?"):
            out.append(x)
    return out


def sentences(text):
    return [s.strip() for s in re.split(r"(?<=[.?!])\s+", re.sub(r"\s+", " ", text.strip())) if s.strip()]


def lines(path):
    op = gzip.open if path.endswith(".gz") else open
    with op(path, "rt") as f:
        for l in f:
            if l.strip():
                yield json.loads(l)


def main():
    for r in lines(sys.argv[1]):
        p = r["planned"]
        if p["condition"] != "step1":
            continue
        g = r["recorded"].get("generation")
        print(f"## {p['id']}\n")
        if not g:
            print(f"failed: {r['recorded'].get('failure')}\n")
            continue
        judged = {j["reading"]: j for j in r["judged"]}
        text = (judged.get("fenced") or {}).get("text") or g["answer"]
        model = read_program(text)
        hand = read_program(open(os.path.join(ROOT, p["reference"])).read())
        for k in ("whole", "fenced"):
            j = judged.get(k) or {}
            word = ("none" if j.get("text") is None else "parses" if (j.get("parsed") or {}).get("forms") is not None
                    else "parse error")
            if j.get("loaded") is not None:
                word = "loads" if j["loaded"] else "load error: " + ", ".join(j["kinds"])
            fb = j.get("feedback") or []
            outs = [f"{f['asked']['question']} {f['outcome'][0]['outcome']}" for f in fb if f.get("outcome")]
            print(f"- {k}: {word}; {'; '.join(outs)}")
        if model is None:
            print("\nthe answer is no s-expression\n")
            continue
        print(f"\n- forms: model {len(model['facts'])} facts, {sum(len(v) for v in model['relations'].values())} "
              f"declarations, {len(model['queries'])} queries, other {model['other']}; hand {len(hand['facts'])} "
              f"facts, {sum(len(v) for v in hand['relations'].values())} declarations")
        print(f"- the query: model {model['queries']}; hand {hand['queries']}")
        undeclared = sorted({b[0] + "/" + str(len(b) - 1) for _, _, b in model["facts"] if isinstance(b[0], str)
                             and b[0] != "not" and len(b) - 1 not in model["relations"].get(b[0], [])})
        print(f"- facts of no declaration at their count: {undeclared}")
        print("\n| sentence | hand | model | same |\n|---|---|---|---|")
        same_total = 0
        for s in sentences(p["text"]):
            h = sorted(c for c, src, _ in hand["facts"] if src == s)
            m = sorted(c for c, src, _ in model["facts"] if src == s)
            same = len(set(h) & set(m))
            same_total += same
            print(f"| {s} | {'<br>'.join(h)} | {'<br>'.join(m)} | {same}/{len(h)} |")
        unsourced = [c for c, src, _ in model["facts"] if src is None or src not in sentences(p["text"])]
        print(f"\n- model facts quoting no sentence of the text: {len(unsourced)} {unsourced[:12]}")
        hn = set(n for _, _, b in hand["facts"] for n in names(b[1:]))
        mn = set(n for _, _, b in model["facts"] for n in names(b[1:]))
        print(f"- names: both {sorted(hn & mn)}; hand only {sorted(hn - mn)}; model only {sorted(mn - hn)}")
        print(f"- facts the same as the hand's, by sentence: {same_total}\n")


if __name__ == "__main__":
    main()
