"""KnowledgeBase — entity registry + reverse indexes — S1.2.1 T1.2.1.2/3.

The KB owns:

- registries keyed by name (``relations``, ``rules``, ``hrules``) and one
  fact list (``facts``). S1.7.23 — there are **no** ``types`` /
  ``instances`` registries: the kernel imposes no type system, so the
  inheritance forest is just ``is-a`` facts with no derived entity-view;
- the reverse indexes (``_facts_by_relation``, ``_rules_by_relation``,
  ``_rule_apps_by_rule`` / ``_rule_apps_on_relation``, ``names``) covering
  the cross-references documented in
  ``M1 S1.2.1``;
- an optional :class:`Query` slot (set when an IR file contains
  ``(query …)``);
- a placeholder :class:`EqClasses` union-find — reserved for F4
  e-graph promotion (T1.2.1.6).

The entity classes expose those cross-references as ``@property``
accessors that delegate here via the entity's ``_kb`` back-pointer;
the indexes are precomputed in :meth:`KnowledgeBase._rebuild_indexes`.

Loading IR is in :mod:`ein.kb.from_ir`; this module is purely
the in-memory store.
"""
from __future__ import annotations

from collections import defaultdict
from collections.abc import Iterable
from dataclasses import dataclass
from typing import TYPE_CHECKING

from ein import events

from .entities import (
    KERNEL_META_RELATIONS,
    Fact,
    NameRef,
    Relation,
    Rule,
    _attach,
)

if TYPE_CHECKING:
    from ..inference.config import SolverConfig
    from ..ir.macros import Macro
    from ..ir.types import KwPair
    from .provenance import DerivationDAG, FactId, Provenance
    from .views import FactView

# S1.21.7 — per-fact cap on recorded ALTERNATIVE justifications. The proof
# graph is an AND/OR graph and a hot fact can be re-derived hundreds of times
# (an exhaustive `zebra2` solve makes ~194k redundant firings); keeping every
# one costs memory for environments that repeat. The list is kept sorted by
# premise count, so the cap retains the shortest — the ones a minimum-
# cardinality explanation search can actually use. See
# :meth:`KnowledgeBase.record_justification`.
MAX_ALT_JUSTIFICATIONS = 32


# ── Equality-class hooks (T1.2.1.6 placeholder) ───────────────────


class EqClasses:
    """A union-find over instance names. M1 placeholder.

    The engine does *not* yet fire equality propagation; this exists
    so that `(= ?a ?b)` rule conclusions can call ``kb.classes.union``
    without surrounding code asking "what union-find?". F4 promotes
    this into an e-graph when the need arrives.
    """
    def __init__(self) -> None:
        self._parent: dict[str, str] = {}

    def find(self, x: str) -> str:
        # Auto-vivify.
        if x not in self._parent:
            self._parent[x] = x
            return x
        # Path-compress.
        root = x
        while self._parent[root] != root:
            root = self._parent[root]
        cur = x
        while self._parent[cur] != root:
            self._parent[cur], cur = root, self._parent[cur]
        return root

    def union(self, a: str, b: str) -> str:
        ra, rb = self.find(a), self.find(b)
        if ra != rb:
            self._parent[rb] = ra
        return ra

    def equivalent(self, a: str, b: str) -> bool:
        return self.find(a) == self.find(b)

    def classes(self) -> dict[str, list[str]]:
        out: dict[str, list[str]] = defaultdict(list)
        for x in self._parent:
            out[self.find(x)].append(x)
        return dict(out)


# ── Query placeholder (T1.2.1.4 step 8) ───────────────────────────


@dataclass(frozen=True)
class Query:
    """A `(query …)` block lifted into a small entity.

    The body keeps the raw `KwPair` tuple — interpretation lives in
    P1.5 (hypothesis loop), which reads ``mode`` and ``goal``.
    """
    kw_pairs: tuple[KwPair, ...] = ()


# ── KnowledgeBase ──────────────────────────────────────────────────


