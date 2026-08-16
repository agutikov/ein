"""Lattice content-snapshot serialiser — S1.5b.31.

Projects a completed lattice solve (a :class:`Verdict` carrying a
non-None :class:`LatticeProof` + the final ``root_kb``) into a
content-addressed :class:`LatticeSnapshotV1` value that is
*invariant* under within-layer traversal-order permutations.
Two solves of the same puzzle at the same ``max_set_size``
under different :attr:`SolverConfig.lattice_order_seed` values
must produce snapshots that compare ``==`` — if they don't, an
order leak has crept into the engine loop (forced-positive
integration order, multilabel representative-id leak, etc.) and the
lattice's "set determines kb" invariant is degraded at the engine
level. The snapshot is **result-level** (S1.7.24): it keys on the
post-saturation STATES reached (solutions / deads / nodes), not the
commitment PATHS or the learned-nogood clauses — both of which are
legitimately order/orientation-sensitive once symmetric pairs are no
longer canonicalised by the kernel.

The snapshot canonicalises per-state_key:

- Group every :class:`SetNode` in ``proof.kb_index`` by
  ``state_key``.
- For each ``state_key``, union all observed labels into one
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

from ein.inference.apriori import CanonicalSetId
from ein.inference.canon import StateKey, state_key
from ein.inference.monotonic.lattice import LatticeProof
from ein.inference.verdict import Verdict
from ein.kb.store import KnowledgeBase


@dataclass(frozen=True)
class LatticeSnapshotV1:
    """Content-addressed depth-``L`` lattice projection.

    Fields use hashable / sorted-tuple shapes so frozen-dataclass
    equality + Python's structural ``==`` are sufficient to
    compare two snapshots without bespoke equality logic.

    Attributes
    ----------
    nodes_by_state_key
        Sorted tuple of ``(state_key, union_labels,
        verdict_labels)`` triples (sorted ``key=repr`` — state
        keys have no useful native order). One entry per distinct
        ``state_key`` observed in :attr:`LatticeProof.kb_index`.
        ``union_labels`` collapses every label across SetNodes
        that share the state_key (a no-op under
        contradictions+store_lattice merge; meaningful under
        gaps where distinct commitments may reach one state).
        ``verdict_labels`` is the union of per-SetNode
        ``SetNode.verdict`` values for that state_key.
    root_state_key
        ``state_key(root_kb)`` at termination. Carries the
        accumulated singleton-death ``(not h)`` writebacks + the
        forced-positive promotions (the only root writes during
        search — P1.21 R2).
    verdict_kind
        ``type(verdict).__name__`` (``"Solution"`` /
        ``"Ambiguity"`` / ``"Contradiction"``) — the mode
        contract's verdict shape.
    solutions
        ``frozenset(state_key(s.kb) for s in proof.solutions)``
        — the set of distinct satisfying *model states* (S1.7.24;
        keyed by post-saturation state_key, NOT commitment path,
        so the two orientations of a symmetric pair count once).
        ``frozenset(())`` when no commitment satisfied (a
        ``Contradiction`` verdict).
    deads
        ``frozenset(d.state_key for d in proof.dead_commitments)``
        — the set of distinct refuted *states* (S1.7.24; state-keyed
        for the same orientation-invariance). ``frozenset(())`` when
        no commitment was refuted.

    Note (S1.7.24): learned **nogoods are NOT in the snapshot**. A
    learned clause ``{(R a b), …}`` and its symmetric mirror
    ``{(R b a), …}`` are the same logical clause but distinct facts,
    and their equivalence is unknowable without ``is_symmetric`` — so
    the final nogood set is order/orientation-sensitive once the kernel
    stops canonicalising symmetric pairs. It is an internal
    optimisation artifact, not part of the solve *result*, so result-
    invariance keys on states (solutions / deads / nodes), not clauses.
    alive_at_end
        ``frozenset(proof.alive_at_end)`` — the surviving
        size-``N`` frontier when the depth cap was the natural
        terminator.
    """

    nodes_by_state_key: tuple[
        tuple[StateKey, frozenset[CanonicalSetId], frozenset[str]], ...,
    ]
    root_state_key:      StateKey
    verdict_kind:        str
    # S1.7.24 — solutions / deads are sets of post-saturation STATE
    # keys (orientation-invariant), not commitment paths.
    solutions:           frozenset[StateKey]
    deads:               frozenset[StateKey]
    alive_at_end:        frozenset[CanonicalSetId]


def lattice_snapshot(
    verdict: Verdict,
    root_kb: KnowledgeBase,
) -> LatticeSnapshotV1:
    """Project a completed lattice solve into a
    :class:`LatticeSnapshotV1`.

    Requires ``verdict.proof`` to be non-None — call this on a
    verdict from a :func:`solve` run with ``store_lattice=True``
    (the default fast path leaves ``proof`` None).
    The ``root_kb`` argument is the kb at termination (the
    solver's ``root_kb`` after the call returns) — its
    ``state_key`` records the cumulative root-side merges +
    forced-positive promotions.
    """
    proof = getattr(verdict, "proof", None)
    if not isinstance(proof, LatticeProof):
        raise ValueError(
            "lattice_snapshot requires verdict.proof to be a "
            "LatticeProof; got "
            f"{type(proof).__name__ if proof is not None else 'None'}",
        )

    # Group SetNodes by state_key so the snapshot collapses any
    # per-commitment dict-keying artefacts (especially under gaps
    # where the dict key is the commitment rather than the
    # state_key).
    labels_by_state: dict[StateKey, set[CanonicalSetId]] = {}
    verdicts_by_state: dict[StateKey, set[str]] = {}
    for node in proof.kb_index.values():
        labels_by_state.setdefault(node.state_key, set()).update(
            node.labels,
        )
        verdicts_by_state.setdefault(node.state_key, set()).add(
            node.verdict,
        )

    # Sort by repr — StateKeys are tuples of heterogeneous tuples with
    # no useful native total order (P1.21 R1).
    nodes = tuple(sorted(
        (
            (
                sk,
                frozenset(labels_by_state[sk]),
                frozenset(verdicts_by_state[sk]),
            )
            for sk in labels_by_state
        ),
        key=lambda t: repr(t[0]),
    ))

    return LatticeSnapshotV1(
        nodes_by_state_key=nodes,
        root_state_key=state_key(root_kb),
        verdict_kind=type(verdict).__name__,
        # S1.7.24 — RESULT-level keys: a solution / dead is identified by
        # the post-saturation STATE it reaches, not the commitment PATH
        # that reached it. This is orientation-invariant — the two
        # orientations of a symmetric pair saturate to the same state —
        # so the snapshot is shuffle-invariant without the kernel
        # canonicalising symmetric pairs. (Learned `nogoods` are NOT in
        # the snapshot: a learned clause `{(R a b),…}` and its mirror
        # `{(R b a),…}` are the same logical clause but distinct facts,
        # and the equivalence is unknowable without `is_symmetric`; the
        # final nogood SET is thus order/orientation-sensitive and is an
        # internal optimisation artifact, not part of the solve result.)
        solutions=frozenset(state_key(s.kb) for s in proof.solutions),
        deads=frozenset(d.state_key for d in proof.dead_commitments),
        alive_at_end=frozenset(proof.alive_at_end),
    )


__all__ = ["LatticeSnapshotV1", "lattice_snapshot"]
