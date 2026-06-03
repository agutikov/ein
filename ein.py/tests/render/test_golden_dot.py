"""S1.7c.25 — golden-capture harness for every DOT emitter.

One parametrised test renders each DOT-emitting entry point on a fixed,
deterministic input and asserts its bytes against a committed golden under
``tests/golden/dot/``. This LOCKS the current byte output of every emitter
so the S1.7c.25 shared-``dot_util``-emitter refactor (+ the S1.7c.26 routing)
can be proven a pure internal consolidation: those stages must keep all
goldens byte-for-byte unchanged.

All emitters are byte-stable across ``PYTHONHASHSEED``: the only hashing in
any node-id is content-keyed ``hashlib.md5(...)[:10]`` (+ ``hash_color`` via
``hashlib.sha1`` for palette indices, not embedded in ids), and ``_Builder``
ids are a decimal counter — no builtin ``hash()``, no set-ordering, no
time/uuid. So no seed pinning is needed.

Refresh after an *intended* change: ``UPDATE_GOLDEN=1 pytest …`` then re-run
without it. (Mirrors the pattern in ``tests/kb/test_render.py``.)
"""
from __future__ import annotations

import os
from pathlib import Path

import pytest

from ein_bot.inference.firing import Firing
from ein_bot.inference.monotonic.lattice import (
    DeadCommitment,
    LatticeProof,
    SolutionRecord,
)
from ein_bot.ir import parse
from ein_bot.ir.to_dot import render_query, render_trace, to_dot
from ein_bot.ir.types import Atom, SForm
from ein_bot.kb import KnowledgeBase, Provenance
from ein_bot.kb.entities import Fact, Layer
from ein_bot.render import (
    render_constraints,
    render_rule,
    render_rules,
    render_slice,
    render_solution,
    render_state,
)
from ein_bot.render.lattice_dag import render_lattice

GOLDEN_DIR = Path(__file__).resolve().parents[1] / "golden" / "dot"


# ── deterministic input builders ───────────────────────────────────

def _kb(text: str) -> KnowledgeBase:
    return KnowledgeBase.from_ir(parse(text))


def _small_kb() -> KnowledgeBase:
    # binary fact + is-a edge + a hyperedge → exercises every node/edge path.
    return _kb(
        "(type Thing)\n(instance a Thing) (instance b Thing) (instance c Thing)\n"
        "(relation r Thing Thing)\n(relation tern Thing Thing Thing)\n"
        '(r a b :source "(1)")\n(tern a b c :source "(2)")\n'
    )


def _prov_dag_dot() -> str:
    kb = _kb(
        "(relation p T T)\n(relation q T T)\n"
        "(instance a T) (instance b T) (instance c T)\n"
        '(p a b :source "(1)")\n(p b c :source "(2)")\n'
    )
    derived = Fact(
        relation_name="q", args=("a", "c"), layer=Layer.REASONING,
        provenance=Provenance.from_rule(
            rule="triangle",
            premises_raw=(("p", ("a", "b")), ("p", ("b", "c"))),
        ),
    )
    kb.add_fact(derived)
    kb._index_fact(kb.facts[-1])
    return kb.derivation_dag(derived).to_dot()


# Slice cone: hypothesis co-located(Blue, H3) + 3 firings (mirrors the
# proven recipe in test_slice_dot.py — inlined to stay self-contained).
def _f(rel: str, *args: object, layer: Layer = Layer.REASONING) -> Fact:
    return Fact(relation_name=rel, args=tuple(args), layer=layer)


def _firing(rule: str, premises: tuple[Fact, ...], derived: Fact) -> Firing:
    return Firing(rule=rule, activator=(), bindings={}, derived=derived,
                  premises=premises, redundant=False)


_SEED = _f("co-located", "Blue", "H3", layer=Layer.FACT)
_NEG_RED = _f("not", _f("co-located", "Red", "H3"))
_NEG_GREEN = _f("not", _f("co-located", "Green", "H3"))
_SLICE_COMMITMENT = (("co-located", ("Blue", "H3")),)
_SLICE_FIRINGS = (
    _firing("symmetric", (_SEED,), _f("co-located", "H3", "Blue")),
    _firing("type-exclusivity", (_SEED,), _NEG_RED),
    _firing("domain-elimination", (_NEG_RED, _NEG_GREEN),
            _f("co-located", "Yellow", "H1")),
)


