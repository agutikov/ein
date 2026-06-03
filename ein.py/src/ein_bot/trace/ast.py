"""Trace AST — S1.6.4 T1.6.4.1.

A trace is itself IR: a list of `(step …)` forms carrying the rule
name, the premises it consumed, the derived edge, the variable
bindings, and a generated English explanation. :class:`TraceStep` is
that AST; :func:`trace_to_ir` / :func:`parse_trace_steps` round-trip it
through the P1.1 parser as a `(trace …)` form (the same form
:func:`ein_bot.ir.to_dot.render_trace` renders).

The serialisable core is ``(n, rule, premises, derived, why)`` +
bindings. ``diagram`` (the inline DOT slice) and ``section`` (the
clustering key) are *render-time* enrichments — not serialised; they
come back ``None`` after a round-trip.
"""
from __future__ import annotations

from dataclasses import dataclass, field

from ..ir.strings import escape_string_literal
from ..ir.types import Atom, KwPair, SForm, String
from ..ir.types import Int as IRInt
from ..render.dot_util import fact_label

# A fact reference: (relation_name, args) — args are str / int / nested
# FactRef. Mirrors the kernel FactId shape.
FactRef = tuple[str, tuple]


@dataclass
class TraceStep:
    """One narrated reasoning move (one rule firing)."""

    n:        int
    rule:     str
    premises: tuple[FactRef, ...]
    derived:  FactRef
    bindings: dict[str, str] = field(default_factory=dict)
    why:      str = ""
    diagram:  str | None = None   # inline DOT slice (S1.6.2) — render-time
    section:  str | None = None   # clustering key (target entity) — render-time
    sources:  tuple[str, ...] = ()  # quoted source sentences (T1.6.4.5)
    conditional: bool = False     # depends on a hypothesis (commitment) fact

    @property
    def derived_label(self) -> str:
        return fact_label(self.derived[0], self.derived[1])

    def premise_labels(self) -> tuple[str, ...]:
        return tuple(fact_label(p[0], p[1]) for p in self.premises)


# ── IR round-trip ──────────────────────────────────────────────────

def _arg_to_sexpr(a: object) -> str:
    """Format one fact argument as S-expression text."""
    if isinstance(a, tuple) and len(a) == 2 and isinstance(a[0], str):
        return _fact_to_sexpr(a[0], a[1])                       # nested FactRef
    if hasattr(a, "relation_name") and hasattr(a, "args"):       # nested Fact
        return _fact_to_sexpr(a.relation_name, a.args)
    if isinstance(a, bool):
        return "true" if a else "false"
    if isinstance(a, int):
        return str(a)
    s = str(a)
    # Atom-safe names pass through; anything else becomes a string literal.
    if s and all(c.isalnum() or c in "-_*?." for c in s) and not s[0].isdigit():
        return s
    return escape_string_literal(s)


def _fact_to_sexpr(rel: str, args: tuple) -> str:
    inner = " ".join(_arg_to_sexpr(a) for a in args)
    return f"({rel} {inner})" if inner else f"({rel})"


def step_to_ir(step: TraceStep) -> str:
    """One `(step …)` S-expression line."""
    parts = [f"(step s{step.n} :rule {step.rule}"]
    if step.premises:
        if len(step.premises) == 1:
            using = _fact_to_sexpr(*step.premises[0])
        else:
            using = "(and " + " ".join(_fact_to_sexpr(*p) for p in step.premises) + ")"
        parts.append(f":using {using}")
    parts.append(f":derives {_fact_to_sexpr(*step.derived)}")
    if step.bindings:
        binds = " ".join(f"?{k} {_arg_to_sexpr(v)}" for k, v in step.bindings.items())
        parts.append(f":bind ({binds})")
    if step.why:
        parts.append(f":why {escape_string_literal(step.why)}")
    return " ".join(parts) + ")"


def trace_to_ir(steps: list[TraceStep]) -> str:
    """The whole trace as a `(trace …)` IR form (round-trips with the parser)."""
    body = "\n  ".join(step_to_ir(s) for s in steps)
    return f"(trace\n  {body})" if steps else "(trace)"


# ── parse back ─────────────────────────────────────────────────────

def _atom_or_value(x: object) -> object:
    """One IR scalar's Python value: ``Atom`` → name, ``Int`` / ``String``
    → value, anything else → ``str(x)``. Shared by the fact-ref and the
    binding parsers (S1.7c.29)."""
    if isinstance(x, Atom):
        return x.name
    if isinstance(x, (IRInt, String)):
        return x.value
    return str(x)


def _sform_to_factref(form: SForm) -> FactRef:
    rel = form.head.name if isinstance(form.head, Atom) else str(form.head)
    args: list = []
    for a in form.args:
        if isinstance(a, KwPair):
            continue
        args.append(
            _sform_to_factref(a) if isinstance(a, SForm) else _atom_or_value(a)
        )
    return (rel, tuple(args))


def _parse_using(val: SForm) -> tuple[FactRef, ...]:
    """Premises from a `:using` value — an `(and …)` of facts, or one fact."""
    if val.head.name == "and":
        return tuple(_sform_to_factref(c) for c in val.args
                     if isinstance(c, SForm))
    return (_sform_to_factref(val),)


def _parse_bindings(val: SForm) -> dict[str, str]:
    """Variable bindings from a `:bind (?v value …)` flat pair list."""
    items = [val.head, *val.args]
    bindings: dict[str, str] = {}
    for i in range(0, len(items) - 1, 2):
        var_name = getattr(items[i], "name", None)
        if var_name:
            bindings[var_name] = _atom_or_value(items[i + 1])
    return bindings


def _parse_step(ev: SForm, default_n: int) -> TraceStep:
    """One `(step …)` form → a :class:`TraceStep` (serialisable core only).

    Keyed off a ``{kw: value}`` map rather than an ``if/elif`` ladder over
    the arg sequence (S1.7c.29) — each field is a flat, guarded lookup, and
    a malformed value (wrong IR node type for its key) falls back to the
    field default exactly as the old ladder's per-arm ``isinstance`` guards
    did.
    """
    name = next((a.name for a in ev.args if isinstance(a, Atom)), "s0")
    n = int(name[1:]) if name[1:].isdigit() else default_n
    kw = {a.key.name: a.value for a in ev.args if isinstance(a, KwPair)}
    rule_val = kw.get("rule")
    rule = (
        "" if rule_val is None
        else rule_val.name if isinstance(rule_val, Atom)
        else str(rule_val)
    )
    using, derives, bind, why = (
        kw.get("using"), kw.get("derives"), kw.get("bind"), kw.get("why"),
    )
    return TraceStep(
        n=n,
        rule=rule,
        premises=_parse_using(using) if isinstance(using, SForm) else (),
        derived=_sform_to_factref(derives) if isinstance(derives, SForm) else ("", ()),
        bindings=_parse_bindings(bind) if isinstance(bind, SForm) else {},
        why=why.value if isinstance(why, String) else "",
    )


def parse_trace_steps(form: SForm) -> list[TraceStep]:
    """Reconstruct `TraceStep`s from a parsed `(trace …)` form.

    Recovers the serialisable core; ``diagram`` / ``section`` are not
    serialised and come back ``None``.
    """
    steps: list[TraceStep] = []
    for ev in form.args:
        if isinstance(ev, SForm) and ev.head.name == "step":
            steps.append(_parse_step(ev, len(steps) + 1))
    return steps


__all__ = ["FactRef", "TraceStep", "parse_trace_steps", "step_to_ir", "trace_to_ir"]
