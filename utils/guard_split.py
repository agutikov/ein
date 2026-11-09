#!/usr/bin/env python3
"""The guard split, prototyped — M1e2 S1e2.1b.6 T1e2.1b.6.3.

    guard_split.py FILE.ein…           the split on each program, held to its oracle node for node
    guard_split.py --corpus            every flagged corpus and register program, and every program
                                       under examples/lattice/synth/; past the limit, the guards only
    guard_split.py … --json            one JSON row per program
    guard_split.py … --keep DIR        keep every cell program it writes

**The strategy** (docs/kernel/inference/monotonicity.md § How to solve). A
program the static check flags has *guards*: the ground facts a third-kind
read can see change as the commitment grows. Fix the truth of each, a *guess*
γ, and the program becomes monotone: the cell program P_γ. In it

- every third-kind read (an `absent` at odd depth in a saturation rule, an
  obligation's trigger) reads `gs-<rel>` instead of `<rel>`, a static
  relation holding the program's given facts of `<rel>` and the facts γ calls
  present;
- an activator a commitment can derive is a guard too: P_γ compiles the rule
  for it when γ calls it present, and never at a node;
- the guess is checked by the cell's own rules, and both checks are monotone:
  a fact γ calls **present** is owed (an obligation its witness pays), and a
  fact γ calls **absent** is a restriction (`(false)` when it holds).

So R(P_γ) is the part of the cube on which the guess is the truth, and **R is
the union over γ of R(P_γ)** — the Gelfond–Lifschitz reduct's move: guess the
negated atoms, solve the monotone remainder, keep what the guess survives.
That presumes a node is built **from scratch** (each cell is its own program),
and the oracle builds it by **fork** from root's fixpoint, so where the two
readings differ the split and the oracle disagree. Which one R is defined over
is S1e2.1b.2's ruling; until then a disagreement is reported, not repaired.

**What it runs.** The guards are taken without the cube: one `ein saturate`
of the *relaxed* program (every guard read empty, every derivable activator
compiled, every candidate given) bounds the facts any node can hold, and a
guard is such a fact, not given, that a rewritten read can match (g of them).
Each of the 2^g cells is solved by `ein solve` with no flag, which on a
monotone program is route 1, and read back from `--json-summary`: a state's
commitments are the intervals [minimal generator … closure], exact because
saturation is a closure operator on a monotone cube
(`ein-infer/tests/region.rs`). The union is held to `ein oracle --dump` on
the original program, node for node. Nothing here enumerates the cube except
the oracle it is checked against.

**What it does not do.** It builds nothing into the engine (Q-M1e2.8). It
reads relation-grain sites as the static check reports them, and narrows the
guards to the facts a rewritten read's pattern can match. It expands
`std.macro`'s `forall` and `unknown` as the loader does, and refuses, by name,
what it cannot rewrite: the wildcard negative, and a stdlib rule a program
imports without `:symbols` when that rule has to be expanded.

The engine is `--bin`, else `$EIN_BIN`, else `ein.rs/target/release/ein`.
"""

from __future__ import annotations

import argparse
import itertools
import json
import os
import subprocess
import sys
import tempfile
import time
import tomllib
from pathlib import Path

REPO = Path(__file__).resolve().parents[1]
class Unsupported(Exception):
    """A construct the prototype does not rewrite; the row names it."""


# ── s-expressions ─────────────────────────────────────────────────────────────

def tokens(text: str):
    i, n = 0, len(text)
    while i < n:
        c = text[i]
        if c.isspace():
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
            yield text[i:j + 1]
            i = j + 1
        else:
            j = i
            while j < n and not text[j].isspace() and text[j] not in '();"':
                j += 1
            yield text[i:j]
            i = j


def parse(text: str) -> list:
    stack: list[list] = [[]]
    for t in tokens(text):
        if t == "(":
            stack.append([])
        elif t == ")":
            done = stack.pop()
            stack[-1].append(done)
        else:
            stack[-1].append(t)
    if len(stack) != 1:
        raise ValueError("unbalanced parentheses")
    return stack[0]


