"""Tests for the unified KB → DOT renderer — S1.2.4."""
from __future__ import annotations

import os
import re
import shutil
import subprocess
from pathlib import Path

import pytest

from ein_bot.ir import parse
from ein_bot.kb import Fact, KnowledgeBase, Layer, Provenance, to_dot

REPO_ROOT = Path(__file__).resolve().parents[3]
GOLDEN_DIR = REPO_ROOT / "ein.py" / "tests" / "golden"


# ── Helpers ────────────────────────────────────────────────────────


def _node_decls(dot: str) -> list[str]:
    """Return only the node-declaration lines (those with `[shape=`)."""
    return [line for line in dot.splitlines() if "[shape=" in line]


def _edges(dot: str) -> list[str]:
    """Return only the edge lines."""
    return [line for line in dot.splitlines() if " -> " in line]


def _nodes_named(dot: str, name: str) -> list[str]:
    """Find node declarations naming `name`."""
    pat = re.compile(rf'^\s*"?{re.escape(name)}"?\s+\[')
    return [line for line in dot.splitlines() if pat.match(line)]


def _dot_available() -> bool:
    return shutil.which("dot") is not None


# ═══════════════════════ Shape mapping ═════════════════════════════


class TestShapeMapping:
    def test_zebra_has_type_boxes(self, zebra_kb):
        dot = to_dot(zebra_kb)
        boxes = [line for line in _node_decls(dot) if "shape=box" in line]
        assert len(boxes) == 7

    def test_zebra_has_instance_ovals(self, zebra_kb):
        dot = to_dot(zebra_kb)
        ovals = [line for line in _node_decls(dot) if "shape=oval" in line]
        assert len(ovals) == 30

    def test_zebra_has_no_octagons_for_binary_only_kb(self, zebra_kb):
        # zebra.ein's facts are all binary; no octagons expected.
        dot = to_dot(zebra_kb)
        octagons = [line for line in _node_decls(dot) if "shape=octagon" in line]
        assert octagons == []

    def test_ternary_fact_produces_octagon(self):
        text = """
        (ontology
          (type T)
          (instance A T) (instance B T) (instance C T)
          (relation r3 T T T))
        (facts
          (r3 A B C :source "(1)"))
        """
        kb = KnowledgeBase.from_ir(parse(text))
        dot = to_dot(kb)
        octagons = [line for line in _node_decls(dot) if "shape=octagon" in line]
        assert len(octagons) == 1


# ═══════════════════════ No duplication ════════════════════════════


class TestNoDuplication:
    def test_norwegian_appears_exactly_once(self, zebra_kb):
        # The gvpack regression: a name in ontology AND fact-layer
        # facts must NOT duplicate in the unified view.
        dot = zebra_kb.to_dot()
        norwegian_nodes = _nodes_named(dot, "Norwegian")
        assert len(norwegian_nodes) == 1
        assert "shape=oval" in norwegian_nodes[0]

    def test_house_1_appears_exactly_once(self, zebra_kb):
        dot = zebra_kb.to_dot()
        nodes = _nodes_named(dot, "House_1")
        assert len(nodes) == 1

    def test_nationality_type_appears_exactly_once(self, zebra_kb):
        dot = zebra_kb.to_dot()
        nodes = _nodes_named(dot, "Nationality")
        assert len(nodes) == 1
        assert "shape=box" in nodes[0]


# ═══════════════════════ Cross-form fusion ═════════════════════════


class TestFusion:
    def test_norwegian_has_type_edge_and_co_located_edge(self, zebra_kb):
        dot = zebra_kb.to_dot()
        type_edges = [
            line for line in _edges(dot)
            if '"Norwegian"' in line and '"Nationality"' in line
            and "dashed" in line and "is-a" in line
        ]
        assert len(type_edges) == 1
        coloc_edges = [
            line for line in _edges(dot)
            if '"Norwegian"' in line and '"House_1"' in line
            and "co-located" in line
        ]
        assert len(coloc_edges) == 1
        assert "(10)" in coloc_edges[0]


# ═══════════════════════ Layer filtering ═══════════════════════════


