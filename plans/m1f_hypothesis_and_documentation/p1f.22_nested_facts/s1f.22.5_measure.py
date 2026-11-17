#!/usr/bin/env python3
"""S1f.22.5, task 1: is a state a function of the program and the commitment?

    python3 s1f.22.5_measure.py [--bin EIN] [--out DIR] [--cap N] [--jobs J] [--only SUBSTR] [--slow] [FILE…]

Run from the repository's root. For every corpus entry the round trip reads per
commit (neither `slow` nor `heavy_dump`; `--slow` adds them), it runs
`ein solve FILE --dump-answer DIR` and, for each state file written (`models/`,
`owing/`, `root.ein`; at most `--cap` a program, spread evenly over its files),
compares three things by value (named FILEs instead of the corpus, if given):

  S  the state the run dumped: the state file's facts, each with the origin its
     comment names (`source "…"`, `rule r`, `hypothesis N`, or none). Its
     `written` facts are root's: the file leaves them out, and only the loader
     states them, once, at load (Q-M1f.275, Q-M1f.279)
  B  the rebuild: the state file's program part with the commitment its header
     names appended as facts, as today's language can say it, saturated:
     `ein saturate --dump`
  L  today's reload: the state file itself, `ein saturate --dump`

and reports which facts differ, which `written` facts B and L hold beyond the
program part's own (root's), which fact's origin differs (a hypothesis or a
rule in S, a given in B or L; one rule in S and another in B), and whether B or
L derives (false).

Per state it also takes today's save -> load -> save: `ein solve F --no-search
--dump-answer D2` on the state file F, and compares `D2/root.ein` with F byte
for byte, and the two loads' `ein saturate --dump` sections. Per program it
runs the same `--dump-answer` again, with `-j 4` and with `--maximal`, and
compares the state files: byte for byte, and by their body (the lines after the
`;;;` header), the maximal ones matched by their facts.

Per program it also runs `ein solve` on the program and on each state file
(exit 1 where a claim fails), and sizes the proposed file
(`s1f.22.5_propose.py`) beside today's. Writes `results.json` (per program and
state) and `summary.json` into `--out` (default `build/s5/measure`), every
dump beside them, and prints the summary.
"""
import argparse
import concurrent.futures as cf
import json
import os
import re
import shutil
import subprocess
import sys
import importlib.util
import tomllib

_spec = importlib.util.spec_from_file_location(
    "propose", os.path.join(os.path.dirname(os.path.abspath(__file__)), "s1f.22.5_propose.py")
)
propose = importlib.util.module_from_spec(_spec)
_spec.loader.exec_module(propose)

STATE_MARK = ";; ── the state"
TIMEOUT = 120


def run(args, cwd=None):
    try:
        p = subprocess.run(args, capture_output=True, text=True, timeout=TIMEOUT, cwd=cwd)
        return p.returncode, p.stdout, p.stderr
    except subprocess.TimeoutExpired:
        return None, "", "timeout"


def sexpr_end(s, i):
    """The index just past the balanced form starting at s[i] == '('."""
    depth = 0
    j = i
    n = len(s)
    while j < n:
        c = s[j]
        if c == '"':
            j += 1
            while j < n and s[j] != '"':
                if s[j] == "\\":
                    j += 1
                j += 1
        elif c == "(":
            depth += 1
        elif c == ")":
            depth -= 1
            if depth == 0:
                return j + 1
        j += 1
    return n


def split_set(text):
    """`∅` or `{(a b), (c (d e))}` -> ['(a b)', '(c (d e))']."""
    text = text.strip()
    if text in ("∅", ""):
        return []
    assert text.startswith("{") and text.endswith("}"), text
    body = text[1:-1]
    out, i = [], 0
    while i < len(body):
        if body[i] == "(":
            j = sexpr_end(body, i)
            out.append(body[i:j])
            i = j
        else:
            i += 1
    return out


ANNOT = re.compile(r'\s+:(rule|source|hypothesis|rejected)\s+("(?:[^"\\]|\\.)*"|\S+?)\)$')


def strip_annotation(fact):
    """`(p a :rule r)` -> (`(p a)`, ('rule', 'r'))."""
    m = ANNOT.search(fact)
    if not m:
        return fact, None
    return fact[: m.start()] + ")", (m.group(1), m.group(2))


