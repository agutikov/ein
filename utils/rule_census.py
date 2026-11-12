#!/usr/bin/env python3
"""The census of rules: shapes and connections — M12's first phase.

    utils/rule_census.py                 # the whole census, into build/rule_census/
    utils/rule_census.py --out DIR       # ... somewhere else
    utils/rule_census.py --strata        # + the engine's strata beside the static graph's ($EIN_BIN)
    utils/rule_census.py --svg           # + the DOT pictures rendered by Graphviz
    utils/rule_census.py --top 30        # how many shapes a table of the summary lists
    utils/rule_census.py --page plans/m12_rules_as_data/rule_census.md
                                         # rewrite the page's generated region (runs --strata)

The page is plans/m12_rules_as_data/rule_census.md: its prose is written by
hand and everything between its two `generated` markers is this script's
`summary.md`, put there by `--page`.  Edit the script, never the tables.  The user's instruction (2026-11-12): *"Census of the
rules shapes - name agnostic, only considering equal names when they reused -
so analyze only match and assert graphs. count and census all of them. Then
connections between rules - first list all of possible connections [...] No
specific goals right now, want to see analyze results."*

**What is read.**  Every tracked `.ein` under `stdlib/`, `examples/` and
`tests/` (`git ls-files`), as text: the script has an S-expression reader of
its own (`;` comments, `#| |#` blocks, strings) and runs no engine but under
`--strata`.  Every `(rule NAME (PARAMS) :match M :assert A)` and `(hrule ...)`
written at the top level of a file is one rule, **as written**: no macro is
expanded for a shape, no import is resolved for a shape, and a rule that the
loader would refuse is a rule here.  Files under `examples/broken/` are read
and counted apart, and are in no table; a file the reader cannot read is
counted and named.

**A. Shapes.**  A rule's shape is its `:match` and `:assert` with every name
replaced by a token of its kind, equal names equal tokens:

    kind  what                                             token
    R     a name written as the head of a form anywhere    R1 R2 ...
          in the rule (a relation)
    c     any other name (a constant)                      c1 c2 ...
    ?P    a variable of the parameter list                 ?P1 ?P2 ...
    ?     any other variable                               ?a ?b ...

Kept as written, being structure and not names: the heads `and`, `or`, `not`,
`absent`, `eq`, `neq`, `false`, `open`; a macro's name at the head of a form
(`forall`, `unknown` of `std.macro` where the file imports it, and a macro the
file declares); and the kernel's two dunders `__closed__` and `__symmetric__`
wherever they are written.  A structure word in an argument's place is a name
(`eq` in `(cmp-of ?x ?y eq)` is the constant it spells).

The levels (each a column of `rules.tsv`):

    alpha   variables abstracted alone, order as written        (the same text)
    reli    relations and variables abstracted, order as written
    coni    constants and variables abstracted, order as written
    alphau  ... conjuncts in any order                           (the same rule)
    relren  relations and variables abstracted, constants kept   (one rule under a renaming of relations)
    conren  constants and variables abstracted, relations kept   (one rule under a renaming of constants)
    i       everything abstracted, order as written
    ii      everything abstracted, conjuncts in any order
    iii     any order, two kinds alone: whatever heads a form, a name or a
            variable, is a relation `R`, and every other name, variable or
            parameter a term `t`; the number of parameters is dropped
    iiib    as iii, every term `_`                      (which term is which is forgotten)
    iv      the skeleton: counts alone
    ivb     five counts of iv

*In any order* is a canonical labelling: the children of `and`, of `or` and of
`eq`/`neq` are a multiset.  Names are first split by colour refinement (a name's
colour is refined by the literals it occurs in), and names the refinement
leaves together are tried in every order while that is at most TIE_LIMIT
orders, the least string winning, which is exact.  Past that the script
individualises one name at a time and refines again: exact wherever the tied
names are interchangeable (the hundred relations of a closing rule), and a
heuristic otherwise.  It runs that fallback twice, picking tied names in
opposite orders, and `summary.md` says how many rules took it and on how many
the two runs differ.

**B. Connections.**  The catalogue is `KINDS` below, each a relation on rules,
counted in three scopes: *global* (any two rules of the census, names equal by
spelling alone), *file* (two rules of one file) and *program* (a rule of a
file and a rule it imports, the imports followed to module files and a
`:symbols` selection closed over the names a kept rule writes: an
approximation of the loader's, stated on the page).  For connections alone the
`:match` is read with `std.macro`'s and the file's macros expanded, so that a
`forall`'s premises are premises.

Deterministic: every listing is sorted, and no hash decides an order.
"""
from __future__ import annotations

import argparse
import itertools
import json
import os
import subprocess
import sys
from collections import Counter, defaultdict
from math import factorial

ROOT = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
ROOTS = ("stdlib", "examples", "tests")
BROKEN = "examples/broken/"

HEAD_STRUCT = frozenset(("and", "or", "not", "absent", "eq", "neq", "false", "open"))
ANY_STRUCT = frozenset(("__closed__", "__symmetric__"))
COMMUTATIVE = frozenset(("and", "or", "eq", "neq"))
DECLARATORS = frozenset(("relation", "rule", "hrule", "query", "solve", "config", "trace",
                         "macro", "import"))
STD_MACROS = ("forall", "unknown")
TIE_LIMIT = 5040
PAGE_OPEN = "<!-- generated: utils/rule_census.py --page -->"
PAGE_CLOSE = "<!-- /generated: utils/rule_census.py -->"


# ───────────────────────────── reading ─────────────────────────────

class Str(str):
    """A string literal, apart from a name that spells the same."""


class ReadError(Exception):
    pass


def read_forms(text):
    """The top-level forms of a text, each `(line, form)`; a form is a nested
    list of `str` atoms and `Str` literals."""
    forms = []
    stack = []
    lines = []
    i, n, line = 0, len(text), 1
    while i < n:
        ch = text[i]
        if ch == "\n":
            line += 1
            i += 1
        elif ch in " \t\r\f\v\x1c\x1d\x1e\x1f":
            i += 1
        elif ch == ";":
            while i < n and text[i] != "\n":
                i += 1
        elif ch == "#" and i + 1 < n and text[i + 1] == "|":
            j = text.find("|#", i + 2)
            if j < 0:
                raise ReadError(f"line {line}: a block comment is not closed")
            line += text.count("\n", i, j)
            i = j + 2
        elif ch == "(":
            stack.append([])
            lines.append(line)
            i += 1
        elif ch == ")":
            if not stack:
                raise ReadError(f"line {line}: a ) closes nothing")
            done = stack.pop()
            at = lines.pop()
            if stack:
                stack[-1].append(done)
            else:
                forms.append((at, done))
            i += 1
        elif ch == '"':
            j = i + 1
            buf = []
            while True:
                if j >= n:
                    raise ReadError(f"line {line}: a string is not closed")
                c = text[j]
                if c == "\\":
                    if j + 1 >= n:
                        raise ReadError(f"line {line}: a string is not closed")
                    buf.append(text[j:j + 2])
                    j += 2
                elif c == '"':
                    break
                else:
                    if c == "\n":
                        line += 1
                    buf.append(c)
                    j += 1
            if not stack:
                raise ReadError(f"line {line}: a string at the top level")
            stack[-1].append(Str("".join(buf)))
            i = j + 1
        else:
            j = i
            while j < n and text[j] not in " \t\r\n\f\v\x1c\x1d\x1e\x1f();":
                j += 1
            atom = text[i:j]
            if not atom.isascii():
                raise ReadError(f"line {line}: {atom!r} is not ASCII")
            if not stack:
                raise ReadError(f"line {line}: {atom!r} at the top level")
            stack[-1].append(atom)
            i = j
    if stack:
        raise ReadError(f"line {lines[-1]}: a ( is not closed")
    return forms


def is_var(x):
    return isinstance(x, str) and not isinstance(x, Str) and x.startswith("?")


def is_name(x):
    return isinstance(x, str) and not isinstance(x, Str) and not x.startswith("?")


class Rule:
    __slots__ = ("idx", "file", "line", "kind", "name", "params", "match", "assert_",
                 "root", "group", "macros", "shapes", "skel", "conn", "notes", "const_seq",
                 "rel_seq", "keys")

    def ref(self):
        return f"{self.file}:{self.name}"


class File:
    __slots__ = ("path", "forms", "rules", "imports", "macros", "facts", "solve_names",
                 "broken", "error", "decls")


def clauses(form, start):
    """The keyword clauses of a declaration from `start`: a list of pairs."""
    out = []
    i = start
    while i < len(form):
        key = form[i]
        if isinstance(key, str) and not isinstance(key, Str) and key.startswith(":"):
            val = form[i + 1] if i + 1 < len(form) else None
            out.append((key, val))
            i += 2
        else:
            out.append((None, key))
            i += 1
    return out


def read_file(path):
    f = File()
    f.path = path
    f.rules, f.imports, f.macros, f.facts, f.solve_names, f.decls = [], [], {}, [], [], []
    f.broken = path.startswith(BROKEN)
    f.error = None
    try:
        with open(os.path.join(ROOT, path), encoding="utf-8") as fh:
            f.forms = read_forms(fh.read())
    except (ReadError, UnicodeDecodeError) as e:
        f.forms = []
        f.error = str(e)
        return f
    for line, form in f.forms:
        if not form or not isinstance(form[0], str) or isinstance(form[0], Str):
            f.facts.append(form)
            continue
        head = form[0]
        if head in ("rule", "hrule"):
            ok = (len(form) >= 3 and is_name(form[1]) and isinstance(form[2], list)
                  and all(is_var(p) for p in form[2]))
            if not ok:
                f.decls.append(("malformed-" + head, line))
                continue
            r = Rule()
            r.file, r.line, r.kind, r.name, r.params = path, line, head, form[1], list(form[2])
            r.match, r.assert_, r.notes = [], [], []
            seen = Counter()
            for key, val in clauses(form, 3):
                seen[key] += 1
                if key == ":match" and val is not None:
                    r.match = val
                elif key == ":assert" and val is not None:
                    r.assert_ = val
            if seen[":match"] == 0:
                r.notes.append("no-match-clause")
            if seen[":assert"] == 0:
                r.notes.append("no-assert-clause")
            if seen[":match"] > 1 or seen[":assert"] > 1:
                r.notes.append("clause-twice")
            extra = sorted(k for k in seen if k not in (":match", ":assert", ":why"))
            if extra:
                r.notes.append("other-clause:" + ",".join(str(k) for k in extra))
            f.rules.append(r)
        elif head == "macro":
            if len(form) >= 4 and is_name(form[1]) and isinstance(form[2], list):
                f.macros[form[1]] = (list(form[2]), form[3])
        elif head == "import":
            if len(form) >= 2 and isinstance(form[1], str):
                symbols, alias = None, None
                for key, val in clauses(form, 2):
                    if key == ":symbols" and isinstance(val, list):
                        symbols = [s for s in val if isinstance(s, str)]
                    elif key == ":as":
                        alias = val
                f.imports.append((str(form[1]), symbols, alias))
        elif head == "solve":
            for key, val in clauses(form, 1):
                if key == ":hrules" and isinstance(val, list):
                    # `(h1 h2)`: two hrules; `(guess (R A B) (S A B))`: one,
                    # and the argument lists it is activated with.
                    f.solve_names.append([item for item in val if is_name(item)])
        elif head in DECLARATORS:
            pass
        else:
            f.facts.append(form)
    return f


def group_of(path):
    """The root of a path and the group the breakdown lists it under."""
    parts = path.split("/")
    root = parts[0]
    if root == "stdlib":
        return root, "stdlib"
    if root == "tests":
        return root, "/".join(parts[:3]) if len(parts) > 3 else "/".join(parts[:2])
    if len(parts) == 2:
        return root, "examples (top level)"
    return root, "/".join(parts[:2])


# ───────────────────────────── trees ─────────────────────────────
#
# A node: ("n", name) a name or a variable; ("s", word) structure; ("E",) the
# empty list; ("L", head, kids) a form, its head ("s", word) or ("n", name).

def to_node(x, macros):
    if isinstance(x, Str):
        return ("s", '"…"')
    if isinstance(x, str):
        if x in ANY_STRUCT:
            return ("s", x)
        return ("n", x)
    if not x:
        return ("E",)
    head = x[0]
    if isinstance(head, list) or isinstance(head, Str):
        return ("L", ("s", "<form>"), tuple(to_node(k, macros) for k in x))
    if head in HEAD_STRUCT or head in ANY_STRUCT or head in macros:
        h = ("s", head)
    else:
        h = ("n", head)
    return ("L", h, tuple(to_node(k, macros) for k in x[1:]))


def walk_names(node, head_names, arg_names, order):
    t = node[0]
    if t == "n":
        arg_names.add(node[1])
        if node[1] not in order:
            order[node[1]] = len(order)
    elif t == "L":
        if node[1][0] == "n":
            head_names.add(node[1][1])
            if node[1][1] not in order:
                order[node[1][1]] = len(order)
        for k in node[2]:
            walk_names(k, head_names, arg_names, order)


def kinds_of(parts, params):
    """name -> kind ('P', 'V', 'R', 'C') and the names in first-occurrence order."""
    heads, args, order = set(), set(), {}
    for p in parts:
        walk_names(p, heads, args, order)
    kinds, coarse = {}, {}
    for name in order:
        if name.startswith("?"):
            kinds[name] = "P" if name in params else "V"
        else:
            kinds[name] = "R" if name in heads else "C"
        # Levels (iii) and (iii′): whatever heads a form is a relation, a
        # variable too; whatever does not is a term.
        coarse[name] = "H" if name in heads else "T"
    return kinds, coarse


def render(node, tok, sort):
    t = node[0]
    if t == "n":
        return tok(node[1])
    if t == "s":
        return node[1]
    if t == "E":
        return "()"
    head = node[1]
    h = head[1] if head[0] == "s" else tok(head[1])
    kids = [render(k, tok, sort) for k in node[2]]
    if sort and head[0] == "s" and h in COMMUTATIVE:
        kids.sort()
    return "(" + " ".join([h] + kids) + ")"


def literals(node, path, out):
    """The outermost forms that are not `and`/`or`, each with the path of
    `and`/`or` above it."""
    if node[0] == "L" and node[1][0] == "s" and node[1][1] in ("and", "or"):
        for k in node[2]:
            literals(k, path + node[1][1][0], out)
    else:
        out.append((path, node))


def names_in(node, acc):
    if node[0] == "n":
        acc.add(node[1])
    elif node[0] == "L":
        if node[1][0] == "n":
            acc.add(node[1][1])
        for k in node[2]:
            names_in(k, acc)


VAR_LETTERS = "abcdefghijklmnopqrstuvwxyz"


def pretty(cls, n, internal):
    if internal:
        return f"#{cls}{n}"
    if cls == "R":
        return f"R{n}"
    if cls == "C":
        return f"c{n}"
    if cls == "P":
        return f"?P{n}"
    if cls == "T":
        return f"t{n}"
    return "?" + VAR_LETTERS[n - 1] if n <= 26 else f"?v{n}"


def shape_ordered(parts, kinds, mode, params, prefix, internal):
    """A shape with the order as written: tokens by first occurrence, the
    parameters by their place in the list."""
    counters = Counter()
    toks = {}
    for i, p in enumerate(params):
        if mode.get("P") and kinds.get(p) == "P":
            cls = mode["P"]
            if cls == "P":
                toks[p] = pretty("P", i + 1, internal)

    def tok(name):
        if name in toks:
            return toks[name]
        cls = mode.get(kinds[name])
        if cls is None:
            toks[name] = name
        elif cls == "_":
            toks[name] = "_"
        else:
            counters[cls] += 1
            toks[name] = pretty(cls, counters[cls], internal)
        return toks[name]

    return prefix + " ⇒ ".join(render(p, tok, False) for p in parts)


