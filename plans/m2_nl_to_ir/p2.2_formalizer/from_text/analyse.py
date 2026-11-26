#!/usr/bin/env python3
"""S2.2.5's tables, read off the runs replayed from their committed records (replayed/CONDITION/ITEM/, which
replay.sh writes) and the committed records themselves (runs/CONDITION/ITEM/: the program, its sidecar, the live
stderr).

Writes tables.md beside it: the Zebra puzzle's three texts; the word problems under each condition, a row a text
and the problems each text is of; each step of each run; the check's findings by step; the Zebra puzzle step by
step against the steps by hand (einaf/contract/zebra/), and the library's rules root fired in each, by name, read
off `ein saturate --events`. Every replayed program is held to the committed one, byte for byte.

    python3 analyse.py            # writes tables.md (EIN, by default the gate's ein, for the fired rules)
"""
import json
import os
import re
import subprocess
import sys
import tempfile
from collections import Counter

HERE = os.path.dirname(os.path.abspath(__file__))
ROOT = os.path.abspath(os.path.join(HERE, "../../../.."))
EIN = os.environ.get("EIN", os.path.join(ROOT, "ein.rs/target/gate/ein"))
# The sixteen word problems each text is of (examples/word_problems/README.md's table).
PROBLEMS = {
    "bob1": ["bob1"], "gap": ["gap", "gap_assume"], "gap_fact": ["gap_fact"], "contra": ["contra"],
    "unused": ["unused"], "bystander": ["bystander"], "before": ["before"],
    "kinds": ["kinds", "kinds_assume", "kinds_listed"], "kinds_full": ["kinds_full"], "more": ["more"],
    "boxes12": ["boxes12"], "boxes13": ["boxes13"], "pencils": ["pencils"],
}
# The answer each text's problem has in the directory's README, in words.
README = {
    "bob1": "3", "gap": "not stated: Alice's apples", "gap_fact": "Bob 3, Alice 6, together 9",
    "contra": "a contradiction", "unused": "3", "bystander": "9",
    "before": "7 (the theory answers no answer)", "kinds": "not stated: Alice's apples", "kinds_full": "8",
    "more": "Alice, by 7", "boxes12": "4", "boxes13": "4 or 5, by the reading of need",
    "pencils": "47 each, 21 left over",
}


def plan():
    rows = []
    for line in open(os.path.join(HERE, "plan.tsv")):
        if line.startswith("#") or not line.strip():
            continue
        f = line.rstrip("\n").split("\t")
        rows.append({"cond": f[0], "id": f[1], "text": f[2], "line": f[3], "gold": f[4],
                     "flags": f[5] if len(f) > 5 else ""})
    return rows


def load(cond, item):
    rep = os.path.join(HERE, "replayed", cond, item)
    run = os.path.join(HERE, "runs", cond, item)
    out = {"replayed": None, "answer": None, "stderr": "", "program_live": None, "program_again": None}
    if os.path.exists(os.path.join(rep, "run.json")):
        out["replayed"] = json.load(open(os.path.join(rep, "run.json")))
        out["answer"] = json.load(open(os.path.join(rep, "answer.json")))
    if os.path.exists(os.path.join(run, "stderr.txt")):
        out["stderr"] = open(os.path.join(run, "stderr.txt")).read()
    for key, d in (("program_live", run), ("program_again", rep)):
        p = os.path.join(d, "program.ein")
        if os.path.exists(p):
            out[key] = open(p).read()
    return out


def word(f):
    """The outcome of a feedback object over all of R."""
    if not f or not f.get("outcome"):
        return "—"
    return f["outcome"][0]["outcome"]


def single(rec):
    for f in rec["asked"]:
        if f["asked"]["ask"] == "single":
            return f
    return None


def esc(s):
    return s.replace("|", "\\|").replace("\n", " / ")


def answer_query(f):
    for q in (f or {}).get("queries") or []:
        if q.get("goal_text") is not None:
            return q
    return None


def classes(rec):
    """The answer query's classes, under the question that chose the answer."""
    asked = [f for f in rec["asked"] if f["asked"]["ask"] != "no-search"]
    q = answer_query(asked[-1] if asked else None)
    if q is None:
        return "no answer query" if asked and asked[-1].get("queries") is not None else "—"
    n = len(q["classes"])
    rows = sum(len(c["rows"]) for c in q["classes"])
    return f"{n} ({rows} rows{'' if q['exact'] else ', a lower bound'})"


