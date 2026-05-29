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
    return '"' + s.replace("\\", "\\\\").replace('"', '\\"') + '"'


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
        parts.append(':why "' + step.why.replace("\\", "\\\\").replace('"', '\\"') + '"')
    return " ".join(parts) + ")"


def trace_to_ir(steps: list[TraceStep]) -> str:
    """The whole trace as a `(trace …)` IR form (round-trips with the parser)."""
    body = "\n  ".join(step_to_ir(s) for s in steps)
    return f"(trace\n  {body})" if steps else "(trace)"


# ── parse back ─────────────────────────────────────────────────────

def _sform_to_factref(form: SForm) -> FactRef:
    rel = form.head.name if isinstance(form.head, Atom) else str(form.head)
    args: list = []
    for a in form.args:
        if isinstance(a, KwPair):
            continue
        if isinstance(a, SForm):
            args.append(_sform_to_factref(a))
        elif isinstance(a, Atom):
            args.append(a.name)
        elif isinstance(a, IRInt):
            args.append(a.value)
        elif isinstance(a, String):
            args.append(a.value)
        else:
            args.append(str(a))
    return (rel, tuple(args))


def parse_trace_steps(form: SForm) -> list[TraceStep]:
    """Reconstruct `TraceStep`s from a parsed `(trace …)` form.

    Recovers the serialisable core; ``diagram`` / ``section`` are not
    serialised and come back ``None``.
    """
    steps: list[TraceStep] = []
    for ev in form.args:
        if not isinstance(ev, SForm) or ev.head.name != "step":
            continue
        name = next((a.name for a in ev.args if isinstance(a, Atom)), "s0")
        n = int(name[1:]) if name[1:].isdigit() else len(steps) + 1
        rule = ""
        premises: tuple[FactRef, ...] = ()
        derived: FactRef = ("", ())
        bindings: dict[str, str] = {}
        why = ""
        for arg in ev.args:
            if not isinstance(arg, KwPair):
                continue
            key, val = arg.key.name, arg.value
            if key == "rule":
                rule = val.name if isinstance(val, Atom) else str(val)
            elif key == "using" and isinstance(val, SForm):
                if val.head.name == "and":
                    premises = tuple(_sform_to_factref(c) for c in val.args
                                     if isinstance(c, SForm))
                else:
                    premises = (_sform_to_factref(val),)
            elif key == "derives" and isinstance(val, SForm):
                derived = _sform_to_factref(val)
            elif key == "why" and isinstance(val, String):
                why = val.value
            elif key == "bind" and isinstance(val, SForm):
                flat = [val.head, *val.args]
                names = [x for x in flat]
                i = 0
                while i + 1 < len(names):
                    var, value = names[i], names[i + 1]
                    var_name = getattr(var, "name", None)
                    if var_name:
                        bindings[var_name] = (
                            value.name if isinstance(value, Atom)
                            else value.value if isinstance(value, (IRInt, String))
                            else str(value)
                        )
                    i += 2
        steps.append(TraceStep(n=n, rule=rule, premises=premises, derived=derived,
                               bindings=bindings, why=why))
    return steps


__all__ = ["FactRef", "TraceStep", "parse_trace_steps", "step_to_ir", "trace_to_ir"]
