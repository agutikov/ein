#!/usr/bin/env python3
"""What a model *set* is made of, and whether it factors — S1d.3.1's instrument.

The third census after [`layer_census.py`](layer_census.py) and
[`openness_census.py`](openness_census.py), and the first one whose subject is
the **answer** rather than the search or the program.
[P1d.3](../docs/history/m1d_satisfiability/README.md#p1d3--model-sets) exists to decide
whether 32 models should be printed or described, and its central hope was that
they would not have to be either — that a state with several independent open
choices *is* the compact answer, because the model count is then the product of
the candidate-set sizes. This measures the "independent".

    utils/model_set_census.py                     # the table, to stdout
    utils/model_set_census.py --json c.json       # + the machine copy
    utils/model_set_census.py -k zebra2-minus-15  # one entry, in full
    utils/model_set_census.py --no-leftover       # skip the blind probe

**The transport is `--json-summary`'s `verdict.solutions`**, read as k fact
sets, plus the `leftover` block the same stage added to it. Nothing here is
re-derived from the event stream: a census that reconstructs its own subject is
a census that can disagree with the engine.

## What a decision variable is, and who says so

A model set varies in *facts*; a factorisation is a claim about *variables*, so
something has to turn one into the other, and it must not be the zebra shape
hand-written into a script. Two rules, and the second is licensed by the
program rather than by this file:

1. **Every varying positive atom is a Boolean variable.** That is the general
   case — a fact is in a model or it is not — and it needs no declaration.
2. **A relation the program declares `functional` (or `bijective`, which fans
   out into it) makes the atoms `(R a ·)` mutually exclusive**, so for each `a`
   they collapse into *one* variable `(R, a)` whose domain is the set of values
   it takes. This is the only refinement, and the declaration is exactly the
   licence for it.

The declarations are read from the **models' own facts** — `(relation R …)`
and `(functional R)` hold in every model — not from the source text, so a
program that says `bijective`, one that says `functional`, and one that derives
the marker by a rule are all read the same way. An atom rule 2 does not reach
stays Boolean and is counted in `unrefined`: an entry whose variation is not
functional is a finding, not an unsupported input.

Varying **negatives** are not variables. Where negative completion writes
`(not (R a b))` beside every excluded value, the negatives mirror the positives
exactly, and counting both would square the description; `mirror` reports any
that do not pair up.

## The three questions, and which one is not trivial

* **by relation** — is one relation's projection independent of another's?
* **by variable, pairwise** — is `proj(u,v)` the full `dom(u) × dom(v)`?
  Components of the "no" graph are the only partition a factorisation could
  use, since two coupled variables can never be in different blocks.
* **by partition** — `Π |proj(component)| == k`. With **one** component this is
  `|proj(all)| == k`, which is true of every model set, so the honest verdict
  needs ≥ 2 components; with fewer than two varying variables there is nothing
  to factor and the entry is reported `degenerate` rather than counted.

And one the reconnaissance did not ask, which turns out to be where the toys
differ from the puzzle: **is the set a free grid over a small basis?** The
minimum determining set — the smallest variable set no two models agree on —
is a hitting set of the pairwise difference sets, and the model set is exactly
the product of that basis's domains when `Π |dom| == k`. A description that is
"these m facts are free, the rest follows" is compact and lossless; one that is
"here are k rows of m columns" is a list with fewer columns.

Argv follows `ein-corpus/src/plan.rs`, mirrored the way the other two censuses
mirror it, and for the same reason.
"""

from __future__ import annotations

import argparse
import itertools
import json
import math
import os
import subprocess
import sys
import tempfile
import time
import tomllib
from collections import Counter, defaultdict
from pathlib import Path

REPO = Path(__file__).resolve().parents[1]
MANIFEST = REPO / "corpus" / "corpus.toml"
EIN = Path(os.environ.get("EIN_BIN", REPO / "ein.rs" / "target" / "release" / "ein"))

#: Depth caps tried **downward** when the uncapped `solve -e` outlives the
#: budget — `layer_census.py`'s ladder, and for its reason: a depth-capped model
#: set is a subset, which is worth saying, and no model set at all is not.
CAPS_DOWN = (3, 2, 1)

#: Depth caps tried **upward** when the uncapped run finished without
#: exhausting. The other census never needed this because its subject was the
#: *layer*; here it is the model **set**, and a set the depth cap truncated is a
#: subset — every claim about it, a shared core most of all, is then a claim
#: about a superset of the truth. `-m 5` is a default rather than an answer, and
#: eight of the nine multi-model entries reach `exhausted` at a deeper cap for
#: under a quarter of a second.
CAPS_UP = (6, 8, 10)

#: …and it is tried **only where the run was cheap**. A deeper cap on an entry
#: that already costs seconds is not a measurement, it is a way to find the OOM
#: killer: `features/01_not_and_absent -e` enters `Σₖ C(35, k)` term for term
#: ([layer census](../docs/history/m1d_satisfiability/layer_census.md)),
#: which is 23.5 M commitments at `-m 8` alone and 2.7 GB before the clock runs
#: out. Nothing is lost by declining: the row reports the cap it used and
#: `exhausted = false` beside it, which is the honest form of "this is what the
#: search reached".
ESCALATE_BELOW_S = 2.0

#: Beyond this many combinations the minimum-key count is reported as a bound
#: rather than taken. Nothing in the corpus reaches it; the guard is so that a
#: future entry with fifty variables says so instead of hanging.
KEY_BUDGET = 4_000_000


# ── the s-expression a fact is rendered as ──────────────────

