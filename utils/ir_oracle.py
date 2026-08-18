#!/usr/bin/env python3
"""ein.py's IR frontend as a batch oracle — the parity gate for M1a P1a.1.

`ein.rs`'s lexer, parser, dumper, macro expander and import resolver are
checked against *this* process, because ein.py is the oracle: any observable
difference is a bug in the port (M1a invariant I1). The conformance harness
cannot do it — it compares two `ein` CLIs, and the frontend has no CLI surface
of its own (the `ir` inspector subcommands were removed in P1.11).

Deliberately a **batch** protocol over stdin/stdout rather than one process per
file: building the Lark grammar costs ~0.5 s, and the differential fuzzer sends
10⁶ inputs.

    $ python3 utils/ir_oracle.py < requests.jsonl > responses.jsonl

One JSON object per line in, one per line out, in order:

    {"op": "accept",   "text": "(a b)"}          → {"ok": true}
    {"op": "parse",    "path": "examples/x.ein"} → {"ok": true, "out": "<dump_canonical>"}
    {"op": "compact",  "path": …}                → one `dump_compact` per form, newline-joined
    {"op": "resolve",  "path": …}                → dump_canonical(resolve_imports(…))
    {"op": "minimize", "path": …}                → dump_canonical(resolve_and_minimize(…))
    {"op": "expand",   "path": …}                → resolve, then expand rule clauses
    {"op": "macro-names"}                        → the names `std.macro` exports, sorted
    {"op": "kb-shape", "path": …}                → the loaded KB's shape (below)
    {"op": "plan-shape", "path": …}              → every compiled JoinPlan (below)
    {"op": "plan-shape", "path": …, "filter": false}  → … without the arity filter
    {"op": "match-shape", "path": …}             → every match every plan produces
    {"op": "saturate-events", "path": …}         → the `--events` log of a root saturation

`kb-shape` runs the **loader** and renders the resulting `KnowledgeBase` as one
deterministic text: the registries in insertion order, the fact list in ingest
order, and each of the seven reverse indexes in a sorted order of its own. It
is what M1a P1a.2 diffs, because a KB has no CLI surface and `ein-conformance`
therefore cannot see any of it. Facts are named by **position** in the fact
list, so the first thing a diff can report is a fact-order difference; values
are rendered with `repr`, so the integer `7` and the atom `7` cannot collide;
and `names` is emitted sorted because ein.py builds that dict over a *set*
union, whose order is not reproducible even run to run. A `KBLoadError` comes
back as the ordinary `{"ok": false, …}`, which is how the accumulated-message
parity is compared on the whole corpus rather than only on the fixtures.

`plan-shape` is the same idea one layer down, for M1a P1a.3: it walks
`kb.rules` x `Engine._activators_for` — the order `compile_all` builds the cache
in, which `_enqueue_pass` then iterates and so is itself observable — and
renders each `JoinPlan`. Slot values go out with `repr`, so the atom `7` and the
integer `7` cannot collide; `watched`, `scope` and a `Join`'s shared variables
go out **sorted**, because they are `frozenset`s whose order is not reproducible
run to run. Registers and probes have no counterpart here and are deliberately
absent: they are the port's own metadata. A `CompileError` comes back as the
ordinary `{"ok": false, …}`, which is how the four S1.22.0 error messages are
compared on the corpus rather than only on fixtures.

`match-shape` is the layer below that, for S1a.3.2: it runs every plan over the
loaded KB — a full run, and a `run_seeded` at every fact — and emits one line
per match. Bindings go out in **dict order**, which is the order the matcher
first bound each variable and therefore the order `Provenance.bindings` records;
premises go out as fact *positions*, so an order or identity difference names
itself. Between them the two sweeps pin the three orders a matcher owes:
matches, bindings, and premises through the semi-naive seed.

`expand` is the one op with no single function behind it: `ein.ir.macros`
provides `expand_macros` / `_substitute`, and the *loader* decides what to run
them over (each rule's `:match` and `:assert`, and nothing else — a
`(forall …)` fact is left alone). The scaffolding below reproduces that
decision so both implementations expand the same nodes; the loader's own
checks around it — a duplicate macro name, one that shadows kernel vocabulary,
the S1.8a.f20 unimported-macro guard — belong to `kb.from_ir` and are compared
when the loader is ported (P1a.2).

`accept` skips the dump, which is what makes the fuzzer affordable. A failure
is `{"ok": false, "err": "<message>", "kind": "IRParseError"|"KBLoadError"|…}`
with the message ein.py would print — byte-for-byte what the port must
produce.

`path` is repo-root-relative or absolute, and is what `filename` becomes, so
`Loc`s and error messages name the same file both sides see. `text` may carry
an explicit `"filename"` (default `null` → ein.py's `<string>`).
"""
from __future__ import annotations

