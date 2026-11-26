#!/usr/bin/env python3
"""S2.2.4's tables, read off the runs' records (runs/CONDITION/ITEM/run.json) and the steps by hand.

Writes tables.md beside it: per condition, per item, per step, what the stage's acceptance asks; and the
check's findings by step and kind. Reads the records alone, and the texts and the steps by hand for the
sentences and the comparison; runs nothing.

    python3 analyse.py            # writes tables.md
"""
import gzip
import json
import os
import re
import sys
from collections import Counter, defaultdict

HERE = os.path.dirname(os.path.abspath(__file__))
ROOT = os.path.abspath(os.path.join(HERE, "../../../.."))
CONDITIONS = ["steps", "whole", "steps-free", "whole-free"]
ITEMS = ["zebra", "youngest", "seating", "bob1", "gap", "gap_fact", "contra", "unused", "bystander",
         "before", "kinds", "kinds_full", "more", "boxes12", "boxes13", "pencils"]
# The texts the steps by hand are of, and their directory under einaf/contract/.
HAND = {"zebra": "zebra", "youngest": "youngest", "seating": "seating", "bob1": "apples", "gap": "together",
        "more": "more"}
# The answers the word problems' README states, as numbers; the Zebra's and the others' as names.
GOLD = {
    "zebra": ["Norwegian", "Japanese"], "youngest": ["Cy"], "seating": ["Bob"],
    "bob1": ["3"], "gap": [], "gap_fact": ["3", "6", "9"], "contra": [], "unused": ["3"],
    "bystander": ["9"], "before": ["7"], "kinds": [], "kinds_full": ["8"], "more": ["Alice", "7"],
    "boxes12": ["4"], "boxes13": ["5"], "pencils": ["48", "10"],
}


def sentences(text):
    out, cur = [], ""
    for i, c in enumerate(text):
        cur += c
        nxt = text[i + 1] if i + 1 < len(text) else None
        if c in ".?!" and (nxt is None or nxt.isspace()):
            s = " ".join(cur.split())
            if s:
                out.append(s)
            cur = ""
    s = " ".join(cur.split())
    if s:
        out.append(s)
    return out


def top_forms(text):
    """The top-level forms of a text, as written."""
    out, depth, start, i = [], 0, 0, 0
    while i < len(text):
        c = text[i]
        if c == ";":
            while i < len(text) and text[i] != "\n":
                i += 1
            continue
        if c == '"':
            i += 1
            while i < len(text) and text[i] != '"':
                i += 2 if text[i] == "\\" else 1
        elif c == "(":
            if depth == 0:
                start = i
            depth += 1
        elif c == ")" and depth > 0:
            depth -= 1
            if depth == 0:
                out.append(text[start:i + 1])
        i += 1
    return out


def head(form):
    m = re.match(r"\(\s*([^\s()]+)", form)
    return m.group(1) if m else ""


def source(form):
    m = re.search(r':source\s+"((?:[^"\\]|\\.)*)"', form)
    return m.group(1) if m else None


def norm(form):
    """A form without its :source and its layout, for comparing step 1 with the hand's."""
    f = re.sub(r':source\s+"(?:[^"\\]|\\.)*"', "", form)
    return " ".join(f.replace("(", " ( ").replace(")", " ) ").split())


def load(cond, item):
    """A run's record, run.json or run.json.gz."""
    p = os.path.join(HERE, "runs", cond, item, "run.json")
    if os.path.exists(p):
        with open(p) as f:
            return json.load(f)
    if os.path.exists(p + ".gz"):
        with gzip.open(p + ".gz", "rt") as f:
            return json.load(f)
    return None


def outcome(fb):
    if not fb or not fb.get("outcome"):
        return "fault" if fb and fb.get("fault") else "—"
    return fb["outcome"][0]["outcome"].replace("-", " ")


def answer_lines(fb):
    out = []
    for q in fb.get("queries") or []:
        if q.get("goal_text") is None:
            continue
        for c in q["classes"]:
            out.extend(c["lines"])
        break
    return out