def ser(e) -> str:
    return "(" + " ".join(ser(x) for x in e) + ")" if isinstance(e, list) else e


def kw(form: list, key: str):
    for i in range(len(form) - 1):
        if form[i] == key:
            return i + 1
    return None


# ── the program ───────────────────────────────────────────────────────────────

Key = tuple[bool, str]      # (negated, relation): `q`, or the stored negatives `(not q)`
Atom = tuple[Key, tuple]    # a ground fact of a guard relation


def gs_name(key: Key) -> str:
    return f"gs-not-{key[1]}" if key[0] else f"gs-{key[1]}"


def atom_of(e) -> Atom | None:
    if not isinstance(e, list) or not e or not isinstance(e[0], str):
        return None
    if e[0] == "not" and len(e) == 2 and isinstance(e[1], list):
        return ((True, e[1][0]), tuple(ser(x) for x in e[1][1:]))
    return ((False, e[0]), tuple(ser(x) for x in e[1:]))


def atom_form(a: Atom, last: str | None = None) -> list:
    (neg, rel), args = a
    args = list(args[:-1]) + [last] if last is not None else list(args)
    inner = [rel] + args
    return ["not", inner] if neg else inner


def atom_text(a: Atom) -> str:
    return ser(atom_form(a))


def why_of(words: str, arity: int) -> str:
    """A relation's `:why` naming each of its `arity` places, as a string token."""
    places = " and ".join(f"{{?{j + 1}}}" for j in range(arity))
    return f'"{words} {places}"' if arity else f'"{words} nothing"'


def key_label(label: str) -> Key:
    if label == "(not ?)":
        raise Unsupported("a wildcard negative `(not ?x)` is a site")
    if label.startswith("(not "):
        return (True, label[5:-1])
    return (False, label)


def rewrite(e, depth: int, targets: set, reads: list):
    """`e` with every read of a target `(key, depth)` pointed at `gs-<rel>`."""
    if not isinstance(e, list) or not e:
        return e
    h = e[0]
    if h == "absent":
        return [h] + [rewrite(x, depth + 1, targets, reads) for x in e[1:]]
    if h in ("and", "or"):
        return [h] + [rewrite(x, depth, targets, reads) for x in e[1:]]
    if h in ("neq", "eq"):
        return e
    # std.macro's two, expanded as the loader expands them.
    if isinstance(h, str) and (h == "forall" or h.endswith(".forall")) and len(e) == 4:
        return rewrite(["absent", ["and", e[2], ["absent", e[3]]]], depth, targets, reads)
    if isinstance(h, str) and (h == "unknown" or h.endswith(".unknown")) and len(e) == 2:
        return rewrite(["and", ["absent", e[1]], ["absent", ["not", e[1]]]], depth, targets, reads)
    a = atom_of(e)
    if a is not None and (a[0], depth) in targets:
        reads.append((a[0], e[1][1:] if a[0][0] else e[1:]))
        return [gs_name(a[0])] + (e[1][1:] if a[0][0] else e[1:])
    return e


def substitute(e, sub: dict[str, str]):
    if isinstance(e, list):
        return [substitute(x, sub) for x in e]
    if e.startswith('"'):
        for k, v in sub.items():
            e = e.replace("{" + k + "}", v)
        return e
    return sub.get(e, e)


