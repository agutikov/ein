"""Unconditional-death analysis + back-prop write — S1.5.7 T1.5.7.1 / .2.

A hypothesis branch dies *unconditionally* when its contradiction
rests on no speculative fact other than the branch's own hypothesis
``h``: the death proves ``¬h`` in the parent node's context, so
``(not h)`` may be written into the parent KB (T1.5.7.2) — where it
becomes an O(1) ``_negated_facts`` filter entry every sibling and
descendant inherits.

A *conditional* death — one whose contradiction additionally rests on
some **other** hypothesis — is held back from the (irreversible)
parent write by the ``enable_back_prop_unconditional`` gate.

``unsat_core`` membership
-------------------------
:meth:`KnowledgeBase.unsat_core` walks each conflict witness's
derivation DAG to its ``source`` / ``hypothesis`` / un-provenanced
terminals. A dead branch's core therefore *always* contains the
branch's own seeded hypothesis ``h`` (a hypothesis-kind terminal of
the conflict). :func:`is_unconditional_death` is told which
hypothesis is ``h`` via ``own_hypothesis`` and treats it as a benign
terminal — otherwise no death would ever read as unconditional.

The remaining judgement is a transitive premise walk. The shallow
read — *"the core contains no ``kind='hypothesis'`` fact"* — is
unsound: a core fact can be ``kind='rule'`` yet derive, through a
chain of firings, from a hypothesis. :func:`reaches_hypothesis`
follows ``Provenance.premises_raw`` (resolved against the KB) until
every chain grounds out at a ``source`` / un-provenanced given, or
any chain reaches a ``hypothesis`` / ``rejected`` terminal that is
not ``own``.

T1.5.7.6 reuses :func:`reaches_hypothesis` (no ``own``) to classify a
fact derived by parent re-saturation as a *forced deduction* vs a
hypothesis-dependent one.
"""
from __future__ import annotations

from ein_bot.kb.entities import Fact, Layer
from ein_bot.kb.provenance import FactId, Provenance
from ein_bot.kb.store import KnowledgeBase

# Provenance kinds marking a fact as speculative — introduced on a
# hypothesis branch rather than given (`source`) or rule-derived.
# `rejected` (a retracted hypothesis) counts: it is still branch-local.
_SPECULATIVE_KINDS = frozenset({"hypothesis", "rejected"})


def _walk(
    kb: KnowledgeBase, fact: Fact, visited: set[FactId],
    own: FactId | None,
) -> bool:
    """Recursive core: True iff ``fact``'s premise chain touches a
    speculative fact *other than* ``own``.

    ``own`` is the FactId of the branch's own hypothesis — expected
    in its death's witness set (it is what ``¬`` is being proven of,
    not an external assumption), so a speculative fact equal to
    ``own`` is a benign terminal.

    ``visited`` guards provenance cycles and memoises across sibling
    walks: a fact left in ``visited`` was reached on a chain that
    did not (yet) yield a non-``own`` hypothesis, so a revisit
    contributes nothing — return False. Sound because a chain that
    *does* reach one short-circuits every caller above it.
    """
    key: FactId = (fact.relation_name, fact.args)
    if key in visited:
        return False
    visited.add(key)
    prov = fact.provenance
    if prov is None:
        return False                       # un-provenanced — a given
    if prov.kind in _SPECULATIVE_KINDS:
        return key != own                  # hypothesis terminal
    if prov.kind == "rule":
        for rid in prov.premises_raw:
            premise = kb._fact_by_id(*rid)
            if premise is not None and _walk(kb, premise, visited, own):
                return True
        return False
    return False                           # source-kind — a given


def reaches_hypothesis(kb: KnowledgeBase, fact: Fact) -> bool:
    """True iff ``fact`` transitively depends on any hypothesis-kind fact.

    The single-fact walk T1.5.7.6 reuses to classify a
    re-saturation-derived fact as forced (False) vs hypothesis-
    dependent (True). No ``own`` exemption — every hypothesis counts.

    A ``rule``-kind premise id absent from ``kb`` is skipped; callers
    must pass a KB in which the derivation chain is fully present.
    """
    return _walk(kb, fact, set(), own=None)


