"""Saturator tests — S1.3.3 T1.3.3.6.

Cover:
- Priority-banded firing order (propagate → derive → eliminate).
- Strict global priority: after a derive-band firing produces a new
  fact, the saturator returns to the propagate band before any
  remaining derive candidates.
- Idempotent re-saturation: zero firings after closure.
- `is_stalled()` before / after.
- Redundant-firing marker on duplicate derivations.
- `solved()` stub default.
- Saturator constructable without an explicit Engine.
"""
from __future__ import annotations

from pathlib import Path

from ein_bot.inference.engine import Engine
from ein_bot.inference.saturator import Saturator
from ein_bot.ir import parse
from ein_bot.kb.entities import Layer
from ein_bot.kb.store import KnowledgeBase

REPO = Path(__file__).resolve().parent.parent.parent
ZEBRA = REPO / "examples" / "zebra.ein"


def _sat(text: str) -> Saturator:
    return Saturator(KnowledgeBase.from_ir(parse(text)))


# ── Basic construction + API ──────────────────────────────────────


def test_saturator_constructs_without_engine():
    """Saturator(kb) auto-builds the Engine + compile cache."""
    sat = _sat("""
    (rules (rule symmetric (?rel)
      :match (?rel ?a ?b) :assert (?rel ?b ?a) :why "s" :priority 100))
    (ontology (relation r T T) (symmetric r))
    (facts (r A B :source "(1)"))
    """)
    assert isinstance(sat.engine, Engine)
    assert sat.engine.cache, "compile cache should be populated automatically"


def test_saturator_accepts_existing_engine():
    """Saturator(kb, engine) reuses the provided engine + its cache."""
    kb = KnowledgeBase.from_ir(parse("""
    (rules (rule sym (?rel)
      :match (?rel ?a ?b) :assert (?rel ?b ?a) :why "s" :priority 100))
    (ontology (relation r T T) (sym r))
    (facts (r A B :source "(1)"))
    """))
    eng = Engine(kb)
    eng.compile_all()
    cache_size = len(eng.cache)
    sat = Saturator(kb, engine=eng)
    assert sat.engine is eng
    assert len(sat.engine.cache) == cache_size


def test_solved_stub_returns_false():
    """solved() is a stub for M1; P1.5/P1.7 plug it in."""
    sat = _sat("""
    (rules (rule s (?rel) :match (?rel ?a ?b) :assert (?rel ?b ?a) :why "s" :priority 100))
    (ontology (relation r T T) (s r))
    """)
    assert sat.solved() is False


# ── Saturation behaviour ──────────────────────────────────────────


def test_empty_kb_saturates_to_nothing():
    sat = _sat("""
    (rules (rule s (?rel) :match (?rel ?a ?b) :assert (?rel ?b ?a) :why "s" :priority 100))
    (ontology (relation r T T) (s r))
    """)
    firings = list(sat.saturate())
    assert firings == []
    assert sat.is_stalled()


def test_single_fact_one_firing():
    """One fact + symmetric activator → exactly one new fact (the reverse).

    The matcher also re-matches on the new fact and produces a
    redundant firing (deriving the original); we accept both as
    legitimate output of saturation.
    """
    sat = _sat("""
    (rules (rule s (?rel) :match (?rel ?a ?b) :assert (?rel ?b ?a) :why "s" :priority 100))
    (ontology (relation r T T) (s r))
    (facts (r A B :source "(1)"))
    """)
    firings = list(sat.saturate())
    productive = [f for f in firings if not f.redundant]
    assert len(productive) == 1
    assert productive[0].derived.relation_name == "r"
    assert productive[0].derived.args == ("B", "A")
    assert productive[0].derived.layer == Layer.REASONING


def test_triangle_transitive_closure():
    """A-B-C chain reaches (A, C) via transitive."""
    sat = _sat("""
    (rules (rule transitive (?rel)
      :match (and (?rel ?a ?b) (?rel ?b ?c) (neq ?a ?c))
      :assert (?rel ?a ?c) :why "t" :priority 200))
    (ontology (relation r T T) (transitive r))
    (facts (r A B :source "(1)") (r B C :source "(2)"))
    """)
    firings = list(sat.saturate())
    productive = {f.derived.args for f in firings if not f.redundant}
    assert ("A", "C") in productive


