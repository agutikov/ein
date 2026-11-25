#!/usr/bin/env python3
"""The six Zebra encodings measured on every axis the analysis page reads.

    python3 measure.py kernel     # every encoding under every question: results/kernel.json
    python3 measure.py damage     # the damaged variants, written to damaged/ and run: results/damage.json
    python3 measure.py order      # each encoding in both form orders, written to order/ and compared: results/order.json
    python3 measure.py order-texts  # the read-outs, traces and event streams of the orders, path taken off: results/order-texts.json
    python3 measure.py text       # the statements against the sentences: results/text.json
    utils/bench_env.sh --cores PT python3 measure.py times   # wall clock, best and median of 7: results/times.json
    python3 measure.py tables     # the markdown tables of README.md, from results/

The binaries are `$EIN_BIN` (default `ein.rs/target/release/ein`) and `$EINAF_BIN`
(default `einaf/target/release/einaf`), both from the repository root.
"""
from __future__ import annotations

import json
import os
import re
import statistics
import subprocess
import sys
import tempfile
import time
from pathlib import Path

HERE = Path(__file__).resolve().parent
REPO = HERE.parents[4]
EIN = os.environ.get("EIN_BIN", str(REPO / "ein.rs/target/release/ein"))
EINAF = os.environ.get("EINAF_BIN", str(REPO / "einaf/target/release/einaf"))
RESULTS = HERE / "results"
sys.path.insert(0, str(HERE))
import forms as F  # noqa: E402

ENCODINGS = {
    "zebra": "examples/zebra.ein",
    "zebra2": "examples/zebra2.ein",
    "zebra3.1": "examples/zebra3.1.ein",
    "zebra3.2": "examples/zebra3.2.ein",
    "zebra3.3": "examples/zebra3.3.ein",
    "zebra3.4": "examples/zebra3.4.ein",
}

QUESTIONS = {
    "describe": [],
    "single": ["--single"],
    "maximal": ["--maximal"],
    "minimal": ["--minimal"],
    "maximal-single": ["--maximal", "--single"],
    "minimal-single": ["--minimal", "--single"],
    "count": ["--count"],
    "no-search": ["--no-search"],
}


def run(argv: list[str], timeout: int = 600) -> subprocess.CompletedProcess:
    return subprocess.run(argv, cwd=REPO, capture_output=True, text=True, timeout=timeout)


def feedback(path: str, q: list[str]) -> dict:
    p = run([EINAF, "feedback", path, *q])
    if p.returncode != 0 or not p.stdout.strip():
        return {"error": (p.stderr or p.stdout).strip()[:2000], "exit": p.returncode}
    return json.loads(p.stdout)


def summary(path: str, q: list[str]) -> tuple[dict, str, str, int]:
    with tempfile.TemporaryDirectory() as d:
        js = Path(d) / "s.json"
        p = run([EIN, "solve", path, *q, "--json-summary", str(js)])
        data = json.loads(js.read_text()) if js.exists() else {}
    return data, p.stdout, p.stderr, p.returncode


def hyps(path: str) -> dict:
    p = run([EIN, "show", "hyps", path])
    m = re.search(r"H, from `([^`]+)`: (\d+) candidates?", p.stderr)
    return {"writer": m.group(1) if m else None, "n": int(m.group(2)) if m else None,
            "lines": [l for l in p.stdout.splitlines() if l.startswith("(")],
            "notes": [l for l in p.stderr.splitlines() if l.strip()]}


def answer_lines(stdout: str) -> list[str]:
    return [l.strip() for l in stdout.splitlines() if "“" in l]


