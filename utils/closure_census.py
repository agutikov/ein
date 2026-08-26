#!/usr/bin/env python3
"""What a closure claim costs, and who makes one — S1d.4.1's instrument.

The fourth census after [`layer_census.py`](layer_census.py),
[`openness_census.py`](openness_census.py) and
[`model_set_census.py`](model_set_census.py), and the first whose subject is
the **claim**: not the search, not the program, not the answer, but the
sentence a file writes about its own answer.

    utils/closure_census.py                  # the four tables, to stdout
    utils/closure_census.py --json c.json    # + the machine copy
    utils/closure_census.py --no-solve       # tables 1-2 only (0.05 s, no sweep)
    utils/closure_census.py -k zebra2        # the counterfactual for one entry

[P1d.4](../plans/m1d_satisfiability/p1d.4_model_set_closure/README.md) exists
because `:expect (or M1 … Mk)` is two claims wearing one coat — *each Mi is a
model*, which the search finds anyway, and *there is no M(k+1)*, which is
established only by exhausting a lattice that does not finish. This measures
how much of the corpus is in that position.

## Parsed, never grepped — and the reason is not hypothetical

`:expect` is a query keyword, and a grep cannot tell a keyword from a comment
about one. **S1d.4.1's own reconnaissance proved it**: grepping `:expect (or`
found two users, and one of them is `examples/features/10_expect.ein`, whose
`:expect` is a `(model …)` and whose *header comment* documents the `(or …)`
form on line 12. So the transport here is `ein test --json-report`, which the
same stage added — one row per `(query …)` of the selection, the claim's shape
read off the **loaded** program, and the outcome the runner came to. A census
that re-implements its subject is a census that can disagree with the engine,
which is the rule the other three already keep.

## Two costs, and only one of them is the phase's headline

A claim that does not exist has both:

* **to verify** — exhaust the lattice. This is what does not finish, and the
  census reads it off the same `solve -e` `ein test` would run: `exhausted`.
* **to write** — `goal relations × models × facts`, because *naming a relation
  closes it*, so an expectation must list the complete extent of every relation
  its `:goal` asks about, in every model. Nothing had put a number on this one.

The write cost is a *counterfactual* on the entries that have no claim, so its
arithmetic is checked against the 38 `(model …)` claims that do exist: for each
of them the predicted extent is compared with the facts the file actually
lists. A formula that is only ever applied where it cannot be checked is a
formula nobody has tested.

## Which run, and why that one

`ein test`'s regime, exactly: **exhausting, `-m 5`, one job** — which is
`ein solve -e` with no `-m`, since both default to 5. That matters because
table 4's question is *what would a closure claim on this entry come back as*,
and the answer is only `NOT CHECKED` if the run `ein test` performs is the run
that fails to exhaust. Where the uncapped run outlives the budget the ladder
drops to `-m 3, 2, 1` — `model_set_census.py`'s `CAPS_DOWN`, for its reason: a
depth-capped model set is a **subset**, which is worth reporting, and no model
set at all is not. Such a row's write cost is a *lower bound* and is marked —
and it is **not** counted in table 4, because `exhausted = false` at `-m 3` is
consistent with `true` at `-m 5`; only exhaustion travels upward. A ladder row
that did not exhaust is reported as *over the census's budget*, which is a
statement about this script and not about the runner.

Argv follows `ein-corpus/src/plan.rs`, mirrored the way the other three
censuses mirror it, and for the same reason.
"""

from __future__ import annotations

import argparse
import json
import os
import subprocess
import sys
import tempfile
import time
import tomllib
from pathlib import Path

REPO = Path(__file__).resolve().parents[1]
MANIFEST = REPO / "corpus" / "corpus.toml"
EIN = Path(os.environ.get("EIN_BIN", REPO / "ein.rs" / "target" / "release" / "ein"))

#: The three corpus roots — the same set `corpus.toml`'s completeness check
#: covers, and the whole of what `ein test` is ever pointed at.
ROOTS = ("examples", "tests", "stdlib")