import json
import os
import sys
from pathlib import Path

REPO = Path(__file__).resolve().parent.parent
sys.path.insert(0, str(REPO / "ein.py" / "src"))

from ein.ir import IRParseError, parse                      # noqa: E402
from ein.ir.dump import dump_canonical, dump_compact        # noqa: E402
from ein.ir.macros import Macro, expand_macros              # noqa: E402
from ein.ir.types import Atom, Int, KwPair, SForm, Var      # noqa: E402
from ein.kb import KnowledgeBase                             # noqa: E402
from ein.kb.imports import (                                # noqa: E402
    resolve_and_minimize,
    resolve_imports,
    stdlib_macro_names,
)


def _macro_registry(forms) -> dict[str, Macro]:
    """`kb.from_ir._ingest_macros` minus its error checks — first wins."""
    out: dict[str, Macro] = {}
    for f in forms:
        if not (isinstance(f, SForm) and isinstance(f.head, Atom)
                and f.head.name == "macro" and len(f.args) >= 3):
            continue
        name, params_form, body = f.args[0], f.args[1], f.args[2]
        if not isinstance(name, Atom) or not isinstance(params_form, SForm):
            continue
        if name.name in out:
            continue
        out[name.name] = Macro(
            name=name.name,
            params=tuple(a.name for a in params_form.args if isinstance(a, Var)),
            body=body,
            loc=f.loc,
        )
    return out


def _expand_rule_clauses(forms, macros: dict[str, Macro]):
    """What the loader runs the expander over: `:match` and `:assert`, on
    `(rule …)` / `(hrule …)` forms only."""
    if not macros:
        return list(forms)
    out = []
    for f in forms:
        if not (isinstance(f, SForm) and isinstance(f.head, Atom)
                and f.head.name in ("rule", "hrule")):
            out.append(f)
            continue
        args = tuple(
            KwPair(key=a.key, value=expand_macros(a.value, macros), loc=a.loc)
            if isinstance(a, KwPair) and a.key.name in ("match", "assert") else a
            for a in f.args
        )
        out.append(SForm(head=f.head, args=args, loc=f.loc))
    return out