def step(run, name):
    for s in run["steps"]:
        if s["step"] == name:
            return s
    return None


def step1_measures(run, text, item):
    """What the acceptance asks of step 1."""
    s1 = step(run, "1")
    out = s1.get("output") or ""
    forms = top_forms(out)
    first_rel = next((i for i, f in enumerate(forms) if head(f) == "relation"), len(forms))
    statements = [f for f in forms[:first_rel] if head(f) not in ("query",)]
    queries = [f for f in forms[:first_rel] if head(f) == "query"]
    others = [f for f in forms if head(f) in ("import", "rule", "solve", "config", "trace", "macro")]
    sents = sentences(text)
    used = {source(f) for f in forms if source(f)}
    no_fact = [s for s in sents if s not in used and not s.endswith("?")]
    unfound = [u for u in used if u not in sents]
    rels = [f for f in forms if head(f) == "relation"]
    no_why = [f for f in rels if ":why" not in f]
    stmt_rels = {head(f) if head(f) != "not" else head(f[4:].strip()) for f in statements}
    goal_rels = set()
    answer = [q for q in queries if ":goal-text" in q]
    for q in answer[:1]:
        g = re.search(r":goal\s+(.*?)\s+:goal-text", q, re.S)
        if g:
            goal_rels = {r for r in re.findall(r"\(\s*([^\s()?]+)", g.group(1))
                         if r not in ("and", "or", "not", "absent", "eq", "neq", "n")}
    m = {
        "forms": len(forms), "statements": len(statements), "queries": len(queries),
        "answer_queries": len(answer), "other_forms": len(others), "declarations": len(rels),
        "memberships": len(forms) - first_rel - len(rels), "no_why": len(no_why),
        "sentences": len(sents), "sentences_no_fact": no_fact, "unfound_sources": unfound,
        "goal_relations": sorted(goal_rels), "goal_in_statements": sorted(goal_rels & stmt_rels),
    }
    if item in HAND:
        hand = open(os.path.join(ROOT, "einaf/contract", HAND[item], "step1.ein")).read()
        hf = top_forms(hand)
        hfirst = next((i for i, f in enumerate(hf) if head(f) == "relation"), len(hf))
        hs = {norm(f) for f in hf[:hfirst] if head(f) != "query"}
        ms = {norm(f) for f in statements}
        hk = {norm(f) for f in hf[hfirst:] if head(f) != "relation"}
        mk = {norm(f) for f in forms[first_rel:] if head(f) != "relation"}
        m["hand_statements"] = len(hs)
        m["statements_as_hand"] = len(hs & ms)
        m["hand_memberships"] = len(hk)
        m["memberships_as_hand"] = len(hk & mk)
    return m


def numeral(text, i=0):
    """Read a numeral `(n X D)` or `z` at text[i:], its value and where it ends."""
    if text.startswith("z", i):
        return 0, i + 1
    m = re.match(r"\(n\s+", text[i:])
    if not m:
        return None, i
    j = i + m.end()
    hi, j = numeral(text, j)
    if hi is None:
        return None, i
    m2 = re.match(r"\s+(\d)\)", text[j:])
    if not m2:
        return None, i
    return hi * 10 + int(m2.group(1)), j + m2.end()


def decimal(line):
    """A line with each numeral written as a decimal."""
    out, i = "", 0
    while i < len(line):
        if line.startswith("(n ", i):
            v, j = numeral(line, i)
            if v is not None:
                out += str(v)
                i = j
                continue
        out += line[i]
        i += 1
    return out


def gold_reading(item, lines):
    """What the lines say beside the gold answer: a rough reading, not S2.5.3's answer rule. A gold of
    numbers is read against every number the lines write, a gold of names against their words."""
    gold = GOLD[item]
    if not lines:
        return "no line"
    text = " ".join(decimal(l) for l in lines)
    words = set(re.findall(r"[A-Za-z0-9_]+", text))
    if not gold:
        return "a line, where the text gives no answer"
    numbers = {w for w in words if w.isdigit()}
    gold_numbers = {g for g in gold if g.isdigit()}
    names_ok = all(g in words for g in gold if not g.isdigit())
    if not names_ok or not gold_numbers <= numbers:
        return "not the gold"
    if numbers - gold_numbers:
        return "the gold, and other values"
    return "the gold"


