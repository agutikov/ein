"""The oracle event protocol — `--events FILE` (M1a S1a.0.2).

One JSON object per line describing what the engine did: every compile miss,
enqueue, firing, mirror, park/admit/retire, quiescence, alternative
justification, hypothesis verdict, entering, no-good and writeback, in order.
It exists for the conformance harness's **T2** tier — "the two engines took the
same steps" — which is the tier that pins the *algorithm* rather than its
answer. The schema is [`conformance/EVENTS.md`](../../../conformance/EVENTS.md).

Three properties the protocol depends on, and how this module gets them:

- **Off is free.** :data:`ON` is a module-level ``bool``. Call sites read it
  before building anything::

      if events.ON:
          events.emit("fire", rule=plan.rule_name, ...)

  so with the flag absent the cost is one global read — no kwargs dict, no
  formatting, no generator materialised. Writing ``events.emit(...)``
  unguarded would pack a `dict` at every call whatever the flag says, which on
  the firing path (≈ 234 k calls on exhaustive zebra2) is not free.

- **Emitting cannot change behaviour.** Nothing here touches engine state. In
  particular it must never advance ``Saturator._tiebreaker``, consume a
  generator the caller will consume again, or iterate a mapping the caller is
  mutating. A protocol that perturbs the run it describes is not an oracle.

- **No internal ids.** Facts go out as the canonical s-expression
  :func:`ein.cli._factdump.fact_sexpr` already produces, so nothing in the
  stream depends on either implementation's interning, object identity or
  dict addresses.

The writer flushes per line: a crashed run's prefix is the most useful
artefact it can leave, and the differ is built to read one.
"""
from __future__ import annotations

import json
from typing import Any, TextIO

SCHEMA = "ein-events/1"

#: True while a run is being recorded. Read this before building an event.
ON: bool = False

_writer: TextIO | None = None
_seq: int = 0
_verbose: bool = False


def open_log(path: str, /, *, level: str = "normal", **run_fields: Any) -> None:
    """Start recording to ``path`` and emit the opening `run` event.

    ``level`` is ``"normal"`` or ``"verbose"``; see :func:`want_verbose`.
    ``run_fields`` are merged into the `run` event — the CLI passes `impl`,
    `file`, `argv` and the resolved config.
    """
    global ON, _writer, _seq, _verbose
    if level not in ("normal", "verbose"):
        raise ValueError(f"--events-level expects normal|verbose, got {level!r}")
    close_log()
    _writer = open(path, "w", encoding="utf-8")
    _seq = 0
    _verbose = level == "verbose"
    ON = True
    emit("run", version=SCHEMA, level=level, **run_fields)


def close_log() -> None:
    """Stop recording. Idempotent."""
    global ON, _writer
    ON = False
    if _writer is not None:
        _writer.close()
        _writer = None


def want_verbose() -> bool:
    """True at ``--events-level verbose``.

    Guards the high-volume events — chiefly redundant firings, ~194 k of them
    on exhaustive zebra2 against ~40 k productive ones. They are off by default
    so the file stays navigable by hand, and **on for T2 comparisons**: a
    dropped redundant firing is exactly the kind of difference a port
    introduces.
    """
    return _verbose


def emit(kind: str, /, **fields: Any) -> None:
    """Write one event. A no-op when recording is off.

    ``kind`` is positional-only: several events carry a field of their own
    called `kind` (an entering's outcome, for one), and a keyword parameter
    would collide with it.

    Call sites still guard on :data:`ON` — this check is the backstop, not the
    optimisation.
    """
    global _seq
    if _writer is None:
        return
    line = {"e": kind, "n": _seq}
    line.update(fields)
    _seq += 1
    _writer.write(json.dumps(line, ensure_ascii=False, default=str) + "\n")
    _writer.flush()


def seq() -> int:
    """Events written so far — for tests that assert a run produced some."""
    return _seq


# ── Value rendering ────────────────────────────────────────────────
#
# Imported lazily: `ein.cli` pulls in argparse and the renderers, and the
# engine must not depend on the CLI at import time.


def fact(f: Any) -> str:
    """A `Fact` (or a bare arg) as its canonical s-expression.

    `fact_sexpr` already handles the whole shape, nesting included — this is
    the one renderer, not a second one that agrees with it by inspection.
    """
    from .cli._factdump import fact_sexpr
    return fact_sexpr(f)


def facts(seq_: Any) -> list[str]:
    """A sequence of facts, in the order given — order is the observable."""
    return [fact(f) for f in seq_]


def fact_id(fid: Any) -> str:
    """A `(relation, args)` id tuple as its s-expression."""
    from .cli._factdump import fact_sexpr
    rel, args = fid
    return f"({rel} {' '.join(fact_sexpr(a) for a in args)})" if args else f"({rel})"


def bindings(b: Any) -> list[list[str]]:
    """Bindings as ordered ``[name, value]`` pairs.

    A list of pairs, not an object: **binding order is the observable** (it is
    the order the matcher bound the variables, and it lands in
    `Provenance.bindings` and thus in the trace), and a JSON object's key order
    is not something a differ should have to trust.
    """
    from .cli._factdump import fact_sexpr
    return [[k, fact_sexpr(v)] for k, v in b.items()]


__all__ = [
    "ON",
    "SCHEMA",
    "bindings",
    "close_log",
    "emit",
    "fact",
    "fact_id",
    "facts",
    "open_log",
    "seq",
    "want_verbose",
]