class KnowledgeBase:
    """The entity registry + reverse indexes.

    Conceptually the underlying form is the canonical graph (every
    Type / Instance / Relation / Rule / Fact is a node); the registry
    dicts and reverse indexes are *cached lookups* into that graph.
    Different views (P1.2 S1.2.2) and the unified DOT rendering
    (P1.2 S1.2.4) read the same indexes.
    """

    def __init__(self) -> None:
        # Registries (entity by name). S1.7.23 — `types` / `instances`
        # are GONE: the kernel keeps no type-system entity-view. The
        # inheritance forest is just `is-a` facts; a puzzle that wants a
        # named-type projection computes it with an ein-lang rule.
        self.relations: dict[str, Relation] = {}
        self.rules: dict[str, Rule] = {}
        # Hypothesis rules (S1.5.6b) — kept separate from `rules`:
        # they generate candidate hypotheses for hypgen and are
        # never fired by the saturator. Loaded from `(hrule …)`.
        self.hrules: dict[str, Rule] = {}
        # Pattern macros (P1.8 S1.5.9) — `(macro …)` AST-rewrite aliases.
        # Consumed at LOAD time only: the loader expands each rule clause's
        # macro invocations before compiling, so nothing reads `macros`
        # after `load()`. Kept on the KB as an inspectable record (and
        # shared by reference across forks like the other registries).
        self.macros: dict[str, Macro] = {}
        # Facts list (origin is on each Fact's `provenance`).
        self.facts: list[Fact] = []
        # Query (optional, only when the IR carries a `(query …)`).
        self.query: Query | None = None
        # SolverConfig (optional, only when the IR carries a `(config
        # …)` head; T1.5.4.4). The forward ref lives under TYPE_CHECKING
        # so the inference → kb import stays a checker-only edge (no
        # runtime cycle; instance-attr annotations are not evaluated).
        # `solve()` resolves precedence as: kwarg > kb.config
        # > SolverConfig().
        self.config: SolverConfig | None = None

        # S1.5a.18 — Learned no-good clauses (path-condition, CDCL-style).
        # Each clause is a frozenset of `(relation_name, args)`
        # FactIds; meaning: "any branch whose path condition is a
        # superset of this clause is dead". Populated by
        # `nogoods.emit_nogood` on the root kb only; forks share
        # by reference for read access during pre-fork filtering.
        # Subsumption is enforced on emit — `_nogoods` is kept
        # minimal (no clause is a strict superset of another).
        self._nogoods: set[frozenset[tuple[str, tuple]]] = set()

        # Equality-class hooks — reserved for F4.
        self.classes: EqClasses = EqClasses()

        # Reverse indexes — populated by `_rebuild_indexes`. S1.7.23 —
        # `_types_by_parent` / `_instances_by_type` / `_facts_by_instance`
        # / `_rules_by_type` are GONE: they served only the deleted
        # `Type` / `Instance` entity accessors.
        self._facts_by_relation: dict[str, tuple[Fact, ...]] = {}
        self._rules_by_relation: dict[str, tuple[Rule, ...]] = {}
        self._rule_apps_by_rule: dict[str, tuple[Fact, ...]] = {}
        self._rule_apps_on_relation: dict[str, tuple[Fact, ...]] = {}

        # Participation index (S1.8.B-idx, 2026-06-14) — facts grouped by
        # `(relation, arg-position, atomic value)` so a Scan/Join step with
        # a bound slot fetches only candidate facts instead of scanning the
        # whole relation extent (`match.py:_candidates`). Keys are `str` /
        # `int` arg values only — the join-key types; nested-`Fact` args are
        # not keyed (they carry a `NestedPattern` slot and full-scan).
        # Maintained on the same rebind-to-fresh-tuple contract as
        # `_facts_by_relation`, so fork/snapshot shallow copies stay
        # byte-identical (F-KB-9).
        self._facts_by_rel_slot_val: dict[
            tuple[str, int, str | int], tuple[Fact, ...]
        ] = {}

        # Global names index — encoding-agnostic node participation
        # set per docs/kernel/ir/01-ein-graph/03_ein_model.md §2.
        # Keyed by name; the NameRef's `category` discriminates
        # `object` / `relation` / `rule`.
        self.names: dict[str, NameRef] = {}

        # Negated-fact index — for each `(not <inner>)` fact, the
        # `(inner.relation_name, inner.args)` tuple. Lets the
        # hypothesis-generator's Tier-A exclusion check land in O(1)
        # instead of an O(|not-facts|) scan over `_facts_by_relation`.
        self._negated_facts: set[tuple[str, tuple]] = set()

        # S1.21.7 — ALTERNATIVE justifications (multi-justification
        # provenance). `Fact.provenance` holds the *primary* (first-recorded)
        # justification; this side table holds every OTHER derivation the
        # saturator attempted for the same `(relation_name, args)` identity,
        # in first-seen order. Together they form the fact's OR-node: the
        # proof structure is an AND/OR graph, one AND-node per
        # `Provenance.premises_raw` tuple.
        #
        # Why a side table and not a field: `Fact` is `frozen=True` and its
        # identity is `(relation_name, args)` — every index shares the one
        # canonical object, so growing the object per re-derivation would
        # break identity-sharing. Keyed by FactId, so it survives the
        # canonical object being shared across forks.
        #
        # Copy contract: this follows the *facts* contract — shallow-copied
        # per fork/snapshot in `_copy_fact_indexes_into`, never shared by
        # reference like `_nogoods`. A justification recorded inside a
        # hypothesis fork may name `hypothesis`-kind premises that root never
        # assumed; sharing would leak a phantom assumption into a root-level
        # unsat core. Values are immutable tuples rebound on append (the same
        # discipline `_index_fact` uses), so the shallow copy is safe.
        #
        # NOT rebuildable: unlike the six reverse indexes this is *history*,
        # not a projection of `self.facts` — `rebuild_indexes` therefore
        # leaves it alone rather than clearing it.
        self._alt_justifications: dict[FactId, tuple[Provenance, ...]] = {}

    # ── from_ir convenience ───────────────────────────────────────

    @classmethod
    def from_ir(cls, forms, *, base_dir=None) -> KnowledgeBase:
        """Construct a KB from parsed IR forms.

        Delegates to :func:`ein.kb.from_ir.load`. Importing here
        avoids a hard import-time cycle (`from_ir` imports `store`).
        ``base_dir`` is the directory file-relative `(import …)` forms
        resolve against (P1.8 S1.8.A3); ``None`` resolves only ``std.*``.
        """
        from .from_ir import load
        return load(forms, base_dir=base_dir)

    @classmethod
    def from_file(cls, path) -> KnowledgeBase:
        """Construct a KB from a ``.ein`` file, resolving its `(import …)`
        forms file-relative to the file's directory (P1.8 S1.8.A3)."""
        from pathlib import Path

        from ein.ir import parse
        p = Path(path)
        forms = parse(p.read_text(encoding="utf-8"), filename=str(p))
        return cls.from_ir(forms, base_dir=p.parent)

    # ── Registry mutation (used by the loader) ────────────────────

    # No `types` / `instances` registries: membership is stated by an
    # ordinary relation a puzzle declares, and the
    # kernel builds no type/instance entity-view over them. The loader
    # just ingests the facts; any named-type projection is a user rule.

    def add_relation(self, rel: Relation) -> Relation:
        """Register a Relation; the *declared* flag wins over open-world.

        If an open-world entry exists, a subsequent ``add_relation``
        call with ``declared=True`` upgrades it in place (replaces the
        registry entry; the previous detached instance becomes
        orphaned but no one was holding it).
        """
        existing = self.relations.get(rel.name)
        if existing is None:
            _attach(rel, self)
            self.relations[rel.name] = rel
            return rel
        if rel.declared and not existing.declared:
            _attach(rel, self)
            self.relations[rel.name] = rel
            return rel
        return existing

    def add_rule(self, rule: Rule) -> Rule:
        if rule.name in self.rules:
            return self.rules[rule.name]
        _attach(rule, self)
        self.rules[rule.name] = rule
        return rule

    def add_hrule(self, rule: Rule) -> Rule:
        """Register a hypothesis rule (S1.5.6b) into ``hrules``.

        A hrule is a :class:`Rule` by shape; it lives in its own
        registry so the engine / saturator — which walk ``rules`` —
        never fire it. ``hypgen`` is its only consumer. Rule and
        hrule names share one name-space — the loader rejects a
        duplicate, so ``hrules`` is keyed by name like ``rules``.
        """
        if rule.name in self.hrules:
            return self.hrules[rule.name]
        _attach(rule, self)
        self.hrules[rule.name] = rule
        return rule

    def add_fact(self, fact: Fact) -> Fact:
        """Append a Fact; dedupe is by ``(relation_name, args)``.

        Provenance is excluded from identity — a single proposition
        lives only once in the KB, and `provenance` records where it
        came from. If a fact arrives twice with different provenance the
        *first* occurrence wins; this preserves the most primitive
        declaration. Duplicate-only-by-source/by-rule are collapsed;
        provenance details (S1.2.3) attach to the canonical Fact.
        """
        _attach(fact, self)
        for existing in self.facts:
            if (existing.relation_name == fact.relation_name
                    and existing.args == fact.args):
                return existing
        self.facts.append(fact)
        return fact

    def add_and_index_fact(self, fact: Fact) -> Fact:
        """Saturation-time add: dedup against the live indexes and, on a
        genuinely-new fact, append it *and* update the incremental indexes.

        The saturation hot path. Unlike :meth:`add_fact` — the loader's
        index-free append that dedups by scanning :attr:`facts` before
        :meth:`rebuild_indexes` runs — this dedups with the O(deg)
        :meth:`_fact_by_id` lookup (saturation keeps
        ``_facts_by_relation`` current) and returns a pre-existing fact
        **without** re-indexing it. So a fact re-derived by a second rule
        lands in the indexes exactly once, rather than the ``add_fact`` +
        unconditional ``_index_fact`` pattern's silent double-index.
        """
        existing = self._fact_by_id(fact.relation_name, fact.args)
        if existing is not None:
            # S1.21.7 — the dedup drop. The incoming fact's justification is
            # a *second* derivation of the same proposition: keep it as an
            # alternative rather than discarding it (this is the "partially
            # novel multi-assert" seam; the bulk of re-derivations never get
            # here because `Saturator._apply` short-circuits a wholly-
            # redundant firing before `fire()` builds a Provenance — it calls
            # :meth:`record_justification` directly).
            if fact.provenance is not None:
                self.record_justification(existing, fact.provenance)
            return existing
        _attach(fact, self)
        self.facts.append(fact)
        self._index_fact(fact)
        return fact

    # ── Multi-justification provenance — S1.21.7 ──────────────────

    def record_justification(self, fact: Fact, prov: Provenance) -> bool:
        """Record an ALTERNATIVE justification for an already-known fact.

        ``fact`` is the **canonical** (already-stored) object — both call
        sites hold it, so this takes no `_fact_by_id` lookup on the
        saturation hot path.

        Returns True iff ``prov`` was newly recorded. The fact's *primary*
        justification stays on ``Fact.provenance`` (first derivation wins,
        unchanged); this appends to :attr:`_alt_justifications`, turning the
        fact into an OR-node of the proof graph.

        Only **rule-kind provenance with at least one premise** is recorded:

        - ``source`` / ``hypothesis`` kinds are *assumptions*, not
          derivations — an assumption has nothing to weigh alternatives
          against, and re-declaring one must not make it look derived.
        - a rule-kind record with an EMPTY ``premises_raw`` is a synthetic
          engine writeback (``<forced-positive>``,
          ``<monotonic-unconditional>``, ``<lookahead-dies-immediately>`` —
          see ``docs/kernel/inference/reserved_engine_strings.md``) whose
          stated contract is that provenance walks *ground out* on it.
          Recording it as an alternative would give the fact the empty
          environment — "derivable from nothing" — collapsing every
          explanation that passes through it.

        Symmetrically, a fact whose *primary* justification is already a
        terminal takes no alternatives at all:

        - ``source`` / ``hypothesis`` primaries are the derivation
          **frontier** — what the engine treats as given. A clue that also
          happens to be re-derivable is still a clue, and explanations that
          expanded it would stop naming the premise the user actually
          supplied.
        - a rule-kind writeback primary keeps its ground-out contract no
          matter what the following ``saturate()`` re-derives.

        Identity of a justification here is its AND-node ``(rule,
        premises_raw)``; ``bindings`` is display metadata and is not part of
        it, so two firings that consumed the same premises collapse.

        The per-fact list is capped at :data:`MAX_ALT_JUSTIFICATIONS` — an
        unbounded list is the feature's one real memory cost and buys little
        (the environments repeat). It is kept **sorted by premise count**,
        so at the cap an arriving justification with fewer premises evicts
        the longest recorded one: the cap biases towards the small
        explanations the minimality search is looking for, rather than
        towards whichever happened to fire first. Sorted order also makes
        the saturation hot path O(1) — a full list rejects an
        arrival no shorter than its last entry without scanning.
        """
        if prov.kind != "rule" or not prov.premises_raw:
            return False
        pp = fact.provenance
        if pp is not None:
            if pp.kind != "rule":
                return False                      # given: a frontier terminal
            if not pp.premises_raw:
                return False                      # ground-out contract
            if pp.rule == prov.rule and pp.premises_raw == prov.premises_raw:
                return False                      # already the primary
        fact_id: FactId = (fact.relation_name, fact.args)
        current = self._alt_justifications.get(fact_id, ())
        n = len(prov.premises_raw)
        full = len(current) >= MAX_ALT_JUSTIFICATIONS
        if full and n >= len(current[-1].premises_raw):
            return False                          # the O(1) hot path
        at = len(current)
        for i, p in enumerate(current):
            if p.rule == prov.rule and p.premises_raw == prov.premises_raw:
                return False                      # already an alternative
            if at == len(current) and len(p.premises_raw) > n:
                at = i
        # Rebind to a fresh tuple (never mutate in place) so the
        # fork/snapshot shallow copy cannot alias into a live branch.
        kept = current[:-1] if full else current
        self._alt_justifications[fact_id] = (*kept[:at], prov, *kept[at:])
        if events.ON:
            events.emit(
                "alt", fact=events.fact_id(fact_id), rule=prov.rule,
                premises=[events.fact_id(p) for p in prov.premises_raw],
            )
        return True

    def accepts_justification(self, fact: Fact, n_premises: int) -> bool:
        """Cheap O(1) pre-check: could ``fact`` still take an alternative
        justification with ``n_premises`` premises?

        Lets the saturator's redundant-firing path skip *building* a
        :class:`Provenance` (and stringifying its bindings) for the common
        case of a hot fact whose alternative list is already full of shorter
        derivations. A True answer is not a promise —
        :meth:`record_justification` still applies the duplicate and
        ground-out rules.
        """
        if n_premises <= 0:
            return False
        pp = fact.provenance
        if pp is not None and (pp.kind != "rule" or not pp.premises_raw):
            return False
        current = self._alt_justifications.get(
            (fact.relation_name, fact.args), ())
        return (len(current) < MAX_ALT_JUSTIFICATIONS
                or n_premises < len(current[-1].premises_raw))

    def justifications(self, fact: Fact) -> tuple[Provenance, ...]:
        """Every recorded justification of ``fact`` — primary first.

        The fact's OR-node: one entry per derivation the engine recorded,
        each an AND-node over its ``premises_raw``. A fact with no recorded
        alternatives yields just its primary (or an empty tuple when it has
        no provenance at all), so single-justification callers can migrate
        to this accessor without behaviour change.
        """
        out: list[Provenance] = []
        if fact.provenance is not None:
            out.append(fact.provenance)
        out.extend(self._alt_justifications.get(
            (fact.relation_name, fact.args), ()))
        return tuple(out)

    def has_alternative_justifications(self) -> bool:
        """True iff any fact carries a recorded alternative derivation."""
        return bool(self._alt_justifications)

    # ── Index rebuild ─────────────────────────────────────────────

    def rebuild_indexes(self) -> None:
        """Recompute all reverse indexes from current registries + facts.

        Cheap enough on Zebra-scale (single-digit milliseconds);
        called once after batch ingest. Incremental maintenance for
        single-fact additions during saturation is provided by
        :meth:`_index_fact` so a saturation loop need not rebuild.

        No type / instance derivation: every fact is indexed the same
        way, with no entity-view built over any particular head.

        S1.21.7 — ``_alt_justifications`` is deliberately NOT touched here.
        Every other index is a projection of ``self.facts`` and so is safe
        to drop and recompute; the alternative-justification table is a
        record of derivations the engine *attempted*, which no amount of
        looking at the current fact set can reconstruct. Clearing it would
        silently discard the OR-structure of the proof graph.
        """
        # Facts indexes — a single pass over `self.facts` feeds every
        # fact-derived grouping (S1.7c.20 — was two walks; the second
        # rebuilt the head→facts grouping `fbr` already holds, and
        # collected `arg_lists` alongside).
        self._facts_by_relation = {}
        self._rule_apps_by_rule = {}
        self._rule_apps_on_relation = {}
        self._negated_facts = set()
        self._facts_by_rel_slot_val = {}
        fbr: dict[str, list[Fact]] = defaultdict(list)
        rabr: dict[str, list[Fact]] = defaultdict(list)
        raor: dict[str, list[Fact]] = defaultdict(list)
        arg_lists: dict[str, list[Fact]] = defaultdict(list)
        fbrsv: dict[tuple[str, int, str | int], list[Fact]] = defaultdict(list)
        for fact in self.facts:
            fbr[fact.relation_name].append(fact)
            is_rule_app = fact.relation_name in self.rules
            if is_rule_app:
                rabr[fact.relation_name].append(fact)
            for i, a in enumerate(fact.args):
                # Participation index — same (rel, position, atomic value)
                # keying as the incremental `_index_fact` path.
                if type(a) is str or type(a) is int:
                    fbrsv[(fact.relation_name, i, a)].append(fact)
                if isinstance(a, str):
                    arg_lists[a].append(fact)
                    # Each arg that is a known Relation is a target of
                    # the rule application.
                    if is_rule_app and a in self.relations:
                        raor[a].append(fact)
            if fact.relation_name == "not" and fact.args:
                inner = fact.args[0]
                if isinstance(inner, Fact):
                    self._negated_facts.add(
                        (inner.relation_name, inner.args)
                    )
        self._facts_by_relation = {k: tuple(v) for k, v in fbr.items()}
        self._rule_apps_by_rule = {k: tuple(v) for k, v in rabr.items()}
        self._rule_apps_on_relation = {k: tuple(v) for k, v in raor.items()}
        self._facts_by_rel_slot_val = {k: tuple(v) for k, v in fbrsv.items()}

        # Global names index — every distinct name that appears
        # anywhere in the KB. The head→facts grouping is exactly `fbr`
        # (built above); `arg_lists` is the per-arg grouping. Registry
        # keys are unioned in so a relation declared with no facts yet
        # still shows up.
        all_names: set[str] = (
            set(fbr)
            | set(arg_lists)
            | set(self.relations)
            | set(self.rules)
        )
        self.names = {
            n: NameRef(
                name=n,
                category=self._categorise_name(n),
                as_head=tuple(fbr.get(n, ())),
                as_arg=tuple(arg_lists.get(n, ())),
            )
            for n in all_names
        }

        # rules_by_relation (named in pattern + via property fact)
        rbr: dict[str, set[str]] = defaultdict(set)
        for rule in self.rules.values():
            for p in (rule.match, rule.assert_):
                if p is None:
                    continue
                for rn in p.relation_names:
                    if rn in self.relations:
                        rbr[rn].add(rule.name)
        # Add the property-fact side: a property fact `(symmetric R)`
        # is a rule application of `symmetric` on `R`, so the rule's
        # name (the fact's head) is implicitly a rule on `R`. The
        # `symmetric` *Rule* doesn't name `R` in its body — `?rel`
        # binds to `R` via the property fact.
        for rel_name, app_facts in self._rule_apps_on_relation.items():
            for f in app_facts:
                rbr[rel_name].add(f.relation_name)
        self._rules_by_relation = {
            rel: tuple(self.rules[n] for n in sorted(names) if n in self.rules)
            for rel, names in rbr.items()
        }

    def _categorise_name(self, name: str) -> str:
        """Map a name to its `NameRef.category`.

        `relation` / `rule` are the only two kernel forms hardcoded as
        `category="relation"` (`KERNEL_META_RELATIONS`). Since S1.7.6
        `type` / `instance` are NOT special: they categorise as
        `"relation"` only when the puzzle declares them (e.g.
        `(relation instance Thing Type)` in zebra.ein), otherwise they
        fall through to `"object"` like any other name. See
        [[project-canonical-zebra2]].
        """
        if name in KERNEL_META_RELATIONS:
            return "relation"
        if name in self.relations:
            return "relation"
        if name in self.rules:
            return "rule"
        return "object"

    def _index_fact(self, fact: Fact) -> None:
        """Incrementally append a single fact to the reverse indexes.

        Used by the engine (P1.3 / P1.5) so a saturation step
        needn't call :meth:`rebuild_indexes`. Writes are **in place**:
        :meth:`fork` and :meth:`snapshot` give every kb its own index
        dicts (no kb aliases another's — every assignment is a fresh
        ``{}`` / ``dict(...)``), so an in-place append never leaks across
        branches — and avoids the per-fact whole-dict rebuild the old
        ``{**d, k: …}`` form paid (O(|relations|) and O(|names|) per add).
        """
        rn = fact.relation_name
        self._facts_by_relation[rn] = (
            *self._facts_by_relation.get(rn, ()), fact,
        )
        # Participation index — key each atomic (str/int) arg by position.
        for i, val in enumerate(fact.args):
            if type(val) is str or type(val) is int:
                psi_key = (rn, i, val)
                self._facts_by_rel_slot_val[psi_key] = (
                    *self._facts_by_rel_slot_val.get(psi_key, ()), fact,
                )
        if rn in self.rules:
            self._rule_apps_by_rule[rn] = (
                *self._rule_apps_by_rule.get(rn, ()), fact,
            )
            for a in fact.args:
                if isinstance(a, str) and a in self.relations:
                    self._rule_apps_on_relation[a] = (
                        *self._rule_apps_on_relation.get(a, ()), fact,
                    )

        # Negated-fact index — O(1) Tier-A exclusion lookup for the
        # hypothesis generator.
        if rn == "not" and fact.args:
            inner = fact.args[0]
            if isinstance(inner, Fact):
                self._negated_facts.add(
                    (inner.relation_name, inner.args)
                )

        # Names index — append to head + arg sets via fresh frozen
        # NameRefs (the value identity shifts on update).
        def _bump_head(name: str) -> None:
            prev = self.names.get(name)
            if prev is None:
                self.names[name] = NameRef(
                    name=name, category=self._categorise_name(name),
                    as_head=(fact,), as_arg=(),
                )
            else:
                self.names[name] = NameRef(
                    name=prev.name, category=prev.category,
                    as_head=(*prev.as_head, fact),
                    as_arg=prev.as_arg,
                )

        def _bump_arg(name: str) -> None:
            prev = self.names.get(name)
            if prev is None:
                self.names[name] = NameRef(
                    name=name, category=self._categorise_name(name),
                    as_head=(), as_arg=(fact,),
                )
            else:
                self.names[name] = NameRef(
                    name=prev.name, category=prev.category,
                    as_head=prev.as_head,
                    as_arg=(*prev.as_arg, fact),
                )

        _bump_head(rn)
        for a in fact.args:
            if isinstance(a, str):
                _bump_arg(a)

    # ── Convenience accessors ─────────────────────────────────────

    # ── Fact view — S1.2.2 T1.2.2.2 ───────────────────────────────

    def all_facts(self) -> FactView:
        """Read-only filtered window over every fact.

        S1.22.1b: the three layer-scoped siblings (`ontology()` /
        `fact_layer()` / `reasoning()`) went with the `Layer` enum. They
        had no caller in the engine, and :class:`FactView`'s
        ``by_source`` / ``by_rule`` filters already express what they
        selected — over the provenance itself rather than a copy of it.
        """
        from .views import FactView
        return FactView(tuple(self.facts), self, "all")

    # ── Provenance + derivation DAG — S1.2.3 ──────────────────────

    def _fact_by_id(
        self, relation_name: str, args: tuple,
    ) -> Fact | None:
        """Look up a fact by its identity tuple — O(deg(relation))."""
        for f in self._facts_by_relation.get(relation_name, ()):
            if f.args == args:
                return f
        return None

    def derivation_dag(
        self, fact: Fact, *, all_justifications: bool = False,
    ) -> DerivationDAG:
        """Build the derivation DAG rooted at `fact`.

        BFS over ``fact.provenance.premises_raw``, resolving each id
        to a Fact via :meth:`_fact_by_id` and recursing into
        ``rule``-kind facts only. ``source``- and ``hypothesis``-kind
        facts terminate the recursion.

        Cycles are broken at re-visit (the revisited fact appears as a
        node but is not re-expanded).

        ``all_justifications=True`` (S1.21.7) expands every recorded
        derivation instead of the primary one, giving an AND/OR graph whose
        conjunction structure is in :attr:`DerivationDAG.and_nodes`. The
        default stays primary-only: the DAG is a *display* object, and one
        derivation per fact is what a reader can follow.
        """
        from .provenance import build_derivation_dag
        return build_derivation_dag(
            fact, self._fact_by_id,
            justifications=self.justifications if all_justifications else None,
        )

    # ── Unified DOT rendering — S1.2.4 ────────────────────────────

    def to_dot(self, **kwargs) -> str:
        """Render the KB as a unified Graphviz ``digraph`` string.

        Delegates to :func:`ein.kb.render.to_dot`. Keyword args
        forwarded — see that function for ``colour_by`` /
        ``include_types`` / ``include_instances`` / ``name``.
        """
        from .render import to_dot
        return to_dot(self, **kwargs)

    def unsat_core(
        self, conflicting: Iterable[Fact], *, all_justifications: bool = False,
    ) -> set[Fact]:
        """Frontier of given/assumed facts across a set of conflicting facts.

        For each conflicting fact, walk its derivation closure and
        accumulate the *frontier* terminals — source-kind, hypothesis-kind,
        or un-provenanced givens. The union is the source-frontier set of
        facts that jointly derive the conflict *per the recorded
        derivations* (the given clues, plus the committed hypotheses when a
        branch is involved); it's what the *contradictions* task class
        returns to the user (idea 03).

        Built on the shared frontier collector
        :func:`~ein.kb.provenance.walk_premises` (E6) — the same
        premise-closure walk as :meth:`derivation_dag`, without
        materialising the edge set. One shared ``visited`` set memoises
        the walk across all the conflicting facts.

        S1.21.7 — this stays **primary-justification only** by default, and
        that is a deliberate choice, not leftover behaviour. Unioning over
        alternative derivations makes the core monotonically *larger*, which
        is the opposite of what a legible explanation needs; and the union
        already over-states the conflict (one cause fanning out into many
        witnesses). ``all_justifications=True`` gives that larger union,
        whose use is as a soundness envelope: every explanation of these
        conflicts is a subset of it. For a minimum-cardinality answer use
        :func:`ein.inference.frontier.smallest_contradiction_frontier`,
        which *chooses* one justification per fact rather than unioning them.
        """
        from .provenance import walk_premises

        def _is_frontier(_key: object, fact: Fact) -> bool:
            return (fact.provenance is None
                    or fact.provenance.kind in ("source", "hypothesis"))

        core: set[Fact] = set()
        visited: set = set()
        justifications = self.justifications if all_justifications else None
        for f in conflicting:
            core |= walk_premises(
                f, self._fact_by_id, keep=_is_frontier, visited=visited,
                justifications=justifications)
        return core

    # ── Fork — S1.2.2 T1.2.2.3 ────────────────────────────────────

    def fork(self) -> KnowledgeBase:
        """Create a branch for hypothesis exploration.

        Shares the immutable populations (`relations`, `rules`,
        `hrules`, `query`) **by reference**. The :attr:`facts` list and
        the reverse indexes are shallow-copied so the fork can append
        derived facts and rebuild its own incremental indexes without
        leaking into the parent.

        Caveat about entity back-pointers: shared entities keep their
        ``_kb`` pointing at the **original** KB. This means a shared
        ``Relation``'s ``.facts`` returns the original KB's facts, NOT
        the fork's view. For fork-scoped queries use
        ``fork.all_facts().about(name)`` or the explicit indexes on the
        fork (``fork._facts_by_relation[name]``). This
        is intentional: hypothesis branches rarely introduce new
        entities, only new derived facts; the entity API tells you
        the root state, the fork view tells you the branch state.

        Cost: O(|facts|) for the shallow copies — bounded by Zebra-
        scale at ~50-200 facts. The plan's "O(|reasoning|)" target is
        relaxed to "O(|facts|) with O(1) per fact" because the indexes
        partition by relation/instance, and shallow-copying a dict
        is constant per entry; if hypothesis branching ever becomes a
        hot path (P1.5 profiling), revisit with a copy-on-write
        wrapper.
        """
        new = KnowledgeBase()
        # Share immutable populations by reference.
        new.relations = self.relations
        new.rules = self.rules
        new.hrules = self.hrules
        new.macros = self.macros
        new.query = self.query
        # Equality classes: fork its own state (the parent's classes
        # remain reachable via the union-find roots that were created
        # before the fork).
        new.classes = EqClasses()
        new.classes._parent = dict(self.classes._parent)
        # Facts list: copy so appends to the fork don't touch parent.
        new.facts = list(self.facts)
        # Reverse indexes — the single fork/snapshot copy contract
        # (`_rules_by_relation` shared by reference, the six fact-derived
        # indexes shallow-copied; see `_copy_fact_indexes_into`).
        self._copy_fact_indexes_into(new)
        # T1.5.4.4 carry-over: `config` is an immutable reference; the
        # fork inherits it as-is.
        new.config = self.config
        # S1.5a.18 — no-good clause set: shared by reference for
        # cross-fork read access during pre-fork filtering.
        new._nogoods = self._nogoods
        return new

    # ── Snapshot — S1.5b.22 T1.5b.22.2 ────────────────────────────

    def snapshot(self) -> KnowledgeBase:
        """Deep-ish archival copy. Used by :class:`SolutionRecord` so
        a satisfying-branch kb survives later mutations of root.

        Differs from :meth:`fork` in one place:

        - ``_nogoods`` is COPIED (fork shares by reference because
          live branches read concurrently; the snapshot is archival
          so we want isolation).

        The reverse indexes are shallow-copied exactly as in
        :meth:`fork` (S1.7c.21): the source kb's indexes are already
        consistent with its ``facts`` (every mutation goes through
        :meth:`_index_fact`, which rebinds keys to fresh immutable
        tuples), so the copy is byte-identical to a full
        :meth:`rebuild_indexes` without paying for one per snapshot.

        Shares by reference (constant across the search, no
        mutation concern): ``relations``, ``rules``, ``hrules``,
        ``classes``, ``query``, ``config``. (Since S1.7.23 there
        are no ``types`` / ``instances`` registries to share or
        re-derive.)

        Soundness invariant: a snapshotted kb's
        :meth:`derivation_dag` walks the same chain as the source
        did at snapshot time. Provenance carries the rule
        reference; we only need name equality, so sharing the rule
        registry by reference is fine.
        """
        new = KnowledgeBase()
        # Share constant registries by reference.
        new.relations = self.relations
        new.rules     = self.rules
        new.hrules    = self.hrules
        new.macros    = self.macros
        new.query     = self.query
        new.config    = self.config
        # Equality classes: copy state (the parent's union-find
        # remains reachable via the roots that were created before
        # the snapshot).
        new.classes = EqClasses()
        new.classes._parent = dict(self.classes._parent)
        # Deep-copy mutable state.
        new.facts = list(self.facts)
        new._nogoods = set(self._nogoods)
        # S1.7c.21 — shallow-copy the in-place-maintained indexes (the same
        # contract `fork` uses) rather than a full `rebuild_indexes()` per
        # recorded solution. See `_copy_fact_indexes_into` for why the
        # shallow copy is byte-identical to a rebuild on this warm path.
        self._copy_fact_indexes_into(new)
        return new

    def _copy_fact_indexes_into(self, new: KnowledgeBase) -> None:
        """The single fork/snapshot reverse-index copy contract (S1.7c.22).

        ``_rules_by_relation`` derives from the post-load-immutable rules
        registry and is never mutated by :meth:`_index_fact`, so it is
        SHARED by reference (like the other registries). The six
        fact-derived indexes ARE appended to during saturation, so each copy
        gets its own shallow copy to mutate in place. That shallow copy is
        byte-identical to a full :meth:`rebuild_indexes` because
        :meth:`_index_fact` rebinds each key to a fresh immutable tuple (and
        re-adds to the set), never mutating a shared container in place
        (S1.7c.21) — so :meth:`fork` and :meth:`snapshot` cannot desync,
        which is the F-KB-9 win. (A full typed-index wrapper was assessed
        and rejected: post-S1.7.23 there is only one static index against
        six mutable, so the wrapper's mutable/static separation collapses to
        ceremony; this helper captures the whole remaining value.)

        S1.21.7 adds a seventh entry, ``_alt_justifications``, on the same
        rebind-to-fresh-tuple discipline — but it is **history, not a
        projection**: :meth:`rebuild_indexes` cannot reconstruct it and
        deliberately leaves it alone, so the "byte-identical to a rebuild"
        argument above does not cover it. What makes the shallow copy correct
        there is the same no-in-place-mutation rule
        (:meth:`record_justification` rebinds the whole tuple), plus the
        reason it must be COPIED rather than shared like ``_nogoods``: a
        justification recorded inside a hypothesis fork can name
        ``hypothesis``-kind premises root never assumed, and sharing would
        surface that phantom assumption in a root-level unsat core.
        """
        new._rules_by_relation = self._rules_by_relation
        new._facts_by_relation = dict(self._facts_by_relation)
        new._rule_apps_by_rule = dict(self._rule_apps_by_rule)
        new._rule_apps_on_relation = dict(self._rule_apps_on_relation)
        new.names = dict(self.names)
        new._negated_facts = set(self._negated_facts)
        new._facts_by_rel_slot_val = dict(self._facts_by_rel_slot_val)
        new._alt_justifications = dict(self._alt_justifications)

    # ── Dunder ────────────────────────────────────────────────────

    def __len__(self) -> int:
        """Total node count: relations + rules + facts."""
        return len(self.relations) + len(self.rules) + len(self.facts)

    def __repr__(self) -> str:
        return (
            f"<KnowledgeBase relations={len(self.relations)} "
            f"rules={len(self.rules)} facts={len(self.facts)}>"
        )


__all__ = ["EqClasses", "KnowledgeBase", "Query"]