def sexpr(text: str):
    """`(not (color-loc Red House-2))` → `['not', ['color-loc', 'Red', 'House-2']]`.

    A whole reader rather than a `split()`, because a fact's argument may be a
    fact: `examples/ein-bugs/nested-fact-hypothesis.ein` is in the corpus
    precisely so that anything reading this transport has to cope.
    """
    toks, cur = [], ""
    for ch in text:
        if ch in "()":
            if cur:
                toks.append(cur)
                cur = ""
            toks.append(ch)
        elif ch.isspace():
            if cur:
                toks.append(cur)
                cur = ""
        else:
            cur += ch
    if cur:
        toks.append(cur)
    pos = 0

    def go():
        nonlocal pos
        t = toks[pos]
        pos += 1
        if t != "(":
            return t
        out = []
        while pos < len(toks) and toks[pos] != ")":
            out.append(go())
        pos += 1
        return out

    return go() if toks else None


# ── the sweep ───────────────────────────────────────────────

def argv_for(path: str, cap: int | None, out: Path) -> list[str]:
    argv = ["solve", path, "-e", "--json-summary", str(out)]
    if cap is not None:
        argv += ["-m", str(cap)]
    return argv


def run_once(path: str, cap: int | None, args, env: dict, out: Path):
    """One `solve -e`, or `None` if it outlived the budget."""
    if out.exists():
        out.unlink()
    t0 = time.perf_counter()
    try:
        subprocess.run(
            [str(args.bin), *argv_for(path, cap, out)],
            cwd=REPO, env=env, capture_output=True, timeout=args.timeout,
        )
    except subprocess.TimeoutExpired:
        return None
    wall = time.perf_counter() - t0
    if not out.exists():
        return {}                      # a load error: no fixpoint, nothing to read
    d = json.loads(out.read_text(encoding="utf-8"))
    d["_wall"] = wall
    d["_cap"] = cap
    return d


def measure(path: str, args, env: dict, out: Path) -> dict:
    """The deepest cap that fits the budget, and the model set it found.

    Both directions, because `-m 5` is a default and the subject is the set:
    down when the run does not finish, up when it finishes without exhausting.
    """
    d = run_once(path, None, args, env, out)
    if d is None:
        for cap in CAPS_DOWN:
            d = run_once(path, cap, args, env, out)
            if d is not None:
                break
        if d is None:
            return {"path": path, "note": f"no cap fits {args.timeout:.0f}s"}
    elif (d and not (d.get("stats") or {}).get("exhausted")
          and d.get("_wall", 0.0) < args.escalate_below):
        for cap in CAPS_UP:
            deeper = run_once(path, cap, args, env, out)
            if deeper is None:
                break
            d = deeper
            if (d.get("stats") or {}).get("exhausted"):
                break
    if not d:
        return {"path": path, "note": "no summary"}
    return {"path": path, "summary": d}


# ── the analysis ────────────────────────────────────────────

def declarations(core: set[str]) -> tuple[set[str], set[str]]:
    """`(relation R …)` and `(functional R)` / `(bijective R)`, from the models.

    Read from the facts every model shares, which is where a declaration
    necessarily lives: it held before the search and no branch removed it.
    """
    rels, funct = set(), set()
    for f in core:
        t = sexpr(f)
        if not isinstance(t, list) or not t or not isinstance(t[0], str):
            continue
        if t[0] == "relation" and len(t) >= 2 and isinstance(t[1], str):
            rels.add(t[1])
        elif t[0] in ("functional", "bijective") and len(t) == 2 and isinstance(t[1], str):
            funct.add(t[1])
    return rels, funct & rels


def variables(models: list[set[str]]):
    """Turn k fact sets into k assignments over decision variables.

    Returns `(assignments, domains, unrefined, mirror_gap)`. A variable is
    `(R, a)` where the program declared `R` functional — see the module
    docstring — and otherwise the atom itself, with domain `{present, absent}`.
    """
    core = set.intersection(*models)
    varies = set.union(*models) - core
    pos = sorted(f for f in varies if not f.startswith("(not "))
    neg = sorted(f for f in varies if f.startswith("(not "))
    _, funct = declarations(core)

    var_of, unrefined = {}, []
    # **A functional slot is a variable wherever it is, varying or not.** The
    # refined variables are read off the *union* of the models, so a slot the
    # puzzle pinned — `(drink-loc Milk House-3)`, a clue — is a variable with a
    # one-value domain rather than an invisible part of the core. That is the
    # number worth having: 25 slots of which 2 are stated is a fact about the
    # puzzle, where 23 slots is a fact about the answer.
    #
    # The unrefined atoms get the opposite treatment, and for the opposite
    # reason: a Boolean variable has no slot apart from its own presence, so a
    # *core* atom is not a fixed decision, it is just a fact. Counting all of
    # them would report `zebra2`'s model as 435 variables of which 340 are
    # fixed — which is the core/varies split, already two columns to the left.
    for f in sorted(set.union(*models)):
        if f.startswith("(not "):
            continue
        t = sexpr(f)
        if (isinstance(t, list) and len(t) == 3 and t[0] in funct
                and isinstance(t[1], str) and isinstance(t[2], str)):
            var_of[f] = ((t[0], t[1]), t[2])
        elif f in varies:
            var_of[f] = ((f,), "1")
            unrefined.append(f)
    names = sorted({v for v, _ in var_of.values()}, key=lambda v: (len(v), v))
    assignments = []
    for m in models:
        a = dict.fromkeys(names, "0")
        for f, (v, val) in var_of.items():
            if f in m:
                a[v] = val
        assignments.append(a)
    domains = {v: sorted({a[v] for a in assignments}) for v in names}
    # Negative completion writes `(not h)` beside each excluded value, so where
    # it runs the two halves mirror each other exactly. Anything left over is a
    # varying negative with no varying positive, and is worth naming.
    gap = sorted(set(f"(not {f})" for f in pos) ^ set(neg))
    return assignments, domains, unrefined, gap, core, pos, neg


