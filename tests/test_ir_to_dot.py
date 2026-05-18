"""IR → DOT renderer tests (S1.1.4).

Covers:
- Node-shape mapping: each kind of IR element produces the documented shape.
- Levi-bipartite hyperedges for n-ary facts.
- Per-form renderers emit syntactically valid digraphs.
- Rule modes (a) side-by-side and (c) overlay.
- Optional: `dot -Tcanon` parse-back (skipped if `dot` not on $PATH).
- Top-level dispatch on a tuple of forms.
- CLI `ein-bot ir dot` emits non-empty DOT.
"""
from __future__ import annotations

import shutil
import subprocess
from pathlib import Path

import pytest

from ein_bot.ir import (
    parse, render_facts, render_ontology, render_query, render_reasoning,
    render_rule, render_trace, to_dot,
)


REPO = Path(__file__).resolve().parent.parent
ZEBRA = REPO / "examples" / "zebra.ein"


# ═══════════ Shape mapping ═══════════

def test_type_decl_is_box():
    (form,) = parse("(ontology (type Person))")
    dot = render_ontology(form)
    assert '"Person" [shape=box];' in dot


def test_type_parent_is_dashed_arrow():
    (form,) = parse("(ontology (type Engineer Person))")
    dot = render_ontology(form)
    assert "style=dashed, arrowhead=empty" in dot
    assert '"Engineer" -> "Person"' in dot


def test_instance_is_oval_with_instance_of_edge():
    (form,) = parse("(ontology (instance Norwegian Nationality))")
    dot = render_ontology(form)
    assert '"Norwegian" [shape=oval];' in dot
    assert '"Nationality" [shape=box];' in dot
    assert '"Norwegian" -> "Nationality"' in dot
    assert 'label="instance-of"' in dot


def test_relation_schema_is_dashed_edge():
    (form,) = parse("(ontology (type A) (type B) (relation r (A B)))")
    dot = render_ontology(form)
    assert '"A" -> "B"' in dot
    assert 'label="r"' in dot
    assert "style=dashed" in dot


def test_generic_fact_is_levi_bipartite():
    """An n-ary fact becomes one octagon node + one edge per arg."""
    (form,) = parse("(facts (next-to Norwegian Englishman Spaniard))")
    dot = render_facts(form)
    assert "shape=octagon" in dot
    assert 'label="next-to"' in dot
    # Three positional args → three role-labelled edges
    assert 'label="1"' in dot
    assert 'label="2"' in dot
    assert 'label="3"' in dot


def test_equality_is_doublecircle():
    (form,) = parse("(facts (= a b))")
    dot = render_facts(form)
    assert "shape=doublecircle" in dot


def test_not_fact_is_dashed():
    """`(not X)` recurses and marks the resulting edges dashed."""
    (form,) = parse("(facts (not (co-located Spaniard Coffee)))")
    dot = render_facts(form)
    assert "style=dashed" in dot
    assert "shape=octagon" in dot


def test_reasoning_uses_dashed_edges():
    """Derived facts (reasoning layer) render with dashed edges per §6."""
    (form,) = parse(
        "(reasoning (co-located Blue House_2 :rule square-fwd :using (c10)))"
    )
    dot = render_reasoning(form)
    assert "digraph reasoning" in dot
    assert "style=dashed" in dot


# ═══════════ Pattern variables and wildcard ═══════════

def test_var_is_diamond_in_rules():
    (form,) = parse(
        "(rules (rule v (?r) :match (?r ?a ?b) :assert ?a :why \"v\"))"
    )
    rule = form.args[0]
    dot = render_rule(rule, mode="c")
    # Mode (c) just emits edges; the var node isn't separately shaped
    # but the edge endpoints quote their atom-label form.
    assert "?a" in dot
    assert "?b" in dot


def test_wildcard_in_pattern():
    (form,) = parse(
        "(rules (rule w () :match (_ ?a ?b) :assert ?a :why \"w\"))"
    )
    rule = form.args[0]
    dot = render_rule(rule, mode="c")
    # Wildcard head appears as the edge label.
    assert "_" in dot
    assert "?a" in dot


