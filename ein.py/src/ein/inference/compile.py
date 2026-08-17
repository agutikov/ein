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
sub-patterns. The runtime matcher (:mod:`ein.inference.match`)
dispatches on type during unification.
"""
from __future__ import annotations

from collections.abc import Iterable
from dataclasses import dataclass, field

from ein.ir.types import Atom, Int, IRNode, KwPair, SForm, Var
from ein.kb.entities import Fact, Rule

from . import predicates, primitives


class CompileError(ValueError):
    """A ``:match`` clause the compiler cannot lower to a faithful plan.

    S1.22.0. Every branch that raises this used to return ``[]`` — silently
    **dropping the premise**, which is never safe in either direction:

    - a dropped positive conjunct makes the match set *larger*, so the rule
      fires on a premise that is false (unsound);
    - a dropped conjunct inside an ``(absent …)`` makes the negative query
      match more often, so the guard fails when it should pass — and, being
      anti-monotone, its candidate is then retired permanently (incomplete);
    - and a plan whose every premise was dropped has ``steps=()``, which the
      matcher treats as one vacuous match, so the rule fires unconditionally.

    All three were reproduced end-to-end before this became an error. An
    unsupported shape is an authoring error and has to say so.
    """


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
class NafGuard:
    """A top-level :class:`AbsentGuard` lifted out of a plan's steps — S1.21.8.

    The closure runs purely positive plans; every ``(absent …)`` is evaluated
    on the closure/world boundary instead, against a positive fixpoint. Which
    means the guard is no longer evaluated *where it was written*, and that
    matters: ``(and (absent (P ?x)) (Q ?x))`` asks "is there no P at all?",
    while ``(and (Q ?x) (absent (P ?x)))`` asks "is there no P for *this* x?".

    :attr:`scope` is what preserves the distinction — the variables bound by
    the positive premises that *preceded* the guard. Boundary evaluation
    projects the completed bindings back down to that set, so lifting a guard
    to the boundary is exactly as strong as evaluating it in place, no more.
    (Rule authors who intended the second reading were already relying on
    premise order; nothing about that changes.)

    :attr:`watched` is the set of relations the negative query reads, nested
    guards included. It is the boundary's invalidation key: if none of those
    relations grew, the sub-plan's match set is unchanged and so is the
    guard's verdict, so a parked candidate can be skipped without re-running
    the query. (This is the same relation set the retired
    ``saturator._absent_relations`` computed — repurposed from "force a full
    re-match" to "decide whether to re-ask", which is the cheaper half.)

    :attr:`monotone` says the guard is **anti-monotone** in the KB: its query
    is purely positive, so once it finds a match it finds one forever (the KB
    only grows within a run) and the guard can never pass again. A candidate
    it rejects is dead, not waiting. The exception — and the reason this is a
    flag rather than an assumption — is a *nested* absent: a ``forall``'s
    ``(absent (and G (absent B)))`` **passes** once enough ``B`` facts exist
    to make the inner absent fail, so it can flip from failing to passing and
    must stay parked.
    """
    guard: AbsentGuard
    scope: frozenset[str]
    watched: frozenset[str] = frozenset()
    monotone: bool = True

    @property
    def sub_steps(self) -> tuple[object, ...]:
        return self.guard.sub_steps


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
    # S1.21.8: `steps` is the disjunct's **purely positive** residue — the
    # closure plan. Its `(absent …)` premises are lifted out to `naf_guards`
    # and evaluated on the closure/world boundary instead
    # (`ein.inference.world.World`), against a positive fixpoint rather than
    # against whatever the KB happened to hold mid-saturation.
    #
    # `naf_guards[0]` belongs to `steps`; `naf_guards[i]` to
    # `extra_match_plans[i-1]`. Use :meth:`disjuncts` rather than indexing —
    # it pairs them and is what makes an or-disjunct's guards impossible to
    # forget (the D5 gap: the old fire-time re-check walked `steps` only).
    naf_guards: tuple[tuple[NafGuard, ...], ...] = ()
    # S1.8.A13: a `:match (or d1 … dm)` compiles to ONE plan with `steps` =
    # the first disjunct and `extra_match_plans` = the remaining disjuncts'
    # step tuples. `match.run` executes `steps` plus every entry here, so
    # every match.run caller transparently sees all disjuncts (no rule-split,
    # no `__or<i>` siblings). Empty for the common single-`:match` rule.
    extra_match_plans: tuple[tuple[object, ...], ...] = ()
    # S1.8.A13: a `:match`/`:assert` may conclude SEVERAL facts. Each entry is
    # a compiled conclusion template (Var / Atom / Int / NestedPattern slots);
    # `fire()` walks them all and emits one Firing with N derived facts. A
    # single-`:assert` rule is a 1-tuple; the `assert_template` property below
    # keeps the single-fact reader path working.
    assert_templates: tuple[object, ...] = ()
    why: str = ""

    @property
    def assert_template(self) -> object | None:
        """The first conclusion template — back-compat for single-assert
        readers (closure/NAF analysis, hrule). Multi-assert consumers
        (`fire`, the saturator's redundancy check, lookahead) walk
        `assert_templates` directly."""
        return self.assert_templates[0] if self.assert_templates else None

    def disjuncts(self) -> tuple[tuple[tuple[object, ...], tuple[NafGuard, ...]], ...]:
        """Every ``(positive_steps, naf_guards)`` pair — S1.21.8.

        One entry for ``steps`` plus one per ``extra_match_plans`` disjunct.
        Every caller that runs a match must iterate this rather than
        ``steps``/``extra_match_plans`` separately: pairing the guards with
        their disjunct is what makes the S1.8.A13 or-case impossible to miss.
        """
        seqs = (self.steps, *self.extra_match_plans)
        guards = self.naf_guards
        return tuple(
            (s, guards[i] if i < len(guards) else ())
            for i, s in enumerate(seqs)
        )

    @property
    def has_naf(self) -> bool:
        """True iff any disjunct carries an ``(absent …)`` guard."""
        return any(g for g in self.naf_guards)


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
        # S1.22.0 — this used to `return []`, dropping the premise. An
        # unbound head var reaches here only when the activator failed to
        # bind the rule's parameter (`compile_rule`'s arity-mismatch path),
        # and a dropped premise is silently wrong: if it was the plan's only
        # one, `steps` is `()` and the matcher yields one vacuous match, so
        # the rule fires unconditionally. Reproduced — a rule with a ground
        # `:assert` stored its conclusion off an arity-mismatched activator.
        raise CompileError(
            f"unbound relation head ?{head.name} in a premise of "
            f"`{node!r}` — M1 matches relations per activator (Q29), so the "
            f"head var must be bound by the rule's activator. Check that the "
            f"activator fact's arity matches the rule's parameter list."
        )
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
        if not sub_steps:
            # S1.22.0 — an empty sub-plan is not "a query that finds nothing",
            # it is a query that matches VACUOUSLY: `_run_steps(())` yields one
            # match, so `World.holds` is True and the guard fails against every
            # possible KB. `monotone` is then True (no nested absent), so the
            # candidate is retired permanently on its first judgement. A
            # guard that can never pass is always an authoring mistake.
            raise CompileError(
                f"`(absent …)` sub-plan compiled to no steps in {node!r} — "
                f"the guard would fail against every KB and its candidates "
                f"would be retired permanently."
            )
        return [AbsentGuard(sub_steps=tuple(sub_steps))]

    # `(forall …)` / `(open …)` are no longer compiler-level sugar: since
    # P1.8 S1.5.9 they are ein-lang `(macro …)` declarations expanded at
    # LOAD time (kb.from_ir → ir.macros), so by the time the compiler runs
    # they have already become `(absent (and G (absent B)))` /
    # `(and (absent P) (absent (not P)))`. See the `std.macro` module
    # (ein/stdlib/macro.ein).

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

    # `(or …)` — disjunction. A *top-level* `(or …)` in a rule :match is split
    # into one match plan per disjunct by `compile_rule` (S1.8.A13; via
    # `_match_disjuncts`) BEFORE `_compile_premise` runs, so each disjunct
    # arrives here already unwrapped — `_compile_premise` never sees the `or`.
    # A *nested* `(or …)` (e.g. inside `(and …)` or inside `(absent …)`) would
    # need DNF expansion and is unsupported.
    #
    # S1.22.0 — this used to `return []`, which did not make the shape
    # unsupported, it made it silently WRONG in both polarities. Reproduced:
    # `(and (a ?x) (or (p ?x) (q ?x)))` fired with neither p nor q in the KB
    # (unsound), and `(absent (or (p ?x) (q ?x)))` never fired with neither in
    # the KB (incomplete, and permanently so — the empty sub-plan makes the
    # guard fail, and a failing monotone guard retires its candidate).
    if head_name == primitives.OR:
        raise CompileError(
            f"nested `(or …)` in a `:match` premise: {node!r}. Only a "
            f"TOP-LEVEL `(or …)` is supported (S1.8.A13 splits it into one "
            f"plan per disjunct); a nested one needs DNF expansion, which "
            f"the M1 compiler does not do. Lift the disjunction to the top "
            f"of the `:match`, or split the rule."
        )

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


def _has_nested_absent(steps: tuple[object, ...]) -> bool:
    """True iff a guard sub-plan contains another ``(absent …)``.

    The one shape whose verdict is not anti-monotone in the KB — see
    :attr:`NafGuard.monotone`.
    """
    return any(isinstance(st, AbsentGuard) for st in steps)


def _watched_relations(steps: tuple[object, ...]) -> frozenset[str]:
    """Every relation a guard sub-plan reads, through nested guards too.

    The boundary's invalidation key — see :attr:`NafGuard.watched`. A
    ``(not (R …))`` pattern contributes ``"not"``, which is right: the query
    reads the *stored negative* fact, and that is the relation whose growth
    can change the answer.
    """
    out: set[str] = set()

    def walk(seq: tuple[object, ...]) -> None:
        for st in seq:
            if isinstance(st, (Scan, Join)):
                out.add(st.relation)
            elif isinstance(st, AbsentGuard):
                walk(st.sub_steps)

    walk(steps)
    return frozenset(out)


def split_naf(
    steps: tuple[object, ...], seed_vars: Iterable[str] = (),
) -> tuple[tuple[object, ...], tuple[NafGuard, ...]]:
    """Partition a compiled step tuple into (positive residue, NAF guards).

    S1.21.8 — the compile half of the closure/worlds split. Top-level
    :class:`AbsentGuard` steps are lifted out; what remains is a purely
    positive Scan/Join/Guard plan the closure can run to a fixpoint without
    ever consulting negation.

    Each lifted guard records the variables bound by the positive premises
    *before* it (:attr:`NafGuard.scope`), so boundary evaluation can restore
    the binding context the guard was written in — see :class:`NafGuard`.

    Nested ``AbsentGuard``s inside a guard's own ``sub_steps`` (what a
    ``forall`` desugars to: ``(absent (and G (absent B)))``) are **not**
    lifted. They are part of the negative query, not of the closure, and the
    boundary evaluates them as one unit.

    ``seed_vars`` are the rule's **parameter** variables (the keys of
    :attr:`JoinPlan.bindings_seed`, bound from the activator fact). They are
    in scope for the *whole* match, at every premise position, so every
    guard's scope starts from them. Omitting them is not cosmetic: a
    predicate :class:`Guard` inside an ``(absent …)`` is compiled from the
    **raw IR nodes** — unlike a relation slot, ``_slot`` never substitutes a
    bound parameter into it — so an `(eq ?y ?PARAM)` under a guard would
    resolve ``?PARAM`` to ``None`` at the boundary, the negative query would
    find nothing, and the guard would pass when it must fail.
    """
    positive: list[object] = []
    guards: list[NafGuard] = []
    bound: set[str] = set(seed_vars)
    for st in steps:
        if isinstance(st, AbsentGuard):
            guards.append(NafGuard(
                guard=st, scope=frozenset(bound),
                watched=_watched_relations(st.sub_steps),
                monotone=not _has_nested_absent(st.sub_steps),
            ))
            continue
        positive.append(st)
        if isinstance(st, (Scan, Join)):
            _collect_vars(st.arg_slots, bound)
    return tuple(positive), tuple(guards)


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


def _match_disjuncts(expr: IRNode) -> list[IRNode]:
    """A top-level ``(or d1 … dm)`` :match → ``[d1, …, dm]``; else ``[expr]``.

    S1.8.A13: the disjunction lowers to multiple compiled match plans on ONE
    rule (`steps` + `extra_match_plans`), not the old load-time `__or<i>` rule
    clones. Only the top-level position is split; a nested `(or …)` still needs
    DNF and is unsupported (`_compile_premise`)."""
    if (isinstance(expr, SForm) and isinstance(expr.head, Atom)
            and expr.head.name == primitives.OR):
        return [a for a in expr.args if not isinstance(a, KwPair)]
    return [expr]


def _assert_conjuncts(expr: IRNode) -> list[IRNode]:
    """A top-level ``(and c1 … ck)`` :assert → ``[c1, …, ck]``; else ``[expr]``.

    S1.8.A13: one match concludes several facts in a single firing (see
    `JoinPlan.assert_templates` / `fire`), not the old `__and<j>` rule clones —
    so a *generic* rule may multi-assert (the rule keeps its name, activation is
    unaffected). Only the top level is split; a nested `(and …)` (inside a
    `(not …)`, say) is left to the asserter."""
    if (isinstance(expr, SForm) and isinstance(expr.head, Atom)
            and expr.head.name == primitives.AND):
        return [a for a in expr.args if not isinstance(a, KwPair)]
    return [expr]


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
            # S1.22.0 — this used to leave `bindings` empty and comment that
            # "the matcher will reject via the 'unbound head var' branch".
            # There is no such branch in `match._run_steps`: the rejection was
            # `_compile_relation` returning `[]`, i.e. dropping the premise —
            # and a plan whose premises all drop has `steps=()`, which the
            # matcher accepts as one VACUOUS match. Reproduced: an
            # arity-mismatched activator fired its rule unconditionally and
            # stored a ground conclusion.
            #
            # Both drivers now filter mismatched activators before they get
            # here (`Engine._activators_for`, `hrule.Hrules`), because a fact
            # that cannot bind the parameters does not authorise the rule.
            # A direct caller that constructs the pair anyway gets told.
            raise CompileError(
                f"activator {activator.relation_name}{args!r} has "
                f"{len(args)} argument(s) but rule `{rule.name}` declares "
                f"{len(params)} parameter(s) {params!r} — it cannot bind "
                f"them, so it does not activate this rule."
            )
        else:
            for p, a in zip(params, args, strict=True):
                if isinstance(a, (str, int)):
                    bindings[p] = a
        activator_args = tuple(
            a for a in activator.args if isinstance(a, str)
        )

    # Match: a top-level `(or …)` lowers to one step-tuple per disjunct
    # (S1.8.A13). `steps` is the first; `extra_match_plans` the rest — `match.run`
    # executes them all. Each disjunct compiles with the same activator bindings
    # and its own fresh body-var scope.
    if rule.match is None:
        raw_seqs: list[tuple[object, ...]] = [()]
    else:
        raw_seqs = [compile_pattern(d, bindings)
                    for d in _match_disjuncts(rule.match.expr)]
    # S1.21.8: lift each disjunct's `(absent …)` premises out of the closure
    # plan. `match_seqs` is what the matcher runs (purely positive);
    # `guard_seqs` is what the closure/world boundary evaluates.
    split = [split_naf(s, bindings) for s in raw_seqs]
    match_seqs = [s for s, _g in split]
    guard_seqs = tuple(g for _s, g in split)

    # Assert: a top-level `(and …)` lowers to one template per conjunct — Var
    # slots stay unbound (filled at firing time); Atom/Int/NestedPattern stay
    # literal. `fire()` emits all of them in one Firing.
    if rule.assert_ is None:
        assert_templates: tuple[object, ...] = ()
    else:
        assert_templates = tuple(
            _lower_assert(c, bindings)
            for c in _assert_conjuncts(rule.assert_.expr)
        )

    return JoinPlan(
        rule_name=rule.name,
        activator_args=activator_args,
        bindings_seed=dict(bindings),
        steps=match_seqs[0],
        naf_guards=guard_seqs,
        extra_match_plans=tuple(match_seqs[1:]),
        assert_templates=assert_templates,
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

    # S1.21.8 — guards now live in `naf_guards`, not in `steps`, and every
    # disjunct's are covered (they were not before: the walk read `steps`
    # only, so an or-disjunct's guards were invisible here too).
    for _steps, guards in plan.disjuncts():
        for g in guards:
            walk(g.sub_steps)
    return out


__all__ = [
    "AbsentGuard",
    "CompileError",
    "Guard",
    "Join",
    "JoinPlan",
    "NafGuard",
    "NestedPattern",
    "Scan",
    "asserted_relation",
    "compile_pattern",
    "compile_rule",
    "naf_relation_refs",
    "negated_relation",
    "split_naf",
]
