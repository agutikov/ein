"""Parse + load smoke test for the three canonical Zebra files — S1.7.1 T1.7.1.4.

The P1.7 acceptance gate forks the canonical encoding into two
three-task-class variants. This test pins the structural contract the
later trace / GAPS / CONTRADICTIONS stages (S1.7.3) rely on:

- :file:`examples/zebra2.ein` — canonical B1 encoding (SOLVE), the
  baseline the variants fork from.
- :file:`examples/zebra2-minus-15.ein` — GAPS fixture: ``zebra2`` minus
  condition (15), the only fact pinning Blue at House-2; the puzzle goes
  under-determined.
- :file:`examples/ein-bugs/zebra2-bad.ein` — CONTRADICTIONS fixture:
  ``zebra2`` plus an injected ``(color-loc Green House-1)`` that collides
  with condition (6)'s spatial endpoint.

Both variants must stay **thin diffs** of the canonical: identical
ontology / relations / rules, differing from ``zebra2`` by exactly the
one fact dropped (minus-15) or added (bad). Asserting the diff relatively
keeps the test in sync if the canonical encoding evolves; the absolute
landmark counts catch accidental drift in the canonical itself.

The solving behaviour (minus-15 ⇒ ambiguous, bad ⇒ contradiction) is the
*phase-level* acceptance — wired in S1.7.3 via the CLI, gated for the
slow runs — not exercised here; this stage owns only parse + load + shape.
"""
from __future__ import annotations

import os
from pathlib import Path

import pytest

from ein.inference.contradiction import ContradictionDetector
from ein.inference.saturator import Saturator
from ein.ir import IRParseError, parse
from ein.kb.entities import Layer
from ein.kb.store import KnowledgeBase

REPO = Path(__file__).resolve().parents[3]
EXAMPLES = REPO / "examples"

ZEBRA2 = EXAMPLES / "zebra2.ein"
MINUS_15 = EXAMPLES / "zebra2-minus-15.ein"
BAD = EXAMPLES / "ein-bugs" / "zebra2-bad.ein"

# Condition (15): the lone fact pinning Blue at House-2 — dropped in
# the GAPS fixture.
COND_15 = ("adjacent-via", ("next-to", "nation-loc", "Norwegian", "color-loc", "Blue"))
# The injected self-contradictory fact in the CONTRADICTIONS fixture.
INJECTED = ("color-loc", ("Green", "House-1"))


def test_zebra2_variants_match_generator():
    """The minus-15 / bad fixtures are *generated* from zebra2.ein by
    examples/gen_zebra2_variants.py (so a zebra2 rule change can't silently
    drift them — the S1.8a.f20 lesson). Fail loudly + tell the maintainer to
    regenerate if the on-disk copies are stale."""
    import subprocess
    import sys

    gen = EXAMPLES / "gen_zebra2_variants.py"
    proc = subprocess.run(
        [sys.executable, str(gen), "--check"], capture_output=True, text=True)
    assert proc.returncode == 0, (
        (proc.stderr or proc.stdout)
        + "\n  → run: python3 examples/gen_zebra2_variants.py")


def _load(path: Path) -> KnowledgeBase:
    return KnowledgeBase.from_ir(parse(path.read_text(), filename=str(path)))


def _has(kb: KnowledgeBase, relation: str, args: tuple, *, layer: Layer = Layer.FACT) -> bool:
    return any(
        f.relation_name == relation and f.args == args and f.layer is layer
        for f in kb.facts
    )


# ── All three load ────────────────────────────────────────────────


#: The nine `(relation …)` signatures declared in the B1 ontology — the
#: five *-loc bijections, the two spatial relations, and the is-a pair.
#: (``kb.relations`` also holds auto-registered fact/rule heads such as
#: ``co-located`` / ``bijective`` with ``declared=False``; those are
#: incidental, so the contract is on the *declared* set.)
DECLARED_RELATIONS = frozenset({
    "is-a", "is-a*", "color-loc", "nation-loc", "drink-loc",
    "smoke-loc", "pet-loc", "right-of", "next-to",
})


@pytest.mark.parametrize("path", [ZEBRA2, MINUS_15, BAD], ids=lambda p: p.name)
def test_canonical_files_parse_and_load(path: Path):
    """Each canonical file parses cleanly and loads to a KB carrying the
    nine declared B1 relation signatures and a query block."""
    assert path.exists(), f"missing fixture: {path}"
    kb = _load(path)
    declared = {name for name, rel in kb.relations.items() if rel.declared}
    assert declared == DECLARED_RELATIONS
    assert kb.query is not None
    # Every file carries the same inlined rule library + the `guess`
    # hrule; a non-trivial rule count guards against a truncated load.
    assert len(kb.rules) > 20


# ── Thin-diff contract ────────────────────────────────────────────


def test_zebra2_landmark_counts():
    """Canonical baseline: 18 FACT-layer facts (the 15 numbered
    conditions; (1) expands to 4 right-of facts). Anchors the relative
    diffs below — if the encoding changes shape this fails loudly so the
    variants get re-synced."""
    z = _load(ZEBRA2)
    assert len(z.fact_layer()) == 18
    assert _has(z, *COND_15)
    assert not _has(z, *INJECTED)