def brief(fb: dict) -> dict:
    """What a feedback object says, cut to what the tables read."""
    if "error" in fb:
        return fb
    out = {
        "outcome": [(o["part"], o["outcome"]) for o in fb.get("outcome", [])],
        "why": [o.get("why") for o in fb.get("outcome", [])],
        "readings": fb.get("readings"),
        "work": {k: fb.get("work", {}).get(k) for k in ("saturations", "facts", "layers")},
        "diagnostics": fb.get("diagnostics"),
        "flagged": fb.get("flagged"),
        "core": fb.get("core"),
        "owed_root": len((fb.get("owed") or {}).get("root") or []),
        "classes": [[c.get("rows") for c in q.get("classes", [])] for q in fb.get("queries", [])],
        "fault": fb.get("fault"),
    }
    return out


# ── kernel ─────────────────────────────────────────────────────────────

def kernel() -> None:
    res: dict = {}
    for name, path in ENCODINGS.items():
        r: dict = {"hyps": hyps(path), "questions": {}}
        r["hyps"].pop("lines")
        for qn, q in QUESTIONS.items():
            fb = feedback(path, q)
            s, out, err, code = summary(path, q)
            r["questions"][qn] = {
                "feedback": brief(fb),
                "exit": code,
                "universe": ((s.get("region") or {}).get("universe") or {}).get("candidates"),
                "root_facts": (s.get("root") or {}).get("facts"),
                "root_owes": (s.get("root") or {}).get("owes"),
                "work": s.get("work"),
                "monotone": (s.get("monotonicity") or {}).get("monotone"),
                "answer": answer_lines(out),
                "notes": [l for l in err.splitlines() if l.startswith(("note:", "warning:", "WARNING"))],
            }
            print(name, qn, r["questions"][qn]["feedback"].get("outcome"), r["questions"][qn]["work"], flush=True)
        res[name] = r
    RESULTS.mkdir(exist_ok=True)
    (RESULTS / "kernel.json").write_text(json.dumps(res, indent=1, ensure_ascii=False) + "\n")


# ── damage ─────────────────────────────────────────────────────────────

INJECT = {
    "zebra": '(co-located Coffee House-1 :source "Coffee is drunk in the first house.")',
    "zebra2": '(drink-loc Coffee House-1 :source "Coffee is drunk in the first house.")',
    "zebra3.1": '(is-drunk-in Coffee First :source "Coffee is drunk in the first house.")',
    "zebra3.2": '(same (the Coffee (is drunk in) House) (the First House) :source "Coffee is drunk in the first house.")',
    "zebra3.3": '(is-drunk-in Coffee (first House) :source "Coffee is drunk in the first house.")',
    "zebra3.4": '(is-drunk-in Coffee First :source "Coffee is drunk in the first house.")',
}

SENTENCE_15 = {
    "zebra": r"\(next-to\s+Norwegian Blue[^\n]*\n",
    "zebra2": r"\(adjacent-via next-to\s+nation-loc Norwegian\s+color-loc Blue[^\n]*\n",
    "zebra3.1": r"\(lives-next-to Norwegian Blue[^\n]*\n",
    "zebra3.2": r"\(next-to \(the Norwegian \(lives in\) House\) \(the Blue House\)\n[^\n]*\n",
    "zebra3.3": r"\(lives-next-to Norwegian \(blue House\)[^\n]*\n",
    "zebra3.4": r"\(house-next-to Norwegian lives-in Blue color-of\n[^\n]*\n",
}

SWAP_6 = {
    "zebra": ("(right-of   Green Ivory", "(right-of   Ivory Green"),
    "zebra2": ("(adjacent-via right-of color-loc Ivory color-loc Green", "(adjacent-via right-of color-loc Green color-loc Ivory"),
    "zebra3.1": ("(is-immediately-to-the-right-of Green Ivory", "(is-immediately-to-the-right-of Ivory Green"),
    "zebra3.2": ("(immediately-to-the-right-of (the Green House) (the Ivory House)", "(immediately-to-the-right-of (the Ivory House) (the Green House)"),
    "zebra3.3": ("(is-immediately-to-the-right-of (green House) (ivory House)", "(is-immediately-to-the-right-of (ivory House) (green House)"),
    "zebra3.4": ("(house-immediately-right-of Green color-of Ivory color-of", "(house-immediately-right-of Ivory color-of Green color-of"),
}

