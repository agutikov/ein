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
    {"op": "hyp-shape", "path": …}               → every hypgen candidate + its verdict
    {"op": "hyp-shape", "path": …, "closed": true}    → … after the auto-closure pass
    {"op": "naf-map",   "path": …}               → the static NAF dependency map
    {"op": "lattice-shape", "path": …}           → the Apriori join + the no-good store
    {"op": "explain-shape", "path": …}           → unsat cores + the ATMS label search
    {"op": "explain-shape", "path": …, "alts": false} → … with alternatives not recorded
    {"op": "commit-shape",  "path": …}           → try_commitment_set over real candidates
    {"op": "commit-shape",  "path": …, "fail-fast": false} → … saturating dead forks to quiescence
    {"op": "solve-shape",   "path": …}           → the whole solve: verdict, counters, events
    {"op": "solve-shape",   "path": …, "mode": "exhaustive"} → … without `stop_after`
    {"op": "solve-shape",   "path": …, "mode": "shuffled"}   → … with `--shuffle --seed 7`
    {"op": "dot-shape",     "path": …, "view": "<view>"}     → one of the DOT views (below)
    {"op": "trace-shape",   "path": …, "mode": "<mode>"}     → the trace / answer surface
    {"op": "dump-shape",    "path": …, "mode": "<mode>"}     → the `--dump-states` tree
    {"op": "help-shape"}                         → the CLI argument surface (below)

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

`hyp-shape` is the search layer's first rung, for S1a.4.1: it saturates the
root and runs the hypothesis generator over it, at `--events` `verbose`, so
every constructed candidate goes out with the **name of the filter that dropped
it** and every pre-candidate skip goes out as its own `hypskip`. That stream is
the observable — candidate order decides `layer_1`'s singleton order and
therefore the whole traversal, and filter *attribution* is a T1 counter — so
there is no second rendering of the candidate list that would have to agree
with it. The trailing block adds what the events do not carry: the
`--hyp-stats` report (the same `as_report_lines`, field widths included), its
`raw == emitted + sum(filtered)` invariant, and two facts about the predicates
built on the generator — whether the KB is `complete` and **how many candidates
the short-circuit built to decide it** (S1.9.E16 is invisible in the answer and
visible in that count), and the size of `open_hypotheses`. Those three calls run
with events off, since each builds its own `Lookahead` and would otherwise bury
the stream in a second copy of every `compile` event. `emit_closed` is
deliberately **not** run: it belongs to S1a.4.2, so what this op sees is the
`(__closed__ R)` facts a puzzle authored or `std.closure` derived.

`hyp-shape`'s `closed` flag runs `closed.emit_closed` on the KB before the
root saturation, which is the *other* regime the generator is asked in — the
one `ein solve --hyp-stats` and the JSON summary use, on a fork. It moves a
lot: over the corpus `closed_relation` goes from 6 pre-candidate skips to 278,
`no_hypothesis_relation` from 36 to 0 (the blacklisted relations are closed
before the blacklist is consulted), `lookahead_killed` from 547 to 279 and
`raw` from 4 479 candidates to 3 022. The newly-closed names go out in registry
order as a `CLOSED` block, because `emit_closed`'s return value is a list and
its order is a relation-registry order that nothing else exposes.

`naf-map` is `naf_deps.compute_naf_map` over a **saturated** engine cache, plus
the `DerivedNafWarning` texts `emit_derived_naf_warnings` would raise. The
cache must be saturated or the map is incomplete: most NAF-bearing rules in the
Zebra family are activated by *derived* facts that do not exist at load, so
their plan is compiled only once the enqueue pass has refreshed the cache. The
warning text is compared rather than summarised because the suite runs under
`filterwarnings=["error"]`, so a caller sees it verbatim.

`lattice-shape` drives the layer arithmetic over a **real** alive set — the
open hypotheses of a saturated root, capped at the first 12 by content order so
the layer sizes stay bounded (`zebra2`'s 56 alive would otherwise make layer 3
about 27 000 sets). The cap costs nothing the op is for: `apriori` never
inspects a KB, so what is under test is the join, the comparator, the filter
and the store, and 12 elements exercise all four.

The no-good workload is deterministic and derived from the data rather than
random, because both implementations have to run *the same* one: every 7th
layer-3 set is emitted, then every 5th layer-2 set, then every 3rd singleton,
then the layer-3 slice again. That order is chosen to make each of the three
outcomes happen — a plain insert, an insert that *removes* stored supersets
(the size-2 and size-1 clauses subsume the size-3 ones), and a clause that is
itself subsumed and dropped (the re-emitted slice). `emit_nogood` is called
with `min_size=1`, as the set-indexed engines call it. The store is then
printed as a sorted list of sorted clauses, and the layer regenerated against
it, so both the subsumption bookkeeping and its effect on candidate generation
are compared.

`explain-shape` covers the three searches over the AND/OR justification graph.
Most corpus files have **no** root contradiction, so an op that only explained
contradictions would be empty on nearly all of them — it therefore also
explains a deterministic sample of *derived* facts (every 5th by content
order, capped at 12), which is where the label propagation actually gets
exercised. Alongside the frontier it reports `rounds` and `facts_considered`,
because those pin the search's behaviour rather than only its answer, and a
second run under a deliberately tight budget pins where the caps cut. `alts:
false` re-saturates with `record_alternative_justifications` off, which is the
recorded-primary path the fallback and the pre-S1.21.7 core take.

Frontiers go out **sorted**: ein.py's is a `frozenset`, so its own iteration
order is not reproducible even run to run, and every display site sorts.

`commit-shape` runs `try_commitment_set` over the layer-1 singletons and the
first few layer-2 sets of a real alive frontier (capped at 8 alive, for the
reason `lattice-shape` caps at 12), and reports what each entering produced:
the kind, the firing count, the fork's fact count, the smallest contradiction
frontier and the hypothesis writes. `fail-fast: false` is the second regime —
a dead fork then saturates to quiescence, so its firing count and its fork
state are the full ones, which is the configuration a DAG builder that merges
dead commitments by state needs.

