#!/usr/bin/env python3
"""ein.py's IR frontend as a batch oracle — the parity gate for M1a P1a.1.

`ein.rs`'s lexer, parser, dumper, macro expander and import resolver are
checked against *this* process, because ein.py is the oracle: any observable
difference is a bug in the port (M1a invariant I1). The conformance harness
cannot do it — it compares two `ein` CLIs, and the frontend has no CLI surface
of its own (the `ir` inspector subcommands were removed in P1.11).

Deliberately a **batch** protocol over stdin/stdout rather than one process per
file: building the Lark grammar costs ~0.5 s, and the differential fuzzer sends
10⁶ inputs.

    $ python3 utils/ir_oracle.py < requests.jsonl > responses.jsonl

One JSON object per line in, one per line out, in order:

    {"op": "accept",   "text": "(a b)"}          → {"ok": true}
    {"op": "parse",    "path": "examples/x.ein"} → {"ok": true, "out": "<dump_canonical>"}
    {"op": "compact",  "path": …}                → one `dump_compact` per form, newline-joined
    {"op": "resolve",  "path": …}                → dump_canonical(resolve_imports(…))
    {"op": "minimize", "path": …}                → dump_canonical(resolve_and_minimize(…))
    {"op": "macro-names"}                        → the names `std.macro` exports, sorted

`accept` skips the dump, which is what makes the fuzzer affordable. A failure
is `{"ok": false, "err": "<message>", "kind": "IRParseError"|"KBLoadError"|…}`
with the message ein.py would print — byte-for-byte what the port must
produce.

`path` is repo-root-relative or absolute, and is what `filename` becomes, so
`Loc`s and error messages name the same file both sides see. `text` may carry
an explicit `"filename"` (default `null` → ein.py's `<string>`).
"""
from __future__ import annotations

import json
import sys
from pathlib import Path

REPO = Path(__file__).resolve().parent.parent
sys.path.insert(0, str(REPO / "ein.py" / "src"))

from ein.ir import IRParseError, parse                      # noqa: E402
from ein.ir.dump import dump_canonical, dump_compact        # noqa: E402
from ein.kb.imports import (                                # noqa: E402
    resolve_and_minimize,
    resolve_imports,
    stdlib_macro_names,
)


def _source(req: dict) -> tuple[str, str | None, Path | None]:
    """`(text, filename, base_dir)` for a request."""
    if "path" in req:
        path = Path(req["path"])
        if not path.is_absolute():
            path = REPO / path
        return path.read_text(encoding="utf-8"), str(path), path.parent
    return req["text"], req.get("filename"), None


def _handle(req: dict) -> dict:
    op = req.get("op", "parse")
    if op == "macro-names":
        return {"ok": True, "out": "\n".join(sorted(stdlib_macro_names()))}

    text, filename, base_dir = _source(req)
    forms = parse(text, filename=filename)
    if op == "accept":
        return {"ok": True}
    if op == "parse":
        return {"ok": True, "out": dump_canonical(forms)}
    if op == "compact":
        return {"ok": True, "out": "\n".join(dump_compact(f) for f in forms)}
    if op == "resolve":
        return {"ok": True, "out": dump_canonical(resolve_imports(forms, base_dir=base_dir))}
    if op == "minimize":
        return {"ok": True,
                "out": dump_canonical(resolve_and_minimize(forms, base_dir=base_dir))}
    raise ValueError(f"unknown op {op!r}")


def main() -> int:
    out = sys.stdout
    for line in sys.stdin:
        line = line.strip()
        if not line:
            continue
        try:
            resp = _handle(json.loads(line))
        except IRParseError as e:
            resp = {"ok": False, "kind": "IRParseError", "err": str(e)}
        except Exception as e:                       # noqa: BLE001 — the oracle
            # reports whatever ein.py raises; the caller decides whether a
            # non-parse failure is a parity question or a fixture bug.
            resp = {"ok": False, "kind": type(e).__name__, "err": str(e)}
        out.write(json.dumps(resp, ensure_ascii=False) + "\n")
        out.flush()
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
