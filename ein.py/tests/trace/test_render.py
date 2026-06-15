"""Markdown trace renderer tests — S1.6.4 T1.6.4.7.

Covers `ein.trace`:

- a synthetic 3-step trace renders to a committed golden markdown;
- `--reorder` clusters by entity with the *same* set of steps;
- refuted hypotheses render as `<details>` reductio blocks;
- each step's inline `dot` slice is valid DOT (Graphviz);
- the `(trace …)` AST round-trips through the parser;
- `linearize` on a real gaps_solve verdict yields a coherent trace;
- the `solve --trace` CLI writes markdown.
"""
from __future__ import annotations

import os
import re
import shutil
import subprocess
from pathlib import Path

import pytest

from ein.cli import main
from ein.inference.monotonic import contradictions_solve, gaps_solve
from ein.ir import parse
from ein.kb import KnowledgeBase
from ein.trace import (
    Reductio,
    Trace,
    TraceStep,
    linearize,
    parse_trace_steps,
    render_markdown,
    trace_to_ir,
)

REPO = Path(__file__).resolve().parents[3]
BRANCHING = REPO / "examples" / "branching"
LATTICE = REPO / "examples" / "lattice"
GOLDEN = REPO / "ein.py" / "tests" / "golden" / "trace_3step.md"

_HAVE_DOT = shutil.which("dot") is not None


def _kb(path: Path) -> KnowledgeBase:
    return KnowledgeBase.from_ir(parse(path.read_text()))


# A deterministic synthetic trace (no diagrams → stable golden).
def _synthetic_trace() -> Trace:
    steps = [
        TraceStep(
            n=1, rule="from-condition", premises=(),
            derived=("nation-loc", ("Norwegian", "H1")),
            why="By condition (10), the Norwegian lives in the first house.",
            section="Norwegian", sources=("condition (10)",),
        ),
        TraceStep(
            n=2, rule="adjacent-via",
            premises=(("nation-loc", ("Norwegian", "H1")),),
            derived=("color-loc", ("Blue", "H2")),
            bindings={"V1": "Norwegian", "V2": "Blue"},
            why="The Norwegian's only neighbour is House-2, so Blue is there.",
            section="Blue",
        ),
        TraceStep(
            n=3, rule="domain-elimination",
            premises=(("not", (("color-loc", ("Blue", "H1")),)),),
            derived=("color-loc", ("Yellow", "H1")),
            why="Only Yellow remains for House-1.", section="Yellow",
        ),
    ]
    reductios = [
        Reductio(
            summary="Assumed {color-loc(Green, H1)} — contradicts condition (6) "
                    "— refuted (dead-post)",
            commitment="{color-loc(Green, H1)}",
            learned_clause="color-loc(Green, H1)", diagram=None,
        ),
    ]
    return Trace(
        steps=steps, reductios=reductios,
        summary="Solved in 3 steps; commitment ∅ (unconditional); "
                "1 solution(s), 1 refuted.",
        commitment="∅ (unconditional)", solved=True, n_solutions=1,
    )


# ── golden ─────────────────────────────────────────────────────────

def test_three_step_trace_matches_golden():
    md = render_markdown(_synthetic_trace(), diagrams=False)
    if os.environ.get("UPDATE_GOLDEN") or not GOLDEN.exists():
        GOLDEN.parent.mkdir(parents=True, exist_ok=True)
        GOLDEN.write_text(md, encoding="utf-8")
        if os.environ.get("UPDATE_GOLDEN"):
            pytest.skip("golden refreshed")
    assert md == GOLDEN.read_text(encoding="utf-8"), (
        "trace markdown diverged from golden; set UPDATE_GOLDEN=1 to refresh."
    )


# ── structure ──────────────────────────────────────────────────────

def test_steps_are_numbered_with_rule_headers():
    md = render_markdown(_synthetic_trace(), diagrams=False)
    assert "## Step 1 — `from-condition`" in md
    assert "## Step 2 — `adjacent-via`" in md
    assert "## Step 3 — `domain-elimination`" in md


def test_source_sentence_is_quoted():
    md = render_markdown(_synthetic_trace(), diagrams=False)
    # the from-condition step quotes its source (T1.6.4.5)
    assert "from condition (10)" in md


def test_reductio_renders_as_details_block():
    md = render_markdown(_synthetic_trace(), diagrams=False)
    assert "<details>" in md and "</details>" in md
    assert "contradicts condition (6)" in md
    assert "Lifted no-good: `color-loc(Green, H1)`" in md


def test_reorder_same_steps_different_grouping():
    trace = _synthetic_trace()
    engine = render_markdown(trace, mode="engine", diagrams=False)
    reordered = render_markdown(trace, mode="reorder", diagrams=False)
    assert engine != reordered
    # reorder groups by entity …
    assert "## About Norwegian" in reordered
    assert "## About Blue" in reordered
    # … but mentions every step exactly once (same set of steps)
    for n in (1, 2, 3):
        assert reordered.count(f"Step {n}") == 1


def test_render_markdown_rejects_bad_mode():
    with pytest.raises(ValueError, match="trace mode"):
        render_markdown(_synthetic_trace(), mode="bogus")


# ── AST round-trip ─────────────────────────────────────────────────

def test_trace_ast_round_trips():
    steps = _synthetic_trace().steps
    (form,) = parse(trace_to_ir(steps))
    back = parse_trace_steps(form)
    assert len(back) == len(steps)
    for b, s in zip(back, steps, strict=True):
        assert (b.n, b.rule, b.premises, b.derived, b.why) == (
            s.n, s.rule, s.premises, s.derived, s.why)
        assert b.bindings == s.bindings