class Canon:
    """The canonical labelling of a rule's parts with conjuncts in any order."""

    def __init__(self):
        self.calls = 0
        self.fallback = 0
        self.unstable = 0

    def refine(self, base, lits_by_part, abstract):
        """Colour refinement: name -> rank, from `base` (name -> initial colour)."""
        colour = {}
        ranks = {c: i for i, c in enumerate(sorted(set(base.values())))}
        for n in abstract:
            colour[n] = ranks[base[n]]
        nclass = len(ranks)
        while True:
            sig = {n: [] for n in abstract}

            for pi, lits in enumerate(lits_by_part):
                for path, lit, members in lits:
                    for n in members:
                        def tok(m, n=n):
                            if m == n:
                                return "*"
                            c = colour.get(m)
                            return m if c is None else f"#{c}"
                        sig[n].append(f"{pi}{path}:{render(lit, tok, True)}")
            keyed = {n: (colour[n], tuple(sorted(sig[n]))) for n in abstract}
            ranks = {k: i for i, k in enumerate(sorted(set(keyed.values())))}
            colour = {n: ranks[keyed[n]] for n in abstract}
            if len(ranks) == nclass:
                return colour
            nclass = len(ranks)

    def shape(self, parts, kinds, mode, prefix, internal):
        """mode: kind -> None (the name as written), '_' (forgotten) or the
        class it is abstracted into."""
        fixed, base, cls_of = {}, {}, {}
        for name, k in kinds.items():
            cls = mode.get(k)
            if cls is None:
                fixed[name] = name
            elif cls == "_":
                fixed[name] = "_"
            else:
                base[name] = cls
                cls_of[name] = cls
        abstract = sorted(base)
        self.calls += 1
        if not abstract:
            return prefix + " ⇒ ".join(render(p, lambda m: fixed[m], True) for p in parts)
        lits_by_part = []
        for p in parts:
            lits = []
            literals(p, "", lits)
            rows = []
            for path, lit in lits:
                acc = set()
                names_in(lit, acc)
                # A forgotten or a written name is rendered as itself.
                rows.append((path, lit, sorted(n for n in acc if n in base)))
            lits_by_part.append(rows)

        # A name kept as written or forgotten must render as `fixed` says
        # inside the refinement too.
        lits_fixed = []
        for rows in lits_by_part:
            lits_fixed.append([(path, substitute(lit, fixed), members)
                               for path, lit, members in rows])
        parts_fixed = [substitute(p, fixed) for p in parts]

        best = self.search(parts_fixed, lits_fixed, base, abstract)
        return prefix + self.display(best, internal)

    def search(self, parts, lits, base, abstract):
        colour = self.refine(base, lits, abstract)
        classes = defaultdict(list)
        for n in abstract:
            classes[colour[n]].append(n)
        ties = [c for c in sorted(classes) if len(classes[c]) > 1]
        cost = 1
        for c in ties:
            cost *= factorial(len(classes[c]))
            if cost > TIE_LIMIT:
                break
        if cost <= TIE_LIMIT:
            return self.brute(parts, classes, base, ties)
        self.fallback += 1
        a = self.greedy(parts, lits, base, abstract, False)
        b = self.greedy(parts, lits, base, abstract, True)
        if a != b:
            self.unstable += 1
        return min(a, b)

    def brute(self, parts, classes, base, ties):
        order = {}
        for c, members in classes.items():
            if len(members) == 1:
                order[members[0]] = (c, 0)
        if not ties:
            return self.finish(parts, order, classes, base)
        best = None
        for perms in itertools.product(*(itertools.permutations(classes[c]) for c in ties)):
            o = dict(order)
            for c, perm in zip(ties, perms):
                for i, m in enumerate(perm):
                    o[m] = (c, i)
            s = self.finish(parts, o, classes, base)
            if best is None or s < best:
                best = s
        return best

    def finish(self, parts, order, classes, base):
        """The labelled string, each label carrying its class's letter."""
        cls = {}
        for c, members in classes.items():
            cls[c] = base[members[0]].split("!")[0]

        def tok(m):
            lab = order.get(m)
            if lab is None:
                return m
            return f"#{lab[0]:05d}.{lab[1]:04d}{cls[lab[0]]}#"
        return " ⇒ ".join(render(p, tok, True) for p in parts)

    def greedy(self, parts, lits, base, abstract, reverse):
        base = dict(base)
        step = 0
        while True:
            colour = self.refine(base, lits, abstract)
            classes = defaultdict(list)
            for n in abstract:
                classes[colour[n]].append(n)
            ties = [c for c in sorted(classes) if len(classes[c]) > 1]
            cost = 1
            for c in ties:
                cost *= factorial(len(classes[c]))
                if cost > 24:
                    break
            if cost <= 24:
                return self.brute(parts, classes, base, ties)
            members = sorted(classes[ties[0]], reverse=reverse)
            base[members[0]] = base[members[0]].split("!")[0] + f"!{step:05d}"
            step += 1

    def display(self, s, internal):
        """Labels renumbered by first occurrence, a count per class."""
        out = []
        seen = {}
        counters = Counter()
        i = 0
        n = len(s)
        while i < n:
            if s[i] == "#" and i + 12 < n and s[i + 6] == "." and s[i + 1:i + 6].isdigit():
                j = s.index("#", i + 1)
                lab = s[i + 1:j]
                if lab not in seen:
                    cls = lab[10:]
                    counters[cls] += 1
                    seen[lab] = pretty(cls, counters[cls], internal)
                out.append(seen[lab])
                i = j + 1
            else:
                out.append(s[i])
                i += 1
        return "".join(out)


def substitute(node, fixed):
    """A tree with every name of `fixed` turned into structure spelling what
    `fixed` gives it, so that later renderings leave it alone."""
    t = node[0]
    if t == "n":
        if node[1] in fixed:
            return ("s", fixed[node[1]])
        return node
    if t == "L":
        head = node[1]
        if head[0] == "n" and head[1] in fixed:
            # Not commutative, whatever it spells: only a word of the grammar is.
            head = ("s", "\x01" + fixed[head[1]])
        return ("L", head, tuple(substitute(k, fixed) for k in node[2]))
    return node


def clean(s):
    return s.replace("\x01", "")


MODES_UNORDERED = {
    "alphau": {"P": "P", "V": "V"},
    "relren": {"P": "P", "V": "V", "R": "R"},
    "conren": {"P": "P", "V": "V", "C": "C"},
    "ii": {"P": "P", "V": "V", "R": "R", "C": "C"},
    "iii": {"H": "R", "T": "T"},
    "iiib": {"H": "R", "T": "_"},
}
MODES_ORDERED = {
    "alpha": {"P": "P", "V": "V"},
    "reli": {"P": "P", "V": "V", "R": "R"},
    "coni": {"P": "P", "V": "V", "C": "C"},
    "i": {"P": "P", "V": "V", "R": "R", "C": "C"},
}
PURE = ("i", "ii", "iii", "iiib")


def depth(node):
    if node[0] != "L":
        return 0
    return 1 + max([depth(k) for k in node[2]] + [0])


def skeleton(rule, match, assert_, kinds):
    """Counts alone."""
    c = Counter()
    arities_m, arities_a = [], []

    def prem(node, neg):
        if node[0] == "E":
            return
        if node[0] != "L":
            c["bare-premises"] += 1
            return
        h = node[1]
        if h[0] == "s":
            w = h[1]
            if w in ("and", "or"):
                if w == "or":
                    c["or"] += 1
                for k in node[2]:
                    prem(k, neg)
                return
            if w == "absent":
                c["absent"] += 1
                for k in node[2]:
                    prem(k, True)
                return
            if w == "not":
                c["not"] += 1
                for k in node[2]:
                    if k[0] == "L" and k[1] == ("s", "eq"):
                        c["guards"] += 1
                    else:
                        arities_m.append(len(k[2]) if k[0] == "L" else 0)
                return
            if w in ("eq", "neq"):
                c["guards"] += 1
                return
            if w == "false":
                c["false-premises"] += 1
                return
            if w in ANY_STRUCT:
                c["premises"] += 1
                arities_m.append(len(node[2]))
                return
            c["macro-calls"] += 1
            return
        c["premises"] += 1
        arities_m.append(len(node[2]))

    def concl(node):
        if node[0] == "E":
            return
        if node[0] != "L":
            c["bare-conclusions"] += 1
            return
        h = node[1]
        if h[0] == "s":
            w = h[1]
            if w == "and":
                for k in node[2]:
                    concl(k)
                return
            if w == "not":
                c["neg-conclusions"] += 1
                for k in node[2]:
                    arities_a.append(len(k[2]) if k[0] == "L" else 0)
                return
            if w == "false":
                c["false"] += 1
                return
            if w == "open":
                c["open"] += 1
                return
            if w in ("eq", "neq"):
                c["guard-conclusions"] += 1
                return
            if w in ANY_STRUCT:
                c["conclusions"] += 1
                arities_a.append(len(node[2]))
                return
            c["macro-conclusions"] += 1
            return
        c["conclusions"] += 1
        arities_a.append(len(node[2]))

    prem(match, False)
    concl(assert_)
    nk = Counter(kinds.values())
    full = (f"{rule.kind} params={len(rule.params)} premises={c['premises']} not={c['not']} "
            f"absent={c['absent']} or={c['or']} guards={c['guards']} macros={c['macro-calls']} "
            f"conclusions={c['conclusions']} neg-conclusions={c['neg-conclusions']} "
            f"false={c['false']} open={c['open']} checks={c['guard-conclusions']} "
            f"premise-arities={','.join(map(str, sorted(arities_m)))} "
            f"conclusion-arities={','.join(map(str, sorted(arities_a)))} "
            f"depth={depth(match)}/{depth(assert_)} "
            f"vars={nk['V']} consts={nk['C']} relations={nk['R']}")
    negs = c["not"] + c["absent"]
    concls = c["conclusions"] + c["neg-conclusions"] + c["false"] + c["open"] + c["guard-conclusions"]
    brief = (f"{rule.kind} params={len(rule.params)} premises={c['premises']} negations={negs} "
             f"guards={c['guards']} conclusions={concls}")
    nums = {"params": len(rule.params), "premises": c["premises"], "negations": negs,
            "guards": c["guards"], "conclusions": concls, "vars": nk["V"], "consts": nk["C"],
            "relations": nk["R"], "depth_match": depth(match), "depth_assert": depth(assert_),
            "max_arity": max(arities_m + arities_a + [0]), "or": c["or"],
            "macros": c["macro-calls"], "false": c["false"], "open": c["open"]}
    return full, brief, nums


def compute_shapes(rule, macros, canon):
    params = set(rule.params)
    m = to_node(rule.match, macros)
    a = to_node(rule.assert_, macros)
    kinds, coarse = kinds_of([m, a], params)
    hr = "hrule " if rule.kind == "hrule" else ""
    prefix = f"{hr}[{len(rule.params)}] "
    s = {}
    for level, mode in MODES_ORDERED.items():
        s[level] = shape_ordered([m, a], kinds, mode, rule.params, prefix, level not in PURE)
    for level, mode in MODES_UNORDERED.items():
        if level in ("iii", "iiib"):
            # A parameter is a term or a relation like any other name here,
            # so the number of parameters is no part of the shape.
            s[level] = clean(canon.shape([m, a], coarse, mode, hr, False))
        else:
            s[level] = clean(canon.shape([m, a], kinds, mode, prefix, level not in PURE))
    s["iv"], s["ivb"], rule.skel = skeleton(rule, m, a, kinds)
    # The two clauses alone, each as a rule of one part: a name's kind is what
    # the part alone shows of it.
    for part, node in (("match", m), ("assert", a)):
        k1, _ = kinds_of([node], params)
        s[part + "-i"] = shape_ordered([node], k1, MODES_ORDERED["i"], rule.params, "", False)
        s[part + "-ii"] = clean(canon.shape([node], k1, MODES_UNORDERED["ii"], "", False))
    rule.shapes = s
    # The names in the order they are first written: two rules of one ordered
    # shape have their constants, and their relations, slot for slot.
    rule.const_seq = [n for n, k in kinds.items() if k == "C"]
    rule.rel_seq = [n for n, k in kinds.items() if k == "R"]
    return kinds


# ───────────────────────────── ways to group shapes ─────────────────────────────

SCHEMAS = (
    ("unary inclusion: R(a) ⇒ S(a)", "(R ?a)", "(S ?a)"),
    ("inclusion: R(a,b) ⇒ S(a,b)", "(R ?a ?b)", "(S ?a ?b)"),
    ("converse: R(a,b) ⇒ S(b,a)", "(R ?a ?b)", "(S ?b ?a)"),
    ("symmetry: R(a,b) ⇒ R(b,a)", "(R ?a ?b)", "(R ?b ?a)"),
    ("transitivity: R(a,b), R(b,c) ⇒ R(a,c)", "(and (R ?a ?b) (R ?b ?c))", "(R ?a ?c)"),
    ("transitivity with a guard: R(a,b), R(b,c), a≠c ⇒ R(a,c)",
     "(and (R ?a ?b) (R ?b ?c) (neq ?a ?c))", "(R ?a ?c)"),
    ("composition: R(a,b), S(b,c) ⇒ T(a,c)", "(and (R ?a ?b) (S ?b ?c))", "(T ?a ?c)"),
    ("intersection: R(a,b), S(a,b) ⇒ T(a,b)", "(and (R ?a ?b) (S ?a ?b))", "(T ?a ?b)"),
    ("unary intersection: R(a), S(a) ⇒ T(a)", "(and (R ?a) (S ?a))", "(T ?a)"),
    ("functional: R(a,b), R(a,c), b≠c ⇒ false", "(and (R ?a ?b) (R ?a ?c) (neq ?b ?c))", "(false)"),
    ("injective: R(a,c), R(b,c), a≠b ⇒ false", "(and (R ?a ?c) (R ?b ?c) (neq ?a ?b))", "(false)"),
    ("irreflexive: R(a,a) ⇒ false", "(R ?a ?a)", "(false)"),
    ("asymmetric: R(a,b), R(b,a) ⇒ false", "(and (R ?a ?b) (R ?b ?a))", "(false)"),
    ("disjoint: R(a,b), S(a,b) ⇒ false", "(and (R ?a ?b) (S ?a ?b))", "(false)"),
    ("unary disjoint: R(a), S(a) ⇒ false", "(and (R ?a) (S ?a))", "(false)"),
    ("empty: R(a,b) ⇒ false", "(R ?a ?b)", "(false)"),
    ("unary empty: R(a) ⇒ false", "(R ?a)", "(false)"),
    ("negative of a functional: R(a,b), b≠c ⇒ not R(a,c)",
     "(and (R ?a ?b) (S ?c) (neq ?b ?c))", "(not (R ?a ?c))"),
    ("default: R(a), absent S(a) ⇒ T(a)", "(and (R ?a) (absent (S ?a)))", "(T ?a)"),
    ("domain: R(a,b) ⇒ S(a)", "(R ?a ?b)", "(S ?a)"),
    ("range: R(a,b) ⇒ S(b)", "(R ?a ?b)", "(S ?b)"),
)


def schema_shapes(canon):
    """Each schema's level-(iii) shape, by the census's own labelling."""
    out = {}
    for name, m, a in SCHEMAS:
        r = Rule()
        r.kind, r.name, r.params, r.notes = "rule", "schema", [], []
        r.file, r.line = "", 0
        r.match = read_forms(m)[0][1]
        r.assert_ = read_forms(a)[0][1]
        compute_shapes(r, {}, canon)
        out.setdefault(r.shapes["iii"], name)
    return out


def gyo_acyclic(edges):
    """Is the hypergraph (a list of variable sets) alpha-acyclic?  The GYO
    reduction: drop a variable only one edge holds, drop an edge another
    contains, until nothing moves."""
    edges = [set(e) for e in edges]
    changed = True
    while changed:
        changed = False
        count = Counter(v for e in edges for v in e)
        for e in edges:
            lone = {v for v in e if count[v] == 1}
            if lone:
                e -= lone
                changed = True
        keep = []
        for i, e in enumerate(edges):
            if any((e <= f and (e != f or i > j)) for j, f in enumerate(edges) if i != j):
                changed = True
            else:
                keep.append(e)
        edges = keep
    return len(edges) <= 1


def bucket(n, cap):
    return str(n) if n < cap else f"{cap} or more"


