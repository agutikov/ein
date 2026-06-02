"""Commitment-set primitive tests — S1.5b.3 T1.5b.3.2.

Pins :func:`ein_bot.inference.commitment.try_commitment_set`
across the trichotomy (alive / dead-pre / dead-post) + the
unconditional-fact extraction (the soundness-critical novel
piece) + isolation between two calls + the empty-commitment
sentinel.
"""
from __future__ import annotations

from ein_bot.inference.commitment import (
    CommitmentSetResult,
    try_commitment_set,
)
from ein_bot.inference.saturator import Saturator
from ein_bot.ir import parse
from ein_bot.kb.entities import Fact, Layer
from ein_bot.kb.provenance import Provenance
from ein_bot.kb.store import KnowledgeBase


def _kb(text: str) -> KnowledgeBase:
    return KnowledgeBase.from_ir(parse(text))


def _put(kb: KnowledgeBase, fact: Fact) -> Fact:
    """add_fact + _index_fact in one call (mirrors the helper in
    test_contradiction)."""
    stored = kb.add_fact(fact)
    kb._index_fact(stored)
    return stored


def _ids(facts) -> set[tuple[str, tuple]]:
    """Project facts to their (relation_name, args) FactIds for
    set-based assertions."""
    return {(f.relation_name, f.args) for f in facts}


# ── Alive: 1-element commitment, derivation is conditional ─────────


def test_alive_one_element_conditional_derivation():
    """A 1-element commitment whose hypothesis triggers a rule
    derivation. The derived fact's chain reaches the hypothesis, so
    it is conditional (NOT in unconditional_facts), but it IS in kb.
    """
    kb = _kb("""
    (rule swap ()
      :match (target ?x ?y) :assert (other ?y ?x)
      :why "swap target → other" :priority 100)
    (type T)
    (relation target T T)
    (relation other T T)
    (instance c T) (instance d T)
    
    """)
    commitment = (("target", ("c", "d")),)
    result = try_commitment_set(kb, commitment)

    assert isinstance(result, CommitmentSetResult)
    assert result.kind == "alive"
    assert _ids(result.hypothesis_facts) == {("target", ("c", "d"))}
    assert result.unconditional_facts == ()  # (other d c) is conditional
    # kb DOES contain the derived (other d c) — just flagged conditional.
    assert ("other", ("d", "c")) in _ids(result.kb.facts)


# ── Alive: derivation is unconditional ────────────────────────────


def test_alive_unconditional_derivation_from_root_facts():
    """A root fact + symmetric rule produces a new fact whose chain
    is grounded entirely in root facts — even though a hypothesis
    is committed in the same fork. Verifies _is_unconditional's
    "chain doesn't touch the commitment" check.
    """
    kb = _kb("""
    (rule sym-r ()
      :match (r ?x ?y) :assert (r ?y ?x)
      :why "symmetric r" :priority 100)
    (type T)
    (relation r T T)
    (relation h T)
    (instance a T) (instance b T)
    (r a b :source "(1)")
    """)
    commitment = (("h", ("a",)),)
    result = try_commitment_set(kb, commitment)
    assert result.kind == "alive"
    # (r b a) was derived from (r a b) alone — root fact, no
    # hypothesis touched → unconditional.
    assert ("r", ("b", "a")) in _ids(result.unconditional_facts)
    # The hypothesis itself is never "unconditional" — it's a
    # premise, not a derivation.
    assert ("h", ("a",)) not in _ids(result.unconditional_facts)
    # And the hypothesis IS in hypothesis_facts.
    assert _ids(result.hypothesis_facts) == {("h", ("a",))}


def test_alive_conditional_excluded_from_unconditional_facts():
    """Same fixture; the (d a) derivation from the hypothesis is
    conditional and must NOT appear in unconditional_facts."""
    kb = _kb("""
    (rule sym-r ()
      :match (r ?x ?y) :assert (r ?y ?x)
      :why "symmetric r" :priority 100)
    (rule from-h ()
      :match (h ?x) :assert (d ?x)
      :why "h gives d" :priority 100)
    (type T)
    (relation r T T)
    (relation h T)
    (relation d T)
    (instance a T) (instance b T)
    (r a b :source "(1)")
    """)
    commitment = (("h", ("a",)),)
    result = try_commitment_set(kb, commitment)

    assert result.kind == "alive"
    fork_ids = _ids(result.kb.facts)
    # (d a) IS in kb (saturator derived it).
    assert ("d", ("a",)) in fork_ids
    # …but it must NOT be in unconditional_facts — its chain
    # reaches (h a) which is in the commitment.
    assert ("d", ("a",)) not in _ids(result.unconditional_facts)
    # (r b a) IS unconditional (cross-check with previous test).
    assert ("r", ("b", "a")) in _ids(result.unconditional_facts)


# ── Dead-pre: root already has (not h) at REASONING ───────────────


