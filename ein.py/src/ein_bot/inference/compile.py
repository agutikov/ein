"""Per-(rule, activator) pattern compiler — S1.3.1 T1.3.1.3.

Q29 (resolved 2026-05-20) picked **option B with compile unit =
per (rule, activator-binding) pair**. The activator fact (e.g.
``(transitive co-located)``) binds the rule's parameter list
*before* matching begins; the compiler substitutes the parameters
and bakes concrete relation names into the program.

Cache key: ``(rule.name, activator.args)``.

The compiler emits a sequence of opcodes (a :class:`JoinPlan`):

- :class:`Scan`           — look up facts by relation name; bind args.
- :class:`Join`           — same as Scan, but expects prior bindings
                            on shared variables (currently informational
                            — the unifier handles both identically).
- :class:`Guard`          — evaluate a built-in predicate on current
                            bindings; pass-through or prune.
- :class:`AbsentGuard`    — negation-as-failure: run a sub-plan; the
                            parent continues iff the sub-plan yields
                            zero matches.
- :class:`NestedPattern`  — sub-pattern unified against a ``Fact``-
                            valued argument (Q40 Option A — relational
                            nodes as args).

Slot values inside opcodes are raw IR nodes — :class:`Var` for
unbound positions, :class:`Atom` for literal symbols, :class:`Int`
for literal integers — or :class:`NestedPattern` for relational-node
sub-patterns. The runtime matcher (:mod:`ein_bot.inference.match`)
dispatches on type during unification.
"""
from __future__ import annotations

from dataclasses import dataclass, field

from ein_bot.ir.types import Atom, Int, IRNode, KwPair, SForm, Var
from ein_bot.kb.entities import Fact, Rule

from . import predicates, primitives

# ── Opcode types ───────────────────────────────────────────────────


@dataclass(frozen=True)
class NestedPattern:
    """A sub-pattern matched against a ``Fact``-valued argument.

    Used when a rule's ``:match`` contains a parenthesised SForm in
    an arg position — e.g. ``(hypothesis (co-located ?a ?b))``. The
    runtime unifier recurses into the nested ``Fact``.
    """
    relation: str
    arg_slots: tuple[object, ...]   # IR slot type — see _slot_type


@dataclass(frozen=True)
class Scan:
    """First relation lookup in a plan.

    The compiler bakes the relation name in (from activator binding
    or literal head); ``arg_slots`` are the rule body's arguments at
    that position, kept as IR nodes for the unifier.
    """
    relation: str
    arg_slots: tuple[object, ...]


@dataclass(frozen=True)
class Join:
    """Subsequent relation lookup expecting prior bindings.

    Semantically identical to :class:`Scan` from the unifier's
    perspective — both call ``_bind_args``. The split exists to make
    trace generation legible (``Scan`` = "this is the entry point";
    ``Join`` = "this depends on earlier scans"). ``shared_vars``
    records which variables are shared with prior steps; the unifier
    does not need it for correctness but it's available for tracing.
    """
    relation: str
    arg_slots: tuple[object, ...]
    shared_vars: frozenset[str] = field(default_factory=frozenset)


@dataclass(frozen=True)
class Guard:
    """Evaluate a built-in predicate against current bindings."""
    predicate: str
    args: tuple[object, ...]


@dataclass(frozen=True)
class AbsentGuard:
    """Explicit negation-as-failure: the parent continues iff the
    sub-plan yields zero matches against the current bindings.

    Emitted by ``(absent P)`` in ``:match`` (S1.5.8c K-Δ.2). The
    older spelling — ``(not P)`` defaulting to NAF — was dropped
    in S1.5.8c K-Δ.1, freeing ``(not P)`` to mean what it always
    meant in ``:assert``: a stored fact with head ``not`` and the
    inner pattern as its nested arg.
    """
    sub_steps: tuple[object, ...]   # tuple of Scan/Join/Guard/AbsentGuard


@dataclass(frozen=True)
class JoinPlan:
    """The compiled form of a rule's ``:match`` clause.

    Steps execute in order; each step extends or filters the running
    bindings dict. A final yield from the matcher hands the bindings
    + premise facts to the firing module for ``:assert`` substitution.
    """
    rule_name: str
    activator_args: tuple[str, ...]
    bindings_seed: dict[str, str | int] = field(default_factory=dict)
    steps: tuple[object, ...] = ()
    # Compiled :assert pattern — its slot values are also IR nodes
    # (Var / Atom / Int / NestedPattern). The asserter walks this
    # and builds the derived Fact at firing time.
    assert_template: object | None = None
    why: str = ""


