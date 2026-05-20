"""IR → KnowledgeBase loader — S1.2.1 T1.2.1.4.

Walks parsed top-level forms (`ontology`, `facts`, `reasoning`,
`rules`, `query`, `trace`) and populates a fresh :class:`KnowledgeBase`.

The load order matters because some entities reference others:
1. Pass 1 — collect raw declarations into local lists (no entities
   created yet).
2. Pass 2 — instantiate entities in dependency order: types,
   relations, instances, rules, facts.
3. Pass 3 — re-classify facts: any fact whose head matches a rule
   name OR a declared relation name is fine; otherwise create an
   open-world :class:`Relation` so cross-references still work.
4. Index rebuild.

Tolerance:
- Undeclared relations become open-world ``declared=False`` Relation
  entities. Examples: in zebra.ein the property tags ``symmetric``,
  ``transitive``, ``square-fwd`` etc. appear as fact heads
  ``(symmetric co-located)`` without explicit ``(relation symmetric …)``
  declarations because they're the names of *rules*; the loader
  creates a relation auto-entity so the fact has a valid
  ``Fact.relation`` link, and the engine can still cross-reference.
- Undeclared types referenced from an `(instance _ T)` are likewise
  auto-created.

Errors:
- Duplicate top-level block of the same kind (two `(ontology …)`) is
  fine; they merge.
- A malformed rule body (missing :match or :assert) is logged via
  :class:`KBLoadError` (raised at end of pass 2 so we don't fail on
  the first malformed rule).
"""
from __future__ import annotations

from collections.abc import Iterable

from ein_bot.ir.types import Atom, Int, IRNode, KwPair, Range, SForm, String, Var

from .entities import Fact, Instance, Layer, Relation, Rule, Type
from .pattern import Pattern
from .provenance import Provenance
from .store import KnowledgeBase, Query


class KBLoadError(ValueError):
    """Raised at end of load with a list of accumulated problems."""


# ── Utility extractors ─────────────────────────────────────────────


def _is_form(node: IRNode, head: str) -> bool:
    return isinstance(node, SForm) and isinstance(node.head, Atom) and node.head.name == head


def _atom_name(node: IRNode) -> str | None:
    return node.name if isinstance(node, Atom) else None


def _kw_pairs(args: tuple[IRNode, ...]) -> dict[str, IRNode]:
    """Extract `:key value` pairs from an SForm's args tuple."""
    out: dict[str, IRNode] = {}
    for a in args:
        if isinstance(a, KwPair):
            out[a.key.name] = a.value
    return out


def _atomic_value(node: IRNode) -> str | int | None:
    """Stringify an Atom/String/Int/Var for use as a Fact arg."""
    if isinstance(node, Atom):
        return node.name
    if isinstance(node, String):
        return node.value
    if isinstance(node, Int):
        return node.value
    if isinstance(node, Var):
        return f"?{node.name}"
    if isinstance(node, Range):
        if node.high is None:
            return f"{node.low}..*"
        return f"{node.low}..{node.high}"
    return None


def _fact_args(args: tuple[IRNode, ...]) -> tuple[str | int | Fact, ...]:
    """Drop kw-pairs; build args admitting nested-fact (relational-node) values.

    Kernel-aligned with ``docs/kernel/ir/01-ein-graph/03_ein_model.md``
    §3 — args can be named nodes (``str`` / ``int``) or **relational
    nodes** (nested ``Fact`` instances). A nested SForm in arg
    position becomes a ``Fact`` with the corresponding head /
    args, recursively. The nested Fact is unregistered (``_kb=None``,
    ``layer=Layer.FACT``); identity by ``(relation_name, args)`` is
    sufficient for equality with any registered fact of the same
    shape.
    """
    out: list[str | int | Fact] = []
    for a in args:
        if isinstance(a, KwPair):
            continue
        v = _atomic_value(a)
        if v is not None:
            out.append(v)
            continue
        if isinstance(a, SForm):
            head = _atom_name(a.head)
            if head is None:
                # Head isn't a bare atom — fall back to its dumped form.
                head = "<nested>"
            nested = Fact(
                relation_name=head,
                args=_fact_args(a.args),
                layer=Layer.FACT,
                raw=a,
                loc=a.loc,
            )
            out.append(nested)
    return tuple(out)


# ── Pass 1 / 2 — collect + instantiate ─────────────────────────────