def parse_dump(stdout):
    """`ein saturate --dump`'s KB section: {fact: origin}, the section's text,
    origin one of ('given',), ('source',), ('rule', r), ('hyp', n)."""
    lines = stdout.splitlines()
    try:
        start = next(i for i, l in enumerate(lines) if l.startswith("=  SATURATED KB DUMP"))
    except StopIteration:
        return None, None
    section = lines[start:]
    facts = {}
    group = None
    for l in section:
        m = re.match(r";; ── (BACKGROUND|GIVEN|DERIVED) \(", l)
        if m:
            group = m.group(1)
            continue
        if l.startswith(";; Query"):
            group = None
        if group and l.startswith("  ("):
            fact, ann = strip_annotation(l[2:])
            if ann is None:
                origin = ("given",)
            elif ann[0] == "source":
                origin = ("source",)
            elif ann[0] == "rule":
                origin = ("rule", ann[1])
            elif ann[0] == "hypothesis":
                origin = ("hyp", ann[1])
            else:
                origin = (ann[0], ann[1])
            facts[fact] = origin
    return facts, "\n".join(section)


def head_of(fact):
    m = re.match(r"\(\s*([^\s()]+)", fact)
    return m.group(1) if m else ""


def parse_state_file(text):
    """A state file: (header lines, program part, {fact: origin} of the state)."""
    head, rest = [], []
    lines = text.split("\n")
    i = 0
    while i < len(lines) and lines[i].startswith(";;;"):
        head.append(lines[i])
        i += 1
    k = text.find(STATE_MARK)
    program = text[:k] if k >= 0 else text
    facts = {}
    if k >= 0:
        for l in text[k:].split("\n")[1:]:
            if not l.startswith("("):
                continue
            j = sexpr_end(l, 0)
            fact, comment = l[:j], l[j:].strip()
            c = comment[1:].strip() if comment.startswith(";") else ""
            if c.startswith("source "):
                origin = ("source",)
            elif c.startswith("rule "):
                name = re.match(r"rule (\S+?)(?::|$| —)", c + " ").group(1)
                origin = ("rule", name)
            elif c.startswith("hypothesis "):
                origin = ("hyp", c.split()[1].rstrip(":"))
            else:
                origin = ("given",)
            facts[fact] = origin
    return head, program, facts


def header_field(head, key):
    for h in head:
        m = re.match(r";;; %s:\s+(.*)$" % re.escape(key), h)
        if m:
            return m.group(1)
    return None


def kind(origin):
    """given and source are one kind for the comparison: typed by a file."""
    return "typed" if origin[0] in ("given", "source") else origin[0]


def compare(S, X, w0):
    """S against a loaded KB X ({fact: origin}, `written` included); w0 is the
    program part's own `written` set. Returns counts and a few examples."""
    xw = {f for f in X if head_of(f) == "written"}
    xf = {f: o for f, o in X.items() if head_of(f) not in ("written", "relation")}
    sf = {f: o for f, o in S.items() if head_of(f) not in ("written", "relation")}
    only_s = sorted(set(sf) - set(xf) - {"(false)"})
    only_x = sorted(set(xf) - set(sf) - {"(false)"})
    dead = "(false)" in xf
    extra_w = sorted(xw - w0)
    lost_w = sorted(w0 - xw)
    cats = {}
    ex = {}
    for f in set(sf) & set(xf):
        a, b = sf[f], xf[f]
        if a == b or (kind(a) == "typed" and kind(b) == "typed"):
            continue
        if kind(a) == "hyp" and kind(b) == "typed":
            c = "hypothesis -> typed"
        elif kind(a) == "rule" and kind(b) == "typed":
            c = "rule -> typed"
        elif kind(a) == "rule" and kind(b) == "rule":
            c = "rule r -> rule r'"
        elif kind(a) == "hyp" and kind(b) == "hyp":
            c = "hypothesis n -> hypothesis n'"
        else:
            c = f"{kind(a)} -> {kind(b)}"
        cats[c] = cats.get(c, 0) + 1
        ex.setdefault(c, []).append(f"{f}  S:{' '.join(a)}  X:{' '.join(b)}")
    return {
        "only_in_state": len(only_s),
        "only_in_loaded": len(only_x),
        "dead": dead,
        "written_beyond_root": len(extra_w),
        "written_lost": len(lost_w),
        "origin": cats,
        "examples": {
            "only_in_state": only_s[:3],
            "only_in_loaded": only_x[:3],
            "written_beyond_root": extra_w[:3],
            "origin": {c: v[:3] for c, v in ex.items()},
        },
    }