def components(vary, domains, project):
    """Connected components of the pairwise-coupling graph, and its edges.

    `u` and `v` are **coupled** when `proj(u,v)` misses a combination their own
    domains allow. Two coupled variables cannot sit in different blocks of an
    independent partition, so a connected graph is a proof that no non-trivial
    partition exists — and the components are the only candidates when it is
    not connected.
    """
    coupled = []
    for u, v in itertools.combinations(vary, 2):
        if len(project([u, v])) < len(domains[u]) * len(domains[v]):
            coupled.append((u, v))
    parent = {v: v for v in vary}

    def find(x):
        while parent[x] != x:
            parent[x] = parent[parent[x]]
            x = parent[x]
        return x

    for u, v in coupled:
        ru, rv = find(u), find(v)
        if ru != rv:
            parent[ru] = rv
    blocks = defaultdict(list)
    for v in vary:
        blocks[find(v)].append(v)
    return coupled, [sorted(b, key=str) for b in blocks.values()]


def separating(vary, assignments):
    """Per variable, the bitmask of model pairs it tells apart.

    A variable set determines the model iff the OR of its masks is full — which
    makes "the minimum determining set" a minimum hitting set, and lets both the
    search and the count run on machine words.
    """
    pairs = list(itertools.combinations(range(len(assignments)), 2))
    sep = {}
    for v in vary:
        mask = 0
        for b, (i, j) in enumerate(pairs):
            if assignments[i][v] != assignments[j][v]:
                mask |= 1 << b
        sep[v] = mask
    return sep, (1 << len(pairs)) - 1


def min_key_size(vary, sep, full, limit):
    """The smallest determining set's size, by branch and bound.

    Iterative deepening over a hitting-set search that always branches on the
    *hardest* uncovered pair — the one fewest variables separate — so the tree
    is narrow where it matters. `None` when no set up to `limit` determines,
    which for a well-formed model set cannot happen: the whole variable set
    always does.
    """
    order = sorted(vary, key=lambda v: -bin(sep[v]).count("1"))

    def rec(covered, depth):
        if covered == full:
            return True
        if depth == 0:
            return False
        rest = full & ~covered
        best = None
        p = rest
        while p:
            bit = p & -p
            p ^= bit
            cands = [v for v in order if sep[v] & bit]
            if best is None or len(cands) < len(best):
                best = cands
            if len(best) <= 1:
                break
        for v in best:
            if rec(covered | sep[v], depth - 1):
                return True
        return False

    for d in range(1, limit + 1):
        if rec(0, d):
            return d
    return None


def all_keys(vary, sep, full, size, domains):
    """Every determining set of exactly `size`, and the **loosest** of them.

    The count says whether the basis is a *choice* or an accident. The one
    returned is the key with the smallest domain product, because that is the
    key the freeness test has to run on: the model set is the full grid over a
    basis exactly when `Π |dom| == k`, and a key with a larger product failing
    that says nothing about a key with a smaller one. Ties go to the first in
    canonical order.
    """
    n = len(vary)
    found, best, best_prod = 0, None, None

    def rec(start, covered, chosen):
        nonlocal found, best, best_prod
        need = size - len(chosen)
        if need == 0:
            if covered == full:
                found += 1
                prod = math.prod(len(domains[v]) for v in chosen)
                if best_prod is None or prod < best_prod:
                    best, best_prod = list(chosen), prod
            return
        if n - start < need:
            return
        for i in range(start, n - need + 1):
            v = vary[i]
            chosen.append(v)
            rec(i + 1, covered | sep[v], chosen)
            chosen.pop()

    rec(0, 0, [])
    return found, best, best_prod