class Program:
    """A program's forms, its stdlib rules by name, and its candidates."""

    def __init__(self, path: Path, candidates: list[str]):
        self.path = path
        self.forms = parse(path.read_text(encoding="utf-8"))
        self.candidates = candidates
        self.rules = {f[1]: f for f in self.forms if isinstance(f, list) and f and f[0] == "rule"}
        # A stdlib rule is the one in the module the program imports it from.
        modules: dict[str, dict[str, list]] = {}
        for lib in sorted((REPO / "stdlib").glob("*.ein")):
            modules["std." + lib.stem] = {f[1]: f for f in parse(lib.read_text(encoding="utf-8"))
                                         if isinstance(f, list) and f and f[0] == "rule"}
        self.stdlib = {}
        for f in self.forms:
            if isinstance(f, list) and f and f[0] == "import" and len(f) > 1:
                for name, form in modules.get(f[1], {}).items():
                    self.stdlib.setdefault(name, form)
        self.given: set[Atom] = set()
        for f in self.forms:
            if isinstance(f, list) and f and isinstance(f[0], str) and f[0] not in (
                    "rule", "hrule", "relation", "import", "query", "solve", "config", "macro", "trace"):
                # A fact, less its trailing keywords (`:source "…"`).
                end = next((i for i, x in enumerate(f) if isinstance(x, str) and x.startswith(":")), len(f))
                a = atom_of(f[:end])
                if a is not None:
                    self.given.add(a)

    def rule_form(self, name: str) -> tuple[list, bool]:
        if name in self.rules:
            return self.rules[name], False
        if name in self.stdlib:
            return self.stdlib[name], True
        raise Unsupported(f"no rule `{name}` in the program or the stdlib")


class Split:
    """The template a cell is written from: the sites rewritten, the param
    rules that carry a site expanded into one copy per activator fact."""

    def __init__(self, prog: Program, sites: list[dict]):
        self.prog = prog
        self.targets: dict[str, set] = {}       # rule (or copy) name → {(key, depth)}
        self.expand: set[str] = set()           # param rules replaced by copies
        self.activators: set[str] = set()       # … whose activator a commitment can derive
        self.reads: dict[Key, list] = {}        # guard key → the patterns its rewritten reads have
        self.add_sites(sites)

    def add_sites(self, sites: list[dict]) -> bool:
        grew = False
        for s in sites:
            rule = s["rule"]
            if s["kind"] == "activator":
                grew |= rule not in self.activators
                self.activators.add(rule)
                self.expand.add(rule)
                continue
            form, _ = self.rule_form_of(rule)
            name = rule
            if form[2]:
                self.expand.add(form[1])
                if s["activator"]:
                    name = copy_name(form[1], s["activator"])
            t = (key_label(s["relation"]), s["depth"])
            grew |= t not in self.targets.setdefault(name, set())
            self.targets[name].add(t)
        return grew

    def rule_form_of(self, name: str) -> tuple[list, bool]:
        base = name.split("--")[0] if "--" in name else name
        return self.prog.rule_form(base)

    def copy(self, rule: str, args: tuple) -> list:
        form, _ = self.prog.rule_form(rule)
        params = form[2]
        if len(params) != len(args):
            raise Unsupported(f"`{rule}` takes {len(params)} activator arguments, given {len(args)}")
        c = substitute(form, dict(zip(params, args)))
        c[1], c[2] = copy_name(rule, args), []
        return c

    def given_activators(self, rule: str) -> list[tuple]:
        return sorted(args for (neg, rel), args in self.prog.given if not neg and rel == rule)

    def forms(self, act_present: set[Atom], gs: dict[Key, set], hrules: bool, extra: list[list]) -> list[list]:
        """The program text of one cell: `gs` is each guard read's extension,
        `act_present` the derivable activators compiled."""
        self.reads_now: dict[Key, list] = {}
        out: list[list] = []
        declared = set()
        for f in self.prog.forms:
            if not isinstance(f, list) or not f:
                continue
            h = f[0]
            if h in ("hrule", "query", "solve"):
                continue
            if h == "relation":
                declared.add(f[1])
            if h == "import":
                i = kw(f, ":symbols")
                if i is None:
                    if self.expand:
                        raise Unsupported("a stdlib rule imported without `:symbols`")
                    out.append(f)
                    continue
                syms = [s for s in f[i] if s not in self.expand]
                if syms:
                    out.append(f[:i] + [syms] + f[i + 1:])
                continue
            if h == "rule":
                if f[1] in self.expand:
                    continue
                out.append(self.rewritten(f))
                continue
            out.append(f)
        for rule in sorted(self.expand):
            form, _ = self.prog.rule_form(rule)
            arity = len(form[2])
            if rule not in declared and arity:
                # Every declaration carries a `:why` since M1f S1f.5.14's
                # T1f.5.14.7 (Q-M1f.63): what the form says, mechanically.
                out.append(["relation", rule] + ["T"] * arity + [":why", why_of(f"rule {rule} is switched on for", arity)])
            args_all = self.given_activators(rule) + sorted(
                args for (neg, rel), args in act_present if rel == rule)
            for args in args_all:
                out.append(self.rewritten(self.copy(rule, args)))
        out.append(["relation", "gs-world", "T", ":why", '"{?1} is the cell\'s world"'])
        out.append(["gs-world", "W"])
        out.append(["relation", "gs-need", "T", "T", ":why", '"the guess owes {?1}, which {?2} pays"'])
        for key in sorted(self.all_keys()):
            arity = self.arity(key)
            what = f"(not {key[1]})" if key[0] else key[1]
            out.append(["relation", gs_name(key)] + ["T"] * arity
                       + [":why", why_of(f"{what}, as given or as the guess calls it present, holds of", arity)])
            for a in sorted(gs.get(key, ())):
                out.append([gs_name(key)] + list(a[1]))
        if hrules:
            for i, c in enumerate(self.prog.candidates, 1):
                out.append(["hrule", f"gs-cand-{i}", [], ":match", ["gs-world", "W"], ":assert", parse(c)[0],
                            ":why", f'"guess the candidate {c}"'])
        return out + extra

    def rewritten(self, form: list) -> list:
        t = self.targets.get(form[1])
        if not t:
            return form
        i = kw(form, ":match")
        reads: list = []
        f = list(form)
        f[i] = rewrite(form[i], 0, t, reads)
        for key, pat in reads:
            self.reads.setdefault(key, [])
            if pat not in self.reads[key]:
                self.reads[key].append(pat)
        return f

    def all_keys(self) -> set[Key]:
        return {k for t in self.targets.values() for k, _ in t}

    def arity(self, key: Key) -> int:
        pats = self.reads.get(key)
        if pats:
            return len(pats[0])
        for (k, args) in self.prog.given:
            if k == key:
                return len(args)
        return 1


