"""Per-rule demo tests — S1.3.2 T1.3.2.10.

Every `.ein` file under `examples/zebra/demos/<rule-name>/` is a
self-contained mini-puzzle that exercises one rule. Each demo's
directory name is the rule name. The test asserts that running the
engine on the demo's KB produces at least one firing whose `:rule`
field matches the directory name.

Deviation from S1.3.2 spec: the spec asks for "exactly one new
reasoning-layer fact". In practice, some rules legitimately fire
more than once on minimal inputs:

- `symmetric` matches its own conclusion (rule re-fires once over
  the derived fact, even though `add_fact` dedupes the result).
- `type-exclusivity` fires over every ordered pair of distinct
  same-type instances — N(N-1) firings for N instances.

The `≥ 1` relaxation preserves the pedagogical intent ("this demo
shows rule X firing") without forcing artificial single-firing
contortions on rules that are naturally multi-firing.
"""
from __future__ import annotations

from pathlib import Path

import pytest

from ein_bot.inference.engine import Engine
from ein_bot.ir import parse
from ein_bot.kb.store import KnowledgeBase

REPO = Path(__file__).resolve().parent.parent.parent
DEMOS_DIR = REPO / "examples" / "zebra" / "demos"

# Collect every .ein under demos/, sorted for stable test order.
DEMO_PATHS = sorted(DEMOS_DIR.rglob("*.ein"))


def _demo_id(path: Path) -> str:
    """Pretty test id: `<rule>/<scenario>`."""
    return f"{path.parent.name}/{path.stem}"


def test_demos_directory_layout():
    """Exactly 7 rule subdirectories, each with 3 named scenarios = 21 files."""
    subdirs = sorted(p.name for p in DEMOS_DIR.iterdir() if p.is_dir())
    assert subdirs == [
        "hypothesis-contradiction", "implies", "square-bwd",
        "square-fwd", "symmetric", "transitive", "type-exclusivity",
    ]
    assert len(DEMO_PATHS) == 21, \
        f"expected 21 demo files (7 rules x 3 scenarios), got {len(DEMO_PATHS)}"
    for sub in subdirs:
        eins = sorted(p.name for p in (DEMOS_DIR / sub).glob("*.ein"))
        assert len(eins) == 3, \
            f"{sub}/ should have 3 demos; got {len(eins)}: {eins}"


@pytest.mark.parametrize("path", DEMO_PATHS, ids=_demo_id)
def test_demo_fires_named_rule(path: Path):
    """The demo's directory name is the rule name; saturation must
    produce ≥ 1 firing of that rule."""
    rule_name = path.parent.name
    kb = KnowledgeBase.from_ir(parse(path.read_text()))
    eng = Engine(kb)
    eng.compile_all()
    firings = list(eng.saturate())
    matched = [f for f in firings if f.rule == rule_name]
    assert matched, (
        f"{_demo_id(path)}: rule {rule_name!r} did not fire. "
        f"Firings observed: {[f.rule for f in firings]}"
    )


@pytest.mark.parametrize("path", DEMO_PATHS, ids=_demo_id)
def test_demo_has_query_block(path: Path):
    """Every demo carries a `(query :mode solve :goal …)` block.

    The goal pattern names the derived fact the demo's rule is
    expected to produce; consumers (P1.5 hypothesis loop, P1.6
    trace renderer) can run a demo and check the query against the
    saturated KB.
    """
    kb = KnowledgeBase.from_ir(parse(path.read_text()))
    assert kb.query is not None, f"{_demo_id(path)}: missing (query …) block"
    # The query has at least :mode and :goal kw_pairs.
    keys = {
        kp.key.name for kp in kb.query.kw_pairs
        if hasattr(kp, "key")
    }
    assert "mode" in keys, f"{_demo_id(path)}: query missing :mode"
    assert "goal" in keys, f"{_demo_id(path)}: query missing :goal"


@pytest.mark.parametrize("path", DEMO_PATHS, ids=_demo_id)
def test_demo_derived_fact_lands_in_reasoning(path: Path):
    """At least one of the named rule's firings must produce a new
    REASONING-layer fact.

    Some rules legitimately re-fire on their own output: `symmetric`
    matches (rel A B), derives (rel B A), then matches (rel B A) and
    tries to derive (rel A B) — which already exists in the FACT
    layer, so add_fact's dedupe returns the FACT-layer instance. The
    *first* firing lands in REASONING; subsequent firings may bounce
    back to FACT via dedupe. Either way, the rule is doing useful
    work; this test just confirms at least one REASONING-layer
    derivation exists.
    """
    from ein_bot.kb.entities import Layer

    rule_name = path.parent.name
    kb = KnowledgeBase.from_ir(parse(path.read_text()))
    eng = Engine(kb)
    eng.compile_all()
    firings = list(eng.saturate())
    matched = [f for f in firings if f.rule == rule_name]
    assert matched
    reasoning_firings = [f for f in matched if f.derived.layer == Layer.REASONING]
    assert reasoning_firings, (
        f"{_demo_id(path)}: rule fired but no firing produced a "
        f"REASONING-layer fact. Layers seen: "
        f"{[f.derived.layer.value for f in matched]}"
    )