def test_trace_control_chars_escaped_on_emit():
    """S1.7c.32 — a `why` (or string fact-arg) containing \\n / \\t must be
    EMITTED backslash-escaped, not as a raw control char. step_to_ir and the
    fact-arg escaper previously escaped only \\ and " (trace/ast.py), so such
    a string did NOT round-trip (the parser unescapes the full set). Asserts
    on emitted BYTES — a value round-trip alone is green pre-fix and proves
    nothing.
    """
    # `parse`, `TraceStep`, `parse_trace_steps`, `trace_to_ir` are
    # module-level; only the submodule-private escaper sites need importing.
    from ein.trace.ast import _fact_to_sexpr, step_to_ir
    step = TraceStep(
        n=1, rule="r", premises=(("p", ("a",)),), derived=("q", ("a",)),
        why='first\nsecond\twith "quote" and \\slash',
    )
    line = step_to_ir(step)
    # Emitted bytes: control chars escaped; no raw newline/tab in the output.
    assert r"\n" in line and r"\t" in line
    assert "\n" not in line and "\t" not in line
    # Whole trace round-trips the value exactly (parser unescapes).
    (form,) = parse(trace_to_ir([step]))
    (back,) = parse_trace_steps(form)
    assert back.why == step.why
    # The fact-arg escaper (ast.py:67) is fixed too.
    assert _fact_to_sexpr("note", ("a\nb",)) == r'(note "a\nb")'


# ── inline dot slices are valid ────────────────────────────────────

@pytest.mark.skipif(not _HAVE_DOT, reason="graphviz `dot` not installed")
def test_each_step_diagram_is_valid_dot():
    kb = _kb(BRANCHING / "04_two_levels.ein")
    verdict, _ = gaps_solve(kb, max_set_size=3, store_lattice=True)
    md = render_markdown(linearize(verdict, diagrams=True), diagrams=True)
    blocks = re.findall(r"```dot\n(.*?)\n```", md, re.DOTALL)
    assert blocks, "expected inline dot blocks"
    for block in blocks:
        res = subprocess.run(["dot", "-Tcanon"], input=block,
                             capture_output=True, text=True)
        assert res.returncode == 0, f"invalid DOT block:\n{block[:200]}"


# ── integration: real verdicts ─────────────────────────────────────

def test_linearize_real_gaps_solution():
    kb = _kb(BRANCHING / "04_two_levels.ein")
    verdict, _ = gaps_solve(kb, max_set_size=3, store_lattice=True)
    trace = linearize(verdict, diagrams=False)
    assert trace.solved
    assert trace.steps                      # has a derivation spine
    assert trace.n_solutions == 2           # two_levels has two solutions
    assert all(s.rule for s in trace.steps)


def test_linearize_real_contradictions_has_reductios():
    kb = _kb(LATTICE / "02_genuine_3set_death.ein")
    verdict, _ = contradictions_solve(kb, max_set_size=3, store_lattice=True)
    trace = linearize(verdict, diagrams=False)
    assert trace.reductios
    assert all("refuted" in r.summary for r in trace.reductios)


# ── --relevant prune (S1.6.5) ──────────────────────────────────────

def test_relevant_prune_reduces_and_dedupes():
    kb = _kb(BRANCHING / "04_two_levels.ein")
    verdict, _ = gaps_solve(kb, max_set_size=3, store_lattice=True)
    full = linearize(verdict, diagrams=False)
    pruned = linearize(verdict, diagrams=False, relevant=True)
    assert len(pruned.steps) < len(full.steps)        # the slice is smaller
    assert "pruned to" in pruned.summary
    # each kept step derives a distinct fact (first, non-redundant)
    derived = [s.derived for s in pruned.steps]
    assert len(derived) == len(set(derived))


def test_relevant_marks_conditional_under_hypothesis():
    kb = _kb(BRANCHING / "04_two_levels.ein")
    verdict, _ = gaps_solve(kb, max_set_size=3, store_lattice=True)
    pruned = linearize(verdict, diagrams=False, relevant=True)
    # branching/04's solution needs the commitment, so kept steps are
    # under the hypothesis → the render shows the divider.
    md = render_markdown(pruned, diagrams=False)
    assert any(s.conditional for s in pruned.steps)
    assert "## Under hypothesis —" in md


def test_cli_solve_relevant(capsys: pytest.CaptureFixture[str]):
    rc = main(["solve", str(BRANCHING / "04_two_levels.ein"),
               "--relevant", "--no-diagrams"])
    assert rc == 0
    assert "pruned to" in capsys.readouterr().out


# ── CLI ────────────────────────────────────────────────────────────

def test_cli_solve_writes_trace(tmp_path: Path, capsys: pytest.CaptureFixture[str]):
    out = tmp_path / "trace.md"
    rc = main(["solve", str(BRANCHING / "04_two_levels.ein"), "--trace", str(out)])
    assert rc == 0
    text = out.read_text(encoding="utf-8")
    assert text.startswith("# Solution trace")
    assert "## Step 1 —" in text


def test_cli_solve_no_diagrams(capsys: pytest.CaptureFixture[str]):
    rc = main(["solve", str(BRANCHING / "04_two_levels.ein"), "--no-diagrams"])
    assert rc == 0
    out = capsys.readouterr().out
    assert "# Solution trace" in out
    assert "```dot" not in out               # all dot blocks suppressed