def findings_by(judged):
    c = Counter()
    for f in (judged or {}).get("check", {}).get("findings", []):
        c[f["kind"]] += 1
    return c


def counts(runs, steps_mode):
    """A condition in one table: how many runs, parses, loads, outcomes and answers."""
    n = len(runs)
    stopped = Counter()
    outcomes = Counter()
    loads = 0
    gold = Counter()
    gen = []
    for item, r in runs.items():
        if r.get("stopped"):
            stopped[r["stopped"]] += 1
        c = r.get("complete")
        if c:
            o = outcome(c["feedback"][1]) if len(c["feedback"]) > 1 else outcome(c["feedback"][0])
            outcomes[o] += 1
            if o != "invalid":
                loads += 1
            lines = answer_lines(c["feedback"][1]) if len(c["feedback"]) > 1 else []
            gold[gold_reading(item, lines)] += 1
        gen.append(sum(s.get("generated_tokens") or 0 for s in r["steps"]))
    out = ["| runs | stopped | a complete program | it loads | --single | the answer (a rough reading) | tokens generated, a run |",
           "|---:|---|---:|---:|---|---|---|"]
    out.append(f"| {n} | {', '.join(f'{k} {v}' for k, v in sorted(stopped.items())) or 'none'} | "
               f"{sum(outcomes.values())} | {loads} | {', '.join(f'{k} {v}' for k, v in sorted(outcomes.items()))} | "
               f"{', '.join(f'{k} {v}' for k, v in sorted(gold.items()))} | "
               f"{min(gen) if gen else 0} to {max(gen) if gen else 0}, {sum(gen)} in all |")
    return out + [""]


def zebra_detail(rows):
    """The Zebra puzzle: each step against the steps by hand, and the program against zebra3.1.ein."""
    out = ["## The Zebra puzzle against the steps by hand and zebra3.1.ein", "",
           "Forms compared by their text without `:source` and layout; relations by name.", "",
           "| condition | step | forms | as the hand's step | the hand's step's forms | relations it writes | the hand's |",
           "|---|---|---:|---:|---:|---|---|"]
    hand = {n: open(os.path.join(ROOT, "einaf/contract/zebra", f"step{n}.ein")).read() for n in (1, 2, 3)}
    hand_parts = {}
    prev = set()
    for n in (1, 2, 3):
        forms = top_forms(hand[n])
        cur = {norm(f) for f in forms}
        hand_parts[n] = [f for f in forms if n == 1 or norm(f) not in prev] if n > 1 else forms
        prev = cur
    # Step 2's part by hand, less step 1's statements; step 3's, less step 2's.
    h1 = top_forms(hand[1])
    first = next(i for i, f in enumerate(h1) if head(f) == "relation")
    stmts = {norm(f) for f in h1[:first]}
    hand_parts[2] = [f for f in top_forms(hand[2]) if norm(f) not in stmts]
    hand_parts[3] = [f for f in top_forms(hand[3]) if norm(f) not in {norm(g) for g in top_forms(hand[2])}]
    target = open(os.path.join(ROOT, "examples/zebra3.1.ein")).read()
    for cond in CONDITIONS:
        r = rows[cond].get("zebra")
        if not r:
            continue
        for s in r["steps"]:
            o = s.get("output") or ""
            forms = top_forms(o)
            n = {"1": 1, "2": 2, "2b": 3, "3": 3, "whole": 0}[s["step"]]
            ref = hand_parts[n] if n else top_forms(target)
            mine = {norm(f) for f in forms}
            theirs = {norm(f) for f in ref}
            rels_m = sorted({head(f) for f in forms if head(f) not in ("relation", "query", "import")})
            rels_h = sorted({head(f) for f in ref if head(f) not in ("relation", "query", "import")})
            out.append(f"| {cond} | {s['step']} | {len(forms)} | {len(mine & theirs)} | {len(theirs)} | "
                       f"{' '.join(rels_m)[:200]} | {' '.join(rels_h)[:200]} |")
        if r.get("program"):
            p = top_forms(r["program"])
            t = top_forms(target)
            pm, tm = {norm(f) for f in p}, {norm(f) for f in t}
            out.append(f"| {cond} | the program against zebra3.1.ein | {len(p)} | {len(pm & tm)} | {len(tm)} | | |")
    return out + [""]