def test_minus_15_is_zebra2_minus_one_fact():
    """GAPS fixture: identical schema + rules, exactly one fewer
    FACT-layer fact — condition (15) is gone, nothing else changed."""
    z, m = _load(ZEBRA2), _load(MINUS_15)
    assert m.relations.keys() == z.relations.keys()
    assert m.rules.keys() == z.rules.keys()
    assert len(m.ontology()) == len(z.ontology())
    assert len(m.fact_layer()) == len(z.fact_layer()) - 1
    assert not _has(m, *COND_15)
    # The only difference is condition (15): every other FACT survives.
    z_facts = {(f.relation_name, f.args) for f in z.fact_layer()}
    m_facts = {(f.relation_name, f.args) for f in m.fact_layer()}
    assert z_facts - m_facts == {COND_15}
    assert m_facts - z_facts == set()


def test_bad_is_zebra2_plus_one_fact():
    """CONTRADICTIONS fixture: identical schema + rules, exactly one
    extra FACT-layer fact — the injected ``(color-loc Green House-1)``."""
    z, b = _load(ZEBRA2), _load(BAD)
    assert b.relations.keys() == z.relations.keys()
    assert b.rules.keys() == z.rules.keys()
    assert len(b.ontology()) == len(z.ontology())
    assert len(b.fact_layer()) == len(z.fact_layer()) + 1
    assert _has(b, *INJECTED)
    b_facts = {(f.relation_name, f.args) for f in b.fact_layer()}
    z_facts = {(f.relation_name, f.args) for f in z.fact_layer()}
    assert b_facts - z_facts == {INJECTED}
    assert z_facts - b_facts == set()


def test_injected_fact_carries_its_source():
    """The injection is provenance-tagged so an unsat-core walk can name
    it (acceptance: a 2-3 edge core *including the injected fact*)."""
    b = _load(BAD)
    injected = [
        f for f in b.facts
        if f.relation_name == INJECTED[0] and f.args == INJECTED[1]
    ]
    assert len(injected) == 1
    assert injected[0].source == "injected contradiction"


# ── Malformed-variant rejection ───────────────────────────────────


def _has_contradiction_during_saturation(kb: KnowledgeBase) -> bool:
    """Step d=0 (root) saturation, stopping at the *first* contradiction.

    A contradictory KB never reaches a clean fixed point cheaply — the
    injected fact cascades both ``X`` and ``(not X)`` across every
    co-located / adjacent chain — so saturating to quiescence and only
    then scanning is wasteful. Break as soon as the detector flags a
    pair; for a satisfiable KB no break ever fires and this runs the
    full saturation (still fast)."""
    det = ContradictionDetector(kb)
    if det.has_contradiction():
        return True
    for _ in Saturator(kb).saturate():
        if det.has_contradiction():
            return True
    return False


#: The two checks below run real d=0 saturation on the full puzzle
#: (~6 s / ~4 s on CPython). They validate the fixtures are fit for
#: purpose — the *solving* acceptance proper is S1.7.3 — so they ride the
#: same EIN_RUN_SLOW gate as the other zebra-solve tests to keep the
#: default suite under 30 s. (Run via PyPy or ``EIN_RUN_SLOW=1``.)
_slow = pytest.mark.skipif(
    not os.environ.get("EIN_RUN_SLOW"),
    reason="root saturation on full zebra2 (~6s CPython); set EIN_RUN_SLOW=1 or run via PyPy",
)


@_slow
def test_bad_is_unsat_at_root_saturation():
    """The injected ``(color-loc Green House-1)`` makes the puzzle UNSAT
    without any hypothesis: condition (6)'s ``adjacent-via-endpoint-bwd``
    derives ``(not (color-loc Green House-1))``, and the co-located
    bridge for condition (4) (Coffee ↔ Green) carries both the positive
    and the negative onto ``drink-loc Coffee House-1`` — a same-layer
    pair the detector flags at d=0. This is what lets
    ``contradictions_solve`` return a tight 2-3 edge core fast."""
    assert _has_contradiction_during_saturation(_load(BAD))


@_slow
def test_zebra2_has_no_root_contradiction():
    """Control: the canonical puzzle is satisfiable, so d=0 saturation
    alone produces no contradiction (it needs hypothesis branching to
    finish) — confirming the injected fact, not the shared encoding, is
    what breaks ``zebra2-bad``."""
    assert not _has_contradiction_during_saturation(_load(ZEBRA2))


def test_malformed_zebra_variant_is_rejected():
    """A corrupted condition fact (unbalanced parens) fails to parse —
    the rejection path the loader leans on for hand-edited variants."""
    text = ZEBRA2.read_text().replace(
        '(nation-loc Norwegian  House-1                                  :source "condition (10)")',
        '(nation-loc Norwegian  House-1                                  :source "condition (10)"',
    )
    with pytest.raises(IRParseError):
        parse(text, filename="zebra2-malformed.ein")