#: Depth caps tried downward when the uncapped `solve -e` outlives the budget.
#: `model_set_census.py`'s ladder and its rationale; the difference is that a
#: row reached this way answers the *write* question and not the *verify* one,
#: since the verify question was already answered by the timeout.
CAPS_DOWN = (3, 2, 1)

#: The three shapes, in the order the tables print them.
SHAPES = ("model", "or", "false")


# ── the report: who claims what ─────────────────────────────

def report(args) -> dict:
    """`ein test <roots> --json-report`, in one process.

    Exit code ignored on purpose: it is 2 over `examples/`, because 41 fixtures
    under `broken/` exist to fail to load, and that is the run behaving. What
    is read is the file.
    """
    with tempfile.TemporaryDirectory(prefix="closure-") as td:
        out = Path(td) / "report.json"
        argv = [str(args.bin), "test", *args.roots, "-q", "--json-report", str(out)]
        p = subprocess.run(argv, cwd=REPO, capture_output=True, text=True,
                           timeout=args.timeout)
        if not out.exists():
            print(p.stderr.strip() or f"{argv} wrote no report", file=sys.stderr)
            return {}
        return json.loads(out.read_text(encoding="utf-8"))


def root_of(path: str) -> str:
    head = path.split("/", 1)[0]
    return head if head in ROOTS else "(other)"


def print_usage(rows: list[dict]) -> None:
    """Table 1 — the denominator, which is the point.

    Not "how many files carry an `:expect`" but *what fraction of the queries
    the corpus asks state a claim about their own answer*, and of those, how
    many state the one about a **set**.
    """
    print("\n## 1. Who states a claim\n")
    cols = ("files", "no-query", "refused", "queries", "claims", *SHAPES)
    print(f"{'root':12} " + " ".join(f"{c:>9}" for c in cols))
    print(f"{'-'*12} " + " ".join("-" * 9 for _ in cols))
    totals = dict.fromkeys(cols, 0)
    for root in (*ROOTS, "(other)"):
        sub = [r for r in rows if root_of(r["path"]) == root]
        if not sub:
            continue
        cell = {
            "files": len({r["path"] for r in sub}),
            "no-query": sum(1 for r in sub if r["outcome"] == "no-query"),
            "refused": sum(1 for r in sub if r["outcome"] == "error" and not r["queries"]),
            "queries": sum(1 for r in sub if r["query"] >= 1),
            "claims": sum(1 for r in sub if r["expect"]),
        }
        for s in SHAPES:
            cell[s] = sum(1 for r in sub if (r["expect"] or {}).get("shape") == s)
        print(f"{root + '/':12} " + " ".join(f"{cell[c]:>9}" for c in cols))
        for c in cols:
            totals[c] += cell[c]
    print(f"{'total':12} " + " ".join(f"{totals[c]:>9}" for c in cols))
    n_or = totals["or"]
    print(f"\n  {totals['claims']} of {totals['queries']} queries state a claim; "
          f"**{n_or}** state a claim about a model *set*.")
    for r in rows:
        if (r["expect"] or {}).get("shape") == "or":
            e = r["expect"]
            print(f"    {r['path']}  ·  k = {e['models']}, {e['facts']} facts, "
                  f"outcome {r['outcome']}")


# ── the report: is the claim checkable ──────────────────────

