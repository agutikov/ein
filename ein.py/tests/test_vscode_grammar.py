"""S1.7c.8 — the VSCode TextMate grammar must not drift from the kernel.

`utils/vscode-ein/ein.tmLanguage.json` highlights three *closed* reserved
name sets in head position — declarators, rule-body / ⊥ primitives, and
computed predicates. Those sets have a single source of truth:

- primitives → :mod:`ein_bot.inference.primitives` (``STRUCTURAL``)
- predicates → :mod:`ein_bot.inference.predicates` (``names()``)
- declarators → ``docs/kernel/ir/03-ein-lang/06_reserved_names.md`` (the
  closed P1.7c set; mirrored as ``EXPECTED_DECLARATORS`` below)

This test re-derives each list straight out of the grammar JSON and
asserts it equals the authoritative set — the "stray copy drifting from
the grammar" failure mode the plan calls out as the main risk. It also
guards the specific S1.7c.4 regression: the removed wrapper heads
(``ontology`` / ``facts`` / ``reasoning`` / ``rules``) must NOT be
highlighted as keywords.
"""
from __future__ import annotations

import json
import re
from pathlib import Path

import pytest

from ein_bot.inference import predicates, primitives

REPO = Path(__file__).resolve().parents[1]
# examples/ and utils/ live at the project root (one level above ein.py/).
GRAMMAR_PATH = REPO.parent / "utils" / "vscode-ein" / "ein.tmLanguage.json"

# The closed declarator set (P1.7c + P1.8 S1.5.9 `macro`). Source of truth:
# docs/kernel/ir/03-ein-lang/06_reserved_names.md.
EXPECTED_DECLARATORS = {
    "relation", "rule", "hrule", "query", "config", "trace", "macro",
}
# Wrapper heads removed in S1.7c.4 — must parse as ordinary fact heads,
# never highlighted as keywords.
REMOVED_WRAPPERS = {"ontology", "facts", "reasoning", "rules"}

DECLARATOR_SCOPE = "keyword.control.declarator.ein"
PRIMITIVE_SCOPE = "keyword.control.primitive.ein"
PREDICATE_SCOPE = "keyword.operator.predicate.ein"

# A lowercase `(a|b|c)` alternation inside a begin/match regex. Group-1
# `(\()` (literal paren, backslash) and the `([A-Za-z]…)` declared-name
# captures both start with a non-`[a-z]` char, so this never picks them up
# — it matches exactly the head-keyword alternation (capture group 2).
_ALT = re.compile(r"\(([a-z][a-z*-]*(?:\|[a-z][a-z*-]*)*)\)")


@pytest.fixture(scope="module")
def grammar() -> dict:
    return json.loads(GRAMMAR_PATH.read_text())


def _names_for_scope(node, scope: str) -> set[str]:
    """Union of every head-keyword alternation whose pattern assigns
    `scope` to capture group 2 (the head), walking the whole grammar."""
    found: set[str] = set()
    if isinstance(node, dict):
        caps = node.get("beginCaptures") or node.get("captures") or {}
        if caps.get("2", {}).get("name") == scope:
            regex = node.get("begin") or node.get("match") or ""
            for alt in _ALT.findall(regex):
                found.update(alt.split("|"))
        for value in node.values():
            found |= _names_for_scope(value, scope)
    elif isinstance(node, list):
        for value in node:
            found |= _names_for_scope(value, scope)
    return found


def test_grammar_is_valid_json_for_source_ein(grammar):
    assert grammar["scopeName"] == "source.ein"
    assert "ein" in grammar["fileTypes"]


def test_declarators_match_closed_set(grammar):
    assert _names_for_scope(grammar, DECLARATOR_SCOPE) == EXPECTED_DECLARATORS


def test_primitives_match_registry(grammar):
    # Since P1.8 S1.5.9 `open`/`forall` are stdlib macros, not primitives —
    # they are no longer highlighted as kernel primitives.
    expected = set(primitives.STRUCTURAL)
    assert _names_for_scope(grammar, PRIMITIVE_SCOPE) == expected


def test_predicates_match_registry(grammar):
    assert _names_for_scope(grammar, PREDICATE_SCOPE) == set(predicates.names())


def test_removed_wrappers_are_not_highlighted(grammar):
    """The S1.7c.4 regression guard: former block-wrapper heads are plain
    facts now, never declarators/primitives."""
    highlighted = (
        _names_for_scope(grammar, DECLARATOR_SCOPE)
        | _names_for_scope(grammar, PRIMITIVE_SCOPE)
        | _names_for_scope(grammar, PREDICATE_SCOPE)
    )
    assert not (highlighted & REMOVED_WRAPPERS)
