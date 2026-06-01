"""Lattice content-snapshot serialiser — S1.5b.31.

Projects a completed lattice solve (a :class:`Verdict` carrying a
non-None :class:`LatticeProof` + the final ``root_kb``) into a
content-addressed :class:`LatticeSnapshotV1` value that is
*invariant* under within-layer traversal-order permutations.
Two solves of the same puzzle at the same ``max_set_size``
under different :attr:`SolverConfig.lattice_order_seed` values
must produce snapshots that compare ``==`` — if they don't, an
order leak has crept into the engine loop (forced-positive
integration order, nogood subsumption order, multilabel
representative-id leak, etc.) and the lattice's "set determines
kb" invariant is degraded at the engine level.

The snapshot canonicalises per-state_hash:

- Group every :class:`SetNode` in ``proof.kb_index`` by
  ``state_hash``.
- For each ``state_hash``, union all observed labels into one
  frozenset (so under-gaps-multi-SetNodes-per-state collapse to
  one entry).
- The verdict label per state is the union of observed
  per-SetNode verdicts (rare; typically all the same).

This keeps :attr:`SetNode.canonical_set` — the "first arrival"
attribution that is permutation-dependent — out of the
snapshot's identity. The shuffle harness in
``tests/inference/lattice/test_shuffle_invariance.py`` compares
two snapshots for ``==`` equality.

Cross-references:

- Stage spec:
  ``plans/m1_core_graph_reasoning/p1.5b_lattice_search/s1.5b.31_lattice_shuffle_invariance.md``.
- Sibling test:
  ``ein.py/tests/inference/lattice/test_shuffle_invariance.py``.
- Snapshot input: :class:`LatticeProof` (S1.5b.22) +
  ``root_kb`` at termination.
"""
from __future__ import annotations

from dataclasses import dataclass

from ein_bot.inference.apriori import CanonicalSetId, FactId
from ein_bot.inference.canon import state_hash
from ein_bot.inference.monotonic.lattice import LatticeProof
from ein_bot.inference.verdict import Verdict
from ein_bot.kb.store import KnowledgeBase


@dataclass(frozen=True)
class LatticeSnapshotV1:
    """Content-addressed depth-``L`` lattice projection.

    Fields use hashable / sorted-tuple shapes so frozen-dataclass
    equality + Python's structural ``==`` are sufficient to
    compare two snapshots without bespoke equality logic.

    Attributes
    ----------
    nodes_by_state_hash
        Sorted tuple of ``(state_hash, union_labels,
        verdict_labels)`` triples. One entry per distinct
        ``state_hash`` observed in :attr:`LatticeProof.kb_index`.
        ``union_labels`` collapses every label across SetNodes
        that share the state_hash (a no-op under
        contradictions+store_lattice merge; meaningful under
        gaps where distinct commitments may hash-collide).
        ``verdict_labels`` is the union of per-SetNode
        ``SetNode.verdict`` values for that state_hash.
    root_state_hash
        ``state_hash(root_kb)`` at termination. Carries the
        accumulated unconditional-facts merges + the
        forced-positive promotions.
    nogoods
        ``frozenset(proof.learned_nogoods)`` — the engine's
        final nogood store. Order of emission must not affect
        the final set.
    verdict_kind
        ``type(verdict).__name__`` (``"Solution"`` /
        ``"Ambiguity"`` / ``"Contradiction"``) — the mode
        contract's verdict shape.
    solutions
        ``frozenset(s.commitment for s in proof.solutions)``
        — the set of satisfying commitments collected by
        :func:`gaps_solve`. ``frozenset(())`` under monotonic
        / contradictions.
    deads
        ``frozenset(d.commitment for d in proof.dead_commitments)``
        — the set of refuted commitments collected by
        :func:`contradictions_solve`. ``frozenset(())`` under
        monotonic / gaps.
    alive_at_end
        ``frozenset(proof.alive_at_end)`` — the surviving
        size-``N`` frontier when the depth cap was the natural
        terminator.
    """

    nodes_by_state_hash: tuple[
        tuple[int, frozenset[CanonicalSetId], frozenset[str]], ...,
    ]
    root_state_hash:     int
    nogoods:             frozenset[frozenset[FactId]]
    verdict_kind:        str
    solutions:           frozenset[CanonicalSetId]
    deads:               frozenset[CanonicalSetId]
    alive_at_end:        frozenset[CanonicalSetId]


def lattice_snapshot(
    verdict: Verdict,
    root_kb: KnowledgeBase,
) -> LatticeSnapshotV1:
    """Project a completed lattice solve into a
    :class:`LatticeSnapshotV1`.

    Requires ``verdict.proof`` to be non-None — call this on
    verdicts from :func:`gaps_solve` or
    :func:`contradictions_solve`, not :func:`solve`.
    The ``root_kb`` argument is the kb at termination (the
    solver's ``root_kb`` after the call returns) — its
    ``state_hash`` records the cumulative root-side merges +
    forced-positive promotions.
    """
    proof = getattr(verdict, "proof", None)
    if not isinstance(proof, LatticeProof):
        raise ValueError(
            "lattice_snapshot requires verdict.proof to be a "
            "LatticeProof; got "
            f"{type(proof).__name__ if proof is not None else 'None'}",
        )

    # Group SetNodes by state_hash so the snapshot collapses any
    # per-commitment dict-keying artefacts (especially under gaps
    # where the dict key is hash(commitment) rather than
    # state_hash).
    labels_by_state: dict[int, set[CanonicalSetId]] = {}
    verdicts_by_state: dict[int, set[str]] = {}
    for node in proof.kb_index.values():
        labels_by_state.setdefault(node.state_hash, set()).update(
            node.labels,
        )
        verdicts_by_state.setdefault(node.state_hash, set()).add(
            node.verdict,
        )

    nodes = tuple(sorted(
        (
            (
                sh,
                frozenset(labels_by_state[sh]),
                frozenset(verdicts_by_state[sh]),
            )
            for sh in labels_by_state
        ),
        key=lambda t: t[0],
    ))

    return LatticeSnapshotV1(
        nodes_by_state_hash=nodes,
        root_state_hash=state_hash(root_kb),
        nogoods=frozenset(proof.learned_nogoods),
        verdict_kind=type(verdict).__name__,
        solutions=frozenset(s.commitment for s in proof.solutions),
        deads=frozenset(d.commitment for d in proof.dead_commitments),
        alive_at_end=frozenset(proof.alive_at_end),
    )


__all__ = ["LatticeSnapshotV1", "lattice_snapshot"]