def state_files(d):
    out = []
    for sub in ("models", "owing"):
        p = os.path.join(d, sub)
        if os.path.isdir(p):
            out += [os.path.join(sub, f) for f in sorted(os.listdir(p)) if f.endswith(".ein")]
    if os.path.exists(os.path.join(d, "root.ein")):
        out.append("root.ein")
    return out


def spread(items, cap):
    if len(items) <= cap:
        return items
    step = (len(items) - 1) / (cap - 1)
    return [items[round(i * step)] for i in range(cap)]


def body(text):
    return "\n".join(l for l in text.split("\n") if not l.startswith(";;;"))


def facts_key(text):
    k = text.find(STATE_MARK)
    if k < 0:
        return ""
    return "\n".join(
        l[: sexpr_end(l, 0)] for l in text[k:].split("\n") if l.startswith("(")
    )


def measure(ein, path, out, cap):
    name = path.replace("/", "__")
    d = os.path.join(out, name)
    shutil.rmtree(d, ignore_errors=True)
    os.makedirs(d)
    r = {"path": path}
    d1 = os.path.join(d, "d1")
    code, so, se = run([ein, "solve", path, "--dump-answer", d1])
    if not os.path.exists(os.path.join(d1, "answer.txt")):
        r["skip"] = "no answer" if code is not None else "timeout"
        return r
    files = state_files(d1)
    r["state_files"] = len(files)
    if not files:
        r["skip"] = "no state file"
        return r
    # Root as the original program loads it.
    code, so, se = run([ein, "saturate", "--dump", path])
    root_orig, _ = parse_dump(so)
    # Determinism of the writer.
    det = {}
    for tag, extra in (("again", []), ("j4", ["-j", "4"]), ("maximal", ["--maximal"])):
        dx = os.path.join(d, tag)
        run([ein, "solve", path, *extra, "--dump-answer", dx])
        fx = state_files(dx) if os.path.isdir(dx) else []
        if tag != "maximal":
            same = sum(
                1
                for f in files
                if f in fx
                and open(os.path.join(d1, f), "rb").read() == open(os.path.join(dx, f), "rb").read()
            )
            det[tag] = {"files": len(files), "byte_identical": same, "theirs": len(fx)}
        else:
            mine, by_c = {}, {}
            for f in files:
                t = open(os.path.join(d1, f)).read()
                mine.setdefault(facts_key(t), []).append(t)
                by_c[header_field(t.split("\n")[:8], "from")] = t
            same_body = same_bytes = matched = 0
            same_c = same_c_body = 0
            for f in fx:
                t = open(os.path.join(dx, f)).read()
                o = by_c.get(header_field(t.split("\n")[:8], "from"))
                if o is not None:
                    same_c += 1
                    same_c_body += body(o) == body(t)
                ours = mine.get(facts_key(t))
                if not ours:
                    continue
                matched += 1
                same_body += any(body(o) == body(t) for o in ours)
                same_bytes += any(o == t for o in ours)
            det[tag] = {
                "files": len(fx),
                "matched": matched,
                "body_identical": same_body,
                "byte_identical": same_bytes,
                "same_commitment": same_c,
                "same_commitment_body_identical": same_c_body,
            }
        shutil.rmtree(dx, ignore_errors=True)
    r["determinism"] = det
    code, so, se = run([ein, "solve", path])
    r["program_exit"] = code
    try:
        r["proposed_overhead"] = len(propose.saved(path).encode())
    except OSError as e:  # a module the resolver cannot find: named
        r["proposed_overhead"] = None
        r["propose_error"] = str(e)
    root_part = None
    states = []
    for f in spread(files, cap):
        fp = os.path.join(d1, f)
        text = open(fp).read()
        head, program, S = parse_state_file(text)
        commit = header_field(head, "from") or header_field(head, "commitment")
        C = split_set(commit) if commit is not None else []
        hyp_in_S = sorted(x for x, o in S.items() if o[0] == "hyp")
        st = {
            "file": f,
            "commitment": C,
            "facts": len(S),
            "derived": sum(1 for o in S.values() if o[0] == "rule"),
            "hyp_facts": len(hyp_in_S),
            "commitment_is_hyp_facts": sorted(C) == hyp_in_S,
            "bytes": len(text.encode()),
            "program_bytes": len(program.encode()),
            "proposed_bytes": None
            if r["proposed_overhead"] is None
            else len(propose.saved(path, fp).encode()),
        }
        code, so, se = run([ein, "solve", fp])
        st["solve_exit"] = code
        rb_dir = os.path.join(d, "rebuild")
        os.makedirs(rb_dir, exist_ok=True)
        if root_part is None:
            pp = os.path.join(rb_dir, "program_part.ein")
            open(pp, "w").write(program)
            code, so, se = run([ein, "saturate", "--dump", pp])
            root_part, _ = parse_dump(so)
            r["root_inlined_vs_original"] = (
                None
                if root_part is None or root_orig is None
                else {
                    "facts_equal": set(root_part) == set(root_orig),
                    "origins_equal": root_part == root_orig,
                    "only_original": sorted(set(root_orig) - set(root_part))[:3],
                    "only_inlined": sorted(set(root_part) - set(root_orig))[:3],
                }
            )
        if root_part is None:
            st["error"] = "the program part does not load"
            states.append(st)
            continue
        w0 = {x for x in root_part if head_of(x) == "written"}
        bp = os.path.join(rb_dir, f.replace("/", "_"))
        open(bp, "w").write(
            program + "\n;; rebuild: the commitment as facts\n" + "".join(c + "\n" for c in C)
        )
        code, so, se = run([ein, "saturate", "--dump", bp])
        B, _ = parse_dump(so)
        st["B"] = compare(S, B, w0) if B is not None else {"error": se.strip()[:300]}
        code, so, se = run([ein, "saturate", "--dump", fp])
        L, Ltext = parse_dump(so)
        st["L"] = compare(S, L, w0) if L is not None else {"error": se.strip()[:300]}
        # Today's save -> load -> save.
        d2 = os.path.join(d, "second", f.replace("/", "_"))
        run([ein, "solve", fp, "--no-search", "--dump-answer", d2])
        f2 = os.path.join(d2, "root.ein")
        if os.path.exists(f2):
            t2 = open(f2).read()
            code, so, se = run([ein, "saturate", "--dump", f2])
            _, L2text = parse_dump(so)
            st["second_save"] = {
                "byte_identical": t2 == text,
                "bytes": len(t2.encode()),
                "lines": t2.count("\n"),
                "first_lines": text.count("\n"),
                "dump_identical": Ltext is not None and Ltext == L2text,
            }
        else:
            st["second_save"] = None
        states.append(st)
    r["states"] = states
    shutil.rmtree(os.path.join(d, "second"), ignore_errors=True)
    return r