def _kb_shape(kb: KnowledgeBase) -> str:
    """The KB as one deterministic text — see the module docstring."""
    from dataclasses import fields

    facts = list(kb.facts)
    at = {(f.relation_name, f.args): i for i, f in enumerate(facts)}
    ids = lambda fs: ",".join(str(at.get((f.relation_name, f.args), "?")) for f in fs)  # noqa: E731
    out: list[str] = []

    for i, f in enumerate(facts):
        out.append(f"F {i} {(f.relation_name, f.args)!r}")
    for name, r in kb.relations.items():
        out.append(f"REL {name} sig=({' '.join(r.signature)}) "
                   f"declared={r.declared} why={r.why!r}")
    for kind, registry in (("RULE", kb.rules), ("HRULE", kb.hrules)):
        for name, r in registry.items():
            variables = " ".join(r.match.variables) if r.match else ""
            relations = " ".join(r.match.relation_names) if r.match else ""
            out.append(f"{kind} {name} params=({' '.join(r.params)}) "
                       f"priority={r.priority} why={r.why!r} "
                       f"vars=({variables}) rels=({relations})")
    for name, m in kb.macros.items():
        out.append(f"MACRO {name} params=({' '.join(m.params)})")
    out.append(f"QUERY {len(kb.query.kw_pairs)}" if kb.query else "QUERY None")
    if kb.config is None:
        out.append("CONFIG None")
    else:
        for field in fields(kb.config):
            value = getattr(kb.config, field.name)
            out.append(f"CONFIG {field.name.replace('_', '-')}={value!r}")

    for rel in sorted({f.relation_name for f in facts}):
        out.append(f"EXTENT {rel} {ids(kb._facts_by_relation.get(rel, ()))}")
    psi = sorted(
        (f"{rel} {slot} {value!r}", key)
        for key, (rel, slot, value) in
        ((k, k) for k in kb._facts_by_rel_slot_val)
    )
    for label, key in psi:
        out.append(f"PSI {label} {ids(kb._facts_by_rel_slot_val[key])}")
    for name in sorted(kb.names):
        ref = kb.names[name]
        out.append(f"NAME {name} {ref.category} "
                   f"head=({ids(ref.as_head)}) arg=({ids(ref.as_arg)})")
    for inner in sorted(repr(x) for x in kb._negated_facts):
        out.append(f"NEG {inner}")
    for name in kb.rules:
        apps = kb._rule_apps_by_rule.get(name, ())
        if apps:
            out.append(f"RULEAPP {name} {ids(apps)}")
    for name in kb.relations:
        apps = kb._rule_apps_on_relation.get(name, ())
        if apps:
            out.append(f"RELAPP {name} {ids(apps)}")
        rules = kb._rules_by_relation.get(name, ())
        if rules:
            out.append(f"RULESREL {name} {','.join(r.name for r in rules)}")
    for i, f in enumerate(facts):
        p = f.provenance
        if p is not None:
            if p.kind == "source":
                detail = f"source={p.source!r}" if p.source is not None else "source=None"
            elif p.kind == "rule":
                detail = (f"rule={p.rule} using="
                          f"[{', '.join(repr(x) for x in p.premises_raw)}]")
            else:
                detail = f"branch={p.branch}"
            out.append(f"PROV {i} {p.kind} {detail}")
        alts = kb._alt_justifications.get((f.relation_name, f.args), ())
        if alts:
            out.append(f"ALT {i} {len(alts)}")
    return "\n".join(out)


def _slot_text(s: object) -> str:
    """One compiled slot, in the vocabulary `ein-infer::shape` mirrors."""
    from ein.inference.compile import NestedPattern
    if isinstance(s, Var):
        return f"?{s.name}"
    if isinstance(s, Atom):
        return repr(s.name)
    if isinstance(s, Int):
        return repr(s.value)
    if isinstance(s, NestedPattern):
        inner = "".join(" " + _slot_text(x) for x in s.arg_slots)
        return f"({s.relation}{inner})"
    return repr(s)


def _steps_text(out: list[str], steps: tuple, depth: int) -> None:
    from ein.inference.compile import AbsentGuard, Guard, Join, Scan
    pad = "  " * depth
    for st in steps:
        if isinstance(st, (Scan, Join)):
            kind = "JOIN" if isinstance(st, Join) else "SCAN"
            shared = (f" shared=({' '.join(sorted(st.shared_vars))})"
                      if isinstance(st, Join) else "")
            slots = " ".join(_slot_text(s) for s in st.arg_slots)
            out.append(f"{pad}{kind} {st.relation}{shared} [{slots}]")
        elif isinstance(st, Guard):
            args = ", ".join(repr(a) for a in st.args)
            out.append(f"{pad}GUARD {st.predicate} [{args}]")
        elif isinstance(st, AbsentGuard):
            out.append(f"{pad}ABSENT {len(st.sub_steps)}")
            _steps_text(out, st.sub_steps, depth + 1)


