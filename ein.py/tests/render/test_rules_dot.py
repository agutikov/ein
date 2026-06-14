"""Rule + constraint DOT renderer tests — S1.6.1.

Covers `ein_bot.render.rules` and `ein_bot.render.constraints`:

- every rule in the example libraries emits DOT that Graphviz parses
  (both modes), skipped if `dot` is not installed;
- the per-panel `_L`/`_R` node-id suffix never leaks into a label;
- guard predicates (`neq`/`eq`) render as `≠`/`=` constraints, not
  relation arrows; `(absent …)` renders as a `cluster_absent`;
- variables are diamonds, ground atoms rectangles;
- the constraint diagram surfaces each relation's structural
  properties;
- the `ein-bot render …` CLI subcommands.
"""
from __future__ import annotations

import re
import shutil
import subprocess
from pathlib import Path

import pytest

from ein_bot.cli import main
from ein_bot.ir import Atom, SForm, parse
from ein_bot.render import render_constraints, render_rule, render_rules

REPO = Path(__file__).resolve().parents[3]
ZEBRA = REPO / "examples" / "zebra.ein"
ZEBRA2 = REPO / "examples" / "zebra2.ein"

_HAVE_DOT = shutil.which("dot") is not None


# ── helpers ────────────────────────────────────────────────────────

def _rules_of(path: Path) -> list[SForm]:
    # P1.7c: rules are flat top-level `(rule …)` / `(hrule …)` forms.
    forms = parse(path.read_text(encoding="utf-8"))
    return [f for f in forms
            if isinstance(f, SForm) and f.head.name in ("rule", "hrule")]


def _rule_named(path: Path, name: str) -> SForm:
    for r in _rules_of(path):
        if r.args and isinstance(r.args[0], Atom) and r.args[0].name == name:
            return r
    raise KeyError(name)


def _labels(dot: str) -> list[str]:
    """All quoted `label="…"` values in the DOT."""
    return re.findall(r'label="((?:[^"\\]|\\.)*)"', dot)


def _dot_parses(dot: str) -> bool:
    res = subprocess.run(["dot", "-Tcanon"], input=dot, capture_output=True, text=True)
    return res.returncode == 0


# ── every rule parses, both modes ──────────────────────────────────

@pytest.mark.skipif(not _HAVE_DOT, reason="graphviz `dot` not installed")
@pytest.mark.parametrize("path", [ZEBRA, ZEBRA2], ids=["zebra", "zebra2"])
@pytest.mark.parametrize("mode", ["sidebyside", "overlay"])
def test_every_rule_renders_valid_dot(path: Path, mode: str):
    for rule in _rules_of(path):
        dot = render_rule(rule, mode=mode)
        assert _dot_parses(dot), f"graphviz rejected {rule.args[0].name} ({mode})"


# ── clean labels — the `_L`/`_R` suffix must not leak ──────────────

def test_panel_suffix_does_not_leak_into_labels():
    (form,) = parse('(rule t () :match (r ?a ?b) :assert (r ?b ?a) :why "t")')
    dot = render_rule(form, mode="sidebyside")
    # the disambiguating suffix is in the node *ids* …
    assert '"?a_L"' in dot
    assert '"?a_R"' in dot
    # … but never in any label (both panels just show ?a / ?b / r).
    for lbl in _labels(dot):
        assert not re.search(r"_[LR]$", lbl), f"suffix leaked into label: {lbl!r}"
    assert set(_labels(dot)) >= {"?a", "?b", "r"}


def test_same_variable_distinct_nodes_per_panel():
    """`?a` appears in both panels as two ids but one shared label."""
    (form,) = parse('(rule t () :match (r ?a ?b) :assert (r ?b ?a) :why "t")')
    dot = render_rule(form, mode="sidebyside")
    assert '"?a_L"' in dot and '"?a_R"' in dot          # two nodes
    assert dot.count('label="?a"') == 2                 # same label, both panels


# ── variable / atom shapes ─────────────────────────────────────────

def test_variables_are_diamonds_atoms_rectangles():
    (form,) = parse(
        '(rule t () :match (color ?a Red) :assert (loc ?a House-1) :why "t")'
    )
    dot = render_rule(form, mode="sidebyside")
    # ?a is a variable → diamond; Red / House-1 are ground → rectangle.
    assert re.search(r'"\?a_[LR]" \[label="\?a", shape=diamond\]', dot)
    assert re.search(r'"Red_L" \[label="Red", shape=rectangle\]', dot)


# ── guard predicates render as constraints, not relations ──────────

def test_neq_renders_as_constraint_not_relation():
    rule = _rule_named(ZEBRA2, "adjacent-via-bwd-negative")
    dot = render_rule(rule, mode="sidebyside")
    # the `neq` is a dotted, undirected ≠ link that does not affect rank
    assert "≠" in dot
    assert "dir=none" in dot
    assert "constraint=false" in dot
    # it is NOT drawn as a relation arrow labelled "neq"
    assert 'label="neq"' not in dot