The last two lines are the purity claims, checked rather than asserted:
`REPEAT` re-enters the first commitment and compares the whole result against
the first call, and `ROOT` reports root's fact count, which every entering must
leave alone.

`solve-shape` is the phase's own gate: one `solve`, with the verdict, `k`,
`exhausted`, every `MonotonicStats` counter, the `LatticeProof`'s shape and the
search-layer **event stream**. Both regimes cap `max_enterings` and take
`on_budget="verdict"`, so no file can run away and the abort path is compared
rather than avoided — `zebra2-minus-15` exhausts its cap and comes back
`Aborted` on both sides.

The event log is written at `verbose` and then **filtered** to
`enter` / `nogood` / `writeback` / `warn`. Generating the rest is unavoidable
(the writer has a level, not a kind filter) but comparing it is not: a
`stop_after=1` `zebra` solve emits 58 000 events of which 19 are the search
layer's, and the other 58 000 are the saturator's, which `saturate-events`
already compares one layer down.

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

from ein import events                                       # noqa: E402
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
    # S1.21.8 negative provenance: what each firing depended on *not* holding.
    # Not in the event stream and not in the KB shape, so it is emitted here —
    # otherwise `absent_premises` is the one thing the boundary produces that
    # nothing compares.
    absents = ""
    for i, f in enumerate(kb.facts):
        provs = [("primary", f.provenance)] + [
            ("alt", a) for a in kb._alt_justifications.get((f.relation_name, f.args), ())
        ]
        for label, prov in provs:
            if prov is not None and getattr(prov, "absent_premises", ()):
                absents += f"ABSENT {i} {label} {list(prov.absent_premises)!r}\n"
    clashes = "".join(
        f"CLASH {c.kind} "
        f"{fact_sexpr(c.positive) if c.positive is not None else '-'} "
        f"{fact_sexpr(c.negative)}\n"
        for c in ContradictionDetector(kb).detect()
    )
    return log + absents + clashes + (
        f"SUMMARY facts={len(kb.facts)} rounds={sat.naf_rounds} "
        f"admitted={sat.naf_admitted} retired={sat.naf_retired} "
        f"dropped={sat.naf_dropped} fired={len(sat.engine._fired)} "
        f"seen={len(sat._seen)} plans={len(sat.engine.cache)}"
    )


def _hyp_shape(kb: KnowledgeBase, closed: bool = False) -> str:
    """Every hypgen candidate, its verdict, and the stats — S1a.4.1.

    Three phases, and the split is what keeps the stream readable: saturate
    with events off, generate with them on at `verbose`, then ask the two
    generator-backed predicates with them off again. Each `_generate` call
    builds its own `Lookahead` — which compiles every plan and emits a
    `compile` event per pair — so running the tail with the log open would
    triple the file for no signal.

    `COMPLETE`'s `raw=` is the point of the third line: `complete` is
    `next(generator, None) is None`, and the short-circuit (S1.9.E16) is
    invisible in the boolean and visible in how many candidates were built to
    reach it.

    `closed` runs the auto-closure pass first — S1a.4.2's regime, and the one
    `--hyp-stats` uses. Its own output is a `CLOSED` line, since `emit_closed`
    returns the newly-closed names in a relation-registry order nothing else
    exposes.
    """
    import tempfile

    from ein import events as ev
    from ein.inference.hypgen import HypGenStats, _generate
    from ein.inference.saturator import Saturator
    from ein.inference.solution import open_hypotheses

    newly: list[str] = []
    if closed:
        from ein.inference.closed import emit_closed
        newly = emit_closed(kb)

    sat = Saturator(kb)
    for _ in sat.saturate():
        pass

    stats = HypGenStats()
    fd, path = tempfile.mkstemp(suffix=".jsonl")
    os.close(fd)
    try:
        ev.open_log(path, level="verbose")
        try:
            for _ in _generate(kb, stats):
                pass
        finally:
            ev.close_log()
        log = Path(path).read_text(encoding="utf-8")
    finally:
        Path(path).unlink(missing_ok=True)

    short = HypGenStats()
    is_complete = next(_generate(kb, short), None) is None
    balanced = stats.raw == stats.emitted + sum(stats.filtered.values())
    lines = [
        *([f"CLOSED {newly!r}"] if closed else []),
        "STATS",
        *stats.as_report_lines(),
        f"BALANCE {balanced}",
        f"COMPLETE {is_complete} raw={short.raw}",
        f"OPEN {len(open_hypotheses(kb))}",
    ]
    return log + "\n".join(lines)


def _naf_map(kb: KnowledgeBase) -> str:
    """The static NAF dependency map over a **saturated** cache — S1a.4.2.

    Saturating first is not a convenience: most NAF-bearing rules in the Zebra
    family are activated by facts a rule derives, so their plan does not exist
    until the enqueue pass has refreshed the cache, and a map taken at load
    time silently omits exactly the rules the analysis is about.

    The warning texts are compared verbatim rather than counted — the suite
    runs under `filterwarnings=["error"]`, so a caller reads the string.
    """
    import warnings

    from ein.inference.naf_deps import compute_naf_map, emit_derived_naf_warnings
    from ein.inference.saturator import Saturator

    sat = Saturator(kb)
    for _ in sat.saturate():
        pass
    deps = compute_naf_map(sat.engine.cache)
    out = [f"NAF {d.rule_name!r} {d.activator_args!r} "
           f"derived={list(d.derived)!r} declared={list(d.declared_only)!r}"
           for d in deps]
    with warnings.catch_warnings(record=True) as caught:
        warnings.simplefilter("always")
        emit_derived_naf_warnings(sat.engine.cache)
    out += [f"WARN {w.category.__name__} {w.message}" for w in caught]
    out.append(f"SUMMARY plans={len(sat.engine.cache)} deps={len(deps)} "
               f"warnings={len(caught)}")
    return "\n".join(out)


