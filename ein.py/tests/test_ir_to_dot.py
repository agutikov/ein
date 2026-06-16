"""IR → DOT renderer tests (S1.1.4).

Covers:
- Node-shape mapping: each kind of IR element produces the documented shape.
- Levi-bipartite hyperedges for n-ary facts.
- Per-form renderers emit syntactically valid digraphs.
- Rule modes (a) side-by-side and (c) overlay.
- Optional: `dot -Tcanon` parse-back (skipped if `dot` not on $PATH).
- Top-level dispatch on a tuple of forms.
- CLI `ein ir dot` emits non-empty DOT.
"""
from __future__ import annotations

import shutil
import subprocess
from pathlib import Path

import pytest

from ein.ir import (
    parse,
    render_query,
    render_rule,
    render_trace,
    to_dot,
)

REPO = Path(__file__).resolve().parents[2]
ZEBRA = REPO / "examples" / "zebra.ein"


# ═══════════ Shape mapping ═══════════

def test_type_decl_is_box():
    dot = to_dot(parse("(type Person)"))
    assert '"Person" [shape=box];' in dot


def test_type_parent_is_dashed_arrow():
    dot = to_dot(parse("(type Engineer Person)"))
    assert "style=dashed, arrowhead=empty" in dot
    assert '"Engineer" -> "Person"' in dot


def test_instance_is_oval_with_instance_of_edge():
    dot = to_dot(parse("(instance Norwegian Nationality)"))
    assert '"Norwegian" [shape=oval];' in dot
    assert '"Nationality" [shape=box];' in dot
    assert '"Norwegian" -> "Nationality"' in dot
    assert 'label="instance-of"' in dot


def test_relation_schema_is_dashed_edge():
    dot = to_dot(parse("(type A) (type B) (relation r A B)"))
    assert '"A" -> "B"' in dot
    assert 'label="r"' in dot
    assert "style=dashed" in dot


def test_nary_fact_is_octagon_even_when_compact():
    """An n-ary (arity ≠ 2) fact stays Levi-bipartite in *both* modes —
    DOT has no native hyperedge, so the octagon is unavoidable."""
    dot = to_dot(parse('(next-to Norwegian Englishman Spaniard :source "(1)")'))
    assert "shape=octagon" in dot
    assert 'label="next-to"' in dot
    # Three positional args → three role-labelled edges
    assert 'label="1"' in dot
    assert 'label="2"' in dot
    assert 'label="3"' in dot


def test_binary_fact_is_compact_arrow_by_default():
    """Compact (default): a binary fact collapses to one labelled,
    relation-coloured arrow — no Levi octagon."""
    dot = to_dot(parse('(co-located Norwegian House-1 :source "(10)")'))
    assert '"Norwegian" -> "House-1"' in dot
    assert 'label="co-located"' in dot
    assert "shape=octagon" not in dot
    # colour-by-relation styling is applied
    assert "color=" in dot


def test_binary_fact_is_levi_octagon_with_flag():
    """`levi=True` restores the canonical Levi-bipartite encoding:
    even a binary fact becomes an octagon list-node with role edges."""
    dot = to_dot(parse('(co-located Norwegian House-1 :source "(10)")'),
                 levi=True)
    assert "shape=octagon" in dot
    assert 'label="1"' in dot
    assert 'label="2"' in dot
    # no collapsed direct arrow between the two atoms
    assert '"Norwegian" -> "House-1"' not in dot


def test_equality_is_doublecircle():
    dot = to_dot(parse('(= a b :source "(1)")'))
    assert "shape=doublecircle" in dot


def test_not_fact_is_dashed():
    """`(not X)` recurses and marks the resulting edges dashed.

    Compact (default): the binary inner fact is a dashed arrow."""
    dot = to_dot(parse('(not (co-located Spaniard Coffee) :source "(1)")'))
    assert "style=dashed" in dot
    assert '"Spaniard" -> "Coffee"' in dot
    assert "shape=octagon" not in dot


def test_not_fact_is_dashed_levi():
    """Under `levi=True` the negated binary fact is a dashed octagon."""
    dot = to_dot(parse('(not (co-located Spaniard Coffee) :source "(1)")'),
                 levi=True)
    assert "style=dashed" in dot
    assert "shape=octagon" in dot


def test_reasoning_uses_dashed_edges():
    """Derived facts (reasoning layer) render with dashed edges per §6."""
    dot = to_dot(parse(
        "(co-located Blue House-2 :rule square-fwd :using (c10))"
    ))
    assert "digraph reasoning" in dot
    assert "style=dashed" in dot


# ═══════════ Pattern variables and wildcard ═══════════