# ── Compile entry points ───────────────────────────────────────────


def _slot(node: IRNode, bindings: dict[str, str | int]) -> object:
    """Lower an IR slot node to a compiled slot value.

    - ``Var`` whose name is in ``bindings`` → substitute to a literal
      :class:`Atom` (we keep the IR type so the unifier treats it
      uniformly with literal-headed slots).
    - ``Var`` unbound → keep as-is.
    - ``Atom`` / ``Int`` → keep as-is (literal slot).
    - ``SForm`` → recursively compile as a :class:`NestedPattern`,
      with the head substituted from bindings when it's a bound Var
      (T2 activator-binding case: the head ``?rel`` becomes a literal
      relation name).
    """
    if isinstance(node, Var):
        if node.name in bindings:
            val = bindings[node.name]
            if isinstance(val, int):
                return Int(value=val)
            return Atom(name=str(val))
        return node
    if isinstance(node, (Atom, Int)):
        return node
    if isinstance(node, SForm):
        head = node.head
        if isinstance(head, Atom):
            rel_name = head.name
        elif isinstance(head, Var) and head.name in bindings:
            # T2: head var bound by the activator → literal relation.
            rel_name = str(bindings[head.name])
        else:
            # Unrecognised — return as-is. Validator catches malformed
            # slots at load time; this is a safety net.
            return node
        return NestedPattern(
            relation=rel_name,
            arg_slots=tuple(_slot(a, bindings) for a in node.args),
        )
    return node


def _shared_vars(slots: tuple[object, ...], known: set[str]) -> frozenset[str]:
    """Variables in `slots` that have already appeared in `known`."""
    out: set[str] = set()

    def walk(s: object) -> None:
        if isinstance(s, Var):
            if s.name in known:
                out.add(s.name)
            return
        if isinstance(s, NestedPattern):
            for inner in s.arg_slots:
                walk(inner)

    for s in slots:
        walk(s)
    return frozenset(out)


def _collect_vars(slots: tuple[object, ...], into: set[str]) -> None:
    """Add the variable names of `slots` to `into` (recursively)."""
    for s in slots:
        if isinstance(s, Var):
            into.add(s.name)
        elif isinstance(s, NestedPattern):
            _collect_vars(s.arg_slots, into)


def _compile_relation(
    node: SForm,
    head: IRNode,
    bindings: dict[str, str | int],
    known_vars: set[str],
) -> list[object]:
    """Compile a relation pattern `(REL args…)` / `(?rel args…)` into a
    Scan or Join step.

    After activator binding the head is a literal relation :class:`Atom`
    or a :class:`Var` bound by the activator; an unbound head var
    (genuinely relation-polymorphic matching across all relations) is
    unsupported in M1's activator model (Q29) and skipped silently.
    `(not P)` arrives here as relation "not" with the inner expression a
    NestedPattern arg, matching stored `(not P)` facts (S1.5.8c K-Δ.1);
    `(instance Ent Type)` is likewise an ordinary binary relation (S1.7.6).
    """
    if isinstance(head, Atom):
        rel_name = head.name
    elif isinstance(head, Var) and head.name in bindings:
        rel_name = str(bindings[head.name])
    else:
        return []
    slots = tuple(_slot(a, bindings) for a in node.args if not isinstance(a, KwPair))
    shared = _shared_vars(slots, known_vars)
    step = Join(rel_name, slots, shared) if shared else Scan(rel_name, slots)
    _collect_vars(slots, known_vars)
    return [step]