def analyse(models: list[set[str]]) -> dict:
    """One model set, reduced to the row the census prints."""
    k = len(models)
    assignments, domains, unrefined, gap, core, pos, neg = variables(models)
    names = sorted(domains, key=lambda v: (len(v), v))
    vary = [v for v in names if len(domains[v]) > 1]
    fixed = [v for v in names if len(domains[v]) == 1]

    def project(S):
        S = list(S)
        return {tuple(a[v] for v in S) for a in assignments}

    product = math.prod(len(domains[v]) for v in vary)
    coupled, blocks = components(vary, domains, project)
    edges = set(coupled)
    block_projs = [len(project(b)) for b in blocks]

    row = dict(
        k=k, facts=len(models[0]),
        core=len(core), core_pos=sum(1 for f in core if not f.startswith("(not ")),
        core_neg=sum(1 for f in core if f.startswith("(not ")),
        varies=len(pos) + len(neg), varies_pos=len(pos), varies_neg=len(neg),
        mirror_gap=gap,
        vars=len(names), fixed=len(fixed), varying=len(vary),
        # **Reported, not dropped.** An atom the `functional` refinement could
        # not reach is a finding about the *program* — it varies over a
        # relation nobody declared single-valued — so the census names them
        # rather than folding them into a count and moving on.
        unrefined=len(unrefined), unrefined_atoms=unrefined,
        fixed_names=[list(v) for v in fixed],
        domain_sizes=dict(sorted(Counter(len(domains[v]) for v in vary).items())),
        product=product,
        pairs=len(vary) * (len(vary) - 1) // 2, coupled=len(coupled),
        free_pairs=[[list(u), list(v)] for u, v in
                    itertools.combinations(vary, 2) if (u, v) not in edges],
        components=[len(b) for b in blocks], component_projections=block_projs,
    )
    # Within a relation against across relations — injectivity makes each
    # relation's own varying values a clique, so the interesting number is the
    # other one.
    within = across = within_max = across_max = 0
    for u, v in itertools.combinations(vary, 2):
        same = len(u) == 2 and len(v) == 2 and u[0] == v[0]
        if same:
            within_max += 1
            within += (u, v) in edges
        else:
            across_max += 1
            across += (u, v) in edges
    row.update(within=within, within_max=within_max, across=across, across_max=across_max)

    # **The coarsest granularity, and the one the phase README asks first**: is
    # one relation's projection independent of another's? Coarser than the
    # pairwise variable test and *not* implied by it — two relations could be
    # pairwise-free variable by variable and still jointly constrained — so it
    # is measured rather than inferred. Only relations with a varying slot
    # appear; a Boolean atom is its own "relation" and is grouped under
    # `(atom)`, where the question is vacuous.
    by_rel = defaultdict(list)
    for v in vary:
        by_rel["(atom)" if len(v) == 1 else v[0]].append(v)
    rel_pairs = []
    for r1, r2 in itertools.combinations(sorted(by_rel), 2):
        p1, p2 = len(project(by_rel[r1])), len(project(by_rel[r2]))
        both = len(project(by_rel[r1] + by_rel[r2]))
        rel_pairs.append([r1, r2, p1, p2, both, both == p1 * p2])
    row["relation_pairs"] = rel_pairs
    row["relation_pairs_coupled"] = sum(1 for r in rel_pairs if not r[5])

    # **Degenerate** — fewer than two varying variables is not a factorisation
    # question. A one-variable model set is its own product and says nothing
    # about whether anything else is.
    row["degenerate"] = len(vary) < 2
    row["partition"] = (not row["degenerate"] and len(blocks) >= 2
                        and math.prod(block_projs) == k)

    if vary:
        sep, full = separating(vary, assignments)
        size = min_key_size(vary, sep, full, min(len(vary), 8))
        row["key_size"] = size
        if size is not None:
            combos = math.comb(len(vary), size)
            row["key_combinations"] = combos
            if combos <= KEY_BUDGET:
                found, example, prod = all_keys(vary, sep, full, size, domains)
                row["key_count"] = found
                row["key_example"] = [list(v) for v in (example or [])]
                row["key_domain_product"] = prod
            else:
                row["key_count"] = None
        row["min_domain"] = min(len(domains[v]) for v in vary)
        row["free_grid"] = (row.get("key_domain_product") == k)
        # A key of size m has product ≥ (min domain)^m and m ≥ `key_size`, so
        # when that bound already exceeds k **no key of any size is free** —
        # which is how "not a grid" is said without enumerating larger keys.
        # `None` when the key search hit its limit and there is no exponent.
        row["no_free_key"] = (None if size is None
                              else row["min_domain"] ** size > k)
    return row


# ── the forms — S1d.3.2 ─────────────────────────────────────
#
# `--form` renders a model set as one of the candidate **representations**
# [S1d.3.2](../docs/history/m1d_satisfiability/README.md#s1d32--representations)
# prices, because *a representation argued about in prose and never printed is
# a representation nobody has read*. Nothing here touches the engine — every
# form is a rendering of the same `verdict.solutions` the census already reads.
#
# `envelope` and `key` are the stage's (a) and (b). `list` is (e) rendered in
# the **same alphabet**, so the readability test compares forms rather than
# formatting — what `solve -e` actually prints is a different thing and is
# quoted beside it in the record. `diagram` is (c), and it is a *price* rather
# than a picture: the exact reduced-MDD node count under several variable
# orders, which is the only way to answer "how big would the diagram be"
# without building one.

def fmt_var(v) -> str:
    return f"{v[0]}:{v[1]}" if len(v) == 2 else v[0]


def _model_view(states):
    """The pieces every form needs: assignments, domains, the split, the key."""
    assignments, domains, unrefined, _gap, core, pos, _neg = variables(states)
    names = sorted(domains, key=lambda v: (len(v), v))
    vary = [v for v in names if len(domains[v]) > 1]
    fixed = [v for v in names if len(domains[v]) == 1]
    return assignments, domains, vary, fixed, core, pos, unrefined


def form_envelope(path, states) -> None:
    """(a) — the certain core and the varying frontier, **labelled**.

    The label is not decoration. This is the smallest box containing the model
    set, and printing it without the ratio is the failure mode
    [S1d.3.2](../docs/history/m1d_satisfiability/README.md#s1d32--representations)
    names: a reader told each slot's range will read the *product* as the
    answer.
    """
    assignments, domains, vary, fixed, core, _pos, _unref = _model_view(states)
    k = len(states)
    cells = math.prod(len(domains[v]) for v in vary)
    print(f"\n## (a) envelope — {path}   [OVER-APPROXIMATION]\n")
    print(f"  the box has {cells:,} cells; the set has {k}."
          f"  over-approximation {cells / k:.3g}×")
    print(f"  what follows says which facts are settled — never which "
          f"combinations occur.\n")
    by_rel = Counter()
    for f in core:
        t = sexpr(f)
        head = t[0] if isinstance(t, list) else t
        if head == "not" and isinstance(t[1], list):
            head = f"not {t[1][0]}"
        by_rel[head] += 1
    print(f"  certain — {len(core)} facts in all {k} models")
    cells_line = [f"{rel} {n}"
                  for rel, n in sorted(by_rel.items(), key=lambda kv: (-kv[1], kv[0]))]
    line = []
    for c in cells_line:
        if line and sum(len(x) + 2 for x in line) + len(c) > 74:
            print("    " + " · ".join(line))
            line = []
        line.append(c)
    if line:
        print("    " + " · ".join(line))
    if fixed:
        print(f"\n    of which decided slots: {len(fixed)}")
        for v in fixed:
            print(f"      {fmt_var(v):24} = {domains[v][0]}")
    print(f"\n  varying — {len(vary)} slots")
    for v in vary:
        print(f"    {fmt_var(v):24} ∈ {{{', '.join(domains[v])}}}")


