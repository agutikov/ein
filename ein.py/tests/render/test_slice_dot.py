"""Derivation-slice + KB-snapshot renderer tests — S1.6.2.

Covers `ein_bot.render.slice`:

- a provenance cone renders only its own facts (not the whole KB);
- seeds are red, derived facts bold, negative (eliminated) facts grey;
- the `:why` template is rendered on firing nodes;
- a refuted-branch slice terminates in ⊥ + the learned no-good;
- `render_state` emits the complete graph, with `since=` thickening
  the facts a step added; `render_solution` renders the solved state;
- everything is valid DOT (Graphviz parse, skipped if `dot` absent).
"""
from __future__ import annotations

import shutil
import subprocess
from pathlib import Path

import pytest

from ein_bot.inference.firing import Firing
from ein_bot.ir import parse
from ein_bot.kb import KnowledgeBase
from ein_bot.kb.entities import Fact, Layer
from ein_bot.render import render_slice, render_solution, render_state

_HAVE_DOT = shutil.which("dot") is not None


# ── synthetic fact / firing builders ───────────────────────────────

def _f(rel: str, *args, layer: Layer = Layer.REASONING) -> Fact:
    return Fact(relation_name=rel, args=tuple(args), layer=layer)


def _firing(rule: str, premises: tuple[Fact, ...], derived: Fact,
            bindings: dict | None = None, redundant: bool = False) -> Firing:
    return Firing(rule=rule, activator=(), bindings=bindings or {},
                  derived=(derived,), premises=premises, redundant=redundant)


def _parses(dot: str) -> bool:
    res = subprocess.run(["dot", "-Tcanon"], input=dot, capture_output=True, text=True)
    return res.returncode == 0


# A 3-firing cone seeded by the hypothesis co-located(Blue, H3):
#   1. symmetric         → co-located(H3, Blue)
#   2. type-exclusivity  → ¬co-located(Red, H3)        (an elimination)
#   3. domain-elimination consumes the ¬-facts          (greyed premises)
SEED = _f("co-located", "Blue", "H3", layer=Layer.FACT)
NEG_RED = _f("not", _f("co-located", "Red", "H3"))
NEG_GREEN = _f("not", _f("co-located", "Green", "H3"))
COMMITMENT = (("co-located", ("Blue", "H3")),)
FIRINGS = (
    _firing("symmetric", (SEED,), _f("co-located", "H3", "Blue")),
    _firing("type-exclusivity", (SEED,), NEG_RED),
    _firing("domain-elimination", (NEG_RED, NEG_GREEN), _f("co-located", "Yellow", "H1")),
)


# ── cone scope + colours ───────────────────────────────────────────

def test_cone_shows_only_its_facts_not_whole_kb():
    dot = render_slice(COMMITMENT, FIRINGS, None)
    # cone facts present …
    assert "co-located(Blue, H3)" in dot
    assert "co-located(H3, Blue)" in dot
    # … a fact that no firing touched is absent (cone, not whole KB)
    assert "Zebra" not in dot
    # three firing rule-nodes
    assert dot.count("fire0_symmetric") >= 1
    assert "type-exclusivity" in dot
    assert "domain-elimination" in dot


def test_seed_is_red():
    dot = render_slice(COMMITMENT, FIRINGS, None)
    seed_line = next(ln for ln in dot.splitlines()
                     if 'label="co-located(Blue, H3)"' in ln)
    assert "#d62728" in seed_line          # seed red
    assert "filled" in seed_line


def test_derived_facts_are_bold():
    dot = render_slice(COMMITMENT, FIRINGS, None)
    derived_line = next(ln for ln in dot.splitlines()
                        if 'label="co-located(H3, Blue)"' in ln)
    assert "penwidth=2" in derived_line


def test_elimination_alternatives_are_grey():
    """A domain-elimination firing's negative premises render greyed."""
    dot = render_slice(COMMITMENT, FIRINGS, None)
    neg_line = next(ln for ln in dot.splitlines()
                    if 'label="not(co-located(Red, H3))"' in ln)
    assert "#7f7f7f" in neg_line            # negative grey


