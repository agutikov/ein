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

from collections.abc import Iterable
from dataclasses import dataclass, field
from typing import TYPE_CHECKING

from ein_bot.ir.types import Loc

if TYPE_CHECKING:
    from .entities import Fact


# ── Type aliases ───────────────────────────────────────────────────


# A fact id is its (relation_name, args) identity tuple — sufficient
# because Fact.__eq__ / __hash__ are exactly on these two fields.
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
    key = f"{f.relation_name}|" + ",".join(str(a) for a in f.args)
    # DOT identifiers must match [A-Za-z_][A-Za-z0-9_]* ; use a
    # surrogate hex hash.
    import hashlib
    return "f_" + hashlib.md5(key.encode()).hexdigest()[:12]


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


__all__ = [
    "DerivationDAG",
    "FactId",
    "Provenance",
    "build_derivation_dag",
    "detect_provenance_cycles",
]
