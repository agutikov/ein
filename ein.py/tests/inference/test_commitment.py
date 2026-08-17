"""Commitment-set primitive tests — S1.5b.3 T1.5b.3.2.

Pins :func:`ein.inference.commitment.try_commitment_set`
across the trichotomy (alive / dead-pre / dead-post) +
isolation between two calls + the empty-commitment sentinel.
(The former unconditional-fact-extraction tests were removed
with the extraction itself — P1.21 R2; root-stability under NAF
is pinned in ``tests/inference/monotonic/test_root_stability_naf.py``.)
"""
from __future__ import annotations

from dataclasses import replace

from ein.inference.commitment import (
    CommitmentSetResult,
    try_commitment_set,
)
from ein.inference.config import SolverConfig
from ein.inference.saturator import Saturator
from ein.ir import parse
from ein.kb.entities import Fact
from ein.kb.provenance import Provenance
from ein.kb.store import KnowledgeBase


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
    derivation. The derived fact stays in the fork's kb — nothing
    is extracted or written back.
    """
    kb = _kb("""
    (rule swap ()
      :match (target ?x ?y) :assert (other ?y ?x)
      :why "swap target → other" :priority 100)
    (relation target T T)
    (relation other T T)
    (is-a c T) (is-a d T)
    
    """)
    commitment = (("target", ("c", "d")),)
    result = try_commitment_set(kb, commitment)

    assert isinstance(result, CommitmentSetResult)
    assert result.kind == "alive"
    assert _ids(result.hypothesis_facts) == {("target", ("c", "d"))}
    # kb DOES contain the derived (other d c) — fork-local only.
    assert ("other", ("d", "c")) in _ids(result.kb.facts)
    # …and the root was not touched by the entering.
    assert ("other", ("d", "c")) not in _ids(kb.facts)


# ── Dead-pre: root already has (not h) at REASONING ───────────────


def test_dead_pre_root_carries_negation_of_committed_hypothesis():
    """If the root kb already has a derived
    `(not (target c d))` (e.g., from a previous back-prop write
    that landed on root), committing `(target c d)` must trigger
    a pre-saturation contradiction.
    """
    kb = _kb("""
    (relation target T T)
    (is-a c T) (is-a d T)
    
    """)
    # Seed a derived (not (target c d)) — pattern from
    # test_contradiction.test_pair_kind_defaults_to_pair.
    _put(kb, Fact(
        relation_name="not",
        args=(Fact(
            relation_name="target", args=("c", "d"),
        ),),
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
    (relation h1 T) (relation h2 T) (relation x T)
    (is-a a T)
    
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
    (relation h1 T) (relation h2 T)
    (is-a a T)
    
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
    pre-saturated root, no new facts are produced and
    hypothesis_facts is empty.
    """
    kb = _kb("""
    (rule sym-r ()
      :match (r ?x ?y) :assert (r ?y ?x)
      :why "symmetric r" :priority 100)
    (relation r T T)
    (is-a a T) (is-a b T)
    (r a b :source "(1)")
    """)
    # Pre-saturate root so the empty-commitment fork has nothing
    # new to derive.
    list(Saturator(kb).saturate())

    result = try_commitment_set(kb, ())

    assert result.kind == "alive"
    assert result.hypothesis_facts == ()
    # Saturator may have stepped (and yielded zero or some
    # already-redundant firings), but the kb content matches the
    # pre-saturated root.
    assert _ids(result.kb.facts) == _ids(kb.facts)


# ── Fail-fast fork saturation — S1.9.E23 ──────────────────────────