class TestLayerFilter:
    def test_ontology_only_has_fewer_edges_than_full(self, zebra_kb):
        dot_full = zebra_kb.to_dot()
        dot_ont = zebra_kb.to_dot(layers=(Layer.ONTOLOGY,))
        # Ontology-only renders type-edges + ontology-layer facts (the
        # spatial right-of pairs); the full view also includes the 14
        # fact-layer condition edges. So ontology-only is strictly
        # smaller.
        assert len(_edges(dot_ont)) < len(_edges(dot_full))

    def test_reasoning_only_starts_empty(self, zebra_kb):
        dot = zebra_kb.to_dot(layers=(Layer.REASONING,))
        # No reasoning-layer facts exist yet (engine hasn't run).
        edges_from_facts = [
            line for line in _edges(dot)
            if "co-located" in line or "right-of" in line or "next-to" in line
        ]
        assert edges_from_facts == []

    def test_no_type_edges_when_ontology_excluded(self, zebra_kb):
        dot = zebra_kb.to_dot(layers=(Layer.FACT,))
        is_a_edges = [line for line in _edges(dot) if "is-a" in line]
        assert is_a_edges == []


# ═══════════════════════ Colour stability ══════════════════════════


class TestColourStability:
    def test_same_relation_same_colour_across_runs(self, zebra_kb):
        dot1 = zebra_kb.to_dot()
        dot2 = zebra_kb.to_dot()

        def colour_for_rel(dot, rel):
            for line in _edges(dot):
                if rel in line:
                    m = re.search(r'color="(#[0-9A-Fa-f]{6})"', line)
                    if m:
                        return m.group(1)
            return None

        assert colour_for_rel(dot1, "co-located") == colour_for_rel(dot2, "co-located")
        assert colour_for_rel(dot1, "right-of") == colour_for_rel(dot2, "right-of")
        assert colour_for_rel(dot1, "co-located") != colour_for_rel(dot1, "right-of")

    def test_colour_by_layer_overrides_relation_colour(self, zebra_kb):
        dot = zebra_kb.to_dot(colour_by="layer")
        for line in _edges(dot):
            if 'color="' in line:
                m = re.search(r'color="(#[0-9A-Fa-f]{6})"', line)
                if m:
                    assert m.group(1) in ("#444444", "#000000", "#1f77b4")


# ═══════════════════════ Reasoning-layer styling ═══════════════════


class TestReasoningStyling:
    def test_derived_fact_renders_dashed_with_rule_label(self, zebra_kb):
        derived = Fact(
            relation_name="co-located",
            args=("Japanese", "Zebra"),
            layer=Layer.REASONING,
            provenance=Provenance.from_rule(
                rule="forced-by-unique-position",
                premises_raw=(),
            ),
        )
        zebra_kb.add_fact(derived)
        zebra_kb._index_fact(zebra_kb.facts[-1])
        try:
            dot = zebra_kb.to_dot()
            edges = [
                line for line in _edges(dot)
                if '"Japanese"' in line and '"Zebra"' in line
            ]
            assert len(edges) == 1
            assert "style=dashed" in edges[0]
            assert "forced-by-unique-position" in edges[0]
        finally:
            # The fixture is session-scoped; pop the synthetic fact so
            # later tests aren't affected.
            zebra_kb.facts.pop()
            zebra_kb.rebuild_indexes()


# ═══════════════════════ Encoding-agnostic ═════════════════════════


class TestEncodingAgnostic:
    def test_zebra2_renders_norwegian_once(self, zebra2_kb):
        dot = zebra2_kb.to_dot()
        nodes = _nodes_named(dot, "Norwegian")
        assert len(nodes) == 1
        assert "shape=oval" in nodes[0]

    def test_zebra2_has_is_a_edges(self, zebra2_kb):
        # Inheritance is explicit `is-a` facts in zebra2 — styled like
        # type-edges (dashed empty arrow).
        dot = zebra2_kb.to_dot()
        is_a_edges = [
            line for line in _edges(dot)
            if "is-a" in line and "dashed" in line
        ]
        assert len(is_a_edges) > 0

    def test_zebra2_nationality_as_type_box(self, zebra2_kb):
        # zebra2 has no Type entities; Nationality is harvested by
        # logical_types from is-a parents → rendered as box.
        dot = zebra2_kb.to_dot()
        nodes = _nodes_named(dot, "Nationality")
        assert len(nodes) == 1
        assert "shape=box" in nodes[0]