def test_absent_guard_is_a_cluster():
    rule = _rule_named(ZEBRA2, "adjacent-via-bwd-negative")
    dot = render_rule(rule, mode="sidebyside")
    assert "cluster_absent" in dot
    # the binder-local var lives inside the guard; the ∄ marker is shown
    assert "∄" in dot
    assert '"?h_o_L"' in dot


def test_negated_premise_is_red_with_neg_prefix():
    rule = _rule_named(ZEBRA2, "adjacent-via-bwd-negative")
    dot = render_rule(rule, mode="sidebyside")
    assert "¬?R1" in dot          # (not (?R1 ?V1 ?h1)) → ¬?R1 edge
    assert "#d62728" in dot       # negative red


# ── n-ary relation pattern → octagon ───────────────────────────────

def test_nary_relation_pattern_is_octagon():
    rule = _rule_named(ZEBRA2, "derive-adjacent-via")  # 5-ary (adjacent-via …) match
    dot = render_rule(rule, mode="sidebyside")
    assert "shape=octagon" in dot


# ── combined library references every rule name ────────────────────

def test_render_rules_references_all_rule_names():
    rules = _rules_of(ZEBRA2)
    dot = render_rules(SForm(head=Atom("rules"), args=tuple(rules)))
    for r in rules:
        if r.args and isinstance(r.args[0], Atom):
            safe = r.args[0].name.replace("-", "_")
            assert f"rule_{safe}_lhs_rhs" in dot


def test_unknown_rule_mode_raises():
    (form,) = parse('(rule x () :match a :assert b :why "x")')
    with pytest.raises(ValueError):
        render_rule(form, mode="bogus")


# ── constraint-scope diagram ───────────────────────────────────────

def test_constraints_surface_structural_properties():
    forms = parse(ZEBRA2.read_text(encoding="utf-8"))
    dot = render_constraints(forms)
    # every *-loc relation carries the bijective badge …
    assert dot.count("bijective") == 5
    assert "«bijective»" in dot
    # … next-to is symmetric, is-a* transitive, and `includes` is an edge
    assert "symmetric" in dot and "transitive" in dot
    assert 'label="includes"' in dot
    # is-a relation facts (data) are NOT mistaken for constraints
    assert "Attribute" not in dot


def test_constraints_zebra_classic():
    forms = parse(ZEBRA.read_text(encoding="utf-8"))
    dot = render_constraints(forms)
    assert "type-exclusivity" in dot
    assert 'label="implies"' in dot
    # condition-(1) right-of facts (head is a declared relation) excluded
    assert "House-2" not in dot


@pytest.mark.skipif(not _HAVE_DOT, reason="graphviz `dot` not installed")
def test_constraints_dot_parses():
    for path in (ZEBRA, ZEBRA2):
        assert _dot_parses(render_constraints(parse(path.read_text(encoding="utf-8"))))


# ── CLI ────────────────────────────────────────────────────────────

def test_cli_render_rules(capsys: pytest.CaptureFixture[str]):
    rc = main(["render", "rules", str(ZEBRA2)])
    assert rc == 0
    out = capsys.readouterr().out
    assert out.count("digraph") == len(_rules_of(ZEBRA2))


# `co-located` — the render commands are faithful static file views (no import
# resolution), so the by-name target must be a rule that stays defined INLINE.
# symmetric/transitive/includes (S1.8.A5-tail) and now functional/injective/
# bijective-properties (S1.8a.f20) all arrive via `(import std.algebra …)`, so
# they're invisible to the static renderer. `co-located` is puzzle-local — it
# exceeds relation algebra's 3-variable ceiling (4 params), so it's destined to
# stay inline — a stable target. The hyphen sanitizes to `_` in the DOT id.
def test_cli_render_rule_by_name(capsys: pytest.CaptureFixture[str]):
    rc = main(["render", "rule", str(ZEBRA2), "--name", "co-located"])
    assert rc == 0
    out = capsys.readouterr().out
    assert "digraph rule_co_located_lhs_rhs" in out


def test_cli_render_rule_overlay(capsys: pytest.CaptureFixture[str]):
    rc = main(["render", "rule", str(ZEBRA2), "--name", "co-located",
               "--rule-mode", "overlay"])
    assert rc == 0
    out = capsys.readouterr().out
    assert "rule_co_located_overlay" in out
    assert "cluster_lhs" not in out


def test_cli_render_rule_missing_name(capsys: pytest.CaptureFixture[str]):
    rc = main(["render", "rule", str(ZEBRA2), "--name", "no-such-rule"])
    assert rc == 1
    assert "no rule named" in capsys.readouterr().err


def test_cli_render_constraints(capsys: pytest.CaptureFixture[str]):
    rc = main(["render", "constraints", str(ZEBRA2)])
    assert rc == 0
    assert "digraph constraints" in capsys.readouterr().out
