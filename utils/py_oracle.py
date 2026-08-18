#!/usr/bin/env python3
"""CPython's `repr()` and `format()` as a batch oracle — the parity gate for
`ein-core::pyrepr` and `ein-core::pyfmt` (M1a S1a.1.2).

ein.py's observable output leans on CPython's own rendering in a handful of
places: `key=repr` sorts, dataclass reprs in DOT labels, `{ms:9.2f}` in the
timing table. ein.rs has no Python, so those renderers are re-implementations
— and a re-implementation of a spec nobody wrote down is checked by
differential test or not at all
(design/02 §7, design/README's Q-M1a.15).

Sibling of `utils/ir_oracle.py`, deliberately separate: that one is *ein.py's
frontend*, this one is *CPython itself*. The only thing here that touches ein
is the `Fact` repr, which is a plain dataclass repr and degrades to a local
stand-in when the package is not importable.

    $ python3 utils/py_oracle.py < requests.jsonl > responses.jsonl

One JSON object per line in, one per line out, in order:

    {"op": "repr",   "v": {"s": "it's"}}          → {"out": "\\"it's\\""}
    {"op": "repr",   "v": {"t": [{"i": "-7"}]}}   → {"out": "(-7,)"}
    {"op": "format", "v": "3ff8000000000000", "spec": "9.2f"} → {"out": "     1.50"}

Values are tagged: `{"s": …}` str · `{"i": "…"}` int (decimal text, so an
arbitrary width survives JSON) · `{"t": [...]}` tuple · `{"f": [name, [args]]}`
`Fact`. A float is its **IEEE-754 bit pattern in hex**, which is the only
encoding that carries `nan`, `-0.0` and the subnormals through JSON intact.
"""
from __future__ import annotations

import json
import struct
import sys
from dataclasses import dataclass, field
from pathlib import Path

REPO = Path(__file__).resolve().parent.parent
sys.path.insert(0, str(REPO / "ein.py" / "src"))

try:
    from ein.kb.entities import Fact
except Exception:                                    # noqa: BLE001
    # The repr under test is the dataclass's, not the class's behaviour, so a
    # stand-in with the same fields and the same `repr=False` flags renders
    # identically. Keeps this script usable in a checkout with no ein.py.
    @dataclass(frozen=True)
    class Fact:                                      # type: ignore[no-redef]
        relation_name: str
        args: tuple
        provenance: object = field(default=None, compare=False, repr=False)
        raw: object = field(default=None, compare=False, repr=False)
        loc: object = field(default=None, compare=False, repr=False)
        _kb: object = field(default=None, compare=False, repr=False)


def _value(v):
    """Decode the tagged value language."""
    if "s" in v:
        return v["s"]
    if "i" in v:
        return int(v["i"])
    if "t" in v:
        return tuple(_value(x) for x in v["t"])
    if "f" in v:
        name, args = v["f"]
        return Fact(relation_name=name, args=tuple(_value(a) for a in args))
    raise ValueError(f"untagged value {v!r}")


def _float(hexbits: str) -> float:
    return struct.unpack(">d", bytes.fromhex(hexbits))[0]


def _handle(req: dict) -> dict:
    op = req["op"]
    if op == "repr":
        return {"ok": True, "out": repr(_value(req["v"]))}
    if op == "format":
        return {"ok": True, "out": format(_float(req["v"]), req["spec"])}
    raise ValueError(f"unknown op {op!r}")


def main() -> int:
    for line in sys.stdin:
        line = line.strip()
        if not line:
            continue
        try:
            resp = _handle(json.loads(line))
        except Exception as e:                       # noqa: BLE001
            resp = {"ok": False, "kind": type(e).__name__, "err": str(e)}
        sys.stdout.write(json.dumps(resp, ensure_ascii=False) + "\n")
        sys.stdout.flush()
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
