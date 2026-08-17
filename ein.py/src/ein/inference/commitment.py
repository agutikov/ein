"""Common commitment-set primitive — `try_commitment_set` + `CommitmentSetResult` — S1.5b.3.

Forks `root_kb`, writes every hypothesis in `commitment` into the
fork, saturates once, and detects contradictions.

Both monotonic and lattice engines call this. Pure-with-fork
semantics — the fork is the function's output, never reused
across calls. No state is shared between two
:func:`try_commitment_set` invocations on the same root (modulo
:meth:`KnowledgeBase.fork`'s shared-by-reference fields, which
the P1.5b channel-isolation rewrite addresses), and the root is
never mutated: every consequence stays in the fork.

(The former "unconditional-fact extraction" — classifying fork
facts whose positive provenance chain avoids the commitment as
"provably true at root" — was retired in P1.21 R2: the
classification is unsound under NAF (`absent`), whose
dependencies leave no provenance edge. See the historical note in
``docs/kernel/inference/README.md``.)

Cross-refs:
- ``Q1.5b.8`` (engine bridge — resolved 2026-05-25 — set-batch
  primitive shared by both engines).
- :mod:`ein.inference.apriori` — produces the
  :data:`CanonicalSetId` inputs.
"""
from __future__ import annotations

from dataclasses import dataclass
from typing import Literal

from ein.inference.apriori import CanonicalSetId
from ein.inference.config import SolverConfig
from ein.inference.contradiction import ContradictionDetector, contradicts
from ein.inference.firing import Firing
from ein.inference.frontier import smallest_contradiction_frontier
from ein.inference.saturator import Saturator
from ein.kb.entities import Fact
from ein.kb.provenance import Provenance
from ein.kb.store import KnowledgeBase


@dataclass(frozen=True)
class CommitmentSetResult:
    """Outcome of one commitment-set entering — :func:`try_commitment_set`'s return.

    Carries the commitment, the forked + saturated kb, and the
    per-entering audit fields. Fork facts stay in the fork — the
    engine never adopts them into root (P1.21 R2).
    """

    commitment:          CanonicalSetId
    kb:                  KnowledgeBase
    firings:             tuple[Firing, ...]
    kind:                Literal["alive", "dead-pre", "dead-post"]
    unsat_core:          frozenset[Fact] = frozenset()

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
    detect.

    ``commitment`` is the canonical-tuple representation (sorted;
    see :data:`ein.inference.apriori.CanonicalSetId`). Each
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
      ``CommitmentSetResult(kind="alive", hypothesis_facts=…)``
        otherwise.

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
            unsat_core=smallest_contradiction_frontier(
                fork, [c.witness for c in pre_contras]),
            hypothesis_facts=tuple(hypothesis_facts),
        )

    sat = Saturator(fork)
    cfg = root_kb.config or SolverConfig()
    if cfg.enable_fail_fast_fork:
        firings = _saturate_until_dead(sat, fork, max_steps=saturator_steps)
    else:
        firings = tuple(sat.saturate(max_steps=saturator_steps))

    post_contras = ContradictionDetector(fork).detect()
    if post_contras:
        return CommitmentSetResult(
            commitment=commitment,
            kb=fork,
            firings=firings,
            kind="dead-post",
            unsat_core=smallest_contradiction_frontier(
                fork, [c.witness for c in post_contras]),
            hypothesis_facts=tuple(hypothesis_facts),
        )

    return CommitmentSetResult(
        commitment=commitment,
        kb=fork,
        firings=firings,
        kind="alive",
        hypothesis_facts=tuple(hypothesis_facts),
    )


def _saturate_until_dead(
    sat: Saturator,
    fork: KnowledgeBase,
    *,
    max_steps: int | None,
) -> tuple[Firing, ...]:
    """Saturate ``fork``, stopping at the firing that kills it — S1.9.E23.

    The fail-fast half of the fork's saturation. Identical to
    ``tuple(sat.saturate(max_steps=…))`` on a fork that survives; on a
    fork that dies it returns the prefix up to and including the firing
    whose conclusion made the KB inconsistent, and abandons the
    generator there.

    Sound because the KB is append-only: a contradiction is *created* by
    an insertion and can never be retracted, so a fork that is
    inconsistent at firing *n* is inconsistent at the fixpoint too. The
    verdict is therefore unchanged — only the amount of dead-branch work
    is (zebra2 exhaustive: 67 dead forks reach their clash after ~320 of
    ~2790 firings, so ~64% of all fork-saturation time was spent
    saturating KBs already known to be dead).

    What the caller sees differently on a dead fork:

    - ``firings`` is the prefix, not the full run — for the trace this is
      an improvement (the dying branch stops at the clash instead of
      grinding to quiescence past it);
    - ``kb`` holds the partial post-clash state, so ``unsat_core`` is
      drawn from the witnesses present at that point (usually the one
      that fired) rather than every witness the fixpoint would hold, and
      the ``DeadCommitment.state_key`` recorded for it is that partial
      state. Nothing in ``solve`` reads a dead fork's state back — it
      builds no per-SetNode DAG (``proof.kb_index`` is always empty) —
      but a DAG builder that merges dead commitments by state
      (``_record_setnode``) would see two orientations of a symmetric
      dead commitment share a fixpoint without sharing a fail-fast
      prefix. That is the case for ``enable_fail_fast_fork=False``.
    """
    firings: list[Firing] = []
    for firing in sat.saturate(max_steps=max_steps):
        firings.append(firing)
        if firing.redundant:
            continue          # wrote nothing; the KB cannot have changed
        for derived in firing.derived:
            if contradicts(fork, derived):
                return tuple(firings)
    return tuple(firings)


__all__ = [
    "CommitmentSetResult",
    "try_commitment_set",
]