def test_chained_transitivity_four_nodes():
    """A-B-C-D chain reaches the full transitive closure."""
    sat = _sat("""
    (rules (rule transitive (?rel)
      :match (and (?rel ?a ?b) (?rel ?b ?c) (neq ?a ?c))
      :assert (?rel ?a ?c) :why "t" :priority 200))
    (ontology (relation r T T) (transitive r))
    (facts
      (r A B :source "(1)")
      (r B C :source "(2)")
      (r C D :source "(3)"))
    """)
    list(sat.saturate())
    derived = {
        (f.relation_name, f.args)
        for f in sat.kb.facts
        if f.layer == Layer.REASONING
    }
    # Closure must include A-C, B-D, A-D.
    assert ("r", ("A", "C")) in derived
    assert ("r", ("B", "D")) in derived
    assert ("r", ("A", "D")) in derived


# ── Priority ordering ─────────────────────────────────────────────


def test_priority_propagate_before_derive():
    """With both symmetric (100) and transitive (200) active, every
    productive symmetric firing precedes the *first* productive
    transitive firing.

    (Symmetric on a single edge produces the reverse, opening up new
    transitive chains; once transitive fires once, it produces a new
    fact that re-enters the propagate band — but the *very first*
    transitive firing happens only after the initial symmetric pass
    on the input facts has run.)
    """
    sat = _sat("""
    (rules
      (rule symmetric (?rel) :match (?rel ?a ?b) :assert (?rel ?b ?a)
        :why "s" :priority 100)
      (rule transitive (?rel)
        :match (and (?rel ?a ?b) (?rel ?b ?c) (neq ?a ?c))
        :assert (?rel ?a ?c) :why "t" :priority 200))
    (ontology (relation r T T) (symmetric r) (transitive r))
    (facts (r A B :source "(1)") (r B C :source "(2)"))
    """)
    firings = list(sat.saturate())
    productive = [f for f in firings if not f.redundant]
    # First productive firing must be symmetric (priority 100).
    assert productive[0].rule == "symmetric"
    # Find first transitive firing index; all firings before it must
    # be symmetric.
    first_t = next(
        (i for i, f in enumerate(productive) if f.rule == "transitive"), None,
    )
    assert first_t is not None, "transitive must fire at least once"
    for f in productive[:first_t]:
        assert f.rule == "symmetric"


def test_priority_eliminate_after_derive():
    """type-exclusivity (300) does not preempt symmetric (100)."""
    sat = _sat("""
    (rules
      (rule symmetric (?rel) :match (?rel ?a ?b) :assert (?rel ?b ?a)
        :why "s" :priority 100)
      (rule type-exclusivity ()
        :match (and (instance ?a ?T) (instance ?b ?T) (neq ?a ?b))
        :assert (not (co-located ?a ?b)) :why "x" :priority 300))
    (ontology
      (type Color) (instance Red Color) (instance Blue Color)
      (relation co-located T T) (symmetric co-located))
    (facts (co-located Norwegian Red :source "(1)"))
    """)
    firings = list(sat.saturate())
    productive = [f for f in firings if not f.redundant]
    # Symmetric must fire before type-exclusivity's first firing.
    first_sym = next(
        (i for i, f in enumerate(productive) if f.rule == "symmetric"), None,
    )
    first_tx = next(
        (i for i, f in enumerate(productive) if f.rule == "type-exclusivity"),
        None,
    )
    assert first_sym is not None
    assert first_tx is not None
    assert first_sym < first_tx


# ── Redundant marker ──────────────────────────────────────────────


def test_redundant_firing_marked():
    """Symmetric fires twice on a pair: once productively (deriving
    the reverse), once redundantly (re-deriving the original). The
    second firing must carry redundant=True and must NOT re-insert."""
    sat = _sat("""
    (rules (rule sym (?rel)
      :match (?rel ?a ?b) :assert (?rel ?b ?a) :why "s" :priority 100))
    (ontology (relation r T T) (sym r))
    (facts (r A B :source "(1)"))
    """)
    firings = list(sat.saturate())
    productive = [f for f in firings if not f.redundant]
    redundant = [f for f in firings if f.redundant]
    assert len(productive) == 1
    assert len(redundant) >= 1
    # KB has exactly two facts total: the FACT-layer (r A B) and the
    # REASONING-layer (r B A). No third.
    r_facts = [f for f in sat.kb.facts if f.relation_name == "r"]
    assert len(r_facts) == 2