def group_keys(rule, macros, schemas):
    """The grouping keys of a rule: name -> value, each read off the rule as
    written and none looking at what a name spells."""
    k = {}
    sk = rule.skel
    struct = lambda h: (not isinstance(h, str)) or h in HEAD_STRUCT or h in macros

    prem, neg, guards = [], [], []      # positive premises at any depth of and/or

    def walk(x):
        if not isinstance(x, list) or not x or isinstance(x[0], list):
            return
        h = x[0]
        if h in ("and", "or"):
            for y in x[1:]:
                walk(y)
        elif h in ("absent", "not"):
            neg.append(x)
        elif h in ("eq", "neq"):
            guards.append(x)
        elif h == "false" or h in macros:
            pass
        else:
            prem.append(x)
    walk(rule.match)
    concl, others = [], 0

    def cwalk(x):
        nonlocal others
        if not isinstance(x, list) or not x or isinstance(x[0], list):
            return
        h = x[0]
        if h == "and":
            for y in x[1:]:
                cwalk(y)
        elif h == "not" and len(x) == 2 and isinstance(x[1], list) and x[1] \
                and not struct(x[1][0]):
            concl.append(x[1])      # a stored negation is a fact concluded
        elif h in ("not", "false", "open", "eq", "neq") or h in macros:
            others += 1
        else:
            concl.append(x)
    cwalk(rule.assert_)

    heads = set()
    arities = []

    def atoms(x, top):
        if not isinstance(x, list) or not x or isinstance(x[0], list):
            return
        h = x[0]
        if not struct(h):
            heads.add(h)
            arities.append(len(x) - 1)
        for y in x[1:]:
            atoms(y, False)
    atoms(rule.match, True)
    atoms(rule.assert_, True)
    k["relations"] = bucket(len(heads), 8)
    k["arity profile"] = " ".join(str(a) for a in sorted(arities)) if len(arities) <= 12 \
        else f"{len(arities)} atoms"
    total = sum(arities)
    k["arguments in all"] = str(total) if total < 12 else ("12 to 23" if total < 24 else "24 or more")
    k["widest atom"] = bucket(max(arities + [0]), 9)
    k["premises × conclusions"] = f"{bucket(len(prem), 6)} × {bucket(len(concl) + others, 4)}"
    nvars = sk["vars"] + sk["params"]
    k["variables"] = bucket(nvars, 10)
    if prem:
        ratio = nvars / len(prem)
        k["variables a premise"] = ("under 1" if ratio < 1 else "1 to under 2" if ratio < 2
                                    else "2 to under 3" if ratio < 3 else "3 to under 5"
                                    if ratio < 5 else "5 or more")
    else:
        k["variables a premise"] = "no premise"
    k["constants"] = bucket(sk["consts"], 5)
    k["parameters"] = str(sk["params"])
    nots = sum(1 for x in neg if x[0] == "not")
    k["negation profile"] = f"not {nots}, absent {len(neg) - nots}" + (
        f", macro {sk['macros']}" if sk["macros"] else "")
    eqs = sum(1 for x in guards if x[0] == "eq")
    k["guard profile"] = f"eq {eqs}, neq {len(guards) - eqs}"
    dm = max([sexpr_depth(p) for p in prem] + [0])
    da = max([sexpr_depth(c) for c in concl] + [0])
    k["nesting"] = f"premises {bucket(dm, 4)}, conclusions {bucket(da, 4)}"
    if not concl:
        k["builds or takes apart"] = "concludes no fact"
    elif not prem:
        k["builds or takes apart"] = "no premise"
    elif da > dm:
        k["builds or takes apart"] = "builds: a conclusion nested deeper than any premise"
    elif da < dm:
        k["builds or takes apart"] = "takes apart: the premises nested deeper"
    elif da == 1:
        k["builds or takes apart"] = "flat: nothing nested"
    else:
        k["builds or takes apart"] = "level: nested alike"

    # the join structure of the positive premises
    # A parameter is bound by the activator before any premise is matched, so
    # it joins nothing.
    pset = set(rule.params)
    varsets = [set(v for v in flatten(p) if is_var(v) and v not in pset) for p in prem]
    n = len(prem)
    if n == 0:
        js, comps = "no premise", 0
    else:
        parent = list(range(n))

        def find(a):
            while parent[a] != a:
                a = parent[a]
            return a
        deg = [0] * n
        for i in range(n):
            for j in range(i + 1, n):
                if varsets[i] & varsets[j]:
                    deg[i] += 1
                    deg[j] += 1
                    parent[find(i)] = find(j)
        comps = len(set(find(i) for i in range(n)))
        if n == 1:
            js = "one premise"
        elif comps > 1:
            js = "a product: premises that share no variable"
        elif not gyo_acyclic(varsets):
            js = "cyclic"
        elif n == 2:
            js = "a pair"
        elif varsets and set.intersection(*varsets):
            js = "a star: one variable in every premise"
        elif max(deg) <= 2 and sum(deg) == 2 * (n - 1):
            js = "a chain"
        else:
            js = "a tree"
    k["join structure"] = js
    k["join components"] = bucket(comps, 4)

    # recursion: conclusions whose relation a premise reads
    ph = set(p[0] for p in prem)
    hit = sum(1 for c in concl if c[0] in ph)
    if not concl:
        k["feeds itself"] = "concludes no fact"
    elif hit == 0:
        k["feeds itself"] = "no conclusion's relation is read"
    elif hit == len(concl):
        k["feeds itself"] = "every conclusion's relation is read"
    else:
        k["feeds itself"] = "some conclusions' relations are read"

    # where the conclusions' variables come from
    cvars = set(v for c in concl for v in flatten(c) if is_var(v))
    params = set(rule.params)
    bound = set().union(*varsets) if varsets else set()
    if not concl:
        prov = "concludes no fact"
    elif not cvars:
        prov = "ground: no variable in a conclusion"
    elif cvars - bound - params:
        prov = "a variable no positive premise binds"
    else:
        free = cvars - params
        owners = [i for i, vs in enumerate(varsets) if free and free <= vs]
        if not free:
            prov = "parameters alone"
        elif owners and len(concl) == 1 and len(prem) == 1:
            p, c = prem[0], concl[0]
            pa, ca = p[1:], c[1:]
            flat = all(not isinstance(x, list) for x in pa + ca)
            if not flat:
                prov = "one premise, one conclusion: nested"
            elif any(is_name(x) for x in ca):
                prov = "one premise, one conclusion: a constant put in"
            elif ca == pa:
                prov = "one premise, one conclusion: a copy"
            elif sorted(ca) == sorted(pa):
                prov = "one premise, one conclusion: a permutation"
            elif set(ca) <= set(pa):
                prov = "one premise, one conclusion: a projection"
            else:
                prov = "one premise, one conclusion: with a parameter"
        elif owners:
            prov = "all from one premise"
        else:
            prov = "from several premises (a join's output)"
    k["conclusion variables"] = prov
    k["schema"] = schemas.get(rule.shapes["iii"], "none of the schemas") \
        if rule.kind == "rule" else "an hrule"
    k["rule or hrule"] = rule.kind
    rule.keys = k


def sexpr_depth(x):
    if not isinstance(x, list):
        return 0
    return 1 + max([sexpr_depth(y) for y in x] + [0])


KEY_DEFS = (
    ("relations", "the number of distinct heads of forms in the two clauses, a name or a variable, at any depth; structure words are not counted"),
    ("arity profile", "the multiset of the numbers of arguments of every form headed by a relation, at any depth, both clauses"),
    ("arguments in all", "the sum of that multiset"),
    ("widest atom", "its greatest member"),
    ("premises × conclusions", "positive premises (at any depth of `and`/`or`) by conclusions (a fact, a stored negation, `(false)`, `(open …)` and a concluded guard each one)"),
    ("variables", "distinct variables, parameters among them"),
    ("variables a premise", "that number over the number of positive premises"),
    ("constants", "distinct names that head no form"),
    ("parameters", "the length of the parameter list"),
    ("negation profile", "how many `not`, how many `absent`, how many macro calls the `:match` holds"),
    ("guard profile", "how many `eq`, how many `neq` the `:match` holds"),
    ("nesting", "the depth of the deepest positive premise and of the deepest conclusion, a flat atom 1"),
    ("builds or takes apart", "the two depths compared: a conclusion deeper than every premise *builds* (a constructor), the reverse *takes apart*"),
    ("join structure", "the positive premises as a hypergraph, a premise the set of its variables: no premise, one, a product (some two share nothing, the graph disconnected), cyclic (the GYO reduction does not empty it), else a pair, a star (one variable in every premise), a chain (each premise shares with at most two) or a tree; guards and negations are not edges, and a parameter is no shared variable, being bound before any premise is matched"),
    ("join components", "the connected components of that graph: 2 or more is a cross product"),
    ("feeds itself", "whether a conclusion's relation is the relation of a positive premise of the same rule"),
    ("conclusion variables", "where the variables of the positive conclusions come from: none (ground), parameters alone, one premise (and for a rule of one premise and one conclusion, flat: a copy, a permutation, a projection, a constant put in), several premises, or a variable no positive premise binds"),
    ("schema", "the level-(iii) shape is exactly one of the schemas the script lists (`SCHEMAS`: inclusion, converse, symmetry, transitivity, composition, functional, injective, …), a relation a name or a parameter alike"),
    ("rule or hrule", "the declarator"),
)


# ───────────────────────────── connections: what a rule holds ─────────────────────────────

def expand(x, macros, fuel=32):
    """Macro calls of `macros` expanded, for the connections alone."""
    if not isinstance(x, list) or not x:
        return x
    head = x[0]
    if isinstance(head, str) and not isinstance(head, Str) and head in macros and fuel > 0:
        params, body = macros[head]
        if len(params) == len(x) - 1:
            env = dict(zip(params, x[1:]))
            return expand(subst_sexpr(body, env), macros, fuel - 1)
    return [expand(k, macros, fuel) for k in x]


def subst_sexpr(x, env):
    if isinstance(x, list):
        return [subst_sexpr(k, env) for k in x]
    if isinstance(x, str) and not isinstance(x, Str) and x in env:
        return env[x]
    return x


def atom_key(tree):
    """An atom with its variables renamed by first occurrence: (string, tree)."""
    env = {}

    def go(x):
        if isinstance(x, list):
            return [go(k) for k in x]
        if is_var(x):
            if x not in env:
                env[x] = f"?{len(env)}"
            return env[x]
        return str(x)
    t = go(tree)
    return show(t), t


def show(x):
    if isinstance(x, list):
        return "(" + " ".join(show(k) for k in x) + ")"
    return x


class Conn:
    """What the connections read of one rule."""
    __slots__ = ("prem", "concl", "guards", "false", "open", "macro_left", "rels", "consts",
                 "positions", "names")


def collect(rule, macros):
    c = Conn()
    c.prem = []      # (polarity '+'/'-', absent depth, key string, tree)
    c.concl = []     # (polarity, key string, tree)
    c.guards, c.false, c.open, c.macro_left = 0, False, False, 0
    match = expand(rule.match, macros)

    def prem(x, d):
        if not isinstance(x, list) or not x or isinstance(x[0], list):
            return
        h = x[0]
        if h in ("and", "or"):
            for k in x[1:]:
                prem(k, d)
        elif h == "absent":
            for k in x[1:]:
                prem(k, d + 1)
        elif h == "not":
            for k in x[1:]:
                if isinstance(k, list) and k and k[0] == "eq":
                    c.guards += 1
                elif isinstance(k, list) and k:
                    s, t = atom_key(k)
                    c.prem.append(("-", d, s, t))
        elif h in ("eq", "neq"):
            c.guards += 1
        elif h == "false":
            pass
        elif isinstance(h, str) and h in macros:
            c.macro_left += 1
        else:
            s, t = atom_key(x)
            c.prem.append(("+", d, s, t))

    def concl(x):
        if not isinstance(x, list) or not x:
            return
        h = x[0]
        if h == "and":
            for k in x[1:]:
                concl(k)
        elif h == "not":
            for k in x[1:]:
                if isinstance(k, list) and k and k[0] != "eq":
                    s, t = atom_key(k)
                    c.concl.append(("-", s, t))
        elif h == "false":
            c.false = True
        elif h == "open":
            c.open = True
        elif h in ("eq", "neq"):
            pass
        elif isinstance(h, list):
            pass
        else:
            s, t = atom_key(x)
            c.concl.append(("+", s, t))

    prem(match, 0)
    concl(rule.assert_)

    # Names: every name a form is headed by (a relation), every other name (a
    # constant), and where a constant sits.
    c.rels, c.consts, c.positions = set(), set(), set()
    c.names = set()

    def names(x, path, role):
        if not isinstance(x, list) or not x:
            return
        h = x[0]
        if isinstance(h, list):
            for k in x:
                names(k, path, role)
            return
        structural = h in HEAD_STRUCT or h in macros or h in ANY_STRUCT
        if is_name(h) and not structural:
            c.rels.add(h)
        hh = h if not is_var(h) else "?"
        for i, k in enumerate(x[1:]):
            if isinstance(k, list):
                names(k, path if (h in HEAD_STRUCT or h in macros) else path + ((hh, i),), role)
            elif is_name(k) and k not in ANY_STRUCT:
                c.consts.add(k)
                if not (h in HEAD_STRUCT or h in macros):
                    c.positions.add((role, path + ((hh, i),), k))
    names(match, (), "match")
    names(rule.assert_, (), "assert")
    c.names = c.rels | c.consts

    return c


def joins_of(rule, macros):
    """The joins of a rule's match: ((R, i), (S, j)) for a variable shared by
    two positive premises at the top level."""
    match = expand(rule.match, macros)
    atoms = []
    if isinstance(match, list) and match and match[0] == "and":
        items = match[1:]
    else:
        items = [match]
    for x in items:
        if isinstance(x, list) and x and isinstance(x[0], str) and x[0] not in HEAD_STRUCT \
                and x[0] not in macros:
            atoms.append(x)
    out = set()
    for a in range(len(atoms)):
        for b in range(a + 1, len(atoms)):
            xa, xb = atoms[a], atoms[b]
            ha = xa[0] if is_name(xa[0]) else "?"
            hb = xb[0] if is_name(xb[0]) else "?"
            for i, u in enumerate(xa[1:]):
                if not is_var(u):
                    continue
                for j, v in enumerate(xb[1:]):
                    if u == v:
                        p, q = (ha, i), (hb, j)
                        out.add((p, q) if p <= q else (q, p))
    return out


def unify(a, b):
    """Do two atoms unify, their variables apart?  `b`'s are renamed."""
    def rename(x):
        if isinstance(x, list):
            return [rename(k) for k in x]
        return "?r" + x[1:] if x.startswith("?") else x
    b = rename(b)
    subst = {}

    def walk(t):
        while isinstance(t, str) and t.startswith("?") and t in subst:
            t = subst[t]
        return t

    def occurs(v, t):
        t = walk(t)
        if isinstance(t, list):
            return any(occurs(v, k) for k in t)
        return t == v
    stack = [(a, b)]
    while stack:
        x, y = stack.pop()
        x, y = walk(x), walk(y)
        if isinstance(x, str) and isinstance(y, str) and x == y:
            continue
        if isinstance(x, str) and x.startswith("?"):
            if occurs(x, y):
                return False
            subst[x] = y
        elif isinstance(y, str) and y.startswith("?"):
            if occurs(y, x):
                return False
            subst[y] = x
        elif isinstance(x, list) and isinstance(y, list) and len(x) == len(y):
            stack.extend(zip(x, y))
        else:
            return False
    return True


# ───────────────────────────── bit sets ─────────────────────────────

def bits(mask):
    out = []
    while mask:
        low = mask & -mask
        out.append(low.bit_length() - 1)
        mask ^= low
    return out


def sym_from_keys(keysets):
    """Rules that share a key: adjacency masks, no self."""
    index = defaultdict(int)
    for i, keys in enumerate(keysets):
        for k in keys:
            index[k] |= 1 << i
    adj = []
    for i, keys in enumerate(keysets):
        m = 0
        for k in keys:
            m |= index[k]
        adj.append(m & ~(1 << i))
    return adj


def dir_from_keys(out_keys, in_keys):
    """A -> B where a key of out_keys[A] is in in_keys[B]; self-loops kept."""
    index = defaultdict(int)
    for i, keys in enumerate(in_keys):
        for k in keys:
            index[k] |= 1 << i
    adj = []
    for keys in out_keys:
        m = 0
        for k in keys:
            m |= index.get(k, 0)
        adj.append(m)
    return adj


class Unifier:
    def __init__(self):
        self.cache = {}

    def link(self, src, dst, generic):
        """A -> B where an atom of src[A] unifies with an atom of dst[B].
        `generic` False: both atoms headed by a name.  True: at least one
        headed by a variable.  src/dst: per rule, a list of (string, tree)."""
        by_head = defaultdict(dict)   # (head, arity) -> {string: tree}
        by_arity = defaultdict(dict)
        owners = defaultdict(int)
        for i, atoms in enumerate(dst):
            for s, t in atoms:
                owners[s] |= 1 << i
                h = t[0] if is_name(t[0]) else None
                by_head[(h, len(t))][s] = t
                by_arity[len(t)][s] = t
        reach = {}
        adj = []
        for atoms in src:
            m = 0
            for s, t in atoms:
                if s not in reach:
                    r = 0
                    h = t[0] if is_name(t[0]) else None
                    if generic:
                        cands = by_arity[len(t)] if h is None else by_head[(None, len(t))]
                    else:
                        cands = {} if h is None else by_head[(h, len(t))]
                    for s2, t2 in cands.items():
                        key = (s, s2)
                        ok = self.cache.get(key)
                        if ok is None:
                            ok = unify(t, t2)
                            self.cache[key] = ok
                            self.cache[(s2, s)] = ok
                        if ok:
                            r |= owners[s2]
                    reach[s] = r
                m |= reach[s]
            adj.append(m)
        return adj