# The fact that brings the drinks into the theory, left out.
DRINKS_LINK = {
    "zebra": "(guess co-located Drink       House)",
    "zebra2": "(bijective drink-loc)",
    "zebra3.1": "(includes drinks        same-house)",
    "zebra3.2": "(the-thing-described same)",
    "zebra3.3": "(includes drinks        same-house)",
    "zebra3.4": "(bijective is-drunk-in)",
}

WATER = {
    "zebra": "(instance Water Drink)",
    "zebra2": "(is-a Water Drink)",
    "zebra3.1": "(instance Water Drink)",
    "zebra3.2": "(instance Water Drink)",
    "zebra3.3": "(instance Water Drink)",
    "zebra3.4": "(is-a Water Drink)",
}

DAMAGES = {
    "drop-15": "sentence (15) dropped, *The Norwegian lives next to the blue house*",
    "add-coffee-first": "a sentence added that contradicts the others, *Coffee is drunk in the first house*",
    "swap-6": "the two arguments of sentence (6) swapped: *the ivory house is immediately to the right of the green house*",
    "misspell-symmetric": "one activating fact misspelt: `(symmetric next-to)` written `(symetric next-to)`",
    "drop-symmetric": "one activating fact left out: `(symmetric next-to)`",
    "drop-drinks-link": "the activating fact that brings the drinks into the theory left out",
    "drop-water": "one membership left out: water is a drink",
}


def damaged(name: str, src: str, dmg: str) -> str:
    if dmg == "drop-15":
        out, n = re.subn(SENTENCE_15[name], "", src, count=1)
    elif dmg == "add-coffee-first":
        i = src.index("(query")
        out, n = src[:i] + INJECT[name] + "\n\n" + src[i:], 1
    elif dmg == "swap-6":
        a, b = SWAP_6[name]
        pat = re.compile("^" + re.escape(a), re.M)
        out, n = pat.subn(b, src, count=1)
        n = len(pat.findall(src))
    elif dmg in ("misspell-symmetric", "drop-symmetric"):
        pat = re.compile(r"^\(symmetric\s+next-to\)", re.M)
        new = "(symetric next-to)" if dmg == "misspell-symmetric" else ""
        out, n = pat.subn(new, src, count=1)
    elif dmg == "drop-drinks-link":
        pat = re.compile("^" + re.escape(DRINKS_LINK[name]), re.M)
        n = len(pat.findall(src))
        out = pat.sub("", src, count=1)
    elif dmg == "drop-water":
        n = src.count(WATER[name])
        out = src.replace(WATER[name], "", 1)
    else:
        raise KeyError(dmg)
    if n != 1:
        raise SystemExit(f"{name} {dmg}: {n} matches")
    return out


def damage() -> None:
    ddir = HERE / "damaged"
    ddir.mkdir(exist_ok=True)
    res: dict = {}
    for name, path in ENCODINGS.items():
        src = (REPO / path).read_text()
        res[name] = {}
        for dmg in DAMAGES:
            text = damaged(name, src, dmg)
            out = ddir / f"{name}.{dmg}.ein"
            out.write_text(
                f";;; {name}, damaged: {DAMAGES[dmg]}. Written by measure.py.\n" + text)
            rel = str(out.relative_to(REPO))
            fb = feedback(rel, [])
            fs = feedback(rel, ["--single"])
            s, sout, err, code = summary(rel, [])
            res[name][dmg] = {
                "file": rel,
                "describe": brief(fb),
                "single": brief(fs),
                "hyps": {k: v for k, v in hyps(rel).items() if k != "lines"},
                "answer": answer_lines(sout),
                "notes": [l for l in err.splitlines() if l.strip()],
                "exit": code,
            }
            print(name, dmg, res[name][dmg]["describe"].get("outcome"), res[name][dmg]["single"].get("outcome"), flush=True)
    RESULTS.mkdir(exist_ok=True)
    (RESULTS / "damage.json").write_text(json.dumps(res, indent=1, ensure_ascii=False) + "\n")