def _plan_shape(kb: KnowledgeBase, filter_activators: bool = True) -> str:
    """Every `(rule, activator)` plan, in `compile_all` order.

    `filter_activators=False` drops `_activators_for`'s S1.22.0 **arity**
    filter and hands `compile_rule` every rule-application fact. Nothing in the
    engine does that — both drivers filter first — which is exactly why the
    arity `CompileError` is otherwise unreachable, and why the fixture for it
    needs a way around the filter.
    """
    from ein.inference.compile import (
        asserted_relation,
        compile_rule,
        naf_relation_refs,
        negated_relation,
    )
    from ein.inference.engine import Engine

    engine = Engine(kb)
    out: list[str] = []
    for rule in kb.rules.values():
        if filter_activators:
            activators = engine._activators_for(rule)
        elif not rule.params:
            activators = (None,)
        else:
            activators = tuple(kb._rule_apps_by_rule.get(rule.name, ()))
        for activator in activators:
            key = tuple(str(a) for a in (activator.args if activator else ()))
            plan = compile_rule(rule, activator)
            out.append(f"PLAN {plan.rule_name} key={key!r} "
                       f"args={plan.activator_args!r} why={plan.why!r}")
            for name, value in plan.bindings_seed.items():
                out.append(f"  SEED {name} {value!r}")
            for i, (steps, guards) in enumerate(plan.disjuncts()):
                out.append(f"  D{i} STEPS {len(steps)}")
                _steps_text(out, steps, 2)
                for j, g in enumerate(guards):
                    out.append(
                        f"  D{i} GUARD {j} scope=({' '.join(sorted(g.scope))}) "
                        f"watched=({' '.join(sorted(g.watched))}) "
                        f"monotone={g.monotone}")
                    _steps_text(out, g.sub_steps, 2)
            for i, t in enumerate(plan.assert_templates):
                out.append(f"  ASSERT {i} {_slot_text(t)}")
            out.append(f"  ASSERTED {asserted_relation(plan)!r} "
                       f"NEGATED {negated_relation(plan)!r}")
            refs = naf_relation_refs(plan)
            if refs:
                out.append(f"  NAFREFS {refs!r}")
    return "\n".join(out)


def _match_shape(kb: KnowledgeBase) -> str:
    """Every match every plan produces over the loaded KB — S1a.3.2.

    Two sweeps per plan, because the matcher has two entry shapes and they owe
    each other an identity: the full run, and a `run_seeded` at **every fact in
    the KB**, which is what forces the premise-order contract (a seeded match's
    provenance must read exactly like a full run's, seeded fact at its own
    step's position).

    Bindings go out in **dict order** — the order the matcher first bound each
    variable, which is what `Provenance.bindings` records and the trace prints.
    Premises go out as fact *positions*, so a premise-order or premise-identity
    difference names itself.
    """
    from ein.inference import match
    from ein.inference.compile import compile_rule
    from ein.inference.engine import Engine

    engine = Engine(kb)
    facts = list(kb.facts)
    at = {(f.relation_name, f.args): i for i, f in enumerate(facts)}
    ids = lambda ps: [at[(p.relation_name, p.args)] for p in ps]   # noqa: E731
    out: list[str] = []
    for rule in kb.rules.values():
        for activator in engine._activators_for(rule):
            plan = compile_rule(rule, activator)
            key = tuple(str(a) for a in (activator.args if activator else ()))
            out.append(f"PLAN {plan.rule_name} key={key!r}")
            for i, (steps, _guards) in enumerate(plan.disjuncts()):
                for bindings, premises in match.run_steps(
                        steps, dict(plan.bindings_seed), (), kb):
                    out.append(f"  RUN D{i} b={list(bindings.items())!r} "
                               f"p={ids(premises)!r}")
            for j, fact in enumerate(facts):
                for i, (steps, _guards) in enumerate(plan.disjuncts()):
                    for bindings, premises in match._seed_steps(
                            steps, plan.bindings_seed, fact, kb):
                        out.append(f"  SEED {j} D{i} b={list(bindings.items())!r} "
                                   f"p={ids(premises)!r}")
    return "\n".join(out)