def _compile_premise(
    node: IRNode,
    bindings: dict[str, str | int],
    known_vars: set[str],
) -> list[object]:
    """Compile a single premise of the match body into one or more steps.

    Skips ``KwPair`` nodes — the engine no longer recognises any
    in-match keywords (Q32 dropped ``:where``). Authors should write
    positional predicates instead.
    """
    if isinstance(node, KwPair):
        # Q32: `:where` and any other in-match kw_pair is dropped at
        # compile time. The grammar still accepts them; the engine
        # ignores them. Loud failure was considered and rejected —
        # the migration path leaves authoring tools tolerant.
        return []
    if not isinstance(node, SForm):
        return []
    head = node.head
    head_name = head.name if isinstance(head, Atom) else None

    # `(absent P)` — explicit negation-as-failure. S1.5.8c K-Δ.2.
    if head_name == primitives.ABSENT and len(node.args) >= 1:
        sub_steps = _compile_body(node.args[0], bindings, known_vars)
        return [AbsentGuard(sub_steps=tuple(sub_steps))]

    # `(forall …)` / `(open …)` are no longer compiler-level sugar: since
    # P1.8 S1.5.9 they are ein-lang `(macro …)` declarations expanded at
    # LOAD time (kb.from_ir → ir.macros), so by the time the compiler runs
    # they have already become `(absent (and G (absent B)))` /
    # `(and (absent P) (absent (not P)))`. See examples/stdlib/macro.ein
    # (the `std.macro` module).

    # `(not P)` falls through to the generic relation handler
    # below — it compiles as a fact pattern with relation "not"
    # and the inner expression as a NestedPattern arg, matching
    # stored `(not P)` facts in the KB (S1.5.8c K-Δ.1). The old
    # NAF default was removed in 2026-05-24; use `(absent P)`
    # explicitly when negation-as-failure is what you want.

    # `(and P1 P2 …)` — flatten into sibling premises in the same plan.
    if head_name == primitives.AND:
        steps: list[object] = []
        for child in node.args:
            steps.extend(_compile_premise(child, bindings, known_vars))
        return steps

    # `(or …)` — disjunction. A *top-level* `(or …)` in a rule :match is
    # lowered to one rule per disjunct at LOAD time (S1.7.6 T1.7.6.5;
    # `kb.from_ir._match_disjuncts`), exploiting the already-disjunctive
    # multiple-rules semantics — so it never reaches the compiler. A
    # *nested* `(or …)` (e.g. inside `(and …)`) would need DNF expansion
    # and is unsupported; emit nothing so the loader doesn't trip.
    if head_name == primitives.OR:
        return []

    # Predicate dispatch: head matches a registered built-in.
    if head_name is not None and predicates.is_predicate(head_name):
        return [Guard(predicate=head_name, args=tuple(node.args))]

    # `neq` is shape-pinned by the grammar (neq_form has its own
    # production), so it arrives with head Atom("neq"). It's also
    # in the predicate registry; the branch above handles it.

    # Relation pattern `(REL args…)` / `(?rel args…)`. `(not P)` and
    # `(instance Ent Type)` are ordinary relations here (S1.5.8c / S1.7.6).
    return _compile_relation(node, head, bindings, known_vars)


def _compile_body(
    expr: IRNode,
    bindings: dict[str, str | int],
    known_vars: set[str] | None = None,
) -> list[object]:
    """Compile a match body (top-level or a ``(not …)`` inner sub-form)."""
    if known_vars is None:
        known_vars = set()
    return _compile_premise(expr, bindings, known_vars)


def compile_pattern(
    expr: IRNode,
    bindings: dict[str, str | int],
) -> tuple[object, ...]:
    """Compile a ``:match`` expression to a step tuple.

    ``bindings`` substitutes the rule's parameter vars (the activator
    binding) into the body before walking. Free vars remain :class:`Var`
    nodes in the emitted opcodes; the runtime matcher binds them.
    """
    return tuple(_compile_body(expr, bindings))


# ── Rule-level compile (driven by the engine) ──────────────────────


def compile_rule(rule: Rule, activator: Fact | None) -> JoinPlan:
    """Compile a (rule, activator) pair into a :class:`JoinPlan`.

    ``activator`` is None for non-T2 (parameter-less) rules like
    ``type-exclusivity``. For T2 rules, the activator's args bind to
    the rule's params positionally.
    """
    bindings: dict[str, str | int] = {}
    activator_args: tuple[str, ...] = ()
    if activator is not None:
        # Bind each rule param to the matching activator arg by position.
        # Activator args are str/int/Fact; we only consume str/int here.
        params = rule.params
        args = activator.args
        if len(params) != len(args):
            # Shape mismatch — leave bindings empty so the compiler
            # produces a plan with unbound head vars (which the
            # matcher will reject via the "unbound head var" branch).
            pass
        else:
            for p, a in zip(params, args, strict=True):
                if isinstance(a, (str, int)):
                    bindings[p] = a
        activator_args = tuple(
            a for a in activator.args if isinstance(a, str)
        )

    if rule.match is None:
        steps: tuple[object, ...] = ()
    else:
        steps = compile_pattern(rule.match.expr, bindings)

    # The :assert template is lowered the same way — Var slots stay
    # unbound (to be filled at firing time); Atom/Int/NestedPattern
    # slots stay literal.
    assert_template: object | None = None
    if rule.assert_ is not None:
        assert_template = _lower_assert(rule.assert_.expr, bindings)

    return JoinPlan(
        rule_name=rule.name,
        activator_args=activator_args,
        bindings_seed=dict(bindings),
        steps=steps,
        assert_template=assert_template,
        why=rule.why,
    )