def form_key(path, states) -> None:
    """(b) — a determining key and its table, exact.

    The key is the minimum determining set with the **smallest domain
    product**, which is the tightest such table; the count of equally minimal
    keys is printed beside it, because *"why these four"* is the objection the
    stage says (b) has to answer.
    """
    assignments, domains, vary, _fixed, _core, _pos, _unref = _model_view(states)
    k = len(states)
    sep, full = separating(vary, assignments)
    size = min_key_size(vary, sep, full, min(len(vary), 8))
    if size is None:
        print(f"\n## (b) key — {path}\n\n  no determining set up to size 8.")
        return
    combos = math.comb(len(vary), size)
    if combos > KEY_BUDGET:
        print(f"\n## (b) key — {path}\n\n  minimum key is {size} variables; "
              f"C({len(vary)}, {size}) = {combos:,} is over the budget, so the "
              f"table is not enumerable here.")
        return
    found, key, prod = all_keys(vary, sep, full, size, domains)
    # Which variables every minimum key contains — the answer to "why these".
    always = None
    seen_all = []

    def collect(start, covered, chosen):
        need = size - len(chosen)
        if need == 0:
            if covered == full:
                seen_all.append(tuple(chosen))
            return
        if len(vary) - start < need:
            return
        for i in range(start, len(vary) - need + 1):
            chosen.append(vary[i])
            collect(i + 1, covered | sep[vary[i]], chosen)
            chosen.pop()

    collect(0, 0, [])
    if seen_all:
        always = set(seen_all[0]).intersection(*[set(c) for c in seen_all])
    print(f"\n## (b) key — {path}   [EXACT]\n")
    print(f"  {size} of {len(vary)} variables determine the model; "
          f"{found} such {size}-sets exist.")
    print(f"  This one's domains allow fewest combinations — {prod}, "
          f"of which {k} occur.")
    if always:
        print(f"  Every one of the {found} contains: "
              f"{', '.join(sorted(fmt_var(v) for v in always))}")
    print()
    head = [fmt_var(v) for v in key]
    w = [max(len(h), max(len(a[v]) for a in assignments)) for h, v in zip(head, key)]
    print(("    " + "  ".join(f"{h:<{n}}" for h, n in zip(head, w))).rstrip())
    print("    " + "  ".join("-" * n for n in w))
    rows = sorted(tuple(a[v] for v in key) for a in assignments)
    for r in rows:
        print(("    " + "  ".join(f"{c:<{n}}" for c, n in zip(r, w))).rstrip())
    print(f"\n  {len(rows)} rows. The other {len(vary) - size} varying slots "
          f"follow:\n  re-saturate with a row and the model is fixed.")


def form_list(path, states) -> None:
    """(e) — the enumeration, in the same alphabet as (a) and (b).

    Not what `solve -e` prints. This is the control arm rendered so that the
    three forms differ in *structure* rather than in formatting, which is what
    the readability test needs; the real output is quoted beside it.
    """
    assignments, domains, vary, _fixed, core, _pos, _unref = _model_view(states)
    print(f"\n## (e) list — {path}   [EXACT]\n")
    print(f"  {len(states)} models × {len(vary)} varying slots, "
          f"+ {len(core)} facts shared by all of them.\n")
    head = [fmt_var(v) for v in vary]
    w = [max(len(h), max(len(a[v]) for a in assignments)) for h, v in zip(head, vary)]
    print(("    " + " ".join(f"{h:<{n}}" for h, n in zip(head, w))).rstrip())
    print("    " + " ".join("-" * n for n in w))
    for a in sorted(assignments, key=lambda a: tuple(a[v] for v in vary)):
        print(("    " + " ".join(f"{a[v]:<{n}}" for v, n in zip(vary, w))).rstrip())


#: How many random variable orders `--form diagram` tries before reporting the
#: best it found. A reduced MDD's size is order-dependent and finding the
#: optimum is NP-hard; the honest report is "the best of N, and the heuristics".
DIAGRAM_ORDERS = 500


def mdd_levels(assignments, order) -> list[int]:
    """Nodes per level of the **reduced** MDD for this variable order.

    A node at level *i* is a distinct *residual set* — the set of suffixes
    still reachable after fixing the first *i* variables — because two prefixes
    share a node exactly when what remains possible after them is the same set.
    That makes the count exact rather than an estimate, and it is what "how big
    would the diagram be" means.

    Two bounds fall straight out and are the reason (c) is priced this way: a
    level has at least one node and at most *k* of them, so the whole diagram
    is between `n + 1` and `n·k + 1` nodes **whatever the order**. A decision
    diagram is a win when *k* is exponential in *n*; at `k = 32` it cannot be
    one, and no variable order changes that.
    """
    out = []
    for i in range(len(order) + 1):
        groups = defaultdict(set)
        for a in assignments:
            groups[tuple(a[v] for v in order[:i])].add(tuple(a[v] for v in order[i:]))
        out.append(len({frozenset(g) for g in groups.values()}))
    return out


def mdd_edges(assignments, order) -> int:
    """Outgoing edges of the reduced MDD — one per (node, value it accepts).

    Nodes alone under-state a diagram: what a reader or a consumer has to hold
    is nodes **plus** the labelled edges between them, and a multi-valued
    variable contributes up to `|dom|` of them per node.
    """
    total = 0
    for i in range(len(order)):
        groups = defaultdict(set)
        for a in assignments:
            groups[tuple(a[v] for v in order[:i])].add(tuple(a[v] for v in order[i:]))
        for node in {frozenset(g) for g in groups.values()}:
            total += len({suffix[0] for suffix in node})
    return total