# ── order ──────────────────────────────────────────────────────────────

def order() -> None:
    odir = HERE / "order"
    odir.mkdir(exist_ok=True)
    res: dict = {}
    for name, path in ENCODINGS.items():
        header, fs = F.split((REPO / path).read_text())
        F.classify(fs)
        files = {"as-written": path}
        for o, seq in (("synthesis", F.SYNTHESIS), ("conventional", F.CONVENTIONAL)):
            p = odir / f"{name}.{o}.ein"
            p.write_text(F.write([f";;; {name}, its forms in {o} order. Written by measure.py; the forms are the file's, unchanged.", ";;;"] + header, fs, seq))
            files[o] = str(p.relative_to(REPO))
        # the contradictory twin in each order, for the core
        for o in ("synthesis", "conventional"):
            src = (REPO / files[o]).read_text()
            p = odir / f"{name}.{o}.add-coffee-first.ein"
            p.write_text(damaged(name, src, "add-coffee-first"))
            files[f"{o}+bad"] = str(p.relative_to(REPO))
        r: dict = {"files": files, "runs": {}}
        for label, f in files.items():
            rr: dict = {}
            for qn, q in QUESTIONS.items():
                fb = brief(feedback(f, q))
                s, sout, err, code = summary(f, q)
                rr[qn] = {
                    "outcome": fb.get("outcome"),
                    "readings": fb.get("readings"),
                    "classes": fb.get("classes"),
                    "core": fb.get("core"),
                    "work": fb.get("work"),
                    "answer": answer_lines(sout),
                    "states": [st.get("facts") for st in ((s.get("region") or {}).get("states") or [])],
                    "readout": sout,
                    "error": fb.get("error"),
                }
            h = hyps(f)
            rr["hyps"] = {"writer": h["writer"], "n": h["n"], "lines": h["lines"]}
            with tempfile.TemporaryDirectory() as d:
                t = Path(d) / "t.md"
                ev = Path(d) / "e.jsonl"
                try:
                    run([EIN, "solve", f, "--single", "--trace", str(t)], timeout=120)
                    rr["trace"] = t.read_text() if t.exists() else ""
                except subprocess.TimeoutExpired:
                    rr["trace"] = "timeout"
                try:
                    run([EIN, "solve", f, "--single", "--events", str(ev)], timeout=120)
                    rr["events"] = ev.read_text() if ev.exists() else ""
                except subprocess.TimeoutExpired:
                    rr["events"] = "timeout"
            r["runs"][label] = rr
            print(name, label, rr["describe"]["outcome"], rr["describe"]["work"], flush=True)
        res[name] = r
    RESULTS.mkdir(exist_ok=True)
    # The traces and event streams are large: kept as digests and line counts.
    import hashlib
    slim = json.loads(json.dumps(res))
    for name in slim:
        for label, rr in slim[name]["runs"].items():
            for k in ("trace", "events"):
                t = rr[k]
                rr[k] = {"lines": t.count("\n"), "sha256": hashlib.sha256(t.encode()).hexdigest()[:16]}
            for qn in QUESTIONS:
                t = rr[qn]["readout"]
                rr[qn]["readout"] = {"lines": t.count("\n"), "sha256": hashlib.sha256(t.encode()).hexdigest()[:16]}
    for name in slim:
        for label, rr in slim[name]["runs"].items():
            rr["hyps"].pop("lines", None)
            for qn in QUESTIONS:
                rr[qn].pop("states", None)
    (RESULTS / "order.json").write_text(json.dumps(slim, indent=1, ensure_ascii=False) + "\n")
    # The comparison: what is equal between the orders and what is not.
    cmp: dict = {}
    for name in res:
        runs = res[name]["runs"]
        c: dict = {}
        for a, b in (("as-written", "synthesis"), ("as-written", "conventional"), ("synthesis", "conventional")):
            ra, rb = runs[a], runs[b]
            same = {}
            for qn in QUESTIONS:
                same[qn] = {
                    "outcome": ra[qn]["outcome"] == rb[qn]["outcome"],
                    "readings": ra[qn]["readings"] == rb[qn]["readings"],
                    "classes": ra[qn]["classes"] == rb[qn]["classes"],
                    "answer": ra[qn]["answer"] == rb[qn]["answer"],
                    "states": ra[qn]["states"] == rb[qn]["states"],
                    "readout": ra[qn]["readout"] == rb[qn]["readout"],
                    "work": ra[qn]["work"] == rb[qn]["work"],
                }
            same["hyps"] = ra["hyps"] == rb["hyps"]
            same["trace"] = ra["trace"] == rb["trace"]
            same["events"] = ra["events"] == rb["events"]
            c[f"{a}~{b}"] = same
        a, b = runs["synthesis+bad"]["describe"], runs["conventional+bad"]["describe"]
        c["bad"] = {"outcome": a["outcome"] == b["outcome"], "core": a["core"] == b["core"],
                    "synthesis_core": a["core"], "conventional_core": b["core"],
                    "readout": a["readout"] == b["readout"]}
        cmp[name] = c
    (RESULTS / "order-compare.json").write_text(json.dumps(cmp, indent=1, ensure_ascii=False) + "\n")