def print_verifiable(rows: list[dict], args) -> None:
    """Table 2 — every claim, with the outcome and the depth it reached.

    The acceptance bullet is *every* expectation-carrying entry gets a row, and
    the interesting column is the one that comes back empty.
    """
    claims = [r for r in rows if r["expect"]]
    print("\n## 2. Is the claim checkable today\n")
    if not args.long:
        print("  (per-entry rows: --long)\n")
    else:
        head = ("shape", "mdl", "facts", "outcome", "verdict", "k", "exh", "lyr", "ms")
        print(f"{'entry':52} {'shape':>6} {'mdl':>4} {'facts':>6} {'outcome':>11} "
              f"{'verdict':>13} {'k':>3} {'exh':>4} {'lyr':>4} {'ms':>7}")
        print(f"{'-'*52} " + " ".join("-" * len(h.rjust(w)) for h, w in
                                      zip(head, (6, 4, 6, 11, 13, 3, 4, 4, 7))))
        for r in sorted(claims, key=lambda r: r["path"]):
            e, ran = r["expect"], (r["ran"] or {})
            print(f"{r['path']:52} {e['shape']:>6} {e['models']:>4} {e['facts']:>6} "
                  f"{r['outcome']:>11} {str(ran.get('verdict')):>13} "
                  f"{str(ran.get('k')):>3} {str(ran.get('exhausted'))[:1]:>4} "
                  f"{str(ran.get('layers')):>4} {ran.get('ms', 0.0):>7.2f}")
        print()
    for word in ("held", "failed", "not-checked", "error"):
        sub = [r for r in claims if r["outcome"] == word]
        note = ""
        if word == "not-checked":
            note = ("  ← the column the phase is about, and it is empty: "
                    "no claim in the corpus is unverifiable"
                    if not sub else "")
        print(f"  {word:12} {len(sub):>4}{note}")
    unexhausted = [r for r in claims if not (r["ran"] or {}).get("exhausted", True)]
    print(f"\n  claims checked under a search that exhausted: "
          f"{len(claims) - len(unexhausted)} of {len(claims)}")


# ── the sweep: what a claim would cost ──────────────────────

def declared_runs(entry: dict) -> list[str]:
    return [r.split()[0] for r in entry.get("runs", [])]


def head_of(fact: str) -> str | None:
    """The relation a rendered fact is *about* — `r` in `(r a b)`.

    Positives only: a stored negative is not closed, so it costs a line only if
    a test chooses to pin one, and a counterfactual must not charge for it.
    """
    s = fact.strip()
    if not s.startswith("(") or s.startswith("(not "):
        return None
    s = s[1:]
    cut = min((i for i in (s.find(" "), s.find(")"), s.find("(")) if i >= 0),
              default=len(s))
    name = s[:cut].strip()
    return name or None


def run_once(path: str, cap: int | None, args, out: Path):
    """One `solve -e` at `ein test`'s settings, or `None` on the budget."""
    if out.exists():
        out.unlink()
    argv = [str(args.bin), "solve", path, "-e", "--json-summary", str(out)]
    if cap is not None:
        argv += ["-m", str(cap)]
    t0 = time.perf_counter()
    try:
        subprocess.run(argv, cwd=REPO, capture_output=True, timeout=args.timeout)
    except subprocess.TimeoutExpired:
        return None
    wall = time.perf_counter() - t0
    if not out.exists():
        return {}
    d = json.loads(out.read_text(encoding="utf-8"))
    d["_wall"], d["_cap"] = wall, cap
    return d


def measure(path: str, args, out: Path) -> dict:
    """`ein test`'s regime first; the ladder only if it does not fit.

    The first attempt is the one table 4 reads, because it is the run the
    runner would perform. A row that needed the ladder has already answered
    table 4's question — *it does not finish* — and what the ladder buys is
    table 3's: the models, so the write cost is a number rather than a dash.
    """
    d = run_once(path, None, args, out)
    if d is not None:
        return {"path": path, "summary": d, "reached": True}
    for cap in CAPS_DOWN:
        d = run_once(path, cap, args, out)
        if d is not None:
            return {"path": path, "summary": d, "reached": False}
    return {"path": path, "note": f"no cap fits {args.timeout:.0f}s", "reached": False}


