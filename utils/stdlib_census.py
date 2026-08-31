#!/usr/bin/env python3
"""What the stdlib declares, and what the corpus actually activates — S1c.1.1's instrument.

Every check this repo has is *relative*: the goldens compare ein.rs to its own
past, and after
[P1a.10](../docs/history/m1a_rust/README.md#p1a10--one-implementation) there is
no second engine to compare it to at all. The stdlib is where that gap is
widest — `std.algebra`, `std.bijection`, `std.elim`, `std.closure`,
`std.slots`, `std.typing` and `std.macro` are exercised only as a side effect
of whatever the zebra corpus happens to need, and **a rule no corpus entry
activates is not tested, it is merely not contradicted**.

This counts which is which.

    utils/stdlib_census.py                        # the table, to stdout
    utils/stdlib_census.py --json census.json     # + the machine copy
    utils/stdlib_census.py -k zebra2              # one entry's contribution
    utils/stdlib_census.py --level normal         # what the elision hides

Two halves, and only the second runs the engine:

1. **The declaration inventory.** `stdlib/*.ein` parsed for `(rule …)` heads:
   module, parameters, priority, whether the head asserts `(false)`, and the
   *guard shapes* in its `:match` — every `neq`, `absent`, `not` and `forall`,
   because each one is a case where firing would be wrong and so is a negative
   test S1c.1.4 owes. Seven modules, and three of the names are declared twice
   (`std.elim` and `std.bijection` both ship `domain-elimination` and
   `typecheck-arg-{0,1}` — the positional and the signature-driven
   formulations). How many rules is what this half **prints**; it read
   "seventy-three" until M1e `DO-M1`, four behind.
2. **The firing census.** Every corpus entry, under every declared run that
   reaches the engine (`solve …` / `saturate …` / `test` — `render` has no
   `--events`, and `render lattice`'s solve is a subset of `solve -e`'s), with
   `--events`. `fire` events counted by rule, split productive vs redundant
   — plus `owe`, which is the same claim for the rules that assert the verdict
   atom `open` and therefore never reach the firing stream (M1d S1d.2.4);
   `load` events read for which rules were *available* to a file at all, which
   is what separates "imported and never activated" from "no corpus entry
   imports it".

**Run it at `verbose`, which is the default here.** At `normal` a redundant
firing is counted but not emitted
([events.md § Levels](../docs/kernel/inference/events.md)), so a rule whose
every firing re-derives an existing fact reads as **zero** — the trap
[S1a.7.0](../docs/history/m1a_rust/README.md#s1a70--the-speculation-audit)'s
audit hit. `--level normal` is there to show the difference, not to be used.

**A fired name is not a stdlib rule just because the stdlib has that name.**
Twenty-five files under `examples/` declare their own `symmetric`,
`transitive`, `functional`, `injective`, `total`, `surjective` or
`domain-elimination` — the inline copies the stdlib README says are kept
deliberately, plus `zebra2-hints.ein`, which *imports* three `std.algebra`
symbols and declares five more of its own names beside them. Crediting those
firings to the stdlib is how a census reports coverage it does not have, so a
firing counts for a module only when the file **does not declare that name
itself** and the module is in its import closure.

**And the two formulations are separated, not summed.** A `fire` event carries
its rule's `activator` — the parameter tuple — so the two
`domain-elimination`s split on arity (4 for `std.elim`'s positional form, 2 for
`std.bijection`'s signature-driven one). The two `typecheck-arg-0`s do not:
both take `(?R ?isa ?Dom)`. Those split on the import closure, computed
statically from the file's `(import std.… )` forms plus the modules' own
self-imports — and a file importing both cannot load, since a same-name
differing body is a conflict. A residue is reported as `ambiguous` rather than
silently attributed.

Argv follows `ein-corpus/src/plan.rs`, mirrored the way
[`corpus_cost.py`](corpus_cost.py) mirrors it, and for the same reason: a
`cargo test` in the middle of a sweep is a worse dependency than six lines.
"""
from __future__ import annotations

import argparse
import collections
import json
import os
import shutil
import subprocess
import sys
import tempfile
import tomllib
from pathlib import Path

REPO = Path(__file__).resolve().parents[1]
MANIFEST = REPO / "corpus" / "corpus.toml"
STDLIB = REPO / "stdlib"
EIN = Path(os.environ.get("EIN_BIN", REPO / "ein.rs" / "target" / "release" / "ein"))