def main():
    rows = defaultdict(dict)
    for cond in CONDITIONS:
        for item in ITEMS:
            r = load(cond, item)
            if r:
                rows[cond][item] = r
    texts = {i: open(os.path.join(HERE, "texts", i + ".txt")).read() for i in ITEMS}
    md = ["# S2.2.4's runs, tabled", "", "Written by `analyse.py` from `runs/*/*/run.json`; nothing here is typed by hand.", ""]
    all_findings = defaultdict(Counter)
    for cond in CONDITIONS:
        if not rows[cond]:
            continue
        md += [f"## {cond}", ""]
        steps_mode = not cond.startswith("whole")
        md += counts(rows[cond], steps_mode)
        md += ["| item | step | prompt tok | gen tok | stop | parses | at root | H from (size) | findings at the step |",
               "|---|---|---:|---:|---|---|---|---|---|"]
        for item in ITEMS:
            r = rows[cond].get(item)
            if not r:
                continue
            for s in r["steps"]:
                j = s.get("judged") or {}
                fb = (j.get("feedback") or [None])[0]
                p = s.get("parsed") or {}
                parses = "yes" if p.get("forms") is not None and (s.get("output") or "").strip() else (
                    "no: " + (p.get("findings") or [{"message": s.get("failure") or "no output"}])[0]["message"].split("\n")[0][:60].replace("|", "/")
                    if p else "no output")
                fk = findings_by(j)
                for k, n in fk.items():
                    all_findings[(cond, s["step"])][k] += n
                root = j.get("root") or {}
                md.append(
                    f"| {item} | {s['step']} | {s.get('prompt_tokens') or 0} | {s.get('generated_tokens') or 0} | "
                    f"{s.get('stop') or s.get('failure') or '—'} | {parses} | {outcome(fb) if fb else '—'} | "
                    f"{root.get('writer') or '—'} ({root.get('candidates', 0)}) | "
                    f"{', '.join(f'{k} {n}' for k, n in sorted(fk.items())) or 'none'} |")
            if r.get("stopped"):
                md.append(f"| {item} | | | | | stopped: {r['stopped']} | | | |")
        md += [""]
        # The complete programs.
        md += ["| item | --no-search | --single | the answer | gold | findings | categories (src/gen/rel/rule/assum/frame) |",
               "|---|---|---|---|---|---|---|"]
        for item in ITEMS:
            r = rows[cond].get(item)
            if not r:
                continue
            c = r.get("complete")
            if not c:
                md.append(f"| {item} | no program | | | | | |")
                continue
            fbs = c["feedback"]
            lines = answer_lines(fbs[1]) if len(fbs) > 1 else []
            cats = Counter(row["category"] for row in r.get("categories", []))
            order = ["source-derived", "generic-theory", "generated-relation", "generated-rule",
                     "generated-assumption", "frame"]
            fk = findings_by(c)
            md.append(f"| {item} | {outcome(fbs[0])} | {outcome(fbs[1]) if len(fbs) > 1 else '—'} | "
                      f"{(' / '.join(decimal(l) for l in lines)[:140] + ' (' + gold_reading(item, lines) + ')') if lines else 'no line'} | {', '.join(GOLD[item]) or 'none'} | "
                      f"{', '.join(f'{k} {n}' for k, n in sorted(fk.items())) or 'none'} | "
                      f"{'/'.join(str(cats.get(o, 0)) for o in order)} |")
        md += [""]
        if steps_mode:
            md += ["Step 1, what the acceptance asks:", "",
                   "| item | forms | statements | answer queries | other forms | declarations (no :why) | memberships | "
                   "sentences with no fact | sources no sentence | goal's relations (in statements) | as the hand's: statements, memberships |",
                   "|---|---:|---:|---:|---:|---|---:|---|---:|---|---|"]
            for item in ITEMS:
                r = rows[cond].get(item)
                if not r or not step(r, "1") or not (step(r, "1").get("output") or "").strip():
                    continue
                m = step1_measures(r, texts[item], item)
                hand = (f"{m['statements_as_hand']} of {m['hand_statements']}, {m['memberships_as_hand']} of {m['hand_memberships']}"
                        if "hand_statements" in m else "—")
                md.append(f"| {item} | {m['forms']} | {m['statements']} | {m['answer_queries']} | {m['other_forms']} | "
                          f"{m['declarations']} ({m['no_why']}) | {m['memberships']} | {len(m['sentences_no_fact'])} of {m['sentences']} | "
                          f"{len(m['unfound_sources'])} | {', '.join(m['goal_relations'])} ({len(m['goal_in_statements'])}) | {hand} |")
            md += ["", "Steps 2 and 3, what the acceptance asks:", "",
                   "| item | step 2: imports | selecting facts | not in catalogue / not listed / inactive | markers | ontology kept / dropped / added / redeclared | rewritten | step 3: rules (markers) | mechanism stated | writer |",
                   "|---|---|---:|---|---:|---|---:|---|---|---|"]
            for item in ITEMS:
                r = rows[cond].get(item)
                if not r or not step(r, "2"):
                    continue
                s2, s3 = step(r, "2"), step(r, "3")
                o2 = s2.get("output") or ""
                f2 = top_forms(o2)
                imports = [re.match(r"\(import\s+([^\s)]+)", f).group(1) for f in f2 if head(f) == "import"]
                sel = [f for f in f2 if head(f) not in ("relation", "import") and ":source" not in f
                       and head(f) in {"transitive", "symmetric", "includes", "converse", "compose", "top", "meet",
                                       "functional", "injective", "total-owed", "slot-partition", "slot-spatial",
                                       "bijective", "bijection-hierarchy", "surjective-owed"}]
                fk = findings_by(s2.get("judged"))
                oc = r.get("ontology_change") or {}
                o3 = (s3 or {}).get("output") or ""
                rules3 = len([f for f in top_forms(o3) if head(f) == "rule"])
                marks3 = o3.count(";; missing:")
                cm = (r.get("complete") or {}).get("check", {}).get("mechanism", {})
                md.append(
                    f"| {item} | {' '.join(imports) or 'none'} | {len(sel)} | "
                    f"{fk.get('selection-not-in-catalogue', 0)} / {fk.get('selection-not-listed', 0)} / {fk.get('selection-inactive', 0)} | "
                    f"{len(s2.get('markers') or [])} | {oc.get('kept', 0)} / {len(oc.get('dropped', []))} / {len(oc.get('added', []))} / {len(oc.get('redeclared', []))} | "
                    f"{fk.get('rewritten', 0)} | {rules3} ({marks3}) | {' and '.join(cm.get('stated', [])) or 'none'} | {cm.get('writer') or '—'} |")
            md += [""]
    md += zebra_detail(rows)
    md += ["## The check's findings, by condition, step and kind", "",
           "| condition | step | findings |", "|---|---|---|"]
    for (cond, st), c in sorted(all_findings.items()):
        md.append(f"| {cond} | {st} | {', '.join(f'{k} {n}' for k, n in sorted(c.items()))} |")
    out = os.path.join(HERE, "tables.md")
    with open(out, "w") as f:
        f.write("\n".join(md) + "\n")
    print(out)


if __name__ == "__main__":
    sys.exit(main())