# ───────────────────────────── subsumption ─────────────────────────────

def flat_literals(rule, macros):
    """A match as a flat list of literals, or None where it is not one."""
    match = expand(rule.match, macros)
    if not isinstance(match, list) or not match:
        return []
    items = match[1:] if match[0] == "and" else [match]
    out = []
    for x in items:
        if not isinstance(x, list) or not x or isinstance(x[0], list):
            return None
        h = x[0]
        if h in ("and", "or") or h in macros:
            return None
        if h in ("absent", "not"):
            if len(x) != 2 or not isinstance(x[1], list) or not x[1]:
                return None
            inner = x[1]
            if isinstance(inner[0], list) or inner[0] in ("and", "or", "absent", "not"):
                return None
            out.append((h, inner))
        elif h in ("eq", "neq"):
            out.append((h, x))
        elif h == "false":
            out.append(("false", x))
        else:
            out.append(("+", x))
    return out


def subsumes(a, b, budget):
    """Is there a substitution of `a`'s variables under which every literal
    of `a` is a literal of `b`?  `b`'s variables are constants here."""
    a = sorted(a, key=lambda l: -sum(1 for _ in flatten(l[1])))
    steps = [0]

    def match_term(x, y, env):
        if isinstance(x, list):
            if not isinstance(y, list) or len(x) != len(y):
                return False
            for u, v in zip(x, y):
                if not match_term(u, v, env):
                    return False
            return True
        if x.startswith("?"):
            if x in env:
                return env[x] == y
            env[x] = y
            return True
        return x == y

    def go(i, env):
        steps[0] += 1
        if steps[0] > budget:
            raise TimeoutError
        if i == len(a):
            return True
        sign, x = a[i]
        for sign2, y in b:
            if sign != sign2:
                continue
            e = dict(env)
            if match_term(x, y, e):
                if go(i + 1, e):
                    return True
            if sign in ("eq", "neq") and len(x) == 3 and len(y) == 3:
                e = dict(env)
                if match_term([x[0], x[2], x[1]], y, e) and go(i + 1, e):
                    return True
        return False
    return go(0, {})


def flatten(x):
    if isinstance(x, list):
        for k in x:
            yield from flatten(k)
    else:
        yield x


def freeze(x):
    """`b`'s side of a subsumption: its variables made constants."""
    if isinstance(x, list):
        return [freeze(k) for k in x]
    return "\x02" + x[1:] if x.startswith("?") else x


# ───────────────────────────── graphs ─────────────────────────────

def sccs(n, succ):
    """Tarjan, iterative.  `succ(v)` a list.  Components in reverse topological order."""
    index = [None] * n
    low = [0] * n
    on = [False] * n
    stack, comps = [], []
    counter = 0
    for root in range(n):
        if index[root] is not None:
            continue
        work = [(root, iter(succ(root)))]
        index[root] = low[root] = counter
        counter += 1
        stack.append(root)
        on[root] = True
        while work:
            v, it = work[-1]
            advanced = False
            for w in it:
                if index[w] is None:
                    index[w] = low[w] = counter
                    counter += 1
                    stack.append(w)
                    on[w] = True
                    work.append((w, iter(succ(w))))
                    advanced = True
                    break
                if on[w]:
                    low[v] = min(low[v], index[w])
            if advanced:
                continue
            work.pop()
            if work:
                u = work[-1][0]
                low[u] = min(low[u], low[v])
            if low[v] == index[v]:
                comp = []
                while True:
                    w = stack.pop()
                    on[w] = False
                    comp.append(w)
                    if w == v:
                        break
                comps.append(sorted(comp))
    return comps


def graph_stats(nodes, adj):
    """Statistics of the digraph `adj` (masks over global ids) on `nodes`."""
    pos = {v: i for i, v in enumerate(nodes)}
    mask = 0
    for v in nodes:
        mask |= 1 << v
    succ = [[pos[w] for w in bits(adj[v] & mask)] for v in nodes]
    n = len(nodes)
    loops = sum(1 for i in range(n) if i in succ[i])
    edges = sum(len(s) for s in succ) - loops
    indeg = [0] * n
    for i, s in enumerate(succ):
        for w in s:
            if w != i:
                indeg[w] += 1
    outdeg = [len([w for w in s if w != i]) for i, s in enumerate(succ)]
    comps = sccs(n, lambda v: succ[v])
    comp_of = [0] * n
    for ci, comp in enumerate(comps):
        for v in comp:
            comp_of[v] = ci
    # Tarjan gives the components in reverse topological order.
    longest = [0] * len(comps)
    for ci, comp in enumerate(comps):
        best = 0
        for v in comp:
            for w in succ[v]:
                if comp_of[w] != ci:
                    best = max(best, 1 + longest[comp_of[w]])
        longest[ci] = best
    nontrivial = [c for c in comps if len(c) > 1]
    # Weakly connected components.
    parent = list(range(n))

    def find(x):
        while parent[x] != x:
            parent[x] = parent[parent[x]]
            x = parent[x]
        return x
    for i, s in enumerate(succ):
        for w in s:
            a, b = find(i), find(w)
            if a != b:
                parent[a] = b
    weak = Counter(find(i) for i in range(n))
    isolated = sum(1 for i in range(n) if indeg[i] == 0 and outdeg[i] == 0)
    return {
        "nodes": n, "edges": edges, "self_loops": loops,
        "sources": sum(1 for i in range(n) if indeg[i] == 0 and outdeg[i] > 0),
        "sinks": sum(1 for i in range(n) if outdeg[i] == 0 and indeg[i] > 0),
        "isolated": isolated,
        "sccs_nontrivial": len(nontrivial),
        "scc_largest": max([len(c) for c in comps] + [0]),
        "rules_in_cycles": sum(len(c) for c in nontrivial)
        + sum(1 for c in comps if len(c) == 1 and c[0] in succ[c[0]]),
        "longest_path": max(longest + [0]),
        "weak_components": len(weak),
        "weak_largest": max(list(weak.values()) + [0]),
        "indeg": indeg, "outdeg": outdeg,
        "largest_scc_members": [nodes[v] for v in max(comps, key=lambda c: (len(c), [-x for x in c]))]
        if comps else [],
    }


def quantiles(values):
    if not values:
        return "—"
    v = sorted(values)
    n = len(v)

    def q(p):
        return v[min(n - 1, int(p * n))]
    mean = sum(v) / n
    return f"{v[0]} / {q(0.5)} / {mean:.1f} / {q(0.9)} / {v[-1]}"


# ───────────────────────────── the census ─────────────────────────────

def tracked():
    out = subprocess.run(["git", "ls-files", "--"] + [r + "/" for r in ROOTS], cwd=ROOT,
                         check=True, capture_output=True, text=True).stdout
    return sorted(p for p in out.splitlines() if p.endswith(".ein"))


def resolve(imp, path):
    """The module file an import names, or None: `std.X` is `stdlib/X.ein`,
    any other name a file beside the importing one."""
    if imp.startswith("std."):
        cand = "stdlib/" + imp[4:].replace(".", "/") + ".ein"
    else:
        cand = os.path.normpath(os.path.join(os.path.dirname(path), imp + ".ein"))
    return cand


def md_table(header, rows, align=None):
    out = ["| " + " | ".join(header) + " |"]
    if align is None:
        align = ["---"] * len(header)
    out.append("|" + "|".join(align) + "|")
    for r in rows:
        out.append("| " + " | ".join(str(x) for x in r) + " |")
    return "\n".join(out)


def code(s):
    return "`" + s.replace("|", "\\|") + "`"


def fmt(n):
    """An integer with thin groups, as the repo's pages write them."""
    s = f"{n:,}".replace(",", " ")
    return s


def coverage(counts, total):
    """How many shapes, largest first, cover 50 % and 90 % of the rules."""
    acc, k50, k90 = 0, None, None
    for k, c in enumerate(sorted(counts, reverse=True), 1):
        acc += c
        if k50 is None and acc * 2 >= total:
            k50 = k
        if k90 is None and acc * 10 >= total * 9:
            k90 = k
    return k50 or 0, k90 or 0