#: Guard forms whose presence in a `:match` is a case where firing is wrong —
#: the negative shapes T1c.1.1.4 asks for. `forall` is `std.macro`'s and
#: expands to `(absent (and G (absent B)))`, so it is a guard twice over.
GUARDS = ("neq", "absent", "not", "forall", "or", "open")


# ── the declaration inventory ───────────────────────────────

def read_forms(text: str) -> list:
    """Top-level s-expressions of an `.ein` source; `;` comments dropped.

    Atoms stay strings, including `:keywords` and `"strings"` (quotes kept, so
    a `:why` is distinguishable from a bare symbol). Enough of a reader for
    declaration heads — it is not the loader and does not need to be.
    """
    i, n, stack, out = 0, len(text), [], []
    emit = lambda x: (stack[-1] if stack else out).append(x)
    while i < n:
        c = text[i]
        if c == ";":
            j = text.find("\n", i)
            i = n if j < 0 else j + 1
        elif c.isspace():
            i += 1
        elif c == "(":
            stack.append([])
            i += 1
        elif c == ")":
            form = stack.pop() if stack else []
            emit(form)
            i += 1
        elif c == '"':
            j = i + 1
            while j < n and text[j] != '"':
                j += 2 if text[j] == "\\" else 1
            emit(text[i:j + 1])
            i = j + 1
        else:
            j = i
            while j < n and not text[j].isspace() and text[j] not in '();"':
                j += 1
            emit(text[i:j])
            i = j
    return out


def keywords(tail: list) -> dict:
    """`:key value` pairs from a declaration's tail."""
    kw, i = {}, 0
    while i + 1 < len(tail):
        if isinstance(tail[i], str) and tail[i].startswith(":"):
            kw[tail[i][1:]] = tail[i + 1]
            i += 2
        else:
            i += 1
    return kw


def guard_shapes(form) -> collections.Counter:
    """Every guard head anywhere in a `:match`, counted."""
    seen = collections.Counter()
    stack = [form]
    while stack:
        node = stack.pop()
        if isinstance(node, list):
            if node and isinstance(node[0], str) and node[0] in GUARDS:
                seen[node[0]] += 1
            stack.extend(node)
    return seen


def sexpr(form) -> str:
    if isinstance(form, str):
        return form
    return "(" + " ".join(sexpr(x) for x in form) + ")"


def alpha(form, seen: dict[str, str]) -> str:
    """A rule body with its variables renamed by first appearance.

    Two rules with the same `alpha` of `(params, :match, :assert)` are the same
    rule under two names — `includes` and `imply2-fwd`, `imply2-reverse` and
    `converse`, which the module comments call twins and aliases. It matters
    for [S1c.1.4](../docs/history/m1c_external_validation/README.md#s1c14--the-stdlib-corpus):
    an expectation made of facts cannot tell two identical bodies apart, so
    they are one test plus whatever says they were reached differently — the
    `route` residue Q-M1c.2 parks.
    """
    if isinstance(form, str):
        if form.startswith("?"):
            return seen.setdefault(form, f"?v{len(seen)}")
        return form
    return "(" + " ".join(alpha(x, seen) for x in form) + ")"


def body_key(rule_form: list) -> str:
    kw = keywords(rule_form[3:])
    seen: dict[str, str] = {}
    return alpha([rule_form[2], kw.get("match", []), kw.get("assert", [])], seen)


def inventory() -> tuple[list[dict], dict[str, list[str]]]:
    """Every `(rule …)` in `stdlib/`, and each module's own imports."""
    rules, imports = [], {}
    for path in sorted(STDLIB.glob("*.ein")):
        module = f"std.{path.stem}"
        imports[module] = []
        for form in read_forms(path.read_text(encoding="utf-8")):
            if not (isinstance(form, list) and form and isinstance(form[0], str)):
                continue
            if form[0] == "import":
                imports[module].append(form[1])
            elif form[0] == "rule":
                kw = keywords(form[3:])
                match = kw.get("match", [])
                rules.append({
                    "rule": form[1],
                    "module": module,
                    "params": [p for p in form[2]] if isinstance(form[2], list) else [],
                    "priority": int(kw["priority"]) if "priority" in kw else None,
                    "asserts": sexpr(kw.get("assert", "")),
                    "refutes": sexpr(kw.get("assert", "")) == "(false)",
                    "guards": dict(guard_shapes(match)),
                    "why": kw.get("why", "").strip('"'),
                    "body": body_key(form),
                })
    return rules, imports


