"""Per-fact provenance + derivation DAG — S1.2.3.

Every :class:`Fact` carries an optional :class:`Provenance` record
that says *where it came from*:

- ``kind='source'`` — ingested from the IR with a ``:source "(N)"``
  annotation (FACT layer) or from the ontology block (ONTOLOGY layer
  — `source` is then the conventional sentence id for the ontology
  origin, or None for purely-implicit facts like instance
  declarations).
- ``kind='rule'`` — derived by a rule firing. ``rule`` names the
  firing rule; ``premises_raw`` is the tuple of (rel, args) fact ids
  the rule consumed; ``bindings`` records the var-to-name binding
  used. The Fact's ``.premises`` property resolves raw ids to Fact
  objects through the owning KB.
- ``kind='hypothesis'`` — speculative branch introduction.
  ``branch`` is an integer branch id assigned by the hypothesis loop
  (P1.5).
- ``kind='rejected'`` — a hypothesis that was contradicted; kept for
  the trace renderer (P1.6) and the *contradictions* task class
  (idea 03).

The full derivation DAG falls out by transitive closure over
``premises_raw`` — see :meth:`KnowledgeBase.derivation_dag`. This is
the substrate that makes the *contradictions* task class
human-readable and that the trace generator reads to produce its
"It follows that …" narrative (idea 08).

Granularity: per-fact (M1 Q18 working answer).
"""
from __future__ import annotations

from collections.abc import Callable, Iterable
from dataclasses import dataclass, field
from typing import TYPE_CHECKING

from ein.ir.types import Loc
from ein.render.dot_util import fact_key, hashed_id

if TYPE_CHECKING:
    from .entities import Fact


# ── Type aliases ───────────────────────────────────────────────────


# A fact id is its (relation_name, args) identity tuple — sufficient
# because Fact.__eq__ / __hash__ are exactly on these two fields.
# Single home for the alias (F-KER-6): inference/{apriori,nogoods,
# solution,commitment}, monotonic/lattice and the render
# DAGs all import it from here — kb sits below inference, so this is
# the one module both layers can share without an import cycle.
FactId = tuple[str, tuple[object, ...]]


# ── Provenance ─────────────────────────────────────────────────────


@dataclass(frozen=True)
class Provenance:
    """Per-fact provenance — where the fact came from.

    All optional fields default to None / ``()`` so a ``source``-kind
    record needs only ``kind`` and ``source`` set; a ``rule``-kind
    record needs ``rule`` + ``premises_raw``; etc.
    """
    kind: str
    # source-kind
    source: str | None = None
    # rule-kind
    rule: str | None = None
    premises_raw: tuple[FactId, ...] = ()
    bindings: tuple[tuple[str, str], ...] = ()
    # rule-kind, S1.21.8 — NEGATIVE premises: the `(absent …)` queries that
    # had to fail on the closure/world boundary for this firing to be
    # admitted. Each entry is a `(relation, args)` pattern with `None` where
    # the query ranged free (`ein.inference.world.NafRef`).
    #
    # This is the missing half of `Deps(Y)`, the union of `PositiveDeps(Y)`
    # and `NegativeDeps(Y)` (REVIEW_M1-01 §2). Without it a firing's
    # dependence on an *absence* is invisible to every provenance walk — C2 of
    # `docs/kernel/inference/absent_semantics.md`, and the root cause behind
    # both the retired `unconditional_facts` extraction (a fact resting on an
    # absence is not unconditionally true) and the NAF-unsoundness of
    # deletion-based core minimisation (dropping a fact can flip an absence
    # and fabricate a contradiction the full KB never had, C3).
    #
    # Recording it does NOT by itself make those revisitable: it makes the
    # dependence *visible*, which is the precondition. A walk that wants to
    # honour it must decide what a negative premise means for its own
    # question — see the frontier discussion in `absent_semantics.md`.
    absent_premises: tuple[tuple[str, tuple[object, ...]], ...] = ()
    # hypothesis-kind
    branch: int | None = None
    # IR location — excluded from compare/hash (metadata).
    loc: Loc | None = field(default=None, compare=False, hash=False, repr=False)

    # ── Convenience constructors ──────────────────────────────────

    @classmethod
    def from_source(cls, source: str | None, loc: Loc | None = None) -> Provenance:
        return cls(kind="source", source=source, loc=loc)

    @classmethod
    def from_rule(
        cls,
        rule: str,
        premises_raw: tuple[FactId, ...] = (),
        bindings: tuple[tuple[str, str], ...] = (),
        loc: Loc | None = None,
        absent_premises: tuple[tuple[str, tuple[object, ...]], ...] = (),
    ) -> Provenance:
        return cls(
            kind="rule", rule=rule,
            premises_raw=premises_raw, bindings=bindings, loc=loc,
            absent_premises=absent_premises,
        )

    @classmethod
    def from_hypothesis(cls, branch: int, loc: Loc | None = None) -> Provenance:
        return cls(kind="hypothesis", branch=branch, loc=loc)

    @classmethod
    def rejected(cls, branch: int, loc: Loc | None = None) -> Provenance:
        return cls(kind="rejected", branch=branch, loc=loc)