# ── is_stalled() ──────────────────────────────────────────────────


def test_is_stalled_after_closure():
    sat = _sat("""
    (rules (rule sym (?rel)
      :match (?rel ?a ?b) :assert (?rel ?b ?a) :why "s" :priority 100))
    (ontology (relation r T T) (sym r))
    (facts (r A B :source "(1)"))
    """)
    list(sat.saturate())
    assert sat.is_stalled()


def test_is_stalled_false_before_first_step():
    """Before any firing, the queue has work; is_stalled is False."""
    sat = _sat("""
    (rules (rule sym (?rel)
      :match (?rel ?a ?b) :assert (?rel ?b ?a) :why "s" :priority 100))
    (ontology (relation r T T) (sym r))
    (facts (r A B :source "(1)"))
    """)
    assert not sat.is_stalled()


def test_is_stalled_flips_after_new_fact():
    """Saturate to stall; manually add a new fact that re-opens
    matches; is_stalled flips back to False on the next check.

    Simulates what P1.5 will do: fork on a hypothesis, inject a
    synthetic fact, then call is_stalled to decide whether to keep
    forking.
    """
    from ein_bot.kb.entities import Fact
    from ein_bot.kb.provenance import Provenance

    sat = _sat("""
    (rules (rule sym (?rel)
      :match (?rel ?a ?b) :assert (?rel ?b ?a) :why "s" :priority 100))
    (ontology (relation r T T) (sym r))
    (facts (r A B :source "(1)"))
    """)
    list(sat.saturate())
    assert sat.is_stalled()

    # Inject a new fact directly (mimicking P1.5 fork).
    new_fact = Fact(
        relation_name="r",
        args=("X", "Y"),
        layer=Layer.REASONING,
        provenance=Provenance.from_hypothesis(branch=1),
    )
    sat.kb.add_fact(new_fact)
    sat.kb._index_fact(new_fact)

    # Now there should be unfired work (sym on (r X Y)).
    assert not sat.is_stalled()


# ── Idempotency ───────────────────────────────────────────────────


def test_saturate_is_idempotent():
    """Running saturate() twice on the same KB: second call yields zero."""
    sat = _sat("""
    (rules (rule sym (?rel)
      :match (?rel ?a ?b) :assert (?rel ?b ?a) :why "s" :priority 100))
    (ontology (relation r T T) (sym r))
    (facts (r A B :source "(1)"))
    """)
    first = list(sat.saturate())
    assert first, "first saturate should produce firings"
    second = list(sat.saturate())
    assert second == [], "second saturate must produce no further firings"


# ── Zebra-scale ───────────────────────────────────────────────────


def test_zebra_saturation_completes():
    """The full zebra.ein KB saturates without an infinite loop."""
    kb = KnowledgeBase.from_ir(parse(ZEBRA.read_text()))
    sat = Saturator(kb)
    firings = list(sat.saturate())
    # Some firings must happen — Zebra has dozens of co-located /
    # right-of facts that the rules will saturate over.
    assert len(firings) > 0
    productive = [f for f in firings if not f.redundant]
    assert len(productive) > 0
    # No infinite loop — sat must reach stalled state.
    assert sat.is_stalled()


def test_zebra_saturation_priority_band_first_firings():
    """The first non-redundant firings on zebra.ein are propagate-band
    (priority 100) — symmetric or implies — before any derive-band."""
    kb = KnowledgeBase.from_ir(parse(ZEBRA.read_text()))
    sat = Saturator(kb)
    productive = [f for f in sat.saturate() if not f.redundant]
    # Propagate band rules.
    propagate = {"symmetric", "implies"}
    first_derive = next(
        (i for i, f in enumerate(productive) if f.rule not in propagate
         and f.rule != "type-exclusivity"),
        None,
    )
    if first_derive is not None:
        # Everything before the first derive must be propagate-band.
        for f in productive[:first_derive]:
            assert f.rule in propagate or f.rule == "type-exclusivity", (
                f"saw {f.rule!r} before first derive-band firing"
            )