def summarise(results):
    progs = [r for r in results if "states" in r]
    sts = [(r["path"], s) for r in progs for s in r["states"]]
    out = {
        "entries": len(results),
        "programs_with_state_files": len(progs),
        "skipped": {},
        "state_files_written": sum(r["state_files"] for r in progs),
        "state_files_measured": len(sts),
    }
    for r in results:
        if "skip" in r:
            out["skipped"][r["skip"]] = out["skipped"].get(r["skip"], 0) + 1
    for side in ("B", "L"):
        agg = {
            "states": 0,
            "error": 0,
            "facts_differ": 0,
            "dead_where_state_alive": 0,
            "written_beyond_root": 0,
            "written_lost": 0,
            "origin": {},
            "programs": {},
        }
        for p, s in sts:
            x = s.get(side)
            if x is None:
                continue
            agg["states"] += 1
            if "error" in x:
                agg["error"] += 1
                agg["programs"].setdefault("error", []).append(p)
                continue
            if x["only_in_state"] or x["only_in_loaded"]:
                agg["facts_differ"] += 1
                agg["programs"].setdefault("facts_differ", []).append(p)
            if x["dead"]:
                agg["dead_where_state_alive"] += 1
                agg["programs"].setdefault("dead", []).append(p)
            if x["written_beyond_root"]:
                agg["written_beyond_root"] += 1
                agg["programs"].setdefault("written_beyond_root", []).append(p)
            if x["written_lost"]:
                agg["written_lost"] += 1
            for c, n in x["origin"].items():
                a = agg["origin"].setdefault(c, {"states": 0, "facts": 0, "programs": []})
                a["states"] += 1
                a["facts"] += n
                a["programs"].append(p)
        for k, v in agg["programs"].items():
            agg["programs"][k] = sorted(set(v))
        for c in agg["origin"].values():
            c["programs"] = sorted(set(c["programs"]))
        out[side] = agg
    ss = [s["second_save"] for _, s in sts if s.get("second_save")]
    out["second_save"] = {
        "measured": len(ss),
        "byte_identical": sum(1 for x in ss if x["byte_identical"]),
        "dump_identical": sum(1 for x in ss if x["dump_identical"]),
        "lines_first_total": sum(x["first_lines"] for x in ss),
        "lines_second_total": sum(x["lines"] for x in ss),
    }
    det = {}
    for r in progs:
        for tag, v in r.get("determinism", {}).items():
            a = det.setdefault(tag, {})
            for k, n in v.items():
                a[k] = a.get(k, 0) + n
    out["determinism"] = det
    rv = [r["root_inlined_vs_original"] for r in progs if r.get("root_inlined_vs_original")]
    out["root_inlined_vs_original"] = {
        "programs": len(rv),
        "facts_equal": sum(1 for x in rv if x["facts_equal"]),
        "origins_equal": sum(1 for x in rv if x["origins_equal"]),
        "differ": [
            r["path"]
            for r in progs
            if r.get("root_inlined_vs_original") and not r["root_inlined_vs_original"]["origins_equal"]
        ],
    }
    out["commitment_is_hyp_facts"] = sum(1 for _, s in sts if s.get("commitment_is_hyp_facts"))
    claims = {}
    for r in progs:
        for s in r["states"]:
            k = (
                f"{'with' if s['commitment'] else 'no'} commitment: "
                f"program exits {r.get('program_exit')}, its state file {s.get('solve_exit')}"
            )
            a = claims.setdefault(k, {"states": 0, "programs": set()})
            a["states"] += 1
            a["programs"].add(r["path"])
    out["solve_the_state_file"] = {
        k: {"states": v["states"], "programs": len(v["programs"]), "some": sorted(v["programs"])[:6]}
        for k, v in sorted(claims.items())
    }
    pairs = [(s["bytes"], s["proposed_bytes"]) for _, s in sts if s.get("proposed_bytes")]
    out["size"] = {
        "states": len(pairs),
        "today_bytes": sum(a for a, _ in pairs),
        "proposed_bytes": sum(b for _, b in pairs),
        "proposed_smaller": sum(1 for a, b in pairs if b < a),
        "propose_errors": [r["path"] for r in progs if r.get("propose_error")],
    }
    return out


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--bin", default=os.environ.get("EIN_BIN", "ein.rs/target/gate/ein"))
    ap.add_argument("--out", default="build/s5/measure")
    ap.add_argument("--cap", type=int, default=20)
    ap.add_argument("--jobs", type=int, default=12)
    ap.add_argument("--only")
    ap.add_argument("--slow", action="store_true")
    ap.add_argument("paths", nargs="*", help="programs to measure instead of the corpus")
    a = ap.parse_args()
    ein = os.path.abspath(a.bin)
    corpus = tomllib.load(open("corpus/corpus.toml", "rb"))
    entries = a.paths or [
        e["path"]
        for e in corpus["entry"]
        if (a.slow or not (e.get("slow") or e.get("heavy_dump")))
        and (a.only is None or a.only in e["path"])
    ]
    os.makedirs(a.out, exist_ok=True)
    results = []
    with cf.ThreadPoolExecutor(a.jobs) as ex:
        futs = {ex.submit(measure, ein, p, a.out, a.cap): p for p in entries}
        for i, fu in enumerate(cf.as_completed(futs)):
            try:
                results.append(fu.result())
            except Exception as e:  # a parse of ours that failed: kept, named
                results.append({"path": futs[fu], "skip": f"script error: {e!r}"})
            print(f"\r{i + 1}/{len(entries)}", end="", file=sys.stderr)
    print(file=sys.stderr)
    results.sort(key=lambda r: r["path"])
    json.dump(results, open(os.path.join(a.out, "results.json"), "w"), indent=1)
    s = summarise(results)
    json.dump(s, open(os.path.join(a.out, "summary.json"), "w"), indent=1)
    print(json.dumps(s, indent=1))


if __name__ == "__main__":
    main()
