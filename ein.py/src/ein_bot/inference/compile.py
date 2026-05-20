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
- :class:`NegativeGuard`  — negation-as-failure: run a sub-plan; the
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

from . import predicates

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
class NegativeGuard:
    """Negation-as-failure: the parent continues iff the sub-plan
    yields zero matches against the current bindings.
    """
    sub_steps: tuple[object, ...]   # tuple of Scan/Join/Guard/NegativeGuard


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

    # `(not P)` — negation-as-failure wrapper.
    if head_name == "not" and len(node.args) >= 1:
        sub_steps = _compile_body(node.args[0], bindings, known_vars)
        return [NegativeGuard(sub_steps=tuple(sub_steps))]

    # `(and P1 P2 …)` — flatten into sibling premises in the same plan.
    if head_name == "and":
        steps: list[object] = []
        for child in node.args:
            steps.extend(_compile_premise(child, bindings, known_vars))
        return steps

    # `(or …)` — disjunction. Not in M1 zebra.ein; the saturation
    # engine handles branching via :match alternatives in a future
    # extension. Leave as a structural step the matcher rejects.
    if head_name == "or":
        # Not yet supported; emit nothing so the loader doesn't trip.
        # Tests in S1.3.2 will reveal if a zebra-class rule needs this.
        return []

    # Predicate dispatch: head matches a registered built-in.
    if head_name is not None and predicates.is_predicate(head_name):
        return [Guard(predicate=head_name, args=tuple(node.args))]

    # `neq` is shape-pinned by the grammar (neq_form has its own
    # production), so it arrives with head Atom("neq"). It's also
    # in the predicate registry; the branch above handles it.

    # `(instance Ent Type)` — kernel meta-primitive; matched as if a
    # binary relation `instance`. The KB's `_facts_by_relation` is
    # populated for "instance" by the loader (instances ARE facts).
    if head_name == "instance":
        slots = tuple(_slot(a, bindings) for a in node.args)
        shared = _shared_vars(slots, known_vars)
        step: object = Join("instance", slots, shared) if shared else Scan("instance", slots)
        _collect_vars(slots, known_vars)
        return [step]

    # Relation pattern: `(REL args…)` or `(?rel args…)`. After the
    # activator binding the head is either an Atom (literal relation
    # name) or a Var that wasn't bound by the activator — the latter
    # would imply genuinely-relation-polymorphic matching across all
    # relations, which M1 does NOT support (Q29 / activator model).
    if isinstance(head, Atom):
        rel_name = head.name
    elif isinstance(head, Var) and head.name in bindings:
        rel_name = str(bindings[head.name])
    else:
        # Unbound head var — not supported in M1's activator model.
        # The saturation engine would have to enumerate all relations.
        # Skip silently for now; S1.3.2's tests will surface it.
        return []

    slots = tuple(_slot(a, bindings) for a in node.args if not isinstance(a, KwPair))
    shared = _shared_vars(slots, known_vars)
    step = Join(rel_name, slots, shared) if shared else Scan(rel_name, slots)
    _collect_vars(slots, known_vars)
    return [step]


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


__all__ = [
    "Guard",
    "Join",
    "JoinPlan",
    "NegativeGuard",
    "NestedPattern",
    "Scan",
    "compile_pattern",
    "compile_rule",
]