def form_diagram(path, states, seed: int = 20260825) -> None:
    """(c) — priced, not built: the exact node count under several orders."""
    import random

    assignments, domains, vary, _fixed, _core, _pos, _unref = _model_view(states)
    k = len(states)
    sep, full = separating(vary, assignments)
    size = min_key_size(vary, sep, full, min(len(vary), 8))
    key = []
    if size is not None and math.comb(len(vary), size) <= KEY_BUDGET:
        _n, key, _p = all_keys(vary, sep, full, size, domains)

    def edges_of(vs):
        e = {v: 0 for v in vs}
        for u, w in itertools.combinations(vs, 2):
            p = {tuple(a[x] for x in (u, w)) for a in assignments}
            if len(p) < len(domains[u]) * len(domains[w]):
                e[u] += 1
                e[w] += 1
        return e

    deg = edges_of(vary)
    orders = {
        "canonical (the census's)": list(vary),
        "domain size, ascending": sorted(vary, key=lambda v: (len(domains[v]), str(v))),
        "coupling degree, descending": sorted(vary, key=lambda v: (-deg[v], str(v))),
        "key variables first": ([v for v in vary if v in set(key)]
                                + [v for v in vary if v not in set(key)]),
    }
    rng = random.Random(seed)
    best, best_order = None, None
    for _ in range(DIAGRAM_ORDERS):
        o = list(vary)
        rng.shuffle(o)
        lv = mdd_levels(assignments, o)
        n = sum(lv[:-1]) + 1
        if best is None or n < best:
            best, best_order = n, o
    orders[f"best of {DIAGRAM_ORDERS} random"] = best_order

    print(f"\n## (c) decision diagram — {path}   [PRICED, NOT BUILT]\n")
    print(f"  Exact reduced-MDD node counts over the {len(vary)} varying "
          f"variables. A node is a\n  distinct residual set, so these are "
          f"counts and not bounds.\n")
    print(f"    {'variable order':30} {'nodes':>7} {'edges':>7} {'widest':>7}")
    print(f"    {'-' * 30} {'-' * 7} {'-' * 7} {'-' * 7}")
    for name, o in orders.items():
        lv = mdd_levels(assignments, o)
        print(f"    {name:30} {sum(lv[:-1]) + 1:>7} "
              f"{mdd_edges(assignments, o):>7} {max(lv):>7}")
    if key:
        lv = mdd_levels(assignments, list(key))
        print(f"    {'the ' + str(len(key)) + '-variable key alone':30} "
              f"{sum(lv[:-1]) + 1:>7} {mdd_edges(assignments, list(key)):>7} "
              f"{max(lv):>7}")
    n = len(vary)
    print(f"\n  bounds, for any order at all: {n + 1} ≤ nodes ≤ {n * k + 1} "
          f"(a level has ≥ 1 node and ≤ k).")
    print(f"  against the enumeration: {k} rows × {n} columns = {k * n} cells; "
          f"against the key\n  table: {k} rows × {len(key)} columns "
          f"= {k * len(key)} cells.")
    print(f"  A diagram is a win when k is exponential in n. At k = {k} it "
          f"cannot be one, and\n  no variable order changes that — which is "
          f"the pricing, not a defeat for this order.")


FORMS = {
    "envelope": form_envelope,
    "key": form_key,
    "list": form_list,
    "diagram": form_diagram,
}


def rows_of(entries, args, env) -> list[dict]:
    out = []
    with tempfile.TemporaryDirectory(prefix="model-set-") as td:
        summary = Path(td) / "summary.json"
        todo = [e for e in entries
                if "solve" in [r.split()[0] for r in e.get("runs", [])]
                and (not args.key or args.key in e["path"])]
        for i, e in enumerate(todo, 1):
            if not args.quiet:
                print(f"  [{i}/{len(todo)}] {e['path']}", file=sys.stderr, flush=True)
            row = measure(e["path"], args, env, summary)
            d = row.pop("summary", None)
            if d is None:
                out.append(row)
                continue
            v = d.get("verdict") or {}
            models = [set(s["facts"]) for s in (v.get("solutions") or [])]
            opens = [set(s["facts"]) for s in (v.get("open_states") or [])]
            row.update(
                verdict=v.get("type"), k=v.get("k"), cap=d.get("_cap"),
                exhausted=(d.get("stats") or {}).get("exhausted"),
                wall=round(d.get("_wall", 0.0), 3),
                leftover=(d.get("leftover") or {}).get("models") or [],
                leftover_open=(d.get("leftover") or {}).get("open_states") or [],
                n_open_states=len(opens),
            )
            states = models if len(models) >= 2 else opens
            if len(states) >= 2:
                row["set"] = analyse(states)
                if states is opens:
                    row["set"]["states"] = "open"
                # Kept for `--form`, stripped before the JSON: a machine copy
                # carrying every fact of every model is the enumeration this
                # phase is trying to price, not a census row.
                if args.form:
                    row["_states"] = states
            out.append(row)
    return out


def multi(rows):
    return [r for r in rows if r.get("set")]