# ── Derivation DAG ─────────────────────────────────────────────────


@dataclass(frozen=True)
class DerivationDAG:
    """A directed acyclic graph of fact derivations.

    Constructed by :meth:`KnowledgeBase.derivation_dag(fact)`. Nodes
    are :class:`Fact` instances; edges go from premise to conclusion
    (``(premise, conclusion)``). The DAG terminates at facts whose
    provenance kind is ``'source'`` or ``'hypothesis'`` (or whose
    provenance is missing entirely — orphan-source case).

    Cycles encountered during construction are *broken* — the
    revisited fact is not re-expanded — so the result is always a
    DAG. Cycles in user-authored provenance are also caught at load
    time and rejected with :class:`KBLoadError`.

    ``and_nodes`` (S1.21.7) carries the conjunction structure ``edges``
    cannot: one entry per *justification*, pairing a conclusion with the
    premises of that one derivation. A flat premise→conclusion edge set
    loses this — with several justifications the in-edges of a node are the
    union over derivations, and which subset constitutes one proof is
    unrecoverable. When a conclusion appears in more than one entry the DAG
    is an AND/**OR** graph (:attr:`is_or_graph`), and :meth:`to_dot` then
    draws a node per justification so the grouping stays legible.
    """
    root: Fact
    nodes: tuple[Fact, ...]
    edges: tuple[tuple[Fact, Fact], ...]
    and_nodes: tuple[tuple[Fact, tuple[Fact, ...]], ...] = ()

    @property
    def is_or_graph(self) -> bool:
        """True iff some fact here has more than one recorded derivation."""
        seen: set[FactId] = set()
        for conclusion, _premises in self.and_nodes:
            key: FactId = (conclusion.relation_name, conclusion.args)
            if key in seen:
                return True
            seen.add(key)
        return False

    @property
    def sources(self) -> tuple[Fact, ...]:
        """Terminal facts: kind='source' or kind='hypothesis'.

        These are the *frontier* — what the engine considers
        *given* rather than derived. ``kb.unsat_core`` returns the
        union of source-kind terminals across a set of conflicting
        facts.
        """
        return tuple(
            n for n in self.nodes
            if n.provenance is None
            or n.provenance.kind in ("source", "hypothesis")
        )

    def to_dot(self) -> str:
        """Render the DAG as a Graphviz ``digraph`` string.

        - Source/hypothesis facts: ``ellipse`` with the source
          sentence (e.g. "condition (10)") or "hypothesis #N".
        - Rule-derived facts: ``box`` labelled with the rule name and
          the fact's compact form ``(rel args)``.

        An AND/OR graph (:attr:`is_or_graph`) additionally draws one small
        ``diamond`` per justification, with the premises feeding *it* and it
        feeding the conclusion — so alternative derivations read as
        alternatives instead of as one big conjunction. A single-derivation
        DAG renders exactly as it did before S1.21.7.
        """
        lines: list[str] = ['digraph derivation {', '  rankdir=BT;']
        for f in self.nodes:
            nid = _fact_dot_id(f)
            label = _fact_dot_label(f)
            if (f.provenance is None
                    or f.provenance.kind in ("source", "hypothesis")):
                lines.append(f'  {nid} [shape=ellipse, label="{label}"];')
            else:
                lines.append(f'  {nid} [shape=box, label="{label}"];')
        if self.is_or_graph:
            for i, (conclusion, premises) in enumerate(self.and_nodes):
                jid = f"j{i}"
                lines.append(
                    f'  {jid} [shape=diamond, width=.2, height=.2, label=""];')
                for p in premises:
                    lines.append(f"  {_fact_dot_id(p)} -> {jid};")
                lines.append(f"  {jid} -> {_fact_dot_id(conclusion)};")
        else:
            for premise, conclusion in self.edges:
                pid = _fact_dot_id(premise)
                cid = _fact_dot_id(conclusion)
                lines.append(f"  {pid} -> {cid};")
        lines.append("}")
        return "\n".join(lines)

    def __len__(self) -> int:
        return len(self.nodes)

    def __iter__(self):
        return iter(self.nodes)