def copy_name(rule: str, args) -> str:
    return "--".join([rule] + [str(a) for a in args])


def matches(pat: list, args: tuple) -> bool:
    if len(pat) != len(args):
        return False
    return all(isinstance(p, str) and (p.startswith("?") or p == "_") or ser(p) == a
               for p, a in zip(pat, args))


# ── the engine ────────────────────────────────────────────────────────────────

def ein_bin(arg: str | None) -> str:
    return arg or os.environ.get("EIN_BIN") or str(REPO / "ein.rs/target/release/ein")


def oracle(path: Path, bin: str, limit: int | None = None) -> dict | None:
    with tempfile.TemporaryDirectory(prefix="guard-split-") as d:
        cmd = [bin, "oracle", str(path), "--dump", d] + (["--region-limit", str(limit)] if limit is not None else [])
        subprocess.run(cmd, capture_output=True, text=True)
        try:
            return json.loads((Path(d) / "region.json").read_text())
        except FileNotFoundError:
            return None


def saturated_facts(text: str, bin: str, tmp: Path) -> set[Atom]:
    f = tmp / "relaxed.ein"
    f.write_text(text, encoding="utf-8")
    p = subprocess.run([bin, "saturate", "--dump", str(f)], capture_output=True, text=True)
    if p.returncode != 0:
        raise Unsupported(f"the relaxed program does not saturate: {p.stderr.strip()[:200]}")
    out: set[Atom] = set()
    body = p.stdout.split("SATURATED KB DUMP", 1)[-1]
    for line in body.splitlines():
        s = line.strip()
        if s.startswith("(") and not line.startswith("("):
            e = parse(s)
            if e and isinstance(e[0], list):
                # A derived fact is dumped with its provenance: `(is-a Ann Diner :rule r)`.
                end = next((i for i, x in enumerate(e[0]) if isinstance(x, str) and x.startswith(":")), len(e[0]))
                a = atom_of(e[0][:end])
                if a is not None:
                    out.add(a)
    return out