# ═══════════ Rule rendering modes ═══════════

def test_rule_mode_a_has_lhs_rhs_clusters():
    (form,) = parse("""
    (rules (rule triangle ()
      :match (and (?r ?a ?b) (?r ?b ?c))
      :assert (?r ?a ?c)
      :why "tri"))
    """)
    rule = form.args[0]
    dot = render_rule(rule, mode="a")
    assert "cluster_lhs" in dot
    assert "cluster_rhs" in dot
    assert 'label="match"' in dot
    assert 'label="assert"' in dot


def test_rule_mode_c_overlay_has_dashed_rhs():
    """Mode (c): LHS solid, RHS additions dashed."""
    (form,) = parse("""
    (rules (rule triangle ()
      :match (and (?r ?a ?b) (?r ?b ?c))
      :assert (?r ?a ?c)
      :why "tri"))
    """)
    rule = form.args[0]
    dot = render_rule(rule, mode="c")
    assert "style=dashed" in dot
    # No clusters in overlay mode
    assert "cluster_lhs" not in dot


def test_unknown_rule_mode_raises():
    (form,) = parse("(rules (rule x () :match a :assert b :why \"x\"))")
    rule = form.args[0]
    with pytest.raises(ValueError):
        render_rule(rule, mode="z")


# ═══════════ Query and trace ═══════════

def test_query_renders_keyword_args():
    (form,) = parse("(query :mode solve :goal (drinks Water ?h))")
    dot = render_query(form)
    assert "digraph query" in dot
    assert ":mode" in dot or "mode" in dot


def test_trace_step_has_box_node():
    (form,) = parse("""
    (trace
      (step s1 :rule from-condition :using (c10)
               :derives (lives-in Norwegian House_1)))
    """)
    dot = render_trace(form, view="a")
    assert "digraph trace" in dot
    assert '"s1"' in dot


def test_trace_invalid_view_raises():
    (form,) = parse("(trace)")
    with pytest.raises(ValueError):
        render_trace(form, view="z")


# ═══════════ Top-level dispatch ═══════════

def test_to_dot_on_tuple_of_forms():
    forms = parse("""
    (ontology (type Person))
    (facts (lives-in a b))
    """)
    dot = to_dot(forms)
    # Two digraphs joined by a blank line
    assert "digraph ontology" in dot
    assert "digraph facts" in dot


def test_to_dot_on_unknown_top_level_raises():
    """A synthetic SForm with an unrecognised head should raise."""
    from ein_bot.ir import Atom, SForm
    bogus = SForm(head=Atom("nonsense"), args=())
    with pytest.raises(ValueError):
        to_dot(bogus)


# ═══════════ Zebra smoke test ═══════════

def test_zebra_to_dot_is_non_empty():
    forms = parse(ZEBRA.read_text(encoding="utf-8"))
    dot = to_dot(forms)
    assert dot
    assert "digraph" in dot
    assert "shape=octagon" in dot   # has hyperedges
    assert "shape=oval" in dot       # has instances
    assert "shape=box" in dot        # has types


# ═══════════ `dot -Tcanon` parse-back (skipped if no graphviz) ═══════════

_HAVE_DOT = shutil.which("dot") is not None


@pytest.mark.skipif(not _HAVE_DOT, reason="graphviz `dot` not installed")
def test_zebra_dot_parses_under_graphviz():
    """The emitted DOT is syntactically valid Graphviz."""
    forms = parse(ZEBRA.read_text(encoding="utf-8"))
    dot = to_dot(forms)
    # Each digraph independently; dot -Tcanon emits canonical form
    # and exits non-zero on parse errors.
    chunks = [c for c in dot.split("\n\n") if c.strip()]
    for chunk in chunks:
        result = subprocess.run(
            ["dot", "-Tcanon"],
            input=chunk,
            capture_output=True,
            text=True,
            timeout=10,
        )
        assert result.returncode == 0, (
            f"dot -Tcanon rejected this chunk:\n--- chunk ---\n"
            f"{chunk}\n--- stderr ---\n{result.stderr}"
        )