def test_var_is_diamond_in_rules():
    (form,) = parse(
        "(rule v (?r) :match (?r ?a ?b) :assert ?a :why \"v\")"
    )
    rule = form  # P1.7c: a flat top-level (rule …) form
    dot = render_rule(rule, mode="c")
    # Mode (c) just emits edges; the var node isn't separately shaped
    # but the edge endpoints quote their atom-label form.
    assert "?a" in dot
    assert "?b" in dot


def test_wildcard_in_pattern():
    (form,) = parse(
        "(rule w () :match (_ ?a ?b) :assert ?a :why \"w\")"
    )
    rule = form  # P1.7c: a flat top-level (rule …) form
    dot = render_rule(rule, mode="c")
    # Wildcard head appears as the edge label.
    assert "_" in dot
    assert "?a" in dot


# ═══════════ Rule rendering modes ═══════════

def test_rule_mode_a_has_lhs_rhs_clusters():
    (form,) = parse("""
    (rule triangle ()
      :match (and (?r ?a ?b) (?r ?b ?c))
      :assert (?r ?a ?c)
      :why "tri")
    """)
    rule = form  # P1.7c: a flat top-level (rule …) form
    dot = render_rule(rule, mode="a")
    assert "cluster_lhs" in dot
    assert "cluster_rhs" in dot
    assert 'label="match"' in dot
    assert 'label="assert"' in dot
    assert "rankdir=TB" in dot  # S1.6.0: side-by-side is left-to-right


def test_rule_default_mode_is_side_by_side():
    """S1.6.0: the default rule mode is side-by-side LHS|RHS (was overlay)."""
    (form,) = parse("""
    (rule triangle ()
      :match (and (?r ?a ?b) (?r ?b ?c)) :assert (?r ?a ?c) :why "tri")
    """)
    dot = to_dot(form)  # no rule_mode → default
    assert "cluster_lhs" in dot
    assert "rankdir=TB" in dot


def test_rule_mode_aliases():
    """Friendly names map onto the legacy single-letter modes."""
    (form,) = parse('(rule x () :match (r ?a ?b) :assert (r ?b ?a) :why "x")')
    rule = form  # P1.7c: a flat top-level (rule …) form
    assert render_rule(rule, mode="sidebyside") == render_rule(rule, mode="a")
    assert render_rule(rule, mode="overlay") == render_rule(rule, mode="c")


def test_rule_mode_c_overlay_has_dashed_rhs():
    """Mode (c): LHS solid, RHS additions dashed."""
    (form,) = parse("""
    (rule triangle ()
      :match (and (?r ?a ?b) (?r ?b ?c))
      :assert (?r ?a ?c)
      :why "tri")
    """)
    rule = form  # P1.7c: a flat top-level (rule …) form
    dot = render_rule(rule, mode="c")
    assert "style=dashed" in dot
    # No clusters in overlay mode
    assert "cluster_lhs" not in dot


def test_unknown_rule_mode_raises():
    (form,) = parse("(rule x () :match a :assert b :why \"x\")")
    rule = form  # P1.7c: a flat top-level (rule …) form
    with pytest.raises(ValueError):
        render_rule(rule, mode="z")


# ═══════════ Query and trace ═══════════

def test_query_renders_keyword_args():
    (form,) = parse("(query :goal (drinks Water ?h))")
    dot = render_query(form)
    assert "digraph query" in dot
    assert ":goal" in dot or "goal" in dot


def test_trace_step_has_box_node():
    (form,) = parse("""
    (trace
      (step s1 :rule from-condition :using (c10)
               :derives (lives-in Norwegian House-1)))
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
    (type Person)
    (lives-in a b :source "(1)")
    """)
    dot = to_dot(forms)
    # Two digraphs joined by a blank line
    assert "digraph ontology" in dot
    assert "digraph facts" in dot


def test_to_dot_unknown_head_renders_as_a_fact():
    """P1.7c flat model: any non-reserved head is a fact, so `to_dot`
    renders it (via its layer's view) rather than raising on an
    'unknown' top-level head."""
    from ein.ir import Atom, SForm
    dot = to_dot(SForm(head=Atom("nonsense"), args=(Atom("x"),)))
    assert isinstance(dot, str) and "digraph" in dot


def test_to_dot_skips_config_form():
    """`(config …)` is solver knobs, not graph structure — emits no
    DOT and doesn't break sibling forms in a tuple render."""
    forms = parse("""
    (config :enable-pre-branch-lookahead true)
    (type Person)
    """)
    dot = to_dot(forms)
    assert "digraph ontology" in dot
    assert "config" not in dot
    # No blank-line gap from the dropped config chunk.
    assert "\n\n\n" not in dot


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
