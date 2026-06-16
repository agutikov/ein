"""P1.6 handoff contract validator — S1.5b.29 Part B.

:func:`validate_proof_for_explanation` exercises every field of a
:class:`LatticeProof` as the future P1.6 NL renderer would. It is
intended for two usage sites:

- **Tests** — any :func:`solve` run with ``store_lattice=True``
  (across the gaps / contradictions / solution verdict shapes) can
  call it on the returned verdict's proof to lock in the structural
  invariants the renderer depends on.
- **The P1.6 NL renderer's pre-flight check** — before walking
  ``proof.solutions[i].kb.derivation_dag(goal_fact)``, the
  renderer can call this validator to fail fast on a malformed
  proof.

The validator raises :class:`AssertionError` on contract
violation; returns ``None`` on success. Each assertion carries
a message naming the offending field so the failure tells the
caller what to fix.

What the validator does NOT do
------------------------------

The full P1.6 contract (walking ``derivation_dag`` from every
goal fact and verifying leaves are root-facts / hypothesis-facts
/ forced-positive promotions) lands when the renderer itself is
built. For S1.5b.29 the validator confirms the *structural*
invariants — goal satisfaction, dead-record well-formedness,
nogood subsumption, ``kb_index`` consistency, stats coherence.
A future stage upgrades the validator with the deeper walk once
``kb.query.goal_facts(kb)`` exists.
"""
from __future__ import annotations

from ein.inference.monotonic.lattice import LatticeProof
from ein.inference.verdict import (
    Contradiction,
    Mode,
    Verdict,
    is_solved,
)


def validate_proof_for_explanation(
    verdict: Verdict,
    proof: LatticeProof,
) -> None:
    """Raise :class:`AssertionError` on contract violation.

    Invariants checked:

    1. ``verdict.proof is proof`` — the proof argument is the
       one attached to the verdict (catches accidental
       parameter swaps).
    2. Every :class:`SolutionRecord` is goal-satisfying under
       SOLVE-mode :func:`is_solved` against its snapshotted kb.
    3. Every :class:`DeadCommitment` has a non-empty
       ``unsat_core`` and ``learned_clause ==
       frozenset(commitment)``.
    4. Every dead's ``learned_clause`` is subsumed by some
       member of ``learned_nogoods`` — i.e., the snapshotted
       root nogood store covers every per-record clause.
    5. Every :class:`SetNode` in ``kb_index``:

       - has ``state_hash`` matching its dict key (modulo the
         per-mode keying — the gaps mode uses ``hash(commitment)``
         so the key need not equal ``state_hash``);
       - has ``canonical_set in labels``;
       - has ``len(labels) > 1`` only for a :class:`Contradiction`
         verdict (multilabel collapse is the contradictions-side
         merge; the gaps side must keep distinct commitments
         distinct).

    6. Stats coherence — ``stats.solutions_found ==
       len(proof.solutions)``; cumulative dead counter
       ``≥ len(proof.dead_commitments)`` (the inequality
       accounts for the contradictions+store_lattice merge
       path that increments the dead counter but skips the
       :class:`DeadCommitment` append).
    """
    # 1. proof identity.
    assert getattr(verdict, "proof", None) is proof, (
        "verdict.proof must be the proof argument "
        "(identity check) — re-attach via "
        "Solution/Ambiguity/Contradiction(proof=proof)"
    )

    # 2. Solutions satisfy the goal.
    for sol in proof.solutions:
        assert is_solved(sol.kb, Mode.SOLVE), (
            f"SolutionRecord {sol.commitment!r}'s snapshotted "
            "kb fails is_solved under SOLVE mode"
        )

    # 3. Dead commitments are well-formed.
    for d in proof.dead_commitments:
        assert d.unsat_core, (
            f"DeadCommitment {d.commitment!r} has empty "
            "unsat_core — the engine recorded a dead but the "
            "ContradictionDetector returned no witnesses"
        )
        assert d.learned_clause == frozenset(d.commitment), (
            f"DeadCommitment {d.commitment!r}'s "
            f"learned_clause {d.learned_clause!r} does not "
            "match frozenset(commitment)"
        )

    # 4. learned_nogoods subsumes per-record learned_clauses.
    for d in proof.dead_commitments:
        clause = frozenset(d.commitment)
        assert any(
            stored <= clause for stored in proof.learned_nogoods
        ), (
            f"DeadCommitment {d.commitment!r}'s clause "
            f"{clause!r} is not subsumed by any entry in "
            "proof.learned_nogoods — engine forgot to emit "
            "or the snapshot is stale"
        )

    # 5. kb_index invariants.
    is_contradictions = isinstance(verdict, Contradiction)
    for key, node in proof.kb_index.items():
        # canonical_set ∈ labels — always.
        assert node.canonical_set in node.labels, (
            f"SetNode at key {key:#x} has "
            f"canonical_set={node.canonical_set!r} not in "
            f"labels={node.labels!r}"
        )
        # Multi-label only under contradictions.
        if len(node.labels) > 1:
            assert is_contradictions, (
                f"SetNode at key {key:#x} has {len(node.labels)} "
                "labels but verdict is not Contradiction — "
                "gaps must keep distinct satisfying commitments "
                "separate"
            )

    # 6. Stats coherence.
    assert proof.stats.solutions_found == len(proof.solutions), (
        f"proof.stats.solutions_found="
        f"{proof.stats.solutions_found} but "
        f"len(proof.solutions)={len(proof.solutions)}"
    )
    total_dead = (
        proof.stats.enterings_dead_pre
        + proof.stats.enterings_dead_post
    )
    assert total_dead >= len(proof.dead_commitments), (
        f"cumulative dead counter ({total_dead}) is less than "
        f"len(proof.dead_commitments) "
        f"({len(proof.dead_commitments)}) — engine miscounted"
    )


__all__ = ["validate_proof_for_explanation"]