def _ingest_ontology(form: SForm, kb: KnowledgeBase, errors: list[str]) -> list[SForm]:
    """Pass over `(ontology …)` body.

    Returns the list of fact-shaped child forms to be processed in
    pass 2 (after rules are known) — type/relation declarations and
    instance forms are handled inline.
    """
    deferred_facts: list[SForm] = []

    for child in form.args:
        if not isinstance(child, SForm):
            continue
        head = _atom_name(child.head) if isinstance(child.head, Atom) else None

        if head == "type":
            # (type Name)  or  (type Name Parent)
            if not child.args:
                errors.append(f"(type) with no name at {child.loc}")
                continue
            name = _atom_name(child.args[0])
            parent = _atom_name(child.args[1]) if len(child.args) >= 2 else None
            if name is None:
                errors.append(f"(type) with non-atom name at {child.loc}")
                continue
            kb.add_type(Type(name=name, parent_name=parent, loc=child.loc))
            continue

        if head == "relation":
            # (relation Name T1 T2 … :kw v ...)
            # Flat args post-R10; grammar guarantees ≥ 2 SYMBOL args
            # (name + at least one type), followed by optional kw_pairs.
            if len(child.args) < 2:
                errors.append(f"(relation) needs name + signature at {child.loc}")
                continue
            name = _atom_name(child.args[0])
            sig = tuple(
                a.name for a in child.args[1:]
                if isinstance(a, Atom)
            )
            if name is None or not sig:
                errors.append(f"malformed (relation) at {child.loc}")
                continue
            kb.add_relation(Relation(
                name=name, signature=sig, declared=True, loc=child.loc,
            ))
            continue

        if head == "a-priori":
            # Same shape as (relation …) for now; future M3 may carry
            # additional metadata. Treat as a Relation with a marker.
            if len(child.args) < 2:
                errors.append(f"(a-priori) needs name + signature at {child.loc}")
                continue
            name = _atom_name(child.args[0])
            sig = tuple(
                a.name for a in child.args[1:]
                if isinstance(a, Atom)
            )
            if name is None or not sig:
                errors.append(f"malformed (a-priori) at {child.loc}")
                continue
            kb.add_relation(Relation(
                name=name, signature=sig, declared=True, loc=child.loc,
            ))
            continue

        if head == "instance":
            # (instance Name TypeName [:kw v ...])
            if len(child.args) < 2:
                errors.append(f"(instance) needs name + type at {child.loc}")
                continue
            iname = _atom_name(child.args[0])
            tname = _atom_name(child.args[1])
            if iname is None or tname is None:
                errors.append(f"(instance) args must be atoms at {child.loc}")
                continue
            # Auto-vivify the type so the link resolves even if the
            # `(type …)` declaration is absent.
            if tname not in kb.types:
                kb.add_type(Type(name=tname, parent_name=None, loc=child.loc))
            kb.add_instance(Instance(
                name=iname, type_name=tname, loc=child.loc,
            ))
            # The instance form is also a Fact (relation = "instance",
            # layer = ONTOLOGY) for the cross-ref machinery.
            deferred_facts.append(child)
            continue

        # Anything else is a fact in the ontology layer (rule-app facts,
        # structural facts, etc.).
        deferred_facts.append(child)

    return deferred_facts


def _ingest_rules(form: SForm, kb: KnowledgeBase, errors: list[str]) -> None:
    """Pass over `(rules …)` body."""
    for child in form.args:
        if not isinstance(child, SForm) or _atom_name(child.head) != "rule":
            errors.append(f"non-rule form in (rules …): {child}")
            continue
        if len(child.args) < 2:
            errors.append(f"(rule) needs name + params at {child.loc}")
            continue
        name = _atom_name(child.args[0])
        params_form = child.args[1]
        if name is None or not isinstance(params_form, SForm):
            errors.append(f"malformed (rule …) at {child.loc}")
            continue
        params = tuple(a.name for a in params_form.args if isinstance(a, Var))
        kws = _kw_pairs(child.args)
        match_node = kws.get("match")
        assert_node = kws.get("assert")
        why_node = kws.get("why")
        priority_node = kws.get("priority")
        if match_node is None or assert_node is None:
            errors.append(f"(rule {name}) missing :match or :assert at {child.loc}")
            continue
        why = why_node.value if isinstance(why_node, String) else ""
        priority = priority_node.value if isinstance(priority_node, Int) else None
        kb.add_rule(Rule(
            name=name,
            params=params,
            match=Pattern.from_ir(match_node),
            assert_=Pattern.from_ir(assert_node),
            why=why,
            priority=priority,
            loc=child.loc,
        ))


