#!/usr/bin/env python3
"""Generate the zebra2 *variant* fixtures from the canonical ``zebra2.ein``.

Each variant is zebra2 with exactly one block added or removed, and an
*identical* schema + rule set. Deriving them here — rather than
hand-maintaining five near-copies — means a change to zebra2's rules never
silently drifts a variant: just re-run this script.

  examples/zebra2-minus-15.ein   — GAPS fixture: condition (15) removed
                                   (under-determined → solve() reports gaps).
  examples/ein-bugs/zebra2-bad.ein — CONTRADICTIONS fixture: an extra
                                   (color-loc Green House-1), which condition
                                   (6) forbids (no house is right-of House-1).
  examples/zebra2-obligations.ein — the hypothesis rule `(rule guess …)`
                                   and its five activators `(guess …)`
                                   removed, and NOTHING else. The theory alone
                                   drives the
                                   search — `(bijective *-loc)` fans out into
                                   `total-owed` / `surjective-owed`, and the
                                   obligations write what would pay it.
                                   "Nothing else changed" is this script's
                                   claim rather than a reader's.
  examples/zebra2-minus-15-obligations.ein — both at once: the under-determined
                                   regime with no hypothesis rule.

Usage:  python3 utils/gen_zebra2_variants.py [--check]
        --check  exit non-zero if the on-disk variants are stale (the gate's
                 guard, run by utils/check_generated.py), without rewriting them.

The variants say they are generated and name no tool: which script writes
them is recorded here and in utils/README.md.
"""
from __future__ import annotations

import argparse
import sys
from pathlib import Path

REPO = Path(__file__).resolve().parent.parent
HERE = REPO / "examples"
ZEBRA2 = HERE / "zebra2.ein"
MINUS_15 = HERE / "zebra2-minus-15.ein"
BAD = HERE / "ein-bugs" / "zebra2-bad.ein"
OBLIGATIONS = HERE / "zebra2-obligations.ein"
MINUS_15_OBLIGATIONS = HERE / "zebra2-minus-15-obligations.ein"

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
    ";; generated from zebra2.ein, not edited by hand.\n"
)

BAD_INJECT = (
    ";; ──── INJECTED CONTRADICTION: zebra2-bad fixture (generated) ──────\n"
    ";; condition (6) forces Green ≠ House-1 (no house is right-of House-1);\n"
    ";; this injected positive contradicts it — a CONTRADICTIONS case for\n"
    ";; solve(). Otherwise identical to zebra2, from which it is\n"
    ";; generated, not edited by hand.\n"
    '(color-loc Green House-1 :source "injected contradiction")\n'
)


# The hypothesis-rule block, verbatim — the rule and the comment that
# introduces it.
RULE_BLOCK = (
    ";; ──── Hypothesis generation ────────────────────────────────\n"
    ";; One rule parameterised over (?R ?T1 ?T2) supposes (?R ?a ?b), a\n"
    ";; hypothesis, for every ?a of type ?T1 and ?b of type ?T2; its\n"
    ";; activators, after the conditions, name every *-loc relation with\n"
    ";; its (attribute-type, House) pair.\n"
    "(rule guess (?R ?T1 ?T2)\n"
    "  :match  (and (is-a ?a ?T1) (is-a ?b ?T2))\n"
    "  :assert (open (?R ?a ?b))\n"
    '  :why    "guess: ({?R} {?a} {?b})?")\n'
)

# …and its activators, facts the program writes after the conditions, with
# the comment that introduces them.
ACTIVATORS = (
    ";; The hypothesis rule's activators: one (?R ?T1 ?T2) triple per\n"
    ";; *-loc relation; `guess` supposes (?R ?v ?h) for every (v, h)\n"
    ";; of the corresponding types.\n"
    "(guess color-loc  Color       House)\n"
    "(guess nation-loc Nationality House)\n"
    "(guess drink-loc  Drink       House)\n"
    "(guess smoke-loc  Cigarette   House)\n"
    "(guess pet-loc    Pet         House)\n"
    "\n"
)

OBLIGATIONS_MARK = (
    ";; ──── Hypothesis generation: THERE IS NONE (generated) ──────\n"
    ";; The canonical zebra2.ein declares a rule `guess (?R ?T1 ?T2)` here,\n"
    ";; which concludes a hypothesis, and writes its activators after the\n"
    ";; conditions. This variant deletes both, and nothing else — which is a\n"
    ";; claim this generator makes rather than a reader.\n"
    ";;\n"
    ";; What proposes the (?R ?v ?h) candidates instead is the theory the file\n"
    ";; already states. `(bijective color-loc)` and its four siblings fan out\n"
    ";; into std.algebra's `total-owed` / `surjective-owed`, each of which\n"
    ";; reports an unwitnessed slot as `(open ?R)`; the obligations write\n"
    ";; exactly the facts that would discharge one. The rule said \"guess a\n"
    ";; (value, house) pair\"; the bijection declaration had already said it,\n"
    ";; and now the engine hears it.\n"
    ";;\n"
    ";; This file solving is the fixture for it: no hypothesis rule is needed\n"
    ";; where the theory (rules + ontology) already says what to guess.\n"
    ";; Generated from zebra2.ein, not edited by hand.\n"
)

OBLIGATIONS_KW_MARK = (
    ";; No `(guess …)` activator (generated) — the five `(bijective *-loc)`\n"
    ";; declarations are the hypothesis-generator activators now: one obligation\n"
    ";; per unlocated value and one per unfilled house, and the branch is that\n"
    ";; obligation's own domain scan.\n"
    "\n"
)


def _render() -> dict[Path, str]:
    src = ZEBRA2.read_text(encoding="utf-8")
    for name, block in (("condition-(15)", COND_15_BLOCK),
                        ("hypothesis rule", RULE_BLOCK),
                        ("activators", ACTIVATORS)):
        if src.count(block) != 1:
            sys.exit(
                f"error: the {name} block is not in zebra2.ein exactly once — "
                "the generator's anchor is stale, update this script."
            )
    minus_15 = src.replace(COND_15_BLOCK, MINUS_15_MARK)
    bad = src.replace(COND_15_BLOCK, COND_15_BLOCK + "\n" + BAD_INJECT)
    no_guess = (src.replace(RULE_BLOCK, OBLIGATIONS_MARK)
                   .replace(ACTIVATORS, OBLIGATIONS_KW_MARK))
    minus_15_no_guess = (minus_15.replace(RULE_BLOCK, OBLIGATIONS_MARK)
                                 .replace(ACTIVATORS, OBLIGATIONS_KW_MARK))
    return {
        MINUS_15: minus_15,
        BAD: bad,
        OBLIGATIONS: no_guess,
        MINUS_15_OBLIGATIONS: minus_15_no_guess,
    }


def main(argv: list[str] | None = None) -> int:
    ap = argparse.ArgumentParser(description=__doc__,
                                 formatter_class=argparse.RawDescriptionHelpFormatter)
    ap.add_argument("--check", action="store_true",
                    help="verify the on-disk variants are up to date; "
                         "exit 1 if stale (do not rewrite).")
    args = ap.parse_args(argv)

    targets = list(_render().items())

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
        print(f"wrote {path.relative_to(REPO)}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
