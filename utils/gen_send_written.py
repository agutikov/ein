#!/usr/bin/env python3
"""Generate ``beyond_arith/send_more_money_written.ein`` from
``beyond_arith/send_more_money.ein``.

The variant is SEND + MORE = MONEY with its eight letters declared in the
order the puzzle is written, S E N D M O R Y, where the canonical file
declares them by column, D E Y N R O S M. The line of ``(is-a … Letter)``
facts is reordered and the header's paragraph about the order is replaced;
**nothing else changes**, which is this script's claim rather than a
reader's. The two programs have one universe, one theory and one answer, so
whatever a search does on one and not on the other it read off the order of
eight facts: ``ein-infer/tests/order.rs`` holds ``--single`` to the same
work on both.

Usage:  python3 utils/gen_send_written.py [--check]
        --check  exit non-zero if the on-disk variant is stale (the gate's
                 guard, run by utils/check_generated.py), without rewriting it.

The variant says it is generated and names no tool: which script writes it is
recorded here and in utils/README.md.
"""
from __future__ import annotations

import argparse
import sys
from pathlib import Path

REPO = Path(__file__).resolve().parent.parent
HERE = REPO / "examples"
SEND = HERE / "beyond_arith" / "send_more_money.ein"
WRITTEN = HERE / "beyond_arith" / "send_more_money_written.ein"

BY_COLUMN = "D E Y N R O S M"
AS_WRITTEN = "S E N D M O R Y"


def _letters(order: str) -> str:
    return " ".join(f"(is-a {x} Letter)" for x in order.split()) + "\n"


# The header's paragraph on the order, verbatim: everything from its first
# line up to the paragraph that follows it.
ORDER_FROM = ";;; The letters are declared by column, units first: D E Y, then N R, O,\n"
ORDER_TO = ";;; I + BB = ILL beside it,"

WRITTEN_MARK = (
    ";;; ──── The letters as the puzzle is written (generated) ──────\n"
    ";;; `send_more_money.ein` declares its letters by column, units first:\n"
    ";;; D E Y, then N R, O, S M. This file declares them S E N D M O R Y, and\n"
    ";;; changes nothing else: one line of `(is-a … Letter)` facts, reordered.\n"
    ";;; The two are one puzzle with one answer, and `--single` does the same\n"
    ";;; work on both: the search reads the columns off the program, not the\n"
    ";;; order. Generated from it, not edited by hand.\n"
    ";;;\n"
)


def _render() -> dict[Path, str]:
    src = SEND.read_text(encoding="utf-8")
    line = _letters(BY_COLUMN)
    for name, block in (("letters'", line), ("order paragraph's first", ORDER_FROM)):
        if src.count(block) != 1:
            sys.exit(
                f"error: the {name} line is not in send_more_money.ein exactly "
                "once — the generator's anchor is stale, update this script."
            )
    start = src.index(ORDER_FROM)
    end = src.find(ORDER_TO, start)
    if end < 0:
        sys.exit(
            "error: the paragraph after the order's is not in "
            "send_more_money.ein — the generator's anchor is stale, update "
            "this script."
        )
    out = src[:start] + WRITTEN_MARK + src[end:]
    out = out.replace(line, _letters(AS_WRITTEN))
    return {WRITTEN: out}


def main(argv: list[str] | None = None) -> int:
    ap = argparse.ArgumentParser(description=__doc__,
                                 formatter_class=argparse.RawDescriptionHelpFormatter)
    ap.add_argument("--check", action="store_true",
                    help="exit non-zero if the on-disk variant is stale")
    args = ap.parse_args(argv)
    stale = []
    for path, text in _render().items():
        on_disk = path.read_text(encoding="utf-8") if path.exists() else None
        if on_disk == text:
            continue
        if args.check:
            stale.append(path.relative_to(REPO))
        else:
            path.write_text(text, encoding="utf-8")
            print(f"wrote {path.relative_to(REPO)}")
    if stale:
        print("stale SEND variant (re-run gen_send_written.py): "
              + ", ".join(str(p) for p in stale))
        return 1
    return 0


if __name__ == "__main__":
    sys.exit(main())