# ── DOT id helpers (private) ───────────────────────────────────────


def _fact_dot_id(f: Fact) -> str:
    """Stable DOT node id for a Fact — derived from its identity."""
    # Hash the (relation, args) tuple deterministically.
    # S1.7c.25 — the shared ``f_<md5[:10]>`` identity scheme
    # (``dot_util.hashed_id`` + ``fact_key``); was a local [:12] md5 (F-KB-8).
    return hashed_id("f_", fact_key(f.relation_name, f.args))


def _fact_dot_label(f: Fact) -> str:
    """Short human-readable label for a Fact in a DAG node."""
    compact = f"({f.relation_name} {' '.join(str(a) for a in f.args)})"
    if f.provenance is None:
        return _esc(compact)
    if f.provenance.kind == "source":
        return _esc(f"{compact}\\n[{f.provenance.source or 'ontology'}]")
    if f.provenance.kind == "rule":
        return _esc(f"{compact}\\n[rule: {f.provenance.rule}]")
    if f.provenance.kind == "hypothesis":
        return _esc(f"{compact}\\n[hyp #{f.provenance.branch}]")
    return _esc(compact)


def _esc(s: str) -> str:
    """Escape DOT label specials."""
    return s.replace('"', '\\"')


# ── BFS construction (used by KnowledgeBase.derivation_dag) ───────


def build_derivation_dag(
    root: Fact,
    resolve: object,
    *,
    justifications: Callable[[Fact], tuple[Provenance, ...]] | None = None,
) -> DerivationDAG:
    """BFS over `premises_raw`, resolving ids via the `resolve` callback.

    `resolve` is a callable ``(rel: str, args: tuple) -> Fact | None``
    — supplied by the owning :class:`KnowledgeBase` so this module
    needn't import store.py at runtime.

    Cycles are broken by tracking visited facts; the revisited fact
    appears as a node but is not re-expanded.

    ``justifications`` (S1.21.7) selects which derivations to expand — see
    :func:`justifications_of`; the default is primary-only, so the DAG shape
    and its DOT rendering are unchanged. Pass
    :meth:`KnowledgeBase.justifications` for the AND/OR graph, whose
    conjunction structure lands in :attr:`DerivationDAG.and_nodes` (``edges``
    is still emitted, as the flattened union, for readers that only want
    reachability).
    """
    nodes: list[Fact] = [root]
    edges: list[tuple[Fact, Fact]] = []
    and_nodes: list[tuple[Fact, tuple[Fact, ...]]] = []
    seen: set[FactId] = {(root.relation_name, root.args)}
    queue: list[Fact] = [root]
    while queue:
        f = queue.pop(0)
        for prov in justifications_of(f, justifications):
            if prov.kind != "rule":
                continue
            group: list[Fact] = []
            for rid in prov.premises_raw:
                premise = resolve(*rid)
                if premise is None:
                    continue
                group.append(premise)
                edges.append((premise, f))
                key = (premise.relation_name, premise.args)
                if key not in seen:
                    seen.add(key)
                    nodes.append(premise)
                    queue.append(premise)
            and_nodes.append((f, tuple(group)))
    return DerivationDAG(
        root=root, nodes=tuple(nodes), edges=tuple(edges),
        and_nodes=tuple(and_nodes),
    )


def detect_provenance_cycles(facts: Iterable[Fact], resolve: object) -> list[list[FactId]]:
    """Return any cycles in the provenance graph (empty list if none).

    Used at load time; if non-empty, the loader raises
    :class:`KBLoadError` with the cycle path.

    Deliberately **primary-justification only**, and deliberately load-time
    only (S1.21.7). A cycle in *user-authored* provenance is a malformed
    input and rightly rejects the KB. A cycle in *engine-recorded*
    provenance is normal — once re-derivations are recorded, symmetric and
    transitive closure make `(R a b)` and `(R b a)` justify each other in
    any ordinary puzzle — so running this over a saturated KB would reject
    well-founded knowledge bases. Consumers that need to reason over the
    recorded AND/OR graph must handle cycles themselves; see
    :mod:`ein.inference.explain`, whose least fixpoint from the frontier up
    never grounds a fact in itself.
    """
    # Build adjacency on (rel, args) keys: premise -> conclusion.
    out: list[list[FactId]] = []
    visited: set[FactId] = set()
    stack_path: list[FactId] = []
    stack_set: set[FactId] = set()

    def dfs(f: Fact) -> None:
        key: FactId = (f.relation_name, f.args)
        if key in stack_set:
            # Found a cycle — extract from the path.
            i = stack_path.index(key)
            out.append([*stack_path[i:], key])
            return
        if key in visited:
            return
        visited.add(key)
        stack_path.append(key)
        stack_set.add(key)
        if f.provenance is not None and f.provenance.kind == "rule":
            for rid in f.provenance.premises_raw:
                premise = resolve(*rid)
                if premise is not None:
                    dfs(premise)
        stack_path.pop()
        stack_set.discard(key)

    for f in facts:
        dfs(f)
    return out