def findings(judged):
    c = Counter(f["kind"] for f in (judged or {}).get("check", {}).get("findings", []))
    return ", ".join(f"`{k}` {n}" for k, n in sorted(c.items())) or "none"


def kinds_of(judged):
    return Counter(f["kind"] for f in (judged or {}).get("check", {}).get("findings", []))


def calls(rec):
    return [s for s in rec["run"]["steps"]]


def parsed(s):
    p = s.get("parsed")
    if not p:
        return "no output" if not s.get("failure") else "failed"
    return f"{p['forms']} forms" if p.get("forms") is not None else "does not parse"


def gen_s(rec):
    return sum((s.get("generation_ms") or 0) for s in rec["run"]["steps"]) / 1000


def top_forms(text):
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


def bare(form):
    """A form with its :source and its layout taken out."""
    f = re.sub(r'\s*:source\s+"(?:[^"\\]|\\.)*"', "", form)
    return " ".join(f.replace("(", " ( ").replace(")", " ) ").split())


def head(form):
    m = re.match(r"\(\s*([^\s()]+)", form)
    return m.group(1) if m else ""


def source(form):
    m = re.search(r':source\s+"((?:[^"\\]|\\.)*)"', form)
    return m.group(1) if m else None


LEVER = "\n(config :record-alternative-justifications false)\n"


def fired_by_rule(path):
    """The rules root fired, by name, off `ein saturate --events --events-level verbose`, the program with the
    lever that keeps one justification a fact appended, which changes no firing and spares a root that dies on a
    link several rules derive its minutes (E-113, E-126)."""
    with tempfile.TemporaryDirectory() as d:
        ev = os.path.join(d, "events.jsonl")
        prog = os.path.join(d, "program.ein")
        open(prog, "w").write(open(path).read() + LEVER)
        subprocess.run([EIN, "saturate", prog, "--events", ev, "--events-level", "verbose"],
                       stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL, timeout=600)
        c = Counter()
        if os.path.exists(ev):
            for line in open(ev):
                try:
                    v = json.loads(line)
                except ValueError:
                    continue
                if v.get("e") == "fire":
                    c[v.get("rule", "")] += 1
        return c


def library_rules(c, program):
    """The library's rules among those fired: every rule but the program's own, a library rule named short where
    the file lists it."""
    own = set(re.findall(r"\(rule\s+([^\s()]+)", program))
    return {r: n for r, n in c.items() if r not in own}