# ═══════════════════════ Suppressions ══════════════════════════════


class TestSuppressions:
    def test_rule_application_facts_suppressed(self, zebra_kb):
        # `(symmetric co-located)` doesn't appear as a node or edge.
        dot = zebra_kb.to_dot()
        symmetric_edges = [line for line in _edges(dot) if '"symmetric"' in line]
        assert symmetric_edges == []

    def test_instance_facts_suppressed_in_favour_of_type_edges(self, zebra_kb):
        # `(instance Norwegian Nationality)` is shown as a dashed
        # type-edge; the kernel `instance` fact form is suppressed to
        # avoid duplicate edges.
        dot = zebra_kb.to_dot()
        instance_edges = [line for line in _edges(dot) if '"instance"' in line]
        assert instance_edges == []


# ═══════════════════════ DOT round-trip ════════════════════════════


@pytest.mark.skipif(not _dot_available(), reason="graphviz not in PATH")
class TestDotParseRoundTrip:
    def test_zebra_dot_parses_canon(self, zebra_kb):
        dot = zebra_kb.to_dot()
        result = subprocess.run(
            ["dot", "-Tcanon"], input=dot, capture_output=True, text=True,
        )
        assert result.returncode == 0, result.stderr

    def test_zebra2_dot_parses_canon(self, zebra2_kb):
        dot = zebra2_kb.to_dot()
        result = subprocess.run(
            ["dot", "-Tcanon"], input=dot, capture_output=True, text=True,
        )
        assert result.returncode == 0, result.stderr

    def test_zebra_dot_renders_svg(self, zebra_kb):
        dot = zebra_kb.to_dot()
        result = subprocess.run(
            ["dot", "-Tsvg"], input=dot, capture_output=True, text=True,
        )
        assert result.returncode == 0
        assert len(result.stdout) > 5000
        assert "<svg" in result.stdout
        assert "<ellipse" in result.stdout
        assert ">Norwegian</" in result.stdout

    def test_zebra_layer_filter_yields_smaller_svg(self, zebra_kb):
        dot_all = zebra_kb.to_dot()
        dot_ont = zebra_kb.to_dot(layers=(Layer.ONTOLOGY,))
        s_all = subprocess.run(
            ["dot", "-Tsvg"], input=dot_all, capture_output=True, text=True,
        ).stdout
        s_ont = subprocess.run(
            ["dot", "-Tsvg"], input=dot_ont, capture_output=True, text=True,
        ).stdout
        assert len(s_ont) < len(s_all)


# ═══════════════════════ Visual regression (golden file) ═══════════


class TestVisualRegression:
    def test_zebra_unified_matches_golden(self, zebra_kb):
        """Golden-file diff for the canonical unified-KB DOT.

        Set ``UPDATE_GOLDEN=1`` in the env to refresh after an
        intentional change.
        """
        golden_path = GOLDEN_DIR / "kb_zebra_unified.dot"
        dot = zebra_kb.to_dot()
        if os.environ.get("UPDATE_GOLDEN"):
            golden_path.parent.mkdir(parents=True, exist_ok=True)
            golden_path.write_text(dot)
            pytest.skip("Golden refreshed; re-run without UPDATE_GOLDEN.")
        if not golden_path.exists():
            golden_path.parent.mkdir(parents=True, exist_ok=True)
            golden_path.write_text(dot)
        expected = golden_path.read_text()
        assert dot == expected, (
            "unified DOT diverged from golden; if intentional, set "
            "UPDATE_GOLDEN=1 and re-run."
        )


# ═══════════════════════ Options ══════════════════════════════════


class TestRenderOptions:
    def test_include_types_false_omits_boxes(self, zebra_kb):
        dot = zebra_kb.to_dot(include_types=False)
        boxes = [line for line in _node_decls(dot) if "shape=box" in line]
        assert boxes == []

    def test_include_instances_false_omits_ovals(self, zebra_kb):
        dot = zebra_kb.to_dot(include_instances=False)
        ovals = [line for line in _node_decls(dot) if "shape=oval" in line]
        assert ovals == []

    def test_custom_graph_name(self, zebra_kb):
        dot = zebra_kb.to_dot(name="my_zebra")
        assert dot.startswith("digraph my_zebra {")
