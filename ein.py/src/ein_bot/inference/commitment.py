"""Common commitment-set primitive — `try_commitment_set` + `CommitmentSetResult` — S1.5b.3.

Forks `root_kb`, writes every hypothesis in `commitment` into the
fork, saturates once, detects contradictions, and extracts
unconditional facts (whose derivation chain doesn't touch any
element of the commitment).

Both monotonic and lattice engines call this. Pure-with-fork
semantics — the fork is the function's output, never reused
across calls. No state is shared between two
:func:`try_commitment_set` invocations on the same root (modulo
:meth:`KnowledgeBase.fork`'s shared-by-reference fields, which
the P1.5b channel-isolation rewrite addresses).

The unconditional-fact extraction is the soundness-critical
novel piece. A fact whose entire derivation grounds out at root
facts — never touching a committed hypothesis — is provably true
at root level given ``root + rules``; the engine merges these
into root before the next layer, monotonically shrinking the
alive set.

Cross-refs:
- ``Q1.5b.8`` (engine bridge — resolved 2026-05-25 — set-batch
  primitive shared by both engines).
- :mod:`ein_bot.inference.apriori` — produces the
  :data:`CanonicalSetId` inputs.
- :func:`ein_bot.kb.provenance.reaches` — the shared provenance DFS
  :func:`_is_unconditional` runs with a commitment-set terminal; now the
  only hypothesis-terminal walk (``back_prop`` removed, S1.9.E6a).
"""
from __future__ import annotations

from dataclasses import dataclass
from typing import Literal

from ein_bot.inference.apriori import CanonicalSetId
from ein_bot.inference.contradiction import ContradictionDetector
from ein_bot.inference.firing import Firing
from ein_bot.inference.saturator import Saturator
from ein_bot.kb.entities import Fact, Layer
from ein_bot.kb.provenance import FactId, Provenance, reaches
from ein_bot.kb.store import KnowledgeBase


@dataclass(frozen=True)
class CommitmentSetResult:
    """Outcome of one commitment-set entering — :func:`try_commitment_set`'s return.

    Carries the commitment, the forked + saturated kb, and the
    facts the parent (root, in monotonic mode) may want to
    adopt.
    """

    commitment:          CanonicalSetId
    kb:                  KnowledgeBase
    firings:             tuple[Firing, ...]
    kind:                Literal["alive", "dead-pre", "dead-post"]
    unsat_core:          frozenset[Fact] = frozenset()

    # Facts derived during this commitment's saturation whose
    # provenance chain doesn't touch any hypothesis in
    # `commitment`. Provably true at root level given root +
    # rules; engine merges these into root.
    unconditional_facts: tuple[Fact, ...] = ()

    # The actual `(h_i)` writes for h_i ∈ commitment (NOT the
    # saturator's additions). Useful for the lattice's per-set
    # audit.
    hypothesis_facts:    tuple[Fact, ...] = ()


def try_commitment_set(
    root_kb: KnowledgeBase,
    commitment: CanonicalSetId,
    *,
    saturator_steps: int | None = None,
) -> CommitmentSetResult:
    """Fork root, write every hypothesis in ``commitment``, saturate,
    detect, extract unconditional facts.

    ``commitment`` is the canonical-tuple representation (sorted;
    see :data:`ein_bot.inference.apriori.CanonicalSetId`). Each
    element is a ``(relation_name, args)`` FactId for a positive
    hypothesis fact. The fork's saturator runs at most
    ``saturator_steps`` rule firings; ``None`` (default) means run
    to fixed point — the M1 ruleset is monotone so saturation
    terminates.

    Returns:
      ``CommitmentSetResult(kind="dead-pre", unsat_core=…)`` if a
        contradiction surfaces immediately after writing the
        hypotheses (no saturation runs).
      ``CommitmentSetResult(kind="dead-post", unsat_core=…)`` if
        saturation runs and the post-saturation kb has a
        contradiction.
      ``CommitmentSetResult(kind="alive", unconditional_facts=…,
        hypothesis_facts=…)`` otherwise.

    Idempotency: ``try_commitment_set(root_kb, C)`` produces an
    independent result every call; calling it twice on the same
    ``root_kb`` returns two separate :class:`CommitmentSetResult`
    objects whose forks share no mutable state.
    """
    fork = root_kb.fork()
    hypothesis_facts: list[Fact] = []
    for rn, args in commitment:
        h_fact = Fact(
            relation_name=rn,
            args=args,
            layer=Layer.REASONING,
            provenance=Provenance.from_hypothesis(branch=0),
        )
        stored = fork.add_and_index_fact(h_fact)
        hypothesis_facts.append(stored)

    # Pre-saturation contradiction check (apriori filter at the
    # kb level — catches newly-negated facts that crept into root
    # between the candidate's generation and this fork's
    # creation).
    pre_contras = ContradictionDetector(fork).detect()
    if pre_contras:
        return CommitmentSetResult(
            commitment=commitment,
            kb=fork,
            firings=(),
            kind="dead-pre",
            unsat_core=frozenset(
                fork.unsat_core(c.witness for c in pre_contras)
            ),
            hypothesis_facts=tuple(hypothesis_facts),
        )

    sat = Saturator(fork)
    firings = tuple(sat.saturate(max_steps=saturator_steps))

    post_contras = ContradictionDetector(fork).detect()
    if post_contras:
        return CommitmentSetResult(
            commitment=commitment,
            kb=fork,
            firings=firings,
            kind="dead-post",
            unsat_core=frozenset(
                fork.unsat_core(c.witness for c in post_contras)
            ),
            hypothesis_facts=tuple(hypothesis_facts),
        )

    # Alive — extract unconditional facts.
    hyp_ids: frozenset[FactId] = frozenset(commitment)
    unconditional: list[Fact] = []
    for f in fork.facts:
        if not _is_new_relative_to(f, root_kb):
            continue
        if _is_unconditional(f, fork, hyp_ids):
            unconditional.append(f)

    return CommitmentSetResult(
        commitment=commitment,
        kb=fork,
        firings=firings,
        kind="alive",
        unconditional_facts=tuple(unconditional),
        hypothesis_facts=tuple(hypothesis_facts),
    )


# ── Helpers ─────────────────────────────────────────────────────


def _is_new_relative_to(fact: Fact, root_kb: KnowledgeBase) -> bool:
    """True iff ``fact`` exists in the fork but not in root.

    Identity is ``(relation_name, args)`` — layer + provenance
    are ignored, matching :meth:`KnowledgeBase.add_fact`'s dedup
    contract.
    """
    return root_kb._fact_by_id(fact.relation_name, fact.args) is None


def _is_unconditional(
    fact: Fact, kb: KnowledgeBase,
    hypothesis_ids: frozenset[FactId],
) -> bool:
    """True iff ``fact``'s derivation chain doesn't touch any
    hypothesis in ``hypothesis_ids``.

    Runs the shared :func:`~ein_bot.kb.provenance.reaches` DFS (F-KER-10)
    with a commitment-set terminal: the chain is "conditional" iff it
    reaches a FactId in the commitment.
    """
    def _commitment_terminal(key: FactId, _fact: Fact) -> bool | None:
        return True if key in hypothesis_ids else None

    visited: set[FactId] = set()
    return not reaches(fact, visited, kb._fact_by_id, _commitment_terminal)


__all__ = [
    "CommitmentSetResult",
    "try_commitment_set",
]