def print_sets(rows):
    print("\n## The model sets\n")
    print(f"{'entry':50} {'cap':>4} {'k':>4} {'exh':>5} {'facts':>6} "
          f"{'core':>6} {'varies':>7} {'vars':>5} {'fix':>4} {'unref':>6} {'wall':>7}")
    print(f"{'-'*50} {'-'*4} {'-'*4} {'-'*5} {'-'*6} {'-'*6} {'-'*7} "
          f"{'-'*5} {'-'*4} {'-'*6} {'-'*7}")
    for r in sorted(multi(rows), key=lambda r: r["path"]):
        s = r["set"]
        cap = "-" if r.get("cap") is None else str(r["cap"])
        print(f"{r['path']:50} {cap:>4} {s['k']:>4} {str(r.get('exhausted')):>5} "
              f"{s['facts']:>6} {s['core']:>6} {s['varies']:>7} {s['varying']:>5} "
              f"{s['fixed']:>4} {s['unrefined']:>6} {r.get('wall', 0):>7.2f}")
    other = [r for r in rows if not r.get("set") and r.get("k") is not None]
    done = [r for r in multi(rows) if r.get("exhausted")]
    print(f"\n{len(multi(rows))} entries with a set to describe; "
          f"{len(other)} with one model or none; "
          f"{len([r for r in rows if r.get('note')])} unmeasured.")
    # **Which of them is a whole set.** Everything below describes what the
    # search recorded; where `exhausted` is false that is a *subset*, and every
    # claim about a model set — a shared core most of all — is then a claim
    # about a superset of the truth. Intersecting fewer models gives more core,
    # so this is the easy mistake rather than a remote one.
    print(f"{len(done)} of the {len(multi(rows))} are exhausted; "
          f"{len(multi(rows)) - len(done)} are what the depth cap reached.")


def print_factorisation(rows):
    print("\n## Does anything factor\n")
    print(f"{'entry':50} {'k':>4} {'Π dom':>14} {'ratio':>10} {'cpl/pairs':>11} "
          f"{'comps':>6} {'part':>5} {'key':>4} {'Πkey':>6} {'grid':>5}")
    print(f"{'-'*50} {'-'*4} {'-'*14} {'-'*10} {'-'*11} {'-'*6} {'-'*5} "
          f"{'-'*4} {'-'*6} {'-'*5}")
    for r in sorted(multi(rows), key=lambda r: r["path"]):
        s = r["set"]
        mark = "  (degenerate)" if s["degenerate"] else ""
        print(f"{r['path']:50} {s['k']:>4} {s['product']:>14} "
              f"{s['product'] / s['k']:>10.4g} "
              f"{s['coupled']:>5}/{s['pairs']:<5} {len(s['components']):>6} "
              f"{str(s['partition']):>5} {str(s.get('key_size')):>4} "
              f"{str(s.get('key_domain_product')):>6} "
              f"{str(s.get('free_grid')):>5}{mark}")
    rp = sum(len(r["set"].get("relation_pairs") or []) for r in multi(rows))
    rc = sum(r["set"].get("relation_pairs_coupled", 0) for r in multi(rows))
    print(f"\nby relation, over every entry: {rc} of {rp} relation pairs coupled")
    part = [r for r in multi(rows) if r["set"]["partition"]]
    grid = [r for r in multi(rows) if r["set"].get("free_grid")]
    deg = [r for r in multi(rows) if r["set"]["degenerate"]]
    print(f"\npartition into independent blocks: {len(part)} of {len(multi(rows))}"
          f"   free grid over a basis: {len(grid)}   "
          f"degenerate (<2 varying): {len(deg)}")
    for r in grid:
        s = r["set"]
        basis = [fmt_var(tuple(v)) for v in s["key_example"]]
        print(f"   grid  {r['path']}: {s['k']} models = the free product of "
              f"{len(basis)} — {basis}")


def print_coupling(rows):
    print("\n## What the coupling is made of\n")
    print(f"{'entry':50} {'within':>12} {'across':>12} {'min deg':>8} {'density':>8}")
    print(f"{'-'*50} {'-'*12} {'-'*12} {'-'*8} {'-'*8}")
    for r in sorted(multi(rows), key=lambda r: r["path"]):
        s = r["set"]
        dens = s["coupled"] / s["pairs"] if s["pairs"] else 1.0
        # The coupling graph is the complete graph minus the free pairs, so the
        # minimum degree is `n − 1` less whatever the most-freed variable
        # escaped. A small minimum degree is the only thing that could become a
        # small separator, and a small separator is what a decision diagram
        # needs.
        freest = max(Counter(tuple(u) for pair in s["free_pairs"] for u in pair)
                     .values(), default=0)
        print(f"{r['path']:50} {s['within']:>5}/{s['within_max']:<6} "
              f"{s['across']:>5}/{s['across_max']:<6} "
              f"{s['varying'] - 1 - freest:>8} {dens:>8.3f}")


def print_leftover(rows):
    print("\n## The leftover-open count\n")
    print("What the **blind** enumerator would still propose at a state the "
          "active rung\ncalled complete — `EIN_LEFTOVER=1`, on a discarded "
          "fork. A model with n of\nthese is one model closed-world and 2ⁿ "
          "open-world.\n")
    print(f"{'entry':50} {'states':>6} {'min':>7} {'max':>7} {'distinct':>9}")
    print(f"{'-'*50} {'-'*6} {'-'*7} {'-'*7} {'-'*9}")
    zero, nonzero = [], []
    for r in rows:
        lo = (r.get("leftover") or []) + (r.get("leftover_open") or [])
        if not lo:
            continue
        (zero if max(lo) == 0 else nonzero).append((r, lo))
    for r, lo in sorted(nonzero, key=lambda t: -max(t[1])):
        print(f"{r['path']:50} {len(lo):>6} {min(lo):>7} {max(lo):>7} "
              f"{len(set(lo)):>9}")
    print(f"\n{len(zero)} entries whose every recorded state is closed — the "
          f"blind enumerator\nproposes nothing there, so open-world and "
          f"closed-world agree; {len(nonzero)} with\nsomething still open, "
          f"listed above.")