def _ingest_facts(
    forms: Iterable[SForm], kb: KnowledgeBase, layer: Layer, errors: list[str]
) -> None:
    """Build Fact entities from a sequence of fact-shaped SForms."""
    for child in forms:
        if not isinstance(child, SForm):
            continue
        head_name = _atom_name(child.head)
        if head_name is None:
            errors.append(f"fact with non-atom head at {child.loc}")
            continue
        # Map kernel meta-primitives to canonical relation names.
        # `not` / `and` / `or` / `neq` / `=` are kernel forms; they
        # appear at top level rarely (mostly inside rule bodies), but
        # if they DO they become facts whose relation is the literal
        # head atom name. The KB treats them as ordinary relations.
        kws = _kw_pairs(child.args)
        source = (
            kws["source"].value if isinstance(kws.get("source"), String) else None
        )
        rule_name = (
            _atom_name(kws.get("rule")) if "rule" in kws else None
        )

        # `:using` carries a list of (rel args) compact forms — each
        # one is an SForm whose head is a relation name and whose
        # args are the fact's args. Extract as (rel, args) fact-id
        # tuples for the Provenance.premises_raw field.
        #
        # **IR round-trip caveat** (S1.2.3 T1.2.3.4 deferred): the
        # current grammar (P1.1) doesn't accept a headless list as a
        # kw-pair value, so `:using ((rel a b) (rel c d))` doesn't
        # parse. The atom-id form `:using (c10 c15)` parses but to a
        # different shape (SForm with c10 as head, c15 as arg) and
        # would need an atom-id → Fact resolver. Both forms wait on
        # P1.1; until then, rule-kind provenance is populated by the
        # engine via direct `Provenance.from_rule()` construction —
        # which DOES work, just not round-trip through IR yet.
        using_node = kws.get("using")
        premises_raw: tuple = ()
        if isinstance(using_node, SForm):
            ids: list = []
            for inner in using_node.args:
                if not isinstance(inner, SForm) or not isinstance(inner.head, Atom):
                    continue
                ids.append((inner.head.name, _fact_args(inner.args)))
            premises_raw = tuple(ids)

        # Build the Provenance object — exactly one of source / rule
        # populates a kind; ONTOLOGY layer with no annotation gets a
        # source-kind record with source=None (the IR location alone
        # marks the origin).
        provenance: Provenance | None
        if rule_name is not None:
            provenance = Provenance.from_rule(
                rule=rule_name, premises_raw=premises_raw, loc=child.loc,
            )
        elif source is not None or layer in (Layer.FACT, Layer.ONTOLOGY):
            provenance = Provenance.from_source(source=source, loc=child.loc)
        else:
            provenance = None

        # Auto-vivify undeclared relations (open-world), UNLESS the
        # head is a built-in predicate (eq, neq — Q33). Predicates
        # dispatch at the matcher level; they are not relations.
        # S1.3.1 T1.3.1.2: prevents phantom eq/neq entries in
        # kb.relations.
        from ein_bot.inference import predicates as _preds
        if head_name not in kb.relations and not _preds.is_predicate(head_name):
            kb.add_relation(Relation(
                name=head_name, signature=(), declared=False, loc=child.loc,
            ))

        args_tuple = _fact_args(child.args)
        kb.add_fact(Fact(
            relation_name=head_name,
            args=args_tuple,
            layer=layer,
            provenance=provenance,
            raw=child,
            loc=child.loc,
        ))


# ── Public entry point ─────────────────────────────────────────────


def load(forms: Iterable[SForm]) -> KnowledgeBase:
    """Build a populated :class:`KnowledgeBase` from parsed IR forms."""
    kb = KnowledgeBase()
    errors: list[str] = []

    # Collect top-level blocks (multiple of each kind merge).
    ontology_blocks: list[SForm] = []
    rules_blocks: list[SForm] = []
    facts_blocks: list[SForm] = []
    reasoning_blocks: list[SForm] = []
    query_blocks: list[SForm] = []
    # trace_blocks unused for S1.2.1; reserved for S1.2.3.

    for form in forms:
        if not isinstance(form, SForm) or not isinstance(form.head, Atom):
            errors.append(f"unexpected top-level form: {form!r}")
            continue
        h = form.head.name
        if h == "ontology":
            ontology_blocks.append(form)
        elif h == "rules":
            rules_blocks.append(form)
        elif h == "facts":
            facts_blocks.append(form)
        elif h == "reasoning":
            reasoning_blocks.append(form)
        elif h == "query":
            query_blocks.append(form)
        elif h == "trace":
            pass  # S1.2.3 territory.
        else:
            errors.append(f"unknown top-level form: ({h} …)")

    # Pass 1 — types, relations, instances (and *collect* fact-shaped
    # children for pass 3 below).
    deferred_ontology_facts: list[SForm] = []
    for block in ontology_blocks:
        deferred_ontology_facts.extend(_ingest_ontology(block, kb, errors))

    # Pass 2 — rules. After this, rule-name resolution is possible.
    for block in rules_blocks:
        _ingest_rules(block, kb, errors)

    # Pass 3 — facts. Each block carries its own layer.
    _ingest_facts(deferred_ontology_facts, kb, Layer.ONTOLOGY, errors)
    for block in facts_blocks:
        _ingest_facts(block.args, kb, Layer.FACT, errors)
    for block in reasoning_blocks:
        _ingest_facts(block.args, kb, Layer.REASONING, errors)

    # Query (last one wins if there are multiple).
    if query_blocks:
        last = query_blocks[-1]
        kb.query = Query(kw_pairs=last.args)

    # Index rebuild.
    kb.rebuild_indexes()

    # Provenance cycle check — load-time validator. User-authored
    # reasoning blocks can produce circular `:using` chains, which
    # would break derivation-DAG traversal; reject them up-front
    # with a clear message.
    from .provenance import detect_provenance_cycles
    cycles = detect_provenance_cycles(kb.facts, kb._fact_by_id)
    if cycles:
        path = " -> ".join(f"({r} {' '.join(map(str, a))})" for r, a in cycles[0])
        errors.append(f"derivation cycle: {path}")

    if errors:
        raise KBLoadError("; ".join(errors))
    return kb


__all__ = ["KBLoadError", "load"]