def sweep(entries: list[dict], args) -> list[dict]:
    rows = []
    with tempfile.TemporaryDirectory(prefix="closure-") as td:
        out = Path(td) / "summary.json"
        todo = [e for e in entries if not args.key or args.key in e["path"]]
        for i, e in enumerate(todo, 1):
            # A run the manifest does not declare is not run here either —
            # `openness_census.py`'s rule, and the four entries it is about
            # (`features/04_open` and the three `square-unique` demos) end in
            # the OOM killer rather than in a verdict.
            if "solve" not in declared_runs(e):
                row = {"path": e["path"], "note": "no solve run declared"}
            else:
                row = measure(e["path"], args, out)
            row["group"] = e.get("group", "")
            rows.append(row)
            if not args.quiet:
                print(f"  [{i}/{len(todo)}] {e['path']}", file=sys.stderr)
    return rows


def write_cost(row: dict, goal_relations: list[str]) -> dict | None:
    """`goal relations × models × facts`, from the models themselves.

    Both `verdict.solutions` and `verdict.open_states`, because
    `ein_infer::expect::check` compares against both: an open state is a state
    the run reached, and all three `:expect` forms are assertions about facts.
    """
    s = row.get("summary") or {}
    v = s.get("verdict") or {}
    states = list(v.get("solutions") or []) + list(v.get("open_states") or [])
    if not states or not goal_relations:
        return None
    want = set(goal_relations)
    # A summary's state is `{facts: [...], goal_bindings: [...]}`; the bare list
    # is accepted too, so a reader of an older summary still gets a number.
    per = [sum(1 for f in (st.get("facts", []) if isinstance(st, dict) else st)
               if head_of(f) in want)
           for st in states]
    facts = sum(per)
    return {
        "models": len(states),
        "facts_per_model": per[0] if len(set(per)) == 1 else None,
        "facts": facts,
        # One line per fact, one `(model` per disjunct, one `:expect (or` to
        # open. A **convention**, and marked as one in the census: the corpus's
        # single `(or …)` packs its two facts per line because two fit, and at
        # fifteen facts per model nothing does. `facts` is the measurement.
        "lines": facts + len(states) + 1,
        "exhausted": bool(v.get("exhausted")),
        "wall_s": s.get("_wall"),
        "cap": s.get("_cap"),
    }


def print_counterfactual(rows: list[dict], goals: dict[str, list[str]], args) -> list[dict]:
    """Table 3 — what writing the claim would cost, on entries that have none."""
    print("\n## 3. The counterfactual: what the claim would cost to write\n")
    out = []
    for r in rows:
        cost = write_cost(r, goals.get(r["path"], []))
        if not cost:
            continue
        src = REPO / r["path"]
        cost["file_lines"] = len(src.read_text(encoding="utf-8").splitlines()) if src.exists() else 0
        cost["path"] = r["path"]
        cost["goal_relations"] = goals.get(r["path"], [])
        cost["reached"] = r.get("reached", False)
        # **A lower bound whenever the search did not exhaust**, and not only
        # when the ladder was needed: a capped model set is a subset, so the
        # facts it would take to list it are a floor. S1d.3.3's rule about what
        # a count may claim, applied to what a claim would cost.
        cost["floor"] = not cost["exhausted"]
        out.append(cost)
    multi = [c for c in out if c["models"] >= 2]
    print(f"{'entry':54} {'k':>4} {'f/m':>4} {'facts':>6} {'lines':>6} "
          f"{'file':>6} {'×file':>6} {'exh':>4}  goal relations")
    print(f"{'-'*54} {'-'*4} {'-'*4} {'-'*6} {'-'*6} {'-'*6} {'-'*6} {'-'*4}  {'-'*30}")
    for c in sorted(multi, key=lambda c: -c["facts"]):
        ratio = c["lines"] / c["file_lines"] if c["file_lines"] else 0.0
        mark = "  (a floor — the search did not exhaust)" if c["floor"] else ""
        print(f"{c['path']:54} {c['models']:>4} {str(c['facts_per_model'] or '-'):>4} "
              f"{c['facts']:>6} {c['lines']:>6} {c['file_lines']:>6} {ratio:>6.2f} "
              f"{str(c['exhausted'])[:1]:>4}  {' '.join(c['goal_relations'])}{mark}")
    singles = [c for c in out if c["models"] == 1]
    print(f"\n  {len(multi)} entries with a model set; {len(singles)} with a unique "
          f"model, whose claim is a `(model …)` and costs "
          f"{sum(c['facts'] for c in singles) / max(1, len(singles)):.1f} facts on average.")
    return out


