"""IR → KnowledgeBase loader — S1.2.1 T1.2.1.4; flat routing S1.7c.3.

Walks a **flat sequence** of parsed top-level forms and classifies each
by its head (P1.7c): `relation` → relation decl; `rule` / `hrule` →
rule; `query` / `config` → their handlers; `trace` → ignored; **any
other head → a fact** (layer from :func:`_layer_of`). The four
deprecated block wrappers (`ontology` / `facts` / `reasoning` / `rules`)
are still accepted behind a back-compat shim until S1.7c.4.

The load order matters because some entities reference others:
0. Pass 0 — macros (P1.8 S1.5.9): build the `(macro …)` registry so the
   rules pass can expand macro invocations in each clause.
1. Pass 1 — relations (declared signatures), collecting fact-shaped
   ontology children for pass 3.
2. Pass 2 — rules / hrules. After this, rule-name resolution is possible.
3. Pass 3 — facts: any fact whose head matches a rule name OR a declared
   relation name is fine; otherwise create an open-world
   :class:`Relation` so cross-references still work.
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
from pathlib import Path

from ein_bot.ir.macros import Macro, MacroError, expand_macros
from ein_bot.ir.types import Atom, Int, IRNode, KwPair, Range, SForm, String, Var

from .entities import Fact, Layer, Relation, Rule
from .pattern import Pattern
from .provenance import Provenance
from .store import KnowledgeBase, Query


class KBLoadError(ValueError):
    """Raised at end of load with a list of accumulated problems."""


# ── Surface vocabulary the flat classifier keys on (P1.7c) ─────────
#
# The closed declarator set: a top-level head `relation` / `rule` / `hrule`
# / `query` / `config` is a declarator; `trace` is the engine-emitted
# sibling (ignored on load); ANY OTHER head is a fact. Source of truth:
# `docs/kernel/ir/03-ein-lang/06_reserved_names.md`.
_LAYER_BY_NAME = {layer.value: layer for layer in Layer}  # "ontology"/"fact"/"reasoning"


def _layer_of(form: SForm, errors: list[str]) -> Layer:
    """The flat-form layer-attribution rule (S1.7c.1).

    An explicit ``:layer <ontology|fact|reasoning>`` keyword is
    **authoritative**; otherwise the layer is **derived** from the
    provenance annotation already on the form — the same signal the
    ``Provenance`` kind keys on:

    - ``:rule`` / ``:using``  → REASONING (engine working-memory dump)
    - ``:source``             → FACT      (an explicit numbered statement)
    - neither                 → ONTOLOGY  (implicit background assumption)

    `:layer` is consumed here only — ``_fact_args`` drops every kw-pair,
    so it never becomes a fact arg, and the provenance arm is untouched.
    """
    kws = _kw_pairs(form.args)
    explicit = kws.get("layer")
    if explicit is not None:
        name = _atom_name(explicit)
        if name in _LAYER_BY_NAME:
            return _LAYER_BY_NAME[name]
        errors.append(
            f"(:layer {name}) — unknown layer at {form.loc}; "
            f"expected one of {', '.join(sorted(_LAYER_BY_NAME))}")
        # fall through to derivation so loading continues
    if "rule" in kws or "using" in kws:
        return Layer.REASONING
    if "source" in kws:
        return Layer.FACT
    return Layer.ONTOLOGY


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


def _ingest_relation(child: SForm, kb: KnowledgeBase, errors: list[str]) -> bool:
    """Ingest one `(relation Name T1 T2 … :kw v …)` declaration.

    Returns ``True`` iff ``child`` is a `relation` form (handled — even
    if malformed, in which case an error is recorded); ``False`` if the
    head isn't `relation` (the caller treats it as a fact). Shared by
    the wrapped `(ontology …)` pass and the flat top-level routing.
    """
    head = _atom_name(child.head) if isinstance(child.head, Atom) else None
    if head != "relation":
        return False
    # Flat args post-R10; grammar guarantees ≥ 2 SYMBOL args
    # (name + at least one type), followed by optional kw_pairs.
    if len(child.args) < 2:
        errors.append(f"(relation) needs name + signature at {child.loc}")
        return True
    name = _atom_name(child.args[0])
    sig = tuple(a.name for a in child.args[1:] if isinstance(a, Atom))
    if name is None or not sig:
        errors.append(f"malformed (relation) at {child.loc}")
        return True
    if name in _reserved_names():
        errors.append(
            f"relation '{name}' shadows a reserved kernel name at {child.loc}")
        return True
    if name in kb.relations:
        errors.append(f"duplicate relation '{name}' at {child.loc}")
        return True
    kb.add_relation(Relation(
        name=name, signature=sig, declared=True, loc=child.loc,
    ))
    # Also store the declaration as an ordinary fact so rules can
    # introspect signatures via (relation ?R ?A ?B) patterns. The
    # relation decl has already validated SYMBOL args; the fact mirrors them.
    kb.add_fact(Fact(
        relation_name="relation",
        args=(name, *sig),
        layer=Layer.ONTOLOGY,
        loc=child.loc,
    ))
    return True


def _reserved_names() -> frozenset[str]:
    """Names a declaration (`rule` / `hrule` / `relation` / `macro`) may not
    BIND — shadowing kernel vocabulary (P1.8 S1.8.A1 decision D3; was the
    macro-only Q-S1.5.9.2 guard, generalised to all declarators).

    The grammar already SYMBOL-excludes
    ``not``/``and``/``or``/``neq``/``rule``/``hrule``/``query``/``config``/
    ``trace``/``macro``/``import``, so the collidable names that still reach
    the loader as a declared name are the structural primitives
    (``absent``/``false``), the computed predicates (``eq``/``neq``), and
    ``relation`` (kept a plain SYMBOL for `(relation ?R ?A ?B)` patterns).
    ``open`` / ``forall`` are deliberately NOT reserved — they migrated INTO
    the `std.macro` module (S1.5.9). This guard is about *binding* a name; a
    *fact* may still have a reserved head (e.g. a stored ``(not X)`` octagon)."""
    from ein_bot.inference import predicates, primitives
    return primitives.STRUCTURAL | frozenset(predicates.names()) | {"relation"}


def _ingest_macros(
    macro_forms: Iterable[SForm], kb: KnowledgeBase, errors: list[str],
) -> None:
    """Pre-pass: register every `(macro NAME (?p…) BODY)` into ``kb.macros``.

    Runs before :func:`_ingest_rules`, whose clause-expansion step reads this
    registry. A macro whose name shadows reserved kernel vocabulary
    (:func:`_reserved_macro_names`) or duplicates an earlier macro is rejected
    with a load-time error (P1.8 S1.5.9 T1.5.9.1).
    """
    reserved = _reserved_names()
    for form in macro_forms:
        # AST shape: SForm("macro", (name_atom, @params SForm, body)).
        if len(form.args) < 3:
            errors.append(f"(macro) needs name + params + body at {form.loc}")
            continue
        name = _atom_name(form.args[0])
        params_form = form.args[1]
        body = form.args[2]
        if name is None or not isinstance(params_form, SForm):
            errors.append(f"malformed (macro …) at {form.loc}")
            continue
        if name in reserved:
            errors.append(
                f"macro '{name}' shadows a reserved kernel name at {form.loc}")
            continue
        if name in kb.macros:
            errors.append(f"duplicate macro '{name}' at {form.loc}")
            continue
        params = tuple(a.name for a in params_form.args if isinstance(a, Var))
        kb.macros[name] = Macro(
            name=name, params=params, body=body, loc=form.loc,
        )


def _match_disjuncts(node: IRNode) -> list[IRNode] | None:
    """If a rule `:match` is a *top-level* `(or d1 … dn)`, return the
    disjuncts ``[d1, …, dn]`` (trailing kw-pairs dropped); else ``None``.

    S1.7.6 T1.7.6.5 — `or` stays a kernel primitive, but an `(or …)` at
    the head of a rule's `:match` **lowers to one rule per disjunct**,
    exploiting ein's already-disjunctive multiple-rules semantics rather
    than a runtime branch (user design note 2026-05-30). The matcher
    therefore never evaluates `or`; the loader desugars it away. Only the
    top-level position is lowered — a nested `(or …)` (e.g. inside an
    `(and …)`) would need full DNF expansion, deferred until a rule needs
    it (no M1 rule uses `or`).
    """
    if (isinstance(node, SForm)
            and isinstance(node.head, Atom)
            and node.head.name == "or"):
        return [a for a in node.args if not isinstance(a, KwPair)]
    return None


def _ingest_rules(
    rule_forms: Iterable[SForm], kb: KnowledgeBase, errors: list[str],
) -> None:
    """Ingest a sequence of `(rule …)` / `(hrule …)` declarations.

    Fed either a deprecated `(rules …)` block's ``form.args`` or the flat
    top-level stream of rule/hrule forms (S1.7c.3) — same logic both ways.

    S1.5.6b: a `(hrule …)` is structurally a rule but is routed to
    ``kb.hrules`` (hypothesis generators), not ``kb.rules``
    (derivation rules the saturator fires).

    S1.7.6 T1.7.6.5: a `:match` headed by `(or …)` is lowered to one
    `Rule` per disjunct (see :func:`_match_disjuncts`); the instances are
    named ``<rule>__or0``, ``<rule>__or1``, … so they stay distinct in
    the rule registry.
    """
    for child in rule_forms:
        head = _atom_name(child.head) if isinstance(child, SForm) else None
        if head not in ("rule", "hrule"):
            errors.append(f"non-rule form in (rules …): {child}")
            continue
        if len(child.args) < 2:
            errors.append(f"({head}) needs name + params at {child.loc}")
            continue
        name = _atom_name(child.args[0])
        params_form = child.args[1]
        if name is None or not isinstance(params_form, SForm):
            errors.append(f"malformed ({head} …) at {child.loc}")
            continue
        if name in _reserved_names():
            # A rule named `absent`/`false`/`eq`/`relation` would never fire
            # (the compiler/matcher read those as primitives, not the rule) —
            # reject rather than silently register a dead rule (D3).
            errors.append(
                f"{head} '{name}' shadows a reserved kernel name at {child.loc}")
            continue
        # `rule` and `hrule` share one name-space — a name must
        # identify a single declaration.
        if name in kb.rules or name in kb.hrules:
            errors.append(
                f"duplicate rule/hrule name '{name}' at {child.loc}")
            continue
        params = tuple(a.name for a in params_form.args if isinstance(a, Var))
        kws = _kw_pairs(child.args)
        match_node = kws.get("match")
        assert_node = kws.get("assert")
        why_node = kws.get("why")
        priority_node = kws.get("priority")
        if match_node is None or assert_node is None:
            errors.append(
                f"({head} {name}) missing :match or :assert at {child.loc}")
            continue

        # P1.8 S1.5.9 — rewrite `(macro …)` invocations in the clauses
        # before they are compiled. Done before disjunct-lowering so a
        # macro that expands to a top-level `(or …)` is still split into
        # one rule per disjunct. The `kb.macros` guard skips the (rebuild)
        # walk entirely for the common macro-free puzzle.
        if kb.macros:
            try:
                match_node = expand_macros(match_node, kb.macros)
                assert_node = expand_macros(assert_node, kb.macros)
            except MacroError as e:
                errors.append(f"({head} {name}): {e}")
                continue

        why = why_node.value if isinstance(why_node, String) else ""
        priority = priority_node.value if isinstance(priority_node, Int) else None

        # T1.7.6.5 — lower a top-level `(or d1 … dn)` :match into one rule
        # per disjunct. A non-`or` :match is a 1-element list, so the name
        # is unchanged for the common case.
        disjuncts = _match_disjuncts(match_node)
        match_nodes = disjuncts if disjuncts is not None else [match_node]
        for i, mnode in enumerate(match_nodes):
            rname = name if len(match_nodes) == 1 else f"{name}__or{i}"
            if rname in kb.rules or rname in kb.hrules:
                errors.append(
                    f"duplicate rule/hrule name '{rname}' at {child.loc}")
                continue
            rule = Rule(
                name=rname,
                params=params,
                match=Pattern.from_ir(mnode),
                assert_=Pattern.from_ir(assert_node),
                why=why,
                priority=priority,
                loc=child.loc,
            )
            if head == "hrule":
                kb.add_hrule(rule)
            else:
                kb.add_rule(rule)


def _ingest_one_fact(
    child: IRNode, kb: KnowledgeBase, layer: Layer, errors: list[str]
) -> None:
    """Build one Fact entity at the given ``layer`` (from :func:`_layer_of`)."""
    if not isinstance(child, SForm):
        return
    head_name = _atom_name(child.head)
    if head_name is None:
        errors.append(f"fact with non-atom head at {child.loc}")
        return
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


def load(forms: Iterable[SForm], *, base_dir: Path | None = None) -> KnowledgeBase:
    """Build a populated :class:`KnowledgeBase` from parsed IR forms.

    P1.7c — the surface is a **flat sequence of forms**, each classified
    by its head: ``relation`` → a relation declaration; ``rule`` /
    ``hrule`` → a rule; ``macro`` → a pattern macro (S1.5.9);
    ``query`` / ``config`` → their handlers; ``trace`` → ignored;
    **anything else → a fact** with layer from :func:`_layer_of`. (A
    former-wrapper head such as ``(facts …)`` is now just a fact whose
    relation is ``facts``.)

    P1.8 S1.8.A3 — `(import …)` forms are resolved **first**, at the form
    level (:func:`ein_bot.kb.imports.resolve_imports`): each is replaced by
    its module's qualified form-list, so by the time the head-classifier
    below runs the stream is import-free (flatten-then-load — A1 D8).
    ``base_dir`` is the directory file-relative imports resolve against
    (``None`` for an in-memory load — only ``std.*`` imports resolve then).
    """
    kb = KnowledgeBase()
    errors: list[str] = []

    # Resolve imports up front into one flat, import-free, qualified stream.
    # Resolution errors (not found / cycle / bad spec) are fatal and raise
    # immediately (a half-resolved program can't be ingested).
    from .imports import resolve_imports
    forms = resolve_imports(forms, base_dir=base_dir)

    # Flat top-level forms, bucketed into the three classic passes
    # (relations → rules → facts) so resolution order is preserved.
    flat_relations: list[SForm] = []
    flat_rules: list[SForm] = []
    flat_macros: list[SForm] = []
    flat_facts: list[tuple[SForm, Layer]] = []   # each carries its own layer
    query_blocks: list[SForm] = []
    config_blocks: list[SForm] = []

    for form in forms:
        if not isinstance(form, SForm) or not isinstance(form.head, Atom):
            errors.append(f"unexpected top-level form: {form!r}")
            continue
        h = form.head.name
        if h == "relation":
            flat_relations.append(form)
        elif h in ("rule", "hrule"):
            flat_rules.append(form)
        elif h == "macro":
            flat_macros.append(form)
        elif h == "import":
            # resolve_imports (above) consumes every import; a survivor here
            # means a resolver bug — surface it rather than ingest it as a fact.
            errors.append(f"unresolved (import …) at {form.loc} — internal error")
        elif h == "query":
            query_blocks.append(form)
        elif h == "config":
            config_blocks.append(form)
        elif h == "trace":
            pass  # engine-emitted output; parsed by trace/ast.py, not here.
        else:
            # The flat default: any non-reserved head is a fact, with its
            # layer derived (or read off an explicit :layer) per S1.7c.1.
            flat_facts.append((form, _layer_of(form, errors)))

    # Pass 0 — macros (P1.8 S1.5.9). Built first so the rules pass can
    # expand `(macro …)` invocations in every clause. (Imports were already
    # resolved into this stream, so any imported macros are present here too.)
    _ingest_macros(flat_macros, kb, errors)

    # Pass 1 — relations (declared signatures + the auto-stored decl facts).
    for rform in flat_relations:
        _ingest_relation(rform, kb, errors)

    # Pass 2 — rules. After this, rule-name resolution is possible.
    _ingest_rules(flat_rules, kb, errors)

    # Pass 3 — facts. Each flat fact carries its own layer (resolved above).
    for fact_form, layer in flat_facts:
        _ingest_one_fact(fact_form, kb, layer, errors)

    # Query (last one wins if there are multiple).
    if query_blocks:
        last = query_blocks[-1]
        kb.query = Query(kw_pairs=last.args)

    # Config (last one wins if there are multiple). Parse into a
    # `SolverConfig`; on key/value error, surface as a load-time
    # error so puzzle authors catch typos early.
    if config_blocks:
        from ein_bot.inference.config import SolverConfig
        last_cfg = config_blocks[-1]
        try:
            kb.config = SolverConfig.from_kw_pairs(last_cfg.args)
        except ValueError as e:
            errors.append(f"(config …): {e}")

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