def _synthetic_proof() -> LatticeProof:
    sol = SolutionRecord(commitment=(("p", ("a",)),), kb=_kb(" "),
                         firings=(), layer=1)
    d1 = DeadCommitment(
        commitment=(("p", ("b",)),),
        unsat_core=frozenset({Fact("p", ("b",)), Fact("q", ("b",))}),
        learned_clause=frozenset({("p", ("b",))}), layer=1, kind="dead-post",
    )
    d2 = DeadCommitment(
        commitment=(("p", ("c",)),),
        unsat_core=frozenset({Fact("p", ("c",))}),
        learned_clause=frozenset({("p", ("c",))}), layer=1, kind="dead-pre",
    )
    return LatticeProof(solutions=(sol,), dead_commitments=(d1, d2))


_RULE = parse('(rule t () :match (r ?a ?b) :assert (r ?b ?a) :why "t")')[0]
_SYMM = parse('(rule symmetric (?rel) :match (?rel ?a ?b) '
              ':assert (?rel ?b ?a) :why "sym")')[0]
_RULES_FORM = SForm(head=Atom("rules"), args=(_RULE, _SYMM))
_CONSTRAINTS_FORMS = parse(
    "(relation co-located Thing House)\n(symmetric co-located)\n"
    "(relation next-to House House)\n(transitive next-to)\n"
)
_TRACE = parse(
    "(trace (step s1 :rule from-condition :using (c10) "
    ":derives (lives-in Norwegian House-1))"
    " (step s2 :rule adjacent :using (and (lives-in Norwegian House-1)) "
    ":derives (color-loc Blue House-2)))"
)[0]

# IR-text inputs for the `to_dot` dispatch cases (named to keep the CASES
# table lines short).
_FACT_TEXT = '(co-located Norwegian House-1 :source "(10)")'
_NEG_TEXT = '(not (co-located Spaniard Coffee) :source "(1)")'
_REASONING_TEXT = "(co-located Blue House-2 :rule square-fwd :using (c10))"
_QUERY_TEXT = "(query :mode solve :goal (drinks Water ?h))"


# ── the cases: (name, zero-arg render thunk) ───────────────────────

CASES = [
    ("ir_to_dot_type",        lambda: to_dot(parse("(type Person)"))),
    ("ir_to_dot_subtype",     lambda: to_dot(parse("(type Engineer Person)"))),
    ("ir_to_dot_fact",        lambda: to_dot(parse(_FACT_TEXT))),
    ("ir_to_dot_neg",         lambda: to_dot(parse(_NEG_TEXT))),
    ("ir_to_dot_reasoning",   lambda: to_dot(parse(_REASONING_TEXT))),
    ("ir_render_query",       lambda: render_query(parse(_QUERY_TEXT)[0])),
    ("ir_render_trace_a",     lambda: render_trace(_TRACE, view="a")),
    ("ir_render_trace_dag",   lambda: render_trace(_TRACE, view="dag")),
    ("kb_render_to_dot",      lambda: _small_kb().to_dot()),
    ("kb_provenance_dag",     _prov_dag_dot),
    ("slice_render_slice",    lambda: render_slice(_SLICE_COMMITMENT, _SLICE_FIRINGS, None)),
    ("slice_render_state",    lambda: render_state(_small_kb(), name="snap")),
    ("slice_render_solution", lambda: render_solution(_small_kb())),
    ("lattice_render",        lambda: render_lattice(_synthetic_proof(), view="solution")),
    ("render_constraints",    lambda: render_constraints(_CONSTRAINTS_FORMS)),
    ("render_rule",           lambda: render_rule(_RULE, mode="sidebyside")),
    ("render_rules",          lambda: render_rules(_RULES_FORM)),
]


@pytest.mark.parametrize("name,thunk", CASES, ids=[c[0] for c in CASES])
def test_emitter_matches_golden(name: str, thunk) -> None:
    golden = GOLDEN_DIR / f"{name}.dot"
    dot = thunk()
    assert isinstance(dot, str) and dot.strip(), f"{name}: emitter returned empty DOT"
    if os.environ.get("UPDATE_GOLDEN") or not golden.exists():
        golden.parent.mkdir(parents=True, exist_ok=True)
        golden.write_text(dot, encoding="utf-8")
        if os.environ.get("UPDATE_GOLDEN"):
            pytest.skip(f"{name}: golden refreshed; re-run without UPDATE_GOLDEN.")
    assert dot == golden.read_text(encoding="utf-8"), (
        f"{name}: DOT diverged from golden; if intentional, "
        f"set UPDATE_GOLDEN=1 and re-run."
    )