def file_context(path: Path, imports: dict[str, list[str]]) -> tuple[set[str], set[str]]:
    """What a corpus file can see, and what it declares for itself.

    Returns (import closure over `std.*`, the rule names the file declares).
    The second is what stops `examples/branching/05_mini_zebra.ein`'s inline
    `symmetric` from being counted as `std.algebra`'s.
    """
    try:
        forms = read_forms(path.read_text(encoding="utf-8"))
    except OSError:
        return set(), set()
    todo, local = [], set()
    for f in forms:
        if not (isinstance(f, list) and len(f) > 1 and isinstance(f[0], str)):
            continue
        if f[0] == "import" and isinstance(f[1], str):
            todo.append(f[1])
        elif f[0] in ("rule", "hrule") and isinstance(f[1], str):
            local.add(f[1])
    seen: set[str] = set()
    while todo:
        module = todo.pop()
        if module in seen or module not in imports:
            continue
        seen.add(module)
        todo.extend(imports[module])
    return seen, local


# ── the firing census ───────────────────────────────────────

def argv_for(run: str, file: str, out: Path) -> list[str]:
    """`ein-corpus::plan::argv`, mirrored — see the module docstring."""
    toks = [t.replace("{out}", str(out)) for t in run.split()]
    if toks[0] == "render":
        return [toks[0], *toks[1:2], file, *toks[2:]]
    return [toks[0], file, *toks[1:]]


def all_runs(entry: dict) -> list[str]:
    return [*entry.get("runs", []), *(f"solve {lv}" for lv in entry.get("levers", []))]


#: Subcommands that reach the engine and accept `--events`. `render` is
#: excluded on both counts: `render rules` / `render constraints` never
#: saturate, and `render lattice` runs a solve the entry's own `solve -e`
#: covers — with no `--events` flag to record it either way.
#:
#: `test` joined the pair at M1c S1c.1.4, when `tests/stdlib/` arrived: a
#: fixture that declared only `test` would otherwise be invisible to the
#: census, which is the one thing this instrument must not be. Adding it moves
#: no cell on the corpus as it stood — the three entries that declared `test`
#: before that stage reach only `symmetric`, which eight others already do —
#: so the before and after stay comparable by construction.
ENGINE_SUBCOMMANDS = ("solve", "saturate", "test")


def inference_runs(entry: dict) -> list[str]:
    """The declared runs that reach the engine and take `--events`.

    A `test` run on a file carrying more than one `:expect` is refused (an
    artefact flag names one path), and the sweep below then records no events
    for that cell — which is why every `tests/stdlib/` entry also declares
    `saturate`.
    """
    seen, out = set(), []
    for run in all_runs(entry):
        if run.split()[0] in ENGINE_SUBCOMMANDS and run not in seen:
            seen.add(run)
            out.append(run)
    return out


def sweep(entries: list[dict], args, imports: dict[str, list[str]],
          by_name: dict[str, list[dict]]) -> dict:
    """Run the corpus under `--events` and tally activations by rule.

    `fire` for a saturation rule, `owe` for an obligation one — see the branch
    below. An `owe` is never redundant, so it lands in the productive column.
    """
    env = dict(os.environ)
    env.pop("EIN_STDLIB", None)          # the checkout's stdlib is the subject
    env["LC_ALL"] = "C"
    tally: dict[tuple[str, str], dict] = {}
    loaded: dict[tuple[str, str], set[str]] = collections.defaultdict(set)
    ambiguous = collections.Counter()
    failures: list[str] = []
    root = Path(tempfile.mkdtemp(prefix="ein-stdlib-census-"))
    try:
        for i, entry in enumerate(entries):
            path = entry["path"]
            ctx = file_context(REPO / path, imports)
            for run in inference_runs(entry):
                out = root / f"{i:04d}"
                out.mkdir(parents=True, exist_ok=True)
                events = out / "events.jsonl"
                argv = [str(args.bin), *argv_for(run, path, out),
                        "--events", str(events), "--events-level", args.level]
                proc = subprocess.run(argv, cwd=REPO, env=env, timeout=args.timeout,
                                      stdin=subprocess.DEVNULL,
                                      stdout=subprocess.DEVNULL,
                                      stderr=subprocess.PIPE)
                if not events.exists():
                    # A `broken/` fixture that never reaches the engine: that is
                    # the entry's point, not a failure of the sweep.
                    if proc.returncode == 0:
                        failures.append(f"{path} [{run}]: exit 0 and no event file")
                    continue
                with events.open(encoding="utf-8") as fh:
                    for line in fh:
                        ev = json.loads(line)
                        kind = ev["e"]
                        if kind == "load":
                            for name in ev.get("rule_names", []):
                                for r in resolve(name, None, ctx, by_name):
                                    loaded[(r["module"], r["rule"])].add(path)
                        elif kind in ("fire", "owe"):
                            # `owe` is the obligation half's activation
                            # evidence — M1d S1d.2.4. A rule whose `:assert` is
                            # the verdict atom `open` derives nothing and is
                            # kept out of the saturation agenda, so it can
                            # never emit a `fire`; the post-fixpoint pass emits
                            # one `owe` per undischarged instance instead, with
                            # the same `rule` / `activator` fields. Counting
                            # only `fire` would have put every obligation rule
                            # permanently in the zero set.
                            cands = resolve(ev["rule"], ev.get("activator"),
                                            ctx, by_name)
                            if not cands:
                                continue        # a rule the puzzle declares itself
                            if len(cands) > 1:
                                ambiguous[ev["rule"]] += 1
                            for r in cands:
                                key = (r["module"], r["rule"])
                                t = tally.setdefault(key, {
                                    "productive": 0, "redundant": 0,
                                    "entries": set(), "runs": set()})
                                t["redundant" if ev.get("redundant") else
                                  "productive"] += 1
                                t["entries"].add(path)
                                t["runs"].add(f"{path} [{run}]")
                shutil.rmtree(out, ignore_errors=True)
    finally:
        shutil.rmtree(root, ignore_errors=True)
    return {"tally": tally, "loaded": loaded,
            "ambiguous": ambiguous, "failures": failures}