def _saturate_events(kb: KnowledgeBase) -> str:
    """The `--events` log of a root saturation, plus the counters — S1a.3.3.

    This is the T2 protocol itself (`conformance/EVENTS.md`), not a second
    rendering that agrees with it by inspection: the same `ein.events` writer
    the CLI drives, at `verbose`, so a redundant firing is emitted rather than
    only counted. The port produces the same lines from its own emitter.

    The trailing `CLASH` and `SUMMARY` lines are the additions — the counters the phase
    gates on (`naf_rounds` / `naf_admitted` / `naf_retired`, `naf_dropped == 0`)
    and the compile-cache size, which are engine state rather than events.
    """
    import tempfile

    from ein import events as ev
    from ein.inference.saturator import Saturator

    fd, path = tempfile.mkstemp(suffix=".jsonl")
    os.close(fd)
    try:
        ev.open_log(path, level="verbose")
        try:
            sat = Saturator(kb)
            for _ in sat.saturate():
                pass
        finally:
            ev.close_log()
        log = Path(path).read_text(encoding="utf-8")
    finally:
        Path(path).unlink(missing_ok=True)
    from ein.inference.contradiction import ContradictionDetector
    from ein.cli._factdump import fact_sexpr
    clashes = "".join(
        f"CLASH {c.kind} "
        f"{fact_sexpr(c.positive) if c.positive is not None else '-'} "
        f"{fact_sexpr(c.negative)}\n"
        for c in ContradictionDetector(kb).detect()
    )
    return log + clashes + (
        f"SUMMARY facts={len(kb.facts)} rounds={sat.naf_rounds} "
        f"admitted={sat.naf_admitted} retired={sat.naf_retired} "
        f"dropped={sat.naf_dropped} fired={len(sat.engine._fired)} "
        f"seen={len(sat._seen)} plans={len(sat.engine.cache)}"
    )


def _source(req: dict) -> tuple[str, str | None, Path | None]:
    """`(text, filename, base_dir)` for a request."""
    if "path" in req:
        path = Path(req["path"])
        if not path.is_absolute():
            path = REPO / path
        return path.read_text(encoding="utf-8"), str(path), path.parent
    return req["text"], req.get("filename"), None


def _handle(req: dict) -> dict:
    op = req.get("op", "parse")
    if op == "macro-names":
        return {"ok": True, "out": "\n".join(sorted(stdlib_macro_names()))}

    text, filename, base_dir = _source(req)
    forms = parse(text, filename=filename)
    if op == "kb-shape":
        return {"ok": True,
                "out": _kb_shape(KnowledgeBase.from_ir(forms, base_dir=base_dir))}
    if op == "plan-shape":
        kb = KnowledgeBase.from_ir(forms, base_dir=base_dir)
        return {"ok": True,
                "out": _plan_shape(kb, req.get("filter", True))}
    if op == "match-shape":
        kb = KnowledgeBase.from_ir(forms, base_dir=base_dir)
        return {"ok": True, "out": _match_shape(kb)}
    if op == "saturate-events":
        kb = KnowledgeBase.from_ir(forms, base_dir=base_dir)
        return {"ok": True, "out": _saturate_events(kb)}
    if op == "accept":
        return {"ok": True}
    if op == "parse":
        return {"ok": True, "out": dump_canonical(forms)}
    if op == "compact":
        return {"ok": True, "out": "\n".join(dump_compact(f) for f in forms)}
    if op == "resolve":
        return {"ok": True, "out": dump_canonical(resolve_imports(forms, base_dir=base_dir))}
    if op == "minimize":
        return {"ok": True,
                "out": dump_canonical(resolve_and_minimize(forms, base_dir=base_dir))}
    if op == "expand":
        resolved = resolve_imports(forms, base_dir=base_dir)
        expanded = _expand_rule_clauses(resolved, _macro_registry(resolved))
        return {"ok": True, "out": dump_canonical(expanded)}
    raise ValueError(f"unknown op {op!r}")


def main() -> int:
    out = sys.stdout
    for line in sys.stdin:
        line = line.strip()
        if not line:
            continue
        try:
            resp = _handle(json.loads(line))
        except IRParseError as e:
            resp = {"ok": False, "kind": "IRParseError", "err": str(e)}
        except Exception as e:                       # noqa: BLE001 — the oracle
            # reports whatever ein.py raises; the caller decides whether a
            # non-parse failure is a parity question or a fixture bug.
            resp = {"ok": False, "kind": type(e).__name__, "err": str(e)}
        out.write(json.dumps(resp, ensure_ascii=False) + "\n")
        out.flush()
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