def test_why_template_rendered_on_firing_node():
    kb = KnowledgeBase.from_ir(parse(
        '(rule symmetric (?rel) :match (?rel ?a ?b) :assert (?rel ?b ?a)'
        ' :why "{?rel} symmetric: {?a} <-> {?b}")'
    ))
    firing = _firing("symmetric", (SEED,), _f("co-located", "H3", "Blue"),
                     bindings={"rel": "co-located", "a": "Blue", "b": "H3"})
    dot = render_slice(COMMITMENT, (firing,), kb)
    assert "co-located symmetric: Blue <-> H3" in dot


def test_empty_commitment_root_slice():
    """A root (empty-commitment) cone with one firing still renders."""
    dot = render_slice((), (FIRINGS[0],), None)
    assert "digraph slice" in dot
    assert "fire0_symmetric" in dot


# ── refuted-branch slice (⊥ + learned no-good) ─────────────────────

def test_contradiction_slice_has_bottom_and_nogood():
    unsat_core = frozenset({SEED, NEG_RED})
    learned = frozenset({("co-located", ("Blue", "H3"))})
    dot = render_slice(COMMITMENT, FIRINGS, None,
                       contradiction=(unsat_core, learned))
    assert "⊥" in dot
    assert "doublecircle" in dot
    assert "learned no-good" in dot
    # the unsat-core facts point at ⊥
    assert '-> "⊥"' in dot


# ── whole-KB snapshot + transition highlight ───────────────────────

def _kb(text: str) -> KnowledgeBase:
    return KnowledgeBase.from_ir(parse(text))


def test_render_state_emits_full_graph():
    kb = _kb(
        '(relation r A A) (instance A T) (instance B T) (r A B :source "(1)")'
    )
    dot = render_state(kb, name="snap")
    assert dot.startswith("digraph snap")
    assert "A" in dot and "B" in dot


def test_render_state_since_thickens_new_facts():
    before = _kb('(relation r X X) (r a b :layer fact)')
    after = _kb('(relation r X X) (r a b :layer fact) (r b c :layer fact)')
    plain = render_state(after)
    highlighted = render_state(after, since=before)
    # the carried fact stays thin; the new one is thickened
    assert "penwidth=3" not in plain
    assert "penwidth=3" in highlighted
    # exactly the one new edge (r b c) is thick
    thick = [ln for ln in highlighted.splitlines() if "penwidth=3" in ln]
    assert len(thick) == 1
    assert '"b" -> "c"' in thick[0]


def test_render_solution_renders_solved_state():
    kb = _kb(
        '(relation co-located A H) (instance Blue A) (instance H3 H)'
        ' (co-located Blue H3 :source "(1)")'
    )
    dot = render_solution(kb)
    assert "digraph solution" in dot
    assert "co-located" in dot


# ── DOT validity ───────────────────────────────────────────────────

@pytest.mark.skipif(not _HAVE_DOT, reason="graphviz `dot` not installed")
def test_all_renderers_emit_valid_dot():
    assert _parses(render_slice(COMMITMENT, FIRINGS, None))
    assert _parses(render_slice(COMMITMENT, FIRINGS, None,
                                contradiction=(frozenset({SEED}),
                                               frozenset({("co-located", ("Blue", "H3"))}))))
    kb = _kb('(relation r X X) (r a b :layer fact) (r b c :layer fact)')
    assert _parses(render_state(kb))
    assert _parses(render_solution(kb))


# ── integration: a real gaps_solve cone ────────────────────────────

@pytest.mark.skipif(not _HAVE_DOT, reason="graphviz `dot` not installed")
def test_real_gaps_solve_cone_parses():
    from ein_bot.inference.monotonic.solver import gaps_solve
    repo = Path(__file__).resolve().parents[3]
    kb = _kb((repo / "examples" / "branching" / "04_two_levels.ein").read_text())
    verdict, _ = gaps_solve(kb, max_set_size=3)
    rec = max(verdict.proof.solutions, key=lambda r: len(r.firings))
    dot = render_slice(rec.commitment, rec.firings, rec.kb)
    assert _parses(dot)
    # the committed hypothesis is the red seed
    assert "#d62728" in dot
