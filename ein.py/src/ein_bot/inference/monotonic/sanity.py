"""Saturation-commutativity sanity check — S1.5b.27.

Verifies P1.5b's foundational premise: for any commitment set
``C`` and any root kb ``R``, ``saturate(R + C)`` (the set union
of ``R`` and ``C``, then closure under the rule set) is uniquely
determined by ``R`` and ``C`` — every lattice path through
``C`` (commit some subset, saturate, add the rest, re-saturate)
yields the same post-saturation kb.

Why this matters
----------------

The lattice's ``d!``-redundancy elimination claim rests on this
premise: if any path through a commitment lattice yields a
different kb, the engine's "set determines kb" invariant
collapses and the lattice's state-hash dedup MERGE (S1.5b.22)
would silently erase distinguishable kbs. Under M1's monotone
rule set the premise holds; this module ships the
release-time regression net that verifies it.

When the check fires
--------------------

Off by default. Enable via :attr:`SolverConfig.lattice_sanity_check`
or the ``--lattice-sanity-check`` CLI flag. On each alive
size-``k`` commitment with ``k >= 2`` registered in the lattice
(S1.5b.22's :func:`_record_setnode` site), the check forks
``root_kb`` for every ``(k-1)``-subset parent, adds the missing
hypothesis, saturates, and asserts the resulting state_hash
matches a direct :func:`try_commitment_set(root_kb, C)`.
:exc:`SanityError` is raised on mismatch with the offending
state_hashes attached for the caller to diagnose.

Cost is ``k+1`` saturations per checked commitment (one for the
direct path, one per parent). That's why the flag is off by
default — release regression only.
"""
from __future__ import annotations

from dataclasses import dataclass

from ein_bot.inference.apriori import CanonicalSetId
from ein_bot.inference.canon import state_hash
from ein_bot.inference.commitment import try_commitment_set
from ein_bot.inference.saturator import Saturator
from ein_bot.kb.entities import Fact, Layer
from ein_bot.kb.provenance import Provenance
from ein_bot.kb.store import KnowledgeBase


@dataclass(frozen=True)
class SanityError(Exception):
    """Raised by :func:`check_commutativity` when two lattice
    paths to the same commitment produce kbs with distinct
    state_hashes.

    Attributes
    ----------
    commitment
        The size-``k`` commitment whose parent paths diverged.
    direct_state_hash
        ``state_hash`` of ``try_commitment_set(root, C).kb``
        (the engine's actual saturation path).
    parent_state_hashes
        Dict mapping each ``(k-1)``-subset parent to the
        ``state_hash`` of ``parent_result.kb`` + missing
        hypothesis + re-saturation. Only mismatching parents
        are listed.
    """

    commitment: CanonicalSetId
    direct_state_hash: int
    parent_state_hashes: tuple[tuple[CanonicalSetId, int], ...]

    def __str__(self) -> str:
        parent_lines = "\n".join(
            f"    {p!r} -> {h:#x}"
            for p, h in self.parent_state_hashes
        )
        return (
            "Saturation commutativity violated for "
            f"{self.commitment!r}\n"
            f"  direct state_hash = {self.direct_state_hash:#x}\n"
            f"  parent paths:\n{parent_lines}"
        )


def check_commutativity(
    root_kb: KnowledgeBase,
    commitment: CanonicalSetId,
) -> None:
    """Verify saturation commutativity for one ``commitment``
    against the current ``root_kb``.

    For a size-``k`` commitment with ``k >= 2``:

    1. Run ``try_commitment_set(root_kb, commitment)`` — the
       direct path. Skip the check if the commitment is
       ``dead-pre`` (the unsaturated fork has no canonical
       state to compare against).
    2. For each ``(k-1)``-subset parent ``P`` of ``commitment``:
       - Run ``try_commitment_set(root_kb, P)``.
       - Skip ``P`` if it's dead (no lattice path exists
         through a dead parent).
       - Fork ``parent_result.kb``, add the missing element
         from ``commitment \\ P``, saturate.
       - Hash the result.
    3. Assert every parent path's hash equals the direct
       path's hash. Raise :exc:`SanityError` on mismatch.

    No-op for ``len(commitment) < 2`` — singletons have no
    parents and the check is trivially satisfied.
    """
    if len(commitment) < 2:
        return

    direct = try_commitment_set(root_kb, commitment)
    if direct.kind == "dead-pre":
        # No saturated fork to compare against. The direct
        # path's failure-to-saturate is itself a deterministic
        # signal (pre-saturation contradiction depends only on
        # ``root + C`` set-union fact-equality), so no commutativity
        # check applies.
        return
    direct_hash = state_hash(direct.kb)

    mismatches: list[tuple[CanonicalSetId, int]] = []
    for i in range(len(commitment)):
        parent = tuple(
            c for j, c in enumerate(commitment) if j != i
        )
        missing = commitment[i]
        parent_result = try_commitment_set(root_kb, parent)
        if parent_result.kind in ("dead-pre", "dead-post"):
            # Dead parent — the lattice path through this
            # parent does not exist; skip rather than fail.
            continue
        rn, args = missing
        h_fact = Fact(
            relation_name=rn, args=args,
            layer=Layer.REASONING,
            provenance=Provenance.from_hypothesis(branch=0),
        )
        fork = parent_result.kb.fork()
        fork.add_and_index_fact(h_fact)
        _ = list(Saturator(fork).saturate())
        parent_hash = state_hash(fork)
        if parent_hash != direct_hash:
            mismatches.append((parent, parent_hash))

    if mismatches:
        raise SanityError(
            commitment=commitment,
            direct_state_hash=direct_hash,
            parent_state_hashes=tuple(mismatches),
        )


__all__ = ["SanityError", "check_commutativity"]