def print_formula_check(rows_report: list[dict], rows_solve: list[dict],
                        goals: dict[str, list[str]]) -> dict:
    """The write-cost arithmetic, checked against the claims that exist.

    `facts = Σ_models |{positive facts of the goal's relations}|` is applied in
    table 3 to entries with no claim. Here it is applied to the 38 `(model …)`
    claims that have one, and compared with what the file lists. Predicted ≤
    listed, always: a claim may name relations beyond the goal's, and may pin a
    negative, and both add lines the formula does not charge for.
    """
    by_path = {r["path"]: r for r in rows_solve}
    exact, under, over, missing = 0, 0, [], 0
    for r in rows_report:
        e = r["expect"]
        if not e or e["shape"] != "model":
            continue
        s = by_path.get(r["path"])
        if not s or not s.get("summary"):
            missing += 1
            continue
        cost = write_cost(s, goals.get(r["path"], []))
        if not cost:
            missing += 1
            continue
        listed = e["facts"] - e["negated"]
        if cost["facts"] == listed:
            exact += 1
        elif cost["facts"] < listed:
            under += 1
        else:
            over.append((r["path"], cost["facts"], listed))
    print("\n## 3b. The formula, against the 38 claims that exist\n")
    print(f"  predicted == listed positives   {exact}")
    print(f"  predicted <  listed positives   {under}   (the claim names more "
          f"relations than the goal does)")
    print(f"  predicted >  listed positives   {len(over)}"
          f"{'   ← the formula over-charges' if over else ''}")
    for p, got, want in over[:10]:
        print(f"      {p}: predicted {got}, listed {want}")
    if missing:
        print(f"  not comparable                  {missing}   (no summary)")
    return {"exact": exact, "under": under, "over": len(over), "missing": missing}


def print_not_checked(rows: list[dict]) -> dict:
    """Table 4 — where a closure claim *would* come back `NOT CHECKED`.

    Table 2's empty column is not evidence that every claim is checkable. It is
    evidence that the entries whose claim would not be have never written one,
    and this is that set.
    """
    print("\n## 4. The counterfactual `NOT CHECKED` set\n")
    reached, capped, undeclared, budget, unloadable = [], [], [], [], []
    for r in rows:
        note = r.get("note")
        if note == "no solve run declared":
            undeclared.append(r)
            continue
        if note:
            budget.append(r)
            continue
        s = r.get("summary") or {}
        exhausted = (s.get("verdict") or {}).get("exhausted")
        if not s:
            unloadable.append(r)
        elif exhausted:
            # **Exhaustion at a shallower cap implies it at a deeper one** — the
            # frontier was empty, which is the lattice ending and not the
            # budget — so a ladder row that exhausted answers the regime's
            # question too.
            reached.append(r)
        elif r.get("reached"):
            capped.append(r)
        else:
            # A ladder row that did **not** exhaust says nothing about the
            # regime: `exhausted = false` at `-m 3` is consistent with `true` at
            # `-m 5`. What is measured here is the census's own timeout, so the
            # entry is reported as unmeasured rather than counted.
            budget.append(r)
    print(f"  exhausted = true    {len(reached):>4}   a closure claim here is checkable")
    print(f"  exhausted = false   {len(capped):>4}   a closure claim here comes back "
          f"NOT CHECKED")
    print(f"  over the budget     {len(budget):>4}   the census stopped, not the runner — "
          f"unmeasured here")
    print(f"  no declared solve   {len(undeclared):>4}   the manifest declines the run — "
          f"an open hypothesis space")
    print(f"  no fixpoint         {len(unloadable):>4}   a load error, by design")
    if capped:
        print("\n  the entries whose claim would not be checked:\n")
        print(f"{'entry':56} {'verdict':>13} {'k':>4} {'wall':>7}")
        for r in sorted(capped, key=lambda r: r["path"]):
            v = (r["summary"].get("verdict") or {})
            print(f"{r['path']:56} {str(v.get('type')):>13} {str(v.get('k')):>4} "
                  f"{r['summary'].get('_wall', 0.0):>7.2f}")
    if budget:
        print(f"\n  over the census's {'budget'} — what the runner would report is not "
              f"measured here:\n")
        print(f"{'entry':56} {'cap':>5} {'verdict':>13} {'k':>4} {'wall':>7}")
        for r in sorted(budget, key=lambda r: r["path"]):
            s = r.get("summary") or {}
            v = (s.get("verdict") or {})
            print(f"{r['path']:56} {str(s.get('_cap') or '-'):>5} "
                  f"{str(v.get('type')):>13} {str(v.get('k')):>4} "
                  f"{s.get('_wall', 0.0):>7.2f}")
    return {
        "exhausted": len(reached),
        "capped": len(capped),
        "undeclared": len(undeclared),
        "unloadable": len(unloadable),
        "budget": len(budget),
        "not_checked_entries": [r["path"] for r in sorted(capped, key=lambda r: r["path"])],
        "over_budget_entries": [r["path"] for r in sorted(budget, key=lambda r: r["path"])],
    }