def solve_summary(text: str, bin: str, path: Path) -> dict:
    path.write_text(text, encoding="utf-8")
    js = path.with_suffix(".json")
    p = subprocess.run([bin, "solve", str(path), "--json-summary", str(js)], capture_output=True, text=True)
    if not js.exists():
        raise Unsupported(f"a cell does not solve: {p.stderr.strip()[:300]}")
    return json.loads(js.read_text())


# ── the split ─────────────────────────────────────────────────────────────────

def consistency(guards: list[Atom], gamma: int) -> list[list]:
    """The guess, checked monotonically: owed when present, refuted when
    absent. A present guess is owed through a marker, `(gs-seen Mi)`, which a
    rule derives from the guard fact: an obligation's witness has to be a
    positive fact with a variable of its own, and a guard can be a stored
    negative."""
    out: list[list] = [["relation", "gs-seen", "T", ":why", '"the guessed fact marked {?1} is seen"']]
    for i, a in enumerate(guards):
        if gamma >> i & 1:
            out.append(["gs-need", f"K{i}", f"M{i}"])
            out.append(["rule", f"gs-mark-{i}", [], ":match", atom_form(a), ":assert", ["gs-seen", f"M{i}"],
                        ":why", f'"{atom_text(a)} holds, so its marker M{i} is seen"'])
            out.append(["rule", f"gs-owe-{i}", [], ":match",
                        ["and", ["gs-world", "?w"],
                         ["absent", ["and", ["gs-need", f"K{i}", "?k"], ["gs-seen", "?k"]]]],
                        ":assert", ["open", "gs-seen"], ":why", f'"the guess: {atom_text(a)} holds"'])
        else:
            out.append(["rule", f"gs-forbid-{i}", [], ":match", atom_form(a), ":assert", ["false"],
                        ":why", f'"the guess: {atom_text(a)} does not hold"'])
    return out