def order_texts() -> None:
    """The texts a run prints in each order, the file's path taken off, compared by `diff`."""
    odir = HERE / "order"
    res: dict = {}
    for name, path in ENCODINGS.items():
        files = {"as-written": path, "synthesis": f"{odir.relative_to(REPO)}/{name}.synthesis.ein",
                 "conventional": f"{odir.relative_to(REPO)}/{name}.conventional.ein",
                 "synthesis+bad": f"{odir.relative_to(REPO)}/{name}.synthesis.add-coffee-first.ein",
                 "conventional+bad": f"{odir.relative_to(REPO)}/{name}.conventional.add-coffee-first.ein"}
        with tempfile.TemporaryDirectory() as t:
            t = Path(t)
            for k, f in files.items():
                (t / k).mkdir()
                for qn, q in (("describe", []), ("single", ["--single"])):
                    p = run([EIN, "solve", f, *q])
                    (t / k / f"readout.{qn}").write_text(p.stdout.replace(f, "FILE"))
                run([EIN, "solve", f, "--single", "--trace", str(t / k / "trace"), "--events", str(t / k / "events")])
                for x in ("trace", "events"):
                    q = t / k / x
                    q.write_text(q.read_text().replace(f, "FILE") if q.exists() else "")
            r: dict = {}
            for a, b in (("as-written", "synthesis"), ("as-written", "conventional"),
                         ("synthesis", "conventional"), ("synthesis+bad", "conventional+bad")):
                rr: dict = {}
                for x in ("readout.describe", "readout.single", "trace", "events"):
                    d = subprocess.run(["diff", str(t / a / x), str(t / b / x)], capture_output=True, text=True).stdout
                    changed = [l for l in d.splitlines() if l[:1] in "<>"]
                    n = (t / a / x).read_text().count("\n")
                    rr[x] = "same" if not changed else {"lines": n, "differing": len(changed),
                                                        "first": changed[:6]}
                r[f"{a}~{b}"] = rr
            res[name] = r
            print(name, json.dumps({k: {x: (v if v == "same" else f"{v['differing']}/{v['lines']}") for x, v in rr.items()} for k, rr in r.items()}), flush=True)
    RESULTS.mkdir(exist_ok=True)
    (RESULTS / "order-texts.json").write_text(json.dumps(res, indent=1, ensure_ascii=False) + "\n")


