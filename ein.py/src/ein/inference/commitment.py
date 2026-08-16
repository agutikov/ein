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
from ein.inference.contradiction import ContradictionDetector
from ein.inference.firing import Firing
from ein.inference.saturator import Saturator
from ein.kb.entities import Fact, Layer
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

    return CommitmentSetResult(
        commitment=commitment,
        kb=fork,
        firings=firings,
        kind="alive",
        hypothesis_facts=tuple(hypothesis_facts),
    )


__all__ = [
    "CommitmentSetResult",
    "try_commitment_set",
]