def main() -> int:
    ap = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    ap.add_argument("--bin", default=EIN, type=Path,
                    help=f"the ein binary (default $EIN_BIN or {EIN})")
    ap.add_argument("--roots", nargs="*", default=list(ROOTS),
                    help="the directories `ein test` is pointed at")
    ap.add_argument("--json", type=Path, help="also write the rows as JSON")
    ap.add_argument("-k", "--key", help="only entries whose path contains this")
    ap.add_argument("--no-solve", action="store_true",
                    help="tables 1-2 only — the report, with no corpus sweep")
    ap.add_argument("--long", action="store_true",
                    help="table 2 as one row per claim, not just the roll-up")
    ap.add_argument("--timeout", type=float, default=60.0,
                    help="seconds per entry (default 60)")
    ap.add_argument("-q", "--quiet", action="store_true", help="no progress lines")
    args = ap.parse_args()

    if not Path(args.bin).exists():
        print(f"no engine at {args.bin} — run ./build.sh, "
              f"or name one with --bin / $EIN_BIN", file=sys.stderr)
        return 2

    doc = report(args)
    if not doc:
        return 2
    rows = doc["rows"]
    print(f"# The closure census — {doc['schema']}, {len(rows)} rows")
    print_usage(rows)
    print_verifiable(rows, args)

    banked = {"report": doc}
    if not args.no_solve:
        goals = {r["path"]: r["goal_relations"] for r in rows if r["query"] >= 1}
        entries = tomllib.loads(MANIFEST.read_text(encoding="utf-8"))["entry"]
        solved = sweep(entries, args)
        costs = print_counterfactual(solved, goals, args)
        banked["formula"] = print_formula_check(rows, solved, goals)
        banked["not_checked"] = print_not_checked(solved)
        banked["costs"] = costs
        # The reduced sweep — everything the tables read, and none of the
        # models, which on the zebra family are 435 facts x 32.
        banked["solve"] = [
            {
                "path": r["path"],
                "group": r.get("group", ""),
                "note": r.get("note"),
                "verdict": ((r.get("summary") or {}).get("verdict") or {}).get("type"),
                "k": ((r.get("summary") or {}).get("verdict") or {}).get("k"),
                "exhausted": ((r.get("summary") or {}).get("verdict") or {}).get("exhausted"),
                "cap": (r.get("summary") or {}).get("_cap"),
                "wall_s": (r.get("summary") or {}).get("_wall"),
            }
            for r in solved
        ]

    if args.json:
        args.json.write_text(json.dumps(banked, indent=1), encoding="utf-8")
        print(f"\nwrote {args.json}", file=sys.stderr)
    return 0


if __name__ == "__main__":
    sys.exit(main())