def _lattice_shape(kb: KnowledgeBase) -> str:
    """The Apriori join, the ordering modes and the no-good store — S1a.4.3.

    Pure set arithmetic over a real alive set. The layers are capped (see the
    module docstring) and the no-good workload is a fixed recipe rather than a
    random one, because the point is that two implementations agree on it.
    """
    import tempfile

    from ein import events as ev
    from ein.inference.apriori import (
        generate_layer,
        layer_1,
        order_candidates,
    )
    from ein.inference.nogoods import emit_nogood
    from ein.inference.saturator import Saturator
    from ein.inference.solution import open_hypotheses

    for _ in Saturator(kb).saturate():
        pass
    alive_all = open_hypotheses(kb)
    capped = sorted(alive_all)[:12]
    alive = frozenset(capped)

    def show(sets) -> str:
        return "[" + ", ".join(
            "{" + " ".join(events.fact_id(f) for f in s) + "}" for s in sets
        ) + "]"

    l1 = layer_1(alive)
    l2 = generate_layer(l1, alive=alive, nogoods=kb._nogoods)
    l3 = generate_layer(l2, alive=alive, nogoods=kb._nogoods)
    out = [
        f"ALIVE {len(alive_all)} capped {len(alive)}",
        f"LAYER1 {show(l1)}",
        f"LAYER2 {show(l2)}",
        f"LAYER3 {show(l3)}",
        f"ORDER lex {show(order_candidates(l2, mode='lex'))}",
        f"ORDER score-sum {show(order_candidates(l2, mode='score-sum', kb=kb))}",
    ]

    fd, path = tempfile.mkstemp(suffix=".jsonl")
    os.close(fd)
    try:
        ev.open_log(path, level="verbose")
        try:
            for batch in (l3[::7], l2[::5], l1[::3], l3[::7]):
                for c in batch:
                    emit_nogood(kb, frozenset(c), min_size=1)
        finally:
            ev.close_log()
        log = Path(path).read_text(encoding="utf-8")
    finally:
        Path(path).unlink(missing_ok=True)

    store = sorted(
        sorted(events.fact_id(f) for f in c) for c in kb._nogoods
    )
    out.append(f"STORE {len(store)}")
    out += [f"  CLAUSE {{{' '.join(c)}}}" for c in store]
    out.append(
        f"FILTERED {show(generate_layer(l2, alive=alive, nogoods=kb._nogoods))}"
    )
    return log + "\n".join(out)


def _explain_shape(kb: KnowledgeBase, alts: bool = True) -> str:
    """Unsat cores and the ATMS label search — S1a.4.6.

    Contradictions first (most files have none), then a bounded, deterministic
    sample of derived facts, then the same sample under a tight budget.
    `rounds` / `facts_considered` are reported because they say what the search
    *did*, and a port can return the right frontier the wrong way.
    """
    from dataclasses import replace

    from ein.inference.config import SolverConfig
    from ein.inference.contradiction import ContradictionDetector
    from ein.inference.explain import (
        ExplanationBudget,
        _recorded_fallback,
        explain,
    )
    from ein.inference.frontier import smallest_contradiction_frontier
    from ein.inference.saturator import Saturator

    kb.config = replace(kb.config or SolverConfig(),
                        record_alternative_justifications=alts)
    for _ in Saturator(kb).saturate():
        pass

    def show(facts) -> str:
        return "[" + " ".join(sorted(events.fact(f) for f in facts)) + "]"

    def line(tag: str, e) -> str:
        target = events.fact(e.target) if e.target is not None else "-"
        return (f"{tag} {len(e)} target={target} exhausted={e.exhausted} "
                f"rounds={e.rounds} considered={e.facts_considered} "
                f"{show(e.frontier)}")

    tight = ExplanationBudget(max_environments=1, max_rounds=2,
                              max_env_size=1, max_facts=10)
    witnesses = [c.witness for c in ContradictionDetector(kb).detect()]
    out = [
        f"ALTS {kb._alt_justifications != {}} witnesses={len(witnesses)}",
        f"CORE {len(kb.unsat_core(witnesses))} {show(kb.unsat_core(witnesses))}",
        f"SCF {len(smallest_contradiction_frontier(kb, witnesses))} "
        f"{show(smallest_contradiction_frontier(kb, witnesses))}",
        line("CONTRA", explain(kb, witnesses)),
        # The **multi-target** budget cut, which is the only way to reach
        # `_recorded_fallback`'s tie-break: with one target its key never has
        # to break a tie, and `zebra2-bad` offers 126 witnesses.
        line("CONTRA-TIGHT", explain(kb, witnesses, budget=tight)),
        # The fallback's tie-break, reached on purpose. It only decides when
        # two targets tie on core *size*, and on this corpus the smallest tie
        # is won by the same witness whichever way it is broken — `zebra2-bad`
        # has four size-1 cores whose repr-smallest is also the first the
        # detector found. Reversing the witness list separates the two, so
        # dropping the `" ".join(sorted(repr(f)))` half of the key becomes
        # visible instead of being a comment nobody can check.
        line("FALLBACK-REV",
             _recorded_fallback(kb, list(reversed(witnesses)), 0, 0)),
    ]

    derived = sorted(
        (f for f in kb.facts
         if f.provenance is not None and f.provenance.kind == "rule"),
        key=repr,
    )[::5][:12]
    for f in derived:
        out.append(line(f"EXPLAIN {events.fact(f)}", explain(kb, [f])))
        out.append(line(f"TIGHT   {events.fact(f)}", explain(kb, [f], budget=tight)))
    out.append(f"SUMMARY facts={len(kb.facts)} derived={len(derived)}")
    return "\n".join(out)