def resolve(name: str, activator, ctx: tuple[set[str], set[str]],
            by_name: dict[str, list[dict]]) -> list[dict]:
    """Which stdlib declaration a fired rule name refers to, if any.

    A local declaration wins outright: a file that declares `symmetric` fired
    *its* `symmetric`, whatever the stdlib also calls that. Then the import
    closure — a module the file never pulled in cannot have fired. Then arity,
    which is what separates `std.elim`'s four-parameter `domain-elimination`
    from `std.bijection`'s two-parameter one.
    """
    closure, local = ctx
    if name in local:
        return []
    cands = [r for r in by_name.get(name, []) if r["module"] in closure]
    if len(cands) > 1 and activator is not None:
        narrowed = [r for r in cands if len(r["params"]) == len(activator)]
        cands = narrowed or cands
    return cands


# ── the report ──────────────────────────────────────────────

def report(rules: list[dict], census: dict, args) -> int:
    tally, loaded = census["tally"], census["loaded"]
    w = max(len(r["rule"]) for r in rules) + 2
    print(f"{'rule':<{w}}{'module':<16}{'pri':>5}{'productive':>12}"
          f"{'redundant':>11}{'entries':>9}  guards")
    print("─" * (w + 66))
    zero, only_redundant, single = [], [], []
    for r in sorted(rules, key=lambda r: (r["module"], r["rule"])):
        key = (r["module"], r["rule"])
        t = tally.get(key)
        prod = t["productive"] if t else 0
        red = t["redundant"] if t else 0
        n = len(t["entries"]) if t else 0
        guards = " ".join(f"{k}×{v}" for k, v in sorted(r["guards"].items()))
        flag = ""
        if prod + red == 0:
            zero.append(r)
            flag = "  ← never fires"
        else:
            if prod == 0:
                only_redundant.append(r)
                flag = "  ← only redundant"
            if n == 1:
                single.append((r, sorted(t["entries"])[0]))
        print(f"{r['rule']:<{w}}{r['module']:<16}"
              f"{r['priority'] if r['priority'] is not None else '—':>5}"
              f"{prod:>12}{red:>11}{n:>9}  {guards}{flag}")

    print(f"\n{len(rules)} rules over "
          f"{len({r['module'] for r in rules})} modules; "
          f"{len(rules) - len(zero)} activated, {len(zero)} never activate.")

    print(f"\n## The zero-firing set — {len(zero)} rules")
    for r in zero:
        seen = sorted(loaded.get((r["module"], r["rule"]), ()))
        where = (f"loaded by {len(seen)} entr{'y' if len(seen) == 1 else 'ies'}"
                 if seen else "no corpus entry loads it")
        print(f"  {r['module']:<16}{r['rule']:<{w}}{where}")

    print(f"\n## Activated by exactly one entry — {len(single)} rules")
    for r, path in single:
        print(f"  {r['module']:<16}{r['rule']:<{w}}{path}")

    if only_redundant:
        print(f"\n## Every firing redundant — {len(only_redundant)} rules")
        for r in only_redundant:
            print(f"  {r['module']:<16}{r['rule']}")

    if census["ambiguous"]:
        print("\n## Unresolved names (attributed to every candidate)")
        for name, n in census["ambiguous"].most_common():
            print(f"  {name}: {n} firings")
    if census["failures"]:
        print("\n## Sweep failures")
        for line in census["failures"]:
            print(f"  ✗ {line}")

    sole = collections.Counter(path for _, path in single)
    if sole:
        print("\n## Sole activator — entries a rule's only coverage depends on")
        for path, n in sole.most_common():
            print(f"  {path:<48}{n:>3} rule{'s' if n != 1 else ''}")

    aliases = collections.defaultdict(list)
    for r in rules:
        aliases[r["body"]].append(f"{r['module']}/{r['rule']}")
    twins = [names for names in aliases.values() if len(names) > 1]
    if twins:
        print(f"\n## Same body under two names — {len(twins)} groups")
        for names in sorted(twins):
            print("  " + " ≡ ".join(sorted(names)))

    by_module = collections.Counter()
    covered = collections.Counter()
    for r in rules:
        by_module[r["module"]] += 1
        if tally.get((r["module"], r["rule"])):
            covered[r["module"]] += 1
    print(f"\n{'module':<16}{'rules':>7}{'covered':>9}{'zero':>6}")
    print("─" * 38)
    for module in sorted(by_module):
        print(f"{module:<16}{by_module[module]:>7}{covered[module]:>9}"
              f"{by_module[module] - covered[module]:>6}")

    if args.check:
        # Sweep failures count — M1e S1e.4.5, `TE-L4`. A cell that exits 0 and
        # narrates nothing has not been censused, and a `--check` that walked
        # past one would be green over a *partial* sweep whose surviving runs
        # happened to cover all 77 rules. (A *total* failure already exits 1
        # through `zero`.) Empty on 2026-09-01 over 217 entries and 663 runs,
        # so this changes no result today; it is what makes the nightly step
        # able to fail for the reason the sweep most plausibly breaks.
        return 1 if zero or census["failures"] else 0
    return 0


