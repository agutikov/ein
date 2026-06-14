#!/usr/bin/env python3
"""Generate the zebra2 *variant* fixtures from the canonical ``zebra2.ein``.

The two variants are zebra2 ± exactly one FACT-layer fact, with an *identical*
schema + rule set (the invariant ``tests/integration/test_zebra_parse.py``
pins). Deriving them here — rather than hand-maintaining three near-copies —
means a change to zebra2's rules never silently drifts the variants: just
re-run this script.

  examples/zebra2-minus-15.ein   — GAPS fixture: condition (15) removed
                                   (under-determined → solve() reports gaps).
  examples/ein-bugs/zebra2-bad.ein — CONTRADICTIONS fixture: an extra
                                   (color-loc Green House-1), which condition
                                   (6) forbids (no house is right-of House-1).

Usage:  python3 examples/gen_zebra2_variants.py [--check]
        --check  exit non-zero if the on-disk variants are stale (CI guard),
                 without rewriting them.
"""
from __future__ import annotations

import argparse
import sys
from pathlib import Path

HERE = Path(__file__).resolve().parent
ZEBRA2 = HERE / "zebra2.ein"
MINUS_15 = HERE / "zebra2-minus-15.ein"
BAD = HERE / "ein-bugs" / "zebra2-bad.ein"

# The condition-(15) block as it appears verbatim in zebra2.ein.
COND_15_BLOCK = (
    ";; (15) The Norwegian lives next to the blue house.\n"
    '(adjacent-via next-to  nation-loc Norwegian   color-loc Blue    '
    ':source "condition (15)")\n'
)

MINUS_15_MARK = (
    ";; ──── GAPS fixture: condition (15) REMOVED (generated) ──────\n"
    ";; The canonical zebra2.ein closes the conditions with (15); this variant\n"
    ";; drops it, leaving the puzzle under-determined — a GAPS case for solve().\n"
    ";; Everything else (schema, rules, ontology, conditions 1-14) is identical;\n"
    ";; regenerate with examples/gen_zebra2_variants.py.\n"
)

BAD_INJECT = (
    ";; ──── INJECTED CONTRADICTION: zebra2-bad fixture (generated) ──────\n"
    ";; condition (6) forces Green ≠ House-1 (no house is right-of House-1);\n"
    ";; this injected positive contradicts it — a CONTRADICTIONS case for\n"
    ";; solve(). Otherwise identical to zebra2; regenerate with\n"
    ";; examples/gen_zebra2_variants.py.\n"
    '(color-loc Green House-1 :source "injected contradiction")\n'
)


def _render() -> tuple[str, str]:
    src = ZEBRA2.read_text(encoding="utf-8")
    if COND_15_BLOCK not in src:
        sys.exit(
            "error: condition-(15) block not found verbatim in zebra2.ein — "
            "the generator's anchor is stale, update COND_15_BLOCK."
        )
    minus_15 = src.replace(COND_15_BLOCK, MINUS_15_MARK)
    bad = src.replace(COND_15_BLOCK, COND_15_BLOCK + "\n" + BAD_INJECT)
    return minus_15, bad


def main(argv: list[str] | None = None) -> int:
    ap = argparse.ArgumentParser(description=__doc__,
                                 formatter_class=argparse.RawDescriptionHelpFormatter)
    ap.add_argument("--check", action="store_true",
                    help="verify the on-disk variants are up to date; "
                         "exit 1 if stale (do not rewrite).")
    args = ap.parse_args(argv)

    minus_15, bad = _render()
    targets = [(MINUS_15, minus_15), (BAD, bad)]

    if args.check:
        stale = [p.name for p, want in targets
                 if not p.exists() or p.read_text(encoding="utf-8") != want]
        if stale:
            print("stale zebra2 variants (re-run gen_zebra2_variants.py): "
                  + ", ".join(stale), file=sys.stderr)
            return 1
        print("zebra2 variants up to date.")
        return 0

    for path, want in targets:
        path.write_text(want, encoding="utf-8")
        print(f"wrote {path.relative_to(HERE.parent)}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
