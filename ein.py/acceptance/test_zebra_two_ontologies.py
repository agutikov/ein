"""S1.22.1a acceptance — one puzzle, two ontologies, one model.

`examples/zebra2.ein` and `examples/zebra.ein` encode the SAME Zebra puzzle
over deliberately different vocabularies:

- **zebra2** — five typed projections (`color-loc : Color → House`, …), each
  declared `(bijective …)`, cross-attribute clues restated as 4- and 5-argument
  `co-located` / `adjacent-via` activator facts.
- **zebra** — one generic `co-located` equivalence over all 30 attribute
  values, whose classes *are* the houses; clues are ordinary facts
  (`(co-located Englishman Red)`, `(right-of Green Ivory)`), and the inference
  comes from one type-scoped `(slot-partition …)` declaration plus two
  `(slot-spatial …)` ones (`std.slots`).

What this gate pins is that the difference is *ontological, not semantic*:
both must reach the same unique model, and neither may be the only one that
works. That is the whole reason the second encoding is kept — it is the only
way to tell which of Ein's reasoning power is general and which is an artefact
of how zebra2 happens to be written.

Lives outside `ein.py/tests/` with the rest of the acceptance gate — the
exhaustive `zebra.ein` solve is ~21 s under PyPy (zebra2: ~9 s) and much
slower under CPython. See `test_zebra_three_classes.py` for how to run it.
"""
from __future__ import annotations

import sys
from pathlib import Path

from ein.inference.monotonic import ProgressDumper, solve
from ein.inference.verdict import Solution
from ein.ir import parse
from ein.kb import KnowledgeBase

REPO = Path(__file__).resolve().parents[2]
EXAMPLES = REPO / "examples"
ZEBRA = EXAMPLES / "zebra.ein"
ZEBRA2 = EXAMPLES / "zebra2.ein"

# The Wikipedia answer as (attribute, house) pairs — vocabulary-independent.
ANSWER = (
    ("Water", "House-1"),
    ("Zebra", "House-5"),
    ("Norwegian", "House-1"),
    ("Japanese", "House-5"),
)

# The full 25-cell grid, as house → its five attribute values. Read off the
# canonical solution; used to check the generic-link model completely, since
# `co-located` has no per-attribute relation to count cells in.
GRID = {
    "House-1": ("Yellow", "Norwegian", "Kools", "Water", "Fox"),
    "House-2": ("Blue", "Ukrainian", "Chesterfields", "Tea", "Horse"),
    "House-3": ("Red", "Englishman", "Old_Gold", "Milk", "Snail"),
    "House-4": ("Ivory", "Spaniard", "Lucky_Strike", "Juice", "Dog"),
    "House-5": ("Green", "Japanese", "Parliament", "Coffee", "Zebra"),
}


def _solve(path: Path, label: str, **kw):
    kb = KnowledgeBase.from_ir(parse(path.read_text()))
    dumper = ProgressDumper(label=label, progress_every=10, stream=sys.stderr)
    return solve(kb, dumper=dumper, **kw)


def _co_located(kb: KnowledgeBase, a: str, b: str) -> bool:
    """`co-located` is symmetric, so accept either argument order."""
    return (
        kb._fact_by_id("co-located", (a, b)) is not None
        or kb._fact_by_id("co-located", (b, a)) is not None
    )


# ── the generic-link ontology solves ──────────────────────────────


def test_zebra_generic_link_is_unique_solution():
    """zebra.ein → exactly one solution node, the full grid, certified
    unique by an exhausted search — the same verdict shape as zebra2's."""
    verdict, stats = _solve(ZEBRA, "zebra (generic link, SOLVE/unique)")

    assert isinstance(verdict, Solution), (
        f"zebra.ein must be a unique Solution, got {type(verdict).__name__}"
    )
    assert stats.solution_nodes == 1, (
        f"k must be 1 (unique), got {stats.solution_nodes}"
    )
    assert stats.exhausted, "uniqueness requires an exhausted search"

    model = verdict.kb
    for attribute, house in ANSWER:
        assert _co_located(model, attribute, house), (
            f"missing answer fact: {attribute} in {house}"
        )


def test_zebra_generic_link_model_is_complete():
    """Every one of the 25 attribute values is placed, in the right house.

    The completeness check the generic encoding needs: with one relation
    spanning all attributes there is no per-attribute relation to count, so
    the grid is checked cell by cell.
    """
    verdict, _stats = _solve(ZEBRA, "zebra (generic link, grid)", stop_after=1)
    assert isinstance(verdict, Solution)
    model = verdict.kb
    missing = [
        (value, house)
        for house, values in GRID.items()
        for value in values
        if not _co_located(model, value, house)
    ]
    assert not missing, f"grid cells unplaced or misplaced: {missing}"


# ── the two ontologies agree ──────────────────────────────────────


def test_both_ontologies_reach_the_same_model():
    """The two encodings' answers agree, read through their own vocabularies.

    zebra2 states placement with five typed relations; zebra with one generic
    one. Project both onto (attribute, house) pairs and require equality on
    the full grid — that is what "two ontologies for one puzzle" has to mean
    to be worth keeping.
    """
    v1, _ = _solve(ZEBRA, "zebra (agreement)", stop_after=1)
    v2, _ = _solve(ZEBRA2, "zebra2 (agreement)", stop_after=1)
    assert isinstance(v1, Solution) and isinstance(v2, Solution)

    loc_rels = ("color-loc", "nation-loc", "drink-loc", "smoke-loc", "pet-loc")
    typed = {
        (f.args[0], f.args[1])
        for rel in loc_rels
        for f in v2.kb._facts_by_relation.get(rel, ())
        if len(f.args) == 2
    }
    assert len(typed) == 25, "zebra2's model must fill all 25 grid cells"

    disagreements = [
        pair for pair in sorted(typed) if not _co_located(v1.kb, *pair)
    ]
    assert not disagreements, (
        "the generic-link model disagrees with the typed model on: "
        f"{disagreements}"
    )


# ── the CLI answers in words, from the generic encoding too ───────


def test_cli_solve_zebra_emits_answer_in_words(capsys):
    """`ein solve zebra.ein` exits 0 and prints the canonical answer in
    English — rendered from the file's `:goal-text` and relation `:why`
    templates, not from anything hardcoded per encoding."""
    from ein.cli import main

    rc = main(["solve", str(ZEBRA)])
    assert rc == 0
    out = capsys.readouterr().out.lower()
    assert "norwegian" in out and "water" in out
    assert "japanese" in out and "zebra" in out