def split(path: Path, bin: str, keep: Path | None, max_g: int) -> dict:
    t0 = time.monotonic()
    row: dict = {"program": rel_path(path)}
    region = oracle(path, bin)
    if region is None:
        return row | {"skipped": "does not load"}
    row["n"] = n = region["n"]
    mono = region["monotonicity"]
    row["sites"] = sorted({s["kind"] for s in mono["sites"]})
    past = region.get("region") is None
    cands = region["candidates"]
    bit = {c: i for i, c in enumerate(cands)}
    if not past:
        want = {x["mask"] for x in region["region"]["nodes"] if x["status"] == "R"}
        row["R"] = len(want)
    prog = Program(path, cands)
    sp = Split(prog, mono["sites"])
    with tempfile.TemporaryDirectory(prefix="guard-split-") as d:
        tmp = Path(keep / path.stem) if keep else Path(d)
        tmp.mkdir(parents=True, exist_ok=True)
        cand_facts = [parse(c)[0] for c in cands]
        given_gs = {k: {a for a in prog.given if a[0] == k} for k in sp.all_keys()}
        # The guards: the relaxed program's fixpoint bounds every node's, and
        # a derivable activator found there is compiled on the next round.
        act: set[Atom] = set()
        for _ in range(8):
            text = "\n".join(ser(f) for f in sp.forms(act, given_gs, False, cand_facts)) + "\n"
            top = saturated_facts(text, bin, tmp)
            new = {a for a in top if not a[0][0] and a[0][1] in sp.activators and a not in prog.given
                   and len(a[1]) == len(prog.rule_form(a[0][1])[0][2])} - act
            given_gs = {k: {a for a in prog.given if a[0] == k} for k in sp.all_keys()}
            # A compiled copy can carry a site of its own: ask the static check.
            if new:
                act |= new
                continue
            (tmp / "max.ein").write_text("\n".join(ser(f) for f in sp.forms(act, given_gs, True, [])) + "\n",
                                         encoding="utf-8")
            mx = oracle(tmp / "max.ein", bin, limit=0)
            if mx is None:
                raise Unsupported("the template does not load")
            left = mx["monotonicity"]["sites"]
            if not left:
                break
            if not sp.add_sites(left):
                raise Unsupported("a site the rewrite does not remove: "
                                  + "; ".join(f"{s['rule']} {s['kind']} {s['relation']}@{s['depth']}" for s in left))
            given_gs = {k: {a for a in prog.given if a[0] == k} for k in sp.all_keys()}
        else:
            raise Unsupported("the activator closure does not settle")
        guards = sorted(
            {a for a in top if a[0] in sp.all_keys() and a not in prog.given
             and any(matches(p, a[1]) for p in sp.reads.get(a[0], []))}
            | act, key=atom_text)
        row["g"] = g = len(guards)
        row["guards"] = [atom_text(a) for a in guards]
        if past:
            return row | {"skipped": f"past --region-limit {region.get('limit')}: the guards only, no cell"}
        if g > max_g:
            return row | {"skipped": f"g = {g} > --max-g {max_g}"}
        got: dict[int, int] = {}
        sats = 0
        cells_nonempty = 0
        for gamma in range(1 << g):
            present = {a for i, a in enumerate(guards) if gamma >> i & 1}
            gs = {k: given_gs.get(k, set()) | {a for a in present if a[0] == k} for k in sp.all_keys()}
            forms = sp.forms({a for a in present if a in act}, gs, True, consistency(guards, gamma))
            s = solve_summary("\n".join(ser(f) for f in forms) + "\n", bin, tmp / f"cell_{gamma:0{max(g, 1)}b}.ein")
            sats += s["work"]["saturations"]
            if not s["monotonicity"]["monotone"]:
                raise Unsupported(f"cell {gamma} is not monotone: {s['monotonicity']['sites']}")
            reg = s["region"]
            if reg.get("route") != "1" or not reg.get("exhausted"):
                raise Unsupported(f"cell {gamma}: route {reg.get('route')}, exhausted {reg.get('exhausted')}")
            # A candidate the cell's root already derives is not proposed
            # there, and committing to it changes no node: the cell's region
            # is lifted over those candidates, free.
            held = 0
            if reg["universe"]["candidates"] != n:
                roots = saturated_facts("\n".join(ser(f) for f in forms) + "\n", bin, tmp)
                held = sum(1 << i for i, c in enumerate(cands) if atom_of(parse(c)[0]) in roots)
                if reg["universe"]["candidates"] + bin_count(held) != n:
                    raise Unsupported(f"cell {gamma}: {reg['universe']['candidates']} candidates, "
                                      f"{bin_count(held)} held at its root, the program {n}")
            free_held = [i for i in range(n) if held >> i & 1]
            members = 0
            for st in reg["states"]:
                cl = sum(1 << bit[c] for c in st["closure"])
                cls = set()
                for gen in st["minimal_generators"]:
                    gm = sum(1 << bit[c] for c in gen)
                    free = [i for i in range(n) if cl >> i & 1 and not gm >> i & 1]
                    for k in range(len(free) + 1):
                        for sub in itertools.combinations(free, k):
                            cls.add(gm | sum(1 << i for i in sub))
                if len(cls) != st["commitments"]:
                    raise Unsupported(f"cell {gamma}: a state's intervals give {len(cls)} commitments, "
                                      f"route 1 counts {st['commitments']}")
                cls = {m | sum(1 << i for i in sub) for m in cls
                       for k in range(len(free_held) + 1) for sub in itertools.combinations(free_held, k)}
                for m in cls:
                    got[m] = got.get(m, 0) + 1
                members += len(cls)
            cells_nonempty += members > 0
    have = set(got)
    name = lambda m: "{" + ", ".join(c for i, c in enumerate(cands) if m >> i & 1) + "}" if m else "∅"
    row |= {
        "cells": 1 << g,
        "cells_nonempty": cells_nonempty,
        "R_split": len(have),
        "overlaps": sum(v > 1 for v in got.values()),
        "missing": [name(m) for m in sorted(want - have)],
        "extra": [name(m) for m in sorted(have - want)],
        "saturations_split": sats,
        "saturations_oracle": 1 << n,
        "agrees": want == have,
        "seconds": round(time.monotonic() - t0, 2),
    }
    return row