def _commit_shape(kb: KnowledgeBase, fail_fast: bool = True) -> str:
    """`try_commitment_set` over real candidates — S1a.4.4.

    Bounded the same way `lattice-shape` is, and for the same reason: the
    primitive is what is under test, not the size of the search.
    """
    from dataclasses import replace

    from ein.inference.apriori import generate_layer, layer_1
    from ein.inference.commitment import try_commitment_set
    from ein.inference.config import SolverConfig
    from ein.inference.saturator import Saturator
    from ein.inference.solution import open_hypotheses

    kb.config = replace(kb.config or SolverConfig(),
                        enable_fail_fast_fork=fail_fast)
    for _ in Saturator(kb).saturate():
        pass
    alive_all = open_hypotheses(kb)
    alive = frozenset(sorted(alive_all)[:8])
    l1 = layer_1(alive)
    l2 = generate_layer(l1, alive=alive, nogoods=kb._nogoods)[:6]
    commitments = l1 + l2

    def show(facts) -> str:
        return "[" + " ".join(sorted(events.fact(f) for f in facts)) + "]"

    def line(c, r) -> str:
        cs = " ".join(events.fact_id(fid) for fid in c)
        # The hypothesis writes' provenance, because `branch=0` is a contract
        # and not a placeholder: it is per-commitment context the lattice
        # search does not use, and changing it changes provenance output.
        prov = " ".join(
            f"{f.provenance.kind}:{f.provenance.branch}" if f.provenance
            else "-" for f in r.hypothesis_facts
        )
        return (f"ENTER {{{cs}}} kind={r.kind} firings={len(r.firings)} "
                f"facts={len(r.kb.facts)} core={show(r.unsat_core)} "
                f"hyps={show(r.hypothesis_facts)} prov=[{prov}]")

    out = [f"ALIVE {len(alive_all)} capped {len(alive)}"]
    first = None
    for c in commitments:
        r = try_commitment_set(kb, c)
        if first is None:
            first = line(c, r)
        out.append(line(c, r))
    if first is not None:
        again = line(commitments[0], try_commitment_set(kb, commitments[0]))
        out.append(f"REPEAT {again == first}")
    out.append(f"ROOT facts={len(kb.facts)} commitments={len(commitments)}")
    return "\n".join(out)


def _solve_shape(kb: KnowledgeBase, mode: str = "fast") -> str:
    """One whole solve — the verdict, the counters, the proof and the search
    layer's events — S1a.4.5.

    Both regimes are budgeted (`max_enterings` + `on_budget="verdict"`) so a
    corpus sweep cannot be held hostage by one puzzle, and so the abort path
    is compared rather than avoided.
    """
    import os
    import tempfile

    from ein import events as ev
    from ein.inference.monotonic import solve
    from ein.inference.verdict import Aborted, Contradiction

    from dataclasses import replace

    from ein.inference.config import SolverConfig

    exhaustive = mode == "exhaustive"
    if mode == "shuffled":
        # Traversal-order probe. The verdict is shuffle-invariant (S1.5b.31),
        # so what this compares is the *traversal*: the `enter` sequence is a
        # permutation of the unshuffled one only if the two implementations
        # draw the same numbers from the same generator (Q-M1a.5).
        kb.config = replace(kb.config or SolverConfig(), lattice_order_seed=7)
    kw = dict(
        stop_after=None if exhaustive else 1,
        max_set_size=5,
        max_enterings=60 if exhaustive else 300,
        on_budget="verdict",
        store_lattice=True,
    )
    fd, path = tempfile.mkstemp(suffix=".jsonl")
    os.close(fd)
    try:
        ev.open_log(path, level="verbose")
        try:
            verdict, stats = solve(kb, **kw)
        finally:
            ev.close_log()
        kinds = ('"enter"', '"nogood"', '"writeback"', '"warn"')
        log = [l for l in Path(path).read_text(encoding="utf-8").splitlines()
               if any(k in l[:24] for k in kinds)]
    finally:
        Path(path).unlink(missing_ok=True)

    def show(facts) -> str:
        return "[" + " ".join(sorted(events.fact(f) for f in facts)) + "]"

    out = [f"VERDICT {type(verdict).__name__} k={stats.solution_nodes} "
           f"exhausted={stats.exhausted}"]
    if isinstance(verdict, Aborted):
        out.append(f"ABORT {verdict.reason}")
    if isinstance(verdict, Contradiction):
        out.append(f"CORE {len(verdict.unsat_core)} {show(verdict.unsat_core)}")
    s = stats
    out.append(
        f"STATS enterings={s.enterings_total} alive={s.enterings_alive} "
        f"dead_pre={s.enterings_dead_pre} dead_post={s.enterings_dead_post} "
        f"merged={s.facts_merged} forced={s.forced_positives} "
        f"saturate={s.saturate_count} layers={s.layers_explored} "
        f"nogoods={s.nogoods_emitted}/{s.nogoods_subsumed}"
    )
    proof = getattr(verdict, "proof", None)
    if proof is not None:
        out.append(
            f"PROOF solutions={len(proof.solutions)} "
            f"dead={len(proof.dead_commitments)} "
            f"alive_at_end={len(proof.alive_at_end)} "
            f"nogoods={len(proof.learned_nogoods)}"
        )
        # The proof's *content*, not only its shape: a solution's commitment
        # (which `_record_node`'s lex-smallest rule decides), and each dead
        # commitment with the core that refuted it.
        for r in proof.solutions:
            cs = " ".join(events.fact_id(f) for f in r.commitment)
            out.append(f"  SOLUTION {{{cs}}} layer={r.layer} "
                       f"firings={len(r.firings)}")
        for d in proof.dead_commitments:
            cs = " ".join(events.fact_id(f) for f in d.commitment)
            out.append(f"  DEAD {{{cs}}} layer={d.layer} kind={d.kind} "
                       f"core={show(d.unsat_core)}")
        clauses = sorted(
            sorted(events.fact_id(f) for f in c) for c in proof.learned_nogoods
        )
        out += [f"  CLAUSE {{{' '.join(c)}}}" for c in clauses]
    out.append(f"EVENTS {len(log)}")
    out += [f"  {l}" for l in log]
    out.append(f"ROOT facts={len(kb.facts)}")
    return "\n".join(out)


# ── DOT renderers — S1a.5.1 ────────────────────────────────────────
#
# `dot-shape` renders one *view* of a file and hands back the bytes. Unlike
# the shape ops above it invents no rendering of its own: every view is a
# renderer entry point called exactly as a CLI subcommand or the trace calls
# it, so what the diff compares is the artefact a user sees. The views split
# three ways by what they need — the parsed forms, a loaded KB, or a whole
# solve — and the Rust side enumerates the same names.