def main():
    ap = argparse.ArgumentParser(description=__doc__.split("\n")[0])
    ap.add_argument("--out", default=os.path.join(ROOT, "build", "rule_census"))
    ap.add_argument("--top", type=int, default=30)
    ap.add_argument("--strata", action="store_true",
                    help="run `ein saturate --dump` on every file and set its strata beside the static graph's")
    ap.add_argument("--bin", default=os.environ.get("EIN_BIN",
                    os.path.join(ROOT, "ein.rs", "target", "release", "ein")))
    ap.add_argument("--svg", action="store_true", help="render the DOT files with Graphviz")
    ap.add_argument("--page", metavar="FILE",
                    help="rewrite the generated region of the results page with the summary (implies --strata)")
    args = ap.parse_args()
    if args.page:
        args.strata = True
    os.makedirs(args.out, exist_ok=True)
    S = []   # the summary's lines

    def say(*lines):
        S.extend(lines)

    paths = tracked()
    files = {p: read_file(p) for p in paths}
    unread = sorted(p for p, f in files.items() if f.error)
    census_files = [files[p] for p in paths if not files[p].broken and not files[p].error]
    broken_files = [files[p] for p in paths if files[p].broken and not files[p].error]

    # Macros in effect per file: its own, and std.macro's where it reaches it.
    def modules_of(path, seen=None):
        seen = seen if seen is not None else {}
        for imp, symbols, alias in files[path].imports:
            mod = resolve(imp, path)
            if mod in files and mod not in seen and not files[mod].error:
                seen[mod] = True
                modules_of(mod, seen)
        return seen

    std_macro = files.get("stdlib/macro.ein")
    macros_of = {}
    for f in census_files + broken_files:
        m = dict(f.macros)
        mods = modules_of(f.path)
        for mod in mods:
            for name, body in files[mod].macros.items():
                m.setdefault(name, body)
        macros_of[f.path] = m

    rules = []
    for f in census_files:
        for r in f.rules:
            r.idx = len(rules)
            r.root, r.group = group_of(f.path)
            r.macros = macros_of[f.path]
            rules.append(r)
    N = len(rules)
    canon = Canon()
    for r in rules:
        compute_shapes(r, r.macros, canon)
        r.conn = collect(r, r.macros)
    schemas = schema_shapes(Canon())
    for r in rules:
        group_keys(r, r.macros, schemas)

    # ── the corpus ──
    say("## The corpus measured", "")
    by_root = defaultdict(lambda: Counter())
    for f in census_files:
        root, _ = group_of(f.path)
        by_root[root]["files"] += 1
        if f.rules:
            by_root[root]["files with a rule"] += 1
        for r in f.rules:
            by_root[root][r.kind] += 1
            if r.params:
                by_root[root]["with parameters"] += 1
    rows = []
    tot = Counter()
    for root in ROOTS:
        c = by_root[root]
        tot.update(c)
        rows.append([f"`{root}/`", c["files"], c["files with a rule"], c["rule"], c["hrule"],
                     c["rule"] + c["hrule"], c["with parameters"]])
    rows.append(["**all**", tot["files"], tot["files with a rule"], tot["rule"], tot["hrule"],
                 tot["rule"] + tot["hrule"], tot["with parameters"]])
    say(md_table(["root", "files", "files with a rule", "`rule`", "`hrule`", "both",
                  "with parameters"], rows, ["---"] + ["---:"] * 6), "")
    nb = sum(len(f.rules) for f in broken_files)
    say(f"- tracked `.ein` files under the three roots: {len(paths)}; in the census: "
        f"{len(census_files)}; under `{BROKEN}` and left out: {len(broken_files) + sum(1 for p in unread if p.startswith(BROKEN))}, "
        f"holding {nb} rules the reader can read; files the reader cannot read: {len(unread)}"
        + (" (" + ", ".join(f"`{p}`" for p in unread) + ")" if unread else "") + ".")
    stdlib_rules = sum(1 for r in rules if r.root == "stdlib" and r.kind == "rule")
    say(f"- `rule` forms in `stdlib/`: {stdlib_rules} (the check: `cat stdlib/*.ein | grep -c '^(rule '`).")
    notes = Counter(n.split(":")[0] for r in rules for n in r.notes)
    say("- rules with a clause missing or written twice: "
        + (", ".join(f"{k} {v}" for k, v in sorted(notes.items())) or "none") + ".")
    malformed = sum(len(f.decls) for f in census_files)
    say(f"- `rule`/`hrule` forms with no name or no parameter list, left out: {malformed}.")
    distinct = len(set(r.shapes["alphau"] for r in rules))
    distinct_text = len(set(r.shapes["alpha"] for r in rules))
    say(f"- distinct rules: {distinct_text} up to a renaming of variables with the order as written "
        f"(`alpha`), {distinct} with conjuncts in any order (`alphau`); the other "
        f"{N - distinct} of the {N} are a rule some other file also writes.")
    say(f"- canonical labelling: {canon.fallback} labellings (of {canon.calls}) took the "
        f"individualise-and-refine fallback, and on {canon.unstable} of them its two runs differ.", "")

    # by group
    groups = defaultdict(list)
    for r in rules:
        groups[r.group].append(r)
    gfiles = Counter()
    for f in census_files:
        gfiles[group_of(f.path)[1]] += 1
    shape_groups = defaultdict(set)
    for r in rules:
        shape_groups[r.shapes["ii"]].add(r.group)
    rows = []
    for g in sorted(gfiles, key=lambda g: (not g.startswith("stdlib"), g)):
        rs = groups.get(g, [])
        if not rs:
            continue
        sh = set(r.shapes["ii"] for r in rs)
        own = sum(1 for s in sh if len(shape_groups[s]) == 1)
        rows.append([f"`{g}`", gfiles[g], len(rs), sum(1 for r in rs if r.kind == "hrule"),
                     len(set(r.shapes["alphau"] for r in rs)), len(sh), own])
    say("### By directory", "",
        "A rule's group is `stdlib`, `tests/stdlib/<module>`, or the first directory under "
        "`examples/`. *Distinct rules* is by `alphau`; *shapes* is level (ii); *its own* counts the "
        "shapes no other group has.", "",
        md_table(["group", "files", "rules", "of them `hrule`", "distinct rules", "shapes (ii)",
                  "its own"], rows, ["---"] + ["---:"] * 6), "")
    nogroup = sorted(g for g in gfiles if not groups.get(g))
    say("Groups with files and no rule: " + ", ".join(f"`{g}` ({gfiles[g]})" for g in nogroup) + ".", "")

    # per stdlib module
    rows = []
    for f in census_files:
        if f.path.startswith("stdlib/"):
            rs = f.rules
            rows.append([f"`{f.path}`", len(rs), sum(1 for r in rs if r.params),
                         len(set(r.shapes["i"] for r in rs)), len(set(r.shapes["ii"] for r in rs)),
                         len(set(r.shapes["iii"] for r in rs)), len(set(r.shapes["iiib"] for r in rs)),
                         len(set(r.shapes["iv"] for r in rs)), len(f.facts)])
    say("### The stdlib's modules", "",
        md_table(["module", "rules", "with parameters", "shapes (i)", "(ii)", "(iii)", "(iii′)",
                  "(iv)", "facts written"], rows, ["---"] + ["---:"] * 8), "")

    # ── A. shapes ──
    say("## A. Shapes", "")
    LEVELS = [("alpha", "the same text: variables renamed"),
              ("alphau", "the same rule: variables renamed, conjuncts in any order"),
              ("relren", "one rule under a renaming of relations"),
              ("conren", "one rule under a renaming of constants"),
              ("i", "(i) every name abstracted, order as written"),
              ("ii", "(ii) every name abstracted, any order"),
              ("iii", "(iii) constants, variables and parameters one kind"),
              ("iiib", "(iii′) relations alone; terms forgotten"),
              ("iv", "(iv) the skeleton, every count"),
              ("ivb", "(iv′) the skeleton, five counts"),
              ("match-i", "`:match` alone, as written"),
              ("match-ii", "`:match` alone, any order"),
              ("assert-i", "`:assert` alone, as written"),
              ("assert-ii", "`:assert` alone, any order")]
    rows = []
    level_classes = {}
    for level, what in LEVELS:
        cls = defaultdict(list)
        for r in rules:
            cls[r.shapes[level]].append(r)
        level_classes[level] = cls
        counts = [len(v) for v in cls.values()]
        k50, k90 = coverage(counts, N)
        singles = sum(1 for c in counts if c == 1)
        # The same over distinct rules: a shape's size counted in rule texts.
        dcounts = [len(set(r.shapes["alphau"] for r in v)) for v in cls.values()]
        d50, d90 = coverage(dcounts, distinct)
        dsingles = sum(1 for c in dcounts if c == 1)
        rows.append([what, N, len(cls), k50, k90, singles, max(counts), d50, d90, dsingles])
    say(md_table(["level", "rules", "distinct shapes", "shapes covering 50 %", "90 %",
                  "shapes of one rule", "largest shape", "over distinct rules: 50 %", "90 %",
                  "of one distinct rule"], rows, ["---"] + ["---:"] * 9), "",
        f"*Over distinct rules* counts a shape's size in rule texts (`alphau`, {distinct} of them) "
        "and not in occurrences, so a rule that forty files copy weighs one.", "")

    def examples(rs, k=3):
        seen, out = set(), []
        for r in sorted(rs, key=lambda r: (r.root != "stdlib", r.file, r.line)):
            key = r.shapes["alphau"]
            if key in seen:
                continue
            seen.add(key)
            out.append(f"`{r.file}:{r.name}`")
            if len(out) == k:
                break
        return ", ".join(out)

    def top_table(level, title, top):
        cls = level_classes[level]
        ranked = sorted(cls.items(), key=lambda kv: (-len(kv[1]), kv[0]))
        rows = []
        for rank, (shape, rs) in enumerate(ranked[:top], 1):
            roots = Counter(r.root for r in rs)
            rows.append([rank, len(rs), len(set(r.shapes["alphau"] for r in rs)),
                         len(set(r.file for r in rs)),
                         "/".join(str(roots[x]) for x in ROOTS), code(shape), examples(rs)])
        say(f"### {title}", "",
            md_table(["#", "rules", "distinct", "files", "stdlib/examples/tests", "shape",
                      "examples"], rows, ["---:", "---:", "---:", "---:", "---", "---", "---"]), "")
        with open(os.path.join(args.out, f"shapes_{level}.tsv"), "w", encoding="utf-8") as fh:
            fh.write("rank\trules\tdistinct_rules\tfiles\tstdlib\texamples\ttests\tshape\tmembers\n")
            for rank, (shape, rs) in enumerate(ranked, 1):
                roots = Counter(r.root for r in rs)
                members = " ".join(sorted(r.ref() for r in rs))
                fh.write(f"{rank}\t{len(rs)}\t{len(set(r.shapes['alphau'] for r in rs))}\t"
                         f"{len(set(r.file for r in rs))}\t{roots['stdlib']}\t{roots['examples']}\t"
                         f"{roots['tests']}\t{shape}\t{members}\n")

    say("Reading a shape: `[k]` is the number of parameters, `hrule` marks an hrule, `⇒` stands "
        "between the `:match` and the `:assert`; `R1` a relation, `c1` a constant, `?a` a variable, "
        "`?P1` a parameter, `t1` a term of level (iii), `_` a forgotten term. In the tables' "
        "*distinct* column a shape's rules are counted up to `alphau`.", "")
    top_table("i", "Level (i): exact shape, order as written", args.top)
    top_table("ii", "Level (ii): any order of conjuncts", args.top)
    top_table("iii", "Level (iii): constants, variables and parameters one kind", args.top)
    top_table("iiib", "Level (iii′): the relations' pattern alone", args.top)
    top_table("iv", "Level (iv): the skeleton", 15)
    top_table("ivb", "Level (iv′): five counts", 15)
    top_table("match-ii", "The `:match` alone (any order)", 20)
    top_table("assert-ii", "The `:assert` alone (any order)", 20)
    for level in ("alpha", "alphau", "relren", "conren", "match-i", "assert-i"):
        cls = level_classes[level]
        ranked = sorted(cls.items(), key=lambda kv: (-len(kv[1]), kv[0]))
        with open(os.path.join(args.out, f"shapes_{level}.tsv"), "w", encoding="utf-8") as fh:
            fh.write("rank\trules\tdistinct_rules\tfiles\tshape\tmembers\n")
            for rank, (shape, rs) in enumerate(ranked, 1):
                fh.write(f"{rank}\t{len(rs)}\t{len(set(r.shapes['alphau'] for r in rs))}\t"
                         f"{len(set(r.file for r in rs))}\t{shape}\t"
                         f"{' '.join(sorted(r.ref() for r in rs))}\n")

    # the skeleton's numbers
    say("### The skeleton's numbers", "",
        "Each over the rules of the census: least / median / mean / 90th percentile / greatest.", "")
    rows = []
    for key, what in (("params", "parameters"), ("premises", "positive premises"),
                      ("negations", "`not` and `absent`"), ("guards", "`eq`/`neq` guards"),
                      ("conclusions", "conclusions"), ("vars", "variables that are no parameter"),
                      ("consts", "constants"), ("relations", "relation names"),
                      ("max_arity", "widest atom (arguments)"),
                      ("depth_match", "nesting depth of the `:match`"),
                      ("depth_assert", "nesting depth of the `:assert`")):
        vals = [r.skel[key] for r in rules]
        dv = {}
        for r in rules:
            dv[r.shapes["alphau"]] = r.skel[key]
        rows.append([what, quantiles(vals), quantiles(list(dv.values()))])
    say(md_table(["", "every rule", "distinct rules"], rows), "")
    feats = [("no premise (`:match ()`)", lambda r: r.match == []),
             ("a `:match` of one positive premise and nothing else",
              lambda r: r.skel["premises"] == 1 and r.skel["negations"] == 0 and r.skel["guards"] == 0
              and r.skel["macros"] == 0),
             ("an `absent`, a `not` or a macro call in the `:match`",
              lambda r: r.skel["negations"] > 0 or r.skel["macros"] > 0),
             ("an `or` in the `:match`", lambda r: r.skel["or"] > 0),
             ("a guard", lambda r: r.skel["guards"] > 0),
             ("concludes `(false)`", lambda r: r.skel["false"] > 0),
             ("concludes `(open …)`", lambda r: r.skel["open"] > 0),
             ("a nested fact (depth of 3 or more in either clause, the connective counted)",
              lambda r: r.skel["depth_match"] >= 3 or r.skel["depth_assert"] >= 3),
             ("a variable at the head of a premise or a conclusion",
              lambda r: any(not is_name(t[0]) for _, _, _, t in r.conn.prem)
              or any(not is_name(t[0]) for _, _, t in r.conn.concl)),
             ("no constant at all", lambda r: r.skel["consts"] == 0)]
    rows = []
    for what, pred in feats:
        hit = [r for r in rules if pred(r)]
        rows.append([what, len(hit), len(set(r.shapes["alphau"] for r in hit)),
                     sum(1 for r in hit if r.root == "stdlib")])
    say(md_table(["rules with", "rules", "distinct", "in `stdlib/`"], rows,
                 ["---", "---:", "---:", "---:"]), "")

    # families
    say("### Families", "",
        "A *family* is a set of two or more distinct rules (`alphau`) that one coarser level "
        "cannot tell apart.", "")
    fam_rows = []
    fam_dump = {}

    def families(level, finer="alphau", same_file=False):
        out = []
        cls = defaultdict(lambda: defaultdict(list))
        for r in rules:
            key = (r.shapes[level], r.file) if same_file else r.shapes[level]
            cls[key][r.shapes[finer]].append(r)
        for key, members in cls.items():
            if len(members) >= 2:
                out.append((key, members))
        out.sort(key=lambda km: (-len(km[1]), str(km[0])))
        return out

    for level, what in (("conren", "differ by constants alone (relations and structure equal)"),
                        ("relren", "differ by relation names alone (constants and structure equal)"),
                        ("ii", "differ by names alone: one shape at level (ii)"),
                        ("iii", "one shape at level (iii)"),
                        ("iiib", "one shape at level (iii′)")):
        fams = families(level)
        covered = sum(len(m) for _, m in fams)
        fam_rows.append([what, len(fams), covered, max([len(m) for _, m in fams] + [0])])
        fam_dump[level] = [{"shape": k, "size": len(m),
                            "members": sorted(sorted(set(r.ref() for r in v))[0] for v in m.values())}
                           for k, m in fams]
    say(md_table(["families whose members", "families", "distinct rules in one", "largest"],
                 fam_rows, ["---", "---:", "---:", "---:"]), "")

    # Families as written: the members of one ordered shape have their
    # constants (their relations) slot for slot, so the slots that vary can be
    # counted.
    def slot_families(level, seq):
        out = []
        for key, members in families(level, finer="alpha"):
            reps = sorted((v[0] for v in members.values()), key=lambda r: (r.file, r.line))
            vecs = [seq(r) for r in reps]
            width = min(len(v) for v in vecs)
            varying = [k for k in range(width) if len(set(v[k] for v in vecs)) > 1]
            out.append((len(varying), key, reps, varying, vecs))
        return out

    def slot_table(fams, what):
        by = defaultdict(list)
        for n, key, reps, varying, vecs in fams:
            by[min(n, 4)].append(reps)
        rows = []
        for n in sorted(by):
            label = {0: "none (the members differ in order alone)", 4: "4 or more"}.get(n, str(n))
            rows.append([label, len(by[n]), sum(len(x) for x in by[n]), max(len(x) for x in by[n])])
        say(md_table([f"{what} that vary", "families", "distinct rules in one", "largest"], rows,
                     ["---", "---:", "---:", "---:"]), "")

    def slot_examples(fams, n_slots, k=12):
        rows = []
        pick = sorted((f for f in fams if f[0] == n_slots), key=lambda f: (-len(f[2]), f[1]))
        for n, key, reps, varying, vecs in pick[:k]:
            vals = []
            for slot in varying:
                seen = []
                for v in vecs:
                    if v[slot] not in seen:
                        seen.append(v[slot])
                vals.append(", ".join(f"`{x}`" for x in seen[:9]) + (" …" if len(seen) > 9 else ""))
            files_ = sorted(set(r.file for r in reps))
            where = f"`{files_[0]}`" if len(files_) == 1 else f"{len(files_)} files"
            rows.append([len(reps), "; ".join(vals), where,
                         ", ".join(f"`{r.name}`" for r in reps[:4]) + (" …" if len(reps) > 4 else "")])
        say(md_table(["rules", "the values of the slot", "where", "members"], rows,
                     ["---:", "---", "---", "---"]), "")

    cfams = slot_families("coni", lambda r: r.const_seq)
    rfams = slot_families("reli", lambda r: r.rel_seq)
    say("**By what is written in a slot.** Two rules of one shape *as written* have their names "
        "slot for slot. Families whose members are one text but for constants (relations, "
        "variables' pattern and order equal), by the number of constant slots that vary:", "")
    slot_table(cfams, "constant slots")
    say("The largest that vary in one constant:", "")
    slot_examples(cfams, 1)
    say("The largest that vary in two:", "")
    slot_examples(cfams, 2, 6)
    say("Families whose members are one text but for relation names (constants equal), by the "
        "number of relation slots that vary; a family that varies in one slot is one rule with "
        "a relation for a parameter, written out:", "")
    slot_table(rfams, "relation slots")
    say("The largest that vary in one relation:", "")
    slot_examples(rfams, 1)
    say("The largest that vary in two:", "")
    slot_examples(rfams, 2, 8)
    fam_dump["one-constant"] = [{"shape": key, "size": len(reps), "members": [r.ref() for r in reps]}
                                for n, key, reps, varying, vecs in cfams if n == 1]
    fam_dump["one-relation"] = [{"shape": key, "size": len(reps), "members": [r.ref() for r in reps]}
                                for n, key, reps, varying, vecs in rfams if n == 1]

    # families inside one file at (iii′) and (i)
    rows = []
    infile = families("iiib", same_file=True)
    infile_i = families("i", same_file=True)
    infile_ii = families("ii", same_file=True)
    say(f"Inside one file: {len(infile_ii)} families at level (ii) "
        f"({sum(len(m) for _, m in infile_ii)} distinct rules), {len(infile)} at level (iii′) "
        f"({sum(len(m) for _, m in infile)} distinct rules). The largest at level (iii′), where a "
        "family's members may differ in which variable sits where:", "")
    for (shape, path), members in infile[:15]:
        reps = sorted((v[0] for v in members.values()), key=lambda r: r.line)
        rows.append([len(members), f"`{path}`", len(set(r.shapes["ii"] for r in reps)),
                     ", ".join(f"`{r.name}`" for r in reps[:6]) + (" …" if len(reps) > 6 else "")])
    say(md_table(["rules", "file", "shapes (ii) among them", "members"], rows,
                 ["---:", "---", "---:", "---"]), "")
    fam_dump["iiib-in-file"] = [{"file": k[1], "shape": k[0], "size": len(m),
                                 "members": sorted(v[0].name for v in m.values())} for k, m in infile]
    with open(os.path.join(args.out, "families.json"), "w", encoding="utf-8") as fh:
        json.dump(fam_dump, fh, indent=1, sort_keys=True, ensure_ascii=False)
        fh.write("\n")

    # ── ways to group shapes ──
    say("## Ways to group shapes", "",
        "A grouping key is a function of a rule that looks at no name's spelling, so it is a "
        "function of the level-(ii) shape and groups shapes. The catalogue:", "")
    key_names = [k for k, _ in KEY_DEFS]
    grp = {}
    rows = []
    for key, what in KEY_DEFS:
        g = defaultdict(list)
        for r in rules:
            g[r.keys[key]].append(r)
        grp[key] = g
        sizes = sorted((len(v) for v in g.values()), reverse=True)
        shapes_n = sorted((len(set(r.shapes["ii"] for r in v)) for v in g.values()), reverse=True)
        rows.append([f"**{key}**", what, len(g), sizes[0], shapes_n[0]])
    say(md_table(["key", "definition", "groups", "largest, in rules", "largest, in shapes (ii)"], rows,
                 ["---", "---", "---:", "---:", "---:"]), "")

    def natural(v):
        head = v.split()[0] if v else ""
        return (0, int(head), v) if head.isdigit() else (1, 0, v)

    def key_table(key, top, by_size=True):
        g = grp[key]
        items = sorted(g.items(), key=lambda kv: (-len(kv[1]), kv[0])) if by_size \
            else sorted(g.items(), key=lambda kv: natural(kv[0]))
        rows = []
        for val, rs in items[:top]:
            sh = Counter(r.shapes["ii"] for r in rs)
            best = sorted(sh.items(), key=lambda kv: (-kv[1], kv[0]))[0]
            ex = best[0] if len(best[0]) <= 110 else best[0][:107] + "…"
            rows.append([val, len(rs), len(set(r.shapes["alphau"] for r in rs)), len(sh),
                         sum(1 for r in rs if r.root == "stdlib"), code(ex), best[1]])
        more = f" The {len(items) - top} smaller groups are in `groups.tsv`." if len(items) > top else ""
        say(f"### By {key}", "",
            md_table([key, "rules", "distinct", "shapes (ii)", "in `stdlib/`",
                      "its most frequent shape", "rules of it"], rows,
                     ["---", "---:", "---:", "---:", "---:", "---", "---:"]), "")
        if more:
            say(more.strip(), "")

    ordered = {"relations", "arguments in all", "widest atom", "variables", "constants",
               "parameters", "join components", "variables a premise"}
    for key in key_names:
        key_table(key, 14, by_size=key not in ordered)

    def cross(ka, kb, cap_a, cap_b):
        ga = sorted(grp[ka], key=natural)[:cap_a]
        gb = sorted(grp[kb], key=natural)[:cap_b]
        cell = Counter((r.keys[ka], r.keys[kb]) for r in rules)
        rows = []
        for a in ga:
            rows.append([f"**{a}**"] + [cell.get((a, b), 0) or "·" for b in gb]
                        + [sum(cell.get((a, b), 0) for b in grp[kb])])
        say(f"**{ka} (rows) by {kb} (columns)**, in rules:", "",
            md_table([""] + gb + ["all"], rows, ["---"] + ["---:"] * (len(gb) + 1)), "")

    say("### Two keys at once", "")
    for r in rules:
        r.keys["premises"] = r.keys["premises × conclusions"].split(" × ")[0]
        r.keys["conclusions"] = r.keys["premises × conclusions"].split(" × ")[1]
    for extra in ("premises", "conclusions"):
        g = defaultdict(list)
        for r in rules:
            g[r.keys[extra]].append(r)
        grp[extra] = g
    cross("premises", "conclusions", 9, 9)
    cross("premises", "relations", 9, 9)
    cross("premises", "variables", 9, 11)
    cross("join structure", "feeds itself", 9, 4)
    cross("builds or takes apart", "join structure", 9, 9)
    cross("schema", "parameters", 30, 6)

    # which key refines which
    say("### Which grouping refines which", "",
        "Grouping A *refines* B when two rules A puts together B puts together too, which is "
        "checked here on the census's rules and is no proof. Every key is refined by level (ii) "
        "by construction. A `✓` says the row's level refines the column's key:", "")
    lv = [("ii", "(ii)"), ("iii", "(iii)"), ("iiib", "(iii′)"), ("iv", "(iv)"), ("ivb", "(iv′)")]

    def refines(fa, fb):
        seen = {}
        for r in rules:
            a, b = fa(r), fb(r)
            if seen.setdefault(a, b) != b:
                return False
        return True
    rows = []
    for level, label in lv:
        rows.append([label] + ["✓" if refines(lambda r: r.shapes[level], lambda r, k=k: r.keys[k])
                               else "·" for k in key_names])
    say(md_table(["level"] + key_names, rows, ["---"] + [":-:"] * len(key_names)), "")
    pairs = []
    for a in key_names:
        for b in key_names:
            if a != b and len(grp[a]) > len(grp[b]) > 1 and \
                    refines(lambda r: r.keys[a], lambda r: r.keys[b]):
                pairs.append(f"*{a}* refines *{b}*")
    chain = [("alpha", "the same text"), ("alphau", "the same rule"), ("relren", "relations renamed"),
             ("conren", "constants renamed"), ("i", "(i)"), ("ii", "(ii)"), ("iii", "(iii)"),
             ("iiib", "(iii′)"), ("iv", "(iv)"), ("ivb", "(iv′)")]
    lrows = []
    for la, na in chain:
        lrows.append([na, len(level_classes[la])] +
                     ["—" if la == lb else ("✓" if refines(lambda r: r.shapes[la], lambda r: r.shapes[lb])
                                            else "·") for lb, _ in chain])
    say("Between the keys: " + ("; ".join(pairs) if pairs else "none refines another") + ".", "",
        "Between the levels (the row refines the column):", "",
        md_table(["level", "classes"] + [n for _, n in chain], lrows,
                 ["---", "---:"] + [":-:"] * len(chain)), "")
    with open(os.path.join(args.out, "groups.tsv"), "w", encoding="utf-8") as fh:
        fh.write("key\tvalue\trules\tdistinct_rules\tshapes_ii\tstdlib\tmost_frequent_shape\n")
        for key in key_names:
            for val, rs in sorted(grp[key].items(), key=lambda kv: (-len(kv[1]), kv[0])):
                sh = Counter(r.shapes["ii"] for r in rs)
                best = sorted(sh.items(), key=lambda kv: (-kv[1], kv[0]))[0][0]
                fh.write(f"{key}\t{val}\t{len(rs)}\t{len(set(r.shapes['alphau'] for r in rs))}\t"
                         f"{len(sh)}\t{sum(1 for r in rs if r.root == 'stdlib')}\t{best}\n")
    with open(os.path.join(args.out, "rule_keys.tsv"), "w", encoding="utf-8") as fh:
        fh.write("file\tname\t" + "\t".join(key_names) + "\n")
        for r in rules:
            fh.write(f"{r.file}\t{r.name}\t" + "\t".join(r.keys[k] for k in key_names) + "\n")

    # ── B. connections ──
    say("## B. Connections", "")
    # scopes
    file_mask = {}
    for r in rules:
        file_mask[r.file] = file_mask.get(r.file, 0) | (1 << r.idx)
    rules_by_file = defaultdict(list)
    for r in rules:
        rules_by_file[r.file].append(r)

    def imported(path):
        """The rules a file's imports bring, as a mask; and the modules."""
        mask = 0
        mods = set()
        todo = [(path, imp, symbols, alias) for imp, symbols, alias in files[path].imports]
        done = set()
        qualified = 0
        while todo:
            src, imp, symbols, alias = todo.pop()
            mod = resolve(imp, src)
            if mod not in files or files[mod].error or files[mod].broken or mod == path:
                continue
            mods.add(mod)
            if alias is not None:
                qualified += 1
            f = files[mod]
            if symbols is None or alias is not None:
                keep = list(f.rules)
            else:
                selected = set(symbols)
                keep = []
                grew = True
                while grew:
                    grew = False
                    for r in f.rules:
                        if r.name in selected and r not in keep:
                            keep.append(r)
                            new = r.conn.names - selected
                            if new:
                                selected |= new
                            grew = True
                    for name, (params, body) in f.macros.items():
                        if name in selected:
                            new = set(x for x in flatten(body) if is_name(x)) - selected
                            if new:
                                selected |= new
                                grew = True
            key = (mod, tuple(sorted(r.name for r in keep)))
            if key in done:
                continue
            done.add(key)
            for r in keep:
                mask |= 1 << r.idx
            for imp2, symbols2, alias2 in f.imports:
                todo.append((mod, imp2, symbols2, alias2))
        return mask, mods, qualified

    programs = []   # (path, own mask, imported mask, modules)
    for f in census_files:
        imp, mods, q = imported(f.path)
        own = file_mask.get(f.path, 0)
        programs.append((f.path, own, imp & ~own, mods))
    P_file_sym = sum(len(v) * (len(v) - 1) // 2 for v in rules_by_file.values())
    P_file_dir = sum(len(v) * (len(v) - 1) for v in rules_by_file.values())
    P_glob_sym = N * (N - 1) // 2
    P_glob_dir = N * (N - 1)
    P_cross = sum(own.bit_count() * imp.bit_count() for _, own, imp, _ in programs)
    n_importing = sum(1 for _, own, imp, _ in programs if imp)
    say(f"Scopes. *Global*: any two of the {fmt(N)} rules, {fmt(P_glob_sym)} unordered pairs; a name "
        "is equal to a name by spelling alone, across programs that have nothing to do with each "
        f"other. *File*: two rules of one file, {fmt(P_file_sym)} unordered pairs over "
        f"{len(rules_by_file)} files. *Program*: a rule a file writes and a rule its imports bring, "
        f"{fmt(P_cross)} pairs over the {n_importing} files whose imports bring a rule. A pair of "
        "imported rules is counted once, in its module's file scope, and not again for every "
        "program that imports it.", "")

    conn_rows = []
    conn_examples = {}
    conn_degrees = {}

    def pick_examples(adj, directed, want=3):
        """Pairs of one file first, the stdlib's first, a file once."""
        out = []
        order = sorted(range(N), key=lambda i: (rules[i].root != "stdlib", rules[i].file, rules[i].line))
        used = set()
        for i in order:
            if rules[i].file in used:
                continue
            m = adj[i] & file_mask[rules[i].file] & ~(1 << i)
            for j in bits(m)[:1]:
                out.append((rules[i].ref(), rules[j].ref()))
                used.add(rules[i].file)
            if len(out) >= want:
                return out
        for i in order:
            if rules[i].file in used:
                continue
            for j in bits(adj[i] & ~(1 << i))[:1]:
                if rules[j].file in used:
                    continue
                out.append((rules[i].ref(), rules[j].ref()))
                used.add(rules[i].file)
                used.add(rules[j].file)
            if len(out) >= want:
                break
        return out

    def count(kind, name, adj, directed):
        """One row of the catalogue."""
        if directed:
            loops = sum(1 for i in range(N) if adj[i] >> i & 1)
            g = sum(a.bit_count() for a in adj) - loops
            fl = sum((adj[i] & file_mask[rules[i].file]).bit_count() for i in range(N)) - loops
            out_c = in_c = 0
            for path, own, imp, _ in programs:
                if not imp:
                    continue
                for i in bits(own):
                    out_c += (adj[i] & imp).bit_count()
                for j in bits(imp):
                    in_c += (adj[j] & own).bit_count()
            deg = [(a & ~(1 << i)).bit_count() for i, a in enumerate(adj)]
            row = [kind, name, "→", fmt(g), f"{100 * g / P_glob_dir:.2f} %", fmt(fl),
                   f"{100 * fl / P_file_dir:.2f} %", f"{fmt(out_c)} / {fmt(in_c)}",
                   loops, sum(1 for d in deg if d == 0), quantiles(deg)]
        else:
            g = sum(a.bit_count() for a in adj) // 2
            fl = sum((adj[i] & file_mask[rules[i].file]).bit_count() for i in range(N)) // 2
            cross = 0
            for path, own, imp, _ in programs:
                if not imp:
                    continue
                for i in bits(own):
                    cross += (adj[i] & imp).bit_count()
            deg = [a.bit_count() for a in adj]
            row = [kind, name, "—", fmt(g), f"{100 * g / P_glob_sym:.2f} %", fmt(fl),
                   f"{100 * fl / P_file_sym:.2f} %", fmt(cross), "", sum(1 for d in deg if d == 0),
                   quantiles(deg)]
        conn_rows.append(row)
        conn_examples[kind] = pick_examples(adj, directed)
        conn_degrees[kind] = deg
        return adj

    C = [r.conn for r in rules]
    U = Unifier()

    def atoms(sel):
        return [sel(c) for c in C]

    prem_pos = atoms(lambda c: [(s, t) for pol, d, s, t in c.prem if pol == "+" and d % 2 == 0])
    prem_pos0 = atoms(lambda c: [(s, t) for pol, d, s, t in c.prem if pol == "+" and d == 0])
    prem_abs = atoms(lambda c: [(s, t) for pol, d, s, t in c.prem if pol == "+" and d % 2 == 1])
    prem_not = atoms(lambda c: [(s, t) for pol, d, s, t in c.prem if pol == "-" and d % 2 == 0])
    prem_notabs = atoms(lambda c: [(s, t) for pol, d, s, t in c.prem if pol == "-" and d % 2 == 1])
    prem_any = atoms(lambda c: [(s, t) for pol, d, s, t in c.prem])
    concl_pos = atoms(lambda c: [(s, t) for pol, s, t in c.concl if pol == "+"])
    concl_neg = atoms(lambda c: [(s, t) for pol, s, t in c.concl if pol == "-"])

    def heads(lists, arity=False):
        out = []
        for atoms_ in lists:
            ks = set()
            for s, t in atoms_:
                if is_name(t[0]):
                    ks.add((t[0], len(t) - 1) if arity else t[0])
            out.append(ks)
        return out

    rule_names = defaultdict(int)
    for r in rules:
        rule_names[r.name] |= 1 << r.idx

    # 1 — the same name
    count("1a", "share a relation name (a name either writes at the head of a form, at any depth)",
          sym_from_keys([c.rels for c in C]), False)
    count("1b", "share a constant (a name either writes as an argument)",
          sym_from_keys([c.consts for c in C]), False)
    cross_kind = []
    rel_index, const_index = defaultdict(int), defaultdict(int)
    for i, c in enumerate(C):
        for k in c.rels:
            rel_index[k] |= 1 << i
        for k in c.consts:
            const_index[k] |= 1 << i
    for i, c in enumerate(C):
        m = 0
        for k in c.rels:
            m |= const_index.get(k, 0)
        for k in c.consts:
            m |= rel_index.get(k, 0)
        cross_kind.append(m & ~(1 << i))
    count("1c", "one's relation is the other's constant (a relation named as a value)", cross_kind, False)
    count("1d", "share any name", sym_from_keys([c.names for c in C]), False)
    names_rule = []
    for i, c in enumerate(C):
        m = 0
        for k in c.names:
            m |= rule_names.get(k, 0)
        names_rule.append(m)
    count("1e", "A writes B's rule name, as a relation or as a constant", names_rule, True)
    names_rule_c = []
    for i, c in enumerate(C):
        m = 0
        for k in c.consts:
            m |= rule_names.get(k, 0)
        names_rule_c.append(m)
    count("1f", "A writes B's rule name as a constant (names the rule as a value)", names_rule_c, True)

    # 2 — the same name in the same place
    roles = {"match+": heads(prem_pos0), "match-absent": heads(prem_abs),
             "match-not": heads(prem_not), "assert+": heads(concl_pos), "assert-not": heads(concl_neg)}
    union = [set() for _ in range(N)]
    for role, ks in roles.items():
        for i, k in enumerate(ks):
            union[i] |= {(role, x) for x in k}
    count("2a", "the same relation in the same clause role (any of the five below)",
          sym_from_keys(union), False)
    count("2b", "… both read it as a positive premise (= 7a)", sym_from_keys(roles["match+"]), False)
    count("2c", "… both read it under `absent`", sym_from_keys(roles["match-absent"]), False)
    count("2d", "… both read its stored negation, `(not (R …))`", sym_from_keys(roles["match-not"]), False)
    count("2e", "… both conclude it (= 6a)", sym_from_keys(roles["assert+"]), False)
    count("2f", "… both conclude its negation", sym_from_keys(roles["assert-not"]), False)
    count("2g", "the same constant at the same argument place of the same relation, either clause",
          sym_from_keys([{(p, k) for role, p, k in c.positions} for c in C]), False)
    count("2h", "… both in the `:match`",
          sym_from_keys([{(p, k) for role, p, k in c.positions if role == "match"} for c in C]), False)
    count("2i", "… both in the `:assert`",
          sym_from_keys([{(p, k) for role, p, k in c.positions if role == "assert"} for c in C]), False)
    count("2j", "… A's `:assert`, B's `:match`",
          dir_from_keys([{(p, k) for role, p, k in c.positions if role == "assert"} for c in C],
                        [{(p, k) for role, p, k in c.positions if role == "match"} for c in C]), True)

    # 3 — assert → match
    a3a = count("3a", "A concludes a relation B reads as a positive premise, by name",
                dir_from_keys(heads(concl_pos), heads(prem_pos)), True)
    count("3b", "… by name and number of arguments",
          dir_from_keys(heads(concl_pos, True), heads(prem_pos, True)), True)
    a3c = count("3c", "… a conclusion of A unifies with a premise of B (both headed by a name)",
                U.link(concl_pos, prem_pos, False), True)
    a3d = count("3d", "… unifies, one of the two headed by a variable (a generic rule)",
                U.link(concl_pos, prem_pos, True), True)

    # 4 — negative and stored-negation dependencies
    count("4a", "A concludes a relation B tests with `absent`, by name",
          dir_from_keys(heads(concl_pos), heads(prem_abs)), True)
    a4b = count("4b", "… a conclusion of A unifies with an atom B tests with `absent`",
                U.link(concl_pos, prem_abs, False), True)
    count("4c", "A concludes `(not (R …))` and B reads `(not (R …))`, by name",
          dir_from_keys(heads(concl_neg), heads(prem_not)), True)
    a4d = count("4d", "… the two unify", U.link(concl_neg, prem_not, False), True)
    count("4e", "A concludes `(not (R …))` and B tests `(absent (not (R …)))`, unifying",
          U.link(concl_neg, prem_notabs, False), True)
    count("4f", "A concludes `(R …)` and B reads `(not (R …))`, unifying: opposite signs",
          U.link(concl_pos, prem_not, False), True)
    count("4g", "A concludes `(not (R …))` and B reads `(R …)`, unifying: opposite signs",
          U.link(concl_neg, prem_pos, False), True)

    # 5 — activators
    act_index = defaultdict(int)
    for r in rules:
        if r.params:
            act_index[(r.name, len(r.params))] |= 1 << r.idx
    a5 = []
    for i, c in enumerate(C):
        m = 0
        for s, t in concl_pos[i]:
            if is_name(t[0]):
                m |= act_index.get((t[0], len(t) - 1), 0)
        a5.append(m)
    count("5a", "A concludes a fact that activates B: headed by B's name, as many arguments as B has parameters",
          a5, True)
    a5b = []
    for i, c in enumerate(C):
        m = 0
        for s, t in prem_any[i]:
            if is_name(t[0]):
                m |= act_index.get((t[0], len(t) - 1), 0)
        a5b.append(m)
    count("5b", "A reads B's activator as a premise (a rule about a rule's being switched on)", a5b, True)

    # 6 — conclusion against conclusion
    count("6a", "both conclude the same relation, by name (= 2e)", sym_from_keys(roles["assert+"]), False)
    a6b = U.link(concl_pos, concl_pos, False)
    a6b = [a6b[i] & ~(1 << i) for i in range(N)]
    count("6b", "… two of their conclusions unify (two ways to derive one fact)", a6b, False)
    a6c = U.link(concl_pos, concl_neg, False)
    back = [0] * N
    for i in range(N):
        for j in bits(a6c[i]):
            back[j] |= 1 << i
    count("6c", "one concludes `(R …)`, the other `(not (R …))`, unifying (a possible conflict)",
          [(a6c[i] | back[i]) & ~(1 << i) for i in range(N)], False)
    false_mask = 0
    for i, c in enumerate(C):
        if c.false:
            false_mask |= 1 << i
    count("6d", "A's conclusion unifies with a premise of B, and B concludes `(false)`",
          [a3c[i] & false_mask for i in range(N)], True)
    count("6e", "both conclude `(false)`",
          [(false_mask & ~(1 << i)) if c.false else 0 for i, c in enumerate(C)], False)
    open_mask = 0
    for i, c in enumerate(C):
        if c.open:
            open_mask |= 1 << i
    count("6f", "both conclude `(open …)`",
          [(open_mask & ~(1 << i)) if c.open else 0 for i, c in enumerate(C)], False)

    # 7 — premise against premise
    count("7a", "both read the same relation as a positive premise, by name (= 2b)",
          sym_from_keys(roles["match+"]), False)
    a7b = U.link(prem_pos0, prem_pos0, False)
    count("7b", "… two of their premises unify (one fact can feed both)",
          [a7b[i] & ~(1 << i) for i in range(N)], False)
    count("7c", "… they have a premise in common, the same up to a renaming of variables",
          sym_from_keys([{s for s, t in atoms_} for atoms_ in prem_pos0]), False)
    joins = [joins_of(r, r.macros) for r in rules]
    count("7d", "the same join: one variable at argument i of R and argument j of S, in both",
          sym_from_keys(joins), False)

    # 8 — shape
    for kind, level, what in (("8a", "alphau", "the same rule (variables renamed, any order)"),
                              ("8b", "relren", "one rule under a renaming of relations"),
                              ("8c", "conren", "one rule under a renaming of constants"),
                              ("8d", "ii", "the same shape at level (ii)"),
                              ("8e", "iii", "the same shape at level (iii)"),
                              ("8f", "iiib", "the same shape at level (iii′)"),
                              ("8g", "iv", "the same skeleton, level (iv)"),
                              ("8h", "match-ii", "the same `:match` shape"),
                              ("8i", "assert-ii", "the same `:assert` shape")):
        count(kind, what, sym_from_keys([{r.shapes[level]} for r in rules]), False)
    # proper renamings: relren-equal and not the same rule
    same = sym_from_keys([{r.shapes["alphau"]} for r in rules])
    ren = sym_from_keys([{r.shapes["relren"]} for r in rules])
    count("8j", "one rule under a renaming of relations, and not the same rule (8b less 8a)",
          [ren[i] & ~same[i] for i in range(N)], False)

    # 8k — subsumption of matches
    flat = [flat_literals(r, r.macros) for r in rules]
    forms = {}
    for i, lits in enumerate(flat):
        if lits:
            key = r_key = rules[i].shapes["alphau"].split(" ⇒ ")[0]
            key = show([[s, x] for s, x in lits])
            key = atom_key([[s, x] for s, x in lits])[0]
            forms.setdefault(key, [lits, 0])
            forms[key][1] |= 1 << i
    fkeys = sorted(forms)
    sigs = {}
    for k in fkeys:
        lits = forms[k][0]
        sigs[k] = (frozenset((s, x[0], len(x)) for s, x in lits if is_name(x[0])),
                   Counter(len(x) for s, x in lits), len(lits),
                   all(is_name(x[0]) for s, x in lits))
    frozen = {k: [(s, freeze(x)) for s, x in forms[k][0]] for k in fkeys}
    sub_adj = [0] * N
    sub_adj_gen = [0] * N
    sub_pairs = sub_timeout = 0
    for ka in fkeys:
        siga, ara, na, conc = sigs[ka]
        for kb in fkeys:
            if ka == kb:
                continue
            sigb, arb, nb_, _ = sigs[kb]
            if not siga <= sigb:
                continue
            if any(arb[a] == 0 for a in ara):
                continue
            try:
                ok = subsumes(forms[ka][0], frozen[kb], 20000)
            except TimeoutError:
                sub_timeout += 1
                ok = False
            if ok:
                sub_pairs += 1
                for i in bits(forms[ka][1]):
                    if conc:
                        sub_adj[i] |= forms[kb][1]
                    else:
                        sub_adj_gen[i] |= forms[kb][1]
    count("8k", "A's `:match` is a sub-pattern of B's: a substitution of A's variables puts every literal "
          "of A among B's (A's heads all names)", sub_adj, True)
    count("8l", "… where A has a variable at a head (a generic match)", sub_adj_gen, True)
    both = [0] * N
    for i in range(N):
        for j in bits(sub_adj[i]):
            if sub_adj[j] >> i & 1:
                both[i] |= 1 << j
    count("8m", "each `:match` a sub-pattern of the other, the two not the same up to renaming",
          both, False)

    # 9 — structure
    count("9a", "in the same file", [file_mask[r.file] & ~(1 << r.idx) for r in rules], False)
    dir_mask = defaultdict(int)
    for r in rules:
        dir_mask[os.path.dirname(r.file)] |= 1 << r.idx
    count("9b", "in the same directory",
          [dir_mask[os.path.dirname(r.file)] & ~(1 << r.idx) for r in rules], False)
    co = [0] * N
    for path, own, imp, _ in programs:
        scope = own | imp
        if imp:
            for i in bits(scope):
                co[i] |= scope
    count("9c", "in the scope of one program that imports (a file's rules and what its imports bring)",
          [co[i] & ~(1 << i) for i in range(N)], False)
    # named together: a rule C in whose text both names occur, C in a file that has both in scope
    scope_of = {}
    for path, own, imp, _ in programs:
        scope_of[path] = own | imp
    named = [0] * N
    for r in rules:
        sc = scope_of[r.file]
        m = 0
        for k in r.conn.names:
            m |= rule_names.get(k, 0) & sc
        if m.bit_count() >= 2:
            for i in bits(m):
                named[i] |= m
    count("9d", "named together by one rule (a closing rule's list): both names in one rule's text, "
          "all three in one program's scope", [named[i] & ~(1 << i) for i in range(N)], False)
    solved = [0] * N
    by_solve = set()
    for f in census_files:
        sc = scope_of[f.path]
        for names in f.solve_names:
            m = 0
            for k in names:
                m |= rule_names.get(k, 0) & sc
            for i in bits(m):
                solved[i] |= m
                by_solve.add(i)
    count("9e", "named together in one `(solve :hrules …)` list",
          [solved[i] & ~(1 << i) for i in range(N)], False)
    activated = [0] * N
    written_act = Counter()
    for f in census_files:
        sc = scope_of[f.path]
        m = 0
        for fact in f.facts:
            if isinstance(fact, list) and fact and is_name(fact[0]):
                hit = act_index.get((fact[0], len([x for x in fact[1:]
                                                   if not (isinstance(x, str) and x.startswith(':'))])), 0) & sc
                # A fact's :source/:rule/:using clauses are not arguments.
                args_n = 0
                j = 1
                while j < len(fact):
                    x = fact[j]
                    if isinstance(x, str) and not isinstance(x, Str) and x.startswith(":"):
                        j += 2
                        continue
                    args_n += 1
                    j += 1
                hit = act_index.get((fact[0], args_n), 0) & sc
                m |= hit
        for i in bits(m):
            activated[i] |= m
            written_act[i] += 1
    count("9f", "activated by written facts of one file (both rules' activators written in it)",
          [activated[i] & ~(1 << i) for i in range(N)], False)

    # 11 — the same rule name
    count("11a", "the same rule name, in two files",
          [rule_names[r.name] & ~file_mask[r.file] for r in rules], False)
    count("11b", "… and not the same rule (the name reused for another text)",
          [rule_names[r.name] & ~file_mask[r.file] & ~same[r.idx] for r in rules], False)
    count("11c", "the same rule name twice in one file",
          [rule_names[r.name] & file_mask[r.file] & ~(1 << r.idx) for r in rules], False)

    say("### The catalogue", "",
        "Every kind is a relation on rules. `—` is symmetric (unordered pairs), `→` directed (ordered "
        "pairs, A to B; a rule connected to itself is counted in *self* and in no other column). "
        "*Density* is the pairs over the pairs the scope has. *Program* is pairs of a file's rule and "
        "an imported one; for a directed kind, *own → imported / imported → own*. *None* is the "
        "number of rules the kind connects to no other rule, globally; *degree* is a rule's number "
        "of partners globally (out-degree for `→`): least / median / mean / 90th percentile / greatest.", "",
        md_table(["kind", "definition", "", "global pairs", "density", "file pairs", "density",
                  "program pairs", "self", "none", "degree"], conn_rows,
                 ["---", "---", ":-:", "---:", "---:", "---:", "---:", "---:", "---:", "---:", "---"]), "")
    say(f"8k to 8m compare the {len(fkeys)} distinct `:match`es that are a flat conjunction of "
        f"literals ({sum(1 for x in flat if x is None)} rules have an `or`, a nested connective or a "
        f"macro call that is left out, {sum(1 for x in flat if x == [])} an empty match, which is a "
        f"sub-pattern of everything and is left out); {sub_pairs} ordered pairs of them subsume, and "
        f"{sub_timeout} comparisons ran out of their budget and are counted as no.", "")

    say("### Examples", "", "Three pairs a kind, a pair inside one file where there is one.", "")
    rows = []
    for row in conn_rows:
        ex = conn_examples[row[0]]
        sep = " → " if row[2] == "→" else " — "
        rows.append([row[0], "; ".join(f"`{a}`{sep}`{b}`" for a, b in ex) or "none"])
    say(md_table(["kind", "pairs"], rows), "")

    with open(os.path.join(args.out, "connections.tsv"), "w", encoding="utf-8") as fh:
        fh.write("kind\tdefinition\tdirected\tglobal_pairs\tglobal_density\tfile_pairs\tfile_density\t"
                 "program_pairs\tself\tnone\tdegree_min_median_mean_p90_max\n")
        for row in conn_rows:
            fh.write("\t".join(str(x) for x in row) + "\n")
    with open(os.path.join(args.out, "degrees.tsv"), "w", encoding="utf-8") as fh:
        kinds = [row[0] for row in conn_rows]
        fh.write("file\trule\t" + "\t".join(kinds) + "\n")
        for r in rules:
            fh.write(f"{r.file}\t{r.name}\t" + "\t".join(str(conn_degrees[k][r.idx]) for k in kinds) + "\n")

    # files alike
    def alike(adj, title, k):
        pairs = Counter()
        for i in range(N):
            for fb in set(rules[j].file for j in bits(adj[i])):
                if fb != rules[i].file:
                    pairs[(rules[i].file, fb)] += 1
        rows = []
        seen = set()
        for (a, b), n in sorted(pairs.items(), key=lambda kv: (-min(kv[1], pairs[(kv[0][1], kv[0][0])]), kv[0])):
            if (b, a) in seen:
                continue
            seen.add((a, b))
            rows.append([f"`{a}`", len(rules_by_file[a]), n, f"`{b}`", len(rules_by_file[b]), pairs[(b, a)]])
            if len(rows) == k:
                break
        say(title, "",
            md_table(["file A", "rules", "of them with a partner in B", "file B", "rules",
                      "of them with a partner in A"], rows, ["---", "---:", "---:", "---", "---:", "---:"]), "")
        return pairs

    say("### Files alike", "")
    pr = alike([ren[i] & ~same[i] for i in range(N)],
               "Pairs of files by 8j: rules of one that are a rule of the other under a renaming of "
               "relations, and not the same rule. The fifteen closest:", 15)
    pc = alike(same, "Pairs of files by 8a, the same rule in both. The ten closest:", 10)
    copies = sorted(set(tuple(sorted(k)) for k in pc))
    say(f"{len(copies)} pairs of files share a rule (8a); {len(set(tuple(sorted(k)) for k in pr))} "
        "pairs share a rule under a renaming of relations (8j).", "")

    # activation
    say("### How a rule with parameters is switched on", "")
    with_params = [r for r in rules if r.params]
    wp = set(r.idx for r in with_params)
    by_fact = set(written_act) & wp
    by_rule = set()
    for i in range(N):
        for j in bits(a5[i] & scope_of[rules[i].file]):
            by_rule.add(j)
    for path, own, imp, _ in programs:
        # A library rule is switched on from a program that imports it.
        for i in bits(own):
            for j in bits(a5[i] & imp):
                by_rule.add(j)
    by_list = by_solve & wp
    some = by_fact | by_rule | by_list
    rows = [["rules with parameters", len(with_params)],
            ["… whose activator a file that has the rule in scope writes as a fact", len(by_fact)],
            ["… whose activator a rule in such a scope concludes (5a)", len(by_rule)],
            ["… named in a `(solve :hrules …)` list of such a file", len(by_list)],
            ["… a written fact and a rule's conclusion both", len(by_fact & by_rule)],
            ["… none of the three", len(wp - some)]]
    say(md_table(["", "rules"], rows, ["---", "---:"]), "")
    never = sorted(r.ref() for r in with_params if r.idx not in some)
    with open(os.path.join(args.out, "never_activated.txt"), "w", encoding="utf-8") as fh:
        fh.write("\n".join(never) + "\n")

    # 10 — the digraph
    say("### 10. The assert → match digraph", "",
        "Nodes are rules; an edge A → B where a conclusion of A unifies with a positive premise of B "
        "(3c, both headed by a name), with 3d's edges through a generic rule added in the second "
        "column, and by relation name alone (3a) in the third.", "")
    a3cd = [a3c[i] | a3d[i] for i in range(N)]
    allnodes = list(range(N))
    cols = [("3c", a3c), ("3c + 3d", a3cd), ("3a", a3a)]
    stats = [graph_stats(allnodes, adj) for _, adj in cols]
    keys = [("edges", "edges"), ("self_loops", "rules that feed themselves"),
            ("sources", "sources (feed a rule, fed by none)"),
            ("sinks", "sinks (fed, feed none)"), ("isolated", "isolated"),
            ("sccs_nontrivial", "strongly connected components of two rules or more"),
            ("scc_largest", "the largest of them"),
            ("rules_in_cycles", "rules on a cycle (a self-loop counted)"),
            ("longest_path", "longest path, in components"),
            ("weak_components", "connected components"),
            ("weak_largest", "the largest of them")]
    rows = [[what] + [fmt(s[k]) for s in stats] for k, what in keys]
    rows.append(["out-degree: least / median / mean / 90th / greatest"] + [quantiles(s["outdeg"]) for s in stats])
    rows.append(["in-degree"] + [quantiles(s["indeg"]) for s in stats])
    say("**Global** (every rule of the census a node; a name equal by spelling):", "",
        md_table(["", "3c", "3c + 3d", "3a"], rows, ["---", "---:", "---:", "---:"]), "")

    # per file
    per_file = []
    for path in sorted(rules_by_file):
        nodes = [r.idx for r in rules_by_file[path]]
        st = graph_stats(nodes, a3c)
        st["file"] = path
        per_file.append(st)
    cyc = [s for s in per_file if s["rules_in_cycles"] > 0]
    say("**Per file** (a file's own rules, edges 3c):", "",
        f"- files with a rule: {len(per_file)}; with an edge: {sum(1 for s in per_file if s['edges'] or s['self_loops'])}; "
        f"with a rule on a cycle: {len(cyc)}; with a component of two rules or more: "
        f"{sum(1 for s in per_file if s['sccs_nontrivial'])}.",
        f"- longest path over the files: least / median / mean / 90th / greatest "
        f"{quantiles([s['longest_path'] for s in per_file])}.",
        f"- largest component over the files: {quantiles([s['scc_largest'] for s in per_file])}.", "")
    rows = []
    for s in sorted(per_file, key=lambda s: (-s["longest_path"], s["file"]))[:12]:
        rows.append([f"`{s['file']}`", s["nodes"], s["edges"], s["self_loops"], s["sccs_nontrivial"],
                     s["scc_largest"], s["longest_path"], s["sources"], s["sinks"], s["isolated"]])
    say("The twelve files with the longest path:", "",
        md_table(["file", "rules", "edges", "self", "components ≥ 2", "largest", "longest path",
                  "sources", "sinks", "isolated"], rows, ["---"] + ["---:"] * 9), "")
    rows = []
    for s in per_file:
        if s["file"].startswith("stdlib/"):
            rows.append([f"`{s['file']}`", s["nodes"], s["edges"], s["self_loops"], s["sccs_nontrivial"],
                         s["scc_largest"], s["rules_in_cycles"], s["longest_path"], s["sources"],
                         s["sinks"], s["isolated"], s["weak_components"]])
    say("The stdlib's modules, each alone:", "",
        md_table(["module", "rules", "edges", "self", "components ≥ 2", "largest", "rules on a cycle",
                  "longest path", "sources", "sinks", "isolated", "connected components"], rows,
                 ["---"] + ["---:"] * 11), "")
    with open(os.path.join(args.out, "graph_per_file.tsv"), "w", encoding="utf-8") as fh:
        cols_ = ["nodes", "edges", "self_loops", "sources", "sinks", "isolated", "sccs_nontrivial",
                 "scc_largest", "rules_in_cycles", "longest_path", "weak_components", "weak_largest"]
        fh.write("file\t" + "\t".join(cols_) + "\n")
        for s in per_file:
            fh.write(s["file"] + "\t" + "\t".join(str(s[c]) for c in cols_) + "\n")

    # per program: own + imported
    prog_stats = []
    for path, own, imp, mods in programs:
        if not imp or not own:
            continue
        nodes = bits(own | imp)
        st = graph_stats(nodes, a3cd)
        st["file"] = path
        st["own"] = own.bit_count()
        st["imported"] = imp.bit_count()
        prog_stats.append(st)
    if prog_stats:
        say(f"**Per program** (a file's rules and the rules its imports bring, edges 3c + 3d; "
            f"{len(prog_stats)} files write a rule and import one):", "",
            f"- rules in scope: {quantiles([s['nodes'] for s in prog_stats])}.",
            f"- longest path: {quantiles([s['longest_path'] for s in prog_stats])}.",
            f"- largest component: {quantiles([s['scc_largest'] for s in prog_stats])}.",
            f"- programs with a rule on a cycle: {sum(1 for s in prog_stats if s['rules_in_cycles'])}.", "")

    # module-level graph
    mod_edges = Counter()
    for i in range(N):
        if rules[i].root != "stdlib":
            continue
        for j in bits(a3c[i]):
            if rules[j].root == "stdlib" and rules[j].file != rules[i].file:
                mod_edges[(rules[i].file, rules[j].file)] += 1
    mod_gen = Counter()
    for i in range(N):
        if rules[i].root != "stdlib":
            continue
        for j in bits(a3d[i]):
            if rules[j].root == "stdlib" and rules[j].file != rules[i].file:
                mod_gen[(rules[i].file, rules[j].file)] += 1
    mods = sorted(p for p in rules_by_file if p.startswith("stdlib/"))
    rows = []
    for a in mods:
        row = [f"`{os.path.basename(a)[:-4]}` →"]
        for b in mods:
            if a == b:
                row.append(f"({sum((a3c[r.idx] & file_mask[a]).bit_count() for r in rules_by_file[a])})")
            else:
                row.append(mod_edges.get((a, b), 0) or "·")
        rows.append(row)
    say("**Between the stdlib's modules** (3c: a conclusion of a rule of the row's module unifies "
        "with a premise of a rule of the column's; the diagonal, in brackets, is the module's own "
        "edges with self-loops):", "",
        md_table([""] + [f"`{os.path.basename(b)[:-4]}`" for b in mods], rows,
                 ["---"] + ["---:"] * len(mods)), "")
    dot = ["digraph modules {", "  rankdir=LR;", '  node [shape=box, fontname="Helvetica"];',
           '  edge [fontname="Helvetica", fontsize=10];']
    for a in mods:
        dot.append(f'  "{os.path.basename(a)[:-4]}" [label="{os.path.basename(a)[:-4]}\\n'
                   f'{len(rules_by_file[a])} rules"];')
    for (a, b), n in sorted(mod_edges.items()):
        dot.append(f'  "{os.path.basename(a)[:-4]}" -> "{os.path.basename(b)[:-4]}" [label="{n}"];')
    for (a, b), n in sorted(mod_gen.items()):
        dot.append(f'  "{os.path.basename(a)[:-4]}" -> "{os.path.basename(b)[:-4]}" '
                   f'[label="{n}", style=dashed, color=gray50, fontcolor=gray50];')
    dot.append("}")
    with open(os.path.join(args.out, "modules.dot"), "w", encoding="utf-8") as fh:
        fh.write("\n".join(dot) + "\n")
    # co-import
    co_import = Counter()
    imported_by = Counter()
    for path, own, imp, ms in programs:
        std = sorted(m for m in ms if m.startswith("stdlib/"))
        for m in std:
            imported_by[m] += 1
        for a, b in itertools.combinations(std, 2):
            co_import[(a, b)] += 1
    allmods = sorted(p for p in files if p.startswith("stdlib/") and not files[p].error)
    rows = []
    for a in allmods:
        row = [f"`{os.path.basename(a)[:-4]}`", imported_by[a]]
        for b in allmods:
            if a == b:
                row.append("—")
            else:
                row.append(co_import.get((min(a, b), max(a, b)), 0) or "·")
        rows.append(row)
    say("**Imported together** (the number of files whose imports reach both modules, a module's own "
        "imports followed; the first column is the files that reach the module at all):", "",
        md_table(["", "files"] + [f"`{os.path.basename(b)[:-4]}`" for b in allmods], rows,
                 ["---"] + ["---:"] * (len(allmods) + 1)), "")

    # the rules of one stdlib module as a picture: std.arith's 3c graph
    for mod in ("stdlib/arith.ein", "stdlib/algebra.ein"):
        if mod not in rules_by_file:
            continue
        nodes = rules_by_file[mod]
        d = ["digraph rules {", "  rankdir=LR;", '  node [shape=box, fontsize=9, fontname="Helvetica"];']
        for r in nodes:
            d.append(f'  "{r.name}";')
        for r in nodes:
            for j in bits(a3c[r.idx] & file_mask[mod]):
                d.append(f'  "{r.name}" -> "{rules[j].name}";')
        d.append("}")
        with open(os.path.join(args.out, f"rules_{os.path.basename(mod)[:-4]}.dot"), "w",
                  encoding="utf-8") as fh:
            fh.write("\n".join(d) + "\n")

    # ── rules.tsv ──
    with open(os.path.join(args.out, "rules.tsv"), "w", encoding="utf-8") as fh:
        levels = [l for l, _ in LEVELS]
        fh.write("file\tline\tkind\tname\tparams\tgroup\tnotes\t" + "\t".join(levels) + "\n")
        for r in rules:
            fh.write(f"{r.file}\t{r.line}\t{r.kind}\t{r.name}\t{len(r.params)}\t{r.group}\t"
                     f"{','.join(r.notes)}\t" + "\t".join(r.shapes[l] for l in levels) + "\n")
    with open(os.path.join(args.out, "broken.tsv"), "w", encoding="utf-8") as fh:
        fh.write("file\trules\terror\n")
        for p in paths:
            f = files[p]
            if f.broken or f.error:
                fh.write(f"{p}\t{len(f.rules)}\t{f.error or ''}\n")

    # ── the engine's strata ──
    if args.strata:
        strata_section(args, say, census_files, files, rules, rules_by_file, macros_of, imported)

    if args.svg:
        for name in sorted(os.listdir(args.out)):
            if name.endswith(".dot"):
                src = os.path.join(args.out, name)
                try:
                    subprocess.run(["dot", "-Tsvg", src, "-o", src[:-4] + ".svg"], check=True,
                                   timeout=300)
                except (OSError, subprocess.SubprocessError) as e:
                    print(f"rule_census: dot did not draw {name}: {e}", file=sys.stderr)

    with open(os.path.join(args.out, "summary.md"), "w", encoding="utf-8") as fh:
        fh.write("\n".join(S) + "\n")
    if args.page:
        with open(args.page, encoding="utf-8") as fh:
            page = fh.read()
        a, b = page.index(PAGE_OPEN), page.index(PAGE_CLOSE)
        page = page[:a + len(PAGE_OPEN)] + "\n\n" + "\n".join(S) + "\n\n" + page[b:]
        with open(args.page, "w", encoding="utf-8") as fh:
            fh.write(page)
    print(f"rule_census: {len(census_files)} files, {N} rules "
          f"({sum(1 for r in rules if r.kind == 'hrule')} hrules), "
          f"{len(level_classes['i'])} shapes at level (i), {len(level_classes['ii'])} at (ii), "
          f"{len(level_classes['iii'])} at (iii), {len(level_classes['iiib'])} at (iii′), "
          f"{len(level_classes['iv'])} at (iv); {len(conn_rows)} kinds of connection; "
          f"written to {os.path.relpath(args.out, ROOT)}/")


# ───────────────────────────── the engine's strata ─────────────────────────────

def static_strata(path, files, rules_by_file, macros_of, imported, rules):
    """Strata of a program from its text: rules instantiated by the activator
    facts the file writes and by activators a rule concludes with every
    argument a name; an edge from each relation read to each written, negative
    under an odd number of `absent`s, and from a rule's name to what it writes.
    Returns (stratified?, strata, {rule name: stratum}) or None."""
    f = files[path]
    imp_mask, _, _ = imported(path)
    scope = list(rules_by_file.get(path, [])) + [rules[i] for i in bits(imp_mask)]
    by_name = defaultdict(list)
    for r in scope:
        by_name[r.name].append(r)
    macros = macros_of[path]
    todo = []
    seen = set()

    def fact_args(fact):
        out, j = [], 1
        while j < len(fact):
            x = fact[j]
            if isinstance(x, str) and not isinstance(x, Str) and x.startswith(":"):
                j += 2
                continue
            out.append(x)
            j += 1
        return out
    for r in scope:
        if not r.params:
            todo.append((r, {}))
    # facts of the file and of the modules it imports
    fact_files = [f]
    _, mods, _ = imported(path)
    fact_files += [files[m] for m in sorted(mods)]
    for ff in fact_files:
        for fact in ff.facts:
            if isinstance(fact, list) and fact and is_name(fact[0]) and fact[0] in by_name:
                a = fact_args(fact)
                for r in by_name[fact[0]]:
                    if len(r.params) == len(a) and r.params:
                        todo.append((r, dict(zip(r.params, a))))
    edges = set()     # (from relation, to relation, negative?)
    rule_writes = defaultdict(set)
    while todo:
        r, env = todo.pop()
        key = (r.idx, show([[k, env[k]] for k in sorted(env)]))
        if key in seen:
            continue
        seen.add(key)
        match = expand(subst_sexpr(r.match, env), macros)
        reads = []

        def prem(x, d):
            if not isinstance(x, list) or not x or isinstance(x[0], list):
                return
            h = x[0]
            if h in ("and", "or"):
                for k in x[1:]:
                    prem(k, d)
            elif h == "absent":
                for k in x[1:]:
                    prem(k, d + 1)
            elif h == "not":
                for k in x[1:]:
                    if isinstance(k, list) and k and is_name(k[0]) and k[0] != "eq":
                        reads.append((k[0], d % 2 == 1))
            elif h in ("eq", "neq", "false") or h in macros:
                pass
            elif is_name(h):
                reads.append((h, d % 2 == 1))
        prem(match, 0)
        writes = []

        def concl(x):
            if not isinstance(x, list) or not x or isinstance(x[0], list):
                return
            h = x[0]
            if h == "and":
                for k in x[1:]:
                    concl(k)
            elif h == "not":
                for k in x[1:]:
                    if isinstance(k, list) and k and is_name(k[0]) and k[0] != "eq":
                        writes.append(k)
            elif h in ("eq", "neq", "open"):
                pass
            elif h == "false":
                writes.append(["false"])
            elif is_name(h):
                writes.append(x)
        concl(subst_sexpr(r.assert_, env))
        for w in writes:
            rule_writes[r.name].add(w[0])
            for rel, neg in reads:
                edges.add((rel, w[0], neg))
            if r.params:
                edges.add((r.name, w[0], False))
            if w[0] in by_name and all(is_name(a) or isinstance(a, list) for a in w[1:]):
                for r2 in by_name[w[0]]:
                    if r2.params and len(r2.params) == len(w) - 1:
                        todo.append((r2, dict(zip(r2.params, w[1:]))))
    rels = sorted(set(a for a, b, n in edges) | set(b for a, b, n in edges))
    pos = {r: i for i, r in enumerate(rels)}
    succ = [[] for _ in rels]
    for a, b, n in sorted(edges):
        succ[pos[a]].append((pos[b], n))
    comps = sccs(len(rels), lambda v: [w for w, _ in succ[v]])
    comp_of = {}
    for ci, comp in enumerate(comps):
        for v in comp:
            comp_of[v] = ci
    stratified = True
    for a, b, n in edges:
        if n and comp_of[pos[a]] == comp_of[pos[b]]:
            stratified = False
    # Longest path into a component counting negative edges; Tarjan's order
    # is reverse topological, so walk it backwards.
    level = [0] * len(comps)
    for ci in range(len(comps) - 1, -1, -1):
        for v in comps[ci]:
            for w, n in succ[v]:
                cj = comp_of[w]
                if cj != ci:
                    level[cj] = max(level[cj], level[ci] + (1 if n else 0))
    stratum = {}
    for name, ws in rule_writes.items():
        stratum[name] = max(level[comp_of[pos[w]]] for w in ws if w in pos) if any(w in pos for w in ws) else 0
    return stratified, (max(level) + 1 if level else 0), stratum


def strata_section(args, say, census_files, files, rules, rules_by_file, macros_of, imported):
    say("### The engine's strata beside the static graph's", "")
    loaded = refused = silent = 0
    refused_files = set()
    verdict_same = count_same = 0
    rule_total = rule_same = 0
    examples = []
    detail = []
    for f in census_files:
        try:
            out = subprocess.run([args.bin, "saturate", f.path, "--dump"], cwd=ROOT,
                                 capture_output=True, text=True, timeout=120)
        except (OSError, subprocess.SubprocessError):
            refused += 1
            refused_files.add(f.path)
            continue
        line = [l for l in out.stdout.splitlines() if l.startswith(";; Strata")]
        if out.returncode != 0:
            refused += 1
            refused_files.add(f.path)
            continue
        if not line:
            silent += 1
            continue
        loaded += 1
        text = line[0]
        eng_strat = "not stratifiable" not in text and "stratified" in text
        eng_n = None
        for w in text.replace(",", " ").split():
            if w.isdigit():
                eng_n = int(w)
                break
        eng_rules = {}
        for l in out.stdout.splitlines():
            if l.startswith(";;   ") and ":params" in l and "stratum" in l:
                parts = l.split()
                try:
                    eng_rules[parts[1]] = int(parts[-1])
                except ValueError:
                    pass
        st = static_strata(f.path, files, rules_by_file, macros_of, imported, rules)
        mine_strat, mine_n, mine_rules = st
        v = eng_strat == mine_strat
        verdict_same += v
        c = eng_n == mine_n
        count_same += c
        same_r = 0
        for name, s in eng_rules.items():
            rule_total += 1
            if mine_rules.get(name, 0) == s:
                rule_same += 1
                same_r += 1
        detail.append((f.path, text, eng_n, mine_strat, mine_n, len(eng_rules), same_r))
        if not (v and c) and eng_rules and len(examples) < 8:
            examples.append([f"`{f.path}`", text.replace(";; Strata — ", ""),
                             ("stratified" if mine_strat else "not stratifiable") + f", {mine_n}",
                             f"{same_r} of {len(eng_rules)}"])
    ok_shapes = set(r.shapes["ii"] for r in rules if r.file not in refused_files)
    only = len(set(r.shapes["ii"] for r in rules if r.file in refused_files) - ok_shapes)
    say(f"`ein saturate FILE --dump` prints a stratum beside each rule and a `;; Strata` line, from "
        f"the graph `ein_infer::strata` builds over the plans the run compiled. The script builds the "
        f"same graph from the text: rules instantiated by the activators a file writes and by those "
        f"a rule concludes with every argument a name, no fact matched.", "",
        f"- files of the census the command saturates and prints the line for: {loaded}; saturated "
        f"with no line, having no rule: {silent}; **refused at load or stopped, exit not 0: {refused}**, "
        f"which hold {sum(1 for r in rules if r.file in refused_files)} of the census's {len(rules)} "
        f"rules ({len(set(r.shapes['alphau'] for r in rules if r.file in refused_files))} distinct), "
        f"and {only} shapes of level (ii) that no accepted file has. They are witnesses of a refusal "
        f"that live outside `{BROKEN}`, and the census reads them as written.",
        f"- the verdict (stratified or not) agrees on {verdict_same} of {loaded}; the number of "
        f"strata on {count_same}.",
        f"- a rule's stratum agrees for {rule_same} of the {rule_total} rules the dumps list.", "")
    if examples:
        say("The first files on which the verdict or the count differs:", "",
            md_table(["file", "the engine", "from the text", "rules' strata agreeing"], examples), "")
    with open(os.path.join(args.out, "strata.tsv"), "w", encoding="utf-8") as fh:
        fh.write("file\tengine\tengine_strata\tstatic_stratified\tstatic_strata\trules\trules_agreeing\n")
        for row in detail:
            fh.write("\t".join(str(x) for x in row) + "\n")
    with open(os.path.join(args.out, "refused.txt"), "w", encoding="utf-8") as fh:
        fh.write("".join(p + "\n" for p in sorted(refused_files)))


if __name__ == "__main__":
    main()