def bin_count(m: int) -> int:
    return bin(m).count("1")


def rel_path(p: Path) -> str:
    try:
        return str(p.resolve().relative_to(REPO))
    except ValueError:
        return str(p)


def corpus_selection(bin: str) -> list[Path]:
    """Every corpus entry and register witness the static check flags, and
    every program under examples/lattice/synth/. A flagged program past the
    limit gets its guards and no cell: there is no oracle to hold it to."""
    files = {REPO / e["path"] for e in tomllib.loads((REPO / "corpus/corpus.toml").read_text())["entry"]}
    reg = tomllib.loads((REPO / "corpus/wrong_answers.toml").read_text())
    for case in reg.get("case", []):
        for k, v in case.items():
            rows = v if isinstance(v, list) else [v]
            for r in rows:
                if isinstance(r, dict) and "witness" in r:
                    files.add(REPO / r["witness"])
        if "witness" in case:
            files.add(REPO / case["witness"])
    synth = set((REPO / "examples/lattice/synth").glob("*.ein"))
    out = []
    for f in sorted(files | synth):
        if not f.exists() or f.suffix != ".ein":
            continue
        r = oracle(f, bin, limit=0)
        if r is None:
            continue
        if f in synth or not r["monotonicity"]["monotone"]:
            out.append(f)
    return out


def main(argv: list[str]) -> int:
    ap = argparse.ArgumentParser(description=__doc__.split("\n\n")[0])
    ap.add_argument("files", nargs="*")
    ap.add_argument("--corpus", action="store_true")
    ap.add_argument("--json", action="store_true")
    ap.add_argument("--keep")
    ap.add_argument("--max-g", type=int, default=12)
    ap.add_argument("--bin")
    a = ap.parse_args(argv)
    bin = ein_bin(a.bin)
    files = corpus_selection(bin) if a.corpus else [Path(f) for f in a.files]
    if not files:
        ap.print_help()
        return 2
    keep = Path(a.keep) if a.keep else None
    rows = []
    for f in files:
        try:
            row = split(f, bin, keep, a.max_g)
        except Unsupported as e:
            row = {"program": rel_path(f), "unsupported": str(e)}
        rows.append(row)
        if a.json:
            print(json.dumps(row, ensure_ascii=False))
        else:
            print_row(row)
    if not a.json:
        done = [r for r in rows if "agrees" in r]
        print(f"\n{len(rows)} programs: {sum(r['agrees'] for r in done)} agree with the oracle node for node, "
              f"{sum(not r['agrees'] for r in done)} disagree, {len(rows) - len(done)} not split")
    return 1 if any("agrees" in r and not r["agrees"] for r in rows) else 0


def print_row(r: dict) -> None:
    if "unsupported" in r or "skipped" in r:
        g = f"n {r['n']} · g {r['g']} · " if "g" in r else ""
        print(f"{r['program']}: {g}not split — {r.get('unsupported') or r.get('skipped')}")
        if r.get("g"):
            print(f"  guards   {', '.join(r['guards'][:12])}{' …' if r['g'] > 12 else ''}")
        return
    verdict = "agrees" if r["agrees"] else "DISAGREES"
    print(f"{r['program']}: n {r['n']} · g {r['g']} · {r['cells']} cells, {r['cells_nonempty']} non-empty · "
          f"|R| oracle {r['R']}, split {r['R_split']} · overlaps {r['overlaps']} · "
          f"saturations {r['saturations_split']} against 2^n = {r['saturations_oracle']} · {verdict}")
    if r["g"]:
        print(f"  guards   {', '.join(r['guards'])}")
    for m in r["missing"][:5]:
        print(f"  missing  {m} — in R, in no cell's")
    for m in r["extra"][:5]:
        print(f"  extra    {m} — in a cell's R, not in R")


if __name__ == "__main__":
    sys.exit(main(sys.argv[1:]))