_DOT_PARSE_VIEWS = (
    "ir", "ir-levi", "ir-overlay", "ir-trace-dag", "ir-forms",
    "rules", "rules-overlay", "constraints",
)
_DOT_KB_VIEWS = (
    "kb", "kb-origin", "kb-none", "kb-no-types", "kb-no-instances",
    "kb-since",
)
_DOT_SOLVE_VIEWS = ("lattice", "lattice-full", "slice")

DOT_VIEWS = _DOT_PARSE_VIEWS + _DOT_KB_VIEWS + _DOT_SOLVE_VIEWS


def _dot_parse_view(forms, view: str) -> str:
    """The views that need only the parsed forms — `ein render rules |
    constraints` and the per-form IR renderer in each of its modes."""
    from ein.ir import Atom, SForm
    from ein.ir.to_dot import to_dot
    from ein.render import render_constraints, render_rules

    if view == "ir":
        return to_dot(forms)
    if view == "ir-levi":
        return to_dot(forms, levi=True)
    if view == "ir-overlay":
        return to_dot(forms, rule_mode="overlay")
    if view == "ir-trace-dag":
        return to_dot(forms, trace_view="dag")
    if view == "ir-forms":
        # One digraph per top-level form, through the single-node dispatch —
        # which is the only way `(config …)`'s empty string is reachable.
        out = []
        for i, f in enumerate(forms):
            head = f.head.name if isinstance(f, SForm) else "?"
            out.append(f"--- {i} {head}\n{to_dot(f)}")
        return "\n".join(out)
    if view in ("rules", "rules-overlay"):
        mode = "overlay" if view.endswith("overlay") else "sidebyside"
        rules = [n for n in forms
                 if isinstance(n, SForm) and n.head.name in ("rule", "hrule")]
        return render_rules(SForm(head=Atom(name="rules"), args=tuple(rules)),
                            mode=mode)
    if view == "constraints":
        return render_constraints(forms)
    raise ValueError(f"unknown dot view {view!r}")


def _dot_kb_view(kb: KnowledgeBase, view: str) -> str:
    """`kb.to_dot`'s whole keyword surface, plus the `since=` transition
    highlight — which needs a *pair* of KBs, so it saturates one."""
    from ein.kb.render import to_dot

    if view == "kb":
        return to_dot(kb)
    if view == "kb-origin":
        return to_dot(kb, colour_by="origin")
    if view == "kb-none":
        return to_dot(kb, colour_by="none", name="plain")
    if view == "kb-no-types":
        return to_dot(kb, include_types=False)
    if view == "kb-no-instances":
        return to_dot(kb, include_instances=False)
    if view == "kb-since":
        from ein.inference.saturator import Saturator
        root = kb.snapshot()
        for _ in Saturator(kb).saturate():
            pass
        return to_dot(kb, since=root, name="state")
    raise ValueError(f"unknown dot view {view!r}")


def _dot_solve_view(kb: KnowledgeBase, view: str) -> str:
    """The two views that *run the engine*: the commitment lattice `ein
    render lattice` prints, and the per-commitment provenance cones the trace
    embeds. Budgeted like `solve-shape`, and for the same reason."""
    from ein.inference.monotonic import solve
    from ein.render import render_slice
    from ein.render.lattice_dag import render_lattice

    verdict, _ = solve(kb, stop_after=None, max_set_size=3,
                       max_enterings=60, on_budget="verdict",
                       store_lattice=True)
    proof = getattr(verdict, "proof", None)
    if proof is None:
        return "NO PROOF"
    if view == "lattice":
        return render_lattice(proof, view="solution")
    if view == "lattice-full":
        # Always the fallback-with-a-note path: `solve` never populates
        # `kb_index`, which is what the `--view full` help text says.
        return render_lattice(proof, view="full")
    # The three shapes `trace.linearize` builds, with its own arguments: the
    # whole-commitment cone, the per-firing step diagram, and the reductio.
    out = []
    for i, s in enumerate(proof.solutions):
        out.append(f"--- solution {i}")
        out.append(render_slice(s.commitment, s.firings, s.kb, name=f"sol{i}"))
        for n, firing in enumerate(s.firings[:5], start=1):
            out.append(f"--- step {i}.{n}")
            out.append(render_slice((), (firing,), s.kb, name=f"step{n}"))
    for i, d in enumerate(proof.dead_commitments):
        out.append(f"--- dead {i}")
        out.append(render_slice(d.commitment, (), kb, name="reductio",
                                contradiction=(d.unsat_core, d.learned_clause)))
    return "\n".join(out)


def _dot_shape(forms, base_dir, view: str) -> str:
    if view in _DOT_PARSE_VIEWS:
        return _dot_parse_view(forms, view)
    kb = KnowledgeBase.from_ir(forms, base_dir=base_dir)
    if view in _DOT_KB_VIEWS:
        return _dot_kb_view(kb, view)
    return _dot_solve_view(kb, view)


# ── Trace and answer rendering — S1a.5.2 ───────────────────────────
#
# Three modes, each one solve. They are grouped this way because a solve is
# the expensive part and the renderers are the cheap part: `trace` runs one
# and renders the markdown in every flag combination plus the IR round-trip,
# `answer` runs one and renders the headline and the table at both
# `exhausted` values, and `no-proof` runs one *without* `store_lattice` to
# reach `linearize`'s three proof-less branches, which the CLI never does
# (`--trace` implies `store_lattice=True`).

_TRACE_MODES = ("trace", "answer", "no-proof")


def _solve_for_trace(kb: KnowledgeBase, *, store_lattice: bool, first: bool):
    """The bounded solve a trace mode runs — budgeted like `solve-shape`, and
    for the same reason.

    ``first=True`` is the *fast* regime (`stop_after=1`), and it is what the
    `trace` mode wants: a puzzle that stops at its first solution reaches one
    in a dozen enterings and hands the renderer a spine of several hundred
    firings, where the exhaustive regime spends its whole budget and aborts
    with nothing to narrate. The exhaustive regime is what the `answer` mode
    wants, for the opposite reason: `Ambiguity`, `Contradiction` and `Aborted`
    are only reachable there, and they are three of the table's four shapes.
    """
    from ein.inference.monotonic import solve
    return solve(kb, stop_after=1 if first else None, max_set_size=3,
                 max_enterings=300 if first else 60,
                 on_budget="verdict", store_lattice=store_lattice)