def test_dead_pre_root_carries_negation_of_committed_hypothesis():
    """If the root kb already has a REASONING-layer
    `(not (target c d))` (e.g., from a previous back-prop write
    that landed on root), committing `(target c d)` must trigger
    a pre-saturation contradiction.
    """
    kb = _kb("""
    (type T)
    (relation target T T)
    (instance c T) (instance d T)
    
    """)
    # Seed REASONING-layer (not (target c d)) — pattern from
    # test_contradiction.test_pair_kind_defaults_to_pair.
    _put(kb, Fact(
        relation_name="not",
        args=(Fact(
            relation_name="target", args=("c", "d"),
            layer=Layer.REASONING,
        ),),
        layer=Layer.REASONING,
        provenance=Provenance.from_rule(rule="prior-back-prop"),
    ))

    result = try_commitment_set(kb, (("target", ("c", "d")),))

    assert result.kind == "dead-pre"
    assert result.firings == ()  # no saturation ran
    # The unsat_core walks back from the conflict; should be non-
    # empty and include the committed positive's identity.
    assert len(result.unsat_core) >= 1
    assert ("target", ("c", "d")) in _ids(result.unsat_core)
    # hypothesis_facts still records the write we made.
    assert _ids(result.hypothesis_facts) == {("target", ("c", "d"))}


# ── Dead-post: two committed hypotheses derive a contradiction ────


def test_dead_post_two_hypotheses_derive_contradiction():
    """Commit {h1(a), h2(a)} — rules derive (x a) and (not (x a))
    respectively, both at REASONING. Post-saturation detector
    flags the pair → dead-post.
    """
    kb = _kb("""
    (rule h1-implies-x ()
      :match (h1 ?x) :assert (x ?x)
      :why "h1 → x" :priority 100)
    (rule h2-forbids-x ()
      :match (h2 ?x) :assert (not (x ?x))
      :why "h2 → ¬x" :priority 100)
    (type T)
    (relation h1 T) (relation h2 T) (relation x T)
    (instance a T)
    
    """)
    commitment = (("h1", ("a",)), ("h2", ("a",)))

    result = try_commitment_set(kb, commitment)

    assert result.kind == "dead-post"
    assert len(result.firings) > 0  # saturation DID run
    # unsat_core is the *source frontier* reachable from the
    # contradiction's witness — the speculative facts the
    # contradiction depends on. The witness is the positive
    # `(x a)`; its derivation chain walks back to `(h1 a)` (the
    # hypothesis that produced it). `(not (x a))`'s chain back to
    # `(h2 a)` is reached if the negative is also passed as a
    # witness, but `c.witness` returns only `positive` for pair
    # contradictions (matches `tree/solver.py`'s convention).
    assert len(result.unsat_core) >= 1
    unsat_ids = _ids(result.unsat_core)
    assert (
        ("h1", ("a",)) in unsat_ids or ("h2", ("a",)) in unsat_ids
    ), f"expected at least one hypothesis in unsat_core, got {unsat_ids}"
    assert _ids(result.hypothesis_facts) == {
        ("h1", ("a",)), ("h2", ("a",)),
    }


# ── Isolation: two calls on the same root produce independent kbs ─


def test_isolation_two_calls_yield_independent_forks():
    """`try_commitment_set(root, C1)` and `try_commitment_set(root, C2)`
    produce results whose kbs are distinct objects; mutating one
    fork's facts list doesn't affect the other or the root.
    """
    kb = _kb("""
    (type T)
    (relation h1 T) (relation h2 T)
    (instance a T)
    
    """)
    root_size = len(kb.facts)

    r1 = try_commitment_set(kb, (("h1", ("a",)),))
    r2 = try_commitment_set(kb, (("h2", ("a",)),))

    assert r1.kind == "alive"
    assert r2.kind == "alive"
    # Distinct fork instances.
    assert r1.kb is not r2.kb
    assert r1.kb is not kb
    assert r2.kb is not kb
    # Root unchanged by either call.
    assert len(kb.facts) == root_size
    # r1 sees h1 but not h2; r2 sees h2 but not h1.
    assert ("h1", ("a",)) in _ids(r1.kb.facts)
    assert ("h2", ("a",)) not in _ids(r1.kb.facts)
    assert ("h2", ("a",)) in _ids(r2.kb.facts)
    assert ("h1", ("a",)) not in _ids(r2.kb.facts)


# ── Empty commitment: sentinel case ───────────────────────────────


def test_empty_commitment_returns_alive_with_empty_results():
    """`try_commitment_set(root, ())` is the layer-0 sentinel —
    no hypothesis written, saturator runs on root-fork. With a
    pre-saturated root, no new facts are produced; both
    hypothesis_facts and unconditional_facts are empty.
    """
    kb = _kb("""
    (rule sym-r ()
      :match (r ?x ?y) :assert (r ?y ?x)
      :why "symmetric r" :priority 100)
    (type T)
    (relation r T T)
    (instance a T) (instance b T)
    (r a b :source "(1)")
    """)
    # Pre-saturate root so the empty-commitment fork has nothing
    # new to derive.
    list(Saturator(kb).saturate())

    result = try_commitment_set(kb, ())

    assert result.kind == "alive"
    assert result.hypothesis_facts == ()
    assert result.unconditional_facts == ()
    # Saturator may have stepped (and yielded zero or some
    # already-redundant firings), but the kb content matches the
    # pre-saturated root.
    assert _ids(result.kb.facts) == _ids(kb.facts)