# ── text ───────────────────────────────────────────────────────────────

STOP = set("the a an in of to is are who with where next lives house there five immediately right "
           "smoker smokes man kept drunk smoked owns drinks".split())
SENTENCES = [l.strip() for l in (REPO / "nl2smt/puzzles/zebra.txt").read_text().splitlines() if l.strip()]


def words(s: str) -> list[str]:
    return [w.lower() for w in re.findall(r"[A-Za-z]+", s)]


def stem(w: str) -> str:
    w = w.lower()
    for suf in ("ies", "es", "s"):
        if w.endswith(suf) and len(w) > 4:
            return w[: -len(suf)] if suf != "ies" else w[:-3] + "y"
    return w


def same_word(a: str, b: str) -> bool:
    """One word in two forms: equal, or sharing a prefix of all but two letters of the
    shorter, and of four letters at least (`houses`/`House`, `smokes`/`smoke`)."""
    if a == b:
        return True
    if min(len(a), len(b)) <= 3:
        return False
    n = 0
    while n < min(len(a), len(b)) and a[n] == b[n]:
        n += 1
    return n >= max(4, min(len(a), len(b)) - 2)


def text() -> None:
    res: dict = {}
    for name, path in ENCODINGS.items():
        header, fs = F.split((REPO / path).read_text())
        F.classify(fs)
        stmts = [f for f in fs if f.kind == "statement"]
        per: list[dict] = []
        for k, sent in enumerate(SENTENCES[:15], start=1):
            if name in ("zebra", "zebra2"):
                mine = [f for f in stmts if f'"condition ({k})"' in f.text]
            else:
                mine = [f for f in stmts if f'"{sent}"' in f.text]
            body = " ".join(re.sub(r':source\s+"[^"]*"', "", f.text) for f in mine)
            toks = {w.lower() for w in re.findall(r"[A-Za-z]+", body)}
            sw = [w for w in words(sent)]
            content = [w for w in sw if w not in {"the", "a", "an", "there", "are", "is"}]
            kept = [w for w in content if any(same_word(w, t) for t in toks)]
            per.append({"k": k, "sentence": sent, "facts": len(mine),
                        "forms": [re.sub(r"\s+\)", ")", " ".join(re.sub(r':source\s+"[^"]*"', "", f.text).split())) for f in mine],
                        "words": len(content), "kept": kept, "lost": [w for w in content if w not in kept]})
        q = [f for f in fs if f.kind == "query"]
        counts: dict = {}
        for f in fs:
            counts[f.kind] = counts.get(f.kind, 0) + 1
        res[name] = {"sentences": per, "query": [" ".join(f.text.split()) for f in q], "kinds": counts}
    RESULTS.mkdir(exist_ok=True)
    (RESULTS / "text.json").write_text(json.dumps(res, indent=1, ensure_ascii=False) + "\n")


# ── times ──────────────────────────────────────────────────────────────

def times() -> None:
    n = 7
    res: dict = {}
    for name, path in ENCODINGS.items():
        res[name] = {}
        for qn, q in QUESTIONS.items():
            ts = []
            for _ in range(n + 1):
                t0 = time.perf_counter()
                run([EIN, "solve", path, *q])
                ts.append(time.perf_counter() - t0)
            ts = ts[1:]
            res[name][qn] = {"best_ms": round(min(ts) * 1000, 1), "median_ms": round(statistics.median(ts) * 1000, 1), "n": n}
            print(name, qn, res[name][qn], flush=True)
    RESULTS.mkdir(exist_ok=True)
    (RESULTS / "times.json").write_text(json.dumps(res, indent=1) + "\n")


if __name__ == "__main__":
    {"kernel": kernel, "damage": damage, "order": order, "order-texts": order_texts, "text": text,
     "times": times}[sys.argv[1]]()