def _trace_markdown(verdict, *, diagrams: bool, full_kb: bool,
                    relevant: bool, reorder: bool) -> str:
    from ein.trace import linearize, render_markdown
    trace = linearize(verdict, diagrams=diagrams,
                      full_kb_snapshots=full_kb, relevant=relevant)
    return render_markdown(trace, mode="reorder" if reorder else "engine",
                           diagrams=diagrams)


def _trace_shape(kb: KnowledgeBase, mode: str) -> str:
    from ein.ir import parse as _parse
    from ein.trace import linearize, parse_trace_steps, trace_to_ir
    from ein.trace.answer import render_answer, render_solution_table

    if mode == "no-proof":
        # `store_lattice=False` — the only way to reach `linearize`'s three
        # proof-less branches, which the CLI never does (`--trace` implies it).
        verdict, _ = _solve_for_trace(kb, store_lattice=False, first=False)
        return _trace_markdown(verdict, diagrams=True, full_kb=False,
                               relevant=False, reorder=False)

    if mode == "answer":
        verdict, stats = _solve_for_trace(kb, store_lattice=True, first=False)
        out = []
        for exhausted in (True, False):
            out.append(f"--- answer exhausted={exhausted}")
            out.append(render_answer(verdict, exhausted=exhausted))
            out.append(f"--- table exhausted={exhausted}")
            out.append(render_solution_table(
                verdict, stats, exhausted=exhausted, source="<source>"))
        # The exhaustive regime's own trace: this is where the unsat and
        # many-solution lattice shapes are, and the `trace` mode never sees one.
        out.append("--- markdown exhaustive")
        out.append(_trace_markdown(verdict, diagrams=True, full_kb=False,
                                   relevant=False, reorder=False))
        return "\n".join(out)

    verdict, stats = _solve_for_trace(kb, store_lattice=True, first=True)

    # mode == "trace": the six flag combinations the CLI can produce, then
    # the `(trace …)` round-trip through the parser.
    flags = (
        ("default",          dict(diagrams=True,  full_kb=False, relevant=False, reorder=False)),
        ("no-diagrams",      dict(diagrams=False, full_kb=False, relevant=False, reorder=False)),
        ("full-kb",          dict(diagrams=True,  full_kb=True,  relevant=False, reorder=False)),
        ("reorder",          dict(diagrams=False, full_kb=False, relevant=False, reorder=True)),
        ("relevant",         dict(diagrams=False, full_kb=False, relevant=True,  reorder=False)),
        ("relevant-reorder", dict(diagrams=False, full_kb=False, relevant=True,  reorder=True)),
    )
    out = []
    for name, kw in flags:
        out.append(f"--- markdown {name}")
        out.append(_trace_markdown(verdict, **kw))
    # The round-trip is a *property*, and its witness is a text both
    # implementations can print: dump the steps as IR, parse them back, dump
    # again, and show both. Equal halves mean the round-trip held.
    steps = linearize(verdict, diagrams=False).steps
    ir = trace_to_ir(steps)
    forms = _parse(ir)
    again = trace_to_ir(parse_trace_steps(forms[0])) if forms else "(trace)"
    out.append("--- ir")
    out.append(ir)
    out.append("--- ir-reparsed")
    out.append(again)
    out.append(f"--- round-trip {'ok' if ir == again else 'DIFFERS'}")
    return "\n".join(out)


# ── State dumps — S1a.5.3 ──────────────────────────────────────────
#
# `--dump-states DIR` persists a whole search as a directory tree. There is no
# way to diff a tree over a line protocol, so the tree is *rendered* as one
# text — every file, sorted by path, with its bytes — and the two texts are
# diffed. The rendering is deliberately dumb: it invents nothing, so a missing
# file, an extra file, a renamed directory and a changed byte all read the same
# way.
#
# The timestamp fields are the one thing that cannot match, and they are on the
# normalisation list (design/01 §5): `ts_ms`, `elapsed_seconds` and the
# progress lines' elapsed column are replaced with a placeholder *here*, on
# both sides, rather than tolerated by the differ.

_DUMP_MODES = ("monotonic", "lattice", "progress", "abort")

_TS_KEYS = ("ts_ms", "elapsed_seconds")

# D3 (plans/m1a_rust/divergences.md): ein.rs's forks resume root's saturation
# and ein.py's re-derive it, so the same entering reaches the same state by
# narrating a quarter as much. Blanked here rather than tolerated by the
# differ, and *only* this field — `outcome`, `kind`, `commitment`,
# `facts_merged`, `unsat_core_size` and the two nogood flags sit on the same
# record and are still compared exactly.
# `ein-render/src/shape.rs`'s `normalise_dump_line` is the other half.
_FIRING_KEYS = ("firings",)


def _normalise_dump_line(line: str) -> str:
    """Blank the clock readings and the firing count — value, not presence: a
    record that lost its `ts_ms` still fails, which is the point of
    normalising rather than dropping."""
    import re
    out = line
    for key in _TS_KEYS:
        # The character class needs the `-` for a negative exponent
        # (`1.5e-05`), or the tail survives normalisation on one side only.
        out = re.sub(rf'"{key}": [-0-9.eE+]+', f'"{key}": <ts>', out)
    for key in _FIRING_KEYS:
        out = re.sub(rf'"{key}": [0-9]+', f'"{key}": <firings>', out)
    # The progress view's `(   12s)` elapsed column.
    out = re.sub(r"\(\s*\d+s\)", "(<el>)", out)
    return out


