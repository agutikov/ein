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

from ein_bot.ir.types import Loc
from ein_bot.render.dot_util import fact_key, hashed_id

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
    ) -> Provenance:
        return cls(
            kind="rule", rule=rule,
            premises_raw=premises_raw, bindings=bindings, loc=loc,
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
    """
    root: Fact
    nodes: tuple[Fact, ...]
    edges: tuple[tuple[Fact, Fact], ...]

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
) -> DerivationDAG:
    """BFS over `premises_raw`, resolving ids via the `resolve` callback.

    `resolve` is a callable ``(rel: str, args: tuple) -> Fact | None``
    — supplied by the owning :class:`KnowledgeBase` so this module
    needn't import store.py at runtime.

    Cycles are broken by tracking visited facts; the revisited fact
    appears as a node but is not re-expanded.
    """
    nodes: list[Fact] = [root]
    edges: list[tuple[Fact, Fact]] = []
    seen: set[FactId] = {(root.relation_name, root.args)}
    queue: list[Fact] = [root]
    while queue:
        f = queue.pop(0)
        if f.provenance is None or f.provenance.kind != "rule":
            continue
        for rid in f.provenance.premises_raw:
            premise = resolve(*rid)
            if premise is None:
                continue
            edges.append((premise, f))
            key = (premise.relation_name, premise.args)
            if key not in seen:
                seen.add(key)
                nodes.append(premise)
                queue.append(premise)
    return DerivationDAG(root=root, nodes=tuple(nodes), edges=tuple(edges))


def detect_provenance_cycles(facts: Iterable[Fact], resolve: object) -> list[list[FactId]]:
    """Return any cycles in the provenance graph (empty list if none).

    Used at load time; if non-empty, the loader raises
    :class:`KBLoadError` with the cycle path.
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


# ── Provenance-chain reachability ─────────────────────────────────


def reaches(
    fact: Fact,
    visited: set[FactId],
    resolve: object,
    is_terminal: Callable[[FactId, Fact], bool | None],
) -> bool:
    """Provenance-chain DFS: True iff some premise chain from ``fact``
    reaches a *terminal* fact, by the caller's ``is_terminal`` test.

    ``is_terminal(key, fact)`` returns ``True`` / ``False`` to
    short-circuit at ``fact`` (it is a terminal), or ``None`` to keep
    walking its ``rule``-kind premises. ``resolve(rel, args) -> Fact |
    None`` looks up a premise id (as in :func:`build_derivation_dag`),
    so this module needn't import ``store.py``.

    ``visited`` guards provenance cycles and memoises across sibling
    walks: a fact left in ``visited`` was reached on a chain that did
    not (yet) yield a terminal, so a revisit contributes nothing — sound
    because a chain that *does* reach one short-circuits every caller
    above it. Pass a shared set to memoise across several roots.

    Factored (F-KER-10) so the caller supplies the terminal test — e.g.
    commitment's ``_is_unconditional`` (commitment terminal).
    """
    key: FactId = (fact.relation_name, fact.args)
    if key in visited:
        return False
    visited.add(key)
    terminal = is_terminal(key, fact)
    if terminal is not None:
        return terminal
    prov = fact.provenance
    if prov is None or prov.kind != "rule":
        return False
    for rid in prov.premises_raw:
        premise = resolve(*rid)
        if premise is not None and reaches(premise, visited, resolve, is_terminal):
            return True
    return False


def walk_premises(
    root: Fact,
    resolve: Callable[[str, tuple[object, ...]], Fact | None],
    *,
    keep: Callable[[FactId, Fact], bool],
    visited: set[FactId] | None = None,
) -> set[Fact]:
    """Collect every fact in ``root``'s transitive premise closure for which
    ``keep`` is True — the **set-collecting dual** of :func:`reaches` (E6).

    Where :func:`reaches` short-circuits at the first terminal and returns a
    ``bool``, this walks the *whole* closure over rule-kind ``premises_raw``
    and accumulates each kept fact. ``resolve(rel, args) -> Fact | None`` looks
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
        prov = f.provenance
        if prov is not None and prov.kind == "rule":
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
    "reaches",
    "walk_premises",
]
