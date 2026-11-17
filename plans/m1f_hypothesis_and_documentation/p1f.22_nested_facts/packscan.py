#!/usr/bin/env python3
"""Read every rule and hrule of the .ein files under the given roots and say,
by text, which pack and which unpack.

    python3 packscan.py REPO stdlib examples tests

A rule *packs by shape* when a conjunct of its :assert nests a form (other
than under a leading `not`); it *packs what it matched* when such a nested
form is, as text, a premise of its :match; it *unpacks* when a conjunct of
its :assert (or the form under its `not`) is, as text, a form nested inside a
premise of its :match; it *negates a variable* when a conjunct is `(not ?x)`.
Text equality after whitespace is normalised; macros are not expanded.
"""
import os
import re
import sys


def tokens(text):
    text = re.sub(r";[^\n]*", "", text)
    return re.findall(r'"(?:[^"\\]|\\.)*"|[()]|[^\s()"]+', text)


def parse(toks):
    out, stack = [], []
    cur = out
    for t in toks:
        if t == "(":
            new = []
            cur.append(new)
            stack.append(cur)
            cur = new
        elif t == ")":
            if not stack:
                continue
            cur = stack.pop()
        else:
            cur.append(t)
    return out


def text(x):
    return x if isinstance(x, str) else "(" + " ".join(text(y) for y in x) + ")"


CONN = {"and", "or", "absent", "forall", "unknown", "eq", "neq"}


def premises(m):
    """The relation premises of a :match, flattened through the connectives."""
    if not isinstance(m, list) or not m:
        return []
    if m[0] in ("and", "or"):
        return [p for x in m[1:] for p in premises(x)]
    if m[0] in ("absent", "forall", "unknown", "eq", "neq"):
        return []
    if m[0] == "not":
        return premises(m[1]) if len(m) > 1 else []
    return [m]


def conjuncts(a):
    if not isinstance(a, list) or not a:
        return []
    if a[0] == "and":
        return [c for x in a[1:] for c in conjuncts(x)]
    return [a]


def nested_forms(f):
    """Every form under an argument of f, at any depth."""
    out = []
    for x in f[1:]:
        if isinstance(x, list):
            out.append(x)
            out += nested_forms(x)
    return out


def main():
    repo = sys.argv[1]
    rows = {}
    for root in sys.argv[2:]:
        n = dict(files=0, rules=0, pack=0, pack_matched=0, unpack=0, neg_var=0, nested_match=0)
        names = dict(pack_matched=[], unpack=[], neg_var=[])
        for d, _, fs in os.walk(os.path.join(repo, root)):
            for fn in sorted(fs):
                if not fn.endswith(".ein"):
                    continue
                n["files"] += 1
                path = os.path.join(d, fn)
                for form in parse(tokens(open(path, errors="replace").read())):
                    if not isinstance(form, list) or not form or form[0] not in ("rule", "hrule"):
                        continue
                    n["rules"] += 1
                    kw = {form[i]: form[i + 1] for i in range(len(form) - 1) if isinstance(form[i], str) and form[i].startswith(":")}
                    prem = premises(kw.get(":match", []))
                    prem_text = {text(p) for p in prem}
                    inner = {text(x) for p in prem for x in nested_forms(p)}
                    if inner:
                        n["nested_match"] += 1
                    concl = conjuncts(kw.get(":assert", []))
                    where = os.path.relpath(path, repo) + ":" + (form[1] if isinstance(form[1], str) else "?")
                    packs = packs_matched = unpacks = negs = False
                    for c in concl:
                        body = c[1] if c[0] == "not" and len(c) == 2 else c
                        if c[0] == "not" and len(c) == 2 and isinstance(c[1], str) and c[1].startswith("?"):
                            negs = True
                            continue
                        if not isinstance(body, list) or body[0] in CONN or body[0] in ("false", "open"):
                            continue
                        nf = nested_forms(body)
                        if nf:
                            packs = True
                        if any(text(x) in prem_text for x in nf):
                            packs_matched = True
                        if text(body) in inner:
                            unpacks = True
                    n["pack"] += packs
                    n["pack_matched"] += packs_matched
                    n["unpack"] += unpacks
                    n["neg_var"] += negs
                    if packs_matched:
                        names["pack_matched"].append(where)
                    if unpacks:
                        names["unpack"].append(where)
                    if negs:
                        names["neg_var"].append(where)
        rows[root] = (n, names)
    for root, (n, names) in rows.items():
        print(root, n)
        for k, v in names.items():
            for w in v[:12]:
                print("   ", k, w)
            if len(v) > 12:
                print("   ", k, "… and", len(v) - 12, "more")


main()