def _render_tree(root) -> str:
    """A directory as one text: every file, sorted by path, with its bytes."""
    from pathlib import Path
    root = Path(root)
    out = []
    for path in sorted(p for p in root.rglob("*") if p.is_file()):
        rel = path.relative_to(root).as_posix()
        out.append(f"=== {rel}")
        text = path.read_text(encoding="utf-8")
        if rel.startswith("enterings/"):
            # D3: a fork's own dump is narration end to end — the firing list,
            # the derivation order and `:rule` annotation of its state dump,
            # and a dying fork's core. The file set is still compared exactly;
            # everything outside `enterings/` is compared byte for byte.
            # `utils/fork_delta_verify.py` is the stronger check that replaces
            # it, and `ein-render/src/shape.rs`'s `render_tree` is this rule's
            # other half.
            out.append("=== <narrated>")
            continue
        out.extend(_normalise_dump_line(ln) for ln in text.splitlines())
        if text and not text.endswith("\n"):
            out.append("=== (no trailing newline)")
    return "\n".join(out)


def _dump_shape(kb: KnowledgeBase, mode: str) -> str:
    import io
    import tempfile
    from pathlib import Path

    from ein.inference.monotonic import solve
    from ein.inference.monotonic._state import BudgetExceededError
    from ein.inference.monotonic.state_dump import (
        LatticeDumper, MonotonicDumper, ProgressDumper,
    )

    with tempfile.TemporaryDirectory() as tmp:
        out_dir = Path(tmp) / "states"
        stream = io.StringIO()
        if mode == "monotonic":
            dumper, budget = MonotonicDumper(out_dir=out_dir), 60
        elif mode == "lattice":
            dumper, budget = LatticeDumper(out_dir=out_dir), 60
        elif mode == "progress":
            # `out_dir` too: `-v` and `--dump-states` compose, and the live
            # view is a *subclass* of the file dumper, so this is the one mode
            # that exercises both at once.
            dumper = ProgressDumper(stream=stream, progress_every=3,
                                    label="p", out_dir=out_dir)
            budget = 60
        else:                                   # abort
            # Small enough to trip the budget mid-search, with the raising
            # policy — so `summary.json` must be absent and the timeline
            # flushed anyway.
            dumper, budget = LatticeDumper(out_dir=out_dir), 3
        try:
            solve(kb, stop_after=None, max_set_size=3, max_enterings=budget,
                  on_budget="raise" if mode == "abort" else "verdict",
                  store_lattice=mode in ("lattice", "abort"), dumper=dumper)
            aborted = False
        except BudgetExceededError:
            aborted = True
        out = [f"ABORTED {aborted}", _render_tree(out_dir)]
        if mode == "progress":
            out.append("=== <stderr>")
            out.extend(_normalise_dump_line(ln)
                       for ln in stream.getvalue().splitlines())
        return "\n".join(out)


def _snapshot_shape(kb: KnowledgeBase) -> str:
    """The `LatticeSnapshotV1` projection, plus the lattice DOT rendered
    *from a snapshot* rather than from the live proof.

    The two renders are not the same picture and are not meant to be: a
    snapshot's `solutions` are post-saturation **state keys**, so its solution
    view draws whole states where the proof's draws commitments. What matters
    is that both implementations draw the same one.
    """
    from ein.inference.monotonic import solve
    from ein.inference.monotonic.snapshot import lattice_snapshot
    from ein.render.lattice_dag import render_lattice

    verdict, _ = solve(kb, stop_after=None, max_set_size=3, max_enterings=60,
                       on_budget="verdict", store_lattice=True)
    proof = getattr(verdict, "proof", None)
    if proof is None:
        return "NO PROOF"
    snap = lattice_snapshot(verdict, kb)

    def canon_text(fid) -> str:
        """One *canonical* fact as an s-expression.

        Not `events.fact_id`: `canon._hashable_args` lowers a nested `Fact` to
        its own `(rel, args)` tuple, and `fact_sexpr` renders a bare tuple with
        `str()` — so a state key carrying `(not (color-loc Blue House-1))`
        would print the tuple's repr where every other fact renderer prints an
        s-expression. This one recurses into that shape.
        """
        rel, args = fid
        parts = [
            canon_text(a)
            if (isinstance(a, tuple) and len(a) == 2
                and isinstance(a[0], str) and isinstance(a[1], tuple))
            else str(a)
            for a in args
        ]
        return f"({rel} {' '.join(parts)})" if parts else f"({rel})"

    def show(sets) -> str:
        return "[" + ", ".join(
            "{" + " ".join(canon_text(f) for f in s) + "}"
            for s in sorted(sets, key=repr)
        ) + "]"

    out = [
        f"SNAPSHOT verdict={snap.verdict_kind} nodes={len(snap.nodes_by_state_key)}",
        f"  root_state_key {show([snap.root_state_key])}",
        f"  solutions      {show(snap.solutions)}",
        # D3: a dead commitment's state key is the fork's state at the firing
        # that killed it, and ein.rs's resumed fork reaches the clash by a
        # different route. The count is compared; the keys are not.
        # `ein-render/src/shape.rs`'s `snapshot_shape` is the other half.
        f"  deads          {len(snap.deads)} key(s)",
        f"  alive_at_end   {show(snap.alive_at_end)}",
        # D3: the snapshot's lattice DOT keys its dead nodes on the dead
        # commitment's state_key, which is the divergent field above. Compared
        # for shape, not bytes; the renderer itself is byte-compared through
        # dot_parity's `lattice` / `lattice-full` views, which read a
        # LatticeProof and label dead nodes by commitment.
        "=== dot solution",
        ("<rendered>" if render_lattice(snap, view="solution") else "<empty>"),
        "=== dot full",
        ("<rendered>" if render_lattice(snap, view="full") else "<empty>"),
    ]
    return "\n".join(out)


def _source(req: dict) -> tuple[str, str | None, Path | None]:
    """`(text, filename, base_dir)` for a request."""
    if "path" in req:
        path = Path(req["path"])
        if not path.is_absolute():
            path = REPO / path
        return path.read_text(encoding="utf-8"), str(path), path.parent
    return req["text"], req.get("filename"), None


# ── help-shape — the CLI argument surface (S1a.5.4 / Q-M1a.13) ──