def main():
    rows = plan()
    out = ["# S2.2.5's tables", "",
           "Written by `analyse.py` from the runs replayed on the final code (`replay.sh`, `replayed/`) and the "
           "committed records (`runs/`). *Word problems, one shot*: prompts were developed on these problems, and "
           "this is no result. The answer is read by the stand-in reader of the answer's comparison "
           "(`einaf::answer::score`), against the gold short answer of `plan.tsv`.", ""]
    mismatched = []
    recs = {}
    for r in rows:
        d = load(r["cond"], r["id"])
        recs[(r["cond"], r["id"])] = d
        if d["program_live"] != d["program_again"]:
            mismatched.append(f"{r['cond']}/{r['id']}")
    out.append(f"**Replay**: {len(rows) - len(mismatched)} of {len(rows)} replayed programs are the committed "
               f"ones byte for byte" + (f"; not: {', '.join(mismatched)}" if mismatched else "") + ".")
    out.append("")
    # Against S2.2.4: the library condition's requests are S2.2.4's `steps` condition's until a step an output
    # the budget cut changes; each call's output, read from its fence, against S2.2.4's.
    same, differ, after_cut = 0, [], []
    for r in rows:
        if r["cond"] != "library":
            continue
        rec = recs[(r["cond"], r["id"])]["replayed"]
        old_dir = os.path.join(HERE, "..", "steps", "runs", "steps", r["id"])
        if rec is None or not os.path.isdir(old_dir):
            continue
        cut_before = False
        for s in rec["run"]["steps"]:
            old = os.path.join(old_dir, f"step{s['step']}.out.ein")
            mine = s.get("output")
            if os.path.exists(old) and mine is not None and (open(old).read() == mine or s.get("cut")):
                if s.get("cut"):
                    after_cut.append(f"{r['id']} {s['step']} (cut)")
                else:
                    same += 1
            elif cut_before:
                after_cut.append(f"{r['id']} {s['step']}")
            else:
                differ.append(f"{r['id']} {s['step']}")
            cut_before = cut_before or bool(s.get("cut"))
    out.append(f"**Against S2.2.4** (its `steps` condition, the same requests): {same} calls of the library "
               f"condition wrote S2.2.4's output byte for byte; read otherwise by the cut or after it: "
               f"{', '.join(after_cut) or 'none'}; otherwise different: {', '.join(differ) or 'none'}.")
    out.append("")

    # The Zebra puzzle.
    out += ["## The Zebra puzzle's three texts (condition library)", "",
            "| text | step 1 | step 2 | step 3 | `--single` | the check's findings | the answer printed | gold | read |",
            "|---|---|---|---|---|---|---|---|---|"]
    for r in rows:
        if not r["id"].startswith("zebra"):
            continue
        d = recs[(r["cond"], r["id"])]
        rec = d["replayed"]
        if rec is None:
            out.append(f"| {r['id']} | not run | | | | | | | |")
            continue
        cells = []
        for k in ["1", "2", "3"]:
            s = next((s for s in rec["run"]["steps"] if s["step"] == k), None)
            if s is None:
                cells.append("—")
                continue
            j = s.get("judged")
            at = word(j["feedback"][0]) if j and j.get("feedback") else "—"
            w = (j or {}).get("root", {}).get("writer") or "—"
            cells.append(f"{parsed(s)}, {s.get('stop') or s.get('failure')}; root {at}, H `{w}`")
        comp = rec["run"].get("complete")
        a = d["answer"]
        out.append(f"| {r['id']} | {' | '.join(cells)} | {word(single(rec))} | {findings(comp)} | "
                   f"{esc(a['text'])} | {esc(r['gold'])} | {a['scored']['verdict'] if a.get('scored') else '—'} |")
    out.append("")

    # The word problems.
    for cond, title in (("library", "the library alone, the main arms' condition"),
                        ("module-dir", "`--module-dir examples/word_problems/`, the widest condition of the "
                                       "library ablation, labelled so")):
        out += [f"## The word problems, one shot: {title}", "",
                "| text | its problems | parsed (steps) | loads | `--single` | the check's findings | the answer query's classes | "
                "the answer printed | the kernel's line | the README's answer | read |",
                "|---|---|---|---|---|---|---|---|---|---|---|"]
        tally = Counter()
        for r in rows:
            if r["cond"] != cond or r["id"].startswith("zebra"):
                continue
            d = recs[(cond, r["id"])]
            rec = d["replayed"]
            if rec is None:
                out.append(f"| {r['id']} | not run |")
                continue
            steps = ", ".join(f"{s['step']}: {parsed(s)}" + (" (cut)" if s.get("cut") else "")
                              for s in rec["run"]["steps"])
            sw = word(single(rec))
            loads = "yes" if rec["run"].get("program") and sw not in ("invalid", "—") else "no"
            a = d["answer"]
            verdict = a["scored"]["verdict"] if a.get("scored") else "—"
            tally[verdict] += 1
            tally["loads"] += loads == "yes"
            out.append(f"| {r['id']} | {', '.join(PROBLEMS[r['id']])} | {steps} | {loads} | {sw} | "
                       f"{findings(rec['run'].get('complete'))} | {classes(rec)} | {esc(a['text'])} | "
                       f"{esc(a['kernel']) or '—'} | {README[r['id']]} | {verdict} |")
        out += ["", f"Of 13 texts: {tally['loads']} load; read **right** {tally['right']}, wrong "
                    f"{tally['wrong']}, no answer {tally['no-answer']}.", ""]

    # Each step.
    out += ["## Each call", "",
            "| condition | text | step | prompt tokens | generated | stop | s | parsed | cut | at root | H | findings |",
            "|---|---|---|---:|---:|---|---:|---|---|---|---|---|"]
    by_step = Counter()
    for r in rows:
        rec = recs[(r["cond"], r["id"])]["replayed"]
        if rec is None:
            continue
        for s in rec["run"]["steps"]:
            j = s.get("judged")
            at = word(j["feedback"][0]) if j and j.get("feedback") else "—"
            cut = s.get("cut")
            cuts = f"kept {cut['kept']}, repeats {cut['repeats']}" if cut else ""
            out.append(f"| {r['cond']} | {r['id']} | {s['step']} | {s.get('prompt_tokens') or 0} | "
                       f"{s.get('generated_tokens') or 0} | {s.get('stop') or s.get('failure') or '—'} | "
                       f"{(s.get('generation_ms') or 0) / 1000:.1f} | {parsed(s)} | {cuts} | {at} | "
                       f"{(j or {}).get('root', {}).get('writer') or '—'} | {findings(j)} |")
            for k, n in kinds_of(j).items():
                by_step[(r["cond"], s["step"], k)] += n
    out.append("")
    out += ["## The check's findings, by condition, step and kind", "",
            "| condition | step | kind | count |", "|---|---|---|---:|"]
    for (c, s, k), n in sorted(by_step.items()):
        out.append(f"| {c} | {s} | `{k}` | {n} |")
    out.append("")

    # The Zebra puzzle against the steps by hand.
    rec = recs[("library", "zebra")]["replayed"]
    if rec is not None:
        hand = {k: open(os.path.join(ROOT, f"einaf/contract/zebra/step{k}.ein")).read() for k in "123"}
        out += ["## The Zebra puzzle, step by step against the steps by hand", "",
                "Forms compared with their `:source` and their layout taken out; a step's part is what it wrote "
                "(the model's output; the hand's step less the forms of the step before).", "",
                "| step | the model's forms | of them the hand's | the hand's forms | facts by sentence: the same / the model's alone / the hand's alone | declared relations: both / the model's alone / the hand's alone | selections, the model's |",
                "|---|---:|---:|---:|---|---|---|"]
        prev = set()
        for k in "123":
            s = next((s for s in rec["run"]["steps"] if s["step"] == k), None)
            mine = top_forms((s or {}).get("output") or "")
            hand_all = top_forms(hand[k])
            theirs = [f for f in hand_all if bare(f) not in prev]
            prev |= {bare(f) for f in hand_all}
            mb, tb = {bare(f) for f in mine}, {bare(f) for f in theirs}

            def facts(fs):
                return {(source(f), bare(f)) for f in fs if source(f)}
            mf, tf = facts(mine), facts(theirs)
            same = len(mf & tf)

            def decl(fs):
                return {f.split()[1] for f in fs if head(f) == "relation" and len(f.split()) > 1}
            md, td = decl(mine), decl(theirs)
            sel = sorted({head(f) for f in mine if head(f) in (
                "includes", "symmetric", "slot-partition", "slot-spatial", "converse", "compose", "top", "meet",
                "transitive", "total-owed", "functional", "injective", "bijective")})
            out.append(f"| {k} | {len(mine)} | {len(mb & tb)} | {len(theirs)} | {same} / {len(mf - tf)} / "
                       f"{len(tf - mf)} | {len(md & td)} / {', '.join(sorted(md - td)) or '—'} / "
                       f"{', '.join(sorted(td - md)) or '—'} | {', '.join(sel) or '—'} |")
        out.append("")
        # The library's rules root fired: the model's program against the reference.
        with tempfile.TemporaryDirectory() as d:
            p = os.path.join(d, "zebra.ein")
            open(p, "w").write(rec["run"]["program"] or "")
            model = library_rules(fired_by_rule(p), rec["run"]["program"] or "")
        ref_path = os.path.join(ROOT, "einaf/contract/zebra/step3.ein")
        ref = library_rules(fired_by_rule(ref_path), open(ref_path).read())
        out += ["## The library's rules root fired, the model's Zebra program against the reference", "",
                "Read off `ein saturate PROGRAM --events F --events-level verbose`, the `fire` lines by rule, each "
                "program with `(config :record-alternative-justifications false)` appended (no firing changes; "
                "E-126); the reference is the steps by hand's step 3, `einaf/contract/zebra/step3.ein`.", "",
                "| rule | the model's | the reference's |", "|---|---:|---:|"]
        for rule in sorted(set(model) | set(ref)):
            out.append(f"| `{rule}` | {model.get(rule, 0)} | {ref.get(rule, 0)} |")
        out.append("")
    open(os.path.join(HERE, "tables.md"), "w").write("\n".join(out) + "\n")
    print(f"tables.md: {len(rows)} runs, {len(mismatched)} replayed programs not the committed ones",
          file=sys.stderr)


if __name__ == "__main__":
    main()