def print_entry(r):
    """`-k`'s output — one entry, with the variables named."""
    print(f"\n## {r['path']}\n")
    if not r.get("set"):
        print(f"  {r.get('note') or f'k={r.get('k')} — no set to describe'}")
        return
    s = r["set"]
    print(f"  verdict    {r.get('verdict')}  k={s['k']}  "
          f"exhausted={r.get('exhausted')}  cap={r.get('cap')}  {r.get('wall')}s")
    print(f"  facts      {s['facts']} per model; core {s['core']} "
          f"({s['core_pos']}+ {s['core_neg']}−), varies {s['varies']} "
          f"({s['varies_pos']}+ {s['varies_neg']}−)")
    if s["mirror_gap"]:
        print(f"  mirror     {len(s['mirror_gap'])} unpaired: "
              f"{s['mirror_gap'][:4]}")
    print(f"  variables  {s['vars']} ({s['varying']} varying, {s['fixed']} fixed, "
          f"{s['unrefined']} unrefined atoms); domains {s['domain_sizes']}")
    for f in (s.get("unrefined_atoms") or [])[:6]:
        print(f"    unref    {f}")
    if len(s.get("unrefined_atoms") or []) > 6:
        print(f"    unref    … and {len(s['unrefined_atoms']) - 6} more")
    for v in s["fixed_names"]:
        print(f"    fixed    {fmt_var(tuple(v))}")
    print(f"  product    {s['product']}  against k={s['k']} — "
          f"ratio {s['product'] / s['k']:.4g}")
    print(f"  coupling   {s['coupled']} of {s['pairs']} pairs; within-relation "
          f"{s['within']}/{s['within_max']}, across {s['across']}/{s['across_max']}")
    for u, v in s["free_pairs"][:8]:
        print(f"    free     {fmt_var(tuple(u))}  ×  {fmt_var(tuple(v))}")
    if len(s["free_pairs"]) > 8:
        print(f"    free     … and {len(s['free_pairs']) - 8} more")
    for r1, r2, p1, p2, both, indep in s.get("relation_pairs") or []:
        print(f"    by-rel   {r1:14} × {r2:14} |P1|={p1:<4} |P2|={p2:<4} "
              f"|P12|={both:<4} of {p1 * p2:<6} "
              f"{'INDEPENDENT' if indep else 'coupled'}")
    print(f"  components {len(s['components'])} {s['components']} → projections "
          f"{s['component_projections']}   partition={s['partition']}")
    print(f"  key        size {s.get('key_size')} — {s.get('key_count')} of "
          f"{s.get('key_combinations')} combinations determine all {s['k']}")
    if s.get("key_example"):
        print(f"    e.g.     {[fmt_var(tuple(v)) for v in s['key_example']]} → "
              f"{s['k']} of {s.get('key_domain_product')} its domains allow")
    print(f"    grid     {s.get('free_grid')}; no key of any size is free: "
          f"{s.get('no_free_key')} (min domain {s.get('min_domain')})")
    lo = r.get("leftover") or []
    if lo:
        print(f"  leftover   {min(lo)}…{max(lo)} facts the blind enumerator "
              f"would still propose")
    print(f"  printed    {s['k']} × {s['facts']} = {s['k'] * s['facts']} fact lines")


def main() -> int:
    ap = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    ap.add_argument("--bin", default=EIN, type=Path,
                    help=f"the ein binary (default $EIN_BIN or {EIN})")
    ap.add_argument("--json", type=Path, help="also write the rows as JSON")
    ap.add_argument("-k", "--key", help="only entries whose path contains this")
    ap.add_argument("--form", choices=sorted(FORMS),
                    help="render the model set as one of S1d.3.2's candidate "
                         "representations instead of the census row")
    ap.add_argument("--no-leftover", action="store_true",
                    help="skip the blind-enumerator probe (EIN_LEFTOVER)")
    ap.add_argument("--timeout", type=float, default=90.0,
                    help="seconds per run (default 90)")
    ap.add_argument("--escalate-below", type=float, default=ESCALATE_BELOW_S,
                    metavar="S", help="try a deeper -m only when the run cost "
                                      f"less than this (default {ESCALATE_BELOW_S})")
    ap.add_argument("-q", "--quiet", action="store_true", help="no progress lines")
    args = ap.parse_args()

    if not Path(args.bin).exists():
        print(f"no engine at {args.bin} — run ./build.sh, "
              f"or name one with --bin / $EIN_BIN", file=sys.stderr)
        return 2

    env = dict(os.environ)
    env.pop("EIN_STDLIB", None)
    env["LC_ALL"] = "C"
    if not args.no_leftover:
        env["EIN_LEFTOVER"] = "1"

    entries = tomllib.loads(MANIFEST.read_text(encoding="utf-8"))["entry"]
    rows = rows_of(entries, args, env)
    if not rows:
        print("no entries matched", file=sys.stderr)
        return 2

    if args.form:
        shown = 0
        for r in rows:
            if not r.get("_states"):
                continue
            FORMS[args.form](r["path"], r["_states"])
            shown += 1
        if not shown:
            print("no entry matched with a model set to render", file=sys.stderr)
            return 2
    elif args.key:
        for r in rows:
            print_entry(r)
    else:
        print_sets(rows)
        print_factorisation(rows)
        print_coupling(rows)
        if not args.no_leftover:
            print_leftover(rows)
    if args.json:
        # `_`-prefixed keys are working state (`--form`'s fact sets), never the
        # machine copy: a JSON carrying every fact of every model is the
        # enumeration this phase exists to price.
        clean = [{k: v for k, v in r.items() if not k.startswith("_")} for r in rows]
        args.json.write_text(json.dumps(clean, indent=1), encoding="utf-8")
        print(f"\nwrote {args.json}", file=sys.stderr)
    return 0


if __name__ == "__main__":
    sys.exit(main())