def _help_shape() -> str:
    """Every parser, as one comparable text.

    `--help` *layout* is on the normalisation list, so the byte diff of it is
    gone — and the byte diff was the only thing checking that ein.rs had not
    silently lost an option. This replaces it: the parser objects are walked
    directly (the parser *is* the structure; scraping the formatted text back
    would only re-import the layout the resolution exempts) and rendered as
    `{command → {option → short, metavar, arity, default, choices, group,
    help}}`.

    Options are sorted, not emitted in declaration order: ordering within a
    section is one of the things Q-M1a.13 freed.
    """
    from ein.cli import _build_parser
    from ein.cli.saturate import main as _saturate_main  # noqa: F401  (import check)

    out: list[str] = []
    _render_parser(out, "ein", _build_parser())
    _render_parser(out, "ein saturate", _saturate_parser())
    return "\n".join(out) + "\n"


def _saturate_parser():
    """`saturate`'s own parser, built the way its `main` builds it."""
    import argparse

    from ein.cli import _events

    p = argparse.ArgumentParser(
        prog="ein saturate",
        description="Benchmark + state dump for the Saturator.",
    )
    p.add_argument("file", help="path to a .ein file")
    p.add_argument("--dump", action="store_true",
                   help="after the benchmark, print the saturated KB grouped by origin")
    p.add_argument("--max-steps", type=int, default=None,
                   help="hard cap on saturator firings (raises SaturatorStepLimitError "
                        "when exceeded); useful for runaway-debugging on a fresh "
                        "input. Default: no cap.")
    p.add_argument("--progress-every", type=int, default=500,
                   help="log a one-line progress sample every N steps "
                        "(0 disables; default: 500).")
    _events.add_arguments(p)
    return p


def _squeeze(s: str) -> str:
    return " ".join(s.split())


def _render_parser(out: list[str], path: str, parser, about: str | None = None) -> None:
    import argparse

    # argparse splits what `clap` unifies: `add_parser(help=…)` is the line the
    # *parent* lists, `description=` is the blurb the subcommand's own `--help`
    # prints. The listed line is the content, so it is what travels.
    out.append(f"COMMAND {path}")
    out.append(f"  ABOUT {about if about is not None else (parser.description or '')}")

    # dest → the mutually-exclusive group it belongs to. argparse names groups
    # only positionally, so they are numbered in declaration order — which is
    # what `clap`'s explicit `ArgGroup` id has to line up with.
    group_of: dict[str, str] = {}
    for g in parser._mutually_exclusive_groups:
        name = getattr(g, "_ein_name", None) or "stop"
        for a in g._group_actions:
            group_of[a.dest] = name

    rows: list[str] = []
    subparsers = None
    for a in parser._actions:
        if isinstance(a, argparse._HelpAction):
            continue
        if isinstance(a, argparse._SubParsersAction):
            subparsers = a
            continue
        arity = 0 if a.nargs == 0 else 1
        metavar = "-" if arity == 0 else (a.metavar or a.dest.upper())
        if arity == 0:
            default = "False"
        else:
            default = "None" if a.default is None else repr(str(a.default))
        choices = "|".join(str(c) for c in a.choices) if a.choices else "-"
        group = group_of.get(a.dest, "-")
        help_text = _squeeze(a.help or "")
        if not a.option_strings:
            rows.append(
                f"  POSITIONAL {a.dest} required={a.required} help={help_text}"
            )
            continue
        longs = [o for o in a.option_strings if o.startswith("--")]
        shorts = [o for o in a.option_strings if not o.startswith("--")]
        long = longs[0][2:] if longs else a.dest
        short = shorts[0][1:] if shorts else "-"
        rows.append(
            f"  OPTION --{long} -{short} metavar={metavar} arity={arity} "
            f"default={default} choices={choices} group={group} "
            f"required={a.required} help={help_text}"
        )
    rows.sort()
    out.extend(rows)

    if subparsers is not None:
        helps = {c.dest: (c.help or "") for c in subparsers._choices_actions}
        for name in sorted(subparsers.choices):
            if name == "saturate" and path == "ein":
                out.append("  SUBCOMMAND saturate (delegated)")
                continue
            _render_parser(out, f"{path} {name}", subparsers.choices[name],
                           helps.get(name, ""))


def _handle(req: dict) -> dict:
    op = req.get("op", "parse")
    if op == "macro-names":
        return {"ok": True, "out": "\n".join(sorted(stdlib_macro_names()))}
    if op == "help-shape":
        return {"ok": True, "out": _help_shape()}

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
    if op == "hyp-shape":
        kb = KnowledgeBase.from_ir(forms, base_dir=base_dir)
        return {"ok": True, "out": _hyp_shape(kb, bool(req.get("closed", False)))}
    if op == "naf-map":
        kb = KnowledgeBase.from_ir(forms, base_dir=base_dir)
        return {"ok": True, "out": _naf_map(kb)}
    if op == "lattice-shape":
        kb = KnowledgeBase.from_ir(forms, base_dir=base_dir)
        return {"ok": True, "out": _lattice_shape(kb)}
    if op == "solve-shape":
        kb = KnowledgeBase.from_ir(forms, base_dir=base_dir)
        return {"ok": True,
                "out": _solve_shape(kb, str(req.get("mode", "fast")))}
    if op == "commit-shape":
        kb = KnowledgeBase.from_ir(forms, base_dir=base_dir)
        return {"ok": True,
                "out": _commit_shape(kb, bool(req.get("fail-fast", True)))}
    if op == "explain-shape":
        kb = KnowledgeBase.from_ir(forms, base_dir=base_dir)
        return {"ok": True,
                "out": _explain_shape(kb, bool(req.get("alts", True)))}
    if op == "dot-shape":
        return {"ok": True,
                "out": _dot_shape(forms, base_dir, str(req.get("view", "ir")))}
    if op == "trace-shape":
        kb = KnowledgeBase.from_ir(forms, base_dir=base_dir)
        return {"ok": True,
                "out": _trace_shape(kb, str(req.get("mode", "trace")))}
    if op == "dump-shape":
        kb = KnowledgeBase.from_ir(forms, base_dir=base_dir)
        mode = str(req.get("mode", "monotonic"))
        return {"ok": True,
                "out": (_snapshot_shape(kb) if mode == "snapshot"
                        else _dump_shape(kb, mode))}
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