def justifications_of(
    fact: Fact,
    justifications: Callable[[Fact], tuple[Provenance, ...]] | None,
) -> tuple[Provenance, ...]:
    """The justifications a walk should expand for ``fact`` (S1.21.7).

    ``justifications is None`` — the default everywhere — selects the
    **primary** justification only (``Fact.provenance``), which is the
    pre-S1.21.7 single-derivation reading every consumer was written
    against. Passing :meth:`KnowledgeBase.justifications` makes the walk
    OR-aware: the fact becomes an OR-node over every recorded derivation.

    This exists so the choice is *explicit* at each call site. Before
    S1.21.7 every walker took the first-recorded derivation by accident, as
    a side effect of the dedup seam; now a walker either says "primary" or
    says "all", and the docstring at that call site has to justify it.
    """
    if justifications is not None:
        return justifications(fact)
    return (fact.provenance,) if fact.provenance is not None else ()


def walk_premises(
    root: Fact,
    resolve: Callable[[str, tuple[object, ...]], Fact | None],
    *,
    keep: Callable[[FactId, Fact], bool],
    visited: set[FactId] | None = None,
    justifications: Callable[[Fact], tuple[Provenance, ...]] | None = None,
) -> set[Fact]:
    """Collect every fact in ``root``'s transitive premise closure for which
    ``keep`` is True (E6) — the shared premise-closure walk.

    Walks the *whole* closure over rule-kind ``premises_raw`` and accumulates
    each kept fact. (Its boolean short-circuiting sibling ``reaches`` — a
    terminal-test DFS — was retired with its sole caller, the P1.21 R2
    unconditional-fact extraction.) ``resolve(rel, args) -> Fact | None`` looks
    up a premise id (as in :func:`build_derivation_dag`), so this module needn't
    import ``store.py``.

    ``keep(key, fact)`` decides set membership only; it does **not** stop the
    walk — rule-kind facts are always expanded. A predicate selecting the
    derivation *frontier* (``provenance is None or kind in {"source",
    "hypothesis"}``) reproduces :meth:`KnowledgeBase.unsat_core`; this is the
    shared substrate for the unsat-core frontier and the core minimiser (E19).

    ``visited`` guards provenance cycles and memoises across roots: pass a
    shared set to collect the frontier across several conflicting facts in one
    pass (the union is identical to walking each separately — a fact is kept
    iff reachable from any root). Iterative (explicit stack) so a deep
    derivation chain can't blow the recursion limit.

    ``justifications`` (S1.21.7) selects which derivations to expand — see
    :func:`justifications_of`. The default (primary only) is what every
    existing caller wants. With an OR-aware resolver the result is the union
    over *all* recorded derivations, which is a strictly larger frontier:
    useful as a soundness envelope ("no explanation can name a fact outside
    this"), but **not** an explanation itself — no single derivation used all
    of those premises. For a minimum-cardinality answer use
    :mod:`ein.inference.explain`, which chooses one justification per fact
    instead of unioning them.

    ``visited`` stays sound under an OR walk for the *union* reading — a
    fact is kept iff reachable through some derivation — but it is not a
    per-environment walk, which is exactly why the minimality search does
    not reuse this function.
    """
    if visited is None:
        visited = set()
    out: set[Fact] = set()
    stack: list[Fact] = [root]
    while stack:
        f = stack.pop()
        key: FactId = (f.relation_name, f.args)
        if key in visited:
            continue
        visited.add(key)
        if keep(key, f):
            out.add(f)
        for prov in justifications_of(f, justifications):
            if prov.kind != "rule":
                continue
            for rid in prov.premises_raw:
                premise = resolve(*rid)
                if premise is not None:
                    stack.append(premise)
    return out


__all__ = [
    "DerivationDAG",
    "FactId",
    "Provenance",
    "build_derivation_dag",
    "detect_provenance_cycles",
    "justifications_of",
    "walk_premises",
]