def main() -> int:
    ap = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    ap.add_argument("--bin", type=Path, default=EIN, help="the ein binary ($EIN_BIN)")
    ap.add_argument("--level", default="verbose", choices=("normal", "verbose"),
                    help="event level (default verbose — `normal` elides "
                         "redundant firings, which reads as zero)")
    ap.add_argument("-k", "--only", default=None, metavar="SUBSTR",
                    help="only corpus entries whose path contains SUBSTR")
    ap.add_argument("--timeout", type=float, default=300.0, metavar="SEC")
    ap.add_argument("--json", type=Path, default=None, metavar="FILE")
    ap.add_argument("--check", action="store_true",
                    help="exit 1 if any stdlib rule is never activated, or if "
                         "any declared run exited 0 and narrated nothing")
    args = ap.parse_args()

    if not args.bin.exists():
        sys.exit(f"{args.bin} does not exist — cargo build --release -p ein-cli")
    rules, imports = inventory()
    by_name: dict[str, list[dict]] = collections.defaultdict(list)
    for r in rules:
        by_name[r["rule"]].append(r)

    manifest = tomllib.loads(MANIFEST.read_text(encoding="utf-8"))
    entries = [e for e in manifest["entry"]
               if not args.only or args.only in e["path"]]
    if not entries:
        sys.exit("no entries selected")
    census = sweep(entries, args, imports, by_name)
    rc = report(rules, census, args)

    if args.json:
        args.json.write_text(json.dumps({
            "level": args.level, "bin": str(args.bin),
            "entries": len(entries),
            "rules": [{**r, **{
                "productive": census["tally"].get((r["module"], r["rule"]), {})
                              .get("productive", 0),
                "redundant": census["tally"].get((r["module"], r["rule"]), {})
                             .get("redundant", 0),
                "entries": sorted(census["tally"].get((r["module"], r["rule"]), {})
                                  .get("entries", ())),
                "loaded_by": sorted(census["loaded"].get((r["module"], r["rule"]), ())),
            }} for r in rules],
        }, indent=2) + "\n", encoding="utf-8")
        print(f"\nartifact: {args.json}", file=sys.stderr)
    return rc


if __name__ == "__main__":
    raise SystemExit(main())