def _lower_assert(expr: IRNode, bindings: dict[str, str | int]) -> object:
    """Lower the ``:assert`` clause into a slot-tree the asserter walks.

    Returns either:
    - ``NestedPattern`` — a fact-shaped assertion ``(REL args…)``.
    - A leaf slot (Var / Atom / Int) — rare; rules typically assert
      whole facts.

    ``(not (REL args…))`` lowers to ``NestedPattern("not",
    (NestedPattern("REL", …),))`` — the outer fact has a nested
    ``Fact`` arg (Q40 / R9's widening).
    """
    return _slot(expr, bindings)


# ── Plan introspection (pure walks; no KB) — S1.7.4 ─────────────────


def asserted_relation(plan: JoinPlan) -> str | None:
    """The positive relation a plan's ``:assert`` concludes, or ``None``.

    A plan whose assert template is ``(R …)`` with head not ``not``
    proves ``R`` is rule-derivable. ``(not (R …))`` asserts a negation
    (use :func:`negated_relation`) and a leaf/absent assert proves
    nothing positive — both return ``None``. This is the building block
    behind [`closed.producible_relations`](closed.py).
    """
    t = plan.assert_template
    if isinstance(t, NestedPattern) and t.relation != primitives.NOT:
        return t.relation
    return None


def negated_relation(plan: JoinPlan) -> str | None:
    """The relation ``R`` a plan's ``:assert`` *negates* via ``(not (R …))``,
    or ``None``. The dual of :func:`asserted_relation` — it answers
    "does some rule derive a ``(not (R …))`` fact?", which is what an
    ``(absent (not (R …)))`` guard (a ``forall``/totality NAF) watches.
    """
    t = plan.assert_template
    if isinstance(t, NestedPattern) and t.relation == primitives.NOT:
        for inner in t.arg_slots:
            if isinstance(inner, NestedPattern):
                return inner.relation
    return None


def naf_relation_refs(plan: JoinPlan) -> list[tuple[str, bool]]:
    """``(relation, negated)`` pairs every ``AbsentGuard`` in ``plan`` watches.

    Recurses through nested ``AbsentGuard`` steps (the ``forall`` macro
    expands to ``(absent (and G (absent B)))`` — S1.7.4 Q-S1.7.4.B says
    enter both levels). A ``(not (R …))`` sub-pattern —
    a ``Scan``/``Join`` on relation ``"not"`` carrying a
    :class:`NestedPattern` arg — yields ``(R, True)`` (the genuine
    watched relation is the nested one); any other relation lookup
    yields ``(rel, False)``. Order-preserving; the caller dedupes.

    Activator-bound head vars (``?S`` in ``adjacent-via-*``, ``?R`` in
    the elimination rules) are already substituted to literal relation
    names by :func:`compile_rule`, so the names returned here are
    concrete and per-activator.
    """
    out: list[tuple[str, bool]] = []

    def walk(steps: tuple[object, ...]) -> None:
        for st in steps:
            if isinstance(st, (Scan, Join)):
                if st.relation == primitives.NOT:
                    for slot in st.arg_slots:
                        if isinstance(slot, NestedPattern):
                            out.append((slot.relation, True))
                else:
                    out.append((st.relation, False))
            elif isinstance(st, AbsentGuard):
                walk(st.sub_steps)

    for st in plan.steps:
        if isinstance(st, AbsentGuard):
            walk(st.sub_steps)
    return out


__all__ = [
    "AbsentGuard",
    "Guard",
    "Join",
    "JoinPlan",
    "NestedPattern",
    "Scan",
    "asserted_relation",
    "compile_pattern",
    "compile_rule",
    "naf_relation_refs",
    "negated_relation",
]
