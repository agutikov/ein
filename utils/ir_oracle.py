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
import sys
from pathlib import Path

REPO = Path(__file__).resolve().parent.parent
sys.path.insert(0, str(REPO / "ein.py" / "src"))

from ein.ir import IRParseError, parse                      # noqa: E402
from ein.ir.dump import dump_canonical, dump_compact        # noqa: E402
from ein.ir.macros import Macro, expand_macros              # noqa: E402
from ein.ir.types import Atom, KwPair, SForm, Var           # noqa: E402
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