def _fail_fast_fixture() -> KnowledgeBase:
    """A dying commitment with work queued *behind* the clash.

    `(h1 a)` derives `(x a)` and `(h2 a)` derives `(not (x a))` — the
    pair that kills the fork. `chain0 → chain1 → … → chain5` is an
    independent derivation ladder the saturator would keep walking
    after the clash: fail-fast is visible as the ladder not being
    finished.
    """
    return _kb("""
    (rule h1-implies-x ()
      :match (h1 ?x) :assert (x ?x)
      :why "h1 → x" :priority 100)
    (rule h2-forbids-x ()
      :match (h2 ?x) :assert (not (x ?x))
      :why "h2 → ¬x" :priority 100)
    (rule step1 () :match (chain0 ?x) :assert (chain1 ?x)
      :why "ladder" :priority 200)
    (rule step2 () :match (chain1 ?x) :assert (chain2 ?x)
      :why "ladder" :priority 200)
    (rule step3 () :match (chain2 ?x) :assert (chain3 ?x)
      :why "ladder" :priority 200)
    (rule step4 () :match (chain3 ?x) :assert (chain4 ?x)
      :why "ladder" :priority 200)
    (rule step5 () :match (chain4 ?x) :assert (chain5 ?x)
      :why "ladder" :priority 200)
    (relation h1 T) (relation h2 T) (relation x T)
    (relation chain0 T) (relation chain1 T) (relation chain2 T)
    (relation chain3 T) (relation chain4 T) (relation chain5 T)
    (is-a a T)
    (chain0 a :source "(1)")
    """)


def _with_fail_fast(kb: KnowledgeBase, on: bool) -> KnowledgeBase:
    kb.config = replace(kb.config or SolverConfig(), enable_fail_fast_fork=on)
    return kb


def test_fail_fast_stops_the_dying_fork_early():
    """With the flag on, a dead-post fork's saturation stops at the
    firing that made it inconsistent — the ladder behind the clash is
    left unwalked, so both the firing count and the fact set are
    strict prefixes of the fixpoint run.
    """
    commitment = (("h1", ("a",)), ("h2", ("a",)))
    full = try_commitment_set(_with_fail_fast(_fail_fast_fixture(), False),
                              commitment)
    fast = try_commitment_set(_with_fail_fast(_fail_fast_fixture(), True),
                              commitment)

    # Same verdict — that is the whole contract.
    assert full.kind == fast.kind == "dead-post"
    # …reached with strictly less work.
    assert len(fast.firings) < len(full.firings)
    assert _ids(fast.kb.facts) < _ids(full.kb.facts)
    # The clash is present in the truncated fork (it is what stopped it).
    assert ("x", ("a",)) in _ids(fast.kb.facts)
    # The ladder finished only in the full run.
    assert ("chain5", ("a",)) in _ids(full.kb.facts)
    assert ("chain5", ("a",)) not in _ids(fast.kb.facts)
    # A dying fork still explains itself: the core names a hypothesis.
    assert _ids(fast.unsat_core) & {("h1", ("a",)), ("h2", ("a",))}


def test_fail_fast_leaves_a_surviving_fork_fully_saturated():
    """No contradiction ⇒ nothing to stop at, so the alive fork is
    saturated to the fixpoint either way — identical firings and
    identical facts.
    """
    commitment = (("h1", ("a",)),)   # no `(h2 a)`, so no clash
    full = try_commitment_set(_with_fail_fast(_fail_fast_fixture(), False),
                              commitment)
    fast = try_commitment_set(_with_fail_fast(_fail_fast_fixture(), True),
                              commitment)

    assert full.kind == fast.kind == "alive"
    assert len(fast.firings) == len(full.firings)
    assert _ids(fast.kb.facts) == _ids(full.kb.facts)
    assert ("chain5", ("a",)) in _ids(fast.kb.facts)


def test_fail_fast_does_not_reach_dead_pre():
    """A commitment that is already refuted at root dies *before* any
    saturation, so the flag is irrelevant to it — pinned so a future
    reordering of the pre-check can't silently move behind the
    saturator.
    """
    kb = _fail_fast_fixture()
    _put(kb, Fact(
        relation_name="not",
        args=(Fact(relation_name="h1", args=("a",)),),
        provenance=Provenance.from_rule(rule="stated"),
    ))
    result = try_commitment_set(_with_fail_fast(kb, True), (("h1", ("a",)),))

    assert result.kind == "dead-pre"
    assert result.firings == ()
