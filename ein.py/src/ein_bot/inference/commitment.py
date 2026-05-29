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
- :mod:`ein_bot.inference.back_prop` —
  :func:`reaches_hypothesis`'s global "any hypothesis-kind"
  variant; :func:`_reaches_commitment` here is the
  commitment-set parameterised analogue.
"""
from __future__ import annotations

from dataclasses import dataclass
from typing import Literal

from ein_bot.inference.apriori import CanonicalSetId, FactId
from ein_bot.inference.contradiction import ContradictionDetector
from ein_bot.inference.firing import Firing
from ein_bot.inference.saturator import Saturator
from ein_bot.kb.entities import Fact, Layer
from ein_bot.kb.provenance import Provenance
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
        stored = fork.add_fact(h_fact)
        fork._index_fact(stored)
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

    Commitment-set parameterised analogue of
    :func:`ein_bot.inference.back_prop.reaches_hypothesis`
    (which uses a global "any hypothesis-kind fact" terminal).
    Here the terminal is matching a specific FactId in the
    commitment — soundness rests on this distinction.
    """
    visited: set[FactId] = set()
    return not _reaches_commitment(kb, fact, visited, hypothesis_ids)


def _reaches_commitment(
    kb: KnowledgeBase, fact: Fact,
    visited: set[FactId], hypothesis_ids: frozenset[FactId],
) -> bool:
    """Recursive walker — True iff some premise chain reaches a
    fact in ``hypothesis_ids``.

    Cycle-safe via ``visited``; skips ``rule``-kind premises whose
    id is absent from ``kb`` (defensive; shouldn't happen in a
    well-formed derivation chain).
    """
    key: FactId = (fact.relation_name, fact.args)
    if key in visited:
        return False
    visited.add(key)
    if key in hypothesis_ids:
        return True
    prov = fact.provenance
    if prov is None or prov.kind != "rule":
        return False
    for rid in prov.premises_raw:
        premise = kb._fact_by_id(*rid)
        if premise is not None and _reaches_commitment(
                kb, premise, visited, hypothesis_ids):
            return True
    return False


__all__ = [
    "CommitmentSetResult",
    "try_commitment_set",
]