def is_unconditional_death(
    kb: KnowledgeBase,
    unsat_core: frozenset[Fact],
    *,
    own_hypothesis: Fact | None = None,
) -> bool:
    """True iff a dead branch's contradiction rests on no hypothesis
    beyond the branch's own.

    ``unsat_core`` is the conflict's source-frontier
    (``BranchResult.unsat_core``, from ``kb.unsat_core``).
    ``own_hypothesis`` is the branch's seeded hypothesis — always
    present in its own core, and exempted from the count (see the
    module docstring). The death is unconditional iff no *other*
    core fact's premise chain reaches a speculative terminal — then
    ``¬own_hypothesis`` holds in the parent's context and
    ``(not h)`` may be back-propagated.

    An empty ``unsat_core`` returns False (treated conditional): a
    real contradiction always grounds out at a non-empty frontier,
    so an empty one signals an analysis gap — and back-prop,
    irreversible against the parent KB, must not fire on an
    unattributable death.
    """
    if not unsat_core:
        return False
    own: FactId | None = (
        (own_hypothesis.relation_name, own_hypothesis.args)
        if own_hypothesis is not None else None
    )
    visited: set[FactId] = set()
    return not any(_walk(kb, f, visited, own) for f in unsat_core)


def is_symmetric_relation(kb: KnowledgeBase, name: str) -> bool:
    """True iff ``(symmetric <name>)`` is asserted in ``kb`` (any layer).

    Activator-style read of the symmetric property tag — the same
    check ``hypgen`` uses; lifted here so the back-prop write site
    can promote the symmetric mirror without importing across
    sibling modules.
    """
    apps = kb._facts_by_relation.get("symmetric", ())
    return any(f.args == (name,) for f in apps)


def _write_negation(
    kb: KnowledgeBase, hypothesis: Fact,
    unsat_core: frozenset[Fact], rule_name: str,
) -> Fact:
    """Write one ``(not hypothesis)`` fact with the given provenance
    rule name; idempotent — a pre-existing ``(not h)`` is returned
    untouched."""
    existing = kb._fact_by_id("not", (hypothesis,))
    if existing is not None:
        return existing
    not_fact = Fact(
        relation_name="not",
        args=(hypothesis,),
        layer=Layer.REASONING,
        provenance=Provenance.from_rule(
            rule=rule_name,
            premises_raw=tuple(
                (f.relation_name, f.args) for f in unsat_core
            ),
        ),
    )
    stored = kb.add_fact(not_fact)
    kb._index_fact(stored)
    return stored


def back_propagate(
    kb: KnowledgeBase, hypothesis: Fact, unsat_core: frozenset[Fact],
    *,
    rule_name: str = "<back-prop-unconditional>",
    promote_symmetric: bool = True,
) -> Fact:
    """Write ``(not hypothesis)`` into ``kb`` on an unconditional death.

    The negation is a REASONING-layer ``(not <hypothesis>)`` fact
    with rule-provenance citing the ``unsat_core`` frontier — so its
    derivation walks back to the same given facts the death rested
    on. Indexing it updates ``kb._negated_facts``, so every
    subsequent ``_candidates_for`` / ``_prune_alive`` drops
    ``hypothesis`` in O(1), and ``kb.fork()`` carries the exclusion
    to descendant branches.

    ``rule_name`` distinguishes the back-prop's origin in traces and
    audits (T1.5.7.2 ``<back-prop-unconditional>`` for consume-loop
    kills, T1.5.7.4 ``<lookahead-dies-immediately>`` for S1.5.6
    lookahead kills).

    With ``promote_symmetric=True`` (the default — T1.5.7.3) the
    symmetric counterpart ``(not (R b a))`` is *also* written when
    ``(symmetric R)`` is asserted and ``hypothesis`` is a 2-arg
    fact with distinct arguments. The symmetric counterpart's death
    is unconditional under the same reasoning — sound to cache
    proactively, saving a redundant ``try_branch`` on the next pass.

    Idempotent: a pre-existing ``(not h)`` is returned untouched.

    Returns the *primary* stored ``(not …)`` Fact.
    """
    primary = _write_negation(kb, hypothesis, unsat_core, rule_name)
    if (promote_symmetric
            and len(hypothesis.args) == 2
            and hypothesis.args[0] != hypothesis.args[1]
            and is_symmetric_relation(kb, hypothesis.relation_name)):
        mirror = Fact(
            relation_name=hypothesis.relation_name,
            args=(hypothesis.args[1], hypothesis.args[0]),
            layer=Layer.REASONING,
        )
        _write_negation(kb, mirror, unsat_core, rule_name)
    return primary


__all__ = [
    "back_propagate",
    "is_symmetric_relation",
    "is_unconditional_death",
    "reaches_hypothesis",
]
